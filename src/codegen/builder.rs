use std::{ffi::OsStr, fs, path::Path};

pub struct Builder {
    src: String,
    pub lines: Vec<String>,
}

impl Builder {
    pub fn n(src: String) -> Self {
        Builder { src, lines: vec![] }
    }

    pub fn add(&mut self, line: &str) {
        self.lines.push(line.into());
    }

    pub fn write_out(&self) {
        let asm = self.lines.join("\n") + "\n";
        let output = Path::new(&self.src)
            .file_stem()
            .and_then(OsStr::to_str)
            .unwrap()
            .to_string()
            + ".s";
        fs::write(output, asm).unwrap();
    }

    pub fn get_src(&self) -> String {
        fs::read_to_string(&self.src).unwrap()
    }
}
