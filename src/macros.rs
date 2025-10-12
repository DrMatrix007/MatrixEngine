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

    (mini $m:ident) => {
        $crate::impl_all!($m, A, B, C, D, E, F, G, H, I, J);
    };
}

#[macro_export]
macro_rules! count_types {
    () => { 0 };
    ($a:ident) => { 1 };
    ($a:ident, $b:ident) => { 2 };
    ($a:ident, $b:ident, $c:ident) => { 3 };
    ($a:ident, $b:ident, $c:ident, $d:ident) => { 4 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident) => { 5 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident) => { 6 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident) => { 7 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident) => { 8 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident) => { 9 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident) => { 10 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident) => { 11 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident) => { 12 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident) => { 13 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident) => { 14 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident) => { 15 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident) => { 16 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident) => { 17 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident) => { 18 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident, $s:ident) => { 19 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident, $s:ident, $t:ident) => { 20 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident, $s:ident, $t:ident, $u:ident) => { 21 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident, $s:ident, $t:ident, $u:ident, $v:ident) => { 22 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident, $s:ident, $t:ident, $u:ident, $v:ident, $w:ident) => { 23 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident, $s:ident, $t:ident, $u:ident, $v:ident, $w:ident, $x:ident) => { 24 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident, $s:ident, $t:ident, $u:ident, $v:ident, $w:ident, $x:ident, $y:ident) => { 25 };
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident, $s:ident, $t:ident, $u:ident, $v:ident, $w:ident, $x:ident, $y:ident, $z:ident) => { 26 };
}