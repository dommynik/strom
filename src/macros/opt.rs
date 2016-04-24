#[macro_export]
macro_rules! opt {
    ($src:expr, $pat:ident!($($args:tt)*)) => ({
        let $crate::Output {src, mat} = $pat!($src, $($args)*);
        match mat {
            Ok(mat) => $crate::Output::ok(
                src,
                $crate::Match::new(Some(mat.val), mat.pos)
            ),
            Err(_) => $crate::Output::ok(
                src,
                $crate::Match::new(None, src.pos)
            )
        }
    });

    ($src:expr, $pat:path) => (
        opt!($src, call!($pat))
    );
}
