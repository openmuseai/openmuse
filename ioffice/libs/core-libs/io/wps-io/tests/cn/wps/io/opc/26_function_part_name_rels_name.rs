// function_id=26_function_part_name_rels_name
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 26_function_part_name_rels_name
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackagingURIHelper.rs
//       libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackagePartName.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/26_function_part_name_rels_name.rs
//
// Java `PackagingURIHelper.getRelationshipPartName(PackagePartName)`（`:312-363`）：
//   ① `null` ⇒ IllegalArgumentException（Rust 以 `&PackagePartName` 表达，不可为 None）
//   ② `getPath() == "/"`（根 part）⇒ 直接返回**静态常量** `PACKAGE_RELATIONSHIPS_ROOT_PART_NAME`
//   ③ 已是关系 part ⇒ InvalidOperationException("Can't be a relationship part")
//   ④ 否则拼 `/<dir>/_rels/<file>.rels`：char 数组长度 = lenFullPath + 12，
//      把文件名右移 6 位插入 `_rels/`，再从 `lenFullPath + 6` 处写 `.rels`；
//      ★注意 `--lenPartName`（末尾那个 NUL 不计入），以及 String 入口的
//      `createPartName(new String(partname, 0, lenPartName))` —— 失败时 **catch ⇒ 返回 null**。
//   ⑤ 无 `/` 的路径（`i < 0`）⇒ 接 `PACKAGE_RELATIONSHIPS_ROOT_URI.getRawPath()`（raw path）。

use core_libs_io::io::opc::PackagePartName::PackagePartName;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;
// 类静态量在 Rust 里是**模块级**函数（与 `package_root_uri()` 同一约定）
use core_libs_io::io::opc::PackagingURIHelper::{
    package_relationships_root_part_name, package_root_part_name,
};

fn name_of(spec: &str) -> PackagePartName {
    PackagingURIHelper::createPartName_str(spec)
        .unwrap_or_else(|e| panic!("{spec:?} 应为合法 part 名，实际: {e}"))
}

fn rels_of(spec: &str) -> Option<String> {
    PackagingURIHelper::getRelationshipPartName(&name_of(spec)).map(|n| n.getName())
}

/// GRN1 / 入口②（根分支）：根 part `/` ⇒ 静态常量 `PACKAGE_RELATIONSHIPS_ROOT_PART_NAME`。
/// 根 part 名只能经 `checkConformance = false` 的 `package_root_part_name()` 取得（校验模式会拒 `/`）。
#[test]
fn grn1_root_part_maps_to_the_relationships_root_constant() {
    let root = package_root_part_name();
    let rels = PackagingURIHelper::getRelationshipPartName(root).expect("根 part 必有关系 part 名");
    assert_eq!(rels.getName(), "/_rels/.rels");
    assert!(rels.isRelationshipPartURI(), "结果必须是关系 part");
    // Java 返回的是**静态常量**本身（不是每次新建）⇒ Rust 门面同样提供该常量
    let constant = package_relationships_root_part_name();
    assert_eq!(constant.getName(), "/_rels/.rels");
    assert!(rels.equals(Some(constant)), "返回的应是与静态常量等值的对象");
}

/// GRN2 / 入口④：普通 part 名 ⇒ 自己目录下的 `_rels/<file>.rels`（顶层、单层、多层、带点目录名）。
#[test]
fn grn2_normal_parts_map_into_their_own_rels_folder() {
    assert_eq!(rels_of("/a.xml").as_deref(), Some("/_rels/a.xml.rels"));
    assert_eq!(rels_of("/1.xml").as_deref(), Some("/_rels/1.xml.rels"));
    assert_eq!(
        rels_of("/word/document.xml").as_deref(),
        Some("/word/_rels/document.xml.rels")
    );
    assert_eq!(
        rels_of("/word/sub/doc.xml").as_deref(),
        Some("/word/sub/_rels/doc.xml.rels")
    );
    assert_eq!(
        rels_of("/word/x.y/z.xml").as_deref(),
        Some("/word/x.y/_rels/z.xml.rels")
    );
    // 无扩展名的 part 同样按「目录 + _rels/ + 文件名 + .rels」拼
    assert_eq!(rels_of("/word/media").as_deref(), Some("/word/_rels/media.rels"));
}

/// GRN3 / 入口③：已是关系 part ⇒ Java 抛 `InvalidOperationException` ⇒ Rust panic（不得返回 None）。
#[test]
fn grn3_relationship_part_input_panics() {
    for spec in ["/word/_rels/document.xml.rels", "/_rels/.rels"] {
        let part = name_of(spec);
        let panicked = std::panic::catch_unwind(move || {
            let _ = PackagingURIHelper::getRelationshipPartName(&part);
        })
        .is_err();
        assert!(
            panicked,
            "{spec:?} 已是关系 part ⇒ 必须 panic（Java InvalidOperationException）"
        );
    }
}

/// GRN4 / 不变量：结果必为关系 part（两套判定都为 true），且名字以 `.rels` 结尾。
#[test]
fn grn4_every_result_is_a_relationship_part() {
    for spec in ["/a.xml", "/word/document.xml", "/word/sub/doc.xml", "/word/x.y/z.xml"] {
        let mapped = PackagingURIHelper::getRelationshipPartName(&name_of(spec)).expect("应有结果");
        assert!(mapped.isRelationshipPartURI(), "{spec:?} 映射结果应为关系 part");
        assert!(
            PackagingURIHelper::isRelationshipPartURI(mapped.getURI()),
            "{spec:?} 映射结果应通过正则口径"
        );
        assert!(mapped.getName().ends_with(".rels"), "{spec:?}");
    }
}

/// GRN5 / 非幂等：把关系 part 名再映射一次 ⇒ Java 抛 `InvalidOperationException` ⇒ panic。
#[test]
fn grn5_mapping_a_mapped_name_panics() {
    let once =
        PackagingURIHelper::getRelationshipPartName(&name_of("/word/document.xml")).expect("应有结果");
    let panicked = std::panic::catch_unwind(move || {
        let _ = PackagingURIHelper::getRelationshipPartName(&once);
    })
    .is_err();
    assert!(panicked, "关系 part 名不能再映射（Java `:321-322`）");
}

/// GRN6 / 入口④ 的失败面：拼出的名字若被 `createPartName` 拒 ⇒ Java `catch ⇒ null` ⇒ Rust `None`。
/// ★实测（与 Java 同因）：`/word/a%20b.xml` 会拼出 `/word/_rels/a%20b.xml.rels`，
/// 而该名字的**最后一段前 2 个字符**（`a`,`%`）正好落在 `checkPCharCompliance` 的扫描窗口内
/// （Java 把「长度当结束下标」⇒ 只扫段首几字符）⇒ `%` 后不足两位 ⇒ `InvalidFormatException` ⇒ null。
#[test]
fn grn6_unmappable_percent_encoded_name_returns_none_like_java() {
    assert_eq!(rels_of("/word/a%20b.xml"), None, "Java 同样返回 null（扫描窗口缺陷）");
    // 底层原因可复现：直接构造该名字即被拒
    let err = PackagingURIHelper::createPartName_str("/word/_rels/a%20b.xml.rels")
        .err()
        .expect("该名字应被拒");
    assert!(err.contains("invalid encoded character"), "{err}");
    // ★对照（同一缺陷的另一面）：`/a%20b.xml` ⇒ 名字为 `/_rels/a%20b.xml.rels`，其**最后一段**
    // 的扫描窗口是 `[start, len-start)`（start=7、len=14 ⇒ 只扫 7..13），`%` 落在窗口里且
    // `len - i = 6 >= 2` ⇒ 走**解码**分支（`%20` → 空格，`ASCII_FLAG_TABLE[' '] = 0` ⇒ 不算 M1.8）
    // ⇒ 不会被拒，映射**成功**。⇒ 「能否映射」取决于 `%` 落在窗口的哪个位置，而不是名字本身合不合法。
    assert_eq!(
        rels_of("/a%20b.xml").as_deref(),
        Some("/_rels/a%20b.xml.rels"),
        "同一 `%20` 在更浅的路径上反而能映射成功（扫描窗口更短）"
    );
}

/// GRN7 / 确定性：同一输入两次映射结果相同。
#[test]
fn grn7_mapping_is_deterministic() {
    for spec in ["/a.xml", "/word/document.xml", "/word/sub/doc.xml"] {
        assert_eq!(rels_of(spec), rels_of(spec), "{spec:?}");
    }
}

/// GRN8 / **已知分歧（util 面欠债，现归属 `util/WpsUriUtils.rs`）**：非 ASCII part 名。
/// Java：`new URI("/word/文档.xml")` 正常，`getPath()` 返回 CJK 路径，`String.length()` 按**字符**计数
/// ⇒ 映射得到 `/word/_rels/文档.xml.rels`。
/// `URI.parseURI` 的首个字节/字符索引 panic 已修；当前仍在
/// `WpsUriUtils.rs:112` 的 UTF-8 字节切片处 panic。
/// ⇒ 本 function 已把自身的 `lenFullPath` 改成 `chars().count()`；
/// 该 util 方法按 Java 字符索引修好后，本断言必须翻转。
#[test]
fn grn8_known_divergence_non_ascii_owned_by_uri_util() {
    let panicked = std::panic::catch_unwind(|| {
        let _ = PackagingURIHelper::createPartName_str("/word/文档.xml");
    })
    .is_err();
    assert!(
        panicked,
        "★当前 Rust：非 ASCII part 名在 WpsUriUtils 字节切片处 panic（Java 正常）"
    );
}
