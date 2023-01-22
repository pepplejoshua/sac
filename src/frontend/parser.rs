use std::rc::Rc;

use regex::Regex;

use super::ast::AST;
use super::source::Source;
use super::span::Span;
use closure::closure;

#[derive(Debug, Clone)]
pub enum ParseResult<T: Clone + 'static> {
    Some(T, Source),
    None,
}

#[derive(Clone)]
pub struct Parser<T: Clone + 'static> {
    pub p: Rc<dyn Fn(&mut Source) -> ParseResult<T>>,
}

impl<T: Clone + 'static> Parser<T> {
    pub fn n(parser: Rc<dyn Fn(&mut Source) -> ParseResult<T>>) -> Parser<T> {
        Parser { p: parser }
    }

    pub fn regexp(regexp: &'static str) -> Parser<String> {
        Parser::n(Rc::new(move |src| -> ParseResult<String> {
            src.match_reg(Regex::new(regexp).unwrap())
        }))
    }

    pub fn constant<U: Clone + 'static>(value: U) -> Parser<U> {
        Parser::n(Rc::new(move |src| -> ParseResult<U> {
            ParseResult::Some(value.clone(), src.clone())
        }))
    }

    pub fn none<U: Clone + 'static>() -> Parser<U> {
        Parser::n(Rc::new(|_| -> ParseResult<U> { ParseResult::None }))
    }

    #[allow(unreachable_code)]
    pub fn error(msg: String) -> Parser<AST> {
        panic!("{msg}");
    }

    pub fn or(self, rhs: Parser<T>) -> Parser<T> {
        Parser::n(Rc::new(move |src| -> ParseResult<T> {
            let res = self.parse(src);
            match res {
                ParseResult::Some(_, _) => res,
                ParseResult::None => rhs.parse(src),
            }
        }))
    }

    pub fn and(self, rhs: Parser<T>) -> Parser<Vec<T>> {
        Parser::n(Rc::new(move |src| -> ParseResult<Vec<T>> {
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
        Parser::n(Rc::new(move |src| -> ParseResult<Vec<U>> {
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

    pub fn bind<U: Clone + 'static>(
        self,
        callback: Rc<dyn Fn(T) -> Parser<U> + 'static>,
    ) -> Parser<U> {
        Parser::n(Rc::new(move |src| -> ParseResult<U> {
            let res = self.parse(src);
            match res {
                ParseResult::Some(v, _) => callback(v).parse(src),
                ParseResult::None => ParseResult::None,
            }
        }))
    }

    pub fn and_drop_left<U: Clone + 'static>(self, rhs: Parser<U>) -> Parser<U> {
        self.bind(Rc::new(move |_: T| -> Parser<U> { rhs.clone() }))
        // return self.bind(Box::new(move |_| rhs));
    }

    pub fn map<U: Clone + 'static>(self, callback: Rc<dyn Fn(T) -> U + 'static>) -> Parser<U> {
        self.bind(Rc::new(move |value| constant(callback(value))))
    }

    pub fn maybe<U: Clone + 'static>(parser: Parser<U>, default: U) -> Parser<U> {
        parser.or(constant(default))
    }

    pub fn parse(&self, src: &mut Source) -> ParseResult<T> {
        (self.p)(src)
    }

    pub fn parse_string(&self, string: String) -> Result<T, String> {
        let mut src = Source::from(string);
        let res = self.parse(&mut src);
        match res {
            ParseResult::Some(v, _) => {
                if src.index != src.content.len() {
                    Err(format!("Parse error at index {}", src.index))
                } else {
                    Ok(v)
                }
            }
            ParseResult::None => Err("Parse error at index 0".into()),
        }
    }
}

pub fn regexp(reg: &'static str) -> Parser<String> {
    Parser::<String>::regexp(reg)
}

pub fn zero_or_more<U: Clone + 'static>(parser: Parser<U>) -> Parser<Vec<U>> {
    Parser::<U>::zero_or_more(parser)
}

pub fn constant<U: Clone + 'static>(value: U) -> Parser<U> {
    Parser::<U>::constant(value)
}

pub fn none<U: Clone + 'static>() -> Parser<U> {
    Parser::<U>::none()
}

pub fn maybe<U: Clone + 'static>(parser: Parser<U>, default: U) -> Parser<U> {
    Parser::<U>::maybe(parser, default)
}

#[allow(unused_variables, non_snake_case)]
pub fn parse() {
    let whitespace = regexp(r"[ \n\r\t]+");
    let comments = regexp(r"[/][/].*").or(regexp(r"(?s)[/][*].*[*][/]"));
    let ignored = zero_or_more(whitespace.or(comments));

    let make_token_parser = closure!(clone ignored, |pattern: &'static str| {
        regexp(pattern).bind(Rc::new(closure!(clone ignored, |value| {
            ignored.clone().and_drop_left(constant(value))
        })))
    });

    // keywords
    let FN = make_token_parser(r"fn\b");
    let IF = make_token_parser(r"if\b");
    let ELSE = make_token_parser(r"else\b");
    let RETURN = make_token_parser(r"ret\b");
    let MUT = make_token_parser(r"mut\b");
    let WHILE = make_token_parser(r"while\b");

    // punctuators
    let COMMA = make_token_parser(r",");
    let SEMI_COLON = make_token_parser(r";");
    let LEFT_PAREN = make_token_parser(r"[(]");
    let RIGHT_PAREN = make_token_parser(r"[)]");
    let LEFT_BRACE = make_token_parser(r"[{]");
    let RIGHT_BRACE = make_token_parser(r"[}]");

    // constants
    let NUMBER = make_token_parser(r"[0-9]+").map(Rc::new(|digits| AST::Number {
        num: digits.parse::<i64>().unwrap(),
        span: Span::new_dud(),
    }));
    let ID = make_token_parser(r"[a-zA-Z_][a-zA-Z0-9_]*");

    // generates AST token for an identifier
    let id = ID.clone().map(Rc::new(|x| AST::Identifier {
        name: x,
        span: Span::new_dud(),
    }));

    // operators
    let NOT = make_token_parser(r"!");
    let EQUAL = make_token_parser(r"==");
    let N_EQUALS = make_token_parser(r"!=");
    let PLUS = make_token_parser(r"[+]");
    let MINUS = make_token_parser(r"[-]");
    let STAR = make_token_parser(r"[*]");
    let SLASH = make_token_parser(r"[/]");

    let expression = Parser::<AST>::error("expression parser used before definition".into());

    let args = expression
        .clone()
        .bind(Rc::new(closure!(clone expression, clone COMMA, |arg| {
            zero_or_more(COMMA.clone().and_drop_left(expression.clone()))
                .bind(Rc::new(move |args| {
                    constant([vec![arg.clone()], args].concat())
                }))
                .or(constant(vec![]))
        })));

    let call = ID.clone().bind(Rc::new(closure!(
        clone LEFT_PAREN,
        clone RIGHT_PAREN,
        |called: String| -> Parser<AST> {
            LEFT_PAREN
                .clone()
                .and_drop_left(args.clone().bind(Rc::new(closure!(clone RIGHT_PAREN, |p_args: Vec<AST>| -> Parser<AST> {
                    RIGHT_PAREN.clone().and_drop_left(constant(AST::Call {
                        called: called.clone(),
                        args: p_args,
                        span: Span::new_dud(),
                    }))
                }))))
        }
    )));

    let atom = call.clone().or(id.clone()).or(NUMBER.clone()).or(LEFT_PAREN
        .clone()
        .and_drop_left(expression.clone())
        .bind(Rc::new(
            closure!(clone RIGHT_PAREN, |e: AST| -> Parser<AST> {
                RIGHT_PAREN.clone().and_drop_left(constant(e))
            }),
        )));

    let unary =
        maybe(NOT.clone(), "".into()).bind(Rc::new(closure!(clone atom, |not_str: String| {
            atom.clone().map(Rc::new(closure!(clone not_str, |term| {
                if not_str == "" {
                    term
                } else {
                    AST::Not { target: Box::new(term), span: Span::new_dud() }
                }
            })))
        })));
}
