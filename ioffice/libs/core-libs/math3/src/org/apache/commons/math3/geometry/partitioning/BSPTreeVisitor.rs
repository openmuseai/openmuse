// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;

// Java: public enum Order
pub enum Order {
}

// Java: public interface is
pub trait is {
}

// Java: public interface which
pub trait which {
}

// Java: public interface BSPTreeVisitor
pub trait BSPTreeVisitor {
    // Java: abstract Order visitOrder(BSPTree<S> node)
    fn visitOrder(&self) -> Box<dyn std::any::Any>;

    // Java: abstract void visitInternalNode(BSPTree<S> node)
    fn visitInternalNode(&self);

    // Java: abstract void visitLeafNode(BSPTree<S> node)
    fn visitLeafNode(&self);

}
