// Java: import java.util.List
// Java: import java.util.ArrayList
// Java: import org.apache.commons.math3.random.UnitSphereRandomVectorGenerator
use crate::org::apache::commons::math3::random::UnitSphereRandomVectorGenerator;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;

// Java: public class for
pub struct r#for {
}

// Java: public class InterpolatingMicrosphere
pub struct InterpolatingMicrosphere {
    // Java field: List<Facet> microsphere
    microsphere: Vec<Facet>,
    // Java field: List<FacetData> microsphereData
    microsphereData: Vec<FacetData>,
    // Java field: int dimension
    pub(crate) dimension: i32,
    // Java field: int size
    pub(crate) size: i32,
    // Java field: double maxDarkFraction
    maxDarkFraction: f64,
    // Java field: double darkThreshold
    darkThreshold: f64,
    // Java field: double background
    background: f64,
}

// Java: public class Facet
pub struct Facet {
    // Java field: double[] normal
    pub(crate) normal: Box<dyn std::any::Any>,
}

// Java: public class FacetData
pub struct FacetData {
    // Java field: double illumination
    pub(crate) illumination: f64,
    // Java field: double sample
    pub(crate) sample: f64,
}

// Java: public class for
impl r#for {
}

// Java: public class InterpolatingMicrosphere
impl InterpolatingMicrosphere {
    // Java: InterpolatingMicrosphere()
    pub fn InterpolatingMicrosphere(&self) {
    }

    // Java: InterpolatingMicrosphere()
    pub fn InterpolatingMicrosphere_2(&self) {
    }

    // Java: InterpolatingMicrosphere(InterpolatingMicrosphere other)
    pub fn InterpolatingMicrosphere_3(&self, other: &InterpolatingMicrosphere) {
        // Java: dimension = other.dimension;
        // Java: size = other.size;
        // Java: maxDarkFraction = other.maxDarkFraction;
        // Java: darkThreshold = other.darkThreshold;
        // Java: background = other.background;
        // Java: // Field can be shared.
        // Java: microsphere = other.microsphere;
        // Java: // Field must be copied.
        // Java: microsphereData = new ArrayList<FacetData>(size);
        // Java: for (FacetData fd : other.microsphereData)
        {
            // Java: microsphereData.add(new FacetData(fd.illumination(), fd.sample()));
        }
    }

    // Java: InterpolatingMicrosphere copy()
    pub fn copy(&self) -> InterpolatingMicrosphere {
        // Java: return new InterpolatingMicrosphere(this);
        todo!()
    }

    // Java: int getDimension()
    pub fn getDimension(&self) -> i32 {
        // Java: return dimension;
        todo!()
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return size;
        todo!()
    }

    // Java: double value()
    pub fn value(&self) -> f64 {
        todo!()
    }

    // Java: void add()
    pub fn add(&self) {
    }

    // Java: double interpolate()
    pub fn interpolate(&self) -> f64 {
        // Java: // Number of non-illuminated facets.
        // Java: int darkCount = 0;
        // Java: double value = 0;
        // Java: double totalWeight = 0;
        // Java: for (FacetData fd : microsphereData)
        {
            // Java: final double iV = fd.illumination();
            // Java: if (iV != 0d)
            {
                // Java: value += iV * fd.sample();
                // Java: totalWeight += iV;
            }
            // Java: else
            {
                // Java: ++darkCount;
            }
        }
        // Java: final double darkFraction = darkCount / (double) size;
        // Java: return darkFraction <= maxDarkFraction ?
            // Java: value / totalWeight :
            // Java: background;
        todo!()
    }

    // Java: void illuminate()
    pub fn illuminate(&self) {
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: for (int i = 0; i < size; i++)
        {
            // Java: microsphereData.set(i, new FacetData(0d, 0d));
        }
    }

}

// Java: public class Facet
impl Facet {
    // Java: double[] getNormal()
    pub fn getNormal(&self) -> Vec<f64> {
            // Java: return normal;
        todo!()
    }

}

// Java: public class FacetData
impl FacetData {
    // Java: double illumination()
    pub fn illumination(&self) -> f64 {
            // Java: return illumination;
        todo!()
    }

    // Java: double sample()
    pub fn sample(&self) -> f64 {
            // Java: return sample;
        todo!()
    }

}
