#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// Java: import java.io.OutputStream

use crate::io::opc::PackagePart::PackagePart;

// Java: public interface PartMarshaller
pub trait PartMarshaller: Send + Sync {
    // Java: abstract boolean marshall(PackagePart var1, OutputStream var2)
    fn marshall(&self, part: &PackagePart, out: &mut dyn std::io::Write) -> bool;

    // Java: abstract void finish()
    fn finish(&self);
}
