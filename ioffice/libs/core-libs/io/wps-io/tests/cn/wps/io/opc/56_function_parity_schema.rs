// 56_function_parity_schema —— 以冻结契约为真源的校验器（SC1–SC11）
//
// 规格：`features/s01-10-parity-harness/functions/56_function_parity_schema/FUNCTION.md` §3.4。
// 契约：`docs/s01/schema-v1.json`（**已冻结**）。
// 实现：`src/cn/wps/io/opc/S01Schema.rs`（**新建**）。
// 退出码：0 = 通过；1 = schema 违规；2 = 工具/schema 自身错误。

use core_libs_io::io::opc::S01Schema::{
    check, check_open_entry_of_schema, check_sort_enum_of_schema, check_whitelist_of_schema,
    schema_path, SchemaCheck, SCHEMA_ID, WHITELIST_LEN,
};
use serde_json::json;

/// 读冻结契约。
fn schema_text() -> String {
    std::fs::read_to_string(schema_path()).expect("契约 schema-v1.json 必须存在且已冻结")
}

/// 一份符合契约的最小 dump JSON（**单行**，无 BOM、无 `\r\n`）。
fn minimal() -> String {
    json!({
        "schema": SCHEMA_ID,
        "meta": {"impl": "rust", "impl_version": "0.1.0",
                 "input": {"sha256": "aa", "size": 1}},
        "zip": {"entries": []},
        "contentTypes": {"defaults": {}, "overrides": {}},
        "parts": [],
        "packageRelationships": [],
        "docProps": {"core": {}, "extended": {}, "custom": [], "customXmlInfos": []},
        "apiTrace": {"open_entry": "bytes", "part_count": 0, "relationship_count": 0,
                     "main_part": null, "error": {"kind": null, "at": null, "detail": null}},
        "diagnostics": {"elapsed_ms": 0, "warnings": []}
    })
    .to_string()
}

fn with_parts(parts: serde_json::Value) -> String {
    let mut v: serde_json::Value = serde_json::from_str(&minimal()).unwrap();
    v["parts"] = parts;
    v.to_string()
}

fn part(name: &str) -> serde_json::Value {
    json!({"partName": name, "contentType": "ct", "size": 1, "sha256": "0", "relationships": []})
}

// ───────────────── SC1：全字段清单（主路径通过） ─────────────────
#[test]
fn sc1_main_path_ok() {
    let r = check(&minimal());
    assert_eq!(
        r,
        SchemaCheck::Ok,
        "★SC1：符合契约的最小 JSON 必须通过，实得 {:?}",
        r
    );
    assert_eq!(r.exit_code(), 0);
    // 真实 C1 dump 也应通过（端到端：57 的产物喂给 56）
    let bytes = std::fs::read(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx"),
    )
    .unwrap();
    let dump = core_libs_io::io::opc::Dump::dump_package(&bytes).json;
    let r2 = check(&dump);
    assert_eq!(
        r2,
        SchemaCheck::Ok,
        "★SC1：**真实 C1 的 dump 产物**必须通过契约校验，实得 {:?}",
        r2
    );
}

// ───────────────── SC2：未知字段 ⇒ 路径精确 ─────────────────
#[test]
fn sc2_unknown_field_reports_path() {
    let mut p = part("/word/a.xml");
    p["foo"] = json!(1);
    let raw = with_parts(json!([p]));
    let r = check(&raw);
    assert_eq!(r.exit_code(), 1, "★SC2：未知字段必须退出 1（静默忽略 = 失败）");
    match r {
        SchemaCheck::Violation { path, .. } => assert_eq!(
            path, "parts[0].foo",
            "★SC2：路径必须**精确**为 `parts[0].foo`，实得 {}",
            path
        ),
        other => panic!("应为 Violation，实得 {:?}", other),
    }
}

// ───────────────── SC3：缺必填 ⇒ 报该路径（非「未知字段」误判） ─────────────────
#[test]
fn sc3_missing_required_reports_its_path() {
    let mut v: serde_json::Value = serde_json::from_str(&minimal()).unwrap();
    v["meta"]["input"].as_object_mut().unwrap().remove("sha256");
    let r = check(&v.to_string());
    assert_eq!(r.exit_code(), 1, "★SC3：缺必填必须退出 1");
    match r {
        SchemaCheck::Violation { path, .. } => assert_eq!(
            path, "meta.input.sha256",
            "★SC3：须报必填字段**自己的路径**（报「未知字段」= 误判），实得 {}",
            path
        ),
        other => panic!("应为 Violation，实得 {:?}", other),
    }
}

// ───────────────── SC4：白名单恰 4 条 + 前两条 required:false ─────────────────
#[test]
fn sc4_whitelist_exact() {
    let r = check_whitelist_of_schema(&schema_text());
    assert_eq!(
        r,
        SchemaCheck::Ok,
        "★SC4：契约白名单必须**恰 {} 条**且前两条 `required:false`，实得 {:?}",
        WHITELIST_LEN,
        r
    );
    // 反向 ①：注入 5 条 ⇒ 必须报违规
    let mut v: serde_json::Value = serde_json::from_str(&schema_text()).unwrap();
    let wl = v["whitelist_allowed_differences"].as_array_mut().unwrap();
    let extra = wl[0].clone();
    wl.push(extra);
    assert_eq!(
        check_whitelist_of_schema(&v.to_string()).exit_code(),
        1,
        "★SC4：5 条必须报违规（5 条或 3 条 = 失败）"
    );
    // 反向 ②：删到 3 条 ⇒ 也必须报违规
    let mut v2: serde_json::Value = serde_json::from_str(&schema_text()).unwrap();
    v2["whitelist_allowed_differences"].as_array_mut().unwrap().pop();
    assert_eq!(
        check_whitelist_of_schema(&v2.to_string()).exit_code(),
        1,
        "★SC4：3 条也必须报违规"
    );
}

// ───────────────── SC5：sort 枚举封闭 ⇒ 非法值退出 2 ─────────────────
#[test]
fn sc5_sort_enum_closed() {
    let r = check_sort_enum_of_schema(&schema_text());
    assert_eq!(r, SchemaCheck::Ok, "★SC5：契约内所有 `sort` 值必须封闭，实得 {:?}", r);
    // 注入 `sort:"alphabetical"` ⇒ **schema 自身非法** ⇒ 退出 2
    let mut v: serde_json::Value = serde_json::from_str(&schema_text()).unwrap();
    v["fields"]["parts"]["sort"] = json!("alphabetical");
    let bad = check_sort_enum_of_schema(&v.to_string());
    assert_eq!(
        bad.exit_code(),
        2,
        "★SC5：`sort:\"alphabetical\"` 必须退出 **2**（schema 自身非法）；\
         当成合法排序 = 失败；实得 {:?}",
        bad
    );
}

// ───────────────── SC6：排序违例 ⇒ 路径精确到首个乱序元素 ─────────────────
#[test]
fn sc6_sort_violation_reports_index() {
    // `parts[]` 按 partName **降序**
    let raw = with_parts(json!([part("/word/z.xml"), part("/word/a.xml")]));
    let r = check(&raw);
    assert_eq!(r.exit_code(), 1, "★SC6：排序违例必须退出 1");
    match r {
        SchemaCheck::Violation { path, .. } => assert_eq!(
            path, "parts[1].partName",
            "★SC6：路径须**精确到首个乱序元素**（只报「未排序」而不给路径 = 弱），实得 {}",
            path
        ),
        other => panic!("应为 Violation，实得 {:?}", other),
    }
    // 正序必须通过
    let ok = with_parts(json!([part("/word/a.xml"), part("/word/z.xml")]));
    assert_eq!(check(&ok), SchemaCheck::Ok, "正序必须通过");
}

// ───────────────── SC7：BOM / \r\n ⇒ 路径 `$encoding`（不得容忍） ─────────────────
#[test]
fn sc7_encoding_violations() {
    // BOM
    let bom = format!("\u{feff}{}", minimal());
    match check(&bom) {
        SchemaCheck::Violation { path, .. } => assert_eq!(
            path, "$encoding",
            "★SC7：BOM 必须报 `$encoding`（容忍 = 失败），实得 {}",
            path
        ),
        other => panic!("★SC7：BOM 必须报违规（容忍 = 失败），实得 {:?}", other),
    }
    assert_eq!(check(&bom).exit_code(), 1);
    // CRLF
    let crlf = minimal().replace(',', ",\r\n");
    match check(&crlf) {
        SchemaCheck::Violation { path, .. } => assert_eq!(
            path, "$encoding",
            "★SC7：CRLF 必须报 `$encoding`，实得 {}",
            path
        ),
        other => panic!("★SC7：CRLF 必须报违规，实得 {:?}", other),
    }
}

// ───────────────── SC8：幂等 ─────────────────
#[test]
fn sc8_idempotent() {
    let raw = minimal();
    let a = check(&raw);
    let b = check(&raw);
    assert_eq!(a, b, "★SC8：同一 JSON 连检两次结果必须相同");
    // 报告也应 byte-identical（此处以 Debug 表示 + 退出码双验）
    assert_eq!(
        format!("{:?}", a),
        format!("{:?}", b),
        "★SC8：两次报告必须 byte-identical"
    );
    assert_eq!(a.exit_code(), b.exit_code());
    // 违规输入同样幂等
    let bad = with_parts(json!([part("/word/z.xml"), part("/word/a.xml")]));
    assert_eq!(check(&bad), check(&bad), "★SC8：违规输入也必须幂等");
}

// ───────────────── SC9：docProps 全 null 但字段齐全 ⇒ 通过 ─────────────────
#[test]
fn sc9_docprops_all_null_ok() {
    let mut v: serde_json::Value = serde_json::from_str(&minimal()).unwrap();
    v["docProps"]["core"] = json!({
        "creator": null, "created": null, "modified": null,
        "title": null, "subject": null, "description": null
    });
    let r = check(&v.to_string());
    assert_eq!(
        r,
        SchemaCheck::Ok,
        "★SC9：`docProps.core` 全 `null` 但字段齐全必须**通过**\
         （要求「非 null 才算有值」= 失败），实得 {:?}",
        r
    );
}

// ───────────────── SC10：无第二张白名单 ─────────────────
#[test]
fn sc10_no_second_whitelist() {
    // ① 契约自身不得含自证豁免字段
    let contract: serde_json::Value = serde_json::from_str(&schema_text()).unwrap();
    let mut hits = Vec::new();
    fn walk(v: &serde_json::Value, path: &str, out: &mut Vec<String>) {
        match v {
            serde_json::Value::Object(m) => {
                for (k, x) in m {
                    let np = if path.is_empty() {
                        k.clone()
                    } else {
                        format!("{}.{}", path, k)
                    };
                    if ["parity", "deferred", "allowed_diff", "ignore"].contains(&k.as_str()) {
                        out.push(np.clone());
                    }
                    walk(x, &np, out);
                }
            }
            serde_json::Value::Array(a) => {
                for (i, x) in a.iter().enumerate() {
                    walk(x, &format!("{}[{}]", path, i), out);
                }
            }
            _ => {}
        }
    }
    // ⚠️ 契约**顶层**的 `no_second_whitelist` 是**禁止性声明本身**（字符串），
    //    不是豁免入口 ⇒ 只扫描 `fields` 段内的自证豁免字段。
    walk(&contract["fields"], "fields", &mut hits);
    assert!(
        hits.is_empty(),
        "★SC10：契约内**不得**出现自证豁免字段（`no_second_whitelist`），实测命中 {:?}",
        hits
    );

    // ② 校验器必须拒绝任何豁免入口
    let mut v: serde_json::Value = serde_json::from_str(&minimal()).unwrap();
    v["parity"] = json!({"ignore": ["parts[0]"]});
    let r = check(&v.to_string());
    assert_eq!(
        r.exit_code(),
        1,
        "★SC10：任何豁免入口都必须被拒（不得有第二张白名单），实得 {:?}",
        r
    );
}

// ───────────────── SC11：open_entry 语义钉死 ─────────────────
#[test]
fn sc11_open_entry_semantics() {
    let r = check_open_entry_of_schema(&schema_text());
    assert_eq!(
        r,
        SchemaCheck::Ok,
        "★SC11：`open_entry` 取值须 ⊆ 封闭集，且不在比较域携带实现细节，实得 {:?}",
        r
    );
    // ★实测更正：契约里存的是 `"enum(bytes|path|raf|create)"` 紧凑字符串，
    //   且 **无** `$comment` 承载两侧实现类名（规格 SC11 的该半句与契约不符，已登记）。
    let contract: serde_json::Value = serde_json::from_str(&schema_text()).unwrap();
    assert_eq!(
        contract["fields"]["apiTrace"]["open_entry"].as_str(),
        Some("enum(bytes|path|raf|create)"),
        "契约的 `open_entry` 声明形态（实测）"
    );

    // 运行时校验：合法值通过
    for v in ["bytes", "path", "raf", "create"] {
        let mut m: serde_json::Value = serde_json::from_str(&minimal()).unwrap();
        m["apiTrace"]["open_entry"] = json!(v);
        assert_eq!(
            check(&m.to_string()),
            SchemaCheck::Ok,
            "`{}` 是封闭集成员，必须通过",
            v
        );
    }
    // 运行时校验：非法值必须被拒
    let mut m: serde_json::Value = serde_json::from_str(&minimal()).unwrap();
    m["apiTrace"]["open_entry"] = json!("stream");
    let bad = check(&m.to_string());
    assert_eq!(
        bad.exit_code(),
        1,
        "★SC11：`open_entry:\"stream\"` 不在封闭集，必须被拒，实得 {:?}",
        bad
    );
}

// ───────────────── 附加：契约文件本身存在且冻结 ─────────────────
#[test]
fn contract_file_is_frozen_and_readable() {
    let p = schema_path();
    assert!(p.exists(), "契约必须已入库：{:?}", p);
    let t = schema_text();
    let v: serde_json::Value = serde_json::from_str(&t).expect("契约必须是合法 JSON");
    assert_eq!(
        v["schema"].as_str().unwrap_or(""),
        SCHEMA_ID,
        "契约的 `schema` 必须与 `S01DumpSchema::SCHEMA_ID` 一致"
    );
    // 契约的 `no_second_whitelist` 必须存在（SC10 的明文依据）
    assert!(
        v["no_second_whitelist"].as_str().is_some(),
        "契约必须含 `no_second_whitelist` 明文"
    );
    // `error_compare` 必须恰为 (kind, at)
    let ec: Vec<String> = v["error_compare"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap().to_string())
        .collect();
    assert_eq!(ec, vec!["error.kind", "error.at"], "契约 `error_compare` 必须恰为这两项");
}
