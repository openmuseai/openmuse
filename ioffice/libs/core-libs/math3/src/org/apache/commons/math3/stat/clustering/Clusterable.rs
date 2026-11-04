// Java: import java.util.Collection

// Java: public interface Clusterable
pub trait Clusterable {
    // Java: abstract double distanceFrom(T p)
    fn distanceFrom(&self) -> f64;

    // Java: abstract T centroidOf(Collection<T> p)
    fn centroidOf(&self) -> Box<dyn std::any::Any>;

}
