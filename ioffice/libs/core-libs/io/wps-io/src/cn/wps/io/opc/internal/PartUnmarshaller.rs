#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.internal.unmarshallers.UnmarshallContext
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import java.io.IOException
// Java: import java.io.InputStream

use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::internal::unmarshallers::UnmarshallContext::UnmarshallContext;

// Java: public interface PartUnmarshaller
pub trait PartUnmarshaller: Send + Sync {
    // Java: abstract PackagePart unmarshall(UnmarshallContext var1, InputStream var2)
    fn unmarshall(&self, context: &UnmarshallContext, input: Option<&mut dyn std::io::Read>) -> Result<Option<PackagePart>, String>;
    // rust: Java 返回 PackagePart（可 null，多态子类）；Rust 组合模式降级为基类值，Option 表达可 null
}
