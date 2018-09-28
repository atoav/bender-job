extern crate bender_job;
extern crate chrono;
mod common;



/// Test the Gaffer trait
mod bouncer{
    use common;
    use bender_job::{Bouncer};

    #[test]
    fn allow() {
        let mut j = common::get_job();
        assert_eq!(j.status.is_untouched(), true);
        j.validate();
        assert_eq!(j.status.is_validated(), true);
    }

    #[test]
    fn deny() {
        let mut j = common::get_invalid_job();
        assert_eq!(j.status.is_untouched(), true);
        j.validate();
        assert_eq!(j.status.is_invalid(), true);
    }
}