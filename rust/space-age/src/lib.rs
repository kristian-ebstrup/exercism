// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
    years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration{ 
            seconds: s as f64,
            years: (s as f64) / (31_557_600 as f64)
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        // Mercury orbital period in Earth years
        let orbital_period = 0.2408467;

        // Return Mercury age based on input duration
        return d.years / orbital_period;
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        // Venus orbital period in Earth years
        let orbital_period = 0.6151972;

        // Return Mercury age based on input duration
        return d.years / orbital_period;
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        // Earth orbital period in Earth years
        let orbital_period = 1.0;

        // Return Mercury age based on input duration
        return d.years / orbital_period;
    }
}

impl Planet for Mars {

    fn years_during(d: &Duration) -> f64 {
        // Mars orbital period in Earth years
        let orbital_period = 1.8808158;

        // Return Mercury age based on input duration
        return d.years / orbital_period;
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        // Jupiter orbital period in Earth years
        let orbital_period = 11.862615;

        // Return Mercury age based on input duration
        return d.years / orbital_period;
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        // Saturn orbital period in Earth years
        let orbital_period = 29.447498;

        // Return Mercury age based on input duration
        return d.years / orbital_period;
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        // Uranus orbital period in Earth years
        let orbital_period = 84.016846;

        // Return Mercury age based on input duration
        return d.years / orbital_period;
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        // Neptune orbital period in Earth years
        let orbital_period = 164.79132;

        // Return Mercury age based on input duration
        return d.years / orbital_period;
    }
}
