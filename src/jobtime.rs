//! The jobtime module defines the JobTime Struct that holds timing information \
//! for the job. It defines convenience functions for storing and measuring age.
//!
//! It also defines serialization and deserialization.

use ::*;
use chrono::Duration;
use chrono_humanize::{Accuracy, HumanTime, Tense};




// ===========================================================================
//                           ASSOCIATED FUNCTIONS
// ===========================================================================


/// Associated function that handles the logic of mergin one Option<DateTime<Utc>>
/// into another
fn merge_date(this: Option<DateTime<Utc>>, that: Option<DateTime<Utc>>) -> Option<DateTime<Utc>>{
    let dates = (this, that);
    // Return the date that has a value or is older
    match dates{
        (None, None)       => None,
        (Some(a), None)    => Some(a),
        (None, Some(b))    => Some(b),
        (Some(a), Some(b)) => {
            if a < b {
                Some(a)
            }else{
                Some(b)
            }
        }
    }
}




// ===========================================================================
//                                JobTime
// ===========================================================================

/// JobTime is used by Job to timestamp different important timestamps throughout the life of a request
/// Times can be updated with `JobTime::create()`, `JobTime::finish()`, and `JobTime::error()`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JobTime {
    pub creation: Option<DateTime<Utc>>,
    pub queued: Option<DateTime<Utc>>,
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
            queued: None,
            start: None,
            finish: None, 
            error: None,
            abort: None,
            pause: None
        }
    }

    /// Returns a fixed time for testing
    pub fn new_deterministic_for_test() -> Self{
        JobTime{ 
            creation: Some(Utc.ymd(2018, 8, 23)
                .and_hms_micro(13, 48, 40, 176_598)), 
            queued: None,
            start: None,
            finish: None, 
            error: None,
            abort: None,
            pause: None
        }
    }

    /// Allow the merging of one JobTime into another
    pub fn merge(&mut self, other: &Self){
        self.creation = merge_date(self.creation, other.creation);
        self.queued   = merge_date(self.queued, other.queued);
        self.start    = merge_date(self.start, other.start);
        self.finish   = merge_date(self.finish, other.finish);
        self.error    = merge_date(self.error, other.error);
        self.abort    = merge_date(self.abort, other.abort);
        self.pause    = merge_date(self.pause, other.pause);
    }

    /// Save time for
    pub fn create(&mut self){
        match self.creation{
            Some(t) => println!("Tried to set time of creation, but there already was a time set: {}", t),
            None => self.creation = Some(Utc::now())
        }
    }

    /// Save time for
    pub fn queue(&mut self){
        match self.queued{
            Some(t) => println!("Tried to set time of queue, but there already was a time set: {}", t),
            None => self.queued = Some(Utc::now())
        }
    }

    /// Save time for
    pub fn start(&mut self){
        match self.start{
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

    //  ------------------------------- AGE ---------------------------------

    /// Return the age (duration since creation) of Job as a chrono duration
    pub fn age(&self) -> Duration{
        let now = Utc::now();
        now - self.creation.unwrap()
    }

    /// Return the age (duration since creation) of Job in seconds
    pub fn age_seconds(&self) -> usize{
        self.age().num_seconds() as usize
    }

    /// Return the Jobs age (duration since creation) in rough human time
    pub fn age_human(&self) -> String{
        let ht = HumanTime::from(self.age());
        ht.to_text_en(Accuracy::Rough, Tense::Present)
    }

    /// Return the Jobs age (duration since creation) in precise human time
    pub fn age_human_precise(&self) -> String{
        let ht = HumanTime::from(self.age());
        ht.to_text_en(Accuracy::Precise, Tense::Present)
    }

    //  -------------------------- DURATION ----------------------------------

    /// Return the duration (duration since start) of Job as a chrono duration
    pub fn duration(&self) -> Option<Duration>{
        match self.start{
            Some(t) =>{
                // Use the finish time if the task finished, otherwise use now
                let end = match self.finish{
                    None => Utc::now(),
                    Some(end) => end
                };
                Some(end - t)
            },
            None => None
        }
        
    }

    /// Return the duration (duration since start) of Job in seconds
    pub fn duration_seconds(&self) -> Option<usize>{
        match self.duration(){
            Some(d) => Some(d.num_seconds() as usize),
            None => None
        }
    }

    /// Return the Jobs duration (duration since start) in rough human time
    pub fn duration_human(&self) -> String{
        match self.duration(){
            Some(d) => {
                let ht = HumanTime::from(d);
                ht.to_text_en(Accuracy::Rough, Tense::Present)
            },
            None => "Not started".to_string()
        }
    }

    /// Return the Jobs duration (duration since start) in precise human time
    pub fn duration_human_precise(&self) -> String{
        match self.duration(){
            Some(d) => {
                let ht = HumanTime::from(d);
                ht.to_text_en(Accuracy::Precise, Tense::Present)
            },
            None => "Not started".to_string()
        }
    }

    //  ------------------------- TIME WAITING ----------------------------

    /// Return the duration (duration since queued) of Job as a chrono duration
    pub fn waiting_for(&self) -> Option<Duration>{
        match self.start{
            Some(t) =>{
                // Use the stat time if the task started, otherwise use now
                let end = match self.start{
                    None => Utc::now(),
                    Some(end) => end
                };
                Some(end - t)
            },
            None => None
        }
        
    }

    /// Return the duration the job has been sitting in the qu Job in seconds
    pub fn waiting_for_seconds(&self) -> Option<usize>{
        match self.waiting_for(){
            Some(d) => Some(d.num_seconds() as usize),
            None => None
        }
    }

    /// Return the Jobs duration the job has been sitting in the qu rough human time
    pub fn waiting_for_human(&self) -> String{
        match self.waiting_for(){
            Some(d) => {
                let ht = HumanTime::from(d);
                ht.to_text_en(Accuracy::Rough, Tense::Present)
            },
            None => "Not started".to_string()
        }
    }

    /// Return the Jobs duration the job has been sitting in the qu precise human time
    pub fn waiting_for_human_precise(&self) -> String{
        match self.waiting_for(){
            Some(d) => {
                let ht = HumanTime::from(d);
                ht.to_text_en(Accuracy::Precise, Tense::Present)
            },
            None => "Not started".to_string()
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