#[macro_export]
macro_rules! impl_all {
    ($m:ident, $name:ident, $($names:ident),+) => {
        $m!($name, $($names),+);
        impl_all!($m, $($names),+);
    };
    ($m:ident, $name:ident) => {
        $m!($name);
    };

    ($m:ident) => {
        $crate::impl_all!($m, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
    };

    (no_single $m:ident, $name:ident, $($names:ident),+) => {
        $m!($name, $($names),+);
        impl_all!(no_single $m, $($names),+);
    };
    (no_single $m:ident, $name:ident) => {
    };

    (no_single $m:ident) => {
        $crate::impl_all!(no_single $m, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
    };
}