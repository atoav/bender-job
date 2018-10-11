//! The data module defines multiple Structs used by the job to store data. E.g. \
//! Render, Frames and Resolution

use ::*;


/// The Render struct stores the render related data of a [Job](struct.Job.html). \
/// Typically it is deserialized by the Job with empty default values until \
/// the information is read via the Jobs [gaffer](trait.Gaffer.html) trait using \
/// its scan_and_optimize() method.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Render{
    pub renderer: String,
    pub cuda: bool,
    pub device: String,
    pub image_format: String,
    pub uses_compositing: bool
}

impl Render{
    /// Check if the Format is valid
    pub fn valid_format(&self) -> bool{
        let valid_formats: [&str; 12] = ["PNG", "BMP", "JPEG", "JPEG2000", "TARGA", "TARGA_RAW", "CINEON", "DPX", "OPEN_EXR_MULTILAYER", "OPEN_EXR", "HDR", "TIFF"];
        valid_formats.contains(&self.image_format.as_str())
    }

    /// Return true if self has still the default value
    pub fn is_default(&self) -> bool{
        self == &Self::default()
    }
}

/// The Frames struct stores frame related data of a [Job](struct.Job.html) (like start, end, current, step and fps). \
/// Typically it is deserialized by the Job with empty default values until \
/// the information is read via the Jobs [gaffer](trait.Gaffer.html) trait using \
/// its scan_and_optimize() method.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Frames {
    pub start: usize,
    pub end: usize,
    pub current: usize,
    pub step: usize,
    pub fps: usize
}

impl Frames {
    /// Return the number of frames in total. This honors the step size specified in the blend
    pub fn count(&self) -> usize {
        if self.is_default(){
            0
        }else{
            self.as_vec().len()
        }
    }

    // Return a Vec of frame numbers. This honors the step size specified in the blend
    pub fn as_vec(&self) -> Vec<usize> {
        (self.start..self.end+1).step_by(self.step).collect()
    }

    /// Return true if self has still the default value
    pub fn is_default(&self) -> bool{
        self == &Self::default()
    }
}


/// The Resolution struct stores and calculates resolution related data of a [Job](struct.Job.html). \
/// Typically it is deserialized by the Job with empty default values until \
/// the information is read via the Jobs [gaffer](trait.Gaffer.html) trait using \
/// its scan_and_optimize() method.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Resolution {
    pub x: usize,
    pub y: usize,
    pub scale: usize
}

impl Resolution {
    /// Returned the scaled (actual) width of the render output
    pub fn scaled_x(&self) -> usize {
        (self.x * self.scale)/100
    }

    /// Returned the scaled (actual) height of the render output
    pub fn scaled_y(&self) -> usize {
        (self.y * self.scale)/100
    }

    /// return the total number of pixels
    pub fn pixels(&self) -> i64{
        self.scaled_x() as i64 * self.scaled_y() as i64
    }

    /// Return true if self has still the default value
    pub fn is_default(&self) -> bool{
        self == &Self::default()
    }
}



/// Represents any Resource (Objects, Textures, Materials, ...) in the blendfile that
/// have been removed because they were unused. This is ultimately stored in a [Job](struct.Job.html). \
/// Typically it is deserialized by the Job with empty default values until \
/// the information is read via the Jobs [gaffer](trait.Gaffer.html) trait using \
/// its scan_and_optimize() method.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Resource {
    pub n: usize,
    pub removed : usize
}






// ================================ TEST RENDER ================================
#[cfg(test)]
mod render {
    use ::*;
    #[test]
    fn is_default() {
        let r = Render::default();
        assert_eq!(r.is_default(), true);
    }

    #[test]
    fn is_not_default() {
        let mut r = Render::default();
        r.renderer = "CYCLES".to_string();
        assert_eq!(r.is_default(), false);
    }

    #[test]
    fn format_is_valid() {
        let mut r = Render::default();
        r.image_format = "PNG".to_string();
        assert_eq!(r.valid_format(), true);
    }

    #[test]
    fn format_is_invalid() {
        let mut r = Render::default();
        r.image_format = "FOOOO".to_string();
        assert_eq!(r.valid_format(), false);
    }
}

// ================================ TEST FRAMES ================================
#[cfg(test)]
mod frames {
    use ::*;
    #[test]
    fn is_default() {
        let f = Frames::default();
        assert_eq!(f.is_default(), true);
    }

    #[test]
    fn is_not_default() {
        let mut f = Frames::default();
        f.end = 100;
        assert_eq!(f.is_default(), false);
    }

    #[test]
    fn basic_count() {
        let f = Frames{
            start: 1,
            end: 100,
            current: 2,
            step: 1,
            fps: 25
        };
        let v = f.as_vec();
        println!("{:?}", v);
        assert_eq!(f.count(), 100);
    }

    #[test]
    fn stepped_count() {
        let f = Frames{
            start: 1,
            end: 100,
            current: 2,
            step: 10,
            fps: 25
        };
        assert_eq!(f.count(), 10);
    }

    #[test]
    fn as_vec_length() {
        let f = Frames{
            start: 0,
            end: 100,
            current: 2,
            step: 10,
            fps: 25
        };
        let v = f.as_vec();
        println!("{:?}", v);
        assert_eq!(v.len(), 11);
    }

    #[test]
    fn as_vec_steps() {
        let f = Frames{
            start: 0,
            end: 100,
            current: 2,
            step: 10,
            fps: 25
        };
        let v1 = f.as_vec();
        let v2 = vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        assert_eq!(v1, v2);
    }

}

// ============================== TEST RESOLUTION ==============================
#[cfg(test)]
mod resolution {
    use ::*;
    #[test]
    fn is_default() {
        let r = Resolution::default();
        assert_eq!(r.is_default(), true);
    }

    #[test]
    fn is_not_default() {
        let mut r = Resolution::default();
        r.x = 1920;
        assert_eq!(r.is_default(), false);
    }

    #[test]
    fn scaled_resolution() {
        let r = Resolution {
            x: 2000,
            y: 1000,
            scale: 50
        };
        assert_eq!(r.scaled_x(), 1000);
        assert_eq!(r.scaled_y(), 500);
    }

    #[test]
    fn pixels() {
        let r = Resolution {
            x: 100,
            y: 100,
            scale: 100
        };
        assert_eq!(r.pixels(), 10000);
    }
}






#[cfg(test)]
mod resource{
    use super::*;

    #[test]
    fn deserialize(){
        let r = Resource{n: 4, removed: 2};
        let data = r#"{"n": 4,"removed": 2}"#;
        let string = serde_json::to_string(&r).expect("Resource: Serialization failed");
        println!("Resource: Attempting to unwrap this:\n{}\n", data);
        println!("Resource: Should look like this:\n{}\n", string);
        let deserialized: Resource = serde_json::from_str(data).expect("Resource: Unwrapping failed");
        assert_eq!(r, deserialized);
    }
}