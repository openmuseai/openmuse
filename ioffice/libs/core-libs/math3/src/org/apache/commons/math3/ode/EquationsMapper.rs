// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;

// Java field: long serialVersionUID

// Java: public class are
pub struct are {
}

// Java: public class EquationsMapper
pub struct EquationsMapper {
    // Java field: int firstIndex
    pub(crate) firstIndex: i32,
    // Java field: int dimension
    pub(crate) dimension: i32,
}

// Java: public class are
impl are {
}

// Java: public class EquationsMapper
impl EquationsMapper {
    // Java: EquationsMapper(final int firstIndex, final int dimension)
    pub fn EquationsMapper(&self) {
        // Java: this.firstIndex = firstIndex;
        // Java: this.dimension  = dimension;
    }

    // Java: int getFirstIndex()
    pub fn getFirstIndex(&self) -> i32 {
        // Java: return firstIndex;
        todo!()
    }

    // Java: int getDimension()
    pub fn getDimension(&self) -> i32 {
        // Java: return dimension;
        todo!()
    }

    // Java: void extractEquationData(double[] complete, double[] equationData)
    pub fn extractEquationData(&self) {
    }

    // Java: void insertEquationData(double[] equationData, double[] complete)
    pub fn insertEquationData(&self) {
    }

}
