#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let new_clock = Self{ hours, minutes };
        let corrected_clock = new_clock.correct_time();

        return corrected_clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Self::new(self.hours, self.minutes + minutes)
    }

    pub fn correct_time(&self) -> Self {
        // compute modulo and quotient of minutes with divisor 60
        let mut minutes_remainder: i32 = &self.minutes % 60;
        let minutes_quotient: i32 = &self.minutes / 60;
        
        // compute remainder of hours with divisor 24
        let mut hours_remainder: i32 = ( &self.hours + minutes_quotient ) % 24;

        // check for negative remainders, and correct if found
        if minutes_remainder < 0 { 
            minutes_remainder = 60 + minutes_remainder;
            hours_remainder = hours_remainder - 1;
        }
        if hours_remainder < 0 { hours_remainder = 24 + hours_remainder }

        // correct the time using the remainders and moduli
        return Self { hours: hours_remainder, minutes: minutes_remainder } 
    }
}

// implement Display trait for clock
impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
