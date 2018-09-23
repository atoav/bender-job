use ::*;


/// A Task must fulfill these requirements:
/// - serialization/deserialization via serde
/// - keep track of time: start, end
/// - keep track of status: waiting, running, finished, errored, aborted, paused
/// - allow to construct commands on the fly


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Task{
    pub status: Status,
    pub time: JobTime,
    pub command: String
}


// Basic functions dealing with creation, serialization and deserialization
impl Task{
    pub fn new<S>(command: S) -> Self where S: Into<String>{
        Self{
            status: Status::Waiting,
            time: JobTime::new(),
            command: command.into()
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
}



impl Task{
    pub fn is_blender(&self) -> bool{
        self.command.starts_with("blender")
    }
}



// Methods dealing with Task.status
impl Task{
    /// Start the task (only if the task is waiting,)
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



/// A Tasks Status
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
        let t = Task::new("ls -a");
        assert_eq!(t.status, Status::Waiting);
        assert_eq!(t.time.start, None);
        assert_eq!(t.time.finish, None);
        assert_eq!(t.time.error, None);
    }

    #[test]
    fn serialize_deserialze() {
        let t1 = Task::new("ls -a");
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
        let t1 = Task::new("ls -a");
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
        let t = Task::new("blender -b");
        assert_eq!(t.is_blender(), true);
        let t = Task::new("ls -a");
        assert_eq!(t.is_blender(), false);
    }

}