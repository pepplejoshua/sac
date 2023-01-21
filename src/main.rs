pub mod frontend;

use frontend::{parser::regexp, source::Source};

use crate::frontend::parser::zero_or_more;

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
}
