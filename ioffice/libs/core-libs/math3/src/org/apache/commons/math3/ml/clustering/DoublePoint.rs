// Java: import java.io.Serializable
// Java: import java.util.Arrays

// Java field: long serialVersionUID

// Java: public class DoublePoint implements Clusterable, Serializable
pub struct DoublePoint {
    // Java field: double[] point
    pub(crate) point: Box<dyn std::any::Any>,
}

// Java: public class DoublePoint implements Clusterable, Serializable
impl DoublePoint {
    // Java: DoublePoint(final double[] point)
    pub fn DoublePoint(&self) {
        // Java: this.point = point;
    }

    // Java: DoublePoint(final int[] point)
    pub fn DoublePoint_2(&self) {
        // Java: this.point = new double[point.length];
        // Java: for ( int i = 0; i < point.length; i++)
        {
            // Java: this.point[i] = point[i];
        }
    }

    // Java: double[] getPoint()
    pub fn getPoint(&self) -> Vec<f64> {
        // Java: return point;
        todo!()
    }

    // Java: boolean equals(final Object other)
    pub fn equals(&self) -> bool {
        // Java: if (!(other instanceof DoublePoint))
        {
            // Java: return false;
        }
        // Java: return Arrays.equals(point, ((DoublePoint) other).point);
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


