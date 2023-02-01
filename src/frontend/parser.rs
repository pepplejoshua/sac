use regex::Regex;

pub type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;

    fn map<F, NewOutput>(self, map_fn: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        F: Fn(Output) -> NewOutput + 'a,
    {
        BoxedParser::new(map(self, map_fn))
    }

    fn pred<F>(self, predicate: F) -> BoxedParser<'a, Output>
    where
        Self: Sized + 'a,
        Output: 'a,
        F: Fn(&Output) -> bool + 'a,
    {
        BoxedParser::new(pred(self, predicate))
    }

    fn and_then<F, NextParser, NewOutput>(self, f: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        NextParser: Parser<'a, NewOutput> + 'a,
        F: Fn(Output) -> NextParser + 'a,
    {
        BoxedParser::new(and_then(self, f))
    }

    fn or<P>(self, parser2: P) -> BoxedParser<'a, Output>
    where
        Self: Sized + 'a,
        Output: 'a,
        P: Parser<'a, Output> + 'a,
    {
        BoxedParser::new(or(self, parser2))
    }

    fn and_right<P, NewOutput>(self, parser2: P) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        P: Parser<'a, NewOutput> + 'a,
    {
        BoxedParser::new(and_right(self, parser2))
    }

    fn and_tuple<P, NewOutput>(self, parser2: P) -> BoxedParser<'a, (Output, NewOutput)>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        P: Parser<'a, NewOutput> + 'a,
    {
        BoxedParser::new(pair(self, parser2))
    }
}

pub struct BoxedParser<'a, Output> {
    pub p: Box<dyn Parser<'a, Output> + 'a>,
}

impl<'a, Output> BoxedParser<'a, Output> {
    pub fn new<P>(p: P) -> Self
    where
        P: Parser<'a, Output> + 'a,
    {
        BoxedParser { p: Box::new(p) }
    }
}

impl<'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self.p.parse(input)
    }
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}

#[allow(dead_code)]
pub fn literal<'a>(exp: &'a str) -> impl Parser<()> {
    move |input: &'a str| match input.get(0..exp.len()) {
        Some(next) if next == exp => Ok((&input[exp.len()..], ())),
        _ => Err(input),
    }
}

#[test]
fn test_literal() {
    let parse_pepple = literal("Pepple");
    assert_eq!(Ok((" Joshua", ())), parse_pepple.parse("Pepple Joshua"));
    assert_eq!(Ok((" Pepple", ())), parse_pepple.parse("Pepple Pepple"));
    assert_eq!(Err("Joshua Pepple"), parse_pepple.parse("Joshua Pepple"));
}

#[allow(dead_code)]
pub fn match_regex<'a>(exp: &'a str) -> impl Parser<String> {
    move |input: &'a str| {
        let reg = Regex::new(exp).unwrap();
        match reg.find_at(input, 0) {
            Some(val) if val.start() == 0 => Ok((
                &input[val.end()..],
                input[val.start()..val.end()].to_string(),
            )),
            _ => Err(input),
        }
    }
}

#[test]
fn test_match_regex() {
    let parse_number = match_regex("[0-9]+");
    assert_eq!(
        Ok(("abcde", "12345".to_string())),
        parse_number.parse("12345abcde")
    );
    assert_eq!(Err("Joshua Pepple"), parse_number.parse("Joshua Pepple"));
}

#[allow(dead_code)]
pub fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(final_input, result2)| (final_input, (result1, result2)))
        })
    }
}

#[test]
fn test_pair() {
    let tag_opener = pair(literal("<"), identifier);

    assert_eq!(
        Ok(("/>", ((), "my_tag".to_string()))),
        tag_opener.parse("<my_tag/>")
    );
}

#[allow(dead_code)]
pub fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, res)| (next_input, map_fn(res)))
    }
}

#[test]
fn test_map() {
    let tag_opener = map(pair(literal("<"), identifier), |(_, res2)| res2);

    assert_eq!(
        Ok(("/>", "my_tag".to_string())),
        tag_opener.parse("<my_tag/>")
    );
}

#[allow(dead_code)]
pub fn identifier(input: &str) -> ParseResult<String> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() => {
            matched.push(next);
        }
        _ => return Err(input),
    }

    for next in chars {
        if next.is_alphabetic() || next == '_' || next == '-' {
            matched.push(next);
        } else if next.is_numeric() {
            matched.push(next);
            break;
        } else {
            break;
        }
    }

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
}

#[test]
fn test_identifier_parser() {
    assert_eq!(
        Ok(("", "i_am_an_identifier".to_string())),
        identifier("i_am_an_identifier")
    );

    assert_eq!(
        Ok((" entirely an identifier", "not".to_string())),
        identifier("not entirely an identifier")
    );

    assert_eq!(
        Err("!not entirely an identifier"),
        identifier("!not entirely an identifier")
    );
}

#[allow(dead_code)]
pub fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _)| left)
}

#[allow(dead_code)]
pub fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_, right)| right)
}

#[test]
fn test_left_and_right() {
    let first = match_regex("joshua");
    let space = match_regex("[ \r\t\n]");
    let last = match_regex("pepple");

    assert_eq!(
        Ok(("", ("joshua".to_string(), "pepple".to_string()))),
        pair(left(first, space), last).parse("joshua pepple")
    );
}

#[allow(dead_code)]
pub fn constant<'a, U>(value: U) -> impl Parser<'a, U>
where
    U: Clone + 'a,
{
    move |input: &'a str| Ok((input, value.clone()))
}

#[test]
fn test_constant() {
    let parser = constant(300);
    assert_eq!(Ok(("doesn't matter", 300)), parser.parse("doesn't matter"));
}

#[allow(dead_code)]
pub fn number_i32(input: &str) -> ParseResult<i32> {
    match match_regex("[0-9]+").parse(input) {
        Ok((new_input, num_str)) => match num_str.parse::<i32>() {
            Ok(num) => Ok((new_input, num)),
            Err(_) => Err(input),
        },
        Err(err) => Err(err),
    }
}

#[test]
fn test_number_i32() {
    assert_eq!(Ok(("", 42)), number_i32("42"));
}

#[allow(dead_code)]
pub fn maybe<'a, P, R>(parser: P, value: R) -> impl Parser<'a, Vec<R>>
where
    P: Parser<'a, R>,
    R: Clone + 'a,
{
    move |input: &'a str| match parser.parse(input) {
        Ok((new_input, res)) => Ok((new_input, vec![res])),
        Err(_) => constant(vec![value.clone()]).parse(input),
    }
}

#[test]
fn test_maybe() {
    let parser = maybe(match_regex("[0-9]+"), "".into());
    assert_eq!(parser.parse("1234"), Ok(("", vec!["1234".into()])));
    assert_eq!(parser.parse("abcd"), Ok(("abcd", vec!["".into()])));
}

#[allow(dead_code)]
pub fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut results = vec![];

        while let Ok((next_input, item)) = parser.parse(input) {
            input = next_input;
            results.push(item);
        }

        Ok((input, results))
    }
}

#[allow(dead_code)]
pub fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut results = vec![];

        if let Ok((next_input, head)) = parser.parse(input) {
            input = next_input;
            results.push(head);
        } else {
            return Err(input);
        }

        while let Ok((next_input, item)) = parser.parse(input) {
            input = next_input;
            results.push(item);
        }

        Ok((input, results))
    }
}

#[test]
fn test_zero_or_more() {
    let parser = zero_or_more(match_regex("ha"));
    assert_eq!(
        Ok(("", vec!["ha".into(), "ha".into(), "ha".into(), "ha".into()])),
        parser.parse("hahahaha")
    );

    assert_eq!(Ok(("ahah", vec![])), parser.parse("ahah"));
    assert_eq!(Ok(("", vec![])), parser.parse(""));
}

#[test]
fn test_one_or_more() {
    let parser = one_or_more(match_regex("ha"));
    assert_eq!(
        Ok(("", vec!["ha".into(), "ha".into(), "ha".into(), "ha".into()])),
        parser.parse("hahahaha")
    );

    assert_eq!(Err("ahah"), parser.parse("ahah"));
    assert_eq!(Err(""), parser.parse(""));
}

#[allow(dead_code)]
pub fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..], next)),
        None => Err(input),
    }
}

#[allow(dead_code)]
pub fn pred<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
{
    move |input| {
        if let Ok((next_input, res)) = parser.parse(input) {
            if predicate(&res) {
                return Ok((next_input, res));
            }
        }
        Err(input)
    }
}

#[test]
fn test_predicate() {
    let parser = pred(any_char, |c| *c == 'o');
    assert_eq!(Ok(("mg", 'o')), parser.parse("omg"));
    assert_eq!(Err("lol"), parser.parse("lol"));
}

#[allow(dead_code)]
pub fn whitespace_char<'a>() -> impl Parser<'a, char> {
    any_char.pred(|c| c.is_whitespace())
}

#[allow(dead_code)]
pub fn space0<'a>() -> impl Parser<'a, Vec<char>> {
    zero_or_more(whitespace_char())
}

#[allow(dead_code)]
pub fn space1<'a>() -> impl Parser<'a, Vec<char>> {
    one_or_more(whitespace_char())
}

#[allow(dead_code)]
pub fn either<'a, P1, P2, A>(parser1: P1, parser2: P2) -> impl Parser<'a, A>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, A>,
{
    move |input| match parser1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => parser2.parse(input),
    }
}

#[test]
fn test_either() {
    let num = match_regex(r"[0-9]{3}");
    assert_eq!(either(num, identifier).parse("abc"), Ok(("", "abc".into())))
}

#[allow(dead_code)]
pub fn or<'a, P1, P2, R>(parser1: P1, parser2: P2) -> impl Parser<'a, R>
where
    P1: Parser<'a, R>,
    P2: Parser<'a, R>,
{
    move |input| match parser1.parse(input) {
        Ok((new_input, res)) => Ok((new_input, res)),
        Err(_) => parser2.parse(input),
    }
}

#[test]
fn test_or() {
    let parser = match_regex("[0-9]+").or(identifier);
    assert_eq!(parser.parse("12345"), Ok(("", "12345".into())));
    assert_eq!(
        parser.parse("identifier_135"),
        Ok(("35", "identifier_1".into()))
    );
    assert_eq!(parser.parse("!23 identifier"), Err("!23 identifier"));
}

#[allow(dead_code)]
pub fn and_right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| match parser1.parse(input) {
        Ok((new_input, _)) => parser2.parse(new_input),
        Err(_) => Err(input),
    }
}

#[test]
fn test_and_right() {
    let parser =
        literal("joshua").and_right(any_char.pred(|c| c.is_whitespace()).and_right(identifier));

    assert_eq!(parser.parse("joshua pepple"), Ok(("", "pepple".into())))
}

#[allow(dead_code)]
pub fn and_then<'a, P, F, A, B, NextParser>(parser: P, f: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    NextParser: Parser<'a, B>,
    F: Fn(A) -> NextParser,
{
    move |input| match parser.parse(input) {
        Ok((next_input, parsed)) => f(parsed).parse(next_input),
        Err(err) => Err(err),
    }
}

#[allow(dead_code)]
pub fn whitespace_wrap<'a, P, A>(parser: P) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
{
    right(space0(), left(parser, space0()))
}
