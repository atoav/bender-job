extern crate bender_job;

use bender_job::common::blendfiles::temporary;


fn main(){
    let (jobs, _tempdirs) = temporary::random::multi::create_n_jobs(10);
    for job in jobs {
        println!("{:#?}", job);
        println!("\n\n\n\n-------------------------------------------------------------\n\n\n\n")
    }
}