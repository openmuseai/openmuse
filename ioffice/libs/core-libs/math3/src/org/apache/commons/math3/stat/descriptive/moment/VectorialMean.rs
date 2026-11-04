#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.io.Serializable
// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 8394952455588297803i64;

// Java: public class VectorialMean implements Serializable
pub struct VectorialMean {
    // Java field: Mean[] means
    pub(crate) means: Vec<Box<dyn std::any::Any>>,
}

// Java: public class VectorialMean implements Serializable
impl VectorialMean {
    // Java: VectorialMean(int dimension)
    pub fn new(dimension: i32) -> Self {
        // Java: means = new Mean[dimension];
        // Java: for (int i = 0; i < dimension; ++i)
        // Java: means[i] = new Mean();
        VectorialMean {
            means: (0..dimension).map(|_| -> Box<dyn std::any::Any> { Box::new(0i32) }).collect(),
        }
    }

    // Java: void increment(double[] v)
    pub fn increment(&mut self, _v: &[f64]) {
        // Java: if (v.length != means.length)
        // Java: throw new DimensionMismatchException(v.length, means.length);
        // Java: for (int i = 0; i < v.length; ++i)
        // Java: means[i].increment(v[i]);
        todo!()
    }

    // Java: double[] getResult()
    pub fn getResult(&self) -> Vec<f64> {
        // Java: double[] result = new double[means.length];
        // Java: for (int i = 0; i < result.length; ++i)
        // Java: result[i] = means[i].getResult();
        // Java: return result;
        todo!()
    }

    // Java: long getN()
    pub fn getN(&self) -> i64 {
        // Java: return (means.length == 0) ? 0 : means[0].getN();
        todo!()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: final int prime = 31;
        // Java: int result = 1;
        // Java: result = prime * result + Arrays.hashCode(means);
        // Java: return result;
        todo!()
    }

    // Java: boolean equals(Object obj)
    pub fn equals_obj(&self, _obj: &dyn std::any::Any) -> bool {
        // Java: if (this == obj)
        // Java: return true;
        // Java: if (!(obj instanceof VectorialMean))
        // Java: return false;
        // Java: VectorialMean other = (VectorialMean) obj;
        // Java: if (!Arrays.equals(means, other.means))
        // Java: return false;
        // Java: return true;
        todo!()
    }

}
