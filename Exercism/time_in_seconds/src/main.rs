use time::{Duration, PrimitiveDateTime, Date, Time};

// Returns a DateTime one billion seconds after start.
pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    start + Duration::seconds(1_000_000_000)
}

fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> PrimitiveDateTime {
    PrimitiveDateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}

fn main() {
    let start_date = dt(2011, 4, 25, 0, 0, 0);
	let after_date = after(start_date);
	println!("Original: {}", start_date);
    println!("After one billion seconds: {}", after_date)
}
