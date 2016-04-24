#[macro_use] pub mod macros;


pub fn is_lowercase(chr: char) -> bool {
    chr >= 'a' && chr <= 'z'
}

pub fn is_uppercase(chr: char) -> bool {
    chr >= 'A' && chr <= 'Z'
}

pub fn is_digit(chr: char) -> bool {
    chr >= '0' && chr <= '9'
}

pub fn is_alpha(chr: char) -> bool {
    is_lowercase(chr) || is_uppercase(chr)
}

pub fn is_alphanumeric(chr: char) -> bool {
    is_alpha(chr) || is_digit(chr)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output<'a, T> {
    pub src: Source<'a>,
    pub mat: Result<Match<T>, String>
}

impl<'a, T> Output<'a, T> {
    pub fn ok(src: Source<'a>, mat: Match<T>) -> Output<'a, T> {
        Output { src: src, mat: Ok(mat) }
    }

    pub fn err(src: Source<'a>, err: String) -> Output<'a, T> {
        Output { src: src, mat: Err(err) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Source<'a> {
    pub src: &'a str,
    pub pos: usize,
}

impl<'a> Source<'a> {
    pub fn new(src: &'a str, pos: usize) -> Source<'a> {
        Source { src: src, pos: pos }
    }

    pub fn fw(mut self, len: usize) -> Source<'a> {
        self.src = &self.src[len..];
        self.pos = self.pos + len;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Match<T> {
    pub val: T,
    pub pos: usize,
}

impl<T> Match<T> {
    pub fn new(val: T, pos: usize) -> Match<T> {
        Match { val: val, pos: pos }
    }
}
