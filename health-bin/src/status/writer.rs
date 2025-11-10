use std::fmt::Write;

pub struct JsonWriter {
    buf: String,
    first: bool,
}

impl JsonWriter {
    pub fn new() -> Self {
        Self {
            buf: String::with_capacity(256),
            first: true,
        }
    }

    pub fn bool(&mut self, key: &str, value: bool) {
        self.prefix(key);
        self.buf.push_str(if value { "true" } else { "false" });
    }

    pub fn u64(&mut self, key: &str, value: u64) {
        self.prefix(key);
        let _ = write!(self.buf, "{value}");
    }

    pub fn f32(&mut self, key: &str, value: f32) {
        self.prefix(key);
        let _ = write!(self.buf, "{value}");
    }

    pub fn str_opt(&mut self, key: &str, value: Option<&str>) {
        self.prefix(key);
        match value {
            Some(text) => self.string(text),
            None => self.buf.push_str("null"),
        }
    }

    pub fn finish(mut self) -> String {
        if !self.first {
            self.buf.push('\n');
        }
        self.buf.push('}');
        self.buf
    }

    fn prefix(&mut self, key: &str) {
        if self.first {
            self.buf.push('{');
            self.buf.push('\n');
            self.first = false;
        } else {
            self.buf.push_str(",\n");
        }
        self.buf.push_str("  \"");
        self.buf.push_str(key);
        self.buf.push_str("\": ");
    }

    fn string(&mut self, value: &str) {
        self.buf.push('"');
        for ch in value.chars() {
            match ch {
                '"' => self.buf.push_str("\\\""),
                '\\' => self.buf.push_str("\\\\"),
                '\n' => self.buf.push_str("\\n"),
                '\r' => self.buf.push_str("\\r"),
                '\t' => self.buf.push_str("\\t"),
                ch if ch < ' ' => {
                    let _ = write!(self.buf, "\\u{:04x}", ch as u32);
                }
                other => self.buf.push(other),
            }
        }
        self.buf.push('"');
    }
}
