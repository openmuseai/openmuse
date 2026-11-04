// Java: public interface XmlName2Id
pub trait XmlName2Id {
    // Java: abstract int getId(String var1, String var2)
    fn getId_str(&self, var1: &str, var2: &str) -> i32;

    // Java: abstract int getId(int var1, int var2)
    fn getId_int(&self, var1: i32, var2: i32) -> i32;
}

pub struct XmlName2IdConst;

// Java: public interface XmlName2Id
impl XmlName2IdConst {
    // Java field: int INVALID_ID
    pub const INVALID_ID: i32 = -1;

    // Java: abstract int getId(String var1, String var2)
    // Java: abstract int getId(int var1, int var2)
}
