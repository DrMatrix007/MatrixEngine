use std::marker::PhantomData;
pub struct CastChecker<A: ?Sized, B: ?Sized>(PhantomData<A>, PhantomData<B>);


impl<A: Sized, B: ?Sized+'static> CastChecker<A, B> {
    pub fn new() -> Self {
        CastChecker(PhantomData, PhantomData)
    }
}


pub trait  CanBe<T: ?Sized + 'static>: Sized {
    fn check(self) -> bool
    where
        Self: Sized,
    {
        true
    }
}
impl<T, Trait: ?Sized + 'static> CanBe<Trait> for &'_ CastChecker<T, Trait> {
    fn check(self) -> bool
    where
        Self: Sized,
    {
        false
    }
}

