use std::cmp;
#[derive(Debug, Clone)]
pub struct Span {
    pub file: String,
    pub start_line: i32,
    pub start_col: i32,
    pub end_line: i32,
    pub end_col: i32,
}

impl Span {
    pub fn new_dud() -> Span {
        Span {
            file: "garb.txt".into(),
            start_line: 1,
            start_col: 1,
            end_line: 1,
            end_col: 1,
        }
    }

    pub fn merge_with(&self, other: &Span) -> Span {
        let start_line = cmp::min(self.start_line, other.start_line);
        let start_col = cmp::min(self.start_col, other.start_col);
        let end_line = cmp::max(self.end_line, other.end_line);
        let end_col = cmp::max(self.end_col, other.end_col);

        Span {
            file: self.file.clone(),
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AST {
    Number {
        num: i64,
        span: Span,
    },
    Identifier {
        name: String,
        span: Span,
    },
    Not {
        target: Box<AST>,
        span: Span,
    },
    Equals {
        lhs: Box<AST>,
        rhs: Box<AST>,
    },
    NEquals {
        lhs: Box<AST>,
        rhs: Box<AST>,
    },
    Add {
        lhs: Box<AST>,
        rhs: Box<AST>,
    },
    Subtract {
        lhs: Box<AST>,
        rhs: Box<AST>,
    },
    Multiply {
        lhs: Box<AST>,
        rhs: Box<AST>,
    },
    Divide {
        lhs: Box<AST>,
        rhs: Box<AST>,
    },
    Grouped {
        inner: Box<AST>,
        span: Span,
    },
    Call {
        called: String,
        args: Vec<AST>,
        span: Span,
    },
    Return {
        value: Box<AST>,
        span: Span,
    },
    Block {
        statements: Vec<AST>,
        span: Span,
    },
    IfCond {
        span: Span,
        condition: Box<AST>,
        then: Box<AST>,
        c_else: Box<AST>,
    },
    FunctionDef {
        span: Span,
        name: String,
        params: Vec<String>,
        body: Box<AST>,
    },
    Variable {
        span: Span,
        name: String,
        value: Box<AST>,
    },
}

impl AST {
    pub fn equals(&self, other: &AST) -> bool {
        match (self, other) {
            (
                AST::Number { ref num, span: _ },
                AST::Number {
                    num: ref onum,
                    span: _,
                },
            ) => num == onum,
            (
                AST::Identifier { ref name, span: _ },
                AST::Identifier {
                    name: ref oname,
                    span: _,
                },
            ) => name == oname,
            (
                AST::Not { target, span: _ },
                AST::Not {
                    target: otarget,
                    span: _,
                },
            ) => target.equals(otarget),
            (
                AST::Equals { lhs, rhs },
                AST::Equals {
                    lhs: olhs,
                    rhs: orhs,
                },
            ) => lhs.equals(olhs) && rhs.equals(orhs),
            (
                AST::NEquals { lhs, rhs },
                AST::NEquals {
                    lhs: olhs,
                    rhs: orhs,
                },
            ) => lhs.equals(olhs) && rhs.equals(orhs),
            (
                AST::Add { lhs, rhs },
                AST::Add {
                    lhs: olhs,
                    rhs: orhs,
                },
            ) => lhs.equals(olhs) && rhs.equals(orhs),
            (
                AST::Subtract { lhs, rhs },
                AST::Subtract {
                    lhs: olhs,
                    rhs: orhs,
                },
            ) => lhs.equals(olhs) && rhs.equals(orhs),
            (
                AST::Multiply { lhs, rhs },
                AST::Multiply {
                    lhs: olhs,
                    rhs: orhs,
                },
            ) => lhs.equals(olhs) && rhs.equals(orhs),
            (
                AST::Divide { lhs, rhs },
                AST::Divide {
                    lhs: olhs,
                    rhs: orhs,
                },
            ) => lhs.equals(olhs) && rhs.equals(orhs),
            (
                AST::Grouped { inner, span: _ },
                AST::Grouped {
                    inner: oinner,
                    span: _,
                },
            ) => inner.equals(oinner),
            (
                AST::Call {
                    called,
                    args,
                    span: _,
                },
                AST::Call {
                    called: ocalled,
                    args: oargs,
                    span: _,
                },
            ) => {
                called == ocalled
                    && args.len() == oargs.len()
                    && args
                        .iter()
                        .zip(oargs.iter())
                        .all(|(arg, oarg)| arg.equals(oarg))
            }
            (
                AST::Return { value, span: _ },
                AST::Return {
                    value: ovalue,
                    span: _,
                },
            ) => value.equals(ovalue),
            (
                AST::Block {
                    statements,
                    span: _,
                },
                AST::Block {
                    statements: ostatements,
                    span: _,
                },
            ) => {
                statements.len() == ostatements.len()
                    && statements
                        .iter()
                        .zip(ostatements.iter())
                        .all(|(stmt, ostmt)| stmt.equals(ostmt))
            }
            (
                AST::IfCond {
                    span: _,
                    condition,
                    then,
                    c_else,
                },
                AST::IfCond {
                    span: _,
                    condition: ocondition,
                    then: othen,
                    c_else: oc_else,
                },
            ) => condition.equals(ocondition) && then.equals(othen) && c_else.equals(oc_else),
            (
                AST::FunctionDef {
                    span: _,
                    name,
                    params,
                    body,
                },
                AST::FunctionDef {
                    span: _,
                    name: oname,
                    params: oparams,
                    body: obody,
                },
            ) => {
                name == oname
                    && params.len() == oparams.len()
                    && params
                        .iter()
                        .zip(oparams.iter())
                        .all(|(param, oparam)| param == oparam)
                    && body.equals(obody)
            }
            (
                AST::Variable {
                    span: _,
                    name,
                    value,
                },
                AST::Variable {
                    span: _,
                    name: oname,
                    value: ovalue,
                },
            ) => name == oname && value.equals(ovalue),
            _ => false,
        }
    }

    pub fn get_span(&self) -> Span {
        match self {
            AST::Number { num: _, ref span } => span.clone(),
            AST::Identifier { name: _, ref span } => span.clone(),
            AST::Not {
                target: _,
                ref span,
            } => span.clone(),
            AST::Equals { lhs, rhs } => lhs.get_span().merge_with(&rhs.get_span()),
            AST::NEquals { lhs, rhs } => lhs.get_span().merge_with(&rhs.get_span()),
            AST::Add { lhs, rhs } => lhs.get_span().merge_with(&rhs.get_span()),
            AST::Subtract { lhs, rhs } => lhs.get_span().merge_with(&rhs.get_span()),
            AST::Multiply { lhs, rhs } => lhs.get_span().merge_with(&rhs.get_span()),
            AST::Divide { lhs, rhs } => lhs.get_span().merge_with(&rhs.get_span()),
            AST::Grouped { inner: _, span } => span.clone(),
            AST::Call {
                called: _,
                args: _,
                span,
            } => span.clone(),
            AST::Return { value, span } => span.clone().merge_with(&value.get_span()),
            AST::Block {
                statements: _,
                span,
            } => span.clone(),
            AST::IfCond {
                span,
                condition: _,
                then: _,
                c_else: _,
            } => span.clone(),
            AST::FunctionDef {
                span,
                name: _,
                params: _,
                body: _,
            } => span.clone(),
            AST::Variable {
                span,
                name: _,
                value: _,
            } => span.clone(),
        }
    }
}
