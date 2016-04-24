#[macro_export]
macro_rules! alt {
    ($src:expr, $($rest:tt)*) => (
        alt_parser!($src, $($rest)*);
    );
}

#[macro_export]
macro_rules! alt_parser {
    ($src:expr, $pat:path | $($rest:tt)*) => (
        alt_parser!($src, call!($pat) | $($rest)*);
    );

    ($src:expr, $pat:path) => (
        alt_parser!($src, call!($pat));
    );

    ($src:expr, $pat:ident!($($args:tt)*) | $($rest:tt)*) => ({
        let output = $pat!($src, $($args)*);
        match output.mat.is_ok() {
            true => output,
            false => alt_parser!(output.src, $($rest)*)
        }
    });

    ($src:expr, $pat:ident!($($args:tt)*)) => (
        $pat!($src, $($args)*)
    );
}
