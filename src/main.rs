pub mod codegen;
pub mod frontend;

use codegen::builder;
use frontend::sac_parser::sac_parser;

fn main() {
    let mut b = builder::Builder::n("./assert.sac".into());
    let (_, res) = sac_parser(&b.get_src()).unwrap();
    res.emit_arm32(&mut b);
    b.write_out();
}
