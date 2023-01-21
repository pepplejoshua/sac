use super::parser::ParseResult;
use regex::Regex;

#[allow(dead_code)]
pub struct Source {
    pub path: String,
    pub content: String,
    pub index: usize,
    pub lines: Vec<String>,
    pub line_no: usize,
    pub col_no: usize,
}

impl Source {
    pub fn match_reg(mut self, exp: &Regex) -> ParseResult<String> {
        let mat = exp.find_at(&self.content, self.index);
        match mat {
            Some(val) => {
                if val.start() != self.index {
                    return ParseResult::None;
                } else {
                    self.index += val.end() - val.start();
                    let matched = self
                        .content
                        .get(val.start()..val.end())
                        .unwrap()
                        .to_string();
                    return ParseResult::Some(matched, self);
                }
            }
            None => ParseResult::None,
        }
    }
}
