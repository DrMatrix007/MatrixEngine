use crate::engine::component::ComponentRegistry;

pub mod entity;
pub mod component;

pub struct Engine {
    registry: ComponentRegistry
}


impl Engine
{
    pub fn new() -> Self{
        Engine {
            registry: ComponentRegistry::default()
        }
    }


    pub fn run(&mut self)
    {

    }
}
