use ::*;
use data::Resource;
use std::process::Command;
use std::path::Path;


/// A thing that implements the Gaffer trait can scan and optimize its own job \
/// run a path with a python file and incorporate the gathered info in it self. \
/// The most important struct implementing this trait is the [Job](struct;Job.html).
pub trait Gaffer{
    fn scan_and_optimize(&mut self);
    fn run_with_python<S>(path: S, python_path: S) -> GenResult<String>where S: Into<String>;
    fn incorporate_info(&mut self, info: MiscInfo);
}

/// The Gaffer trait is implemented by the [Job](struct;Job.html).
/// It gives the job the ability to scan its own blendfile for basic information \
/// about resolution, frames and renderer by executing the blendfile with \
/// a python script (optimize_blend.py)
impl Gaffer for Job{
    /// Execute the jobs blendfile with optimize_blend.py, gather data and optimize settings.
    fn scan_and_optimize(&mut self){
        // Use the local file for debug builds, use the installed file for release builds
        let python_path = if cfg!(debug_assertions) {
            format!("{}/src/optimize_blend.py", env!("CARGO_MANIFEST_DIR"))
        }else{
            "/usr/local/bin/optimize_blend.py".to_string()
        };
        if Path::new(&python_path).exists() {
            if self.status.is_validated(){
                // Run Blend with Python
                match Self::run_with_python(self.paths.blend.clone(), python_path.clone()){
                    Ok(output) =>{
                        // Deserialize from blender output
                        match MiscInfo::deserialize(&output[..]){
                            Ok(info) => {
                                self.incorporate_info(info);
                                self.set_scan();
                            },
                            Err(err) => {
                                let error_message = format!("Error: failed to deserialize output to MiscInfo:\n{}\nOutput:\n{}", err, output);
                                println!("{}", error_message);
                                self.set_error(error_message);
                            }
                        }
                    },
                    Err(err) =>{
                        let error_message = format!("Error: while running with {}: {}", python_path, err);
                        println!("{}", error_message);
                        self.set_error(error_message);
                    }
                }
            }else{
                let error_message = format!("Warning: Couldn't scan_and_optimize() because job wasn't validated");
                println!("{}", error_message);
                self.set_error(error_message);
            }
        }else{
            let error_message = format!("Error: Didn't find optimize_blend.py at {}\nYou might try to reinstall.", python_path);
            println!("{}", error_message); 
            self.set_error(error_message);
        }
    }


    /// Execute the checked blend-file at blend_path with the python file at python_path
    fn run_with_python<S>(path: S, python_path: S) -> GenResult<String>where S: Into<String>{
        let path = path.into();
        let python_path = python_path.into();
        // Pass variables as environment variables, let blender run optimize_blend.py
        // to set some things straight and save a new file
        let command = Command::new("blender")
                .arg("-b")
                .arg(path)
                .arg("--disable-autoexec")
                .arg("--python")
                .arg(python_path)
                .env("BENDER_OVERRIDEFORMAT", "PNG")
                .output()?;

        // Collect all lines starting with "{" for JSON
        let output: String = String::from_utf8(command.stdout)?
            .lines()
            .filter(|line|line.starts_with("{"))
            .collect();

        Ok(output)
    }


    /// Integrates the MiscInfo deserialized from the optimize_blend.py output
    /// into the Job's fields'
    fn incorporate_info(&mut self, info: MiscInfo){
        self.render = info.render.clone();
        self.frames = info.frames.clone();
        self.resolution = info.resolution.clone();
        self.incorporate_alternate_history(&mut info.history.clone())
    }

}





// ============================== MISCINFO STRUCT ==============================

/// This represents the info of the blendfile. In the optimize_blend() function
/// we run the blendfile with the optimize_blend.py as a argument. The optimize_blend.py
/// will gather some data, put it into a python dict, serialize it to JSON and print
/// it back to be read for the optimize_blend() function. This is the Rust equivalent
/// to that python dict and needs to mimic it exactly.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MiscInfo {
    pub render: Render,
    pub materials: Resource,
    pub objects: Resource,
    pub textures: Resource,
    pub frames: Frames,
    pub resolution: Resolution,
    pub history: History
}



impl MiscInfo {
    /// Deserialize something that fullfills Into<String> into a MiscInfo
    pub fn deserialize<S>(s: S) -> GenResult<Self> where S: Into<String> {
        let deserialized: Self = serde_json::from_str(&s.into()[..])?;
        Ok(deserialized)
    }

    /// Serialize a MiscInfo into a String. Return a Error if this fails
    pub fn serialize(&self) -> GenResult<String> {
        let string = serde_json::to_string_pretty(&self)?;
        Ok(string)
    }

    /// Serialize a MiscInfo into a Vec<u8>. Return a Error if this fails
    /// you might want to use this with a reference
    pub fn serialize_to_u8(&self) -> GenResult<Vec<u8>> {
        let string = serde_json::to_string_pretty(&self)?;
        Ok(string.into_bytes())
    }
}







#[cfg(test)]
mod miscinfo{
    use super::*;

    #[test]
    fn deserialize(){
        let data = r#"{"path":"/home/atoav/Blender/bender-job/tests/resources/data/blendfiles/jwxle4hc4xpc8go862hmxrecntw6ewnk/untitled.blend","render":{"renderer":"CYCLES","cuda":false,"device":"CPU","image_format":"PNG","uses_compositing":true},"materials":{"n":4,"removed":2},"objects":{"n":9,"removed":0},"textures":{"n":1,"removed":0},"frames":{"start":1,"end":250,"current":68,"step":1,"fps":25},"resolution":{"x":1920,"y":1080,"scale":50},"history":{"2018-09-26T09:37:55.393101+00:00":"optimize_blend.py: Sucessfully started blender with optimize_blend.py","2018-09-26T09:37:55.393124+00:00":"optimize_blend.py: Active scene.name='Scene'","2018-09-26T09:37:55.393157+00:00":"optimize_blend.py: active renderer is CYCLES","2018-09-26T09:37:55.393175+00:00":"optimize_blend.py: Found these cycles devices: <bpy_struct, CyclesDeviceSettings(\"Intel Core i7-6700K CPU @ 4.00GHz\")>","2018-09-26T09:37:55.393183+00:00":"optimize_blend.py: Error: Failed to set compute_device_type toCUDA","2018-09-26T09:37:55.440340+00:00":"optimize_blend.py: Stored changes in file at /home/atoav/Blender/bender-job/tests/resources/data/blendfiles/jwxle4hc4xpc8go862hmxrecntw6ewnk/untitled.blend"}}"#;
        assert!(match MiscInfo::deserialize(data){
            Ok(_info) => {
                true
            },
            Err(err) => {
                let error_message = format!("Error: failed to deserialize output to MiscInfo:\n{}\nOutput:\n{}", err, data);
                println!("{}", error_message);
                false
            }
        })
    }
}