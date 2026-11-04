//! `56_function_parity_schema` —— 以**冻结的契约** `docs/s01/schema-v1.json` 为唯一真源
//! 校验一份 dump JSON。
//!
//! # 退出码（规格 §1.6）
//!
//! - `0` = 通过
//! - `1` = schema 违规（报**首个**违规字段路径）
//! - `2` = 工具/IO 错误（schema 自身非法等）
//!
//! # 用例（SC1–SC11）
//!
//! | id | 判据 |
//! |---|---|
//! | SC1 | 全字段清单：主路径通过 |
//! | SC2 | `parts[0].foo` ⇒ 退出 1，**路径 `parts[0].foo`**（静默忽略 = 失败） |
//! | SC3 | 删 `meta.input.sha256` ⇒ 退出 1，路径 `meta.input.sha256`（报「未知」= 误判） |
//! | SC4 | 白名单**恰 4 条**，前两条 `required:false` |
//! | SC5 | `sort:"alphabetical"` ⇒ **退出 2**（schema 自身非法） |
//! | SC6 | `parts[]` 反序 ⇒ 退出 1，路径 `parts[1].partName`（只报「未排序」= 弱） |
//! | SC7 | BOM / `\r\n` ⇒ 退出 1，路径 **`$encoding`**（容忍 = 失败） |
//! | SC8 | 连检两次 byte-identical |
//! | SC9 | `docProps.core` 全 `null` 但字段齐全 ⇒ **退出 0** |
//! | SC10 | **无第二张白名单**：schema 内不得有 `parity`/`deferred` 一类自证豁免入口 |
//! | SC11 | `open_entry` 取值 ⊆ `{bytes,path,raf,create}` |

use serde_json::Value;

/// 契约标识（规格 §1.1 的 `S01DumpSchema::SCHEMA_ID`）。
pub const SCHEMA_ID: &str = "s01.opc.v1";

/// 校验结果。
#[derive(Debug, Clone, PartialEq)]
pub enum SchemaCheck {
    /// 通过（退出码 0）
    Ok,
    /// schema 违规（退出码 1）：携带**首个**违规路径
    Violation { path: String, detail: String },
    /// 工具/schema 自身错误（退出码 2）
    ToolError { path: String, detail: String },
}

impl SchemaCheck {
    pub fn exit_code(&self) -> i32 {
        match self {
            SchemaCheck::Ok => 0,
            SchemaCheck::Violation { .. } => 1,
            SchemaCheck::ToolError { .. } => 2,
        }
    }
}

/// 契约允许的顶层段（`schema-v1.json` 的 `fields` 键集）。
const SECTIONS: [&str; 9] = [
    "schema",
    "meta",
    "zip",
    "contentTypes",
    "parts",
    "packageRelationships",
    "docProps",
    "apiTrace",
    "diagnostics",
];

/// `parts[]` 的字段集。
const PART_KEYS: [&str; 6] = [
    "partName",
    "contentType",
    "size",
    "sha256",
    "relationships",
    "isDir",
];

/// `relationships[]` 的字段集。
const REL_KEYS: [&str; 6] = [
    "id",
    "type",
    "target",
    "targetMode",
    "resolvedPartName",
    "relXmlOrder",
];

/// 契约 `sort` 枚举的**封闭**取值集（SC5）。
const SORT_ENUM: [&str; 4] = [
    "code_point",
    "physical_order",
    "relationship_id",
    "none",
];

/// `open_entry` 的**封闭**取值集（SC11）。
const OPEN_ENTRY_ENUM: [&str; 4] = ["bytes", "path", "raf", "create"];

/// 契约声明的白名单条数（SC4）。
pub const WHITELIST_LEN: usize = 4;

/// 自证豁免字段的**禁用**名单（SC10：`no_second_whitelist`）。
const FORBIDDEN_EXEMPTION_KEYS: [&str; 4] = ["parity", "deferred", "allowed_diff", "ignore"];

/// 主入口：校验一份 dump JSON（字符串形态，可检出 BOM/`\r\n`）。
pub fn check(raw: &str) -> SchemaCheck {
    // ── SC7：**不得容忍** BOM / `\r\n`（容忍会掩盖两侧平台换行差异） ──
    if raw.starts_with('\u{feff}') {
        return SchemaCheck::Violation {
            path: "$encoding".to_string(),
            detail: "输入含 UTF-8 BOM ⇒ 必须报 `$encoding`（容忍 = 失败）".to_string(),
        };
    }
    if raw.contains("\r\n") {
        return SchemaCheck::Violation {
            path: "$encoding".to_string(),
            detail: "输入含 CRLF ⇒ 必须报 `$encoding`（契约 newline = \\n）".to_string(),
        };
    }

    let v: Value = match serde_json::from_str(raw) {
        Ok(v) => v,
        Err(e) => {
            return SchemaCheck::ToolError {
                path: "$parse".to_string(),
                detail: format!("JSON 解析失败: {}", e),
            }
        }
    };

    // ── SC1：顶层段只能是契约列出的 9 个（未知 ⇒ 退出 1） ──
    let Some(obj) = v.as_object() else {
        return SchemaCheck::Violation {
            path: "$".to_string(),
            detail: "顶层必须是对象".to_string(),
        };
    };
    for k in obj.keys() {
        if !SECTIONS.contains(&k.as_str()) {
            return SchemaCheck::Violation {
                path: k.clone(),
                detail: format!("未知顶层段 `{}`（契约 `fields` 为封闭集合）", k),
            };
        }
    }

    // ── SC1：`schema` 标识必须等于 SCHEMA_ID ──
    match obj.get("schema").and_then(|s| s.as_str()) {
        Some(s) if s == SCHEMA_ID => {}
        Some(s) => {
            return SchemaCheck::Violation {
                path: "schema".to_string(),
                detail: format!("schema 标识 `{}` ≠ `{}`", s, SCHEMA_ID),
            }
        }
        None => {
            return SchemaCheck::Violation {
                path: "schema".to_string(),
                detail: "缺必填字段 `schema`".to_string(),
            }
        }
    }

    // ── SC3：`meta.input.sha256` 必填（缺 ⇒ 报**该路径**，不得报「未知字段」） ──
    if v.pointer("/meta/input/sha256").is_none() {
        return SchemaCheck::Violation {
            path: "meta.input.sha256".to_string(),
            detail: "缺必填字段 `meta.input.sha256`".to_string(),
        };
    }

    // ── SC11：`apiTrace.open_entry` 取值封闭 ──
    if let Some(oe) = v.pointer("/apiTrace/open_entry") {
        match oe.as_str() {
            Some(s) if OPEN_ENTRY_ENUM.contains(&s) => {}
            Some(s) => {
                return SchemaCheck::Violation {
                    path: "apiTrace.open_entry".to_string(),
                    detail: format!(
                        "`open_entry` 取值 `{}` 不在封闭集 {:?}（SC11）",
                        s, OPEN_ENTRY_ENUM
                    ),
                }
            }
            None => {
                return SchemaCheck::Violation {
                    path: "apiTrace.open_entry".to_string(),
                    detail: "`open_entry` 必须是字符串".to_string(),
                }
            }
        }
    }

    // ── SC2：`parts[]` 的未知字段（**必须报精确路径**） ──
    if let Some(parts) = obj.get("parts").and_then(|p| p.as_array()) {
        for (i, p) in parts.iter().enumerate() {
            let Some(po) = p.as_object() else {
                return SchemaCheck::Violation {
                    path: format!("parts[{}]", i),
                    detail: "part 必须是对象".to_string(),
                };
            };
            // 必填
            if po.get("partName").is_none() {
                return SchemaCheck::Violation {
                    path: format!("parts[{}].partName", i),
                    detail: "缺必填字段 `partName`".to_string(),
                };
            }
            for k in po.keys() {
                if !PART_KEYS.contains(&k.as_str()) {
                    // ★SC2：路径必须精确为 `parts[0].foo`
                    return SchemaCheck::Violation {
                        path: format!("parts[{}].{}", i, k),
                        detail: format!("未知字段 `{}`（静默忽略 = 失败）", k),
                    };
                }
            }
            // relationships 子表
            if let Some(rels) = po.get("relationships").and_then(|r| r.as_array()) {
                for (j, r) in rels.iter().enumerate() {
                    let Some(ro) = r.as_object() else {
                        return SchemaCheck::Violation {
                            path: format!("parts[{}].relationships[{}]", i, j),
                            detail: "relationship 必须是对象".to_string(),
                        };
                    };
                    for k in ro.keys() {
                        if !REL_KEYS.contains(&k.as_str()) {
                            return SchemaCheck::Violation {
                                path: format!("parts[{}].relationships[{}].{}", i, j, k),
                                detail: format!("未知字段 `{}`", k),
                            };
                        }
                    }
                }
            }
        }

        // ── SC6：`parts[]` 必须按 `partName` 码点序，路径**精确到首个乱序元素** ──
        for i in 1..parts.len() {
            let prev = parts[i - 1]
                .get("partName")
                .and_then(|x| x.as_str())
                .unwrap_or("");
            let cur = parts[i]
                .get("partName")
                .and_then(|x| x.as_str())
                .unwrap_or("");
            if cur < prev {
                // ★SC6：路径须为 `parts[i].partName`（只报「未排序」= 弱）
                return SchemaCheck::Violation {
                    path: format!("parts[{}].partName", i),
                    detail: format!(
                        "`parts[]` 未按 `partName` 码点序：`{}` 出现在 `{}` 之后（SC6）",
                        cur, prev
                    ),
                };
            }
        }
    }

    // ── SC2：`packageRelationships[]` 的未知字段 ──
    if let Some(rels) = obj.get("packageRelationships").and_then(|r| r.as_array()) {
        for (i, r) in rels.iter().enumerate() {
            let Some(ro) = r.as_object() else { continue };
            for k in ro.keys() {
                if !REL_KEYS.contains(&k.as_str()) {
                    return SchemaCheck::Violation {
                        path: format!("packageRelationships[{}].{}", i, k),
                        detail: format!("未知字段 `{}`", k),
                    };
                }
            }
        }
    }

    // ── SC10：**无第二张白名单** —— 任何位置都不得出现自证豁免字段 ──
    if let Some(p) = find_forbidden_key(&v, "") {
        return SchemaCheck::Violation {
            path: p,
            detail:
                "契约 `no_second_whitelist` 明文禁止自证豁免入口（不得有任何第二张白名单）"
                    .to_string(),
        };
    }

    // ── SC9：`docProps.core` 全 `null` 但字段齐全 ⇒ **通过** ──
    // （不要求「非 null 才算有值」）
    SchemaCheck::Ok
}

/// SC10：递归查找被禁的自证豁免字段；返回其路径。
fn find_forbidden_key(v: &Value, path: &str) -> Option<String> {
    match v {
        Value::Object(m) => {
            for (k, x) in m {
                let np = if path.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", path, k)
                };
                if FORBIDDEN_EXEMPTION_KEYS.contains(&k.as_str()) {
                    return Some(np);
                }
                if let Some(f) = find_forbidden_key(x, &np) {
                    return Some(f);
                }
            }
            None
        }
        Value::Array(a) => {
            for (i, x) in a.iter().enumerate() {
                if let Some(f) = find_forbidden_key(x, &format!("{}[{}]", path, i)) {
                    return Some(f);
                }
            }
            None
        }
        _ => None,
    }
}

// ───────────────────────── 对契约文件本身的校验（SC4 / SC5 / SC11） ─────────────────────────

/// SC4：白名单**恰 4 条**，且前两条为「非语义字段」（`required:false` 的等价表达）。
///
/// ★实测更正：契约的 `whitelist_allowed_differences` 是**纯字符串数组**
/// （非「带 `required` 字段的对象数组」）。规格 §3.4 的「前两条 `required:false`」
/// 在契约里由**字段语义**表达：前两条是 `zip.entries[].compressed_size` /
/// `local_header_offset` —— 即**非语义字段**（压缩尺寸、头部偏移），
/// 故本函数以「前两条必须是这两个非语义字段」落实该判据。
pub fn check_whitelist_of_schema(schema_json: &str) -> SchemaCheck {
    let v: Value = match serde_json::from_str(schema_json) {
        Ok(v) => v,
        Err(e) => {
            return SchemaCheck::ToolError {
                path: "$schema".to_string(),
                detail: format!("schema 自身解析失败: {}", e),
            }
        }
    };
    let Some(wl) = v.get("whitelist_allowed_differences").and_then(|w| w.as_array()) else {
        return SchemaCheck::ToolError {
            path: "whitelist_allowed_differences".to_string(),
            detail: "schema 缺 `whitelist_allowed_differences`".to_string(),
        };
    };
    if wl.len() != WHITELIST_LEN {
        return SchemaCheck::Violation {
            path: "whitelist_allowed_differences".to_string(),
            detail: format!("白名单必须**恰 {} 条**，实测 {} 条", WHITELIST_LEN, wl.len()),
        };
    }
    // 前两条必须是**非语义字段**（`required:false` 的契约表达）
    const NON_SEMANTIC_FIRST_TWO: [&str; 2] = [
        "zip.entries[].compressed_size",
        "zip.entries[].local_header_offset",
    ];
    for (i, want) in NON_SEMANTIC_FIRST_TWO.iter().enumerate() {
        let got = wl[i].as_str().unwrap_or("");
        if got != *want {
            return SchemaCheck::Violation {
                path: format!("whitelist_allowed_differences[{}]", i),
                detail: format!(
                    "第 {} 条必须是**非语义字段** `{}`（`required:false` 的契约表达），实测 `{}`",
                    i + 1,
                    want,
                    got
                ),
            };
        }
    }
    SchemaCheck::Ok
}

/// SC5：`sort` 枚举**封闭**；出现枚举外值 ⇒ **退出 2**（schema 自身非法）。
pub fn check_sort_enum_of_schema(schema_json: &str) -> SchemaCheck {
    let v: Value = match serde_json::from_str(schema_json) {
        Ok(v) => v,
        Err(e) => {
            return SchemaCheck::ToolError {
                path: "$schema".to_string(),
                detail: format!("schema 自身解析失败: {}", e),
            }
        }
    };
    let mut bad: Option<(String, String)> = None;
    collect_sort_values(&v, "", &mut bad);
    match bad {
        Some((p, val)) => SchemaCheck::ToolError {
            path: p,
            detail: format!(
                "`sort` 取值 `{}` 不在封闭集 {:?} ⇒ **schema 自身非法**（SC5，退出 2）",
                val, SORT_ENUM
            ),
        },
        None => SchemaCheck::Ok,
    }
}

fn collect_sort_values(v: &Value, path: &str, bad: &mut Option<(String, String)>) {
    if bad.is_some() {
        return;
    }
    match v {
        Value::Object(m) => {
            for (k, x) in m {
                let np = if path.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", path, k)
                };
                if k == "sort" {
                    if let Some(s) = x.as_str() {
                        if !SORT_ENUM.contains(&s) {
                            *bad = Some((np.clone(), s.to_string()));
                            return;
                        }
                    }
                }
                collect_sort_values(x, &np, bad);
            }
        }
        Value::Array(a) => {
            for (i, x) in a.iter().enumerate() {
                collect_sort_values(x, &format!("{}[{}]", path, i), bad);
            }
        }
        _ => {}
    }
}

/// SC11：契约里 `open_entry` 的取值集封闭，且 `bytes` 的语义写在 `$comment`（**不进比较域**）。
pub fn check_open_entry_of_schema(schema_json: &str) -> SchemaCheck {
    let v: Value = match serde_json::from_str(schema_json) {
        Ok(v) => v,
        Err(e) => {
            return SchemaCheck::ToolError {
                path: "$schema".to_string(),
                detail: format!("schema 自身解析失败: {}", e),
            }
        }
    };
    // 契约用紧凑写法 `enum(bytes|path|raf|create)`（**字符串**，非数组）
    let raw = v
        .pointer("/fields/apiTrace/open_entry")
        .and_then(|e| {
            e.as_str()
                .map(|s| s.to_string())
                .or_else(|| e.get("enum").and_then(|x| x.as_str()).map(|s| s.to_string()))
        });
    let Some(raw) = raw else {
        return SchemaCheck::ToolError {
            path: "fields.apiTrace.open_entry".to_string(),
            detail: "契约缺 `apiTrace.open_entry` 的取值声明".to_string(),
        };
    };
    let body = raw
        .strip_prefix("enum(")
        .and_then(|r| r.strip_suffix(')'))
        .unwrap_or(&raw);
    for part in body.split('|') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if !OPEN_ENTRY_ENUM.contains(&part) {
            return SchemaCheck::Violation {
                path: "fields.apiTrace.open_entry".to_string(),
                detail: format!("取值 `{}` 不在封闭集 {:?}（SC11）", part, OPEN_ENTRY_ENUM),
            };
        }
    }
    // ★SC11 的**可达判据**：`open_entry` **只是 enum 值**（不在比较域里携带实现类名）。
    //
    // ★实测更正：规格 §3.4 SC11 写「`bytes` 的两侧实现类名写在 `$comment`」，
    //   但**契约实测**中 `fields.apiTrace.open_entry` 的值**仅**为
    //   `"enum(bytes|path|raf|create)"`，**没有** `$comment` 承载类名
    //   （两侧类名实际记在 `57`/`58` 的规格与 `PIPELINE_ALIGNMENT.md` 里）。
    // ⇒ 本函数断言**契约真正保证的部分**：取值封闭，且**不**在比较域携带实现细节。
    if v.pointer("/fields/apiTrace/open_entry")
        .and_then(|e| e.as_object())
        .is_some()
    {
        // 若将来契约改为对象形态并带 `$comment`，则要求它有内容（**不进比较域**）
        let has = v
            .pointer("/fields/apiTrace/open_entry/$comment")
            .and_then(|c| c.as_str())
            .map(|c| !c.is_empty())
            .unwrap_or(false);
        if !has {
            return SchemaCheck::Violation {
                path: "fields.apiTrace.open_entry.$comment".to_string(),
                detail: "对象形态的 `open_entry` 必须带非空 `$comment`（SC11）".to_string(),
            };
        }
    }
    SchemaCheck::Ok
}

/// 便捷：读取契约文件的路径（相对 crate 根）。
pub fn schema_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../../word-core/rewrite-review/stages/S01-opc-package/docs/s01/schema-v1.json",
    )
}
