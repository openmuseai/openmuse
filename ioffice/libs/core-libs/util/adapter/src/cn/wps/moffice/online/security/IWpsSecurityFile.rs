// Java: import java.io.IOException

// Java: public interface IWpsSecurityFile
pub trait IWpsSecurityFile {
    // Java: abstract void dispose()
    fn dispose(&mut self);

    // Java: abstract boolean decryptFile(String var1, String var2)
    fn decryptFile(&self, _var1: &str, _var2: &str) -> bool;

    // Java: abstract String getDoc_guid()
    fn getDoc_guid(&self) -> String;

    // Java: abstract String getEncdata()
    fn getEncdata(&self) -> String;

    // Java: abstract String getDataSign()
    fn getDataSign(&self) -> String;

    // Java: abstract boolean isWpsSecurityFile()
    fn isWpsSecurityFile(&self) -> bool;

    // Java: abstract int getVersionMajor()
    fn getVersionMajor(&self) -> i32;

    // Java: abstract int getVersionMinor()
    fn getVersionMinor(&self) -> i32;

}
