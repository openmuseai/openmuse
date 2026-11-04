#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
// Java: import cn.wps.moffice.kfs.mfs.core.iface.IMfsDisk
use crate::kfs::mfs::core::iface::IMfsDisk::*;

// Java: public interface IMfsDiskManager
pub trait IMfsDiskManager {
    // Java: IMfsDisk getMfsDisk(String var1);
    fn getMfsDisk(&mut self, var1: String) -> Box<dyn IMfsDisk>;

    // Java: void install(Object var1);
    fn install(&mut self, var1: Box<dyn std::any::Any>);

    // Java: void preInstall(Object var1, String var2, Runnable var3);
    fn preInstall(&mut self, var1: Box<dyn std::any::Any>, var2: String, var3: Box<dyn Fn()>);
}
