#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.internal.PartMarshaller
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// Java: import java.io.OutputStream

use crate::io::opc::PackagePart::PackagePart;

// Java: public class DefaultMarshaller implements PartMarshaller
pub struct DefaultMarshaller;

// Java: public class DefaultMarshaller implements PartMarshaller
impl DefaultMarshaller {
    // Java: DefaultMarshaller()
    pub fn new() -> Self {
        // Java: super();
        DefaultMarshaller
    }

    // Java: boolean marshall(PackagePart part, OutputStream out)
    pub fn marshall(&self, _part: &PackagePart, _out: &mut dyn std::io::Write) -> bool {
        // Java: return part.save(out);
        todo!()
    }

    // Java: void finish()
    pub fn finish(&self) {
    }
}


// NOTE(impl-trait): public class DefaultMarshaller implements PartMarshaller
impl crate::io::opc::internal::PartMarshaller::PartMarshaller for DefaultMarshaller {
    // NOTE(impl-trait): boolean marshall(PackagePart part, OutputStream out)
    fn marshall(&self, part: &crate::io::opc::PackagePart::PackagePart, out: &mut dyn std::io::Write) -> bool {
        DefaultMarshaller::marshall(self, part, out)
    }
    // NOTE(impl-trait): void finish()
    fn finish(&self) {
        DefaultMarshaller::finish(self)
    }
}
