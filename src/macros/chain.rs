#[macro_export]
macro_rules! chain {
    ($src:expr, $($rest:tt)*) => (
        chain_parser!($src, $src, $($rest)*)
    );
}

#[macro_export]
macro_rules! chain_parser {
    ($src:expr, $start:expr, $pat:path, $($rest:tt)*) => (
        chain_parser!($src, $start, call!($pat), $($rest)*);
    );

    ($src:expr, $start:expr, $pat:ident!($($args:tt)*), $($rest:tt)*) => (
        match $pat!($src, $($args)*) {
            $crate::Output {src, mat: Ok(_)} => chain_parser!(src, $start, $($rest)*),
            $crate::Output {src: _, mat: Err(err)} => $crate::Output::err($start, err)
        }
    );

    ($src:expr, $start:expr, $field:ident : $pat:path, $($rest:tt)*) => (
        chain_parser!($src, $start, $field: call!($pat), $($rest)*);
    );

    ($src:expr, $start:expr, $field:ident : $pat:ident!($($args:tt)*), $($rest:tt)*) => (
        match $pat!($src, $($args)*) {
            $crate::Output {src, mat: Ok(mat)} => {
                let $field = mat;
                chain_parser!(src, $start, $($rest)*)
            },
            $crate::Output {src: _, mat: Err(err)} => $crate::Output::err($start, err)
        }
    );

    ($src:expr, $start:expr, $pat:path, $assemble:expr) => (
        chain_parser!($src, $start, call!($pat), $assemble);
    );

    ($src:expr, $start:expr, $pat:ident!($($args:tt)*); $assemble:expr) => (
        match $pat!($src, $($args)*) {
            $crate::Output {src, mat: Ok(_)} =>
                $crate::Output::ok(src, $crate::Match::new($assemble(), $start.pos)),
            $crate::Output {src: _, mat: Err(err)} => $crate::Output::err($start, err)
        }
    );

    ($src:expr, $start:expr, $field:ident : $pat:path, $assemble:expr) => (
      chain_parser!($src, $start, $field: call!($pat), $assemble);
    );

    ($src:expr, $start:expr, $field:ident : $pat:ident!($($args:tt)*); $assemble:expr) => (
        match $pat!($src, $($args)*) {
            $crate::Output {src, mat: Ok(mat)} => {
                let $field = mat;
                $crate::Output::ok(src, $crate::Match::new($assemble(), $start.pos))
            },
            $crate::Output {src: _, mat: Err(err)} => $crate::Output::err($start, err)
        }
    );
}
