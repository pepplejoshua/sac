pub mod frontend;

use frontend::sac_parser::sac_parser;

fn main() {
    let src = r#"
:factorial n {
    mut res = 1;
    while n != 1 {
        res = res * n;
        n = n - 1;
    }
    ret res;
}"#;
    let res = sac_parser(src);
    println!("{res:#?}")
}
