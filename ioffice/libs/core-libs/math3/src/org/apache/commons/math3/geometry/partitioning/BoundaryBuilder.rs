// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;

// Java: public class BoundaryBuilder extends Space implements BSPTreeVisitor
pub struct BoundaryBuilder {
}

// Java: public class BoundaryBuilder extends Space implements BSPTreeVisitor
impl BoundaryBuilder {
    // Java: Order visitOrder(BSPTree<S> node)
    pub fn visitOrder(&self) -> Box<dyn std::any::Any> {
        // Java: return Order.PLUS_MINUS_SUB;
        todo!()
    }

    // Java: void visitInternalNode(BSPTree<S> node)
    pub fn visitInternalNode(&self) {
        // Java: SubHyperplane<S> plusOutside = null;
        // Java: SubHyperplane<S> plusInside  = null;
        // Java: NodesSet<S>      splitters   = null;
        // Java: // characterize the cut sub-hyperplane,
        // Java: // first with respect to the plus sub-tree
        // Java: final Characterization<S> plusChar = new Characterization<S>(node.getPlus(), node.getCut().copySelf());
        // Java: if (plusChar.touchOutside())
        {
            // Java: // plusChar.outsideTouching() corresponds to a subset of the cut sub-hyperplane
            // Java: // known to have outside cells on its plus side, we want to check if parts
            // Java: // of this subset do have inside cells on their minus side
            // Java: final Characterization<S> minusChar = new Characterization<S>(node.getMinus(), plusChar.outsideTouching());
            // Java: if (minusChar.touchInside())
            {
                // Java: // this part belongs to the boundary,
                // Java: // it has the outside on its plus side and the inside on its minus side
                // Java: plusOutside = minusChar.insideTouching();
                // Java: splitters = new NodesSet<S>();
                // Java: splitters.addAll(minusChar.getInsideSplitters());
                // Java: splitters.addAll(plusChar.getOutsideSplitters());
            }
        }
        // Java: if (plusChar.touchInside())
        {
            // Java: // plusChar.insideTouching() corresponds to a subset of the cut sub-hyperplane
            // Java: // known to have inside cells on its plus side, we want to check if parts
            // Java: // of this subset do have outside cells on their minus side
            // Java: final Characterization<S> minusChar = new Characterization<S>(node.getMinus(), plusChar.insideTouching());
            // Java: if (minusChar.touchOutside())
            {
                // Java: // this part belongs to the boundary,
                // Java: // it has the inside on its plus side and the outside on its minus side
                // Java: plusInside = minusChar.outsideTouching();
                // Java: if (splitters == null)
                {
                    // Java: splitters = new NodesSet<S>();
                }
                // Java: splitters.addAll(minusChar.getOutsideSplitters());
                // Java: splitters.addAll(plusChar.getInsideSplitters());
            }
        }
        // Java: if (splitters != null)
        {
            // Java: // the parent nodes are natural splitters for boundary sub-hyperplanes
            // Java: for (BSPTree<S> up = node.getParent(); up != null; up = up.getParent())
            {
                // Java: splitters.add(up);
            }
        }
        // Java: // set the boundary attribute at non-leaf nodes
        // Java: node.setAttribute(new BoundaryAttribute<S>(plusOutside, plusInside, splitters));
    }

    // Java: void visitLeafNode(BSPTree<S> node)
    pub fn visitLeafNode(&self) {
    }

}
