use crate::engine::{
    scenes::components::{transform::Transform, Component},
    systems::{query::components::WriteC, QuerySystem},
};

pub struct ParticleSystem;

impl QuerySystem for ParticleSystem {
    type Query = (WriteC<Particle>, WriteC<Transform>);

    fn run(
        &mut self,
        events: &crate::engine::events::event_registry::EventRegistry,
        args: &mut Self::Query,
    ) -> crate::engine::systems::SystemControlFlow {
        crate::engine::systems::SystemControlFlow::Continue
    }
}

pub struct Particle {}

impl Component for Particle {}
