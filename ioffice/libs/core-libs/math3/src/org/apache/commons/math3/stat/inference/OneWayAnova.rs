// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import org.apache.commons.math3.distribution.FDistribution
use crate::org::apache::commons::math3::distribution::FDistribution;
// Java: import org.apache.commons.math3.exception.ConvergenceException
use crate::org::apache::commons::math3::exception::ConvergenceException;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.stat.descriptive.SummaryStatistics
use crate::org::apache::commons::math3::stat::descriptive::SummaryStatistics;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java: public class OneWayAnova
pub struct OneWayAnova {
}

// Java: public class AnovaStats
pub struct AnovaStats {
    // Java field: int dfbg
    pub(crate) dfbg: i32,
    // Java field: int dfwg
    pub(crate) dfwg: i32,
    // Java field: double F
    pub(crate) F: f64,
}

// Java: public class OneWayAnova
impl OneWayAnova {
    // Java: OneWayAnova()
    pub fn OneWayAnova(&self) {
    }

    // Java: double anovaFValue(final Collection<double[]> categoryData)
    pub fn anovaFValue(&self) -> f64 {
        todo!()
    }

    // Java: double anovaPValue(final Collection<double[]> categoryData)
    pub fn anovaPValue(&self) -> f64 {
        todo!()
    }

    // Java: double anovaPValue()
    pub fn anovaPValue_2(&self) -> f64 {
        todo!()
    }

    // Java: AnovaStats anovaStats(final Collection<double[]> categoryData)
    pub fn anovaStats(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: boolean anovaTest()
    pub fn anovaTest(&self) -> bool {
        todo!()
    }

    // Java: AnovaStats anovaStats()
    pub fn anovaStats_2(&self) -> AnovaStats {
        todo!()
    }

}

// Java: public class AnovaStats
impl AnovaStats {
    // Java: AnovaStats(int dfbg, int dfwg, double F)
    pub fn AnovaStats(&self) {
            // Java: this.dfbg = dfbg;
            // Java: this.dfwg = dfwg;
            // Java: this.F = F;
    }

}
