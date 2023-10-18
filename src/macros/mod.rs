#[macro_export]
macro_rules! impl_all {
    ($mac:ident, $t:ident, $($ts:ident),+) => {
        $mac!($t,$($ts),*);
        $crate::macros::impl_all!($mac,$($ts),+);
    };
    ($mac:ident, $t:ident) => {
        $mac!($t);
    };
    ($mac:ident) => {
        $crate::macros::impl_all!(
            $mac,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
            T,
            U,
            V,
            W,
            X,
            Y,
            Z
        );
    }
}
pub use impl_all;
