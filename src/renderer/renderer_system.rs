
use crate::engine::systems::QuerySystem;

pub struct RendererSystem { 
}

impl QuerySystem for RendererSystem {
    type Query = ();

    fn run(&mut self, args: &mut <Self::Query as crate::engine::systems::query::Query<crate::engine::systems::query::ComponentQueryArgs>>::Target) {
        
    }
}