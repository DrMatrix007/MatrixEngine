#[macro_export]
macro_rules! impl_all {
    ($mac:ident $t:tt $($ts:tt)+) => {
        $mac!($t $($ts)*);
        $crate::impl_all!($mac $($ts)+);
    };
    ($mac:ident $t:ident) => {
        $mac!($t);
    };
    ($mac:ident) => {
        $crate::impl_all!(
            $mac
            A
            B
            C
            D
            E
            F
            G
            H
            I
            J
            K
            L
            M
            N
            O
            P
            Q
            R
            S
            T
            U
            V
            W
            X
            Y
            Z
        );
    }
}
