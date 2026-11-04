#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java field:    public static final int EEXIST = 0;
pub const EEXIST: i32 = 0;
// Java field:    public static final int ENOENT = 1;
pub const ENOENT: i32 = 1;

// Java: public class MfsException extends Exception
#[derive(Default, Debug)]
pub struct MfsException {
    // Java field:    public final int mErrno;
    pub mErrno: i32,
}

// Java: public class MfsException extends Exception
impl MfsException {
    // Java: public MfsException(int errno)
    pub fn new(errno: i32) -> Self {
        Self { mErrno: errno }
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: switch (this.mErrno)
        match self.mErrno {
            // Java: case 0: sb.append("The file already exist!");
            0 => sb.push_str("The file already exist!"),
            // Java: case 1: sb.append("No such directory or file!");
            1 => sb.push_str("No such directory or file!"),
            _ => {}
        }
        // Java: return sb.toString();
        sb
    }
}

impl std::fmt::Display for MfsException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.toString())
    }
}
