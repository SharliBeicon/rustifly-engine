use chrono::prelude::*;

pub fn current_time_string() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}

pub struct Clock {
    previous_time: u64,
}

impl Clock {
    pub fn new() -> Clock {
        Clock { previous_time: 0 }
    }

    pub fn delta(&mut self) -> u64 {
        let new_previous = Utc::now().timestamp_nanos_opt().unwrap() as u64;
        let duration = new_previous - self.previous_time;
        self.previous_time = new_previous;

        duration
    }

    pub fn split(&self) -> u64 {
        let new_previous = Utc::now().timestamp_nanos_opt().unwrap() as u64;

        new_previous - self.previous_time
    }
}
