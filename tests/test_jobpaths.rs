extern crate bender_job;
extern crate chrono;
mod common;


/// This module tests the creation of a `JobPaths` struct via its `from_uploadfolder()` function
mod from_uploadfolder{
    use bender_job::JobPaths;
    use std::path::PathBuf;
    use common;

    #[test]
    fn uploadpath() {
        // Create a path for uploaddir
        let uploadpath = common::get_jobpath();
        // Run the actual folder
        let paths = JobPaths::from_uploadfolder(uploadpath.clone());
        println!("{}", paths);
        assert_eq!(paths.upload, uploadpath);
    }

    #[test]
    fn datapath() {
        // Create a path for uploaddir
        let uploadpath = common::get_jobpath();
        // Create a path for data.json
        let mut databuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        databuf.push("tests");
        databuf.push("resources");
        databuf.push("data");
        databuf.push("blendfiles");
        databuf.push("1be554e1f51b804637326e3faf41d2c9");
        databuf.push("data.json");
        let datapath = format!("{:?}", databuf).replace("\"", "");
        // Run the actual folder
        let paths = JobPaths::from_uploadfolder(uploadpath.clone());
        println!("{}", paths);
        assert_eq!(paths.upload, uploadpath);
        assert_eq!(paths.data, datapath);
    }

    #[test]
    fn blendpath() {
        // Create a path for uploaddir
        let uploadpath = common::get_jobpath();
        // Create a path for horizon_splash.blend
        let mut blendbuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        blendbuf.push("tests");
        blendbuf.push("resources");
        blendbuf.push("data");
        blendbuf.push("blendfiles");
        blendbuf.push("1be554e1f51b804637326e3faf41d2c9");
        blendbuf.push("horizon_splash.blend");
        let blendpath = format!("{:?}", blendbuf).replace("\"", "");
        // Run the actualfolder
        let paths = JobPaths::from_uploadfolder(uploadpath.clone());
        println!("{}", paths);
        assert_eq!(paths.upload, uploadpath);
        assert_eq!(paths.blend, blendpath);
    }

    #[test]
    fn framepath() {
        // Create a path for uploaddir
        let uploadpath = common::get_jobpath();
        // Create a path for frames
        let mut framebuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        framebuf.push("tests");
        framebuf.push("resources");
        framebuf.push("data");
        framebuf.push("frames");
        framebuf.push("1be554e1f51b804637326e3faf41d2c9");
        let framepath = format!("{:?}", framebuf).replace("\"", "");
        // Run the actualfolder
        let paths = JobPaths::from_uploadfolder(uploadpath.clone());
        println!("{}", paths);
        assert_eq!(paths.upload, uploadpath);
        assert_eq!(paths.frames, framepath);
    }

    #[test]
    fn filename() {
        // Create a path for uploaddir
        let uploadpath = common::get_jobpath();
        // Run the actualfolder
        let paths = JobPaths::from_uploadfolder(uploadpath.clone());
        println!("{}", paths);
        assert_eq!(paths.upload, uploadpath);
        assert_eq!(paths.filename, "horizon_splash.blend".to_owned());
    }
}


/// This module tests additional functions specified within the JobPath object
mod test_jobpath_functions{
    use bender_job::JobPaths;
    use common;

    #[test]
    fn get_id() {
        // Create a path for uploaddir
        let uploadpath = common::get_jobpath();
        // Run the actualfolder
        let paths = JobPaths::from_uploadfolder(uploadpath.clone());
        println!("{}", paths);
        assert_eq!(paths.upload, uploadpath);
        assert_eq!(paths.get_id(), "1be554e1f51b804637326e3faf41d2c9".to_owned());
    }
}