use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::manager::*;

pub mod manager;
mod utility;

lazy_static! {
    pub static ref MANAGERS: Mutex<Managers> = Mutex::new(Managers {
        log: Some(log_manager::LogManager::new()),
        game: Some(game_manager::GameManager::new()),
    });
}
