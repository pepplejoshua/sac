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
        span: moded_span,
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

    assert!(!a.equals(&b));
    assert!(a.equals(&d));
    assert!(!c.equals(&e));
    assert!(c.equals(&c));
    assert!(f.equals(&g));

    println!("{:#?}\n{:#?}", h.get_span(), i.get_span());
}
