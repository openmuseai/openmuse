// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.stat.descriptive.AbstractStorelessUnivariateStatistic
use crate::org::apache::commons::math3::stat::descriptive::AbstractStorelessUnivariateStatistic;
// Java: import org.apache.commons.math3.stat.descriptive.WeightedEvaluation
use crate::org::apache::commons::math3::stat::descriptive::WeightedEvaluation;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: long serialVersionUID

// Java: public class concurrently
pub struct concurrently {
}

// Java: public class Product
pub struct Product {
    // Java field: long n
    pub(crate) n: i64,
    // Java field: double value
    pub(crate) value: f64,
}

// Java: public class concurrently
impl concurrently {
}

// Java: public class Product
impl Product {
    // Java: Product()
    pub fn Product(&self) {
        // Java: n = 0;
        // Java: value = 1;
    }

    // Java: Product(Product original)
    pub fn Product_2(&self) {
        // Java: copy(original, this);
    }

    // Java: void increment(final double d)
    pub fn increment(&self) {
        // Java: value *= d;
        // Java: n++;
    }

    // Java: double getResult()
    pub fn getResult(&self) -> f64 {
        // Java: return value;
        todo!()
    }

    // Java: long getN()
    pub fn getN(&self) -> i64 {
        // Java: return n;
        todo!()
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: value = 1;
        // Java: n = 0;
    }

    // Java: double evaluate(final double[] values, final int begin, final int length)
    pub fn evaluate(&self) -> f64 {
        todo!()
    }

    // Java: double evaluate()
    pub fn evaluate_2(&self) -> f64 {
        todo!()
    }

    // Java: double evaluate(final double[] values, final double[] weights)
    pub fn evaluate_3(&self) -> f64 {
        todo!()
    }

    // Java: Product copy()
    pub fn copy(&self) -> Product {
        // Java: Product result = new Product();
        // Java: // No try-catch or advertised exception because args are valid
        // Java: copy(this, result);
        // Java: return result;
        todo!()
    }

    // Java: static void copy(Product source, Product dest)
    pub fn copy_2(&self) {
    }

}
