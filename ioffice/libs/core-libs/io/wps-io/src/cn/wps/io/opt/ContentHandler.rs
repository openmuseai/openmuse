#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

// Java: public interface ContentHandler
pub trait ContentHandler {
    // Java: abstract void startElement(int var1, String var2, String var3, String var4, Attributes var5)
    fn startElement(&mut self, var1: i32, var2: &str, var3: &str, var4: &str, var5: &dyn std::any::Any);

    // Java: abstract void endElement(int var1, String var2, String var3, String var4)
    fn endElement(&mut self, var1: i32, var2: &str, var3: &str, var4: &str);

    // Java: abstract void character(char[] var1, int var2, int var3)
    fn character_buf(&mut self, var1: &[char], var2: i32, var3: i32);

    // Java: abstract void character(String var1)
    fn character_str(&mut self, var1: &str);
}
