// Java: import java.io.OutputStream
use stubs::java::io::OutputStream::*;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
use std::collections::HashMap;

// java public interface ICmtCustDatasWriter
pub trait ICmtCustDatasWriter {
    // java  HashMap<Integer, ArrayList<String>> getHiddenBkmks()
    fn getHiddenBkmks(&self) -> HashMap<i32, Vec<String>>;

    // java  boolean writeItem(String var1)
    fn writeItem__string(&mut self, var1: &str) -> bool;

    // java  boolean writeItem(OutputStream var1)
    fn writeItem__outputstream(&mut self, var1: &dyn std::io::Write) -> bool;

    // java  boolean writeProperties(String var1)
    fn writeProperties__string(&mut self, var1: &str) -> bool;

    // java  boolean writeProperties(OutputStream var1)
    fn writeProperties__outputstream(&mut self, var1: &dyn std::io::Write) -> bool;

}
