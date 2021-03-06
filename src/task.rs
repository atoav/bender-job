//! The task module defines the Task Structholding the atomized units of work which \
//! are distributed among the workers
use ::*;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use chrono::Duration;
use common::random_id;


// ===========================================================================
//                                   Task
// ===========================================================================

/// A Task is what gets passed from qu to worker. Tasks are essentially atomic \
/// units of work, broken down so they can be fairly managed by the queue.
///
/// This is basically a wrapper around a command, that allows us to keep track \
/// of a Tasks status, its start and end times etc. It consists of:
/// - a [Status](task/enum.Status.html) which manages the States of a Task and \
/// the allowed transitions between it (e.g. a finished task cannot be aborted,\
/// a errored task cannot start etc.)
/// - a [JobTime](jobtime/struct.JobTime.html) which allows to keep track of \
/// _when_ a certain state change has occured, as well as the calculation of \
/// durations (the same construct is used for [Job](struct.Job.html))
/// - a [Command](command/enum.Command.html) which allows to abstract CLI \
/// commands to be executed on the worker machines in such way, that we don't \
/// need to know input and output paths beforehand
/// 
/// Construct a new basic task like this:
/// ```
/// # extern crate bender_job;
/// use bender_job::Task;
/// 
/// // Create a basic task that lists files
/// let basic_task = Task::new_basic("ls -a", "55067970443c49eaafdb60541fbde157");
///
/// // Create a blender task for a single frame (121. with PNG as image format)
/// let mut single_frame_task = Task::new_blender_single(121, "PNG", "55067970443c49eaafdb60541fbde157");
///
/// // Create a blender task for a range of frames (1 to 250, with a step size of 1)
/// let mut range_frame_task = Task::new_blender_range(1, 250, 1, "PNG", "55067970443c49eaafdb60541fbde157");
///
/// // Tasks with a blender command must be constructed with paths before usage. 
/// single_frame_task.construct("my/blend/file.blend", "some/out/folder");
/// range_frame_task.construct("my/blend/file.blend", "some/out/folder");
///
/// // Get the constructed tasks (unwrap fails, if you didn't construct the blender tasks before):
/// let basic_command = basic_task.to_string().unwrap();
/// let single_frame_command = single_frame_task.to_string().unwrap();
/// let range_frame_command = range_frame_task.to_string().unwrap();
///
/// // This yields following strings as a result:
/// assert_eq!(basic_command, "ls -a".to_string());
/// assert_eq!(single_frame_command, "blender -b --disable-autoexec my/blend/file.blend -o some/out/folder/######.png -F PNG -f 121".to_string());
/// assert_eq!(range_frame_command, "blender -b --disable-autoexec my/blend/file.blend -o some/out/folder/######.png -F PNG -s 1 -e 250".to_string());
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task{
    pub id: String,
    pub status: Status,
    pub time: JobTime,
    pub command: Command,
    pub data: HashMap<String, String>,
    pub parent_id: String
}

impl Hash for Task {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


// Basic functions dealing with creation, serialization and deserialization
impl Task{
    /// Create a new Task with a basic command
    /// ```
    /// # extern crate bender_job;
    /// use bender_job::Task;
    /// // Create a new Task with the command ls -a
    /// let t = Task::new_basic("ls -a", "55067970443c49eaafdb60541fbde157");
    /// ```
    pub fn new_basic<S>(command: S, parent_id: S) -> Self where S: Into<String>{
        Self{
            id: random_id(),
            status: Status::Waiting,
            time: JobTime::new(),
            command: Command::new(command.into()),
            parent_id: parent_id.into(),
            data: HashMap::new()
        }
    }

    /// Create a new Task for a single blender frame
    /// ```
    /// # extern crate bender_job;
    /// # use bender_job::Task;
    /// // Create a new blender task that renders frame 1 as PNG
    /// let t = Task::new_blender_single(1, "PNG", "55067970443c49eaafdb60541fbde157");
    /// ```
    pub fn new_blender_single<S>(frame: usize, image_format: S, id: S) -> Self where S: Into<String>{
        Self{
            id: random_id(),
            status: Status::Waiting,
            time: JobTime::new(),
            command: Command::new_blender_single(frame, image_format.into()),
            parent_id: id.into(),
            data: HashMap::new()
        }
    }

    /// Create a new Task for a range of blender frames
    /// ```
    /// # extern crate bender_job;
    /// # use bender_job::Task;
    /// // Create a Task that renders every 10th frame from 1 to 250)
    /// let t = Task::new_blender_range(1, 250, 10, "PNG", "55067970443c49eaafdb60541fbde157");
    /// ```
    pub fn new_blender_range<S>(start: usize, end: usize, step: usize, image_format: S, id: S) -> Self where S: Into<String>{
        Self{
            id: random_id(),
            status: Status::Waiting,
            time: JobTime::new(),
            command: Command::new_blender_range(start, end, step, image_format.into()),
            parent_id: id.into(),
            data: HashMap::new()
        }
    }

    /// Construct a Command with the given paths. Mandatory for Blender Tasks
    /// ```
    /// # extern crate bender_job;
    /// # use bender_job::Task;
    /// // Create a Task that renders frame 121 to a PNG file with the given id
    /// let t = Task::new_blender_single(121, "PNG", "55067970443c49eaafdb60541fbde157");
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
    /// let outpath = "render/files/here";
    ///
    /// // Construct the command with the new paths
    /// t.construct(blendfile, outpath);
    ///
    /// // Get the string of the command (unwrap will panic if you forgot construct):
    /// let c = t.to_string().unwrap();
    /// assert_eq!(c, "blender -b --disable-autoexec my/custom/file.blend -o render/files/here/######.png -F PNG -f 121".to_string());
    /// ```
    pub fn construct<S>(&mut self, blendfile: S, outpath: S) where S: Into<String>{
        self.command.construct(blendfile.into(), outpath.into())
    }

    /// allows to quickly add data to Self::data
    pub fn add_data<S>(&mut self, key: S, value: S) where S: Into<String> {
        self.data.insert(key.into(), value.into());
    }

    /// Convert the command to string. This returns an Error when the command is a variant \
    /// that needs construction first (see explaination for construct method).
    ///
    /// ```
    /// # extern crate bender_job;
    /// # use bender_job::Task;
    /// // Create a Task with the command ls -a for a job with the given id
    /// let t = Task::new_basic("ls -a", "55067970443c49eaafdb60541fbde157");
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
    /// // Create a Task with the command ls -a for a job with the given id
    /// let basic_task = Task::new_basic("ls -a", "55067970443c49eaafdb60541fbde157");
    ///
    /// // Create a BlenderTask that renders frame 121 as a PNG fo a job with the id
    /// let blender_task = Task::new_blender_single(121, "PNG", "55067970443c49eaafdb60541fbde157");
    /// 
    /// // Let's check for a blend file
    /// assert_eq!(basic_task.is_blender(), false);
    /// assert_eq!(blender_task.is_blender(), true);
    /// ```
    pub fn is_blender(&self) -> bool{
        self.command.is_blender()
    }

    /// Allows to merge one Task with another. This uses "smart" rules in order \
    /// to ensure no relevant fields get overwritten and will print an Error if \
    /// the user tries to merge two Tasks with differing id or parent_id fields.
    /// This means it is the users responsibility to ensure these match. 
    pub fn merge(&mut self, other: &Self) {
        if !(self.id != other.id || self.parent_id != other.parent_id) {
            self.status.merge(&other.status);
            self.time.merge(&other.time);
            self.command.merge(&other.command);
            self.merge_data(&other);
        }else{
            eprintln!("Error: you tried to merge two Tasks with differing ids or parent_ids");      
        }
    }

    /// Extend `self.data` with `other.data`
    pub fn merge_data(&mut self, other: &Self){
        self.data.extend(other.data.clone());
    }
}



impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Task [{}]({:?}): {}", self.id, self.status, self.command)
    }
}




// ===========================================================================
//                                Task.status
// ===========================================================================

// Methods dealing with Task.status
impl Task{
    /// Queue the task (only if the task is waiting) and log the time of this call
    pub fn queue(&mut self){
        match self.status{
            Status::Running => (),
            Status::Finished => (),
            Status::Waiting => {
                self.time.queue();
                self.status = Status::Queued;
            },
            _ => ()
        }
    }

    /// Start the task (only if the task is waiting) and log the time of this call
    pub fn start(&mut self){
        match self.status{
            Status::Running => (),
            Status::Finished => (),
            Status::Queued => {
                self.time.start();
                self.status = Status::Running;
            },
            _ => ()
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
            Status::Queued => {
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

    /// Abort the task (only if it is either running, waiting, queued or paused)
    /// and log the time of this call
    pub fn abort(&mut self){
        match self.status{
            Status::Running|Status::Waiting|Status::Paused|Status::Queued => {
                self.time.abort();
                self.status = Status::Aborted;
            },
            _ => ()
        }
    }

    /// Pause the task (only if it is running)
    /// and log the time of this call
    pub fn pause(&mut self){
        if let Status::Running = self.status {
            self.time.pause();
            self.status = Status::Paused;
        }
    }

    /// Resume the Task if it is running
    pub fn resume(&mut self){
        if let Status::Paused = self.status {
            self.status = Status::Running;
        }
    }

    pub fn is_started(&self) -> bool{
         match self.status{
            Status::Running => true,
            _ => false
        }
    }

    pub fn is_running(&self) -> bool{
         match self.status{
            Status::Running => true,
            _ => false
        }
    }

    pub fn is_finished(&self) -> bool{
         match self.status{
            Status::Finished => true,
            _ => false
        }
    }

    pub fn is_errored(&self) -> bool{
         match self.status{
            Status::Errored => true,
            _ => false
        }
    }

    pub fn is_aborted(&self) -> bool{
         match self.status{
            Status::Aborted => true,
            _ => false
        }
    }

    pub fn is_paused(&self) -> bool{
         match self.status{
            Status::Paused => true,
            _ => false
        }
    }

    pub fn is_waiting(&self) -> bool{
         match self.status{
            Status::Waiting => true,
            _ => false
        }
    }

    pub fn is_queued(&self) -> bool{
         match self.status{
            Status::Queued => true,
            _ => false
        }
    }

    pub fn is_alive(&self) -> bool{
         match self.status{
            Status::Waiting|Status::Running|Status::Paused|Status::Queued => true,
            _ => false
        }
    }

    pub fn is_ended(&self) -> bool{
         match self.status{
            Status::Finished|Status::Errored|Status::Aborted => true,
            _ => false
        }
    }

    pub fn needs_dispatching(&self) -> bool{
         match self.status{
            Status::Waiting => true,
            _ => false
        }
    }
}




// ===========================================================================
//                                  Status
// ===========================================================================

/// A Tasks Status describes the different states a [Task](struct.Task.html) can be in and allows
/// the Task to manage all possible transitions between them. Invalid transitions
/// are just ignored.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status{
    Waiting,
    Queued,
    Running,
    Finished,
    Errored,
    Aborted,
    Paused
}



impl Status{
    /// Merge other status into self based on the other status' value
    pub fn merge(&mut self, other: &Self){
        // Check if the status
        let should_update_status = match self{
            Status::Waiting => {
                match other{
                    _ => true
                }
            },
            Status::Queued => {
                match other{
                    Status::Waiting => false,
                    _ => true
                }
            },
            Status::Running => {
                match other{
                    Status::Waiting => false,
                    Status::Queued => false,
                    _ => true
                }
            },
            Status::Paused => {
                match other{
                    _ => true
                }
            },
            Status::Aborted => {
                false
            },
            Status::Errored => {
                false
            },
            Status::Finished => {
                match other{
                    Status::Finished => true,
                    _ => false
                }
            }
        };

        // Update the status if it has been
        if should_update_status{
            *self = other.clone();
        }
    }
}




// ===========================================================================
//                                  Tasks
// ===========================================================================

pub type Tasks = VecDeque<Task>;

/// The TaskQueue trait is responsible for implementing queue utilities for the \
/// `Job::tasks` field. It returns references to potentially interesting tasks \
/// like the next waiting task, a vector of running tasks etc.
/// 
/// Additionally this trait allows to start the next task, abort or pause running \
/// tasks as well as displaying average and total runtimes
pub trait TaskQueue{
    /// Get the position a Task has in VecDeque by id
    fn position_by_id<S>(&self, id: S) -> Option<usize> where S: Into<String>;

    /// Get a reference to the task with the given ID
    fn get_by_id<S>(&self, id: S) -> Option<&Task> where S: Into<String>;

    /// Get a mutable reference to the task with the given ID
    fn get_mut_by_id<S>(&mut self, id: S) -> Option<&mut Task> where S: Into<String>;

    /// Return true if there is a Task with the given id
    fn has_task<S>(&self, id: S) -> bool where S: Into<String>;

    /// Put the next Task into Queue status and return a mutable reference to it.
    /// The next task is the next task that is waiting
    fn queue_next(&mut self) -> Option<&mut Task>;

    /// Dispatch the next Task in the queue and return a mutable reference to it.
    /// The next task is the next task that is queued
    fn start_next(&mut self) -> Option<&mut Task>;

    /// Abort all running tasks
    fn abort_all_running(&mut self);

    /// Pause all running tasks
    fn pause_all_running(&mut self);

    /// Resume all paused tasks
    fn resume_all_paused(&mut self);

    /// Return a reference to the next Task without starting it
    fn get_next(&self) -> Option<&Task>;

    /// Return a mutable reference to the next Task without starting it
    fn get_next_mut(&mut self) -> Option<&mut Task>;

    /// Calculate the total time it took each task to finish (this includes the\
    /// time of currently running tasks that have yet to finish)
    fn total_duration(&self) -> Duration;

    /// Calculate the average time it took each task to finish (this includes the\
    /// time of currently running tasks that have yet to finish)
    fn average_duration(&self) -> Duration;

    /// Calculate the total time it took each task to finish (this includes the\
    /// time of currently running tasks that have yet to finish)
    fn total_duration_seconds(&self) -> usize;

    /// Calculate the average time it took each task to finish (this includes the\
    /// time of currently running tasks that have yet to finish)
    fn average_duration_seconds(&self) -> usize;

    /// Return a vector of references to running Tasks
    fn running(&self) -> Vec<&Task>;

    /// Return a vector of mutable references to running Tasks
    fn running_mut(&mut self) -> Vec<&mut Task>;

    /// Return a vector of references to finished Tasks
    fn finished(&self) -> Vec<&Task>;

    /// Return a vector of mutable references to finished Tasks
    fn finished_mut(&mut self) -> Vec<&mut Task>;

    /// Return a vector of references to paused Tasks
    fn paused(&self) -> Vec<&Task>;

    /// Return a vector of mutable references to paused Tasks
    fn paused_mut(&mut self) -> Vec<&mut Task>;

    /// Return a vector of references to errored Tasks
    fn errored(&self) -> Vec<&Task>;

    /// Return a vector of mutable references to errored Tasks
    fn errored_mut(&mut self) -> Vec<&mut Task>;

    /// Return a vector of references to aborted Tasks
    fn aborted(&self) -> Vec<&Task>;

    /// Return a vector of mutable references to aborted Tasks
    fn aborted_mut(&mut self) -> Vec<&mut Task>;

    /// Returns true if all tasks finished
    fn is_all_finished(&self) -> bool;

    /// Returns true if all tasks ended
    fn is_all_ended(&self) -> bool;

    /// Returns true if all tasks are waiting
    fn is_all_waiting(&self) -> bool;

    /// Returns true if all tasks are queued
    fn is_all_queued(&self) -> bool;

    /// Returns true if all tasks are running
    fn is_all_running(&self) -> bool;

    /// Returns true if any of the tasks is running
    fn is_any_running(&self) -> bool;

    /// Returns true if any of the tasks is errored
    fn is_any_errored(&self) -> bool;

    /// Returns true if any of the tasks is waiting
    fn is_any_waiting(&self) -> bool;

    // Returns true if any of the tasks is queued
    fn is_any_queued(&self) -> bool;

    /// Returns true if any of the tasks is paused
    fn is_any_paused(&self) -> bool;

    /// Returns true if any of the tasks is aborted
    fn is_any_aborted(&self) -> bool;

    /// Returns true if any of the tasks is finished
    fn is_any_finished(&self) -> bool;

    /// Returns true if any of the tasks is ended
    fn is_any_ended(&self) -> bool;

    /// Returns the number of waiting tasks
    fn count_waiting(&self) -> usize;

    /// Returns the number of queued tasks
    fn count_queued(&self) -> usize;

    /// Returns the number tasks
    fn count(&self) -> usize;

    /// Returns the number of running tasks
    fn count_running(&self) -> usize;

    /// Returns the number of errored tasks
    fn count_errored(&self) -> usize;

    /// Returns the number of aborted tasks
    fn count_aborted(&self) -> usize;

    /// Returns the number of paused tasks
    fn count_paused(&self) -> usize;

    /// Returns the number of finished tasks
    fn count_finished(&self) -> usize;

    /// Returns the number of ended tasks
    fn count_ended(&self) -> usize;

    /// Update Task from other Task
    fn update_task_from(&mut self, task: &Task);

    /// Update Tasks from other tasks
    fn update_from(&mut self, other: &Self, force: bool);

    /// Merge Tasks one by one
    fn merge(&mut self, other: &Self);
}





impl TaskQueue for Tasks{
    // ================== BY ID METHODS ====================

    fn position_by_id<S>(&self, id: S) -> Option<usize> where S: Into<String>{
        let id = id.into();
        self.iter()
            .position(|task|task.id == id)
    }

    fn get_by_id<S>(&self, id: S) -> Option<&Task> where S: Into<String>{
        let id = id.into();
        self.iter()
            .find(|ref task|task.id == id)
    }

    fn get_mut_by_id<S>(&mut self, id: S) -> Option<&mut Task> where S: Into<String>{
        let id = id.into();
        self.iter_mut()
            .find(|ref mut task|task.id == id)
    }

    fn has_task<S>(&self, id: S) -> bool where S: Into<String>{
        let id = id.into();
        self.iter()
            .any(|ref task|task.id == id)
    }

    // ================== CONTROL METHODS ====================

    fn queue_next(&mut self) -> Option<&mut Task>{
        match self.iter().position(|t| t.is_waiting()){
            Some(position) => {
                self[position].queue();
                Some(&mut self[position])
            },
            None => None
        }
    }

    fn start_next(&mut self) -> Option<&mut Task>{
        match self.iter().position(|t| t.is_queued()){
            Some(position) => {
                self[position].start();
                Some(&mut self[position])
            },
            None => None
        }
    }

    fn get_next_mut(&mut self) -> Option<&mut Task>{
        match self.iter().position(|t| t.is_waiting()){
            Some(position) => {
                Some(&mut self[position])
            },
            None => None
        }
    }

    fn get_next(&self) -> Option<&Task>{
        match self.iter().position(|t| t.is_waiting()){
            Some(position) => Some(&self[position]),
            None => None
        }
    }

    fn abort_all_running(&mut self){
        self.running_mut().into_iter().for_each(|t|t.abort());
    }

    fn pause_all_running(&mut self){
        self.running_mut().into_iter().for_each(|t|t.pause());
    }

    fn resume_all_paused(&mut self){
        self.paused_mut().into_iter().for_each(|t|t.resume());
    }

    // ================== VECTOR METHODS ====================

    fn running(&self) -> Vec<&Task>{
        self.iter().filter(|t| t.is_running()).collect()
    }

    fn running_mut(&mut self) -> Vec<&mut Task>{
        self.iter_mut().filter(|t| t.is_running()).collect()
    }

    fn finished(&self) -> Vec<&Task>{
        self.iter().filter(|t| t.is_finished()).collect()
    }

    fn finished_mut(&mut self) -> Vec<&mut Task>{
        self.iter_mut().filter(|t| t.is_finished()).collect()
    }

    fn paused(&self) -> Vec<&Task>{
        self.iter().filter(|t| t.is_paused()).collect()
    }

    fn paused_mut(&mut self) -> Vec<&mut Task>{
        self.iter_mut().filter(|t| t.is_paused()).collect()
    }

    fn errored(&self) -> Vec<&Task>{
        self.iter().filter(|t| t.is_errored()).collect()
    }

    fn errored_mut(&mut self) -> Vec<&mut Task>{
        self.iter_mut().filter(|t| t.is_errored()).collect()
    }

    fn aborted(&self) -> Vec<&Task>{
        self.iter().filter(|t| t.is_aborted()).collect()
    }

    fn aborted_mut(&mut self) -> Vec<&mut Task>{
        self.iter_mut().filter(|t| t.is_aborted()).collect()
    }


    // ================== DURATION/AGE METHODS ====================

    fn total_duration(&self) -> Duration{
        // Use both finished and running tasks
        let mut v = self.finished();
        v.append(&mut self.running());
        // Accumulate all durations
        v.into_iter()
         .fold(Duration::zero(), |d, t| d + t.time.duration().unwrap())
    }

    fn average_duration(&self) -> Duration{
        // Use both finished and running tasks
        let mut v = self.finished();
        v.append(&mut self.running());
        // Calculate the average duration
        let len: usize = v.len();
        match len{
            0 => Duration::zero(),
            _ => self.total_duration()/len as i32
        }
    }

    fn total_duration_seconds(&self) -> usize{
        // Use both finished and running tasks
        let mut v = self.finished();
        v.append(&mut self.running());
        // Accumulate all durations
        v.into_iter()
         .fold(0, |d, t| d + t.time.duration_seconds().unwrap())
    }

    fn average_duration_seconds(&self) -> usize{
        // Use both finished and running tasks
        let mut v = self.finished();
        v.append(&mut self.running());
        // Calculate the average duration
        let len: usize = v.len();
        match len{
            0 => 0,
            _ => self.total_duration_seconds()/len
        }
    }



    // ================== COUNTING METHODS ====================

    fn is_all_finished(&self) -> bool{
        self.iter().all(|t| t.is_finished())
    }

    fn is_all_ended(&self) -> bool{
        self.iter().all(|t| t.is_ended())
    }

    fn is_all_waiting(&self) -> bool{
        self.iter().all(|t| t.is_waiting())
    }

    fn is_all_running(&self) -> bool{
        self.iter().all(|t| t.is_running())
    }

    fn is_all_queued(&self) -> bool{
        self.iter().all(|t| t.is_queued())
    }

    fn is_any_running(&self) -> bool{
        self.iter().any(|t| t.is_running())
    }

    fn is_any_queued(&self) -> bool{
        self.iter().any(|t| t.is_queued())
    }

    fn is_any_errored(&self) -> bool{
        self.iter().any(|t| t.is_errored())
    }

    fn is_any_waiting(&self) -> bool{
        self.iter().any(|t| t.is_waiting())
    }

    fn is_any_paused(&self) -> bool{
        self.iter().any(|t| t.is_paused())
    }

    fn is_any_aborted(&self) -> bool{
        self.iter().any(|t| t.is_aborted())
    }

    fn is_any_finished(&self) -> bool{
        self.iter().any(|t| t.is_finished())
    }

    fn is_any_ended(&self) -> bool{
        self.iter().any(|t| t.is_ended())
    }


    // ================== COUNTING METHODS ====================

    /// Returns the number tasks
    fn count(&self) -> usize{
        self.iter()
            .count()
    }

    /// Returns the number of waiting tasks
    fn count_waiting(&self) -> usize{
        self.iter()
            .filter(|t| t.is_waiting())
            .count()
    }

    /// Returns the number of queued tasks
    fn count_queued(&self) -> usize{
        self.iter()
            .filter(|t| t.is_queued())
            .count()
    }

    /// Returns the number of running tasks
    fn count_running(&self) -> usize{
        self.iter()
            .filter(|t| t.is_running())
            .count()
    }

    /// Returns the number of errored tasks
    fn count_errored(&self) -> usize{
        self.iter()
            .filter(|t| t.is_errored())
            .count()
    }

    /// Returns the number of aborted tasks
    fn count_aborted(&self) -> usize{
        self.iter()
            .filter(|t| t.is_aborted())
            .count()
    }

    /// Returns the number of paused tasks
    fn count_paused(&self) -> usize{
        self.iter()
            .filter(|t| t.is_paused())
            .count()
    }

    /// Returns the number of finished tasks
    fn count_finished(&self) -> usize{
        self.iter()
            .filter(|t| t.is_finished())
            .count()
    }

    /// Returns the number of ended tasks
    fn count_ended(&self) -> usize{
        self.iter()
            .filter(|t| t.is_ended())
            .count()
    }



    // ============== UPDATE METHODS ===============
    fn update_from(&mut self, other: &Self, force: bool){
        self.iter_mut()
            .zip(other.iter())
            .for_each(|(this, that)|{
                // Decide if the other task is considered newer or older based on
                // it's status first
                let mut should_update = match this.status{
                    Status::Waiting => {
                        match that.status{
                            _ => true
                        }
                    },
                    Status::Queued => {
                        match that.status{
                            Status::Waiting => false,
                            _ => true
                        }
                    },
                    Status::Running => {
                        match that.status{
                            Status::Waiting => false,
                            Status::Queued => false,
                            _ => true
                        }
                    },
                    Status::Paused => {
                        match that.status{
                            Status::Waiting => false,
                            Status::Queued => false,
                            Status::Running => false,
                            _ => true
                        }
                    },
                    Status::Aborted => {
                        false
                    },
                    Status::Errored => {
                        false
                    },
                    Status::Finished => {
                        match that.status{
                            Status::Finished => true,
                            _ => false
                        }
                    }
                };

                // Do not update if this command is constructed and the other isn't
                should_update = (!this.command.is_constructed() || that.command.is_constructed()) && should_update;

                // Finally do the updatin' if all checks say yes
                if should_update || force{
                    *this = that.clone();
                }
            });
    }

    fn update_task_from(&mut self, task: &Task){
        // Find the other task in self:
        if let Some(index) = self.position_by_id(task.id.as_str()) {
            self.remove(index);
            self.insert(index, task.clone());
        }      
    }

    /// Run merge on all tasks of Tasks
    fn merge(&mut self, other: &Self){
        // Get the bigger of the two
        let taskcount = std::cmp::max(self.len(), other.len());
        let difference = taskcount - std::cmp::min(self.len(), other.len());

        // Create a list for appending tasks
        let mut newtasks = Vec::with_capacity(difference);

        // Iterate over the maximum length of the two VecDeque<Task>
        for i in 0 .. taskcount{
            // Get a tuple of (Option<&mut Task>, Option<&Task>)
            let tasks = (self.get_mut(i), other.get(i));
            // Match all possible combinations of old and new
            match tasks{
                (Some(o), Some(n)) => {    // Both tasks exist, merge new in old
                    if n.id == o.id && n.parent_id == o.parent_id{
                        o.merge(n);
                    }else{
                        eprintln!("Error: Tried to merge Tasks with differing parent_id or id:");
                        eprintln!("Old: {:#?}", o);
                        eprintln!("New: {:#?}", n);
                    }
                },
                (Some(_), None)    => (),  // Only old task exists, do nothing
                (None, Some(n))    => newtasks.push(n.clone()),
                (None, None)       => ()   // Both are none (this should not happen)
            }
        }
    }

}








// ===========================================================================
//                                 UNIT TESTS
// ===========================================================================

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tasks_length() {
        let mut a = Tasks::new();
        a.push_front(Task::new_basic("a1", "a"));
        a.push_front(Task::new_basic("a2", "a"));
        a.push_front(Task::new_basic("a3", "a"));
        a.push_front(Task::new_basic("a4", "a"));

        let mut b = Tasks::new();
        b.push_front(Task::new_basic("b1", "a"));
        b.push_front(Task::new_basic("b2", "a"));
        b.push_front(Task::new_basic("b3", "a"));
        b.push_front(Task::new_basic("b4", "a"));

        a.merge(&b);

        assert_eq!(a.len(), b.len());
    }

    #[test]
    fn tasks_status() {
        let mut a = Tasks::new();

        let mut t1 = Task::new_blender_single(1, "PNG", "id");
        let mut t2 = Task::new_blender_single(2, "PNG", "id");
        let mut t3 = Task::new_blender_single(3, "PNG", "id");
        let mut t4 = Task::new_blender_single(4, "PNG", "id");
        a.push_front(t1.clone());
        a.push_front(t2.clone());
        a.push_front(t3.clone());
        a.push_front(t4.clone());

        let mut b = Tasks::new();

        t1.status = Status::Finished;
        t2.status = Status::Finished;
        t3.status = Status::Finished;
        t4.status = Status::Finished;
        b.push_front(t1);
        b.push_front(t2);
        b.push_front(t3);
        b.push_front(t4);

        a.merge(&b);

        assert_eq!(a.len(), b.len());
        assert_eq!(a[0].status, b[0].status);
        assert_eq!(a[1].status, b[1].status);
        assert_eq!(a[2].status, b[2].status);
        assert_eq!(a[3].status, b[3].status);
    }

    #[test]
    fn tasks_status_many_waiting_to_queued() {
        let mut a = Tasks::new();
        let mut b = Tasks::new();

        for i in 0..20000 {
            let mut t = Task::new_blender_single(i, "PNG", "id");
            a.push_front(t.clone());
            t.status = Status::Queued;
            b.push_front(t);
        }

        a.merge(&b);

        assert_eq!(a.len(), b.len());
        for i in 0..20000 {
            assert_eq!(a[i].status, b[i].status);
        }
    }

    #[test]
    fn tasks_status_many_queued_to_running() {
        let mut a = Tasks::new();
        let mut b = Tasks::new();

        for i in 0..20000 {
            let mut t = Task::new_blender_single(i, "PNG", "id");
            t.status = Status::Queued;
            a.push_front(t.clone());
            t.status = Status::Running;
            b.push_front(t);
        }

        a.merge(&b);

        assert_eq!(a.len(), b.len());
        for i in 0..20000 {
            assert_eq!(a[i].status, b[i].status);
        }
    }


    #[test]
    fn tasks_status_many_running_to_finished() {
        let mut a = Tasks::new();
        let mut b = Tasks::new();

        for i in 0..20000 {
            let mut t = Task::new_blender_single(i, "PNG", "id");
            t.status = Status::Running;
            a.push_front(t.clone());
            t.status = Status::Finished;
            b.push_front(t);
        }

        a.merge(&b);

        assert_eq!(a.len(), b.len());
        for i in 0..20000 {
            assert_eq!(a[i].status, b[i].status);
        }
    }

    #[test]
    fn initial_status() {
        let t = Task::new_basic("ls -a", "55067970443c49eaafdb60541fbde157");
        assert_eq!(t.status, Status::Waiting);
        assert_eq!(t.time.start, None);
        assert_eq!(t.time.finish, None);
        assert_eq!(t.time.error, None);
    }

    #[test]
    fn serialize_deserialize() {
        let t1 = Task::new_basic("ls -a", "55067970443c49eaafdb60541fbde157");
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
        let t1 = Task::new_basic("ls -a", "55067970443c49eaafdb60541fbde157");
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
        let t = Task::new_blender_single(121, "PNG", "55067970443c49eaafdb60541fbde157");
        assert_eq!(t.is_blender(), true);
        let t = Task::new_basic("ls -a", "55067970443c49eaafdb60541fbde157");
        assert_eq!(t.is_blender(), false);
    }

    // ------------------------------ Merge Waiting --------------------------
    #[test]
    fn merge_waiting_with_queued() {
        let mut old = Status::Waiting;
        let new = Status::Queued;
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_waiting_with_running() {
        let mut old = Status::Waiting;
        let new = Status::Running;
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_waiting_with_finished() {
        let mut old = Status::Waiting;
        let new = Status::Finished;
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_waiting_with_errored() {
        let mut old = Status::Waiting;
        let new = Status::Errored;
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_waiting_with_aborted() {
        let mut old = Status::Waiting;
        let new = Status::Aborted;
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_waiting_with_paused() {
        let mut old = Status::Waiting;
        let new = Status::Paused;
        old.merge(&new);
        assert_eq!(old, new);
    }

    // ------------------------------ Merge Queued --------------------------
    #[test]
    fn merge_queued_with_waiting() {
        let mut old = Status::Queued;
        let new = Status::Waiting;
        old.merge(&new);
        assert_eq!(old, Status::Queued);
    }

    #[test]
    fn merge_queued_with_running() {
        let mut old = Status::Queued;
        let new = Status::Running;
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_queued_with_finished() {
        let mut old = Status::Queued;
        let new = Status::Finished;
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_queued_with_errored() {
        let mut old = Status::Queued;
        let new = Status::Errored;
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_queued_with_aborted() {
        let mut old = Status::Queued;
        let new = Status::Aborted;
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_queued_with_paused() {
        let mut old = Status::Queued;
        let new = Status::Paused;
        old.merge(&new);
        assert_eq!(old, new);
    }

    // ------------------------------ Merge Running --------------------------
    #[test]
    fn merge_running_with_waiting() {
        let mut old = Status::Running;
        let new = Status::Waiting;
        old.merge(&new);
        assert_eq!(old, Status::Running);
    }

    #[test]
    fn merge_running_with_queued() {
        let mut old = Status::Running;
        let new = Status::Queued;
        old.merge(&new);
        assert_eq!(old, Status::Running);
    }

    #[test]
    fn merge_running_with_finished() {
        let mut old = Status::Running;
        let new = Status::Finished;
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_running_with_errored() {
        let mut old = Status::Running;
        let new = Status::Errored;
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_running_with_aborted() {
        let mut old = Status::Running;
        let new = Status::Aborted;
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_running_with_paused() {
        let mut old = Status::Running;
        let new = Status::Paused;
        old.merge(&new);
        assert_eq!(old, new);
    }

    // ------------------------------ Merge Finished --------------------------
    #[test]
    fn merge_finished_with_waiting() {
        let mut old = Status::Finished;
        let new = Status::Waiting;
        old.merge(&new);
        assert_eq!(old, Status::Finished);
    }

    #[test]
    fn merge_finished_with_queued() {
        let mut old = Status::Finished;
        let new = Status::Queued;
        old.merge(&new);
        assert_eq!(old, Status::Finished);
    }

    #[test]
    fn merge_finished_with_running() {
        let mut old = Status::Finished;
        let new = Status::Running;
        old.merge(&new);
        assert_eq!(old, Status::Finished);
    }

    #[test]
    fn merge_finished_with_errored() {
        let mut old = Status::Finished;
        let new = Status::Errored;
        old.merge(&new);
        assert_eq!(old, Status::Finished);
    }

    #[test]
    fn merge_finished_with_aborted() {
        let mut old = Status::Finished;
        let new = Status::Aborted;
        old.merge(&new);
        assert_eq!(old, Status::Finished);
    }

    #[test]
    fn merge_finished_with_paused() {
        let mut old = Status::Finished;
        let new = Status::Paused;
        old.merge(&new);
        assert_eq!(old, Status::Finished);
    }

    // ------------------------------ Merge Errored --------------------------
    #[test]
    fn merge_errored_with_waiting() {
        let mut old = Status::Errored;
        let new = Status::Waiting;
        old.merge(&new);
        assert_eq!(old, Status::Errored);
    }

    #[test]
    fn merge_errored_with_queued() {
        let mut old = Status::Errored;
        let new = Status::Queued;
        old.merge(&new);
        assert_eq!(old, Status::Errored);
    }

    #[test]
    fn merge_errored_with_running() {
        let mut old = Status::Errored;
        let new = Status::Running;
        old.merge(&new);
        assert_eq!(old, Status::Errored);
    }

    #[test]
    fn merge_errored_with_finished() {
        let mut old = Status::Errored;
        let new = Status::Finished;
        old.merge(&new);
        assert_eq!(old, Status::Errored);
    }

    #[test]
    fn merge_errored_with_aborted() {
        let mut old = Status::Errored;
        let new = Status::Aborted;
        old.merge(&new);
        assert_eq!(old, Status::Errored);
    }

    #[test]
    fn merge_errored_with_paused() {
        let mut old = Status::Errored;
        let new = Status::Paused;
        old.merge(&new);
        assert_eq!(old, Status::Errored);
    }

    // ------------------------------ Merge Aborted --------------------------
    #[test]
    fn merge_aborted_with_waiting() {
        let mut old = Status::Aborted;
        let new = Status::Waiting;
        old.merge(&new);
        assert_eq!(old, Status::Aborted);
    }

    #[test]
    fn merge_aborted_with_queued() {
        let mut old = Status::Aborted;
        let new = Status::Queued;
        old.merge(&new);
        assert_eq!(old, Status::Aborted);
    }

    #[test]
    fn merge_aborted_with_running() {
        let mut old = Status::Aborted;
        let new = Status::Running;
        old.merge(&new);
        assert_eq!(old, Status::Aborted);
    }

    #[test]
    fn merge_aborted_with_finished() {
        let mut old = Status::Aborted;
        let new = Status::Finished;
        old.merge(&new);
        assert_eq!(old, Status::Aborted);
    }

    #[test]
    fn merge_aborted_with_errored() {
        let mut old = Status::Aborted;
        let new = Status::Errored;
        old.merge(&new);
        assert_eq!(old, Status::Aborted);
    }

    #[test]
    fn merge_aborted_with_paused() {
        let mut old = Status::Aborted;
        let new = Status::Paused;
        old.merge(&new);
        assert_eq!(old, Status::Aborted);
    }

    // ------------------------------ Merge Paused --------------------------
    #[test]
    fn merge_paused_with_waiting() {
        let mut old = Status::Paused;
        let new = Status::Waiting;
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_paused_with_queued() {
        let mut old = Status::Paused;
        let new = Status::Queued;
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_paused_with_running() {
        let mut old = Status::Paused;
        let new = Status::Running;
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_paused_with_finished() {
        let mut old = Status::Paused;
        let new = Status::Finished;
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_paused_with_errored() {
        let mut old = Status::Paused;
        let new = Status::Errored;
        old.merge(&new);
        assert_eq!(old, new);
    }

    #[test]
    fn merge_paused_with_aborted() {
        let mut old = Status::Paused;
        let new = Status::Aborted;
        old.merge(&new);
        assert_eq!(old, new);
    }
}