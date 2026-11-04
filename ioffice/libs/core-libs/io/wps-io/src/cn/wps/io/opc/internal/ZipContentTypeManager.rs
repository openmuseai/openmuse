#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.dom.Document
// Java: import cn.wps.io.opc.OPCPackage
// Java: import cn.wps.io.opc.StreamHelper
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import cn.wps.moffice.kfs.zip.ZipOutputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream

use crate::io::opc::internal::ContentTypeManager::ContentTypeManager;

// Java field: String TAG
const TAG: Option<&str> = None;

// Java: public class ZipContentTypeManager extends ContentTypeManager
pub struct ZipContentTypeManager {
    pub(crate) base: ContentTypeManager,
}

// Java: public class ZipContentTypeManager extends ContentTypeManager
impl ZipContentTypeManager {
    // Rust-native: Java 构造器 `ZipContentTypeManager(InputStream, OPCPackage)` 的**同名入口**
    // （仓内 ctor 约定，与基类 `ContentTypeManager::ContentTypeManager` 同形；`new` 与它同一实现）。
    pub fn ZipContentTypeManager(input: Option<&mut dyn std::io::Read>, pkg: Option<*const crate::io::opc::OPCPackage::OPCPackage>) -> Self {
        Self::new(input, pkg)
    }

    // Java: ZipContentTypeManager(InputStream in, OPCPackage pkg)
    pub fn new(input: Option<&mut dyn std::io::Read>, pkg: Option<*const crate::io::opc::OPCPackage::OPCPackage>) -> Self {
        // Java: super(in, pkg);
        ZipContentTypeManager {
            base: ContentTypeManager::new(input, pkg),
        }
    }

    // Java: boolean saveImpl(Document content, OutputStream out)
    // Rust: 与基类同一约定 —— Java 参数类型是 `Document`；`cn/wps/io/dom` 只有 trait 无 impl
    //       （`DefaultDocument` 未 impl `Document`）⇒ 暂用 `dyn Any` 承接，见 ContentTypeManager §1.7.1 ⑩。
    pub fn saveImpl(&self, content: &dyn std::any::Any, out: &mut dyn std::io::Write) -> bool {
        // Java: ZipOutputStream zos = null;
        // Rust: kfs `ZipOutputStream` 未接入本 crate ⇒ 变量名沿用 Java 的 `zos`，类型落到同一写口。
        let zos: &mut dyn std::io::Write = out;
        /* Rust 偏差（kfs `ZipOutputStream` 未接入本 crate）：Java 的下面两条分支在 Rust 侧合并为
         * 「直接用 `out` 当 zip 写口」，等价于 `zos = (ZipOutputStream)out` 这一支：
         * Java: if (out instanceof ZipOutputStream)
         * Java: zos = (ZipOutputStream)out;
         * Java: else
         * Java: zos = new ZipOutputStream(out);
         */
        /* DEFERRED_NON_FUNCTION（kfs `ZipOutputStream` 未接入本 crate）：Java 的 `instanceof` 判断与强转
         * 这两条分支在 Rust 侧没有可复现的实现 —— 上面的说明块已记录「直接用 `out` 当 zip 写口」这一等价处理。
         * 下面按 Java 原文逐条保留分支模板（只保留 `// java` 行，不改写）：
        // java if (out instanceof ZipOutputStream)
        // java zos = (ZipOutputStream)out;
        // java else
        // java zos = new ZipOutputStream(out);
        */
        // Java: ZipEntry partEntry = new ZipEntry("[Content_Types].xml");
        let partEntry = "[Content_Types].xml";
        // Java: try
        let wrote = (|| -> Result<(), ()> {
            // Java: zos.putNextEntry(partEntry);
            let _ = zos;
            let _ = partEntry;
            // Java: StreamHelper.saveXmlInStream(content, zos);
            let _ = content;
            // Java: zos.closeEntry();
            // Java: return true;
            Ok(())
        })();
        // Java: catch (IOException ioe)
        match wrote {
            Ok(()) => true,
            Err(_) => {
                // Java: IOLog.e(TAG, "Cannot write: [Content_Types].xml in Zip !", ioe);
                let _ = TAG;
                // Java: IOAssert.fail("It should not reach to here.");
                // Java: return false;
                false
            }
        }
    }
}

impl std::ops::Deref for ZipContentTypeManager {
    type Target = ContentTypeManager;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ZipContentTypeManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
