use ::*;




/// A Task is what gets passed from watchdog to qu and from qu to worker. Tasks
/// are essentially atomic units of work, broken down so they can be fairly managed
/// by the queue.
///
/// This is basically a wrapper around a command, that allows us to keep track of
/// a Tasks status, its start and end times etc. It consists of:
/// - a [Status](task/enum.Status.html) which manages the States of a Task and the allowed transitions between it \
/// (e.g. a finished task cannot be aborted, a errored task cannot start etc.)
/// - a [JobTime](jobtime/struct.JobTime.html) which allows to keep track of _when_ a certain state change has occured, as \
/// well as the calculation of durations (the same construct is used for [Job](struct.Job.html))
/// - a [Command](command/enum.Command.html) which allows to abstract CLI commands to be executed on the worker machines \
/// in such way, that we don't need to know input and output paths beforehand
/// 
/// Construct a new basic task like this:
/// ```
/// # extern crate bender_job;
/// use bender_job::Task;
/// 
/// // Create a basic task that lists files
/// let basic_task = Task::new_basic("ls -a");
///
/// // Create a blender task for a single frame (121. with PNG as image format)
/// let mut single_frame_task = Task::new_blender_single(121, "PNG");
///
/// // Create a blender task for a range of frames (1 to 250, with a step size of 1)
/// let mut range_frame_task = Task::new_blender_range(1, 250, 1, "PNG");
///
/// // Tasks with a blender command must be constructed with paths before usage. 
/// single_frame_task.construct("my/blend/file.blend", "some/out/folder/####.png");
/// range_frame_task.construct("my/blend/file.blend", "some/out/folder/####.png");
///
/// // Get the constructed tasks (unwrap fails, if you didn't construct the blender tasks before):
/// let basic_command = basic_task.to_string().unwrap();
/// let single_frame_command = single_frame_task.to_string().unwrap();
/// let range_frame_command = range_frame_task.to_string().unwrap();
///
/// // This yields following strings as a result:
/// assert_eq!(basic_command, "ls -a".to_string());
/// assert_eq!(single_frame_command, "blender -b --disable-autoexec my/blend/file.blend -f 121 -o some/out/folder/####.png -F PNG".to_string());
/// assert_eq!(range_frame_command, "blender -b --disable-autoexec my/blend/file.blend -s 1 -e 250 -j 1 -o some/out/folder/####.png -F PNG".to_string());
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Task{
    pub status: Status,
    pub time: JobTime,
    pub command: Command,
    pub delivery_tag: Option<String>
}


// Basic functions dealing with creation, serialization and deserialization
impl Task{
    /// Create a new Task with a basic command
    /// ```
    /// # extern crate bender_job;
    /// use bender_job::Task;
    /// // Create a new Task with the command ls -a
    /// let t = Task::new_basic("ls -a");
    /// ```
    pub fn new_basic<S>(command: S) -> Self where S: Into<String>{
        Self{
            status: Status::Waiting,
            time: JobTime::new(),
            command: Command::new(command.into()),
            delivery_tag: None
        }
    }

    /// Create a new Task for a single blender frame
    /// ```
    /// # extern crate bender_job;
    /// # use bender_job::Task;
    /// // Create a new blender task that renders frame 1 as PNG
    /// let t = Task::new_blender_single(1, "PNG");
    /// ```
    pub fn new_blender_single<S>(frame: usize, image_format: S) -> Self where S: Into<String>{
        Self{
            status: Status::Waiting,
            time: JobTime::new(),
            command: Command::new_blender_single(frame, image_format.into()),
            delivery_tag: None
        }
    }

    /// Create a new Task for a range of blender frames
    /// ```
    /// # extern crate bender_job;
    /// # use bender_job::Task;
    /// // Create a Task that renders every 10th frame from 1 to 250)
    /// let t = Task::new_blender_range(1, 250, 10, "PNG");
    /// ```
    pub fn new_blender_range<S>(start: usize, end: usize, step: usize, image_format: S) -> Self where S: Into<String>{
        Self{
            status: Status::Waiting,
            time: JobTime::new(),
            command: Command::new_blender_range(start, end, step, image_format.into()),
            delivery_tag: None
        }
    }

    /// Construct a Command with the given paths. Mandatory for Blender Tasks
    /// ```
    /// # extern crate bender_job;
    /// # use bender_job::Task;
    /// // Create a Task that renders frame 121 to a PNG file
    /// let t = Task::new_blender_single(121, "PNG");
    ///
    /// // Let's serialize the Task and pretend we send it to another machine:
    /// let serialized = t.serialize_to_u8().expect("Serialization failed!");
    /// // imagine a incredible journey through the data highway here, on the
    /// // end of which we deserialize. t needs to be mut because we construct it later
    /// let mut t = Task::deserialize_from_u8(&serialized).expect("Deserialization failed!");
    ///
    /// // Now that we are on a different machine, it could make sense to let the worker
    /// // decide which paths to use:
    /// let blendfile = "my/custom/file.blend";
    /// let outpath = "render/files/here/####.png";
    ///
    /// // Construct the command with the new paths
    /// t.construct(blendfile, outpath);
    ///
    /// // Get the string of the command (unwrap will panic if you forgot construct):
    /// let c = t.to_string().unwrap();
    /// assert_eq!(c, "blender -b --disable-autoexec my/custom/file.blend -f 121 -o render/files/here/####.png -F PNG".to_string());
    /// ```
    pub fn construct<S>(&mut self, blendfile: S, outpath: S) where S: Into<String>{
        self.command.construct(blendfile.into(), outpath.into())
    }

    /// Convert the command to string. This returns an Error when the command is a variant \
    /// that needs construction first (see explaination for construct method).
    ///
    /// ```
    /// # extern crate bender_job;
    /// # use bender_job::Task;
    /// // Create a Task with the command ls -a
    /// let t = Task::new_basic("ls -a");
    ///
    /// // Convert it to a string and unwrap the Result (this will never panic for a basic task)
    /// let command = t.to_string().unwrap();
    /// assert_eq!(command, "ls -a".to_string());
    /// ```
    pub fn to_string(&self) -> GenResult<String>{
        self.command.to_string()
    }

    /// Serialize a Task into a String. Return a Error if this fails
    pub fn serialize(&self) -> GenResult<String> {
        let string = serde_json::to_string_pretty(&self)?;
        Ok(string)
    }

    /// Serialize a Task into a Vec<u8>. Return a Error if this fails
    /// you might want to use this with a reference
    pub fn serialize_to_u8(&self) -> GenResult<Vec<u8>> {
        let string = serde_json::to_string_pretty(&self)?;
        Ok(string.into_bytes())
    }

    /// Deserialize something that is Into<String> into a Task
    pub fn deserialize<S>(s: S) -> GenResult<Self> where S: Into<String> {
        let deserialized: Self = serde_json::from_str(&s.into()[..])?;
        Ok(deserialized)
    }

    /// Deserialize a &[u8] into a Task. This is the mirror opposite of serialize_to_u8
    pub fn deserialize_from_u8(v:&[u8]) -> GenResult<Self> {
        let s = str::from_utf8(v)?;
        let deserialized: Self = serde_json::from_str(&s)?;
        Ok(deserialized)
    }

    /// Returns true if the Tasks command is a blender Task
    /// Construct a Command with the given paths. Mandatory for Blender Tasks
    /// ```
    /// # extern crate bender_job;
    /// # use bender_job::Task;
    /// // Create a Task with the command ls -a
    /// let basic_task = Task::new_basic("ls -a");
    ///
    /// // Create a BlenderTask that renders frame 121 as a PNG
    /// let blender_task = Task::new_blender_single(121, "PNG");
    /// 
    /// // Let's check for a blend file
    /// assert_eq!(basic_task.is_blender(), false);
    /// assert_eq!(blender_task.is_blender(), true);
    /// ```
    pub fn is_blender(&self) -> bool{
        self.command.is_blender()
    }
}



// Methods dealing with Task.status
impl Task{
    /// Start the task (only if the task is waiting)
    /// and log the time of this call
    pub fn start(&mut self){
        match self.status{
            Status::Running => (),
            Status::Finished => (),
            _ => {
                self.time.start();
                self.status = Status::Running;
            }
        }
    }

    /// Finish the task (only if the task is running)
    /// and log the time of this call
    pub fn finish(&mut self){
        match self.status{
            Status::Running => {
                self.time.finish();
                self.status = Status::Finished;
            },
            _ => ()
        }
    }

    /// Error the task (only if it didn't error or finish)
    /// and log the time of this call
    pub fn error(&mut self){
        match self.status{
            Status::Errored|Status::Finished => (),
            _ => {
                self.time.error();
                self.status = Status::Errored;
            }
        } 
    }

    /// Abort the task (only if it is either running, waiting or paused)
    /// and log the time of this call
    pub fn abort(&mut self){
        match self.status{
            Status::Running|Status::Waiting|Status::Paused => {
                self.time.abort();
                self.status = Status::Aborted;
            },
            _ => ()
        }
    }

    /// Pause the task (only if it is running)
    /// and log the time of this call
    pub fn pause(&mut self){
        match self.status{
            Status::Running => {
                self.time.pause();
                self.status = Status::Paused;
            },
            _ => ()
        }
    }

    /// Resume the Task if it is running
    pub fn resume(&mut self){
        match self.status{
            Status::Paused => {
                self.status = Status::Running;
            },
            _ => ()
        }
    }
}



/// A Tasks Status describes the different states a [Task](struct.Task.html) can be in and allows
/// the Task to manage all possible transitions between them. Invalid transitions
/// are just ignored.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status{
    Waiting,
    Running,
    Finished,
    Errored,
    Aborted,
    Paused
}




#[cfg(test)]
mod tests {
    use task::{Task, Status}; 
    #[test]
    fn initial_status() {
        let t = Task::new_basic("ls -a");
        assert_eq!(t.status, Status::Waiting);
        assert_eq!(t.time.start, None);
        assert_eq!(t.time.finish, None);
        assert_eq!(t.time.error, None);
    }

    #[test]
    fn serialize_deserialze() {
        let t1 = Task::new_basic("ls -a");
        match t1.serialize(){
            Ok(serialized) => {
                if let Ok(t2) = Task::deserialize(serialized) {
                    assert_eq!(t1, t2);
                }
            },
            Err(e) => println!("Error: {}", e)
        }
    }

    #[test]
    fn serialize_deserialze_u8() {
        let t1 = Task::new_basic("ls -a");
        match t1.serialize_to_u8(){
            Ok(serialized) => {
                if let Ok(t2) = Task::deserialize_from_u8(&serialized) {
                    assert_eq!(t1, t2);
                }
            },
            Err(e) => println!("Error: {}", e)
        }
    }

    #[test]
    fn is_blender(){
        let t = Task::new_blender_single(121, "PNG");
        assert_eq!(t.is_blender(), true);
        let t = Task::new_basic("ls -a");
        assert_eq!(t.is_blender(), false);
    }

}