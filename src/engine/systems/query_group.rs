use crate::impl_all;

use super::query::{Query,QueryError,QueryCleanup};

macro_rules! impl_query_components {
    ($t1:tt $(,$t:tt)*) => {
        #[allow(non_snake_case)]
        impl<Args, $t1:Query<Args>,$($t:Query<Args>),*> Query<Args> for ($t1,$($t),*) {

            fn get(args:&mut Args) -> Result<Self,QueryError>{
                Ok(($t1::get(args)?,$($t::get(args)?),*))
            }

            fn available(args:&mut Args) -> bool {
                $t1::available(args) $(&& $t::available(args))*
            }
        }
        #[allow(non_snake_case)]
        impl<Args, $t1:Query<Args>,$($t:Query<Args>),*> QueryCleanup<Args> for ($t1,$($t),*) {

            fn cleanup(&mut self,args:&mut Args){
                let (ref mut $t1,$(ref mut $t),*)= self;
                $t1::cleanup($t1, args);$($t::cleanup($t,args));*
            }
        }
    };
}

impl_all!(impl_query_components);

