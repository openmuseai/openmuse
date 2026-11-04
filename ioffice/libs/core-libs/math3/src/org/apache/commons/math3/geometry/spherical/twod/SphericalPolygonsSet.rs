#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.Collections
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.geometry.enclosing.EnclosingBall
use crate::org::apache::commons::math3::geometry::enclosing::EnclosingBall;
// Java: import org.apache.commons.math3.geometry.enclosing.WelzlEncloser
use crate::org::apache::commons::math3::geometry::enclosing::WelzlEncloser;
// Java: import org.apache.commons.math3.geometry.euclidean.threed.Euclidean3D
use crate::org::apache::commons::math3::geometry::euclidean::threed::Euclidean3D;
// Java: import org.apache.commons.math3.geometry.euclidean.threed.Rotation
use crate::org::apache::commons::math3::geometry::euclidean::threed::Rotation;
// Java: import org.apache.commons.math3.geometry.euclidean.threed.RotationConvention
use crate::org::apache::commons::math3::geometry::euclidean::threed::RotationConvention;
// Java: import org.apache.commons.math3.geometry.euclidean.threed.SphereGenerator
// use crate::org::apache::commons::math3::geometry::euclidean::threed::SphereGenerator;
// Java: import org.apache.commons.math3.geometry.euclidean.threed.Vector3D
use crate::org::apache::commons::math3::geometry::euclidean::threed::Vector3D;
// Java: import org.apache.commons.math3.geometry.partitioning.AbstractRegion
use crate::org::apache::commons::math3::geometry::partitioning::AbstractRegion;
// Java: import org.apache.commons.math3.geometry.partitioning.BSPTree
use crate::org::apache::commons::math3::geometry::partitioning::BSPTree;
// Java: import org.apache.commons.math3.geometry.partitioning.BoundaryProjection
use crate::org::apache::commons::math3::geometry::partitioning::BoundaryProjection;
// Java: import org.apache.commons.math3.geometry.partitioning.RegionFactory
use crate::org::apache::commons::math3::geometry::partitioning::RegionFactory;
// Java: import org.apache.commons.math3.geometry.partitioning.SubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::SubHyperplane;
// Java: import org.apache.commons.math3.geometry.spherical.oned.Sphere1D
use crate::org::apache::commons::math3::geometry::spherical::oned::Sphere1D;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java: public class represents
pub struct represents {
}

// Java: public class SphericalPolygonsSet
pub struct SphericalPolygonsSet {
    // Java field: List<Vertex> loops
    pub(crate) loops: Option<Vec<Box<dyn std::any::Any>>>,
}

// Java: public class represents
impl represents {
}

// Java: public class SphericalPolygonsSet
impl SphericalPolygonsSet {
    // Java: SphericalPolygonsSet(final double tolerance)
    pub fn new(tolerance: f64) -> Self {
        // Java: super(tolerance);
        SphericalPolygonsSet {
            loops: None,
        }
    }

    // Java: SphericalPolygonsSet(final Vector3D pole, final double tolerance)
    pub fn new_with_pole(_pole: &dyn std::any::Any, tolerance: f64) -> Self {
        // Java: super(new BSPTree<Sphere2D>(new Circle(pole, tolerance).wholeHyperplane(),
        // Java: new BSPTree<Sphere2D>(Boolean.FALSE),
        // Java: new BSPTree<Sphere2D>(Boolean.TRUE),
        // Java: null),
        // Java: tolerance);
        SphericalPolygonsSet {
            loops: None,
        }
    }

    // Java: SphericalPolygonsSet()
    pub fn new_default() -> Self {
        SphericalPolygonsSet {
            loops: None,
        }
    }

    // Java: SphericalPolygonsSet(final BSPTree<Sphere2D> tree, final double tolerance)
    pub fn new_with_tree(_tree: &dyn std::any::Any, _tolerance: f64) -> Self {
        // Java: super(tree, tolerance);
        SphericalPolygonsSet {
            loops: None,
        }
    }

    // Java: SphericalPolygonsSet(final Collection<SubHyperplane<Sphere2D>> boundary, final double tolerance)
    pub fn new_with_boundary(_boundary: &Vec<Box<dyn std::any::Any>>, _tolerance: f64) -> Self {
        // Java: super(boundary, tolerance);
        SphericalPolygonsSet {
            loops: None,
        }
    }

    // Java: SphericalPolygonsSet(final double hyperplaneThickness, final S2Point ... vertices)
    pub fn new_with_vertices(_hyperplaneThickness: f64, _vertices: &[Box<dyn std::any::Any>]) -> Self {
        // Java: super(verticesToTree(hyperplaneThickness, vertices), hyperplaneThickness);
        SphericalPolygonsSet {
            loops: None,
        }
    }

    // Java: static S2Point[] createRegularPolygonVertices()
    pub fn createRegularPolygonVertices() -> Vec<Box<dyn std::any::Any>> {
        todo!()
    }

    // Java: static BSPTree<Sphere2D> verticesToTree()
    pub fn verticesToTree() -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static void insertEdges()
    pub fn insertEdges() {
        todo!()
    }

    // Java: SphericalPolygonsSet buildNew(final BSPTree<Sphere2D> tree)
    pub fn buildNew(&self, _tree: &dyn std::any::Any) -> SphericalPolygonsSet {
        // Java: return new SphericalPolygonsSet(tree, getTolerance());
        todo!()
    }

    // Java: void computeGeometricalProperties()
    pub fn computeGeometricalProperties(&mut self) {
        // Java: final BSPTree<Sphere2D> tree = getTree(true);
        // Java: if (tree.getCut() == null)
        // Java: // the instance has a single cell without any boundaries
        // Java: if (tree.getCut() == null && (Boolean) tree.getAttribute())
        // Java: // the instance covers the whole space
        // Java: setSize(4 * FastMath.PI);
        // Java: setBarycenter(new S2Point(0, 0));
        // Java: else
        // Java: setSize(0);
        // Java: setBarycenter(S2Point.NaN);
        // Java: else
        // Java: // the instance has a boundary
        // Java: final PropertiesComputer pc = new PropertiesComputer(getTolerance());
        // Java: tree.visit(pc);
        // Java: setSize(pc.getArea());
        // Java: setBarycenter(pc.getBarycenter());
        todo!()
    }

    // Java: List<Vertex> getBoundaryLoops()
    pub fn getBoundaryLoops(&mut self) -> &Vec<Box<dyn std::any::Any>> {
        // Java: if (loops == null)
        // Java: if (getTree(false).getCut() == null)
        // Java: loops = Collections.emptyList();
        // Java: else
        // Java: // sort the arcs according to their start point
        // Java: final BSPTree<Sphere2D> root = getTree(true);
        // Java: final EdgesBuilder visitor = new EdgesBuilder(root, getTolerance());
        // Java: root.visit(visitor);
        // Java: final List<Edge> edges = visitor.getEdges();
        // Java: // convert the list of all edges into a list of start vertices
        // Java: loops = new ArrayList<Vertex>();
        // Java: while (!edges.isEmpty())
        // Java: // this is an edge belonging to a new loop, store it
        // Java: Edge edge = edges.get(0);
        // Java: final Vertex startVertex = edge.getStart();
        // Java: loops.add(startVertex);
        // Java: // remove all remaining edges in the same loop
        // Java: do
        // Java: // remove one edge
        // Java: for (final Iterator<Edge> iterator = edges.iterator(); iterator.hasNext();)
        // Java: if (iterator.next() == edge)
        // Java: iterator.remove();
        // Java: break;
        // Java: // go to next edge following the boundary loop
        // Java: edge = edge.getEnd().getOutgoing();
        // Java: return Collections.unmodifiableList(loops);
        todo!()
    }

    // Java: public EnclosingBall<Sphere2D, S2Point> getEnclosingCap()
    pub fn getEnclosingCap(&self) -> Box<dyn std::any::Any> {
        // Java: // handle special cases first
        // Java: if (isEmpty())
        // Java: return new EnclosingBall<Sphere2D, S2Point>(S2Point.PLUS_K, Double.NEGATIVE_INFINITY);
        // Java: if (isFull())
        // Java: return new EnclosingBall<Sphere2D, S2Point>(S2Point.PLUS_K, Double.POSITIVE_INFINITY);
        // Java: // as the polygons is neither empty nor full, it has some boundaries and cut hyperplanes
        // Java: final BSPTree<Sphere2D> root = getTree(false);
        // Java: if (isEmpty(root.getMinus()) && isFull(root.getPlus()))
        // Java: // the polygon covers an hemisphere, and its boundary is one 2π long edge
        // Java: final Circle circle = (Circle) root.getCut().getHyperplane();
        // Java: return new EnclosingBall<Sphere2D, S2Point>(new S2Point(circle.getPole()).negate(),
        // Java: 0.5 * FastMath.PI);
        // Java: if (isFull(root.getMinus()) && isEmpty(root.getPlus()))
        // Java: // the polygon covers an hemisphere, and its boundary is one 2π long edge
        // Java: final Circle circle = (Circle) root.getCut().getHyperplane();
        // Java: return new EnclosingBall<Sphere2D, S2Point>(new S2Point(circle.getPole()),
        // Java: 0.5 * FastMath.PI);
        // Java: // gather some inside points, to be used by the encloser
        // Java: final List<Vector3D> points = getInsidePoints();
        // Java: // extract points from the boundary loops, to be used by the encloser as well
        // Java: final List<Vertex> boundary = getBoundaryLoops();
        // Java: for (final Vertex loopStart : boundary)
        // Java: int count = 0;
        // Java: for (Vertex v = loopStart; count == 0 || v != loopStart; v = v.getOutgoing().getEnd())
        // Java: ++count;
        // Java: points.add(v.getLocation().getVector());
        // Java: // find the smallest enclosing 3D sphere
        // Java: final SphereGenerator generator = new SphereGenerator();
        // Java: final WelzlEncloser<Euclidean3D, Vector3D> encloser =
        // Java: new WelzlEncloser<Euclidean3D, Vector3D>(getTolerance(), generator);
        // Java: EnclosingBall<Euclidean3D, Vector3D> enclosing3D = encloser.enclose(points);
        // Java: final Vector3D[] support3D = enclosing3D.getSupport();
        // Java: // convert to 3D sphere to spherical cap
        // Java: final double r = enclosing3D.getRadius();
        // Java: final double h = enclosing3D.getCenter().getNorm();
        // Java: if (h < getTolerance())
        // Java: // the 3D sphere is centered on the unit sphere and covers it
        // Java: // fall back to a crude approximation, based only on outside convex cells
        // Java: EnclosingBall<Sphere2D, S2Point> enclosingS2 =
        // Java: new EnclosingBall<Sphere2D, S2Point>(S2Point.PLUS_K, Double.POSITIVE_INFINITY);
        // Java: for (Vector3D outsidePoint : getOutsidePoints())
        // Java: final S2Point outsideS2 = new S2Point(outsidePoint);
        // Java: final BoundaryProjection<Sphere2D> projection = projectToBoundary(outsideS2);
        // Java: if (FastMath.PI - projection.getOffset() < enclosingS2.getRadius())
        // Java: enclosingS2 = new EnclosingBall<Sphere2D, S2Point>(outsideS2.negate(),
        // Java: FastMath.PI - projection.getOffset(),
        // Java: (S2Point) projection.getProjected());
        // Java: return enclosingS2;
        // Java: final S2Point[] support = new S2Point[support3D.length];
        // Java: for (int i = 0; i < support3D.length; ++i)
        // Java: support[i] = new S2Point(support3D[i]);
        // Java: final EnclosingBall<Sphere2D, S2Point> enclosingS2 =
        // Java: new EnclosingBall<Sphere2D, S2Point>(new S2Point(enclosing3D.getCenter()),
        // Java: FastMath.acos((1 + h * h - r * r) / (2 * h)),
        // Java: support);
        // Java: return enclosingS2;
        todo!()
    }

    // Java: List<Vector3D> getInsidePoints()
    pub fn getInsidePoints(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: final PropertiesComputer pc = new PropertiesComputer(getTolerance());
        // Java: getTree(true).visit(pc);
        // Java: return pc.getConvexCellsInsidePoints();
        todo!()
    }

    // Java: List<Vector3D> getOutsidePoints()
    pub fn getOutsidePoints(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: final SphericalPolygonsSet complement =
        // Java: (SphericalPolygonsSet) new RegionFactory<Sphere2D>().getComplement(this);
        // Java: final PropertiesComputer pc = new PropertiesComputer(getTolerance());
        // Java: complement.getTree(true).visit(pc);
        // Java: return pc.getConvexCellsInsidePoints();
        todo!()
    }

}
