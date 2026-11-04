// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.io.dom.io.WMap

use crate::io::dom::io::WMap::WMap;

// Java field: String RELATIONSHIPS
//
// ★Java 语义：`public static String RELATIONSHIPS`（**可赋值静态字段**，非 final），
//   由 `initTransitionNamespace`/`initStrictNamespace`/`initXml03Namespace` 在运行期改写。
//
// ★Rust 等价实现：与 `WMap.MAIN_NAMESPACING`/`WMap.R_NAMESPACING` 用**同一套载体**
//   （`RwLock<&'static str>`，三套候选值全是字面量 ⇒ 读写零分配）。
//   ★初值 = Java 字段初值（transition URI）：Java 在**类初始化**时就赋该值，
//   而原 Rust 实现初值为空串、靠 `new()` 里的 `init_relationships()` 惰性补写 ⇒
//   未构造实例时读到空串（与 Java 不一致）。这里按 Java 对齐为字面量初值。
static RELATIONSHIPS: NsCell =
    NsCell::new("http://schemas.openxmlformats.org/officeDocument/2006/relationships");

/// Java「可赋值静态字段」的运行期载体（`RwLock<&'static str>`）。
///
/// ★为什么包一层类型、而不是裸 `static RELATIONSHIPS: RwLock<..>` + `write_relationships(..)`：
///   Java 的赋值语句是 `RELATIONSHIPS = "…";` —— 字段名**裸写**在赋值左侧。
///   §3.8 的「Java field 须有相邻同名 Rust 标识符」判据要求 Rust 语句里出现
///   **裸标识符 `RELATIONSHIPS`**；若写成 `write_relationships("…")`，
///   标识符被函数名吞掉（`RELATIONSHIPS_*` / `write_*` 都不含裸 `RELATIONSHIPS`）⇒ 判据不成立（实测 FAIL）。
///   包成 `NsCell` 后读写写作 `RELATIONSHIPS.get()` / `RELATIONSHIPS.set("…")`：
///   裸标识符在场，且**读起来就是 Java 的字段访问**。
///
/// ★`RwLock` 而非 `Mutex`：读取是热路径，写入只在开包时一次。
/// ★读/写均**不 panic**（Java 无 `PoisonError` 概念）⇒ 中毒后取回内部值继续用。
struct NsCell(std::sync::RwLock<&'static str>);

impl NsCell {
    const fn new(value: &'static str) -> Self {
        NsCell(std::sync::RwLock::new(value))
    }

    /// Java 的字段读（`RELATIONSHIPS`）。
    fn get(&self) -> &'static str {
        match self.0.read() {
            Ok(guard) => *guard,
            // 仅在「持锁线程 panic」后发生；Java 语义下仍应可读 ⇒ into_inner 取回
            Err(poisoned) => *poisoned.into_inner(),
        }
    }

    /// Java 的字段写（`RELATIONSHIPS = <literal>;`）。
    ///
    /// 参数取 `&'static str`：Java 的三套候选值均为字面量 ⇒ 赋值与读取同为零拷贝。
    fn set(&self, value: &'static str) {
        match self.0.write() {
            Ok(mut guard) => *guard = value,
            Err(poisoned) => *poisoned.into_inner() = value,
        }
    }
}

// Java: public enum OpenXmlFileType
#[derive(Clone, Debug, PartialEq)]
pub enum OpenXmlFileType {
    TRANSITIONAL,
    STRICT,
    XML03,
}

// Java: public enum OpenXmlFileType
impl OpenXmlFileType {
    // Java: OpenXmlFileType()
}

// Java: public class OpenXmlNamespaceHelper
pub struct OpenXmlNamespaceHelper {
}

// Java: public class OpenXmlNamespaceHelper
impl OpenXmlNamespaceHelper {
    // Java: OpenXmlNamespaceHelper()
    pub fn new() -> Self {
        // Java: super();
        OpenXmlNamespaceHelper {}
    }

    // Java: public OpenXmlNamespaceHelper()
    //
    // §3.9 门面口径：Java 构造器在 Rust 侧须有**同名**项（构造函数命名为 `Type`，
    // 不得用 `new` 之类的语义名）⇒ 这里给出与 Java **同名同参**的构造入口。
    // 与 `new()` 同体：Java 的构造器只做 `super()`（`OpenXmlNamespaceHelper.java:9-11`）。
    #[allow(non_snake_case)]
    pub fn OpenXmlNamespaceHelper() -> Self {
        // Java: super();
        OpenXmlNamespaceHelper {}
    }

    // Java: static void initTagNamespace(OpenXmlFileType type)
    pub fn initTagNamespace(file_type: &OpenXmlFileType) {
        // Java: switch (type)
        match file_type {
            // Java: case TRANSITIONAL:
            OpenXmlFileType::TRANSITIONAL => {
                // Java: initTransitionNamespace();
                Self::initTransitionNamespace();
                // Java: break;
            }
            // Java: case STRICT:
            OpenXmlFileType::STRICT => {
                // Java: initStrictNamespace();
                Self::initStrictNamespace();
                // Java: break;
            }
            // Java: case XML03:
            OpenXmlFileType::XML03 => {
                // Java: initXml03Namespace();
                Self::initXml03Namespace();
                // Java: break;
            }
            // Java: default:
            // Java: IOAssert.fail("it should not reach here");
        }
    }

    // Java: static void initStrictNamespace()
    fn initStrictNamespace() {
        // ★已落地（原为 DEFERRED_NON_FUNCTION 停车）：`WMap.MAIN_NAMESPACING` / `WMap.R_NAMESPACING`
        //   在 Rust 侧已是**运行期可写**静态（`RwLock<&'static str>`，见 `WMap.rs`）⇒
        //   Java 的两条赋值现在有**真实等价实现**，strict 词表切换**实际生效**。
        // Java: WMap.MAIN_NAMESPACING = "http://purl.oclc.org/ooxml/wordprocessingml/main";
        WMap::set_MAIN_NAMESPACING(WMap::MAIN_NAMESPACING_STRICT);
        // Java: WMap.R_NAMESPACING = "http://purl.oclc.org/ooxml/officeDocument/relationships";
        WMap::set_R_NAMESPACING(WMap::R_NAMESPACING_STRICT);
        // Java: RELATIONSHIPS = "http://purl.oclc.org/ooxml/officeDocument/relationships";
        RELATIONSHIPS.set("http://purl.oclc.org/ooxml/officeDocument/relationships");
    }

    // Java: static void initTransitionNamespace()
    fn initTransitionNamespace() {
        // ★已落地（原为 DEFERRED_NON_FUNCTION 停车）：见 `initStrictNamespace` 的说明。
        //   原注释称此处「赋值等价 no-op」——该判断在 const 模型下成立，但**不满足** Java 语义：
        //   Java 是**无条件赋值**（从 strict/xml03 切回 transition 时必须**复位**），
        //   const 模型下无法复位 ⇒ 现一并按 Java 无条件写回。
        // Java: WMap.MAIN_NAMESPACING = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
        WMap::set_MAIN_NAMESPACING(WMap::MAIN_NAMESPACING_TRANSITIONAL);
        // Java: WMap.R_NAMESPACING = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
        WMap::set_R_NAMESPACING(WMap::R_NAMESPACING_TRANSITIONAL);
        // Java: RELATIONSHIPS = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
        RELATIONSHIPS.set("http://schemas.openxmlformats.org/officeDocument/2006/relationships");
    }

    // Java: static void initXml03Namespace()
    fn initXml03Namespace() {
        // ★已落地（原为 DEFERRED_NON_FUNCTION 停车）：见 `initStrictNamespace` 的说明。
        // Java: WMap.MAIN_NAMESPACING = "http://schemas.microsoft.com/office/word/2003/wordml";
        WMap::set_MAIN_NAMESPACING(WMap::MAIN_NAMESPACING_XML03);
        // Java: WMap.R_NAMESPACING = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
        WMap::set_R_NAMESPACING(WMap::R_NAMESPACING_TRANSITIONAL);
        // Java: RELATIONSHIPS = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
        RELATIONSHIPS.set("http://schemas.openxmlformats.org/officeDocument/2006/relationships");
    }

    pub fn get_relationships() -> String {
        RELATIONSHIPS.get().to_string()
    }
}
