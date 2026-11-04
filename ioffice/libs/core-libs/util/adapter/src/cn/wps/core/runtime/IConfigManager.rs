// Java: public interface IConfigManager
pub trait IConfigManager {
    // Java: abstract boolean isRtl()
    fn isRtl(&self) -> bool;

    // Java: abstract boolean isOverseaVersion()
    fn isOverseaVersion(&self) -> bool;

    // Java: abstract boolean isDebugLogVersion()
    fn isDebugLogVersion(&self) -> bool;

    // Java: abstract boolean shouldCompatSAF()
    fn shouldCompatSAF(&self) -> bool;

}
