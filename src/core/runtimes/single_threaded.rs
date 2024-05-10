
use crate::core::{systems::Queryable};

use super::Runtime;

pub struct SingleThreaded;

impl<Q:Queryable+'static> Runtime<Q> for SingleThreaded {
    fn run(&mut self, systems: &mut crate::core::systems::SystemRegistry<Q>, queryable: &mut Q) {
        for sys in systems.send_systems_mut() {
            sys.run(queryable).unwrap();
        }
        for sys in systems.non_send_systems_mut() {
            sys.run(queryable).unwrap();
        }
    }

}
