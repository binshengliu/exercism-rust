use std::fmt;
// use std::string;

const MINUTES_A_DAY: i32 = 60 * 24;

#[derive(Debug)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hour: i32, minute: i32) -> Clock {
        Clock {minutes: Clock::normalize_minutes(hour * 60 + minute)}
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Clock {
        Clock {minutes: Clock::normalize_minutes(self.minutes + minutes)}
    }

    fn normalize_minutes(minutes: i32) -> i32 {
        let mut normalized = minutes;
        while normalized < 0 {
            normalized += MINUTES_A_DAY;
        }
        normalized % MINUTES_A_DAY
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.minutes == other.minutes
    }
}
