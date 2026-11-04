// Java: public class MyStringBuilder
pub struct MyStringBuilder {
    // Java field: StringBuilder sb
    pub(crate) sb: String,
}

// Java: public class MyStringBuilder
impl MyStringBuilder {
    // Java: MyStringBuilder()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.sb = new StringBuilder();
        MyStringBuilder { sb: String::new() }
    }

    // Java: MyStringBuilder(String s)
    pub fn from_str(s: &str) -> Self {
        // Java: super();
        // Java: this.sb = new StringBuilder(s);
        MyStringBuilder { sb: String::from(s) }
    }

    // Java: void app()
    pub fn app(&mut self) {
        // Java: this.sb.append("\n");
        self.sb.push('\n');
    }

    // Java: void app(String s)
    pub fn app_str(&mut self, s: &str) {
        // Java: if (s.endsWith("\n"))
        if s.ends_with('\n') {
            // Java: this.sb.append(s);
            self.sb.push_str(s);
        // Java: else
        } else {
            // Java: this.sb.append(s + "\n");
            self.sb.push_str(s);
            self.sb.push('\n');
        }
    }

    // Java: void app(long l)
    pub fn app_long(&mut self, l: i64) {
        // Java: this.sb.append(l);
        self.sb.push_str(&l.to_string());
        // Java: this.sb.append("\n");
        self.sb.push('\n');
    }

    // Java: void appn(String s)
    pub fn appn(&mut self, s: &str) {
        // Java: this.sb.append(s);
        self.sb.push_str(s);
    }

    // Java: byte[] getBytes()
    pub fn getBytes(&self) -> Vec<u8> {
        // Java: return this.toString().getBytes();
        self.toString().into_bytes()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return this.sb.toString();
        self.sb.clone()
    }

    // Java: void insert(int offset, String str)
    pub fn insert(&mut self, offset: i32, str: &str) {
        // Java: this.sb.insert(offset, str);
        self.sb.insert_str(offset as usize, str);
    }
}
