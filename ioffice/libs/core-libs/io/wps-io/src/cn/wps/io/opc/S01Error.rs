// S01 失败信号 → 统一 `(kind, at, detail)` 分类器。
//
// 契约来源（**已冻结，不得偏离**）：
//   - `libs/word-core/rewrite-review/stages/S01-opc-package/docs/s01/schema-v1.json`
//     · `error_kinds`（10 值封闭集合）· `error_compare = ["error.kind","error.at"]`
//   - `features/s01-09-error-model/STRUCTURE_ANALYSIS.md` §2.1（判定顺序，先命中先返回）
//   - `features/s01-09-error-model/functions/53_function_error_taxonomy/FUNCTION.md` §1–§3
//
// 本文件**不是** Java 类的移植：Java 侧的失败是 `Throwable` 子类，Rust 侧是
// `panic!` payload / `Result::Err` / `Option::None` / 静默 return —— **输入形态不同，`kind` 必须相同**。
// 因此按 D0 口径以 `// Java:` 注释逐条登记 Java 侧的**对应失败点**，而无可直接镜像的单一 Java 方法。

use std::fmt;

/// 10 类 canonical `kind`（**封闭枚举**）。
///
/// 取值域与顺序**逐项等于**冻结契约 `schema-v1.json:error_kinds`。
/// **不允许**为「不知道」新增第 11 类；兜底一律 `IoError`（§2.1 兜底口径）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum S01ErrorKind {
    /// 规则 1：输入字节无法被识别为 zip。
    NotZip,
    /// 规则 2：已识别为 zip，但条目读取失败（含 size_guard：条目 > 1 GiB）。
    ZipCorrupt,
    /// 规则 5：CT part 缺失。
    MissingContentTypes,
    /// 规则 6：CT part 存在但不可解析。
    MalformedContentTypes,
    /// 规则 7：`officeDocument` 关系数 ≠ 1 或主 part 不可定位。
    MissingMainPart,
    /// 规则 4：part 名（或 zip entry 名）不满足 M1.x。
    InvalidPartName,
    /// 规则 8：目标 part 名合法但不在 partList。
    MissingPart,
    /// 规则 9：关系 id / 索引查不到。
    RelationshipNotFound,
    /// 规则 3：加密 / 受保护容器。
    UnsupportedEncryption,
    /// 规则 10：以上皆非，且来自 I/O 层（**唯一允许兜底的一类**）。
    IoError,
}

impl S01ErrorKind {
    /// 枚举 → canonical 字符串（小写下划线）。
    pub fn as_str(&self) -> &'static str {
        match self {
            S01ErrorKind::NotZip => "not_zip",
            S01ErrorKind::ZipCorrupt => "zip_corrupt",
            S01ErrorKind::MissingContentTypes => "missing_content_types",
            S01ErrorKind::MalformedContentTypes => "malformed_content_types",
            S01ErrorKind::MissingMainPart => "missing_main_part",
            S01ErrorKind::InvalidPartName => "invalid_part_name",
            S01ErrorKind::MissingPart => "missing_part",
            S01ErrorKind::RelationshipNotFound => "relationship_not_found",
            S01ErrorKind::UnsupportedEncryption => "unsupported_encryption",
            S01ErrorKind::IoError => "io_error",
        }
    }

    /// canonical 字符串 → 枚举（`as_str` 的逆；往返必须同一，ET19）。
    pub fn from_str(s: &str) -> Option<S01ErrorKind> {
        match s {
            "not_zip" => Some(S01ErrorKind::NotZip),
            "zip_corrupt" => Some(S01ErrorKind::ZipCorrupt),
            "missing_content_types" => Some(S01ErrorKind::MissingContentTypes),
            "malformed_content_types" => Some(S01ErrorKind::MalformedContentTypes),
            "missing_main_part" => Some(S01ErrorKind::MissingMainPart),
            "invalid_part_name" => Some(S01ErrorKind::InvalidPartName),
            "missing_part" => Some(S01ErrorKind::MissingPart),
            "relationship_not_found" => Some(S01ErrorKind::RelationshipNotFound),
            "unsupported_encryption" => Some(S01ErrorKind::UnsupportedEncryption),
            "io_error" => Some(S01ErrorKind::IoError),
            _ => None,
        }
    }

    /// 10 条 `kind`，顺序与冻结契约 `error_kinds` 逐项一致。
    pub fn all() -> [S01ErrorKind; 10] {
        [
            S01ErrorKind::NotZip,
            S01ErrorKind::ZipCorrupt,
            S01ErrorKind::MissingContentTypes,
            S01ErrorKind::MalformedContentTypes,
            S01ErrorKind::MissingMainPart,
            S01ErrorKind::InvalidPartName,
            S01ErrorKind::MissingPart,
            S01ErrorKind::RelationshipNotFound,
            S01ErrorKind::UnsupportedEncryption,
            S01ErrorKind::IoError,
        ]
    }
}

impl fmt::Display for S01ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 分类器的输入：五形态失败信号（`FUNCTION.md` §1.2）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum S01Failure {
    /// Rust `panic!` 的 payload 文本。
    ///
    /// Java 对应：dumper 在设备上 `catch (Throwable t)` 后取类名（Java 侧异常即 payload）。
    Panic { payload: String },
    /// `Result<_, String>::Err` 的消息。
    ///
    /// Java 对应：`OPCPackage.java:529-539`（zip 读取失败）、
    /// `PackagePartName.java:120-247`（M1.x 名合规失败）。
    ZipErr { msg: String },
    /// `Option::None` 的**调用上下文**；调用方**必须**提供 `at`。
    ///
    /// Java 对应：`OPCPackage.java:510-512`（part 查不到）、
    /// `PackageRelationshipCollection.java:135-137`（rId 查不到）。
    OptionMiss { at: String },
    /// Java dumper 捕获的异常类名 + 行号。
    ///
    /// Java 对应：`FileDamagedException` / `InvalidFormatException` /
    /// `InvalidOperationException` / `OpenXML4JRuntimeException` / `POIXMLException`。
    JavaThrowable { class_name: String, line: u32 },
    /// 被显式化的静默 return 点。
    ///
    /// Java 对应：`OPCPackage.java:395-398`、`:274-275`（空吞降级）。
    SilentReturn { at: String },
}

/// 分类结果三元组。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct S01Error {
    pub kind: S01ErrorKind,
    /// `<Rust 类型>::<方法>#<锚点>`；形状见 [`is_valid_at`]。
    pub at: String,
    /// 原始消息文本（人读）。**不参与两侧比较**。
    pub detail: String,
}

impl S01Error {
    /// 投影出**比较面**：丢弃 `detail`（契约 `error_compare` 只含 `kind`/`at`，ET18）。
    pub fn canonical_tuple(&self) -> (&'static str, &str, &'static str) {
        (self.kind.as_str(), self.at.as_str(), "")
    }

    /// 10 条 `kind` 字符串常量（封闭集合，ET1）。
    pub fn kinds() -> [&'static str; 10] {
        [
            "not_zip",
            "zip_corrupt",
            "missing_content_types",
            "malformed_content_types",
            "missing_main_part",
            "invalid_part_name",
            "missing_part",
            "relationship_not_found",
            "unsupported_encryption",
            "io_error",
        ]
    }
}

impl fmt::Display for S01Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}: {}", self.kind.as_str(), self.at, self.detail)
    }
}

/// 分类被拒绝（空 `at`、空 payload 等畸形输入）。
///
/// **拒绝而非静默兜底**：`at` 为空时不得返回带空 `at` 的结果（不变量 3 / ET16）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassifyRejected {
    pub reason: String,
}

/// `at` 形状：`<路径>::<方法>#<锚点>`（不变量 3 / ET17）。
///
/// ⚠️ **口径澄清（★本轮实测发现规格内部张力，以权威表为准）**：
/// `FUNCTION.md` §1.6 不变量 3 给出的正则
/// `^[A-Za-z_][A-Za-z0-9_]*::[A-Za-z_][A-Za-z0-9_]*#[A-Za-z0-9_.]+$`
/// 只允许**两段** `::` 且锚点段不含 `:`；但**权威 `at` 绑定表**
/// （`FEATURE.md` §1，10 主 + 2 子触发点，「唯一真源」）的 canonical 锚点**实际形态**为：
///
/// ```text
/// ① xwpf::util::ZipInputStreamZipEntrySource::parse_zip_bytes#ZipArchive::new   （#1 not_zip）
/// ② xwpf::util::ZipInputStreamZipEntrySource::parse_zip_bytes#by_index          （#2 zip_corrupt）
/// ③ opc::OPCPackage::getPartsImpl#pass1.size_guard                              （#2b）
/// ④ opc::OPCPackage::getPartsImpl#pass1.ct_part_absent                          （#3）
/// ⑤ opc::internal::ContentTypeManager::parseContentTypesFile#parse_err          （#4）
/// ⑥ xwpf::usermodel::XWPFDocument::XWPFDocument__pkg#main_rel_assert            （#5）
/// ⑦ opc::PackagingURIHelper::createPartName_uri#conformance                     （#6）
/// ⑧ opc::OPCPackage::getPart#part_list_miss                                     （#7）
/// ⑨ opc::PackageRelationshipCollection::getRelationshipByID#miss                （#8）
/// ⑩ xwpf::util::ZipInputStreamZipEntrySource::parse_zip_bytes#unsupported_archive（#9）
/// ⑪ opc::OPCPackage::from_path#file_read                                        （#10）
/// ⑫ opc::PackageRelationshipCollection::getRelationship_by_index#bounds         （#8 次）
/// ```
///
/// ⇒ 实测：**路径段为 2–4 个标识符**（`opc::internal::ContentTypeManager` 是 3 段、
/// `xwpf::util::ZipInputStreamZipEntrySource` 是 3 段），**锚点段可含 `::`**（①②）。
/// 本函数因此以**绑定表 12 行为准**实现：
/// `<ident>(::<ident>){1,3} # <anchor>`，`anchor` 允许 `[A-Za-z0-9_.:]`。
/// 这样 12 行**全部**通过 ET17，且仍拒绝空串 / 缺 `#` / 空段 / 首字符非法等畸形。
pub fn is_valid_at(at: &str) -> bool {
    let bytes = at.as_bytes();
    let mut i = 0usize;

    /// 读一个标识符段；返回新的下标，失败返回 `None`。
    fn ident(bytes: &[u8], mut i: usize) -> Option<usize> {
        if i >= bytes.len() || !(bytes[i].is_ascii_alphabetic() || bytes[i] == b'_') {
            return None;
        }
        i += 1;
        while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
            i += 1;
        }
        Some(i)
    }

    // 路径段：1 个标识符 + 1–3 组 `::标识符`
    i = match ident(bytes, i) {
        Some(v) => v,
        None => return false,
    };
    let mut segs = 1usize;
    while i + 1 < bytes.len() && bytes[i] == b':' && bytes[i + 1] == b':' {
        match ident(bytes, i + 2) {
            Some(v) => {
                i = v;
                segs += 1;
            }
            None => break, // `::` 后非标识符 ⇒ 交给下面按 `#` 处理（非法）
        }
    }
    if segs < 2 {
        return false; // 至少 `Type::method`
    }

    // 分隔符 `#`
    if i >= bytes.len() || bytes[i] != b'#' {
        return false;
    }
    i += 1;

    // 锚点段：字母数字 / `_` / `.` / `:`，至少 1 字符
    let s3 = i;
    while i < bytes.len()
        && (bytes[i].is_ascii_alphanumeric()
            || bytes[i] == b'_'
            || bytes[i] == b'.'
            || bytes[i] == b':')
    {
        i += 1;
    }
    if i == s3 {
        return false;
    }

    i == bytes.len()
}

/// 判定顺序（`STRUCTURE_ANALYSIS.md` §2.1）：**先命中先返回**。
pub struct S01Taxonomy;

impl S01Taxonomy {
    /// 把 `S01Failure` 归到 10 类之一。
    ///
    /// `at` 非空且形状合法是**前置**；畸形输入返回 `Err(ClassifyRejected)`。
    pub fn classify(f: &S01Failure) -> Result<S01Error, ClassifyRejected> {
        match f {
            // Java: catch (IOException e) { throw new InvalidFormatException("Zip archive is not valid", e); }
            // —— `OPCPackage.java:529-539`
            S01Failure::ZipErr { msg } => Self::classify_zip_err(msg),

            // Java: catch (Throwable t) { kind = t.getClass().getSimpleName(); }
            S01Failure::JavaThrowable { class_name, line } => {
                Self::classify_java_throwable(class_name, *line)
            }

            // Rust panic 特有形态（Java 侧异常即 payload）
            S01Failure::Panic { payload } => Self::classify_panic(payload),

            // Java: if (!partList.containsKey(partName)) return null;  —— `OPCPackage.java:510-512`
            S01Failure::OptionMiss { at } => Self::classify_option_miss(at),

            // Java: `OPCPackage.java:395-398` / `:274-275` —— 空吞降级点
            S01Failure::SilentReturn { at } => Self::classify_silent_return(at),
        }
    }

    fn reject<T>(reason: &str) -> Result<T, ClassifyRejected> {
        Err(ClassifyRejected {
            reason: reason.to_string(),
        })
    }

    /// `at` 必须非空**且形状合法**。
    fn require_at(at: &str) -> Result<(), ClassifyRejected> {
        if at.trim().is_empty() {
            return Self::reject("at must not be empty");
        }
        if !is_valid_at(at) {
            return Self::reject("at does not match the required shape");
        }
        Ok(())
    }

    /// 规则 1/2/3/4 的 `Err` 分流；**if 链顺序即判定顺序**。
    fn classify_zip_err(msg: &str) -> Result<S01Error, ClassifyRejected> {
        let m = msg.to_lowercase();

        // 规则 1：无法识别为 zip
        // Java: throw new InvalidFormatException("Zip archive is not valid");
        // —— ET2：`at` 锚点为 `#ZipArchive::new`
        if m.contains("invalid zip")
            || m.contains("not a zip")
            || m.contains("not zip")
            || m.contains("end of central directory")
            || m.contains("invalid central directory")
        {
            return Ok(Self::mk(
                S01ErrorKind::NotZip,
                "xwpf::util::ZipInputStreamZipEntrySource::parse_zip_bytes#ZipArchive::new",
                msg,
            ));
        }

        // 规则 3：加密 / 受保护容器（判定顺序 #3 在 #1 之后，须能与 not_zip 区分 —— ET5）
        // Java: throw new UnsupportedOperationException("encrypted package");
        if m.contains("encrypt")
            || m.contains("password")
            || m.contains("unsupported_archive")
            || m.contains("protected")
        {
            return Ok(Self::mk(
                S01ErrorKind::UnsupportedEncryption,
                "xwpf::util::ZipInputStreamZipEntrySource::parse_zip_bytes#unsupported_archive",
                msg,
            ));
        }

        // 规则 4：part 名不满足 M1.x
        // Java: throw new InvalidOperationException("A segment shall not hold any characters other than pchar characters. [M1.6]");
        // —— `PackagePartName.java:120-247`；M1.6 实测文本见 s01-03 §2.2
        if m.contains("[m1.") || m.contains("pchar") || m.contains("shall not hold") {
            return Ok(Self::mk(
                S01ErrorKind::InvalidPartName,
                "opc::PackagingURIHelper::createPartName_str#m1x_conformance",
                msg,
            ));
        }

        // 规则 2：已识别为 zip，但条目读取失败（含 size_guard）
        // Java: throw new FileDamagedException();
        // —— ET3：锚点为 `#by_index`；`OPCPackage.java:47` MAX_SIZE_ENTRY 判据为 `>`
        if m.contains("damaged")
            || m.contains("too large")
            || m.contains("size guard")
            || m.contains("corrupt")
            || m.contains("crc")
            || m.contains("unexpected eof")
            || m.contains("decompress")
        {
            return Ok(Self::mk(
                S01ErrorKind::ZipCorrupt,
                "xwpf::util::ZipInputStreamZipEntrySource::parse_zip_bytes#by_index",
                msg,
            ));
        }

        // 规则 10：兜底
        // —— ET14：不可分类文本 ⇒ io_error（不得返回 Option::None / 空 kind）
        Ok(Self::mk(
            S01ErrorKind::IoError,
            "xwpf::util::ZipInputStreamZipEntrySource::parse_zip_bytes#io",
            msg,
        ))
    }

    /// Java 异常类名 → `kind`（只比 `kind`，**不比类名**）。
    fn classify_java_throwable(
        class_name: &str,
        line: u32,
    ) -> Result<S01Error, ClassifyRejected> {
        if class_name.trim().is_empty() {
            return Self::reject("class_name must not be empty");
        }
        // ET6：`at` 锚点 = `PackagingURIHelper::createPartName_uri#conformance`
        let n = class_name.to_lowercase();

        // Java: throw new InvalidFormatException("Invalid part name") / InvalidOperationException(M1.x)
        if n.contains("invalidformat") || n.contains("invalidoperation") {
            return Ok(Self::mk(
                S01ErrorKind::InvalidPartName,
                "opc::PackagingURIHelper::createPartName_uri#conformance",
                &format!("{}#line{}", class_name, line),
            ));
        }
        // Java: throw new FileDamagedException();
        if n.contains("filedamaged") {
            return Ok(Self::mk(
                S01ErrorKind::ZipCorrupt,
                "opc::OPCPackage::getPartsImpl#size_guard",
                &format!("{}#line{}", class_name, line),
            ));
        }
        // Java: throw new POIXMLException / OpenXML4JRuntimeException
        if n.contains("poixml") || n.contains("openxml") {
            return Ok(Self::mk(
                S01ErrorKind::MissingMainPart,
                "opc::model::POIXMLDocument#main_part",
                &format!("{}#line{}", class_name, line),
            ));
        }
        Ok(Self::mk(
            S01ErrorKind::IoError,
            "opc::OPCPackage::throwable#java",
            &format!("{}#line{}", class_name, line),
        ))
    }

    /// Rust `panic!` payload → `kind`。
    ///
    /// **`at` 语境优先于 payload**：ET4 与 ET7 的 payload 相同
    /// （都是 `FileDamagedException`），但 `at` 不同 ⇒ `kind` 必须不同。
    fn classify_panic(payload: &str) -> Result<S01Error, ClassifyRejected> {
        if payload.trim().is_empty() {
            return Self::reject("panic payload must not be empty");
        }
        let _ = payload;
        // 说明：`at` 由调用方在 `PanicContext` 中提供（见 `classify_panic_at`）；
        // 本入口在无 `at` 语境时按 payload 归类，供 payload-only 场景使用。
        Ok(Self::classify_panic_at(payload, None))
    }

    /// 带 `at` 语境的 panic 分类（ET4 / ET7 / ET13 的分流入口）。
    ///
    /// 这是**产品与 dump 侧应当使用的入口**：只有 `at` 才能区分
    /// 「尺寸门禁触发（zip_corrupt）」与「entry 名门禁触发（invalid_part_name）」。
    pub fn classify_panic_at(payload: &str, at: Option<&str>) -> S01Error {
        // ★**空 payload 优先于 `at` 语境**（`54` 的 PG4 / PG5）：
        // payload 为空（或不可读）时**没有真实证据**，必须兜底 `io_error`，
        // **不得**因为 `at` 恰好落在某个语境内就产出具体 `kind`
        // （否则「无 payload + size_guard 语境」会凭空得到 `zip_corrupt`，属把假推理写进实现）。
        if payload.trim().is_empty() {
            let anchor = at.filter(|s| !s.trim().is_empty()).unwrap_or("unknown");
            return Self::mk(S01ErrorKind::IoError, anchor, payload);
        }
        let p = payload.to_lowercase();
        let a = at.unwrap_or("").to_lowercase();

        // `at` 语境优先：entry_name_guard ⇒ invalid_part_name
        // Java: checkEntryNameLegal → throw new InvalidFormatException(...)
        if a.contains("entry_name_guard") || a.contains("name_guard") {
            return Self::mk(
                S01ErrorKind::InvalidPartName,
                "opc::OPCPackage::getPartsImpl#pass1.entry_name_guard",
                payload,
            );
        }
        // ★`read_document_xml` 的 **4 个 Zip 构造点**（`54` 不变量 4 / PG8 / PG9）：
        // 产品链直读 zip 的入口，`LayoutErrorKind::Zip` 一档必须靠 `at` 四分。
        // Java: 无对应（Java 经 `OPCPackage.getPart` 走关系图；见 PLAN §1.4-3）。
        if a.contains("pk_header") {
            // 前 2 字节不是 `PK` ⇒ 根本不是 zip
            return Self::mk(
                S01ErrorKind::NotZip,
                "word::moffice::writer::api::LayoutDocument::read_document_xml#PK_header",
                payload,
            );
        }
        if a.contains("by_name") {
            // 找不到 `word/document.xml` ⇒ 主 part 缺失（**不是** not_zip）
            return Self::mk(
                S01ErrorKind::MissingMainPart,
                "word::moffice::writer::api::LayoutDocument::read_document_xml#by_name",
                payload,
            );
        }
        if a.contains("ziparchive_new") {
            return Self::mk(
                S01ErrorKind::ZipCorrupt,
                "word::moffice::writer::api::LayoutDocument::read_document_xml#ZipArchive_new",
                payload,
            );
        }
        if a.contains("read_document_xml") && a.contains("#read") {
            return Self::mk(
                S01ErrorKind::ZipCorrupt,
                "word::moffice::writer::api::LayoutDocument::read_document_xml#read",
                payload,
            );
        }
        // `at` 语境优先：size_guard ⇒ zip_corrupt（绑定表 #2b）
        if a.contains("size_guard") {
            return Self::mk(
                S01ErrorKind::ZipCorrupt,
                "opc::OPCPackage::getPartsImpl#pass1.size_guard",
                payload,
            );
        }
        // `at` 语境优先：zip 条目读失败 ⇒ zip_corrupt（绑定表 #2，#by_index）
        // Java: `ZipInputStreamZipEntrySource.java` 的 catch ⇒ 静默；Rust 侧为 panic
        // ★绑定表 #1（`not_zip`）必须先于 #2（`zip_corrupt`）判定：
        // 两者的 `at` **都含** `parse_zip_bytes`，只有锚点段不同
        // （`#ZipArchive::new` vs `#by_index`）⇒ 必须**先**匹配更具体的锚点，
        // 否则 `#ZipArchive::new` 会被下面的宽规则吞掉（PG7 实测到的顺序缺陷）。
        if a.contains("ziparchive::new") {
            return Self::mk(
                S01ErrorKind::NotZip,
                "xwpf::util::ZipInputStreamZipEntrySource::parse_zip_bytes#ZipArchive::new",
                payload,
            );
        }
        // 绑定表 #2：条目读失败 ⇒ `zip_corrupt`（锚点 `#by_index`）。
        // 注意：只认 **zip 源**路径的 `#by_index`；
        // 关系集合的 `getRelationship_by_index#bounds` 属绑定表 #8 次，必须在下面命中。
        if a.contains("#by_index") || a.contains("zip_corrupt")
        {
            return Self::mk(
                S01ErrorKind::ZipCorrupt,
                "xwpf::util::ZipInputStreamZipEntrySource::parse_zip_bytes#by_index",
                payload,
            );
        }
        // `at` 语境优先：关系索引越界 / rId 查不到 ⇒ relationship_not_found（绑定表 #8 次）
        // Java: return null;（`PackageRelationshipCollection.java:135-137`）
        if a.contains("bounds")
            || a.contains("getrelationshipbyid")
            || a.contains("by_id")
            || a.contains("by_index")
        {
            return Self::mk(
                S01ErrorKind::RelationshipNotFound,
                "opc::PackageRelationshipCollection::getRelationship_by_index#bounds",
                payload,
            );
        }
        // `at` 语境优先：CT 解析失败 ⇒ malformed_content_types
        if a.contains("parse_err") || a.contains("parsecontenttypes") {
            return Self::mk(
                S01ErrorKind::MalformedContentTypes,
                "opc::internal::ContentTypeManager::parseContentTypesFile#parse_err",
                payload,
            );
        }
        // `at` 语境优先：主 part 断言 ⇒ missing_main_part
        if a.contains("main_rel") {
            return Self::mk(
                S01ErrorKind::MissingMainPart,
                "xwpf::usermodel::XWPFDocument::XWPFDocument__pkg#main_rel_assert",
                payload,
            );
        }

        // 无 `at` 语境时按 payload 归类
        if p.contains("filedamaged") {
            return Self::mk(
                S01ErrorKind::ZipCorrupt,
                "opc::OPCPackage::getPartsImpl#pass1.size_guard",
                payload,
            );
        }
        if p.contains("damaged") || p.contains("corrupt") {
            return Self::mk(
                S01ErrorKind::ZipCorrupt,
                "opc::OPCPackage::getPartsImpl#zip_corrupt",
                payload,
            );
        }
        if p.contains("[m1.") || p.contains("pchar") || p.contains("shall not hold") {
            return Self::mk(
                S01ErrorKind::InvalidPartName,
                "opc::PackagingURIHelper::createPartName_str#m1x_conformance",
                payload,
            );
        }
        if p.contains("relationship part") {
            return Self::mk(
                S01ErrorKind::InvalidPartName,
                "opc::PackagingURIHelper::getRelationshipPartName#rels_guard",
                payload,
            );
        }
        // ET9：`Can't read content types part !` ⇒ malformed_content_types
        if p.contains("content types") || p.contains("contenttypes") {
            return Self::mk(
                S01ErrorKind::MalformedContentTypes,
                "opc::internal::ContentTypeManager::parseContentTypesFile#parse_err",
                payload,
            );
        }
        // ET10：主文档关系数 ≠ 1 ⇒ missing_main_part
        if p.contains("main document relationship") || p.contains("main part") {
            return Self::mk(
                S01ErrorKind::MissingMainPart,
                "xwpf::usermodel::XWPFDocument::XWPFDocument__pkg#main_rel_assert",
                payload,
            );
        }
        // ET13：索引越界 ⇒ relationship_not_found（这是 Java 返回 null / Rust panic 的同 kind 归一点）
        if p.contains("index") || p.contains("bounds") {
            return Self::mk(
                S01ErrorKind::RelationshipNotFound,
                "opc::PackageRelationshipCollection::getRelationship_by_index#bounds",
                payload,
            );
        }
        Self::mk(S01ErrorKind::IoError, "opc::OPCPackage#panic", payload)
    }

    /// 规则 5–9：`None` 的**语境**决定 `kind`（不变量 5 / ET8 / ET11 / ET12 / ET22）。
    fn classify_option_miss(at: &str) -> Result<S01Error, ClassifyRejected> {
        Self::require_at(at)?;
        let a = at.to_lowercase();

        // 规则 9：关系 id / 索引查不到
        // Java: if (!relationshipsMap.containsKey(id)) return null;
        if a.contains("relationshipbyid")
            || a.contains("getrelationshipbyid")
            || a.contains("#rid")
            || a.contains("by_id")
            || a.contains("rel_id")
        {
            return Ok(Self::mk(
                S01ErrorKind::RelationshipNotFound,
                at,
                "relationship id lookup miss",
            ));
        }
        // 规则 5/6：CT part
        if a.contains("ct_part") || a.contains("content_types") || a.contains("contenttype") {
            if a.contains("parse") || a.contains("unmarshall") {
                return Ok(Self::mk(
                    S01ErrorKind::MalformedContentTypes,
                    at,
                    "content types present but unparsable",
                ));
            }
            return Ok(Self::mk(
                S01ErrorKind::MissingContentTypes,
                at,
                "content types part missing",
            ));
        }
        // 规则 7：主 part 不可定位
        if a.contains("main_part") || a.contains("officedocument") || a.contains("main_rel") {
            return Ok(Self::mk(
                S01ErrorKind::MissingMainPart,
                at,
                "main part not resolvable",
            ));
        }
        // 规则 8：查 part 语境（默认）
        // Java: if (!partList.containsKey(partName)) return null;
        Ok(Self::mk(
            S01ErrorKind::MissingPart,
            at,
            "part not in partList",
        ))
    }

    /// 静默 return 点被显式化后的分类。
    ///
    /// Java: `OPCPackage.java:395-398`、`:274-275` 空吞降级 ⇒ 归入其所处阶段。
    fn classify_silent_return(at: &str) -> Result<S01Error, ClassifyRejected> {
        Self::require_at(at)?;
        let a = at.to_lowercase();
        // ET8：`#pass1.ct_part_absent` ⇒ missing_content_types
        if a.contains("ct_part") || a.contains("content_types") || a.contains("contenttype") {
            return Ok(Self::mk(
                S01ErrorKind::MissingContentTypes,
                at,
                "content types part absent (silent return)",
            ));
        }
        if a.contains("part") {
            return Ok(Self::mk(
                S01ErrorKind::MissingPart,
                at,
                "silent fallback while resolving part",
            ));
        }
        Ok(Self::mk(
            S01ErrorKind::IoError,
            at,
            "silent fallback while reading package",
        ))
    }

    fn mk(kind: S01ErrorKind, at: &str, detail: &str) -> S01Error {
        S01Error {
            kind,
            at: at.to_string(),
            detail: detail.to_string(),
        }
    }
}

// NOTE(impl-trait): 本文件为 S01 阶段的**工装侧**分类器（dump/compare 用），
// 不是产品运行路径的一部分；产品侧的错误收敛由 `54_function_error_panic_guard`
// 在 `DocumentOpen` 边界处完成。
//
// NOTE(IOAssert): Java `IOAssert` 43 个 void 方法**全为 no-op**（`IOAssert.java:4/10-165`），
// `opc/`+`xwpf/` 202 处断言调用点**不产生任何信号** ⇒ 本分类器**只接受真实 `throw`/`panic`**，
// **禁止**把「断言本该抛」映射成任何 `kind`（不变量 6 / ET21）。
