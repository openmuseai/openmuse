#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.moffice.kfs.zip.ZipInputStream
// Java: import java.util.concurrent.Callable

// Java: public class ZipEntryCallable implements Callable<Void>
pub struct ZipEntryCallable {
    // Java field: ZipInputStream mInp
    mInp: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class ZipEntryCallable implements Callable<Void>
impl ZipEntryCallable {
    // Java: ZipEntryCallable(ZipInputStream inp)
    pub fn new(inp: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super();
        // Java: this.mInp = inp;
        ZipEntryCallable {
            mInp: Some(inp),
        }
    }

    // Java: Void call()
    pub fn call(&self) {
        // Java: this.mInp.closeEntry();
        // Java: return null;
        todo!()
    }
}
