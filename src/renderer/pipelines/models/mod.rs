pub mod square;

use std::any::Any;

use super::vertecies::Vertexable;

pub trait Model<V: Vertexable>: Send + Sync + Any {
    fn vertices(&self) -> &'static [V];
    fn indexes(&self) -> &'static [u16];
}
