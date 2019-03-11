//! The command module defines the Command enum, which is either a BasicCommand \
//! or a BlenderCommand. For details check the Command Enum documentation.

use ::*;
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
        if let (Command::Blender(a), Command::Blender(b)) = commands {
            if b.is_constructed() || !a.is_constructed(){
                *a = b.clone();
            }
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
            Command::Blender(ref c) => format!("{}", c.frame.to_string()),
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





/// A basic command, that is basically just a command-line executable string
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BasicCommand{
    pub command: String
}

impl BasicCommand{
    pub fn new<S>(command: S) -> Self where S: Into<String>{
        BasicCommand{
            command: command.into()
        }
    }

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

    /// Convert the command to String, return Error if Self::construct() hasn't been called before
    pub fn to_string(&self) -> GenResult<String>{
        match self.command{
            Some(ref command) => Ok(command.clone()),
            None => Err(From::from("Error: Couldn't convert Blender Command to_string(). Forgot to call construct() first?"))
        }
    }

    pub fn is_constructed(&self) -> bool{
        self.blendfile.is_some()
    }

    /// Return a Vector of Strings where each String is the Path of one frame generated by the command
    pub fn renderpaths(&self) -> Vec<String>{
        self.frame.iter()
            .map(|(i, _)| {
                self.outpath.clone().unwrap()+"/"+&format!("{:06}", i)+"."+&self.image_format.to_lowercase()
            })
            .collect()
    }
}

/// Implement Formating for basic command
impl fmt::Display for BlenderCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Render {} ({})", self.frame.to_string(), self.image_format)
    }
}

    









// ================================ TEST RENDER ================================
#[cfg(test)]
mod command {
    use super::*;

    #[test]
    fn basic() {
        let r = Command::new("ls -a");
        assert_eq!(r.to_string().unwrap(), "ls -a".to_string());
    }
}