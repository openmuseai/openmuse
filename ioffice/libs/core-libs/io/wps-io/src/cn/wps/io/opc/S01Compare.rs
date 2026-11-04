//! `60_function_parity_compare` —— 按 schema 白名单逐叶子路径比较两份 canonical JSON。
//!
//! 契约：`docs/s01/schema-v1.json`。
//!
//! # 退出码（规格 §3.4）
//!
//! - `0` = 相等（含**仅白名单字段**不同的情形）
//! - `1` = 不等 ⇒ 产出**第一处**差异的完整证据（路径 + 两侧值 + 是否白名单）
//! - `2` = schema/工具问题（`tool_error` / `schema_violation` / `order_violation`）
//!
//! # 关键判据（规格 CP1–CP13）
//!
//! - **只报第一处**（CP13）且**两次运行报同一处**（确定性遍历序）
//! - `parts` 长度不同 ⇒ `kind=length_mismatch`，须报 `len` 与**下标**（CP4）
//! - 一侧**无**某键 vs 另一侧 `null` ⇒ **相等**（CP5，避免把「未实现」误判为差异）
//! - `"100"` vs `100` ⇒ `type_mismatch`（CP6，**不得**静默转类型）
//! - `meta.input.sha256` 不同 ⇒ 直接判不等并提示「同输入前提被破坏」（CP9）
//! - `error.detail` 不同 ⇒ **相等**（CP10，文本不参与比较域）
//! - `parts[]` 逆序 ⇒ `order_violation`（CP11，比较器**不自己排序**）

use crate::io::opc::S01Canonical::{canonicalize, CanonError, WHITELIST};
use serde_json::Value;

/// 比较结果（规格 §1.3）。
#[derive(Debug, Clone, PartialEq)]
pub enum CompareOutcome {
    /// 相等（`exit 0`）
    Equal,
    /// 不等（`exit 1`）：第一处差异
    FirstDiff(FirstDiff),
    /// schema/工具问题（`exit 2`）
    ToolError(CanonError),
}

impl CompareOutcome {
    /// 规格的退出码契约。
    pub fn exit_code(&self) -> i32 {
        match self {
            CompareOutcome::Equal => 0,
            CompareOutcome::FirstDiff(_) => 1,
            CompareOutcome::ToolError(_) => 2,
        }
    }
}

/// 第一处差异的完整证据。
#[derive(Debug, Clone, PartialEq)]
pub struct FirstDiff {
    /// JSON 路径（如 `parts[3].relationships[1].targetMode`）
    pub path: String,
    /// 差异类别：`value_mismatch` / `length_mismatch` / `type_mismatch` / `missing_key`
    pub kind: &'static str,
    /// Rust 侧值（已 JSON 化）
    pub rust: String,
    /// Java 侧值（已 JSON 化）
    pub java: String,
    /// 该路径是否命中白名单
    pub whitelisted: bool,
}

impl FirstDiff {
    /// 规格要求的可读报告（**含 WHITELIST 标记**，CP3）。
    pub fn report(&self) -> String {
        let mark = if self.whitelisted { "WHITELIST" } else { "DIFF" };
        format!(
            "{} {} {} kind={} rust={} java={}",
            mark, self.path, self.kind, self.kind, self.rust, self.java
        )
    }
}

/// 主入口（规格 §1.1）：`compare(rust, java) -> Result<(), FirstDiff>` 的扩展形态。
pub fn compare(rust_json: &str, java_json: &str) -> CompareOutcome {
    // ── ★CP11：顺序违规必须在 **canonical 之前**判定 ──
    //
    // 关键：`canonicalize` 会**主动排序** `parts[]`（它的职责就是归一）。
    // 若等 canonical 之后再查顺序，逆序输入已被静默排好 ⇒ **永远发现不了**
    // （本实现首版正是如此：CP11 实测返回 `Equal`，即规格所说的**
    // 「比较器自己排序后退出 0 = 严重失败**」）。
    // ⇒ 因此必须先在**原始输入**上检查顺序。
    for (label, raw) in [("rust", rust_json), ("java", java_json)] {
        if let Ok(v) = serde_json::from_str::<Value>(
            raw.strip_prefix('\u{feff}').unwrap_or(raw),
        ) {
            if let Some(e) = check_order(&v) {
                return CompareOutcome::ToolError(CanonError {
                    kind: "order_violation",
                    path: e.path,
                    detail: format!("{} 侧：{}", label, e.detail),
                });
            }
        }
    }

    // ── CP7：输入不存在/非法 ⇒ exit 2（tool_error） ──
    let r = match canonicalize(rust_json) {
        Ok(s) => s,
        Err(e) => {
            return CompareOutcome::ToolError(CanonError {
                kind: "tool_error",
                path: e.path,
                detail: format!("rust 侧输入非法: {}", e.detail),
            })
        }
    };
    let j = match canonicalize(java_json) {
        Ok(s) => s,
        Err(e) => {
            return CompareOutcome::ToolError(CanonError {
                kind: "tool_error",
                path: e.path,
                detail: format!("java 侧输入非法: {}", e.detail),
            })
        }
    };

    let rv: Value = match serde_json::from_str(&r) {
        Ok(v) => v,
        Err(e) => {
            return CompareOutcome::ToolError(CanonError {
                kind: "tool_error",
                path: String::new(),
                detail: format!("rust 侧再解析失败: {}", e),
            })
        }
    };
    let jv: Value = match serde_json::from_str(&j) {
        Ok(v) => v,
        Err(e) => {
            return CompareOutcome::ToolError(CanonError {
                kind: "tool_error",
                path: String::new(),
                detail: format!("java 侧再解析失败: {}", e),
            })
        }
    };

    // ── CP11：顺序违规由 canonical **前置**报出 ⇒ 此处若发现逆序须 exit 2 ──
    if let Some(order_err) = check_order(&rv) {
        return CompareOutcome::ToolError(order_err);
    }
    if let Some(order_err) = check_order(&jv) {
        return CompareOutcome::ToolError(order_err);
    }

    // ── CP9：`meta.input.sha256` 不同 ⇒ 同输入前提被破坏 ──
    let rs = rv.pointer("/meta/input/sha256").and_then(|v| v.as_str());
    let js = jv.pointer("/meta/input/sha256").and_then(|v| v.as_str());
    if rs.is_some() && js.is_some() && rs != js {
        return CompareOutcome::FirstDiff(FirstDiff {
            path: "meta.input.sha256".to_string(),
            kind: "value_mismatch",
            rust: rs.unwrap_or("").to_string(),
            java: js.unwrap_or("").to_string(),
            whitelisted: false,
        });
    }

    // ── 逐叶子比较（确定性遍历序 ⇒ CP12/CP13 成立） ──
    match walk(&rv, &jv, "", &mut Vec::new()) {
        Some(d) => {
            if d.whitelisted {
                CompareOutcome::Equal
            } else {
                CompareOutcome::FirstDiff(d)
            }
        }
        None => CompareOutcome::Equal,
    }
}

/// CP11：`parts[]` / `zip.entries[]` 必须已按契约有序（比较器**不**自己排序）。
fn check_order(v: &Value) -> Option<CanonError> {
    if let Some(parts) = v.get("parts").and_then(|p| p.as_array()) {
        let names: Vec<&str> = parts
            .iter()
            .map(|p| p.get("partName").and_then(|x| x.as_str()).unwrap_or(""))
            .collect();
        let mut sorted = names.clone();
        sorted.sort();
        if names != sorted {
            return Some(CanonError {
                kind: "order_violation",
                path: "parts[]".to_string(),
                detail: "`parts[]` 未按 partName 码点序 ⇒ 应由 canonical 前置归一（比较器不得自己排序）"
                    .to_string(),
            });
        }
    }
    if let Some(z) = v.get("zip").and_then(|z| z.get("entries")).and_then(|e| e.as_array()) {
        for (i, e) in z.iter().enumerate() {
            if let Some(idx) = e.get("index").and_then(|x| x.as_i64()) {
                if idx != i as i64 {
                    return Some(CanonError {
                        kind: "order_violation",
                        path: format!("zip.entries[{}].index", i),
                        detail: "`zip.entries[].index` 必须等于下标（物理序不得重排）".to_string(),
                    });
                }
            }
        }
    }
    None
}

/// 递归比较：返回**第一处**非白名单差异（若有）。
///
/// 遍历顺序：对象键按码点序（`BTreeMap`）、数组按**下标升序** ⇒ 确定性（CP12/CP13）。
fn walk(r: &Value, j: &Value, path: &str, whitelist_stack: &mut Vec<bool>) -> Option<FirstDiff> {
    // CP5：一侧**缺键** vs 另一侧 `null` ⇒ 视为相等
    match (r, j) {
        (Value::Null, Value::Null) => return None,
        _ => {}
    }

    // CP6：类型不同 ⇒ type_mismatch（**不得**静默转类型）
    if std::mem::discriminant(r) != std::mem::discriminant(j) {
        // 特例（CP5）：`null` vs 缺失由调用方处理；此处是**两侧都存在但类型不同**
        let wl = is_whitelisted(path);
        return Some(FirstDiff {
            path: path.to_string(),
            kind: "type_mismatch",
            rust: r.to_string(),
            java: j.to_string(),
            whitelisted: wl,
        });
    }

    match (r, j) {
        (Value::Object(rm), Value::Object(jm)) => {
            // 键的并集，按码点序（确定性）
            let mut keys: Vec<&String> = rm.keys().chain(jm.keys()).collect();
            keys.sort();
            keys.dedup();
            for k in keys {
                let np = if path.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", path, k)
                };
                match (rm.get(k), jm.get(k)) {
                    // CP5：一侧无该键、另一侧 null ⇒ 相等（不报差异）
                    (None, Some(Value::Null)) | (Some(Value::Null), None) => continue,
                    (None, Some(v)) | (Some(v), None) => {
                        // 真·缺键（另一侧非 null）⇒ missing_key
                        let wl = is_whitelisted(&np);
                        if wl {
                            continue;
                        }
                        return Some(FirstDiff {
                            path: np.clone(),
                            kind: "missing_key",
                            rust: rm.get(k).map(|x| x.to_string()).unwrap_or_else(|| "<absent>".into()),
                            java: jm.get(k).map(|x| x.to_string()).unwrap_or_else(|| "<absent>".into()),
                            whitelisted: wl,
                        });
                    }
                    (Some(rv), Some(jv)) => {
                        if let Some(d) = walk(rv, jv, &np, whitelist_stack) {
                            return Some(d);
                        }
                    }
                    // 两侧都无该键（并集去重后不应出现，保守跳过）
                    (None, None) => continue,
                }
            }
            None
        }
        // CP4：数组长度不同 ⇒ length_mismatch，须报 len 与**下标**
        (Value::Array(ra), Value::Array(ja)) => {
            if ra.len() != ja.len() {
                let np = format!("{}[len]", path);
                return Some(FirstDiff {
                    path: np,
                    kind: "length_mismatch",
                    rust: format!("len={}", ra.len()),
                    java: format!("len={}", ja.len()),
                    whitelisted: is_whitelisted(path),
                });
            }
            for (i, (rv, jv)) in ra.iter().zip(ja.iter()).enumerate() {
                let np = format!("{}[{}]", path, i);
                if let Some(d) = walk(rv, jv, &np, whitelist_stack) {
                    return Some(d);
                }
            }
            None
        }
        _ => {
            // CP10：`error.detail` 不参与比较域（文本不可比）
            if path.ends_with("error.detail") {
                return None;
            }
            if r == j {
                return None;
            }
            let wl = is_whitelisted(path);
            Some(FirstDiff {
                path: path.to_string(),
                kind: "value_mismatch",
                rust: r.to_string(),
                java: j.to_string(),
                whitelisted: wl,
            })
        }
    }
}

/// 该路径是否命中契约白名单（`whitelist_allowed_differences`）。
pub fn is_whitelisted(path: &str) -> bool {
    for w in WHITELIST {
        // `zip.entries[].compressed_size` 这类模板 → 转成前缀匹配
        if let Some(prefix) = w.strip_suffix("[]") {
            if path.starts_with(prefix) {
                return true;
            }
        } else if w.contains("[]") {
            // `zip.entries[].compressed_size`
            let parts: Vec<&str> = w.split("[].").collect();
            if parts.len() == 2 && path.starts_with(parts[0]) && path.ends_with(parts[1]) {
                return true;
            }
        } else if path == w {
            return true;
        }
    }
    false
}
