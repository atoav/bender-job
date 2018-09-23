use ::*;


/// A JobPath Struct holds all path-related data for the Job
/// It can be created from a uploadfolder
/// ```ignore
/// use bender_job::JobPaths;
/// let j = JobPaths::from_uploadfolder("/data/blendfiles/5873c0033e78b222bec2cb2a221487cf");
/// ```
/// or by deserializing a `data.json`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JobPaths{
    pub upload:    String,
    pub data:      String,
    pub blend:     String,
    pub frames:    String,
    pub filename:  String
}

impl JobPaths{

    /// You can create a JobPath via `JobPaths::from_uploadfolder`
    pub fn from_uploadfolder<S>(p: S) -> Self where S: Into<String>{
        // lets say we have a path called "/data/blendfiles/5873c0033e78b222bec2cb2a221487cf"
        let s = p.into();
        // Extract the id
        let id = PathBuf::from(&s);
        let id = id.file_name().expect("Error when aquiring id from path");
        // Create a path to "/data/blendfiles/5873c0033e78b222bec2cb2a221487cf/data.json"
        let mut data = PathBuf::from(&s);
        data.push("data.json");
        // Find a blendfile in the uploadfolder
        // e.g. "/data/blendfiles/5873c0033e78b222bec2cb2a221487cf/foo.blend"
        let blend = Self::first_blend(&s[..]).expect("Error: no blendfile in the directory");
        // Return frames folder at "/data/frames/5873c0033e78b222bec2cb2a221487cf"
        let mut frames = PathBuf::from(&s);
        frames.pop();
        frames.pop();
        frames.push("frames");
        frames.push(id);
        // Return filename of the blend
        let filename = blend.clone();
        let filename = filename.file_name().unwrap();

        JobPaths{
            upload: s.to_owned(),
            data: data.into_os_string().into_string().unwrap(),
            blend: blend.into_os_string().into_string().unwrap(),
            frames: frames.into_os_string().into_string().unwrap(),
            filename: filename.to_os_string().into_string().unwrap()
        }
    }

    /// Returns the ID used in the uploaddirectory by returning the last element of the upload path
    pub fn get_id(&self) -> String{
        let id = PathBuf::from(&self.upload[..]);
        id.file_name().unwrap().to_os_string().into_string().unwrap()
    }


    /// Returns a Vector of files with .blend extension found in a directory `p`
    pub fn find_blends<S>(p: S) -> Vec<PathBuf> where S: Into<String>{
        let path = &p.into()[..];
        let mut matches = Vec::new();
        // Search all files in path, push matches to vec
        for direntry in fs::read_dir(&path).expect(&format!("Couldn't read directory for {}", &path)[..]){
            let dirpath = direntry.unwrap().path();
            match dirpath.extension(){
                Some(e) => {
                    if e == "blend"{
                        matches.push(dirpath.clone());
                    }
                },
                None => ()
            }
        }
        matches
    }

    /// Return the first file with a .blend extension found in a directory `p`
    pub fn first_blend<S>(p: S) -> Option<PathBuf> where S: Into<String>{
        let mut matches = Self::find_blends(&p.into()[..]);
        if !matches.is_empty(){
            Some(matches.remove(0))
        } else {
            None
        }
    }

}

/// String formatting for JobPaths
impl fmt::Display for JobPaths {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let st = &format!("[JobPaths] 
├╴[upload:   \"{}\"]  
├╴[data:     \"{}\"]  
├╴[blend:    \"{}\"]  
├╴[frames:   \"{}\"]  
└╴[filename: \"{}\"]", 
            self.upload, self.data, self.blend, self.frames, self.filename)[..];
        fmt.write_str(st)?;
        Ok(())
    }
}