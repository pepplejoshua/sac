use super::ast::*;
use super::parser::*;
use super::span::Span;

#[allow(dead_code)]
fn id(input: &str) -> ParseResult<AST> {
    identifier
        .map(|x| -> AST {
            AST::Identifier {
                name: x,
                span: Span::new_dud(),
            }
        })
        .parse(input)
}

#[test]
fn test_id() {
    assert_eq!(
        Ok((
            "",
            AST::Identifier {
                name: "abcd_1".into(),
                span: Span::new_dud()
            }
        )),
        id("abcd_1")
    );
}

#[allow(dead_code)]
fn expression(_input: &str) -> ParseResult<AST> {
    id.parse(_input)
}

#[allow(dead_code)]
fn ignored(input: &str) -> ParseResult<()> {
    let whitespace = match_regex(r"[ \n\r\t]+");
    let comments = match_regex(r"[/][/].*").or(match_regex(r"(?s)[/][*].*[*][/]"));
    zero_or_more(whitespace.or(comments))
        .map(|_| ())
        .parse(input)
}

#[allow(dead_code)]
fn args(input: &str) -> ParseResult<Vec<AST>> {
    expression
        .and_then(|arg| {
            zero_or_more(literal(",").and_right(ignored).and_right(expression))
                .and_then(move |args| constant(vec![vec![arg.clone()], args].concat()))
        })
        .or(constant(vec![]))
        .parse(input)
}

#[test]
fn test_args() {
    assert_eq!(
        args("a, b, c"),
        Ok((
            "",
            vec![
                AST::Identifier {
                    name: "a".into(),
                    span: Span::new_dud()
                },
                AST::Identifier {
                    name: "b".into(),
                    span: Span::new_dud()
                },
                AST::Identifier {
                    name: "c".into(),
                    span: Span::new_dud()
                }
            ],
        ))
    );
    assert_eq!(args(""), Ok(("", vec![],)))
}



#[allow(dead_code, unused_variables, non_snake_case)]
pub fn parse(input: &str) {}
