use ::*;
use std::io::prelude::Read;
use std::collections::BTreeMap;
use blake2::{Blake2b, Digest};


/// A Frames holds all frames for a given task.
/// It implements the FrameMap trait in order to achieve it's functionality
///
/// Create them like this
/// ```
/// # extern crate bender_job;
/// # use bender_job::frames::{Frames, FrameMap};
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
pub type Frames = BTreeMap<usize, Frame>;


pub trait FrameMap{

    /// Create a new single Frames, where start and end are the same
    fn new_single(framenumber: usize) -> Self;

    /// Create a new Frames from a start frame, a end frame and a frame step
    /// which indicates multiple frames for that Task
    fn new_range(start: usize, end: usize, step: usize) -> Self;

    /// Return the number of frames
    fn count(&self) -> usize;

    /// Return true if self is just a single Frame
    fn is_single(&self) -> bool;

    /// Return start as usize
    fn start(&self) -> usize;

    /// Return end as usize
    fn end(&self) -> usize;

    /// Return step as usize
    fn step(&self) -> usize;

    /// Return true if the given Frame is contained
    fn has_frame(&self, framenumber: usize) -> bool;

    /// Return a string that describes the Frames
    fn to_string(&self) -> String;

    /// Convert a Frames to the fitting command flags for a BlenderCommand. E.g. a
    /// Frames::new_single(121) would convert to the String "-f 121". A Frames with the
    /// start at frame 1, the end at frame 250 and a frame step of 1 would yield the
    /// String "-s 1 -e 250" and the same range with a step of 2 would result in
    /// a String of "-s 1 -e 250 -j 2"
    fn to_flags(&self) -> String;

    /// Set the filesize for a given frame, return Ok if this suceeds and 
    /// Err if this fails
    fn set_filesize(&mut self, framenumber: usize, filesize: usize) -> GenResult<()>;

    /// Set the hash String for a given frame to a String. Return Ok if the \
    /// set sucessfully, return an Err if out of bounds or not contained
    fn set_hash<S>(&mut self, framenumber: usize, hash: S) -> GenResult<()> where S: Into<String>;

    /// Set the uploaded flag for a frame to true. Return Ok if this succeeds,
    /// return Error if not
    fn set_uploaded(&mut self, framenumber: usize) -> GenResult<()>;

    /// Returns the filesize of the given frame in bytes if it has been \
    /// rendered. If the frame hasn't been rendered or is out of bounds, \
    /// return None
    fn get_filesize(&self, framenumber: usize) -> Option<usize>;

    /// Returns the hash of the given frame as a String if it has been \
    /// rendered. If the frame hasn't been rendered or is out of bounds, \
    /// return None
    fn get_hash(&self, framenumber: usize) -> Option<&str>;

    /// Returns true if the given frame ihas been rendered. If the frame hasn't\
    /// been rendered or is out of bounds, return false
    fn get_uploaded(&self, framenumber: usize) -> bool;

    /// Return true if the filsize for a given frame is set, return false \
    /// if not, or out of bounds
    fn is_filesize(&self, framenumber: usize) -> bool;

    /// Return true if the hash for a given frame is set, return false \
    /// if not, or out of bounds
    fn is_hash(&self, framenumber: usize) -> bool;

    /// Return true if uploaded for a given frame is set, return false \
    /// if not, or out of bounds
    fn is_uploaded(&self, framenumber: usize) -> bool;

    /// Return true if all filesizes have been set
    fn all_filesize(&self) -> bool;

    /// Return true if all hashes have been set
    fn all_hash(&self) -> bool;

    /// Return true if all uploaded have been set
    fn all_uploaded(&self) -> bool;

    /// Return true if any of the filesizes has been set
    fn any_filesize(&self) -> bool;

    /// Return true if any of the hashes has been set
    fn any_hash(&self) -> bool;

    /// Return true if any of the frames has been uploaded
    fn any_uploaded(&self) -> bool;
}





impl FrameMap for Frames{

    fn new_single(framenumber: usize) -> Self{
        let mut new = Self::new();
        new.insert(framenumber, Frame::new());
        new
    }

    fn new_range(start: usize, end: usize, step: usize) -> Self{
        let mut new = Self::new();
        for f in (start..=end).step_by(step){
            new.insert(f, Frame::new());
        }
        new
    }

    fn count(&self) -> usize{
        self.len()
    }

    fn is_single(&self) -> bool{
        self.count() == 1
    }

    fn start(&self) -> usize{
        *self.keys().min().unwrap()
    }

    fn end(&self) -> usize{
        *self.keys().max().unwrap()
    }

    fn step(&self) -> usize{
        if self.is_single(){
            1
        }else{
            let v: Vec<usize> = self.keys().cloned().take(2).collect();
            v[1] - v[0]
        }
    }

    fn has_frame(&self, framenumber: usize) -> bool{
        self.iter().any(|(number, _)| number == &framenumber)
    }

    fn to_string(&self) -> String{
        if self.is_single(){
            format!("Frame {}", self.start())
        }else{
            let step = self.step();
            match step{
                1 => format!("Frames {} to {}", self.start(), self.end()),
                _ => format!("Frames {} to {} (step: {})", self.start(), self.end(), step)
            }
        }
    }

    fn to_flags(&self) -> String{
        if self.is_single(){
            format!("-f {}", self.start())
        }else{
            let step = self.step();
            match step{
                1 => format!("-s {} -e {}", self.start(), self.end()),
                _ => format!("-s {} -e {} -j {}", self.start(), self.end(), step)
            }
        }
    }

    fn set_filesize(&mut self, framenumber: usize, filesize: usize) -> GenResult<()>{
        match self.get_mut(&framenumber){
            Some(frame) => {
                frame.set_filesize(filesize);
                Ok(())
            },
            None => {
                let errmessage = format!("Error: Couldn't set_filesize({}) for frame {}. Frame not contained in this Task.", filesize, framenumber);
                Err(From::from(&*errmessage))
            }
        }
    }

    fn set_hash<S>(&mut self, framenumber: usize, hash: S) -> GenResult<()> where S: Into<String>{
        let hash = hash.into();

        match self.get_mut(&framenumber){
            Some(frame) => {
                frame.set_hash(hash);
                Ok(())
            },
            None => {
                let errmessage = format!("Error: Couldn't set_hash({}) for frame {}. Frame not contained in this Task.", hash, framenumber);
                Err(From::from(&*errmessage))
            }
        }
    }

    fn set_uploaded(&mut self, framenumber: usize) -> GenResult<()>{
        match self.get_mut(&framenumber){
            Some(frame) => {
                frame.set_uploaded();
                Ok(())
            },
            None => {
                let errmessage = format!("Error: Couldn't set_uploaded() for frame {}. Frame not contained in this Task.", framenumber);
                Err(From::from(&*errmessage))
            }
        }
    }

    fn get_filesize(&self, framenumber: usize) -> Option<usize>{
        match self.get(&framenumber){
            Some(frame) => frame.filesize,
            None => None
        }
    }

    fn get_hash(&self, framenumber: usize) -> Option<&str>{
        match self.get(&framenumber){
            Some(frame) => frame.get_hash(),
            None => None
        }
    }

    fn get_uploaded(&self, framenumber: usize) -> bool{
        match self.get(&framenumber){
            Some(frame) => frame.uploaded,
            None => false
        }
    }

    fn is_filesize(&self, framenumber: usize) -> bool{
        match self.get(&framenumber){
            Some(frame) => frame.filesize.is_some(),
            None => false
        }
    }

    fn is_hash(&self, framenumber: usize) -> bool{
        match self.get(&framenumber){
            Some(frame) => frame.hash.is_some(),
            None => false
        }
    }

    fn is_uploaded(&self, framenumber: usize) -> bool{
        match self.get(&framenumber){
            Some(frame) => frame.uploaded,
            None => false
        }
    }

    fn all_filesize(&self) -> bool{
        self.iter().all(|(_, frame)| frame.is_filesize())
    }

    fn all_hash(&self) -> bool{
        self.iter().all(|(_, frame)| frame.is_hash())
    }

    fn all_uploaded(&self) -> bool{
        self.iter().all(|(_, frame)| frame.is_uploaded())
    }

    fn any_filesize(&self) -> bool{
        self.iter().any(|(_, frame)| frame.is_filesize())
    }

    fn any_hash(&self) -> bool{
        self.iter().any(|(_, frame)| frame.is_hash())
    }

    fn any_uploaded(&self) -> bool{
        self.iter().any(|(_, frame)| frame.is_uploaded())
    }

}




// ===========================================================================
//                                  FRAME
// ===========================================================================

/// The Frame struct holds a single frames data and is used by the \
/// `frames::Frames` type in conjunction with the `frames::FrameMap` \
/// trait. The data it holds is focused on a _rendered_ frame:
/// - filesize in bytes
/// - hash is the Blake2b result of the rendered frame
/// - uploaded is a flag that signifies a sucessful upload
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Frame{
    filesize: Option<usize>,
    hash: Option<String>,
    uploaded: bool
}




impl Frame{
    /// Create a new Frame struct with the default configuration
    pub fn new() -> Self{
        Frame::default()
    }

    /// Return a default Frame struct, without filesize, hash and uploaded flag
    pub fn default() -> Self{
        Frame{
            filesize: None,
            hash: None,
            uploaded: false
        }
    }

    /// Set the Frame's filesize
    pub fn set_filesize(&mut self, filesize: usize){
        self.filesize = Some(filesize)
    }

    /// Set the Frame's hash to the supplied String
    pub fn set_hash<S>(&mut self, hash: S) where S: Into<String>{
        let hash = hash.into();
        self.hash = Some(hash)
    }

    /// Set the Frame's upload flag to true
    pub fn set_uploaded(&mut self){
        self.uploaded = true
    }

    /// Get a the Frame's filesize
    pub fn get_filesize(&self) -> Option<usize>{
        match self.filesize{
            Some(s) => Some(s),
            None => None
        }
    }

    /// Get a Option<&str> of the Frame's hash 
    pub fn get_hash(&self) -> Option<&str>{
        match self.hash{
            Some(ref h) => Some(&*h),
            None => None
        }
    }

    /// Return true if the Frame's filesize is set
    pub fn is_filesize(&self) -> bool{
        self.filesize.is_some()
    }

    /// Return true if the Frame's hash is set
    pub fn is_hash(&self) -> bool{
        self.hash.is_some()
    }

    /// Return true if the Frame's upload flag is set
    pub fn is_uploaded(&self) -> bool{
        self.uploaded
    }

    /// Set the filesize from the Frame's file. This takes anything that \
    /// implements the Read trait. If the read doesn't error, return the \
    /// resulting filesize. In practice this should use the Frame's \
    /// file after rendering has been finished.
    ///
    /// ```
    /// use bender_job::frames::Frame;
    ///
    /// // Create a new default Frame without Filesize
    /// let mut frame = Frame::new();
    ///
    /// // Create some Bytes. This could also be something like 
    /// // let f = File::open("frame_0001.png")?;
    /// let f = "12345678".as_bytes();
    ///
    /// // The function reads the bytes from the Reader, sets frame.filesize
    /// // to the bytes it read, and returns that number as a result
    /// let result = frame.filesize_from_file(f);
    ///
    /// assert_eq!(result.unwrap(), 8);
    /// ```
    pub fn filesize_from_file<R: Read>(&mut self, mut reader: R) -> GenResult<usize>{
        let mut buffer = Vec::new();
        self.set_filesize(reader.read_to_end(&mut buffer)?);
        Ok(self.filesize.unwrap())
    }

    /// Generate and set the hash from the Frame's file. This uses the Blake2b\
    /// cryptographic hash function and mainly acts as a checksum for the server.
    /// This method takes anything that implements the Read trait. If the \
    /// read doesn't error, return the resulting filesize. In practice this\
    /// should use the Frame's file after rendering has been finished.
    ///
    /// ```
    /// use bender_job::frames::Frame;
    ///
    /// // Create a new default Frame without Hash
    /// let mut frame = Frame::new();
    ///
    /// // Create some Bytes. This could also be something like 
    /// // let f = File::open("frame_0001.png")?;
    /// let f = "12345678".as_bytes();
    ///
    /// // The function reads the bytes from the Reader, sets frame.hash
    /// // to the result of Blake2b, and returns that hash as a result
    /// let result = frame.hash_from_file(f);
    ///
    /// assert_eq!(result.unwrap(), "f5560c3296de4e0ef868574bf96fc778bc580931a8cae2d2631de27ba055db1be2afd769d658c684d8bc5ee0c1b2a7583ec862d5e994b806c6fa2ab4d54cd7f4".to_string());
    /// ```
    pub fn hash_from_file<R: Read>(&mut self, mut reader: R) -> GenResult<String>{
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let mut hasher = Blake2b::new();
        hasher.input(buffer);
        self.set_hash(format!("{:x}", hasher.result()));
        Ok(self.hash.clone().unwrap())
    }

}







// ===========================================================================
//                                UNIT TESTS
// ===========================================================================




// --------------------------------- FRAME -----------------------------------
#[cfg(test)]
mod frame {
    use super::*;

    #[test]
    fn basic() {
        let f = Frame::new();
        assert_eq!(f.filesize, None);
        assert_eq!(f.hash, None);
        assert_eq!(f.uploaded, false);
    }

    #[test]
    fn set_get_filesize() {
        let mut f = Frame::new();
        assert_eq!(f.filesize, None);
        assert_eq!(f.hash, None);
        assert_eq!(f.uploaded, false);

        f.set_filesize(1001);
        assert_eq!(f.filesize, Some(1001));
    }

    #[test]
    fn set_get_hash() {
        let mut f = Frame::new();
        assert_eq!(f.filesize, None);
        assert_eq!(f.hash, None);
        assert_eq!(f.uploaded, false);

        f.set_hash("hashhh");
        assert_eq!(f.hash, Some("hashhh".to_string()));

        f.set_uploaded();
        assert_eq!(f.uploaded, true);
    }

    #[test]
    fn set_get_uploaded() {
        let mut f = Frame::new();
        assert_eq!(f.filesize, None);
        assert_eq!(f.hash, None);
        assert_eq!(f.uploaded, false);

        f.set_uploaded();
        assert_eq!(f.uploaded, true);
    }

    #[test]
    fn filesize_from_file() {
        let mut f = Frame::new();
        let b = "12345678".as_bytes();
        let x = f.filesize_from_file(b);

        assert_eq!(x.unwrap(), 8);
        assert_eq!(f.get_filesize().unwrap(), 8);
    }

    #[test]
    fn hash_from_file() {
        let mut f = Frame::new();
        let b = "12345678".as_bytes();
        let x = f.hash_from_file(b);

        println!("{:#?}",x);
        assert_eq!(x.unwrap(), "f5560c3296de4e0ef868574bf96fc778bc580931a8cae2d2631de27ba055db1be2afd769d658c684d8bc5ee0c1b2a7583ec862d5e994b806c6fa2ab4d54cd7f4".to_string());
        assert_eq!(f.get_hash().unwrap(), "f5560c3296de4e0ef868574bf96fc778bc580931a8cae2d2631de27ba055db1be2afd769d658c684d8bc5ee0c1b2a7583ec862d5e994b806c6fa2ab4d54cd7f4");
    }
}





// --------------------------------- FRAMES -----------------------------------
#[cfg(test)]
mod frames {
    use super::*;

    #[test]
    fn is_single() {
        let f = frames::Frames::new_single(66);
        assert!(f.is_single());
    }

    #[test]
    fn is_not_single() {
        let f = frames::Frames::new_range(1, 10, 1);
        assert!(!f.is_single());
    }

    #[test]
    fn count_single() {
        let f = frames::Frames::new_single(66);
        assert_eq!(f.count(), 1);
    }

    #[test]
    fn count_range() {
        let f = frames::Frames::new_range(1, 10, 1);
        assert_eq!(f.count(), 10);
    }

    #[test]
    fn count_stepped_range() {
        let f = frames::Frames::new_range(1, 10, 2);
        assert_eq!(f.count(), 5);
    }

    #[test]
    fn start_single() {
        let f = frames::Frames::new_single(66);
        assert_eq!(f.start(), 66);
    }

    #[test]
    fn end_single() {
        let f = frames::Frames::new_single(66);
        assert_eq!(f.end(), 66);
    }

    #[test]
    fn step_single() {
        let f = frames::Frames::new_single(66);
        assert_eq!(f.step(), 1);
    }

    #[test]
    fn start_range() {
        let f = frames::Frames::new_range(1, 10, 1);
        assert_eq!(f.start(), 1);
    }

    #[test]
    fn end_range() {
        let f = frames::Frames::new_range(1, 10, 1);
        assert_eq!(f.end(), 10);
    }

    #[test]
    fn step_range() {
        let f = frames::Frames::new_range(1, 10, 1);
        assert_eq!(f.step(), 1);
    }

    #[test]
    fn start_stepped_range() {
        let f = frames::Frames::new_range(1, 10, 2);
        assert_eq!(f.start(), 1);
    }

    #[test]
    fn end_stepped_range() {
        let f = frames::Frames::new_range(1, 10, 2);
        assert_eq!(f.end(), 9);
    }

    #[test]
    fn step_stepped_range() {
        let f = frames::Frames::new_range(1, 10, 2);
        assert_eq!(f.count(), 5);
        assert_eq!(f.step(), 2);
    }

    #[test]
    fn step_stepped_ranges() {
        for i in 1..1000{
            let f = frames::Frames::new_range(1, 2000, i);
            assert_eq!(f.step(), i);
        }
    }

    #[test]
    fn all_filesize() {
        let mut f = frames::Frames::new_range(1, 10, 1);
        f.iter_mut().for_each(|(_, frame)| frame.set_filesize(9001));
        assert!(f.any_filesize());
        assert!(f.all_filesize());
    }

    #[test]
    fn all_hash() {
        let mut f = frames::Frames::new_range(1, 10, 1);
        f.iter_mut().for_each(|(_, frame)| frame.set_hash("foo"));
        assert!(f.any_hash());
        assert!(f.all_hash());
    }

    #[test]
    fn all_uploaded() {
        let mut f = frames::Frames::new_range(1, 10, 1);
        f.iter_mut().for_each(|(_, frame)| frame.set_uploaded());
        assert!(f.any_uploaded());
        assert!(f.all_uploaded());
    }

    #[test]
    fn any_filesize() {
        let mut f = frames::Frames::new_range(1, 10, 1);
        f.iter_mut().take(1).for_each(|(_, frame)| frame.set_filesize(9001));
        assert!(f.any_filesize());
        assert_eq!(f.all_filesize(), false);
    }

    #[test]
    fn any_hash() {
        let mut f = frames::Frames::new_range(1, 10, 1);
        f.iter_mut().take(1).for_each(|(_, frame)| frame.set_hash("foo"));
        assert!(f.any_hash());
        assert_eq!(f.all_hash(), false);
    }

    #[test]
    fn any_uploaded() {
        let mut f = frames::Frames::new_range(1, 10, 1);
        f.iter_mut().take(1).for_each(|(_, frame)| frame.set_uploaded());
        assert!(f.any_uploaded());
        assert_eq!(f.all_uploaded(), false);
    }



}

