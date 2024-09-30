use std::marker::PhantomData;

use paste::paste;

use crate::impl_all;

use super::data_state::DataStateAccessError;
use super::entity::SystemEntity;
use super::query::{Query, QueryError};
use super::systems::{
    FnNonSendPlaceHolderWithArgs, FnPlaceHolderNonSend, FnPlaceHolderSend,
    FnSendPlaceHolderWithArgs, IntoNonSendSystem, IntoSendSystem, QuerySystem, System, SystemError,
};

macro_rules! impl_queries {
    ($($t:tt)*) => {
        impl<Queryable, $($t:Query<Queryable>),*> Query<Queryable> for ($($t,)*) {
            fn query(q: &mut Queryable, e: &SystemEntity) -> Result<Self,DataStateAccessError> {
                let ($($t,)*)  =($($t::query(q,e),)*);
                match($($t,)*) {
                    ($(Ok($t),)*) => { Ok(($($t,)*)) },
                    ($($t,)*) => {
                        $(if let Ok($t) = $t {<$t>::consume($t,q,e);})*;
                        Err(DataStateAccessError::NotAvailableError)
                    }
                }
            }
            fn consume(self, q: &mut Queryable, e: &SystemEntity) -> Result<(), DataStateAccessError> {
                #[allow(non_snake_case)]
                let ($($t,)*) = self;
                $($t::consume($t, q, e)?;)*
                Ok(())
            }
        }
    };
}

impl_all!(impl_queries);

macro_rules! impl_systems {
    ($($t:tt)*) => {
        paste!{
        pub struct [<QuerySystemFn $($t)*>]<$($t:Query<Queryable>),*, Queryable, Fn: FnMut($(&mut $t),*)>(
            Fn,
            PhantomData<($($t,)* Queryable)>,
        );

        pub struct [<QuerySystemFnWithArgs $($t)*>]<
            $($t:Query<Queryable>,)*
            Queryable,
            EngineArgs,
            Fn: FnMut(&EngineArgs, $(&mut $t,)*),
        >(Fn, PhantomData<($($t,)* Queryable, EngineArgs)>);

        impl<$($t:Query<Queryable>,)* Queryable, EngineArgs, Fn: FnMut(&EngineArgs, $(&mut $t,)*)>
            [<QuerySystemFnWithArgs $($t)*>]<$($t,)* Queryable, EngineArgs, Fn>
        {
            pub fn new(f: Fn) -> Self {
                Self(f, PhantomData)
            }
        }

        impl<$($t:Query<Queryable>),*, Queryable, Fn: FnMut($(&mut $t,)*)> [<QuerySystemFn $($t)*>]<$($t,)* Queryable, Fn> {
            pub fn new(f: Fn) -> Self {
                Self(f, PhantomData)
            }
        }

        impl<$($t:Query<Queryable>,)* Queryable, Fn: FnMut($(&mut $t,)*), EngineArgs>
            QuerySystem<Queryable, EngineArgs> for [<QuerySystemFn $($t)*>]<$($t,)* Queryable, Fn>
        {
            type Query = ($($t,)*);

            fn run(&mut self, _engine_args: &mut EngineArgs, args: &mut Self::Query) {
                #[allow(non_snake_case)]
                let ($($t,)*) = args;
                (self.0)($($t,)*);
            }
        }

        impl<$($t:Query<Queryable>,)* Queryable, Fn: FnMut(&EngineArgs, $(&mut $t,)*), EngineArgs>
            QuerySystem<Queryable, EngineArgs>
            for [<QuerySystemFnWithArgs $($t)*>]<$($t,)* Queryable, EngineArgs, Fn>
        {
            type Query = ($($t,)*);

            fn run(&mut self, engine_args: &mut EngineArgs, args: &mut Self::Query) {
                #[allow(non_snake_case)]
                let ($($t,)*) = args;
                (self.0)(engine_args, $($t,)*);
            }
        }

        pub struct [<QuerySystemWrapper $($t)*>]<
            $($t:Query<Queryable>,)*
            Queryable,
            EngineArgs,
            QS: QuerySystem<Queryable, EngineArgs, Query = ($($t,)*)>,
        > {
            system: QS,
            args: Option<($($t,)*)>,
            marker: PhantomData<(Queryable, EngineArgs)>,
        }

        impl<
        $($t:Query<Queryable>,)*
                Queryable,
                EngineArgs,
                QS: QuerySystem<Queryable, EngineArgs, Query =  ($($t,)*)>,
            > [<QuerySystemWrapper $($t)*>]<$($t,)* Queryable, EngineArgs, QS>
        {
            pub fn new(system: QS) -> Self {
                Self {
                    system,
                    args: None,
                    marker: PhantomData,
                }
            }
        }
        impl<
            $($t:Query<Queryable>,)*
            Queryable,
            EngineArgs,
            QS: QuerySystem<Queryable, EngineArgs, Query = ($($t,)*)>,
        > System<Queryable, EngineArgs> for [<QuerySystemWrapper $($t)*>]<$($t,)* Queryable, EngineArgs, QS>
        {
            fn prepare_args(
                &mut self,
                queryable: &mut Queryable,
                system_id: &SystemEntity,
            ) -> Result<(), QueryError> {
                assert!(self.args.is_none());
                self.args = Some(<($($t,)*)>::query(queryable, system_id).map_err(|_|QueryError::NotAvailable)?);
                Ok(())
            }

            fn run(&mut self, engine_args: &mut EngineArgs) -> Result<(), SystemError> {
                let args = self.args.as_mut();
                if let Some(args) = args {
                    self.system.run(engine_args, args);
                    Ok(())
                } else {
                    Err(SystemError::MissingArgs)
                }
            }

            fn consume(
                &mut self,
                queryable: &mut Queryable,
                system_id: &SystemEntity,
            ) -> Result<(), DataStateAccessError> {
                self.args.take().unwrap().consume(queryable, system_id)
            }
        }



        impl<$($t:Query<Queryable>,)* Queryable, EngineArgs, Fn: FnMut($(&mut $t,)*)>
            IntoNonSendSystem<Queryable, EngineArgs, FnPlaceHolderNonSend<($($t,)*), Queryable>> for Fn
        {
            fn into_system(self) -> impl System<Queryable, EngineArgs> {
                [<QuerySystemWrapper $($t)*>]::new([<QuerySystemFn $($t)*>]::new(self))
            }
        }
        impl<
                $($t:Query<Queryable>+Send,)*
                Queryable: Send,
                EngineArgs: Send,
                Fn: FnMut($(&mut $t,)*) + Send,
            > IntoSendSystem<Queryable, EngineArgs, FnPlaceHolderSend<($($t,)*), Queryable>> for Fn
        {
            fn into_system(self) -> impl System<Queryable, EngineArgs> + Send {
                [<QuerySystemWrapper $($t)*>]::new([<QuerySystemFn $($t)*>]::new(self))
            }
        }
        impl<
                $($t:Query<Queryable>+Send,)*
                Queryable: Send,
                EngineArgs: Send,
                Fn: FnMut(&EngineArgs,$(&mut $t,)*) + Send,
            > IntoSendSystem<Queryable, EngineArgs, FnSendPlaceHolderWithArgs<($($t,)*), Queryable>> for Fn
        {
            fn into_system(self) -> impl System<Queryable, EngineArgs> + Send {
                [<QuerySystemWrapper $($t)*>]::new([<QuerySystemFnWithArgs $($t)*>]::new(self))
            }
        }


        impl<$($t:Query<Queryable>,)* Queryable, EngineArgs, Fn: FnMut(&EngineArgs, $(&mut $t,)*)>
            IntoNonSendSystem<Queryable, EngineArgs, FnNonSendPlaceHolderWithArgs<($($t,)*), Queryable>>
            for Fn
        {
            fn into_system(self) -> impl System<Queryable, EngineArgs> {
                [<QuerySystemWrapper $($t)*>]::new([<QuerySystemFnWithArgs $($t)*>]::new(self))
            }

        }
        }
    };
}

// impl_systems!(A B C);
impl_all!(impl_systems);
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
            (|_args: &mut (), _readc: &mut ReadC<()>, _write_i: &mut WriteC<i32>| {}).into_system(),
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
