//! The bouncer module extends Job with the functionality to run a quick binary \
//! check on its blendfile, without the need for a installed blender  
//!
//! It does so by defining the Bouncer trait which then is implemented for Job
use ::*;

/// This trait implements the binary file checking from the `bender-bouncer` \
/// library for the job itself. This way a job can check the validity of the \
/// blendfile it is supposed to render.
pub trait Bouncer{
    fn check_with_bouncer(&mut self);
}

impl Bouncer for Job{
    /// The check function allows the job to validate the binary blendfile it \
    /// stores for processing. A successful check will run `Job::validate()` \
    /// while a Error will deny the Job. The check utilizes the check_blend() \
    /// function implemented in `bender-bouncer`.
    fn check_with_bouncer(&mut self){
        match bender_bouncer::check_blend(self.paths.blend.as_str()){
        Ok(version) => {
            self.version = version;
            self.set_validate();
        },
        Err(_err) => self.deny()
    }
    }
}


