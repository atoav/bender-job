use ::*;

// "request.untouched":                "waiting",
// "request.invalid":                  "denied",
// "request.errored":                  "errored",
// "request.checked":                  "checked",
// "request.scanned":                  "scanned",
// "request.atomized":                 "atomized",
// "job.queued":                       "queued",
// "job.running":                      "running",
// "job.finished":                     "finished",
// "job.canceled":                     "canceled"
// "job.errored":                      "errored"


pub type Status = String;


pub trait JobStatus{
    fn get(&self, index: usize) -> Option<String>;
    fn set<S>(&mut self, index: usize, value: S) where S: Into<String>;
    fn is_request(&self) -> bool;
    fn is_job(&self) -> bool;
    fn is_untouched(&self) -> bool;
    fn is_invalid(&self) -> bool;
    fn is_errored(&self) -> bool;
    fn is_unvalidated(&self) -> bool;
    fn is_validated(&self) -> bool;
    fn is_checked(&self) -> bool;
    fn is_scanned(&self) -> bool;
    fn is_atomized(&self) -> bool;
    fn is_queued(&self) -> bool;
    fn is_running(&self) -> bool;
    fn is_finished(&self) -> bool;
    fn is_canceled(&self) -> bool;
    fn to_job(&mut self);
    fn invalidate(&mut self);
    fn error(&mut self);
    fn validate(&mut self);
    fn scan(&mut self);
    fn atomize(&mut self);
    fn queue(&mut self);
    fn run(&mut self);
    fn finish(&mut self);
    fn cancel(&mut self);
}

impl JobStatus for Status{
    fn get(&self, index: usize) -> Option<String>{
        match self.split(".").collect::<Vec<&str>>().get(index){
            Some(element) => Some(element.to_string()),
            None => None
        }
    }

    fn set<S>(&mut self, index: usize, value: S) where S: Into<String>{
        let mut v = self
            .clone()
            .split(".")
            .collect::<Vec<&str>>()
            .iter()
            .map(|p|p.to_string())
            .collect::<Vec<String>>();
        v.remove(index);
        v.insert(index, value.into());
        self.clear();
        self.push_str(v.join(".").as_str());
    }

    fn is_request(&self) -> bool{
        self.get(0) == Some("request".to_string())
    }
    
    fn is_job(&self) -> bool{
        self.get(0) == Some("job".to_string())
    }

    fn is_untouched(&self) -> bool{
        self.get(1) == Some("untouched".to_string())
    }

    fn is_invalid(&self) -> bool{
        self.get(1) == Some("invalid".to_string())
    }

    fn is_errored(&self) -> bool{
        self.get(1) == Some("errored".to_string())
    }

    fn is_unvalidated(&self) -> bool{
        !self.is_checked()
    }

    fn is_validated(&self) -> bool{
        !self.is_unvalidated()
    }

    fn is_checked(&self) -> bool{
        match self.is_request(){
            true => {
                let checked = self.get(1) == Some("checked".to_string());
                let scanned = self.get(1) == Some("scanned".to_string());
                let atomized = self.get(1) == Some("atomized".to_string());
                checked || scanned || atomized
            },
            false => true
        }
    }

    fn is_scanned(&self) -> bool{
        match self.is_request(){
            true => {
                let scanned = self.get(1) == Some("scanned".to_string());
                let atomized = self.get(1) == Some("atomized".to_string());
                scanned || atomized
            },
            false => true
        }
    }

    fn is_atomized(&self) -> bool{
        match self.is_request(){
            true => self.get(1) == Some("atomized".to_string()),
            false => true
        }
    }

    fn is_queued(&self) -> bool{
        self.get(1) == Some("queued".to_string())
    }

    fn is_running(&self) -> bool{
        self.get(1) == Some("running".to_string())
    }

    fn is_finished(&self) -> bool{
        self.get(1) == Some("finished".to_string())
    }

    fn is_canceled(&self) -> bool{
        self.get(1) == Some("canceled".to_string())
    }

    fn to_job(&mut self){
        match self.is_request(){
            true => {
                if self.is_checked(){
                    self.set(0, "job");
                    self.set(1, "queued")
                }
            },
            false => ()
        }
    }

    fn invalidate(&mut self){
        match self.is_request(){
            true => self.set(1, "invalid"),
            false => ()
        }
    }

    /// Set to errored
    fn error(&mut self){
        self.set(1, "errored")
    }

    fn validate(&mut self){
        match self.is_request(){
            true => self.set(1, "checked"),
            false => ()
        }
    }

    fn scan(&mut self){
        match self.is_request(){
            true => self.set(1, "scanned"),
            false => ()
        }
    }

    fn atomize(&mut self){
        match self.is_request(){
            true => self.set(1, "atomized"),
            false => ()
        }
    }

    fn queue(&mut self){
        match self.is_job(){
            true => self.set(1, "queued"),
            false => {
                self.set(0, "job");
                self.set(1, "queued");
            }
        }
    }

    fn run(&mut self){
        match self.is_job(){
            true => self.set(1, "running"),
            false => ()
        }
    }

    fn finish(&mut self){
        match self.is_job(){
            true => self.set(1, "finished"),
            false => ()
        }
    }

    fn cancel(&mut self){
        match self.is_job(){
            true => self.set(1, "canceled"),
            false => ()
        }
    }

}




#[cfg(test)]
mod tests {
    use ::*; 

    #[test]
    fn get() {
        let s: Status = "request.untouched".to_string();
        assert_eq!(s.get(0), Some("request".to_string()));
        assert_eq!(s.get(1), Some("untouched".to_string()));
        assert_eq!(s.get(2), None);
    }

    #[test]
    fn is_untouched() {
        let stati: Vec<Status> = vec![
            "request.invalid".to_string(),
            "request.errored".to_string(),
            "request.checked".to_string(),
            "request.scanned".to_string(),
            "request.atomized".to_string(),
            "job.queued".to_string(),
            "job.running".to_string(),
            "job.finished".to_string(),
            "job.canceled".to_string()
        ];
        let s: Status = "request.untouched".to_string();
        assert_eq!(s.is_untouched(), true);

        assert!(stati.iter().all(|status| !status.is_untouched()));
    }

    #[test]
    fn is_job(){
        let s1: Status = "request.untouched".to_string();
        let s2: Status = "job.untouched".to_string();
        assert_eq!(s1.is_job(), false);
        assert_eq!(s2.is_job(), true);
    }

    #[test]
    fn is_request(){
        let s1: Status = "request.untouched".to_string();
        let s2: Status = "job.untouched".to_string();
        assert_eq!(s1.is_request(), true);
        assert_eq!(s2.is_request(), false);
    }

    #[test]
    fn invalidate(){
        let mut s: Status = "request.untouched".to_string();
        let x: Status = "request.invalid".to_string();
        s.invalidate();
        assert_eq!(s, x);
    }

    #[test]
    fn error(){
        let mut s: Status = "request.untouched".to_string();
        let x: Status = "request.errored".to_string();
        s.error();
        assert_eq!(s, x);
        let mut s: Status = "job.untouched".to_string();
        let x: Status = "job.errored".to_string();
        s.error();
        assert_eq!(s, x);
    }

    #[test]
    fn validate(){
        let mut s: Status = "request.untouched".to_string();
        let x: Status = "request.checked".to_string();
        s.validate();
        assert_eq!(s, x);
    }

    #[test]
    fn scan(){
        let mut s: Status = "request.untouched".to_string();
        let x: Status = "request.scanned".to_string();
        s.scan();
        assert_eq!(s, x);
    }

    #[test]
    fn atomize(){
        let mut s: Status = "request.untouched".to_string();
        let x: Status = "request.atomized".to_string();
        s.atomize();
        assert_eq!(s, x);
    }

    #[test]
    fn queue(){
        let mut s: Status = "request.checked".to_string();
        let x: Status = "job.queued".to_string();
        s.queue();
        assert_eq!(s, x);
    }

    #[test]
    fn run(){
        let mut s: Status = "job.queued".to_string();
        let x: Status = "job.running".to_string();
        s.run();
        assert_eq!(s, x);
    }

    #[test]
    fn finish(){
        let mut s: Status = "job.running".to_string();
        let x: Status = "job.finished".to_string();
        s.finish();
        assert_eq!(s, x);
    }

    #[test]
    fn cancel(){
        let mut s: Status = "job.running".to_string();
        let x: Status = "job.canceled".to_string();
        s.cancel();
        assert_eq!(s, x);
    }




}