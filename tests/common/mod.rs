extern crate chrono;
extern crate bender_job;

/// Commonly used functions
use std::path::PathBuf;
use bender_job::{Job, JobPaths, JobTimes};
use chrono::Utc;
use chrono::prelude::*;
use std::collections::{HashMap, BTreeMap};



/// Get a Jobpath to the thing in resources
#[allow(dead_code)]
pub fn get_jobpath() -> String {
    let mut uploadbuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    uploadbuf.push("tests");
    uploadbuf.push("resources");
    uploadbuf.push("data");
    uploadbuf.push("blendfiles");
    uploadbuf.push("1be554e1f51b804637326e3faf41d2c9");
    format!("{:?}", uploadbuf).replace("\"", "")
}


/// Get a Job
#[allow(dead_code)]
pub fn get_job() -> Job {
    let uploadfolder = get_jobpath();
    Job {
        id: "1be554e1f51b804637326e3faf41d2c9".to_owned(),
        paths: JobPaths::from_uploadfolder(uploadfolder),
        email: "harold@harold.com".to_owned(),
        times: JobTimes {
            creationtime: Some(Utc.ymd(2015, 5, 15).and_hms(10, 0, 0)),
            finishtime: None,
            errortime: None
        },
        status: "request.untouched".to_owned(),
        data: HashMap::new(),
        history: BTreeMap::new()
    } 
}
