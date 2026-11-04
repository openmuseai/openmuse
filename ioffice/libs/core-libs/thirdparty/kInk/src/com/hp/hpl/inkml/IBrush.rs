use std::collections::HashMap;
use super::ContextElement::ContextElement;
use super::InkMLSerializer::InkMLWriter;

// Java: public interface IBrush extends ContextElement
pub trait IBrush: ContextElement {
    // Java: abstract String getBrushProperty(String var1)
    fn getBrushProperty(&self, var1: &str) -> String;

    // Java: abstract Annotation getAnnotation()
    fn getAnnotation(&self) -> Option<String>;

    // Java: abstract AnnotationXML getAnnotationXML()
    fn getAnnotationXML(&self) -> Option<String>;

    // Java: abstract String toInkML()
    fn toInkML(&self) -> String;

    // Java: abstract void writeXML(InkMLWriter var1)
    fn writeXML(&self, var1: &mut InkMLWriter);

    // Java: abstract boolean isDefault()
    fn isDefault(&self) -> bool;

    // Java: abstract HashMap<String, BrushProperty> getProperties()
    fn getProperties(&self) -> &HashMap<String, String>;

    // Java: abstract void setBrushProperty(String var1, String var2, String var3)
    fn setBrushProperty(&mut self, var1: &str, var2: &str, var3: &str);

    // Java: abstract IBrush clone()
    fn clone_brush(&self) -> Box<dyn IBrush>;
}
