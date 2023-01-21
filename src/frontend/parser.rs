use regex::Regex;

use super::source::Source;

#[derive(Debug, Clone)]
pub enum ParseResult<T: Clone + 'static> {
    Some(T, Source),
    None,
}

pub trait CanParse<T: Clone> {
    fn parse(&self, src: &mut Source) -> ParseResult<T>;
}

pub struct Parser<T: Clone + 'static> {
    pub p: Box<dyn Fn(&mut Source) -> ParseResult<T>>,
}

impl<T: Clone + 'static> Parser<T> {
    pub fn n(parser: Box<dyn Fn(&mut Source) -> ParseResult<T>>) -> Parser<T> {
        Parser { p: parser }
    }

    pub fn regexp(regexp: &'static str) -> Parser<String> {
        Parser::n(Box::new(move |src| -> ParseResult<String> {
            src.match_reg(Regex::new(regexp).unwrap())
        }))
    }

    pub fn constant<U: Clone + 'static>(value: U) -> Parser<U> {
        Parser::n(Box::new(move |src| -> ParseResult<U> {
            ParseResult::Some(value.clone(), src.clone())
        }))
    }

    pub fn error(msg: String) {
        panic!("{msg}");
    }

    pub fn or(self, rhs: Parser<T>) -> Parser<T> {
        Parser::n(Box::new(move |src| -> ParseResult<T> {
            let res = self.parse(src);
            match res {
                ParseResult::Some(_, _) => res,
                ParseResult::None => rhs.parse(src),
            }
        }))
    }

    pub fn and(self, rhs: Parser<T>) -> Parser<Vec<T>> {
        Parser::n(Box::new(move |src| -> ParseResult<Vec<T>> {
            let res = self.parse(src);
            let mut results: Vec<T> = vec![];
            match res {
                ParseResult::Some(v, _) => {
                    results.push(v);
                    let res_r = rhs.parse(src);
                    match res_r {
                        ParseResult::Some(v, src) => {
                            results.push(v);
                            ParseResult::Some(results, src.clone())
                        }
                        ParseResult::None => ParseResult::None,
                    }
                }
                ParseResult::None => ParseResult::None,
            }
        }))
    }

    pub fn zero_or_more<U: Clone + 'static>(parser: Parser<U>) -> Parser<Vec<U>> {
        Parser::n(Box::new(move |src| -> ParseResult<Vec<U>> {
            let mut results: Vec<U> = vec![];
            let mut item;
            'outer: loop {
                item = parser.parse(src);
                match item {
                    ParseResult::Some(v, _) => {
                        results.push(v);
                    }
                    ParseResult::None => {
                        break 'outer;
                    }
                }
            }
            ParseResult::Some(results, src.clone())
        }))
    }

    pub fn parse(&self, src: &mut Source) -> ParseResult<T> {
        (self.p)(src)
    }
}

pub fn regexp(reg: &'static str) -> Parser<String> {
    Parser::<String>::regexp(reg)
}

pub fn zero_or_more<U: Clone + 'static>(parser: Parser<U>) -> Parser<Vec<U>> {
    Parser::<U>::zero_or_more(parser)
}
