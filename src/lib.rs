#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate chrono;







use chrono::prelude::*;
use chrono::Utc;
use std::collections::HashMap;
use serde_json::Error as SerdeJsonError;
use std::str;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::error::Error;


/* --------------------------------[ Job ]-------------------------------- */

/// The Job struct holds all information about a job request for rendering
/// it gets created simply by reading from its `data.json`.
/// Ways to create a request are:
///
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Job {
    pub id: String,
    pub paths: JobPaths,
    pub email: String,
    pub times: JobTimes,
    pub status: String,
    pub data: HashMap<String, String>,
    pub history: HashMap<DateTime<Utc>, String>
}

#[allow(dead_code)]
impl Job{
    /// Add to the history of a Job
    pub fn add_history<S>(&mut self, text: S) where S: Into<String> {
        self.history.insert(Utc::now(), text.into());
    }

    /// Append a key-value-pair to the data of a Job
    /// e.g. `Job::append("watchdog.queueposition", "22")`
    pub fn add_data<S>(&mut self, key: S, value: S) where S: Into<String> {
        self.data.insert(key.into(), value.into());
    }

    /// Serialize a Job into a String. Return a Error if this fails
    pub fn serialize(&self) -> Result<String, Box<Error>> {
        let string = serde_json::to_string(&self)?;
        Ok(string)
    }

    /// Serialize a Job into a Vec<u8>. Return a Error if this fails
    /// you might want to use this with a reference
    pub fn serialize_to_u8(&self) -> Result<Vec<u8>, Box<Error>> {
        let string = serde_json::to_string(&self)?;
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
    /// **Warning:** _This must only be used within ONE service!_
    pub fn write_to_file(&self) -> Result<(), Box<Error>> {
        // Step 1: Serialize
        let serialized = self.serialize()?;
        // Step 2: Write
        fs::write(&self.paths.data, serialized)?;
        Ok(())
    }

    // TODO: Write function to check if the job changed in file
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

// TODO: This is very unsafe. better write something that fails gracefully!
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



/* ---------------------------[ JobTimes ]--------------------------- */



/// JobTimes is used by Job to timestamp different important timestamps throughout the life of a request
/// Times can be updated with `JobTimes::create()`, `JobTimes::finish()`, and `JobTimes::error()`
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct JobTimes {
    pub creationtime: Option<DateTime<Utc>>,
    pub finishtime: Option<DateTime<Utc>>,
    pub errortime: Option<DateTime<Utc>>
}



#[allow(dead_code)]
impl JobTimes{

    pub fn new() -> Self{
        JobTimes{ 
            creationtime: None, 
            finishtime: None, 
            errortime: None
        }
    }

    /// Save time for
    pub fn create(&mut self){
        match self.creationtime{
            Some(t) => println!("Tried to set time of creation, but there already was a time set: {}", t),
            None => self.creationtime = Some(Utc::now())
        }
    }

    /// Save time for
    pub fn finish(&mut self){
        match self.finishtime{
            Some(t) => println!("Tried to set time of finishing, but there already was a time set: {}", t),
            None => self.finishtime = Some(Utc::now())
        }
    }

    /// Save time for
    pub fn error(&mut self){
        match self.errortime{
            Some(t) => println!("Tried to set time of error, but there already was a time set: {}", t),
            None => self.errortime = Some(Utc::now())
        }
    }
}

// String formatting for JobTimes
impl fmt::Display for JobTimes {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let ctime = match self.creationtime{
            Some(t) => format!("{}", t),
            None => "- ".to_owned()
        };
        let ftime = match self.finishtime{
            Some(t) => format!("{}", t),
            None => "- ".to_owned()
        };
        let etime = match self.errortime{
            Some(t) => format!("{}", t),
            None => "- ".to_owned()
        };
        let st = &format!("[JobTimes]\n  ├╴[creationtime: {}]\n  ├╴[finishtime: {}]\n  └╴[errortime: {}]\n", ctime, ftime, etime)[..];
        fmt.write_str(st)?;
        Ok(())
    }
}

/* ---------------------------[ JobPaths ]--------------------------- */

/// A JobPath Struct holds all path-related data for the Job
/// It can be created from a uploadfolder
/// ```
/// use job::JobPaths;
/// let j = JobPaths::from_uploadfolder("/data/blendfiles/1be554e1f51b804637326e3faf41d2c9");
/// ```
/// or by deserializing a `data.json`
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
        // lets say we have a path called "/data/blendfiles/1be554e1f51b804637326e3faf41d2c9"
        let s = p.into();
        // Extract the id
        let id = PathBuf::from(&s);
        let id = id.file_name().unwrap();
        // Create a path to "/data/blendfiles/1be554e1f51b804637326e3faf41d2c9/data.json"
        let mut data = PathBuf::from(&s);
        data.push("data.json");
        // Find a blendfile in the uploadfolder
        // e.g. "/data/blendfiles/1be554e1f51b804637326e3faf41d2c9/foo.blend"
        let blend = Self::first_blend(&s[..]).unwrap();
        // Return frames folder at "/data/frames/1be554e1f51b804637326e3faf41d2c9"
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
