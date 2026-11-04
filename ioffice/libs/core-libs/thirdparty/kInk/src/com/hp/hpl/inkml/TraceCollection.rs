use super::TraceDataElement::TraceDataElement;

// Java: public interface TraceCollection extends TraceDataElement
pub trait TraceCollection: TraceDataElement {
    // Java: abstract TraceDataElement getTraceDataAt(int var1)
    fn getTraceDataAt(&self, var1: i32) -> Option<Box<dyn TraceDataElement>>;

}
