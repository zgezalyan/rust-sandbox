use std::fmt;

fn main() {
    let mut my_clock = Clock::new(-2, 40);
	println!("{}", my_clock);
	my_clock.add_minutes(70);
	println!("{}", my_clock);
	my_clock.add_minutes(720);
	println!("{}", my_clock);
}

pub struct Clock {
	hours: i32,
	minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let total_minutes_in_a_day = 24 * 60;

        let total_minutes_normalized = ((total_minutes % total_minutes_in_a_day) + total_minutes_in_a_day) % total_minutes_in_a_day;
        
        let (h, m) = (total_minutes_normalized / 60, total_minutes_normalized % 60);
        
        Clock {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) {
        *self = Self::new(self.hours, self.minutes + minutes);
    }
}

impl fmt::Display for Clock {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {		
		write!(f, "{:02}:{:02}", self.hours, self.minutes)
	}
}