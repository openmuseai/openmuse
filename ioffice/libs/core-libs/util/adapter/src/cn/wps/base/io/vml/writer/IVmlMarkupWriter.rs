#![allow(dead_code, non_snake_case)]
// Java: import java.io.IOException
// Java: import java.util.ArrayList

// Java: public interface IVmlMarkupWriter
pub trait IVmlMarkupWriter {
    // Java: abstract void startElement(String var1, ArrayList<String> var2)
    fn startElement_list(&mut self, _var1: &str, _var2: &[String]);

    // Java: abstract void startElement(String var1, String... var2)
    fn startElement_varargs(&mut self, _var1: &str, _var2: &[&str]);

    // Java: abstract void endElement(String var1)
    fn endElement(&mut self, _var1: &str);

    // Java: abstract void writeElement(String var1, String... var2)
    fn writeElement_varargs(&mut self, _var1: &str, _var2: &[&str]);

    // Java: abstract void writeElement(String var1, String[] var2, int var3)
    fn writeElement_array(&mut self, _var1: &str, _var2: &[&str], _var3: i32);

}
