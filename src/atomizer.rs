//! The atomizer module is responsible for extending Job with the functionality \
//! to generate its own tasks (by splitting the render job into atomic units of \
//! work)  
//!
//! It does so by creating the Atomizer trait
use ::*;
use std::iter::FromIterator;

/// This Trait is implemented by a [Job](struct.Job.html) and deals with atomizing (aka splitting)
/// the Jobs blendfile into [Tasks](struct.Task.html).
pub trait Atomizer{
    fn atomize_to_tasks(&mut self);
    fn generate_commands(&self, chunk_size: usize) -> VecDeque<Task>;
}

impl Atomizer for Job{
    /// Genenerate Tasks for the command. The chunk size controls how many \
    /// Frames are grouped together if `job::animation == true`.
    fn atomize_to_tasks(&mut self){
        let chunk_size = 1;
        self.tasks = self.generate_commands(chunk_size);
        self.set_atomize();
    }

    /// Generate a list of commands for a Job
    fn generate_commands(&self, chunk_size: usize) -> VecDeque<Task>{
        let mut frames = Vec::new();
        let iformat = &self.render.image_format;
        // Return the frame/frames depending on the split settings
        match self.animation{
            false => frames.push(self.frames.current),
            true => self.frames.as_vec().iter().for_each(|frame| frames.push(*frame))
        }
        if chunk_size == 1{
            // Run construct_command on every frame and return as a VecDeque<Task>
            VecDeque::from_iter(frames.iter().map(|frame| Task::new_blender_single(*frame, iformat.clone(), self.id.clone())))
        } else {
            // Get a chunk of frames (a Vec<usize>) and map it to the construct_range_command
            VecDeque::from_iter(frames.chunks(chunk_size as usize)
                    .map(|frame_chunk| {
                        let start = frame_chunk.iter().min().unwrap();
                        let end = frame_chunk.iter().max().unwrap();
                        let step = self.frames.step;
                        debug_assert!(self.frames.start <= self.frames.end);
                        debug_assert!(step > 0);
                        Task::new_blender_range(*start, *end, step, iformat.clone(), self.id.clone())
                    }))
        }
    }
}