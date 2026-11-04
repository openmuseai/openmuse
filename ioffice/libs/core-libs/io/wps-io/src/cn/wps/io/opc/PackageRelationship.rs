#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

// Java: import cn.wps.io.util.URI
// Java: import cn.wps.io.util.WpsUriUtils
// Java: import java.net.URISyntaxException

use crate::io::util::URI::URI;
use crate::io::util::WpsUriUtils::WpsUriUtils;
use crate::io::opc::TargetMode::TargetMode;
use crate::io::opc::PackagingURIHelper::PackagingURIHelper;
use crate::io::opc::OPCPackage::OPCPackage;
use crate::io::opc::PackagePart::PackagePart;

use std::sync::OnceLock;

fn java_string_hash(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.chars() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}

// Java field: URI containerRelationshipPart
fn get_containerRelationshipPart() -> &'static URI {
    static CONTAINER_RELATIONSHIP_PART: OnceLock<URI> = OnceLock::new();
    CONTAINER_RELATIONSHIP_PART.get_or_init(|| {
        // Java: containerRelationshipPart = new URI("/_rels/.rels");
        let containerRelationshipPart = URI::new("/_rels/.rels").unwrap();
        containerRelationshipPart
    })
}

// Java field: String ID_ATTRIBUTE_NAME
pub const ID_ATTRIBUTE_NAME: &str = "Id";
// Java field: String RELATIONSHIPS_TAG_NAME
pub const RELATIONSHIPS_TAG_NAME: &str = "Relationships";
// Java field: String RELATIONSHIP_TAG_NAME
pub const RELATIONSHIP_TAG_NAME: &str = "Relationship";
// Java field: String TARGET_ATTRIBUTE_NAME
pub const TARGET_ATTRIBUTE_NAME: &str = "Target";
// Java field: String TARGET_MODE_ATTRIBUTE_NAME
pub const TARGET_MODE_ATTRIBUTE_NAME: &str = "TargetMode";
// Java field: String TYPE_ATTRIBUTE_NAME
pub const TYPE_ATTRIBUTE_NAME: &str = "Type";

// Java: public class PackageRelationship
pub struct PackageRelationship {
    // Java field: String id
    pub(crate) id: String,
    // Java field: OPCPackage container
    pub(crate) container: Option<*const OPCPackage>,
    // Java field: String relationshipType
    pub(crate) relationshipType: String,
    // Java field: PackagePart source
    pub(crate) source: Option<*const PackagePart>,
    // Rust: `source` 的**稳定快照**（无 Java 对应字段）。
    // ★为什么需要：Java 的 `this.source` 是堆对象引用，`getSourceURI()` 读 `source._partName.getURI()`
    //   永远安全；Rust 的 `PackagePart` 是**值类型**且存放在 `PackagePartCollection` 的
    //   `HashMap<PackagePartName, PackagePart>` 里 —— 每次 `put` 都可能 rehash/搬移既有条目，
    //   故任何指向包内 part 的 `*const` 在后续插入后即可能悬垂（实测 SIGSEGV）。
    //   本字段在关系构造时把 `source` 的 partName URI 取出来存成**值**，`getSourceURI()` 优先用它，
    //   从而与 Java 的可观察结果一致，且不再解引用可能失效的别名。
    pub(crate) sourceUri: Option<URI>,
    // Java field: TargetMode targetMode
    pub(crate) targetMode: TargetMode,
    // Java field: URI targetUri
    pub(crate) targetUri: URI,
}

// Java 引用语义：`container`/`source` 存的是**引用**（不是副本）⇒ Rust 用裸指针承载，生命周期由
// 包 / part 的持有者保证（与 `PackagePart::_container`、`PackageRelationshipCollection::container` 同模式；
// 单线程 parse ⇒ 共享只读访问）。
unsafe impl Send for PackageRelationship {}
unsafe impl Sync for PackageRelationship {}

// Java: public class PackageRelationship
impl PackageRelationship {
    // Java: PackageRelationship(OPCPackage pkg, PackagePart sourcePart, URI targetUri, TargetMode targetMode, String relationshipType, String id)
    // Rust: Java 同名门面（`new` 是同一实现的 Rust-native 名，调用点沿用）。
    pub fn PackageRelationship(
        pkg: Option<*const OPCPackage>,
        source_part: Option<*const PackagePart>,
        target_uri: URI,
        target_mode: TargetMode,
        relationship_type: String,
        id: String,
    ) -> Self {
        Self::new(pkg, source_part, target_uri, target_mode, relationship_type, id)
    }

    pub fn new(
        pkg: Option<*const OPCPackage>,
        source_part: Option<*const PackagePart>,
        target_uri: URI,
        target_mode: TargetMode,
        relationship_type: String,
        id: String,
    ) -> Self {
        // Java: super();
        // Java: if (pkg == null)
        if pkg.is_none() {
            // Java: throw new IllegalArgumentException("pkg");
            panic!("pkg");
        }
        // Java: else if (targetUri == null)
        // (targetUri is not Option in Rust, always present)
        // Java: throw new IllegalArgumentException("targetUri");
        // Java: else if (relationshipType == null)
        // (relationshipType is String in Rust, always present)
        // Java: throw new IllegalArgumentException("relationshipType");
        // Java: else if (id == null)
        // (id is String in Rust, always present)
        // Java: throw new IllegalArgumentException("id");
        // Java: else
        // Java: this.container = pkg;
        // Java: this.source = sourcePart;
        // Java: this.targetUri = targetUri;
        // Java: this.targetMode = targetMode;
        // Java: this.relationshipType = relationshipType;
        // Java: this.id = id;
        // rust: 构造期把 source 的 partName URI 快照下来（此刻 source_part 必然有效），
        //       供 `getSourceURI()` 在别名可能失效后仍能与 Java 同结果。
        let source_uri = source_part.map(|ptr| unsafe { (*ptr)._partName.getURI().clone() });
        PackageRelationship {
            id,
            container: pkg,
            relationshipType: relationship_type,
            source: source_part,
            sourceUri: source_uri,
            targetMode: target_mode,
            targetUri: target_uri,
        }
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self, obj: &dyn std::any::Any) -> bool {
        // Java: if (!(obj instanceof PackageRelationship))
        let Some(rel) = obj.downcast_ref::<PackageRelationship>() else {
        // Java: return false;
          return false;
        };
        // Java: else {
        // Rust: 下方同名绑定 `rel`（由上方 `downcast_ref` 取得，与 Java 同一语义）
        // Java: PackageRelationship rel = (PackageRelationship)obj;
        let mut rel = rel;
        // Java: return this.id.equals(rel.id) && this.relationshipType.equals(rel.relationshipType) && (rel.source == null || rel.source.equals(this.source)) && this.targetMode == rel.targetMode && this.targetUri.equals(rel.targetUri);
        self.id == rel.id
          && self.relationshipType == rel.relationshipType
          && (rel.source.is_none() || rel.source == self.source)
          && self.targetMode == rel.targetMode
          && self.targetUri.toString() == rel.targetUri.toString()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.id.hashCode() + this.relationshipType.hashCode() + (this.source == null ? 0 : this.source.hashCode()) + this.targetMode.hashCode() + this.targetUri.hashCode();
        let mut uri = self.targetUri.clone();
        java_string_hash(&self.id)
          .wrapping_add(java_string_hash(&self.relationshipType))
          .wrapping_add(if self.source.is_none() {
            0
          } else {
            // Object.hashCode identity — use part name hash when PackagePart
            self.source
              .map(|p| unsafe { java_string_hash(&(*p)._partName.getName()) })
              .unwrap_or(0)
          })
          .wrapping_add(self.targetMode as i32)
          .wrapping_add(uri.hashCode())
    }

    // Java: static URI getContainerPartRelationship()
    pub fn getContainerPartRelationship() -> &'static URI {
        // Java: return containerRelationshipPart;
        get_containerRelationshipPart()
    }

    // Java: OPCPackage getPackage()
    pub fn getPackage(&self) -> &Option<*const OPCPackage> {
        // Java: return this.container;
        &self.container
    }

    // Java: String getId()
    pub fn getId(&self) -> &str {
        // Java: return this.id;
        &self.id
    }

    // Java: String getRelationshipType()
    pub fn getRelationshipType(&self) -> &str {
        // Java: return this.relationshipType;
        &self.relationshipType
    }

    // Java: PackagePart getSource()
    pub fn getSource(&self) -> &Option<*const PackagePart> {
        // Java: return this.source;
        &self.source
    }

    // Java: URI getSourceURI()
    pub fn getSourceURI(&self) -> URI {
        // Java: return this.source == null ? PackagingURIHelper.PACKAGE_ROOT_URI : this.source._partName.getURI();
        // rust: ★快照优先 —— `sourceUri` 是源 part 的 partName URI（如 `/word/document.xml`），
        //   必须在 `source.is_none()` 之前判定。若按 Java 字面顺序先判 `source == null`，
        //   则以「只有 partName、无 part 别名」方式构造的关系（`new_with_container_and_part_name`）
        //   会落进包根分支 ⇒ 返回**绝对**包根 URI ⇒ `resolvePartUri` 因 source 绝对而**跳过解析**
        //   ⇒ 相对 Target（`styles.xml`）得不到 `/word/` 前缀，最终解析成 `/styles.xml` 而非
        //   `/word/styles.xml` ⇒ `getPart` 全部落空（实测：4 条关系 target_part.is_some 皆为 false）。
        if let Some(ref uri) = self.sourceUri {
            return uri.clone();
        }
        if self.source.is_none() {
            PackagingURIHelper::getPackageRootUri()
        } else {
            // Java: this.source._partName.getURI();
            unsafe { (*self.source.unwrap())._partName.getURI().clone() }
        }
    }

    // Java: TargetMode getTargetMode()
    pub fn getTargetMode(&self) -> TargetMode {
        // Java: return this.targetMode;
        self.targetMode
    }

    // Java: URI getTargetURI()
    pub fn getTargetURI(&self) -> URI {
        // Java: if (this.targetMode == TargetMode.EXTERNAL)
        if self.targetMode == TargetMode::EXTERNAL {
            // Java: return this.targetUri;
            return self.targetUri.clone();
        // Java: else
        } else {
            // Java: char[] chars = WpsUriUtils.toASCIIString(this.targetUri.toString());
            let chars = WpsUriUtils::toASCIIString(&self.targetUri.toString());
            // Java: return chars[0] != '/' ? PackagingURIHelper.resolvePartUri(this.getSourceURI(), this.targetUri) : this.targetUri;
            if chars.len() > 0 && chars[0] != '/' {
                PackagingURIHelper::resolvePartUri(&self.getSourceURI(), &self.targetUri)
            } else {
                self.targetUri.clone()
            }
        }
    }

    // Java: String toString()
    pub fn toString_(&self) -> String {
        // Java: char[] chars = null;
        let chars: Option<Vec<u8>> = None;
        let _ = &chars;
        // rust: no-stmt（`chars` 在 Java 侧仅作 `toASCIIString` 中转；Rust 侧按需就地构造）
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append(this.id == null ? "id=null" : "id=" + this.id);
        sb.push_str(&format!("id={}", self.id));
        // Java: sb.append(this.container == null ? " - container=null" : " - container=" + this.container.toString());
        if self.container.is_none() {
            sb.push_str(" - container=null");
        } else {
            sb.push_str(" - container=...");
        }
        // Java: sb.append(this.relationshipType == null ? " - relationshipType=null" : " - relationshipType=" + this.relationshipType);
        sb.push_str(&format!(" - relationshipType={}", self.relationshipType));
        // Java: sb.append(this.source == null ? " - source=null" : " - source=");
        if self.source.is_none() {
            sb.push_str(" - source=null");
        } else {
            sb.push_str(" - source=");
        }
        // Java: chars = WpsUriUtils.toASCIIString(this.getSourceURI().toString());
        let chars = WpsUriUtils::toASCIIString(&self.getSourceURI().toString());
        // Java: sb.append(chars, 0, chars.length - 1);
        if chars.len() > 1 {
            let s: String = chars[..chars.len() - 1].iter().collect();
            sb.push_str(&s);
        }
        // Java: sb.append(this.targetUri == null ? " - target=null" : " - target=");
        sb.push_str(" - target=");
        // Java: chars = WpsUriUtils.toASCIIString(this.getTargetURI().toString());
        let chars = WpsUriUtils::toASCIIString(&self.getTargetURI().toString());
        // Java: sb.append(chars, 0, chars.length - 1);
        if chars.len() > 1 {
            let s: String = chars[..chars.len() - 1].iter().collect();
            sb.push_str(&s);
        }
        // Java: sb.append(this.targetMode == null ? ",targetMode=null" : ",targetMode=" + this.targetMode.toString());
        sb.push_str(&format!(",targetMode={:?}", self.targetMode));
        // Java: return sb.toString();
        sb
    }
}
