extern crate chrono;
extern crate bender_job;

/// Commonly used functions
use std::path::PathBuf;
use bender_job::{Job, JobPaths, JobTime};
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
    uploadbuf.push("5873c0033e78b222bec2cb2a221487cf");
    format!("{:?}", uploadbuf).replace("\"", "")
}


/// Get a Job
#[allow(dead_code)]
pub fn get_job() -> Job {
    let uploadfolder = get_jobpath();
    Job {
        id: "5873c0033e78b222bec2cb2a221487cf".to_owned(),
        paths: JobPaths::from_uploadfolder(uploadfolder),
        email: "dh@atoav.com".to_owned(),
        time: JobTime {
            creation: Some(Utc.ymd(2018, 8, 23)
                .and_hms_micro(13, 48, 40, 176598)),
            finish: None,
            error: None
        },
        status: "request.untouched".to_owned(),
        data: HashMap::new(),
        history: BTreeMap::new()
    } 
}
