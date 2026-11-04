// 60_function_parity_compare —— 逐叶子路径比较器（CP1–CP13）
//
// 规格：`features/s01-10-parity-harness/functions/60_function_parity_compare/FUNCTION.md` §3.4。
// 实现：`src/cn/wps/io/opc/S01Compare.rs`（**新建**）。
//
// 退出码契约：`0` = 相等（含仅白名单不同）；`1` = 第一处差异；`2` = schema/工具问题。

use core_libs_io::io::opc::S01Compare::{compare, CompareOutcome};

/// 最小合法 dump JSON（契约全段）。
fn doc(parts: &str, extra_meta: &str, error_detail: &str, elapsed: i64) -> String {
    format!(
        r#"{{"schema":"s01.opc.v1","meta":{{"impl":"rust","impl_version":"0.1.0"{extra_meta},"input":{{"sha256":"aa","size":1}},"schema":"s01.opc.v1"}},"zip":{{"entries":[]}},"contentTypes":{{"defaults":{{}},"overrides":{{}}}},"parts":{parts},"packageRelationships":[],"docProps":{{"core":{{}},"extended":{{}},"custom":[],"customXmlInfos":[]}},"apiTrace":{{"open_entry":"bytes","part_count":0,"relationship_count":0,"main_part":null,"error":{{"kind":null,"at":null,"detail":{error_detail}}}}},"diagnostics":{{"elapsed_ms":{elapsed},"warnings":[]}}}}"#
    )
}

fn part(name: &str, target_mode: &str) -> String {
    format!(
        r#"{{"partName":"{name}","contentType":"ct","size":1,"sha256":"0","relationships":[{{"id":"rId1","type":"t","target":"g","targetMode":"{target_mode}","resolvedPartName":null,"relXmlOrder":0}}]}}"#
    )
}

fn code(r: &str, j: &str) -> (i32, CompareOutcome) {
    let o = compare(r, j);
    (o.exit_code(), o)
}

// ───────────────── CP1：同一份 ⇒ 退出 0 ─────────────────
#[test]
fn cp1_identical_exit_0() {
    let d = doc(&format!("[{}]", part("/word/a.xml", "Internal")), "", "null", 0);
    let (c, o) = code(&d, &d);
    assert_eq!(c, 0, "★CP1：同一份必须退出 0，实得 {:?}", o);
}

// ───────────────── CP2：字段级差异 ⇒ 退出 1 且路径精确 ─────────────────
#[test]
fn cp2_field_level_diff_path_precise() {
    let r = doc(&format!("[{}]", part("/word/a.xml", "Internal")), "", "null", 0);
    let j = doc(&format!("[{}]", part("/word/a.xml", "External")), "", "null", 0);
    let (c, o) = code(&r, &j);
    assert_eq!(c, 1, "★CP2：必须退出 1");
    match o {
        CompareOutcome::FirstDiff(d) => {
            assert_eq!(
                d.path, "parts[0].relationships[0].targetMode",
                "★CP2：路径必须**精确到该串**（报 `parts` 或「不一致」= 弱，不可定位）"
            );
            assert_eq!(d.kind, "value_mismatch");
            assert!(!d.whitelisted, "该字段不在白名单");
        }
        other => panic!("应为 FirstDiff，实得 {:?}", other),
    }
}

// ───────────────── CP3：仅白名单字段不同 ⇒ 退出 0 + WHITELIST ─────────────────
#[test]
fn cp3_whitelisted_only_exit_0() {
    // 仅 `meta.impl`（白名单 ④）与 `diagnostics.elapsed_ms`（白名单 ③）不同
    // ⚠️ 契约 `fields.meta` 是**闭合集合** ⇒ 不得塞入 `impl_extra` 之类的假键
    //    （那会被 CN8 判为 `unknown_field` ⇒ 退出 2，与 CP3 的意图无关）
    let r = doc("[]", "", "null", 1);
    let j = {
        let s = doc("[]", "", "null", 999);
        s.replace(r#""impl":"rust""#, r#""impl":"java""#)
    };
    let (c, o) = code(&r, &j);
    assert_eq!(
        c, 0,
        "★CP3：**仅**白名单字段不同必须退出 0（退出 1 = 白名单失效）；实得 {:?}",
        o
    );
    // 白名单识别本身
    use core_libs_io::io::opc::S01Compare::is_whitelisted;
    assert!(is_whitelisted("meta.impl"), "`meta.impl` 必须在白名单");
    assert!(
        is_whitelisted("diagnostics.elapsed_ms"),
        "`diagnostics.elapsed_ms` 必须在白名单"
    );
    assert!(
        is_whitelisted("zip.entries[0].compressed_size"),
        "`zip.entries[].compressed_size` 必须在白名单"
    );
    assert!(
        !is_whitelisted("parts[0].partName"),
        "`parts[0].partName` **不得**在白名单（任何「暂不等」入口都不许）"
    );
}

// ───────────────── CP4：数组长度不同 ⇒ length_mismatch + 报 len 与下标 ─────────────────
#[test]
fn cp4_length_mismatch_reports_len() {
    let r = doc(
        &format!(
            "[{},{}]",
            part("/word/a.xml", "Internal"),
            part("/word/b.xml", "Internal")
        ),
        "",
        "null",
        0,
    );
    let j = doc(&format!("[{}]", part("/word/a.xml", "Internal")), "", "null", 0);
    let (c, o) = code(&r, &j);
    assert_eq!(c, 1, "★CP4：长度不同必须退出 1");
    match o {
        CompareOutcome::FirstDiff(d) => {
            assert_eq!(d.kind, "length_mismatch", "★CP4：kind 必须是 length_mismatch");
            assert!(
                d.rust.contains("len=2") && d.java.contains("len=1"),
                "★CP4：必须报 **len**（只报「长度不同」而无下标/长度 = 弱）；实得 rust={} java={}",
                d.rust,
                d.java
            );
            assert!(
                d.path.contains("parts"),
                "路径应体现数组位置，实得 {}",
                d.path
            );
        }
        other => panic!("应为 FirstDiff，实得 {:?}", other),
    }
}

// ───────────────── CP5：一侧无键 vs 另一侧 null ⇒ 退出 0 ─────────────────
#[test]
fn cp5_absent_key_vs_null_is_equal() {
    // 一侧有 `docProps.core` 的 `creator:null`，另一侧**无**该键
    let r = doc("[]", "", "null", 0).replace(r#""core":{}"#, r#""core":{"creator":null}"#);
    let j = doc("[]", "", "null", 0);
    let (c, o) = code(&r, &j);
    assert_eq!(
        c, 0,
        "★CP5：一侧无键、另一侧 null 必须**相等**（退出 1 = 会把「未实现」误判为差异）；实得 {:?}",
        o
    );
}

// ───────────────── CP6：`"100"` vs `100` ⇒ type_mismatch ─────────────────
#[test]
fn cp6_type_mismatch_not_coerced() {
    let r = doc("[]", "", "null", 0).replace(r#""size":1"#, r#""size":"100""#);
    let j = doc("[]", "", "null", 0).replace(r#""size":1"#, r#""size":100"#);
    let (c, o) = code(&r, &j);
    assert_eq!(c, 1, "★CP6：类型不同必须退出 1");
    match o {
        CompareOutcome::FirstDiff(d) => {
            assert_eq!(
                d.kind, "type_mismatch",
                "★CP6：kind 必须是 type_mismatch（静默转类型 = **严重失败**）"
            );
        }
        other => panic!("应为 FirstDiff，实得 {:?}", other),
    }
}

// ───────────────── CP7：输入非法/不存在 ⇒ 退出 2 tool_error ─────────────────
#[test]
fn cp7_bad_input_exit_2() {
    let good = doc("[]", "", "null", 0);
    // 不存在的文件在 CLI 层处理；此处用非法 JSON 代表工具错误
    let (c, o) = code("{", &good);
    assert_eq!(
        c, 2,
        "★CP7：非法输入必须退出 2（退出 1 = CI 会误以为实现不一致）；实得 {:?}",
        o
    );
    match o {
        CompareOutcome::ToolError(e) => assert_eq!(e.kind, "tool_error"),
        other => panic!("应为 ToolError，实得 {:?}", other),
    }
    let (c2, _) = code(&good, "{");
    assert_eq!(c2, 2, "★CP7：java 侧非法同样退出 2");
}

// ───────────────── CP8：schema 违规 ⇒ 退出 2 schema_violation ─────────────────
#[test]
fn cp8_unknown_field_is_schema_violation() {
    let good = doc("[]", "", "null", 0);
    // 多一个顶层段
    let bad = doc("[]", r#","foo":"bar""#, "null", 0);
    let (c, o) = code(&bad, &good);
    assert_eq!(
        c, 2,
        "★CP8：schema 违规必须退出 2（退出 1 = 失败）；实得 {:?}",
        o
    );
    match o {
        CompareOutcome::ToolError(e) => {
            // 顶层未知段在 canonical 层报 `unknown_field`；比较器包成 tool_error（退出 2）
            assert_eq!(e.kind, "tool_error", "实得 {}", e.kind);
        }
        other => panic!("应为 ToolError，实得 {:?}", other),
    }
}

// ───────────────── CP9：sha256 不同 ⇒ 退出 1 + 路径 + 前提破坏 ─────────────────
#[test]
fn cp9_input_sha_mismatch() {
    let r = doc("[]", "", "null", 0);
    let j = doc("[]", "", "null", 0).replace(r#""sha256":"aa""#, r#""sha256":"bb""#);
    let (c, o) = code(&r, &j);
    assert_eq!(c, 1, "★CP9：sha256 不同必须退出 1");
    match o {
        CompareOutcome::FirstDiff(d) => {
            assert_eq!(
                d.path, "meta.input.sha256",
                "★CP9：路径必须为 `meta.input.sha256`"
            );
        }
        other => panic!("应为 FirstDiff，实得 {:?}", other),
    }
}

// ───────────────── CP10：`error.detail` 不同 ⇒ 退出 0 ─────────────────
#[test]
fn cp10_error_detail_not_compared() {
    let r = doc("[]", "", r#""detail-rust""#, 0);
    let j = doc("[]", "", r#""detail-java""#, 0);
    let (c, o) = code(&r, &j);
    assert_eq!(
        c, 0,
        "★CP10：`error.detail` 不同必须退出 0（因 detail 退出 1 = 失败，文本不可比）；实得 {:?}",
        o
    );
}

// ───────────────── CP11：parts[] 逆序 ⇒ 退出 2 order_violation（不得自己排序） ─────────────────
#[test]
fn cp11_order_violation_exit_2() {
    // 构造逆序的 rust.json（z 在 a 之前）
    let reversed = doc(
        &format!(
            "[{},{}]",
            part("/word/z.xml", "Internal"),
            part("/word/a.xml", "Internal")
        ),
        "",
        "null",
        0,
    );
    let good = doc(
        &format!(
            "[{},{}]",
            part("/word/a.xml", "Internal"),
            part("/word/z.xml", "Internal")
        ),
        "",
        "null",
        0,
    );
    let (c, o) = code(&reversed, &good);
    assert_eq!(
        c, 2,
        "★CP11：`parts[]` 逆序必须退出 2（`order_violation`）；\
         比较器自己排序后退出 0 = **严重失败**；实得 {:?}",
        o
    );
    match o {
        CompareOutcome::ToolError(e) => assert_eq!(e.kind, "order_violation", "实得 {}", e.kind),
        other => panic!("应为 ToolError，实得 {:?}", other),
    }
}

// ───────────────── CP12 / CP13：确定性与「只报第一处」 ─────────────────
#[test]
fn cp12_cp13_deterministic_first_diff_only() {
    // 同时存在 3 处差异：meta.impl_version、parts[0].contentType、apiTrace.part_count
    let r = doc(&format!("[{}]", part("/word/a.xml", "Internal")), "", "null", 0);
    let j = doc(&format!("[{}]", part("/word/a.xml", "Internal")), "", "null", 0)
        .replace(r#""impl_version":"0.1.0""#, r#""impl_version":"9.9.9""#)
        .replace(r#""contentType":"ct""#, r#""contentType":"OTHER""#)
        .replace(r#""part_count":0"#, r#""part_count":42"#);

    let o1 = compare(&r, &j);
    // ★CP12：连跑两次结果逐字节相同
    let o2 = compare(&r, &j);
    assert_eq!(o1, o2, "★CP12：两次比较必须逐字节相同（差异 = 遍历序不确定）");
    assert_eq!(o1.exit_code(), 1);
    // ★CP13：只报**第一处**
    match &o1 {
        CompareOutcome::FirstDiff(d) => {
            assert_eq!(
                d.kind, "value_mismatch",
                "★CP13：应报第一处 value_mismatch"
            );
            assert!(
                !d.path.is_empty() && !d.path.contains(','),
                "★CP13：只报**一处**（报多处/随机一处 = 失败）；实得路径 {}",
                d.path
            );
            // 报告须可读且带标记
            let rep = d.report();
            assert!(rep.contains("DIFF"), "报告须含 DIFF 标记：{}", rep);
        }
        other => panic!("应为 FirstDiff，实得 {:?}", other),
    }
}

// ───────────────── 附加：`report()` 的 WHITELIST 标记（CP3 的可读面） ─────────────────
#[test]
fn report_marks_whitelist() {
    use core_libs_io::io::opc::S01Compare::FirstDiff;
    let d = FirstDiff {
        path: "meta.impl".to_string(),
        kind: "value_mismatch",
        rust: "\"rust\"".to_string(),
        java: "\"java\"".to_string(),
        whitelisted: true,
    };
    assert!(
        d.report().starts_with("WHITELIST"),
        "白名单差异的报告必须以 WHITELIST 开头（CP3 的两行 WHITELIST 之一）：{}",
        d.report()
    );
}
