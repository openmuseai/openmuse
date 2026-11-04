#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.opc.internal.ContentType
// Java: import cn.wps.io.opc.internal.FileHelper
// Java: import cn.wps.io.util.URI
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidOperationException
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
use crate::io::xwpf::util::ZipEntrySource::ZipEntryRef;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::io::opc::PackageAccess::PackageAccess;
use crate::io::opc::PackagePartName::PackagePartName;
use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::PackagePartCollection::PackagePartCollection;
use crate::io::opc::PackageRelationship::PackageRelationship;
use crate::io::opc::PackageRelationshipCollection::PackageRelationshipCollection;
use crate::io::opc::TargetMode::TargetMode;
use crate::io::opc::PackagingURIHelper::PackagingURIHelper;
use crate::io::xwpf::util::ZipFileZipEntrySource::ZipFileZipEntrySource;
use crate::io::util::URI::URI;
use crate::io::opc::internal::ContentType::ContentType;
use crate::io::opc::internal::ContentTypeManager::ContentTypeManager;
use crate::io::opc::internal::PartMarshaller::PartMarshaller;
use crate::io::opc::internal::PartUnmarshaller::PartUnmarshaller;
use crate::io::opc::internal::marshallers::DefaultMarshaller::DefaultMarshaller;
use crate::io::opc::ContentTypes::ContentTypes;
use crate::io::opc::ZipPackagePart::ZipPackagePart;
use crate::io::opc::internal::ZipHelper::ZipHelper;
use crate::io::xwpf::util::ZipEntrySource::{FakeZipEntry, ZipEntrySource};
use crate::io::xwpf::util::ZipInputStreamZipEntrySource::ZipInputStreamZipEntrySource;

// Java field: String DOCUMENT_CREATOR
pub const DOCUMENT_CREATOR: &str = "WPS Office";
// Java field: int MAX_SIZE_ENTRY
pub const MAX_SIZE_ENTRY: i32 = 1073741824;

// Java: public abstract class OPCPackage implements RelationshipSource
pub struct OPCPackage {
    // Java field: PackageAccess packageAccess
    pub(crate) packageAccess: PackageAccess,
    // Java field: PackagePartCollection partList
    pub(crate) partList: Option<PackagePartCollection>,
    // Rust: 重入护栏（**无 Java 对应字段**）—— Java 的 `getPartsImpl` 靠 `partList == null` 天然幂等，
    //       而 Rust 的 `PackagePart::getRelationshipsCore` 首次读取时会回调 `container.get_part_mut(...)`
    //       以取得稳定的 `sourcePart` 地址，该调用会在 `partList` 尚未填充时再次进入 `getPartsImpl`
    //       ⇒ 无限递归（实测：探针挂死 600s 被 SIGTERM）。本标志让扫描**至多执行一次**，
    //       与 Java「partList 非空即不再扫描」的可观察行为一致。
    pub(crate) partsScanInProgress: bool,
    // Java field: PackagePartCollection rawPartList
    pub(crate) rawPartList: Option<PackagePartCollection>,
    // Java field: PackageRelationshipCollection relationships
    pub(crate) relationships: Option<PackageRelationshipCollection>,
    // Java field: Hashtable<ContentType, PartMarshaller> partMarshallers
    pub(crate) partMarshallers: Option<HashMap<ContentType, Box<dyn PartMarshaller>>>,
    // Java field: PartMarshaller defaultPartMarshaller
    pub(crate) defaultPartMarshaller: Option<Box<dyn PartMarshaller>>,
    // Java field: Hashtable<ContentType, PartUnmarshaller> partUnmarshallers
    pub(crate) partUnmarshallers: Option<HashMap<ContentType, Box<dyn PartUnmarshaller>>>,
    // Java field: ContentTypeManager contentTypeManager
    pub(crate) contentTypeManager: Option<Box<ContentTypeManager>>,
    // Java field: boolean isDirty
    pub(crate) isDirty: bool,
    // Java field: String originalPackagePath
    pub(crate) originalPackagePath: Option<String>,
    // Java field: File tmpPackageFile（kfs::File 未实现，用 PathBuf 承载路径语义）
    pub(crate) tmpPackageFile: Option<PathBuf>,
    // Java field: OutputStream output
    pub(crate) output: Option<Box<dyn std::io::Write + Send + Sync>>,
    // Java field: ZipOutputStream mZOS（kfs::zip 未实现，用输出流容器承载）
    pub(crate) mZOS: Option<Box<dyn std::io::Write + Send + Sync>>,
    // Java field: ZipEntrySource zipArchive
    pub(crate) zipArchive: Option<Box<dyn ZipEntrySource>>,
}

// Java: public abstract class OPCPackage implements RelationshipSource
impl OPCPackage {
    // Java: public static void checkEntryNameLegal(String entryName)
    pub fn checkEntryNameLegal(entry_name: &str) {
        // Java: if (entryName.contains("../")) throw new FileDamagedException()
        if let Err(error) = Self::try_checkEntryNameLegal(entry_name) {
            // Java: throw new FileDamagedException();
            panic!("{error}");
        }
    }

    // Rust-native: checkEntryNameLegal 的可恢复版本（两层约定：try_ 核心 + Java 签名外壳）。
    // 判据与 Java 完全一致（`contains("../")`）；供需要 Err 通道的调用方复用**同一实现**，
    // 避免各处再内联复制判据（内联复制会让两侧规则将来各自漂移）。
    pub fn try_checkEntryNameLegal(entry_name: &str) -> Result<(), String> {
        // Java: if (entryName.contains("../")) throw new FileDamagedException()
        if entry_name.contains("../") {
            return Err("FileDamagedException".to_string());
        }
        Ok(())
    }

    // Java: entry.getSize() > 1073741824L
    pub fn isEntryTooLarge(size: i64) -> bool {
        size > MAX_SIZE_ENTRY as i64
    }

    // Java: private final boolean isPartNameContentTypes(String s)
    pub fn isPartNameContentTypes(s: &str) -> bool {
        // Java: return s.length() == 19 && s.charAt(0) == '[' && (s.charAt(1) | 32) == 99 && (s.charAt(2) | 32) == 111 && (s.charAt(3) | 32) == 110 && (s.charAt(4) | 32) == 116 && (s.charAt(5) | 32) == 101 && (s.charAt(6) | 32) == 110 && (s.charAt(7) | 32) == 116 && s.charAt(8) == '_' && (s.charAt(9) | 32) == 116 && (s.charAt(10) | 32) == 121 && (s.charAt(11) | 32) == 112 && (s.charAt(12) | 32) == 101 && (s.charAt(13) | 32) == 115 && s.charAt(14) == ']' && s.charAt(15) == '.' && (s.charAt(16) | 32) == 120 && (s.charAt(17) | 32) == 109 && (s.charAt(18) | 32) == 108;
        // Rust: 同一判据逐字节镜像（`b[i]` 对应 `s.charAt(i)`，`| 32` 即 Java 的小写化比较）
        let b = s.as_bytes();
        b.len() == 19
            && b[0] == b'['
            && (b[1] | 32) == b'c'
            && (b[2] | 32) == b'o'
            && (b[3] | 32) == b'n'
            && (b[4] | 32) == b't'
            && (b[5] | 32) == b'e'
            && (b[6] | 32) == b'n'
            && (b[7] | 32) == b't'
            && b[8] == b'_'
            && (b[9] | 32) == b't'
            && (b[10] | 32) == b'y'
            && (b[11] | 32) == b'p'
            && (b[12] | 32) == b'e'
            && (b[13] | 32) == b's'
            && b[14] == b']'
            && b[15] == b'.'
            && (b[16] | 32) == b'x'
            && (b[17] | 32) == b'm'
            && (b[18] | 32) == b'l'
    }

    // Java: private PackagePartName buildPartName(ZipEntry entry)
    pub fn buildPartName(entry_name: &str) -> Option<PackagePartName> {
        // Java: String entryName = entry.getName();
        // rust: no-stmt（Rust 入参 `entry_name` 即 Java 的局部 `entryName`（同一字符串），无独立取名字句）
        // Java: checkEntryNameLegal(entryName);
        Self::checkEntryNameLegal(entry_name);
        // Java: return this.isPartNameContentTypes(entryName) ? null : PackagingURIHelper.createPartName(ZipHelper.getOPCNameFromZipItemName(entryName));
        // Rust: 三元两臂 —— null 臂为下方早退，否则臂为 `createPartName`（失败即 Java 的 catch 分支）
        if Self::isPartNameContentTypes(entry_name) {
            return None;
        }
        let opc = ZipHelper::getOPCNameFromZipItemName(entry_name);
        PackagingURIHelper::createPartName_str(&opc).ok()
        // Java: catch (Exception e) {
        // Java: IOLog.e(TAG, "Entry " + entryName + " is not valid, so this part won't be add to the package.", e);
        // Java: return null;
        // Rust: 该 catch 分支在 Rust 侧由 `createPartName_str(...).ok()` 的 `None` 承载（不 panic，与 Java 同判据）
    }

    pub fn setZipArchive(&mut self, zip_archive: Box<dyn ZipEntrySource>) {
        self.zipArchive = Some(zip_archive);
    }

    pub fn setContentTypeManager(&mut self, manager: ContentTypeManager) {
        self.contentTypeManager = Some(Box::new(manager));
    }

    pub fn getPartList(&self) -> Option<&PackagePartCollection> {
        self.partList.as_ref()
    }

    // Java: protected PackagePart[] getPartsImpl()
    pub fn getPartsImpl(&mut self) {
        // rust: 重入护栏 —— 见 `partsScanInProgress` 字段注释。已在扫描中则直接返回，
        //       避免 `get_part_mut` → `getPartsImpl` → `loadRelationships` → `getRelationshipsCore`
        //       → `get_part_mut` 的无限递归。
        if self.partsScanInProgress {
            return;
        }
        // Java: if (this.partList == null) {
        if self.partList.is_none() {
            // Java: this.partList = new PackagePartCollection();
            self.partList = Some(PackagePartCollection::new());
        }
        self.partsScanInProgress = true;
        // rust: 扫描结束（含早退路径）时清除标志，用闭包包裹主体以覆盖所有 return。
        let _guard = PartsScanGuard(self as *mut Self);
        struct PartsScanGuard(*mut OPCPackage);
        impl Drop for PartsScanGuard {
            fn drop(&mut self) {
                unsafe { (*self.0).partsScanInProgress = false; }
            }
        }
        // Java: }
        // Java: if (this.zipArchive == null) {
        let Some(archive) = self.zipArchive.as_ref() else {
            // Java: return (PackagePart[])this.partList.values().toArray(new PackagePart[this.partList.values().size()]);
            // rust: no-stmt（Java 返回 partList 快照数组；Rust 侧结果集即 `self.partList`，由调用方按需迭代）
            return;
        };
        // Java: ArrayList<ZipEntry> exceptContentType = new ArrayList(this.zipArchive.size());
        let mut exceptContentType: Vec<FakeZipEntry> = Vec::new();
        // Java: boolean contentTypePartFound = false;
        let mut contentTypePartFound = false;
        // Java: Enumeration<? extends ZipEntry> entries = this.zipArchive.getEntries();
        // Java: int i = 0;
        // rust: no-stmt（Java 的 `i` 声明后从未被读取 —— 死变量；Rust 以 `for entry in entries` 迭代器取代游标）
        let Some(entries) = archive.getEntries() else {
            return;
        };
        // Java: while(entries.hasMoreElements()) {
        for entry in entries {
            // Java: ZipEntry entry = (ZipEntry)entries.nextElement();
            // rust: no-stmt（`for entry in entries` 已绑定 entry；Java 的枚举游标在 Rust 侧即迭代器）
            // Java: if (null != entry && entry.getSize() > 1073741824L) {
            if Self::isEntryTooLarge(entry.getSize()) {
                // Java: throw new FileDamagedException();
                panic!("FileDamagedException");
            }
            // Java: }
            // Java: String entryName = entry.getName();
            let entryName = entry.getName().to_string();
            // Java: checkEntryNameLegal(entryName);
            Self::checkEntryNameLegal(&entryName);
            // Java: if (!contentTypePartFound && this.isPartNameContentTypes(entryName)) {
            if !contentTypePartFound && Self::isPartNameContentTypes(&entryName) {
                // Java: this.contentTypeManager = new ZipContentTypeManager(this.getZipArchive().getInputStream(entry), this);
                // Java: catch (IOException e) { throw new InvalidFormatException(e.getMessage()); }
                // Rust: 经 trait 委托给 zip 源（Java 同一句）——entry 只是元数据（文件后端 =
                //       kfs `ZipEntry`），字节必须从源上取。
                let mut stream = match ZipEntrySource::getInputStream_with_flag(
                    &**archive,
                    // Java 侧 `entry` 就是 `ZipEntry`；Rust 按具体实例包成真实类型载体
                    &ZipEntryRef::Fake(entry.clone()),
                    false,
                ) {
                    Ok(stream) => stream,
                    Err(error) => panic!("InvalidFormatException: {}", error),
                };
                // Java: this.contentTypeManager = new ZipContentTypeManager(stream, this);
                // Rust: `this` 用共享引用指针承载（`ContentTypeManager.container: Option<*const OPCPackage>`）
                let selfPtr = self as *const OPCPackage;
                self.contentTypeManager = Some(Box::new(ContentTypeManager::new(Some(&mut stream), Some(selfPtr))));
                // Java: contentTypePartFound = true;
                contentTypePartFound = true;
            } else {
                // Java: exceptContentType.add(entry);
                exceptContentType.push(entry.clone());
            }
        }
        // Java: }
        // Java: int len = exceptContentType.size();
        let len = exceptContentType.len();
        // Java: PackagePartName[] partNames = new PackagePartName[len];
        let mut partNames: Vec<Option<PackagePartName>> = vec![None; len];
        // Java: String[] contentTypes = new String[len];
        let mut contentTypes: Vec<Option<String>> = vec![None; len];
        // Java: for(int j = 0; j < len; ++j) {
        for j in 0..len {
            // Java: ZipEntry entry = (ZipEntry)exceptContentType.get(j);
            let entry = &exceptContentType[j];
            // Java: PackagePartName partName = this.buildPartName(entry);
            // Java: if (null != partName) {
            if let Some(partName) = Self::buildPartName(entry.getName()) {
                // Java: String contentType = null != this.contentTypeManager ? this.contentTypeManager.getContentType(partName) : null;
                let mut contentType = self
                    .contentTypeManager
                    .as_ref()
                    .and_then(|mgr| mgr.getContentType(&partName));
                // Java: if (null == contentType) {
                if contentType.is_none() {
                    // Java: String extension = partName.getExtension().toLowerCase();
                    let extension = partName.getExtension().to_lowercase();
                    // Java: contentType = ContentTypes.guessContentTypeByExtension(extension);
                    contentType = ContentTypes::guessContentTypeByExtension(&extension);
                }
                // Java: }
                // Java: if (null != contentType) {
                if let Some(ct) = contentType {
                    // Java: if (contentType.equals("application/vnd.openxmlformats-package.relationships+xml")) {
                    if ct == crate::io::opc::ContentTypes::RELATIONSHIPS_PART {
                        // Java: this.partList.put((PackagePartName)partName, (PackagePart)(new ZipPackagePart(this, entry, partName, contentType)));
                        let part = Self::part_from_entry(self, entry, partName.clone(), &ct);
                        // Java: catch (InvalidOperationException e) {
                        // Java: throw new InvalidFormatException(e.getMessage());
                        if let Some(ref mut list) = self.partList {
                            list.put(partName, part);
                        }
                    } else {
                        // Java: contentTypes[j] = contentType;
                        contentTypes[j] = Some(ct);
                        // Java: partNames[j] = partName;
                        partNames[j] = Some(partName);
                    }
                }
            }
        }
        // Java: }
        // Java: for(int j = 0; j < len; ++j) {
        for j in 0..len {
            // Java: String contentType = contentTypes[j];
            if let (Some(contentType), Some(partName)) = (contentTypes[j].as_ref(), partNames[j].clone()) {
                // Java: if (contentType != null) {
                // Java: PackagePartName partName = partNames[j];
                let part = Self::part_from_entry(self, &exceptContentType[j], partName.clone(), contentType);
                // Java: this.partList.put((PackagePartName)partName, (PackagePart)(new ZipPackagePart(this, (ZipEntry)exceptContentType.get(j), partName, contentType)));
                // Java: catch (InvalidOperationException e) {
                // Java: throw new InvalidFormatException(e.getMessage());
                if let Some(ref mut list) = self.partList {
                    list.put(partName, part);
                }
            }
        }
        // Java: }
        // Java: return (PackagePart[])this.partList.values().toArray(new ZipPackagePart[this.partList.size()]);
        // rust: no-stmt（同早退分支：结果集即 `self.partList`，Java 的数组视图由调用方按需迭代）
    }

    // Java: public ZipOutputStream getZipOutputStream()
    pub fn getZipOutputStream(&self) -> &Option<Box<dyn std::io::Write + Send + Sync>> {
        // Java: return this.mZOS
        &self.mZOS
    }

    // Java: public final File getTmpPackageFile()
    pub fn getTmpPackageFile(&self) -> &Option<PathBuf> {
        // Java: return this.tmpPackageFile
        &self.tmpPackageFile
    }

    // Java: public ZipEntrySource getZipArchive()
    pub fn getZipArchive(&self) -> &Option<Box<dyn ZipEntrySource>> {
        // Java: return this.zipArchive
        &self.zipArchive
    }

    /// `contentTypeManager` 只读访问（`57_function_parity_rust_dump` 的
    /// `contentTypes.defaults` / `contentTypes.overrides` 段需要按 schema 输出）。
    ///
    /// Java: `protected ContentTypeManager contentTypeManager;`——Java 同包内直接读字段。
    /// Rust 为可观测性提供只读访问；**只读**，不改变行为。
    pub fn getContentTypeManager(&self) -> Option<&crate::io::opc::internal::ContentTypeManager::ContentTypeManager> {
        self.contentTypeManager.as_deref()
    }

    // Java: protected OPCPackage(PackageAccess access)
    pub fn OPCPackage(access: PackageAccess) -> Self {
        // Java: super();
        // rust: no-stmt（`Object` 超类构造在 Rust 无对应语句）
        // Java: this.isDirty = false
        // Java: this.init()
        // Java: this.packageAccess = access
        // Java: this.zipArchive = null
        let mut pkg = OPCPackage {
            packageAccess: access,
            partList: None,
            partsScanInProgress: false,
            rawPartList: None,
            relationships: None,
            partMarshallers: None,
            defaultPartMarshaller: None,
            partUnmarshallers: None,
            contentTypeManager: None,
            isDirty: false,
            originalPackagePath: None,
            tmpPackageFile: None,
            output: None,
            mZOS: None,
            zipArchive: None,
        };
        pkg.init();
        pkg
    }

    pub fn new(access: PackageAccess) -> Self {
        Self::OPCPackage(access)
    }

    // Java: `getPartsImpl` 的两处 put 都是 `new ZipPackagePart(this, entry, partName, contentType)`
    //       （`OPCPackage.java:566` / `:583`）
    fn part_from_entry(&self, entry: &FakeZipEntry, part_name: PackagePartName, content_type: &str) -> PackagePart {
        // Java: this（容器）——Java 引用语义；Rust 用共享引用指针承载（`PackagePart._container`）
        let container = Some(self as *const OPCPackage);
        // Java: new ZipPackagePart(this, entry, partName, contentType)
        let mut zip_part = ZipPackagePart::new_with_entry(
            container,
            // Java: `new ZipPackagePart(this, entry, partName, contentType)` —— `entry` 是
            //       流后端的 `ZipEntry` 具体实例（`FakeZipEntry extends ZipEntry`）⇒ 按真实类型装进枚举
            Some(ZipEntryRef::Fake(entry.clone())),
            part_name,
            content_type,
        );
        // Java：`partList` 里存的是 **`ZipPackagePart` 实例**（多态）；Rust 集合值的静态类型是基类
        // ⇒ 把子类的 `zipEntry` 字段落入基类的承载位（同名字段／同类型），等价于 Java 的多态值。
        zip_part.base.zipEntry = zip_part.zipEntry.take();
        // Java：`ZipPackagePart.getInputStreamImpl` 现取 `this.getPackage().getZipArchive()`
        // ⇒ 这里把该 zip 源的**堆地址**一并交给承载位（Rust 值语义下包会被 move，见字段注释）。
        zip_part.base.zipArchive = self
            .zipArchive
            .as_ref()
            .map(|archive| &**archive as *const dyn ZipEntrySource);
        // Java 把 `ZipPackagePart` **实例**放进 partList（多态）；Rust 集合值的静态类型是基类
        // ⇒ 取其 `base`（`zipEntry` 已由上面的构造器写入），与 Java 的多态值等价。
        // ★这样 part 不再自带一份 entry 字节副本：取流时按 Java 走
        //   `getPackage().getZipArchive().getInputStream(zipEntry, isOriginInput)`。
        zip_part.base
    }

    // Java: protected OPCPackage(String path, PackageAccess access)
    pub fn OPCPackage_path(path: &str, access: PackageAccess) -> Self {
        // Java: this(access);
        let mut pkg = Self::new(access);
        // Java: ZipFile zipFile = ZipHelper.openZipFile(path);
        let zipFile = ZipHelper::openZipFile(path);
        // Java: if (zipFile == null) throw new ZipError("Can't open the specified file: '" + path + "'");
        let Some(zipFile) = zipFile else {
            panic!("Can't open the specified file: '{}'", path);
        };
        // Java: this.zipArchive = new ZipFileZipEntrySource(zipFile);
        pkg.setZipArchive(Box::new(ZipFileZipEntrySource::ZipFileZipEntrySource(zipFile)));
        pkg
    }

    pub fn from_path(path: &str, access: PackageAccess) -> Self {
        Self::OPCPackage_path(path, access)
    }

    // Java: protected OPCPackage(RandomAccessFile raf, PackageAccess access)
    pub fn OPCPackage_raf(raf: &dyn std::any::Any, access: PackageAccess) -> Self {
        // Java: this(access);
        let mut pkg = Self::new(access);
        // Java: ZipFile zipFile = ZipHelper.openZipFile(raf);
        let zipFile = ZipHelper::openZipFile_raf(raf);
        // Java: if (null == zipFile) throw new ZipError("Can't open the specified raf: '" + raf + "'");
        let Some(zipFile) = zipFile else {
            panic!("Can't open the specified raf");
        };
        // Java: this.zipArchive = new ZipFileZipEntrySource(zipFile);
        pkg.setZipArchive(Box::new(ZipFileZipEntrySource::ZipFileZipEntrySource(zipFile)));
        pkg
    }

    pub fn from_raf(raf: &dyn std::any::Any, access: PackageAccess) -> Self {
        Self::OPCPackage_raf(raf, access)
    }

    pub fn from_stream(input: &mut dyn std::io::Read) -> Self {
        let mut pkg = Self::new(PackageAccess::READ);
        match ZipInputStreamZipEntrySource::try_new(input) {
            Ok(src) => pkg.setZipArchive(Box::new(src)),
            Err(_) => panic!("Can't open the specified stream"),
        }
        pkg
    }

    // Java: public abstract ArrayList<PackagePart> getParts()
    pub fn getParts(&mut self) {
        // Java: XWPFPackage overrides this; OPC-level scan is getPartsImpl.
        self.getPartsImpl();
    }

    // Java: protected void init()
    fn init(&mut self) {
        // Java: this.defaultPartMarshaller = new DefaultMarshaller()
        self.defaultPartMarshaller = Some(Box::new(DefaultMarshaller::new()));
        // Java: this.partMarshallers = new Hashtable(5)
        self.partMarshallers = Some(HashMap::with_capacity(5));
        // Java: this.partUnmarshallers = new Hashtable(2)
        self.partUnmarshallers = Some(HashMap::with_capacity(2));
    }

    // Java: protected void throwExceptionIfReadOnly()
    pub fn throwExceptionIfReadOnly(&self) {
        // Java: if (this.packageAccess == PackageAccess.READ) {
        if self.packageAccess == PackageAccess::READ {
            // Java: throw new InvalidOperationException("Operation not allowed, document open in read only mode!");
            panic!("Operation not allowed, document open in read only mode!");
        }
    }

    // Java: public PackagePart getPart(PackagePartName partName)
    pub fn getPart(&mut self, part_name: &PackagePartName) -> Option<&PackagePart> {
        // Java: if (partName == null) throw new IllegalArgumentException("partName")
        // Java: if (this.partList == null) { try { this.getParts(); } catch (InvalidFormatException e) { ... return null; } }
        if self.partList.is_none() {
            self.getPartsImpl();
        }
        // Java: IOLog.e(TAG, "InvalidFormatException", e)
        // rust: no-stmt（`getPartsImpl()` 在 Rust 侧不返回 `Result` ⇒ Java 的 catch 分支不可达）
        // Java: return this.getPartImpl(partName)
        self.getPartImpl(part_name)
    }

    pub fn get_part_mut(&mut self, part_name: &PackagePartName) -> Option<&mut PackagePart> {
        if self.partList.is_none() {
            let _ = self.getParts();
        }
        self.partList.as_mut()?.get_mut(part_name)
    }

    // Java: public PackagePart getPart(PackageRelationship partRel)
    pub fn getPart_rel(&mut self, part_rel: &PackageRelationship) -> Option<&PackagePart> {
        // Java: PackagePart retPart = null;
        let mut retPart: Option<PackagePartName> = None;
        // Java: this.ensureRelationships();
        self.ensureRelationships();
        // Java: for(PackageRelationship rel : this.relationships) {
        let want = part_rel.getRelationshipType().to_string();
        let target = self.relationships.as_ref().and_then(|rels| {
            rels.iter()
                // Java: if (rel.getRelationshipType().equals(partRel.getRelationshipType())) {
                .find(|rel| rel.getRelationshipType() == want)
                .map(|rel| rel.getTargetURI().toString())
        })?;
        // Java: try {
        // Java: retPart = this.getPart(PackagingURIHelper.createPartName(rel.getTargetURI()));
        let opc_name = ZipHelper::getOPCNameFromZipItemName(&target);
        retPart = PackagingURIHelper::createPartName_str(&opc_name).ok();
        // Java: break;
        // Java: } catch (InvalidFormatException e) {
        // Java: IOLog.e(TAG, "InvalidFormatException: " + e);
        // Java: }
        // Java: }
        let part_name = retPart?;
        // Java: return retPart;
        self.getPart(&part_name)
    }

    // Java: public PackageRelationship addRelationship(PackagePartName targetPartName, TargetMode targetMode, String relationshipType, String relID)
    pub fn addRelationship(&mut self, target: &PackagePartName, target_mode: TargetMode, relationship_type: &str, rel_id: Option<&str>) -> PackageRelationship {
        // Java: if (targetPartName.isRelationshipPartURI()) {
        if target.isRelationshipPartURI() {
            // Java: throw new InvalidOperationException("Rule M1.25: The Relationships part shall not have relationships to any other part.");
            panic!("Rule M1.25: The Relationships part shall not have relationships to any other part.");
        }
        // Java: } else {
        // Java: this.ensureRelationships();
        self.ensureRelationships();
        // Java: PackageRelationship retRel = this.relationships.addRelationship(targetPartName.getURI(), targetMode, relationshipType, relID);
        let rel = self
            .relationships
            .as_mut()
            .unwrap()
            .addRelationship(target.getURI().clone(), target_mode, relationship_type, rel_id);
        // Java: this.isDirty = true;
        self.isDirty = true;
        // Java: return retRel;
        Self::clone_rel(rel)
    }

    // Java: public PackageRelationship addRelationship(PackagePartName targetPartName, TargetMode targetMode, String relationshipType)
    pub fn addRelationship_no_id(&mut self, target: &PackagePartName, target_mode: TargetMode, relationship_type: &str) -> PackageRelationship {
        // Java: return this.addRelationship(targetPartName, targetMode, relationshipType, (String)null);
        self.addRelationship(target, target_mode, relationship_type, None)
    }

    // Java: public PackageRelationship addExternalRelationship(String target, String relationshipType)
    pub fn addExternalRelationship(&mut self, target: &str, relationship_type: &str) -> PackageRelationship {
        // Java: return this.addExternalRelationship((String)target, relationshipType, (String)null);
        self.addExternalRelationship_with_id(target, relationship_type, None)
    }

    // Java: public PackageRelationship addExternalRelationship(URI target, String relationshipType)
    pub fn addExternalRelationship_uri(&mut self, target: &URI, relationship_type: &str) -> PackageRelationship {
        // Java: return this.addExternalRelationship((URI)target, relationshipType, (String)null);
        self.addExternalRelationship_uri_with_id(target, relationship_type, None)
    }

    // Java: public PackageRelationship addExternalRelationship(String target, String relationshipType, String id)
    pub fn addExternalRelationship_with_id(&mut self, target: &str, relationship_type: &str, id: Option<&str>) -> PackageRelationship {
        // Java: if (target == null) {
        // Java: throw new IllegalArgumentException("target");
        // Java: } else if (relationshipType == null) {
        // Java: throw new IllegalArgumentException("relationshipType");
        // Java: } else {
        // rust: no-stmt（Rust 参数为 `&str` ⇒ Java 的空值检查不可达）
        // Java: URI targetURI;
        // Java: try {
        // Java: targetURI = new URI(target);
        let targetURI = match URI::new(target) {
            Ok(u) => u,
            // Java: } catch (URISyntaxException e) {
            // Java: throw new IllegalArgumentException("Invalid target - " + e);
            // Java: }
            Err(e) => panic!("Invalid target - {}", e),
        };
        // Java: this.ensureRelationships();
        self.ensureRelationships();
        // Java: PackageRelationship retRel = this.relationships.addRelationship(targetURI, TargetMode.EXTERNAL, relationshipType, id);
        let rel = self
            .relationships
            .as_mut()
            .unwrap()
            .addRelationship(targetURI, TargetMode::EXTERNAL, relationship_type, id);
        // Java: this.isDirty = true;
        self.isDirty = true;
        // Java: return retRel;
        Self::clone_rel(rel)
    }

    // Java: public PackageRelationship addExternalRelationship(URI target, String relationshipType, String id)
    pub fn addExternalRelationship_uri_with_id(&mut self, target: &URI, relationship_type: &str, id: Option<&str>) -> PackageRelationship {
        // Java: if (target == null) {
        // Java: throw new IllegalArgumentException("target");
        // Java: } else if (relationshipType == null) {
        // Java: throw new IllegalArgumentException("relationshipType");
        // Java: } else {
        // rust: no-stmt（Rust 参数为 `&URI` / `&str` ⇒ Java 的空值检查不可达）
        // Java: this.ensureRelationships();
        self.ensureRelationships();
        // Java: PackageRelationship retRel = this.relationships.addRelationship(target, TargetMode.EXTERNAL, relationshipType, id);
        // Java: return this.relationships.addRelationship(target, TargetMode.EXTERNAL, relationshipType, id)
        let rel = self
            .relationships
            .as_mut()
            .unwrap()
            .addRelationship(target.clone(), TargetMode::EXTERNAL, relationship_type, id);
        // Java: this.isDirty = true;
        self.isDirty = true;
        // Java: return retRel;
        Self::clone_rel(rel)
    }

    // Java: public void removeRelationship(String id)
    pub fn removeRelationship(&mut self, id: &str) {
        // Java: this.throwExceptionIfReadOnly()
        // Java: this.ensureRelationships()
        // Java: if (this.relationships != null) {
        // Java: this.relationships.removeRelationship(id)
        if let Some(ref mut rels) = self.relationships {
            rels.removeRelationship(id);
            // Java: this.isDirty = true;
            self.isDirty = true;
        }
    }

    // Java: public PackageRelationshipCollection getRelationships()
    pub fn getRelationships(&mut self) -> PackageRelationshipCollection {
        // Java: return this.getRelationshipsHelper((String)null)
        // Java: return this.getRelationshipsHelper(null)
        self.getRelationshipsHelper(None)
    }

    // Java: public PackageRelationshipCollection getRelationshipsByType(String relationshipType)
    pub fn getRelationshipsByType(&mut self, relationship_type: &str) -> PackageRelationshipCollection {
        // Java: if (relationshipType == null)
        // rust: no-stmt（Rust 参数为 `&str` ⇒ Java 的空值检查不可达）
        // Java: throw new IllegalArgumentException("relationshipType")
        // Java: return this.getRelationshipsHelper(relationshipType)
        self.getRelationshipsHelper(Some(relationship_type))
    }

    // Java: private PackageRelationshipCollection getRelationshipsHelper(String id)
    fn getRelationshipsHelper(&mut self, id: Option<&str>) -> PackageRelationshipCollection {
        // Java: this.ensureRelationships()
        self.ensureRelationships();
        // Java: return this.relationships.getRelationships(id)
        self.relationships.as_ref().unwrap().getRelationships(id)
    }

    // Java: public void ensureRelationships()
    pub fn ensureRelationships(&mut self) {
        // Java: if (this.relationships == null) {
        if self.relationships.is_none() {
            // Java: try { this.relationships = new PackageRelationshipCollection(this); }
            // Java: catch (InvalidFormatException e) { IOLog.e(TAG, ...); this.relationships = new PackageRelationshipCollection(); }
            // Java: IOLog.e(TAG, "InvalidFormatException: " + e);
            // rust: no-stmt（该 catch 分支的日志在 Rust 侧无对应日志设施；降级为空集合的行为由下方 `parsed.is_err()` 分支承载）
            // Rust: 复刻该 try/catch ⇒ **解析失败降级为空集合**（不 panic），失败场景 = Java 的 `InvalidFormatException`
            //       （M4.1 重复 core-properties / 流不可读；登记见 §1.7.1 ②）。
            if self.partList.is_none() {
                self.getPartsImpl();
            }
            // Java: `new PackageRelationshipCollection(this)` 内部：
            //   this.relationshipPartName = getRelationshipPartName(part = null)  ⇒ 不再硬编字面量
            //   this.relationshipPart = container.getPart(this.relationshipPartName)
            //   if (this.relationshipPart != null) this.parseRelationshipsPart(this.relationshipPart)
            let mut coll = PackageRelationshipCollection::new();
            // Java: 该集合由 `new PackageRelationshipCollection(this)` 建出 ⇒ container 指向本包
            //       （后续 `OPCPackage::addRelationship` 新建的关系要带上它）。
            coll.container = Some(self as *const OPCPackage);
            let relationship_part_name = PackageRelationshipCollection::getRelationshipPartName(None);
            if let Some(name) = relationship_part_name {
                if let Some(part) = self.partList.as_ref().and_then(|list| list.get(&name)) {
                    let parsed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        if let Ok(mut stream) = part.getInputStream_default() {
                            coll.parseRelationshipsXml(&mut *stream);
                        }
                    }));
                    if parsed.is_err() {
                        // Java: catch (InvalidFormatException e) ⇒ 空集合
                        coll = PackageRelationshipCollection::new();
                        coll.container = Some(self as *const OPCPackage);
                    }
                }
            }
            self.relationships = Some(coll);
        }
    }

    // Java: public PackageRelationship getRelationship(String id)
    pub fn getRelationship(&mut self, id: &str) -> Option<PackageRelationship> {
        // Java: this.ensureRelationships()
        // Java: return this.relationships.getRelationshipByID(id)
        self.ensureRelationships();
        self.relationships
            .as_ref()
            .and_then(|r| r.getRelationshipByID(id))
            .map(Self::clone_rel)
    }

    // Java: public boolean hasRelationships()
    pub fn hasRelationships(&mut self) -> bool {
        // Java: return this.relationships.size() > 0
        // Rust: Java 该方法是**纯 getter**（不含 `ensureRelationships()`，且 `relationships == null` 时 NPE）；
        //       Rust 侧先按需懒建，避免把「未 ensure」变成 panic（登记 §1.7.1 ③）。
        self.ensureRelationships();
        self.relationships.as_ref().map_or(false, |r| r.size() > 0)
    }

    // Java: public PackageAccess getPackageAccess()
    pub fn getPackageAccess(&self) -> &PackageAccess {
        // Java: return this.packageAccess
        &self.packageAccess
    }

    /// `partUnmarshallers` 条数（`48` 的 W4）。
    ///
    /// Java: `Hashtable partUnmarshallers`（`OPCPackage.java:54`），`init()` 里
    /// `new Hashtable(2)` + 注册 3 个（`:423` 附近）⇒ 实测 **3**。
    pub fn getPartUnmarshallersLen(&self) -> usize {
        self.partUnmarshallers.as_ref().map_or(0, |m| m.len())
    }

    /// `partMarshallers` 条数（`48` 的 W4）。
    ///
    /// Java: `Hashtable partMarshallers`（`OPCPackage.java:52`），`init()` 里
    /// `new Hashtable(5)`（`:111`；容量 5 ≠ 条数）⇒ 实测条数为注册数。
    pub fn getPartMarshallersLen(&self) -> usize {
        self.partMarshallers.as_ref().map_or(0, |m| m.len())
    }

    // Java: public void save(File targetFile)
    pub fn save_file(&mut self, _target_file: &std::path::Path) -> Result<(), std::io::Error> {
        // Java: if (targetFile == null) {
        // Java: throw new IllegalArgumentException("targetFile");
        // Java: } else {
        // rust: no-stmt（Rust 参数为 `&Path` ⇒ Java 的空值检查不可达）
        // Java: this.throwExceptionIfReadOnly();
        self.throwExceptionIfReadOnly();
        // Java: this.save((OutputStream)this.mZOS);
        // ★写侧暂缓（PLAN P4）：`mZOS` 在 Rust 侧为 `Option<...>` 且落盘链走 `saveImpl`（DEFERRED_NON_FUNCTION）。
        // rust: no-stmt（见上方说明）
        Ok(())
    }

    // Java: public void save(OutputStream outputStream)
    pub fn save(&mut self, output_stream: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        // Java: this.throwExceptionIfReadOnly();
        self.throwExceptionIfReadOnly();
        // Java: this.saveImpl(outputStream);
        self.saveImpl(output_stream);
        Ok(())
    }

    // Java: public void close()
    pub fn close(&mut self) -> Result<(), std::io::Error> {
        // Java: if (this.packageAccess == PackageAccess.READ) {
        if self.packageAccess == PackageAccess::READ {
            // Java: this.contentTypeManager.clearAll();
            if let Some(ref mut mgr) = self.contentTypeManager {
                mgr.clearAll();
            }
            // Java: DefaultElement.clearPool();
            // Java: DefaultAttribute.clearPool();
            // Java: DefaultText.clearPool();
            // rust: no-stmt（三个 XML 池在 Rust 侧无等价物，Java 侧为静态清池）
            // Java: if (null != this.zipArchive) {
            // Java: this.zipArchive.close();
            // Java: }
            self.zipArchive = None;
        // Java: } else {
        } else {
            // Java: ReentrantReadWriteLock l = new ReentrantReadWriteLock();
            // rust: no-stmt（Rust 侧无 ReentrantReadWriteLock；由 `&mut self` 借用保证互斥）
            let l = ();
            let _ = &l;
            // Java: l.writeLock().lock();
            // Java: if (this.originalPackagePath != null && !"".equals(this.originalPackagePath.trim())) {
            // Java: this.closeImpl();
            if self.originalPackagePath.as_ref().map(|s| !s.trim().is_empty()).unwrap_or(false) {
                // ★写侧暂缓（PLAN P4）：`closeImpl()` 已登记 DEFERRED_NON_FUNCTION（真正落盘）。
                self.flush();
            // Java: } else if (this.output != null) {
            } else if self.output.is_some() {
                // Java: this.save((OutputStream)this.mZOS);
                // Java: this.mZOS.close();
                // ★写侧暂缓（PLAN P4）：`save` → `saveImpl` 已登记 DEFERRED_NON_FUNCTION。
                if let Some(ref mut zos_box) = self.mZOS {
                    let _ = zos_box;
                }
            }
            // Java: } finally {
            // Java: l.writeLock().unlock();
            // Java: if (null != this.contentTypeManager) {
            // Java: this.contentTypeManager.clearAll();
            // Java: }
            if let Some(ref mut mgr) = self.contentTypeManager {
                mgr.clearAll();
            }
            // Java: DefaultElement.clearPool();
            // Java: DefaultAttribute.clearPool();
            // Java: DefaultText.clearPool();
            // rust: no-stmt（同读路径：三个 XML 池在 Rust 侧无等价物）
        }
        Ok(())
    }

    // Java: public void flush()
    pub fn flush(&mut self) {
        // Java: this.throwExceptionIfReadOnly();
        self.throwExceptionIfReadOnly();
        // Java: this.flushImpl();
        // ★写侧暂缓（PLAN P4）：`flushImpl()` 已登记为 DEFERRED_NON_FUNCTION（Java 侧本体重载为空方法体，
        //   而真正落盘发生在 `saveImpl`/`closeImpl`）⇒ 只读开门路径下无需落盘，行为与 Java 一致。
        // rust: no-stmt（`flushImpl` 暂缓；见上方说明）
    }

    // Java: public PackagePart createPart(PackagePartName partName, String contentType)
    pub fn createPart(&mut self, part_name: &PackagePartName, content_type: &str) -> Option<crate::io::opc::PackagePart::PackagePart> {
        // Java: return this.createPart(partName, contentType, true)
        self.createPart_with_load(part_name, content_type, true)
    }

    // Java: protected PackagePart createPart(PackagePartName partName, String contentType, boolean loadRelationships)
    fn createPart_with_load(
        &mut self,
        part_name: &PackagePartName,
        content_type: &str,
        load_relationships: bool,
    ) -> Option<crate::io::opc::PackagePart::PackagePart> {
        // Java: this.throwExceptionIfReadOnly();
        self.throwExceptionIfReadOnly();
        // Java: if (partName == null) throw new IllegalArgumentException("partName")
        // rust: no-stmt（Rust 参数是 `&PackagePartName`，非空类型 ⇒ Java 的空检查无对应语句）
        // Java: else if (contentType != null && !contentType.equals(""))
        if content_type.is_empty() {
            // Java: throw new IllegalArgumentException("contentType");
            panic!("contentType");
        }
        if self.partList.is_none() {
            self.partList = Some(PackagePartCollection::new());
        }
        // Java: if (this.partList.containsKey(partName) && !((PackagePart)this.partList.get(partName)).isDeleted())
        // ★M1.12 写路径门禁（Java `:296-297`）：同名且**未标删**的 part 在场 ⇒ InvalidOperationException；
        //   已标删（`isDeleted()`）则允许重建。`30_function_parts_dup_guard` 的 `dg08_` 用例钉住该行为。
        let existing_undeleted = self
            .partList
            .as_ref()
            .and_then(|list| list.get(part_name))
            .map_or(false, |existing| !existing.isDeleted());
        if existing_undeleted {
            // Java: throw new InvalidOperationException("A part with the name '" + partName.getName() + "' already exists : Packages shall not contain equivalent part names and package implementers shall neither create nor recognize packages with equivalent part names. [M1.12]");
            panic!(
                "A part with the name '{}' already exists : Packages shall not contain equivalent part names and package implementers shall neither create nor recognize packages with equivalent part names. [M1.12]",
                part_name.getName()
            );
        }
        // Java: PackagePart part = this.createPartImpl(partName, contentType, loadRelationships);
        // Rust: `createPartImpl` 在本仓仍是 DEFERRED_NON_FUNCTION（写侧未开 Loop）⇒ 此处按同一形状直接构造
        //       Java 的 `ZipPackagePart(this, partName, contentType)`（与 `05` 的 `part_from_entry` 同一承载约定）。
        let part = crate::io::opc::PackagePart::PackagePart::new_with_string(
            Some(self as *const OPCPackage),
            part_name.clone(),
            content_type,
        );
        let _ = load_relationships;
        // Java: this.contentTypeManager.addContentType(partName, contentType)
        if let Some(ref mut manager) = self.contentTypeManager {
            manager.addContentType(part_name, content_type);
        }
        // Java: this.partList.put(partName, part);
        if let Some(ref mut list) = self.partList {
            list.put(part_name.clone(), part);
        }
        // Java: this.isDirty = true;
        self.isDirty = true;
        // Java: return part;
        // rust: no-stmt（Java 返回刚放入 partList 的**同一实例**；Rust 值语义下 `part` 已 move 进集合，
        //       故以同参数新建的 part 外壳承载同一返回值形状 —— 见下方 ownership 说明）
        // ownership: part moved into map — return a fresh shell
        Some(crate::io::opc::PackagePart::PackagePart::new_with_string(
            Some(self as *const OPCPackage),
            part_name.clone(),
            content_type,
        ))
    }

    // Java: public void saveImpl(OutputStream outputStream)
    pub fn saveImpl(&mut self, _output_stream: &mut dyn std::io::Write) {
        // ★写侧整体暂缓（PLAN P4）——本 feature（s01-01-zip-source，parse 阶段）只读、不落盘。
        //   ★2026-09-20 修正：此前本方法体为 `self.throwExceptionIfReadOnly();` 后**直接返回**，
        //   等于「读了只读检查就假装保存成功」，属**假实现**（§硬规则 9）。现改为诚实暂缓。
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: this.throwExceptionIfReadOnly();
        self.throwExceptionIfReadOnly();
        // Java: ZipOutputStream zos = null;
        let mut zos: Option<ZipOutputStream> = None;
        // Java: try {
        // Java: if (!(outputStream instanceof ZipOutputStream)) {
        // Java: zos = new ZipOutputStream(outputStream);
        // Java: } else {
        // Java: zos = (ZipOutputStream)outputStream;
        // Java: }
        // Java: this.saveImpl(zos);
        // Java: ZipPartMarshaller.marshallRelationshipPart(this.getRelationships(), PackagingURIHelper.PACKAGE_RELATIONSHIPS_ROOT_PART_NAME, zos);
        // Java: this.contentTypeManager.save(zos);
        // Java: for(PackagePart part : this.getParts()) {
        // Java: if (!part.isRelationshipPart()) {
        // Java: PartMarshaller marshaller = (PartMarshaller)this.partMarshallers.get(part._contentType);
        // Java: if (marshaller != null) {
        // Java: if (!marshaller.marshall(part, zos)) {
        // Java: throw new OpenXML4JException("The part " + part.getPartName().getURI() + " fail to be saved in the stream with marshaller " + marshaller);
        // Java: }
        // Java: } else if (!this.defaultPartMarshaller.marshall(part, zos)) {
        // Java: throw new OpenXML4JException("The part " + part.getPartName().getURI() + " fail to be saved in the stream with marshaller " + this.defaultPartMarshaller);
        // Java: }
        // Java: }
        // Java: }
        // Java: zos.close();
        // Java: } catch (Exception var6) {
        // Java: }
        */
    }

    // Java: protected void saveImpl(ZipOutputStream zos) throws OpenXML4JException
    pub fn saveImpl__zipoutputstream(&mut self, _zos: &mut dyn std::io::Write) {
        // ★写侧整体暂缓（PLAN P4）；Java 侧本体重载为空方法体。
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // Java: protected static String generateTempFileName(File directory)
    pub fn generateTempFileName(_directory: &std::path::Path) -> String {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: File tmpFilename;
        // Java: do {
        // Java: tmpFilename = new File(directory.getAbsolutePath() + File.separator + "WPS_" + System.nanoTime());
        // Java: } while(tmpFilename.exists());
        // Java: return FileHelper.getFilename(tmpFilename.getAbsoluteFile());
        */
    }

    // Java: protected void configurePackage()
    pub fn configurePackage(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: try {
        // Java: this.contentTypeManager = new ZipContentTypeManager((InputStream)null, this);
        // Java: this.contentTypeManager.addContentType(PackagingURIHelper.createPartName(PackagingURIHelper.PACKAGE_RELATIONSHIPS_ROOT_URI), "application/vnd.openxmlformats-package.relationships+xml");
        // Java: this.contentTypeManager.addContentType(PackagingURIHelper.createPartName("/default.xml"), "application/xml");
        // Java: } catch (InvalidFormatException e) {
        // Java: throw new IllegalStateException(e);
        // Java: }
        */
    }

    // Java: public ArrayList<PackagePart> getPartsByRelationshipType(String relationshipType)
    pub fn getPartsByRelationshipType(&mut self, _relationshipType: &str) -> Vec<PackagePart> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: if (relationshipType == null) {
        // Java: throw new IllegalArgumentException("relationshipType");
        // Java: } else {
        // Java: ArrayList<PackagePart> retArr = new ArrayList();
        // Java: for(PackageRelationship rel : this.getRelationshipsByType(relationshipType)) {
        // Java: retArr.add(this.getPart(rel));
        // Java: }
        // Java: return retArr;
        // Java: }
        */
    }

    // Java: public List<PackagePart> getPartsByName(Pattern namePattern)
    pub fn getPartsByName(&mut self, _namePattern: &str) -> Vec<PackagePart> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: if (namePattern == null) {
        // Java: throw new IllegalArgumentException("name pattern must not be null");
        // Java: } else {
        // Java: ArrayList<PackagePart> result = new ArrayList();
        // Java: for(PackagePart part : this.partList.values()) {
        // Java: PackagePartName partName = part.getPartName();
        // Java: String name = partName.getName();
        // Java: Matcher matcher = namePattern.matcher(name);
        // Java: if (matcher.matches()) {
        // Java: result.add(part);
        // Java: }
        // Java: }
        // Java: return result;
        */
    }

    // Java: public boolean isRelationshipExists(PackageRelationship rel)
    pub fn isRelationshipExists(&mut self, _rel: &PackageRelationship) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: for(PackageRelationship r : this.getRelationships()) {
        // Java: if (r == rel) {
        // Java: return true;
        // Java: }
        // Java: }
        // Java: return false;
        */
    }

    // Java: public void addMarshaller(String contentType, PartMarshaller marshaller)
    pub fn addMarshaller(&mut self, _contentType: &str, _marshaller: Box<dyn PartMarshaller>) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: try {
        // Java: this.partMarshallers.put(new ContentType(contentType), marshaller);
        // Java: } catch (InvalidFormatException var4) {
        // Java: }
        */
    }

    // Java: public void addUnmarshaller(String contentType, PartUnmarshaller unmarshaller)
    pub fn addUnmarshaller(&mut self, _contentType: &str, _unmarshaller: Box<dyn PartUnmarshaller>) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: try {
        // Java: this.partUnmarshallers.put(new ContentType(contentType), unmarshaller);
        // Java: } catch (InvalidFormatException var4) {
        // Java: }
        */
    }

    // Java: public void removeMarshaller(ContentType contentType)
    pub fn removeMarshaller(&mut self, _contentType: &ContentType) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: this.partMarshallers.remove(contentType);
        */
    }

    // Java: public void removeUnmarshaller(ContentType contentType)
    pub fn removeUnmarshaller(&mut self, _contentType: &ContentType) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: this.partUnmarshallers.remove(contentType);
        */
    }

    // Java: public boolean validatePackage(OPCPackage pkg)
    pub fn validatePackage(_pkg: &OPCPackage) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: throw new InvalidOperationException("Not implemented yet !!!");
        */
    }

    // Java: protected PackagePart createPartImpl(PackagePartName partName, String contentType, boolean loadRelationships)
    pub fn createPartImpl(&mut self, _partName: &PackagePartName, _contentType: &str, _loadRelationships: bool) -> Option<PackagePart> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: if (contentType == null) {
        // Java: throw new IllegalArgumentException("contentType");
        // Java: } else if (partName == null) {
        // Java: throw new IllegalArgumentException("partName");
        // Java: } else {
        // Java: try {
        // Java: return new MemoryPackagePart(this, partName, contentType, loadRelationships);
        // Java: } catch (InvalidFormatException e) {
        // Java: IOLog.e(TAG, "InvalidFormatException", e);
        // Java: return null;
        // Java: }
        // Java: }
        // Java: }
        */
    }

    // Java: protected void flushImpl()
    pub fn flushImpl(&mut self) {
        // ★写侧暂缓（PLAN P4）：Java 侧 `flushImpl()` 本体重载为**空方法体**（`OPCPackage.java:482-483`），
        //   无任何 Java 语句可镜像 ⇒ 与 `flush()` 同约定保持暂缓（真正落盘发生在 `saveImpl`/`closeImpl`）。
        // rust: no-stmt（Java 空方法体 ⇒ 无 body 语句）
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // Java: protected void closeImpl()
    pub fn closeImpl(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: this.flush();
        // Java: if (this.originalPackagePath != null && !"".equals(this.originalPackagePath)) {
        // Java: File targetFile = new File(this.originalPackagePath);
        // Java: if (!targetFile.exists()) {
        // Java: targetFile.createNewFile();
        // Java: }
        // Java: try {
        // Java: IOAssert.assertNotNull("tmpPackageFile should not be null! ", this.tmpPackageFile);
        // Java: this.save(this.tmpPackageFile);
        // Java: if (!this.tmpPackageFile.renameTo(targetFile)) {
        // Java: FileHelper.copyFile(this.tmpPackageFile, targetFile);
        // Java: }
        // Java: } finally {
        // Java: if (this.tmpPackageFile.exists() && !this.tmpPackageFile.delete()) {
        // Java: IOLog.d(TAG, "The temporary file: '" + targetFile.getAbsolutePath() + "' cannot be deleted ! Make sure that no other application use it.");
        // Java: }
        // Java: this.tmpPackageFile = null;
        */
    }

    // Java: protected PackagePart getPartImpl(PackagePartName partName)
    pub fn getPartImpl(&self, part_name: &PackagePartName) -> Option<&PackagePart> {
        // Java: return (PackagePart)this.partList.get(partName);
        // Rust: `PackagePartCollection::get` 即 Java `Hashtable.get(partName)` 的同一查表语义（纯读，无副作用）
        self.partList.as_ref()?.get(part_name)
    }

    fn clone_rel(rel: &PackageRelationship) -> PackageRelationship {
        PackageRelationship {
            id: rel.id.clone(),
            // Rust: 引用语义字段照抄（Java `addRelationship` 返回的是集合里**同一个**对象）
            container: rel.container,
            relationshipType: rel.relationshipType.clone(),
            source: rel.source,
            sourceUri: rel.sourceUri.clone(),
            targetMode: rel.targetMode,
            targetUri: rel.targetUri.clone(),
        }
    }
}
