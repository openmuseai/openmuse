// Java: public interface FixedField
pub trait FixedField {
    // Java: abstract void readFromBytes(_var1: &[u8])
    fn readFromBytes(&mut self, var1: &[u8]);
    // Java: abstract void readFromStream(InputStream var1)
    fn readFromStream(&mut self, var1: &mut dyn std::io::Read);
    // Java: abstract void writeToBytes(_var1: &[u8])
    fn writeToBytes(&self, var1: &mut [u8]);
    // Java: abstract String toString()
    fn to_string(&self) -> String;
}
