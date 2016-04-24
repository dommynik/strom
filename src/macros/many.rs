#[macro_export]
macro_rules! many0 {
    ($src:expr, $pat:ident!($($args:tt)*)) => ({
        let mut result = Vec::new();
        let mut i = $src;

        while let $crate::Output {src, mat: Ok(mat)} = $pat!(i, $($args)*) {
            i = src;
            result.push(mat)
        }

        $crate::Output::ok(
            i,
            $crate::Match::new(result, $src.pos)
        )
    });

    ($src:expr, $pat:path) => (
        many0!($src, call!($pat))
    );
}

#[macro_export]
macro_rules! many1 {
    ($src:expr, $pat:ident!($($args:tt)*)) => (
        match many0!($src, $pat!($($args)*)) {
            $crate::Output {src, mat: Ok(mat)} => if mat.val.len() > 0 {
                $crate::Output::ok(src, mat)
            } else {
                $crate::Output::err($src, format!("No match."))
            },
            _ => $crate::Output::err($src, format!("Impossible."))
        }
    );

    ($src:expr, $pat:path) => (
        many1!($src, call!($pat))
    );
}
