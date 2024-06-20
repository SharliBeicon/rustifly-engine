use crate::world::assets;

pub struct WorldManager {
    objects: assets::ObjectList,
}

impl WorldManager {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn insert_object(&mut self, object: Box<dyn assets::Object>) {
        self.objects.push(object)
    }

    pub fn objects_ref(&self) -> &assets::ObjectList {
        &self.objects
    }

    pub fn objects_mut(&mut self) -> &mut assets::ObjectList {
        &mut self.objects
    }

    pub fn objects_of_type(&self, kind: String) -> assets::ObjectList {
        self.objects
            .iter()
            .filter(|object| *object.kind() == kind)
            .cloned()
            .collect()
    }

    pub fn update(&mut self) {
        self.objects.retain(|object| !object.deletion_mark());
    }
}
