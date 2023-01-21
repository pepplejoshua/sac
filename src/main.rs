pub mod frontend;

use frontend::{parser::Parser, source::Source};

fn main() {
    let mut src = Source {
        path: "dud.txt".to_string(),
        content: "abc1234".to_string(),
        index: 0,
        lines: vec![],
        line_no: 0,
        col_no: 0,
    };

    let letter = Parser::<String>::regexp(r"[a-zA-Z]{3}");
    let digit = Parser::<String>::regexp(r"\d{4}");
    let letter_and_dig = letter.and(digit);
    let res = letter_and_dig.parse(&mut src);
    println!("{:?}", res);
}
