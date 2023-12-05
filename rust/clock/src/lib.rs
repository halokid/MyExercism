use std::fmt;
use std::fmt::Formatter;

fn total_to_mins(total_mins: i32, day_mins: i32) -> i32 {
    (total_mins % day_mins + day_mins ) % day_mins
}

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    mins: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.mins)
    }
}

impl Clock {
    pub fn new(input_hours: i32, input_minutes: i32) -> Clock {
        let total_mins = input_hours * 60 + input_minutes;
        let day_mins = 60 * 24;
        let count_mins = total_to_mins(total_mins, day_mins);

        let hours = count_mins / 60;
        let mins = count_mins % 60;
        Clock{ hours, mins }
    }

    pub fn add_minutes(&self, minutes: i32) -> Clock {
        Clock::new(self.hours, self.mins + minutes)
    }
}



