// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.util.Pair
use crate::org::apache::commons::math3::util::Pair;

use std::sync::Arc;

// Java field: long serialVersionUID
// Java field: long serialVersionUID

// Java: public class holds
pub struct holds {
}

// Java: public class PointVectorValuePair
pub struct PointVectorValuePair {
    point: Vec<f64>,
    value: Vec<f64>,
}

// Java: public class DataTransferObject
pub struct DataTransferObject {
    // Java field: double[] point
    pub(crate) point: Vec<f64>,
    // Java field: double[] value
    pub(crate) value: Vec<f64>,
}

// Java: public class holds
impl holds {
}

// Java: public class PointVectorValuePair
impl PointVectorValuePair {
    // Java: PointVectorValuePair()
    pub fn new(point: Vec<f64>, value: Vec<f64>) -> Self {
        PointVectorValuePair { point, value }
    }

    // Java: PointVectorValuePair()
    pub fn new_with_copy(point: Vec<f64>, value: Vec<f64>, copy_array: bool) -> Self {
        if copy_array {
            PointVectorValuePair {
                point: point.clone(),
                value: value.clone(),
            }
        } else {
            PointVectorValuePair { point, value }
        }
    }

    // Java: double[] getPoint()
    pub fn getPoint(&self) -> Vec<f64> {
        // Java: final double[] p = getKey();
        // Java: return p == null ? null : p.clone();
        self.point.clone()
    }

    // Java: double[] getPointRef()
    pub fn getPointRef(&self) -> &[f64] {
        // Java: return getKey();
        &self.point
    }

    // Java: double[] getValue()
    pub fn getValue(&self) -> Vec<f64> {
        // Java: final double[] v = super.getValue();
        // Java: return v == null ? null : v.clone();
        self.value.clone()
    }

    // Java: double[] getValueRef()
    pub fn getValueRef(&self) -> &[f64] {
        // Java: return super.getValue();
        &self.value
    }

    // Java: Object writeReplace()
    pub fn writeReplace(&self) -> DataTransferObject {
        // Java: return new DataTransferObject(getKey(), getValue());
        DataTransferObject {
            point: self.point.clone(),
            value: self.value.clone(),
        }
    }

}

// Java: public class DataTransferObject
impl DataTransferObject {
    // Java: Object readResolve()
    pub fn readResolve(&self) -> PointVectorValuePair {
            // Java: return new PointVectorValuePair(point, value, false);
            PointVectorValuePair::new_with_copy(self.point.clone(), self.value.clone(), false)
    }

}
