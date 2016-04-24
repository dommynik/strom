#[macro_export]
macro_rules! chr {
    ($src:expr, $fun:expr) => (
        if let Some(chr) = $src.src.chars().next() {
            if $fun(chr) == true {
                $crate::Output::ok(
                    $src.fw(chr.len_utf8()),
                    $crate::Match::new(&$src.src[..chr.len_utf8()], $src.pos))
            } else {
                $crate::Output::err($src, format!("Char `{}` doesn't match.", chr))
            }
        } else {
            $crate::Output::err($src, format!("Source is empty. (`{}`)", $src.src))
        }
    );
}
