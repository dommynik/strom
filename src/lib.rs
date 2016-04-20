#[derive(Debug, Clone, Copy)]
pub struct Input<'a> {
    pub pos: usize,
    pub src: &'a str,
}

impl<'a> Input<'a> {
    pub fn new(pos: usize, src: &'a str) -> Input<'a> {
        Input { pos: pos, src: src }
    }
}

#[derive(Debug)]
pub struct Output<'a, T> {
    pub pos: usize,
    pub src: &'a str,
    pub hit: T,
}

impl<'a, T> Output<'a, T> {
    pub fn new(pos: usize, src: &'a str, hit: T) -> Output<'a, T> {
        Output { pos: pos, src: src, hit: hit }
    }
}

#[macro_export]
macro_rules! tag {
    ($i:expr, $pattern:expr) => (
        if $i.src.starts_with($pattern) {
            let (hit, src) = $i.src.split_at($pattern.len());
            Some($crate::Output::new($i.pos, src, hit))
        } else {
            None
        }
    );

    ($i:expr, cl $pattern:expr) => (
        if $i.src.len() < $pattern.len() {
            None
        } else {
            let cl_src = $i.src[0..$pattern.len()].to_lowercase();
            let cl_pat = $pattern.to_lowercase();

            if cl_src == cl_pat {
                let (hit, src) = $i.src.split_at($pattern.len());
                Some($crate::Output::new($i.pos, src, hit))
            } else {
                None
            }
        }
    );
}

#[macro_export]
macro_rules! call {
    ($i:expr, $fun:expr) => (
        $fun($i)
    );

    ($i:expr, $fun:expr, $($args:expr),*) => (
        $fun($i, $($args),*)
    );
}

#[macro_export]
macro_rules! alt {
    ($i:expr, $($rest:tt)*) => (
        alt_parser!($i, $($rest)*);
    );
}

#[macro_export]
macro_rules! alt_parser {
    ($i:expr, $pattern:ident | $($rest:tt)*) => (
        alt_parser!($i, call!($pattern) | $($rest)*);
    );

    ($i:expr, $pattern:ident) => (
        alt_parser!($i, call!($pattern));
    );

    ($i:expr, $pattern:ident!( $($args:tt)* ) | $($rest:tt)*) => ({
        let result = $pattern!($i, $($args)*);
        match result {
            Some(_) => result,
            None => alt_parser!($i, $($rest)*)
        }
    });

    ($i:expr, $pattern:ident!( $($args:tt)* )) => (
        $pattern!($i, $($args)*)
    );
}

#[macro_export]
macro_rules! preceded_by {
    ($i:expr, $pattern:ident!($($args:tt)*) + $pattern2:ident!($($args2:tt)*)) => (
        match $pattern!($i, $($args)*) {
            Some(out) => match $pattern2!($crate::Input::new(out.pos, out.src), $($args2)*) {
                Some(out2) => Some($crate::Output::new(out2.pos, out2.src, out2.hit)),
                None => None
            },
            None => None
        }
    );
}

#[macro_export]
macro_rules! followed_by {
    ($i:expr, $pattern:ident!($($args:tt)*) + $pattern2:ident!($($args2:tt)*)) => (
        match $pattern!($i, $($args)*) {
            Some(out) => match $pattern2!($crate::Input::new(out.pos, out.src), $($args2)*) {
                Some(out2) => Some($crate::Output::new(out2.pos, out2.src, out.hit)),
                None => None
            },
            None => None
        }
    );
}

#[macro_export]
macro_rules! opt {
    ($i:expr, $pattern:ident!($($args:tt)*)) => (
        match $pattern!($i, $($args)*) {
            Some(out) => Some($crate::Output::new(out.pos, out.src, Some(out.hit))),
            None => Some($crate::Output::new($i.pos, $i.src, None))
        }
    );
}

#[macro_export]
macro_rules! many0 {
    ($i:expr, $pattern:ident!($($args:tt)*)) => ({
        let mut result = Vec::new();
        let mut i = $i;

        while let Some(out) = $pattern!(i, $($args)*) {
            i = $crate::Input::new(out.pos, out.src);
            result.push(out.hit)
        }

        Some($crate::Output::new(i.pos, i.src, result))
    });
}

#[macro_export]
macro_rules! many1 {
    ($i:expr, $pattern:ident!($($args:tt)*)) => (
        match many0!($i, $pattern!($($args)*)) {
            Some(ref out) if out.hit.len() > 0 => Some(out),
            _ => None
        }
    );
}

#[macro_export]
macro_rules! to_string {
    ($i:expr, $pattern:ident!($($args:tt)*)) => (
        match $pattern!($i, $($args)*) {
            Some(out) => Some($crate::Output::new(out.pos, out.src, String::from(out.hit))),
            None => None
        }
    );
}

#[macro_export]
macro_rules! function {
    ($name:ident -> $out:ty, $pattern:ident!($($args:tt)*)) => (
        fn $name(i: $crate::Input) -> Option<$crate::Output<$out>> {
            $pattern!(i, $($args)*)
        }
    );

    (pub $name:ident -> $out:ty, $pattern:ident!($($args:tt)*)) => (
        pub fn $name(i: $crate::Input) -> Option<$crate::Output<$out>> {
            $pattern!(i, $($args)*)
        }
    );
}

#[macro_export]
macro_rules! value {
    ($i:expr, $pattern:ident!($($args:tt)*) => $value:expr) => (
        match $pattern!($i, $($args)*) {
            Some(out) => Some($crate::Output::new(out.pos, out.src, $value)),
            None => None
        }
    );
}
