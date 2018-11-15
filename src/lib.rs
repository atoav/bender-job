//! bender_job is a rust library, that serializes and deserializes jobs
//! from `data.json` files. The deserialization yields a Job struct.  
//!
//! It can be loaded into a rust project via git by putting this in your Cargo.toml:  
//! ```text
//! [dependencies]
//! bender_job = { git = "ssh://git@code.hfbk.net:4242/bendercode/bender-job.git" }
//! ```
//! To update this run
//! ```text
//! cargo clean
//! cargo update
//! ```
//!
//! ## Testing
//! The libary is implemented with a extensive amount of tests to make
//! sure that repeated deserialization/serialization won't introduce
//! losses or glitches to the `data.json`. The tests can be run with
//! ```text
//! cargo test
//! ```
//! *Note:* some tests might fail on your system, because the test jobs use absolute \
//! paths. Run `cargo test` a _second_ time to test with updated paths
//!
//! ## Documentation
//! If you want to view the documentation run
//! ```text
//! cargo doc --no-deps --open
//! ```
//! 
//! ## Installation
//! To run cargo, make sure you have rust installed. Go to [rustup.rs](http://rustup.rs) and follow the instructions there
//! 


#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate chrono_humanize;
extern crate bender_bouncer;
extern crate regex;
#[macro_use] extern crate lazy_static;

use chrono::prelude::*;
use chrono::Utc;
use std::collections::{HashMap, BTreeMap};
use std::str;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::error::Error;
use std::collections::VecDeque;


// Modules Structure
pub mod jobtime;
pub use jobtime::JobTime;

pub mod jobpaths;
pub use jobpaths::JobPaths;

pub mod task;
pub use task::{Task, Tasks, TaskQueue};

pub mod status;
pub use status::{Status, JobStatus, RequestStatus};

pub mod data;
pub use data::{Render, Frames, Resolution};

pub mod gaffer;
pub use gaffer::{Gaffer};

pub mod command;
pub use command::Command;

pub mod atomizer;
pub use atomizer::Atomizer;

pub mod bouncer;
pub use bouncer::Bouncer;

pub mod history;
pub use history::{History, HistoryMethods};

pub mod common;

pub mod job;
pub use job::Job;


// Generic Error Types
pub type GenError = Box<std::error::Error>;
pub type GenResult<T> = Result<T, GenError>;



/// Read all Jobs from the directory specified and return a Vector of Jobs.
/// ```
/// # extern crate bender_job;
/// # use bender_job::{read_all, Job};
///
/// // Read all jobs into Vector
/// let jobs = read_all("/data/blendfiles");
///
/// // Apply filters..
/// let valid_jobs: Vec<Job> = jobs.into_iter()
///                      .filter(|job| job.is_validated())
///                      .collect();
/// ```
pub fn read_all<S>(directory: S) -> Vec<Job> where S: Into<String>{
    let directory = directory.into();
    let mut vec = Vec::<Job>::new();
    if let Ok(paths) = fs::read_dir(directory.as_str()){
        for path in paths{
            match path{
                Ok(p) => {
                    match Job::from_directory(p.path()){
                        Ok(job) =>{
                            vec.push(job);
                        },
                        Err(err) => println!("Error: Job::read_all({}) couldn't deserialize Job: {}", directory.as_str(), err)
                    }
                },
                Err(err) => println!("Error: Job::read_all({}) failed with Error: {}", 
                    directory.as_str(), err)
            }
        }
    }
    vec
}
