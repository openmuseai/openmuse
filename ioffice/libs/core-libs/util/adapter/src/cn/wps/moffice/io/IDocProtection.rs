// Java: public interface IDocProtection
pub trait IDocProtection {
    // Java: abstract boolean isReadOnlyProtection()
    fn isReadOnlyProtection(&self) -> bool;

    // Java: abstract boolean isFormProtection()
    fn isFormProtection(&self) -> bool;

    // Java: abstract boolean isCommentProtection()
    fn isCommentProtection(&self) -> bool;

    // Java: abstract boolean isTrackedchangesProtection()
    fn isTrackedchangesProtection(&self) -> bool;

    // Java: abstract boolean isProtectOnEdit()
    fn isProtectOnEdit(&self) -> bool;

}
