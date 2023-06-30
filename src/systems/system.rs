use super::query::Query;

pub trait System {
    type Query: Query;
    
}
