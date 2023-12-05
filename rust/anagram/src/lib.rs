use std::fmt;
use std::fmt::Formatter;

fn all_to_minutes(total_min: i32, day_min: i32) -> i32 {
    (total_min % day_min + day_min ) % day_min
}

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    mins: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}{:02}", self.hours, self.mins)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        todo!()
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        todo!("Add {minutes} minutes to existing Clock time");
    }
}
