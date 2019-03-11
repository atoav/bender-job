//! The history module defines functionality related to a Jobs history logging capabilities

use ::*;


// ===========================================================================
//                                History
// ===========================================================================

pub type History = BTreeMap<DateTime<Utc>, String>;



pub trait HistoryMethods{
    fn last(&self) -> Option<Event>;
    fn format_last(&self) -> String;
    fn last_message(&self) -> String;
}



impl HistoryMethods for History{
    fn last(&self) -> Option<Event>{
        match self.iter().next_back(){
            Some(e) => Some((e.0.clone(), e.1.clone())),
            None => None
        }
    }

    fn format_last(&self) -> String{
        match self.last(){
            Some(event) => event.format(),
            None => "".to_string()
        }
    }
    fn last_message(&self) -> String{
        match self.last(){
            Some(event) => event.message(),
            None => "".to_string()
        }
    }
}


// ===========================================================================
//                                Event
// ===========================================================================

pub type Event = (DateTime<Utc>, String);



pub trait EventMethods{
    fn format(&self) -> String;
    fn message(&self) -> String;
}



impl EventMethods for Event{
    fn format(&self) -> String{
        format!("[{}]: {}", self.0.to_string(), self.1)
    }

    fn message(&self) -> String{
        self.1.clone()
    }
}