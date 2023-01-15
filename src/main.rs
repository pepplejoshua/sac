mod ast;

use ast::{Span, AST};

fn main() {
    let mut moded_span = Span::new_dud();
    moded_span.start_line = 1;
    moded_span.start_col = 5;
    moded_span.end_line = 7;
    moded_span.end_col = 8;

    let a = AST::Number {
        num: 3,
        span: Span::new_dud(),
    };

    let b = AST::Number {
        num: 4,
        span: moded_span.clone(),
    };

    let c = AST::Identifier {
        name: "id".into(),
        span: Span::new_dud(),
    };

    let d = a.clone();

    let e = AST::Identifier {
        name: "uid".into(),
        span: Span::new_dud(),
    };

    let f = AST::Not {
        target: Box::new(b.clone()),
        span: Span::new_dud(),
    };

    let g = AST::Not {
        target: Box::new(b.clone()),
        span: Span::new_dud(),
    };

    let h = AST::Equals {
        lhs: Box::new(a.clone()),
        rhs: Box::new(b.clone()),
    };

    let i = AST::NEquals {
        lhs: Box::new(c.clone()),
        rhs: Box::new(e.clone()),
    };

    let j = AST::Add {
        lhs: Box::new(i.clone()),
        rhs: Box::new(g.clone()),
    };

    let k = AST::Grouped {
        inner: Box::new(j.clone()),
        span: moded_span.clone(),
    };

    if let AST::Grouped { inner, span: _ } = k {
        assert!(inner.equals(&j));
    }

    let l = AST::Call {
        called: "add".into(),
        args: vec![a.clone(), b.clone(), c.clone(), e.clone()],
        span: moded_span.clone(),
    };

    let m = AST::Call {
        called: "add".into(),
        args: vec![a.clone(), b.clone(), c.clone(), e.clone()],
        span: moded_span,
    };

    assert!(!a.equals(&b));
    assert!(a.equals(&d));
    assert!(!c.equals(&e));
    assert!(c.equals(&c));
    assert!(f.equals(&g));
    assert!(j.equals(&j));
    assert!(l.equals(&m));

    println!("{:#?}", j);
    println!("{:#?}\n{:#?}", h.get_span(), i.get_span());
}
