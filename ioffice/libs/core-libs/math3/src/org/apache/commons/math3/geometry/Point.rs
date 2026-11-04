// Java: import java.io.Serializable

// Java: public interface represents
pub trait represents {
}

// Java: public interface Point
pub trait Point {
    // Java: abstract Space getSpace()
    fn getSpace(&self) -> Box<dyn std::any::Any>;

    // Java: abstract boolean isNaN()
    fn isNaN(&self) -> bool;

    // Java: abstract double distance(Point<S> p)
    fn distance(&self, p: &dyn std::any::Any) -> f64;

}
