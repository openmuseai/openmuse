// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;
// Java: import org.apache.commons.math3.geometry.partitioning.BSPTree.VanishingCutHandler
use crate::org::apache::commons::math3::geometry::partitioning::BSPTree::VanishingCutHandler;
// Java: import org.apache.commons.math3.geometry.partitioning.Region.Location
use crate::org::apache::commons::math3::geometry::partitioning::Region::Location;
// Java: import org.apache.commons.math3.geometry.partitioning.SubHyperplane.SplitSubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::SubHyperplane::SplitSubHyperplane;

// Java: public class is
pub struct is {
}

// Java: public class RegionFactory extends Space
pub struct RegionFactory {
    // Java field: NodesCleaner nodeCleaner
    nodeCleaner: NodesCleaner,
}

// Java: public class UnionMerger
pub struct UnionMerger {
}

// Java: public class IntersectionMerger
pub struct IntersectionMerger {
}

// Java: public class XorMerger
pub struct XorMerger {
}

// Java: public class DifferenceMerger
pub struct DifferenceMerger {
    // Java field: Region<S> region1
    pub(crate) region1: Box<dyn std::any::Any>,
    // Java field: Region<S> region2
    pub(crate) region2: Box<dyn std::any::Any>,
}

// Java: public class NodesCleaner
pub struct NodesCleaner {
}

// Java: public class VanishingToLeaf
pub struct VanishingToLeaf {
    // Java field: boolean inside
    pub(crate) inside: bool,
}

// Java: public class is
impl is {
}

// Java: public class RegionFactory extends Space
impl RegionFactory {
    // Java: RegionFactory()
    pub fn RegionFactory(&self) {
        // Java: nodeCleaner = new NodesCleaner();
    }

    // Java: Region<S> buildConvex(final Hyperplane<S> ... hyperplanes)
    pub fn buildConvex(&self) -> Box<dyn std::any::Any> {
        // Java: if ((hyperplanes == null) || (hyperplanes.length == 0))
        {
            // Java: return null;
        }
        // Java: // use the first hyperplane to build the right class
        // Java: final Region<S> region = hyperplanes[0].wholeSpace();
        // Java: // chop off parts of the space
        // Java: BSPTree<S> node = region.getTree(false);
        // Java: node.setAttribute(Boolean.TRUE);
        // Java: for (final Hyperplane<S> hyperplane : hyperplanes)
        {
            // Java: if (node.insertCut(hyperplane))
            {
                // Java: node.setAttribute(null);
                // Java: node.getPlus().setAttribute(Boolean.FALSE);
                // Java: node = node.getMinus();
                // Java: node.setAttribute(Boolean.TRUE);
            }
            // Java: else
            {
                // Java: // the hyperplane could not be inserted in the current leaf node
                // Java: // either it is completely outside (which means the input hyperplanes
                // Java: // are wrong), or it is parallel to a previous hyperplane
                // Java: SubHyperplane<S> s = hyperplane.wholeHyperplane();
                // Java: for (BSPTree<S> tree = node; tree.getParent() != null && s != null; tree = tree.getParent())
                {
                    // Java: final Hyperplane<S>         other = tree.getParent().getCut().getHyperplane();
                    // Java: final SplitSubHyperplane<S> split = s.split(other);
                    // Java: switch (split.getSide())
                    {
                        // Java: case HYPER :
                            // Java: // the hyperplane is parallel to a previous hyperplane
                            // Java: if (!hyperplane.sameOrientationAs(other))
                            {
                                // Java: // this hyperplane is opposite to the other one,
                                // Java: // the region is thinner than the tolerance, we consider it empty
                                // Java: return getComplement(hyperplanes[0].wholeSpace());
                            }
                            // Java: // the hyperplane is an extension of an already known hyperplane, we just ignore it
                            // Java: break;
                        // Java: case PLUS :
                        // Java: // the hyperplane is outside of the current convex zone,
                        // Java: // the input hyperplanes are inconsistent
                        // Java: throw new MathIllegalArgumentException(LocalizedFormats.NOT_CONVEX_HYPERPLANES);
                        // Java: default :
                            // Java: s = split.getMinus();
                    }
                }
            }
        }
        // Java: return region;
        todo!()
    }

    // Java: Region<S> union(final Region<S> region1, final Region<S> region2)
    pub fn union(&self) -> Box<dyn std::any::Any> {
        // Java: final BSPTree<S> tree =
            // Java: region1.getTree(false).merge(region2.getTree(false), new UnionMerger());
        // Java: tree.visit(nodeCleaner);
        // Java: return region1.buildNew(tree);
        todo!()
    }

    // Java: Region<S> intersection(final Region<S> region1, final Region<S> region2)
    pub fn intersection(&self) -> Box<dyn std::any::Any> {
        // Java: final BSPTree<S> tree =
            // Java: region1.getTree(false).merge(region2.getTree(false), new IntersectionMerger());
        // Java: tree.visit(nodeCleaner);
        // Java: return region1.buildNew(tree);
        todo!()
    }

    // Java: Region<S> xor(final Region<S> region1, final Region<S> region2)
    pub fn xor(&self) -> Box<dyn std::any::Any> {
        // Java: final BSPTree<S> tree =
            // Java: region1.getTree(false).merge(region2.getTree(false), new XorMerger());
        // Java: tree.visit(nodeCleaner);
        // Java: return region1.buildNew(tree);
        todo!()
    }

    // Java: Region<S> difference(final Region<S> region1, final Region<S> region2)
    pub fn difference(&self) -> Box<dyn std::any::Any> {
        // Java: final BSPTree<S> tree =
            // Java: region1.getTree(false).merge(region2.getTree(false), new DifferenceMerger(region1, region2));
        // Java: tree.visit(nodeCleaner);
        // Java: return region1.buildNew(tree);
        todo!()
    }

    // Java: Region<S> getComplement(final Region<S> region)
    pub fn getComplement(&self) -> Box<dyn std::any::Any> {
        // Java: return region.buildNew(recurseComplement(region.getTree(false)));
        todo!()
    }

    // Java: BSPTree<S> recurseComplement(final BSPTree<S> node)
    pub fn recurseComplement(&self) -> Box<dyn std::any::Any> {
        // Java: // transform the tree, except for boundary attribute splitters
        // Java: final Map<BSPTree<S>, BSPTree<S>> map = new HashMap<BSPTree<S>, BSPTree<S>>();
        // Java: final BSPTree<S> transformedTree = recurseComplement(node, map);
        // Java: // set up the boundary attributes splitters
        // Java: for (final Map.Entry<BSPTree<S>, BSPTree<S>> entry : map.entrySet())
        {
            // Java: if (entry.getKey().getCut() != null)
            {
                // Java: @SuppressWarnings("unchecked")
                // Java: BoundaryAttribute<S> original = (BoundaryAttribute<S>) entry.getKey().getAttribute();
                // Java: if (original != null)
                {
                    // Java: @SuppressWarnings("unchecked")
                    // Java: BoundaryAttribute<S> transformed = (BoundaryAttribute<S>) entry.getValue().getAttribute();
                    // Java: for (final BSPTree<S> splitter : original.getSplitters())
                    {
                        // Java: transformed.getSplitters().add(map.get(splitter));
                    }
                }
            }
        }
        // Java: return transformedTree;
        todo!()
    }

    // Java: BSPTree<S> recurseComplement()
    pub fn recurseComplement_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

}

// Java: public class UnionMerger
impl UnionMerger {
    // Java: BSPTree<S> merge()
    pub fn merge(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

}

// Java: public class IntersectionMerger
impl IntersectionMerger {
    // Java: BSPTree<S> merge()
    pub fn merge_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

}

// Java: public class XorMerger
impl XorMerger {
    // Java: BSPTree<S> merge()
    pub fn merge_3(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

}

// Java: public class DifferenceMerger
impl DifferenceMerger {
    // Java: BSPTree<S> merge()
    pub fn merge_4(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: BSPTree<S> fixNode(final BSPTree<S> node)
    pub fn fixNode(&self) -> Box<dyn std::any::Any> {
            // Java: // get a representative point in the degenerate cell
            // Java: final BSPTree<S> cell = node.pruneAroundConvexCell(Boolean.TRUE, Boolean.FALSE, null);
            // Java: final Region<S> r = region1.buildNew(cell);
            // Java: final Point<S> p = r.getBarycenter();
            // Java: return new BSPTree<S>(region1.checkPoint(p) == Location.INSIDE &&
                                  // Java: region2.checkPoint(p) == Location.OUTSIDE);
        todo!()
    }

}

// Java: public class NodesCleaner
impl NodesCleaner {
    // Java: Order visitOrder(final BSPTree<S> node)
    pub fn visitOrder(&self) -> Box<dyn std::any::Any> {
            // Java: return Order.PLUS_SUB_MINUS;
        todo!()
    }

    // Java: void visitInternalNode(final BSPTree<S> node)
    pub fn visitInternalNode(&self) {
            // Java: node.setAttribute(null);
    }

    // Java: void visitLeafNode(final BSPTree<S> node)
    pub fn visitLeafNode(&self) {
    }

}

// Java: public class VanishingToLeaf
impl VanishingToLeaf {
    // Java: BSPTree<S> fixNode(final BSPTree<S> node)
    pub fn fixNode_2(&self) -> Box<dyn std::any::Any> {
            // Java: if (node.getPlus().getAttribute().equals(node.getMinus().getAttribute()))
            {
                // Java: // no ambiguity
                // Java: return new BSPTree<S>(node.getPlus().getAttribute());
            }
            // Java: else
            {
                // Java: // ambiguous node
                // Java: return new BSPTree<S>(inside);
            }
        todo!()
    }

}
