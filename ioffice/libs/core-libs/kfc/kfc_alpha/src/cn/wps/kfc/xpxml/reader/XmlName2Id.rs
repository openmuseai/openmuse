// Java field: int INVALID_ID
pub const INVALID_ID: i32 = -1;

// Java: public interface XmlName2Id
pub trait XmlName2Id {
    // Java: abstract int getId(int var1, int var2)
    fn get_id(&self, var1: i32, var2: i32) -> i32;
}
