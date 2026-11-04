// Java: public interface InkMLSerializer
pub struct InkMLWriter;

impl InkMLWriter {
    pub fn writeStartTag(&mut self, _tag: &str, _attrs: Option<&std::collections::HashMap<String, String>>) {}
    pub fn writeEmptyStartTag(&mut self, _tag: &str, _attrs: Option<&std::collections::HashMap<String, String>>) {}
    pub fn writeEndTag(&mut self, _tag: &str) {}
    pub fn incrementTagLevel(&mut self) {}
    pub fn decrementTagLevel(&mut self) {}
}

// Java: public interface InkMLSerializer
pub trait InkMLSerializer {
    // Java: abstract String toInkML()
    fn toInkML(&self) -> String;

    // Java: abstract void writeXML(InkMLWriter var1)
    fn writeXML(&self, var1: &mut InkMLWriter);
}
