#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.io.IOException
// Java: import java.io.ObjectInputStream
// Java: import java.io.Serializable
// Java: import java.text.DecimalFormat
// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.Collection
// Java: import java.util.Collections
// Java: import java.util.List
// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.analysis.interpolation.LinearInterpolator
use crate::org::apache::commons::math3::analysis::interpolation::LinearInterpolator;
// Java: import org.apache.commons.math3.analysis.interpolation.NevilleInterpolator
use crate::org::apache::commons::math3::analysis::interpolation::NevilleInterpolator;
// Java: import org.apache.commons.math3.analysis.interpolation.UnivariateInterpolator
use crate::org::apache::commons::math3::analysis::interpolation::UnivariateInterpolator;
// Java: import org.apache.commons.math3.exception.InsufficientDataException
use crate::org::apache::commons::math3::exception::InsufficientDataException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.stat.descriptive.AbstractStorelessUnivariateStatistic
use crate::org::apache::commons::math3::stat::descriptive::AbstractStorelessUnivariateStatistic;
// Java: import org.apache.commons.math3.stat.descriptive.StorelessUnivariateStatistic
use crate::org::apache::commons::math3::stat::descriptive::StorelessUnivariateStatistic;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

use std::any::Any;

// Java field: int PSQUARE_CONSTANT
pub(crate) const PSQUARE_CONSTANT: i32 = 5;
// Java field: double DEFAULT_QUANTILE_DESIRED
pub(crate) const DEFAULT_QUANTILE_DESIRED: f64 = 50.0;
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 2283912083175715479;

// Java: public class PSquarePercentile extends AbstractStorelessUnivariateStatistic
pub struct PSquarePercentile {
    // Java field: List<Double> initialFive
    pub(crate) initialFive: Vec<f64>,
    // Java field: double quantile
    pub(crate) quantile: f64,
    // Java field: double lastObservation
    pub(crate) lastObservation: f64,
    // Java field: PSquareMarkers markers
    pub(crate) markers: Option<Box<dyn Any>>,
    // Java field: double pValue
    pub(crate) pValue: f64,
    // Java field: long countOfObservations
    pub(crate) countOfObservations: i64,
}

// Java: private static class Markers
pub struct Markers {
    // Java field: long serialVersionUID
    // Java field: Marker[] markerArray
    pub(crate) marker_array: Vec<Marker>,
}

// Java: private static class Marker
#[derive(Clone)]
pub struct Marker {
    // Java field: long serialVersionUID
    // Java field: int index
    pub(crate) index: i32,
    // Java field: double intMarkerPosition
    pub(crate) intMarkerPosition: f64,
    // Java field: double desiredMarkerPosition
    pub(crate) desiredMarkerPosition: f64,
    // Java field: double markerHeight
    pub(crate) markerHeight: f64,
    // Java field: double desiredMarkerIncrement
    pub(crate) desiredMarkerIncrement: f64,
}

// Java: private static class FixedCapacityList
pub struct FixedCapacityList {
    pub(crate) data: Vec<f64>,
    pub(crate) capacity: usize,
}

// Java: protected interface PSquareMarkers
pub trait PSquareMarkers {
    fn processDataPoint(&mut self, input: f64) -> f64;
    fn getPercentileValue(&self) -> f64;
}

// Java: public class PSquarePercentile extends AbstractStorelessUnivariateStatistic
impl PSquarePercentile {
    // Java: PSquarePercentile(final double p)
    pub fn new(p: f64) -> Self {
        PSquarePercentile {
            initialFive: Vec::with_capacity(PSQUARE_CONSTANT as usize),
            quantile: p / 100.0,
            lastObservation: f64::NAN,
            markers: None,
            pValue: f64::NAN,
            countOfObservations: 0,
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        todo!()
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, _o: &dyn Any) -> bool {
        todo!()
    }

    // Java: void increment(final double observation)
    pub fn increment(&mut self, _observation: f64) {
        todo!()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        todo!()
    }

    // Java: long getN()
    pub fn getN(&self) -> i64 {
        // Java: return countOfObservations;
        self.countOfObservations
    }

    // Java: double quantile()
    pub fn quantile(&self) -> f64 {
        // Java: return quantile;
        self.quantile
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        todo!()
    }

    // Java: double getResult()
    pub fn getResult(&self) -> f64 {
        // Java: return pValue;
        self.pValue
    }

    // Java: StorelessUnivariateStatistic copy()
    pub fn copy(&self) -> Box<dyn Any> {
        todo!()
    }
}

// Java: private static class Markers
impl Markers {
    // Java: double processDataPoint(final double inputDataPoint)
    pub fn processDataPoint(&mut self, _inputDataPoint: f64) -> f64 {
        todo!()
    }

    // Java: double getPercentileValue()
    pub fn getPercentileValue(&self) -> f64 {
        todo!()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        todo!()
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, _o: &dyn Any) -> bool {
        todo!()
    }

    // Java: double height(final int markerIndex)
    pub fn height(&self, _markerIndex: i32) -> f64 {
        todo!()
    }

    // Java: Object clone()
    pub fn clone(&self) -> Box<dyn Any> {
        todo!()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        todo!()
    }
}

// Java: private static class Marker
impl Marker {
    // Java: String toString()
    pub fn toString(&self) -> String {
        todo!()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        todo!()
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, _o: &dyn Any) -> bool {
        todo!()
    }
}

// Java: private static class FixedCapacityList
impl FixedCapacityList {
    pub fn new(capacity: usize) -> Self {
        FixedCapacityList {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }
}
