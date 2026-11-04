#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
// Java: import cn.wps.moffice.kfs.mfs.MfsException
use crate::kfs::mfs::MfsException::*;
// Java: import cn.wps.moffice.kfs.mfs.core.iface.IMfsFile
use crate::kfs::mfs::core::iface::IMfsFile::*;

// Java: public interface IMfsDisk
pub trait IMfsDisk {
    // Java: IMfsFile newMfsFile(String var1, boolean var2, boolean var3) throws MfsException;
    fn newMfsFile(&mut self, var1: String, var2: bool, var3: bool) -> Box<dyn IMfsFile>;

    // Java: IMfsFile getMfsFile(String var1);
    fn getMfsFile(&mut self, var1: String) -> Box<dyn IMfsFile>;

    // Java: boolean renameFile(String var1, String var2);
    fn renameFile(&mut self, var1: String, var2: String) -> bool;
}
