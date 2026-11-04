#![allow(dead_code, non_snake_case)]

use super::Node::Node;
use super::VisitorContext::VisitorContext;

// Java: public interface INodeVisitor
pub trait INodeVisitor {
    // Java: abstract void init(int var1)
    fn init(&mut self, var1: i32);

    // Java: abstract void visit(Node var1, VisitorContext var2)
    fn visit(&mut self, var1: *mut Node, var2: &mut VisitorContext);

    // Java: abstract void finish(Node var1)
    fn finish(&mut self, var1: *mut Node);
}
