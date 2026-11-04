// Java: import java.io.Serializable
// Java: import java.util.Arrays
// Java: import java.util.Collection
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;

// Java field: long serialVersionUID

// Java: public class EuclideanIntegerPoint implements Clusterable, Serializable
pub struct EuclideanIntegerPoint {
    // Java field: int[] point
    pub(crate) point: Vec<i32>,
}

// Java: public class EuclideanIntegerPoint implements Clusterable, Serializable
impl EuclideanIntegerPoint {
    // Java: EuclideanIntegerPoint(final int[] point)
    pub fn EuclideanIntegerPoint(&self) {
        // Java: this.point = point;
    }

    // Java: int[] getPoint()
    pub fn getPoint(&self) -> Vec<i32> {
        // Java: return point;
        todo!()
    }

    // Java: double distanceFrom(final EuclideanIntegerPoint p)
    pub fn distanceFrom(&self) -> f64 {
        // Java: return MathArrays.distance(point, p.getPoint());
        todo!()
    }

    // Java: EuclideanIntegerPoint centroidOf(final Collection<EuclideanIntegerPoint> points)
    pub fn centroidOf(&self) -> Box<dyn std::any::Any> {
        // Java: int[] centroid = new int[getPoint().length];
        // Java: for (EuclideanIntegerPoint p : points)
        {
            // Java: for (int i = 0; i < centroid.length; i++)
            {
                // Java: centroid[i] += p.getPoint()[i];
            }
        }
        // Java: for (int i = 0; i < centroid.length; i++)
        {
            // Java: centroid[i] /= points.size();
        }
        // Java: return new EuclideanIntegerPoint(centroid);
        todo!()
    }

    // Java: boolean equals(final Object other)
    pub fn equals(&self) -> bool {
        // Java: if (!(other instanceof EuclideanIntegerPoint))
        {
            // Java: return false;
        }
        // Java: return Arrays.equals(point, ((EuclideanIntegerPoint) other).point);
        todo!()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return Arrays.hashCode(point);
        todo!()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return Arrays.toString(point);
        todo!()
    }

}


