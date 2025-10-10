use std::hash::{Hash, Hasher};

use crate::impl_all;
use paste::paste;
pub trait IDable: Hash + Eq + Clone + Copy {}
impl<T: Hash + Eq + Clone + Copy> IDable for T {}

pub struct IDWrapper<ID>(pub ID);
macro_rules! impl_idable_for_tuple {
    ( $( $t:ident ),+ ) => {
        #[allow(non_snake_case)]
        impl<$( $t: Hash + Eq + Clone + Copy ),+> Hash for IDWrapper<($( $t, )+)> {
            fn hash<Ha: Hasher>(&self, state: &mut Ha) {
                let ($( $t, )+) = &self.0;
                $( $t.hash(state); )+
            }
        }

        #[allow(non_snake_case)]
        impl<$( $t: Hash + Eq + Clone + Copy ),+> PartialEq for IDWrapper<($( $t, )+)> {
            fn eq(&self, other: &Self) -> bool {
                let ($( $t, )+) = &self.0;
                let ($( paste!([<$t _other>]), )+) = &other.0;
                true $( && $t == paste!([<$t _other>]) )+
            }
        }

        impl<$( $t: Hash + Eq + Clone + Copy ),+> Eq for IDWrapper<($( $t, )+)> {}

        #[allow(non_snake_case)]
        impl<$( $t: Hash + Eq + Clone + Copy ),+> Clone for IDWrapper<($( $t, )+)> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<$( $t: Hash + Eq + Clone + Copy ),+> Copy for IDWrapper<($( $t, )+)> {}
    };
}
impl_all!(impl_idable_for_tuple);
