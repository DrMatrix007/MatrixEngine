#[macro_export]
macro_rules! impl_ops_unary {
    ($op:tt |$t:tt : &$ty:ty| -> $output:ty $b:block generic($($generics:tt)*)) => {
        $crate::impl_op_unary_internal!($op ($t : &$ty) -> $output $b $($generics)*);
        $crate::impl_op_unary_internal!($op ($t : $ty) -> $output $b $($generics)*);
    };
}

#[macro_export]
macro_rules! impl_op_unary_internal {
    (- ($t:tt : $ty:ty) -> $output:ty $b:block $($generics:tt)*) => {
        impl $($generics)* ::core::ops::Neg for $ty {
            type Output = $output;

            fn neg(self) -> $output {
                let $t = self;
                $b
            }

        }
    };
    (! ($t:tt : $ty:ty) -> $output:ty $b:block $($generics:tt)*) => {
        impl $($generics)* ::core::ops::Not for $ty {
            type Output = $output;

            fn not(self) -> $output {
                let $t = self;
                $b
            }

        }
    };
}

// Define your struct A<T>

// Use the macro to implement the unary Neg trait for A<T>
impl_ops_unary!(- |a: &A<T>| -> A<T> {A(-a.0)} generic(<T: num_traits::Float>));
impl_ops_unary!(! |_a: &A<T>| -> A<T> {A(T::zero())} generic(<T: num_traits::Float>));

#[macro_export]
macro_rules! impl_ops_binary {
    ($op:tt |$lhs:tt: ?$lty:ty, $rhs:tt: ?$rty:ty| -> $output:ty $b:block generic($($generics:tt)*)) => {
        $crate::impl_op_binary_internal!($op ($lhs: $lty, $rhs: $rty) -> $output $b $($generics)*);
        $crate::impl_op_binary_internal!($op ($lhs: &$lty, $rhs: &$rty) -> $output $b $($generics)*);
        $crate::impl_op_binary_internal!($op ($lhs: &$lty, $rhs: $rty) -> $output $b $($generics)*);
        $crate::impl_op_binary_internal!($op ($lhs: $lty, $rhs: &$rty) -> $output $b $($generics)*);
    };
}

#[macro_export]
macro_rules! impl_op_binary_internal {
    (+ ($lhs:tt: $lty:ty, $rhs:tt: $rty:ty) -> $output:ty $b:block $($generics:tt)*) => {
        #[allow(non_snake_case,)]
        impl $($generics)* ::core::ops::Add<$rty> for $lty {
            type Output = $output;

            fn add(self, rhs: $rty) -> $output {
                let ($lhs, $rhs) = (self, rhs);
                $b
            }
        }
    };
    (- ($lhs:tt: $lty:ty, $rhs:tt: $rty:ty) -> $output:ty $b:block $($generics:tt)*) => {
        impl $($generics)* ::core::ops::Sub<$rty> for $lty {
            type Output = $output;

            fn sub(self, rhs: $rty) -> $output {
                let ($lhs, $rhs) = (self, rhs);
                $b
            }
        }
    };
    (* ($lhs:tt: $lty:ty, $rhs:tt: $rty:ty) -> $output:ty $b:block $($generics:tt)*) => {
        impl $($generics)* ::core::ops::Mul<$rty> for $lty {
            type Output = $output;

            fn mul(self, rhs: $rty) -> $output {
                let ($lhs, $rhs) = (self, rhs);
                $b
            }
        }
    };
    (/ ($lhs:tt: $lty:ty, $rhs:tt: $rty:ty) -> $output:ty $b:block $($generics:tt)*) => {
        impl $($generics)* ::core::ops::Div<$rty> for $lty {
            type Output = $output;

            fn div(self, rhs: $rty) -> $output {
                let ($lhs, $rhs) = (self, rhs);
                $b
            }
        }
    };
    (% ($lhs:tt: $lty:ty, $rhs:tt: $rty:ty) -> $output:ty $b:block $($generics:tt)*) => {
        impl $($generics)* ::core::ops::Rem<$rty> for $lty {
            type Output = $output;

            fn rem(self, rhs: $rty) -> $output {
                let ($lhs, $rhs) = (self, rhs);
                $b
            }
        }
    };
}

// Define your struct A<T>
struct A<T: num_traits::Float>(pub T);

// Use the macro to implement the binary Add, Sub, Mul, Div, and Rem traits for A<T>
impl_ops_binary!(+ |a: ?A<T>, b: ?A<T>| -> A<T> { A(a.0 + b.0) } generic(<T: num_traits::Float>));
impl_ops_binary!(- |a: ?A<T>, b: ?A<T>| -> A<T> { A(a.0 - b.0) } generic(<T: num_traits::Float>));
impl_ops_binary!(* |a: ?A<T>, b: ?A<T>| -> A<T> { A(a.0 * b.0) } generic(<T: num_traits::Float>));
impl_ops_binary!(/ |a: ?A<T>, b: ?A<T>| -> A<T> { A(a.0 / b.0) } generic(<T: num_traits::Float>));
impl_ops_binary!(% |a: ?A<T>, b: ?A<T>| -> A<T> { A(a.0 % b.0) } generic(<T: num_traits::Float>));
