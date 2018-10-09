use ::*;
use common::path::get_resourcepath;
use std::fs;



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




fn get_jobmap() -> HashMap<String, Job>{
    let mut h = HashMap::new();
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