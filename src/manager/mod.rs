pub mod game_manager;
pub mod log_manager;

pub struct Managers {
    pub log: log_manager::LogManager,
    pub game: game_manager::GameManager,
}

impl Managers {
    pub fn new() -> Managers {
        Managers {
            log: log_manager::LogManager::new(),
            game: game_manager::GameManager::new(),
        }
    }
}
