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
            Command::Blender(ref c) => format!("{}", c.frame),
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
    pub frame: Frames,
    pub image_format: String,
    pub blendfile: Option<String>,
    pub outpath: Option<String>,
    pub command: Option<String>
    
}

/// Implement formatting for Blender command



impl BlenderCommand{
    /// Return a new Blender command for a single Frames
    pub fn new_single<S>(frame: usize, image_format: S) -> Self where S: Into<String>{
        BlenderCommand{
            frame: Frames::new_single(frame),
            image_format: image_format.into(),
            blendfile: None,
            outpath: None,
            command: None
        }
    }

    /// Return a new Blender command with a range
    pub fn new_range<S>(start: usize, end: usize, step: usize, image_format: S) -> Self where S: Into<String>{
        BlenderCommand{
            frame: Frames::new_range(start, end, step),
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
        self.frame.numbers.iter()
                          .map(|i| {
                                self.outpath.clone().unwrap()+"/"+&format!("{:06}", i)+"."+&self.image_format.to_lowercase()
                          })
                          .collect()
    }
}

/// Implement Formating for basic command
impl fmt::Display for BlenderCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Render {} ({})", self.frame, self.image_format)
    }
}

    
/// A Frames holds all frames for a given task.
///
/// Create them like this
/// ```
/// # extern crate bender_job;
/// # use bender_job::command::{Frames};
/// // For a single Frame
/// let f = Frames::new_single(121);
///
/// // For a range of Frames (1 to 250, with a step size of 1 frame)
/// let r = Frames::new_range(1, 250, 1);
/// 
/// // Both can be converted to command flags like this:
/// let single_command = f.to_flags();
/// assert_eq!(single_command, "-f 121");
///
/// let range_command = r.to_flags();
/// assert_eq!(range_command, "-s 1 -e 250");
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Frames{
    numbers:  FrameNumbers,
    filesizes: Vec<Option<usize>>,
    hashes:   Vec<Option<String>>,
    uploaded: Vec<bool>
}

impl Frames{
    /// Convert a Frames to the fitting command flags for a BlenderCommand. E.g. a
    /// Frames::new_single(121) would convert to the String "-f 121". A Frames with the
    /// start at frame 1, the end at frame 250 and a frame step of 1 would yield the
    /// String "-s 1 -e 250 -j 1"
    pub fn to_flags(&self) -> String{
        match self.numbers{
            FrameNumbers{start, end, step: 1} if start==end => {
                format!("-f {}", start)
            },
            FrameNumbers{start, end, step: 1} => {
                format!("-s {} -e {}", start, end)
            },
            FrameNumbers{start, end, step} => {
                format!("-s {} -e {} -j {}", start, end, step)
            }
        }
    }

    /// Create a new single Frames, where start and end are the same
    pub fn new_single(f: usize) -> Self{
        Frames{
            numbers: FrameNumbers{
                start: f, 
                end:   f, 
                step:  1
            },
            filesizes: vec![None; 1],
            hashes:    vec![None; 1],
            uploaded:  vec![false; 1]
        }
    }

    /// Create a new Frames from a start frame, a end frame and a frame step
    /// which indicates multiple frames for that Task
    pub fn new_range(start: usize, end: usize, step: usize) -> Self{
        let numbers = FrameNumbers{ start, end, step };
        let count = numbers.len();

        Frames{
            numbers,
            filesizes: vec![None; count],
            hashes:    vec![None; count],
            uploaded:  vec![false; count]
        }
    }

    /// Count the frames
    pub fn count(&self) -> usize{
        self.numbers.len()
    }

    /// Returns true if the specified framenumber is in self
    pub fn has_frame(&self, framenumber: usize) -> bool{
        self.numbers.iter()
                    .any(|i| i == framenumber)
    }

    /// Returns the filesize of the given frame in bytes if it has been \
    /// rendered. If the frame hasn't been rendered or is out of bounds, \
    /// return None
    pub fn get_filesize(&self, framenumber: usize) -> Option<usize>{
        match self.filesizes.get(framenumber){
            Some(option_filesize) => *option_filesize,
            None => None
        }
    }

    /// Returns the hash of the given frame as a String if it has been \
    /// rendered. If the frame hasn't been rendered or is out of bounds, \
    /// return None
    pub fn get_hash(&self, framenumber: usize) -> Option<String>{
        match self.hashes.get(framenumber){
            Some(option_hash) => option_hash.clone(),
            None => None
        }
    }

    /// Returns true if the given frame ihas been rendered. If the frame hasn't\
    /// been rendered or is out of bounds, return false
    pub fn get_uploaded(&self, framenumber: usize) -> bool{
        match self.uploaded.get(framenumber){
            Some(uploaded) => *uploaded,
            None => false
        }
    }

    /// Return true if the filsize for a given frame is set, return false \
    /// if not, or out of bounds
    pub fn is_filesize(&self, framenumber: usize) -> bool{
        match self.filesizes.get(framenumber){
            Some(option_filesize) => option_filesize.is_some(),
            None => false
        }
    }

    /// Return true if the hash for a given frame is set, return false \
    /// if not, or out of bounds
    pub fn is_hash(&self, framenumber: usize) -> bool{
        match self.hashes.get(framenumber){
            Some(option_hash) => option_hash.is_some(),
            None => false
        }
    }

    /// Return true if uploaded for a given frame is set, return false \
    /// if not, or out of bounds
    pub fn is_uploaded(&self, framenumber: usize) -> bool{
        match self.uploaded.get(framenumber){
            Some(uploaded) => *uploaded,
            None => false
        }
    }

    /// Return true if all filesizes have been set
    pub fn all_filesizes(&self) -> bool{
        self.numbers.iter()
                    .all(|i| self.filesizes[i].is_some())
    }

    /// Return true if all hashes have been set
    pub fn all_hashes(&self) -> bool{
        self.numbers.iter()
                    .all(|i| self.hashes[i].is_some())
    }

    /// Return true if all uploaded have been set
    pub fn all_uploaded(&self) -> bool{
        self.numbers.iter()
                    .all(|i| self.uploaded[i])
    }

    /// Return true if any of the filesizes has been set
    pub fn any_filesizes(&self) -> bool{
        self.numbers.iter()
                    .any(|i| self.filesizes[i].is_some())
    }

    /// Return true if any of the hashes has been set
    pub fn any_hashes(&self) -> bool{
        self.numbers.iter()
                    .any(|i| self.hashes[i].is_some())
    }

    /// Return true if any of the frames has been uploaded
    pub fn any_uploaded(&self) -> bool{
        self.numbers.iter()
                    .any(|i| self.uploaded[i])
    }
}



impl fmt::Display for Frames {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.numbers{
            FrameNumbers{start, end, step: 1} if start==end => {
                write!(f, "Frame {}", start)
            },
            FrameNumbers{start, end, step: 1} => {
                write!(f, "Frames {} to {}", start, end)
            },
            FrameNumbers{start, end, step} => {
                write!(f, "Frames {} to {} (Framestep: {})", start, end, step)
            }
        }
    }
}



/// Describes a Frames Range of Framenumbers, with a start, a end and a step size.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FrameNumbers{
    start: usize,
    end: usize,
    step: usize,
}


impl FrameNumbers {
    fn iter(&self) -> FrameNumbersIter {
        FrameNumbersIter {
            framenumbers: self,
            cur: self.start,
        }
    }

    fn len(&self) -> usize {
        self.iter().count()
    }
}


impl<'a> Iterator for FrameNumbersIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.cur <= self.framenumbers.end {
            let n = self.cur;
            self.cur += self.framenumbers.step;
            Some(n)
        } else {
            None
        }
    }
}


pub struct FrameNumbersIter<'a>{
    framenumbers: &'a FrameNumbers,
    cur: usize
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


#[cfg(test)]
mod frames {
    use super::*;

    #[test]
    fn single_frame_count() {
        let f = Frames::new_single(66);
        assert_eq!(f.count(), 1);
    }

    #[test]
    fn range_frame_count() {
        let f = Frames::new_range(1, 10, 1);
        assert_eq!(f.count(), 10);
    }

    #[test]
    fn stepped_range_frame_count() {
        let f = Frames::new_range(1, 10, 2);
        assert_eq!(f.count(), 5);
    }

    #[test]
    fn single_frame_flag() {
        let f = Frames::new_single(66);
        assert_eq!(f.to_flags(), "-f 66".to_string());
    }

    #[test]
    fn range_frame_flag() {
        let f = Frames::new_range(1, 10, 1);
        assert_eq!(f.to_flags(), "-s 1 -e 10".to_string());
    }

    #[test]
    fn stepped_range_frame_flag() {
        let f = Frames::new_range(1, 10, 2);
        assert_eq!(f.to_flags(), "-s 1 -e 10 -j 2".to_string());
    }

    #[test]
    fn single_frame_iter_count() {
        let f = Frames::new_single(66);
        assert_eq!(f.numbers.iter().count(), 1);
    }

    #[test]
    fn range_frame_iter_count() {
        let f = Frames::new_range(1, 10, 1);
        assert_eq!(f.numbers.iter().count(), 10);
    }

    #[test]
    fn stepped_range_frame_iter_count() {
        let f = Frames::new_range(1, 10, 2);
        assert_eq!(f.numbers.iter().count(), 5);
    }

    #[test]
    fn single_frame_iter_values() {
        let f = Frames::new_single(66);
        let v: Vec<usize> = f.numbers.iter().collect();
        assert_eq!(v[0], 66);
    }

    #[test]
    fn range_frame_iter_values() {
        let f = Frames::new_range(1, 10, 1);
        let v: Vec<usize> = f.numbers.iter().collect();
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);
        assert_eq!(v[3], 4);
        assert_eq!(v[4], 5);
        assert_eq!(v[5], 6);
        assert_eq!(v[6], 7);
        assert_eq!(v[7], 8);
        assert_eq!(v[8], 9);
        assert_eq!(v[9], 10);
    }

    #[test]
    fn stepped_range_frame_iter_values() {
        let f = Frames::new_range(1, 10, 2);
        let v: Vec<usize> = f.numbers.iter().collect();
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 3);
        assert_eq!(v[2], 5);
        assert_eq!(v[3], 7);
        assert_eq!(v[4], 9);
    }
}