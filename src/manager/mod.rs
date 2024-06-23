mod display_manager;
mod log_manager;
mod world_manager;

use crate::{
    utility,
    world::{events, EventStep},
};
use display_manager::*;
use std::{thread::sleep, time::Duration};

const REFRESH_RATE: u64 = 33333333;
pub struct GameManager {
    pub game_over: bool,
    clock: utility::Clock,

    pub log: log_manager::LogManager,
    pub world: world_manager::WorldManager,
    pub display: display_manager::DisplayManager,
}

impl GameManager {
    pub fn new() -> GameManager {
        GameManager {
            game_over: false,
            clock: utility::Clock::new(),
            log: log_manager::LogManager::new(),
            world: world_manager::WorldManager::new(),
            display: DisplayManagerBuilder::new()
                .with_title("Rustifly engine".to_string())
                .with_size(1024, 768)
                .build(),
        }
    }

    pub fn on_event(self: &mut Self, event: &mut dyn events::Event) -> i32 {
        let mut count = 0;

        self.world.objects_mut().iter().for_each(|object| {
            object.event_handler(event);
            count += 1;
        });

        count
    }

    pub fn run(&mut self) {
        let mut adjust_time: i32 = 0;
        let mut event = EventStep::new();

        while !self.game_over {
            self.clock.delta();
            /* **** MAIN GAME LOOP **** */

            self.on_event(&mut event);

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

            /* **** END MAIN GAME LOOP **** */
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
