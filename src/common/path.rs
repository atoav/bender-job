//! The path module provides common path functions for the rest of the common \
//! modules. This mainly involves getting paths from the `./tests/resources`
//! directory
//!
//! 


use ::*;


/// Get a path to the resources in ./tests
#[allow(dead_code)]
pub fn get_resourcepath() -> PathBuf {
    let mut buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    buf.push("tests");
    buf.push("resources");
    buf
}


/// Get a path to the blendfiles folder in ./tests/resources
#[allow(dead_code)]
pub fn get_blendfilespath() -> PathBuf {
    let mut buf = get_resourcepath();
    buf.push("blendfiles");
    buf
}


/// Get a path to the blendfiles folder in ./tests/ressources/data
#[allow(dead_code)]
pub fn get_data_blendfilespath() -> PathBuf {
    let mut buf = get_resourcepath();
    buf.push("data");
    buf.push("blendfiles");
    buf
}


/// Get all the Blendfiles in `./tests/resources/blendfiles` and return a 
/// Vector of PathBufs
pub fn get_blendfiles() -> Vec<PathBuf>{
    let blendfilespath = get_blendfilespath();
    let paths = fs::read_dir(blendfilespath).unwrap();
    let mut vec = Vec::new();
    for entry in paths{
        let path = entry.expect("Unwrapping Entry failed").path();
        if path.is_file() {
            vec.push(path);
        }
    }
    vec
}

/// Get a Blendfile from `./tests/resources/blendfiles` by name, return a \
/// Option<PathBuf> for the file found. This is case sensitive!
pub fn get_blendfile_by_name<S>(name: S) -> Option<PathBuf> where S: Into<String>{
    let name = name.into();
    let blendfiles = get_blendfiles();
    // Find a blendfile with `name` in its filename and a Option<PathBuf> for the
    // first match
    blendfiles.into_iter()
              .filter(|path| path.file_name()
                                .unwrap()
                                .to_string_lossy()
                                .contains(name.as_str()))
              .collect::<Vec<PathBuf>>()
              .first()
              .cloned()

}


/// Get the path to a example blend file
#[allow(dead_code)]
pub fn get_blendfile() -> PathBuf {
    let mut p = get_data_blendfilespath();
    p.push("5873c0033e78b222bec2cb2a221487cf");
    p.push("untitled.blend");
    p
}


/// Get the path to a invalid example blend file
#[allow(dead_code)]
pub fn get_invalid_blendfile() -> PathBuf {
    let mut p = get_data_blendfilespath();
    p.push("9ac9b18f5e6d4f329acda411e3de8cde");
    p.push("invalid.blend");
    p
}


/// Get the path to a different example blend file
#[allow(dead_code)]
pub fn get_other_blendfile() -> PathBuf {
    let mut p = get_data_blendfilespath();
    p.push("7841becc23339d86ef0ec0a18e312ba1");
    p.push("a.blend");
    p
}




/// Get a Jobpath to the thing in resources
#[allow(dead_code)]
pub fn get_jobpath() -> String {
    let mut buf = get_data_blendfilespath();
    buf.push("5873c0033e78b222bec2cb2a221487cf");
    format!("{:?}", buf).replace("\"", "")
}


/// Get a Jobpath to a invalid blendfile
#[allow(dead_code)]
pub fn get_invalid_jobpath() -> String {
    let mut buf = get_data_blendfilespath();
    buf.push("9ac9b18f5e6d4f329acda411e3de8cde");
    format!("{:?}", buf).replace("\"", "")
}


/// Get a Jobpath to a different blendfile
#[allow(dead_code)]
pub fn get_other_jobpath() -> String {
    let mut buf = get_data_blendfilespath();
    buf.push("7841becc23339d86ef0ec0a18e312ba1");
    format!("{:?}", buf).replace("\"", "")
}
