use std::fmt;

const MINUTES: i32 = 24 * 60;

pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = Clock::normalize(hours * 60 + minutes);
        Clock { hours: hours, minutes: minutes }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        (self.hours, self.minutes) = Clock::normalize(self.hours * 60 + self.minutes + minutes);
        self    
    }

    fn normalize(mut minutes: i32) -> (i32, i32) {
        while minutes < 0 {
            minutes += MINUTES;
        }
        (minutes / 60 % 24, minutes % 60)
        
    } 
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.minutes == other.minutes && self.hours == other.hours
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}