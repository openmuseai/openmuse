// Java: public interface Branch extends Node
pub trait Branch {
    // Java: abstract Node node(int var1)
    fn node(&self, var1: i32) -> Option<&dyn super::Node::Node>;

    // Java: abstract int indexOf(Node var1)
    fn indexOf(&self, var1: &dyn super::Node::Node) -> i32;

    // Java: abstract int nodeCount()
    fn nodeCount(&self) -> i32;

    // Java: abstract Element elementByID(String var1)
    fn elementByID(&self, var1: &str) -> Option<Box<dyn std::any::Any>>;

    // Java: abstract Iterator<Node> nodeIterator()
    fn nodeIterator(&self) -> Box<dyn Iterator<Item = Box<dyn super::Node::Node>>>;

    // Java: abstract Iterator<Node> iterator()
    fn iterator(&self) -> Box<dyn Iterator<Item = Box<dyn super::Node::Node>>>;

    // Java: abstract void appendContent(Branch var1)
    fn appendContent(&mut self, var1: &dyn Branch);

    // Java: abstract void clearContent()
    fn clearContent(&mut self);

    // Java: abstract Element addElement(String var1)
    fn addElement_by_name(&mut self, var1: &str) -> Box<dyn std::any::Any>;

    // Java: abstract Element addElement(String var1, String var2, String var3)
    fn addElement_by_qname(&mut self, var1: &str, var2: &str, var3: &str) -> Box<dyn std::any::Any>;

    // Java: abstract void add(Node var1)
    fn add_node(&mut self, var1: Box<dyn super::Node::Node>);

    // Java: abstract void add(Comment var1)
    fn add_comment(&mut self, var1: Box<dyn std::any::Any>);

    // Java: abstract void add(Element var1)
    fn add_element(&mut self, var1: Box<dyn std::any::Any>);

    // Java: abstract void add(ProcessingInstruction var1)
    fn add_processing_instruction(&mut self, var1: Box<dyn std::any::Any>);

    // Java: abstract boolean remove(Node var1)
    fn remove_node(&mut self, var1: &dyn super::Node::Node) -> bool;

    // Java: abstract boolean removeRecursively(Node var1)
    fn removeRecursively(&mut self, var1: &dyn super::Node::Node) -> bool;

    // Java: abstract void removeAllRecursively()
    fn removeAllRecursively(&mut self);

    // Java: abstract void setContent(List var1)
    fn setContent(&mut self, var1: Vec<Box<dyn super::Node::Node>>);

    // Java: abstract boolean remove(Comment var1)
    fn remove_comment(&mut self, var1: &dyn std::any::Any) -> bool;

    // Java: abstract boolean remove(Element var1)
    fn remove_element(&mut self, var1: &dyn std::any::Any) -> bool;

    // Java: abstract boolean remove(ProcessingInstruction var1)
    fn remove_processing_instruction(&mut self, var1: &dyn std::any::Any) -> bool;

    // Java: abstract void normalize()
    fn normalize(&mut self);
}
