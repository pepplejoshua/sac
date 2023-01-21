use regex::Regex;

use super::source::Source;

#[derive(Debug, Clone)]
pub enum ParseResult<T: Clone + 'static> {
    Multiple(Vec<T>, Source),
    Some(T, Source),
    None,
}

pub trait CanParse<T: Clone> {
    fn parse(&self, src: &mut Source) -> ParseResult<T>;
}

pub struct Parser<T: Clone + 'static> {
    pub p: Box<dyn Fn(&mut Source) -> ParseResult<T>>,
}

impl<T: Clone> Parser<T> {
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
                ParseResult::Multiple(_, _) => res,
            }
        }))
    }

    pub fn and(self, rhs: Parser<T>) -> Parser<T> {
        Parser::n(Box::new(move |src| -> ParseResult<T> {
            let res = self.parse(src);
            let mut results: Vec<T> = vec![];
            match res {
                ParseResult::Some(v, _) => {
                    results.push(v);
                    let mut res_r = rhs.parse(src);
                    match res_r {
                        ParseResult::Multiple(ref mut vs, src) => {
                            results.append(vs);
                            ParseResult::Multiple(results, src.clone())
                        }
                        ParseResult::Some(v, src) => {
                            results.push(v);
                            ParseResult::Multiple(results, src.clone())
                        }
                        ParseResult::None => res_r,
                    }
                }
                ParseResult::None => return res,
                ParseResult::Multiple(vs, _) => {
                    results = vs;
                    let mut res_r = rhs.parse(src);
                    match res_r {
                        ParseResult::Multiple(ref mut vs, src) => {
                            results.append(vs);
                            ParseResult::Multiple(results, src.clone())
                        }
                        ParseResult::Some(v, src) => {
                            results.push(v);
                            ParseResult::Multiple(results, src.clone())
                        }
                        ParseResult::None => res_r,
                    }
                }
            }
        }))
    }

    pub fn parse(&self, src: &mut Source) -> ParseResult<T> {
        (self.p)(src)
    }
}
