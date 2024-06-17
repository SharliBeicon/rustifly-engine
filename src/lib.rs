mod manager;
mod utility;
mod world;

use crate::manager::Managers;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref MANAGERS: Mutex<Managers> = Mutex::new(Managers::new());
}
