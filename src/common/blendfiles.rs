//! The blendfiles module is used to create permanent, temporary or virtual jobs \
//! from the existing blendfiles located at `./tests/resources/blendfiles`. 
//! 
//! 
//! The structure of the submodules is as follows:  
//! # Permanent
//! Permanent jobs are created at the default location specified in the config \
//! via bender-config. They are _not_ deleted automatically, so you have to clean \
//! up yourself.  
//!
//! Deterministic jobs are jobs with defined deterministic parameters, so you can \
//! compare test results in a easy and meaningful way:  
//! [permanent::deterministic::single](blendfiles/permanent/deterministic/single/index.html)  
//! [permanent::deterministic::multi](blendfiles/permanent/deterministic/multi/index.html)  
//!
//! Random jobs use a set of randomized parameters and are not deterministic in \
//! the exact values beeing used:  
//! [permanent::random::single](blendfiles/permanent/random/single/index.html)  
//! [permanent::random::multi](blendfiles/permanent/random/multi/index.html)  
//!
//! # Temporary
//! Temporary jobs are created in `./tests/resources/blendfiles`. They should \
//! get automatically deleted after going out of scope.  
//!
//! Deterministic jobs are jobs with defined deterministic parameters, so you can \
//! compare test results in a easy and meaningful way: 
//! [temporary::deterministic::single](blendfiles/temporary/deterministic/single/index.html)  
//! [temporary::deterministic::multi](blendfiles/temporary/deterministic/multi/index.html)  
//! 
//! Random jobs use a set of randomized parameters and are not deterministic in \
//! the exact values beeing used:  
//! [temporary::random::single](blendfiles/temporary/random/single/index.html)  
//! [temporary::random::multi](blendfiles/temporary/random/multi/index.html)  
//!
//! # Creation
//! You can use the following functions to generate permanent deterministic jobs \
//! (from files):  
//! [permanent::deterministic::single::get_job(job_selector, id)]()  
//! [permanent::deterministic::multi::create_all_jobs()]()  
//! [permanent::deterministic::multi::create_valid_jobs()]()  
//! [permanent::deterministic::multi::create_invalid_jobs()]()  
//! [permanent::deterministic::multi::create_qu()]()  
//! [permanent::deterministic::multi::create_qu_imbalanced()]()  
//!
//! You can use the following functions to generate permanent random jobs \
//! (from files):  
//! [permanent::random::single::get_job()]()  
//! [permanent::random::single::get_still_job()]()  
//! [permanent::random::single::get_animation_job()]()  
//! [permanent::random::multi::create_n_jobs(n)]()  
//! [permanent::random::multi::create_n_still_jobs(n)]()  
//! [permanent::random::multi::create_n_animation_jobs(n)]()  
//!
//! Or following functions to generate temporary jobs (from files):  
//! [temporary::deterministic::single::get_job(job_selector, id)]()  
//! [temporary::deterministic::multi::create_all_jobs()]()  
//! [temporary::deterministic::multi::create_valid_jobs()]()  
//! [temporary::deterministic::multi::create_invalid_jobs()]()  
//! [temporary::deterministic::multi::create_qu()]()  
//! [temporary::deterministic::multi::create_qu_imbalanced()]()  
//!
//! You can use the following functions to generate temporary random jobs \
//! (from files):  
//! [temporary::random::single::get_job()]()  
//! [temporary::random::single::get_still_job()]()  
//! [temporary::random::single::get_animation_job()]()  
//! [temporary::random::multi::create_n_jobs(n)]()  
//! [temporary::random::multi::create_n_still_jobs(n)]()  
//! [temporary::random::multi::create_n_animation_jobs(n)]()  

extern crate bender_config;
use job::Job;
use common::random_id;
use common::path;





/// A generic generate job function which takes a Function as an argument that generates
/// a job from a source path, a id a email and a boolean deciding if it is an \
/// animation or a still. This function is itself passed to the \
/// `get_deterministic_job` function as an argument
fn apply_job_function<S>(blendfile: S, id: S, email: S, animation: bool, f: &Fn(String, String, String, bool) -> Job) -> Job 
where S: Into<String>{
    let blendfile = blendfile.into();
    let error_message = format!("Error: Couldn't find blendfile named {}", blendfile);
    let source_path = path::get_blendfile_by_name(blendfile.as_str()).expect(error_message.as_str());
    let source_path = source_path.into_os_string().into_string().unwrap();
    let id = id.into();
    let email = email.into();
    f(source_path, id, email, animation)
}

/// Get a job based on a letter e.g. with something like `get_deterministic_job("highres")`. 
/// Pass an empty id ("") if you don't want the predefined ids, pass a function \
/// that does something with the job once it is generated. This means depending \
/// 
fn get_deterministic_job(job_selector: String, id: String, f: &Fn(String, String, String, bool) -> Job) -> Job{
    let mut id = id;
    let id_defined = id != "".to_string();
    match job_selector.as_ref(){
        "animation" =>{
            if !id_defined { id = "aaaaaaacycles1to250xxxanimationa".to_string(); }
            apply_job_function("cycles_1-250.blend", id.as_str(), "dh@atoav.com", true, f)
        },
        "still" =>{
            if !id_defined { id = "sssssssscurrentframe66xstillssss".to_string(); }
            apply_job_function("current_frame_66.blend", id.as_str(), "dh@atoav.com", false, f)
        },
        "step10" =>{
            if !id_defined { id = "uuuuuuuucycles1to250step10animuu".to_string(); }
            apply_job_function("cycles_1-250_step10.blend", id.as_str(), "dh@atoav.com", true, f)
        },
        "invalid" =>{
            if !id_defined { id = "iiiiiiiiinvalidinvalidinvalidiii".to_string(); }
            apply_job_function("invalid.blend", id.as_str(), "dh@atoav.com", true, f)
        },
        "packed" =>{
            if !id_defined { id = "ppppppppppackedtexturepppppppppp".to_string(); }
            apply_job_function("packed_texture.blend", id.as_str(), "dh@atoav.com", true, f)
        },
        "blenderrender" =>{
            if !id_defined { id = "thissssssusesblenderrenderxxxxxx".to_string(); }
            apply_job_function("blenderrender_1-250.blend", id.as_str(), "dh@atoav.com", true, f)
        },
        "highres" =>{
            if !id_defined { id = "highresppuhd3840x2160stillpppppp".to_string(); }
            apply_job_function("UHD_3840x2160.blend", id.as_str(), "dh@atoav.com", false, f)
        },
        "video" =>{
            if !id_defined { id = "dfgdfgdsfagthishasvideooutputppp".to_string(); }
            apply_job_function("cycles_video_output.blend", id.as_str(), "dh@atoav.com", true, f)
        },
        "twoscenes" =>{
            if !id_defined { id = "tttttttthishastwosceneslllllllll".to_string(); }
            apply_job_function("cycles_video_output.blend", id.as_str(), "dh@atoav.com", false, f)
        },
        "qu 1s" =>{
            if !id_defined { id = "250framesforcycleswith1sperframe".to_string(); }
            apply_job_function("qu_1-250_1s_1080p_x0.25.blend", id.as_str(), "dh@atoav.com", true, f)
        },
        "qu 5s" =>{
            if !id_defined { id = "250framesforcycleswith5sperframe".to_string(); }
            apply_job_function("qu_1-250_5s_1080p_x0.5.blend", id.as_str(), "dh@atoav.com", true, f)
        },
        "qu 11s" =>{
            if !id_defined { id = "250frameforcycleswith11sperframe".to_string(); }
            apply_job_function("qu_1-250_11s_1080p_x0.5.blend", id.as_str(), "dh@atoav.com", true, f)
        },
        "qu 20s" =>{
            if !id_defined { id = "250frameforcycleswith20sperframe".to_string(); }
            apply_job_function("qu_1-250_20s_1080p_x0.5.blend", id.as_str(), "dh@atoav.com", true, f)
        },
        "qu 1s B" =>{
            if !id_defined { id = "250framesforcycleswith1sperfrbbb".to_string(); }
            apply_job_function("qu_1-250_1s_1080p_x0.25.blend", id.as_str(), "foo@bar.com", true, f)
        },
        "qu 5s B" =>{
            if !id_defined { id = "250framesforcycleswith5sperfrbbb".to_string(); }
            apply_job_function("qu_1-250_5s_1080p_x0.5.blend", id.as_str(), "foo@bar.com", true, f)
        },
        "qu 11s B" =>{
            if !id_defined { id = "250frameforcycleswith11sperfrbbb".to_string(); }
            apply_job_function("qu_1-250_11s_1080p_x0.5.blend", id.as_str(), "foo@bar.com", true, f)
        },
        "qu 20s B" =>{
            if !id_defined { id = "250frameforcycleswith20sperfrbbb".to_string(); }
            apply_job_function("qu_1-250_20s_1080p_x0.5.blend", id.as_str(), "foo@bar.com", true, f)
        },
        _ => {
            // Default job if everything fails
            apply_job_function("cycles_1-250.blend", random_id().as_str(), "dh@atoav.com", true, f)
        }
    }
}


// ================================= PERMANENT =================================
/// Permanent Jobs are beeing created in the blend directory specified via \
/// indrectly via bender-config. Permanent Jobs are _no_ deleted automatically, \
/// you have to deal with them yourself.
pub mod permanent{

    // ======================== PERMANENT::DETERMINISTIC =======================
    /// functions regarding the creation of permanent deterministic jobs (single 
    /// or multi) with a non-temporary direcory beeing created
    pub mod deterministic{
        use common::blendfiles::bender_config::Config;
        use job::Job;
        use jobpaths::JobPaths;
        use jobtime::JobTime;
        use status::Status;
        use chrono::{Utc, TimeZone};
        use std::path::PathBuf;
        use std::fs;
        use std::collections::{HashMap, BTreeMap};

        // ================== PERMANENT::DETERMINISTIC::SINGLE =================
        /// Creation of permanent deterministic jobs
        pub mod single{
            use ::*;
            use common::blendfiles;
            use common::blendfiles::permanent;

            /// Get a permanent deterministic job with the given id. Passing a \
            /// empty id ("") will yield the default ids defined in the function \
            /// `blendfiles::get_deterministic_job` 
            pub fn get_job<S>(job_selector: S, id: S) -> Job where S: Into<String>{
                let job_selector = job_selector.into();
                let id = id.into();
                blendfiles::get_deterministic_job(job_selector, id, &permanent::deterministic::from_blendfile)
            }

        }

        // ================== PERMANENT::DETERMINISTIC::MULTI ==================
        /// Creation of Vectors filled with n ...
        pub mod multi{
            use job::Job;
            use common::blendfiles::permanent::deterministic;

            /// Create all jobs defined in the get_deterministic_job function.
            /// This includes invalid jobs
            pub fn create_all_jobs() -> Vec<Job>{
                let mut vec = Vec::<Job>::new();
                vec.push(deterministic::single::get_job("animation", ""));
                vec.push(deterministic::single::get_job("still", ""));
                vec.push(deterministic::single::get_job("step10", ""));
                vec.push(deterministic::single::get_job("invalid", ""));
                vec.push(deterministic::single::get_job("packed", ""));
                vec.push(deterministic::single::get_job("blenderrender", ""));
                vec.push(deterministic::single::get_job("highres", ""));
                vec.push(deterministic::single::get_job("video", ""));
                vec.push(deterministic::single::get_job("twoscenes", ""));
                vec.push(deterministic::single::get_job("qu 1s", ""));
                vec.push(deterministic::single::get_job("qu 5s", ""));
                vec.push(deterministic::single::get_job("qu 11s", ""));
                vec.push(deterministic::single::get_job("qu 20s", ""));
                vec
            }

            /// Create all valid jobs defined in the get_deterministic_job function.
            pub fn create_valid_jobs() -> Vec<Job>{
                let mut vec = Vec::<Job>::new();
                vec.push(deterministic::single::get_job("animation", ""));
                vec.push(deterministic::single::get_job("still", ""));
                vec.push(deterministic::single::get_job("step10", ""));
                vec.push(deterministic::single::get_job("packed", ""));
                vec.push(deterministic::single::get_job("blenderrender", ""));
                vec.push(deterministic::single::get_job("highres", ""));
                vec.push(deterministic::single::get_job("video", ""));
                vec.push(deterministic::single::get_job("twoscenes", ""));
                vec.push(deterministic::single::get_job("qu 1s", ""));
                vec.push(deterministic::single::get_job("qu 5s", ""));
                vec.push(deterministic::single::get_job("qu 11s", ""));
                vec.push(deterministic::single::get_job("qu 20s", ""));
                vec
            }

            /// Create invalid jobs defined in the get_deterministic_job function.
            pub fn create_invalid_jobs() -> Vec<Job>{
                let mut vec = Vec::<Job>::new();
                vec.push(deterministic::single::get_job("invalid", ""));
                vec
            }

            /// Create a simple queue with two people and two jobs
            pub fn create_qu() -> Vec<Job>{
                let mut vec = Vec::<Job>::new();
                vec.push(deterministic::single::get_job("qu 1s", ""));
                vec.push(deterministic::single::get_job("qu 1s B", ""));

                // Do all the processing necessary to get a valid queue
                let vec = vec.iter()
                             .cloned()
                             .map(|mut job|{
                                job.validate();
                                job.scan();
                                job.atomize();
                                job.queue();
                                job
                             })
                             .collect();
                vec
            }

            /// Create a simple queue with two people and two jobs (imbalanced)
            pub fn create_qu_imbalanced() -> Vec<Job>{
                let mut vec = Vec::<Job>::new();
                vec.push(deterministic::single::get_job("qu 5s", ""));
                vec.push(deterministic::single::get_job("qu 1s B", ""));

                // Do all the processing necessary to get a valid queue
                let vec = vec.iter()
                             .cloned()
                             .map(|mut job|{
                                job.validate();
                                job.scan();
                                job.atomize();
                                job.queue();
                                job
                             })
                             .collect();
                vec
            }
        }

        // ---------------------- PERMANENT::DETERMINISTIC ---------------------
        /// Create a Job from a existing blendfile and copy it to a new folder in
        /// the data directory specified in the config. This is the base
        /// function creating _all_ permanent jobs within `common::blendfiles::permanent`
        pub fn from_blendfile<P, S>(source_path: P, id: S, email: S, animation: bool) -> Job 
        where P: Into<PathBuf>, S: Into<String>{
            let config = Config::from_file(Config::location()).unwrap();
            let data_blendfilespath = config.paths.blend();
            // Common variables
            let source_path = source_path.into();
            let id = id.into();
            let email = email.into();

            // Create a Path for the job (e.g. /data/blendfiles/<id>)
            let mut jobpathbuf = PathBuf::from(data_blendfilespath.clone());
            jobpathbuf.push(id.as_str());
            let jobpath = jobpathbuf.clone().into_os_string().into_string().unwrap();

            // Create the directory
            fs::create_dir_all(jobpath.as_str()).expect("Couldn't create directory for permanent Job..");
            
            // Copy the file from blendfile to the temp folder
            let source_filename = source_path.clone();
            let source_filename = source_filename.file_name().unwrap();
            let source_filename = source_filename.to_os_string().into_string().unwrap();
            let mut target_blendfile = jobpathbuf.clone();
            target_blendfile.push(source_filename.as_str());
            let error_message = format!("Couldn't copy blendfile for permanent Job from {:?} to {:?}", source_path, target_blendfile);
            fs::copy(&source_path, &target_blendfile).expect(error_message.as_str());

            // Construct Job with fixed creation time (for comparison)
            let job = Job {
                id: id,
                paths: JobPaths::from_uploadfolder(jobpath.as_str()),
                animation: animation,
                email: email,
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

            // Write the "data.json" to the temporary folder
            job.write_to_file().expect("Couldn't write new random job to file!");

            job
        }
    }

    // =========================== PERMANENT::RANDOM ===========================
    /// functions regarding the creation of permanent random jobs (single or multi)
    /// with a temp directory beeing created
    pub mod random{
        use common::blendfiles::bender_config::Config;
        use job::Job;
        use jobpaths::JobPaths;
        use jobtime::JobTime;
        use status::Status;
        use chrono::Utc;
        use std::path::PathBuf;
        use std::fs;
        use std::collections::{HashMap, BTreeMap};

        // ===================== PERMANENT::RANDOM::SINGLE =====================
        /// Creation of permanent random jobs
        pub mod single{
            use ::*;
            use common::random_id;
            use common::blendfiles::permanent;
            use common::rand::{thread_rng, Rng};

            /// Create a randomized job
            pub fn get_job<S>(source_path: S) -> Job where S: Into<String>{
                let source_path = source_path.into();
                let emails = ["a@b.de", "dh@atoav.com", "don@mafia.com", "foo@bar.de"];
                let mut rng = thread_rng();
                let email = rng.choose(&emails).unwrap().to_string();
                let id = random_id();
                let animation = rng.gen_bool(0.5);
                permanent::random::from_blendfile(source_path, id, email, animation)
            }


            /// Create a randomized job (still)
            pub fn get_still_job<S>(source_path: S) -> Job where S: Into<String>{
                let source_path = source_path.into();
                let emails = ["a@b.de", "dh@atoav.com", "don@mafia.com", "foo@bar.de"];
                let mut rng = thread_rng();
                let email = rng.choose(&emails).unwrap().to_string();
                let id = random_id();
                permanent::random::from_blendfile(source_path, id, email, false)
            }


            /// Create a randomized job (animation)
            pub fn get_animation_job<S>(source_path: S) -> Job where S: Into<String>{
                let source_path = source_path.into();
                let emails = ["a@b.de", "dh@atoav.com", "don@mafia.com", "foo@bar.de"];
                let mut rng = thread_rng();
                let email = rng.choose(&emails).unwrap().to_string();
                let id = random_id();
                permanent::random::from_blendfile(source_path, id, email, true)
            }
        }

        // ====================== PERMANENT::RANDOM::MULTI =====================
        /// Creation of Vectors filled with n ...
        pub mod multi{
            use ::*;
            use common::blendfiles::permanent;
            use common::path;
            use common::rand::{thread_rng, Rng};

            /// Create n jobs that are completely random. That means they are either valid or
            /// invalid, still or animation and have random email adresses
            pub fn create_n_jobs(n: usize) -> Vec<Job>{
                let mut vec = Vec::<Job>::with_capacity(n);
                let blendfiles = path::get_blendfiles();
                let mut rng = thread_rng();
                for _ in 0 .. n{
                    let blendfile = rng.choose(&blendfiles).unwrap();
                    let blendfile = blendfile.clone().into_os_string().into_string().unwrap();
                    vec.push(permanent::random::single::get_job(blendfile));
                }
                vec
            }

            /// Create n random still jobs. That means they have random email adresses
            pub fn create_n_still_jobs(n: usize) -> Vec<Job>{
                let mut vec = Vec::<Job>::with_capacity(n);
                let blendfiles = path::get_blendfiles();

                // Filter all files containing invalid in their filename
                let blendfiles: Vec<PathBuf> = blendfiles.iter()
                    .filter(|path| path.file_name().unwrap().to_string_lossy().contains("invalid"))
                    .cloned()
                    .collect();
                let mut rng = thread_rng();
                for _ in 0 .. n{
                    let blendfile = rng.choose(&blendfiles).unwrap();
                    let blendfile = blendfile.clone().into_os_string().into_string().unwrap();
                    vec.push(permanent::random::single::get_still_job(blendfile));
                }
                vec
            }

            /// Create n random animation jobs. That means they have random email adresses
            pub fn create_n_animation_jobs(n: usize) -> Vec<Job>{
                let mut vec = Vec::<Job>::with_capacity(n);
                let blendfiles = path::get_blendfiles();

                // Filter all files containing invalid in their filename
                let blendfiles: Vec<PathBuf> = blendfiles.iter()
                    .filter(|path| path.file_name().unwrap().to_string_lossy().contains("invalid"))
                    .cloned()
                    .collect();
                let mut rng = thread_rng();
                for _ in 0 .. n{
                    let blendfile = rng.choose(&blendfiles).unwrap();
                    let blendfile = blendfile.clone().into_os_string().into_string().unwrap();
                    vec.push(permanent::random::single::get_animation_job(blendfile));
                }
                vec
            }
        }

        // ------------------------- PERMANENT::RANDOM -------------------------
        /// Create a Job from a existing blendfile and copy it to a new folder in
        /// the data directory specified in the config. This is the base
        /// function creating _all_ permanent jobs within `common::blendfiles::permanent`
        pub fn from_blendfile<P, S>(source_path: P, id: S, email: S, animation: bool) -> Job 
        where P: Into<PathBuf>, S: Into<String>{
            let config = Config::from_file(Config::location()).unwrap();
            let data_blendfilespath = config.paths.blend();
            // Common variables
            let source_path = source_path.into();
            let id = id.into();
            let email = email.into();

            // Create a Path for the job (e.g. /data/blendfiles/<id>)
            let mut jobpathbuf = PathBuf::from(data_blendfilespath.clone());
            jobpathbuf.push(id.as_str());
            let jobpath = jobpathbuf.clone().into_os_string().into_string().unwrap();

            // Create the directory
            fs::create_dir_all(jobpath.as_str()).expect("Couldn't create directory for permanent Job..");
            
            // Copy the file from blendfile to the temp folder
            let source_filename = source_path.clone();
            let source_filename = source_filename.file_name().unwrap();
            let source_filename = source_filename.to_os_string().into_string().unwrap();
            let mut target_blendfile = jobpathbuf.clone();
            target_blendfile.push(source_filename.as_str());
            let error_message = format!("Couldn't copy blendfile for permanent Job from {:?} to {:?}", source_path, target_blendfile);
            fs::copy(&source_path, &target_blendfile).expect(error_message.as_str());

            // Construct Job with fixed creation time (for comparison)
            let job = Job {
                id: id,
                paths: JobPaths::from_uploadfolder(jobpath.as_str()),
                animation: animation,
                email: email,
                version: "".to_owned(),
                time: JobTime {
                    creation: Some(Utc::now()),
                    start: None,
                    finish: None,
                    error: None,
                    abort: None,
                    pause: None
                },
                status:     Status::new(),
                data:       HashMap::new(),
                history:    BTreeMap::new(),
                resolution: Default::default(),
                render:     Default::default(),
                frames:     Default::default(),
                tasks:      Default::default()
            };

            // Write the "data.json" to the temporary folder
            job.write_to_file().expect("Couldn't write new random job to file!");

            job
        }
    }
}



// ================================= TEMPORARY =================================
/// Creation of _temporary_ jobs. Temporary means, there is a temporary \
/// directory beeing created, that will automatically get erased in case \
/// after the work is done.
pub mod temporary{
    use ::*;
    use std::path::PathBuf;
    use common::path::{get_data_blendfilespath};
    use std::fs;
    use common::tempfile::Builder;


    // ======================== TEMPORARY::DETERMINISTIC =======================
    /// functions regarding the creation of temporary random jobs (single or 
    /// multi) with a non-temporary direcory beeing created
    pub mod deterministic{

        // ================== TEMPORARY::DETERMINISTIC::SINGLE =================
        /// Creation of temporary deterministic _single_ jobs
        pub mod single{
            use ::*;
            use common::blendfiles;
            use common::temporary;

            /// Get a temporary deterministic job
            pub fn get_job<S>(job_selector: S) -> Job where S: Into<String>{
                let job_selector = job_selector.into();
                blendfiles::get_deterministic_job(job_selector, "".to_string(), &temporary::from_blendfile)
            }
        }

        // ================== TEMPORARY::DETERMINISTIC::MULTI ==================
        /// Creation of Vectors filled with n deterministic temporary jobs
        pub mod multi{
            use job::Job;
            use common::blendfiles::temporary::deterministic;

            /// Create all jobs defined in the get_deterministic_job function.
            /// This includes invalid jobs
            pub fn create_all_jobs() -> Vec<Job>{
                let mut vec = Vec::<Job>::new();
                vec.push(deterministic::single::get_job("animation"));
                vec.push(deterministic::single::get_job("still"));
                vec.push(deterministic::single::get_job("step10"));
                vec.push(deterministic::single::get_job("invalid"));
                vec.push(deterministic::single::get_job("packed"));
                vec.push(deterministic::single::get_job("blenderrender"));
                vec.push(deterministic::single::get_job("highres"));
                vec.push(deterministic::single::get_job("video"));
                vec.push(deterministic::single::get_job("twoscenes"));
                vec.push(deterministic::single::get_job("qu 1s"));
                vec.push(deterministic::single::get_job("qu 5s"));
                vec.push(deterministic::single::get_job("qu 11s"));
                vec.push(deterministic::single::get_job("qu 20s"));
                vec
            }

            /// Create all valid jobs defined in the get_deterministic_job function.
            pub fn create_valid_jobs() -> Vec<Job>{
                let mut vec = Vec::<Job>::new();
                vec.push(deterministic::single::get_job("animation"));
                vec.push(deterministic::single::get_job("still"));
                vec.push(deterministic::single::get_job("step10"));
                vec.push(deterministic::single::get_job("packed"));
                vec.push(deterministic::single::get_job("blenderrender"));
                vec.push(deterministic::single::get_job("highres"));
                vec.push(deterministic::single::get_job("video"));
                vec.push(deterministic::single::get_job("twoscenes"));
                vec.push(deterministic::single::get_job("qu 1s"));
                vec.push(deterministic::single::get_job("qu 5s"));
                vec.push(deterministic::single::get_job("qu 11s"));
                vec.push(deterministic::single::get_job("qu 20s"));
                vec
            }

            /// Create invalid jobs defined in the get_deterministic_job function.
            pub fn create_invalid_jobs() -> Vec<Job>{
                let mut vec = Vec::<Job>::new();
                vec.push(deterministic::single::get_job("invalid"));
                vec
            }

            /// Create a simple queue with two people and two jobs
            pub fn create_qu() -> Vec<Job>{
                let mut vec = Vec::<Job>::new();
                vec.push(deterministic::single::get_job("qu 1s"));
                vec.push(deterministic::single::get_job("qu 1s B"));

                // Do all the processing necessary to get a valid queue
                let vec = vec.iter()
                             .cloned()
                             .map(|mut job|{
                                job.validate();
                                job.scan();
                                job.atomize();
                                job.queue();
                                job
                             })
                             .collect();
                vec
            }

            /// Create a simple queue with two people and two jobs (imbalanced)
            pub fn create_qu_imbalanced() -> Vec<Job>{
                let mut vec = Vec::<Job>::new();
                vec.push(deterministic::single::get_job("qu 5s"));
                vec.push(deterministic::single::get_job("qu 1s B"));

                // Do all the processing necessary to get a valid queue
                let vec = vec.iter()
                             .cloned()
                             .map(|mut job|{
                                job.validate();
                                job.scan();
                                job.atomize();
                                job.queue();
                                job
                             })
                             .collect();
                vec
            }
        }

    }

    // =========================== TEMPORARY::RANDOM ===========================
    /// functions regarding the creation of temporary random jobs (single or multi) \
    /// with a temp directory beeing created
    pub mod random {

        // ===================== TEMPORARY::RANDOM::SINGLE =====================
        /// Creation of temporary random _single_ jobs
        pub mod single{
            use ::*;
            use common::random_id;
            use common::blendfiles::temporary;
            use common::rand::{thread_rng, Rng};

            /// Create a randomized job
            pub fn get_job<S>(source_path: S) -> Job where S: Into<String>{
                let source_path = source_path.into();
                let emails = ["a@b.de", "dh@atoav.com", "don@mafia.com", "foo@bar.de"];
                let mut rng = thread_rng();
                let email = rng.choose(&emails).unwrap().to_string();
                let id = random_id();
                let animation = rng.gen_bool(0.5);
                temporary::from_blendfile(source_path, id, email, animation)
            }


            /// Create a randomized job (still)
            pub fn get_still_job<S>(source_path: S) -> Job where S: Into<String>{
                let source_path = source_path.into();
                let emails = ["a@b.de", "dh@atoav.com", "don@mafia.com", "foo@bar.de"];
                let mut rng = thread_rng();
                let email = rng.choose(&emails).unwrap().to_string();
                let id = random_id();
                temporary::from_blendfile(source_path, id, email, false)
            }


            /// Create a randomized job (animation)
            pub fn get_animation_job<S>(source_path: S) -> Job where S: Into<String>{
                let source_path = source_path.into();
                let emails = ["a@b.de", "dh@atoav.com", "don@mafia.com", "foo@bar.de"];
                let mut rng = thread_rng();
                let email = rng.choose(&emails).unwrap().to_string();
                let id = random_id();
                temporary::from_blendfile(source_path, id, email, true)
            }
        }

        // ====================== TEMPORARY::RANDOM::MULTI =====================
        /// Creation of Vectors filled with n random temporary jobs
        pub mod multi{
            use ::*;
            use common::blendfiles::temporary;
            use common::path;
            use common::rand::{thread_rng, Rng};

            /// Create n jobs that are completely random. That means they are either valid or
            /// invalid, still or animation and have random email adresses
            pub fn create_n_jobs(n: usize) -> Vec<Job>{
                let mut vec = Vec::<Job>::with_capacity(n);
                let blendfiles = path::get_blendfiles();
                let mut rng = thread_rng();
                for _ in 0 .. n{
                    let blendfile = rng.choose(&blendfiles).unwrap();
                    let blendfile = blendfile.clone().into_os_string().into_string().unwrap();
                    vec.push(temporary::random::single::get_job(blendfile));
                }
                vec
            }

            /// Create n random still jobs. That means they have random email adresses
            pub fn create_n_still_jobs(n: usize) -> Vec<Job>{
                let mut vec = Vec::<Job>::with_capacity(n);
                let blendfiles = path::get_blendfiles();

                // Filter all files containing invalid in their filename
                let blendfiles: Vec<PathBuf> = blendfiles.iter()
                    .filter(|path| path.file_name().unwrap().to_string_lossy().contains("invalid"))
                    .cloned()
                    .collect();
                let mut rng = thread_rng();
                for _ in 0 .. n{
                    let blendfile = rng.choose(&blendfiles).unwrap();
                    let blendfile = blendfile.clone().into_os_string().into_string().unwrap();
                    vec.push(temporary::random::single::get_still_job(blendfile));
                }
                vec
            }

            /// Create n random animation jobs. That means they have random email adresses
            pub fn create_n_animation_jobs(n: usize) -> Vec<Job>{
                let mut vec = Vec::<Job>::with_capacity(n);
                let blendfiles = path::get_blendfiles();

                // Filter all files containing invalid in their filename
                let blendfiles: Vec<PathBuf> = blendfiles.iter()
                    .filter(|path| path.file_name().unwrap().to_string_lossy().contains("invalid"))
                    .cloned()
                    .collect();
                let mut rng = thread_rng();
                for _ in 0 .. n{
                    let blendfile = rng.choose(&blendfiles).unwrap();
                    let blendfile = blendfile.clone().into_os_string().into_string().unwrap();
                    vec.push(temporary::random::single::get_animation_job(blendfile));
                }
                vec
            }
        }
    }


    // --------------------------- TEMPORARY::RANDOM ---------------------------
    /// Create a Job from a existing blendfile and copy it to a new temp folder in
    /// the jobs `./tests/resources/data/blendfiles/<id>`. This is the base
    /// function creating _all_ temporary jobs within `common::blendfiles::temporary`
    pub fn from_blendfile<P, S>(source_path: P, id: S, email: S, animation: bool) -> Job 
    where P: Into<PathBuf>, S: Into<String>{
        // Common variables
        let source_path = source_path.into();
        let id = id.into();
        let email = email.into();
        let data_blendfilespath = get_data_blendfilespath();

        // Create a Path for the job
        let mut jobpath = data_blendfilespath.clone();
        jobpath.push(id.as_str());
        let jobpath = jobpath.into_os_string().into_string().unwrap();

        // Create a Temp dir
        let tempdir = Builder::new()
                .prefix(jobpath.as_str())
                .rand_bytes(0)
                .tempdir()
                .expect("Couldn't create directory for temporary Job..");

        // Test if the temp path is as expected
        debug_assert_eq!(jobpath, tempdir.path().to_string_lossy());
        
        // Copy the file from blendfile to the temp folder
        let source_filename = source_path.clone();
        let source_filename = source_filename.file_name().unwrap();
        let source_filename = source_filename.to_os_string().into_string().unwrap();
        let temp_blendfile = tempdir.path().join(source_filename.as_str());
        let error_message = format!("Couldn't copy blendfile for temporary Job from {:?} to {:?}", source_path, temp_blendfile);
        fs::copy(&source_path, &temp_blendfile).expect(error_message.as_str());

        // Get a string representing the uploadfolder
        let uploadfolder: PathBuf = tempdir.path().to_path_buf();
        let uploadfolder: String = uploadfolder.into_os_string().into_string().unwrap();

        // Construct Job with fixed creation time (for comparison)
        let job = Job {
            id: id,
            paths: JobPaths::from_uploadfolder(uploadfolder.as_str()),
            animation: animation,
            email: email,
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

        // Write the "data.json" to the temporary folder
        job.write_to_file().expect("Couldn't write new random job to file!");

        job
    }

}