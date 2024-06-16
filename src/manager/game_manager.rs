use crate::utility;
use std::{thread::sleep, time::Duration};

const REFRESH_RATE: u64 = 33333333;

pub struct GameManager {
    pub game_over: bool,
    clock: utility::Clock,
}

impl GameManager {
    pub fn new() -> GameManager {
        GameManager {
            game_over: false,
            clock: utility::Clock::new(),
        }
    }

    pub fn run(&mut self) {
        let mut adjust_time: i32 = 0;
        let mut cont = 0;
        while !self.game_over {
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

            cont += 1;
            if cont > 200 {
                println!("GAME OVER BITCH");
                self.game_over = true;
            }
        }
    }
}
