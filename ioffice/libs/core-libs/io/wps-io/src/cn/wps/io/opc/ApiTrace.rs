// 55_function_error_trace —— S01 观测点与 `apiTrace` 落盘。
//
// 契约来源：
//   - `libs/word-core/rewrite-review/stages/S01-opc-package/docs/s01/schema-v1.json`（冻结）
//     · `open_entry` 为 `enum(bytes|path|raf|create)`（**4 值，不是方法名**）
//     · `diagnostics.warnings[]` 为 **`string[]`**（不是对象数组）
//     · 白名单 4 条：`zip.entries[].compressed_size`、`zip.entries[].local_header_offset`、
//       `diagnostics.elapsed_ms`、`meta.impl` ⇒ **`warnings[]` 不在白名单内，参与比较**
//   - `features/s01-09-error-model/functions/55_function_error_trace/FUNCTION.md` §1–§3
//   - 上游 `53_function_error_taxonomy`（`S01Error` / `kind` 词表）
//
// ★不变量（钉死）：
//   1. 字段齐全优先于字段正确：错误路径下五个 key **必须**仍在（值可为 null/0）。
//   2. `open_entry` 落盘值恒为 4 个枚举小写串之一；读链上 `create` 应恒不出现。
//   3. `detail` 落盘但不比较。
//   4. `part_count` = `parts[]` 条目数；`relationship_count` = 包级 + part 级之和
//      （★实测：Rust 侧 part 级恒为 0，因构造期不绑定关系）。
//   5. 静默面必须落 `warnings[]`（W1–W4）。
//   6. `warnings[]` 内容参与比较（不在白名单）。
//   7. 同一对象快照：计数不得二次遍历。

use crate::io::opc::S01Error::S01Error;
use std::fmt::Write as _;

/// `open_entry` 的 schema 枚举（**恰好 4 值**）。
///
/// `FUNCTION.md` §1.6-2：落盘值是这 4 个小写串，**不是**方法名（TR4 拒绝方法名）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OpenEntry {
    /// 从内存/流认包。Rust `XWPFPackage::open_stream` / Java `XWPFPackage.open(InputStream)`。
    Bytes,
    /// 从文件路径认包。
    Path,
    /// 从随机访问句柄认包。
    Raf,
    /// 新建空包（写侧；S01 读链**应恒不出现**，出现即失败）。
    Create,
}

impl OpenEntry {
    /// 落盘取值（小写串）。
    pub fn as_str(&self) -> &'static str {
        match self {
            OpenEntry::Bytes => "bytes",
            OpenEntry::Path => "path",
            OpenEntry::Raf => "raf",
            OpenEntry::Create => "create",
        }
    }

    /// 由落盘串回解；方法名等非法值返回 `None`（TR3 / TR4）。
    pub fn from_str(s: &str) -> Option<OpenEntry> {
        match s {
            "bytes" => Some(OpenEntry::Bytes),
            "path" => Some(OpenEntry::Path),
            "raf" => Some(OpenEntry::Raf),
            "create" => Some(OpenEntry::Create),
            _ => None,
        }
    }

    /// 4 个合法取值（顺序与 schema 枚举书写顺序一致）。
    pub fn all() -> [OpenEntry; 4] {
        [
            OpenEntry::Bytes,
            OpenEntry::Path,
            OpenEntry::Raf,
            OpenEntry::Create,
        ]
    }
}

impl std::fmt::Display for OpenEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// `record_open_entry` 的拒绝原因。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraceRejected {
    pub reason: String,
}

/// S01 管线观测点：`apiTrace` + `diagnostics.warnings[]`。
///
/// 字段与 schema 一一对应；`into_json()` 产出**确定性** JSON（key 排序稳定、UTF-8、`\n` 结尾）。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ApiTrace {
    open_entry: Option<OpenEntry>,
    part_count: Option<usize>,
    relationship_count: Option<usize>,
    main_part: Option<String>,
    error: Option<S01Error>,
    warnings: Vec<String>,
}

impl ApiTrace {
    pub fn new() -> Self {
        Self::default()
    }

    /// 记录实际走的认包入口。**只接受 4 个枚举值**（TR3 / TR4）。
    pub fn record_open_entry(&mut self, entry: OpenEntry) -> Result<(), TraceRejected> {
        self.open_entry = Some(entry);
        Ok(())
    }

    /// 同 [`Self::record_open_entry`]，但按落盘串记录；非法串**拒绝**（TR4）。
    ///
    /// 这是「拒绝，而不是静默接受方法名」的那道闸。
    pub fn record_open_entry_str(&mut self, entry: &str) -> Result<(), TraceRejected> {
        match OpenEntry::from_str(entry) {
            Some(v) => {
                self.open_entry = Some(v);
                Ok(())
            }
            None => Err(TraceRejected {
                reason: format!(
                    "open_entry must be one of bytes|path|raf|create, got {:?}",
                    entry
                ),
            }),
        }
    }

    /// 记录已认出的 part 数与关系数（**失败路径也必须给数**）。
    pub fn record_counts(&mut self, parts: usize, rels: usize) {
        self.part_count = Some(parts);
        self.relationship_count = Some(rels);
    }

    /// 记录主 part（`None` ⇒ 落盘 `null`，**key 仍须存在**）。
    pub fn record_main_part(&mut self, part_name: Option<&str>) {
        self.main_part = part_name.map(|s| s.to_string());
    }

    /// 记录 `53` 的分类结果（`None` ⇒ 落盘 `null`）。
    pub fn record_error(&mut self, e: S01Error) {
        self.error = Some(e);
    }

    /// 记录静默面条目。
    ///
    /// 落盘格式为 `<at>: <消息>`（schema 定型 `string[]`，非对象数组 ⇒ `at` 内嵌于串内）。
    pub fn record_warning(&mut self, at: &str, msg: &str) {
        self.warnings.push(format!("{}: {}", at, msg));
    }

    /// 静默面 W1–W4 的 `at` 前缀常量（`FUNCTION.md` §1.6-5）。
    pub const W1_PARSE_PART_XML_EARLY_RETURN: &'static str =
        "XWPFDocument::parse_part_xml#early_return";
    pub const W2_PART_DROPPED_NO_CT: &'static str = "OPCPackage::getPartsImpl#part_dropped_no_ct";
    pub const W3_NO_ARCHIVE: &'static str = "OPCPackage::getPartsImpl#no_archive";
    pub const W4_ERROR_TEXT_DROPPED: &'static str = "OPCPackage::from_stream#error_text_dropped";

    /// W4：丢弃原始 zip 错误文本时，**同时**保留 `error.detail`（TR10）。
    pub fn record_error_text_dropped(&mut self, raw: &str) {
        self.record_warning(Self::W4_ERROR_TEXT_DROPPED, "raw zip error text dropped");
        let _ = raw; // 原始文本由调用方经 `record_error` 写入 `detail`（见 TR10）
    }

    pub fn open_entry(&self) -> Option<OpenEntry> {
        self.open_entry
    }
    pub fn part_count(&self) -> Option<usize> {
        self.part_count
    }
    pub fn relationship_count(&self) -> Option<usize> {
        self.relationship_count
    }
    pub fn main_part(&self) -> Option<&str> {
        self.main_part.as_deref()
    }
    pub fn error(&self) -> Option<&S01Error> {
        self.error.as_ref()
    }
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// 比较投影：`(kind, at)`（`detail` 丢弃，与 `53` 的 `canonical_tuple` 同口径，TR15）。
    pub fn error_projection(&self) -> Option<(&'static str, &str)> {
        self.error.as_ref().map(|e| e.canonical_tuple())
            .map(|(k, at, _)| (k, at))
    }

    /// 落盘 JSON。
    ///
    /// **确定性**：key 固定顺序、UTF-8、以 `\n` 结尾（TR16 / TR17）。
    /// **字段齐全优先**：五个 key **恒在**，值为 `null`/`0` 也不缺（不变量 1 / TR1 / TR2 / TR5 / TR6）。
    /// **`elapsed_ms` 不得出现在 `apiTrace` 段内**（它在白名单里，属 `diagnostics` 兄弟段，TR19）。
    pub fn into_json(&self) -> String {
        let mut s = String::new();
        s.push_str("{\n");
        s.push_str("  \"apiTrace\": {\n");

        // open_entry：未记录 ⇒ null（key 仍须存在）
        match self.open_entry {
            Some(v) => {
                let _ = writeln!(s, "    \"open_entry\": \"{}\",", v.as_str());
            }
            None => s.push_str("    \"open_entry\": null,\n"),
        }

        // part_count / relationship_count：未记录 ⇒ 0（不变量 1：错误路径也必须给数）
        let _ = writeln!(
            s,
            "    \"part_count\": {},",
            self.part_count.unwrap_or(0)
        );
        let _ = writeln!(
            s,
            "    \"relationship_count\": {},",
            self.relationship_count.unwrap_or(0)
        );

        // main_part：None ⇒ null
        match &self.main_part {
            Some(v) => {
                let _ = writeln!(s, "    \"main_part\": \"{}\",", escape(v));
            }
            None => s.push_str("    \"main_part\": null,\n"),
        }

        // error：None ⇒ null；否则三个子 key 恒在
        match &self.error {
            Some(e) => {
                s.push_str("    \"error\": {\n");
                let _ = writeln!(s, "      \"kind\": \"{}\",", e.kind.as_str());
                let _ = writeln!(s, "      \"at\": \"{}\",", escape(&e.at));
                let _ = writeln!(s, "      \"detail\": \"{}\"", escape(&e.detail));
                s.push_str("    },\n");
            }
            None => s.push_str("    \"error\": null,\n"),
        }

        s.push_str("  },\n");

        // diagnostics.warnings[]（string[]；不在白名单 ⇒ 参与比较）
        s.push_str("  \"diagnostics\": {\n");
        s.push_str("    \"warnings\": [");
        if self.warnings.is_empty() {
            s.push_str("]\n");
        } else {
            s.push('\n');
            for (i, w) in self.warnings.iter().enumerate() {
                let comma = if i + 1 == self.warnings.len() { "" } else { "," };
                let _ = writeln!(s, "      \"{}\"{}", escape(w), comma);
            }
            s.push_str("    ]\n");
        }
        s.push_str("  }\n");
        s.push_str("}\n");
        s
    }
}

/// 最小 JSON 字符串转义（`"` / `\` / 控制字符）。
fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out
}

// NOTE(impl-trait): 本文件为 S01 阶段的**工装侧**观测点，不作为产品运行路径的一部分。
// 产品侧的错误收敛（`ImportOutcome`）由 `54_function_error_panic_guard` 在 `DocumentOpen` 边界完成。
//
// NOTE(IOAssert): 断言层（两侧全 no-op）**不产生任何** `warnings[]`/`error` 条目
// （不变量 6 的推论 / TR18）：本模块不存在任何以「断言失败」为来源的记录入口。
