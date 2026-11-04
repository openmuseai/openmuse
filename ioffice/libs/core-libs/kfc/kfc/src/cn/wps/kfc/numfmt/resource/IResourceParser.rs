// Java: import java.io.InputStream
use std::io::Read;
// Java: import java.util.List

// Java: public interface IResourceParser
pub trait IResourceParser {
    // Java: abstract boolean parse(InputStream var1)
    fn parse(&mut self, input: &mut dyn Read) -> bool;

    // Java: abstract String getItem(String var1)
    fn getItem(&self, key: &str) -> String;

    // Java: abstract List<String> getArray(String var1)
    fn getArray(&self, key: &str) -> Vec<String>;

    // Java: abstract void destroy()
    fn destroy(&mut self);

}
