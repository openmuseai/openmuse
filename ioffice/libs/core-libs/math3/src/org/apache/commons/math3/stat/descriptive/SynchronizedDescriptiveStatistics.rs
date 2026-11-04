// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: long serialVersionUID

// Java: public class SynchronizedDescriptiveStatistics extends DescriptiveStatistics
pub struct SynchronizedDescriptiveStatistics {
}

// Java: public class SynchronizedDescriptiveStatistics extends DescriptiveStatistics
impl SynchronizedDescriptiveStatistics {
    // Java: SynchronizedDescriptiveStatistics()
    pub fn SynchronizedDescriptiveStatistics(&self) {
        // Java: // no try-catch or advertized IAE because arg is valid
        // Java: this(INFINITE_WINDOW);
    }

    // Java: SynchronizedDescriptiveStatistics(int window)
    pub fn SynchronizedDescriptiveStatistics_2(&self) {
        // Java: super(window);
    }

    // Java: SynchronizedDescriptiveStatistics(SynchronizedDescriptiveStatistics original)
    pub fn SynchronizedDescriptiveStatistics_3(&self) {
    }

    // Java: void addValue(double v)
    pub fn addValue(&self) {
        // Java: super.addValue(v);
    }

    // Java: double apply(UnivariateStatistic stat)
    pub fn apply(&self) -> f64 {
        // Java: return super.apply(stat);
        todo!()
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: super.clear();
    }

    // Java: double getElement(int index)
    pub fn getElement(&self) -> f64 {
        // Java: return super.getElement(index);
        todo!()
    }

    // Java: long getN()
    pub fn getN(&self) -> i64 {
        // Java: return super.getN();
        todo!()
    }

    // Java: double getStandardDeviation()
    pub fn getStandardDeviation(&self) -> f64 {
        // Java: return super.getStandardDeviation();
        todo!()
    }

    // Java: double getQuadraticMean()
    pub fn getQuadraticMean(&self) -> f64 {
        // Java: return super.getQuadraticMean();
        todo!()
    }

    // Java: double[] getValues()
    pub fn getValues(&self) -> Vec<f64> {
        // Java: return super.getValues();
        todo!()
    }

    // Java: int getWindowSize()
    pub fn getWindowSize(&self) -> i32 {
        // Java: return super.getWindowSize();
        todo!()
    }

    // Java: void setWindowSize(int windowSize)
    pub fn setWindowSize(&self) {
        // Java: super.setWindowSize(windowSize);
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return super.toString();
        todo!()
    }

    // Java: SynchronizedDescriptiveStatistics copy()
    pub fn copy(&self) -> SynchronizedDescriptiveStatistics {
        // Java: SynchronizedDescriptiveStatistics result =
            // Java: new SynchronizedDescriptiveStatistics();
        // Java: // No try-catch or advertised exception because arguments are guaranteed non-null
        // Java: copy(this, result);
        // Java: return result;
        todo!()
    }

    // Java: static void copy()
    pub fn copy_2(&self) {
    }

}
