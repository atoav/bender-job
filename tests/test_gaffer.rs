extern crate bender_job;
extern crate chrono;
mod common;



/// Test the Gaffer trait
mod gaffer{
    use common;
    use bender_job::{Gaffer, Render, Frames, Resolution};

    /// Test if scan_and_optimize() errors when passed a unverified job
    #[test]
    fn scan_invalid_errors() {
        let mut j = common::get_random_job();
        j.scan_and_optimize();
        assert_eq!(j.status.is_errored(), true);
        common::delete_random_job(j);
    }

    /// Test if scan_and_optimize() fails with an unexpected error
    #[test]
    fn scan() {
        let mut j = common::get_random_job();
        j.validate();
        assert_eq!(j.status.is_validated(), true);
        j.scan();
        assert_eq!(j.status.is_errored(), false);
        common::delete_random_job(j);
    }

    /// Test if scan_and_optimize() fails with an unexpected error
    #[test]
    fn scan_other() {
        let mut j = common::get_other_random_job();
        j.validate();
        assert_eq!(j.status.is_validated(), true);
        j.scan();
        assert_eq!(j.status.is_errored(), false);
        common::delete_random_job(j);
    }

    /// Check if the gathered info matches the info in the blendfile at
    /// 5873c0033e78b222bec2cb2a221487cf/untitled.blend
    #[test]
    fn checkinfo() {
        let frames = Frames{
            start: 1,
            end: 250,
            step: 1,
            current: 68,
            fps: 25
        };

        let render = Render{
            renderer: "CYCLES".to_string(),
            cuda: false,
            device: "CPU".to_string(),
            image_format: "PNG".to_string(),
            uses_compositing: true
        };

        let resolution = Resolution{
            x: 1920,
            y: 1080,
            scale: 50
        };

        let mut j = common::get_random_job();
        j.set_validate();
        assert_eq!(j.status.is_validated(), true);
        j.scan_and_optimize();
        assert_eq!(j.frames, frames);
        assert_eq!(j.resolution, resolution);
        assert_eq!(j.render, render);
        common::delete_random_job(j);
    }

    /// Check if the history generated in optimize_blend.py gets appended into the
    /// jobs history
    #[test]
    fn integrate_history(){
        let mut j = common::get_random_job();
        j.set_validate();
        assert_eq!(j.status.is_validated(), true);
        j.scan_and_optimize();
        assert_eq!(j.history.iter().any(|(_, value)| value.starts_with("optimize_blend.py")), true);
        common::delete_random_job(j);        
    }

 

}