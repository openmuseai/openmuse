// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.MathInternalError
use crate::org::apache::commons::math3::exception::MathInternalError;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;

// Java: public class Characterization extends Space
pub struct Characterization {
    // Java field: SubHyperplane<S> outsideTouching
    pub(crate) outsideTouching: Box<dyn std::any::Any>,
    // Java field: SubHyperplane<S> insideTouching
    pub(crate) insideTouching: Box<dyn std::any::Any>,
    // Java field: NodesSet<S> outsideSplitters
    pub(crate) outsideSplitters: Box<dyn std::any::Any>,
    // Java field: NodesSet<S> insideSplitters
    pub(crate) insideSplitters: Box<dyn std::any::Any>,
}

// Java: public class Characterization extends Space
impl Characterization {
    // Java: void characterize()
    pub fn characterize(&self) {
    }

    // Java: void addOutsideTouching()
    pub fn addOutsideTouching(&self) {
    }

    // Java: void addInsideTouching()
    pub fn addInsideTouching(&self) {
    }

    // Java: boolean touchOutside()
    pub fn touchOutside(&self) -> bool {
        // Java: return outsideTouching != null && !outsideTouching.isEmpty();
        todo!()
    }

    // Java: SubHyperplane<S> outsideTouching()
    pub fn outsideTouching(&self) -> Box<dyn std::any::Any> {
        // Java: return outsideTouching;
        todo!()
    }

    // Java: NodesSet<S> getOutsideSplitters()
    pub fn getOutsideSplitters(&self) -> Box<dyn std::any::Any> {
        // Java: return outsideSplitters;
        todo!()
    }

    // Java: boolean touchInside()
    pub fn touchInside(&self) -> bool {
        // Java: return insideTouching != null && !insideTouching.isEmpty();
        todo!()
    }

    // Java: SubHyperplane<S> insideTouching()
    pub fn insideTouching(&self) -> Box<dyn std::any::Any> {
        // Java: return insideTouching;
        todo!()
    }

    // Java: NodesSet<S> getInsideSplitters()
    pub fn getInsideSplitters(&self) -> Box<dyn std::any::Any> {
        // Java: return insideSplitters;
        todo!()
    }

}
