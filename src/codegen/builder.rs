use std::{collections::HashMap, ffi::OsStr, fs, path::Path};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Label {
    tag: String,
    value: i32,
}

static mut LABEL_COUNTER: i32 = 0;
#[allow(dead_code)]
impl Label {
    pub fn n() -> Self {
        let label = Label {
            value: unsafe { LABEL_COUNTER },
            tag: String::new(),
        };
        unsafe {
            LABEL_COUNTER += 1;
        }
        label
    }

    pub fn nn(value: i32, tag: String) -> Self {
        Label { value, tag }
    }

    pub fn s(&self) -> String {
        format!(".SacLabel{}{}", self.tag, self.value)
    }
}

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

    fn exit(&self) -> Option<Box<Context>> {
        self.outer.clone()
    }

    fn set(&mut self, local: String, offset: i32) {
        self.locals.insert(local, offset);
    }

    fn get(&self, local: &String) -> Option<&i32> {
        self.locals.get(local)
    }
}

pub struct Builder {
    src: String,
    lines: Vec<String>,
    context: Context,
    interned_strings: Vec<(String, Label)>,
}

impl Builder {
    pub fn n(src: String) -> Self {
        Builder {
            src,
            lines: vec![],
            context: Context::n(),
            interned_strings: vec![],
        }
    }

    pub fn add(&mut self, line: &str) {
        self.lines.push(line.into());
    }

    pub fn add_at(&mut self, line: &str, loc: usize) {
        self.lines.insert(loc, line.into());
    }

    pub fn write_out(&mut self) {
        if !self.interned_strings.is_empty() {
            self.add_at("", 0);
            for (istr, label) in self.interned_strings.clone().iter() {
                // generates:
                // label:
                //   .string "string contents"
                //   .balign
                self.add_at(
                    &format!(
                        r#"
{}:
  .string "{}"
  .balign 4"#,
                        label.s(),
                        istr,
                    ),
                    0,
                );
            }
        }
        let mut asm = self.lines.join("\n").trim_start().to_string();
        asm += "\n";
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
            self.context
                .set(local.clone(), (4 * i - (locals.len() * 4)) as i32);
        }
    }

    pub fn enter_ctx(&mut self) {
        let new_ctx = self.context.enter();
        self.context = new_ctx;
    }

    pub fn exit_ctx(&mut self) {
        if let Some(ctx) = self.context.exit() {
            self.context = *ctx;
        } else {
            panic!("cannot leave orphaned scope :(");
        }
    }

    pub fn try_get(&self, local: &String) -> Option<&i32> {
        self.context.get(local)
    }

    pub fn add_interned_str(&mut self, istr: String) -> Label {
        for (e_istr, ilabel) in &self.interned_strings {
            if e_istr == &istr {
                return ilabel.clone();
            }
        }
        let index = self.interned_strings.len() as i32;
        let label = Label::nn(index, "Interned".into());
        self.interned_strings.push((istr, label.clone()));
        label
    }
}
