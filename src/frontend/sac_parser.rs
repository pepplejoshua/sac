use closure::closure;

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

#[allow(dead_code)]
fn sidentifier(input: &str) -> ParseResult<String> {
    ignored.and_right(identifier).parse(input)
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
fn expression(input: &str) -> ParseResult<AST> {
    ignored.and_right(comparison).parse(input)
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
            zero_or_more(sliteral(",").and_right(expression))
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

#[allow(dead_code)]
fn sliteral(exp: &str) -> impl Parser<String> {
    ignored.and_right(match_regex(exp))
}

#[allow(dead_code)]
fn sident(input: &str) -> ParseResult<String> {
    ignored.and_right(identifier).parse(input)
}

#[allow(dead_code)]
fn call(input: &str) -> ParseResult<AST> {
    sident
        .and_then(|callee| {
            sliteral("[(]").and_right(args.and_then(move |args| {
                sliteral("[)]").and_right(constant(AST::Call {
                    called: callee.clone(),
                    args,
                    span: Span::new_dud(),
                }))
            }))
        })
        .parse(input)
}

#[test]
fn test_call() {
    assert_eq!(
        call("fib(a,      b,      c)"),
        Ok((
            "",
            AST::Call {
                called: "fib".into(),
                args: vec![
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
                span: Span::new_dud()
            }
        ))
    )
}

#[allow(dead_code)]
fn number(input: &str) -> ParseResult<AST> {
    number_i64
        .map(|num| AST::Number {
            num,
            span: Span::new_dud(),
        })
        .parse(input)
}

#[allow(dead_code)]
fn atom(input: &str) -> ParseResult<AST> {
    ignored
        .and_right(
            call.or(id).or(number).or(sliteral("[(]")
                .and_right(expression)
                .and_then(|expr| sliteral("[)]").and_right(constant(expr)))),
        )
        .parse(input)
}

#[test]
fn test_atom() {
    assert_eq!(
        atom("a"),
        Ok((
            "",
            AST::Identifier {
                name: "a".into(),
                span: Span::new_dud(),
            }
        ))
    );
    assert_eq!(
        atom("321"),
        Ok((
            "",
            AST::Number {
                span: Span::new_dud(),
                num: 321
            }
        ))
    );
    assert_eq!(
        atom("(   (321))"),
        Ok((
            "",
            AST::Number {
                span: Span::new_dud(),
                num: 321
            }
        ))
    );
}

#[allow(dead_code)]
fn unary(input: &str) -> ParseResult<AST> {
    maybe(sliteral("!"), "".into())
        .and_then(|not| {
            atom.map(move |term| {
                if not[0].is_empty() {
                    term
                } else {
                    AST::Not {
                        target: Box::new(term),
                        span: Span::new_dud(),
                    }
                }
            })
        })
        .parse(input)
}

#[test]
fn test_unary() {
    assert_eq!(
        unary("!abcd"),
        Ok((
            "",
            AST::Not {
                target: Box::new(AST::Identifier {
                    name: "abcd".into(),
                    span: Span::new_dud()
                }),
                span: Span::new_dud()
            }
        ))
    );

    assert_eq!(
        unary("abcd"),
        Ok((
            "",
            AST::Identifier {
                name: "abcd".into(),
                span: Span::new_dud()
            },
        ))
    );
}

#[allow(dead_code)]
fn product(input: &str) -> ParseResult<AST> {
    unary
        .and_then(|left| {
            zero_or_more(
                sliteral("[*]")
                    .or(sliteral("[/]"))
                    .and_then(move |operator| {
                        unary.and_then(move |rhs| constant((operator.clone(), rhs)))
                    }),
            )
            .map(move |ops_and_terms| {
                ops_and_terms
                    .into_iter()
                    .fold(left.clone(), |lhs, (operator, rhs)| {
                        match operator.as_ref() {
                            "*" => AST::Multiply {
                                lhs: Box::new(lhs),
                                rhs: Box::new(rhs),
                            },
                            "/" => AST::Divide {
                                lhs: Box::new(lhs),
                                rhs: Box::new(rhs),
                            },
                            &_ => AST::Error {
                                span: Span::new_dud(),
                                msg: "".into(),
                            },
                        }
                    })
            })
        })
        .parse(input)
}

#[test]
fn test_product() {
    assert_eq!(
        product("1"),
        Ok((
            "",
            AST::Number {
                num: 1,
                span: Span::new_dud()
            }
        ))
    );
    assert_eq!(
        product("1     *    3 /   4"),
        Ok((
            "",
            AST::Divide {
                lhs: Box::new(AST::Multiply {
                    lhs: Box::new(AST::Number {
                        num: 1,
                        span: Span::new_dud()
                    }),
                    rhs: Box::new(AST::Number {
                        num: 3,
                        span: Span::new_dud()
                    })
                }),
                rhs: Box::new(AST::Number {
                    num: 4,
                    span: Span::new_dud()
                })
            }
        ))
    )
}

#[allow(dead_code)]
fn sum(input: &str) -> ParseResult<AST> {
    product
        .and_then(|left| {
            zero_or_more(
                sliteral("[+]")
                    .or(sliteral("[-]"))
                    .and_then(move |operator| {
                        product.and_then(move |rhs| constant((operator.clone(), rhs)))
                    }),
            )
            .map(move |ops_and_terms| {
                ops_and_terms
                    .into_iter()
                    .fold(left.clone(), |lhs, (operator, rhs)| {
                        match operator.as_ref() {
                            "+" => AST::Add {
                                lhs: Box::new(lhs),
                                rhs: Box::new(rhs),
                            },
                            "-" => AST::Subtract {
                                lhs: Box::new(lhs),
                                rhs: Box::new(rhs),
                            },
                            &_ => AST::Error {
                                span: Span::new_dud(),
                                msg: "".into(),
                            },
                        }
                    })
            })
        })
        .parse(input)
}

#[test]
fn test_sum() {
    assert_eq!(
        sum("1"),
        Ok((
            "",
            AST::Number {
                num: 1,
                span: Span::new_dud()
            }
        ))
    );
    assert_eq!(
        sum("1     *    3 /   4"),
        Ok((
            "",
            AST::Divide {
                lhs: Box::new(AST::Multiply {
                    lhs: Box::new(AST::Number {
                        num: 1,
                        span: Span::new_dud()
                    }),
                    rhs: Box::new(AST::Number {
                        num: 3,
                        span: Span::new_dud()
                    })
                }),
                rhs: Box::new(AST::Number {
                    num: 4,
                    span: Span::new_dud()
                })
            }
        ))
    );
    assert_eq!(
        sum("1     +    3 -   4"),
        Ok((
            "",
            AST::Subtract {
                lhs: Box::new(AST::Add {
                    lhs: Box::new(AST::Number {
                        num: 1,
                        span: Span::new_dud()
                    }),
                    rhs: Box::new(AST::Number {
                        num: 3,
                        span: Span::new_dud()
                    })
                }),
                rhs: Box::new(AST::Number {
                    num: 4,
                    span: Span::new_dud()
                })
            }
        ))
    );
    assert_eq!(
        sum("a     *    3 /   4 + 5"),
        Ok((
            "",
            AST::Add {
                lhs: Box::new(AST::Divide {
                    lhs: Box::new(AST::Multiply {
                        lhs: Box::new(AST::Identifier {
                            name: "a".into(),
                            span: Span::new_dud(),
                        }),
                        rhs: Box::new(AST::Number {
                            num: 3,
                            span: Span::new_dud()
                        })
                    }),
                    rhs: Box::new(AST::Number {
                        num: 4,
                        span: Span::new_dud()
                    })
                }),
                rhs: Box::new(AST::Number {
                    num: 5,
                    span: Span::new_dud()
                })
            }
        ))
    );
}

#[allow(dead_code)]
fn comparison(input: &str) -> ParseResult<AST> {
    sum.and_then(move |left| {
        zero_or_more(sliteral("==").or(sliteral("!=")).and_then(move |operator| {
            sum.and_then(move |right| constant((operator.clone(), right)))
        }))
        .map(move |ops_and_terms| {
            ops_and_terms
                .into_iter()
                .fold(left.clone(), |lhs, (operator, rhs)| {
                    match operator.as_ref() {
                        "==" => AST::Equals {
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        },
                        "!=" => AST::NEquals {
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        },
                        &_ => AST::Error {
                            span: Span::new_dud(),
                            msg: input.into(),
                        },
                    }
                })
        })
    })
    .parse(input)
}

#[test]
fn test_comparison() {
    assert_eq!(
        comparison("1"),
        Ok((
            "",
            AST::Number {
                num: 1,
                span: Span::new_dud()
            }
        ))
    );
    assert_eq!(
        comparison("1     *    3 /   4"),
        Ok((
            "",
            AST::Divide {
                lhs: Box::new(AST::Multiply {
                    lhs: Box::new(AST::Number {
                        num: 1,
                        span: Span::new_dud()
                    }),
                    rhs: Box::new(AST::Number {
                        num: 3,
                        span: Span::new_dud()
                    })
                }),
                rhs: Box::new(AST::Number {
                    num: 4,
                    span: Span::new_dud()
                })
            }
        ))
    );
    assert_eq!(
        comparison("1 != 2"),
        Ok((
            "",
            AST::NEquals {
                lhs: Box::new(AST::Number {
                    num: 1,
                    span: Span::new_dud()
                }),
                rhs: Box::new(AST::Number {
                    num: 2,
                    span: Span::new_dud()
                })
            }
        ))
    );
    assert_eq!(
        comparison("1     +    3 -   4"),
        Ok((
            "",
            AST::Subtract {
                lhs: Box::new(AST::Add {
                    lhs: Box::new(AST::Number {
                        num: 1,
                        span: Span::new_dud()
                    }),
                    rhs: Box::new(AST::Number {
                        num: 3,
                        span: Span::new_dud()
                    })
                }),
                rhs: Box::new(AST::Number {
                    num: 4,
                    span: Span::new_dud()
                })
            }
        ))
    );
    assert_eq!(
        comparison("1     *    3 /   4 + 5"),
        Ok((
            "",
            AST::Add {
                lhs: Box::new(AST::Divide {
                    lhs: Box::new(AST::Multiply {
                        lhs: Box::new(AST::Number {
                            num: 1,
                            span: Span::new_dud()
                        }),
                        rhs: Box::new(AST::Number {
                            num: 3,
                            span: Span::new_dud()
                        })
                    }),
                    rhs: Box::new(AST::Number {
                        num: 4,
                        span: Span::new_dud()
                    })
                }),
                rhs: Box::new(AST::Number {
                    num: 5,
                    span: Span::new_dud()
                })
            }
        ))
    );
    assert_eq!(
        comparison("a + 1 == b - 1 != c"),
        Ok((
            "",
            AST::NEquals {
                lhs: Box::new(AST::Equals {
                    lhs: Box::new(AST::Add {
                        lhs: Box::new(AST::Identifier {
                            name: "a".into(),
                            span: Span::new_dud()
                        }),
                        rhs: Box::new(AST::Number {
                            num: 1,
                            span: Span::new_dud()
                        })
                    }),
                    rhs: Box::new(AST::Subtract {
                        lhs: Box::new(AST::Identifier {
                            name: "b".into(),
                            span: Span::new_dud()
                        }),
                        rhs: Box::new(AST::Number {
                            num: 1,
                            span: Span::new_dud()
                        })
                    }),
                }),
                rhs: Box::new(AST::Identifier {
                    name: "c".into(),
                    span: Span::new_dud()
                })
            }
        ))
    );
}

#[allow(dead_code)]
fn statement(_input: &str) -> ParseResult<AST> {
    return_s(_input)
}

#[allow(dead_code)]
fn return_s(input: &str) -> ParseResult<AST> {
    sliteral("ret")
        .and_right(expression)
        .and_then(|val| {
            sliteral("[;]").and_right(constant(AST::Return {
                value: Box::new(val),
                span: Span::new_dud(),
            }))
        })
        .parse(input)
}

#[test]
fn test_return_s() {
    assert_eq!(
        return_s("ret a;"),
        Ok((
            "",
            AST::Return {
                value: Box::new(AST::Identifier {
                    name: "a".into(),
                    span: Span::new_dud()
                }),
                span: Span::new_dud()
            }
        ))
    );
}

#[allow(dead_code)]
fn expr_s(input: &str) -> ParseResult<AST> {
    expression
        .and_then(|expr| sliteral("[;]").and_right(constant(expr)))
        .parse(input)
}

#[test]
fn test_expr_s() {
    assert_eq!(
        expr_s("   1   ;"),
        Ok((
            "",
            AST::Number {
                num: 1,
                span: Span::new_dud()
            }
        ))
    )
}

#[allow(dead_code)]
fn if_s(input: &str) -> ParseResult<AST> {
    sliteral("if")
        .and_right(expression)
        .and_then(|conditional| {
            statement.and_then(closure!(clone conditional, |then_body| {
                sliteral("else")
                    .and_right(statement)
                    .and_then(closure!(clone conditional, clone then_body, |else_body| {
                        constant(AST::IfCond {
                            span: Span::new_dud(),
                            condition: Box::new(conditional.clone()),
                            then: Box::new(then_body.clone()),
                            c_else: Box::new(else_body),
                        })
                    }))
            }))
        })
        .parse(input)
}

#[test]
fn test_if_s() {
    assert_eq!(
        if_s("if a == b ret a; else ret b;"),
        Ok((
            "",
            AST::IfCond {
                span: Span::new_dud(),
                condition: Box::new(AST::Equals {
                    lhs: Box::new(AST::Identifier {
                        name: "a".into(),
                        span: Span::new_dud()
                    }),
                    rhs: Box::new(AST::Identifier {
                        name: "b".into(),
                        span: Span::new_dud()
                    })
                }),
                then: Box::new(AST::Return {
                    value: Box::new(AST::Identifier {
                        name: "a".into(),
                        span: Span::new_dud()
                    }),
                    span: Span::new_dud()
                }),
                c_else: Box::new(AST::Return {
                    value: Box::new(AST::Identifier {
                        name: "b".into(),
                        span: Span::new_dud()
                    }),
                    span: Span::new_dud()
                }),
            }
        ))
    )
}

#[allow(dead_code)]
fn while_s(input: &str) -> ParseResult<AST> {
    sliteral("while")
        .and_right(expression)
        .and_then(|conditional| {
            statement.and_then(move |body| {
                constant(AST::WhileLoop {
                    span: Span::new_dud(),
                    condition: Box::new(conditional.clone()),
                    body: Box::new(body),
                })
            })
        })
        .parse(input)
}

#[test]
fn test_while_s() {
    assert_eq!(
        while_s("while a == b ret a;"),
        Ok((
            "",
            AST::WhileLoop {
                span: Span::new_dud(),
                condition: Box::new(AST::Equals {
                    lhs: Box::new(AST::Identifier {
                        name: "a".into(),
                        span: Span::new_dud()
                    }),
                    rhs: Box::new(AST::Identifier {
                        name: "b".into(),
                        span: Span::new_dud()
                    })
                }),
                body: Box::new(AST::Return {
                    value: Box::new(AST::Identifier {
                        name: "a".into(),
                        span: Span::new_dud()
                    }),
                    span: Span::new_dud()
                }),
            }
        ))
    )
}

#[allow(dead_code)]
fn var_s(input: &str) -> ParseResult<AST> {
    sliteral("mut")
        .and_right(sidentifier)
        .and_then(|var_name| {
            sliteral("=").and_right(expression).and_then(move |value| {
                sliteral("[;]").and_right(constant(AST::Variable {
                    span: Span::new_dud(),
                    name: var_name.clone(),
                    value: Box::new(value),
                }))
            })
        })
        .parse(input)
}

#[test]
fn test_var_s() {
    assert_eq!(
        var_s("mut a = c;"),
        Ok((
            "",
            AST::Variable {
                span: Span::new_dud(),
                name: "a".into(),
                value: Box::new(AST::Identifier {
                    name: "c".into(),
                    span: Span::new_dud()
                })
            }
        ))
    );
}

#[allow(dead_code, unused_variables, non_snake_case)]
pub fn parse(input: &str) {}
