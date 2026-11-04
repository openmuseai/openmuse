//! `59_function_parity_canonical` —— 把任意符合 schema 的 JSON 归一成**唯一**表示。
//!
//! 契约：`docs/s01/schema-v1.json` 的 `canonical_rules`。
//!
//! # 设计要点（规格 §3.4 CN1–CN11）
//!
//! - **幂等**（CN1/CN5/CN9）：`canonicalize(canonicalize(x)) == canonicalize(x)`
//! - **排序**（CN2/CN3/CN10）：`parts[]` 按 `partName` 码点序；`relationships[]` 按 `id`
//!   （但 `relXmlOrder` **不改**；`zip.entries[]` 按 `index` **不得重排**）
//! - **顺序违规必须报错**（CN4）：`zip.entries[].index` 不递增 ⇒ `sort_violation`（**不得静默重排**）
//! - **编码归一**（CN6）：去 BOM；`\r\n` → `\n`（**二选一并写死**：本实现选择「归一」）
//! - **整型化**（CN7）：`1.0` → `1`；**不允许**出现非整值浮点
//! - **未知字段报错**（CN8）：`unknown_field`
//! - **重复 key 报错**（CN11）：`parse`
//!
//! # 与 Java 侧的关系
//!
//! Java 侧 `58_function_parity_java_dump` 用**同一套规则**产 canonical JSON；
//! 比较器（`60`）只比白名单外的字段。

use serde_json::{Map, Value};

/// canonical 化失败的原因（规格 §1.3 的 `CanonError`）。
#[derive(Debug, Clone, PartialEq)]
pub struct CanonError {
    /// 错误类别：`parse` / `sort_violation` / `unknown_field` / `encoding` / `type_mismatch`
    pub kind: &'static str,
    /// 出错路径（JSON 路径形态，如 `zip.entries[3].index`）
    pub path: String,
    /// 诊断文本（**不参与比较**）
    pub detail: String,
}

impl CanonError {
    fn new(kind: &'static str, path: impl Into<String>, detail: impl Into<String>) -> Self {
        CanonError {
            kind,
            path: path.into(),
            detail: detail.into(),
        }
    }
}

/// 契约允许的顶层段（CN8：多一个即 `unknown_field`）。
const TOP_LEVEL_SECTIONS: [&str; 9] = [
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

/// 契约允许的**白名单**差异（`whitelist_allowed_differences`，共 4 项）。
pub const WHITELIST: [&str; 4] = [
    "zip.entries[].compressed_size",
    "zip.entries[].local_header_offset",
    "diagnostics.elapsed_ms",
    "meta.impl",
];

/// 主入口：把 `raw` 归一成**唯一**表示。
pub fn canonicalize(raw: &str) -> Result<String, CanonError> {
    // ── CN6：编码归一（去 BOM；`\r\n` → `\n`）——本实现选择「归一」而非「报错」 ──
    let cleaned = raw.strip_prefix('\u{feff}').unwrap_or(raw).replace("\r\n", "\n");

    // ── CN11：拒绝重复 key（`serde_json` 默认取**后值** ⇒ 必须自己检测） ──
    check_duplicate_keys(&cleaned)?;

    let v: Value = serde_json::from_str(&cleaned)
        .map_err(|e| CanonError::new("parse", "", format!("JSON 解析失败: {}", e)))?;

    // ── CN7：整型化（递归；`1.0` → `1`；非整值浮点 ⇒ `type_mismatch`） ──
    let v = integerize(v, "")?;

    // ── CN8：未知字段（顶层段 **+ 已知段内的未知键**） ──
    if let Value::Object(m) = &v {
        for k in m.keys() {
            if !TOP_LEVEL_SECTIONS.contains(&k.as_str()) {
                return Err(CanonError::new(
                    "unknown_field",
                    k.clone(),
                    format!("未知顶层段 `{}`", k),
                ));
            }
        }
        // `meta` 是闭合段（契约 `fields.meta` 只列 schema/impl/impl_version/input.*）
        if let Some(meta) = m.get("meta").and_then(|x| x.as_object()) {
            const META_KEYS: [&str; 5] =
                ["schema", "impl", "impl_version", "input", "input.sha256"];
            for k in meta.keys() {
                if !META_KEYS.contains(&k.as_str()) && k != "input" {
                    return Err(CanonError::new(
                        "unknown_field",
                        format!("meta.{}", k),
                        format!("`meta` 段内的未知键 `{}`（契约 `fields.meta` 为闭合集合）", k),
                    ));
                }
            }
        }
    }

    // ── CN4：`zip.entries[].index` 必须递增（**不得静默重排**） ──
    if let Some(z) = v.get("zip").and_then(|z| z.get("entries")).and_then(|e| e.as_array()) {
        for (i, e) in z.iter().enumerate() {
            if let Some(idx) = e.get("index").and_then(|x| x.as_i64()) {
                if idx != i as i64 {
                    return Err(CanonError::new(
                        "sort_violation",
                        format!("zip.entries[{}].index", i),
                        format!(
                            "`zip.entries[].index` 必须等于下标（实测 {} ≠ {}）；\
                             物理序差异不得被静默重排抹掉",
                            idx, i
                        ),
                    ));
                }
            }
        }
    }

    // ── CN2：`parts[]` 按 `partName` 码点序 ──
    let v = sort_parts(v);

    // ── CN3：`relationships[]` 按 `id`（`relXmlOrder` **不改**） ──
    let v = sort_relationships(v, "parts")?;
    let v = sort_relationships(v, "packageRelationships")?;

    // ── CN10：`contentTypes.defaults/overrides` 各自码点序 ──
    let v = sort_content_types(v);

    // 输出：`serde_json` 的 `Map` = `BTreeMap` ⇒ 键自动码点序（契约 `json_keys: sorted`）
    serde_json::to_string(&v)
        .map_err(|e| CanonError::new("parse", "", format!("序列化失败: {}", e)))
}

/// CN11：检测重复 key（`serde_json` 不报，需自己扫）。
fn check_duplicate_keys(s: &str) -> Result<(), CanonError> {
    // 极简扫描：只检测**同一对象内**同层重复 key。
    // 用栈记录每层的已见 key 集合（字符串解析需处理转义）。
    let b: Vec<char> = s.chars().collect();
    let mut seen: Vec<std::collections::HashSet<String>> = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        match b[i] {
            '{' => seen.push(std::collections::HashSet::new()),
            '}' => {
                seen.pop();
            }
            '"' => {
                // 读一个字符串
                let (text, next) = read_string(&b, i)?;
                i = next;
                // 判断它是否是一个 key：跳过空白后是 ':'
                let mut j = i;
                while j < b.len() && b[j].is_whitespace() {
                    j += 1;
                }
                if j < b.len() && b[j] == ':' {
                    if let Some(top) = seen.last_mut() {
                        if !top.insert(text.clone()) {
                            return Err(CanonError::new(
                                "parse",
                                text.clone(),
                                format!("重复 key `{}`（契约要求拒绝重复 key）", text),
                            ));
                        }
                    }
                }
                continue;
            }
            _ => {}
        }
        i += 1;
    }
    Ok(())
}

/// 读一个 JSON 字符串（含转义处理）；返回 (内容, 下一位置)。
fn read_string(b: &[char], start: usize) -> Result<(String, usize), CanonError> {
    debug_assert_eq!(b[start], '"');
    let mut out = String::new();
    let mut i = start + 1;
    while i < b.len() {
        match b[i] {
            '\\' => {
                i += 1;
                if i < b.len() {
                    match b[i] {
                        'n' => out.push('\n'),
                        't' => out.push('\t'),
                        'r' => out.push('\r'),
                        '"' => out.push('"'),
                        '\\' => out.push('\\'),
                        '/' => out.push('/'),
                        other => out.push(other),
                    }
                }
            }
            '"' => return Ok((out, i + 1)),
            c => out.push(c),
        }
        i += 1;
    }
    Err(CanonError::new("parse", "", "字符串未闭合"))
}

/// CN7：整型化（`1.0` → `1`；非整值 ⇒ `type_mismatch`）。
fn integerize(v: Value, path: &str) -> Result<Value, CanonError> {
    Ok(match v {
        Value::Number(n) => {
            if n.is_i64() || n.is_u64() {
                Value::Number(n)
            } else if let Some(f) = n.as_f64() {
                if f.fract() == 0.0 && f.is_finite() && f.abs() < 9.007_199_254_740_992e15 {
                    Value::Number(serde_json::Number::from(f as i64))
                } else {
                    return Err(CanonError::new(
                        "type_mismatch",
                        path.to_string(),
                        format!(
                            "契约 `numbers: integer-only` ⇒ 非整值浮点 `{}` 不允许",
                            f
                        ),
                    ));
                }
            } else {
                Value::Number(n)
            }
        }
        Value::Array(a) => {
            let mut out = Vec::with_capacity(a.len());
            for (i, x) in a.into_iter().enumerate() {
                out.push(integerize(x, &format!("{}[{}]", path, i))?);
            }
            Value::Array(out)
        }
        Value::Object(m) => {
            let mut out = Map::new();
            for (k, x) in m {
                let p = if path.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", path, k)
                };
                out.insert(k, integerize(x, &p)?);
            }
            Value::Object(out)
        }
        other => other,
    })
}

/// CN2：`parts[]` 按 `partName` **码点序**（Rust `str` 序 = 码点序，非 locale）。
fn sort_parts(mut v: Value) -> Value {
    if let Some(parts) = v.get_mut("parts").and_then(|p| p.as_array_mut()) {
        parts.sort_by(|a, b| {
            let ka = a.get("partName").and_then(|x| x.as_str()).unwrap_or("");
            let kb = b.get("partName").and_then(|x| x.as_str()).unwrap_or("");
            ka.cmp(kb)
        });
    }
    v
}

/// CN3：`relationships[]` 按 `id` 排序；`relXmlOrder` **保持原值**。
fn sort_relationships(mut v: Value, section: &str) -> Result<Value, CanonError> {
    if section == "parts" {
        if let Some(parts) = v.get_mut("parts").and_then(|p| p.as_array_mut()) {
            for p in parts.iter_mut() {
                if let Some(rels) = p.get_mut("relationships").and_then(|r| r.as_array_mut()) {
                    rels.sort_by(|a, b| {
                        let ka = a.get("id").and_then(|x| x.as_str()).unwrap_or("");
                        let kb = b.get("id").and_then(|x| x.as_str()).unwrap_or("");
                        ka.cmp(kb)
                    });
                }
            }
        }
    } else if let Some(rels) = v.get_mut(section).and_then(|r| r.as_array_mut()) {
        rels.sort_by(|a, b| {
            let ka = a.get("id").and_then(|x| x.as_str()).unwrap_or("");
            let kb = b.get("id").and_then(|x| x.as_str()).unwrap_or("");
            ka.cmp(kb)
        });
    }
    Ok(v)
}

/// CN10：`contentTypes.defaults` / `overrides` 各自按 key 码点序。
fn sort_content_types(mut v: Value) -> Value {
    if let Some(ct) = v.get_mut("contentTypes").and_then(|c| c.as_object_mut()) {
        for sub in ["defaults", "overrides"] {
            if let Some(m) = ct.get_mut(sub).and_then(|x| x.as_object_mut()) {
                // `serde_json` 的 `Map` 是 `BTreeMap` ⇒ 插入即有序；
                // 重新构造以确保（即便上游用了 preserve_order 也归一）
                let mut sorted: Vec<(String, Value)> =
                    m.iter().map(|(k, x)| (k.clone(), x.clone())).collect();
                sorted.sort_by(|a, b| a.0.cmp(&b.0));
                m.clear();
                for (k, x) in sorted {
                    m.insert(k, x);
                }
            }
        }
    }
    v
}
