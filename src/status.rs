use ::*;
use std::fmt;

/// Describes the Lifecycle of a Request/Job. Set a Status
/// via it's methods:
/// ```
/// use bender_job::Status;
/// let mut s = Status::new();
/// s.error();
/// assert_eq!(s.is_errored(), true);
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status{
    Request(RequestStatus),
    Job(JobStatus)
}

/// Describes the States a Request can have
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RequestStatus{
    Untouched,
    Invalid,
    Errored,
    Checked,
    Scanned,
    Atomized
}

/// Describes the States a Job can have
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum JobStatus{
    Queued,
    Running,
    Canceled,
    Errored,
    Finished
}


impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Request(request_status) => {
                let s = format!("{:?}", request_status);
                write!(f, "request.{}", s.to_lowercase())
            },
            Status::Job(job_status) => {
                let s = format!("{:?}", job_status);
                write!(f, "job.{}", s.to_lowercase())
            }
        }
    }
}

impl Status{
    /// Returns a String describing the primary status (e.g. "Job" or "Request")
    pub fn format_primary(&self) -> String{
        match self{
            Status::Request(_) => "Request".to_string(),
            Status::Job(_) => "Job".to_string()
        }
    }

    /// Returns a String describing the secondary status (e.g. "untouched")
    pub fn format_secondary(&self) -> String{
        match self{
            Status::Request(secondary) => {
                let s = format!("{:?}", secondary).to_lowercase();
                s
            },
            Status::Job(secondary) => {
                let s = format!("{:?}", secondary).to_lowercase();
                s
            }
        }
    }
}

impl Default for Status {
    fn default() -> Status {
        Status::new()
    }
}



impl Status{
    pub fn new() -> Self{
        Status::Request(RequestStatus::Untouched)
    }

    // =============== Check General States ===============
    pub fn is_request(&self) -> bool{
        match self{
            Status::Request(_) => true,
            _ => false
        }
    }

    pub fn is_job(&self) -> bool{
        match self{
            Status::Job(_) => true,
            _ => false
        }
    }

    // =============== Check Sub States ===============
    pub fn is_untouched(&self) -> bool{
        match self{
            Status::Request(RequestStatus::Untouched) => true,
            _ => false
        }
    }

    pub fn is_invalid(&self) -> bool{
        match self{
            Status::Request(RequestStatus::Invalid) => true,
            _ => false
        }
    }

    pub fn is_errored(&self) -> bool{
        match self{
            Status::Request(RequestStatus::Errored) => true,
            Status::Job(JobStatus::Errored) => true,
            _ => false
        }
    }

    pub fn is_checked(&self) -> bool{
        match self{
            Status::Request(RequestStatus::Checked) => true,
            _ => false
        }
    }

    pub fn is_scanned(&self) -> bool{
        match self{
            Status::Request(RequestStatus::Scanned) => true,
            _ => false
        }
    }

    pub fn is_atomized(&self) -> bool{
        match self{
            Status::Request(RequestStatus::Atomized) => true,
            _ => false
        }
    }

    pub fn is_queued(&self) -> bool{
        match self{
            Status::Job(JobStatus::Queued) => true,
            _ => false
        }
    }

    pub fn is_running(&self) -> bool{
        match self{
            Status::Job(JobStatus::Running) => true,
            _ => false
        }
    }

    pub fn is_canceled(&self) -> bool{
        match self{
            Status::Job(JobStatus::Canceled) => true,
            _ => false
        }
    }

    pub fn is_finished(&self) -> bool{
        match self{
            Status::Job(JobStatus::Finished) => true,
            _ => false
        }
    }

    // =============== Check Meta States ===============
    pub fn is_validated(&self) -> bool{
        match self{
            Status::Request(RequestStatus::Checked) => true,
            Status::Request(RequestStatus::Scanned) => true,
            Status::Request(RequestStatus::Atomized) => true,
            Status::Job(_) => true,
            _ => false
        }
    }

    pub fn is_invalidated(&self) -> bool{
        !self.is_validated()
    }

    pub fn has_ended(&self) -> bool{
        match self{
            Status::Job(JobStatus::Canceled) => true,
            Status::Job(JobStatus::Errored) => true,
            Status::Job(JobStatus::Finished) => true,
            Status::Request(RequestStatus::Errored) => true,
            Status::Request(RequestStatus::Invalid) => true,
            _ => false
        }
    }

    pub fn is_alive(&self) -> bool{
        !self.has_ended()
    }


}




// =============== SET STATUS ===============
impl Status{
    /// Set to Errored only if self.is_alive()
    pub fn error(&mut self) -> GenResult<()>{
        if self.is_alive(){
            match self{
                Status::Request(_) => {
                    *self = Status::Request(RequestStatus::Errored);
                    Ok(())
                },
                Status::Job(_) => {
                    *self = Status::Job(JobStatus::Errored);
                    Ok(())
                }
            }
        }else{
            Err(From::from("Couldn't Status::error(): already canceled, errored or finished"))
        }
    }

    /// Set to Invalid only if self is a request that has not errored or hasn't
    /// been invalidated already
    pub fn deny(&mut self) -> GenResult<()>{
        match self{
            Status::Request(RequestStatus::Errored) => Err(From::from("Couldn't Status::deny(): already errored")),
            Status::Request(RequestStatus::Invalid) => Err(From::from("Couldn't Status::deny(): already denied")),
            Status::Request(_) => { *self = Status::Request(RequestStatus::Invalid); Ok(()) },
            _ => Err(From::from("Couldn't Status::deny(): is Job"))
        }
    }

    /// Set to checked only if self is a untouched request
    pub fn validate(&mut self) -> GenResult<()>{
        match self{
            Status::Request(RequestStatus::Untouched) => { *self = Status::Request(RequestStatus::Checked); Ok(()) },
            _ => Err(From::from("Couldn't Status::validate(): was not a untouched request"))
        }
    }

    /// Set to scanned only if self is a checked request
    pub fn scan(&mut self) -> GenResult<()>{
        match self{
            Status::Request(RequestStatus::Checked) => { *self = Status::Request(RequestStatus::Scanned); Ok(()) },
            _ => Err(From::from("Couldn't Status::validate(): was not a checked request"))
        }
    }

    /// Set to atomize only if self is a scanned request
    pub fn atomize(&mut self) -> GenResult<()>{
        match self{
            Status::Request(RequestStatus::Scanned) => { *self = Status::Request(RequestStatus::Atomized); Ok(()) },
            _ => Err(From::from("Couldn't Status::validate(): was not a scanned request"))
        }
    }

    /// Set to queued only if self is a atomized request
    pub fn queue(&mut self) -> GenResult<()>{
        match self{
            Status::Request(RequestStatus::Atomized) => { *self = Status::Job(JobStatus::Queued); Ok(()) },
            _ => Err(From::from("Couldn't Status::queue(): was not a atomized request"))
        }
    }

    /// Set to running only if self is a queued job
    pub fn run(&mut self) -> GenResult<()>{
        match self{
            Status::Job(JobStatus::Queued) => { *self = Status::Job(JobStatus::Running); Ok(()) },
            _ => Err(From::from("Couldn't Status::run(): was not a queued job"))
        }
    }

    /// Set to finished only if self is a running job
    pub fn finish(&mut self) -> GenResult<()>{
        match self{
            Status::Job(JobStatus::Running) => { *self = Status::Job(JobStatus::Finished); Ok(()) },
            _ => Err(From::from("Couldn't Status::finish(): was not a running job"))
        }
    }

    /// Set to canceled only if self is a queued or running job
    pub fn cancel(&mut self) -> GenResult<()>{
        match self{
            Status::Job(JobStatus::Queued) => { *self = Status::Job(JobStatus::Canceled); Ok(()) },
            Status::Job(JobStatus::Running) => { *self = Status::Job(JobStatus::Canceled); Ok(()) },
            _ => Err(From::from("Couldn't Status::cancel(): was not a queued or running request"))
        }
    }

    pub fn reset(&mut self){
        *self = Status::Request(RequestStatus::Untouched)
    }
}