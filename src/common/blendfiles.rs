use ::*;
use common::path::get_resourcepath;
use std::fs;


pub type Jobs = HashMap<String, Job>;

pub trait JobManager{
    /// Return a clone of self
    fn all(&self) -> Jobs;

    /// Filter all Jobs out that contain this string in their name
    fn filter_by_name<S>(&self, name: S) -> Jobs where S: Into<String>;
}

impl JobManager for Jobs{
    fn all(&self) -> Jobs{
        self.clone()
    }

    fn filter_by_name<S>(&self, name: S) -> Jobs where S: Into<String> {
        let name = name.into();
        self.iter()
            .filter(|&(_path, job)| job.paths.filename.to_lowercase().contains(name.to_lowercase().as_str()))
            .map(|(k, v)|(k.clone(), v.clone()))
            .collect()
    }
}




/// Get Blendfiles in ./tests/resources/blendfiles
fn get_blendfiles() -> Vec<PathBuf>{
    let resourcepath = get_resourcepath();
    let paths = fs::read_dir(resourcepath).unwrap();
    let mut vec = Vec::new();
    for entry in paths{
        let path = entry.expect("Unwrapping Entry failed").path();
        if path.is_file() {
            vec.push(path);
        }
    }
    vec
}



/// Put all jobs found by the get_blendfiles() function into a HashMap with the
/// path as key and the Job as value.
fn get_jobmap() -> Jobs{
    let mut h = Jobs::new();
    let paths = get_blendfiles();
    paths.iter()
         .for_each(|path|{
            match Job::from_blend(path){
                Ok(job) => {
                    h.insert(
                        path.to_str().unwrap().to_string(), 
                        job
                    );
                },
                Err(err) => println!("Error while creating job from path: {:?}, Error: {}", path, err)
            }
        });
    h
}

