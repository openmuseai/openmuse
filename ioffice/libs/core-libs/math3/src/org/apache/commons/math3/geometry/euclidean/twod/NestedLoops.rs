#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.ArrayList
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.IntervalsSet
use crate::org::apache::commons::math3::geometry::euclidean::oned::IntervalsSet;
// Java: import org.apache.commons.math3.geometry.partitioning.Region
use crate::org::apache::commons::math3::geometry::partitioning::Region;
// Java: import org.apache.commons.math3.geometry.partitioning.RegionFactory
use crate::org::apache::commons::math3::geometry::partitioning::RegionFactory;
// Java: import org.apache.commons.math3.geometry.partitioning.SubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::SubHyperplane;

// Java: public class represent
pub struct represent {
}

// Java: public class NestedLoops
pub struct NestedLoops {
    // Java field: Vector2D[] loop
    pub(crate) loop_vertices: Vec<Box<dyn std::any::Any>>,
    // Java field: List<NestedLoops> surrounded
    pub(crate) surrounded: Vec<NestedLoops>,
    // Java field: Region<Euclidean2D> polygon
    pub(crate) polygon: Option<Box<dyn std::any::Any>>,
    // Java field: boolean originalIsClockwise
    pub(crate) originalIsClockwise: bool,
    // Java field: double tolerance
    pub(crate) tolerance: f64,
}

// Java: public class represent
impl represent {
}

// Java: public class NestedLoops
impl NestedLoops {
    // Java: NestedLoops(final Vector2D[] loop, final double tolerance)
    pub fn new(_loop_arr: Vec<Box<dyn std::any::Any>>, _tolerance: f64) -> Self {
        todo!()
    }

    // Java: void add(final Vector2D[] bLoop)
    pub fn add_loop(&mut self, _b_loop: Vec<Box<dyn std::any::Any>>) {
        // Java: add(new NestedLoops(bLoop, tolerance));
        todo!()
    }

    // Java: void add(final NestedLoops node)
    pub fn add_node(&mut self, _node: NestedLoops) {
        // Java: // check if we can go deeper in the tree
        // Java: for (final NestedLoops child : surrounded)
        // Java: if (child.polygon.contains(node.polygon))
        // Java: child.add(node);
        // Java: return;
        // Java: // check if we can absorb some of the instance children
        // Java: for (final Iterator<NestedLoops> iterator = surrounded.iterator(); iterator.hasNext();)
        // Java: final NestedLoops child = iterator.next();
        // Java: if (node.polygon.contains(child.polygon))
        // Java: node.surrounded.add(child);
        // Java: iterator.remove();
        // Java: // we should be separate from the remaining children
        // Java: RegionFactory<Euclidean2D> factory = new RegionFactory<Euclidean2D>();
        // Java: for (final NestedLoops child : surrounded)
        // Java: if (!factory.intersection(node.polygon, child.polygon).isEmpty())
        // Java: throw new MathIllegalArgumentException(LocalizedFormats.CROSSING_BOUNDARY_LOOPS);
        // Java: surrounded.add(node);
        todo!()
    }

    // Java: void correctOrientation()
    pub fn correctOrientation(&mut self) {
        // Java: for (NestedLoops child : surrounded)
        // Java: child.setClockWise(true);
        for child in self.surrounded.iter_mut() {
            child.setClockWise(true);
        }
    }

    // Java: void setClockWise(final boolean clockwise)
    pub fn setClockWise(&mut self, clockwise: bool) {
        // Java: if (originalIsClockwise ^ clockwise)
        if self.originalIsClockwise ^ clockwise {
            // Java: // we need to inverse the original loop
            // Java: int min = -1;
            // Java: int max = loop.length;
            // Java: while (++min < --max)
            // Java: final Vector2D tmp = loop[min];
            // Java: loop[min] = loop[max];
            // Java: loop[max] = tmp;
            self.loop_vertices.reverse();
        }
        // Java: // go deeper in the tree
        // Java: for (final NestedLoops child : surrounded)
        // Java: child.setClockWise(!clockwise);
        for child in self.surrounded.iter_mut() {
            child.setClockWise(!clockwise);
        }
    }

}
