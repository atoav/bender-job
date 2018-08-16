extern crate job;
extern crate chrono;
mod common;



/// Test a Jobs functions
mod job_functions{
    use common;

    #[test]
    fn display() {
        let j = common::get_job();
        let x = &format!("{}", j)[..];
        assert_eq!("Job [id: 1be554e1f51b804637326e3faf41d2c9][status: request.untouched]", x);    
    }
}



mod job_creation{
    use common;
    use job::Job;
    use std::path::PathBuf;

    /// Test if the Job read from PathBuf is equal to a job created via 
    /// the `common::get_job()` function
    #[test]
    fn from_pathbuf() {
        let p = PathBuf::from(&common::get_jobpath());
        let j = Job::from(p);
        let job = common::get_job();
        assert_eq!(j, job);
    }
}


/// Test the serialization and deserialization of a job
mod job_serialize_deserialize{
    use common;
    use job::Job;
    use std::path::PathBuf;

    #[test]
    fn roundtrip_via_string() {
        let j = common::get_job();
        // Serialize
        let serialized = j.serialize().unwrap();
        // Deserialize from String
        let deserialized = Job::from(serialized.clone());
        assert_eq!(deserialized, j);
    }

    #[test]
    fn roundtrip_via_refstring() {
        let j = common::get_job();
        // Serialize
        let serialized = j.serialize().unwrap();
        // Deserialize from &String
        let deserialized = Job::from(&serialized);
        assert_eq!(deserialized, j);
    }

    #[test]
    fn roundtrip_via_str() {
        let j = common::get_job();
        // Serialize
        let serialized = j.serialize().unwrap();
        // Deserialize from &str
        let deserialized = Job::from(&serialized[..]);
        assert_eq!(deserialized, j);
    }

    #[test]
    fn roundtrip_via_deserialize() {
        let j = common::get_job();
        // Serialize
        let serialized = j.serialize().unwrap();
        // Deserialize via deserialize method
        let deserialized = Job::deserialize(&serialized[..]).expect("Deserialization via ::deserialize() failed!");
        assert_eq!(deserialized, j);
    }

    #[test]
    fn roundtrip_via_u8vec() {
        let j = common::get_job();
        // Serialize
        let serialize = &(j.serialize_to_u8().unwrap());
        // Deserialize via from &[u8]
        let deserialized = Job::deserialize_from_u8(serialize).expect("Deserialization via ::deserialize_from_vec() failed!");
        assert_eq!(deserialized, j);
    }

    #[test]
    fn roundtrip_via_filesystem() {
        let j = common::get_job();
        // write
        j.write_to_file().unwrap();
        // Deserialize via from &[u8]
        let deserialized = Job::from(PathBuf::from(&j.paths.upload));
        assert_eq!(deserialized, j);
    }
}

