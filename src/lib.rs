use crate::manager::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

pub mod manager;
mod utility;

lazy_static! {
    pub static ref MANAGERS: Mutex<Managers> = Mutex::new(Managers::new());
}
