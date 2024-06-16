pub mod game_manager;
pub mod log_manager;

use std::mem::replace;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManagerKind {
    LOG,
    DISPLAY,
    GAME,
    INPUT,
    RESOURCE,
    WORLD,
}

pub struct Managers {
    pub log: Option<log_manager::LogManager>,
    pub game: Option<game_manager::GameManager>,
}

impl Managers {
    pub fn take_log(&mut self) -> log_manager::LogManager {
        let m = replace(&mut self.log, None);
        m.unwrap()
    }

    pub fn take_game(&mut self) -> game_manager::GameManager {
        let m = replace(&mut self.game, None);
        m.unwrap()
    }
}
