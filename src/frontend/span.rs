use std::cmp;

#[derive(Debug, Clone)]
pub struct Span {
    pub file: String,
    pub start_line: i32,
    pub start_col: i32,
    pub end_line: i32,
    pub end_col: i32,
}

#[allow(dead_code)]
impl Span {
    pub fn new_dud() -> Span {
        Span {
            file: "garb.txt".into(),
            start_line: 1,
            start_col: 1,
            end_line: 1,
            end_col: 1,
        }
    }

    pub fn merge_with(&self, other: &Span) -> Span {
        let start_line = cmp::min(self.start_line, other.start_line);
        let start_col = cmp::min(self.start_col, other.start_col);
        let end_line = cmp::max(self.end_line, other.end_line);
        let end_col = cmp::max(self.end_col, other.end_col);

        Span {
            file: self.file.clone(),
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }
}
