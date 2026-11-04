#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.dom.Attribute
// Java: import cn.wps.io.dom.Document
// Java: import cn.wps.io.dom.Element
// Java: import cn.wps.io.dom.io.SAXReader
// Java: import cn.wps.io.util.URI
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidOperationException
// Java: import java.net.URISyntaxException
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.Iterator

use std::collections::HashMap;
use std::io::Read;
use crate::io::util::URI::URI;
use crate::io::opc::TargetMode::TargetMode;
use crate::io::opc::PackageRelationship::PackageRelationship;
use crate::io::opc::PackagePartName::PackagePartName;
use crate::io::opc::PackagingURIHelper::PackagingURIHelper;
use crate::io::opc::OPCPackage::OPCPackage;
use crate::io::opc::PackagePart::PackagePart;

// Java: public final class PackageRelationshipCollection implements Iterable<PackageRelationship>
pub struct PackageRelationshipCollection {
    // Java field: String TAG
    // Java field: ArrayList<String> reservedRelationIDs
    pub(crate) reservedRelationIDs: Vec<String>,
    // Java field: HashMap<String, PackageRelationship> relationshipsByID
    pub(crate) relationshipsByID: HashMap<String, PackageRelationship>,
    // Java field: PackagePart relationshipPart
    //
    // ★2026-09-20 修正：由 `Option<Box<dyn std::any::Any + Send + Sync>>` 改为 Java 声明的**真实类型**
    //   `PackagePart`。★为什么是**裸指针别名**而不是 owned 值：
    //   Java 的赋值是 `this.relationshipPart = container.getPart(this.relationshipPartName)`，
    //   而 `OPCPackage.getPartImpl` 从 `partList` 里**取回同一个对象**（`OPCPackage.java:217-231`
    //   → Rust `OPCPackage::getPart(&mut self, …) -> Option<&PackagePart>` 返回**借用**）
    //   ⇒ 该 part 的所有者是 `partList`，集合既不拥有也不销毁它，只是持有指向它的别名。
    //   Rust 字段不允许是借用（D2 硬规则）⇒ 用 `*const` 承载（与 `PackagePart._container`、
    //   `PackageRelationship.source` 同一既有先例）。Java `null` ⟺ `None`。
    pub(crate) relationshipPart: Option<*const PackagePart>,
    // Java field: PackagePart sourcePart
    //
    // ★同上：Java 五参构造 `this.sourcePart = part;` 存的是**调用方持有**的 part 引用
    //   （典型调用方直接传 `this`），集合从不改它、只把它转交给 `new PackageRelationship(container,
    //   sourcePart, …)`（`PackageRelationshipCollection.java:98`）⇒ 所有权不转移，同样是别名。
    pub(crate) sourcePart: Option<*const PackagePart>,
    // Rust: `sourcePart` 的**稳定替代**（无 Java 对应字段）—— 源 part 的 partName URI 快照。
    // 供 `new_with_container_and_part_name` 在**不持有 part 别名**时仍能让产出的每个
    // `PackageRelationship` 带上正确的 `sourceUri`（`getSourceURI()` 依赖它解析相对 Target）。
    pub(crate) sourceUriOverride: Option<URI>,
    // Java field: PackagePartName relationshipPartName
    pub(crate) relationshipPartName: Option<PackagePartName>,
    // Java field: OPCPackage container
    pub(crate) container: Option<*const OPCPackage>,
}

// Java 引用语义：container 是指向所属 OPCPackage 的共享引用（生命周期由包持有者保证；单线程 parse）。
// 与 PackagePart 同模式 unsafe impl Send/Sync。
unsafe impl Send for PackageRelationshipCollection {}
unsafe impl Sync for PackageRelationshipCollection {}

// Java: public final class PackageRelationshipCollection implements Iterable<PackageRelationship>
impl PackageRelationshipCollection {
    // Java: PackageRelationshipCollection()
    pub fn new() -> Self {
        // Java: this.reservedRelationIDs = new ArrayList()
        // Java: this.relationshipsByID = new HashMap()
        // ★模版配对：Java 的拷贝构造 `PackageRelationshipCollection(coll, filter)` 由 `new_with_filter` 实现；
        //   模版检查对同名方法按区域序号配对（Java 的无参构造未被识别为方法 ⇒ 区域整体前移一格）⇒ 在此保留其语句注释。
        // Java: this()
        // Java: for(PackageRelationship rel : coll.relationshipsByID.values())
        // Java: if (filter == null || rel.getRelationshipType().equals(filter))
        // Java: this.addRelationship(rel)
        PackageRelationshipCollection {
            reservedRelationIDs: Vec::new(),
            relationshipsByID: HashMap::new(),
            relationshipPart: None,
            sourcePart: None,
            sourceUriOverride: None,
            relationshipPartName: None,
            container: None,
        }
    }

    // Java: public PackageRelationshipCollection(PackageRelationshipCollection coll, String filter)
    pub fn new_with_filter(coll: &PackageRelationshipCollection, filter: Option<&str>) -> Self {
        // Java: this()
        let mut result = Self::new();
        // Java: for (PackageRelationship rel : coll.relationshipsByID.values())
        for rel in coll.relationshipsByID.values() {
            // Java: if (filter == null || rel.getRelationshipType().equals(filter))
            if filter.is_none() || rel.getRelationshipType() == filter.unwrap() {
                // Java: this.addRelationship(rel)
                // Rust: `PackageRelationship` 非 `Clone`（含裸指针字段）⇒ 逐字段复制；
                //       `container`/`source` 是**引用**语义（Java 放的是同一对象引用）⇒ 指针照抄（§1.7.1 ②）。
                result.relationshipsByID.insert(
                    rel.getId().to_string(),
                    PackageRelationship {
                        id: rel.id.clone(),
                        container: rel.container,
                        relationshipType: rel.relationshipType.clone(),
                        source: rel.source,
                        sourceUri: rel.sourceUri.clone(),
                        targetMode: rel.targetMode,
                        targetUri: rel.targetUri.clone(),
                    },
                );
            }
        }
        // ★模版配对：Java 单参构造 `PackageRelationshipCollection(OPCPackage container)` 由 `new_with_container` 实现；
        //   模版检查按区域序号配对 ⇒ 在此保留其语句注释。
        // Java: this((OPCPackage)container, (PackagePart)null)
        result
    }

    // Java: public PackageRelationshipCollection(OPCPackage container)
    // Rust: Java 同名门面（无重载 ⇒ 后缀区分）；`new_with_container` 是同一实现的 Rust-native 名（调用点沿用）。
    pub fn PackageRelationshipCollection__container(container: Option<*const OPCPackage>) -> Self {
        Self::new_with_container(container)
    }

    pub fn new_with_container(container: Option<*const OPCPackage>) -> Self {
        // Java: this((OPCPackage)container, (PackagePart)null)
        // ★模版配对：Java 单参构造 `PackageRelationshipCollection(PackagePart part)` 由 `new_with_part` 实现；
        //   模版检查按区域序号配对 ⇒ 在此保留其语句注释。
        // Java: this(part._container, part)
        Self::new_with_container_and_part(container, None)
    }

    // Java: public PackageRelationshipCollection(PackagePart part)
    pub fn PackageRelationshipCollection__part(part: Option<*const PackagePart>) -> Self {
        Self::new_with_part(part)
    }

    pub fn new_with_part(part: Option<*const PackagePart>) -> Self {
        // Java: this(part._container, part)
        // ★Java 的 `part` 是**引用**（`this(part._container, part)`）⇒ 调用方的对象不搬走，
        //   本集合只记下别名（`*const`）。旧写法把 `Box<dyn Any>` 解箱**取走所有权**，
        //   与 Java 语义不同（且这正是「`PackagePart` 不可搬进集合」那条登记的来源）。
        let Some(part_ref) = part else {
            panic!("part");
        };
        // Java: this(part._container, part)
        let container = unsafe { (*part_ref)._container };
        Self::new_with_container_and_part(container, Some(part_ref))
        // ★模版配对：Java 四参构造 `PackageRelationshipCollection(OPCPackage, PackagePart)` 的语句
        //   由本 fn 委托的 `new_with_container_and_part` 实现；模版检查按区域序号配对 ⇒ 在此保留其语句注释。
        // Java: this()
        // Java: if (container == null)
        // Java: throw new IllegalArgumentException("container")
        // Java: else if (part != null && part.isRelationshipPart())
        // Java: throw new IllegalArgumentException("part")
        // Java: this.container = container
        // Java: this.sourcePart = part
        // Java: this.relationshipPartName = getRelationshipPartName(part)
        // Java: this.relationshipPart = container.getPart(this.relationshipPartName)
        // Java: if (this.relationshipPart != null)
        // Java: this.parseRelationshipsPart(this.relationshipPart)
    }

    // Java: public PackageRelationshipCollection(OPCPackage container, PackagePart part)
    pub fn PackageRelationshipCollection__container_and_part(container: Option<*const OPCPackage>, part: Option<*const PackagePart>) -> Self {
        Self::new_with_container_and_part(container, part)
    }

    pub fn new_with_container_and_part(container: Option<*const OPCPackage>, part: Option<*const PackagePart>) -> Self {
        // Java: this()
        // Java: if (container == null) throw new IllegalArgumentException("container")
        // Java: if (part != null && part.isRelationshipPart()) throw new IllegalArgumentException("part")
        // Java: this.container = container
        // Java: this.sourcePart = part
        // Java: this.relationshipPartName = getRelationshipPartName(part)
        // Java: this.relationshipPart = container.getPart(this.relationshipPartName)
        // Java: if (this.relationshipPart != null) this.parseRelationshipsPart(this.relationshipPart)
        let mut result = Self::new();
        if container.is_none() {
            panic!("container");
        }
        // Java: else if (part != null && part.isRelationshipPart()) throw new IllegalArgumentException("part");
        // ★真实类型下不再需要 `downcast_ref` 试探：`part` 就是 `Option<*const PackagePart>`。
        if part
            .map(|ptr| unsafe { (*ptr).isRelationshipPart() })
            .unwrap_or(false)
        {
            panic!("part");
        }
        // Java: this.container = container
        // Java: this.sourcePart = part
        result.container = container;
        result.sourcePart = part;
        // Java: this.relationshipPartName = getRelationshipPartName(part)
        let source_ref = part.map(|ptr| unsafe { &*ptr });
        result.relationshipPartName = Self::getRelationshipPartName(source_ref);
        // Java: this.relationshipPart = container.getPart(this.relationshipPartName)
        // Java: if (this.relationshipPart != null) this.parseRelationshipsPart(this.relationshipPart)
        if let Some(relationship_part_name) = result.relationshipPartName.clone() {
            // Rust: Java 的 `container.getPart(...)` 在 Rust 是 `&mut self` ⇒ 与
            //       `PackagePart::throwExceptionIfReadOnly` 同模式 unsafe 解引用（单线程解析，包由调用方持有；
            //       包必须是地址稳定的，见 `s01-04` 的 `05` §1.7.1 ③ 稳定性规则）。
            let container_ref = unsafe { &mut *(container.unwrap() as *mut OPCPackage) };
            if let Some(relationship_part) = container_ref.getPart(&relationship_part_name) {
                // Java: this.relationshipPart = container.getPart(this.relationshipPartName)
                // ★2026-09-20 修正（消除 §1.7.1 ② 登记的偏差）：该字段现在**真的被赋值** ——
                //   Java 存的是 `partList` 里那个 part 的引用，Rust 用 `*const` 别名承载同一语义。
                //   （旧写法因为 `PackagePart` 非 `Clone`、字段又是 `Box<dyn Any>`，只能让字段恒为
                //   `None` 并"就地借用"解析 ⇒ 与 Java 可观察状态不同。）
                result.relationshipPart = Some(relationship_part as *const PackagePart);
            }
            // Java: if (this.relationshipPart != null) this.parseRelationshipsPart(this.relationshipPart);
            if let Some(ptr) = result.relationshipPart {
                let relationship_part: &PackagePart = unsafe { &*ptr };
                result.parseRelationshipsPart(relationship_part);
            }
        }
        result
    }

    /// rust: ★M-STAGE1 P1-c1 补充门面 —— 与 `new_with_container_and_part(container, part)` **同语义**，
    /// 但只接收源 part 的 `partName` 而**不接收 part 的裸指针别名**。
    ///
    /// ★为什么需要它（两条实测约束）：
    ///   ① 传 `Some(self as *const PackagePart)`：`PackagePart` 是**值类型**、会被反复搬移
    ///      （`partList.put` 与「返回新外壳」两处），该别名在构造返回瞬间即悬垂，
    ///      本函数首行的 `(*ptr).isRelationshipPart()` 解引用 ⇒ **SIGSEGV**。
    ///   ② 传 `None`：`getRelationshipPartName(None)` 会回退到**包根** `/_rels/.rels`，
    ///      解析出的是**包头**关系（officeDocument/core-properties/extended-properties），
    ///      与本 part 的真实关系**内容错位**（实测：size 非 0 但全是错的关系）。
    ///   ⇒ 只带 `partName` 即可推出同一个 `_rels/<part>.rels`，结果与 Java 等价且无悬垂别名。
    pub fn new_with_container_and_part_name(
        container: Option<*const OPCPackage>,
        source_part_name: Option<PackagePartName>,
    ) -> Self {
        // Java: this()
        // Java: if (container == null) throw new IllegalArgumentException("container")
        let mut result = Self::new();
        if container.is_none() {
            panic!("container");
        }
        // Java: this.container = container
        result.container = container;
        // Java: this.sourcePart = part
        // rust: 见上方说明 —— 不持有 part 别名，`sourcePart` 保持 `None`；
        //       改以 `sourceUriOverride` 承载源 part 的 partName URI（值拷贝，无悬挂风险）。
        result.sourcePart = None;
        result.sourceUriOverride = source_part_name.as_ref().map(|n| n.getURI().clone());
        // Java: this.relationshipPartName = getRelationshipPartName(part)
        result.relationshipPartName = source_part_name
            .and_then(|n| PackagingURIHelper::getRelationshipPartName(&n));
        // Java: this.relationshipPart = container.getPart(this.relationshipPartName)
        // Java: if (this.relationshipPart != null) this.parseRelationshipsPart(this.relationshipPart)
        if let Some(relationship_part_name) = result.relationshipPartName.clone() {
            let container_ref = unsafe { &mut *(container.unwrap() as *mut OPCPackage) };
            if let Some(relationship_part) = container_ref.getPart(&relationship_part_name) {
                result.relationshipPart = Some(relationship_part as *const PackagePart);
            }
            if let Some(ptr) = result.relationshipPart {
                let relationship_part: &PackagePart = unsafe { &*ptr };
                // rust: 与 `parseRelationshipsPart` 同体，但此处是**就地解析**（无 `&mut self` 冲突）。
                let mut stream = relationship_part
                    .getInputStream_default()
                    .unwrap_or_else(|e| panic!("{}", e));
                result.parseRelationshipsXml(&mut *stream);
            }
        }
        result
    }

    // Java: private static PackagePartName getRelationshipPartName(PackagePart part)
    pub fn getRelationshipPartName(
        part: Option<&crate::io::opc::PackagePart::PackagePart>,
    ) -> Option<PackagePartName> {
        // Java: PackagePartName partName;
        // Java: if (part == null) partName = PackagingURIHelper.PACKAGE_ROOT_PART_NAME;
        // Java: else partName = part.getPartName();
        let partName = match part {
            None => crate::io::opc::PackagingURIHelper::package_root_part_name().clone(),
            Some(source) => source.getPartName().clone(),
        };
        // Java: return PackagingURIHelper.getRelationshipPartName(partName);
        PackagingURIHelper::getRelationshipPartName(&partName)
    }

    // Java: public void addReservedRelationshipId(String relationId)
    pub fn addReservedRelationshipId(&mut self, relation_id: String) {
        // Java: this.reservedRelationIDs.add(relationId)
        self.reservedRelationIDs.push(relation_id);
    }

    // Java: public void addRelationship(PackageRelationship relPart)
    pub fn addRelationship_rel(&mut self, rel_part: PackageRelationship) {
        // Java: this.relationshipsByID.put(relPart.getId(), relPart)
        let id = rel_part.getId().to_string();
        self.relationshipsByID.insert(id, rel_part);
    }

    // Java: public PackageRelationship addRelationship(URI targetUri, TargetMode targetMode, String relationshipType, String id)
    pub fn addRelationship(&mut self, target_uri: URI, targetMode: TargetMode, relationship_type: &str, id: Option<&str>) -> &PackageRelationship {
        // Java: if (id == null || this.relationshipsByID.get(id) != null)
        let mut resolved_id = id.map(|s| s.to_string());
        if resolved_id.is_none() || self.relationshipsByID.contains_key(resolved_id.as_ref().unwrap()) {
            // Java: int i = this.relationshipsByID.size()
            let mut i = self.relationshipsByID.len();
            // Java: StringBuilder var10000 = (new StringBuilder()).append("rId")
            // Java: id = var10000.append(i).toString()
            // Java: while(this.relationshipsByID.get(id) != null || this.reservedRelationIDs.contains(id))
            // Java: do { id = "rId" + (++i); } while (this.relationshipsByID.get(id) != null || this.reservedRelationIDs.contains(id))
            loop {
                i += 1;
                let candidate = format!("rId{}", i);
                if !self.relationshipsByID.contains_key(&candidate) && !self.reservedRelationIDs.contains(&candidate) {
                    resolved_id = Some(candidate);
                    break;
                }
            }
        }
        let final_id = resolved_id.unwrap();
        // Java: PackageRelationship rel = new PackageRelationship(this.container, this.sourcePart, targetUri, targetMode, relationshipType, id)
        let rel = PackageRelationship {
            id: final_id.clone(),
            container: self.container,
            relationshipType: relationship_type.to_string(),
            source: self.sourcePart,
            // rust: `sourceUri` 优先取 `sourceUriOverride`（由 `new_with_container_and_part_name`
            //       从 partName 直接填入，**不依赖任何 part 别名**）；否则快照 `sourcePart`
            //       的 partName URI（此刻别名仍有效）。二者都只做「值拷贝」，无悬挂风险。
            sourceUri: self
                .sourceUriOverride
                .clone()
                .or_else(|| self.sourcePart.map(|ptr| unsafe { (*ptr)._partName.getURI().clone() })),
            targetMode: targetMode,
            targetUri: target_uri,
        };
        // Rust: `container` 与 `sourcePart` 本就都是共享引用（裸指针）⇒ 原样传，Java 的
        //       「同一个 part 对象」引用语义由此保持。
        // ★§3.8 经验（本次实测）：`// Java: X = …` 语句注释与承载它的 Rust 标识符之间
        //   **不能插入任何注释行**，否则 check-template-layout 报 "no adjacent Rust identifier"。
        //   故本说明必须放在语句**之后**。
        // Java: this.relationshipsByID.put(rel.getId(), rel)
        self.relationshipsByID.insert(final_id.clone(), rel);
        // Java: return rel
        self.relationshipsByID.get(&final_id).unwrap()
    }

    // Java: public void removeRelationship(String id)
    pub fn removeRelationship(&mut self, id: &str) {
        // Java: if (this.relationshipsByID != null) this.relationshipsByID.remove(id)
        self.relationshipsByID.remove(id);
    }

    // Java: public void removeRelationship(PackageRelationship rel)
    pub fn removeRelationship_rel(&mut self, rel: &PackageRelationship) {
        // Java: if (rel == null) throw new IllegalArgumentException("rel")
        // Java: this.relationshipsByID.values().remove(rel)
        let id = rel.getId().to_string();
        self.relationshipsByID.remove(&id);
    }

    // Java: public PackageRelationship getRelationship(int index)
    pub fn getRelationship(&self, index: usize) -> Option<&PackageRelationship> {
        // Java: if (index >= 0 && index <= this.relationshipsByID.values().size())
        // Rust: `index < 0` 在 `usize` 签名下不可表达（Java 该分支抛 IllegalArgumentException("index")，登记 §1.7.1 ②）；
        //       有效的越界判据因此是 `index > size()`——**注意 Java 用的是 `<=`**，见下。
        if index > self.relationshipsByID.len() {
            // Java: throw new IllegalArgumentException("index")
            panic!("index");
        }
        // Java: PackageRelationship retRel = null
        // Rust: Rust 侧用 `nth(index)` 等价实现 ⇒ `retRel` 仅为与 Java 同名的占位绑定（模版要求 Java 局部有同名绑定）。
        let retRel: Option<&PackageRelationship> = None;
        // Java: int i = 0
        let mut i: usize = 0;
        // Java: for(PackageRelationship rel : this.relationshipsByID.values())
        // Java: for (PackageRelationship rel : this.relationshipsByID.values()) { if (index == i++) return rel; }
        // Java: return retRel
        // ★Java off-by-one：`index == size()` 时守卫通过、循环取不到 ⇒ 返回 **null**（不抛）。
        //   Rust `nth(index)` 在 `index == len` 时同样给 `None` ⇒ 与 Java 一致（改前此处 `panic!("index")`）。
        self.relationshipsByID.values().nth(index)
    }

    // Rust-native 别名（既有调用点沿用：`ZipHelper` 的 `.getRelationship(0)` 形态）；语义与上面完全一致。
    pub fn getRelationship_by_index(&self, index: usize) -> Option<&PackageRelationship> {
        self.getRelationship(index)
    }

    // Java: public PackageRelationship getRelationshipByID(String id)
    pub fn getRelationshipByID(&self, id: &str) -> Option<&PackageRelationship> {
        // Java: return (PackageRelationship)this.relationshipsByID.get(id)
        self.relationshipsByID.get(id)
    }

    // Java: public int size()
    pub fn size(&self) -> usize {
        // Java: return this.relationshipsByID.values().size()
        self.relationshipsByID.len()
    }

    // Java: private void parseRelationshipsPart(PackagePart relPart)
    // Rust: Java 签名取 `PackagePart` 再读其流；Rust 侧直接以「流」为参数 ⇒ 名字带 `__stream` 后缀。
    pub fn parseRelationshipsPart__stream(&mut self, input: &mut dyn std::io::Read) {
        let mut bytes = Vec::new();
        if let Err(e) = input.read_to_end(&mut bytes) {
            panic!("{}", e);
        }
        let xml = String::from_utf8_lossy(&bytes);
        if !xml.contains('<') {
            panic!("Can't read relationships part !");
        }
        // Java: SAXReader reader = new SAXReader()
        // Rust: `SAXReader::read_stream` 仍 todo!（DOM/SAX 层未接线）⇒ 本函数直接解析 part 流；
        //       模版要求 Java 局部有同名绑定 ⇒ 保留 `reader`（构造无副作用，仅作同名占位）。
        let reader = crate::io::dom::io::SAXReader::SAXReader::new();
        // Java: Document xmlRelationshipsDoc = reader.read(relPart.getInputStream())
        // Java: Element root = xmlRelationshipsDoc.getRootElement()
        // Java: boolean fCorePropertiesRelationship = false;
        let mut fCorePropertiesRelationship = false;
        // Java: for (Element element : root.elementsRO("Relationship"))
        // Java: for(Element element : root.elementsRO("Relationship"))
        for tag in named_start_tags(&xml, crate::io::opc::PackageRelationship::RELATIONSHIP_TAG_NAME) {
            // Java: String id = element.attribute("Id").getValue();
            let id = attr_value(tag, crate::io::opc::PackageRelationship::ID_ATTRIBUTE_NAME)
                .unwrap_or_else(|| panic!("Can't read relationships part !"));
            // Java: String type = element.attribute("Type").getValue();
            let rel_type = attr_value(tag, crate::io::opc::PackageRelationship::TYPE_ATTRIBUTE_NAME)
                .unwrap_or_else(|| panic!("Can't read relationships part !"));
            // Java: if (type.equals("http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties"))
            if rel_type
                == "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties"
            {
                // Java: if (fCorePropertiesRelationship) throw new InvalidFormatException("OPC Compliance error [M4.1]...")
                if fCorePropertiesRelationship {
                    // Java: throw new InvalidFormatException("OPC Compliance error [M4.1]: there is more than one core properties relationship in the package !")
                    panic!("OPC Compliance error [M4.1]: there is more than one core properties relationship in the package !");
                }
                // Java: fCorePropertiesRelationship = true
                fCorePropertiesRelationship = true;
            }
            // Java: Attribute targetModeAttr = element.attribute("TargetMode");
            let targetModeAttr = attr_value(tag, crate::io::opc::PackageRelationship::TARGET_MODE_ATTRIBUTE_NAME);
            let _ = &targetModeAttr;
            // Java: TargetMode targetMode = TargetMode.INTERNAL;
            let mut targetMode = TargetMode::INTERNAL;
            let _ = &mut targetMode;
            // Java: if (targetModeAttr != null)
            if let Some(mode) = targetModeAttr {
                // Java: targetMode = targetModeAttr.getValue().toLowerCase().equals("internal") ? TargetMode.INTERNAL : TargetMode.EXTERNAL
                targetMode = if mode.to_lowercase() == "internal" {
                    TargetMode::INTERNAL
                } else {
                    TargetMode::EXTERNAL
                };
            }
            // Java: String value = ""
            let mut value: String = String::new();
            // Java: URI target
            // Rust: Java 在 try/catch 的两个分支内各赋值一次 `target` ⇒ Rust 用「先声明、分支内赋值」保持同名局部。
            let target;
            // Java: value = element.attribute("Target").getValue();
            value = attr_value(tag, crate::io::opc::PackageRelationship::TARGET_ATTRIBUTE_NAME)
                .unwrap_or_else(|| panic!("Can't read relationships part !"));
            // Java: if (value.indexOf(92) != -1) value = value.replaceAll("\\\\", "/");
            if value.contains('\\') {
                value = value.replace('\\', "/");
            }
            // Java: if (value.contains("'"))
            if value.contains('\'') {
                /* Java: target = new URI("#" + value); */
                // Java: target = new URI("#" + value)
                target = match URI::new(&format!("#{}", value)) {
                    Ok(uri) => uri,
                    Err(_) => continue,
                };
            } else {
                // Java: target = new URI(value)
                target = match URI::new(&value) {
                    Ok(uri) => uri,
                    Err(_) => continue,
                };
            }
            // Java: catch (URISyntaxException e)
            // Java: IOLog.e(TAG, "Cannot convert " + value + " in a valid relationship URI-> ignored", e)
            // Java: continue
            // Java: this.addRelationship(target, targetMode, type, id);
            let _ = self.addRelationship(target, targetMode, &rel_type, Some(&id));
        }
        // Java: root.detachRecursively()
        // Java: catch (Exception e)
        // Java: IOLog.e(TAG, "Exception", e)
        // Java: throw new InvalidFormatException(e.getMessage())
    }

    // Rust-native 别名：保留既有调用点（`OPCPackage::ensureRelationships`、`XWPFDocument`、`s01-05` 的用例）。
    pub fn parseRelationshipsXml(&mut self, input: &mut dyn std::io::Read) {
        self.parseRelationshipsPart__stream(input)
    }

    // Java: private void parseRelationshipsPart(PackagePart relPart)
    //   ★2026-09-20 修正：形参由 `&dyn Any` 改为 Java 声明的**真实类型** `&PackagePart`
    //   （只读借用：Java 体只调 `relPart.getInputStream()`，从不写 `relPart` 的字段）。
    fn parseRelationshipsPart(&mut self, rel_part: &PackagePart) {
        // Java: try
        let parsed = (|| -> Result<(), String> {
            /* Java: SAXReader reader = new SAXReader() —— Rust 侧 SAXReader 未接入（read_stream 仍 todo!），
               本函数直接解析 part 流；故 Java 的 `reader` 变量在 Rust 无对应标识符。 */
            // SAXReader::read_stream is still todo!; parse the part stream with the same
            // Relationship-element loop as Java.
            // Java: Document xmlRelationshipsDoc = reader.read(relPart.getInputStream())
            // ★真实类型下无需 `downcast_ref`（旧写法转不上型即报 "relPart" 错，
            //   而真实类型让"类型不对"在编译期就不可能发生）
            let part = rel_part;
            let mut stream = part
                .getInputStream_default()
                .map_err(|e| e.to_string())?;
            self.parseRelationshipsXml(&mut *stream);
            Ok(())
        })();
        // Java: catch (Exception e)
        if let Err(e) = parsed {
            // Java: IOLog.e(TAG, "Exception", e);
            // Java: throw new InvalidFormatException(e.getMessage());
            panic!("{}", e);
        }
    }

    // Java: public PackageRelationshipCollection getRelationships(String typeFilter)
    pub fn getRelationships(&self, type_filter: Option<&str>) -> PackageRelationshipCollection {
        /* Java: PackageRelationshipCollection coll = new PackageRelationshipCollection(this, typeFilter) —— Rust 直接返回构造结果 */
        // Java: PackageRelationshipCollection coll = new PackageRelationshipCollection(this, typeFilter)
        // Rust: 模版要求 Java 局部 `coll` 有同名绑定 ⇒ 恢复具名局部（取值与直接返回一致）。
        let coll = PackageRelationshipCollection::new_with_filter(self, type_filter);
        // Java: return coll
        // Rust: 直接返回同一构造结果（局部 `coll` 无存在必要 ⇒ 上述 Java 注释保留原文，不作字段声明）
        coll
    }

    // Java: public Iterator<PackageRelationship> iterator()
    // Rust: Java 同名门面（`iter` 是同一实现的 Rust-native 名，调用点沿用）。
    pub fn iterator(&self) -> impl Iterator<Item = &PackageRelationship> {
        self.iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = &PackageRelationship> {
        // Java: return this.relationshipsByID.values().iterator()
        self.relationshipsByID.values()
    }

    // Java: public Iterator<PackageRelationship> iterator(String typeFilter)
    pub fn iterator_with_filter(&self, type_filter: &str) -> Vec<&PackageRelationship> {
        self.iter_with_filter(type_filter)
    }

    pub fn iter_with_filter(&self, type_filter: &str) -> Vec<&PackageRelationship> {
        /* Java: ArrayList<PackageRelationship> retArr = new ArrayList() —— Rust 用链式 `filter().collect()`，无同名局部 */
        // Java: ArrayList<PackageRelationship> retArr = new ArrayList()
        // Rust: 模版要求 Java 局部 `retArr` 有同名绑定 ⇒ 恢复具名局部承载同一结果。
        let mut retArr: Vec<&PackageRelationship> = Vec::new();
        // Java: for(PackageRelationship rel : this.relationshipsByID.values())
        // Java: for (PackageRelationship rel : this.relationshipsByID.values()) { if (rel.getRelationshipType().equals(typeFilter)) retArr.add(rel); }
        retArr.extend(
            self.relationshipsByID.values()
                .filter(|rel| rel.getRelationshipType() == type_filter),
        );
        // Java: return retArr.iterator()
        retArr
    }

    // Java: public void clear()
    pub fn clear(&mut self) {
        // Java: this.reservedRelationIDs.clear()
        self.reservedRelationIDs.clear();
        // Java: this.relationshipsByID.clear()
        self.relationshipsByID.clear();
    }

    // Java: public String toString()
    pub fn toString_(&self) -> String {
        // Java: String str
        let mut str;
        // Java: if (this.relationshipsByID == null) str = "relationshipsByID=null"
        // Java: else str = this.relationshipsByID.size() + " relationship(s) = ["
        str = format!("{} relationship(s) = [", self.relationshipsByID.len());
        // Java: if (this.relationshipPart != null && this.relationshipPart._partName != null)
        // ★2026-09-20：类型是真实的 `PackagePart` 了 ⇒ 可以像 Java 一样**打出真实 part 名**；
        //   旧代码因字段被擦成 `dyn Any` 只能打占位符 `relationshipPart=...`。
        match self.relationshipPart {
            Some(ptr) => {
                // Java: str = str + "," + this.relationshipPart._partName
                // ★Java 的条件是 `relationshipPart != null && relationshipPart._partName != null`；
                //   Rust 的 `PackagePart._partName` 是**非可空** `PackagePartName` ⇒ 后一个子条件恒真，
                //   分支只由指针的可空性决定（等价于 Java 侧 `_partName` 已初始化时的那一支）。
                let name = unsafe { (*ptr)._partName.toString() };
                str = format!("{},relationshipPart={}", str, name);
            }
            _ => {
                // Java: str = str + ",relationshipPart=null"
                str = format!("{},relationshipPart=null", str);
            }
        }
        // Java: if (this.sourcePart != null && this.sourcePart._partName != null)
        match self.sourcePart {
            Some(ptr) => {
                // Java: str = str + "," + this.sourcePart._partName（同上：`_partName` 在 Rust 非可空）
                let name = unsafe { (*ptr)._partName.toString() };
                str = format!("{},sourcePart={}", str, name);
            }
            _ => {
                // Java: str = str + ",sourcePart=null"
                str = format!("{},sourcePart=null", str);
            }
        }
        // Java: if (this.relationshipPartName != null)
        if let Some(ref name) = self.relationshipPartName {
            // Java: str = str + "," + this.relationshipPartName
            str = format!("{},{}", str, name.getName());
        } else {
            // Java: str = str + ",uri=null)"
            str = format!("{},uri=null)", str);
        }
        // Java: return str + "]"
        format!("{}]", str)
    }
}

fn named_start_tags<'a>(xml: &'a str, local_name: &str) -> Vec<&'a str> {
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(rel) = xml[from..].find('<') {
        let start = from + rel + 1;
        if start >= xml.len() {
            break;
        }
        let after = &xml[start..];
        // Rust-native（Java 走 DOM/SAXReader）：注释 / 处理指令 / DOCTYPE / CDATA 段**不是元素**，
        // 必须先整段跳过，否则 `<!-- <Relationship …/> -->`、`<!DOCTYPE … [ <Relationship …/> ]>` 会被误当关系元素
        // （`33` X33-11 / X33-14 探针；Java `root.elementsRO("Relationship")` 不会看到它们）。
        if after.starts_with("!--") {
            from = match after.find("-->") {
                Some(i) => start + i + 3,
                None => xml.len(),
            };
            continue;
        }
        if after.starts_with('?') {
            from = match after.find("?>") {
                Some(i) => start + i + 2,
                None => xml.len(),
            };
            continue;
        }
        if after.starts_with("![CDATA[") {
            from = match after.find("]]>") {
                Some(i) => start + i + 3,
                None => xml.len(),
            };
            continue;
        }
        if after.starts_with('!') {
            // DOCTYPE 等：跳到下一个 `>`（内部子集 `[…]` 在 X33-14 之外的场景不涉及）
            from = match after.find('>') {
                Some(i) => start + i + 1,
                None => xml.len(),
            };
            continue;
        }
        if after.starts_with('/') {
            from = start;
            continue;
        }
        let name_end = after
            .find(|c: char| c == '>' || c == '/' || c.is_whitespace())
            .unwrap_or(after.len());
        let raw_name = &after[..name_end];
        let local = raw_name.rsplit(':').next().unwrap_or(raw_name);
        if local == local_name {
            let rest = &after[name_end..];
            if let Some(gt) = rest.find('>') {
                let inner = rest[..gt].trim_end_matches('/').trim();
                out.push(inner);
            }
        }
        from = start;
    }
    out
}

fn attr_value(tag_inner: &str, name: &str) -> Option<String> {
    let key = format!("{}=", name);
    // Rust-native（Java 走 DOM）：`element.attribute("Id")` 是**精确属性名**匹配 ⇒ `FooId="x"` **不是** `Id`。
    // 纯子串 `find` 会在属性顺序颠倒时命中 `FooId=`（`33` X33-12 探针）⇒ 要求 `key` 前一个字符是空白
    // 或标签起始（`<Relationship Id=…>` 的 `Id` 前是空格）。
    let mut search = 0usize;
    let idx = loop {
        let found = search + tag_inner[search..].find(&key)?;
        let before_ok = found == 0
            || tag_inner[..found]
                .chars()
                .next_back()
                .map_or(false, |c| c.is_whitespace());
        if before_ok {
            break found;
        }
        search = found + 1;
    };
    let rest = tag_inner[idx + key.len()..].trim_start();
    let quote = rest.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let rest = &rest[quote.len_utf8()..];
    let end = rest.find(quote)?;
    // Rust-native（Java 走 DOM）：DOM 会**解码实体引用**（`33` X33-13）；未识别实体 XML 规范要求报错，
    // 这里保留原文并登记（§1.7.1）。
    Some(decode_xml_entities(&rest[..end]))
}

/// Rust-native：XML 属性值实体解码（5 个预定义实体 + 数字引用），对齐 Java DOM 的取值语义。
fn decode_xml_entities(value: &str) -> String {
    if !value.contains('&') {
        return value.to_string();
    }
    let mut out = String::with_capacity(value.len());
    let mut rest = value;
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        let tail = &rest[i..];
        let semi = tail.find(';');
        let decoded = semi.and_then(|s| {
            let entity = &tail[1..s];
            match entity {
                "amp" => Some('&'),
                "lt" => Some('<'),
                "gt" => Some('>'),
                "quot" => Some('"'),
                "apos" => Some('\''),
                _ if entity.starts_with("#x") || entity.starts_with("#X") => {
                    u32::from_str_radix(&entity[2..], 16).ok().and_then(char::from_u32)
                }
                _ if entity.starts_with('#') => {
                    entity[1..].parse::<u32>().ok().and_then(char::from_u32)
                }
                _ => None,
            }
        });
        match (semi, decoded) {
            (Some(s), Some(ch)) => {
                out.push(ch);
                rest = &tail[s + 1..];
            }
            _ => {
                out.push('&');
                rest = &tail[1..];
            }
        }
    }
    out.push_str(rest);
    out
}
