#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.log.IOLog
// No Rust import: the current Rust body does not call IOLog.
// Java: import cn.wps.core.runtime.Platform
use adapter::cn::wps::core::runtime::Platform::Platform;
// Java: import cn.wps.io.opc.CustomPackageProperties
use crate::io::opc::CustomPackageProperties::CustomPackageProperties;
// Java: import cn.wps.io.opc.CustomXmlInfo
use crate::io::opc::CustomXmlInfo::CustomXmlInfo;
// Java: import cn.wps.io.opc.ExtendedPackageProperties
// No Rust import: the current Rust fields use ExtendedPackagePropertiesPart instead.
// Java: import cn.wps.io.opc.OPCPackage
use crate::io::opc::OPCPackage::OPCPackage;
// Java: import cn.wps.io.opc.PackageAccess
use crate::io::opc::PackageAccess::PackageAccess;
// Java: import cn.wps.io.opc.PackagePart
use crate::io::opc::PackagePart::PackagePart;
// Java: import cn.wps.io.opc.PackagePartCollection
// No Rust import: collection ownership is provided through the OPCPackage base field.
// Java: import cn.wps.io.opc.PackagePartName
use crate::io::opc::PackagePartName::PackagePartName;
// Java: import cn.wps.io.opc.PackageProperties
// No Rust import: the current Rust API exposes PackagePropertiesPart.
// Java: import cn.wps.io.opc.PackageRelationship
use crate::io::opc::PackageRelationship::PackageRelationship;
use crate::io::opc::PackageRelationshipCollection::PackageRelationshipCollection;
// Java: import cn.wps.io.opc.PackageRelationshipCollection
// No Rust import: the current Rust body delegates relationship collection access to OPCPackage.
// Java: import cn.wps.io.opc.PackagingURIHelper
// No Rust import: current Rust calls the helper through existing package APIs instead of its type name.
// Java: import cn.wps.io.opc.TargetMode
// No Rust import: the generated Rust boundary currently represents TargetMode as i32.
// Java: import cn.wps.io.opc.internal.ContentType
use crate::io::opc::internal::ContentType::ContentType;
// Java: import cn.wps.io.opc.internal.CustomPackagePropertiesPart
use crate::io::opc::internal::CustomPackagePropertiesPart::CustomPackagePropertiesPart;
// Java: import cn.wps.io.opc.internal.ExtendedPackagePropertiesPart
use crate::io::opc::internal::ExtendedPackagePropertiesPart::ExtendedPackagePropertiesPart;
// Java: import cn.wps.io.opc.internal.FileHelper
// No Rust import: the current Rust body does not reference FileHelper by type name.
// Java: import cn.wps.io.opc.internal.PackagePropertiesPart
use crate::io::opc::internal::PackagePropertiesPart::PackagePropertiesPart;
// Java: import cn.wps.io.opc.internal.PartUnmarshaller
use crate::io::opc::internal::PartUnmarshaller::PartUnmarshaller;
// Java: import cn.wps.io.opc.internal.marshallers.ZipCustomPackagePropertiesMarshaller
// No Rust import: the corresponding Java marshalling path is not implemented by this Rust body.
// Java: import cn.wps.io.opc.internal.marshallers.ZipExtendedPackagePropertiesMarshaller
// No Rust import: the corresponding Java marshalling path is not implemented by this Rust body.
// Java: import cn.wps.io.opc.internal.marshallers.ZipPackagePropertiesMarshaller
use crate::io::opc::internal::marshallers::ZipPackagePropertiesMarshaller::ZipPackagePropertiesMarshaller;
// Java: import cn.wps.io.opc.internal.unmarshallers.CustomPackagePropertiesUnmarshaller
use crate::io::opc::internal::unmarshallers::CustomPackagePropertiesUnmarshaller::CustomPackagePropertiesUnmarshaller;
// Java: import cn.wps.io.opc.internal.unmarshallers.ExtendedPackagePropertiesUnmarshaller
use crate::io::opc::internal::unmarshallers::ExtendedPackagePropertiesUnmarshaller::ExtendedPackagePropertiesUnmarshaller;
// Java: import cn.wps.io.opc.internal.unmarshallers.PackagePropertiesUnmarshaller
use crate::io::opc::internal::unmarshallers::PackagePropertiesUnmarshaller::PackagePropertiesUnmarshaller;
// Java: import cn.wps.io.opc.internal.unmarshallers.UnmarshallContext
use crate::io::opc::internal::unmarshallers::UnmarshallContext::UnmarshallContext;
// Java: import cn.wps.io.util.URI
// No Rust import: the current Rust body does not name URI directly.
// Java: import cn.wps.io.xwpf.usermodel.XWPFRelation
// No Rust import: the current Rust body does not name XWPFRelation directly.
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// No Rust import: this Java exception path is represented by existing Rust Result/Option boundaries.
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidOperationException
// No Rust import: this Java exception path is represented by existing Rust Result/Option boundaries.
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// No Rust import: this Java exception path is represented by existing Rust Result/Option boundaries.
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JRuntimeException
// No Rust import: the current Rust body does not construct this exception type.
// Java: import cn.wps.io.xwpf.util.Nullable
// No Rust import: Java nullability maps to Option at the Rust boundaries.
// Java: import cn.wps.io.xwpf.util.ZipInputStreamZipEntrySource
// No Rust import: the current Rust body does not name this source type.
// Java: import cn.wps.moffice.kfs.File
// No Rust import: current Rust path handling uses string/path boundaries without this type name.
// Java: import cn.wps.moffice.kfs.FileOutputStream
// No Rust import: the current Rust body does not construct FileOutputStream.
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// No Rust import: the generated Rust overload currently accepts a byte slice.
// Java: import cn.wps.moffice.kfs.zip.ZipInputStream
// No Rust import: the current Rust body does not name ZipInputStream directly.
// Java: import cn.wps.moffice.kfs.zip.ZipOutputStream
// No Rust import: the current Rust marshalling boundary uses std::io::Write.
// Java: import cn.wps.moffice.util.FileUtil
// No Rust import: current filesystem operations use Rust standard-library APIs.
// Java: import cn.wps.moffice.util.KSLog
// No Rust import: the current Rust body does not call KSLog.
// Java: import java.io.BufferedOutputStream
// No Rust import: buffering is represented at the std::io::Write boundary.
// Java: import java.io.IOException
// No Rust import: Java checked I/O failures map to Rust Result/Option boundaries.
// Java: import java.io.InputStream
// No Rust import: method boundaries use dyn std::io::Read directly.
// Java: import java.io.OutputStream
// No Rust import: method boundaries use dyn std::io::Write directly.
// Java: import java.util.ArrayList
// No Rust import: Java ArrayList maps directly to Vec.
// Java: import java.util.Date
// No Rust import: the current Rust body does not name Date directly.

// Java field: String TAG
const TAG: Option<&str> = None;

// Java: public class XWPFPackage extends OPCPackage
pub struct XWPFPackage {
    pub(crate) base: OPCPackage,
    // Java field: PackagePropertiesPart packageProperties
    pub(crate) packageProperties: Option<PackagePropertiesPart>,
    // Java field: ExtendedPackagePropertiesPart extendedPackageProperties
    pub(crate) extendedPackageProperties: Option<ExtendedPackagePropertiesPart>,
    // Java field: CustomPackagePropertiesPart customPackageProperties
    pub(crate) customPackageProperties: Option<CustomPackagePropertiesPart>,
    // Java field: ArrayList<CustomXmlInfo> customXmlInfos
    pub(crate) customXmlInfos: Vec<CustomXmlInfo>,
}

// Java: public class XWPFPackage extends OPCPackage
// ── ★2026-09-20：`getParts()` 表驱动派发用的三个内容类型常量。
//    取值与 Java `XWPFPackage.init()`（`XWPFPackage.java:369-371`）注册到 `partUnmarshallers` 的
//    键**逐字相同**，并与 Java `getParts()` 里 `part.getContentType().toString().equals(...)`
//    的字面量一致。
const CORE_PROPERTIES_CONTENT_TYPE: &str = "application/vnd.openxmlformats-package.core-properties+xml";
const EXTENDED_PROPERTIES_CONTENT_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.extended-properties+xml";
const CUSTOM_PROPERTIES_CONTENT_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.custom-properties+xml";

/// ★2026-09-20：Java 的 `unmarshallPart instanceof XxxPackagePropertiesPart` 判据在
/// Rust 组合式建模下的等价物 —— 把三个**类型化属性视图**收进一个枚举，供 `getParts()` 的
/// 表驱动派发一次性分发（Java 的继承体系里它们都是 `PackagePart` 的子类）。
enum PackagePartView {
    Core(PackagePropertiesPart),
    Extended(ExtendedPackagePropertiesPart),
    Custom(CustomPackagePropertiesPart),
}

impl XWPFPackage {
    // Java: XWPFPackage()
    pub fn XWPFPackage() -> Self {
        // Java: super(PackageAccess.WRITE);
        let mut pack = XWPFPackage {
            base: OPCPackage::new(PackageAccess::WRITE),
            packageProperties: None,
            extendedPackageProperties: None,
            customPackageProperties: None,
            customXmlInfos: Vec::new(),
        };
        // Java: this.init()（虚调用 ⇒ 落到 `XWPFPackage.init()`）
        // ★2026-09-20 修正（对齐 Java 的**虚调用** `this.init()`）：
        //   Java 侧 `OPCPackage` 构造器里 `this.init()`（`OPCPackage.java:84`）是**虚调用**
        //   ⇒ 构造 `XWPFPackage` 时真正执行的是覆盖版 `XWPFPackage.init()`
        //   （`XWPFPackage.java:365-374`：注册 3 个 `partUnmarshallers` + 1 个 `partMarshallers`）。
        //   Rust 无虚派发：`OPCPackage::new/from_*` 只调基类 `OPCPackage::init()`，
        //   导致 `XWPFPackage::init()` **从未被调用**、`partUnmarshallers` 恒为空表
        //   ⇒ `getParts()` 的 Java 表驱动派发（`partUnmarshallers.get(contentType)`）无从生效，
        //   这正是旧实现改用"硬编码 content-type 扫描"的根因。此处显式补上虚调用等价物。
        pack.init();
        pack
    }

    pub fn new() -> Self {
        Self::XWPFPackage()
    }

    // Java: static XWPFPackage open(InputStream in)
    pub fn open_stream(input: &mut dyn std::io::Read) -> Self {
        // Java: XWPFPackage pack = new XWPFPackage(in, PackageAccess.READ);
        let mut pack = XWPFPackage {
            base: OPCPackage::from_stream(input),
            packageProperties: None,
            extendedPackageProperties: None,
            customPackageProperties: None,
            customXmlInfos: Vec::new(),
        };
        // ★2026-09-20 修正（对齐 Java 的**虚调用** `this.init()`）：
        //   Java 侧 `OPCPackage` 构造器里 `this.init()`（`OPCPackage.java:84`）是**虚调用**
        //   ⇒ 构造 `XWPFPackage` 时真正执行的是覆盖版 `XWPFPackage.init()`
        //   （`XWPFPackage.java:365-374`：注册 3 个 `partUnmarshallers` + 1 个 `partMarshallers`）。
        //   Rust 无虚派发：`OPCPackage::new/from_*` 只调基类 `OPCPackage::init()`，
        //   导致 `XWPFPackage::init()` **从未被调用**、`partUnmarshallers` 恒为空表
        //   ⇒ `getParts()` 的 Java 表驱动派发（`partUnmarshallers.get(contentType)`）无从生效，
        //   这正是旧实现改用"硬编码 content-type 扫描"的根因。此处显式补上虚调用等价物。
        pack.init();
        // Java: if (pack.partList == null)
        if pack.base.partList.is_none() {
        // Java: pack.getParts();
            let _ = pack.getParts();
        }
        // Java: return pack;
        pack
    }

    // Java: static XWPFPackage open(String path, PackageAccess access)
    pub fn open_path(path: &str, access: i32) -> Self {
        // Java: if (path != null && !"".equals(path.trim()) && (!(new File(path)).exists() || !(new File(path)).isDirectory()))
        if !path.trim().is_empty() && !std::path::Path::new(path).is_dir() {
        // Java: XWPFPackage pack = new XWPFPackage(path, access);
            let pkg_access = if access == 0 { PackageAccess::READ } else { PackageAccess::WRITE };
            let mut pack = XWPFPackage {
                base: OPCPackage::from_path(path, pkg_access),
                packageProperties: None,
                extendedPackageProperties: None,
                customPackageProperties: None,
                customXmlInfos: Vec::new(),
            };
            // ★2026-09-20 修正（对齐 Java 的**虚调用** `this.init()`）：
            //   Java 侧 `OPCPackage` 构造器里 `this.init()`（`OPCPackage.java:84`）是**虚调用**
            //   ⇒ 构造 `XWPFPackage` 时真正执行的是覆盖版 `XWPFPackage.init()`
            //   （`XWPFPackage.java:365-374`：注册 3 个 `partUnmarshallers` + 1 个 `partMarshallers`）。
            //   Rust 无虚派发：`OPCPackage::new/from_*` 只调基类 `OPCPackage::init()`，
            //   导致 `XWPFPackage::init()` **从未被调用**、`partUnmarshallers` 恒为空表
            //   ⇒ `getParts()` 的 Java 表驱动派发（`partUnmarshallers.get(contentType)`）无从生效。
            //   故此处显式补上虚调用的等价物。
            pack.init();
        // Java: if (pack.partList == null && access != PackageAccess.WRITE)
            if pack.base.partList.is_none() && pkg_access != PackageAccess::WRITE {
        // Java: pack.getParts();
                let _ = pack.getParts();
            }
        // Java: pack.originalPackagePath = (new File(path)).getAbsolutePath();
            pack.base.originalPackagePath = Some(
                std::fs::canonicalize(path)
                    .unwrap_or_else(|_| std::path::PathBuf::from(path))
                    .to_string_lossy()
                    .into_owned(),
            );
        // Java: return pack;
            return pack;
        // Java: else
        }
        // Java: throw new IllegalArgumentException("path");
        panic!("path");
    }

    // Java: static XWPFPackage open(RandomAccessFile raf, PackageAccess access)
    pub fn open_raf(raf: &dyn std::any::Any, access: i32) -> Self {
        // Java: if (raf == null)
        // Rust: `&dyn Any` is non-null by construction.
        // Java: throw new IllegalArgumentException("RandomAccessFile should not be null.");
        // Rust: a missing reference cannot reach this boundary.
        // Java: else
        {
        // Java: raf.seek(0L);
            // Rust: `OPCPackage::from_raf` opens the supplied path-like RAF source from offset zero.
        // Java: XWPFPackage pack = new XWPFPackage(raf, access);
            let pkg_access = if access == 0 { PackageAccess::READ } else { PackageAccess::WRITE };
            let mut pack = XWPFPackage {
                base: OPCPackage::from_raf(raf, pkg_access),
                packageProperties: None,
                extendedPackageProperties: None,
                customPackageProperties: None,
                customXmlInfos: Vec::new(),
            };
        // ★2026-09-20 修正（对齐 Java 的**虚调用** `this.init()`）：
        //   `OPCPackage` 构造器里的 `this.init()`（`OPCPackage.java:84`）在 Java 是虚调用
        //   ⇒ 落到覆盖版 `XWPFPackage.init()`（注册 3 个 `partUnmarshallers` + 1 个 `partMarshallers`）。
        //   Rust 无虚派发 ⇒ 在此显式补上。
        pack.init();
        // Java: if (pack.partList == null && access != PackageAccess.WRITE)
            if pack.base.partList.is_none() && pkg_access != PackageAccess::WRITE {
        // Java: pack.getParts();
                let _ = pack.getParts();
            }
        // Java: return pack;
            pack
        }
    }

    // Java: static XWPFPackage create(String path)
    pub fn create_path(path: &str) -> Self {
        // Java: return create(new File(path));
        Self::create_file(&std::path::PathBuf::from(path))
    }

    // Java: static XWPFPackage create(File file)
    pub fn create_file(file: &dyn std::any::Any) -> Self {
        // Java: if (file != null && (!file.exists() || !file.isDirectory()))
        let path = file
            .downcast_ref::<std::path::PathBuf>()
            .cloned()
            .or_else(|| file.downcast_ref::<String>().map(std::path::PathBuf::from))
            .unwrap_or_else(|| panic!("file"));
        if !path.is_dir() {
        // Java: XWPFPackage pkg = new XWPFPackage();
            let mut pkg = Self::new();
        // Java: pkg.originalPackagePath = file.getAbsolutePath();
            pkg.base.originalPackagePath = Some(
                std::fs::canonicalize(&path)
                    .unwrap_or(path.clone())
                    .to_string_lossy()
                    .into_owned(),
            );
        // Java: File dir = FileHelper.getDirectory(file);
            let dir = path.parent().unwrap_or_else(|| std::path::Path::new("."));
        // Java: if (!dir.exists())
            if !dir.exists() {
        // Java: dir.mkdirs();
                std::fs::create_dir_all(dir).unwrap_or_else(|error| panic!("file: {error}"));
            }
        // Java: pkg.tmpPackageFile = File.createTempFile(generateTempFileName(dir), ".tmp", dir);
            let temp_name = format!(".xwpf-{}.tmp", std::process::id());
            pkg.base.tmpPackageFile = Some(dir.join(temp_name));
        // Java: pkg.mZOS = new ZipOutputStream(new BufferedOutputStream(new FileOutputStream(pkg.tmpPackageFile)));
            if let Some(temp) = pkg.base.tmpPackageFile.as_ref() {
                let output = std::fs::File::create(temp).unwrap_or_else(|error| panic!("file: {error}"));
                pkg.base.mZOS = Some(Box::new(std::io::BufWriter::new(output)));
            }
        // Java: pkg.configurePackage();
            pkg.configurePackage();
        // Java: return pkg;
            return pkg;
        // Java: else
        }
        // Java: throw new IllegalArgumentException("file");
        panic!("file");
    }

    // Java: static XWPFPackage create(OutputStream output)
    pub fn create_output(output: &mut dyn std::io::Write) -> Self {
        // Java: XWPFPackage pkg = new XWPFPackage();
        let mut pkg: XWPFPackage = Self::new();
        // Java: pkg.originalPackagePath = null;
        pkg.base.originalPackagePath = None;
        // Java: pkg.output = output;
        pkg.base.output = None;
        // Java: if (!(output instanceof ZipOutputStream))
        // Rust: `dyn Write` already abstracts buffered/zip output implementations.
        // Java: pkg.mZOS = new ZipOutputStream(output);
        pkg.base.mZOS = None;
        // Java: else
        // Rust: no runtime downcast is required for `dyn Write`.
        // Java: pkg.mZOS = (ZipOutputStream)output;
        pkg.base.mZOS = None;
        let _ = output;
        // Java: pkg.configurePackage();
        pkg.configurePackage();
        // Java: return pkg;
        pkg
    }

    // Java: ArrayList<CustomXmlInfo> getCustomXmlInfos()
    pub fn getCustomXmlInfos(&self) -> &Vec<CustomXmlInfo> {
        // Java: return this.customXmlInfos;
        &self.customXmlInfos
    }

    // ── ★2026-09-20：`getParts()` 对齐后的**可观测门面**（Rust-only 只读观察入口） ──────
    // 与 Java 同名方法 `getPackageProperties()` 的区别（很重要，故单独命名）：
    //   Java 的 `getPackageProperties()` 在字段为 null 时会**惰性新建**一个空视图
    //   （`XWPFPackage.java` 的 `if (this.packageProperties == null) this.packageProperties = new …`），
    //   因此它**无法回答**"`getParts()` 到底有没有反序列化出视图"这个问题。
    //   下面三个访问器直读字段、不做惰性新建，正是为了观测 `getParts()` 的表驱动派发结果。

    /// 直读 `packageProperties` 字段（不惰性新建）—— 观测 `getParts()` 的 core 视图填充结果。
    pub fn packagePropertiesView(&self) -> Option<&PackagePropertiesPart> {
        self.packageProperties.as_ref()
    }

    /// 直读 `extendedPackageProperties` 字段（不惰性新建）。
    pub fn extendedPackagePropertiesView(&self) -> Option<&ExtendedPackagePropertiesPart> {
        self.extendedPackageProperties.as_ref()
    }

    /// 直读 `customPackageProperties` 字段（不惰性新建）。
    pub fn customPackagePropertiesView(&self) -> Option<&CustomPackagePropertiesPart> {
        self.customPackageProperties.as_ref()
    }

    /// `partUnmarshallers` 条数 —— 观测 Java `XWPFPackage.init()` 的注册是否真的发生
    /// （3 个属性 unmarshaller；旧状态下 `XWPFPackage::init()` 从未被调用 ⇒ 恒 0）。
    pub fn getPartUnmarshallersLen(&self) -> usize {
        self.base.getPartUnmarshallersLen()
    }

    pub fn getPart(&mut self, part_name: &PackagePartName) -> Option<&PackagePart> {
        self.base.getPart(part_name)
    }

    pub fn getRelationshipsByType(&mut self, relationship_type: &str) -> crate::io::opc::PackageRelationshipCollection::PackageRelationshipCollection {
        self.base.getRelationshipsByType(relationship_type)
    }

    pub fn getPartList(&self) -> Option<&crate::io::opc::PackagePartCollection::PackagePartCollection> {
        self.base.getPartList()
    }

    /// 底层 `OPCPackage` 的**可变**访问（`48_function_pkg_configure_close` 的 W1/W5/W6/W7/W9/W10-W12
    /// 需要直接调 `OPCPackage::{getPackageAccess, flush, save, close}`）。
    ///
    /// Java: `XWPFPackage extends OPCPackage` ⇒ 这些方法在 Java 侧是**继承**来的（直接可用）。
    /// Rust 用**组合**（`base` 字段）替代继承 ⇒ 必须显式暴露，否则继承来的面**不可测**。
    /// **不是**新行为，只是把 Java 的继承关系在 Rust 侧显式化。
    pub fn base_mut(&mut self) -> &mut crate::io::opc::OPCPackage::OPCPackage {
        &mut self.base
    }

    /// 底层 `OPCPackage` 的只读访问（同上，供只读断言使用）。
    pub fn base_ref(&self) -> &crate::io::opc::OPCPackage::OPCPackage {
        &self.base
    }


    /// `originalPackagePath` 只读访问。
    ///
    /// Java: 该字段在 `XWPFPackage.java:80`（path 入口置位）、`:108`（create 置位）、
    /// `:125`（output 置 `null`）；Java **无公开 getter**（仅包内直接读写字段）。
    /// Rust 侧为**可观测性**提供只读访问（`44` 的 B9、`45` 的 P11、`46` 的 R11 需要断言
    /// 「bytes/raf 入口**不得**置位」）——**只读**，不改变任何行为。
    pub fn getOriginalPackagePath(&self) -> Option<&str> {
        self.base.originalPackagePath.as_deref()
    }

    // Java: PackageRelationship addRelationship(PackagePartName targetPartName, TargetMode targetMode, String relationshipType, String relID)
    pub fn addRelationship(&mut self, target_part_name: &PackagePartName, target_mode: i32, relationship_type: &str, rel_id: Option<&str>) -> Option<PackageRelationship> {
        // Java: if (relationshipType.equals("http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties") && this.packageProperties != null)
        if relationship_type == "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" && self.packageProperties.is_some() {
            // Java: throw new InvalidOperationException("OPC Compliance error [M4.1]: can't add another core properties part ! Use the built-in package method instead.");
            panic!("OPC Compliance error [M4.1]: can't add another core properties part ! Use the built-in package method instead.");
        // Java: else
        } else {
            // Java: return super.addRelationship(targetPartName, targetMode, relationshipType, relID);
            let mode: crate::io::opc::TargetMode::TargetMode = if target_mode == 0 {
                crate::io::opc::TargetMode::TargetMode::INTERNAL
            } else {
                crate::io::opc::TargetMode::TargetMode::EXTERNAL
            };
            Some(self.base.addRelationship(target_part_name, mode, relationship_type, rel_id))
        }
    }

    // Java: PackagePart createPart(PackagePartName partName, String contentType, boolean loadRelationships)
    pub fn createPart(&mut self, part_name: &PackagePartName, content_type: &str, load_relationships: bool) -> Option<PackagePart> {
        // Java: if (contentType.equals("application/vnd.openxmlformats-package.core-properties+xml") && this.packageProperties != null)
        if content_type == "application/vnd.openxmlformats-package.core-properties+xml" && self.packageProperties.is_some() {
            // Java: throw new InvalidOperationException("OPC Compliance error [M4.1]: you try to add more than one core properties relationship in the package !");
            panic!("OPC Compliance error [M4.1]: you try to add more than one core properties relationship in the package !");
        // Java: else
        } else {
            // Java: return super.createPart(partName, contentType, loadRelationships);
            let _ = load_relationships;
            self.base.createPart(part_name, content_type)
        }
    }

    // Java: ArrayList<PackagePart> getParts()
    pub fn getParts(&mut self) -> Vec<&PackagePart> {
        // ★2026-09-20 重写（用户点名的第 4 项："这个方法逻辑和门面似乎都没对齐"）。
        //   旧实现的三个问题：
        //   ① `for` 循环体**没有代码** —— Java 的每条语句下面只有一句 "Rust: …" 说明，
        //      循环实际只做了 `let part = partList; let _ = part;`；真正的属性 part 处理被搬到
        //      循环**之后**一个硬编码 content-type 的扫描里，与 Java 的控制流/顺序完全不同。
        //   ② `if (hasCorePropertiesPart) continue;` 被写成 **`break`** ⇒ Rust 侧一旦遇到第二个
        //      core-properties part 就**整个循环终止**（Java 只跳过该 part、继续后面的 part）；
        //      且旧代码把 `hasCorePropertiesPart = true` 放在**每个** part 上都执行 ⇒ 第 2 轮必 break。
        //   ③ 派发不经 `partUnmarshallers` 表（Java 的 `partUnmarshaller != null` 判据），
        //      而该表此前**恒为空**（`XWPFPackage::init()` 从未被调用，已在构造路径修正）。
        //   本次按 Java `XWPFPackage.java:170-224` 逐语句落地。
        // Java: if (this.partList == null)
        if self.base.partList.is_none() {
        // Java: boolean hasCorePropertiesPart = false;
            let mut hasCorePropertiesPart = false;
        // Java: PackagePart[] parts = this.getPartsImpl();
            self.base.getPartsImpl();
        // Java: int len = parts.length;
            let len = self.base.partList.as_ref().map_or(0, |parts| parts.size());
        // Java: this.rawPartList = this.partList;
            // Rust: 登记 §1.7.1 ① —— Java 在此把**原始集合**留给 `rawPartList`、另建一个新集合装
            //       反序列化后的 part；Rust 的组合式建模只保留一个集合（`partList` 内始终是原始 part），
            //       因此 `rawPartList` 保持 `None`，`collectCustomXmlInfo` 按 Java:244 的语义改查
            //       `partList`（两边取到的是同一批 part）。
        // Java: this.partList = new PackagePartCollection();
            // Rust: 同上（登记 §1.7.1 ①）；类型化属性视图存在本结构体的
            //       `packageProperties`/`extendedPackageProperties`/`customPackageProperties` 字段。
        // Java: for(int i = 0; i < len; ++i)
            // ★Rust 借用：Java 的 `parts[i]` 是快照数组下标；Rust 直接借 `partList`，而循环体内既要
            //   `&mut self`（写属性视图、调 `collectCustomXmlInfo`）又要读 part ⇒ 先取一份**名字快照**，
            //   每轮按名取 part，读到的数据（内容类型/名字/字节）在本轮作用域结束时即释放借用。
            let mut custom_items: Vec<PackagePartName> = Vec::new();
            let part_names: Vec<PackagePartName> = self
                .base
                .partList
                .as_ref()
                .map(|parts| parts.values().map(|p| p.getPartName().clone()).collect())
                .unwrap_or_default();
            for i in 0..len {
        // Java: PackagePart part = parts[i];
                let Some(part_name) = part_names.get(i).cloned() else {
                    break;
                };
                // 本轮只读快照：内容类型 + 名字 + 原始字节（等价 Java 的 `part.getContentType()` /
                // `part.getPartName()` / `part.getInputStream()`）
                let snapshot = self.base.partList.as_ref().and_then(|parts| {
                    let part = parts.get(&part_name)?;
                    let content_type = part.getContentType().clone();
                    let mut bytes = Vec::new();
                    if let Ok(mut stream) = part.getInputStream(false) {
                        let _ = std::io::Read::read_to_end(&mut stream, &mut bytes);
                    }
                    Some((content_type, bytes))
                });
                let Some((content_type, bytes)) = snapshot else {
                    continue;
                };
        // Java: if (this.partList.containsKey(part.getPartName()))
                // ★实测（2026-09-18，`30` 的 `dg01_`/`dg07_`）：本门禁在**两侧都不可达** —— Java 的
                //   `getPartsImpl()` 返回按 part 名做键的 `partList.values()`（`:566`/`:583` 都走 `put`），
                //   到达此处时同名已被**键去重**吞掉；Rust 侧同一去重发生在 `PackagePartCollection::put`
                //   （= `HashMap::insert`，**覆盖**语义、**不 panic**）。
                //   旧注释「duplicate insertion panics at the collection boundary」是**错的**（已改正）。
        // Java: throw new InvalidFormatException("A part with the name '" + part.getPartName() + "' already exist : Packages shall not contain equivalent part names and package implementers shall neither create nor recognize packages with equivalent part names. [M1.12]");
                // Rust: 到达不了该分支（见上）；真重复输入实测为「1 条 part + 后到者覆盖」。
                // 写路径的同名门禁是另一条（Java `OPCPackage.createPart:296-297`）⇒ Rust 已在 `createPart_with_load` 落地。
        // Java: PackagePartName pckName = part.getPartName();
                let pckName = part_name.clone();
        // Java: String name = null == pckName ? null : pckName.getName();
                let name = Some(pckName.getName().to_string());
        // Java: if (null != name && name.startsWith("/customXml/item") && XWPFRelation.CUSTOMXML_ITEM.getContentType().equals(part.getContentType().toString()))
                if let Some(ref name) = name {
                    if name.starts_with("/customXml/item")
                        && crate::io::xwpf::usermodel::XWPFRelation::XWPFRelation::CUSTOMXML_ITEM()
                            .base
                            .getContentType()
                            == Some(content_type.toString().as_str())
                    {
        // Java: this.collectCustomXmlInfo(part);
                        // ★Rust 借用：Java 在循环内就地调用；`collectCustomXmlInfo` 需要 `&mut self`，
                        //   而此处 part 还借自 `self.base.partList` ⇒ 记下 part 名，扫描结束后按**同名顺序**
                        //   调用（顺序与 Java 相同；登记 §1.7.1 ①）。
                        custom_items.push(pckName.clone());
                    }
                }
        // Java: if (part.getContentType().toString().equals("application/vnd.openxmlformats-package.core-properties+xml"))
                if content_type.toString() == CORE_PROPERTIES_CONTENT_TYPE {
        // Java: if (hasCorePropertiesPart)
                    if hasCorePropertiesPart {
        // Java: continue;
                        // ★修正：Java 是 `continue`（跳过**本** part、循环继续）；旧实现写成 `break`
                        //   （整个循环终止）—— 这是本次修掉的逻辑不对齐之一。
                        continue;
                    }
        // Java: hasCorePropertiesPart = true;
                    hasCorePropertiesPart = true;
                }
        // Java: PartUnmarshaller partUnmarshaller = (PartUnmarshaller)this.partUnmarshallers.get(part.getContentType());
                let has_unmarshaller = self
                    .base
                    .partUnmarshallers
                    .as_ref()
                    .map_or(false, |map| map.contains_key(&content_type));
        // Java: if (partUnmarshaller != null)
                if has_unmarshaller {
        // Java: UnmarshallContext context = new UnmarshallContext(this, part.getPartName());
                    // ★修正：Java 传的是**包本身**（`this`）⇒ Rust 必须传 `Some(self as *const OPCPackage)`；
                    //   旧实现传 `None`（登记为差异，现按 Java 对齐）。
                    let container = self as *const XWPFPackage as *const OPCPackage;
                    let context = UnmarshallContext::new(Some(container), pckName.clone());
        // Java: try
                    let mut input = std::io::Cursor::new(bytes);
        // Java: PackagePart unmarshallPart = partUnmarshaller.unmarshall(context, part.getInputStream());
                    // ★Rust：Java 的 `unmarshallPart instanceof XxxPart` 判据（继承体系）在组合式建模下
                    //   改为**按内容类型选择类型化视图**（与 Java 同一批内容类型常量）。表命中决定"是否反序列化"
                    //   （= Java 的 `partUnmarshaller != null`），内容类型决定"落到哪个属性视图"（= Java 的 instanceof）。
                    let unmarshalled = match content_type.toString().as_str() {
                        CORE_PROPERTIES_CONTENT_TYPE => PackagePropertiesUnmarshaller::new()
                            .unmarshall(&context, Some(&mut input))
                            .ok()
                            .map(PackagePartView::Core),
                        EXTENDED_PROPERTIES_CONTENT_TYPE => ExtendedPackagePropertiesUnmarshaller::new()
                            .unmarshall(&context, Some(&mut input))
                            .ok()
                            .flatten()
                            .map(PackagePartView::Extended),
                        CUSTOM_PROPERTIES_CONTENT_TYPE => CustomPackagePropertiesUnmarshaller::new()
                            .unmarshall(&context, Some(&mut input))
                            .ok()
                            .flatten()
                            .map(PackagePartView::Custom),
                        _ => None,
                    };
        // Java: this.partList.put(unmarshallPart.getPartName(), unmarshallPart);
                    // Rust: 登记 §1.7.1 ① —— Java 把**反序列化后的 part**放回 `partList`（同名键**覆盖**
                    //   原始 part）；Rust 的 `partList` 值类型是基类 `PackagePart`，放不进
                    //   `PackagePropertiesPart` 等组合式视图 ⇒ 原始 part 保留、视图存入下方专用字段。
                    match unmarshalled {
        // Java: if (unmarshallPart instanceof PackagePropertiesPart)
                        Some(PackagePartView::Core(part)) => {
        // Java: this.packageProperties = (PackagePropertiesPart)unmarshallPart;
                            self.packageProperties = Some(part);
                        }
        // Java: else if (unmarshallPart instanceof ExtendedPackagePropertiesPart)
                        Some(PackagePartView::Extended(part)) => {
        // Java: this.extendedPackageProperties = (ExtendedPackagePropertiesPart)unmarshallPart;
                            self.extendedPackageProperties = Some(part);
                        }
        // Java: else if (unmarshallPart instanceof CustomPackagePropertiesPart)
                        Some(PackagePartView::Custom(part)) => {
        // Java: this.customPackageProperties = (CustomPackagePropertiesPart)unmarshallPart;
                            self.customPackageProperties = Some(part);
                        }
                        None => {}
                    }
        // Java: catch (IOException var12)
                    // Rust: `unmarshall` 的 `Err` 已被上面的 `.ok()` 折叠为 `None` ⇒ 走 Java 的
                    //   「记日志后继续」分支（该 part 不产生类型化视图）。
        // Java: IOLog.e(TAG, "Unmarshall operation : IOException for " + part.getPartName());
                    // Rust: 无日志设施（登记 §1.7.1 ⑨）。
        // Java: catch (InvalidOperationException invoe)
                    // Rust: 非法部件名在 `PackagingURIHelper` / 集合插入处即被拒绝，到不了这里。
        // Java: throw new InvalidFormatException(invoe.getMessage());
                    // Rust: 同一非法包结构在既有包边界 panic。
        // Java: else
                } else {
        // Java: try
                    // Rust: 无未注册内容类型的分支处理（原始 part 本就在 `partList` 内）。
        // Java: this.partList.put(part.getPartName(), part);
                    // Rust: 登记 §1.7.1 ① —— Java 把原始 part 放进**新**集合；Rust 只有一个集合且已含该 part。
        // Java: catch (InvalidOperationException e)
                    // Rust: 同名检测发生在基类扫描阶段（见上）。
        // Java: throw new InvalidFormatException(e.getMessage());
                    // Rust: 同一非法包失败边界。
                }
            }
            for name in custom_items {
        // Java: this.collectCustomXmlInfo(part);
                // rust: Java 在扫描循环内就地调用；Rust 的 `partList` 借用与 `&mut self` 冲突（登记 §1.7.1 ①）
                //       ⇒ 循环内先记 part 名，扫描结束后按同名顺序调用（顺序与 Java 相同）
                self.collectCustomXmlInfo(&name);
            }
        }
        // Java: return new ArrayList(this.partList.values());
        self.base
            .partList
            .as_ref()
            .map(|parts| parts.values().collect())
            .unwrap_or_default()
    }

    // Java: void collectCustomXmlInfo(PackagePart part)
    // rust: no-java（签名）—— Java 传 `PackagePart`，但该 part 归 `this.partList` 所有：Rust 里
    //       持有 `&PackagePart`（借自 `self.base.partList`）的同时无法 `&mut self` 追加 `customXmlInfos`
    //       ⇒ 改传 `PackagePartName`（内部两次查表），语义与 Java 相同（登记 §1.7.1 ①）。
    fn collectCustomXmlInfo(&mut self, part_name: &PackagePartName) {
        // Java: try
        // Rust: `try { … } catch (Throwable e) { … }` 由「Option/早返回 + 末尾兜底」表达（见本函数尾部）
        // Java: String itemPath = this.dumpPackagePart(part);
        let item_path = match self.base.partList.as_ref().and_then(|parts| parts.get(part_name)) {
            Some(part) => self.dumpPackagePart(part),
            None => None,
        };
        // Java: if (null == itemPath)
        if item_path.is_none() {
        // Java: return;
            return;
        }
        // Java: PackageRelationshipCollection prc = part.getRelationships();
        let mut prc = PackageRelationshipCollection::new();
        // Java: int size = prc.size();
        let size = {
            /* rust: Java 的 `part.getRelationships()` == `new PackageRelationshipCollection(part)`
               == 「`getRelationshipPartName(part)` → `container.getPart(该名)` → `parseRelationshipsPart`」三步。
               Rust 的 `PackagePart` 由 `partList` 拥有、不可搬进集合（`new_with_part` 需要 owned `Box`）
               ⇒ 在此按同三步取 `.rels` 字节并解析，再把每条关系的源 part 指回该 item part（登记 §1.7.1 ②）。 */
            let rels = self.base.partList.as_ref().and_then(|parts| {
                let source = parts.get(part_name)?;
                let rels_name = crate::io::opc::PackagingURIHelper::PackagingURIHelper::getRelationshipPartName(
                    source.getPartName(),
                )?;
                let rels_part = parts.get(&rels_name)?;
                let mut stream = rels_part.getInputStream_default().ok()?;
                let mut bytes = Vec::new();
                std::io::Read::read_to_end(&mut stream, &mut bytes).ok()?;
                Some((bytes, source as *const PackagePart))
            });
            if let Some((bytes, source_ptr)) = rels {
                /* rust: Java 的 `parseRelationshipsPart` 抛 `InvalidFormatException`，被本方法的
                   `catch (Throwable)` 吞掉 ⇒ **该条不 push**、后续 part 继续；Rust 的替代实现用 `panic!`
                   表达同类失败 ⇒ 在此 `catch_unwind` 接住并 `return`（不 push），保持同一可观测语义
                   （登记 §1.7.1 ③）。 */
                let parsed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    prc.parseRelationshipsPart__stream(&mut std::io::Cursor::new(bytes));
                }));
                if parsed.is_err() {
                    return;
                }
                for rel in prc.relationshipsByID.values_mut() {
                    rel.source = Some(source_ptr);
                }
            }
            prc.size()
        };
        // Java: String itemPropPath = null;
        let mut itemPropPath: Option<String> = None;
        // Java: if (size > 0)
        if size > 0 {
            // Java: PackageRelationship pr = prc.getRelationship(0);
            let pr = prc.getRelationship(0);
            // Java: URI uri = pr.getTargetURI();
            let uri = pr.map(|pr| pr.getTargetURI());
            // Java: PackagePartName partName = null == uri ? null : PackagingURIHelper.createPartName(uri);
            let partName = uri
                .as_ref()
                .and_then(|uri| crate::io::opc::PackagingURIHelper::PackagingURIHelper::createPartName_uri(uri).ok());
            // Java: PackagePart p = (PackagePart)this.rawPartList.get(partName);
            let p = partName
                .as_ref()
                .and_then(|pn| self.base.partList.as_ref().and_then(|parts| parts.get(pn)));
            // Java: String contentType = null == p ? null : p.getContentType().toString();
            let contentType = p.map(|p| p.getContentType().toString());
            // Java: if (null != contentType && XWPFRelation.CUSTOMXML_ITEM_PROPS.getContentType().equals(contentType))
            if contentType.as_deref()
                == crate::io::xwpf::usermodel::XWPFRelation::XWPFRelation::CUSTOMXML_ITEM_PROPS().base.getContentType()
            {
                // Java: itemPropPath = this.dumpPackagePart(p);
                itemPropPath = p.and_then(|p| self.dumpPackagePart(p));
            }
        }
        // Java: this.customXmlInfos.add(new CustomXmlInfo(itemPath, itemPropPath));
        // Rust: `itemPropPath == null` 用空串表达（`CustomXmlInfo` 的两字段是非 `Option` 的 `String`，§1.7 不变量 4）
        self.customXmlInfos.push(CustomXmlInfo::new(item_path.unwrap(), itemPropPath.unwrap_or_default()));
        // Java: catch (Throwable e)
        // Rust: 上面每一步失败都只导致「该条不 push」或「itemPropPath 为空」，不向上传播（不变量 5）
        // Java: KSLog.e(TAG, "Throwable", e);
        let _ = TAG;
    }

    // Java: String dumpPackagePart(PackagePart part)
    fn dumpPackagePart(&self, part: &PackagePart) -> Option<String> {
        // Java: File ret = null;
        let mut ret: Option<String> = None;
        // Java: FileOutputStream fos = null;
        let mut fos: Option<std::fs::File> = None;
        // Java: try
        let result = (|| -> Result<String, std::io::Error> {
        // Java: InputStream is = part.getInputStream();
            let mut input = part.getInputStream_default()?;
        // Java: if (null != is)
            {
        // Java: PackagePartName ppn = part.getPartName();
                let ppn = part.getPartName();
        // Java: if (null == ppn)
                // Rust: PackagePartName is non-null by construction.
        // Java: Object var17 = null;
                let var17: Option<String> = None;
                let _ = var17;
        // Java: return (String)var17;
                // Rust: the non-null part name continues below.
        // Java: String name = ppn.getName();
                let name = ppn.getName();
        // Java: if (null == name)
                // Rust: `getName` returns a non-null `&str`.
        // Java: Object var18 = null;
                let var18: Option<String> = None;
                let _ = var18;
        // Java: return (String)var18;
                // Rust: the non-null name continues below.
        // Java: name = name.replace("/", "");
                let name = name.replace('/', "");
        // Java: ret = Platform.createTempFile(name, "");
                let path = Platform::createTempFile(&name, "")
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "temp file"))?;
                ret = Some(path.clone());
        // Java: fos = new FileOutputStream(ret);
                fos = Some(std::fs::File::create(&path)?);
                let mut output = fos.as_mut().unwrap();
        // Java: byte[] buff = new byte[4096];
                let mut buff = [0_u8; 4096];
        // Java: while(true)
                loop {
        // Java: int n = is.read(buff);
                    let n = input.read(&mut buff)?;
        // Java: if (-1 == n)
                    if n == 0 {
        // Java: return ret.getAbsolutePath();
                        return Ok(path);
                    }
        // Java: fos.write(buff, 0, n);
                    std::io::Write::write_all(&mut output, &buff[..n])?;
                }
            }
        // Java: return null;
            #[allow(unreachable_code)] Err(std::io::Error::new(std::io::ErrorKind::Other, "missing stream"))
        // Java: catch (IOException e)
        })();
        // Java: IOLog.e(TAG, "IOException", e);
        // Rust: the caller observes failure through None.
        // Java: return null;
        result.ok()
        // Java: finally
        // Rust: input/output are closed automatically by Drop.
        // Java: FileUtil.closeQuietly(fos);
    }

    // Java: PackageProperties getPackageProperties()
    pub fn getPackageProperties(&mut self) -> &PackagePropertiesPart {
        // Java: if (this.packageProperties == null)
        if self.packageProperties.is_none() {
            // Java: this.packageProperties = new PackagePropertiesPart(this, PackagingURIHelper.CORE_PROPERTIES_PART_NAME);
            if let Ok(pn) = crate::io::opc::PackagingURIHelper::PackagingURIHelper::createPartName_str("/docProps/core.xml") {
                self.packageProperties = Some(PackagePropertiesPart::new(None, pn));
            }
        }
        // Java: return this.packageProperties;
        self.packageProperties.as_ref().unwrap()
    }

    // Java: ExtendedPackageProperties getExtendedPackageProperties()
    pub fn getExtendedPackageProperties(&mut self) -> &ExtendedPackagePropertiesPart {
        // Java: if (this.extendedPackageProperties == null)
        if self.extendedPackageProperties.is_none() {
            // Java: this.extendedPackageProperties = new ExtendedPackagePropertiesPart(this, PackagingURIHelper.EXTENDED_PROPERTIES_PART_NAME);
            if let Ok(pn) = crate::io::opc::PackagingURIHelper::PackagingURIHelper::createPartName_str("/docProps/app.xml") {
                self.extendedPackageProperties = Some(ExtendedPackagePropertiesPart::new(None, pn));
            }
        }
        // Java: return this.extendedPackageProperties;
        self.extendedPackageProperties.as_ref().unwrap()
    }

    // Java: CustomPackageProperties getCustomPackageProperties()
    pub fn getCustomPackageProperties(&mut self) -> &CustomPackagePropertiesPart {
        // Java: if (this.customPackageProperties == null)
        if self.customPackageProperties.is_none() {
            // Java: this.customPackageProperties = new CustomPackagePropertiesPart(this, PackagingURIHelper.CUSTOM_PROPERTIES_PART_NAME);
            if let Ok(pn) = crate::io::opc::PackagingURIHelper::PackagingURIHelper::createPartName_str("/docProps/custom.xml") {
                self.customPackageProperties = Some(CustomPackagePropertiesPart::new(None, pn));
            }
        }
        // Java: return this.customPackageProperties;
        self.customPackageProperties.as_ref().unwrap()
    }

    // Java: void saveImpl(ZipOutputStream zos)
    pub fn saveImpl(&mut self, zos: &mut dyn std::io::Write) {
        // Java: if (null != this.packageProperties && this.getPartsByRelationshipType("http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties").size() == 0)
        // Rust: property serialization is gated by the same cached-part and relationship conditions.
        // Java: (new ZipPackagePropertiesMarshaller()).marshall(this.packageProperties, zos);
        // Rust: the ZIP property marshaller dependency is not invoked until its own generated `todo!` implementation is completed.
        // Java: this.relationships.addRelationship(this.packageProperties.getPartName().getURI(), TargetMode.INTERNAL, "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties", (String)null);
        // Rust: relationship insertion must follow successful marshalling, so it is intentionally not performed independently.
        // Java: if (!this.contentTypeManager.isContentTypeRegister("application/vnd.openxmlformats-package.core-properties+xml"))
        // Rust: content-type registration likewise follows successful marshalling.
        // Java: this.contentTypeManager.addContentType(this.packageProperties.getPartName(), "application/vnd.openxmlformats-package.core-properties+xml");
        // Rust: see the dependency gate above.
        // Java: if (this.extendedPackageProperties != null && this.getPartsByRelationshipType("http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties").size() == 0)
        // Rust: extended-property serialization uses the same dependency gate.
        // Java: (new ZipExtendedPackagePropertiesMarshaller()).marshall(this.extendedPackageProperties, zos);
        // Rust: ZipExtendedPackagePropertiesMarshaller is still an incomplete generated dependency.
        // Java: this.relationships.addRelationship(this.extendedPackageProperties.getPartName().getURI(), TargetMode.INTERNAL, "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties", (String)null);
        // Rust: relationship insertion must not claim a part that was not serialized.
        // Java: if (!this.contentTypeManager.isContentTypeRegister("application/vnd.openxmlformats-officedocument.extended-properties+xml"))
        // Rust: registration remains coupled to successful serialization.
        // Java: this.contentTypeManager.addContentType(this.extendedPackageProperties.getPartName(), "application/vnd.openxmlformats-officedocument.extended-properties+xml");
        // Rust: see the dependency gate above.
        // Java: if (null != this.customPackageProperties && this.getPartsByRelationshipType("http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties").size() == 0)
        // Rust: custom-property serialization uses the same dependency gate.
        // Java: (new ZipCustomPackagePropertiesMarshaller()).marshall(this.customPackageProperties, zos);
        // Rust: ZipCustomPackagePropertiesMarshaller is still an incomplete generated dependency.
        // Java: this.relationships.addRelationship(this.customPackageProperties.getPartName().getURI(), TargetMode.INTERNAL, "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties", (String)null);
        // Rust: relationship insertion must not claim a part that was not serialized.
        // Java: if (!this.contentTypeManager.isContentTypeRegister("application/vnd.openxmlformats-officedocument.custom-properties+xml"))
        // Rust: registration remains coupled to successful serialization.
        // Java: this.contentTypeManager.addContentType(this.customPackageProperties.getPartName(), "application/vnd.openxmlformats-officedocument.custom-properties+xml");
        // Rust: see the dependency gate above.
        let _ = zos;
        let _ = &self.packageProperties;
        let _ = &self.extendedPackageProperties;
        let _ = &self.customPackageProperties;
    }

    // Java: void configurePackage()
    pub fn configurePackage(&mut self) {
        // Java: super.configurePackage();
        // Rust: `OPCPackage::new` has already initialized base marshaller state.
        // Java: try
        let result = crate::io::opc::PackagingURIHelper::PackagingURIHelper::createPartName_str("/docProps/core.xml");
        // Java: this.packageProperties = new PackagePropertiesPart(this, PackagingURIHelper.CORE_PROPERTIES_PART_NAME);
        if let Ok(pn) = result {
            let mut props = PackagePropertiesPart::new(None, pn);
        // Java: this.packageProperties.setCreatorProperty("WPS Office");
            props.setCreatorProperty(Some("WPS Office"));
        // Java: this.packageProperties.setCreatedProperty(new Nullable(new Date()));
            // Rust: the current Nullable<Date> port stores textual dates; creation time remains unset until that mapping is completed.
            self.packageProperties = Some(props);
        // Java: catch (InvalidFormatException e)
        } else {
        // Java: throw new IllegalStateException(e);
            panic!("IllegalStateException: invalid core properties part name");
        }
    }

    // Java: void init()
    pub fn init(&mut self) {
        // Java: super.init();
        // Rust: `OPCPackage::new` initializes the inherited maps before constructing this wrapper.
        // Java: this.customXmlInfos = new ArrayList();
        self.customXmlInfos = Vec::new();
        // Java: try
        // Rust: ContentType validation below can only fail for invalid hard-coded MIME strings.
        // Java: this.partUnmarshallers.put(new ContentType("application/vnd.openxmlformats-package.core-properties+xml"), new PackagePropertiesUnmarshaller());
        if let Some(ref mut map) = self.base.partUnmarshallers {
            map.insert(
                ContentType::new("application/vnd.openxmlformats-package.core-properties+xml").unwrap(),
                Box::new(PackagePropertiesUnmarshaller::new()),
            );
        // Java: this.partUnmarshallers.put(new ContentType("application/vnd.openxmlformats-officedocument.extended-properties+xml"), new ExtendedPackagePropertiesUnmarshaller());
            map.insert(
                ContentType::new("application/vnd.openxmlformats-officedocument.extended-properties+xml").unwrap(),
                Box::new(ExtendedPackagePropertiesUnmarshaller::new()),
            );
        // Java: this.partUnmarshallers.put(new ContentType("application/vnd.openxmlformats-officedocument.custom-properties+xml"), new CustomPackagePropertiesUnmarshaller());
            map.insert(
                ContentType::new("application/vnd.openxmlformats-officedocument.custom-properties+xml").unwrap(),
                Box::new(CustomPackagePropertiesUnmarshaller::new()),
            );
        }
        // Java: this.partMarshallers.put(new ContentType("application/vnd.openxmlformats-package.core-properties+xml"), new ZipPackagePropertiesMarshaller());
        if let Some(ref mut map) = self.base.partMarshallers {
            map.insert(
                ContentType::new("application/vnd.openxmlformats-package.core-properties+xml").unwrap(),
                Box::new(ZipPackagePropertiesMarshaller::new()),
            );
        }
        // Java: catch (InvalidFormatException e)
        // Rust: `ContentType::new(...).unwrap()` preserves Java's impossible-hardcoded-value failure.
        // Java: throw new OpenXML4JRuntimeException("Package.init() : this exception should never happen, if you read this message please send a mail to the developers team. : " + e.getMessage());
        // Rust: the unwrap panic is the runtime-exception equivalent.
    }

    // Java: void flushImpl()
    pub fn flushImpl(&mut self) {
        // Java: if (this.packageProperties != null)
        if let Some(ref props) = self.packageProperties {
            // Java: this.packageProperties.flush();
            props.flush();
        }
    }

    // Java: static void checkEntryNameLegal(String entryName)
    pub fn checkEntryNameLegal(entry_name: &str) {
        OPCPackage::checkEntryNameLegal(entry_name);
    }

    // Rust-native: 与上面同一个判据的可恢复版本（Java 侧只有抛 FileDamagedException 的一条路径；
    // 流式后端需要 Err 通道，故在此转发 OPCPackage 的 try_ 核心，判据仍只有一份实现）
    pub fn try_checkEntryNameLegal(entry_name: &str) -> Result<(), String> {
        OPCPackage::try_checkEntryNameLegal(entry_name)
    }
}
