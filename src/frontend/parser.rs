use regex::Regex;

use super::source::Source;

pub enum ParseResult<T: Clone> {
    Some(T, Source),
    None,
}

pub struct Parser<T: Clone> {
    pub p: Box<dyn Fn(&mut Source) -> ParseResult<T>>,
}

impl<T: Clone> Parser<T> {
    pub fn n(parser: Box<dyn Fn(&mut Source) -> ParseResult<T>>) -> Parser<T> {
        Parser { p: parser }
    }

    pub fn regexp(regexp: &'static str) -> Parser<String> {
        Parser::n(Box::new(|src| -> ParseResult<String> {
            src.match_reg(Regex::new(regexp).unwrap())
        }))
    }

    pub fn parse(&self, src: &mut Source) -> ParseResult<T> {
        (self.p)(src)
    }
}
