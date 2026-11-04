#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.InsufficientDataException
use crate::org::apache::commons::math3::exception::InsufficientDataException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Euclidean2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Euclidean2D;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Line
use crate::org::apache::commons::math3::geometry::euclidean::twod::Line;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Segment
use crate::org::apache::commons::math3::geometry::euclidean::twod::Segment;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Vector2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Vector2D;
// Java: import org.apache.commons.math3.geometry.hull.ConvexHull
use crate::org::apache::commons::math3::geometry::hull::ConvexHull;
// Java: import org.apache.commons.math3.geometry.partitioning.Region
use crate::org::apache::commons::math3::geometry::partitioning::Region;
// Java: import org.apache.commons.math3.geometry.partitioning.RegionFactory
use crate::org::apache::commons::math3::geometry::partitioning::RegionFactory;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = -7746278232277323425i64;

// Java: public class represents
pub struct represents {
}

// Java: public class ConvexHull2D
pub struct ConvexHull2D {
    // Java field: Vector2D[] vertices
    pub(crate) vertices: Vec<Box<dyn std::any::Any>>,
    // Java field: double tolerance
    pub(crate) tolerance: f64,
    // Java field: Segment[] lineSegments
    pub(crate) lineSegments: Option<Vec<Box<dyn std::any::Any>>>,
}

// Java: public class represents
impl represents {
}

// Java: public class ConvexHull2D
impl ConvexHull2D {
    // Java: ConvexHull2D(final Vector2D[] vertices, final double tolerance)
    pub fn new(_vertices: Vec<Box<dyn std::any::Any>>, tolerance: f64) -> Self {
        ConvexHull2D {
            vertices: _vertices,
            tolerance,
            lineSegments: None,
        }
    }

    // Java: boolean isConvex(final Vector2D[] hullVertices)
    pub fn isConvex(_hullVertices: &[Box<dyn std::any::Any>]) -> bool {
        // Java: if (hullVertices.length < 3)
        // Java: return true;
        // Java: int sign = 0;
        // Java: for (int i = 0; i < hullVertices.length; i++)
        // Java: final Vector2D p1 = hullVertices[i == 0 ? hullVertices.length - 1 : i - 1];
        // Java: final Vector2D p2 = hullVertices[i];
        // Java: final Vector2D p3 = hullVertices[i == hullVertices.length - 1 ? 0 : i + 1];
        // Java: final Vector2D d1 = p2.subtract(p1);
        // Java: final Vector2D d2 = p3.subtract(p2);
        // Java: final double crossProduct = MathArrays.linearCombination(d1.getX(), d2.getY(), -d1.getY(), d2.getX());
        // Java: final int cmp = Precision.compareTo(crossProduct, 0.0, tolerance);
        // Java: // in case of collinear points the cross product will be zero
        // Java: if (cmp != 0.0)
        // Java: if (sign != 0.0 && cmp != sign)
        // Java: return false;
        // Java: sign = cmp;
        // Java: return true;
        todo!()
    }

    // Java: Vector2D[] getVertices()
    pub fn getVertices(&self) -> &Vec<Box<dyn std::any::Any>> {
        // Java: return vertices.clone();
        &self.vertices
    }

    // Java: Segment[] getLineSegments()
    pub fn getLineSegments(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: return retrieveLineSegments().clone();
        todo!()
    }

    // Java: Segment[] retrieveLineSegments()
    pub fn retrieveLineSegments(&mut self) -> &Vec<Box<dyn std::any::Any>> {
        // Java: if (lineSegments == null)
        // Java: // construct the line segments - handle special cases of 1 or 2 points
        // Java: final int size = vertices.length;
        // Java: if (size <= 1)
        // Java: this.lineSegments = new Segment[0];
        // Java: else if (size == 2)
        // Java: this.lineSegments = new Segment[1];
        // Java: final Vector2D p1 = vertices[0];
        // Java: final Vector2D p2 = vertices[1];
        // Java: this.lineSegments[0] = new Segment(p1, p2, new Line(p1, p2, tolerance));
        // Java: else
        // Java: this.lineSegments = new Segment[size];
        // Java: Vector2D firstPoint = null;
        // Java: Vector2D lastPoint = null;
        // Java: int index = 0;
        // Java: for (Vector2D point : vertices)
        // Java: if (lastPoint == null)
        // Java: firstPoint = point;
        // Java: lastPoint = point;
        // Java: else
        // Java: this.lineSegments[index++] =
        // Java: new Segment(lastPoint, point, new Line(lastPoint, point, tolerance));
        // Java: lastPoint = point;
        // Java: this.lineSegments[index] =
        // Java: new Segment(lastPoint, firstPoint, new Line(lastPoint, firstPoint, tolerance));
        // Java: return lineSegments;
        if self.lineSegments.is_none() {
            self.lineSegments = Some(Vec::new());
        }
        self.lineSegments.as_ref().unwrap()
    }

    // Java: Region<Euclidean2D> createRegion()
    pub fn createRegion(&self) -> Box<dyn std::any::Any> {
        // Java: if (vertices.length < 3)
        // Java: throw new InsufficientDataException();
        // Java: final RegionFactory<Euclidean2D> factory = new RegionFactory<Euclidean2D>();
        // Java: final Segment[] segments = retrieveLineSegments();
        // Java: final Line[] lineArray = new Line[segments.length];
        // Java: for (int i = 0; i < segments.length; i++)
        // Java: lineArray[i] = segments[i].getLine();
        // Java: return factory.buildConvex(lineArray);
        todo!()
    }

}
