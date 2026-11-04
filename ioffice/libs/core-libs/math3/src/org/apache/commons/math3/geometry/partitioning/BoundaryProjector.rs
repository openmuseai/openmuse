// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;
// Java: import org.apache.commons.math3.geometry.partitioning.Region.Location
use crate::org::apache::commons::math3::geometry::partitioning::Region::Location;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class BoundaryProjector extends Space implements BSPTreeVisitor
pub struct BoundaryProjector {
    // Java field: Point<S> original
    original: Box<dyn std::any::Any>,
    // Java field: Point<S> projected
    projected: Option<Box<dyn std::any::Any>>,
    // Java field: BSPTree<S> leaf
    leaf: Option<Box<dyn std::any::Any>>,
    // Java field: double offset
    offset: f64,
}

// Java: public class BoundaryProjector extends Space implements BSPTreeVisitor
impl BoundaryProjector {
    // Java: Order visitOrder(final BSPTree<S> node)
    pub fn visitOrder(&self) -> Box<dyn std::any::Any> {
        // Java: // we want to visit the tree so that the first encountered
        // Java: // leaf is the one closest to the test point
        // Java: if (node.getCut().getHyperplane().getOffset(original) <= 0)
        {
            // Java: return Order.MINUS_SUB_PLUS;
        }
        // Java: else
        {
            // Java: return Order.PLUS_SUB_MINUS;
        }
        todo!()
    }

    // Java: void visitInternalNode(final BSPTree<S> node)
    pub fn visitInternalNode(&self) {
        // Java: // project the point on the cut sub-hyperplane
        // Java: final Hyperplane<S> hyperplane = node.getCut().getHyperplane();
        // Java: final double signedOffset = hyperplane.getOffset(original);
        // Java: if (FastMath.abs(signedOffset) < offset)
        {
            // Java: // project point
            // Java: final Point<S> regular = hyperplane.project(original);
            // Java: // get boundary parts
            // Java: final List<Region<T>> boundaryParts = boundaryRegions(node);
            // Java: // check if regular projection really belongs to the boundary
            // Java: boolean regularFound = false;
            // Java: for (final Region<T> part : boundaryParts)
            {
                // Java: if (!regularFound && belongsToPart(regular, hyperplane, part))
                {
                    // Java: // the projected point lies in the boundary
                    // Java: projected    = regular;
                    // Java: offset       = FastMath.abs(signedOffset);
                    // Java: regularFound = true;
                }
            }
            // Java: if (!regularFound)
            {
                // Java: // the regular projected point is not on boundary,
                // Java: // so we have to check further if a singular point
                // Java: // (i.e. a vertex in 2D case) is a possible projection
                // Java: for (final Region<T> part : boundaryParts)
                {
                    // Java: final Point<S> spI = singularProjection(regular, hyperplane, part);
                    // Java: if (spI != null)
                    {
                        // Java: final double distance = original.distance(spI);
                        // Java: if (distance < offset)
                        {
                            // Java: projected = spI;
                            // Java: offset    = distance;
                        }
                    }
                }
            }
        }
    }

    // Java: void visitLeafNode(final BSPTree<S> node)
    pub fn visitLeafNode(&self) {
        // Java: if (leaf == null)
        {
            // Java: // this is the first leaf we visit,
            // Java: // it is the closest one to the original point
            // Java: leaf = node;
        }
    }

    // Java: BoundaryProjection<S> getProjection()
    pub fn getProjection(&self) -> Box<dyn std::any::Any> {
        // Java: // fix offset sign
        // Java: offset = FastMath.copySign(offset, (Boolean) leaf.getAttribute() ? -1 : +1);
        // Java: return new BoundaryProjection<S>(original, projected, offset);
        todo!()
    }

    // Java: List<Region<T>> boundaryRegions(final BSPTree<S> node)
    pub fn boundaryRegions(&self) -> Box<dyn std::any::Any> {
        // Java: final List<Region<T>> regions = new ArrayList<Region<T>>(2);
        // Java: @SuppressWarnings("unchecked")
        // Java: final BoundaryAttribute<S> ba = (BoundaryAttribute<S>) node.getAttribute();
        // Java: addRegion(ba.getPlusInside(),  regions);
        // Java: addRegion(ba.getPlusOutside(), regions);
        // Java: return regions;
        todo!()
    }

    // Java: void addRegion(final SubHyperplane<S> sub, final List<Region<T>> list)
    pub fn addRegion(&self) {
        // Java: if (sub != null)
        {
            // Java: @SuppressWarnings("unchecked")
            // Java: final Region<T> region = ((AbstractSubHyperplane<S, T>) sub).getRemainingRegion();
            // Java: if (region != null)
            {
                // Java: list.add(region);
            }
        }
    }

    // Java: boolean belongsToPart()
    pub fn belongsToPart(&self) -> bool {
        todo!()
    }

    // Java: Point<S> singularProjection()
    pub fn singularProjection(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

}

