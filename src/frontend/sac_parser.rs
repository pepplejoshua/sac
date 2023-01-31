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
    todo!()
}

// #[allow(dead_code)]
// fn args<'a>(input: &str) -> ParseResult<Vec<AST>> {
//     expression
//         .and_then(|arg| zero_or_more(literal(",").and))
//         .parse(input)
// }

#[allow(dead_code, unused_variables, non_snake_case)]
pub fn parse(input: &str) {}
