// function_id=23_function_part_name_create
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 23_function_part_name_create
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackagingURIHelper.rs
//       libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackagePartName.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/23_function_part_name_create.rs

use core_libs_io::io::opc::PackagePartName::PackagePartName;
use core_libs_io::io::opc::PackagingURIHelper::{package_root_part_name, PackagingURIHelper};
use core_libs_io::io::util::URI::URI;

fn uri(spec: &str) -> URI {
    URI::new(spec).unwrap_or_else(|e| panic!("URI::new({spec:?}) 应成功，实际: {e}"))
}

fn pnc_str(name: &str) -> Result<PackagePartName, String> {
    PackagingURIHelper::createPartName_str(name)
}

/// 取 `Err` 文本（`PackagePartName` 未实现 `Debug` ⇒ 不能用 `expect_err`；Java 侧对应物是 `toString()`）。
fn err_of(res: Result<PackagePartName, String>) -> String {
    match res {
        Ok(v) => panic!("本用例期望失败，实际得到 {}", v.getName()),
        Err(err) => err,
    }
}

fn pnc_uri(u: &URI) -> Result<PackagePartName, String> {
    PackagingURIHelper::createPartName_uri(u)
}

/// PNC1 / PNC-01: String 入口主路径。
#[test]
fn pnc1_create_from_string_main_path() {
    let name = pnc_str("/word/document.xml").expect("C1 的 word/document.xml 必须合法");
    assert_eq!(name.getName(), "/word/document.xml");
    assert_eq!(name.getExtension(), "xml");
    assert_eq!(name.getURI().toString(), "/word/document.xml");
    assert!(!name.isRelationshipPartURI(), "document.xml 不是关系 part");
}

/// PNC2 / PNC-02: URI 入口与 String 入口结果等价。
#[test]
fn pnc2_create_from_uri_matches_string_entry() {
    let a = pnc_str("/word/document.xml").expect("String 入口");
    let b = pnc_uri(&uri("/word/document.xml")).expect("URI 入口");
    assert_eq!(a.getName(), b.getName());
    assert_eq!(a.getExtension(), b.getExtension());
    assert!(a.equals(Some(&b)), "同一 part 名的两个入口必须相等");
    assert!(b.equals(Some(&a)));
}

/// PNC3 / PNC-03: 两个 relative 重载（`../` 归一为绝对 part 名）。
#[test]
fn pnc3_relative_overloads_resolve_against_relative_part() {
    let base = uri("/word/document.xml");

    let by_uri = PackagingURIHelper::createPartName_uri_relative(&uri("../media/image1.png"), &base)
        .expect("URI relative 重载");
    assert_eq!(by_uri.getName(), "/media/image1.png");

    let by_str = PackagingURIHelper::createPartName_str_relative("../media/image1.png", &base)
        .expect("String relative 重载");
    assert_eq!(by_str.getName(), "/media/image1.png");
    assert!(by_uri.equals(Some(&by_str)), "两个 relative 重载应得同一 part 名");

    // 同目录相对名
    let same_dir = PackagingURIHelper::createPartName_uri_relative(&uri("styles.xml"), &base)
        .expect("同目录相对名");
    assert_eq!(same_dir.getName(), "/word/styles.xml");
}

/// PNC4 / PNC-04: **只有 String 入口**归一 `\` ⇒ 归一后通过（Java `resolvePartName` 只做 `\` → `/`）。
/// ★实测更正：`resolvePartName` **不补前导 `/`** ⇒ `"word\\document.xml"` 归一为 `"word/document.xml"` 后被 M1.4 拒
/// （Java 同：`createPartName(String)` 只替换反斜杠）。因此本用例的合法输入必须自带前导 `\`。
#[test]
fn pnc4_string_entry_normalizes_backslash() {
    let name = pnc_str("\\word\\document.xml").expect("前导 \\ 归一为 / 后应合法");
    assert_eq!(name.getName(), "/word/document.xml");
    assert_eq!(name.getExtension(), "xml");

    let multi = pnc_str("\\word\\media\\image1.png").expect("多个 \\ 也应全部归一");
    assert_eq!(multi.getName(), "/word/media/image1.png");

    // 无前导 `/`：归一后仍缺前导斜杠 ⇒ M1.4（Java 同）
    let err = err_of(pnc_str("word\\document.xml"));
    assert!(err.contains("[M1.4]"), "应命中 M1.4，实际: {err}");

    // resolvePartName 本身（`\` → `/`，不做别的）
    assert_eq!(PackagingURIHelper::resolvePartName("a\\b\\\\c"), "a/b//c");
    assert_eq!(PackagingURIHelper::resolvePartName("/word/document.xml"), "/word/document.xml");
    assert_eq!(PackagingURIHelper::resolvePartName("no\\slash"), "no/slash");
}

/// PNC5 / PNC-05: **入口差异**（关键不变量 ②）—— `\` 只在 String 入口被 `resolvePartName` 归一，
/// URI 入口不归一 ⇒ 同一个名字 String 入口 `Ok`、URI 入口 `Err`（M1.6，★实测消息已钉住）。
#[test]
fn pnc5_backslash_differs_between_string_and_uri_entries() {
    // String 入口：`\` → `/` 归一 ⇒ 合法
    let normalized = pnc_str("/word\\document.xml").expect("String 入口归一后应合法");
    assert_eq!(normalized.getName(), "/word/document.xml");

    // URI 入口：不归一 ⇒ 首段含 `\` ⇒ M1.6
    let err = err_of(pnc_uri(&uri("/word\\document.xml")));
    assert!(err.contains("[M1.6]"), "URI 入口应命中 M1.6，实际: {err}");

    // 同一字符串、两个入口 ⇒ 结果必须不同（Java 语义，不是缺陷）
    assert!(pnc_str("/word\\document.xml").is_ok());
    assert!(pnc_uri(&uri("/word\\document.xml")).is_err());
}

/// PNC5b / **Java 缺陷照抄探针**：`checkPCharCompliance(segment, begin, len)` 在 Java 里把**长度当结束下标**用
/// （`for (i = begin; i < len; ++i)`，而调用方传的是 `i - start`）⇒ **第二个及之后的段几乎扫不到**。
/// 结果：`/word/a\b.xml` 的 `\` 在第二段 ⇒ M1.6 **不触发**，URI 入口返回 `Ok` 且名字**保留** `\`；
/// 而 String 入口归一后是另一个名字 ⇒ 「同一输入两个入口得到两个不同 part 名」。
/// Rust 侧照抄同一行为（本用例即防「顺手修好」导致偏离 Java）。
#[test]
fn pnc5b_m1_6_only_scans_first_segment_java_quirk() {
    let via_uri = pnc_uri(&uri("/word/a\\b.xml")).expect("第二段的 \\ 因 Java 缺陷未被扫到 ⇒ 放过");
    assert_eq!(via_uri.getName(), "/word/a\\b.xml", "URI 入口保留原 `\\`");

    let via_str = pnc_str("/word/a\\b.xml").expect("String 入口归一后合法");
    assert_eq!(via_str.getName(), "/word/a/b.xml");

    assert_ne!(
        via_uri.getName(),
        via_str.getName(),
        "同一字符串、两个入口 ⇒ 两个不同的 part 名（Java 同）"
    );
    // 对照：`\` 出现在**首段**时确实被抓（PNC5 已钉 M1.6）
    assert!(pnc_uri(&uri("/word\\document.xml")).is_err());
}

/// PNC6 / PNC-06: M1.3 / M1.9 类非法名必须 `Err`（★实测消息逐条钉住）。
#[test]
fn pnc6_malformed_segment_names_are_rejected() {
    for (name, marker) in [
        ("/word/../evil.xml", "[M1.9]"),
        ("/word/..", "[M1.9]"),
        ("//word//document.xml", "[M1.3]"),
        ("/word//document.xml", "[M1.3]"),
        ("/word/x./y.xml", "[M1.9]"),
    ] {
        let err = err_of(pnc_str(name));
        assert!(err.contains(marker), "{name:?} 应命中 {marker}，实际: {err}");
    }
    // 点段（`/./`）也必须被拒
    assert!(pnc_str("/word/./document.xml").is_err(), "/./ 段必须被拒");
    // 对照（★实测放过）：段**以点开头但含非点字符**是合法的（Java `nonDotCharCount > 0` ⇒ 不报 M1.3）
    assert_eq!(
        pnc_str("/word/.hidden/x.xml").expect(".hidden 段合法").getName(),
        "/word/.hidden/x.xml"
    );
}

/// PNC6b / **已知分歧探针（util 面欠债）**：Java 的 `new URI(...)` 会先拒掉空格与 `<`
/// （`URISyntaxException` ⇒ `InvalidFormatException`），而 Rust `URI::new`（`util/URI.rs`，属 util 面，
/// **本 function 不得改**）接受了它们；随后 Java 的 `WpsUriUtils.toASCIIString` 把空格映射为 `+`（★实测 Java `:67-69`
/// 确有 `if (c == ' ') chars[i] = '+';`，Rust 侧一致）⇒ Rust 得到 `Ok("/word/do+c.xml")`。
/// **本用例断言的是当前实现行为**，不是期望行为；`URI.rs` 对齐 Java 解析器后此用例必须翻转（撤回条件）。
#[test]
fn pnc6b_known_divergence_uri_parser_accepts_illegal_chars() {
    // 空格：Java ⇒ Err；Rust ⇒ Ok（空格被 toASCIIString 映射为 '+'，这一步与 Java 一致）
    let spaced = pnc_str("/word/do c.xml").expect("当前实现：空格未被 URI 解析层拒绝");
    assert_eq!(spaced.getName(), "/word/do+c.xml", "与 Java toASCIIString 的 ' '→'+' 映射一致");
    // `<`：Java ⇒ Err；Rust ⇒ Ok（原样保留）
    let angle = pnc_str("/word/a<b.xml").expect("当前实现：'<' 未被 URI 解析层拒绝");
    assert_eq!(angle.getName(), "/word/a<b.xml");
}

/// PNC6c / **Java 死方法对齐**：`PackagePartName.throwExceptionIfPartNameEndsWithForwardSlashChar`（M1.5）
/// 在 Java 里是 **private 且无调用者**（`throwExceptionIfInvalidPartUri` 不调它）⇒ 尾斜杠**不被拒**，两侧都是 `Ok`。
/// 本用例把这一「看起来该拒却放过」的行为钉住，防止有人「顺手补上 M1.5」而偏离 Java。
#[test]
fn pnc6c_trailing_slash_is_accepted_because_m1_5_is_dead_in_java() {
    let trailing = pnc_str("/word/document.xml/").expect("Java 的 M1.5 是死方法 ⇒ 尾斜杠被放过");
    assert_eq!(trailing.getName(), "/word/document.xml/");
    // Java `getExtension()` 从尾部找 '.' ⇒ 尾斜杠进入扩展名
    assert_eq!(trailing.getExtension(), "xml/");
}

/// PNC7 / PNC-07: 空名 / 根名 ⇒ M1.1（`createPartName_str` 与 URI 入口都拒）。
#[test]
fn pnc7_empty_and_root_names_are_rejected() {
    for name in ["", "/"] {
        assert!(pnc_str(name).is_err(), "工具入口 {name:?} 必须被拒（M1.1）");
    }
    // Java `createPartName(URI)` 同样走校验 ⇒ 根 URI 在**校验模式**下也被拒
    assert!(
        pnc_uri(&uri("/")).is_err(),
        "校验模式下的根 URI 必须被拒（Java M1.1；根 part 名只能走 checkConformance=false 常量）"
    );
}

/// PNC8 / PNC-08: 绝对 URI ⇒ M1.x 拒绝。
#[test]
fn pnc8_absolute_uri_is_rejected() {
    for spec in ["http://example.com/a.xml", "file:///a.xml"] {
        assert!(pnc_str(spec).is_err(), "绝对 URI {spec:?} 必须被拒");
    }
}

/// PNC9 / PNC-09: 4 个重载**共用同一校验路径** ⇒ 同一非法名 4 入口同 `Err`。
#[test]
fn pnc9_all_four_entry_points_share_one_validation_path() {
    let base = uri("/word/document.xml");
    // ★实测校正：①「缺前导 `/`」的相对名经 relative 重载解析成绝对名后**合法**；
    // ② 绝对 target 里的 `..` 会被 `resolvePartUri` **归一掉**（`/word/../x.xml` ⇒ `/x.xml`，Java `URI.resolve` 同）
    // ⇒ 「4 入口同 Err」只能用**解析后仍非法**的名字断言：空段 `/word//x.xml`（M1.3，★实测 4 条路径都被拒）。
    let bad = "/word//x.xml";
    assert!(pnc_str(bad).is_err(), "String 入口");
    assert!(pnc_uri(&uri(bad)).is_err(), "URI 入口");
    assert!(
        PackagingURIHelper::createPartName_str_relative(bad, &base).is_err(),
        "String relative 入口（绝对 target 解析后仍是自身）"
    );
    assert!(
        PackagingURIHelper::createPartName_uri_relative(&uri(bad), &base).is_err(),
        "URI relative 入口"
    );
    // 对照：相对名经解析后合法 ⇒ 两个 relative 重载返回 Ok（Java 同）
    assert!(PackagingURIHelper::createPartName_str_relative("word/evil.xml", &base).is_ok());

    // 合法名：4 条路径都该过，且得同一 part 名
    let good = "/word/ok.xml";
    let a = pnc_str(good).expect("String");
    let b = pnc_uri(&uri(good)).expect("URI");
    let c = PackagingURIHelper::createPartName_str_relative("ok.xml", &uri("/word/document.xml"))
        .expect("String relative");
    let d = PackagingURIHelper::createPartName_uri_relative(&uri("ok.xml"), &uri("/word/document.xml"))
        .expect("URI relative");
    for other in [&b, &c, &d] {
        assert!(a.equals(Some(other)), "4 个入口的合法结果必须相等");
    }
    assert_eq!(c.getName(), "/word/ok.xml");
}

/// PNC10 / PNC-10: 幂等 / 确定性（同一输入两次调用视图全等）。
#[test]
fn pnc10_same_input_yields_identical_views() {
    for name in ["/word/document.xml", "/docProps/core.xml", "/_rels/.rels"] {
        let a = pnc_str(name).expect(name);
        let b = pnc_str(name).expect(name);
        assert_eq!(a.getName(), b.getName(), "{name}");
        assert_eq!(a.getExtension(), b.getExtension(), "{name}");
        assert_eq!(a.getURI().toString(), b.getURI().toString(), "{name}");
        assert_eq!(a.isRelationshipPartURI(), b.isRelationshipPartURI(), "{name}");
    }
}

/// PNC11 / PNC-11: **缺陷探针（已修复）** —— 校验接线后 `partName` 已被写入，
/// `equals` / `hashCode` / `compareTo` 必须正常返回（修复前它们在 `partName: None` 上 `unwrap()` panic）。
#[test]
fn pnc11_value_semantics_no_longer_panic_after_wiring() {
    let mut a = pnc_str("/a.xml").expect("/a.xml");
    let b = pnc_str("/a.xml").expect("/a.xml");
    let c = pnc_str("/b.xml").expect("/b.xml");

    assert!(a.equals(Some(&b)), "同名 ⇒ 相等");
    assert!(!a.equals(Some(&c)), "不同名 ⇒ 不等");
    assert!(!a.equals(None), "null ⇒ false（Java `otherPartName != null && instanceof` 分支）");

    let h1 = a.hashCode();
    let h2 = a.hashCode();
    assert_eq!(h1, h2, "hash 首次计算后缓存（Java `hash != -1`）");

    assert_eq!(a.compareTo(Some(&b)), 0, "同名 compareTo == 0");
    assert_eq!(a.compareTo(None), -1, "Java：otherPartName == null ⇒ -1");
    assert_eq!(a.compareTo(Some(&c)), -1, "'/a.xml' < '/b.xml'");
    assert_eq!(c.compareTo(Some(&a)), 1);

    // 关系 part 判定同时被写入
    let rels = pnc_str("/_rels/.rels").expect("/_rels/.rels");
    assert!(rels.isRelationshipPartURI(), "_rels/.rels 是关系 part");
}

/// PNC12（本 function 新增用例）：Java 静态量 `PACKAGE_ROOT_PART_NAME`（`PackagingURIHelper.java:30/:402`，
/// 全仓唯一 `checkConformance = false` 构造点）—— 必须存在且**绕过**校验（根 URI 在 M1.1 下会被拒）。
#[test]
fn pnc12_package_root_part_name_bypasses_conformance() {
    let root = package_root_part_name();
    assert_eq!(root.getName(), "/", "Java getName 去掉 toASCIIString 的结尾 NUL ⇒ \"/\"");
    assert_eq!(root.getExtension(), "", "根名无扩展名");
    assert_eq!(root.getURI().toString(), "/");
    assert!(!root.isRelationshipPartURI(), "根名不是关系 part");
    // 对照：同一 URI 走**校验模式**必须被拒（PNC7）⇒ 证明它不是走 createPartName 得到的
    assert!(pnc_uri(&uri("/")).is_err(), "校验模式下根 URI 必须被拒");
    // 幂等（Java 是静态字段，只初始化一次）
    assert_eq!(
        package_root_part_name().getName(),
        root.getName()
    );
}



/// 门面：Java `public PackagingURIHelper()`（`PackagingURIHelper.java:32`）⇒ Rust 同名入口存在且与 `new()` 等价。
/// （Java `PackagePartName(URI, boolean)` 是包内可见 ⇒ Rust 对应 `pub(crate) fn PackagePartName::new`，
/// 集成测试不可直接调用，其两个分支分别由 PNC1–PNC10（`checkConformance = true`）与 PNC12（`false`）覆盖。）
#[test]
fn facade_packaging_uri_helper_same_named_constructor_matches_new() {
    let by_java_name = PackagingURIHelper::PackagingURIHelper();
    let by_rust_name = PackagingURIHelper::new();
    // 该类的实例无状态（Java 侧同样是空构造器 `super();`）⇒ 断言两者都能构造、形态一致，
    // 且类级静态方法仍按 Java 的调用形态可用（`getPackageRootUri()` 是 static，Rust 为关联函数）。
    assert_eq!(
        std::mem::size_of_val(&by_java_name),
        std::mem::size_of_val(&by_rust_name)
    );
    assert_eq!(PackagingURIHelper::getPackageRootUri().toString(), "/");
}
