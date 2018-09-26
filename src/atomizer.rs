use ::*;

/// This Trait is implemented by a [Job](struct.Job.html) and deals with atomizing (aka splitting)
/// the Jobs blendfile into [Tasks](struct.Task.html).
pub trait Atomizer{
    fn atomize(&mut self);
}

impl Atomizer for Job{
    /// Create Tasks for the Job
    fn atomize(&mut self){
        
    }
}