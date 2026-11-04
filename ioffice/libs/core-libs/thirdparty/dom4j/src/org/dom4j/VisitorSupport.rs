use super::Visitor::Visitor;
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

// Java: public class VisitorSupport implements Visitor
pub struct VisitorSupport {
}

// Java: public class VisitorSupport implements Visitor
impl VisitorSupport {
    // Java: VisitorSupport()
    pub fn new() -> Self {
        // Java: super();
        VisitorSupport {}
    }
}

impl Visitor for VisitorSupport {
    // Java: void visit(Document document)
    fn visit_document(&self, _document: &dyn Document) {
    }

    // Java: void visit(DocumentType documentType)
    fn visit_document_type(&self, _documentType: &dyn DocumentType) {
    }

    // Java: void visit(Element node)
    fn visit_element(&self, _node: &dyn Element) {
    }

    // Java: void visit(Attribute node)
    fn visit_attribute(&self, _node: &dyn Attribute) {
    }

    // Java: void visit(CDATA node)
    fn visit_cdata(&self, _node: &dyn CDATA) {
    }

    // Java: void visit(Comment node)
    fn visit_comment(&self, _node: &dyn Comment) {
    }

    // Java: void visit(Entity node)
    fn visit_entity(&self, _node: &dyn Entity) {
    }

    // Java: void visit(Namespace namespace)
    fn visit_namespace(&self, _namespace: &Namespace) {
    }

    // Java: void visit(ProcessingInstruction node)
    fn visit_processing_instruction(&self, _node: &dyn ProcessingInstruction) {
    }

    // Java: void visit(Text node)
    fn visit_text(&self, _node: &dyn Text) {
    }
}
