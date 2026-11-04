// Java: public interface InkMLParser
pub trait InkMLParser {
    // Java: abstract void parseInkMLFile(String var1)
    fn parseInkMLFile(&mut self, var1: &str);

    // Java: abstract void parseInkMLString(String var1)
    fn parseInkMLString(&mut self, var1: &str);

}
