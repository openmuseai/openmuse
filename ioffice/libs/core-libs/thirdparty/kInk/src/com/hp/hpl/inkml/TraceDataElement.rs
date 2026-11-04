use super::InkElement::InkElement;
use super::InkMLSerializer::InkMLWriter;

// Java: public interface TraceDataElement extends InkElement
pub trait TraceDataElement: InkElement {
    // Java: abstract TraceDataElement getSelectedTraceDataByRange(String var1, String var2)
    fn getSelectedTraceDataByRange(&self, var1: &str, var2: &str) -> Result<Box<dyn TraceDataElement>, String>;

    // Java: abstract void setAssociatedContext(Context var1)
    fn setAssociatedContext(&mut self, var1: i64);

    // Java: abstract String toInkML(Definitions var1)
    fn toInkML_defs(&self, var1: i64) -> String;

    // Java: abstract void writeXML(InkMLWriter var1)
    fn writeXML(&self, var1: &mut InkMLWriter);
}
