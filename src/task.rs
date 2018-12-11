//! The task module defines the Task Structholding the atomized units of work which \
//! are distributed among the workers
use ::*;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use chrono::Duration;
use common::random_id;




/// A Task is what gets passed from qu to worker. Tasks are essentially atomic \
/// units of work, broken down so they can be fairly managed by the queue.
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
/// assert_eq!(range_frame_command, "blender -b --disable-autoexec my/blend/file.blend -o some/out/folder/######.png -F PNG -s 1 -e 250 -j 1".to_string());
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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


// Basic functions dealing with creation, serialization and deserialization
impl Task{
    /// Create a new Task with a basic command
    /// ```
    /// # extern crate bender_job;
    /// use bender_job::Task;
    /// // Create a new Task with the command ls -a
    /// let t = Task::new_basic("ls -a", "55067970443c49eaafdb60541fbde157");
    /// ```
    pub fn new_basic<S>(command: S, id: S) -> Self where S: Into<String>{
        Self{
            id: random_id(),
            status: Status::Waiting,
            time: JobTime::new(),
            command: Command::new(command.into()),
            parent_id: id.into(),
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
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Task [{}]({:?}): {}", self.id, self.status, self.command)
    }
}



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




#[cfg(test)]
mod tests {
    use task::{Task, Status}; 
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

}




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
    fn update_from(&mut self, other: &Self);
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
    fn update_from(&mut self, other: &Self){
        self.iter_mut()
            .zip(other.iter())
            .for_each(|(this, that)|{
                // Decide if the other task is considered newer or older based on
                // it's status first
                let mut should_update = match this.status{
                    Status::Waiting => {
                        match that.status{
                            Status::Waiting => false,
                            _ => true
                        }
                    },
                    Status::Queued => {
                        match that.status{
                            Status::Waiting => false,
                            Status::Queued => false,
                            _ => true
                        }
                    },
                    Status::Running => {
                        match that.status{
                            Status::Waiting => false,
                            Status::Queued => false,
                            Status::Running => false,
                            _ => true
                        }
                    },
                    Status::Paused => {
                        match that.status{
                            Status::Waiting => false,
                            Status::Queued => false,
                            Status::Running => false,
                            Status::Paused => false,
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
                        false
                    }
                };

                // Don't update if this task is constructed and the other isn't
                if should_update{
                    should_update = match this.command.is_constructed(){
                        true => {
                            // Don't update if this task is constructed and the \
                            // other isn't
                            match that.command.is_constructed(){
                                true => true,
                                false => false
                            }
                        },
                        false => {
                            // Always update if this command is (or isn't) \
                            // constructed and the other isn't
                            match that.command.is_constructed(){
                                true => true,
                                false => true
                            }
                        }
                    }
                }

                // Finally do the updatin' if all checks say yes
                if should_update{
                    *this = that.clone();
                }
            });
    }

    fn update_task_from(&mut self, task: &Task){
        // Find the other task in self:
        match self.position_by_id(task.id.as_str()){
            Some(index) => {
                self.remove(index);
                self.insert(index, task.clone());
            },
            None => ()
        }      
    }

}

