use crate::{
    dispatchers::{DispatchData, Dispatcher},
    scene::Scene, access::Access,
};


pub struct BoxedFunction<Q: DispatchData> {
    f: Box<dyn FnMut(Q)>,
}

impl<Q: DispatchData> BoxedFunction<Q> {
    pub fn call(&mut self, q: Q) {
        (self.f)(q);
    }
}

pub struct UnsafeBoxedDispatcher(Box<dyn Dispatcher<DispatchArgs = Scene>>,Access);

impl UnsafeBoxedDispatcher {
    pub unsafe fn get_ptr_mut(&mut self) -> *mut dyn Dispatcher<DispatchArgs = Scene> {
        self.0.as_mut()
    }

    pub(crate) fn get_mut(&mut self) -> &mut dyn Dispatcher<DispatchArgs = Scene> {
        self.0.as_mut()
    }
}



impl<T:Dispatcher<DispatchArgs = Scene>+'static> From<T> for UnsafeBoxedDispatcher {
    fn from(value: T) -> Self {
        let  access= value.access();
        UnsafeBoxedDispatcher(Box::new(value),access)
    }
}

unsafe impl Send for UnsafeBoxedDispatcher{}

impl UnsafeBoxedDispatcher {
    pub(crate) fn as_ref(&self) -> &(dyn Dispatcher<DispatchArgs = Scene>) {
        self.0.as_ref()
    }

    pub(crate) fn as_mut(&mut self) -> &mut (dyn Dispatcher<DispatchArgs = Scene>) {
        self.0.as_mut()
    }
    pub(crate) fn as_access(&self) -> &Access {
        &self.1
    }
    
}

pub trait System: Dispatcher<DispatchArgs = Scene> {
    type Query<'a>: DispatchData;

    fn run<'a>(&mut self, comps: Self::Query<'a>);
}
