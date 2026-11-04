// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.util.Pair
use crate::org::apache::commons::math3::util::Pair;

// Java field: long serialVersionUID
// Java field: long serialVersionUID

// Java: public class holds
pub struct holds {
}

// Java: public class PointValuePair
pub struct PointValuePair {
}

// Java: public class DataTransferObject
pub struct DataTransferObject {
    // Java field: double[] point
    pub(crate) point: Box<dyn std::any::Any>,
    // Java field: double value
    pub(crate) value: f64,
}

// Java: public class holds
impl holds {
}

// Java: public class PointValuePair
impl PointValuePair {
    // Java: PointValuePair()
    pub fn PointValuePair(&self) {
    }

    // Java: PointValuePair()
    pub fn PointValuePair_2(&self) {
    }

    // Java: double[] getPoint()
    pub fn getPoint(&self) -> Vec<f64> {
        // Java: final double[] p = getKey();
        // Java: return p == null ? null : p.clone();
        todo!()
    }

    // Java: double[] getPointRef()
    pub fn getPointRef(&self) -> Vec<f64> {
        // Java: return getKey();
        todo!()
    }

    // Java: Object writeReplace()
    pub fn writeReplace(&self) -> Box<dyn std::any::Any> {
        // Java: return new DataTransferObject(getKey(), getValue());
        todo!()
    }

}

// Java: public class DataTransferObject
impl DataTransferObject {
    // Java: Object readResolve()
    pub fn readResolve(&self) -> Box<dyn std::any::Any> {
            // Java: return new PointValuePair(point, value, false);
        todo!()
    }

}
