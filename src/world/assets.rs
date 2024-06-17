use super::geometry;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

pub struct Object {
    id: usize,
    pub kind: String,
    pub position: geometry::Vector,
}

impl Object {
    pub fn new() -> Object {
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);

        Object {
            id,
            kind: String::from("DEFAULT"),
            position: geometry::Vector::new(0.0, 0.0),
        }
    }
}

pub type ObjectList = Vec<Object>;
