use chrono::{DateTime, Utc};
use domain::clock::Clock;

pub struct RealClock;

impl Clock for RealClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
