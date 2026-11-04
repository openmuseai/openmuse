use super::Document::Document;
use super::DocumentType::DocumentType;
use super::Element::Element;
use super::Attribute::Attribute;
use super::CDATA::CDATA;
use super::Comment::Comment;
use super::Entity::Entity;
use super::Namespace::Namespace;
use super::ProcessingInstruction::ProcessingInstruction;
use super::Text::Text;

// Java: public interface Visitor
pub trait Visitor {
    // Java: abstract void visit(Document var1)
    fn visit_document(&self, var1: &dyn Document);

    // Java: abstract void visit(DocumentType var1)
    fn visit_document_type(&self, var1: &dyn DocumentType);

    // Java: abstract void visit(Element var1)
    fn visit_element(&self, var1: &dyn Element);

    // Java: abstract void visit(Attribute var1)
    fn visit_attribute(&self, var1: &dyn Attribute);

    // Java: abstract void visit(CDATA var1)
    fn visit_cdata(&self, var1: &dyn CDATA);

    // Java: abstract void visit(Comment var1)
    fn visit_comment(&self, var1: &dyn Comment);

    // Java: abstract void visit(Entity var1)
    fn visit_entity(&self, var1: &dyn Entity);

    // Java: abstract void visit(Namespace var1)
    fn visit_namespace(&self, var1: &Namespace);

    // Java: abstract void visit(ProcessingInstruction var1)
    fn visit_processing_instruction(&self, var1: &dyn ProcessingInstruction);

    // Java: abstract void visit(Text var1)
    fn visit_text(&self, var1: &dyn Text);
}
