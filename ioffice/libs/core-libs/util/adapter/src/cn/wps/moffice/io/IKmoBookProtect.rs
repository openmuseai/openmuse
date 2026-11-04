// Java: public interface IKmoBookProtect
pub trait IKmoBookProtect {
    // Java: abstract boolean isDefaultBookProtect()
    fn isDefaultBookProtect(&self) -> bool;

    // Java: abstract boolean getLockWindow()
    fn getLockWindow(&self) -> bool;

    // Java: abstract boolean getLockStructure()
    fn getLockStructure(&self) -> bool;

    // Java: abstract boolean checkPassword(String var1)
    fn checkPassword(&self, var1: Option<&str>) -> bool;

    // Java: abstract void setPassword(String var1)
    fn setPassword_from_str(&self, var1: Option<&str>);

    // Java: abstract void setPassword(int var1)
    fn setPassword(&self, var1: i32);

    // Java: abstract int getPassword()
    fn getPassword(&self) -> i32;

    // Java: abstract boolean isBookProtected()
    fn isBookProtected(&self) -> bool;

    // Java: abstract void cancelBookProtected()
    fn cancelBookProtected(&self);

    // Java: abstract boolean isCorrectPassword(String var1)
    fn isCorrectPassword(&self, var1: Option<&str>) -> bool;

    // Java: abstract boolean isLockRevion()
    fn isLockRevion(&self) -> bool;

    // Java: abstract void setLockRevion(boolean var1)
    fn setLockRevion(&self, var1: bool);

    // Java: abstract int getRevisionPassword()
    fn getRevisionPassword(&self) -> i32;

    // Java: abstract void setRevisionPassword(int var1)
    fn setRevisionPassword(&self, var1: i32);

    // Java: abstract boolean isCorrectRevisionPassword(String var1)
    fn isCorrectRevisionPassword(&self, var1: Option<&str>) -> bool;

}
