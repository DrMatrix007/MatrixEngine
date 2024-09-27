use std::any::Any;

use super::{components::Component, scene::SceneRegistry};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum QueryError {
    NotAvailable,
}



pub trait Query<Queryable>:Any {

    fn check(queryable: &mut Queryable) -> bool;

    fn query_unchecked(queryable: &mut Queryable) -> Self;

    fn query(queryable: &mut Queryable) -> Result<Self, QueryError> where Self: std::marker::Sized {
        if Self::check(queryable) {
            Ok(Self::query_unchecked(queryable))
        } else {
            Err(QueryError::NotAvailable)
        }
    }

    fn consume(self,queryable: &mut Queryable);
}

// impl<C:Component> Query<SceneRegistry> for C {
//     fn check(queryable: &mut SceneRegistry) -> bool {
//     }

//     fn query_unchecked(queryable: &mut SceneRegistry) -> Self {
//         todo!()
//     }

//     fn consume(self,queryable: &mut SceneRegistry) {
//         todo!()
//     }
// }



