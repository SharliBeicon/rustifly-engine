use std::any::Any;

pub trait Event: Any {
    fn as_any(&mut self) -> &mut dyn Any;
    fn get_kind(&self) -> &String;
    fn set_kind(&mut self, t: String);
}

pub struct EventStep {
    kind: String,
    pub step_count: i32,
}

impl EventStep {
    pub fn new() -> Self {
        Self {
            kind: String::from("rf::step"),
            step_count: 0,
        }
    }
}

impl Event for EventStep {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn get_kind(&self) -> &String {
        &self.kind
    }

    fn set_kind(&mut self, kind: String) {
        self.kind = kind;
    }
}
