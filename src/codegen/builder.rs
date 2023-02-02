use std::{collections::HashMap, ffi::OsStr, fs, path::Path};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]

pub struct Context {
    locals: HashMap<String, i32>,
    outer: Option<Box<Context>>,
}

#[allow(dead_code)]
impl Context {
    fn n() -> Self {
        Context {
            locals: HashMap::new(),
            outer: None,
        }
    }

    fn enter(&self) -> Self {
        Context {
            locals: HashMap::new(),
            outer: Some(Box::new(self.clone())),
        }
    }

    fn set(&mut self, local: String, offset: i32) {
        self.locals.insert(local, offset);
    }
}

pub struct Builder {
    src: String,
    lines: Vec<String>,
    context: Context,
}

impl Builder {
    pub fn n(src: String) -> Self {
        Builder {
            src,
            lines: vec![],
            context: Context::n(),
        }
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

    pub fn set_up_env(&mut self, locals: &[String]) {
        for (i, local) in locals.iter().enumerate() {
            self.context.set(local.clone(), (4 * i - 16) as i32);
        }
    }
}
