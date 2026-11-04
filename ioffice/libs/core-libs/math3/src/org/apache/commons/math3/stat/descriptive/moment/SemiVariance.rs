#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.stat.descriptive.AbstractUnivariateStatistic
use crate::org::apache::commons::math3::stat::descriptive::AbstractUnivariateStatistic;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

use std::sync::OnceLock;
use std::any::Any;

// Java field: Direction UPSIDE_VARIANCE
static UPSIDE_VARIANCE: OnceLock<Direction> = OnceLock::new();
// Java field: Direction DOWNSIDE_VARIANCE
static DOWNSIDE_VARIANCE: OnceLock<Direction> = OnceLock::new();
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = -2653430366886024994;

// Java: public class is
pub struct is {}

// Java: public class SemiVariance
pub struct SemiVariance {
    // Java field: boolean biasCorrected
    pub(crate) biasCorrected: bool,
    // Java field: Direction varianceDirection
    pub(crate) varianceDirection: Direction,
}

// Java: public enum Direction
#[derive(Clone, Copy)]
pub struct Direction {
    // Java field: boolean direction
    pub(crate) direction: bool,
}

// Java: public class is
impl is {}

// Java: public class SemiVariance
impl SemiVariance {
    // Java: SemiVariance()
    pub fn new() -> Self {
        SemiVariance {
            biasCorrected: true,
            varianceDirection: Direction { direction: false },
        }
    }

    // Java: SemiVariance(final boolean biasCorrected)
    pub fn new_with_bias(biasCorrected: bool) -> Self {
        // Java: this.biasCorrected = biasCorrected;
        SemiVariance {
            biasCorrected,
            varianceDirection: Direction { direction: false },
        }
    }

    // Java: SemiVariance(final Direction direction)
    pub fn new_with_direction(direction: Direction) -> Self {
        // Java: this.varianceDirection = direction;
        SemiVariance {
            biasCorrected: true,
            varianceDirection: direction,
        }
    }

    // Java: SemiVariance(final boolean corrected, final Direction direction)
    pub fn new_with_bias_and_direction(corrected: bool, direction: Direction) -> Self {
        // Java: this.biasCorrected = corrected;
        // Java: this.varianceDirection = direction;
        SemiVariance {
            biasCorrected: corrected,
            varianceDirection: direction,
        }
    }

    // Java: SemiVariance(final SemiVariance original)
    pub fn new_copy(original: &SemiVariance) -> Self {
        // Java: copy(original, this);
        SemiVariance {
            biasCorrected: original.biasCorrected,
            varianceDirection: original.varianceDirection,
        }
    }

    // Java: SemiVariance copy()
    pub fn copy(&self) -> SemiVariance {
        // Java: SemiVariance result = new SemiVariance();
        // Java: // No try-catch or advertised exception because args are guaranteed non-null
        // Java: copy(this, result);
        // Java: return result;
        SemiVariance::new_copy(self)
    }

    // Java: static void copy(final SemiVariance source, SemiVariance dest)
    pub fn copy_to(source: &SemiVariance, dest: &mut SemiVariance) {
        // Java: dest.biasCorrected = source.biasCorrected;
        dest.biasCorrected = source.biasCorrected;
        // Java: dest.varianceDirection = source.varianceDirection;
        dest.varianceDirection = source.varianceDirection;
    }

    // Java: double evaluate(final double[] values, final int start, final int length)
    pub fn evaluate(&self, values: &[f64], start: usize, length: usize) -> f64 {
        // Java: double m = (new Mean()).evaluate(values, start, length);
        // Java: return evaluate(values, m, varianceDirection, biasCorrected, 0, values.length);
        todo!()
    }

    // Java: double evaluate(final double[] values, Direction direction)
    pub fn evaluate_with_direction(&self, values: &[f64], direction: Direction) -> f64 {
        // Java: double m = (new Mean()).evaluate(values);
        // Java: return evaluate (values, m, direction, biasCorrected, 0, values.length);
        todo!()
    }

    // Java: double evaluate(final double[] values, final double cutoff)
    pub fn evaluate_with_cutoff(&self, values: &[f64], cutoff: f64) -> f64 {
        // Java: return evaluate(values, cutoff, varianceDirection, biasCorrected, 0, values.length);
        todo!()
    }

    // Java: double evaluate(final double[] values, final double cutoff, final Direction direction)
    pub fn evaluate_with_cutoff_direction(&self, values: &[f64], cutoff: f64, direction: Direction) -> f64 {
        // Java: return evaluate(values, cutoff, direction, biasCorrected, 0, values.length);
        todo!()
    }

    // Java: double evaluate()
    pub fn evaluate_full(&self, values: &[f64], cutoff: f64, direction: Direction, corrected: bool, start: usize, length: usize) -> f64 {
        // Java: if (values.length == 0)
        if values.is_empty() {
            return f64::NAN;
        }
        // Java: if (values.length == 1)
        if values.len() == 1 {
            return 0.0;
        }
        // Java: final boolean booleanDirection = direction.getDirection();
        let boolean_direction = direction.getDirection();
        // Java: double dev = 0.0;
        let mut dev: f64;
        // Java: double sumsq = 0.0;
        let mut sumsq: f64 = 0.0;
        // Java: for (int i = start; i < length; i++)
        for i in start..length {
            // Java: if ((values[i] > cutoff) == booleanDirection)
            if (values[i] > cutoff) == boolean_direction {
                // Java: dev = values[i] - cutoff;
                dev = values[i] - cutoff;
                // Java: sumsq += dev * dev;
                sumsq += dev * dev;
            }
        }
        // Java: if (corrected)
        if corrected {
            // Java: return sumsq / (length - 1.0);
            sumsq / (length as f64 - 1.0)
        } else {
            // Java: return sumsq / length;
            sumsq / length as f64
        }
    }

    // Java: boolean isBiasCorrected()
    pub fn isBiasCorrected(&self) -> bool {
        // Java: return biasCorrected;
        self.biasCorrected
    }

    // Java: void setBiasCorrected(boolean biasCorrected)
    pub fn setBiasCorrected(&mut self, biasCorrected: bool) {
        // Java: this.biasCorrected = biasCorrected;
        self.biasCorrected = biasCorrected;
    }

    // Java: Direction getVarianceDirection()
    pub fn getVarianceDirection(&self) -> Direction {
        // Java: return varianceDirection;
        self.varianceDirection
    }

    // Java: void setVarianceDirection(Direction varianceDirection)
    pub fn setVarianceDirection(&mut self, varianceDirection: Direction) {
        // Java: this.varianceDirection = varianceDirection;
        self.varianceDirection = varianceDirection;
    }
}

// Java: public enum Direction
impl Direction {
    // Java: boolean getDirection()
    pub fn getDirection(&self) -> bool {
        // Java: return direction;
        self.direction
    }
}
