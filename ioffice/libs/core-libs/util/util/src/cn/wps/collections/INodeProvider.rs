#![allow(dead_code, non_snake_case)]

use super::Node::Node;

// Java: public interface INodeProvider
pub trait INodeProvider {
    // Java: abstract int getNodeCount()
    fn getNodeCount(&self) -> i32;

    // Java: abstract boolean hasNext()
    fn hasNext(&self) -> bool;

    // Java: abstract Node next()
    fn next(&mut self) -> *mut Node;
}
