#[macro_export]
macro_rules! preceded_by {
    ($src:expr, $pat:ident!($($args:tt)*), $pat2:ident!($($args2:tt)*)) => (
        match $pat!($src, $($args)*) {
            $crate::Output {src, mat: Ok(mat)} =>
                match $pat2!(src, $($args2)*) {
                    $crate::Output {src: src2, mat: Ok(mat2)} =>
                        $crate::Output::ok(src2, $crate::Match::new(mat2.val, mat.pos)),
                    $crate::Output {src: _, mat: Err(err)} =>
                        $crate::Output::err($src, format!("Second pattern not found: {}", err))
                },
            $crate::Output {src: _, mat: Err(err)} =>
                $crate::Output::err($src, format!("First pattern not found: {}", err))
        }
    );

    ($src:expr, $pat:path, $pat2:ident!($($args2:tt)*)) => (
        preceded_by!($src, call!($pat), $pat2!($($args2)*))
    );

    ($src:expr, $pat:ident!($($args:tt)*), $pat2:path) => (
        preceded_by!($src, $pat!($($args)*), call!($pat2))
    );

    ($src:expr, $pat:path, $pat2:path) => (
        preceded_by!($src, call!($pat1), call!($pat2))
    );
}
