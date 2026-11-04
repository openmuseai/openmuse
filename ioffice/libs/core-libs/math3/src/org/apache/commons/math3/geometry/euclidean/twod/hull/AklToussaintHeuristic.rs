#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.List
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Vector2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Vector2D;

// Java: public class AklToussaintHeuristic
pub struct AklToussaintHeuristic {
}

// Java: public class AklToussaintHeuristic
impl AklToussaintHeuristic {
    // Java: AklToussaintHeuristic()
    fn new() -> Self {
        AklToussaintHeuristic {}
    }

    // Java: static Collection<Vector2D> reducePoints(final Collection<Vector2D> points)
    pub fn reducePoints(_points: &Vec<Box<dyn std::any::Any>>) -> Vec<Box<dyn std::any::Any>> {
        // Java: // find the leftmost point
        // Java: int size = 0;
        // Java: Vector2D minX = null;
        // Java: Vector2D maxX = null;
        // Java: Vector2D minY = null;
        // Java: Vector2D maxY = null;
        // Java: for (Vector2D p : points)
        // Java: if (minX == null || p.getX() < minX.getX())
        // Java: minX = p;
        // Java: if (maxX == null || p.getX() > maxX.getX())
        // Java: maxX = p;
        // Java: if (minY == null || p.getY() < minY.getY())
        // Java: minY = p;
        // Java: if (maxY == null || p.getY() > maxY.getY())
        // Java: maxY = p;
        // Java: size++;
        // Java: if (size < 4)
        // Java: return points;
        // Java: final List<Vector2D> quadrilateral = buildQuadrilateral(minY, maxX, maxY, minX);
        // Java: // if the quadrilateral is not well formed, e.g. only 2 points, do not attempt to reduce
        // Java: if (quadrilateral.size() < 3)
        // Java: return points;
        // Java: final List<Vector2D> reducedPoints = new ArrayList<Vector2D>(quadrilateral);
        // Java: for (final Vector2D p : points)
        // Java: // check all points if they are within the quadrilateral
        // Java: // in which case they can not be part of the convex hull
        // Java: if (!insideQuadrilateral(p, quadrilateral))
        // Java: reducedPoints.add(p);
        // Java: return reducedPoints;
        todo!()
    }

    // Java: static List<Vector2D> buildQuadrilateral(final Vector2D... points)
    pub fn buildQuadrilateral(_points: &[Box<dyn std::any::Any>]) -> Vec<Box<dyn std::any::Any>> {
        // Java: List<Vector2D> quadrilateral = new ArrayList<Vector2D>();
        // Java: for (Vector2D p : points)
        // Java: if (!quadrilateral.contains(p))
        // Java: quadrilateral.add(p);
        // Java: return quadrilateral;
        todo!()
    }

    // Java: static boolean insideQuadrilateral()
    pub fn insideQuadrilateral() -> bool {
        todo!()
    }

}
