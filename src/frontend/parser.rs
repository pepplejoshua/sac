use super::source::Source;
use regex::Regex;

pub enum ParseResult<T> {
    Some(T, Source),
    None,
}

pub trait Parser<T> {
    fn parse(self, src: Source) -> ParseResult<T>;
}

pub struct RegExp {
    pub regex: Regex,
}

impl Parser<String> for RegExp {
    fn parse(self, src: Source) -> ParseResult<String> {
        src.match_reg(self.regex)
    }
}
