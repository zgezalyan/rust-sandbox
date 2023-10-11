// 1. Implementing the Duration struct.
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

// 2. Implementing the Planet trait.
pub trait Planet {
    const EARTH_YEAR_SECONDS: f64 = 31_557_600.0; // 365.25 days in seconds
    const ORBITAL_PERIOD: f64;  // In Earth years

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / Self::EARTH_YEAR_SECONDS / Self::ORBITAL_PERIOD
    }
}

// 3. Implementing the structures for each planet and the Planet trait for them.
pub struct Mercury;
impl Planet for Mercury {
    const ORBITAL_PERIOD: f64 = 0.2408467;
}

pub struct Venus;
impl Planet for Venus {
    const ORBITAL_PERIOD: f64 = 0.61519726;
}

pub struct Earth;
impl Planet for Earth {
    const ORBITAL_PERIOD: f64 = 1.0;
}

pub struct Mars;
impl Planet for Mars {
    const ORBITAL_PERIOD: f64 = 1.8808158;
}

pub struct Jupiter;
impl Planet for Jupiter {
    const ORBITAL_PERIOD: f64 = 11.862615;
}

pub struct Saturn;
impl Planet for Saturn {
    const ORBITAL_PERIOD: f64 = 29.447498;
}

pub struct Uranus;
impl Planet for Uranus {
    const ORBITAL_PERIOD: f64 = 84.016846;
}

pub struct Neptune;
impl Planet for Neptune {
    const ORBITAL_PERIOD: f64 = 164.79132;
}

fn main() {
    let duration = Duration::from(1_000_000_000); // 1 billion seconds

    println!("Years passed on Mercury: {}", Mercury::years_during(&duration));
    println!("Years passed on Venus: {}", Venus::years_during(&duration));
    // ... repeat for other planets
}
