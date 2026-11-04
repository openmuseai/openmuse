// Java: public interface CppXmlInteraction
pub trait CppXmlInteraction {
    // Java: abstract void addByteArray(byte[] var1, int var2)
    fn addByteArray(&self, _var1: &[u8], _var2: i32);

    // Java: abstract void addAttr(int var1, String var2)
    fn addAttr(&self, _var1: i32, _var2: &str);

    // Java: abstract void addText(String var1)
    fn addText(&self, _var1: &str);

}
