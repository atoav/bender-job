//! The jobtime module defines the JobTime Struct that holds timing information \
//! for the job. It defines convenience functions for storing and measuring age.
//!
//! It also defines serialization and deserialization.

use ::*;
use chrono::Duration;


/// JobTime is used by Job to timestamp different important timestamps throughout the life of a request
/// Times can be updated with `JobTime::create()`, `JobTime::finish()`, and `JobTime::error()`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JobTime {
    pub creation: Option<DateTime<Utc>>,
    pub start: Option<DateTime<Utc>>,
    pub finish: Option<DateTime<Utc>>,
    pub error: Option<DateTime<Utc>>,
    pub abort: Option<DateTime<Utc>>,
    pub pause: Option<DateTime<Utc>>
}



#[allow(dead_code)]
impl JobTime{

    pub fn new() -> Self{
        JobTime{ 
            creation: Some(Utc::now()), 
            start: None,
            finish: None, 
            error: None,
            abort: None,
            pause: None
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
    pub fn start(&mut self){
        match self.creation{
            Some(t) => println!("Tried to set time of start, but there already was a time set: {}", t),
            None => self.start = Some(Utc::now())
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

    /// Save time for
    pub fn abort(&mut self){
        match self.abort{
            Some(t) => println!("Tried to set time of abortion, but there already was a time set: {}", t),
            None => self.abort = Some(Utc::now())
        }
    }

    /// Save time for
    pub fn pause(&mut self){
        match self.pause{
            Some(t) => println!("Tried to set time of pause, but there already was a time set: {}", t),
            None => self.pause = Some(Utc::now())
        }
    }


    /// Return the age of self as a chrono duration
    pub fn age(&self) -> Duration{
        let now = Utc::now();
        now - self.creation.unwrap()
    }

    /// Return the age of self in seconds
    pub fn age_seconds(&self) -> usize{
        self.age().num_seconds() as usize
    }
}

// String formatting for JobTime
impl fmt::Display for JobTime {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let ctime = match self.creation{
            Some(t) => format!("{}", t),
            None => "- ".to_owned()
        };
        let stime = match self.start{
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
        let atime = match self.abort{
            Some(t) => format!("{}", t),
            None => "- ".to_owned()
        };
        let ptime = match self.pause{
            Some(t) => format!("{}", t),
            None => "- ".to_owned()
        };
        let st = &format!("[JobTime]\n  ├╴[creation: {}]\n  ├╴[start: {}]\n  ├╴[finish: {}]\n  ├╴[error: {}]\n  ├╴[abort: {}]\n  └╴[pause: {}]\n", ctime, stime, ftime, etime, atime, ptime)[..];
        fmt.write_str(st)?;
        Ok(())
    }
}