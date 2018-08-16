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


/// Test the serialization and deserialization of a job
mod job_serialize_deserialize{
    use common;
    use job::Job;

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
        let serialize = j.serialize().unwrap();
        let utf8stream = serialize.as_bytes();
        // Deserialize via from &[u8]
        let deserialized = Job::deserialize_from_vec(utf8stream).expect("Deserialization via ::deserialize_from_vec() failed!");
        assert_eq!(deserialized, j);
    }
}

