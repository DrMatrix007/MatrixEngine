use self::query::QueryError;

pub mod query;

pub trait Dispatcher<Args> {
    fn dispatch(args: &mut Args) -> Result<Box<dyn Fn()>, QueryError>;
}

pub trait DispatcherSend<Args>: Dispatcher<Args> {
    fn dispatch(args: &mut Args) -> Result<Box<dyn Fn() + Send>, QueryError>;
}
