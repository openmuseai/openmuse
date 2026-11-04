// Java: import cn.wps.moffice.kfs.File

// Java: public interface IFileOp
pub trait IFileOp {
    // Java: abstract boolean moveFile(File var1, File var2)
    fn moveFile(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any) -> bool;

    // Java: abstract boolean deleteFile(File var1)
    fn deleteFile(&self, _var1: &dyn std::any::Any) -> bool;

}
