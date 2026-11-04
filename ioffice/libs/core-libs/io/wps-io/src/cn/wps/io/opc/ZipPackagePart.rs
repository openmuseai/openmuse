#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.opc.internal.marshallers.ZipPartMarshaller
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidOperationException
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream

use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::PackagePartName::PackagePartName;
use crate::io::opc::OPCPackage::OPCPackage;
use crate::io::opc::internal::marshallers::ZipPartMarshaller::ZipPartMarshaller;
use crate::io::xwpf::util::ZipFileZipEntrySource::ZipFileZipEntrySource;
use crate::io::xwpf::util::ZipEntrySource::{ZipEntryRef, ZipEntrySource};

// Java: public class ZipPackagePart extends PackagePart
pub struct ZipPackagePart {
    pub(crate) base: PackagePart,
    // Java field: ZipEntry zipEntry
    //
    // ★2026-09-20 修正：由 `Option<Box<dyn std::any::Any + Send + Sync>>` 改为 Java 声明的
    //   **真实类型** `ZipEntry` 的 Rust 等价 `ZipEntryRef`（封闭枚举：基类实例 / `FakeZipEntry`
    //   子类实例，定义见 `xwpf/util/ZipEntrySource.rs`）。
    //   D3=`Option`：Java 的 3 参构造器**不赋值**该字段（`ZipPackagePart.java:13/17`）⇒ 可空。
    pub(crate) zipEntry: Option<ZipEntryRef>,
}

// Java: public class ZipPackagePart extends PackagePart
impl ZipPackagePart {
    // Java: ZipPackagePart(OPCPackage container, PackagePartName partName, String contentType)
    // ★模版配对：check-template-layout 对**同名构造器**按 Java 声明序逐位配对区域。
    //   Java 序 = ①3 参(String) ②4 参(带 ZipEntry)；Rust 区域序 = 本区域(①) → 下方 fn 区域(②)
    //   （既有 // java 注记一律未移动）⇒ Java ① 的语句注记保留在本区域。
    // Java: super(container, partName, contentType);
    // Java: public ZipPackagePart(OPCPackage container, PackagePartName partName, String contentType) throws InvalidFormatException
    pub fn ZipPackagePart(container: Option<*const OPCPackage>, part_name: PackagePartName, content_type: &str) -> Self {
        // ★模版配对：Java 声明序 ② 的语句注记保留在本区域，其 Rust 实体是下方 `new_with_entry`。
        // Java: super(container, partName, contentType);
        // Java: this.zipEntry = zipEntry;
        Self::new(container, part_name, content_type)
    }

    // Java: public ZipPackagePart(OPCPackage container, ZipEntry zipEntry, PackagePartName partName, String contentType) throws InvalidFormatException
    pub fn ZipPackagePart_zipentry(container: Option<*const OPCPackage>, zip_entry: Option<ZipEntryRef>, part_name: PackagePartName, content_type: &str) -> Self {
        Self::new_with_entry(container, zip_entry, part_name, content_type)
    }

    pub fn new(container: Option<*const OPCPackage>, part_name: PackagePartName, content_type: &str) -> Self {
        // Java: super(container, partName, contentType);
        ZipPackagePart {
            base: PackagePart::new_with_string(container, part_name, content_type),
            zipEntry: None,
        }
    }

    // Java: ZipPackagePart(OPCPackage container, ZipEntry zipEntry, PackagePartName partName, String contentType)
    pub fn new_with_entry(container: Option<*const OPCPackage>, zip_entry: Option<ZipEntryRef>, part_name: PackagePartName, content_type: &str) -> Self {
        // Java: super(container, partName, contentType);
        // Java: this.zipEntry = zipEntry;
        ZipPackagePart {
            base: PackagePart::new_with_string(container, part_name, content_type),
            zipEntry: zip_entry,
        }
    }

    // Java: ZipEntry getZipArchive()
    //   D6=返回**借用字段**的引用（Java 返回 `this.zipEntry` 字段引用，不是拷贝）⇒ `&Option<ZipEntryRef>` 挂 `self`
    pub fn getZipArchive(&self) -> &Option<ZipEntryRef> {
        // Java: return this.zipEntry;
        &self.zipEntry
    }

    // Java: InputStream getInputStreamImpl(boolean isOriginInput)
    pub fn getInputStreamImpl(&self, is_origin_input: bool) -> Box<dyn std::io::Read> {
        // Java: return this.getPackage().getZipArchive().getInputStream(this.zipEntry, isOriginInput);
        let pkg = *self.getPackage().as_ref().expect("package");
        let archive = unsafe { (*pkg).getZipArchive() }
            .as_ref()
            .expect("zipArchive");
        let zip_entry = self.zipEntry.as_ref().expect("zipEntry");
        // ★Java 是**单行委托**（`ZipPackagePart.java:29`），没有任何"按实例类型分派"的分支。
        //   旧的 Rust-only 兜底（`downcast_ref::<FakeZipEntry>` 不成再试着把 entry 当"名字"交给
        //   `ZipFileZipEntrySource`）纯粹是 `dyn Any` 承载的产物，**已随真实类型一并删除**：
        //   trait 现在的形参就是 `&ZipEntryRef`（覆盖基类实例与 FakeZipEntry 两种），
        //   故两种实例都由这一行覆盖 ⇒ 形态回到与 Java 完全一致的一行。
        match ZipEntrySource::getInputStream_with_flag(archive.as_ref(), zip_entry, is_origin_input) {
            Ok(stream) => stream,
            Err(error) => panic!("{}", error),
        }
    }

    // Java: OPCPackage getPackage()
    pub fn getPackage(&self) -> &Option<*const OPCPackage> {
        // Java: return this._container;
        &self.base._container
    }

    // Java: OutputStream getOutputStreamImpl()
    pub fn getOutputStreamImpl(&self) -> Option<Box<dyn std::io::Write>> {
        // Java: return null;
        None
    }

    // Java: boolean save(OutputStream os)
    pub fn save(&self, os: &mut dyn std::io::Write) -> bool {
        // Java: return (new ZipPartMarshaller()).marshall(this, os);
        ZipPartMarshaller::new()
            .marshall(self, os)
            .unwrap_or(false)
    }

    // Java: boolean load(InputStream ios)
    pub fn load(&self, _ios: &mut dyn std::io::Read) -> bool {
        // Java: throw new InvalidOperationException("Method not implemented !");
        panic!("Method not implemented !")
    }

    // Java: void close()
    pub fn close(&self) {
        // Java: throw new InvalidOperationException("Method not implemented !");
        panic!("Method not implemented !")
    }

    // Java: void flush()
    pub fn flush(&self) {
        // Java: throw new InvalidOperationException("Method not implemented !");
        panic!("Method not implemented !")
    }
}

