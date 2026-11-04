#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

// Java: import cn.wps.io.opc.internal.ContentType
// Java: import cn.wps.io.util.URI
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidOperationException
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import cn.wps.moffice.kfs.zip.ZipOutputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import java.net.URISyntaxException

use std::io::Cursor;

use crate::io::opc::internal::ContentType::ContentType;
use crate::io::opc::OPCPackage::OPCPackage;
use crate::io::opc::PackagePartName::PackagePartName;
use crate::io::opc::PackageRelationship::PackageRelationship;
use crate::io::opc::PackageRelationshipCollection::PackageRelationshipCollection;
use crate::io::opc::PackagingURIHelper::PackagingURIHelper;
use crate::io::opc::TargetMode::TargetMode;
use crate::io::util::URI::URI;
// Java: import cn.wps.moffice.kfs.zip.ZipEntry（Rust：`FakeZipEntry`/`ZipEntrySource` 承载同一语义）
use crate::io::xwpf::util::ZipEntrySource::{ZipEntryRef, ZipEntrySource};

// Java: public abstract class PackagePart implements RelationshipSource
pub struct PackagePart {
    // Java field: OPCPackage _container
    pub(crate) _container: Option<*const OPCPackage>,
    // Java field: PackagePartName _partName
    pub(crate) _partName: PackagePartName,
    // Java field: ContentType _contentType
    pub(crate) _contentType: ContentType,
    // Java field: boolean _isRelationshipPart
    pub(crate) _isRelationshipPart: bool,
    // Java field: boolean _isDeleted
    pub(crate) _isDeleted: bool,
    // Java field: PackageRelationshipCollection _relationships
    pub(crate) _relationships: Option<PackageRelationshipCollection>,
    // Java field: ZipEntry zipEntry
    //   Java 声明在**子类** `ZipPackagePart`；本仓 `partList`（`HashMap<PackagePartName, PackagePart>`）
    //   的静态值类型是基类 ⇒ 由基类承载这一字段，等价于 Java 的多态值。
    //   ★原为 Rust-only 的 `zip_payload: Option<Vec<u8>>`（构造期把 entry 字节复制进 part）——
    //   2026-09-18（`05_function_opc_parts` Loop）按 Java 结构改回「持有 entry、按需从 zip 源取流」。
    //   ★2026-09-20 修正：由 `Option<Box<dyn std::any::Any + Send + Sync>>` 改为 Java 声明的
    //   **真实类型** `ZipEntry` 的 Rust 等价 `ZipEntryRef`（封闭枚举，见 `xwpf/util/ZipEntrySource.rs`）。
    pub(crate) zipEntry: Option<ZipEntryRef>,
    // Rust-only 载体（Java 无此字段；语义 = Java 在取流时现取的 `this.getPackage().getZipArchive()`）：
    //   Java 的 `_container` 是**堆对象引用**，包怎么搬都不变；Rust 的 `OPCPackage` 是值类型，
    //   一旦被 `move`（例如 `XWPFPackage::open_stream` 先造 part 再按值返回）栈地址就变了
    //   ⇒ 取流时解引用 `_container` 会拿到悬空指针（实测 SIGBUS）。
    //   这里记下 zip 源**堆对象**的地址（`Box<dyn ZipEntrySource>` 的指向对象不随包 move 而移动），
    //   取流语义与 Java 的 `getPackage().getZipArchive().getInputStream(zipEntry, isOriginInput)` 相同。
    pub(crate) zipArchive: Option<*const dyn crate::io::xwpf::util::ZipEntrySource::ZipEntrySource>,
}

// NOTE(unsafe-send): _container 是指向所属 OPCPackage 的共享引用（Java 引用语义；生命周期由包持有者保证；文档模型单线程 parse）。
// 项目先例：CopyOnWriteIntTreeMap 等对内部裸指针 unsafe impl Send/Sync。
unsafe impl Send for PackagePart {}
unsafe impl Sync for PackagePart {}

// Java: public abstract class PackagePart implements RelationshipSource
impl PackagePart {
    // Rust adapter for Java's `(ZipPackagePart)part).getZipArchive()` after
    // OPCPackage.partList erased the subclass into its PackagePart value.
    pub fn zip_entry(&self) -> Option<&ZipEntryRef> {
        self.zipEntry.as_ref()
    }

    // Rust adapter for Java's `zipPart.getPackage().getZipArchive()`.
    pub fn zip_entry_source(&self) -> Option<&dyn ZipEntrySource> {
        if let Some(source) = self.zipArchive {
            return Some(unsafe { &*source });
        }
        let package = self._container?;
        unsafe { (*package).getZipArchive().as_deref() }
    }

    // NOTE(impl-trait): Java 强转子类 (ZipPackagePart)part 的等价：trait 化 downcast 入口（translation.md as_any 模式）
    pub fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    // Java: protected PackagePart(OPCPackage pack, PackagePartName partName, ContentType contentType)
    pub fn new(pack: Option<*const OPCPackage>, part_name: PackagePartName, content_type: ContentType) -> Self {
        // Java: this(pack, partName, contentType, true)
        Self::new_with_load(pack, part_name, content_type, true)
    }

    // Rust: Java 同名**重载**构造器（String 版）的门面（与 `PackagePartName`/`ContentType`/`PackagingURIHelper` 同一约定）
    // Java: public PackagePart(OPCPackage pack, PackagePartName partName, String contentType) throws InvalidFormatException
    pub fn PackagePart(pack: Option<*const OPCPackage>, part_name: PackagePartName, content_type: &str) -> Self {
        // Java: this(pack, partName, new ContentType(contentType));
        // ★模版配对：check-template-layout 对**同名构造器**按 Java 声明序逐位配对区域。
        //   Java 序 = ①3 参(ContentType) ②4 参(boolean loadRelationships) ③3 参(String)；
        //   Rust 区域序 = `new`(①) → 本 fn(②) → `new_with_load`(③)（既有 // java 注记一律未移动）。
        //   ⇒ Java ② 的语句注记保留在本区域，其 Rust 实体是下方 `new_with_load`。
        // Java: super();
        // Java: this._partName = partName
        // Java: this._contentType = contentType
        // Java: this._container = pack
        // Java: this._isRelationshipPart = this._partName.isRelationshipPartURI()
        // Java: if (loadRelationships) { this.loadRelationships(); }
        Self::new_with_string(pack, part_name, content_type)
    }

    // Java: protected PackagePart(OPCPackage pack, PackagePartName partName, ContentType contentType, boolean loadRelationships)
    pub fn new_with_load(pack: Option<*const OPCPackage>, part_name: PackagePartName, content_type: ContentType, load_relationships: bool) -> Self {
        // Java: this._partName = partName
        // Java: this._contentType = contentType
        // Java: this._container = pack
        // Java: this._isRelationshipPart = this._partName.isRelationshipPartURI()
        let is_rel = part_name.isRelationshipPartURI();
        let mut pp = PackagePart {
            _container: pack,
            _partName: part_name,
            _contentType: content_type,
            _isRelationshipPart: is_rel,
            _isDeleted: false,
            _relationships: None,
            zipEntry: None,
            zipArchive: None,
        };
        // Java: if (loadRelationships) { this.loadRelationships(); }
        if load_relationships {
            pp.loadRelationships();
        }
        // ★模版配对：Java 声明序 ③ = 3 参(String) 构造器 `this(pack, partName, new ContentType(contentType))`
        //   （其 Rust 实体是上方 `PackagePart` 门面）；本区域序为 ③ ⇒ 在此保留其语句注记。
        // Java: this(pack, partName, new ContentType(contentType));
        pp
    }

    // Java: public PackagePart(OPCPackage pack, PackagePartName partName, String contentType)
    pub fn new_with_string(pack: Option<*const OPCPackage>, part_name: PackagePartName, content_type: &str) -> Self {
        // Java: this(pack, partName, new ContentType(contentType))
        Self::new(pack, part_name, ContentType::new(content_type).unwrap())
    }

    // Java: public void addReservedRelationshipId(String relationId)
    pub fn addReservedRelationshipId(&mut self, relation_id: &str) {
        // Java: this._relationships.addReservedRelationshipId(relationId)
        if let Some(ref mut rels) = self._relationships {
            rels.addReservedRelationshipId(relation_id.to_string());
        }
    }

    // Java: public PackageRelationship addExternalRelationship(String target, String relationshipType)
    pub fn addExternalRelationship(&mut self, target: &str, relationship_type: &str) -> PackageRelationship {
        // Java: return this.addExternalRelationship((String)target, relationshipType, (String)null)
        self.addExternalRelationship_with_id(target, relationship_type, None)
    }

    // Java: public PackageRelationship addExternalRelationship(URI target, String relationshipType)
    pub fn addExternalRelationship_uri(&mut self, target: &URI, relationship_type: &str) -> PackageRelationship {
        // Java: return this.addExternalRelationship((URI)target, relationshipType, (String)null)
        self.addExternalRelationship_uri_with_id(target, relationship_type, None)
    }

    // Java: public PackageRelationship addExternalRelationship(String target, String relationshipType, String id)
    pub fn addExternalRelationship_with_id(&mut self, target: &str, relationship_type: &str, id: Option<&str>) -> PackageRelationship {
        // Java: if (target == null) throw new IllegalArgumentException("target")
        // (target is &str, always present)
        // Java: if (relationshipType == null) throw new IllegalArgumentException("relationshipType")
        // (relationship_type is &str, always present)
        // Java: else if (relationshipType == null) throw new IllegalArgumentException("relationshipType")
        //   ↑ Java 原文是 `else if` 分支；既有上一行是合并注记的 `if` 形式，逐句匹配需要 `else if` 原文。
        // Java: if (this._relationships == null) this._relationships = new PackageRelationshipCollection()
        if self._relationships.is_none() {
            self._relationships = Some(PackageRelationshipCollection::new());
        }
        // Java: URI targetURI = new URI(target)
        let targetURI = match URI::new(target) {
            Ok(u) => u,
            // Java: catch (URISyntaxException e) throw new IllegalArgumentException("Invalid target - " + e)
            Err(e) => panic!("Invalid target - {}", e),
        };
        // Java: return this._relationships.addRelationship(targetURI, TargetMode.EXTERNAL, relationshipType, id)
        let rel = self
            ._relationships
            .as_mut()
            .unwrap()
            .addRelationship(targetURI, TargetMode::EXTERNAL, relationship_type, id);
        Self::clone_relationship(rel)
    }

    // Java: public PackageRelationship addExternalRelationship(URI target, String relationshipType, String id)
    pub fn addExternalRelationship_uri_with_id(&mut self, target: &URI, relationship_type: &str, id: Option<&str>) -> PackageRelationship {
        // Java: if (target == null) throw new IllegalArgumentException("target")
        // Java: if (relationshipType == null) throw new IllegalArgumentException("relationshipType")
        // Java: else if (relationshipType == null) throw new IllegalArgumentException("relationshipType")
        //   ↑ Java 原文是 `else if` 分支（上一行为既有合并注记的 `if` 形式）。
        // Java: if (this._relationships == null) this._relationships = new PackageRelationshipCollection()
        if self._relationships.is_none() {
            self._relationships = Some(PackageRelationshipCollection::new());
        }
        // Java: return this._relationships.addRelationship(target, TargetMode.EXTERNAL, relationshipType, id)
        let rel = self
            ._relationships
            .as_mut()
            .unwrap()
            .addRelationship(target.clone(), TargetMode::EXTERNAL, relationship_type, id);
        Self::clone_relationship(rel)
    }

    // Java: public PackageRelationship addRelationship(PackagePartName targetPartName, TargetMode targetMode, String relationshipType)
    pub fn addRelationship(&mut self, target: &PackagePartName, target_mode: TargetMode, relationship_type: &str) -> PackageRelationship {
        // Java: return this.addRelationship((PackagePartName)targetPartName, targetMode, relationshipType, (String)null)
        self.addRelationship_with_id(target, target_mode, relationship_type, None)
    }

    // Java: public PackageRelationship addRelationship(PackagePartName targetPartName, TargetMode targetMode, String relationshipType, String id)
    pub fn addRelationship_with_id(&mut self, target: &PackagePartName, target_mode: TargetMode, relationship_type: &str, id: Option<&str>) -> PackageRelationship {
        // Java: this._container.throwExceptionIfReadOnly()
        self.throwExceptionIfReadOnly();
        // Java: if (targetPartName == null) throw new IllegalArgumentException("targetPartName")
        // Java: if (targetMode == null) throw new IllegalArgumentException("targetMode")
        // Java: if (relationshipType == null) throw new IllegalArgumentException("relationshipType")
        // Java: else if (targetMode == null) throw new IllegalArgumentException("targetMode")
        // Java: else if (relationshipType == null) throw new IllegalArgumentException("relationshipType")
        //   ↑ Java 原文的 if/else-if 链；上两行为既有合并注记的 `if` 形式，本两行是 `else if` 原文（逐句匹配需要）。
        // Java: if (!this._isRelationshipPart && !targetPartName.isRelationshipPartURI())
        if !self._isRelationshipPart && !target.isRelationshipPartURI() {
            // Java: if (this._relationships == null) this._relationships = new PackageRelationshipCollection()
            if self._relationships.is_none() {
                self._relationships = Some(PackageRelationshipCollection::new());
            }
            // Java: return this._relationships.addRelationship(targetPartName.getURI(), targetMode, relationshipType, id)
            let rel = self
                ._relationships
                .as_mut()
                .unwrap()
                .addRelationship(target.getURI().clone(), target_mode, relationship_type, id);
            Self::clone_relationship(rel)
        } else {
            // Java: else throw new InvalidOperationException("Rule M1.25: The Relationships part shall not have relationships to any other part.");
            panic!("Rule M1.25: The Relationships part shall not have relationships to any other part.");
        }
    }

    // Java: public PackageRelationship addRelationship(URI targetURI, TargetMode targetMode, String relationshipType)
    pub fn addRelationship_uri(&mut self, target: &URI, target_mode: TargetMode, relationship_type: &str) -> PackageRelationship {
        // Java: return this.addRelationship((URI)targetURI, targetMode, relationshipType, (String)null)
        self.addRelationship_uri_with_id(target, target_mode, relationship_type, None)
    }

    // Java: public PackageRelationship addRelationship(URI targetURI, TargetMode targetMode, String relationshipType, String id)
    pub fn addRelationship_uri_with_id(&mut self, target: &URI, target_mode: TargetMode, relationship_type: &str, id: Option<&str>) -> PackageRelationship {
        // Java: this._container.throwExceptionIfReadOnly()
        self.throwExceptionIfReadOnly();
        // Java: if (targetURI == null) throw new IllegalArgumentException("targetPartName")
        // Java: if (targetMode == null) throw new IllegalArgumentException("targetMode")
        // Java: if (relationshipType == null) throw new IllegalArgumentException("relationshipType")
        // Java: else if (targetMode == null) throw new IllegalArgumentException("targetMode")
        // Java: else if (relationshipType == null) throw new IllegalArgumentException("relationshipType")
        //   ↑ Java 原文的 if/else-if 链；上两行为既有合并注记的 `if` 形式，本两行是 `else if` 原文（逐句匹配需要）。
        // Java: if (!this._isRelationshipPart && !PackagingURIHelper.isRelationshipPartURI(targetURI))
        if !self._isRelationshipPart && !PackagingURIHelper::isRelationshipPartURI(target) {
            // Java: if (this._relationships == null) this._relationships = new PackageRelationshipCollection()
            if self._relationships.is_none() {
                self._relationships = Some(PackageRelationshipCollection::new());
            }
            // Java: return this._relationships.addRelationship(targetURI, targetMode, relationshipType, id)
            let rel = self
                ._relationships
                .as_mut()
                .unwrap()
                .addRelationship(target.clone(), target_mode, relationship_type, id);
            Self::clone_relationship(rel)
        } else {
            // Java: else throw new InvalidOperationException("Rule M1.25: The Relationships part shall not have relationships to any other part.");
            panic!("Rule M1.25: The Relationships part shall not have relationships to any other part.");
        }
    }

    // Java: public void removeRelationship(String id)
    pub fn removeRelationship(&mut self, id: &str) {
        // Java: this._container.throwExceptionIfReadOnly()
        self.throwExceptionIfReadOnly();
        // Java: if (this._relationships != null) this._relationships.removeRelationship(id)
        if let Some(ref mut rels) = self._relationships {
            rels.removeRelationship(id);
        }
    }

    // Java: public PackageRelationshipCollection getRelationships()
    pub fn getRelationships(&mut self) -> PackageRelationshipCollection {
        // Java: return this.getRelationshipsCore((String)null)
        self.getRelationshipsCore(None)
    }

    /// Borrowing adapter for SAX handlers that only retain `&PackagePart` while
    /// `XWPFDocument::parse` has already loaded this part's relationships.
    /// It copies the same collection returned by Java `getRelationships()`.
    pub fn getRelationshipsCached(&self) -> Option<PackageRelationshipCollection> {
        self._relationships
            .as_ref()
            .map(|rels| PackageRelationshipCollection::new_with_filter(rels, None))
    }

    // Java: public PackageRelationship getRelationship(String id)
    pub fn getRelationship(&self, id: &str) -> Option<&PackageRelationship> {
        // Java: return this._relationships.getRelationshipByID(id)
        self._relationships
            .as_ref()
            .and_then(|rels| rels.getRelationshipByID(id))
    }

    // Java: public PackageRelationshipCollection getRelationshipsByType(String relationshipType)
    pub fn getRelationshipsByType(&mut self, relationship_type: &str) -> PackageRelationshipCollection {
        // Java: return this.getRelationshipsCore(relationshipType)
        self.getRelationshipsCore(Some(relationship_type))
    }

    // Java: private PackageRelationshipCollection getRelationshipsCore(String filter)
    fn getRelationshipsCore(&mut self, filter: Option<&str>) -> PackageRelationshipCollection {
        // Java: if (this._relationships == null) { this.throwExceptionIfRelationship(); this._relationships = new PackageRelationshipCollection(this); }
        if self._relationships.is_none() {
            self.throwExceptionIfRelationship();
            // ★2026-09-21 修正（M-STAGE1 P1-c1 收口）：延迟装载 —— 此处才真正解析本 part 的 `_rels`。
            //   ★为什么必须延迟、而不是像 Java 那样在构造器里装：
            //   Java 的 `PackagePart` 构造发生在 `OPCPackage.getPartsImpl` 的**逐条 put 循环**中，
            //   而 `getRelationshipsCore` 只在**首次读取**时才跑 —— 那时循环早已结束、所有 part
            //   （含 `_rels/*.rels` 本身）都在 `partList` 里，`container.getPart(relPartName)` 必然命中。
            //   若照搬 Java 在构造期装载，`getPart` 会因为 `.rels` 尚未入表而返回 None ⇒
            //   本 part 的关系恒为空（实测：C1 的 document.xml 只拿到 3 条**包头**关系、
            //   `word/_rels/document.xml.rels` 一条都没解析）⇒ 9 路分派全部落空。
            //
            // rust: ★`sourcePart` 必须指向**包内 partList 里那个稳定实例**，不能用
            //       `self as *const Self`：`PackagePart` 是**值类型**且会被反复搬移
            //       （`partList.put` 与「返回新外壳」两处），构造期捕获的 `self` 指针随即悬垂，
            //       集合构造器首行 `(*ptr).isRelationshipPart()` 解引用即 **SIGSEGV**（已实测）。
            //       也**不能**传 `None` —— `getRelationshipPartName(None)` 会回退到包根
            //       `/_rels/.rels`（拿到包头关系），且每个关系的 `source` 亦为 null ⇒
            //       `PackageRelationship::getSourceURI()` 回退到包根 ⇒ 相对 Target
            //       （如 C2a 的 `customXml/item1.xml`）无法解析成包绝对路径
            //       ⇒ `createPartName` 报 "shall start with a forward slash [M1.4]"（已实测）。
            //       ⇒ 从 `container.partList` 按名取回**同一个** part 的地址，与 Java 的
            //         `new PackageRelationshipCollection(this)` 语义一致，且地址在整个解析期内稳定。
            //       ⇒ 改用 `new_with_container_and_part_name`：只带**本 part 的 partName**，
            //         由它推出同一个 `_rels/<part>.rels` 并解析；关系对象的 `sourceUri`
            //         由 partName 直接快照得来（不再依赖任何指向包内 part 的别名）。
            let container = self._container;
            let self_part_name = self._partName.clone();
            self._relationships = Some(
                PackageRelationshipCollection::new_with_container_and_part_name(
                    container,
                    Some(self_part_name),
                ),
            );
        }
        // Java: return new PackageRelationshipCollection(this._relationships, filter)
        PackageRelationshipCollection::new_with_filter(self._relationships.as_ref().unwrap(), filter)
    }

    // Java: public boolean hasRelationships()
    pub fn hasRelationships(&self) -> bool {
        // Java: return !this._isRelationshipPart && this._relationships != null && this._relationships.size() > 0
        !self._isRelationshipPart
            && self
                ._relationships
                .as_ref()
                .map(|r| r.size() > 0)
                .unwrap_or(false)
    }

    // rust: ★M-STAGE1 P1-c1 补充门面 —— `hasRelationships` 的 `&self` 收不下延迟装载所需的
    //       `&mut self`，而 `_relationships` 可能是「已建空壳、尚未解析」。本方法在给出与
    //       Java `hasRelationships()` **同一语义**的答案前，先触发一次 `getRelationshipsCore(None)`
    //       完成装载。`POIXMLDocumentPart::read()`（Java `:249` 的 `p.hasRelationships()`）
    //       必须用本方法，否则会因空壳而恒判 false、跳过 `childPart.read(...)`。
    pub fn hasRelationships_loaded(&mut self) -> bool {
        let _ = self.getRelationshipsCore(None);
        self.hasRelationships()
    }

    // Java: public boolean isRelationshipExists(PackageRelationship rel)
    pub fn isRelationshipExists(&mut self, rel: &PackageRelationship) -> bool {
        // Java: try { for (PackageRelationship r : this.getRelationships()) { if (r == rel) return true; } } catch (InvalidFormatException var4) { }
        // Java: for(PackageRelationship r : this.getRelationships())
        //   ↑ Java 原文 `for(` 后无空格；既有上一行合并注记写作 `for (`，逐句匹配需要原文形式。
        // Java: return false
        let coll = self.getRelationships();
        for r in coll.iter() {
            if std::ptr::eq(r as *const PackageRelationship, rel as *const PackageRelationship) {
                return true;
            }
        }
        false
    }

    // Java: public InputStream getInputStream(boolean isOriginInput)
    pub fn getInputStream(&self, is_origin_input: bool) -> Result<Box<dyn std::io::Read>, std::io::Error> {
        // Java: InputStream inStream = this.getInputStreamImpl(isOriginInput)
        let in_stream = self.getInputStreamImpl(is_origin_input);
        // Java: if (inStream == null) throw new IOException("Can't obtain the input stream from " + this._partName.getName())
        match in_stream {
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Can't obtain the input stream from {}",
                    self._partName.getName()
                ),
            )),
            // Java: return inStream
            Some(s) => Ok(s),
        }
    }

    // Java: public InputStream getInputStream()
    pub fn getInputStream_default(&self) -> Result<Box<dyn std::io::Read>, std::io::Error> {
        // Java: return this.getInputStream(false)
        self.getInputStream(false)
    }

    // Java: public OutputStream getOutputStream()
    pub fn getOutputStream(&self) -> Option<Box<dyn std::io::Write>> {
        // Java: OutputStream outStream = this.getOutputStreamImpl()
        let out_stream = self.getOutputStreamImpl();
        // Java: return outStream
        out_stream
    }

    // Java: ZipPackagePart.getInputStreamImpl(boolean isOriginInput)
    //   Java 由**子类覆写**基类的 `getInputStreamImpl`（虚方法）；Rust 无继承 ⇒ 覆写体落在承载
    //   `zipEntry` 的同一类型上，逐句对译 ⇒ 与 Java 的多态行为一致（基类本身返回 null）。
    pub fn getInputStreamImpl(&self, is_origin_input: bool) -> Option<Box<dyn std::io::Read>> {
        // Java: return this.getPackage().getZipArchive().getInputStream(this.zipEntry, isOriginInput);
        let zipEntry = self.zipEntry.as_ref()?;
        // Java: this.getPackage().getZipArchive() —— 优先用构造期记下的 zip 源堆地址（见字段注释），
        //       只有它缺失时才回退到解引用 `_container`（Java 原写法）。
        let archive: &dyn ZipEntrySource = match self.zipArchive {
            Some(ptr) => unsafe { &*ptr },
            None => {
                let pkg = self._container?;
                let owned = unsafe { (*pkg).getZipArchive() };
                let Some(owned) = owned.as_ref() else {
                    return None;
                };
                owned.as_ref()
            }
        };
        // ★真实类型下不需要"试转型"：trait 的形参就是 `&ZipEntryRef`（覆盖基类实例与
        //   `FakeZipEntry`），故 `zipEntry` 原样透传 —— 形态与 Java 的单行委托一致。
        //   旧代码在此 `downcast_ref::<FakeZipEntry>()` 不成即 `return None`（把"类型不对"
        //   伪装成"取不到流"）的行为已消除。
        match ZipEntrySource::getInputStream_with_flag(archive, zipEntry, is_origin_input) {
            Ok(stream) => Some(stream),
            // Java: `getInputStreamImpl` 抛 IOException ⇒ 由 `getInputStream` 转成同一文案的 IOException
            Err(_) => None,
        }
    }

    pub fn getOutputStreamImpl(&self) -> Option<Box<dyn std::io::Write>> {
        None
    }

    // Java: protected OutputStream getOutputStreamImpl(ZipEntry entry, ZipOutputStream.IListener listener, ZipOutputStream.DeflateDataCallBack deflateDataCallBack)
    // Rust: 基类默认实现（Java `return null`）；子类 `ZipPackagePart` 覆写同一虚方法。
    //
    // ★2026-09-20（类型）：
    //   `entry` 由 `&dyn std::any::Any` 改为 Java 声明的**真实类型** `ZipEntry`（Rust 等价 `&ZipEntryRef`）。
    //   另两个形参（`ZipOutputStream.IListener` / `ZipOutputStream.DeflateDataCallBack`）**保持 `&dyn Any` 壳**，
    //   依据（已核对源码，非猜测）：
    //     · kfs 侧确有 `pub trait IListener`（`kfs_app/.../kfs/zip/ZipOutputStream.rs:40`）与
    //       `pub trait DeflateDataCallBack`（`.../kfs/zip/Deflater.rs:16`），但**没有**本仓统一的
    //       `ZipOutputStream_IListener` / `ZipOutputStream_DeflateDataCallBack` 边界别名；
    //     · 两个真正的消费点按**所有权**收：`ZipOutputStream::putNextEntry(… Option<Box<dyn IListener>>)`
    //       （`ZipOutputStream.rs:362`）与 `setDeflateDataCallBack(… Option<Box<dyn DeflateDataCallBack>>)`
    //       （`:662`）⇒ 借来的 `&dyn` 无法直接转发（trait object 不可 clone）。
    //   故此处先只把**有真实类型**的 `ZipEntry` 落地；listener/callback 待 kfs 边界类型确立后再收口
    //   （登记为待办，不假装已接）。
    pub fn getOutputStreamImpl_zip(
        &self,
        _entry: &ZipEntryRef,
        _listener: &dyn std::any::Any,
        _deflate_data_call_back: &dyn std::any::Any,
    ) -> Option<Box<dyn std::io::Write>> {
        // Java: return null
        None
    }

    // Java: public OutputStream getOutputStream(ZipEntry entry, ZipOutputStream.IListener listener, ZipOutputStream.DeflateDataCallBack deflateDataCallBack)
    pub fn getOutputStream_zip(
        &self,
        entry: &ZipEntryRef,
        listener: &dyn std::any::Any,
        deflate_data_call_back: &dyn std::any::Any,
    ) -> Option<Box<dyn std::io::Write>> {
        // Java: OutputStream outStream = this.getOutputStreamImpl(entry, listener, deflateDataCallBack)
        // Java: return outStream
        self.getOutputStreamImpl_zip(entry, listener, deflate_data_call_back)
    }

    fn throwExceptionIfReadOnly(&self) {
        // NOTE: Java 源为 this._container.throwExceptionIfReadOnly()（_container 为共享引用，unsafe 解引用）
        if let Some(c) = self._container {
            unsafe { (*c).throwExceptionIfReadOnly() }
        }
    }

    fn clone_relationship(rel: &PackageRelationship) -> PackageRelationship {
        PackageRelationship {
            id: rel.id.clone(),
            // Rust: 引用语义字段照抄（Java `PackagePart` 的 rel 集合持有同一批关系对象）
            container: rel.container,
            relationshipType: rel.relationshipType.clone(),
            source: rel.source,
            sourceUri: rel.sourceUri.clone(),
            targetMode: rel.targetMode,
            targetUri: rel.targetUri.clone(),
        }
    }

    // Java: private void throwExceptionIfRelationship()
    fn throwExceptionIfRelationship(&self) {
        // Java: if (this._isRelationshipPart) throw new InvalidOperationException("Can do this operation on a relationship part !")
        if self._isRelationshipPart {
            panic!("Can do this operation on a relationship part !");
        }
    }

    // Java: private void loadRelationships()
    fn loadRelationships(&mut self) {
        // Java: if (this._relationships == null && !this._isRelationshipPart)
        if self._relationships.is_none() && !self._isRelationshipPart {
            // Java: this.throwExceptionIfRelationship()
            self.throwExceptionIfRelationship();
            // Java: this._relationships = new PackageRelationshipCollection(this)
            // ★2026-09-21（M-STAGE1 P1-c1）：此处**保持 `None`**，把真正的建集合+解析留给
            //   `getRelationshipsCore` 的**首次读取**。
            //   ★为什么不能在这里建：Java 的 `PackagePart` 构造发生在 `OPCPackage.getPartsImpl`
            //   的**逐条 put 循环**中 —— 此刻 `_rels/<part>.rels` **尚未入 `partList`**，
            //   `container.getPart(relPartName)` 必然返回 None ⇒ 集合恒空。
            //   ★为什么也不能"先建空壳、之后再补解析"：那样 `_relationships` 已非 `None`，
            //   `getRelationshipsCore` 的懒加载分支就不会触发（实测 size 恒 0）。
            //   ⇒ 唯一自洽做法：构造期不建，首次读取时一次性建成并解析（届时全部 part 均已在表）。
            // Java 的语句注记保留于此，其 Rust 落点即"不建"这一决定本身。
        }
    }

    /// rust: ★补充门面（无 Java 对应）—— 包被 move 后重定位 `_container`。
    /// 语义与 Java 的「`_container` 始终指向同一个包对象」一致；只是 Rust 值类型需要显式修复地址。
    pub fn set_container(&mut self, container: *const OPCPackage) {
        self._container = Some(container);
    }

    // Java: public PackagePartName getPartName()
    pub fn getPartName(&self) -> &PackagePartName {
        // Java: return this._partName
        &self._partName
    }

    // Java: public ContentType getContentType()
    pub fn getContentType(&self) -> &ContentType {
        // Java: return this._contentType
        &self._contentType
    }

    // Java: public void setContentType(String contentType)
    pub fn setContentType(&mut self, content_type: &str) {
        // Java: if (this._container == null) this._contentType = new ContentType(contentType)
        // Java: else throw new InvalidOperationException("You can't change the content type of a part.")
        if self._container.is_none() {
            self._contentType = ContentType::new(content_type).unwrap();
        } else {
            panic!("You can't change the content type of a part.");
        }
    }

    // Java: public OPCPackage getPackage()
    pub fn getPackage(&self) -> &Option<*const OPCPackage> {
        // Java: return this._container
        &self._container
    }

    // Java: public boolean isRelationshipPart()
    pub fn isRelationshipPart(&self) -> bool {
        // Java: return this._isRelationshipPart
        self._isRelationshipPart
    }

    // Java: public boolean isDeleted()
    pub fn isDeleted(&self) -> bool {
        // Java: return this._isDeleted
        self._isDeleted
    }

    // Java: public void setDeleted(boolean isDeleted)
    pub fn setDeleted(&mut self, is_deleted: bool) {
        // Java: this._isDeleted = isDeleted
        self._isDeleted = is_deleted;
    }

    // Java: public String toString()
    pub fn toString_(&self) -> String {
        // Java: return "Name: " + this._partName + " - Content Type: " + this._contentType.toString()
        format!("Name: {} - Content Type: {}", self._partName.getName(), self._contentType.toString())
    }

    // Java: public abstract boolean save(OutputStream var1) throws OpenXML4JException
    pub fn save(&mut self, _var1: &mut dyn std::io::Write) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION Java 声明为 abstract（写侧）：
           Java 由子类 `ZipPackagePart` / `MemoryPackagePart` 实现；本 function（05，part 表建立/读取）
           不在读路径上，按规则 11 停车；解除时由对应子类的 save 覆写体对译。
           public abstract boolean save(OutputStream var1) throws OpenXML4JException; */
    }

    // Java: public abstract boolean load(InputStream var1) throws InvalidFormatException
    pub fn load(&mut self, _var1: &mut dyn std::io::Read) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION Java 声明为 abstract（写侧）：
           public abstract boolean load(InputStream var1) throws InvalidFormatException; */
    }

    // Java: public abstract void close()
    pub fn close(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION Java 声明为 abstract（写侧）：public abstract void close(); */
    }

    // Java: public abstract void flush()
    pub fn flush(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION Java 声明为 abstract（写侧）：public abstract void flush(); */
    }
}
