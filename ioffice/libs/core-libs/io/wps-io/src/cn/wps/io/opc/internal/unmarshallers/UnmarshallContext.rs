#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.opc.OPCPackage
// Java: import cn.wps.io.opc.PackagePartName
// Java: import cn.wps.moffice.kfs.zip.ZipEntry

use crate::io::xwpf::util::ZipEntrySource::ZipEntryRef;
use crate::io::opc::OPCPackage::OPCPackage;
use crate::io::opc::PackagePartName::PackagePartName;

// Java: public class UnmarshallContext
pub struct UnmarshallContext {
    // Java field: OPCPackage _package
    //
    // ★2026-09-20 修正（"滥用 any，需要接入真实类型"）：由 `Option<Box<dyn std::any::Any + Send + Sync>>`
    //   改为 Java 声明的**真实类型** `OPCPackage`。★为什么是**裸指针别名**而不是 owned 值：
    //   Java 构造体 `this._package = targetPackage;`（`UnmarshallContext.java:14`）与
    //   `setPackage` 的 `this._package = container;`（`:23`）存的是**调用方的引用**，本类从不
    //   构造/关闭 `OPCPackage`；调用点 `XWPFPackage.java:200`
    //   `new UnmarshallContext(this, part.getPartName())` 传的就是调用方自己的 `this`；
    //   `getPackage()` 的结果又被当作第三方的背引用继续传（`PackagePropertiesUnmarshaller.java:54`
    //   → `PackagePart._container`，那里已是 `Option<*const OPCPackage>`）。
    //   此处的裸指针仅代表"指向调用方持有的包的别名"（单线程解析期内有效），
    //   与 `PackagePart._container`（`opc/PackagePart.rs:34`）、`PackageRelationship.source`
    //   同一既有先例；Java `null` ⟺ `None`。
    _package: Option<*const crate::io::opc::OPCPackage::OPCPackage>,
    // Java field: PackagePartName partName
    partName: Option<PackagePartName>,
    // Java field: ZipEntry zipEntry
    //   ★2026-09-20 修正：由 `Option<*const crate::io::opc::OPCPackage::OPCPackage>` 改为 Java 声明的
    //   **真实类型** `ZipEntry` 的 Rust 等价 `ZipEntryRef`（封闭枚举）
    zipEntry: Option<ZipEntryRef>,
}

// Java: public class UnmarshallContext
impl UnmarshallContext {
    // Java: UnmarshallContext(OPCPackage targetPackage, PackagePartName partName)
    pub fn UnmarshallContext(target_package: Option<*const crate::io::opc::OPCPackage::OPCPackage>, part_name: PackagePartName) -> Self {
        // Java: super();
        // Java: this._package = targetPackage;
        // Java: this.partName = partName;
        UnmarshallContext {
            _package: target_package,
            partName: Some(part_name),
            zipEntry: None,
        }
    }

    // rust: no-java（Rust 惯用别名，crate 内沿用 `new`）
    pub fn new(target_package: Option<*const crate::io::opc::OPCPackage::OPCPackage>, part_name: PackagePartName) -> Self {
        Self::UnmarshallContext(target_package, part_name)
    }

    // Java: UnmarshallContext(OPCPackage container, PackagePartName partName)  —— Java 允许 partName 为 null
    // rust: no-java（Rust 侧 `PackagePartName` 非空类型，用独立构造器表达 Java 的 `partName == null` 形态；
    //        仅供 `PackagePropertiesUnmarshaller` 的 `IllegalArgumentException("partName")` 失败路径测试，登记 §1.7.1 ⑦）
    pub fn new_with_null_part_name(target_package: Option<*const crate::io::opc::OPCPackage::OPCPackage>) -> Self {
        UnmarshallContext {
            _package: target_package,
            partName: None,
            zipEntry: None,
        }
    }

    // Java: OPCPackage getPackage()
    pub fn getPackage(&self) -> &Option<*const crate::io::opc::OPCPackage::OPCPackage> {
        // Java: return this._package;
        &self._package
    }

    // Java: void setPackage(OPCPackage container)
    pub fn setPackage(&mut self, container: Option<*const crate::io::opc::OPCPackage::OPCPackage>) {
        // Java: this._package = container;
        self._package = container;
    }

    // Java: PackagePartName getPartName()
    pub fn getPartName(&self) -> &Option<PackagePartName> {
        // Java: return this.partName;
        &self.partName
    }

    // Java: void setPartName(PackagePartName partName)
    pub fn setPartName(&mut self, part_name: PackagePartName) {
        // Java: this.partName = partName;
        self.partName = Some(part_name);
    }

    // Java: ZipEntry getZipEntry()
    //   D6=返回**借用字段**的引用（Java 返回字段引用，不是拷贝）；★2026-09-20：字段已改真实类型
    //   `ZipEntryRef`，故此处不再回传被抹掉类型的 `&dyn Any` 壳 —— 调用方现在能直接调 `getName()`。
    pub fn getZipEntry(&self) -> &Option<ZipEntryRef> {
        // Java: return this.zipEntry;
        &self.zipEntry
    }

    // Java: void setZipEntry(ZipEntry zipEntry)
    pub fn setZipEntry(&mut self, zip_entry: Option<ZipEntryRef>) {
        // Java: this.zipEntry = zipEntry;
        self.zipEntry = zip_entry;
    }
}
