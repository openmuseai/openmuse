#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.StreamHelper
// Java: import cn.wps.io.opc.internal.ZipHelper
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import cn.wps.moffice.kfs.zip.ZipOutputStream
// Java: import java.io.IOException
// Java: import java.io.OutputStream

use crate::io::opc::PackagePart::PackagePart;
use super::PackagePropertiesMarshaller::PackagePropertiesMarshaller;

// Java: public class ZipPackagePropertiesMarshaller extends PackagePropertiesMarshaller
pub struct ZipPackagePropertiesMarshaller {
    pub(crate) base: PackagePropertiesMarshaller,
}

// Java: public class ZipPackagePropertiesMarshaller extends PackagePropertiesMarshaller
impl ZipPackagePropertiesMarshaller {
    // Java: ZipPackagePropertiesMarshaller()
    pub fn new() -> Self {
        // Java: super();
        ZipPackagePropertiesMarshaller {
            base: PackagePropertiesMarshaller::new(),
        }
    }

    // Java: boolean marshall(PackagePart part, OutputStream out)
    pub fn marshall(&mut self, _part: &PackagePart, _out: &mut dyn std::io::Write) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（写侧 marshaller：kfs `ZipOutputStream` / `ZipEntry` 未接入本 crate ⇒ 方法体整体暂缓）
         * 以下为 Java 方法体模板原文，逐条语句一行 `// Java:`（未删除、未改写）：
        // Java: if (!(out instanceof ZipOutputStream))
        // Java: throw new IllegalArgumentException("ZipOutputStream expected!");
        // Java: else
        // Java: ZipOutputStream zos = (ZipOutputStream)out;
        // Java: ZipEntry ctEntry = new ZipEntry(ZipHelper.getZipItemNameFromOPCName(part.getPartName().getURI().toString()));
        // Java: try
        // Java: zos.putNextEntry(ctEntry);
        // Java: super.marshall(part, out);
        // Java: if (!StreamHelper.saveXmlInStream(this.xmlDoc, out))
        // Java: this.finish();
        // Java: return false;
        // Java: else
        // Java: zos.closeEntry();
        // Java: this.finish();
        // Java: return true;
        // Java: catch (IOException e)
        // Java: this.finish();
        // Java: throw new OpenXML4JException(e.getLocalizedMessage());
        */
    }
}


// NOTE(impl-trait): public class ZipPackagePropertiesMarshaller extends PackagePropertiesMarshaller（implements PartMarshaller）
impl crate::io::opc::internal::PartMarshaller::PartMarshaller for ZipPackagePropertiesMarshaller {
    // NOTE(impl-trait): boolean marshall(PackagePart part, OutputStream out)
    // &self trait 接口对应 Java 多态（无 mut）；方法体当前为壳（kfs ZipOutputStream 缺失）
    fn marshall(&self, _part: &crate::io::opc::PackagePart::PackagePart, _out: &mut dyn std::io::Write) -> bool {
        todo!()
    }
    // NOTE(impl-trait): void finish()
    fn finish(&self) {
    }
}
