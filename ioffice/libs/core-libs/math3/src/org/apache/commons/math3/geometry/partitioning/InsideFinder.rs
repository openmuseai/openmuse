// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;

// Java: public class checking
pub struct checking {
}

// Java: public class InsideFinder
pub struct InsideFinder {
    // Java field: Region<S> region
    pub(crate) region: Box<dyn std::any::Any>,
    // Java field: boolean plusFound
    pub(crate) plusFound: bool,
    // Java field: boolean minusFound
    pub(crate) minusFound: bool,
}

// Java: public class checking
impl checking {
}

// Java: public class InsideFinder
impl InsideFinder {
    // Java: void recurseSides(final BSPTree<S> node, final SubHyperplane<S> sub)
    pub fn recurseSides(&self) {
        // Java: if (node.getCut() == null)
        {
            // Java: if ((Boolean) node.getAttribute())
            {
                // Java: // this is an inside cell expanding across the hyperplane
                // Java: plusFound  = true;
                // Java: minusFound = true;
            }
            // Java: return;
        }
        // Java: final Hyperplane<S> hyperplane = node.getCut().getHyperplane();
        // Java: final SubHyperplane.SplitSubHyperplane<S> split = sub.split(hyperplane);
        // Java: switch (split.getSide())
        {
        // Java: case PLUS :
            // Java: // the sub-hyperplane is entirely in the plus sub-tree
            // Java: if (node.getCut().split(sub.getHyperplane()).getSide() == Side.PLUS)
            {
                // Java: if (!region.isEmpty(node.getMinus()))
                {
                    // Java: plusFound  = true;
                }
            }
            // Java: else
            {
                // Java: if (!region.isEmpty(node.getMinus()))
                {
                    // Java: minusFound = true;
                }
            }
            // Java: if (!(plusFound && minusFound))
            {
                // Java: recurseSides(node.getPlus(), sub);
            }
            // Java: break;
        // Java: case MINUS :
            // Java: // the sub-hyperplane is entirely in the minus sub-tree
            // Java: if (node.getCut().split(sub.getHyperplane()).getSide() == Side.PLUS)
            {
                // Java: if (!region.isEmpty(node.getPlus()))
                {
                    // Java: plusFound  = true;
                }
            }
            // Java: else
            {
                // Java: if (!region.isEmpty(node.getPlus()))
                {
                    // Java: minusFound = true;
                }
            }
            // Java: if (!(plusFound && minusFound))
            {
                // Java: recurseSides(node.getMinus(), sub);
            }
            // Java: break;
        // Java: case BOTH :
            // Java: // the sub-hyperplane extends in both sub-trees
            // Java: // explore first the plus sub-tree
            // Java: recurseSides(node.getPlus(), split.getPlus());
            // Java: // if needed, explore the minus sub-tree
            // Java: if (!(plusFound && minusFound))
            {
                // Java: recurseSides(node.getMinus(), split.getMinus());
            }
            // Java: break;
        // Java: default :
            // Java: // the sub-hyperplane and the cut sub-hyperplane share the same hyperplane
            // Java: if (node.getCut().getHyperplane().sameOrientationAs(sub.getHyperplane()))
            {
                // Java: if ((node.getPlus().getCut() != null) || ((Boolean) node.getPlus().getAttribute()))
                {
                    // Java: plusFound  = true;
                }
                // Java: if ((node.getMinus().getCut() != null) || ((Boolean) node.getMinus().getAttribute()))
                {
                    // Java: minusFound = true;
                }
            }
            // Java: else
            {
                // Java: if ((node.getPlus().getCut() != null) || ((Boolean) node.getPlus().getAttribute()))
                {
                    // Java: minusFound = true;
                }
                // Java: if ((node.getMinus().getCut() != null) || ((Boolean) node.getMinus().getAttribute()))
                {
                    // Java: plusFound  = true;
                }
            }
        }
    }

    // Java: boolean plusFound()
    pub fn plusFound(&self) -> bool {
        // Java: return plusFound;
        todo!()
    }

    // Java: boolean minusFound()
    pub fn minusFound(&self) -> bool {
        // Java: return minusFound;
        todo!()
    }

}
