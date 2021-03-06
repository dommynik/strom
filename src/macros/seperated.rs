#[macro_export]
macro_rules! seperated0 {
    ($src:expr, $pat:ident!($($args:tt)*), $sep:ident!($($args2:tt)*)) => ({
        let mut result = Vec::new();
        let mut i = $src;

        while let $crate::Output {src, mat: Ok(mat)} = $pat!(i, $($args)*) {
            let sep_output = $sep!(src, $($args2)*);
            i = sep_output.src;
            result.push(mat);
            if sep_output.mat.is_err() {
                break;
            }
        }

        $crate::Output::ok(
            i,
            $crate::Match::new(result, $src.pos)
        )
    });

    ($src:expr, $pat:ident!($($args:tt)*), $sep:path) => (
        seperated0!($src, $pat!($($args)*), call!($sep))
    );

    ($src:expr, $pat:path, $sep:ident!($($args2:tt)*)) => (
        seperated0!($src, call!($pat), $sep!($($args2)*))
    );

    ($src:expr, $pat:path, $sep:path) => (
        seperated0!($src, call!($pat), call!($sep))
    );
}

#[macro_export]
macro_rules! seperated1 {
    ($src:expr, $pat:ident!($($args:tt)*), $sep:ident!($($args2:tt)*)) => (
        match seperated0!($src, $pat!($($args)*), $sep!($($args2)*)) {
            $crate::Output {src, mat: Ok(mat)} => if mat.val.len() > 0 {
                $crate::Output::ok(src, mat)
            } else {
                $crate::Output::err($src, format!("No match."))
            },
            _ => $crate::Output::err($src, format!("Impossible."))
        }
    );

    ($src:expr, $pat:ident!($($args:tt)*), $sep:path) => (
        seperated1!($src, $pat!($($args)*), call!($sep))
    );

    ($src:expr, $pat:path, $sep:ident!($($args2:tt)*)) => (
        seperated1!($src, call!($pat), $sep!($($args2)*))
    );

    ($src:expr, $pat:path, $sep:path) => (
        seperated1!($src, call!($pat), call!($sep))
    );
}
