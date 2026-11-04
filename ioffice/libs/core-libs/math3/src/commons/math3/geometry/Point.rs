// Java: import java.io.Serializable
// TODO_RUST_CODE use ???; // java.io.Serializable

// Java: public interface represents
pub trait represents {
}

// Java: public interface Point
pub trait Point {
    // Java: abstract Space getSpace()
    fn getSpace(&self) -> Space;

    // Java: abstract boolean isNaN()
    fn isNaN(&self) -> bool;

    // Java: abstract double distance(Point<S> p)
    fn distance(&self, Point<S> p) -> f64;

}
