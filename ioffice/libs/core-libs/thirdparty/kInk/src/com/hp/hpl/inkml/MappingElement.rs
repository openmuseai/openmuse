#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use super::DefinitionElement::DefinitionElement;

// Java: public interface MappingElement extends DefinitionElement
pub trait MappingElement: DefinitionElement {
    // Java: abstract MappingElement clone()
    fn clone_me(&self) -> Box<dyn MappingElement>;
}
