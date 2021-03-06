//! The common module contains all kind of functionality used by tests


use ::*;
extern crate chrono;
extern crate rand;
pub extern crate tempfile;


/// Commonly used functions
use std::path::PathBuf;
use std::collections::{HashMap, BTreeMap};
use self::rand::{thread_rng, prelude::SliceRandom};
use std::fs;
use self::tempfile::{Builder, TempDir};

pub mod path;
pub use self::path::*;

pub mod blendfiles;
pub use self::blendfiles::*;






// ===========================================================================
//                            common
// ===========================================================================

/// Generate a cryptographically random id with 32 alphanumeric characters
#[allow(dead_code)]
pub fn random_id() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = thread_rng();
    let id: String = (0..32)
        .map(|_| *CHARSET.choose(&mut rng).expect("Unwrapping of random uuid failed") as char)
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
        time: JobTime::new_deterministic_for_test(),
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
        time: JobTime::new_deterministic_for_test(),
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
        time: JobTime::new_deterministic_for_test(),
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
    let mut jobpath = get_data_blendfilespath();
    jobpath.push(&id);
    
    // Create a temp dir
    let jobpath: PathBuf = jobpath.to_path_buf();
    let jobpath = jobpath.into_os_string().into_string().unwrap();
    let tempdir = Builder::new()
            .prefix(jobpath.as_str())
            .rand_bytes(0)
            .tempdir()
            .expect("Couldn't create directory for random Job..");

    // Test if the temp path is as expected
    debug_assert_eq!(jobpath, tempdir.path().to_string_lossy());

    // Copy blendfile
    let mut source_file_path = get_data_blendfilespath();
    source_file_path.push(source_id.as_str());
    source_file_path.push(source_filename.as_str());
    let temp_blendfile = tempdir.path().join(source_filename.as_str());
    let error_message = format!("Couldn't copy blendfile for random Job from {:?} to {:?}", source_file_path, temp_blendfile);
    fs::copy(&source_file_path, &temp_blendfile).expect(&*error_message); 
    
    // Copy data.json
    let mut source_file_path = get_data_blendfilespath();
    source_file_path.push(source_id.as_str());
    source_file_path.push("data.json");
    let temp_datafile = tempdir.path().join("data.json");
    let error_message = format!("Couldn't copy blendfile for random Job from {:?} to {:?}", source_file_path, temp_datafile);
    fs::copy(&source_file_path, &temp_datafile).expect(&*error_message); 

    // Get a string representing the uploadfolder
    let uploadfolder: PathBuf = tempdir.path().to_path_buf();
    let uploadfolder: String = uploadfolder.into_os_string().into_string().unwrap();

    // Construct Job with fixed creation time (for comparison)
    let job = Job {
        id: id.to_string(),
        paths: JobPaths::from_uploadfolder(uploadfolder.as_str()),
        animation: false,
        email: "dh@atoav.com".to_owned(),
        version: "".to_owned(),
        time: JobTime::new_deterministic_for_test(),
        status: Status::new(),
        data: HashMap::new(),
        history: BTreeMap::new(),
        resolution: Default::default(),
        render: Default::default(),
        frames: Default::default(),
        tasks: Default::default()
    };

    // Create data.json
    job.write_to_file().expect("Couldn't write new random job to file!");
    
    (job, tempdir)
}
