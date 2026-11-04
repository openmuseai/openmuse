// Java: import java.util.Collection

// Java: public interface enables
pub trait enables {
}

// Java: public interface Parameterizable
pub trait Parameterizable {
    // Java: abstract Collection<String> getParametersNames()
    fn getParametersNames(&self) -> Vec<Box<dyn std::any::Any>>;

    // Java: abstract boolean isSupported(String name)
    fn isSupported(&self) -> bool;

}
