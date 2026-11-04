// function_id=25_function_part_name_is_rels
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 25_function_part_name_is_rels
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackagePartName.rs
//       libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackagingURIHelper.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/25_function_part_name_is_rels.rs
//
// Java 侧有**两套**关系 part 判定，本 function 必须两套都保真：
//   ① `PackagePartName.isRelationshipPartURI()`（getter `:98-100`）—— 值由构造期状态机
//      `private boolean isRelationshipPartURI(URI)`（`:58-96`）算出：扫 `uri.getPath()`，
//      在段首遇到 `/_rels/` ⇒ state=1（并要求该 `/` 之后仍有 ≥12 个字符），state=1 时遇到 `.`
//      ⇒ 只有剩余正好 5 个字符且为 `rels` 才返回 true（否则继续扫，最后 false）。
//   ② `PackagingURIHelper.isRelationshipPartURI(URI)`（`:40-46`）—— `String.matches(".*_rels.*.rels$")`，
//      ★常量 `".rels"` 里的 `.` **未转义 ⇒ 正则通配符**，因此条件是「以 `rels` 结尾（其前还有 1 个任意字符），
//      且 `_rels` 整体出现在末尾 5 字符之前」，比 ① 松得多。
// ⇒ 两者在同一输入上必须给出**不同**结果（`isr5_`），任一侧被「修成一致」都是偏离 Java。

use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;
use core_libs_io::io::util::URI::URI;

fn uri(spec: &str) -> URI {
    URI::new(spec).unwrap_or_else(|e| panic!("URI::new({spec:?}) 应成功，实际: {e}"))
}

/// 入口①：构造期状态机的结果（经 getter，= Java `partName.isRelationshipPartURI()`）。
fn via_state_machine(spec: &str) -> bool {
    match PackagingURIHelper::createPartName_str(spec) {
        Ok(name) => name.isRelationshipPartURI(),
        Err(e) => panic!("{spec:?} 应为合法 part 名（本用例只测关系判定），实际: {e}"),
    }
}

/// 入口②：正则版（= Java `PackagingURIHelper.isRelationshipPartURI(URI)`）。
fn via_regex(spec: &str) -> bool {
    PackagingURIHelper::isRelationshipPartURI(&uri(spec))
}

/// ISR1 / 入口① 正例：状态机能认出真实的关系 part 名（`/_rels/…` 段 + `.rels` 结尾）。
#[test]
fn isr1_state_machine_accepts_real_relationship_parts() {
    for spec in [
        "/_rels/.rels",                       // 包级关系
        "/word/_rels/document.xml.rels",      // 子目录关系
        "/_rels/a.rels",                      // 顶层 part 的关系
        "/a/_rels/b.xml.rels",
        "/a/_rels/.rels",
        "/ab/_rels/.rels",
    ] {
        assert!(via_state_machine(spec), "{spec:?} 应被状态机判为关系 part");
    }
}

/// ISR2 / 入口① 反例：`_rels` 不在段首、`/` 之后不足 12 字符、结尾不是 `.rels` 都要 false。
#[test]
fn isr2_state_machine_rejects_everything_else() {
    for spec in [
        "/word/document.xml",   // 普通 part
        "/_rels/.rels2",        // 结尾不是 `.rels`
        "/word/_relsx/a.rels",  // `_relsx` 段（段首是 `_rels` 但第 6 字符不是 `/`）
        "/_rels/arels",         // ★ 无点：仅正则版认，状态机必须 false
        "/_rels/rels",          // ★ `/` 后只剩 11 字符 ⇒ `n - i < 12` 提前 false
        "/_relsrels",           // `_rels` 与结尾 5 字符重叠 ⇒ 状态机 false
        "/x_rels.rels",         // `_rels` 不在 `/_rels/` 段首
        "/_rels/",              // 只是目录
        "/word/_rels/",
    ] {
        assert!(!via_state_machine(spec), "{spec:?} 不应被状态机判为关系 part");
    }
}

/// ISR3 / 入口② 正例：Java 正则 `.*_rels.*.rels$`（`.rels` 的 `.` 是通配符）⇒ 不必有点。
#[test]
fn isr3_regex_accepts_rels_suffix_without_the_dot() {
    for spec in [
        "/_rels/.rels",
        "/word/_rels/document.xml.rels",
        "/_rels/a.rels",
        "/a/_rels/b.xml.rels",
        "/_rels/arels",         // ★ 末 5 字符 = `arels` ⇒ 通配符吃 `a`
        "/_rels/rels",          // ★ 末 5 字符 = `/rels`
        "/word/_relsx/a.rels",  // ★ 正则只要「出现过 `_rels`」
        "/x_rels.rels",         // ★ `_rels` 可在文件名里
    ] {
        assert!(via_regex(spec), "{spec:?} 应被 Java 正则式判为关系 part");
    }
}

/// ISR4 / 入口② 反例：正则要求 `_rels` **整体**位于末尾 5 字符之前（顺序消费字符）⇒
/// `/_relsrels` 这种「`_rels` 与结尾重叠」的串**不**匹配。
#[test]
fn isr4_regex_rejects_when_rels_overlaps_the_tail() {
    for spec in [
        "/_relsrels",          // `_rels` 结束时只剩 4 字符 < 5 ⇒ 匹配失败
        "/word/document.xml",
        "/_rels/.rels2",
        "/_rels/",
        "/word/_rels/",
    ] {
        assert!(!via_regex(spec), "{spec:?} 不应被 Java 正则式判为关系 part");
    }
}

/// ISR5 / **两套实现必须保持不同**（★本 function 的核心保真点）：
/// 正则版（Java 的松判定）为 true，而构造期状态机为 false。若有人把两者改成一致，
/// 就会同时偏离 Java 的两处调用方（`PackagePart.java:125`/`PackagingURIHelper.java:228` 用正则，
/// 构造器用状态机）。
#[test]
fn isr5_two_implementations_stay_different_like_java() {
    for spec in [
        "/_rels/arels",         // 无点：正则 true / 状态机 false
        "/_rels/rels",          // `/` 后 11 字符：正则 true / 状态机 false
        "/word/_relsx/a.rels",  // `_rels` 不在段首：正则 true / 状态机 false
        "/x_rels.rels",
    ] {
        assert!(via_regex(spec), "{spec:?}: 正则版应为 true（Java）");
        assert!(!via_state_machine(spec), "{spec:?}: 状态机应为 false（Java）");
    }
    // 两侧都 true / 都 false 的对照，确保上面的差异不是「一边永远为真」
    assert!(via_regex("/_rels/.rels") && via_state_machine("/_rels/.rels"));
    assert!(!via_regex("/word/document.xml") && !via_state_machine("/word/document.xml"));
}

/// ISR6 / 结构判据：构造器用的是**状态机**而不是正则。
/// 证据：`/_rels/arels` 在两个实现上分叉，而 `PackagePartName.isRelationshipPartURI()` 给出 false
/// （若构造器改用正则，本用例会失败）。
#[test]
fn isr6_constructor_uses_the_state_machine() {
    let divergent = "/_rels/arels";
    assert!(via_regex(divergent), "正则版（Java 松判定）为 true");
    let name = PackagingURIHelper::createPartName_str(divergent).expect("合法 part 名");
    assert!(
        !name.isRelationshipPartURI(),
        "Java 构造器走的是状态机（`PackagePartName.java:57`）⇒ 必须为 false"
    );
}

/// ISR7 / 幂等：getter 是纯读（Java `return this.isRelationship`）⇒ 多次调用结果稳定。
#[test]
fn isr7_getter_is_stable_and_readonly() {
    for spec in ["/_rels/.rels", "/word/_rels/document.xml.rels", "/word/document.xml"] {
        let name = PackagingURIHelper::createPartName_str(spec).expect("合法 part 名");
        let first = name.isRelationshipPartURI();
        assert_eq!(first, name.isRelationshipPartURI(), "{spec:?}");
        assert_eq!(first, name.isRelationshipPartURI(), "{spec:?}");
        // 与「重新构造一次」一致（构造期判定是确定性的）
        let again = PackagingURIHelper::createPartName_str(spec).expect("合法 part 名");
        assert_eq!(first, again.isRelationshipPartURI(), "{spec:?}");
    }
}

/// ISR8 / 两套实现都作用于 `URI.getPath()`（Java 都是 `partUri.getPath()`）；
/// 但 Rust 的 `getPath()` 与 Java 尚有两处差距（**归属 `util/URI.rs`，本 function 禁改**）⇒
/// 本用例只钉住**当前**行为并在注释里写明 Java 期望，`URI.rs` 对齐后必须翻转。
#[test]
fn isr8_known_divergences_owned_by_uri_util() {
    // ① Java 的 `URI.getPath()` 会 **percent 解码**（`%2E` → `.`）⇒ Java 两侧都 true；
    //    Rust 拿到未解码串 ⇒ 状态机 false（正则恰好仍 true，因为原始串本身以 `rels` 结尾）
    assert_eq!(
        uri("/_rels/a%2Erels").getPath().as_deref(),
        Some("/_rels/a%2Erels"),
        "★当前 Rust：getPath() 未解码（Java 期望 `/_rels/a.rels`）"
    );
    assert!(via_regex("/_rels/a%2Erels"));
    assert!(!via_state_machine("/_rels/a%2Erels"), "★当前 Rust 为 false；Java 为 true");

    // ② Java 的 `getPath()` **不含** fragment；Rust 当前把 `#` 留在 path 里且 fragment 解析错位
    //    （`/_rels/a.rels#frag` ⇒ getPath `/_rels/a.rels#`、fragment `rag`）⇒ 状态机 false
    assert_eq!(
        uri("/_rels/a.rels#frag").getPath().as_deref(),
        Some("/_rels/a.rels#"),
        "★当前 Rust：`#` 未剥离（Java 期望 `/_rels/a.rels`）"
    );
    assert_eq!(
        uri("/_rels/a.rels#frag").getFragment().map(|f| f.to_string()),
        Some("rag".to_string()),
        "★当前 Rust：fragment 首字符丢失（Java 期望 `frag`）"
    );
    assert!(!via_state_machine("/_rels/a.rels#frag"), "★当前 Rust 为 false；Java 为 true");

    // 对照：query 已被正确剥离 ⇒ 与 Java 一致（两侧都 true）
    assert_eq!(uri("/_rels/a.rels?x=1").getPath().as_deref(), Some("/_rels/a.rels"));
    assert!(via_state_machine("/_rels/a.rels?x=1"));
    assert!(via_regex("/_rels/a.rels?x=1"));
}
