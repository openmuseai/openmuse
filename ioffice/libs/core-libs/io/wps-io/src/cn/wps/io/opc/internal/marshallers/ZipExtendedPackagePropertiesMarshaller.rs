#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.StreamHelper
// Java: import cn.wps.io.opc.internal.ZipHelper
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import cn.wps.moffice.kfs.zip.ZipOutputStream
// Java: import java.io.IOException
// Java: import java.io.OutputStream

use crate::io::opc::PackagePart::PackagePart;
use super::ExtendedPackagePropertiesMarshaller::ExtendedPackagePropertiesMarshaller;

// Java: public class ZipExtendedPackagePropertiesMarshaller extends ExtendedPackagePropertiesMarshaller
pub struct ZipExtendedPackagePropertiesMarshaller {
    pub(crate) base: ExtendedPackagePropertiesMarshaller,
}

// Java: public class ZipExtendedPackagePropertiesMarshaller extends ExtendedPackagePropertiesMarshaller
impl ZipExtendedPackagePropertiesMarshaller {
    // Java: ZipExtendedPackagePropertiesMarshaller()
    pub fn new() -> Self {
        // Java: super();
        ZipExtendedPackagePropertiesMarshaller {
            base: ExtendedPackagePropertiesMarshaller::new(),
        }
    }

    // Java: boolean marshall(PackagePart part, OutputStream out)
    pub fn marshall(&mut self, _part: &PackagePart, _out: &mut dyn std::io::Write) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（写侧 marshaller：kfs `ZipOutputStream` / `ZipEntry` 未接入本 crate ⇒ 方法体整体暂缓）
         * 以下为 Java 方法体模板原文，逐条语句一行（未删除、未改写）：
        // Java: if (!(out instanceof ZipOutputStream))
        // Java: throw new IllegalArgumentException("ZipOutputStream expected!");
        // Java: else
        // Java: IOAssert.assertNotNull("part should not be null!", part);
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
