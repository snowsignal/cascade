#[macro_export]
macro_rules! cascade {
    ($i:ident : $e: expr; $($tail: tt)*) => {
        {
            let mut $i = $e;
            cascade!(@line $i, $($tail)*);
            $i
        };
    };
    ($e: expr; $($tail: tt)*) => {
        {
            let mut __tmp = $e;
            cascade!(@line __tmp, $($tail)*);
            __tmp
        };
    };
    (@line $i:ident, | $s: stmt; $($tail: tt)*) => {
        $s;
        cascade!(@line $i, $($tail)*);
    };
    (@line $i: ident, .. $v: ident = $e: expr; $($tail: tt)*) => {
        $i.$v = $e;
        cascade!(@line $i, $($tail)*);
    };
    (@line $i:ident, .. $v:ident += $e:expr; $($tail:tt)*) => {
        $i.$v += $e;
        cascade!(@line $i, $($tail)*);
    };
    (@line $i:ident, .. $v:ident -= $e:expr; $($tail:tt)*) => {
        $i.$v -= $e;
        cascade!(@line $i, $($tail)*);
    };
    (@line $i:ident, .. $v:ident *= $e:expr; $($tail:tt)*) => {
        $i.$v *= $e;
        cascade!(@line $i, $($tail)*);
    };
    (@line $i:ident, .. $($q: ident ($($e: expr),*)).+; $($tail: tt)*) => {
        $i.$($q($($e),*)).+;
        cascade!(@line $i, $($tail)*);
    };
    (@line $i:ident, .. $($q: ident ($($e: expr),*)).+?; $($tail: tt)*) => {
        $i.$($q($($e),*)).+?;
        cascade!(@line $i, $($tail)*);
    };
    (@line $i:ident,) => {};
    () => {}
}
