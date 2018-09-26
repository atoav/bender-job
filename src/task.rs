use ::*;




/// A Task is what gets passed from watchdog to qu and from qu to worker. Tasks
/// are essentially atomic units of work, broken down so they can be fairly managed
/// by the queue.
///
/// This is basically a wrapper around a command, that allows us to keep track of
/// a Tasks status, its start and end times etc. It consists of:
/// - a [Status](enum.Status.html) which manages the States of a Task and the allowed transitions between it \
/// (e.g. a finished task cannot be aborted, a errored task cannot start etc.)
/// - a [JobTime](struct.JobTime.html) which allows to keep track of _when_ a certain state change has occured, as \
/// well as the calculation of durations (the same construct is used for [Job](struct.Job.html))
/// - a [Command](struct.Command.html) which allows to abstract CLI commands to be executed on the worker machines \
/// in such way, that we don't need to know input and output paths beforehand
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Task{
    pub status: Status,
    pub time: JobTime,
    pub command: Command
}


// Basic functions dealing with creation, serialization and deserialization
impl Task{
    /// Create a new Task with a basic command
    pub fn new_basic<S>(command: S) -> Self where S: Into<String>{
        Self{
            status: Status::Waiting,
            time: JobTime::new(),
            command: Command::new(command.into())
        }
    }

    /// Create a new Task for a single blender frame
    pub fn new_blender_single<S>(frame: usize, image_format: S) -> Self where S: Into<String>{
        Self{
            status: Status::Waiting,
            time: JobTime::new(),
            command: Command::new_blender_single(frame, image_format.into())
        }
    }

    /// Create a new Task for a range of blender frames
    pub fn new_blender_range<S>(start: usize, end: usize, step: usize, image_format: S) -> Self where S: Into<String>{
        Self{
            status: Status::Waiting,
            time: JobTime::new(),
            command: Command::new_blender_range(start, end, step, image_format.into())
        }
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

    /// Deserialize something that fullfills Into<String> into a Task
    pub fn deserialize<S>(s: S) -> GenResult<Self> where S: Into<String> {
        let deserialized: Self = serde_json::from_str(&s.into()[..])?;
        Ok(deserialized)
    }

    /// Deserialize something that fullfills Into<String> into a Task
    pub fn deserialize_from_u8(v:&[u8]) -> GenResult<Self> {
        let s = str::from_utf8(v)?;
        let deserialized: Self = serde_json::from_str(&s)?;
        Ok(deserialized)
    }

    /// Returns true if the Tasks command is a blender Task
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