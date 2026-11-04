// Java: public interface CipherParameters
pub trait CipherParameters {
    fn as_any(&self) -> &dyn std::any::Any;
}
