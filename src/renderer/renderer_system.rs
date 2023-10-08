use winit::window::Window;

use crate::engine::{
    scenes::resources::Resource,
};

impl Resource for Window {}



pub struct RendererSystem {}

// impl QuerySystem for RendererSystem {
//     type Query = ReadR<Window>;

//     fn run(
//         &mut self,
//         args: &mut <Self::Query as crate::engine::systems::query::Query<
//             crate::engine::systems::query::ComponentQueryArgs,
//         >>::Target,
//     ) {
//     }
// }
