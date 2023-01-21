pub mod frontend;

use frontend::{
    parser::{Parser, RegExp},
    source::Source,
};

fn main() {
    let r = RegExp {
        regex: regex::Regex::new(r"hello\d{1}").unwrap(),
    };

    let src = Source {
        path: "dud.txt".to_string(),
        content: "hello1 world2".to_string(),
        index: 0,
        lines: vec![],
        line_no: 0,
        col_no: 0,
    };

    match r.parse(src) {
        frontend::parser::ParseResult::Some(text, sc) => {
            println!("new index: {:}, text: {text}", sc.index);
        }
        frontend::parser::ParseResult::None => println!("Not a match"),
    }
}
