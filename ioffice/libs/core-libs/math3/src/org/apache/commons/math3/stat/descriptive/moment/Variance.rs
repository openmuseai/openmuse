// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.stat.descriptive.WeightedEvaluation
use crate::org::apache::commons::math3::stat::descriptive::WeightedEvaluation;
// Java: import org.apache.commons.math3.stat.descriptive.AbstractStorelessUnivariateStatistic
use crate::org::apache::commons::math3::stat::descriptive::AbstractStorelessUnivariateStatistic;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: long serialVersionUID

// Java: public class concurrently
pub struct concurrently {
}

// Java: public class Variance
pub struct Variance {
    // Java field: SecondMoment moment
    pub(crate) moment: Box<dyn std::any::Any>,
    // Java field: boolean incMoment
    pub(crate) incMoment: bool,
    // Java field: boolean isBiasCorrected
    pub(crate) isBiasCorrected: bool,
}

// Java: public class concurrently
impl concurrently {
}

// Java: public class Variance
impl Variance {
    // Java: Variance()
    pub fn Variance(&self) {
        // Java: moment = new SecondMoment();
    }

    // Java: Variance(final SecondMoment m2)
    pub fn Variance_2(&self) {
        // Java: incMoment = false;
        // Java: this.moment = m2;
    }

    // Java: Variance(boolean isBiasCorrected)
    pub fn Variance_3(&self) {
        // Java: moment = new SecondMoment();
        // Java: this.isBiasCorrected = isBiasCorrected;
    }

    // Java: Variance(boolean isBiasCorrected, SecondMoment m2)
    pub fn Variance_4(&self) {
        // Java: incMoment = false;
        // Java: this.moment = m2;
        // Java: this.isBiasCorrected = isBiasCorrected;
    }

    // Java: Variance(Variance original)
    pub fn Variance_5(&self) {
        // Java: copy(original, this);
    }

    // Java: void increment(final double d)
    pub fn increment(&self) {
        // Java: if (incMoment)
        {
            // Java: moment.increment(d);
        }
    }

    // Java: double getResult()
    pub fn getResult(&self) -> f64 {
            // Java: if (moment.n == 0)
            {
                // Java: return Double.NaN;
            }
            // Java: else if (moment.n == 1)
            {
                // Java: return 0d;
            }
            // Java: else
            {
                // Java: if (isBiasCorrected)
                {
                    // Java: return moment.m2 / (moment.n - 1d);
                }
                // Java: else
                {
                    // Java: return moment.m2 / (moment.n);
                }
            }
        todo!()
    }

    // Java: long getN()
    pub fn getN(&self) -> i64 {
        // Java: return moment.getN();
        todo!()
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: if (incMoment)
        {
            // Java: moment.clear();
        }
    }

    // Java: double evaluate(final double[] values)
    pub fn evaluate(&self) -> f64 {
        // Java: if (values == null)
        {
            // Java: throw new NullArgumentException(LocalizedFormats.INPUT_ARRAY);
        }
        // Java: return evaluate(values, 0, values.length);
        todo!()
    }

    // Java: double evaluate(final double[] values, final int begin, final int length)
    pub fn evaluate_2(&self) -> f64 {
        todo!()
    }

    // Java: double evaluate()
    pub fn evaluate_3(&self) -> f64 {
        todo!()
    }

    // Java: double evaluate(final double[] values, final double[] weights)
    pub fn evaluate_4(&self) -> f64 {
        todo!()
    }

    // Java: double evaluate()
    pub fn evaluate_5(&self) -> f64 {
        todo!()
    }

    // Java: double evaluate(final double[] values, final double mean)
    pub fn evaluate_6(&self) -> f64 {
        // Java: return evaluate(values, mean, 0, values.length);
        todo!()
    }

    // Java: double evaluate()
    pub fn evaluate_7(&self) -> f64 {
        todo!()
    }

    // Java: double evaluate(final double[] values, final double[] weights, final double mean)
    pub fn evaluate_8(&self) -> f64 {
        todo!()
    }

    // Java: boolean isBiasCorrected()
    pub fn isBiasCorrected(&self) -> bool {
        // Java: return isBiasCorrected;
        todo!()
    }

    // Java: void setBiasCorrected(boolean biasCorrected)
    pub fn setBiasCorrected(&self) {
        // Java: this.isBiasCorrected = biasCorrected;
    }

    // Java: Variance copy()
    pub fn copy(&self) -> Variance {
        // Java: Variance result = new Variance();
        // Java: // No try-catch or advertised exception because parameters are guaranteed non-null
        // Java: copy(this, result);
        // Java: return result;
        todo!()
    }

    // Java: static void copy(Variance source, Variance dest)
    pub fn copy_2(&self) {
    }

}
