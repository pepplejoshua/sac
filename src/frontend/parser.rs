use super::source::Source;
use regex::Regex;

pub enum ParseResult<T> {
    Some(T, Source),
    None,
}

pub trait Parser<T> {
    fn parse(&self, src: Source) -> ParseResult<T>;
}

pub struct RegExp {
    pub regex: Regex,
}

impl Parser<String> for RegExp {
    fn parse(&self, src: Source) -> ParseResult<String> {
        src.match_reg(&self.regex)
    }
}

pub struct Constant<T: Clone> {
    value: T,
}

impl<T: Clone> Parser<T> for Constant<T> {
    fn parse(&self, src: Source) -> ParseResult<T> {
        ParseResult::Some(self.value.clone(), src)
    }
}

// improve this to take a text span and report on it
pub struct Error {
    message: String,
}

impl Parser<()> for Error {
    fn parse(&self, src: Source) -> ParseResult<()> {
        panic!("{:} => {:}", src.index, self.message);
    }
}
