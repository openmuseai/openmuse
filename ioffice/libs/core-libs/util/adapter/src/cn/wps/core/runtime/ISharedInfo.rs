// Java: public interface ISharedInfo
pub trait ISharedInfo {
    // Java: abstract boolean isInRomReadMode()
    fn isInRomReadMode(&self) -> bool;

    // Java: abstract boolean isProVersion()
    fn isProVersion(&self) -> bool;

    // Java: abstract boolean isShowInHWScreen(Object var1)
    fn isShowInHWScreen(&self, var1: &dyn std::any::Any) -> bool;

    // Java: abstract boolean isRecordVersion()
    fn isRecordVersion(&self) -> bool;

    // Java: abstract boolean isOemPhoneShrink()
    fn isOemPhoneShrink(&self) -> bool;

    // Java: abstract boolean isMultiWindowVersion()
    fn isMultiWindowVersion(&self) -> bool;

    // Java: abstract int[] getDisplayWidthAndHeight(Object var1)
    fn getDisplayWidthAndHeight(&self, var1: &dyn std::any::Any) -> Vec<i32>;

    // Java: abstract boolean isPadScreen(Object var1)
    fn isPadScreen(&self, var1: &dyn std::any::Any) -> bool;

    // Java: abstract boolean isDebugLogVersion()
    fn isDebugLogVersion(&self) -> bool;

    // Java: abstract boolean isOverseaVersion()
    fn isOverseaVersion(&self) -> bool;

    // Java: abstract boolean isAndroidN()
    fn isAndroidN(&self) -> bool;

    // Java: abstract boolean isParamsOn(String var1)
    fn isParamsOn(&self, var1: &str) -> bool;

    // Java: abstract boolean isLeshiDevice()
    fn isLeshiDevice(&self) -> bool;

    // Java: abstract boolean isOnlyHuaweiDevice()
    fn isOnlyHuaweiDevice(&self) -> bool;

    // Java: abstract boolean isHonorDevice()
    fn isHonorDevice(&self) -> bool;

    // Java: abstract String getDeviceBrand()
    fn getDeviceBrand(&self) -> String;

}
