extern crate bender_job;
extern crate chrono;
mod common;



/// Test a Jobs functions
mod status{
    use common;

    #[test]
    fn validate() {
        let mut j = common::get_job();
        assert_eq!(j.status.is_untouched(), true);
        assert_eq!(j.status.is_finished(), false);
        assert_eq!(j.status.is_request(), true);
        assert_eq!(j.status.is_job(), false);
        assert_eq!(j.status.is_errored(), false);
        assert_eq!(j.status.is_checked(), false);
        assert_eq!(j.status.is_scanned(), false);
        assert_eq!(j.status.is_atomized(), false);
        assert_eq!(j.status.is_queued(), false);
        assert_eq!(j.status.is_running(), false);
        assert_eq!(j.status.is_canceled(), false);
        j.validate();
        assert_eq!(j.status.is_finished(), false);
        assert_eq!(j.status.is_untouched(), false);
        assert_eq!(j.status.is_request(), true);
        assert_eq!(j.status.is_job(), false);
        assert_eq!(j.status.is_errored(), false);
        assert_eq!(j.status.is_checked(), true);
        assert_eq!(j.status.is_scanned(), false);
        assert_eq!(j.status.is_atomized(), false);
        assert_eq!(j.status.is_queued(), false);
        assert_eq!(j.status.is_running(), false);
        assert_eq!(j.status.is_canceled(), false);
        assert_eq!(j.status.is_alive(), true);
    }

    #[test]
    fn error_request() {
        let mut j = common::get_job();
        j.validate();
        j.error("Some message");
        assert_eq!(j.status.is_finished(), false);
        assert_eq!(j.status.is_untouched(), false);
        assert_eq!(j.status.is_request(), true);
        assert_eq!(j.status.is_job(), false);
        assert_eq!(j.status.is_errored(), true);
        assert_eq!(j.status.is_checked(), false);
        assert_eq!(j.status.is_scanned(), false);
        assert_eq!(j.status.is_atomized(), false);
        assert_eq!(j.status.is_queued(), false);
        assert_eq!(j.status.is_running(), false);
        assert_eq!(j.status.is_canceled(), false);
        assert_eq!(j.status.is_alive(), false);
    }

    #[test]
    fn scan() {
        let mut j = common::get_job();
        j.validate();
        j.scan();
        assert_eq!(j.status.is_finished(), false);
        assert_eq!(j.status.is_untouched(), false);
        assert_eq!(j.status.is_request(), true);
        assert_eq!(j.status.is_job(), false);
        assert_eq!(j.status.is_errored(), false);
        assert_eq!(j.status.is_validated(), true);
        assert_eq!(j.status.is_scanned(), true);
        assert_eq!(j.status.is_atomized(), false);
        assert_eq!(j.status.is_queued(), false);
        assert_eq!(j.status.is_running(), false);
        assert_eq!(j.status.is_canceled(), false);
        assert_eq!(j.status.is_alive(), true);
    }

    #[test]
    fn atomize() {
        let mut j = common::get_job();
        j.validate();
        j.scan();
        j.atomize();
        assert_eq!(j.status.is_finished(), false);
        assert_eq!(j.status.is_untouched(), false);
        assert_eq!(j.status.is_request(), true);
        assert_eq!(j.status.is_job(), false);
        assert_eq!(j.status.is_errored(), false);
        assert_eq!(j.status.is_validated(), true);
        assert_eq!(j.status.is_scanned(), false);
        assert_eq!(j.status.is_atomized(), true);
        assert_eq!(j.status.is_queued(), false);
        assert_eq!(j.status.is_running(), false);
        assert_eq!(j.status.is_canceled(), false);
        assert_eq!(j.status.is_alive(), true);
    }

    #[test]
    fn queue() {
        let mut j = common::get_job();
        j.validate();
        j.scan();
        j.atomize();
        j.queue();
        assert_eq!(j.status.is_finished(), false);
        assert_eq!(j.status.is_untouched(), false);
        assert_eq!(j.status.is_request(), false);
        assert_eq!(j.status.is_job(), true);
        assert_eq!(j.status.is_errored(), false);
        assert_eq!(j.status.is_validated(), true);
        assert_eq!(j.status.is_scanned(), false);
        assert_eq!(j.status.is_atomized(), false);
        assert_eq!(j.status.is_queued(), true);
        assert_eq!(j.status.is_running(), false);
        assert_eq!(j.status.is_canceled(), false);
        assert_eq!(j.status.is_alive(), true);
    }

    #[test]
    fn run() {
        let mut j = common::get_job();
        j.validate();
        j.scan();
        j.atomize();
        j.queue();
        j.run();
        assert_eq!(j.status.is_finished(), false);
        assert_eq!(j.status.is_untouched(), false);
        assert_eq!(j.status.is_request(), false);
        assert_eq!(j.status.is_job(), true);
        assert_eq!(j.status.is_errored(), false);
        assert_eq!(j.status.is_validated(), true);
        assert_eq!(j.status.is_scanned(), false);
        assert_eq!(j.status.is_atomized(), false);
        assert_eq!(j.status.is_queued(), false);
        assert_eq!(j.status.is_running(), true);
        assert_eq!(j.status.is_canceled(), false);
        assert_eq!(j.status.is_alive(), true);
    }

    #[test]
    fn cancel() {
        let mut j = common::get_job();
        j.validate();
        j.scan();
        j.atomize();
        j.queue();
        j.run();
        j.cancel();
        assert_eq!(j.status.is_finished(), false);
        assert_eq!(j.status.is_untouched(), false);
        assert_eq!(j.status.is_request(), false);
        assert_eq!(j.status.is_job(), true);
        assert_eq!(j.status.is_errored(), false);
        assert_eq!(j.status.is_validated(), true);
        assert_eq!(j.status.is_scanned(), false);
        assert_eq!(j.status.is_atomized(), false);
        assert_eq!(j.status.is_queued(), false);
        assert_eq!(j.status.is_running(), false);
        assert_eq!(j.status.is_canceled(), true);
        assert_eq!(j.status.has_ended(), true);
    }

    #[test]
    fn error_job() {
        let mut j = common::get_job();
        j.validate();
        j.scan();
        j.atomize();
        j.queue();
        j.run();
        j.error("Some error message");
        assert_eq!(j.status.is_finished(), false);
        assert_eq!(j.status.is_untouched(), false);
        assert_eq!(j.status.is_request(), false);
        assert_eq!(j.status.is_job(), true);
        assert_eq!(j.status.is_errored(), true);
        assert_eq!(j.status.is_validated(), true);
        assert_eq!(j.status.is_scanned(), false);
        assert_eq!(j.status.is_atomized(), false);
        assert_eq!(j.status.is_queued(), false);
        assert_eq!(j.status.is_running(), false);
        assert_eq!(j.status.is_canceled(), false);
        assert_eq!(j.status.has_ended(), true);
    }


}