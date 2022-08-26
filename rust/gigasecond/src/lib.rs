use time::{
    PrimitiveDateTime as DateTime,
    ext::NumericalDuration,
};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // do a checked add to the start date.
    // if the maximum time is passed, simply return the maximum allowed time.
    match start.checked_add(1000000000.seconds()) {
        Some(end_time) => return end_time,
        None => {
            println!("Time overflow!");
            println!("Returning a maximum time.");
            return time::Date::MAX.midnight();
        }
    }
}
