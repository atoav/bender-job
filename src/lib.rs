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

use chrono::prelude::*;
use chrono::Utc;
use std::collections::{HashMap, BTreeMap};
use std::str;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::error::Error;

pub mod jobtime;
pub use jobtime::JobTime;

pub mod jobpaths;
pub use jobpaths::JobPaths;

pub mod task;
pub use task::Task;

pub mod status;
pub use status::{Status};



type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;





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
    pub animation: bool,
    pub paths: JobPaths,
    pub email: String,
    pub time: JobTime,
    pub status: Status,
    pub data: HashMap<String, String>,
    pub history: BTreeMap<DateTime<Utc>, String>,
    #[serde(default)]
    pub resolution: Resolution,
    #[serde(default)]
    pub render: Render,
    #[serde(default)]
    pub frames: Frames
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Render{
    pub renderer: String,
    pub cuda: bool,
    pub device: String,
    pub image_format: String,
    pub uses_compositing: bool,
    pub fps: usize
}

impl Render{
    /// Check if the Format is valid
    pub fn valid_format(&self) -> bool{
        let valid_formats: [&str; 12] = ["PNG", "BMP", "JPEG", "JPEG2000", "TARGA", "TARGA_RAW", "CINEON", "DPX", "OPEN_EXR_MULTILAYER", "OPEN_EXR", "HDR", "TIFF"];
        valid_formats.contains(&self.image_format.as_str())
    }

    /// Return true if self has still the default value
    pub fn is_default(&self) -> bool{
        self == &Self::default()
    }
}

/// Represents the Frames section of the MiscInfo Struct. This is important for
/// generating the commands with the generate_commands function
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Frames {
    pub start: usize,
    pub end: usize,
    pub current: usize,
    pub step: usize
}

impl Frames {
    /// Return the number of frames in total. This honors the step size specified in the blend
    pub fn count(&self) -> usize {
        if self.is_default(){
            0
        }else{
            self.as_vec().len()
        }
    }

    // Return a Vec of frame numbers. This honors the step size specified in the blend
    pub fn as_vec(&self) -> Vec<usize> {
        (self.start..self.end+1).step_by(self.step).collect()
    }

    /// Return true if self has still the default value
    pub fn is_default(&self) -> bool{
        self == &Self::default()
    }
}


/// Stores the Resolution data
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Resolution {
    pub x: usize,
    pub y: usize,
    pub scale: usize
}

impl Resolution {
    /// Returned the scaled (actual) width of the render output
    pub fn scaled_x(&self) -> usize {
        (self.x * self.scale)/100
    }

    /// Returned the scaled (actual) height of the render output
    pub fn scaled_y(&self) -> usize {
        (self.y * self.scale)/100
    }

    /// return the total number of pixels
    pub fn pixels(&self) -> i64{
        self.scaled_x() as i64 * self.scaled_y() as i64
    }

    /// Return true if self has still the default value
    pub fn is_default(&self) -> bool{
        self == &Self::default()
    }
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

    /// Read a ID directly from the existing uploadfolder
    pub fn id(&self) -> String{
        self.paths.get_id()
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

    /// Check if self is a request
    pub fn is_request(&self) -> bool{
        self.status.is_request()
    }

    /// Check if self is a invalid request
    pub fn is_invalid(&self) -> bool{
        self.status.is_invalid()
    }

    /// Check if self has been validated
    pub fn is_valid(&self) -> bool{
        self.status.is_invalid()
    }   

    /// Check if self is a job
    pub fn is_job(&self) -> bool{
        self.status.is_job()
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









#[cfg(test)]
mod render {
    use ::*;
    #[test]
    fn is_default() {
        let r = Render::default();
        assert_eq!(r.is_default(), true);
    }

    #[test]
    fn is_not_default() {
        let mut r = Render::default();
        r.renderer = "CYCLES".to_string();
        assert_eq!(r.is_default(), false);
    }

    #[test]
    fn format_is_valid() {
        let mut r = Render::default();
        r.image_format = "PNG".to_string();
        assert_eq!(r.valid_format(), true);
    }

    #[test]
    fn format_is_invalid() {
        let mut r = Render::default();
        r.image_format = "FOOOO".to_string();
        assert_eq!(r.valid_format(), false);
    }
}

#[cfg(test)]
mod frames {
    use ::*;
    #[test]
    fn is_default() {
        let f = Frames::default();
        assert_eq!(f.is_default(), true);
    }

    #[test]
    fn is_not_default() {
        let mut f = Frames::default();
        f.end = 100;
        assert_eq!(f.is_default(), false);
    }

    #[test]
    fn basic_count() {
        let f = Frames{
            start: 1,
            end: 100,
            current: 2,
            step: 1
        };
        let v = f.as_vec();
        println!("{:?}", v);
        assert_eq!(f.count(), 100);
    }

    #[test]
    fn stepped_count() {
        let f = Frames{
            start: 1,
            end: 100,
            current: 2,
            step: 10
        };
        assert_eq!(f.count(), 10);
    }

    #[test]
    fn as_vec_length() {
        let f = Frames{
            start: 0,
            end: 100,
            current: 2,
            step: 10
        };
        let v = f.as_vec();
        println!("{:?}", v);
        assert_eq!(v.len(), 11);
    }

    #[test]
    fn as_vec_steps() {
        let f = Frames{
            start: 0,
            end: 100,
            current: 2,
            step: 10
        };
        let v1 = f.as_vec();
        let v2 = vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        assert_eq!(v1, v2);
    }

}


#[cfg(test)]
mod resolution {
    use ::*;
    #[test]
    fn is_default() {
        let r = Resolution::default();
        assert_eq!(r.is_default(), true);
    }

    #[test]
    fn is_not_default() {
        let mut r = Resolution::default();
        r.x = 1920;
        assert_eq!(r.is_default(), false);
    }

    #[test]
    fn scaled_resolution() {
        let r = Resolution {
            x: 2000,
            y: 1000,
            scale: 50
        };
        assert_eq!(r.scaled_x(), 1000);
        assert_eq!(r.scaled_y(), 500);
    }

    #[test]
    fn pixels() {
        let r = Resolution {
            x: 100,
            y: 100,
            scale: 100
        };
        assert_eq!(r.pixels(), 10000);
    }
}



