#[macro_export]
macro_rules! call {
    ($src:expr, $fun:expr) => (
        $fun($src)
    );

    ($src:expr, $fun:expr, $($args:expr),*) => (
        $fun($src, $($args),*)
    );
}

#[macro_export]
macro_rules! recognize {
    ($src:expr, $pat:ident!($($args:tt)*)) => (
        match $pat!($src, $($args)*) {
            $crate::Output {src, mat: Ok(mat)} => $crate::Output::ok(
                src, $crate::Match::new(&$src.src[..(src.pos - mat.pos)], $src.pos)),
            $crate::Output {src, mat: Err(err)} => $crate::Output::err(src, err)
        }
    );

    ($src:expr, $pat:path) => (
        recognize!($src, call!($pat))
    );
}

#[macro_export]
macro_rules! val {
    ($src:expr, $pat:ident!($($args:tt)*) => $value:expr) => (
        match $pat!($src, $($args)*) {
            $crate::Output {src, mat: Ok(mat)} => $crate::Output::ok(
                src, $crate::Match::new($value, mat.pos)),
            err @ $crate::Output {src: _, mat: Err(_)} => err
        }
    );
}

#[macro_export]
macro_rules! dbg {
    ($src:expr, $pat:ident!($($args:tt)*)) => ({
        let out = $pat!($src, $($args)*);
        println!("{} => {:#?}", stringify!($pat), out);
        out
    });

    ($src:expr, $pat:path) => ({
        dbg!($src, call!($pat))
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn call1() {
        function!(g, tag!("Hello"));
        function!(f, call!(g));

        let src = ::Source::new("Hello", 0);
        let out = f(src);
        let mat_exp = Ok(::Match::new("Hello", 0));
        let src_exp = ::Source::new("", 5);

        assert_eq!(out.src, src_exp);
        assert_eq!(out.mat, mat_exp);
    }
}
