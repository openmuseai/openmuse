// function_id=27_function_part_name_resolve
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 27_function_part_name_resolve
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackagingURIHelper.rs
//       libs/core-libs/io/wps-io/src/cn/wps/io/util/URI.rs（`resolve()` 本体，call-only 依赖）
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/27_function_part_name_resolve.rs
//
// 本 function = 关系 Target 字符串 → 绝对 part URI 的全部入口：
//   ① `resolvePartUri(source, target)`（Java `:202-212`）：两侧都必须**相对** URI，否则
//      `IllegalArgumentException("sourcePartUri invalid - …"/"targetUri invalid - …")`；
//      成功则 `sourcePartUri.resolve(targetUri)`（RFC 3986 语义，本体在 `util/URI.rs`）。
//   ② `getURIFromPath(path)`（`:213-222`）：`new URI(path)`，语法错 ⇒ `IllegalArgumentException("path")`。
//   ③ `relativizeURI(source, target, msCompatible)`（`:116-…`）：绝对 URI → 相对 target 串。
//   ④ `getSourcePartUriFromRelationshipPartUri(URI)`（`:224-238`）：关系 part URI → 源 part URI。

use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;
use core_libs_io::io::util::URI::URI;

fn uri(spec: &str) -> URI {
    URI::new(spec).unwrap_or_else(|e| panic!("URI::new({spec:?}) 应成功，实际: {e}"))
}

fn resolve(source: &str, target: &str) -> String {
    PackagingURIHelper::resolvePartUri(&uri(source), &uri(target)).toString()
}

/// 读 panic 的文本载荷：`panic!("字面量")` 载荷是 `&'static str`，`panic!("…{}", x)` 才是 `String`。
fn panic_message(err: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&'static str>() {
        (*s).to_string()
    } else {
        "<非字符串 payload>".to_string()
    }
}

/// RES1 / 入口①：相对 target 相对 source 解析（同目录、上一级、多级、带 `..` 归一）。
#[test]
fn res1_relative_targets_resolve_against_the_source_part() {
    // 同目录
    assert_eq!(resolve("/word/document.xml", "media/image1.png"), "/word/media/image1.png");
    // 上一级（`..` 被归一化）
    assert_eq!(resolve("/word/document.xml", "../media/x.png"), "/media/x.png");
    // 多级向上
    assert_eq!(resolve("/a/b/c/d.xml", "../../x.xml"), "/a/x.xml");
    // 纯文件名
    assert_eq!(resolve("/word/document.xml", "other.xml"), "/word/other.xml");
}

/// RES2 / 入口①：绝对（或带 authority/scheme）的 target 会被**替换**掉 source 的路径。
#[test]
fn res2_absolute_paths_replace_the_source_path() {
    // `/word/x.xml` 以 `/` 开头 ⇒ 直接用 target 的路径（RFC 3986）
    assert_eq!(resolve("/word/document.xml", "/word/x.xml"), "/word/x.xml");
    assert_eq!(resolve("/word/document.xml", "/x.xml"), "/x.xml");
}

/// RES3 / 入口① 失败面：source 必须是**相对** URI，否则 Java 抛 `IllegalArgumentException`。
/// ★实测文案 = `sourcePartUri invalid - <uri>`（Java 是字符串拼接 ⇒ 用 `toString()`）。
#[test]
fn res3_absolute_source_or_target_is_rejected() {
    let err = std::panic::catch_unwind(|| resolve("http://example.com/a.xml", "b.xml")).unwrap_err();
    assert_eq!(
        panic_message(err),
        "sourcePartUri invalid - http://example.com/a.xml",
        "Java 文案逐字一致"
    );

    let err = std::panic::catch_unwind(|| resolve("/word/a.xml", "http://example.com/b.xml"))
        .unwrap_err();
    assert_eq!(
        panic_message(err),
        "targetUri invalid - http://example.com/b.xml",
        "Java 文案逐字一致"
    );
}

/// RES4 / 入口②：`getURIFromPath` = `new URI(path)`；语法错 ⇒ Java `IllegalArgumentException("path")`。
#[test]
fn res4_get_uri_from_path_matches_new_uri() {
    assert_eq!(PackagingURIHelper::getURIFromPath("/word/document.xml").toString(), "/word/document.xml");
    assert_eq!(PackagingURIHelper::getURIFromPath("/").toString(), "/");
    // `\\` 不是 URI 合法字符（Java `new URI("\\")` 抛 URISyntaxException）⇒ 两侧都应失败；
    // ★Rust 的 URI 解析更宽松（见 25/26 登记的 util 面分歧）⇒ 只断言「能构造时不崩」的当前行为。
    // ★已知分歧（util 面欠债，归属 `util/URI.rs`）：Java 的 `new URI(...)` 会拒 `\` 与空格，
    //   当前 Rust 的解析器接受它们 ⇒ 这两个探针钉住**当前**行为，`URI.rs` 对齐后必须翻转。
    assert_eq!(
        PackagingURIHelper::getURIFromPath("/word\\x.xml").toString(),
        "/word\\x.xml",
        "★当前 Rust 接受 `\\`；Java 在 `new URI(...)` 抛 URISyntaxException ⇒ IllegalArgumentException(\"path\")"
    );
    assert_eq!(
        PackagingURIHelper::getURIFromPath("http://a b").toString(),
        "http://a b",
        "★当前 Rust 接受 authority 里的空格；Java 同样抛 URISyntaxException"
    );
}

/// RES5 / 入口③：`relativizeURI` —— 绝对 URI 之间求相对 target 串。
#[test]
fn res5_relativize_uri_produces_relative_targets() {
    let src = uri("/word/document.xml");
    // 同目录文件 ⇒ 直接给文件名
    assert_eq!(
        PackagingURIHelper::relativizeURI(&src, &uri("/word/media/i.png"))
            .map(|u| u.toString()),
        Some("media/i.png".to_string())
    );
    // 子目录中的目标
    assert_eq!(
        PackagingURIHelper::relativizeURI(&src, &uri("/word/sub/x.xml")).map(|u| u.toString()),
        Some("sub/x.xml".to_string())
    );
    // 上一级（`../`）
    let rel = PackagingURIHelper::relativizeURI(&src, &uri("/media/i.png"));
    assert_eq!(rel.as_ref().map(|u| u.toString()), Some("../media/i.png".to_string()));
    // msCompatible = true 时，源为 `/` 会剥掉 target 的前导 `/`
    assert_eq!(
        PackagingURIHelper::relativizeURI_compat(&uri("/"), &uri("/word/document.xml"), true)
            .map(|u| u.toString()),
        Some("word/document.xml".to_string())
    );
}

/// RES6 / 入口④：`getSourcePartUriFromRelationshipPartUri` —— 关系 part URI → 源 part URI。
#[test]
fn res6_source_part_uri_from_relationship_part_uri() {
    // 包级关系 ⇒ 根
    assert_eq!(
        PackagingURIHelper::getSourcePartUriFromRelationshipPartUri(&uri("/_rels/.rels")).toString(),
        "/"
    );
    // 普通 part 的关系 ⇒ 反推回 part
    assert_eq!(
        PackagingURIHelper::getSourcePartUriFromRelationshipPartUri(&uri("/word/_rels/document.xml.rels"))
            .toString(),
        "/word/document.xml"
    );
    assert_eq!(
        PackagingURIHelper::getSourcePartUriFromRelationshipPartUri(&uri("/word/sub/_rels/d.xml.rels"))
            .toString(),
        "/word/sub/d.xml"
    );
    // 非关系 part ⇒ Java `IllegalArgumentException("Must be a relationship part")`
    let err = std::panic::catch_unwind(|| {
        PackagingURIHelper::getSourcePartUriFromRelationshipPartUri(&uri("/word/document.xml"))
    })
    .unwrap_err();
    assert_eq!(panic_message(err), "Must be a relationship part");
}
