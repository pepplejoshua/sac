pub mod frontend;

use frontend::{
    parser::{ParseResult, Parser},
    source::Source,
};

fn main() {
    let mut src = Source {
        path: "dud.txt".to_string(),
        content: "hello1 world2".to_string(),
        index: 0,
        lines: vec![],
        line_no: 0,
        col_no: 0,
    };

    let re = Parser::<String>::regexp(r"hello\d{1}");

    match re.parse(&mut src) {
        ParseResult::Some(val, _) => println!("matched {val}."),
        ParseResult::None => println!("No match"),
    }
}
