




/// Creation of _temporary_ jobs (for testing purposes).
pub mod temporary{
    use ::*;
    use std::path::PathBuf;
    use common::path::{get_data_blendfilespath};
    use std::fs;
    use common::tempfile::Builder;

    /// Creation of temporary random Jobs (with a temp directory beeing created)
    pub mod random {

        /// Creation of temporary random _single_ jobs
        pub mod single{
            use ::*;
            use common::random_id;
            use common::blendfiles::temporary::{from_blendfile};
            use common::rand::{thread_rng, Rng};

            /// Create a randomized job
            pub fn create_completely_random_job<S>(source_path: S) -> Job where S: Into<String>{
                let source_path = source_path.into();
                let emails = ["a@b.de", "dh@atoav.com", "don@mafia.com", "foo@bar.de"];
                let mut rng = thread_rng();
                let email = rng.choose(&emails).unwrap().to_string();
                let id = random_id();
                let animation = rng.gen_bool(0.5);
                from_blendfile(source_path, id, email, animation)
            }


            /// Create a randomized job (still)
            pub fn create_random_still_job<S>(source_path: S) -> Job where S: Into<String>{
                let source_path = source_path.into();
                let emails = ["a@b.de", "dh@atoav.com", "don@mafia.com", "foo@bar.de"];
                let mut rng = thread_rng();
                let email = rng.choose(&emails).unwrap().to_string();
                let id = random_id();
                from_blendfile(source_path, id, email, false)
            }


            /// Create a randomized job (animation)
            pub fn create_random_animation_job<S>(source_path: S) -> Job where S: Into<String>{
                let source_path = source_path.into();
                let emails = ["a@b.de", "dh@atoav.com", "don@mafia.com", "foo@bar.de"];
                let mut rng = thread_rng();
                let email = rng.choose(&emails).unwrap().to_string();
                let id = random_id();
                from_blendfile(source_path, id, email, true)
            }
        }


        ///
        pub mod multi{
            use ::*;
            use common::blendfiles::temporary::random::single::create_completely_random_job;
            use common::blendfiles::temporary::random::single::create_random_still_job;
            use common::blendfiles::temporary::random::single::create_random_animation_job;
            use common::path::get_blendfiles;
            use common::rand::{thread_rng, Rng};

            /// Create n jobs that are completely random. That means they are either valid or
            /// invalid, still or animation and have random email adresses
            pub fn create_n_completely_random_jobs(n: usize) -> Vec<Job>{
                let mut vec = Vec::<Job>::with_capacity(n);
                let blendfiles = get_blendfiles();
                let mut rng = thread_rng();
                for _ in 0 .. n{
                    let blendfile = rng.choose(&blendfiles).unwrap();
                    let blendfile = blendfile.clone().into_os_string().into_string().unwrap();
                    vec.push(create_completely_random_job(blendfile));
                }
                vec
            }

            /// Create n random still jobs. That means they have random email adresses
            pub fn create_n_random_still_jobs(n: usize) -> Vec<Job>{
                let mut vec = Vec::<Job>::with_capacity(n);
                let blendfiles = get_blendfiles();

                // Filter all files containing invalid in their filename
                let blendfiles: Vec<PathBuf> = blendfiles.iter()
                    .filter(|path| path.file_name().unwrap().to_string_lossy().contains("invalid"))
                    .cloned()
                    .collect();
                let mut rng = thread_rng();
                for _ in 0 .. n{
                    let blendfile = rng.choose(&blendfiles).unwrap();
                    let blendfile = blendfile.clone().into_os_string().into_string().unwrap();
                    vec.push(create_random_still_job(blendfile));
                }
                vec
            }

            /// Create n random animation jobs. That means they have random email adresses
            pub fn create_n_random_animation_jobs(n: usize) -> Vec<Job>{
                let mut vec = Vec::<Job>::with_capacity(n);
                let blendfiles = get_blendfiles();

                // Filter all files containing invalid in their filename
                let blendfiles: Vec<PathBuf> = blendfiles.iter()
                    .filter(|path| path.file_name().unwrap().to_string_lossy().contains("invalid"))
                    .cloned()
                    .collect();
                let mut rng = thread_rng();
                for _ in 0 .. n{
                    let blendfile = rng.choose(&blendfiles).unwrap();
                    let blendfile = blendfile.clone().into_os_string().into_string().unwrap();
                    vec.push(create_random_animation_job(blendfile));
                }
                vec
            }
        }
    }



    /// Create a Job from a existing blendfile and copy it to a new temp folder in
    /// the jobs `./tests/resources/data/blendfiles/<id>`
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
                .tempdir()
                .expect("Couldn't create directory for temporary Job..");
        
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