use super::InkElement::InkElement;

// Java: public interface DefinitionElement extends InkElement
pub trait DefinitionElement: InkElement {
    // Java: abstract String getId()
    fn getId(&self) -> String;
}
