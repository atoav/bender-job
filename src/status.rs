//! The status module defines the Status Struct that describes both the current \
//! position of a job within its lifecycle and the valid transitions between them.

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


impl fmt::Display for RequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request ({:?})", self)
    }
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
                format!("{:?}", secondary).to_lowercase()
            },
            Status::Job(secondary) => {
                format!("{:?}", secondary).to_lowercase()
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

    /// Merge one Status with another
    pub fn merge(&mut self, other: &Self){
        let should_merge = match self{
            Status::Request(request_status) =>{
                match request_status{
                    RequestStatus::Untouched    => true,
                    RequestStatus::Invalid      => false,
                    RequestStatus::Errored      => false,
                    RequestStatus::Checked      => {
                        match other{
                            Status::Request(RequestStatus::Scanned)  => true,
                            Status::Request(RequestStatus::Atomized) => true,
                            Status::Request(RequestStatus::Errored)  => true,
                            Status::Job(_)               => true,
                            _ => false                        
                        }
                    },
                    RequestStatus::Scanned      => {
                        match other{
                            Status::Request(RequestStatus::Atomized) => true,
                            Status::Request(RequestStatus::Errored)  => true,
                            Status::Job(_)               => true,
                            _ => false                        
                        }
                    },
                    RequestStatus::Atomized     => {
                        match other{
                            Status::Request(RequestStatus::Errored) => true,
                            Status::Job(_) => true,
                            _ => false                        
                        }
                    }
                }
            },
            Status::Job(job_status)         => {
                match job_status{
                    JobStatus::Queued           => {
                        match other{
                            Status::Job(_) => true,
                            _ => false                        
                        }
                    },
                    JobStatus::Running          => {
                        match other{
                            Status::Job(JobStatus::Queued) => false,
                            Status::Job(_) => true,
                            _ => false                        
                        }

                    },
                    JobStatus::Canceled         => false,
                    JobStatus::Errored          => false,
                    JobStatus::Finished         => false,
                }
            }
        };

        if should_merge{
            *self = other.clone();
        }
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
            Status::Request(RequestStatus::Checked) => Ok(()),
            _ => Err(From::from("Couldn't Status::validate(): was not a untouched request"))
        }
    }

    /// Set to scanned only if self is a checked request
    pub fn scan(&mut self) -> GenResult<()>{
        match self{
            Status::Request(RequestStatus::Checked) => { *self = Status::Request(RequestStatus::Scanned); Ok(()) },
            Status::Request(RequestStatus::Scanned) => Ok(()),
            _ => Err(From::from("Couldn't Status::validate(): was not a checked request"))
        }
    }

    /// Set to atomize only if self is a scanned request
    pub fn atomize(&mut self) -> GenResult<()>{
        match self{
            Status::Request(RequestStatus::Scanned) => { *self = Status::Request(RequestStatus::Atomized); Ok(()) },
            Status::Request(RequestStatus::Atomized) => Ok(()),
            _ => Err(From::from("Couldn't Status::validate(): was not a scanned request"))
        }
    }

    /// Set to queued only if self is a atomized request
    pub fn queue(&mut self) -> GenResult<()>{
        match self{
            Status::Request(RequestStatus::Atomized) => { *self = Status::Job(JobStatus::Queued); Ok(()) },
            Status::Job(JobStatus::Queued) => Ok(()),
            _ => Err(From::from("Couldn't Status::queue(): was not a atomized request"))
        }
    }

    /// Set to running only if self is a queued job
    pub fn run(&mut self) -> GenResult<()>{
        match self{
            Status::Job(JobStatus::Queued) => { *self = Status::Job(JobStatus::Running); Ok(()) },
            Status::Job(JobStatus::Running) => Ok(()),
            _ => Err(From::from("Couldn't Status::run(): was not a queued job"))
        }
    }

    /// Set to finished only if self is a running job
    pub fn finish(&mut self) -> GenResult<()>{
        match self{
            Status::Job(JobStatus::Running) => { *self = Status::Job(JobStatus::Finished); Ok(()) },
            Status::Job(JobStatus::Queued) => { *self = Status::Job(JobStatus::Finished); Ok(()) },
            Status::Job(JobStatus::Finished) => Ok(()),
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







// /// Describes the States a Request can have
// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// pub enum RequestStatus{
//     Untouched,
//     Invalid,
//     Errored,
//     Checked,
//     Scanned,
//     Atomized
// }

// /// Describes the States a Job can have
// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// pub enum JobStatus{
//     Queued,
//     Running,
//     Canceled,
//     Errored,
//     Finished
// }


#[cfg(test)]
mod tests {
    use super::*;

    // --------------------- Merge Request Untouched --------------------------
    #[test]
    fn merge_request_untouched_with_invalid() {
        let mut old = Status::Request(RequestStatus::Untouched);
        let new = Status::Request(RequestStatus::Invalid);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_untouched_with_errored() {
        let mut old = Status::Request(RequestStatus::Untouched);
        let new = Status::Request(RequestStatus::Errored);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_untouched_with_checked() {
        let mut old = Status::Request(RequestStatus::Untouched);
        let new = Status::Request(RequestStatus::Checked);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_untouched_with_scanned() {
        let mut old = Status::Request(RequestStatus::Untouched);
        let new = Status::Request(RequestStatus::Scanned);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_untouched_with_atomized() {
        let mut old = Status::Request(RequestStatus::Untouched);
        let new = Status::Request(RequestStatus::Atomized);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_untouched_with_jobqueued() {
        let mut old = Status::Request(RequestStatus::Untouched);
        let new = Status::Job(JobStatus::Queued);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_untouched_with_jobrunning() {
        let mut old = Status::Request(RequestStatus::Untouched);
        let new = Status::Job(JobStatus::Running);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_untouched_with_jobcanceled() {
        let mut old = Status::Request(RequestStatus::Untouched);
        let new = Status::Job(JobStatus::Canceled);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_untouched_with_joberrored() {
        let mut old = Status::Request(RequestStatus::Untouched);
        let new = Status::Job(JobStatus::Errored);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_untouched_with_jobfinished() {
        let mut old = Status::Request(RequestStatus::Untouched);
        let new = Status::Job(JobStatus::Finished);
        old.merge(&new);
        assert_eq!(old, new);
    }

    // --------------------- Merge Request Invalid ----------------------------
    // Should always stay invalid

    #[test]
    fn merge_request_invalid_with_untouched() {
        let mut old = Status::Request(RequestStatus::Invalid);
        let new = Status::Request(RequestStatus::Untouched);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Invalid));
    }

    #[test]
    fn merge_request_invalid_with_errored() {
        let mut old = Status::Request(RequestStatus::Invalid);
        let new = Status::Request(RequestStatus::Errored);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Invalid));
    }

    #[test]
    fn merge_request_invalid_with_checked() {
        let mut old = Status::Request(RequestStatus::Invalid);
        let new = Status::Request(RequestStatus::Checked);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Invalid));
    }

    #[test]
    fn merge_request_invalid_with_scanned() {
        let mut old = Status::Request(RequestStatus::Invalid);
        let new = Status::Request(RequestStatus::Scanned);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Invalid));
    }

    #[test]
    fn merge_request_invalid_with_atomized() {
        let mut old = Status::Request(RequestStatus::Invalid);
        let new = Status::Request(RequestStatus::Atomized);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Invalid));
    }

    #[test]
    fn merge_request_invalid_with_jobqueued() {
        let mut old = Status::Request(RequestStatus::Invalid);
        let new = Status::Job(JobStatus::Queued);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Invalid));
    }

    #[test]
    fn merge_request_invalid_with_jobrunning() {
        let mut old = Status::Request(RequestStatus::Invalid);
        let new = Status::Job(JobStatus::Running);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Invalid));
    }

    #[test]
    fn merge_request_invalid_with_jobcanceled() {
        let mut old = Status::Request(RequestStatus::Invalid);
        let new = Status::Job(JobStatus::Canceled);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Invalid));
    }

    #[test]
    fn merge_request_invalid_with_joberrored() {
        let mut old = Status::Request(RequestStatus::Invalid);
        let new = Status::Job(JobStatus::Errored);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Invalid));
    }

    #[test]
    fn merge_request_invalid_with_jobfinished() {
        let mut old = Status::Request(RequestStatus::Invalid);
        let new = Status::Job(JobStatus::Finished);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Invalid));
    }

    // --------------------- Merge Request Errored ----------------------------
    // Should always stay errored

    #[test]
    fn merge_request_errored_with_untouched() {
        let mut old = Status::Request(RequestStatus::Errored);
        let new = Status::Request(RequestStatus::Untouched);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Errored));
    }

    #[test]
    fn merge_request_errored_with_invalid() {
        let mut old = Status::Request(RequestStatus::Errored);
        let new = Status::Request(RequestStatus::Invalid);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Errored));
    }

    #[test]
    fn merge_request_errored_with_checked() {
        let mut old = Status::Request(RequestStatus::Errored);
        let new = Status::Request(RequestStatus::Checked);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Errored));
    }

    #[test]
    fn merge_request_errored_with_scanned() {
        let mut old = Status::Request(RequestStatus::Errored);
        let new = Status::Request(RequestStatus::Scanned);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Errored));
    }

    #[test]
    fn merge_request_errored_with_atomized() {
        let mut old = Status::Request(RequestStatus::Errored);
        let new = Status::Request(RequestStatus::Atomized);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Errored));
    }

    #[test]
    fn merge_request_errored_with_jobqueued() {
        let mut old = Status::Request(RequestStatus::Errored);
        let new = Status::Job(JobStatus::Queued);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Errored));
    }

    #[test]
    fn merge_request_errored_with_jobrunning() {
        let mut old = Status::Request(RequestStatus::Errored);
        let new = Status::Job(JobStatus::Running);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Errored));
    }

    #[test]
    fn merge_request_errored_with_jobcanceled() {
        let mut old = Status::Request(RequestStatus::Errored);
        let new = Status::Job(JobStatus::Canceled);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Errored));
    }

    #[test]
    fn merge_request_errored_with_joberrored() {
        let mut old = Status::Request(RequestStatus::Errored);
        let new = Status::Job(JobStatus::Errored);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Errored));
    }

    #[test]
    fn merge_request_errored_with_jobfinished() {
        let mut old = Status::Request(RequestStatus::Errored);
        let new = Status::Job(JobStatus::Finished);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Errored));
    }


    // --------------------- Merge Request Checked ----------------------------
    // Should update on all stati that are more advanced

    #[test]
    fn merge_request_checked_with_untouched() {
        let mut old = Status::Request(RequestStatus::Checked);
        let new = Status::Request(RequestStatus::Untouched);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Checked));
    }

    #[test]
    fn merge_request_checked_with_invalid() {
        let mut old = Status::Request(RequestStatus::Checked);
        let new = Status::Request(RequestStatus::Invalid);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Checked));
    }

    #[test]
    fn merge_request_checked_with_errored() {
        let mut old = Status::Request(RequestStatus::Checked);
        let new = Status::Request(RequestStatus::Errored);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Errored));
    }

    #[test]
    fn merge_request_checked_with_scanned() {
        let mut old = Status::Request(RequestStatus::Checked);
        let new = Status::Request(RequestStatus::Scanned);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Scanned));
    }

    #[test]
    fn merge_request_checked_with_atomized() {
        let mut old = Status::Request(RequestStatus::Checked);
        let new = Status::Request(RequestStatus::Atomized);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Atomized));
    }

    #[test]
    fn merge_request_checked_with_jobqueued() {
        let mut old = Status::Request(RequestStatus::Checked);
        let new = Status::Job(JobStatus::Queued);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_checked_with_jobrunning() {
        let mut old = Status::Request(RequestStatus::Checked);
        let new = Status::Job(JobStatus::Running);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_checked_with_jobcanceled() {
        let mut old = Status::Request(RequestStatus::Checked);
        let new = Status::Job(JobStatus::Canceled);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_checked_with_joberrored() {
        let mut old = Status::Request(RequestStatus::Checked);
        let new = Status::Job(JobStatus::Errored);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_checked_with_jobfinished() {
        let mut old = Status::Request(RequestStatus::Checked);
        let new = Status::Job(JobStatus::Finished);
        old.merge(&new);
        assert_eq!(old, new);
    }

    // --------------------- Merge Request Scanned ----------------------------
    // Should update on all stati that are more advanced

    #[test]
    fn merge_request_scanned_with_untouched() {
        let mut old = Status::Request(RequestStatus::Scanned);
        let new = Status::Request(RequestStatus::Untouched);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Scanned));
    }

    #[test]
    fn merge_request_scanned_with_invalid() {
        let mut old = Status::Request(RequestStatus::Scanned);
        let new = Status::Request(RequestStatus::Invalid);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Scanned));
    }

    #[test]
    fn merge_request_scanned_with_errored() {
        let mut old = Status::Request(RequestStatus::Scanned);
        let new = Status::Request(RequestStatus::Errored);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_scanned_with_checked() {
        let mut old = Status::Request(RequestStatus::Scanned);
        let new = Status::Request(RequestStatus::Checked);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Scanned));
    }

    #[test]
    fn merge_request_scanned_with_atomized() {
        let mut old = Status::Request(RequestStatus::Scanned);
        let new = Status::Request(RequestStatus::Atomized);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Atomized));
    }

    #[test]
    fn merge_request_scanned_with_jobqueued() {
        let mut old = Status::Request(RequestStatus::Scanned);
        let new = Status::Job(JobStatus::Queued);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_scanned_with_jobrunning() {
        let mut old = Status::Request(RequestStatus::Scanned);
        let new = Status::Job(JobStatus::Running);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_scanned_with_jobcanceled() {
        let mut old = Status::Request(RequestStatus::Scanned);
        let new = Status::Job(JobStatus::Canceled);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_scanned_with_joberrored() {
        let mut old = Status::Request(RequestStatus::Scanned);
        let new = Status::Job(JobStatus::Errored);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_scanned_with_jobfinished() {
        let mut old = Status::Request(RequestStatus::Scanned);
        let new = Status::Job(JobStatus::Finished);
        old.merge(&new);
        assert_eq!(old, new);
    }

    // --------------------- Merge Request Atomized ---------------------------
    // Should update on all stati that are more advanced

    #[test]
    fn merge_request_atomized_with_untouched() {
        let mut old = Status::Request(RequestStatus::Atomized);
        let new = Status::Request(RequestStatus::Untouched);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Atomized));
    }

    #[test]
    fn merge_request_atomized_with_invalid() {
        let mut old = Status::Request(RequestStatus::Atomized);
        let new = Status::Request(RequestStatus::Invalid);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Atomized));
    }

    #[test]
    fn merge_request_atomized_with_errored() {
        let mut old = Status::Request(RequestStatus::Atomized);
        let new = Status::Request(RequestStatus::Errored);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_atomized_with_checked() {
        let mut old = Status::Request(RequestStatus::Atomized);
        let new = Status::Request(RequestStatus::Checked);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Atomized));
    }

    #[test]
    fn merge_request_atomized_with_scanned() {
        let mut old = Status::Request(RequestStatus::Atomized);
        let new = Status::Request(RequestStatus::Scanned);
        old.merge(&new);
        assert_eq!(old, Status::Request(RequestStatus::Atomized));
    }

    #[test]
    fn merge_request_atomized_with_jobqueued() {
        let mut old = Status::Request(RequestStatus::Atomized);
        let new = Status::Job(JobStatus::Queued);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_atomized_with_jobrunning() {
        let mut old = Status::Request(RequestStatus::Atomized);
        let new = Status::Job(JobStatus::Running);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_atomized_with_jobcanceled() {
        let mut old = Status::Request(RequestStatus::Atomized);
        let new = Status::Job(JobStatus::Canceled);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_atomized_with_joberrored() {
        let mut old = Status::Request(RequestStatus::Atomized);
        let new = Status::Job(JobStatus::Errored);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_request_atomized_with_jobfinished() {
        let mut old = Status::Request(RequestStatus::Atomized);
        let new = Status::Job(JobStatus::Finished);
        old.merge(&new);
        assert_eq!(old, new);
    }

    // --------------------- Merge Job Queued ---------------------------
    // Should update on all stati that are more advanced

    #[test]
    fn merge_job_queued_with_untouched() {
        let mut old = Status::Job(JobStatus::Queued);
        let new = Status::Request(RequestStatus::Untouched);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Queued));
    }

    #[test]
    fn merge_job_queued_with_invalid() {
        let mut old = Status::Job(JobStatus::Queued);
        let new = Status::Request(RequestStatus::Invalid);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Queued));
    }

    #[test]
    fn merge_job_queued_with_errored() {
        let mut old = Status::Job(JobStatus::Queued);
        let new = Status::Request(RequestStatus::Errored);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Queued));
    }

    #[test]
    fn merge_job_queued_with_checked() {
        let mut old = Status::Job(JobStatus::Queued);
        let new = Status::Request(RequestStatus::Checked);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Queued));
    }

    #[test]
    fn merge_job_queued_with_scanned() {
        let mut old = Status::Job(JobStatus::Queued);
        let new = Status::Request(RequestStatus::Scanned);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Queued));
    }

    #[test]
    fn merge_job_queued_with_atomized() {
        let mut old = Status::Job(JobStatus::Queued);
        let new = Status::Request(RequestStatus::Atomized);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Queued));
    }

    #[test]
    fn merge_job_queued_with_jobrunning() {
        let mut old = Status::Job(JobStatus::Queued);
        let new = Status::Job(JobStatus::Running);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_job_queued_with_jobcanceled() {
        let mut old = Status::Job(JobStatus::Queued);
        let new = Status::Job(JobStatus::Canceled);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_job_queued_with_joberrored() {
        let mut old = Status::Job(JobStatus::Queued);
        let new = Status::Job(JobStatus::Errored);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_job_queued_with_jobfinished() {
        let mut old = Status::Job(JobStatus::Queued);
        let new = Status::Job(JobStatus::Finished);
        old.merge(&new);
        assert_eq!(old, new);
    }

    // --------------------- Merge Job Running ---------------------------
    // Should update on all stati that are more advanced

    #[test]
    fn merge_job_running_with_untouched() {
        let mut old = Status::Job(JobStatus::Running);
        let new = Status::Request(RequestStatus::Untouched);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Running));
    }

    #[test]
    fn merge_job_running_with_invalid() {
        let mut old = Status::Job(JobStatus::Running);
        let new = Status::Request(RequestStatus::Invalid);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Running));
    }

    #[test]
    fn merge_job_running_with_errored() {
        let mut old = Status::Job(JobStatus::Running);
        let new = Status::Request(RequestStatus::Errored);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Running));
    }

    #[test]
    fn merge_job_running_with_checked() {
        let mut old = Status::Job(JobStatus::Running);
        let new = Status::Request(RequestStatus::Checked);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Running));
    }

    #[test]
    fn merge_job_running_with_scanned() {
        let mut old = Status::Job(JobStatus::Running);
        let new = Status::Request(RequestStatus::Scanned);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Running));
    }

    #[test]
    fn merge_job_running_with_atomized() {
        let mut old = Status::Job(JobStatus::Running);
        let new = Status::Request(RequestStatus::Atomized);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Running));
    }

    #[test]
    fn merge_job_running_with_jobqueued() {
        let mut old = Status::Job(JobStatus::Running);
        let new = Status::Job(JobStatus::Queued);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Running));
    }

    #[test]
    fn merge_job_running_with_jobcanceled() {
        let mut old = Status::Job(JobStatus::Running);
        let new = Status::Job(JobStatus::Canceled);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_job_running_with_joberrored() {
        let mut old = Status::Job(JobStatus::Running);
        let new = Status::Job(JobStatus::Errored);
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_job_running_with_jobfinished() {
        let mut old = Status::Job(JobStatus::Running);
        let new = Status::Job(JobStatus::Finished);
        old.merge(&new);
        assert_eq!(old, new);
    }

    // --------------------- Merge Job Canceled ---------------------------
    // Should update on all stati that are more advanced

    #[test]
    fn merge_job_canceled_with_untouched() {
        let mut old = Status::Job(JobStatus::Canceled);
        let new = Status::Request(RequestStatus::Untouched);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Canceled));
    }

    #[test]
    fn merge_job_canceled_with_invalid() {
        let mut old = Status::Job(JobStatus::Canceled);
        let new = Status::Request(RequestStatus::Invalid);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Canceled));
    }

    #[test]
    fn merge_job_canceled_with_errored() {
        let mut old = Status::Job(JobStatus::Canceled);
        let new = Status::Request(RequestStatus::Errored);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Canceled));
    }

    #[test]
    fn merge_job_canceled_with_checked() {
        let mut old = Status::Job(JobStatus::Canceled);
        let new = Status::Request(RequestStatus::Checked);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Canceled));
    }

    #[test]
    fn merge_job_canceled_with_scanned() {
        let mut old = Status::Job(JobStatus::Canceled);
        let new = Status::Request(RequestStatus::Scanned);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Canceled));
    }

    #[test]
    fn merge_job_canceled_with_atomized() {
        let mut old = Status::Job(JobStatus::Canceled);
        let new = Status::Request(RequestStatus::Atomized);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Canceled));
    }

    #[test]
    fn merge_job_canceled_with_jobqueued() {
        let mut old = Status::Job(JobStatus::Canceled);
        let new = Status::Job(JobStatus::Queued);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Canceled));
    }

    #[test]
    fn merge_job_canceled_with_jobrunning() {
        let mut old = Status::Job(JobStatus::Canceled);
        let new = Status::Job(JobStatus::Canceled);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Canceled));
    }

    #[test]
    fn merge_job_canceled_with_joberrored() {
        let mut old = Status::Job(JobStatus::Canceled);
        let new = Status::Job(JobStatus::Errored);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Canceled));
    }

    #[test]
    fn merge_job_canceled_with_jobfinished() {
        let mut old = Status::Job(JobStatus::Canceled);
        let new = Status::Job(JobStatus::Finished);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Canceled));
    }

    // --------------------- Merge Job Errored ---------------------------
    // Should update on all stati that are more advanced

    #[test]
    fn merge_job_errored_with_untouched() {
        let mut old = Status::Job(JobStatus::Errored);
        let new = Status::Request(RequestStatus::Untouched);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Errored));
    }

    #[test]
    fn merge_job_errored_with_invalid() {
        let mut old = Status::Job(JobStatus::Errored);
        let new = Status::Request(RequestStatus::Invalid);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Errored));
    }

    #[test]
    fn merge_job_errored_with_errored() {
        let mut old = Status::Job(JobStatus::Errored);
        let new = Status::Request(RequestStatus::Errored);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Errored));
    }

    #[test]
    fn merge_job_errored_with_checked() {
        let mut old = Status::Job(JobStatus::Errored);
        let new = Status::Request(RequestStatus::Checked);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Errored));
    }

    #[test]
    fn merge_job_errored_with_scanned() {
        let mut old = Status::Job(JobStatus::Errored);
        let new = Status::Request(RequestStatus::Scanned);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Errored));
    }

    #[test]
    fn merge_job_errored_with_atomized() {
        let mut old = Status::Job(JobStatus::Errored);
        let new = Status::Request(RequestStatus::Atomized);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Errored));
    }

    #[test]
    fn merge_job_errored_with_jobqueued() {
        let mut old = Status::Job(JobStatus::Errored);
        let new = Status::Job(JobStatus::Queued);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Errored));
    }

    #[test]
    fn merge_job_errored_with_jobrunning() {
        let mut old = Status::Job(JobStatus::Errored);
        let new = Status::Job(JobStatus::Errored);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Errored));
    }

    #[test]
    fn merge_job_errored_with_jobcanceled() {
        let mut old = Status::Job(JobStatus::Errored);
        let new = Status::Job(JobStatus::Canceled);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Errored));
    }

    #[test]
    fn merge_job_errored_with_jobfinished() {
        let mut old = Status::Job(JobStatus::Errored);
        let new = Status::Job(JobStatus::Finished);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Errored));
    }

// --------------------- Merge Job Finished ---------------------------
    // Should update on all stati that are more advanced

    #[test]
    fn merge_job_finished_with_untouched() {
        let mut old = Status::Job(JobStatus::Finished);
        let new = Status::Request(RequestStatus::Untouched);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Finished));
    }

    #[test]
    fn merge_job_finished_with_invalid() {
        let mut old = Status::Job(JobStatus::Finished);
        let new = Status::Request(RequestStatus::Invalid);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Finished));
    }

    #[test]
    fn merge_job_finished_with_errored() {
        let mut old = Status::Job(JobStatus::Finished);
        let new = Status::Request(RequestStatus::Errored);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Finished));
    }

    #[test]
    fn merge_job_finished_with_checked() {
        let mut old = Status::Job(JobStatus::Finished);
        let new = Status::Request(RequestStatus::Checked);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Finished));
    }

    #[test]
    fn merge_job_finished_with_scanned() {
        let mut old = Status::Job(JobStatus::Finished);
        let new = Status::Request(RequestStatus::Scanned);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Finished));
    }

    #[test]
    fn merge_job_finished_with_atomized() {
        let mut old = Status::Job(JobStatus::Finished);
        let new = Status::Request(RequestStatus::Atomized);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Finished));
    }

    #[test]
    fn merge_job_finished_with_jobqueued() {
        let mut old = Status::Job(JobStatus::Finished);
        let new = Status::Job(JobStatus::Queued);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Finished));
    }

    #[test]
    fn merge_job_finished_with_jobrunning() {
        let mut old = Status::Job(JobStatus::Finished);
        let new = Status::Job(JobStatus::Finished);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Finished));
    }

    #[test]
    fn merge_job_finished_with_jobcanceled() {
        let mut old = Status::Job(JobStatus::Finished);
        let new = Status::Job(JobStatus::Canceled);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Finished));
    }

    #[test]
    fn merge_job_finished_with_joberrored() {
        let mut old = Status::Job(JobStatus::Finished);
        let new = Status::Job(JobStatus::Errored);
        old.merge(&new);
        assert_eq!(old, Status::Job(JobStatus::Finished));
    }

}
