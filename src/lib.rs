//! bender_job is a rust library, that serializes and deserializes jobs
//! from `data.json` files. The deserialization yields a Job struct.  
//!
//! It can be loaded in a rust library via the public git mirror by putting this in your Cargo.toml:  
//! ```ignore
//! [dependencies]
//! bender_job = { git = "https://github.com/atoav/bender-job.git" }
//! ```
//! To update this run
//! ```ignore
//! cargo clean
//! cargo update
//! ```
//!
//! ## Testing
//! The libary is implemented with a extensive amount of tests to make
//! sure that repeated deserialization/serialization won't introduce
//! losses or glitches to the `data.json`. The tests can be run with
//! ```ignore
//! cargo test
//! ```
//!
//! ## Documentation
//! If you want to view the documentation run
//! ```ignore
//! cargo doc --no-deps --open
//! ```
//! 
//! ## Installation
//! To run cargo, make sure you have rust installed. Go to [rustup.rs](http://rustup.rs) and follow the instructions there
//! 


#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate bender_bouncer;

use chrono::prelude::*;
use chrono::Utc;
use std::collections::{HashMap, BTreeMap};
use std::str;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::error::Error;
use std::collections::VecDeque;

pub mod jobtime;
pub use jobtime::JobTime;

pub mod jobpaths;
pub use jobpaths::JobPaths;

pub mod task;
pub use task::Task;

pub mod status;
pub use status::{Status};

pub mod data;
pub use data::{Render, Frames, Resolution};

pub mod gaffer;
pub use gaffer::{Gaffer};

pub mod command;
pub use command::Command;

pub mod atomizer;
pub use atomizer::Atomizer;

pub mod bouncer;
pub use bouncer::Bouncer;



pub type GenError = Box<std::error::Error>;
pub type GenResult<T> = Result<T, GenError>;


pub type History = BTreeMap<DateTime<Utc>, String>;




/* --------------------------------[ Job ]-------------------------------- */

/// The Job struct holds all information about a job request for rendering
/// it gets created simply by reading from its `data.json`.
/// 
/// ## Create a Job
/// ### 1. from a data.json 
/// ```
/// # use bender_job::Job;
/// Job::from_datajson("some/path/to/data.json");
/// ```
///
/// ### 2. deserialized from a string 
/// ```
/// # use bender_job::Job;
/// Job::deserialize("myjsonstring".to_owned());
/// ```
///
/// ### 3. deserialization from bytes: &[u8]
/// ```
/// # use bender_job::Job;
/// let somebytes = "myjsonstring".as_bytes();
/// Job::deserialize_from_u8(somebytes);
/// ```
///
/// ### 4. direct construction 
/// (see tests/common/mod.rs for example)
/// 
/// ## Fields
/// - `Job::id: String` uniquely identifies a job, and stays the same always
/// - `Job::paths: JobPaths` a struct that holds the Paths relevant for a job. Also see [JobPaths](jobpaths/struct.JobPaths.html)
/// - `Job::email: String` stores the users email for updates on their job
/// - `Job::version: String` stores the version number of the blendfile, only set after validation in watchdog
/// - `Job::time: JobTime` a struct that holds all timestamps relevant for a job. Also see [JobTime](jobtime/struct.JobTime.html)
/// - `Job::status: String` the dot delimited Status of a job (e.g. "request.denied", "request.bouncer.finished", "job.done", etc)
/// - `Job::data: HashMap<String, String>` a HashMap that holds arbitrary data for the job that cannot be known on startup (e.g. "frames: 250")
/// - `Job::history: History` a ordered Treemap that acts as a timestampable Log for each Job.
/// - `Job::resolution: Resolution` stores x and y size, as well as the scale of the scene
/// - `Job::render: Render` stores general values about the renderer, such as fps etc
/// - `Job::frames: Frames` stores data related to the frame range
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Job {
    pub id: String,
    pub animation: bool,
    pub paths: JobPaths,
    pub email: String,
    #[serde(default)]
    pub version: String,
    pub time: JobTime,
    pub status: Status,
    pub data: HashMap<String, String>,
    pub history: History,
    #[serde(default)]
    pub resolution: Resolution,
    #[serde(default)]
    pub render: Render,
    #[serde(default)]
    pub frames: Frames,
    #[serde(default)]
    pub tasks: VecDeque<Task>
}





#[allow(dead_code)]
impl Job{
    /// Read a ID directly from the existing uploadfolder
    pub fn id(&self) -> String{
        self.paths.get_id()
    }

    /// Add to the history of a Job
    /// key is a DateTime constructed via `chrono::Utc::now()`
    /// value can be any String
    pub fn add_history<S>(&mut self, value: S) where S: Into<String> {
        self.history.insert(Utc::now(), value.into());
    }

    /// Add to the history of a job only if the added value changed from the last value
    /// Return Ok(()) if the value has been added otherwise return a boxed error
    pub fn add_history_debounced<S>(&mut self, value: S) where S: Into<String>{
        let value = value.into();
        let addtohistory =  match self.history.values().next_back(){
            Some(oldvalue) => {
                match &value  != oldvalue{
                    true => true,
                    false => false
                }
            },
            None => true
        };
        if addtohistory{
            self.add_history(value);
        }
    }

    /// Append a other history to the self.history
    pub fn incorporate_alternate_history(&mut self, other_history: &mut History){
        self.history.append(other_history);
    }

    /// Append a key-value-pair to the data of a Job
    /// e.g. `Job::add_data("watchdog.queueposition", "22")`
    pub fn add_data<S>(&mut self, key: S, value: S) where S: Into<String> {
        self.data.insert(key.into(), value.into());
    }

    /// Update data only if it changed, return an Error if something failed else return Ok
    pub fn add_data_debounced<S>(&mut self, key: S, value: S) -> GenResult<()> where S: Into<String> {
        // Insert returns Some(String) when the old value has been overwritten
        // or None when there was no value, let's use that
        let value = value.into();
        match self.data.insert(key.into(), value.clone()){
            Some(oldvalue) => {
                match value != oldvalue{
                    true => Ok(()),
                    false => Ok(())
                }
            },
            None => Ok(())
        }
    }

    /// Serialize a Job into a String. Return a Error if this fails
    pub fn serialize(&self) -> Result<String, Box<Error>> {
        let string = serde_json::to_string_pretty(&self)?;
        Ok(string)
    }

    /// Serialize a Job into a Vec<u8>. Return a Error if this fails
    /// you might want to use this with a reference
    pub fn serialize_to_u8(&self) -> Result<Vec<u8>, Box<Error>> {
        let string = serde_json::to_string_pretty(&self)?;
        Ok(string.into_bytes())
    }

    /// Deserialize something that fullfills Into<String> into a Job
    pub fn deserialize<S>(s: S) -> GenResult<Self> where S: Into<String> {
        let deserialized: Job = serde_json::from_str(&s.into()[..])?;
        Ok(deserialized)
    }

    /// Deserialize something that fullfills Into<String> into a Job
    pub fn deserialize_from_u8(v:&[u8]) -> GenResult<Self> {
        let s = str::from_utf8(v)?;
        let deserialized: Job = serde_json::from_str(&s)?;
        Ok(deserialized)
    }

    /// Write a serialized version of the Job to the path specified in `Job::paths::data`
    /// **Warning:** _This must only be used within ONE service!_e
    pub fn write_to_file(&self) -> GenResult<()> {
        // Step 1: Serialize
        let serialized = self.serialize()?;
        // Step 2: Write
        fs::write(&self.paths.data, serialized)?;
        Ok(())
    }

    /// Creates a file from a `data.json`, like
    /// ```
    /// # use bender_job::Job;
    /// let j = Job::from_datajson("some/path/to/data.json");
    /// ```
    pub fn from_datajson<S>(p: S) -> GenResult<Self> where S: Into<PathBuf>{
        let p = p.into();
        let bytes = &fs::read(p)?;
        let job = Self::deserialize_from_u8(bytes)?;
        Ok(job)
    }

    /// Convenience Function to create a Job from the path of a blend file.
    /// This assumes the data.json is stored right besides the blend file!
    /// ```
    /// # use bender_job::Job;
    /// let j = Job::from_blend("some/path/to/some.blend");
    /// ```
    pub fn from_blend<S>(p: S) -> GenResult<Self> where S: Into<PathBuf>{
        let mut p = p.into();
        p.pop();
        p.push("data.json");
        Self::from_datajson(p)
    }

    /// Convenience Function to create a Job from the directory containing a
    /// data.json.
    /// ```
    /// # use bender_job::Job;
    /// let j = Job::from_directory("some/path/to/blenddirectory");
    /// ``` 
    pub fn from_directory<S>(p: S) -> GenResult<Self> where S: Into<PathBuf>{
        let mut p = p.into();
        p.push("data.json");
        Self::from_datajson(p)
    }

    /// Return Ok(true) when the data on disk is different than self
    /// Return Ok(false) when the data is the same
    /// Return Error when reading from disk failed
    pub fn changed_on_disk(&self) -> GenResult<bool> {
        let datapath = self.paths.data.clone();
        let on_disk = &Self::from_datajson(datapath)?;
        Ok(self != on_disk)
    }

    /// Only write changes to data.json if there is a difference between the data
    /// stored on disk and self, Return Error if something failed, otherwise Ok()
    pub fn update_on_disk(&self) -> GenResult<()>{
        let shouldupdate = self.changed_on_disk()?;
        if shouldupdate{
            self.write_to_file()?;
        }
        Ok(())
    }
}



// =============================== State Checks ===============================
impl Job{
    /// Check if self is a request
    pub fn is_request(&self) -> bool{
        self.status.is_request()
    }

    /// Check if self is a invalid request
    pub fn is_invalid(&self) -> bool{
        self.status.is_invalid()
    }

    /// Check if self has been validated
    pub fn is_validated(&self) -> bool{
        self.status.is_validated()
    }   

    /// Check if self is a job
    pub fn is_job(&self) -> bool{
        self.status.is_job()
    }

    /// Return true if self is queued
    pub fn is_queued(&self) -> bool{
        self.status.is_queued()
    }
}




// ============================= Process Functions =============================
impl Job {
    pub fn validate(&mut self){
        self.check_with_bouncer();
    }

    pub fn deny(&mut self){
        self.set_deny();
    }

    pub fn error<S>(&mut self, error_message: S) where S: Into<String>{
        self.set_error(error_message.into());
    }

    pub fn scan(&mut self){
        self.scan_and_optimize();
    }

    pub fn atomize(&mut self){
        self.atomize_to_tasks();
    }

    pub fn queue(&mut self){
        self.set_queue();
    }

    pub fn run(&mut self){
        self.set_run();
    }

    pub fn finish(&mut self){
        self.set_finish();
    }

    pub fn cancel(&mut self){
        self.set_cancel();
    }
}



// =============================== State Setters ===============================
impl Job{
    /// Validate the self and log it to history, log errors
    pub fn set_validate(&mut self){
        match self.status.validate(){
            Ok(_) => {
                let message = format!("Validated with version: {}", self.version);
                self.add_history(message.as_str());
            },
            Err(err) => {
                let message = format!("Error: Job::status::validate() failed: {}", err);
                self.add_history(message.as_str());
            }
        }
    }

    /// Deny the self and log it to history, log errors
    pub fn set_deny(&mut self){
        match self.status.deny(){
            Ok(_) => {
                let message = format!("Denied Blendfile as invalid");
                self.add_history(message.as_str());
            },
            Err(err) => {
                let message = format!("Error: Job::status::deny() failed: {}", err);
                self.add_history(message.as_str());
            }
        }
    }

    /// Error self and log it to history, log errors
    pub fn set_error<S>(&mut self, error_message: S) where S: Into<String>{
        let error_message = error_message.into();
        match self.status.error(){
            Ok(_) => {
                let message = format!("Error: {}", error_message);
                self.add_history(message.as_str());
                self.time.error();
            },
            Err(err) => {
                let message = format!("Error: Job::status::validate() failed with: {}\nat Error:{}", err, error_message);
                self.add_history(message.as_str());
            }
        }
    }

    /// Scan self and log it to history, log errors
    pub fn set_scan(&mut self){
        match self.status.scan(){
            Ok(_) => {
                let message = format!("Scanning finished");
                self.add_history(message.as_str());
            },
            Err(err) => {
                let message = format!("Error: Job::status::scan() failed: {}", err);
                self.add_history(message.as_str());
            }
        }
    }

    /// Atomize self and log it to history, log errors
    pub fn set_atomize(&mut self){
        match self.status.atomize(){
            Ok(_) => {
                let message = format!("Atomization finished: created {} atomic tasks", self.tasks.len());
                self.add_history(message.as_str());
            },
            Err(err) => {
                let message = format!("Error: Job::status::atomize() failed: {}", err);
                self.add_history(message.as_str());
            }
        }
    }

    /// Queue self and log it to history, log errors
    pub fn set_queue(&mut self){
        match self.status.queue(){
            Ok(_) => {
                let message = format!("Queued Job");
                self.add_history(message.as_str());
            },
            Err(err) => {
                let message = format!("Error: Job::status::queue() failed: {}", err);
                self.add_history(message.as_str());
            }
        }
    }

    /// Run self and log it to history, log errors
    pub fn set_run(&mut self){
        match self.status.run(){
            Ok(_) => {
                let message = format!("running Job");
                self.add_history(message.as_str());
                self.time.start();
            },
            Err(err) => {
                let message = format!("Error: Job::status::run() failed: {}", err);
                self.add_history(message.as_str());
            }
        }
    }

    /// Finish self and log it to history, log errors
    pub fn set_finish(&mut self){
        match self.status.finish(){
            Ok(_) => {
                let message = format!("Finished Job");
                self.add_history(message.as_str());
                self.time.finish();
            },
            Err(err) => {
                let message = format!("Error: Job::status::finish() failed: {}", err);
                self.add_history(message.as_str());
            }
        }
    }

    /// Cancel self and log it to history, log errors
    pub fn set_cancel(&mut self){
        match self.status.cancel(){
            Ok(_) => {
                let message = format!("Canceled Job");
                self.add_history(message.as_str());
                self.time.abort();
            },
            Err(err) => {
                let message = format!("Error: Job::status::cancel() failed: {}", err);
                self.add_history(message.as_str());
            }
        }
    }
}



// ============================= JOB FROM METHODS =============================


/// Allows to create a Job by using `let request = Job::from(String);`
/// Only use this when you are 100% sure it will work, otherwise use Job::deserialize()
impl From<String> for Job{
    fn from(s: String) -> Self{
        let deserialized: Job = serde_json::from_str(&s).expect("Deserialization failed");
        deserialized
    }
}

/// Allows to create a Job by using `let request = Job::from(&String);`
/// Only use this when you are 100% sure it will work, otherwise use Job::deserialize()
impl <'a>From<&'a String> for Job{
    fn from(s: &String) -> Self{
        let deserialized: Job = serde_json::from_str(&s).expect("Deserialization failed");
        deserialized
    }
}

/// Allows to create a Job by using `let request = Job::from(&str);`
/// Only use this when you are 100% sure it will work, otherwise use Job::deserialize()
impl <'a>From<&'a str> for Job{
    fn from(s: &str) -> Self{
        let deserialized: Job = serde_json::from_str(&s).expect("Deserialization failed");
        deserialized
    }
}

/// This is very unsafe. Better use the `Job::from_datajson` method!
impl From<PathBuf> for Job{
    fn from(p: PathBuf) -> Self{
        let mut jsonbuf = PathBuf::from(&p);
        // Add data.json to the end of string if it isn't there already
        if !p.ends_with("data.json"){ jsonbuf.push("data.json"); }
        Self::deserialize_from_u8(&fs::read(jsonbuf).expect("Fuck, couldn't read from data.json"))
        .expect("Fuck, couldn't deserialize from data.json")
    }
}


/// String formatting for Job
/// Returns something that looks like this:
/// `"Job [id: 245869245686258gtre9524][status: request.untouched]"`
impl fmt::Display for Job {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let st = &format!("Job [id: {}][status: {}]", self.id, self.status)[..];
        fmt.write_str(st)?;
        Ok(())
    }
}








