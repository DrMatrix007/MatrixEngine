use super::{component::ComponentRegistry, systems::{Queryable, SystemRegistry}};

pub mod single_threaded;

pub trait Runtime<Q:Queryable> {
    fn run(&mut self,systems: &mut SystemRegistry<Q>,queryable: &mut Q);
}
