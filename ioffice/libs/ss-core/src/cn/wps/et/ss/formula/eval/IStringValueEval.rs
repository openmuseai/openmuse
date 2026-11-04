// java public interface IStringValueEval extends IValueEval
// Rust's Java String adapter retains UTF-16 code units.
pub trait IStringValueEval: super::IValueEval {
    fn getStringValue(&self) -> Vec<u16>;
}
