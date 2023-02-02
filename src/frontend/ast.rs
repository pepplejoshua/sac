use super::span::Span;
use crate::codegen::builder::Builder;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AST {
    Number {
        num: i32,
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
    Assignment {
        span: Span,
        name: String,
        value: Box<AST>,
    },
    WhileLoop {
        span: Span,
        condition: Box<AST>,
        body: Box<AST>,
    },
    Error {
        span: Span,
        msg: String,
    },
    Assert {
        cond: Box<AST>,
    },
    Main {
        stmts: Box<AST>,
    },
}

#[allow(dead_code)]
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
            (
                AST::Assignment {
                    span: _,
                    name,
                    value,
                },
                AST::Assignment {
                    span: _,
                    name: oname,
                    value: ovalue,
                },
            ) => name == oname && value.equals(ovalue),
            (
                AST::WhileLoop {
                    span: _,
                    condition,
                    body,
                },
                AST::WhileLoop {
                    span: _,
                    condition: ocondition,
                    body: obody,
                },
            ) => condition.equals(ocondition) && body.equals(obody),
            (AST::Main { stmts }, AST::Main { stmts: ostmts }) => stmts.equals(ostmts),
            (AST::Assert { cond }, AST::Assert { cond: ocond }) => cond.equals(ocond),
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
            AST::Assignment {
                span,
                name: _,
                value: _,
            } => span.clone(),
            AST::WhileLoop {
                span,
                condition: _,
                body: _,
            } => span.clone(),
            AST::Error { span, msg: _ } => span.clone(),
            AST::Assert { cond: _ } => Span::new_dud(),
            AST::Main { stmts: _ } => Span::new_dud(),
        }
    }

    pub fn emit_arm32(&self, b: &mut Builder) {
        match self {
            AST::Main { stmts } => {
                b.add(".global main");
                b.add("main:");
                b.add("  push {fp, lr}");
                stmts.emit_arm32(b);
                b.add("  mov r0, #0");
                b.add("  pop {fp, pc}");
            }
            AST::Block {
                statements,
                span: _,
            } => statements.iter().for_each(|stmt| {
                stmt.emit_arm32(b);
            }),
            AST::Assert { cond } => {
                cond.emit_arm32(b);
                b.add("  cmp r0, #1");
                b.add("  moveq r0, #'.'");
                b.add("  movne r0, #'F'");
                b.add("  bl putchar");
                b.add("  mov r0, #'\\n'");
                b.add("  bl putchar");
            }
            AST::Not { target, span: _ } => {
                target.emit_arm32(b);
                b.add("  cmp r0, #0");
                b.add("  moveq r0, #1");
                b.add("  movne r0, #0");
            }
            AST::Add { lhs, rhs } => {
                lhs.emit_arm32(b);
                b.add("  push {r0, ip}"); // stores r0 to be later used by r1
                rhs.emit_arm32(b);
                b.add("  pop {r1, ip}"); // gets r1 value
                b.add("  add r0, r0, r1");
            }
            AST::Subtract { lhs, rhs } => {
                lhs.emit_arm32(b);
                b.add("  push {r0, ip}"); // stores r0 to be later used by r1
                rhs.emit_arm32(b);
                b.add("  pop {r1, ip}"); // gets r1 value
                b.add("  sub r0, r0, r1");
            }
            AST::Multiply { lhs, rhs } => {
                lhs.emit_arm32(b);
                b.add("  push {r0, ip}"); // stores r0 to be later used by r1
                rhs.emit_arm32(b);
                b.add("  pop {r1, ip}"); // gets r1 value
                b.add("  mul r2, r0, r1");
                b.add("  mov r0, r2");
            }
            AST::Divide { lhs, rhs } => {
                lhs.emit_arm32(b);
                b.add("  push {r0, ip}"); // stores r0 to be later used by r1
                rhs.emit_arm32(b);
                b.add("  pop {r1, ip}"); // gets r1 value
                b.add("  udiv r0, r1, r0");
            }
            AST::Equals { lhs, rhs } => {
                lhs.emit_arm32(b);
                b.add("  push {r0, ip}"); // stores r0 to be later used by r1
                rhs.emit_arm32(b);
                b.add("  pop {r1, ip}"); // gets r1 value
                b.add("  cmp r0, r1");
                b.add("  moveq r0, #1");
                b.add("  movne r0, #0");
            }
            AST::NEquals { lhs, rhs } => {
                lhs.emit_arm32(b);
                b.add("  push {r0, ip}"); // stores r0 to be later used by r1
                rhs.emit_arm32(b);
                b.add("  pop {r1, ip}"); // gets r1 value
                b.add("  cmp r0, r1");
                b.add("  moveq r0, #0");
                b.add("  movne r0, #1");
            }
            AST::Number { num, span: _ } => b.add(format!("  ldr r0, ={num}").as_str()),
            _ => b.add("unimplemented"),
        }
    }
}
