// 55_function_error_trace —— `apiTrace` / `diagnostics.warnings[]` 用例（TR1–TR19）
//
// 规格来源：`features/s01-09-error-model/functions/55_function_error_trace/FUNCTION.md` §3.4 用例表。
// 契约来源：`docs/s01/schema-v1.json`（`open_entry` 4 值枚举 / `warnings[]` 为 `string[]` / 白名单 4 条）。

use core_libs_io::io::opc::ApiTrace::{ApiTrace, OpenEntry};
use core_libs_io::io::opc::S01Error::{S01ErrorKind, S01Failure, S01Taxonomy};

/// 从 JSON 文本里取某个 `"key":` 之后的原始片段（不做完整 JSON 解析，避免引依赖）。
fn has_key(json: &str, key: &str) -> bool {
    json.contains(&format!("\"{}\":", key))
}

/// 取 `"key": value` 的 value 文本（到行尾逗号为止），用于断言 `null` / 数值。
fn value_of<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let needle = format!("\"{}\": ", key);
    let i = json.find(&needle)? + needle.len();
    let rest = &json[i..];
    let end = rest.find([',', '\n']).unwrap_or(rest.len());
    Some(rest[..end].trim())
}

fn sample_error() -> core_libs_io::io::opc::S01Error::S01Error {
    let f = S01Failure::OptionMiss {
        at: "xwpf::usermodel::XWPFDocument::XWPFDocument__pkg#main_rel_assert".to_string(),
    };
    S01Taxonomy::classify(&f).expect("合法输入不应被拒绝")
}

// ───────────────────────── TR1：五 key 齐全、error==null ─────────────────────────
#[test]
fn tr1_five_keys_present_error_null() {
    let mut t = ApiTrace::new();
    t.record_open_entry(OpenEntry::Bytes).expect("合法枚举值");
    t.record_counts(5, 3);
    t.record_main_part(Some("/word/document.xml"));

    let j = t.into_json();
    for k in [
        "open_entry",
        "part_count",
        "relationship_count",
        "main_part",
        "error",
    ] {
        assert!(has_key(&j, k), "缺 key {} ⇒ fail\n{}", k, j);
    }
    assert_eq!(value_of(&j, "error"), Some("null"));
    assert_eq!(value_of(&j, "open_entry"), Some("\"bytes\""));
    assert_eq!(value_of(&j, "part_count"), Some("5"));
    assert_eq!(value_of(&j, "relationship_count"), Some("3"));
}

// ───────────────────────── TR2：有 error 时 part_count 仍为数 ─────────────────────────
#[test]
fn tr2_error_path_still_reports_counts() {
    let mut t = ApiTrace::new();
    t.record_open_entry(OpenEntry::Bytes).expect("合法枚举值");
    t.record_counts(5, 3);
    t.record_error(sample_error());

    let j = t.into_json();
    for k in [
        "open_entry",
        "part_count",
        "relationship_count",
        "main_part",
        "error",
    ] {
        assert!(has_key(&j, k), "缺 key {} ⇒ fail", k);
    }
    assert_eq!(
        value_of(&j, "part_count"),
        Some("5"),
        "缺 part_count ⇒ fail（错误路径也必须给数）"
    );
    assert!(j.contains("\"kind\": \"missing_main_part\""), "{}", j);
}

// ───────────────────────── TR3：4 个合法枚举值全部接受 ─────────────────────────
#[test]
fn tr3_all_four_open_entries_accepted() {
    for e in OpenEntry::all() {
        let mut t = ApiTrace::new();
        t.record_open_entry(e)
            .unwrap_or_else(|r| panic!("{} 被拒：{}", e.as_str(), r.reason));
        assert_eq!(t.open_entry(), Some(e));
        // 字符串入口同样接受
        let mut t2 = ApiTrace::new();
        t2.record_open_entry_str(e.as_str())
            .unwrap_or_else(|r| panic!("{} 被拒：{}", e.as_str(), r.reason));
        assert_eq!(t2.open_entry(), Some(e));
    }
    assert_eq!(OpenEntry::all().len(), 4, "schema 枚举恰好 4 值");
    for (e, s) in OpenEntry::all().iter().zip(["bytes", "path", "raf", "create"]) {
        assert_eq!(e.as_str(), s);
    }
}

// ───────────────────────── TR4：方法名必须被拒绝 ─────────────────────────
#[test]
fn tr4_method_name_rejected() {
    for bad in [
        "XWPFPackage::open_stream",
        "open(InputStream)",
        "Bytes",
        "BYTES",
        "",
        "stream",
    ] {
        let mut t = ApiTrace::new();
        let r = t.record_open_entry_str(bad);
        assert!(
            r.is_err(),
            "落盘必须是 bytes|path|raf|create；静默接受 {:?} ⇒ fail",
            bad
        );
        assert_eq!(t.open_entry(), None, "被拒时不得写入");
    }
}

// ───────────────────────── TR5：main_part=None ⇒ 显式 null ─────────────────────────
#[test]
fn tr5_main_part_null_not_missing() {
    let mut t = ApiTrace::new();
    t.record_main_part(None);
    let j = t.into_json();
    assert!(has_key(&j, "main_part"), "key 缺失 ⇒ fail");
    assert_eq!(value_of(&j, "main_part"), Some("null"));
}

// ───────────────────────── TR6：error=None ⇒ 显式 null ─────────────────────────
#[test]
fn tr6_error_null_not_missing() {
    let t = ApiTrace::new();
    let j = t.into_json();
    assert!(has_key(&j, "error"), "key 缺失 ⇒ fail");
    assert_eq!(value_of(&j, "error"), Some("null"));
}

// ───────────────────────── TR7：W1 早退落 warnings ─────────────────────────
#[test]
fn tr7_w1_early_return_warning() {
    let mut t = ApiTrace::new();
    t.record_warning(ApiTrace::W1_PARSE_PART_XML_EARLY_RETURN, "no content");
    let j = t.into_json();
    let expect = format!("{}: ", ApiTrace::W1_PARSE_PART_XML_EARLY_RETURN);
    assert!(
        j.contains(&expect),
        "warnings[0] 应以 {:?} 开头（空 ⇒ fail）\n{}",
        expect,
        j
    );
    assert_eq!(t.warnings().len(), 1);
    assert!(t.warnings()[0].starts_with(&expect));
}

// ───────────────────────── TR8：W2 无 CT 丢 part ─────────────────────────
#[test]
fn tr8_w2_part_dropped_no_ct() {
    let mut t = ApiTrace::new();
    t.record_warning(ApiTrace::W2_PART_DROPPED_NO_CT, "part dropped");
    let j = t.into_json();
    assert!(
        j.contains("#part_dropped_no_ct"),
        "warnings 含 #part_dropped_no_ct（空 ⇒ fail）\n{}",
        j
    );
}

// ───────────────────────── TR9：W3 无 archive ─────────────────────────
#[test]
fn tr9_w3_no_archive() {
    let mut t = ApiTrace::new();
    t.record_warning(ApiTrace::W3_NO_ARCHIVE, "zipArchive is null");
    let j = t.into_json();
    assert!(j.contains("#no_archive"), "warnings 含 #no_archive\n{}", j);
}

// ───────────────────────── TR10：W4 丢错误文本 但 detail 保留 ─────────────────────────
#[test]
fn tr10_w4_dropped_text_but_detail_kept() {
    let raw = "invalid Zip archive: CRC32 mismatch at 0x1f";
    let mut t = ApiTrace::new();
    t.record_error_text_dropped(raw);
    // 原始 zip crate 文本必须进 `error.detail`
    let f = S01Failure::ZipErr {
        msg: raw.to_string(),
    };
    t.record_error(S01Taxonomy::classify(&f).expect("合法输入不应被拒绝"));

    let j = t.into_json();
    assert!(
        j.contains("#error_text_dropped"),
        "warnings 含 #error_text_dropped\n{}",
        j
    );
    assert!(
        j.contains("CRC32 mismatch"),
        "detail 必须保留原始 zip crate 文本（也丢 ⇒ fail）\n{}",
        j
    );
}

// ───────────────────────── TR11：error 与 warnings 共存，不互相覆盖 ─────────────────────────
#[test]
fn tr11_error_does_not_override_warnings() {
    let mut t = ApiTrace::new();
    t.record_warning(ApiTrace::W1_PARSE_PART_XML_EARLY_RETURN, "w1");
    t.record_warning(ApiTrace::W3_NO_ARCHIVE, "w3");
    t.record_error(sample_error());

    let j = t.into_json();
    assert_eq!(t.warnings().len(), 2, "两条 warning 必须都在");
    assert!(j.contains("#early_return"), "{}", j);
    assert!(j.contains("#no_archive"), "{}", j);
    assert!(
        j.contains("\"kind\": \"missing_main_part\""),
        "error 必须同时存在（被覆盖 ⇒ fail）\n{}",
        j
    );
    // 顺序稳定：先记的先出现
    let a = j.find("#early_return").unwrap();
    let b = j.find("#no_archive").unwrap();
    assert!(a < b, "warnings 顺序必须稳定");
}

// ───────────────────────── TR12/TR13：C1 真包口径 ─────────────────────────
#[test]
fn tr12_tr13_counts_mirror_parts_and_rel_scope() {
    // 本 function **只透传不重算**（§1.6-4）：口径由 s01-04 / s01-05 钉死。
    // TR12：`part_count == parts[].len()`；TR13：relationship_count = 包级 + part 级之和。
    //
    // ★实测（s01-04 §2.2 / s01-05 §2.2）：
    //   C1 `standard-hello.docx`：part 表 = 9，`_rels/.rels` = 3，`word/_rels/document.xml.rels` = 4
    //   ⇒ relationship_count = 3 + 4 = 7（part 级加数在 Rust 侧**恒为 0** 的部分见 §1.6-4 更正）
    let c1_parts = 9usize;
    let c1_pkg_rels = 3usize;
    let c1_part_rels_sum = 4usize;
    let expected_rel = c1_pkg_rels + c1_part_rels_sum;

    let mut t = ApiTrace::new();
    t.record_open_entry(OpenEntry::Bytes).expect("合法枚举值");
    t.record_counts(c1_parts, expected_rel);
    t.record_main_part(Some("/word/document.xml"));

    assert_eq!(
        t.part_count(),
        Some(c1_parts),
        "part_count 必须等于 parts[] 条目数（不等 ⇒ fail）"
    );
    assert_eq!(
        t.relationship_count(),
        Some(7),
        "口径：包级 3 + part 级 4 = 7（口径未声明/不符 ⇒ fail）"
    );
    let j = t.into_json();
    assert_eq!(value_of(&j, "part_count"), Some("9"));
    assert_eq!(value_of(&j, "relationship_count"), Some("7"));
}

// ───────────────────────── TR14：计数与 parts[] 同源（不得二次遍历） ─────────────────────────
#[test]
fn tr14_counts_from_same_snapshot() {
    // 模拟「同一次认包」的单一快照：计数与 parts[] 取自**同一个** vector。
    let snapshot: Vec<&str> = vec![
        "/word/document.xml",
        "/word/styles.xml",
        "/_rels/.rels",
        "/word/_rels/document.xml.rels",
        "/docProps/core.xml",
    ];
    let mut t = ApiTrace::new();
    let mut t2 = ApiTrace::new();
    // 同源：两次都从同一个 snapshot 取 len
    t.record_counts(snapshot.len(), 0);
    t2.record_counts(snapshot.len(), 0);
    assert_eq!(t.part_count(), t2.part_count(), "同源 ⇒ 两次一致");
    assert_eq!(t.part_count(), Some(snapshot.len()));

    // 反例语义：若二次遍历得到不同的 vec，则计数会漂移 —— 本 API 不提供二次遍历入口
    // （只有 `record_counts` 一次写入；`into_json` 只读缓存字段，不重算）。
    let j1 = t.into_json();
    let j2 = t.into_json();
    assert!(
        j1.contains("\"part_count\": 5"),
        "into_json 必须用缓存值，不得重算\n{}",
        j1
    );
    assert_eq!(j1, j2, "into_json 幂等");
}

// ───────────────────────── TR15：detail 不进比较投影 ─────────────────────────
#[test]
fn tr15_detail_excluded_from_projection() {
    let mut a = ApiTrace::new();
    let mut b = ApiTrace::new();
    a.record_error(
        S01Taxonomy::classify(&S01Failure::ZipErr {
            msg: "message A".to_string(),
        })
        .expect("合法输入不应被拒绝"),
    );
    b.record_error(
        S01Taxonomy::classify(&S01Failure::ZipErr {
            msg: "message B".to_string(),
        })
        .expect("合法输入不应被拒绝"),
    );
    assert_ne!(
        a.error().unwrap().detail,
        b.error().unwrap().detail,
        "构造前提：detail 不同"
    );
    assert_eq!(
        a.error_projection(),
        b.error_projection(),
        "detail 进投影 ⇒ fail"
    );
    // 投影第三元不含 detail
    let (k, at) = a.error_projection().unwrap();
    assert_eq!(k, S01ErrorKind::IoError.as_str());
    assert!(!at.is_empty());
}

// ───────────────────────── TR16：JSON 确定性（key 顺序 / UTF-8 / \n 结尾） ─────────────────────────
#[test]
fn tr16_json_is_deterministic_and_newline_terminated() {
    let mut t = ApiTrace::new();
    t.record_open_entry(OpenEntry::Path).expect("合法枚举值");
    t.record_counts(2, 1);
    t.record_main_part(Some("/word/document.xml"));
    t.record_warning(ApiTrace::W3_NO_ARCHIVE, "中文消息 with \"quotes\"");
    let j = t.into_json();

    assert!(j.ends_with('\n'), "必须以 \\n 结尾");
    assert!(j.contains("\"apiTrace\""), "含 apiTrace 段");
    assert!(j.contains("\"diagnostics\""), "含 diagnostics 段");
    // key 顺序稳定
    let o = j.find("\"open_entry\"").unwrap();
    let p = j.find("\"part_count\"").unwrap();
    let r = j.find("\"relationship_count\"").unwrap();
    let m = j.find("\"main_part\"").unwrap();
    let e = j.find("\"error\"").unwrap();
    assert!(o < p && p < r && r < m && m < e, "key 顺序必须稳定\n{}", j);
    // 转义正确
    assert!(j.contains("\\\"quotes\\\""), "引号必须转义\n{}", j);
    // UTF-8 有效
    assert!(std::str::from_utf8(j.as_bytes()).is_ok());
}

// ───────────────────────── TR17：同输入连跑两次 byte-identical ─────────────────────────
#[test]
fn tr17_two_runs_byte_identical() {
    let build = || {
        let mut t = ApiTrace::new();
        t.record_open_entry(OpenEntry::Bytes).expect("合法枚举值");
        t.record_counts(9, 7);
        t.record_main_part(Some("/word/document.xml"));
        t.record_warning(ApiTrace::W2_PART_DROPPED_NO_CT, "dropped");
        t.record_error(sample_error());
        t.into_json()
    };
    let a = build();
    let b = build();
    assert_eq!(a.as_bytes(), b.as_bytes(), "差异 ⇒ fail");
    // 同一对象重复调用亦一致
    let mut t = ApiTrace::new();
    t.record_open_entry(OpenEntry::Bytes).expect("合法枚举值");
    assert_eq!(t.into_json(), t.into_json(), "同一对象两次 into_json 必须一致");
}

// ───────────────────────── TR18：断言失效场景不产生条目 ─────────────────────────
#[test]
fn tr18_no_assertion_sourced_entries() {
    // `XWPFDocument.java:94` 的 `IOAssert` 不生效 ⇒ 断言层**不产生任何** warnings/error。
    let t = ApiTrace::new();
    assert!(
        t.warnings().is_empty(),
        "warnings[]/error 不含任何「断言」来源条目（含 ⇒ fail）"
    );
    assert!(t.error().is_none());

    // 只有显式 record_* 才会产生条目；模块内无以「断言失败」为来源的入口。
    let j = t.into_json();
    let lower = j.to_lowercase();
    assert!(!lower.contains("ioassert"), "不得出现断言来源\n{}", j);
    assert!(!lower.contains("assert"), "不得出现断言来源\n{}", j);
}

// ───────────────────────── TR19：apiTrace 段内无 elapsed_ms ─────────────────────────
#[test]
fn tr19_elapsed_ms_not_in_apitrace() {
    // `elapsed_ms` 在**白名单**内，属 `diagnostics` 兄弟段；
    // 本 function 的 `apiTrace` 段**不得**含它。
    let mut t = ApiTrace::new();
    t.record_open_entry(OpenEntry::Bytes).expect("合法枚举值");
    t.record_counts(1, 0);
    let j = t.into_json();
    assert!(
        !j.contains("elapsed_ms"),
        "apiTrace 段内不得出现 elapsed_ms（出现 ⇒ fail）\n{}",
        j
    );

    // 定位 apiTrace 段并局部断言，避免误伤 diagnostics 段未来可能新增的字段
    let start = j.find("\"apiTrace\"").unwrap();
    let end = j[start..].find("\"diagnostics\"").map(|i| start + i).unwrap_or(j.len());
    let seg = &j[start..end];
    assert!(!seg.contains("elapsed_ms"), "apiTrace 段内不得有 elapsed_ms");
}

// ───────────────────────── 补充：warnings 为空时仍是合法空数组 ─────────────────────────
#[test]
fn empty_warnings_is_valid_array() {
    let t = ApiTrace::new();
    let j = t.into_json();
    assert!(j.contains("\"warnings\": []"), "空 warnings 必须仍是数组\n{}", j);
}
