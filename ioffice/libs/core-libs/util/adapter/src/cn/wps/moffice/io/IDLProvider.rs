// Java: public interface IDLProvider
pub trait IDLProvider {
    // Java: abstract void setOpenForceQuit(boolean var1)
    fn setOpenForceQuit(&mut self, var1: bool);

    // Java: abstract boolean isOpenForceQuit()
    fn isOpenForceQuit(&self) -> bool;

    // Java: abstract boolean isOpenFirstPageForceQuit()
    fn isOpenFirstPageForceQuit(&self) -> bool;

    // Java: abstract Integer getReadingLimit()
    fn getReadingLimit(&self) -> Option<i32>;

    // Java: abstract String getNotePath()
    fn getNotePath(&self) -> String;

    // Java: abstract String getWritePassword()
    fn getWritePassword(&self) -> String;

    // Java: abstract boolean isOpenOnReadOnly()
    fn isOpenOnReadOnly(&self) -> bool;

}
