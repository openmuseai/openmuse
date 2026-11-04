// Java: import java.util.Iterator
// (Rust uses Iterator trait from std)
// Java: import java.util.List
// (Rust uses Vec<T>)

use super::Node::Node;
use super::Element::Element;
use super::QName::QName;
use super::Comment::Comment;
use super::ProcessingInstruction::ProcessingInstruction;

// Java: public interface Branch extends Node
pub trait Branch: Node {
    // Java: abstract Node node(int var1)
    fn node(&self, var1: i32) -> Option<Box<dyn Node>>;

    // Java: abstract int indexOf(Node var1)
    fn indexOf(&self, var1: &dyn Node) -> i32;

    // Java: abstract int nodeCount()
    fn nodeCount(&self) -> i32;

    // Java: abstract Element elementByID(String var1)
    fn elementByID(&self, var1: &str) -> Option<Box<dyn Element>>;

    // Java: abstract List<Node> content()
    fn content(&self) -> Vec<Box<dyn Node>>;

    // Java: abstract Iterator<Node> nodeIterator()
    fn nodeIterator(&self) -> Box<dyn Iterator<Item = Box<dyn Node>>>;

    // Java: abstract Iterator<Node> iterator()
    fn iterator(&self) -> Box<dyn Iterator<Item = Box<dyn Node>>>;

    // Java: abstract void setContent(List<Node> var1)
    fn setContent(&mut self, var1: Vec<Box<dyn Node>>);

    // Java: abstract void appendContent(Branch var1)
    fn appendContent(&mut self, var1: &dyn Branch);

    // Java: abstract void clearContent()
    fn clearContent(&mut self);

    // Java: abstract List<ProcessingInstruction> processingInstructions()
    fn processingInstructions(&self) -> Vec<Box<dyn ProcessingInstruction>>;

    // Java: abstract List<ProcessingInstruction> processingInstructions(String var1)
    fn processingInstructions_by_target(&self, var1: &str) -> Vec<Box<dyn ProcessingInstruction>>;

    // Java: abstract ProcessingInstruction processingInstruction(String var1)
    fn processingInstruction(&self, var1: &str) -> Option<Box<dyn ProcessingInstruction>>;

    // Java: abstract void setProcessingInstructions(List<ProcessingInstruction> var1)
    fn setProcessingInstructions(&mut self, var1: Vec<Box<dyn ProcessingInstruction>>);

    // Java: abstract Element addElement(String var1)
    fn addElement(&mut self, var1: &str) -> Box<dyn Element>;

    // Java: abstract Element addElement(QName var1)
    fn addElement_with_qname(&mut self, var1: QName) -> Box<dyn Element>;

    // Java: abstract Element addElement(String var1, String var2)
    fn addElement_with_namespace(&mut self, var1: &str, var2: &str) -> Box<dyn Element>;

    // Java: abstract boolean removeProcessingInstruction(String var1)
    fn removeProcessingInstruction(&mut self, var1: &str) -> bool;

    // Java: abstract void add(Node var1)
    fn add_node(&mut self, var1: Box<dyn Node>);

    // Java: abstract void add(Comment var1)
    fn add_comment(&mut self, var1: Box<dyn Comment>);

    // Java: abstract void add(Element var1)
    fn add_element(&mut self, var1: Box<dyn Element>);

    // Java: abstract void add(ProcessingInstruction var1)
    fn add_processing_instruction(&mut self, var1: Box<dyn ProcessingInstruction>);

    // Java: abstract boolean remove(Node var1)
    fn remove_node(&mut self, var1: &dyn Node) -> bool;

    // Java: abstract boolean remove(Comment var1)
    fn remove_comment(&mut self, var1: &dyn Comment) -> bool;

    // Java: abstract boolean remove(Element var1)
    fn remove_element(&mut self, var1: &dyn Element) -> bool;

    // Java: abstract boolean remove(ProcessingInstruction var1)
    fn remove_processing_instruction(&mut self, var1: &dyn ProcessingInstruction) -> bool;

    // Java: abstract void normalize()
    fn normalize(&mut self);
}
