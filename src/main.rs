mod ast;

use ast::{Span, AST};

fn main() {
    let a = AST::Number {
        num: 3,
        span: Span {
            file: "garb.txt".into(),
            line: 1,
            col: 1,
            flat_index_start: 1,
            length: 1,
        },
    };

    let b = AST::Number {
        num: 4,
        span: Span {
            file: "garb.txt".into(),
            line: 1,
            col: 1,
            flat_index_start: 1,
            length: 1,
        },
    };

    let c = AST::Identifier {
        name: "id".into(),
        span: Span {
            file: "garb.txt".into(),
            line: 1,
            col: 1,
            flat_index_start: 1,
            length: 1,
        },
    };

    let d = a.clone();

    let e = AST::Identifier {
        name: "uid".into(),
        span: Span {
            file: "garb.txt".into(),
            line: 1,
            col: 1,
            flat_index_start: 1,
            length: 1,
        },
    };

    let f = AST::Not {
        target: Box::new(b.clone()),
        span: Span {
            file: "garb.txt".into(),
            line: 1,
            col: 1,
            flat_index_start: 1,
            length: 1,
        },
    };

    let g = AST::Not {
        target: Box::new(b.clone()),
        span: Span {
            file: "garb.txt".into(),
            line: 1,
            col: 1,
            flat_index_start: 1,
            length: 1,
        },
    };

    assert!(!a.equals(&b));
    assert!(a.equals(&d));
    assert!(!c.equals(&e));
    assert!(c.equals(&c));

    assert!(f.equals(&g));

    println!("{:#?}", a.get_span());
}
