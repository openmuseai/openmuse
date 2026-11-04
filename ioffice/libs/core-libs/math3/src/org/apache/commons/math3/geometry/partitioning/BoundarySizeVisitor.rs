// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;

// Java: public class BoundarySizeVisitor extends Space implements BSPTreeVisitor
pub struct BoundarySizeVisitor {
    // Java field: double boundarySize
    pub(crate) boundarySize: f64,
}

// Java: public class BoundarySizeVisitor extends Space implements BSPTreeVisitor
impl BoundarySizeVisitor {
    // Java: Order visitOrder(final BSPTree<S> node)
    pub fn visitOrder(&self) -> Box<dyn std::any::Any> {
        // Java: return Order.MINUS_SUB_PLUS;
        todo!()
    }

    // Java: void visitInternalNode(final BSPTree<S> node)
    pub fn visitInternalNode(&self) {
        // Java: @SuppressWarnings("unchecked")
        // Java: final BoundaryAttribute<S> attribute =
            // Java: (BoundaryAttribute<S>) node.getAttribute();
        // Java: if (attribute.getPlusOutside() != null)
        {
            // Java: boundarySize += attribute.getPlusOutside().getSize();
        }
        // Java: if (attribute.getPlusInside() != null)
        {
            // Java: boundarySize += attribute.getPlusInside().getSize();
        }
    }

    // Java: void visitLeafNode(final BSPTree<S> node)
    pub fn visitLeafNode(&self) {
    }

    // Java: double getSize()
    pub fn getSize(&self) -> f64 {
        // Java: return boundarySize;
        todo!()
    }

}

