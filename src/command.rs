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
        match commands{
            (Command::Blender(a), Command::Blender(b)) => {
                if !(a.is_constructed() && !b.is_constructed()){
                    *a = b.clone();
                }
            },
            _ => ()
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
            Command::Blender(ref c) => format!("{}", c.frame),
            Command::Basic(ref c) => {
                match c.to_string(){
                    Ok(b) => format!("{}", b),
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
    pub frame: Frame,
    pub image_format: String,
    pub blendfile: Option<String>,
    pub outpath: Option<String>,
    pub command: Option<String>
    
}

/// Implement formatting for Blender command



impl BlenderCommand{
    /// Return a new Blender command for a single Frame
    pub fn new_single<S>(frame: usize, image_format: S) -> Self where S: Into<String>{
        BlenderCommand{
            frame: Frame::new_single(frame),
            image_format: image_format.into(),
            blendfile: None,
            outpath: None,
            command: None
        }
    }

    /// Return a new Blender command with a range
    pub fn new_range<S>(start: usize, end: usize, step: usize, image_format: S) -> Self where S: Into<String>{
        BlenderCommand{
            frame: Frame::new_range(start, end, step),
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
        match self.blendfile{
            Some(_) => true,
            None => false
        }
    }
}

/// Implement Formating for basic command
impl fmt::Display for BlenderCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.is_constructed(){
            true => {
                write!(f, "Render {} ({})", self.frame, self.image_format)
            },
            false => {
                write!(f, "Render {} ({})", self.frame, self.image_format)
            }
        }
    }
}

    
/// A Frame holds either a `Single(usize)` or a `Range(Range{start: usize, end: usize, step: usize})`.
/// These describe either a single frame, or a range of frames with a certain step size.
///
/// Create them like this
/// ```
/// # extern crate bender_job;
/// # use bender_job::command::{Frame};
/// // For a single Frame
/// let f = Frame::new_single(121);
///
/// // For a range of Frames (1 to 250, with a step size of 1 frame)
/// let r = Frame::new_range(1, 250, 1);
/// 
/// // Both can be converted to command flags like this:
/// let single_command = f.to_flags();
/// assert_eq!(single_command, "-f 121");
///
/// let range_command = r.to_flags();
/// assert_eq!(range_command, "-s 1 -e 250 -j 1");
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Frame{
    Single(usize),
    Range(Range)
}

impl Frame{
    /// Convert a Frame to the fitting command flags for a BlenderCommand. E.g. a
    /// Frame::Single(121) would convert to the String "-f 121". A Range with the
    /// start at frame 1, the end at frame 250 and a frame step of 1 would yield the
    /// String "-s 1 -e 250 -j 1"
    pub fn to_flags(&self) -> String{
        match self{
            Frame::Single(f) => {
                format!("-f {}", f)
            },
            Frame::Range(r) => {
                format!("-s {} -e {} -j {}", r.start, r.end, r.step)
            }
        }
    }

    /// Create a new `Frame::Single(usize)` enum from a frame number
    pub fn new_single(f: usize) -> Self{
        Frame::Single(f)
    }

    /// Create a new `Frame::Range(Range)` enum from a start frame, a end frame
    /// and a frame step
    pub fn new_range(start: usize, end: usize, step: usize) -> Self{
        Frame::Range(Range{ start, end, step})
    }

    /// Count the frames independed of type (Single, Range)
    pub fn count(&self) -> usize{
        match self{
            Frame::Single(f) => *f,
            Frame::Range(r) => (r.end-r.start+1)/r.step
        }
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            Frame::Single(u) => {
                write!(f, "Frame {}", u)
            },
            Frame::Range(r) => {
                if r.step == 1{
                    write!(f, "Frames {} to {}", r.start, r.end)
                }else{
                    write!(f, "Frames {} to {} (Framestep: {})", r.start, r.end, r.step)
                }
            }
        }
    }
}

/// Describes a Range of Frames, with a start, a end and a step size. I used by the Frame enum
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Range{
    start: usize,
    end: usize,
    step: usize
}



// ================================ TEST RENDER ================================
#[cfg(test)]
mod command {
    use ::*;
    #[test]
    fn basic() {
        let r = Command::new("ls -a");
        assert_eq!(r.to_string().unwrap(), "ls -a".to_string());
    }
}