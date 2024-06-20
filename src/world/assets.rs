use super::{events, geometry};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
pub fn get_next_id() -> usize {
    NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

pub trait Object: ObjectClone + Sync + Send {
    fn new() -> Self
    where
        Self: Sized;
    fn id(self: &Self) -> usize;
    fn kind(self: &Self) -> &String;
    fn position(self: &Self) -> &geometry::Vector;
    fn deletion_mark(self: &Self) -> bool;
    fn delete(self: &mut Self);
    fn update(self: &mut Self);
    fn event_handler(self: &Self, event: &mut dyn events::Event);
}

pub trait ObjectClone {
    fn clone_box(&self) -> Box<dyn Object>;
}

impl<T> ObjectClone for T
where
    T: 'static + Object + Clone,
{
    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Box<dyn Object> {
        self.clone_box()
    }
}

pub type ObjectList = Vec<Box<dyn Object>>;
