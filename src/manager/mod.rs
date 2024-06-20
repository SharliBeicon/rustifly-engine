mod log_manager;
mod world_manager;
use once_cell::sync::Lazy;

use crate::{utility, world::EventStep};
use std::{sync::Mutex, thread::sleep, time::Duration};

pub static GAME_MANAGER: Lazy<Mutex<GameManager>> = Lazy::new(|| Mutex::new(GameManager::new()));

const REFRESH_RATE: u64 = 33333333;
pub struct GameManager {
    pub game_over: bool,
    clock: utility::Clock,

    pub log: log_manager::LogManager,
    pub world: world_manager::WorldManager,
}

impl GameManager {
    fn new() -> GameManager {
        GameManager {
            game_over: false,
            clock: utility::Clock::new(),
            log: log_manager::LogManager::new(),
            world: world_manager::WorldManager::new(),
        }
    }

    pub fn run(&mut self) {
        let mut adjust_time: i32 = 0;
        let mut event = EventStep::new();

        while !self.game_over {
            self.clock.delta();
            /* ********* */

            self.world.objects_mut().iter().for_each(|object| {
                object.event_handler(&mut event);
            });

            if self.world.objects_ref().is_empty() {
                self.game_over = true;
            }

            if event.step_count > 200 {
                if let Some(n) = self.world.objects_mut().get_mut(0) {
                    n.delete();
                }

                event.step_count = 0;
            }

            self.world.update();
            /* ********* */
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
