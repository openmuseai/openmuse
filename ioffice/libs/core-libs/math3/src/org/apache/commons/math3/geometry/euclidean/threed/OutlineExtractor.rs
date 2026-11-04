#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.ArrayList
// (no direct Rust equivalent)
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Euclidean2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Euclidean2D;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.PolygonsSet
use crate::org::apache::commons::math3::geometry::euclidean::twod::PolygonsSet;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Vector2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Vector2D;
// Java: import org.apache.commons.math3.geometry.partitioning.AbstractSubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::AbstractSubHyperplane;
// Java: import org.apache.commons.math3.geometry.partitioning.BSPTree
use crate::org::apache::commons::math3::geometry::partitioning::BSPTree;
// Java: import org.apache.commons.math3.geometry.partitioning.BSPTreeVisitor
use crate::org::apache::commons::math3::geometry::partitioning::BSPTreeVisitor;
// Java: import org.apache.commons.math3.geometry.partitioning.BoundaryAttribute
use crate::org::apache::commons::math3::geometry::partitioning::BoundaryAttribute;
// Java: import org.apache.commons.math3.geometry.partitioning.RegionFactory
use crate::org::apache::commons::math3::geometry::partitioning::RegionFactory;
// Java: import org.apache.commons.math3.geometry.partitioning.SubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::SubHyperplane;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;

// Java: public class extracts
pub struct extracts {
}

// Java: public class OutlineExtractor
pub struct OutlineExtractor {
    // Java field: Vector3D u
    pub(crate) u: Box<dyn std::any::Any>,
    // Java field: Vector3D v
    pub(crate) v: Box<dyn std::any::Any>,
    // Java field: Vector3D w
    pub(crate) w: Box<dyn std::any::Any>,
}

// Java: public class BoundaryProjector
pub struct BoundaryProjector {
    // Java field: PolygonsSet projected
    pub(crate) projected: Option<Box<dyn std::any::Any>>,
    // Java field: double tolerance
    pub(crate) tolerance: f64,
}

// Java: public class extracts
impl extracts {
}

// Java: public class OutlineExtractor
impl OutlineExtractor {
    // Java: OutlineExtractor(final Vector3D u, final Vector3D v)
    pub fn new(u: Box<dyn std::any::Any>, v: Box<dyn std::any::Any>) -> Self {
        // Java: this.u = u;
        // Java: this.v = v;
        // Java: w = Vector3D.crossProduct(u, v);
        todo!()
    }

    // Java: Vector2D[][] getOutline(final PolyhedronsSet polyhedronsSet)
    pub fn getOutline(&self, _polyhedronsSet: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: // project all boundary facets into one polygons set
        // Java: final BoundaryProjector projector = new BoundaryProjector(polyhedronsSet.getTolerance());
        // Java: polyhedronsSet.getTree(true).visit(projector);
        // Java: final PolygonsSet projected = projector.getProjected();
        // Java: // Remove the spurious intermediate vertices from the outline
        // Java: final Vector2D[][] outline = projected.getVertices();
        // Java: for (int i = 0; i < outline.length; ++i)
        // Java: final Vector2D[] rawLoop = outline[i];
        // Java: int end = rawLoop.length;
        // Java: int j = 0;
        // Java: while (j < end)
        // Java: if (pointIsBetween(rawLoop, end, j))
        // Java: // the point should be removed
        // Java: for (int k = j; k < (end - 1); ++k)
        // Java: rawLoop[k] = rawLoop[k + 1];
        // Java: --end;
        // Java: else
        // Java: // the point remains in the loop
        // Java: ++j;
        // Java: if (end != rawLoop.length)
        // Java: // resize the array
        // Java: outline[i] = new Vector2D[end];
        // Java: System.arraycopy(rawLoop, 0, outline[i], 0, end);
        // Java: return outline;
        todo!()
    }

    // Java: boolean pointIsBetween(final Vector2D[] loop, final int n, final int i)
    pub fn pointIsBetween(&self, _loop_arr: &[Box<dyn std::any::Any>], _n: usize, _i: usize) -> bool {
        // Java: final Vector2D previous = loop[(i + n - 1) % n];
        // Java: final Vector2D current  = loop[i];
        // Java: final Vector2D next     = loop[(i + 1) % n];
        // Java: final double dx1       = current.getX() - previous.getX();
        // Java: final double dy1       = current.getY() - previous.getY();
        // Java: final double dx2       = next.getX()    - current.getX();
        // Java: final double dy2       = next.getY()    - current.getY();
        // Java: final double cross     = dx1 * dy2 - dx2 * dy1;
        // Java: final double dot       = dx1 * dx2 + dy1 * dy2;
        // Java: final double d1d2      = FastMath.sqrt((dx1 * dx1 + dy1 * dy1) * (dx2 * dx2 + dy2 * dy2));
        // Java: return (FastMath.abs(cross) <= (1.0e-6 * d1d2)) && (dot >= 0.0);
        todo!()
    }

}

// Java: public class BoundaryProjector
impl BoundaryProjector {
    // Java: Order visitOrder(final BSPTree<Euclidean3D> node)
    pub fn visitOrder(&self, _node: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return Order.MINUS_SUB_PLUS;
        todo!()
    }

    // Java: void visitInternalNode(final BSPTree<Euclidean3D> node)
    pub fn visitInternalNode(&mut self, _node: &dyn std::any::Any) {
        // Java: @SuppressWarnings("unchecked")
        // Java: final BoundaryAttribute<Euclidean3D> attribute =
        // Java: (BoundaryAttribute<Euclidean3D>) node.getAttribute();
        // Java: if (attribute.getPlusOutside() != null)
        // Java: addContribution(attribute.getPlusOutside(), false);
        // Java: if (attribute.getPlusInside() != null)
        // Java: addContribution(attribute.getPlusInside(), true);
        todo!()
    }

    // Java: void visitLeafNode(final BSPTree<Euclidean3D> node)
    pub fn visitLeafNode(&self, _node: &dyn std::any::Any) {
        // (empty body in Java)
    }

    // Java: void addContribution(final SubHyperplane<Euclidean3D> facet, final boolean reversed)
    pub fn addContribution(&mut self, _facet: &dyn std::any::Any, _reversed: bool) {
        // Java: // extract the vertices of the facet
        // Java: @SuppressWarnings("unchecked")
        // Java: final AbstractSubHyperplane<Euclidean3D, Euclidean2D> absFacet =
        // Java: (AbstractSubHyperplane<Euclidean3D, Euclidean2D>) facet;
        // Java: final Plane plane    = (Plane) facet.getHyperplane();
        // Java: final double scal = plane.getNormal().dotProduct(w);
        // Java: if (FastMath.abs(scal) > 1.0e-3)
        // Java: Vector2D[][] vertices =
        // Java: ((PolygonsSet) absFacet.getRemainingRegion()).getVertices();
        // Java: if ((scal < 0) ^ reversed)
        // Java: // the facet is seen from the inside,
        // Java: // we need to invert its boundary orientation
        // Java: final Vector2D[][] newVertices = new Vector2D[vertices.length][];
        // Java: for (int i = 0; i < vertices.length; ++i)
        // Java: final Vector2D[] loop = vertices[i];
        // Java: final Vector2D[] newLoop = new Vector2D[loop.length];
        // Java: if (loop[0] == null)
        // Java: newLoop[0] = null;
        // Java: for (int j = 1; j < loop.length; ++j)
        // Java: newLoop[j] = loop[loop.length - j];
        // Java: else
        // Java: for (int j = 0; j < loop.length; ++j)
        // Java: newLoop[j] = loop[loop.length - (j + 1)];
        // Java: newVertices[i] = newLoop;
        // Java: // use the reverted vertices
        // Java: vertices = newVertices;
        // Java: // compute the projection of the facet in the outline plane
        // Java: final ArrayList<SubHyperplane<Euclidean2D>> edges = new ArrayList<SubHyperplane<Euclidean2D>>();
        // Java: for (Vector2D[] loop : vertices)
        // Java: final boolean closed = loop[0] != null;
        // Java: int previous         = closed ? (loop.length - 1) : 1;
        // Java: Vector3D previous3D  = plane.toSpace((Point<Euclidean2D>) loop[previous]);
        // Java: int current          = (previous + 1) % loop.length;
        // Java: Vector2D pPoint       = new Vector2D(previous3D.dotProduct(u),
        // Java: previous3D.dotProduct(v));
        // Java: while (current < loop.length)
        // Java: final Vector3D current3D = plane.toSpace((Point<Euclidean2D>) loop[current]);
        // Java: final Vector2D  cPoint    = new Vector2D(current3D.dotProduct(u),
        // Java: current3D.dotProduct(v));
        // Java: final org.apache.commons.math3.geometry.euclidean.twod.Line line =
        // Java: new org.apache.commons.math3.geometry.euclidean.twod.Line(pPoint, cPoint, tolerance);
        // Java: SubHyperplane<Euclidean2D> edge = line.wholeHyperplane();
        // Java: if (closed || (previous != 1))
        // Java: // the previous point is a real vertex
        // Java: // it defines one bounding point of the edge
        // Java: final double angle = line.getAngle() + 0.5 * FastMath.PI;
        // Java: final org.apache.commons.math3.geometry.euclidean.twod.Line l =
        // Java: new org.apache.commons.math3.geometry.euclidean.twod.Line(pPoint, angle, tolerance);
        // Java: edge = edge.split(l).getPlus();
        // Java: if (closed || (current != (loop.length - 1)))
        // Java: // the current point is a real vertex
        // Java: // it defines one bounding point of the edge
        // Java: final double angle = line.getAngle() + 0.5 * FastMath.PI;
        // Java: final org.apache.commons.math3.geometry.euclidean.twod.Line l =
        // Java: new org.apache.commons.math3.geometry.euclidean.twod.Line(cPoint, angle, tolerance);
        // Java: edge = edge.split(l).getMinus();
        // Java: edges.add(edge);
        // Java: previous   = current++;
        // Java: previous3D = current3D;
        // Java: pPoint     = cPoint;
        // Java: final PolygonsSet projectedFacet = new PolygonsSet(edges, tolerance);
        // Java: // add the contribution of the facet to the global outline
        // Java: projected = (PolygonsSet) new RegionFactory<Euclidean2D>().union(projected, projectedFacet);
        todo!()
    }

    // Java: PolygonsSet getProjected()
    pub fn getProjected(&self) -> &Option<Box<dyn std::any::Any>> {
        // Java: return projected;
        &self.projected
    }

}
