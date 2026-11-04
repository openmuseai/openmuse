// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;

// Java: public interface represents
pub trait represents {
}

// Java: public interface is
pub trait is {
}

// Java: public interface SubHyperplane
pub trait SubHyperplane {
    // Java: abstract SubHyperplane<S> copySelf()
    fn copySelf(&self) -> Box<dyn std::any::Any>;

    // Java: abstract Hyperplane<S> getHyperplane()
    fn getHyperplane(&self) -> Box<dyn std::any::Any>;

    // Java: abstract boolean isEmpty()
    fn isEmpty(&self) -> bool;

    // Java: abstract double getSize()
    fn getSize(&self) -> f64;

    // Java: abstract Side side(Hyperplane<S> hyperplane)
    fn side(&self) -> Box<dyn std::any::Any>;

    // Java: abstract SplitSubHyperplane<S> split(Hyperplane<S> hyperplane)
    fn split(&self) -> Box<dyn std::any::Any>;

    // Java: abstract SubHyperplane<S> reunite(SubHyperplane<S> other)
    fn reunite(&self) -> Box<dyn std::any::Any>;

}

// Java: public class SplitSubHyperplane
pub struct SplitSubHyperplane {
    // Java field: SubHyperplane<U> plus
    pub(crate) plus: Box<dyn std::any::Any>,
    // Java field: SubHyperplane<U> minus
    pub(crate) minus: Box<dyn std::any::Any>,
}

// Java: public class SplitSubHyperplane
impl SplitSubHyperplane {
    // Java: SplitSubHyperplane()
    pub fn SplitSubHyperplane(&self) {
        // Java: this.plus  = plus;
        // Java: this.minus = minus;
    }

    // Java: SubHyperplane<U> getPlus()
    pub fn getPlus(&self) -> Box<dyn std::any::Any> {
        // Java: return plus;
        todo!()
    }

    // Java: SubHyperplane<U> getMinus()
    pub fn getMinus(&self) -> Box<dyn std::any::Any> {
        // Java: return minus;
        todo!()
    }

    // Java: Side getSide()
    pub fn getSide(&self) -> Box<dyn std::any::Any> {
        // Java: if (plus != null && !plus.isEmpty())
        {
            // Java: if (minus != null && !minus.isEmpty())
            {
                // Java: return Side.BOTH;
            }
            // Java: else
            {
                // Java: return Side.PLUS;
            }
        }
        // Java: else if (minus != null && !minus.isEmpty())
        {
            // Java: return Side.MINUS;
        }
        // Java: else
        {
            // Java: return Side.HYPER;
        }
        todo!()
    }
}
