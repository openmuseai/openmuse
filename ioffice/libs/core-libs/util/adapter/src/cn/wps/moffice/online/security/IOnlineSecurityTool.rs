// Java: public interface IOnlineSecurityTool
pub trait IOnlineSecurityTool {
    // Java: abstract boolean isWpsSecurityFile(String var1)
    fn isWpsSecurityFile(&self, _var1: &str) -> bool;

    // Java: abstract String getGuid()
    fn getGuid(&self) -> String;

    // Java: abstract boolean isEnable()
    fn isEnable(&self) -> bool;

    // Java: abstract boolean canEdit()
    fn canEdit(&self) -> bool;

    // Java: abstract boolean canCopy()
    fn canCopy(&self) -> bool;

    // Java: abstract boolean canSaveas()
    fn canSaveas(&self) -> bool;

    // Java: abstract boolean canPrint()
    fn canPrint(&self) -> bool;

    // Java: abstract boolean canAuthorize()
    fn canAuthorize(&self) -> bool;

}
