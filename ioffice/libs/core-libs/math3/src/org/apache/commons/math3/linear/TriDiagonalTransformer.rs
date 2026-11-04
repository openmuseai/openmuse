// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class is
pub struct is {
}

// Java: public class is
pub struct is_2 {
}

// Java: public class is
pub struct is_3 {
}

// Java: public class is
pub struct is_4 {
}

// Java: public class is
impl is {
}

// Java: public class is
impl is_2 {
    // Java: double[][] getHouseholderVectorsRef()
    pub fn getHouseholderVectorsRef(&self) -> Vec<f64> {
        // Java: return householderVectors;
        todo!()
    }

}

// Java: public class is
impl is_3 {
    // Java: double[] getMainDiagonalRef()
    pub fn getMainDiagonalRef(&self) -> Vec<f64> {
        // Java: return main;
        todo!()
    }

}

// Java: public class is
impl is_4 {
    // Java: double[] getSecondaryDiagonalRef()
    pub fn getSecondaryDiagonalRef(&self) -> Vec<f64> {
        // Java: return secondary;
        todo!()
    }

}

// Java: public class TriDiagonalTransformer
pub struct TriDiagonalTransformer {
    pub(crate) householderVectors: Vec<Vec<f64>>,
    pub(crate) main: Vec<f64>,
    pub(crate) secondary: Vec<f64>,
    pub(crate) cachedQ: Box<dyn std::any::Any>,
    pub(crate) cachedQt: Box<dyn std::any::Any>,
    pub(crate) cachedT: Box<dyn std::any::Any>,
}

// Java: public class TriDiagonalTransformer
impl TriDiagonalTransformer {
}
