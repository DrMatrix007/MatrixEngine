use crate::scenes::scene::Scene;

pub enum QueryError {
    NotAvailable,
}

pub struct QueryArgs<'a> {
    scene: &'a mut Scene,
}

impl<'a> QueryArgs<'a> {
    pub fn new(scene: &'a mut Scene) -> Self {
        Self { scene }
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        self.scene
    }
    pub fn scene(&self) -> &Scene {
        &self.scene
    }
}

pub trait Query {
    type Target;
    fn query(args: &mut QueryArgs<'_>) -> Result<Self::Target, QueryError>;
}

// struct ReadComponent

