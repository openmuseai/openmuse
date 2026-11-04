// Java: public enum SecurityMode
pub struct SecurityMode {
    value: i32,
}

// Java: public enum SecurityMode
impl SecurityMode {
    pub const DEFAULT: SecurityMode = SecurityMode { value: 0 };
    pub const NORMAL: SecurityMode = SecurityMode { value: 1 };
    pub const SECURITY: SecurityMode = SecurityMode { value: 2 };

    // Java: SecurityMode()
    pub fn new(value: i32) -> Self {
        SecurityMode { value }
    }

}
