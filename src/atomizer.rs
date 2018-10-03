use ::*;
use std::iter::FromIterator;
use blake2::{Digest};

/// This Trait is implemented by a [Job](struct.Job.html) and deals with atomizing (aka splitting)
/// the Jobs blendfile into [Tasks](struct.Task.html).
pub trait Atomizer{
    fn atomize_to_tasks(&mut self);
    fn generate_commands(&self, chunk_size: usize) -> GenResult<VecDeque<Task>>;
}

impl Atomizer for Job{
    /// Genenerate Tasks for the command. The chunk size controls how many \
    /// Frames are grouped together if `job::animation == true`.
    fn atomize_to_tasks(&mut self){
        let chunk_size = 1;
        match self.generate_commands(chunk_size){
            Ok(tasks) => {
                self.tasks =  tasks;
                self.set_atomize();
            },
            Err(err) => println!("Error: {}", err)
        }
        
    }

    /// Generate a list of commands for a Job
    fn generate_commands(&self, chunk_size: usize) -> GenResult<VecDeque<Task>>{
        let mut frames = Vec::new();
        match Config::from_file(Config::location()){
            Ok(config) => {
                match config.get_salt(){
                    Ok(salt) => {
                        let message = salt+self.email.clone().as_str();
                        let mut hash = Blake2b::new();
                        hash.input(&message.into_bytes());
                        let hash = hex::encode(&hash.result());
                        let iformat = &self.render.image_format;
                        // Return the frame/frames depending on the split settings
                        match self.animation{
                            false => frames.push(self.frames.current),
                            true => self.frames.as_vec().iter().for_each(|frame| frames.push(*frame))
                        }
                        if chunk_size == 1{
                            // Run construct_command on every frame and return as a VecDeque<Task>
                            Ok(VecDeque::from_iter(frames.iter().map(|frame| Task::new_blender_single(*frame, iformat.clone(), hash.clone()))))
                        } else {
                            // Get a chunk of frames (a Vec<usize>) and map it to the construct_range_command
                            Ok(VecDeque::from_iter(frames.chunks(chunk_size as usize)
                                    .map(|frame_chunk| {
                                        let start = frame_chunk.iter().min().unwrap();
                                        let end = frame_chunk.iter().max().unwrap();
                                        let step = self.frames.step;
                                        debug_assert!(step >= self.frames.start);
                                        debug_assert!(step <= self.frames.end);
                                        Task::new_blender_range(*start, *end, step, iformat.clone(), hash.clone())
                                    })))
                        }
                    },
                    Err(err) => {
                        println!("Error: Couldn't read salt"); Err(err)
                    }
                }
            },
            Err(err) => {
                println!("Error: Couldn't read the config at {}", Config::location()); Err(err)
            }
        }
    }
}