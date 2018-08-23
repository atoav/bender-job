extern crate chrono;
extern crate bender_job;
extern crate rand;

/// Commonly used functions
use std::path::PathBuf;
use bender_job::{Job, JobPaths, JobTime};
use chrono::Utc;
use chrono::prelude::*;
use std::collections::{HashMap, BTreeMap};
use self::rand::{thread_rng, Rng};
use std::fs;

/// Get a path to the resources uploadpath
#[allow(dead_code)]
pub fn get_blendpath() -> PathBuf {
    let mut buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    buf.push("tests");
    buf.push("resources");
    buf.push("data");
    buf.push("blendfiles");
    buf
}

/// Get a Jobpath to the thing in resources
#[allow(dead_code)]
pub fn get_jobpath() -> String {
    let mut buf = get_blendpath();
    buf.push("5873c0033e78b222bec2cb2a221487cf");
    format!("{:?}", buf).replace("\"", "")
}

// Return a random id
#[allow(dead_code)]
pub fn random_id() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = thread_rng();
    let id: String = (0..32)
        .map(|_| *rng.choose(CHARSET).expect("Unwrapping of random uuid failed") as char)
        .collect();
    id
}


/// Get a Job
#[allow(dead_code)]
pub fn get_job() -> Job {
    let jobfolder = get_jobpath();
    Job {
        id: "5873c0033e78b222bec2cb2a221487cf".to_owned(),
        paths: JobPaths::from_uploadfolder(jobfolder),
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


/// Generate a random job
#[allow(dead_code)]
pub fn get_random_job() -> Job {
    let id = random_id();
    let mut jobpath = get_blendpath();
    jobpath.push(&id);
    let jobstring = jobpath.clone().into_os_string().into_string().expect("Unwrapping pathbuf in random job failed");
    // Create a directory for the random job
    fs::create_dir(&jobpath).expect("Couldn't create directory for random Job..");
    // Copy untitled.blend there
    let mut blendfile = get_blendpath();
    blendfile.push("5873c0033e78b222bec2cb2a221487cf");
    blendfile.push("untitled.blend");
    jobpath.push("untitled.blend");
    fs::copy(blendfile, jobpath).expect("Couldn't copy blendfile for random Job..");
    // Create a job struct
    let j = Job {
        id: id.to_string(),
        paths: JobPaths::from_uploadfolder(jobstring),
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
    };
    // Create data.json
    j.write_to_file().expect("Couldn't write new random job to file!");
    
    j
}


/// Delete a random Job
#[allow(dead_code)]
pub fn delete_random_job(j: Job) {
    fs::remove_dir_all(j.paths.upload).expect("Couldn't delete random Job..");
}