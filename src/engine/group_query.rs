use crate::impl_all;

use super::data_state::DataStateAccessError;
use super::entity::Entity;
use super::query::Query;
macro_rules! impl_queries {
    ($($t:tt)*) => {
        impl<Queryable, $($t:Query<Queryable>),*> Query<Queryable> for ($($t,)*) {
            fn check(q: &mut Queryable, e: &Entity) -> bool {
                ($($t::check(q,e))&&*)
            }
            fn query_unchecked(q: &mut Queryable, e: &Entity) -> Self {
                ($($t::query_unchecked(q,e),)*)
            }
            fn consume(self, q: &mut Queryable, e: &Entity) -> Result<(), DataStateAccessError> {
                #[allow(non_snake_case)]
                let ($($t,)*) = self;
                $($t::consume($t, q, e)?;)*
                Ok(())
            }
        }
    };
}

impl_all!(impl_queries);

#[cfg(test)]
mod tests {
    use crate::engine::{
        query::{ReadC, WriteC},
        scene::DummySceneRegistry,
        systems::{BoxedSendSystem, IntoNonSendSystem},
    };

    #[test]
    fn complex_queries() {
        let reg = <DummySceneRegistry>::new();
        let mut reg = reg.registry;

        let mut sys1 = BoxedSendSystem::from_system(
            (|_args: &mut (), (_, _): &mut (ReadC<()>, WriteC<i32>)| {}).into_system(),
        );
        let mut sys2 = BoxedSendSystem::from_system(
            (|_args: &mut (), _data: &mut (ReadC<()>, WriteC<i16>)| {}).into_system(),
        );
        let mut sys3 = BoxedSendSystem::from_system(
            (|_args: &mut (), _data: &mut (ReadC<()>, WriteC<i32>)| {}).into_system(),
        );
        sys1.prepare_args(&mut reg).unwrap();
        sys1.run(&mut ()).unwrap();

        sys2.prepare_args(&mut reg).unwrap();
        sys2.run(&mut ()).unwrap();

        sys3.prepare_args(&mut reg).unwrap_err();

        sys1.consume(&mut reg).unwrap();
        sys2.consume(&mut reg).unwrap();

        sys3.prepare_args(&mut reg).unwrap();
        sys3.run(&mut ()).unwrap();
        sys3.consume(&mut reg).unwrap();
    }
}
