pub mod frontend;

use std::rc::Rc;

use frontend::{parser::regexp, source::Source};

use crate::frontend::parser::{constant, zero_or_more};

fn main() {
    let mut src = Source {
        path: "dud.txt".to_string(),
        content: "abc1234".to_string(),
        index: 0,
        lines: vec![],
        line_no: 0,
        col_no: 0,
    };

    let letter = regexp(r"[a-zA-Z]{3}");
    let digit = regexp(r"\d{4}");
    let letter_and_dig = letter.and(digit);
    let res = letter_and_dig.parse(&mut src);
    println!("letter and digit: {:?}.\n", res);

    let mut dud = Source::dud();
    dud.content = "ab123dcsdcer232435".into();
    let letter = regexp(r"[a-zA-Z]{1}");
    let digit = regexp(r"\d{1}");
    let letter_or_digit = letter.or(digit);
    let some_letters_or_digits = zero_or_more(letter_or_digit);
    let res = some_letters_or_digits.parse(&mut dud);
    println!("some letters or digits: {:?}.\n", res);

    // method 1
    let mut dud = Source::dud();
    dud.content = "12,34".into();
    let pair = regexp("[0-9]+").bind(Rc::new(|first| {
        regexp(",").bind(Rc::new(move |_| {
            let f = first.clone();
            regexp("[0-9]+").bind(Rc::new(move |second| {
                let results = vec![f.clone(), second.clone()];
                constant(results)
            }))
        }))
    }));
    let res = pair.parse(&mut dud);
    println!("binding: {:?}.\n", res);

    // method 2
    let mut dud = Source::dud();
    dud.content = "12,34".into();
    let _pair = regexp("[0-9]+").bind(Rc::new(|first| {
        regexp(",")
            .and_drop_left(regexp("[0-9]+").map(Rc::new(move |second| [first.clone(), second])))
    }));
    let res = pair.parse(&mut dud);
    println!("binding: {:?}.\n", res);
}
