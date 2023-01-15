#[derive(Debug, Clone)]
pub struct Span {
    pub file: String,
    pub line: i32,
    pub col: i32,
    pub flat_index_start: i32,
    pub length: usize,
}

#[derive(Debug, Clone)]
pub enum AST {
    Number { num: i64, span: Span },
    Identifier { name: String, span: Span },
    Not { target: Box<AST>, span: Span },
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
        }
    }
}
