use crate::clock::Clock;
use chrono::{DateTime, Utc};

pub struct FixedClock {
    time: DateTime<Utc>,
}

impl FixedClock {
    pub fn new(time: DateTime<Utc>) -> Self {
        Self { time }
    }
}

impl Clock for FixedClock {
    fn now(&self) -> DateTime<Utc> {
        self.time
    }
}
