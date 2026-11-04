// function_id=24_function_part_name_conformance
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 24_function_part_name_conformance
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackagePartName.rs
//       libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackagingURIHelper.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/24_function_part_name_conformance.rs
//
// 校验链 = Java `PackagePartName.throwExceptionIfInvalidPartUri`（`:102-116`）：
//   ① uriPath 空 / `"/"` ⇒ M1.1  ② 绝对 URI ⇒ "Absolute URI forbidden"
//   ③ 不以 `/` 开头 ⇒ M1.4（用 uriPath 判，非 ASCII 串）
//   ④ `partName = toASCIIString(uri.toString())` 后逐段扫：M1.3 空段 / M1.9 段尾点 /
//      M1.10 全点段 / M1.6 非 pchar（`%` 走 M1.7/M1.8 + hex 解码）/ 收尾 M1.3（count < 1）
//   ⑤ `toLowerCase(partName)` 回写 + `isLowercase`
// 规则判定顺序 = Java 源码顺序，`error.at` 由「先命中的那条」决定。

use core_libs_io::io::opc::PackagePartName::PackagePartName;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;
use core_libs_io::io::util::URI::URI;

fn uri(spec: &str) -> URI {
    URI::new(spec).unwrap_or_else(|e| panic!("URI::new({spec:?}) 应成功，实际: {e}"))
}

/// 走**校验模式**工厂（= Java `new PackagePartName(uri, true)` 经 `createPartName`）：
/// `Ok` = Java 不抛异常，`Err` = Java 抛 `InvalidFormatException`。
fn check(spec: &str) -> Result<PackagePartName, String> {
    PackagingURIHelper::createPartName_str(spec)
}

fn err_of(res: Result<PackagePartName, String>) -> String {
    match res {
        Ok(v) => panic!("本用例期望被拒，实际得到 {}", v.getName()),
        Err(err) => err,
    }
}

fn assert_rejected(spec: &str, marker: &str) -> String {
    let err = err_of(check(spec));
    assert!(err.contains(marker), "{spec:?} 应命中 {marker}，实际: {err}");
    err
}

/// PNV1 / PNV1: **接线存在性** —— 校验链必须被 4 个工厂入口真实走到。
/// 源码文本判据（§3.1）：`grep -rn "throwExceptionIfInvalidPartUri" libs/core-libs/io/wps-io/src/` 命中 ≥ 3
/// = 注释 + 定义 + **调用者**（`PackagePartName::new`）；集成测试无法 grep ⇒ 这里用「非法名必须被拒」间接证明。
#[test]
fn pnv1_validation_chain_is_reachable_from_all_factory_entries() {
    let base = uri("/word/document.xml");
    // ★实测校正：`/word/../evil.xml` 作为 relative 重载的 target 会被 `resolvePartUri` **归一掉** `..`
    // （⇒ `/evil.xml`，合法）⇒ 「4 入口同 Err」必须用**解析后仍非法**的名字：空段 `/word//x.xml`。
    let bad = "/word//x.xml";
    assert!(check(bad).is_err(), "String 入口必须经校验");
    assert!(
        PackagingURIHelper::createPartName_uri(&uri(bad)).is_err(),
        "URI 入口必须经校验"
    );
    assert!(
        PackagingURIHelper::createPartName_str_relative(bad, &base).is_err(),
        "String relative 入口必须经校验"
    );
    assert!(
        PackagingURIHelper::createPartName_uri_relative(&uri(bad), &base).is_err(),
        "URI relative 入口必须经校验"
    );
    assert!(
        !PackagingURIHelper::isValidPartName(&uri(bad)),
        "isValidPartName 探针同源（Java 内部就是 try createPartName）"
    );
}

/// PNV2 / PNV2: M1.1 —— 空串与根名（校验模式下根名也被拒；根 part 名只能走 `checkConformance=false`）。
#[test]
fn pnv2_empty_and_root_are_rejected_m1_1() {
    let empty = assert_rejected("", "[M1.1]");
    assert_eq!(
        empty, "A part name shall not be empty [M1.1]: ",
        "Java 拼接的是 uriPath（此处为空串）"
    );
    assert_eq!(
        assert_rejected("/", "[M1.1]"),
        "A part name shall not be empty [M1.1]: /"
    );
}

/// PNV3 / PNV3: 绝对 URI ⇒ `throwExceptionIfAbsoluteUri`（在 M1.4 之前执行）。
#[test]
fn pnv3_absolute_uri_is_rejected() {
    assert_eq!(
        assert_rejected("http://example.com/a.xml", "Absolute URI forbidden"),
        "Absolute URI forbidden: http://example.com/a.xml"
    );
    assert_rejected("file:///a.xml", "Absolute URI forbidden");
}

/// PNV4 / PNV4: M1.4 —— 不以 `/` 开头（★实测消息已与 Java 原文逐字一致，含 `('/')` 与 uriPath）。
#[test]
fn pnv4_must_start_with_forward_slash_m1_4() {
    assert_eq!(
        assert_rejected("word/document.xml", "[M1.4]"),
        "A part name shall start with a forward slash ('/') character [M1.4]: word/document.xml"
    );
    assert_rejected("word", "[M1.4]");
    // 对照：首字符是 `/` ⇒ 不再触发 M1.4
    assert!(check("/word").is_ok());
}

/// PNV5 / PNV5: M1.3 —— 空段（`//`），消息含去掉结尾 NUL 的 partName。
#[test]
fn pnv5_empty_segments_m1_3() {
    for spec in ["/word//document.xml", "/a//b.xml", "//word/document.xml"] {
        let err = assert_rejected(spec, "[M1.3]");
        assert!(
            err.ends_with(&format!("[M1.3]: {spec}")),
            "Java 会把 `new String(partName, 0, len-1)` 拼进消息：{err}"
        );
    }
}

/// PNV6 / PNV6: M1.9 —— 段尾点（含 `.` 段与 `..` 段）。
#[test]
fn pnv6_segment_ends_with_dot_m1_9() {
    assert_eq!(
        assert_rejected("/word/./evil.xml", "[M1.9]"),
        "A segment shall not end with a dot ('.') character [M1.9]: /word/./evil.xml"
    );
    for spec in ["/word/a./b.xml", "/word/..", "/a/b./c.xml"] {
        assert_rejected(spec, "[M1.9]");
    }
}

/// PNV7 / PNV7: **M1.10 实际不可达** —— 任何全点段必然以 `.` 结尾，M1.9 判定在前
/// （Java `:144-150` 先于 `:152-156`；Rust 同序）⇒ 只断言「被拒」，不断言 M1.10。
#[test]
fn pnv7_all_dot_segment_hits_m1_9_not_m1_10() {
    let err = assert_rejected("/word/.../x.xml", "[M1.9]");
    assert!(!err.contains("[M1.10]"), "M1.9 必须先命中：{err}");
    assert_rejected("/...", "[M1.9]");
}

/// PNV8 / PNV8: M1.6 —— 非 pchar。★实测关键：`checkPCharCompliance(seg, begin, len)` 把**长度当结束下标**，
/// 而调用方传的是 `i - start` ⇒ 只有「首段的前 `len-1` 个字符」会被扫到。
/// 因此 `\` / `<` 放在**首段**必被拒，放在后续段则不触发（另见 `23` 的 `pnc5b_`）。
#[test]
fn pnv8_non_pchar_m1_6_first_segment_only() {
    let m1_6 = "A segment shall not hold any characters other than pchar characters. [M1.6]";
    // `<` 走 String 入口：`resolvePartName` 只归一 `\`，`<` 原样进入首段 ⇒ M1.6
    assert_eq!(assert_rejected("/w<rd/a.xml", "[M1.6]"), m1_6);
    // `\` 走 **URI 入口**：String 入口会先把 `\` 归一为 `/`（Java 同）⇒ 只有 URI 入口才命中 M1.6
    let backslash = err_of(PackagingURIHelper::createPartName_uri(&uri("/word\\document.xml")));
    assert_eq!(backslash, m1_6, "URI 入口不归一 `\\` ⇒ 首段命中 M1.6");
    assert_eq!(
        check("/word\\document.xml").expect("String 入口归一后合法").getName(),
        "/word/document.xml",
        "String 入口 \\ → / 归一（Java resolvePartName）"
    );
    // 后续段的 `<` 扫不到 ⇒ 放过（Java 同因）
    assert!(
        check("/word/a<b.xml").is_ok(),
        "第二段的 `<` 因 Java 的长度/下标缺陷不被扫到"
    );
    // 空格在 `toASCIIString` 就被映射成 `+`（Java `WpsUriUtils.java:67-69` 同样）⇒ M1.6 拿不到空格
    let spaced = check("/word/do c.xml").expect("空格被转义为 '+'（与 Java toASCIIString 一致）");
    assert_eq!(spaced.getName(), "/word/do+c.xml");
}

/// PNV9 / PNV9: M1.7 / M1.8 / `%` 后不足两位。★实测：`%XX` 必须放在**首段**才会被扫到。
#[test]
fn pnv9_percent_encoded_violations_m1_7_m1_8() {
    let m1_7 = "A segment shall not contain percent-encoded forward slash ('/'), or backward slash ('') characters. [M1.7]";
    assert_eq!(assert_rejected("/a%2Fb/x.xml", "[M1.7]"), m1_7);
    assert_eq!(assert_rejected("/a%5Cb/x.xml", "[M1.7]"), m1_7);
    assert_eq!(
        assert_rejected("/a%41b/x.xml", "[M1.8]"),
        "A segment shall not contain percent-encoded unreserved characters. [M1.8]"
    );
    // `%` 后不足两位：Java 消息 = "The segment " + new String(segment, begin, len) + " contain invalid encoded character !"
    assert_eq!(
        assert_rejected("/aa%b", "invalid encoded character"),
        "The segment aa%b contain invalid encoded character !"
    );
}

/// PNV9b / PNV9（★新增探针）: hex 解码失败 ⇒ Java 抛**未受检** `NumberFormatException`
/// （`hexChar2Int` 里 `throw new NumberFormatException(String.valueOf(c))` 会**穿透** `throws InvalidFormatException`）
/// ⇒ Rust 对应物是 `panic!`（RuntimeException 家族在本仓一律 panic）。
#[test]
fn pnv9b_invalid_hex_escape_is_unchecked_like_java() {
    let panicked = std::panic::catch_unwind(|| {
        let _ = check("/aa%zz/x.xml");
    })
    .is_err();
    assert!(
        panicked,
        "非 hex 的 %zz 必须像 Java NumberFormatException 一样**逃出**校验（panic），而不是返回 Err"
    );
    // 对照：合法的 %2F 走 M1.7 分支 ⇒ 是 Err（受检路径）
    assert!(check("/a%2Fb/x.xml").is_err());
}

/// PNV10 / PNV10: `..` 段（C7d 正本）—— ★实测由 **M1.9** 先命中（不是 M1.7/M1.3）。
#[test]
fn pnv10_dotdot_segment_is_rejected() {
    assert_rejected("/word/../evil.xml", "[M1.9]");
    assert_rejected("/a/../b.xml", "[M1.9]");
}

/// PNV11 / PNV11: 主路径 —— 合法名通过，且**内部 `partName` 已小写化**（`getName()` 仍保留原大小写）。
/// 观测方式：`compareTo`/`equals`/`hashCode` 都基于小写化后的 `partName`
/// （Java 侧同理：三者读 `this.partName`，`getName()` 读 `partNameURI`）。
#[test]
fn pnv11_legal_name_writes_lowercased_part_name() {
    let upper = check("/WORD/Document.XML").expect("合法（大小写不影响合法性）");
    let lower = check("/word/document.xml").expect("合法");

    assert_eq!(upper.getName(), "/WORD/Document.XML", "getName 保留原大小写");
    assert_eq!(lower.getName(), "/word/document.xml");
    assert_eq!(upper.getExtension(), "XML", "getExtension 也来自原串");
    assert_eq!(
        upper.compareTo(Some(&lower)),
        0,
        "compareTo 比较的是**小写化后**的 partName ⇒ 大小写差异被抹平"
    );
    assert!(upper.equals(Some(&lower)), "equals 同样基于小写化后的数组");

    let mut a = upper.clone();
    let mut b = lower.clone();
    assert_eq!(a.hashCode(), b.hashCode(), "hashCode 亦基于小写化后的 partName");
}

/// PNV12 / PNV12: **M1.5 空挂一致性** —— Java 的 `throwExceptionIfPartNameEndsWithForwardSlashChar`
/// 是 private 且**无调用者**（`throwExceptionIfInvalidPartUri` 不调它）⇒ 尾斜杠**不被拒**，两侧都 `Ok`。
#[test]
fn pnv12_trailing_slash_passes_because_m1_5_is_dead() {
    let trailing = check("/word/").expect("Java 的 M1.5 是死方法 ⇒ 尾斜杠被放过");
    assert_eq!(trailing.getName(), "/word/");
    assert_eq!(trailing.getExtension(), "", "`/word/` 内无 `.` ⇒ 扩展名为空串");
    assert!(check("/a/b/").is_ok(), "若有人顺手补 M1.5，本断言会失败（那将偏离 Java）");
}

/// PNV13 / PNV13: `isValidPartName` 探针与工厂结果**逐条一致**（Java `:277-288` 内部就是 `try createPartName`）。
#[test]
fn pnv13_is_valid_part_name_probe_agrees_with_factory() {
    for spec in [
        "/word/document.xml",
        "/word/",
        "/word//document.xml",
        "/word/../evil.xml",
        "/word/./evil.xml",
        "word/document.xml",
        "/",
        "",
    ] {
        let via_uri = match URI::new(spec) {
            Ok(u) => PackagingURIHelper::isValidPartName(&u),
            Err(_) => false, // URI 解析层已拒（Java 侧工厂同样会抛 ⇒ 等价于 false）
        };
        let via_factory = check(spec).is_ok();
        assert_eq!(via_uri, via_factory, "{spec:?}: isValidPartName 与 createPartName 必须一致");
    }
    // ★实测钉住 4 个关键值
    assert!(PackagingURIHelper::isValidPartName(&uri("/word/document.xml")));
    assert!(!PackagingURIHelper::isValidPartName(&uri("/word//document.xml")));
    assert!(!PackagingURIHelper::isValidPartName(&uri("/word/../evil.xml")));
    assert!(
        PackagingURIHelper::isValidPartName(&uri("/word/")),
        "/word/ 合法（M1.5 死方法）"
    );
}

/// PNV14 / PNV14: 幂等 / 确定性 —— 同一 URI 两次校验结果与视图相同。
#[test]
fn pnv14_same_uri_yields_identical_result_twice() {
    for spec in ["/word/document.xml", "/WORD/Document.XML", "/word/", "/_rels/.rels"] {
        let a = check(spec).expect(spec);
        let b = check(spec).expect(spec);
        assert_eq!(a.getName(), b.getName(), "{spec}");
        assert_eq!(a.getExtension(), b.getExtension(), "{spec}");
        assert_eq!(a.compareTo(Some(&b)), 0, "{spec}");
        assert!(a.equals(Some(&b)), "{spec}");
    }
    for spec in ["/word//document.xml", "/word/../evil.xml", "/a%2Fb/x.xml"] {
        assert_eq!(err_of(check(spec)), err_of(check(spec)), "{spec} 的错误文本必须稳定");
    }
}

/// PNV15 / PNV15: 规则顺序 = Java 源码顺序，`error.at` 由**先命中**的规则决定。
#[test]
fn pnv15_first_matching_rule_follows_java_order() {
    // ① 空 ⇒ M1.1（不是 M1.4）
    assert_rejected("", "[M1.1]");
    // ② 绝对 URI 先于 M1.4：用**有 path** 的绝对 URI（`http://example.com/a.xml` ⇒ path `/a.xml`，非空）
    assert_rejected("http://example.com/a.xml", "Absolute URI forbidden");
    // ②' ★实测：`http://x` 的 path 是**空串** ⇒ M1.1 先于「绝对 URI」判定（Java 执行顺序同）
    assert_rejected("http://x", "[M1.1]");
    // ③ 非空且无前导 `/` ⇒ M1.4（不是 M1.3）
    let err = assert_rejected("word", "[M1.4]");
    assert!(!err.contains("[M1.3]"), "{err}");
    // ④ 空段先于其后的段尾点：`/word//./x.xml` ⇒ M1.3
    assert_rejected("/word//./x.xml", "[M1.3]");
    // ⑤ 段尾点先于「全点段」M1.10（PNV7 已证 M1.10 不可达）
    assert_rejected("/word/.../x.xml", "[M1.9]");
}
