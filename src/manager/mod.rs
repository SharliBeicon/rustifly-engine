pub mod game_manager;
pub mod log_manager;

pub struct Managers {
    log: log_manager::LogManager,
    game: game_manager::GameManager,
}

impl Managers {
    pub fn new() -> Managers {
        Managers {
            log: log_manager::LogManager::new(),
            game: game_manager::GameManager::new(),
        }
    }

    pub fn get_log_manager(&self) -> &log_manager::LogManager {
        &self.log
    }

    pub fn get_game_manager(&self) -> &game_manager::GameManager {
        &self.game
    }
}
