use super::InkMLSerializer::InkMLSerializer;

// Java: public interface InkElement extends InkMLSerializer
pub trait InkElement: InkMLSerializer {
    // Java: abstract String getInkElementType()
    fn getInkElementType(&self) -> String;

    // Java: abstract String getId()
    fn getId(&self) -> String;
}
