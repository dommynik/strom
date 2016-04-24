#[macro_export]
macro_rules! tag {
    ($src:expr, $tag:expr) => ({
        if $src.src.starts_with($tag) {
            $crate::Output::ok(
                $src.fw($tag.len()),
                $crate::Match::new(&$src.src[..$tag.len()], $src.pos))
        } else {
            $crate::Output::err($src, format!("Tag (`{}`) not found.", $tag))
        }
    });

    ($src:expr, cl $tag:expr) => (
        if $src.src.len() >= $tag.len() {
            let src = $src.src[..$tag.len()].to_lowercase();
            let tag = $tag.to_lowercase();

            if src == tag {
                $crate::Output::ok(
                    $src.fw($tag.len()),
                    $crate::Match::new(&$src.src[..$tag.len()], $src.pos))
            } else {
                $crate::Output::err($src, format!("Tag (`{}`) not found.", $tag))
            }
        } else {
            $crate::Output::err($src, format!("Source too short. (`{}`)", $src.src))
        }
    );

    ($src:expr, $tag:expr => $val:expr) => (
        if $src.src.starts_with($tag) {
            $crate::Output::ok(
                $src.fw($tag.len()),
                $crate::Match::new($val, $src.pos))
        } else {
            $crate::Output::err($src, format!("Tag (`{}`) not found.", $tag))
        }
    );

    ($src:expr, cl $tag:expr => $val:expr) => (
        if $src.src.len() >= $tag.len() {
            let src = $src.src[..$tag.len()].to_lowercase();
            let tag = $tag.to_lowercase();

            if src == tag {
                $crate::Output::ok(
                    $src.fw($tag.len()),
                    $crate::Match::new($val, $src.pos))
            } else {
                $crate::Output::err($src, format!("Tag (`{}`) not found.", $tag))
            }
        } else {
            $crate::Output::err($src, format!("Source too short. (`{}`)", $src.src))
        }
    );
}

#[cfg(test)]
mod tests {

    #[test]
    fn tag1() {
        let src = ::Source::new("Hello", 0);
        let out = tag!(src, "Hello");
        let mat_exp = Ok(::Match::new("Hello", 0));
        let src_exp = ::Source::new("", 5);

        assert_eq!(out.src, src_exp);
        assert_eq!(out.mat, mat_exp);
    }

    #[test]
    fn tag2() {
        let src = ::Source::new("Hello World", 0);
        let out = tag!(src, "Hello");
        let mat_exp = Ok(::Match::new("Hello", 0));
        let src_exp = ::Source::new(" World", 5);

        assert_eq!(out.src, src_exp);
        assert_eq!(out.mat, mat_exp);
    }

    #[test]
    fn tag_cl1() {
        let src = ::Source::new("Hello World", 0);
        let out = tag!(src, cl "hello");
        let mat_exp = Ok(::Match::new("Hello", 0));
        let src_exp = ::Source::new(" World", 5);

        assert_eq!(out.src, src_exp);
        assert_eq!(out.mat, mat_exp);
    }

    #[test]
    fn tag_cl2() {
        let src = ::Source::new("Hello World", 0);
        let out = tag!(src, cl "hello");
        let mat_exp = Ok(::Match::new("Hello", 0));
        let src_exp = ::Source::new(" World", 5);

        assert_eq!(out.src, src_exp);
        assert_eq!(out.mat, mat_exp);
    }

    #[test]
    fn tag_val1() {
        let src = ::Source::new("Hello", 0);
        let out = tag!(src, "Hello" => 1234);
        let mat_exp = Ok(::Match::new(1234, 0));
        let src_exp = ::Source::new("", 5);

        assert_eq!(out.src, src_exp);
        assert_eq!(out.mat, mat_exp);
    }

    #[test]
    fn tag_val2() {
        let src = ::Source::new("Hello World", 0);
        let out = tag!(src, "Hello" => 1234);
        let mat_exp = Ok(::Match::new(1234, 0));
        let src_exp = ::Source::new(" World", 5);

        assert_eq!(out.src, src_exp);
        assert_eq!(out.mat, mat_exp);
    }
}
