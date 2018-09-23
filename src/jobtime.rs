use ::*;


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