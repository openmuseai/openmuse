// Java: import cn.wps.moffice.kfs.FileInputStream
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::FileInputStream;
// Java: import cn.wps.moffice.kfs.FileOutputStream
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::FileOutputStream;
// Java: import java.io.FileDescriptor

// Java: public interface IAssetFileDescriptor
pub trait IAssetFileDescriptor {
    // Java: abstract void close()
    fn close(&self);

    // Java: abstract FileInputStream createInputStream()
    fn createInputStream(&self) -> Box<dyn std::any::Any>;

    // Java: abstract FileOutputStream createOutputStream()
    fn createOutputStream(&self) -> Box<dyn std::any::Any>;

    // Java: abstract FileDescriptor getFileDescriptor()
    fn getFileDescriptor(&self) -> Box<dyn std::any::Any>;

    // Java: abstract String getPath()
    fn getPath(&self) -> String;

}
