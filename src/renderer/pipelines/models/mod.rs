pub mod square;
pub mod cube;

use std::any::Any;

use super::vertecies::MatrixVertexBufferable;

pub trait Model<V: MatrixVertexBufferable>: Send + Sync + Any {
    fn vertices(&self) -> Vec<V>;
    fn indexes(&self) -> Vec<u16>;
}
