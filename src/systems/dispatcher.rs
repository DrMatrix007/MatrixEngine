pub enum DispatchError {
    NotAvailable
}

pub trait Dispatcher<In,Out> {
    fn dispatch(input:&mut In) -> Result<Out,DispatchError>;
}
