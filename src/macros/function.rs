#[macro_export]
macro_rules! function {
    ($name:ident -> $out:ty, $pat:ident!($($args:tt)*)) => (
        fn $name(i: $crate::Source) -> $crate::Output<$out> {
            $pat!(i, $($args)*)
        }
    );

    (pub $name:ident -> $out:ty, $pat:ident!($($args:tt)*)) => (
        pub fn $name(i: $crate::Source) -> $crate::Output<$out> {
            $pat!(i, $($args)*)
        }
    );

    ($name:ident, $pat:ident!($($args:tt)*)) => (
        fn $name(i: $crate::Source) -> $crate::Output<&str> {
            $pat!(i, $($args)*)
        }
    );

    (pub $name:ident, $pat:ident!($($args:tt)*)) => (
        pub fn $name(i: $crate::Source) -> $crate::Output<&str> {
            $pat!(i, $($args)*)
        }
    );
}

#[cfg(test)]
mod tests {

    #[test]
    fn function1() {
        function!(f, tag!("Hello"));

        let src = ::Source::new("Hello", 0);
        let out = f(src);
        let mat_exp = Ok(::Match::new("Hello", 0));
        let src_exp = ::Source::new("", 5);

        assert_eq!(out.src, src_exp);
        assert_eq!(out.mat, mat_exp);
    }
}
