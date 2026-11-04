// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.Collection
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.Euclidean1D
use crate::org::apache::commons::math3::geometry::euclidean::oned::Euclidean1D;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Euclidean2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Euclidean2D;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.PolygonsSet
use crate::org::apache::commons::math3::geometry::euclidean::twod::PolygonsSet;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.SubLine
use crate::org::apache::commons::math3::geometry::euclidean::twod::SubLine;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Vector2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Vector2D;
// Java: import org.apache.commons.math3.geometry.partitioning.AbstractRegion
use crate::org::apache::commons::math3::geometry::partitioning::AbstractRegion;
// Java: import org.apache.commons.math3.geometry.partitioning.BSPTree
use crate::org::apache::commons::math3::geometry::partitioning::BSPTree;
// Java: import org.apache.commons.math3.geometry.partitioning.BSPTreeVisitor
use crate::org::apache::commons::math3::geometry::partitioning::BSPTreeVisitor;
// Java: import org.apache.commons.math3.geometry.partitioning.BoundaryAttribute
use crate::org::apache::commons::math3::geometry::partitioning::BoundaryAttribute;
// Java: import org.apache.commons.math3.geometry.partitioning.Hyperplane
use crate::org::apache::commons::math3::geometry::partitioning::Hyperplane;
// Java: import org.apache.commons.math3.geometry.partitioning.Region
use crate::org::apache::commons::math3::geometry::partitioning::Region;
// Java: import org.apache.commons.math3.geometry.partitioning.RegionFactory
use crate::org::apache::commons::math3::geometry::partitioning::RegionFactory;
// Java: import org.apache.commons.math3.geometry.partitioning.SubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::SubHyperplane;
// Java: import org.apache.commons.math3.geometry.partitioning.Transform
use crate::org::apache::commons::math3::geometry::partitioning::Transform;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: double DEFAULT_TOLERANCE
pub(crate) const DEFAULT_TOLERANCE: f64 = 1.0e-10;

// Java: public class represents
pub struct represents {
}

// Java: public class PolyhedronsSet
pub struct PolyhedronsSet {
    pub tolerance: f64,
    pub tree: Option<Box<dyn std::any::Any>>,
    pub boundary: Option<Vec<Box<dyn std::any::Any>>>,
}

// Java: public class FacetsContributionVisitor
pub struct FacetsContributionVisitor {
}

// Java: public class RotationTransform
pub struct RotationTransform {
    // Java field: Vector3D center
    pub(crate) center: Box<dyn std::any::Any>,
    // Java field: Rotation rotation
    pub(crate) rotation: Box<dyn std::any::Any>,
    // Java field: Plane cachedOriginal
    cachedOriginal: Option<Box<dyn std::any::Any>>,
    // Java field: Transform<Euclidean2D, Euclidean1D> cachedTransform
    cachedTransform: Option<Box<dyn std::any::Any>>,
}

// Java: public class TranslationTransform
pub struct TranslationTransform {
    // Java field: Vector3D translation
    pub(crate) translation: Box<dyn std::any::Any>,
    // Java field: Plane cachedOriginal
    cachedOriginal: Option<Box<dyn std::any::Any>>,
    // Java field: Transform<Euclidean2D, Euclidean1D> cachedTransform
    cachedTransform: Option<Box<dyn std::any::Any>>,
}

// Java: public class represents
impl represents {
}

// Java: public class PolyhedronsSet
#[allow(non_snake_case)]
impl PolyhedronsSet {
    // Java: PolyhedronsSet(final double tolerance)
    pub fn PolyhedronsSet_with_tolerance(tolerance: f64) -> Self {
        // Java: super(tolerance);
        PolyhedronsSet { tolerance, tree: None, boundary: None }
    }

    // Java: PolyhedronsSet(final BSPTree<Euclidean3D> tree, final double tolerance)
    pub fn PolyhedronsSet_with_tree(tree: Box<dyn std::any::Any>, tolerance: f64) -> Self {
        // Java: super(tree, tolerance);
        PolyhedronsSet { tolerance, tree: Some(tree), boundary: None }
    }

    // Java: PolyhedronsSet()
    pub fn PolyhedronsSet_with_boundary(boundary: Vec<Box<dyn std::any::Any>>, tolerance: f64) -> Self {
        // Java: super(boundary, tolerance);
        PolyhedronsSet { tolerance, tree: None, boundary: Some(boundary) }
    }

    // Java: PolyhedronsSet()
    pub fn PolyhedronsSet_with_bounds(_xMin: f64, _xMax: f64, _yMin: f64, _yMax: f64, _zMin: f64, _zMax: f64, tolerance: f64) -> Self {
        // Java: super(buildBoundary(...));
        PolyhedronsSet { tolerance, tree: None, boundary: None }
    }

    // Java: PolyhedronsSet()
    pub fn PolyhedronsSet_from_vertices(_vertices: Vec<Box<dyn std::any::Any>>, _facets: Vec<Vec<i32>>, tolerance: f64) -> Self {
        // Java: super(buildBoundary(vertices, facets, tolerance), tolerance);
        PolyhedronsSet { tolerance, tree: None, boundary: None }
    }

    // Java: PolyhedronsSet()
    pub fn new() -> Self {
        // Java: this(DEFAULT_TOLERANCE);
        PolyhedronsSet { tolerance: DEFAULT_TOLERANCE, tree: None, boundary: None }
    }

    // Java: PolyhedronsSet(final BSPTree<Euclidean3D> tree)
    pub fn PolyhedronsSet_tree_default(tree: Box<dyn std::any::Any>) -> Self {
        // Java: this(tree, DEFAULT_TOLERANCE);
        PolyhedronsSet { tolerance: DEFAULT_TOLERANCE, tree: Some(tree), boundary: None }
    }

    // Java: PolyhedronsSet(final Collection<SubHyperplane<Euclidean3D>> boundary)
    pub fn PolyhedronsSet_boundary_default(boundary: Vec<Box<dyn std::any::Any>>) -> Self {
        // Java: this(boundary, DEFAULT_TOLERANCE);
        PolyhedronsSet { tolerance: DEFAULT_TOLERANCE, tree: None, boundary: Some(boundary) }
    }

    // Java: PolyhedronsSet()
    pub fn PolyhedronsSet_bounds_default(_xMin: f64, _xMax: f64, _yMin: f64, _yMax: f64, _zMin: f64, _zMax: f64) -> Self {
        // Java: this(xMin, xMax, yMin, yMax, zMin, zMax, DEFAULT_TOLERANCE);
        todo!()
    }

    // Java: static BSPTree<Euclidean3D> buildBoundary()
    pub fn buildBoundary_from_bounds(_xMin: f64, _xMax: f64, _yMin: f64, _yMax: f64, _zMin: f64, _zMax: f64, _tolerance: f64) -> Box<dyn std::any::Any> {
        // Java: ... builds BSPTree from bounds
        todo!()
    }

    // Java: static List<SubHyperplane<Euclidean3D>> buildBoundary()
    pub fn buildBoundary_from_vertices(_vertices: &[Box<dyn std::any::Any>], _facets: &[Vec<i32>], _tolerance: f64) -> Vec<Box<dyn std::any::Any>> {
        // Java: ... builds boundary from vertices and facets
        todo!()
    }

    // Java: static int[][] findReferences(final List<Vector3D> vertices, final List<int[]> facets)
    pub fn findReferences(vertices: &[Box<dyn std::any::Any>], facets: &[Vec<i32>]) -> Vec<Vec<i32>> {
        // Java: // find the maximum number of facets a vertex belongs to
        // Java: final int[] nbFacets = new int[vertices.size()];
        // Java: int maxFacets  = 0;
        // Java: for (final int[] facet : facets)
        {
            // Java: if (facet.length < 3)
            {
                // Java: throw new NumberIsTooSmallException(LocalizedFormats.WRONG_NUMBER_OF_POINTS,
                // Java: 3, facet.length, true);
            }
            // Java: for (final int index : facet)
            {
                // Java: maxFacets = FastMath.max(maxFacets, ++nbFacets[index]);
            }
        }
        // Java: // set up the references array
        // Java: final int[][] references = new int[vertices.size()][maxFacets];
        // Java: for (int[] r : references)
        {
            // Java: Arrays.fill(r, -1);
        }
        // Java: for (int f = 0; f < facets.size(); ++f)
        {
            // Java: for (final int v : facets.get(f))
            {
                // Java: // vertex v is referenced by facet f
                // Java: int k = 0;
                // Java: while (k < maxFacets && references[v][k] >= 0)
                {
                    // Java: ++k;
                }
                // Java: references[v][k] = f;
            }
        }
        // Java: return references;
        todo!()
    }

    // Java: static int[][] successors()
    pub fn successors(_vertices: &[Box<dyn std::any::Any>], _facets: &[Vec<i32>], _references: &[Vec<i32>]) -> Vec<Vec<i32>> {
        // Java: ... computes successors
        todo!()
    }

    // Java: PolyhedronsSet buildNew(final BSPTree<Euclidean3D> tree)
    pub fn buildNew(&self, tree: Box<dyn std::any::Any>) -> PolyhedronsSet {
        // Java: return new PolyhedronsSet(tree, getTolerance());
        PolyhedronsSet { tolerance: self.tolerance, tree: Some(tree), boundary: None }
    }

    // Java: void computeGeometricalProperties()
    pub fn computeGeometricalProperties(&self) {
        // Java: // compute the contribution of all boundary facets
        // Java: getTree(true).visit(new FacetsContributionVisitor());
        // Java: if (getSize() < 0)
        {
            // Java: // the polyhedrons set as a finite outside
            // Java: // surrounded by an infinite inside
            // Java: setSize(Double.POSITIVE_INFINITY);
            // Java: setBarycenter((Point<Euclidean3D>) Vector3D.NaN);
        }
        // Java: else
        {
            // Java: // the polyhedrons set is finite, apply the remaining scaling factors
            // Java: setSize(getSize() / 3.0);
            // Java: setBarycenter((Point<Euclidean3D>) new Vector3D(1.0 / (4 * getSize()), (Vector3D) getBarycenter()));
        }
        todo!()
    }

    // Java: SubHyperplane<Euclidean3D> firstIntersection(final Vector3D point, final Line line)
    pub fn firstIntersection(&self, _point: &Box<dyn std::any::Any>, _line: &Box<dyn std::any::Any>) -> Option<Box<dyn std::any::Any>> {
        // Java: return recurseFirstIntersection(getTree(true), point, line);
        todo!()
    }

    // Java: SubHyperplane<Euclidean3D> recurseFirstIntersection()
    pub fn recurseFirstIntersection(&self, _node: &Box<dyn std::any::Any>, _point: &Box<dyn std::any::Any>, _line: &Box<dyn std::any::Any>) -> Option<Box<dyn std::any::Any>> {
        // Java: ... recursive intersection logic
        todo!()
    }

    // Java: SubHyperplane<Euclidean3D> boundaryFacet()
    pub fn boundaryFacet(&self, _point: &Box<dyn std::any::Any>, _node: &Box<dyn std::any::Any>) -> Option<Box<dyn std::any::Any>> {
        // Java: ... boundary facet logic
        todo!()
    }

    // Java: PolyhedronsSet rotate(final Vector3D center, final Rotation rotation)
    pub fn rotate(&self, _center: &Box<dyn std::any::Any>, _rotation: &Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {
        // Java: return (PolyhedronsSet) applyTransform(new RotationTransform(center, rotation));
        todo!()
    }

    // Java: PolyhedronsSet translate(final Vector3D translation)
    pub fn translate(&self, _translation: &Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {
        // Java: return (PolyhedronsSet) applyTransform(new TranslationTransform(translation));
        todo!()
    }

}

// Java: public class FacetsContributionVisitor
#[allow(non_snake_case)]
impl FacetsContributionVisitor {
    // Java: Order visitOrder(final BSPTree<Euclidean3D> node)
    pub fn visitOrder(&self, _node: &Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {
            // Java: return Order.MINUS_SUB_PLUS;
            todo!()
    }

    // Java: void visitInternalNode(final BSPTree<Euclidean3D> node)
    pub fn visitInternalNode(&self, _node: &Box<dyn std::any::Any>) {
            // Java: @SuppressWarnings("unchecked")
            // Java: final BoundaryAttribute<Euclidean3D> attribute =
                // Java: (BoundaryAttribute<Euclidean3D>) node.getAttribute();
            // Java: if (attribute.getPlusOutside() != null)
            {
                // Java: addContribution(attribute.getPlusOutside(), false);
            }
            // Java: if (attribute.getPlusInside() != null)
            {
                // Java: addContribution(attribute.getPlusInside(), true);
            }
            todo!()
    }

    // Java: void visitLeafNode(final BSPTree<Euclidean3D> node)
    pub fn visitLeafNode(&self, _node: &Box<dyn std::any::Any>) {
        // Java: // nothing to do
    }

    // Java: void addContribution(final SubHyperplane<Euclidean3D> facet, final boolean reversed)
    pub fn addContribution(&self, _facet: &Box<dyn std::any::Any>, _reversed: bool) {
            // Java: final Region<Euclidean2D> polygon = ((SubPlane) facet).getRemainingRegion();
            // Java: final double area    = polygon.getSize();
            // Java: if (Double.isInfinite(area))
            {
                // Java: setSize(Double.POSITIVE_INFINITY);
                // Java: setBarycenter((Point<Euclidean3D>) Vector3D.NaN);
            }
            // Java: else
            {
                // Java: final Plane    plane  = (Plane) facet.getHyperplane();
                // Java: final Vector3D facetB = plane.toSpace(polygon.getBarycenter());
                // Java: double   scaled = area * facetB.dotProduct(plane.getNormal());
                // Java: if (reversed)
                {
                    // Java: scaled = -scaled;
                }
                // Java: setSize(getSize() + scaled);
                // Java: setBarycenter((Point<Euclidean3D>) new Vector3D(1.0, (Vector3D) getBarycenter(), scaled, facetB));
            }
            todo!()
    }

}

// Java: public class RotationTransform
#[allow(non_snake_case)]
impl RotationTransform {
    // Java: Vector3D apply(final Point<Euclidean3D> point)
    pub fn apply_point(&self, _point: &Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {
            // Java: final Vector3D delta = ((Vector3D) point).subtract(center);
            // Java: return new Vector3D(1.0, center, 1.0, rotation.applyTo(delta));
            todo!()
    }

    // Java: Plane apply(final Hyperplane<Euclidean3D> hyperplane)
    pub fn apply_hyperplane(&self, _hyperplane: &Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {
            // Java: return ((Plane) hyperplane).rotate(center, rotation);
            todo!()
    }

    // Java: SubHyperplane<Euclidean2D> apply()
    pub fn apply_sub(&self, _sub: &Box<dyn std::any::Any>, _original: &Box<dyn std::any::Any>, _transformed: &Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {
        // Java: ... applies transform to sub hyperplane
        todo!()
    }

}

// Java: public class TranslationTransform
#[allow(non_snake_case)]
impl TranslationTransform {
    // Java: Vector3D apply(final Point<Euclidean3D> point)
    pub fn apply_point(&self, _point: &Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {
            // Java: return new Vector3D(1.0, (Vector3D) point, 1.0, translation);
            todo!()
    }

    // Java: Plane apply(final Hyperplane<Euclidean3D> hyperplane)
    pub fn apply_hyperplane(&self, _hyperplane: &Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {
            // Java: return ((Plane) hyperplane).translate(translation);
            todo!()
    }

    // Java: SubHyperplane<Euclidean2D> apply()
    pub fn apply_sub(&self, _sub: &Box<dyn std::any::Any>, _original: &Box<dyn std::any::Any>, _transformed: &Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {
        // Java: ... applies transform to sub hyperplane
        todo!()
    }

}
