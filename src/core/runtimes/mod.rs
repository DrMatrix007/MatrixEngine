use std::collections::vec_deque;

use super::{plugins::Plugin, scene::Scene, systems::{Queryable, SystemRegistry}};

pub mod single_threaded;

pub trait Runtime<Q:Queryable> {
    fn run(&mut self,systems: &mut SystemRegistry<Q>,queryable: &mut Q);
}
