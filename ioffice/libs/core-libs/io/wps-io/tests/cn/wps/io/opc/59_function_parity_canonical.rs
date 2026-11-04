// 59_function_parity_canonical —— canonical 归一（CN1–CN11）
//
// 规格：`features/s01-10-parity-harness/functions/59_function_parity_canonical/FUNCTION.md` §3.4。
// 契约：`docs/s01/schema-v1.json` 的 `canonical_rules`。
// 实现：`src/cn/wps/io/opc/S01Canonical.rs`（**新建**）。

use core_libs_io::io::opc::S01Canonical::canonicalize;

/// 最小合法输入（含契约要求的顶层段）。
fn ok(extra: &str) -> String {
    format!(
        r#"{{"schema":"s01.opc.v1","meta":{{"impl":"rust","impl_version":"0.1.0","input":{{"sha256":"aa","size":1}},"schema":"s01.opc.v1"}},"zip":{{"entries":[]}},"contentTypes":{{"defaults":{{}},"overrides":{{}}}},"parts":[],"packageRelationships":[],"docProps":{{"core":{{}},"extended":{{}},"custom":[],"customXmlInfos":[]}},"apiTrace":{{"open_entry":"bytes","part_count":0,"relationship_count":0,"main_part":null,"error":{{"kind":null,"at":null,"detail":null}}}},"diagnostics":{{"elapsed_ms":0,"warnings":[]}}{}}}"#,
        if extra.is_empty() {
            String::new()
        } else {
            format!(",{}", extra)
        }
    )
}

// ───────────────── CN1 / CN5 / CN9：幂等与确定性 ─────────────────
#[test]
fn cn1_cn5_cn9_idempotent_and_deterministic() {
    let raw = ok("");
    let a = canonicalize(&raw).expect("合法输入必须成功");
    // CN1：输出再喂一次 ⇒ 逐字节相同
    let b = canonicalize(&a).expect("输出必须仍是合法输入");
    assert_eq!(a, b, "★CN1：canonicalize 必须**幂等**（多/少一个换行 = 失败）");
    // CN5/CN9：两次 canonical 相同
    let c = canonicalize(&raw).unwrap();
    assert_eq!(a, c, "★CN5/CN9：同一输入两次 canonical 必须逐字节相同");
}

// ───────────────── CN2：parts[] 按 partName 码点序 ─────────────────
#[test]
fn cn2_parts_sorted_by_code_point() {
    let raw = ok("").replace(
        r#""parts":[]"#,
        r#""parts":[{"partName":"/word/z.xml","contentType":"a","size":0,"sha256":"0","relationships":[]},{"partName":"/word/a.xml","contentType":"a","size":0,"sha256":"0","relationships":[]}]"#,
    );
    let out = canonicalize(&raw).expect("必须成功");
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    let names: Vec<String> = v["parts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["partName"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(
        names,
        vec!["/word/a.xml", "/word/z.xml"],
        "★CN2：parts[] 必须按 partName **码点序**（别的顺序 = 失败）"
    );
}

// ───────────────── CN3：rels 按 id 排序，但 relXmlOrder 不变 ─────────────────
#[test]
fn cn3_relationships_sorted_by_id_keeping_rel_xml_order() {
    let raw = ok("").replace(
        r#""packageRelationships":[]"#,
        r#""packageRelationships":[{"id":"rId2","type":"t","target":"g","targetMode":"Internal","resolvedPartName":null,"relXmlOrder":0},{"id":"rId1","type":"t","target":"g","targetMode":"Internal","resolvedPartName":null,"relXmlOrder":1}]"#,
    );
    let out = canonicalize(&raw).expect("必须成功");
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    let rels = v["packageRelationships"].as_array().unwrap();
    assert_eq!(
        rels.iter().map(|r| r["id"].as_str().unwrap()).collect::<Vec<_>>(),
        vec!["rId1", "rId2"],
        "★CN3：relationships 必须按 `id` 排序"
    );
    // ★关键：`relXmlOrder` **保持原值**（rId1 原为 1、rId2 原为 0）
    assert_eq!(
        rels[0]["relXmlOrder"].as_i64(),
        Some(1),
        "★CN3：排序后 `rId1` 的 `relXmlOrder` 必须仍为 **1**（不得随排序重写）"
    );
    assert_eq!(
        rels[1]["relXmlOrder"].as_i64(),
        Some(0),
        "★CN3：排序后 `rId2` 的 `relXmlOrder` 必须仍为 **0**"
    );
}

// ───────────────── CN4：zip.entries 乱序 ⇒ sort_violation（不得静默重排） ─────────────────
#[test]
fn cn4_zip_entries_out_of_order_is_error() {
    // 把两个 entry 对调 ⇒ `index` 不再递增（0,1 → 1,0）
    let raw = ok("").replace(
        r#""zip":{"entries":[]}"#,
        r#""zip":{"entries":[{"index":1,"name":"b","is_dir":false,"method":0,"uncompressed_size":0,"compressed_size":0,"local_header_offset":0,"sha256":"0"},{"index":0,"name":"a","is_dir":false,"method":0,"uncompressed_size":0,"compressed_size":0,"local_header_offset":0,"sha256":"0"}]}"#,
    );
    let e = canonicalize(&raw).expect_err("★CN4：乱序必须报错（不得静默重排）");
    assert_eq!(
        e.kind, "sort_violation",
        "★CN4：kind 必须是 `sort_violation`，实得 {}",
        e.kind
    );
    assert!(
        e.path.contains("zip.entries"),
        "路径应指向 zip.entries，实得 {}",
        e.path
    );
}

// ───────────────── CN6：BOM + \r\n 归一 ─────────────────
#[test]
fn cn6_bom_and_crlf_normalized() {
    // ★实现时**二选一并写死**：本实现选择「归一」（去 BOM、\r\n → \n）
    let raw = format!("\u{feff}{}", ok("").replace(",", ",\r\n"));
    let out = canonicalize(&raw).expect("BOM/CRLF 必须被**归一**（本实现写死为归一）");
    assert!(
        !out.contains('\r'),
        "★CN6：归一后不得含 `\\r`（本实现行为写死为归一）"
    );
    assert!(
        !out.starts_with('\u{feff}'),
        "★CN6：归一后不得有 BOM"
    );
    // 归一结果应与「本就干净」的输入一致
    let clean = canonicalize(&ok("")).unwrap();
    assert_eq!(
        out, clean,
        "★CN6：含 BOM/CRLF 的输入归一后必须与干净输入**逐字节相同**"
    );
}

// ───────────────── CN7：整型化 ─────────────────
#[test]
fn cn7_integers_normalized() {
    // `size: 1.0` ⇒ `1`；`elapsed_ms: 1.5` 白名单允许但**仍须整型化** ⇒ 非整值必须报错
    let raw = ok("").replace(r#""size":1"#, r#""size":1.0"#);
    let out = canonicalize(&raw).expect("`1.0` 必须被整型化为 `1`");
    assert!(
        out.contains(r#""size":1"#),
        "★CN7：`1.0` 必须整型化为 `1`；实得片段：{}",
        &out[out.find("\"size\"").map(|i| i.saturating_sub(20)).unwrap_or(0)..][..40.min(out.len())]
    );
    // 非整值 ⇒ type_mismatch
    let raw2 = ok("").replace(r#""elapsed_ms":0"#, r#""elapsed_ms":1.5"#);
    let e = canonicalize(&raw2).expect_err("★CN7：非整值浮点必须报错（契约 integer-only）");
    assert_eq!(e.kind, "type_mismatch", "实得 {}", e.kind);
}

// ───────────────── CN8：未知顶层段 ─────────────────
#[test]
fn cn8_unknown_top_level_field() {
    let raw = ok(r#""extra":{"a":1}"#);
    let e = canonicalize(&raw).expect_err("★CN8：未知顶层段必须报错（忽略 = 失败）");
    assert_eq!(e.kind, "unknown_field", "实得 {}", e.kind);
    assert_eq!(e.path, "extra", "路径应为 `extra`，实得 {}", e.path);
}

// ───────────────── CN10：contentTypes 各自码点序 ─────────────────
#[test]
fn cn10_content_types_sorted() {
    let raw = ok("").replace(
        r#""contentTypes":{"defaults":{},"overrides":{}}"#,
        r#""contentTypes":{"defaults":{"z":"1","a":"2"},"overrides":{"/word/z.xml":"1","/word/a.xml":"2"}}"#,
    );
    let out = canonicalize(&raw).expect("必须成功");
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    let dk: Vec<&String> = v["contentTypes"]["defaults"].as_object().unwrap().keys().collect();
    assert_eq!(dk, vec!["a", "z"], "★CN10：defaults 必须按 extension 码点序");
    let ok_: Vec<&String> = v["contentTypes"]["overrides"].as_object().unwrap().keys().collect();
    assert_eq!(
        ok_,
        vec!["/word/a.xml", "/word/z.xml"],
        "★CN10：overrides 必须按 partName 码点序（不得与 extension 混排）"
    );
}

// ───────────────── CN11：重复 key ⇒ parse ─────────────────
#[test]
fn cn11_duplicate_key_rejected() {
    let raw = r#"{"a":1,"a":2}"#;
    let e = canonicalize(raw).expect_err("★CN11：重复 key 必须报错（取后值 = 失败）");
    assert_eq!(e.kind, "parse", "实得 {}", e.kind);
}

// ───────────────── 附加：canonical 输出的两条硬规则 ─────────────────
#[test]
fn output_is_single_line_and_keys_sorted() {
    let out = canonicalize(&ok("")).expect("必须成功");
    assert!(!out.contains('\n'), "canonical 输出必须为**单行**（契约 newline）");
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    let ks: Vec<&String> = v.as_object().unwrap().keys().collect();
    let mut s = ks.clone();
    s.sort();
    assert_eq!(ks, s, "契约 `json_keys: sorted` 必须成立");
}
