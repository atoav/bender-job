//! The command module defines the Command enum, which is either a BasicCommand \
//! or a BlenderCommand. For details check the Command Enum documentation.

use ::*;
use reqwest::{header::USER_AGENT, multipart};
use std::thread;
use std::time::Duration;



// ===========================================================================
//                                 Command
// ===========================================================================


/// A command is a command line callable enum. There are currently two types of
/// commands: Basic and Blender.  
///
/// Creatre a basic command like this:
/// ```
/// # extern crate bender_job;
/// # use bender_job::command::{Command};
/// let c = Command::new("ls -a");
/// // Convert c to String and unwrap (a basic command can't fail to unwrap)
/// let command_string = c.to_string().unwrap();
/// assert_eq!(command_string, "ls -a".to_string())
/// ```
///
/// Some Commands (e.g. a BlenderCommand) have to be constructed first, in order to
/// allow the paths to be more flexible:
/// ```
/// # extern crate bender_job;
/// # use bender_job::command::{Command};
/// // Create a command for a single blender frame (121), to be rendered as PNG
/// let mut c = Command::new_blender_single(121, "PNG");
///
/// // Construct the command with a input and a output path
/// c.construct("some/blendfile.blend", "/data/render/here");
///
/// // Now we can get the constructed String with
/// c.to_string().unwrap();
/// ```
/// Note: if we would have forgotten to call construct() before converting
/// the command to string, the unwrap would have raised a panic
/// 
/// The above to_string() calls would result in two Strings
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Command{
    Basic(BasicCommand),
    Blender(BlenderCommand)
}



impl Command{

    /// Return a new Basic Command
    pub fn new<S>(command: S) -> Self where S: Into<String>{
        Command::Basic(BasicCommand::new(command.into()))
    }

    /// Return a new Blender Command for a single Frame
    pub fn new_blender_single<S>(f: usize, image_format: S) -> Self where S: Into<String>{
        Command::Blender(BlenderCommand::new_single(f, image_format.into()))
    }

    /// Return a new Blender Command with Range (Startframe, Endframe, Framestep)
    pub fn new_blender_range<S>(start: usize, end: usize, step: usize, image_format: S) -> Self where S: Into<String>{
        Command::Blender(BlenderCommand::new_range(start, end, step, image_format.into()))
    }

    /// Merge one Command into another
    pub fn merge(&mut self, other: &Self){
        let commands = (self, other);

        // Only unconstructed Blender commands from constructed ones
        if let (Command::Blender(this), Command::Blender(other)) = commands {
            this.merge(&other);
        }
    }

    /// Convert the Command to a String and return a Result<String> (Error if,
    /// construct() was needed and not called)
    pub fn to_string(&self) -> GenResult<String>{
        match self{
            Command::Basic(c) => c.to_string(),
            Command::Blender(c) => c.to_string()
        }
    }

    /// Return a string that represents the Command in essence
    pub fn short(&self) -> String{
        match self{
            Command::Blender(ref c) => c.frame.to_string(),
            Command::Basic(ref c) => {
                match c.to_string(){
                    Ok(b) => b.to_string(),
                    Err(_err) => "".to_string()
                }
            }
        }
    }

    /// Construct the Command (useful to update the paths on a different system)
    pub fn construct<S>(&mut self, input: S, output: S) where S: Into<String>{
        let input = input.into();
        let output = output.into();
        match self{
            Command::Basic(_) => (),
            Command::Blender(c) => c.construct(input, output)
        }
    }

    /// Returns true if the Command is a blender command
    pub fn is_blender(&self) -> bool{
        match self{
            Command::Basic(_) => false,
            Command::Blender(_) => true
        }
    }

    /// Returns true if the Command is constructed
    pub fn is_constructed(&self) -> bool{
        match self{
            Command::Basic(_) => true,
            Command::Blender(b) => b.is_constructed()
        }
    }


    /// Return true if all frames of the underlying BlenderCommand have a \
    /// filesize. If the command is _not_ a BlenderCommand, return Error.
    pub fn all_filesize(&self) -> GenResult<bool>{
        if let Command::Blender(blender_command)  = self{
            Ok(blender_command.frame.all_filesize())
        }else{
            Err(From::from("Couldn't check if all frames have a filesize, because the Command was not a BlenderCommand"))
        }
    }


    /// Return true if all frames of the underlying BlenderCommand have been \
    /// hashed. If the command is _not_ a BlenderCommand, return Error.
    pub fn all_hashed(&self) -> GenResult<bool>{
        if let Command::Blender(blender_command)  = self{
            Ok(blender_command.frame.all_hash())
        }else{
            Err(From::from("Couldn't check if all frames have a hash, because the Command was not a BlenderCommand"))
        }
    }

    /// Post the frame in self to flaskbender via http
    pub fn post_frames<S>(&self, bender_url: S) -> GenResult<Vec<reqwest::Response>> where S: Into<String>{
        let bender_url = bender_url.into();
        let mut v = Vec::new();

        match self{
            Command::Blender(ref blender_command) => {
                for (i, frame) in blender_command.frame.iter(){
                    let path = blender_command.path_for_frame(*i);

                    let form = multipart::Form::new()
                                    .text("filesize", frame.get_filesize().unwrap().to_string())
                                    .text("filehash", frame.get_hash().unwrap().to_string())
                                    .file("file", &*path)?;

                    let client = reqwest::Client::new();
                    let url    = reqwest::Url::parse(bender_url.as_str())?;
                    
                    let res = client.post(url)
                                    .header(USER_AGENT, "bender-worker")
                                    .multipart(form)
                                    .send()?;
                    v.push(res);
                }
                thread::sleep(Duration::from_millis(2000));
                Ok(v)
            },
            _ => Err(From::from("The Command was not a blender command"))
        }


    }

}



impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            Command::Basic(b) => {
                write!(f, "{}", b)
            },
            Command::Blender(b) => {
                write!(f, "{}", b)
            }
        }
    }
}





// ===========================================================================
//                               BasicCommand
// ===========================================================================


/// A basic command, that is basically just a command-line executable string
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BasicCommand{
    pub command: String
}



impl BasicCommand{

    /// Create a new basic command
    pub fn new<S>(command: S) -> Self where S: Into<String>{
        BasicCommand{
            command: command.into()
        }
    }

    /// Return a string representing the basic command
    pub fn to_string(&self) -> GenResult<String>{
        let c = self.command.clone();
        Ok(c)
    }

}




/// Implement Formating for basic command
impl fmt::Display for BasicCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.command)
    }
}







// ===========================================================================
//                              BlenderCommand
// ===========================================================================


/// This holds a blender command and allows for local construction of commands with
/// different paths for input and output
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BlenderCommand{
    pub frame: frames::Frames,
    pub image_format: String,
    pub blendfile: Option<String>,
    pub outpath: Option<String>,
    pub command: Option<String>
    
}



impl BlenderCommand{

    /// Return a new Blender command for a single Frames
    pub fn new_single<S>(frame: usize, image_format: S) -> Self where S: Into<String>{
        BlenderCommand{
            frame: frames::Frames::new_single(frame),
            image_format: image_format.into(),
            blendfile: None,
            outpath: None,
            command: None
        }
    }

    /// Return a new Blender command with a range
    pub fn new_range<S>(start: usize, end: usize, step: usize, image_format: S) -> Self where S: Into<String>{
        BlenderCommand{
            frame: frames::Frames::new_range(start, end, step),
            image_format: image_format.into(),
            blendfile: None,
            outpath: None,
            command: None
        }
    }

    /// Convert the command to String, return Error if Self::construct() hasn't been called before
    pub fn to_string(&self) -> GenResult<String>{
        match self.command{
            Some(ref command) => Ok(command.clone()),
            None => Err(From::from("Error: Couldn't convert Blender Command to_string(). Forgot to call construct() first?"))
        }
    }

    /// Construct the command with the given paths
    pub fn construct<S>(&mut self, blendfile: S, outpath: S) where S: Into<String>{
        self.blendfile = Some(blendfile.into());
        self.outpath = Some(outpath.into());
        let framestring = self.frame.to_flags();
        let out = self.outpath.clone().unwrap()+"/######."+&self.image_format.to_lowercase();
        self.command = Some(format!("blender -b --disable-autoexec {blendfile} -o {out} -F {format} {f}", 
            blendfile=self.blendfile.clone().unwrap(), 
            out=out, 
            format=self.image_format,
            f=framestring));
    }

    /// Merge one BlenderCommand into another based on its values
    pub fn merge(&mut self, other: &Self){
        self.frame.merge(&other.frame);

        if self.image_format != other.image_format { self.image_format = other.image_format.clone(); }

        if self.blendfile.is_none() && other.blendfile.is_some(){
            self.blendfile = other.blendfile.clone();
        }

        if self.outpath.is_none() && other.outpath.is_some(){
            self.outpath = other.outpath.clone();
        }

        if self.command.is_none() && other.command.is_some(){
            self.command = other.command.clone();
        }
    }

    /// Return true if the blendfile has been constructed
    pub fn is_constructed(&self) -> bool{
        self.blendfile.is_some()
    }

    /// Return a Vector of PathBuf where each PathBuf is the Path of one frame generated by the command
    pub fn renderpaths(&self) -> Vec<PathBuf>{
        self.frame.iter()
            .map(|(framenumber, _)| {
                self.path_for_frame(*framenumber)
            })
            .collect()
    }

    /// Return the path for a constructed frame
    pub fn path_for_frame(&self, framenumber: usize) -> PathBuf{
        let s = self.outpath.clone().unwrap()+"/"+&format!("{:06}", framenumber)+"."+&self.image_format.to_lowercase();
        PathBuf::from(s.clone())
    }

    /// Read and set the filesizes for all rendered frames
    pub fn get_frame_filesizes(&mut self) -> GenResult<()>{
        // Collect the paths where the frame should be rendered first
        let mut framepaths = HashMap::new(); 
        self.frame.iter()
                  .for_each(|(i, _)|{
                    framepaths.insert(*i, self.path_for_frame(*i));
                  });

        let _v: GenResult<Vec<usize>> = 
        self.frame.iter_mut()
                  .map(|(i, frame)|{
                    match framepaths.get(i){
                        Some(path) => {
                            if path.exists(){
                                let file = std::fs::File::open(&path)?;
                                Ok(frame.filesize_from_file(file)?)
                            }else{
                                let message = format!("Couldn't filesize Frame {}, because the file doesn't exist: {}", 
                                    i, &path.to_string_lossy());
                                Err(From::from(message))
                            }
                        },
                        None => {
                            let message = format!("Couldn't filesize Frame {} because the index is out of bounds", 
                                    i);
                                Err(From::from(message))
                        }
                    }
                  })
                  .collect();
        Ok(())
    }

    /// Generate and set the hashes for all rendered frames
    pub fn get_frame_hashes(&mut self) -> GenResult<()>{
        // Collect the paths where the frame should be rendered first
        let mut framepaths = HashMap::new(); 
        self.frame.iter()
                  .for_each(|(i, _)|{
                    framepaths.insert(*i, self.path_for_frame(*i));
                  });

        let _v: GenResult<Vec<String>> = 
        self.frame.iter_mut()
                  .map(|(i, frame)|{
                    match framepaths.get(i){
                        Some(path) => {
                            if path.exists(){
                                let file = std::fs::File::open(&path)?;
                                Ok(frame.hash_from_file(file)?)
                            }else{
                                let message = format!("Couldn't hash Frame {}, because the file doesn't exist: {}", 
                                    i, &path.to_string_lossy());
                                Err(From::from(message))
                            }
                        },
                        None => {
                            let message = format!("Couldn't hash Frame {} because the index is out of bounds", 
                                    i);
                                Err(From::from(message))
                        }
                    }
                  })
                  .collect();
        Ok(())
    }

    /// Set a rendered Frame's uploaded flag
    pub fn set_uploaded(&mut self, framenumber: usize) -> GenResult<()>{
        self.frame.set_uploaded(framenumber)
    }

    /// Set all frames to uploaded
    pub fn set_all_uploaded(&mut self) -> GenResult<()>{
        self.frame.iter_mut()
                  .for_each(|(_, frames)| frames.set_uploaded());
        Ok(())
    }

}



/// Implement Formating for BlenderCommand
impl fmt::Display for BlenderCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Render {} ({})", self.frame.to_string(), self.image_format)
    }
}

    










// ===========================================================================
//                                 UNIT TESTS
// ===========================================================================

#[cfg(test)]
mod command {
    use super::*;

    #[test]
    fn basic() {
        let r = Command::new("ls -a");
        assert_eq!(r.to_string().unwrap(), "ls -a".to_string());
    }
}