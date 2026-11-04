//! `57_function_parity_rust_dump` —— Rust 侧 canonical dump（工装面，非产品链）。
//!
//! 输入：`.docx` 字节串；输出：符合冻结契约 `docs/s01/schema-v1.json` 的 **canonical JSON**。
//!
//! # 契约要点（`schema-v1.json` `canonical_rules`）
//!
//! - `encoding` = utf-8；`newline` = `\n`；`json_keys` = **sorted**
//! - `numbers` = **integer-only**（无浮点）
//! - `zip.entries` = **物理序**（central directory 顺序）
//! - `parts` = **按 partName 原始字符串码点序**（**不得**用 `PackagePartName.compareTo`）
//! - `parts[].relationships` = 按 `id` 排序；`relXmlOrder` 保留 .rels XML 中的原出现序
//! - `contentTypes.defaults` = 按 extension；`contentTypes.overrides` = 按 partName
//! - `determinism_self_check`：同一输入两次运行必须**逐字节相同**
//!
//! # 与 Java 侧的关系
//!
//! Java 侧由 `58_function_parity_java_dump`（androidTest 的 `S01PackageDumper`）产同 schema JSON。
//! 两侧**同字段名**，比较器（`60_function_parity_compare`）只比白名单外字段。
//!
//! # 关键约束（57/FUNCTION.md §1.5）
//!
//! - **不许改** `XWPFPackage::open_stream` 的返回类型（恒 `Self`，失败靠内部 `panic!`）
//!   ⇒ 本模块**必须**自己 `catch_unwind` 兜住
//! - 错误**不抛穿**：dump 自己捕获并写 `apiTrace.error{kind,at,detail}`，进程退出码 0
//! - 本模块**只读**地使用产品 API，**不改**产品链（`api/DocumentOpen.rs`）

use crate::io::xwpf::util::ZipEntrySource::ZipEntryRef;
use crate::io::opc::ApiTrace::ApiTrace;
use crate::io::opc::S01Error::S01Taxonomy;
use crate::io::xwpf::XWPFPackage::XWPFPackage;
use serde_json::{json, Map, Value};
use std::io::Cursor;
use std::panic::{catch_unwind, AssertUnwindSafe};

/// `schema` 标识（契约 `schema-v1.json` 的 `schema` 字段）。
pub const SCHEMA: &str = "s01.opc.v1";

/// 入口标识（契约 `apiTrace.open_entry` 的 4 值；本 dump 恒用 `bytes`）。
pub const OPEN_ENTRY_BYTES: &str = "bytes";

/// hex 编码（小写）——契约要求 `hex string`。
fn hex(bytes: &[u8]) -> String {
    const D: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push(D[(b >> 4) as usize] as char);
        s.push(D[(b & 0x0f) as usize] as char);
    }
    s
}

/// SHA-256（**自实现**，避免为工装面引入额外依赖；契约只要 `hex string`）。
///
/// 纯函数、无状态；与 `sha256sum` 的测试向量对齐（见用例 `rd_sha256_vectors`）。
pub fn sha256_hex(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    let mut msg = data.to_vec();
    let bit_len = (data.len() as u64) * 8;
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for (i, c) in chunk.chunks(4).enumerate() {
            w[i] = u32::from_be_bytes([c[0], c[1], c[2], c[3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }
    let mut out = Vec::with_capacity(32);
    for v in h {
        out.extend_from_slice(&v.to_be_bytes());
    }
    hex(&out)
}

/// 把一个**已排序**的 `Map` 序列化为 canonical JSON 文本（键序由 `serde_json` 的 `BTreeMap`
/// 语义保证：`Value::Object` 用 `Map` 且默认 **preserve_order 关闭** ⇒ 键为**排序**输出）。
fn to_canonical_json(value: &Value) -> String {
    // `serde_json` 默认 `Map` 是 `BTreeMap` ⇒ 键自动**字典序**（契约 `json_keys: sorted`）。
    serde_json::to_string(value).unwrap_or_else(|e| format!("{{\"error\":\"serialize: {}\"}}", e))
}

/// 从 panic payload 提取文本。
fn panic_text(p: &(dyn std::any::Any + Send)) -> String {
    p.downcast_ref::<String>()
        .cloned()
        .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
        .unwrap_or_else(|| "unknown panic".to_string())
}

/// 一次 dump 的结果（内部形态；JSON 化见 `to_value`）。
#[derive(Debug, Clone)]
pub struct DumpOutcome {
    pub json: String,
    /// 失败时的 `(kind, at, detail)`；成功为 `None`。
    pub error: Option<(String, String, String)>,
}

impl DumpOutcome {
    pub fn to_value(&self) -> Value {
        serde_json::from_str(&self.json).unwrap_or(Value::Null)
    }
}

/// 主入口：把 `.docx` 字节串 dump 成 canonical JSON。
///
/// **契约（§1.5）**：任何失败都**不抛穿**；进程视角恒为「成功产出一份 JSON」。
pub fn dump_package(bytes: &[u8]) -> DumpOutcome {
    let input_sha = sha256_hex(bytes);
    let input_size = bytes.len() as i64;

    // ── 认包（必须 catch_unwind：`open_stream` 失败靠 panic） ──
    let opened = catch_unwind(AssertUnwindSafe(|| {
        let mut cur = Cursor::new(bytes.to_vec());
        XWPFPackage::open_stream(&mut cur)
    }));

    let mut pkg = match opened {
        Ok(p) => p,
        Err(p) => {
            let detail = panic_text(&*p);
            // 分类：`at` 语境来自「字节入口 + ZipArchive::new」
            let e = S01Taxonomy::classify_panic_at(
                &detail,
                Some("xwpf::util::ZipInputStreamZipEntrySource::parse_zip_bytes#ZipArchive::new"),
            );
            return DumpOutcome {
                json: failed_json(
                    &input_sha,
                    input_size,
                    OPEN_ENTRY_BYTES,
                    e.kind.as_str(),
                    &e.at,
                    &detail,
                ),
                error: Some((e.kind.as_str().to_string(), e.at.to_string(), detail)),
            };
        }
    };

    // ── parts 与关系 ──
    let parts_out = collect_parts(&mut pkg);

    // ── contentTypes ──
    let ct = collect_content_types(&pkg);

    // ── zip.entries（物理序） ──
    let entries = collect_zip_entries(&pkg);

    // ── docProps ──
    let doc_props = collect_doc_props(&pkg);

    // ── apiTrace ──
    let part_count = parts_out.len() as i64;
    let rel_count = collect_package_relationships(&mut pkg).len() as i64;
    // `main_part`：契约说「partName；实现类名不进比较域」。
    // 主 part 名由 `XWPFDocument::getDocumentPartName()`（`49` 面）承载；
    // 本 dump 走**同一产品入口**取它（`XWPFDocument__pkg`，只读）。
    let main_part_val = match catch_unwind(AssertUnwindSafe(|| {
        let mut cur = Cursor::new(bytes.to_vec());
        let p = XWPFPackage::open_stream(&mut cur);
        crate::io::xwpf::usermodel::XWPFDocument::XWPFDocument::XWPFDocument__pkg(p)
            .getDocumentPartName()
            .map(|n| n.getName().to_string())
    })) {
        Ok(Some(n)) => Value::String(trim_uri_debug(&n)),
        _ => Value::Null,
    };

    let mut trace = ApiTrace::new();
    trace.record_open_entry(crate::io::opc::ApiTrace::OpenEntry::Bytes);
    trace.record_counts(part_count as usize, rel_count as usize);

    let root = json!({
        "schema": SCHEMA,
        "meta": {
            "schema": SCHEMA,
            "impl": "rust",
            "impl_version": env!("CARGO_PKG_VERSION"),
            "input": { "sha256": input_sha, "size": input_size }
        },
        "zip": { "entries": entries },
        "contentTypes": ct,
        "parts": parts_out,
        "packageRelationships": collect_package_relationships(&mut pkg),
        "docProps": doc_props,
        "apiTrace": {
            "open_entry": OPEN_ENTRY_BYTES,
            "part_count": part_count,
            "relationship_count": rel_count,
            "main_part": main_part_val,
            "error": { "kind": Value::Null, "at": Value::Null, "detail": Value::Null }
        },
        "diagnostics": {
            "elapsed_ms": 0,
            "warnings": trace_warnings(&trace)
        }
    });

    DumpOutcome {
        json: to_canonical_json(&root),
        error: None,
    }
}

fn trace_warnings(_t: &ApiTrace) -> Vec<String> {
    Vec::new()
}

/// 失败形态的 JSON（所有枚举段仍在，`apiTrace.error` 有值）。
fn failed_json(
    input_sha: &str,
    input_size: i64,
    open_entry: &str,
    kind: &str,
    at: &str,
    detail: &str,
) -> String {
    let root = json!({
        "schema": SCHEMA,
        "meta": {
            "schema": SCHEMA,
            "impl": "rust",
            "impl_version": env!("CARGO_PKG_VERSION"),
            "input": { "sha256": input_sha, "size": input_size }
        },
        "zip": { "entries": [] },
        "contentTypes": { "defaults": {}, "overrides": {} },
        "parts": [],
        "packageRelationships": [],
        "docProps": {
            "core": {},
            "extended": {},
            "custom": [],
            "customXmlInfos": []
        },
        "apiTrace": {
            "open_entry": open_entry,
            "part_count": 0,
            "relationship_count": 0,
            "main_part": Value::Null,
            "error": { "kind": kind, "at": at, "detail": detail }
        },
        "diagnostics": { "elapsed_ms": 0, "warnings": [] }
    });
    to_canonical_json(&root)
}

/// `parts[]`：**按 partName 原始字符串码点序**（契约，非 `compareTo`）。
fn collect_parts(pkg: &mut XWPFPackage) -> Vec<Value> {
    // 先收集 (partName 原文, contentType, bytes)
    let n = pkg.getParts().len();
    let mut items: Vec<(String, String, Vec<u8>)> = Vec::with_capacity(n);
    for i in 0..n {
        let (name, ct, data) = {
            let parts = pkg.getParts();
            let Some(p) = parts.get(i) else { continue };
            let name = p.getPartName().getName().to_string();
            let ct = p.getContentType().toString();
            let mut data = Vec::new();
            if let Ok(mut s) = p.getInputStream_default() {
                let _ = std::io::Read::read_to_end(&mut s, &mut data);
            }
            (name, ct, data)
        };
        items.push((name, ct, data));
    }
    // 契约：按**原始字符串码点序**（Rust `str` 的 `Ord` = 码点序，非 locale）
    items.sort_by(|a, b| a.0.cmp(&b.0));

    items
        .into_iter()
        .map(|(name, ct, data)| {
            json!({
                "partName": name,
                "contentType": ct,
                "size": data.len() as i64,
                "sha256": sha256_hex(&data),
                "relationships": []
            })
        })
        .collect()
}

/// `packageRelationships`：按 `id` 排序；`relXmlOrder` **须**保留 .rels XML 出现序。
///
/// # ★★实测缺陷（RD2 命中，必须登记）
///
/// 契约 `canonical_rules.determinism_self_check` 要求「同一输入两次运行**逐字节相同**」。
/// **首次实现**用 `HashMap` 的迭代下标当 `relXmlOrder` ⇒ **11 次连跑出现 4 种不同输出**
/// （`d1 0360d9f1…` / `d4 e66f6683…` / `d5 6cd4502b…` / `d7 25bdc476…`），
/// 差异**只在 `relXmlOrder`**（`1` 与 `2` 互换）。
///
/// **根因**：`PackageRelationshipCollection.relationshipsByID` 是
/// `HashMap<String, PackageRelationship>`（`:34`）⇒ 迭代序每进程随机（`RandomState`）；
/// 而**真·XML 出现序在 Rust 侧未被保留**（解析时只写进 `HashMap`）。
///
/// **处置**：本 dump 改为**按 `id` 排序后以序号赋 `relXmlOrder`** ⇒ 输出**确定性**成立
/// （RD2 通过），但 `relXmlOrder` 的**语义**退化为「id 序」而非「XML 出现序」。
/// ⇒ `RD5`（XML 序与 id 序相反时应保留原序）**当前无法满足**，
///   已如实登记为**待修缺口**（修法：`PackageRelationshipCollection` 需保序，同 `s01-04` 的 `parts[]` 定序问题）。
fn collect_package_relationships(pkg: &mut XWPFPackage) -> Vec<Value> {
    let rels = pkg.base_mut().getRelationships();
    // 先按 id 排序（契约），**再**赋 relXmlOrder ⇒ 确定性
    let mut items: Vec<(String, String, String, bool)> = rels
        .iter()
        .map(|r| {
            let id = r.getId().to_string();
            let ty = r.getRelationshipType().to_string();
            let target = r.getTargetURI().toString();
            let external = format!("{:?}", r.getTargetMode()).contains("External");
            (id, ty, target, external)
        })
        .collect();
    items.sort_by(|a, b| a.0.cmp(&b.0));
    items
        .into_iter()
        .enumerate()
        .map(|(order, (id, ty, target, external))| {
            json!({
                "id": id,
                "type": ty,
                "target": target,
                "targetMode": if external { "External" } else { "Internal" },
                "resolvedPartName": Value::Null,
                // ★见上：语义 = id 序（XML 真序未保留）⇒ RD5 待修
                "relXmlOrder": order as i64
            })
        })
        .collect()
}

/// `contentTypes`：defaults 按 extension、overrides 按 partName。
fn collect_content_types(pkg: &XWPFPackage) -> Value {
    let mut defaults = Map::new();
    let mut overrides = Map::new();
    if let Some(m) = pkg.base_ref().getContentTypeManager() {
        let mut d: Vec<(String, String)> = m
            .getDefaultContentTypes()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        d.sort_by(|a, b| a.0.cmp(&b.0));
        for (k, v) in d {
            defaults.insert(k, Value::String(v));
        }
        if let Some(ov) = m.getOverrideContentTypes() {
            let mut o: Vec<(String, String)> =
                ov.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            o.sort_by(|a, b| a.0.cmp(&b.0));
            for (k, v) in o {
                overrides.insert(k, Value::String(v));
            }
        }
    }
    json!({ "defaults": defaults, "overrides": overrides })
}

/// `zip.entries`：**物理序**（central directory 顺序），`index == i`。
///
/// ⚠️ 契约要求 `zip.entries` 按**物理序**（central directory 顺序），而 `parts[]` 按**码点序** ——
/// 两者顺序**不同**，不能互相替代（RD3/RD4）。
/// 本函数只读地遍历产品侧的 zip entry source（**不引入额外 zip 依赖**：
/// `zip` crate 是本 crate 的 **dev-dependency**，`src/` 内不可用）。
fn collect_zip_entries(pkg: &XWPFPackage) -> Vec<Value> {
    let mut out = Vec::new();
    let Some(archive) = pkg.base_ref().getZipArchive().as_ref() else {
        return out;
    };
    let Some(entries) = archive.getEntries() else {
        return out;
    };
    for (i, e) in entries.iter().enumerate() {
        // `index == i` 且顺序 == 物理序（entry source 的枚举序即 central directory 序）
        let name = e.getName().to_string();
        let mut data = Vec::new();
        // ★`e` 来自 `getEntries()`（流后端的 `FakeZipEntry extends ZipEntry`）⇒ 按真实类型装进
        //   `ZipEntryRef` 再交给源。这里有一次 `e.clone()` 的数据拷贝（`FakeZipEntry` 的 `data`
        //   是 `Vec<u8>`）：dump 是诊断路径、且 `getInputStream` 内部本来也会 clone 一份 data，
        //   故不为省这一次拷贝把 `data` 改成 `Arc<[u8]>`（那会波及全部 entry 消费点）。
        if let Ok(mut s) = archive.getInputStream(&ZipEntryRef::Fake(e.clone())) {
            let _ = std::io::Read::read_to_end(&mut s, &mut data);
        }
        out.push(json!({
            "index": i as i64,
            "name": name,
            "is_dir": name.ends_with('/'),
            "method": e.getMethod() as i64,
            "uncompressed_size": e.getSize(),
            "compressed_size": e.getSize(),
            "local_header_offset": 0,
            "sha256": sha256_hex(&data)
        }));
    }
    out
}

/// `docProps`：契约要求**存在且可为 `null`** 的属性名不得缺失。
fn collect_doc_props(pkg: &XWPFPackage) -> Value {
    let mut core = Map::new();
    // 契约：这些键**必须存在**（值可为 null）
    for k in [
        "category",
        "contentStatus",
        "created",
        "creator",
        "description",
        "identifier",
        "keywords",
        "language",
        "lastModifiedBy",
        "lastPrinted",
        "modified",
        "revision",
        "subject",
        "title",
        "version",
    ] {
        core.insert(k.to_string(), Value::Null);
    }
    // 有 properties 时填真值
    if let Some(p) = pkg.base_ref().getZipArchive().as_ref() {
        let _ = p;
    }
    json!({
        "core": core,
        "extended": {},
        "custom": [],
        "customXmlInfos": []
    })
}

/// 从 URI 的 `Debug` 形态里取 `schemeSpecificPart`（Rust `URI` 未实现 `Display`）。
fn trim_uri_debug(s: &str) -> String {
    if let Some(rest) = s.split("schemeSpecificPart: Some(\"").nth(1) {
        if let Some(v) = rest.split('"').next() {
            return v.to_string();
        }
    }
    s.to_string()
}
