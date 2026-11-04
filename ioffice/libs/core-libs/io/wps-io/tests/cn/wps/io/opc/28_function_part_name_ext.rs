// function_id=28_function_part_name_ext
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 28_function_part_name_ext
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackagePartName.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/28_function_part_name_ext.rs
//
// 本 function = `PackagePartName` 的**派生视图 + 值语义**（Java `PackagePartName.java`）：
//   ① `getExtension()`（`:?`）：从 `partNameURI.getPath()` **从后往前**找第一个 `.`，返回其后子串；无 `.` ⇒ `""`
//   ② `getName()`：`new String(WpsUriUtils.toASCIIString(partNameURI.toString()), 0, len - 1)`（丢末尾 NUL）
//   ③ `toString()`：`return this.getName();`
//   ④ `getURI()`：`return this.partNameURI;`
//   ⑤ `equals(Object)`：逐字符比 `partName`（不含末尾 NUL），类型不同/`null` ⇒ false
//   ⑥ `hashCode()`：`hash == -1` 时算一次并缓存
//   ⑦ `compareTo(PackagePartName)`：`null` ⇒ `-1`；否则逐字符比 `partName`（**含**末尾 NUL），全等 ⇒ `0`
// ★值语义的 Java 原貌：`equals`/`hashCode`/`compareTo` 都直接读 `this.partName`，而该字段**只在
//   `checkConformance = true` 的构造分支里被赋值**；`PACKAGE_ROOT_PART_NAME`（`checkConformance = false`）
//   的 `partName` 在 Java 里是 `null` ⇒ 三者对该对象都抛 NPE。Rust 用 `unwrap()` 表达同一「不可用」状态。

use core_libs_io::io::opc::PackagePartName::PackagePartName;
use core_libs_io::io::opc::PackagingURIHelper::{package_root_part_name, PackagingURIHelper};
use std::collections::HashMap;

fn part(spec: &str) -> PackagePartName {
    PackagingURIHelper::createPartName_str(spec).unwrap_or_else(|e| panic!("{spec:?} 应合法，实际: {e}"))
}

/// EXT1 / 入口①：`getExtension()` = 路径里**最后一个** `.` 之后的子串；无 `.` ⇒ `""`。
#[test]
fn ext1_get_extension_scans_backwards_for_the_last_dot() {
    // 常规
    assert_eq!(part("/word/document.xml").getExtension(), "xml");
    // ★实测（= Java 原貌）：`getExtension()` 读的是 `partNameURI.getPath()`，而 Java 的
    //   `throwExceptionIfInvalidPartUri` **只**小写化 `partName` 字段、**不改写** `partNameURI`
    //   ⇒ 扩展名**保留原大小写**（`getName()` 同样保留）。
    assert_eq!(part("/word/document.XML").getExtension(), "XML", "大小写保留（partNameURI 未被改写）");
    // 多扩展名 ⇒ 取最后一个点之后
    assert_eq!(part("/word/a.tar.gz").getExtension(), "gz");
    assert_eq!(part("/a.b.c").getExtension(), "c");
    // ★「点后为空 ⇒ `""`」这条分支**经工厂不可达**：`/x.` 在构造期就被 M1.9
    //   （`A segment shall not end with a dot ('.') character [M1.9]`）拒掉 ⇒ Java 同样不可达；
    //   只有 `checkConformance = false` 的对象（根 part，路径 `/` 无点）能绕过工厂。
    assert!(PackagingURIHelper::createPartName_str("/x.").is_err(), "M1.9 挡住尾点");
    // 无点 ⇒ 空串
    assert_eq!(part("/word/media").getExtension(), "");
    assert_eq!(part("/word").getExtension(), "");
    // ★尾随 `/`：Java `getPath()` 保留它 ⇒ 最后一个点之后是 `xml/`（不是 `xml`）
    assert_eq!(part("/word/document.xml/").getExtension(), "xml/");
    // ★实测（= Java 原貌）：`getExtension()` 不看 `/`，只是「最后一个 `.` 之后的**整个**子串」
    //   ⇒ `/word/x.y/z` 的点在**目录名**里，结果却是 `y/z`（跨了斜杠）。Java 逐字相同。
    assert_eq!(part("/word/x.y/z").getExtension(), "y/z", "跨斜杠：只看最后一个点");
}

/// EXT2 / 入口②③：`getName()` / `toString()`；`toString()` 恒等于 `getName()`（Java `:return this.getName()`）。
#[test]
fn ext2_name_view_is_ascii_normalized_and_to_string_forwards() {
    for spec in ["/word/document.xml", "/a.xml", "/word/sub/doc.xml", "/word/x.y/z"] {
        let p = part(spec);
        assert_eq!(p.getName(), spec, "合法名字的 `getName()` = 规范化后的名字");
        assert_eq!(p.toString(), p.getName(), "Java `toString()` 就是 `getName()`");
    }
    // ★实测（= Java 原貌）：`getName()` 由 `partNameURI.toString()` 派生，而构造期**未改写** `partNameURI`
    //   ⇒ **保留原大小写**；被小写化的只是 `partName` 字段（`equals`/`hashCode`/`compareTo` 用的那个）。
    assert_eq!(part("/Word/Document.XML").getName(), "/Word/Document.XML", "大小写保留");
    assert!(
        part("/Word/Document.XML").equals(Some(&part("/word/document.xml"))),
        "值语义按小写化的 partName ⇒ 大小写不同的同名 part 视为同一个"
    );
    // `getURI()` 是同一个对象视图（Java 返回字段本身）
    assert_eq!(part("/a.xml").getURI().toString(), "/a.xml");
}

/// EXT3 / 入口⑤⑥⑦ 的正例：同名对象的值语义自洽（相等、同哈希、比较为 0），可作 `HashMap` 键。
#[test]
fn ext3_equal_names_are_interchangeable_values() {
    let a = part("/word/document.xml");
    let b = part("/word/document.xml");
    assert!(a.equals(Some(&b)) && b.equals(Some(&a)));
    assert!(a == b, "Rust `PartialEq` 与 Java `equals` 同义");
    let (mut ha, mut hb) = (a.clone(), b.clone());
    assert_eq!(ha.hashCode(), hb.hashCode(), "同名的 `hashCode` 相同");
    assert_ne!(ha.hashCode(), -1);
    assert_eq!(a.compareTo(Some(&b)), 0);
    assert_eq!(a.compareTo(None), -1, "Java `compareTo(null)` ⇒ -1");
    // `HashMap` 键语义：两个等价对象只占一个键
    let mut map: HashMap<PackagePartName, u32> = HashMap::new();
    map.insert(a, 1);
    map.insert(b, 2);
    assert_eq!(map.len(), 1, "值语义正确 ⇒ 等价对象是同一个键");
    assert_eq!(map.get(&part("/word/document.xml")).copied(), Some(2), "后写覆盖前写");
}

/// EXT4 / 入口⑤：`equals` 的否定面（不同名 / `None` / 大小写不同经构造后同名）。
#[test]
fn ext4_equals_rejects_other_names_and_null() {
    let a = part("/word/document.xml");
    assert!(!a.equals(Some(&part("/word/other.xml"))));
    assert!(!a.equals(None), "Java `equals(null)` ⇒ false");
    // 长度不同 ⇒ 直接 false（Java 先比 `len != charsOther.length - 1`）
    assert!(!a.equals(Some(&part("/a"))));
    // 构造期小写化 ⇒ 大小写不同在**构造后**已同名
    assert!(a.equals(Some(&part("/WORD/DOCUMENT.XML"))));
}

/// EXT5 / 入口⑥：`hashCode()` 缓存（`hash != -1` 时直接返回），且与 `equals` 相容。
#[test]
fn ext5_hashcode_is_cached_and_consistent_with_equals() {
    let mut a = part("/word/document.xml");
    let mut other = part("/word/document.xml");
    let first = a.hashCode();
    assert_eq!(a.hashCode(), first, "第二次走缓存分支");
    assert_ne!(first, -1, "缓存哨兵值不得泄漏为真实哈希");
    // 不同名 ⇒ 哈希不同（不是 Java 的保证，但本实现下应成立；仅作弱断言，值语义的真实保证是「相等 ⇒ 同哈希」）
    let h_other = other.hashCode();
    assert_eq!(h_other, first);
    let mut c = part("/word/other.xml");
    assert_ne!(c.hashCode(), first);
}

/// EXT6 / 入口⑦：`compareTo` 逐字符比较（**含**末尾 NUL ⇒ 前缀关系恒为 `-1`）。
#[test]
fn ext6_compare_to_is_lexicographic_over_the_part_name() {
    let a = part("/a.xml");
    let b = part("/b.xml");
    assert_eq!(a.compareTo(Some(&b)), -1);
    assert_eq!(b.compareTo(Some(&a)), 1);
    // ★Java 比的是含 NUL 的字符数组 ⇒ `/a` 的第 3 个字符是 NUL(0)，`/a.xml` 的第 3 个是 `.`(46) ⇒ -1
    assert_eq!(part("/a").compareTo(Some(&part("/a.xml"))), -1);
    // 完全同名 ⇒ 0
    assert_eq!(a.compareTo(Some(&part("/a.xml"))), 0);
    // ★`compareTo` 与 `equals` 的**不一致**（Java 原样）：前缀关系下两者可以「不相等但比较为…」——这里比较是 -1，
    //   而 `/a` 与 `/a.xml` 也 `!equals`；真正的分歧在**末尾 NUL 参与比较** ⇒ 前缀永远小于全体。
    assert!(!part("/a").equals(Some(&part("/a.xml"))));
}

/// EXT7 / ★Java 原貌（**纠正文档旧结论**）：根 part（`checkConformance = false`）在 Java 里
/// `this.partName = new char[2]; this.partName[0] = '/';` ⇒ `partName == ['/','\0']`，**不是 null**
/// ⇒ `equals`/`hashCode`/`compareTo` **照常可用**，Rust 侧同样不 panic（旧文档的「必 panic」是把
/// 「`partName` 只在 checkConformance 分支赋值」误读成「false 分支不赋值」）。
#[test]
fn ext7_root_part_has_full_view_and_value_semantics() {
    let root = package_root_part_name();
    // 视图
    assert_eq!(root.getName(), "/");
    assert_eq!(root.getExtension(), "", "`/` 无扩展名");
    assert_eq!(root.getURI().toString(), "/");
    // 值语义可用（`partName` = `['/','\0']`）
    assert!(root.equals(Some(&package_root_part_name())), "根 part 与自己相等");
    assert_eq!(root.compareTo(Some(&package_root_part_name())), 0);
    assert_eq!(root.compareTo(None), -1);
    let mut cloned = root.clone();
    let h = cloned.hashCode();
    assert_ne!(h, -1);
    assert_eq!(cloned.hashCode(), h, "缓存后稳定");
    // 根 part（`['/','\0']`）与普通 part 的长度不同 ⇒ 不相等；比较也因 NUL(0) < 任意 pchar 而更小
    assert!(!root.equals(Some(&part("/a.xml"))));
    assert_eq!(root.compareTo(Some(&part("/a.xml"))), -1, "末尾 NUL 参与比较 ⇒ 前缀恒小");
}

/// EXT8 / ★**已知分歧登记**：`hashCode()` 的**算法**不同（Java `String.hashCode()` 多项式 vs Rust `DefaultHasher`/SipHash）。
/// 影响面：该值不是 dump 字段，也没有跨语言比较点 ⇒ 只登记不改算法；两侧共享的保证是
/// 「`equals` ⇒ 同哈希」（EXT3 已测）。本用例钉住「值不同」这一事实，避免日后误以为已对齐。
#[test]
fn ext8_hashcode_algorithm_differs_from_java_string_hashcode() {
    let name = "/word/document.xml";
    let mut p = part(name);
    // Java `String.hashCode()` = s[0]*31^(n-1) + … + s[n-1]（int 溢出）
    let java_hash = name
        .chars()
        .fold(0i32, |acc, c| acc.wrapping_mul(31).wrapping_add(c as i32));
    println!("P28 rust hashCode = {}, java String.hashCode = {}", p.hashCode(), java_hash);
    assert_ne!(
        p.hashCode(),
        java_hash,
        "★若哪天相等，说明 Rust 侧改成了 Java 多项式 ⇒ 本登记与 §1.7.1 必须更新"
    );
    // 证据 2：根 part（`partName == ['/','\0']` ⇒ Java `"/".hashCode() == 47`）同样不是 Java 值
    let mut root = package_root_part_name().clone();
    assert_ne!(root.hashCode(), 47, "★同一约定：`PackagePartName::hashCode` 全类走 Rust 哈希");
    // 证据 3：`URI::hashCode`（本仓既有实现，`util/URI.rs:949`）也是 Rust 哈希 ⇒ 这不是 28 独有的偏差，
    //   而是**跨文件约定**（`URI.rs` / `StringIntMap.rs` 同形）；真正的影响面是「依赖桶序」的场景。
    let mut uri = core_libs_io::io::util::URI::URI::new("/word/document.xml").expect("URI");
    let java_uri_hash = "/word/document.xml"
        .chars()
        .fold(0i32, |acc, c| acc.wrapping_mul(31).wrapping_add(c as i32));
    assert_ne!(
        uri.hashCode(),
        java_uri_hash,
        "★`URI::hashCode` 同为 Rust 哈希 ⇒ 归属跨文件约定，本 function 只登记"
    );
}
