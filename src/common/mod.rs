use ::*;
extern crate chrono;
extern crate rand;
extern crate tempdir;

/// Commonly used functions
use std::path::PathBuf;
use chrono::Utc;
use std::collections::{HashMap, BTreeMap};
use self::rand::{thread_rng, Rng};
use std::fs;
use self::tempdir::TempDir;

pub mod path;
pub use self::path::*;

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
        animation: false,
        email: "dh@atoav.com".to_owned(),
        version: "".to_owned(),
        time: JobTime {
            creation: Some(Utc.ymd(2018, 8, 23)
                .and_hms_micro(13, 48, 40, 176598)),
            start: None,
            finish: None,
            error: None,
            abort: None,
            pause: None
        },
        status: Status::new(),
        data: HashMap::new(),
        history: BTreeMap::new(),
        resolution: Default::default(),
        render: Default::default(),
        frames: Default::default(),
        tasks: Default::default()
    } 
}

/// Get a Job
#[allow(dead_code)]
pub fn get_other_job() -> Job {
    let jobfolder = get_other_jobpath();
    Job {
        id: "7841becc23339d86ef0ec0a18e312ba1".to_owned(),
        paths: JobPaths::from_uploadfolder(jobfolder),
        animation: true,
        email: "dh@atoav.com".to_owned(),
        version: "".to_owned(),
        time: JobTime {
            creation: Some(Utc.ymd(2018, 8, 23)
                .and_hms_micro(13, 48, 40, 176598)),
            start: None,
            finish: None,
            error: None,
            abort: None,
            pause: None
        },
        status: Status::new(),
        data: HashMap::new(),
        history: BTreeMap::new(),
        resolution: Default::default(),
        render: Default::default(),
        frames: Default::default(),
        tasks: Default::default()
    } 
}


/// Get a invalid Job
#[allow(dead_code)]
pub fn get_invalid_job() -> Job {
    let jobfolder = get_invalid_jobpath();
    Job {
        id: "9ac9b18f5e6d4f329acda411e3de8cde".to_owned(),
        paths: JobPaths::from_uploadfolder(jobfolder),
        animation: false,
        email: "dh@atoav.com".to_owned(),
        version: "".to_owned(),
        time: JobTime {
            creation: Some(Utc.ymd(2018, 8, 23)
                .and_hms_micro(13, 48, 40, 176598)),
            start: None,
            finish: None,
            error: None,
            abort: None,
            pause: None
        },
        status: Status::new(),
        data: HashMap::new(),
        history: BTreeMap::new(),
        resolution: Default::default(),
        render: Default::default(),
        frames: Default::default(),
        tasks: Default::default()
    } 
}




/// Generate a random job
#[allow(dead_code)]
pub fn get_random_job() -> (Job, TempDir) {
    get_random_job_from("5873c0033e78b222bec2cb2a221487cf", "untitled.blend")
}

/// Generate a random job
#[allow(dead_code)]
pub fn get_other_random_job() -> (Job, TempDir) {
    get_random_job_from("7841becc23339d86ef0ec0a18e312ba1", "a.blend")
}

/// Generate a generic random job from the given input
#[allow(dead_code)]
pub fn get_random_job_from<S>(source_id: S, source_filename: S) -> (Job, TempDir) where S: Into<String>{
    let source_id = source_id.into();
    let source_filename = source_filename.into();
    // Create a random ID
    let id = random_id();
    let mut jobpath = get_blendpath();
    jobpath.push(&id);
    // Create a temp dir
    let jobpath: PathBuf = jobpath.to_path_buf();
    let jobpath = jobpath.into_os_string().into_string().unwrap();
    let tempdir = TempDir::new(jobpath.as_str()).expect("Couldn't create directory for other random Job..");
    // Copy blendfile
    let mut source_file_path = get_blendpath();
    source_file_path.push(source_id.as_str());
    source_file_path.push(source_filename.as_str());
    let temp_blendfile = tempdir.path().join(source_filename.as_str());
    let error_message = format!("Couldn't copy blendfile for random Job from {:?} to {:?}", source_file_path, temp_blendfile);
    fs::copy(&source_file_path, &temp_blendfile).expect(error_message.as_str()); 
    // Copy data.json
    let mut source_file_path = get_blendpath();
    source_file_path.push(source_id.as_str());
    source_file_path.push("data.json");
    let temp_datafile = tempdir.path().join("data.json");
    let error_message = format!("Couldn't copy blendfile for random Job from {:?} to {:?}", source_file_path, temp_datafile);
    fs::copy(&source_file_path, &temp_datafile).expect(error_message.as_str()); 
    // Create uploadfolder
    let uploadfolder: PathBuf = tempdir.path().to_path_buf();
    let uploadfolder: String = uploadfolder.into_os_string().into_string().unwrap();
    // Create a job struct
    let j = Job {
        id: id.to_string(),
        paths: JobPaths::from_uploadfolder(uploadfolder.as_str()),
        animation: false,
        email: "dh@atoav.com".to_owned(),
        version: "".to_owned(),
        time: JobTime {
            creation: Some(Utc.ymd(2018, 8, 23)
                .and_hms_micro(13, 48, 40, 176598)),
            start: None,
            finish: None,
            error: None,
            abort: None,
            pause: None
        },
        status: Status::new(),
        data: HashMap::new(),
        history: BTreeMap::new(),
        resolution: Default::default(),
        render: Default::default(),
        frames: Default::default(),
        tasks: Default::default()
    };
    // Create data.json
    j.write_to_file().expect("Couldn't write new random job to file!");
    
    (j, tempdir)
}
