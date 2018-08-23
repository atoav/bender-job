//! bender_job is a rust library, that serializes and deserializes jobs
//! from `data.json` files. The deserialization yields a Job struct.  
//!
//! It can be loaded in a rust library via the public git mirror:  
//! ```ignore
//! job = { git = "https://github.com/atoav/bender-job.git" }
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

use chrono::prelude::*;
use chrono::Utc;
use std::collections::{HashMap, BTreeMap};
use std::str;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::error::Error;





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
/// - `Job::paths: JobPaths` a struct that holds the Paths relevant for a job. Also see [JobPaths](struct.JobPaths.html)
/// - `Job::email: String` stores the users email for updates on their job
/// - `Job::time: JobTime` a struct that holds all timestamps relevant for a job. Also see [JobTime](struct.JobTime.html)
/// - `Job::status: String` the dot delimited Status of a job (e.g. "request.denied", "request.bouncer.finished", "job.done", etc)
/// - `Job::data: HashMap<String, String>` a HashMap that holds arbitrary data for the job that cannot be known on startup (e.g. "frames: 250")
/// - `Job::history: BTreeMap<DateTime<Utc>, String>` a ordered Treemap that acts as a timestampable Log for each Job.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Job {
    pub id: String,
    pub paths: JobPaths,
    pub email: String,
    pub time: JobTime,
    pub status: String,
    pub data: HashMap<String, String>,
    pub history: BTreeMap<DateTime<Utc>, String>
}

#[allow(dead_code)]
impl Job{
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

    /// Append a key-value-pair to the data of a Job
    /// e.g. `Job::add_data("watchdog.queueposition", "22")`
    pub fn add_data<S>(&mut self, key: S, value: S) where S: Into<String> {
        self.data.insert(key.into(), value.into());
    }

    /// Update data only if it changed, return an Error if something failed else return Ok
    pub fn add_data_debounced<S>(&mut self, key: S, value: S) -> Result<(), Box<Error>> where S: Into<String> {
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
    pub fn deserialize<S>(s: S) -> Result<Self, Box<Error>> where S: Into<String> {
        let deserialized: Job = serde_json::from_str(&s.into()[..])?;
        Ok(deserialized)
    }

    /// Deserialize something that fullfills Into<String> into a Job
    pub fn deserialize_from_u8(v:&[u8]) -> Result<Self, Box<Error>> {
        let s = str::from_utf8(v)?;
        let deserialized: Job = serde_json::from_str(&s)?;
        Ok(deserialized)
    }

    /// Read a ID directly from the existing uploadfolder
    pub fn id(&self) -> String{
        self.paths.get_id()
    }

    /// Write a serialized version of the Job to the path specified in `Job::paths::data`
    /// **Warning:** _This must only be used within ONE service!_e
    pub fn write_to_file(&self) -> Result<(), Box<Error>> {
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
    pub fn from_datajson<S>(p: S) -> Result<Self, Box<Error>> where S: Into<String>{
        let p = PathBuf::from(&p.into()[..]);
        let bytes = &fs::read(p)?;
        let job = Self::deserialize_from_u8(bytes)?;
        Ok(job)
    }

    /// Check if self is a request
    pub fn is_request(&self) -> bool{
        self.status.split(".").collect::<Vec<&str>>()[0] == "request"
    }

    /// Check if self is a job
    pub fn is_job(&self) -> bool{
        self.status.split(".").collect::<Vec<&str>>()[0] == "job"
    }

    /// Return Ok(true) when the data on disk is different than self
    /// Return Ok(false) when the data is the same
    /// Return Error when reading from disk failed
    pub fn changed_on_disk(&self) -> Result<bool, Box<Error>> {
        let datapath = self.paths.data.clone();
        let on_disk = &Self::from_datajson(datapath)?;
        Ok(self != on_disk)
    }

    /// Only write changes to data.json if there is a difference between the data
    /// stored on disk and self, Return Error if something failed, otherwise Ok()
    pub fn update_on_disk(&self) -> Result<(), Box<Error>>{
        let shouldupdate = self.changed_on_disk()?;
        if shouldupdate{
            self.write_to_file()?;
        }
        Ok(())
    }
}

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



/* ---------------------------[ JobTime ]--------------------------- */



/// JobTime is used by Job to timestamp different important timestamps throughout the life of a request
/// Times can be updated with `JobTime::create()`, `JobTime::finish()`, and `JobTime::error()`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JobTime {
    pub creation: Option<DateTime<Utc>>,
    pub finish: Option<DateTime<Utc>>,
    pub error: Option<DateTime<Utc>>
}



#[allow(dead_code)]
impl JobTime{

    pub fn new() -> Self{
        JobTime{ 
            creation: None, 
            finish: None, 
            error: None
        }
    }

    /// Save time for
    pub fn create(&mut self){
        match self.creation{
            Some(t) => println!("Tried to set time of creation, but there already was a time set: {}", t),
            None => self.creation = Some(Utc::now())
        }
    }

    /// Save time for
    pub fn finish(&mut self){
        match self.finish{
            Some(t) => println!("Tried to set time of finishing, but there already was a time set: {}", t),
            None => self.finish = Some(Utc::now())
        }
    }

    /// Save time for
    pub fn error(&mut self){
        match self.error{
            Some(t) => println!("Tried to set time of error, but there already was a time set: {}", t),
            None => self.error = Some(Utc::now())
        }
    }
}

// String formatting for JobTime
impl fmt::Display for JobTime {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let ctime = match self.creation{
            Some(t) => format!("{}", t),
            None => "- ".to_owned()
        };
        let ftime = match self.finish{
            Some(t) => format!("{}", t),
            None => "- ".to_owned()
        };
        let etime = match self.error{
            Some(t) => format!("{}", t),
            None => "- ".to_owned()
        };
        let st = &format!("[JobTime]\n  ├╴[creation: {}]\n  ├╴[finish: {}]\n  └╴[error: {}]\n", ctime, ftime, etime)[..];
        fmt.write_str(st)?;
        Ok(())
    }
}

/* ---------------------------[ JobPaths ]--------------------------- */

/// A JobPath Struct holds all path-related data for the Job
/// It can be created from a uploadfolder
/// ```
/// use bender_job::JobPaths;
/// let j = JobPaths::from_uploadfolder("/data/blendfiles/5873c0033e78b222bec2cb2a221487cf");
/// ```
/// or by deserializing a `data.json`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JobPaths{
    pub upload:    String,
    pub data:      String,
    pub blend:     String,
    pub frames:    String,
    pub filename:  String
}

impl JobPaths{

    /// You can create a JobPath via `JobPaths::from_uploadfolder`
    pub fn from_uploadfolder<S>(p: S) -> Self where S: Into<String>{
        // lets say we have a path called "/data/blendfiles/5873c0033e78b222bec2cb2a221487cf"
        let s = p.into();
        // Extract the id
        let id = PathBuf::from(&s);
        let id = id.file_name().expect("Error when aquiring id from path");
        // Create a path to "/data/blendfiles/5873c0033e78b222bec2cb2a221487cf/data.json"
        let mut data = PathBuf::from(&s);
        data.push("data.json");
        // Find a blendfile in the uploadfolder
        // e.g. "/data/blendfiles/5873c0033e78b222bec2cb2a221487cf/foo.blend"
        let blend = Self::first_blend(&s[..]).expect("Error: no blendfile in the directory");
        // Return frames folder at "/data/frames/5873c0033e78b222bec2cb2a221487cf"
        let mut frames = PathBuf::from(&s);
        frames.pop();
        frames.pop();
        frames.push("frames");
        frames.push(id);
        // Return filename of the blend
        let filename = blend.clone();
        let filename = filename.file_name().unwrap();

        JobPaths{
            upload: s.to_owned(),
            data: data.into_os_string().into_string().unwrap(),
            blend: blend.into_os_string().into_string().unwrap(),
            frames: frames.into_os_string().into_string().unwrap(),
            filename: filename.to_os_string().into_string().unwrap()
        }
    }

    /// Returns the ID used in the uploaddirectory by returning the last element of the upload path
    pub fn get_id(&self) -> String{
        let id = PathBuf::from(&self.upload[..]);
        id.file_name().unwrap().to_os_string().into_string().unwrap()
    }


    /// Returns a Vector of files with .blend extension found in a directory `p`
    pub fn find_blends<S>(p: S) -> Vec<PathBuf> where S: Into<String>{
        let path = &p.into()[..];
        let mut matches = Vec::new();
        // Search all files in path, push matches to vec
        for direntry in fs::read_dir(&path).expect(&format!("Couldn't read directory for {}", &path)[..]){
            let dirpath = direntry.unwrap().path();
            match dirpath.extension(){
                Some(e) => {
                    if e == "blend"{
                        matches.push(dirpath.clone());
                    }
                },
                None => ()
            }
        }
        matches
    }

    /// Return the first file with a .blend extension found in a directory `p`
    pub fn first_blend<S>(p: S) -> Option<PathBuf> where S: Into<String>{
        let mut matches = Self::find_blends(&p.into()[..]);
        if !matches.is_empty(){
            Some(matches.remove(0))
        } else {
            None
        }
    }

}

/// String formatting for JobPaths
impl fmt::Display for JobPaths {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let st = &format!("[JobPaths] 
├╴[upload:   \"{}\"]  
├╴[data:     \"{}\"]  
├╴[blend:    \"{}\"]  
├╴[frames:   \"{}\"]  
└╴[filename: \"{}\"]", 
            self.upload, self.data, self.blend, self.frames, self.filename)[..];
        fmt.write_str(st)?;
        Ok(())
    }
}





/* ---------------------------[ WHITEBOX TESTS ]------------------------------ */



#[cfg(test)]
mod tests {

}
