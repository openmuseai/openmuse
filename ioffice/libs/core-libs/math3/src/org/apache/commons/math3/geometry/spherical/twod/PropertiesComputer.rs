#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.MathInternalError
use crate::org::apache::commons::math3::exception::MathInternalError;
// Java: import org.apache.commons.math3.geometry.euclidean.threed.Vector3D
use crate::org::apache::commons::math3::geometry::euclidean::threed::Vector3D;
// Java: import org.apache.commons.math3.geometry.partitioning.BSPTree
use crate::org::apache::commons::math3::geometry::partitioning::BSPTree;
// Java: import org.apache.commons.math3.geometry.partitioning.BSPTreeVisitor
use crate::org::apache::commons::math3::geometry::partitioning::BSPTreeVisitor;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java: public class PropertiesComputer implements BSPTreeVisitor
pub struct PropertiesComputer {
    // Java field: double tolerance
    pub(crate) tolerance: f64,
    // Java field: double summedArea
    pub(crate) summedArea: f64,
    // Java field: Vector3D summedBarycenter
    pub(crate) summedBarycenter: Box<dyn std::any::Any>,
    // Java field: List<Vector3D> convexCellsInsidePoints
    pub(crate) convexCellsInsidePoints: Vec<Box<dyn std::any::Any>>,
}

// Java: public class PropertiesComputer implements BSPTreeVisitor
impl PropertiesComputer {
    // Java: Order visitOrder(final BSPTree<Sphere2D> node)
    pub fn visitOrder(&self, _node: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return Order.MINUS_SUB_PLUS;
        todo!()
    }

    // Java: void visitInternalNode(final BSPTree<Sphere2D> node)
    pub fn visitInternalNode(&self, _node: &dyn std::any::Any) {
        // Java: // nothing to do here
    }

    // Java: void visitLeafNode(final BSPTree<Sphere2D> node)
    pub fn visitLeafNode(&mut self, _node: &dyn std::any::Any) {
        // Java: if ((Boolean) node.getAttribute())
        // Java: // transform this inside leaf cell into a simple convex polygon
        // Java: final SphericalPolygonsSet convex =
        // Java: new SphericalPolygonsSet(node.pruneAroundConvexCell(Boolean.TRUE,
        // Java: Boolean.FALSE,
        // Java: null),
        // Java: tolerance);
        // Java: // extract the start of the single loop boundary of the convex cell
        // Java: final List<Vertex> boundary = convex.getBoundaryLoops();
        // Java: if (boundary.size() != 1)
        // Java: // this should never happen
        // Java: throw new MathInternalError();
        // Java: // compute the geometrical properties of the convex cell
        // Java: final double area  = convexCellArea(boundary.get(0));
        // Java: final Vector3D barycenter = convexCellBarycenter(boundary.get(0));
        // Java: convexCellsInsidePoints.add(barycenter);
        // Java: // add the cell contribution to the global properties
        // Java: summedArea      += area;
        // Java: summedBarycenter = new Vector3D(1, summedBarycenter, area, barycenter);
        todo!()
    }

    // Java: double convexCellArea(final Vertex start)
    pub fn convexCellArea(&self, _start: &dyn std::any::Any) -> f64 {
        // Java: int n = 0;
        // Java: double sum = 0;
        // Java: // loop around the cell
        // Java: for (Edge e = start.getOutgoing(); n == 0 || e.getStart() != start; e = e.getEnd().getOutgoing())
        // Java: // find path interior angle at vertex
        // Java: final Vector3D previousPole = e.getCircle().getPole();
        // Java: final Vector3D nextPole     = e.getEnd().getOutgoing().getCircle().getPole();
        // Java: final Vector3D point        = e.getEnd().getLocation().getVector();
        // Java: double alpha = FastMath.atan2(Vector3D.dotProduct(nextPole, Vector3D.crossProduct(point, previousPole)),
        // Java: -Vector3D.dotProduct(nextPole, previousPole));
        // Java: if (alpha < 0)
        // Java: alpha += MathUtils.TWO_PI;
        // Java: sum += alpha;
        // Java: n++;
        // Java: // compute area using extended Girard theorem
        // Java: // see Spherical Trigonometry: For the Use of Colleges and Schools by I. Todhunter
        // Java: // article 99 in chapter VIII Area Of a Spherical Triangle. Spherical Excess.
        // Java: // book available from project Gutenberg at http://www.gutenberg.org/ebooks/19770
        // Java: return sum - (n - 2) * FastMath.PI;
        todo!()
    }

    // Java: Vector3D convexCellBarycenter(final Vertex start)
    pub fn convexCellBarycenter(&self, _start: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: int n = 0;
        // Java: Vector3D sumB = Vector3D.ZERO;
        // Java: // loop around the cell
        // Java: for (Edge e = start.getOutgoing(); n == 0 || e.getStart() != start; e = e.getEnd().getOutgoing())
        // Java: sumB = new Vector3D(1, sumB, e.getLength(), e.getCircle().getPole());
        // Java: n++;
        // Java: return sumB.normalize();
        todo!()
    }

    // Java: double getArea()
    pub fn getArea(&self) -> f64 {
        // Java: return summedArea;
        self.summedArea
    }

    // Java: S2Point getBarycenter()
    pub fn getBarycenter(&self) -> Box<dyn std::any::Any> {
        // Java: if (summedBarycenter.getNormSq() == 0)
        // Java: return S2Point.NaN;
        // Java: else
        // Java: return new S2Point(summedBarycenter);
        todo!()
    }

    // Java: List<Vector3D> getConvexCellsInsidePoints()
    pub fn getConvexCellsInsidePoints(&self) -> &Vec<Box<dyn std::any::Any>> {
        // Java: return convexCellsInsidePoints;
        &self.convexCellsInsidePoints
    }

}
