use std::any::Any;

// Java: public interface CipherParameters
pub trait CipherParameters: Any {
    fn as_any(&self) -> &dyn Any;
}
