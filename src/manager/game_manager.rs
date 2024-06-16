use crate::utility;
use std::{thread::sleep, time::Duration};

const REFRESH_RATE: u64 = 33333333;

pub struct GameManager {
    clock: utility::Clock,
}

impl GameManager {
    pub fn new() -> GameManager {
        GameManager {
            clock: utility::Clock::new(),
        }
    }

    pub fn run(&mut self) {
        let mut adjust_time: i32 = 0;
        loop {
            self.clock.delta();

            println!("GAME LOOP GOES HERE");

            let loop_time = self.clock.split();
            let intended_sleep_time =
                Duration::from_nanos(REFRESH_RATE - loop_time - adjust_time as u64);
            let intended_sleep_time_ns = intended_sleep_time.as_nanos();
            self.clock.delta();

            sleep(intended_sleep_time);

            let actual_sleep_time = self.clock.split();
            adjust_time = (actual_sleep_time - intended_sleep_time_ns as u64) as i32;

            if adjust_time < 0 {
                adjust_time = 0;
            }
        }
    }
}
