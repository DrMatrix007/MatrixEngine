use std::alloc::Layout;
use crate::components::{Component, ComponentType};

pub struct Column {
    #[cfg(debug_assertions)]
    component_type: ComponentType,
    layout: Layout,
    data: Vec<u8>
}

impl Column {
    fn new<C: Component>() -> Self {
        Self {
            #[cfg(debug_assertions)]
            component_type: ComponentType::from::<C>(),
            layout: Layout::new::<C>(),
            data: vec![]
        }
    }
    
    pub fn push<C:Component>(&mut self) {
        #[cfg(debug_assertions)]
        {
            assert_eq!(self.component_type, ComponentType::from::<C>());
            assert_eq!(self.layout, Layout::new::<C>());
        }


    }
}