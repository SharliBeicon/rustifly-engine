use rustifly_engine::{manager::GameManager, world::*};

#[tokio::main]
async fn main() {
    let game_manager = GameManager::new();
}

#[derive(Clone)]
struct Monster {
    id: usize,
    kind: String,
    position: Vector,
    deletion_mark: bool,
}

impl Monster {
    fn handle_step_event(&self, event: &mut dyn events::Event) {
        match event.as_any().downcast_mut::<EventStep>() {
            Some(e) => {
                e.step_count = e.step_count + 1;
                println!(
                    "Monster {0} took a step: {1} steps taken",
                    self.id, e.step_count
                );
            }
            None => println!("Not a step event"),
        }
    }
}

impl Object for Monster {
    fn new() -> Self {
        Self {
            id: get_next_id(),
            kind: String::from("Monster"),
            position: Vector::new(0.0, 0.0),
            deletion_mark: false,
        }
    }

    fn id(self: &Self) -> usize {
        self.id
    }

    fn kind(self: &Self) -> &String {
        &self.kind
    }

    fn position(self: &Self) -> &Vector {
        &self.position
    }

    fn update(self: &mut Self) {}

    fn event_handler(self: &Self, event: &mut dyn events::Event) {
        match &**event.get_kind() {
            "rf::step" => self.handle_step_event(event),
            _ => println!(
                "Event {0} for object {1} - {2} not recognized",
                event.get_kind(),
                self.kind,
                self.id
            ),
        }
    }

    fn deletion_mark(self: &Self) -> bool {
        self.deletion_mark
    }

    fn delete(self: &mut Self) {
        self.deletion_mark = true;
    }
}
