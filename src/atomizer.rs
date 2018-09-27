use ::*;
use std::iter::FromIterator;

/// This Trait is implemented by a [Job](struct.Job.html) and deals with atomizing (aka splitting)
/// the Jobs blendfile into [Tasks](struct.Task.html).
pub trait Atomizer{
    fn atomize(&mut self);
    fn generate_commands(&self, chunk_size: usize) -> VecDeque<Task>;
}

impl Atomizer for Job{
    /// Create Tasks for the Job
    /// 
    fn atomize(&mut self){
        self.tasks = self.generate_commands(1)
    }

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
            VecDeque::from_iter(frames.iter().map(|frame| Task::new_blender_single(*frame, iformat.clone())))       
        } else {
            // Get a chunk of frames (a Vec<usize>) and map it to the construct_range_command
            VecDeque::from_iter(frames.chunks(chunk_size as usize)
                    .map(|frame_chunk| {
                        let start = frame_chunk.iter().min().unwrap();
                        let end = frame_chunk.iter().max().unwrap();
                        let step = self.frames.step;
                        debug_assert!(step >= self.frames.start);
                        debug_assert!(step <= self.frames.end);
                        Task::new_blender_range(*start, *end, step, iformat.clone())
                    }))
        }
    }
}