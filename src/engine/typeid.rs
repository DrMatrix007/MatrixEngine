use std::any::TypeId;

pub trait TypeIDable {
    fn get_type_id() -> TypeId;
}

impl<T:'static> TypeIDable for T {
    fn get_type_id() -> TypeId {
        TypeId::of::<T>()
    }
}