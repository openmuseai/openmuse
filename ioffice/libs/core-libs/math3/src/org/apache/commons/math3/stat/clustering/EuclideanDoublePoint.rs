// Java: import java.io.Serializable
// Java: import java.util.Collection
// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;

// Java field: long serialVersionUID

// Java: public class EuclideanDoublePoint implements Clusterable, Serializable
pub struct EuclideanDoublePoint {
    // Java field: double[] point
    pub(crate) point: Vec<f64>,
}

// Java: public class EuclideanDoublePoint implements Clusterable, Serializable
impl EuclideanDoublePoint {
    // Java: EuclideanDoublePoint(final double[] point)
    pub fn EuclideanDoublePoint(&self) {
        // Java: this.point = point;
    }

    // Java: EuclideanDoublePoint centroidOf(final Collection<EuclideanDoublePoint> points)
    pub fn centroidOf(&self) -> Box<dyn std::any::Any> {
        // Java: final double[] centroid = new double[getPoint().length];
        // Java: for (final EuclideanDoublePoint p : points)
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
        // Java: return new EuclideanDoublePoint(centroid);
        todo!()
    }

    // Java: double distanceFrom(final EuclideanDoublePoint p)
    pub fn distanceFrom(&self) -> f64 {
        // Java: return MathArrays.distance(point, p.getPoint());
        todo!()
    }

    // Java: boolean equals(final Object other)
    pub fn equals(&self) -> bool {
        // Java: if (!(other instanceof EuclideanDoublePoint))
        {
            // Java: return false;
        }
        // Java: return Arrays.equals(point, ((EuclideanDoublePoint) other).point);
        todo!()
    }

    // Java: double[] getPoint()
    pub fn getPoint(&self) -> Vec<f64> {
        // Java: return point;
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


