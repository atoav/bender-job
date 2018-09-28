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
        assert_eq!(j.status.is_invalid(), false);
        j.set_validate();
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
        assert_eq!(j.status.is_invalid(), false);
    }

    #[test]
    fn deny() {
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
        assert_eq!(j.status.is_invalid(), false);
        j.set_deny();
        assert_eq!(j.status.is_finished(), false);
        assert_eq!(j.status.is_untouched(), false);
        assert_eq!(j.status.is_request(), true);
        assert_eq!(j.status.is_job(), false);
        assert_eq!(j.status.is_errored(), false);
        assert_eq!(j.status.is_checked(), false);
        assert_eq!(j.status.is_scanned(), false);
        assert_eq!(j.status.is_atomized(), false);
        assert_eq!(j.status.is_queued(), false);
        assert_eq!(j.status.is_running(), false);
        assert_eq!(j.status.is_canceled(), false);
        assert_eq!(j.status.is_alive(), false);
        assert_eq!(j.status.is_invalid(), true);
    }

    #[test]
    fn error_request() {
        let mut j = common::get_job();
        j.set_validate();
        j.set_error("Some message");
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
        assert_eq!(j.status.is_invalid(), false);
    }

    #[test]
    fn scan() {
        let mut j = common::get_job();
        j.set_validate();
        j.set_scan();
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
        assert_eq!(j.status.is_invalid(), false);
    }

    #[test]
    fn atomize() {
        let mut j = common::get_job();
        j.set_validate();
        j.set_scan();
        j.set_atomize();
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
        assert_eq!(j.status.is_invalid(), false);
    }

    #[test]
    fn queue() {
        let mut j = common::get_job();
        j.set_validate();
        j.set_scan();
        j.set_atomize();
        j.set_queue();
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
        assert_eq!(j.status.is_invalid(), false);
    }

    #[test]
    fn run() {
        let mut j = common::get_job();
        j.set_validate();
        j.set_scan();
        j.set_atomize();
        j.set_queue();
        j.set_run();
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
        assert_eq!(j.status.is_invalid(), false);
    }

    #[test]
    fn cancel() {
        let mut j = common::get_job();
        j.set_validate();
        j.set_scan();
        j.set_atomize();
        j.set_queue();
        j.set_run();
        j.set_cancel();
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
        assert_eq!(j.status.is_invalid(), false);
    }

    #[test]
    fn error_job() {
        let mut j = common::get_job();
        j.set_validate();
        j.set_scan();
        j.set_atomize();
        j.set_queue();
        j.set_run();
        j.set_error("Some error message");
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
        assert_eq!(j.status.is_invalid(), false);
    }


}