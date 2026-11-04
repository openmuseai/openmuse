// Java: public class EnumalhsEx
pub struct EnumalhsEx {
    // Java field: int aiAlgid
    pub aiAlgid: i32,
    // Java field: int dwDefaultLen
    pub dwDefaultLen: i32,
    // Java field: int dwMinLen
    pub dwMinLen: i32,
    // Java field: int dwMaxLen
    pub dwMaxLen: i32,
    // Java field: int dwProtocols
    pub dwProtocols: i32,
    // Java field: int dwNameLen
    pub dwNameLen: i32,
    // Java field: int dwLongNameLen
    pub dwLongNameLen: i32,
}

// Java: public class EnumalhsEx
impl EnumalhsEx {
    // Java: EnumalhsEx()
    pub fn new() -> Self {
        // Java: super();
        EnumalhsEx {
            aiAlgid: 0,
            dwDefaultLen: 0,
            dwMinLen: 0,
            dwMaxLen: 0,
            dwProtocols: 0,
            dwNameLen: 0,
            dwLongNameLen: 0,
        }
    }
}
