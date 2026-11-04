// function_id=22_function_content_type_value
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 22_function_content_type_value
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/internal/ContentType.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/internal/ContentType.rs

// ── ★2026-09-20 测试助手：`ZipEntry` 已是**真实类型**（`ZipEntryRef` 封闭枚举），
//    `getInputStream*` 的形参不再接受 `&dyn Any` ⇒ 测试也要传真实 entry。
fn ref_of(e: &FakeZipEntry) -> ZipEntryRef {
    ZipEntryRef::Fake(e.clone())
}
use core_libs_io::io::xwpf::util::ZipEntrySource::{FakeZipEntry, ZipEntryRef};
use core_libs_io::io::opc::internal::ContentType::ContentType;

/// Java `VALID_CHAR_TABLE`（`ContentType.java:9`）里值为 **1** 的 128 项索引（★实测逐字节比对过 Java 字面量）。
/// 可打印的 0 值里，`'/'`(47) 走专门分支，其余 17 项一律失败。
const VALID_INDEXES: [usize; 77] = [
    33, 35, 36, 37, 38, 39, 42, 43, 45, 46, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68,
    69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 94, 95,
    96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115,
    116, 117, 118, 119, 120, 121, 122, 124, 126,
];

/// Java 里表值为 0 的**可打印** ASCII（17 项，`'/'` 除外）；它们都必须失败。
const INVALID_PRINTABLE: [char; 17] = [
    ' ', '"', '(', ')', ',', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '{', '}',
];

const SLASH: usize = 47;

fn malformed_message(input: &str, pos: usize) -> String {
    format!(
        "The specified content type '{}' is not compliant with RFC 2616: malformed content type. pos: {}",
        input, pos
    )
}

/// 取 `Err` 消息（`ContentType` 未实现 `Debug` ⇒ 不能用 `expect_err`；Java 侧也没有 `Debug` 对应物，
/// enum/值对象只有 `toString()`）。
fn err_of(input: &str) -> String {
    match ContentType::new(input) {
        Ok(value) => panic!("{input} 应失败，实际得到 {}", value.toString()),
        Err(err) => err,
    }
}

fn pct(s: &str) -> ContentType {
    ContentType::new(s).unwrap_or_else(|e| panic!("{s} 应合法，实际: {e}"))
}

/// V1 / V-01: 主路径切分 + `toString` 恒等。
#[test]
fn v1_application_xml_splits_and_round_trips() {
    let ct = pct("application/xml");
    assert_eq!(ct.getType(), "application");
    assert_eq!(ct.getSubType(), "xml");
    assert_eq!(ct.toString(), "application/xml");
}

/// V2 / V-02: 真实长 MIME（含 `+` / `.` / `-`）必须被接受。
#[test]
fn v2_long_ooxml_mime_with_plus_dot_dash_is_accepted() {
    let long = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
    let ct = pct(long);
    assert_eq!(ct.getType(), "application");
    assert_eq!(
        ct.getSubType(),
        "vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"
    );
    assert_eq!(ct.toString(), long, "toString 对合法输入必须恒等");
}

/// V3 / V-03: 单字符 type / subType。
#[test]
fn v3_single_char_type_and_subtype() {
    let ct = pct("a/b");
    assert_eq!((ct.getType(), ct.getSubType()), ("a", "b"));
    assert_eq!(ct.toString(), "a/b");
}

/// V4 / V-04: 尾斜杠 ⇒ **合法**，`subType` 为空串（非直觉边界）。
#[test]
fn v4_trailing_slash_is_legal_with_empty_subtype() {
    let ct = pct("a/");
    assert_eq!(ct.getType(), "a");
    assert_eq!(ct.getSubType(), "");
    assert_eq!(ct.toString(), "a/");
}

/// V5 / V-05: 前导斜杠 / 纯斜杠 ⇒ 失败（`i < 1` 分支），位置必须准确。
#[test]
fn v5_leading_slash_is_rejected() {
    for input in ["/a", "/"] {
        assert_eq!(err_of(input), malformed_message(input, 0), "输入 {input}");
    }
    // 位置 0 是唯一的判定点：`a/` 合法（V4）而 `/a` 非法
    assert!(ContentType::new("a/").is_ok());
}

/// V6 / V-06: 无斜杠、空串 ⇒ 失败（收尾 `posSlash == -1`）。
#[test]
fn v6_missing_slash_and_empty_are_rejected() {
    assert_eq!(err_of(""), malformed_message("", 0));
    assert_eq!(
        err_of("image"),
        malformed_message("image", 5),
        "无斜杠时 Java 报的 pos 是循环结束后的 i == n"
    );
}

/// V7 / V-07: 第二个斜杠 ⇒ 失败（`posSlash >= 0` 分支，不能只切第一个）。
#[test]
fn v7_second_slash_is_rejected() {
    assert_eq!(
        err_of("a/b/c"),
        malformed_message("a/b/c", 3)
    );
    assert_eq!(
        err_of("application/xml+foo/bar"),
        malformed_message("application/xml+foo/bar", 19)
    );
}

/// V8 + V11 / V-08 + V-11: 掩码表**行为快照** —— 128 项逐个插入被测串：
/// 表值 1 ⇒ 合法且恒等；表值 0（`'/'` 除外）⇒ 失败且 `pos == 1`。
#[test]
fn v8_v11_char_table_snapshot_over_all_128_ascii() {
    let mut valid_seen: Vec<usize> = Vec::new();
    for code in 0usize..128 {
        if code == SLASH {
            continue; // `'/'` 由 V4/V5/V7 覆盖
        }
        let ch = char::from_u32(code as u32).unwrap();
        let input = format!("a{ch}b/c");
        if VALID_INDEXES.contains(&code) {
            let ct = ContentType::new(&input)
                .unwrap_or_else(|e| panic!("表值 1 的 {code} ({ch:?}) 应合法，实际: {e}"));
            assert_eq!(ct.toString(), input, "合法输入必须恒等（code {code}）");
            valid_seen.push(code);
        } else {
            match ContentType::new(&input) {
                Ok(_) => panic!("表值 0 的 {code} ({ch:?}) 应失败，却成功了"),
                Err(err) => assert_eq!(err, malformed_message(&input, 1), "code {code} ({ch:?})"),
            }
        }
    }
    assert_eq!(valid_seen, VALID_INDEXES.to_vec(), "合法字符集合必须与 Java 表一致（77 项）");
    assert_eq!(valid_seen.len(), 77);
    // 可打印的 0 值（17 项）逐个复核
    for ch in INVALID_PRINTABLE {
        let input = format!("a{ch}b/c");
        assert!(
            ContentType::new(&input).is_err(),
            "可打印的非法字符 {ch:?} 必须失败（Java 表值 0）"
        );
    }
}

/// V9 / V-09: 非 ASCII ⇒ 失败，且走 **ASCII 门禁**（消息不含 pos），不查表。
#[test]
fn v9_non_ascii_is_rejected_by_ascii_gate() {
    const ASCII_MSG: &str = "The specified content type is not an ASCII value.";
    for input in ["caf\u{e9}/xml", "\u{6587}\u{6863}/xml", "a/\u{1f600}", "\u{80}/x"] {
        assert_eq!(
            err_of(input),
            ASCII_MSG,
            "输入 {input} 必须命中 ASCII 门禁（先于掩码表判定）"
        );
    }
}

/// V10 / V-10: 大小写 —— 合法、`toString` **保留原样**（不得擅自小写化），`equals`/`==` 忽略大小写。
#[test]
fn v10_case_is_preserved_but_equality_ignores_it() {
    let upper = pct("APPLICATION/XML");
    assert_eq!(upper.toString(), "APPLICATION/XML");
    assert_eq!(upper.getType(), "APPLICATION");
    assert_eq!(upper.getSubType(), "XML");

    let lower = pct("application/xml");
    assert!(upper.equals(&lower), "equals 忽略大小写（Java equalsIgnoreCase）");
    assert!(lower.equals(&upper));
    assert!(upper == lower, "PartialEq 委托 equals");
}

/// V12 / V-12: `hashCode` 首次缓存后固定 + 对样本集合 `toString` 恒等。
#[test]
fn v12_hash_code_is_cached_and_strings_round_trip() {
    let mut ct = pct("application/vnd.openxmlformats-package.relationships+xml");
    let first = ct.hashCode();
    let second = ct.hashCode();
    assert_eq!(first, second, "同一实例的 hashCode 必须稳定（Java 的 hash 字段缓存）");
    assert_eq!(ct.toString(), "application/vnd.openxmlformats-package.relationships+xml");

    // 样本集合（本 feature 实测出现过的 CT 串）逐条恒等
    let samples = [
        "application/vnd.openxmlformats-package.relationships+xml",
        "application/xml",
        "text/xml",
        "image/png",
        "image/jpeg",
        "application/vnd.openxmlformats-officedocument.extended-properties+xml",
        "application/vnd.openxmlformats-package.core-properties+xml",
    ];
    for s in samples {
        let ct = pct(s);
        assert_eq!(ct.toString(), s, "round-trip 必须恒等：{s}");
        assert_eq!(format!("{}/{}", ct.getType(), ct.getSubType()), s);
    }
}

/// V13 / V-13: **契约违反的观测** —— `equals` 忽略大小写但 `Hash` 大小写敏感 ⇒
/// `"A/b"` 与 `"a/B"` 相等却哈希不同 ⇒ 在 `HashMap` 里落入**两个**条目。
/// Java `Hashtable` 同因（`equals` 用 `equalsIgnoreCase`、`hashCode` 用大小写敏感的 `toString().hashCode()`）
/// ⇒ 照抄即保持 parity，**不得**为「自洽」把 `Hash` 改成忽略大小写。
#[test]
fn v13_eq_hash_contract_violation_is_faithfully_mirrored() {
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashMap;
    use std::hash::BuildHasherDefault;
    use std::hash::{Hash, Hasher};

    let a = pct("A/b");
    let b = pct("a/B");
    assert!(a.equals(&b) && (a == b), "两个值对象按 equals/PartialEq 相等");

    fn hash_of(ct: &ContentType) -> u64 {
        let mut hasher = DefaultHasher::new();
        ct.hash(&mut hasher);
        hasher.finish()
    }
    assert_ne!(
        hash_of(&a),
        hash_of(&b),
        "`Hash` 用大小写敏感的 toString ⇒ 相等的两个值对象哈希不同（与 Java 同样违反 Eq/Hash 契约）"
    );

    // 固定 hasher（不用随机 seed）以保证用例确定性
    let mut map: HashMap<ContentType, &str, BuildHasherDefault<DefaultHasher>> = HashMap::default();
    map.insert(a, "upper");
    map.insert(b, "lower");
    assert_eq!(map.len(), 2, "契约违反的后果：相等（equals）的两把键占两个槽（Java Hashtable 同）");
}

/// 门面：Java `public ContentType(String)` ⇒ Rust 同名入口与 `new` 同实现。
#[test]
fn facade_same_named_constructor_matches_new() {
    let by_java_name = ContentType::ContentType("image/png").expect("同名入口应可用");
    let by_rust_name = ContentType::new("image/png").expect("new 应可用");
    assert_eq!(by_java_name.toString(), by_rust_name.toString());
    assert!(by_java_name == by_rust_name);
    // 失败路径同样一致（错误消息逐字相同）
    assert_eq!(
        err_of("nope"),
        ContentType::new("nope").map(|v| v.toString()).unwrap_err()
    );
}

/// V14 / §3.3 建议项：**真实语料** —— C1 的 `[Content_Types].xml` 里每条 CT 都必须能构造 `ContentType`
/// 且 `toString` 恒等（4 Default + 7 Override = 11 条，★03_function_opc_content_types 的 ct16 已实测计数）。
#[test]
fn v14_c1_real_content_types_all_parse_and_round_trip() {
    use core_libs_io::io::opc::internal::ContentTypeManager::{
        ContentTypeManager, CONTENT_TYPES_PART_NAME,
    };
    use core_libs_io::io::xwpf::util::ZipFileZipEntrySource::ZipFileZipEntrySource;
    use std::io::Read as _;
    use std::io::Cursor;

    let fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx");
    let source =
        ZipFileZipEntrySource::from_path(fixture.to_str().unwrap()).expect("C1 应能打开");
    let entry = source
        .getEntries()
        .expect("C1 有 entry")
        .iter()
        .find(|e| e.getName() == CONTENT_TYPES_PART_NAME)
        .expect("C1 必有 [Content_Types].xml");
    let mut stream = source
        .getInputStream(&ref_of(entry), false)
        .expect("取流不应失败")
        .expect("应有该 entry 的流");
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes).expect("读到底");

    let mut input = Cursor::new(bytes);
    let mgr = ContentTypeManager::new(Some(&mut input), None);

    let mut checked = 0usize;
    for (_, ct) in mgr.getDefaultContentTypes().iter() {
        let value = pct(ct);
        assert_eq!(value.toString(), ct.as_str(), "Default 的 CT 必须恒等：{ct}");
        checked += 1;
    }
    for (_, ct) in mgr.getOverrideContentTypes().expect("C1 有 Override").iter() {
        let value = pct(ct);
        assert_eq!(value.toString(), ct.as_str(), "Override 的 CT 必须恒等：{ct}");
        checked += 1;
    }
    assert_eq!(checked, 11, "C1 = 4 Default + 7 Override（ct16 实测计数）");
}
