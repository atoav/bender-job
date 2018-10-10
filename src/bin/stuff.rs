extern crate bender_job;

use bender_job::common::blendfiles::temporary::random::multi::{create_n_completely_random_jobs};


fn main(){
    let jobs = create_n_completely_random_jobs(10);
    for job in jobs {
        println!("{:#?}", job);
        println!("\n\n\n\n-------------------------------------------------------------\n\n\n\n")
    }
}