extern crate bender_job;
extern crate chrono;
mod common;



/// Test a Jobs functions
mod job_functions{
    use common;

    #[test]
    fn display() {
        let j = common::get_job();
        let x = &format!("{}", j)[..];
        assert_eq!("Job [id: 5873c0033e78b222bec2cb2a221487cf][status: request.untouched]", x);    
    }

    #[test]
    fn add_history() {
        let mut j = common::get_job();
        j.add_history("Something very complex");
        // Get last element from history
        let (_key, value) = j.history.iter().next_back().unwrap();
        assert_eq!(value, "Something very complex");
    }

    #[test]
    #[should_panic]
    fn add_history_debounced1() {
        let mut j = common::get_job();
        j.add_history_debounced("Something very complex").unwrap();
        j.add_history_debounced("Something completely different").unwrap();
        j.add_history_debounced("Something completely different").expect("This should return Error");
    }

    #[test]
    fn add_history_debounced2() {
        let mut j = common::get_job();
        j.add_history_debounced("Something very complex").unwrap();
        j.add_history_debounced("Something completely different").unwrap();
        let result = match j.add_history_debounced("Something completely different"){
            Ok(()) => true,
            Err(_e) => false
        };
        assert!(!result);
        j.add_history_debounced("Something very complex").unwrap();
        // Get last element from history
        let (_key, value) = j.history.iter().next_back().unwrap();
        assert_eq!(value, "Something very complex");
    }

    #[test]
    fn add_data() {
        let mut j = common::get_job();
        j.add_data("somekey", "some1234567890foo");
        let value = j.data.get("somekey").unwrap();
        assert_eq!("some1234567890foo", value);
    }

    #[test]
    fn add_data_debounced1() {
        let mut j = common::get_job();
        j.add_data_debounced("somekey", "Something very complex").unwrap();
        j.add_data_debounced("somekey", "Something completely different").unwrap();
        j.add_data_debounced("somekey", "Something completely different").expect("This should return Error");       
    }

    #[test]
    fn add_data_debounced2() {
        let mut j = common::get_job();
        j.add_data_debounced("somekey", "Something very complex").unwrap();
        j.add_data_debounced("somekey", "Something completely different").unwrap();
        let result = match j.add_data_debounced("somekey", "Something completely different"){
            Ok(()) => true,
            Err(_e) => false
        };
        assert!(result);
        j.add_data_debounced("somekey", "Something very complex").unwrap();
        let value = j.data.get("somekey").unwrap();
        assert_eq!(value, "Something very complex");
    }

    /// Make sure this actually knows if a file changed on disk or not
    #[test]
    fn changed_on_disk() {
        let j = common::get_job();
        let mut x = common::get_job();
        j.write_to_file().expect("Couldn't write to file!");
        assert_eq!(j.changed_on_disk().unwrap(), false);
        x.add_data("somefield", "somedata");
        x.write_to_file().expect("Couldn't write to file!");
        assert_eq!(j.changed_on_disk().unwrap(), true);
        // Clean up after yourself
        j.write_to_file().expect("Couldn't write to file!");
    }

    /// Make sure this works when there is no change on disk
    #[test]
    fn update_on_disk_no_change() {
        let j = common::get_job();
        j.write_to_file().expect("Couldn't write to file!");
        let result = match j.update_on_disk(){
            Ok(()) => true,
            Err(_e) => false
        };
        assert_eq!(result, true);
    }

    /// Make sure this works when there is no change on disk
    #[test]
    fn update_on_disk_with_change() {
        let mut j = common::get_job();
        j.write_to_file().expect("Couldn't write to file!");
        let result = match j.update_on_disk(){
            Ok(()) => true,
            Err(_e) => false
        };
        assert_eq!(result, true);
        j.add_data("somekey", "foo");
        let result = match j.update_on_disk(){
            Ok(()) => true,
            Err(_e) => false
        };
        assert_eq!(result, true);
        assert_eq!(j.data.get("somekey").unwrap(), "foo");
        // Clean up after yourself
        let j = common::get_job();
        j.write_to_file().expect("Couldn't write to file!");
    }
}



mod job_creation{
    use common;
    use bender_job::Job;
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
    use bender_job::Job;

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
        // let deserialized = Job::from(PathBuf::from(&j.paths.upload));
        let deserialized = Job::from_datajson(&j.paths.data[..]).expect("Deserialization failed!");
        assert_eq!(deserialized, j);
    }
}

