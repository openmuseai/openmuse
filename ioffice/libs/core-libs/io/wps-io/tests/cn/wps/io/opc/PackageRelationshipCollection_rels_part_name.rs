// function_id=36_function_rels_rels_part_name
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 36_function_rels_rels_part_name
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackageRelationshipCollection.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/PackageRelationshipCollection_rels_part_name.rs
//
// 覆盖 Java `private static PackagePartName getRelationshipPartName(PackagePart part)`（`:68-78`）：
//   part == null ⇒ PackagingURIHelper.PACKAGE_ROOT_PART_NAME（"/"）→ PackagingURIHelper.getRelationshipPartName ⇒ "/_rels/.rels"
//   part != null ⇒ part.getPartName() → 同上 ⇒ "/<dir>/_rels/<name>.rels"
// ★本轮（2026-09-18）把该助手从构造器里的内联分支**抽成同名 Rust 函数**（门面/结构对齐），构造器改为调用它。

use core_libs_io::io::opc::PackagePart::PackagePart;
use core_libs_io::io::opc::PackagePartName::PackagePartName;
use core_libs_io::io::opc::PackageRelationshipCollection::PackageRelationshipCollection;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;

const MAIN_CT: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const RELS_CT: &str = "application/vnd.openxmlformats-package.relationships+xml";

fn name(s: &str) -> PackagePartName {
    PackagingURIHelper::createPartName_str(s).unwrap()
}

/// 造一个「名字 = `part_name`」的自有 part（`_container = None`；本助手只读 `getPartName()`）。
fn part_named(part_name: &str) -> PackagePart {
    PackagePart::new_with_string(None, name(part_name), MAIN_CT)
}

fn rels_name_of(part_name: &str) -> String {
    let p = part_named(part_name);
    PackageRelationshipCollection::getRelationshipPartName(Some(&p))
        .unwrap_or_else(|| panic!("{part_name} 的关系 part 名"))
        .getName()
        .to_string()
}

fn panic_message(err: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&'static str>() {
        (*s).to_string()
    } else {
        "<非字符串 payload>".to_string()
    }
}

/// N36-1 / 主路径（包级）：`part == null` ⇒ `/_rels/.rels`。
#[test]
fn n36_1_null_part_maps_to_package_relationships_root() {
    let got = PackageRelationshipCollection::getRelationshipPartName(None)
        .expect("包级关系 part 名")
        .getName()
        .to_string();
    assert_eq!(got, "/_rels/.rels");
}

/// N36-2 / 边界：根 part 名 `"/"` 与 `part == null` **同结果**（Java 两条分支都落到
/// `PackagingURIHelper.getRelationshipPartName` 的 `fullPath == "/"` 分支）。
/// ★注意：`createPartName("/")` 在 Rust 会因 M1.1 拒绝 ⇒ 必须走 Java 的 `PACKAGE_ROOT_PART_NAME`
/// （Rust = `PackagingURIHelper::package_root_part_name()`，即 `PackagePartName::new(uri, false)` 的同义写法）。
#[test]
fn n36_2_root_part_name_maps_to_the_same_root_rels() {
    let root = core_libs_io::io::opc::PackagingURIHelper::package_root_part_name().clone();
    assert_eq!(root.getName(), "/");
    let p = PackagePart::new_with_string(None, root, MAIN_CT);
    let got = PackageRelationshipCollection::getRelationshipPartName(Some(&p))
        .expect("根 part 的关系 part 名")
        .getName()
        .to_string();
    assert_eq!(got, "/_rels/.rels");
}

/// N36-3 / 主路径：`/word/document.xml` ⇒ `/word/_rels/document.xml.rels`。
#[test]
fn n36_3_document_part_name() {
    assert_eq!(rels_name_of("/word/document.xml"), "/word/_rels/document.xml.rels");
}

/// N36-4 / 边界：根下无目录的 part `/foo` ⇒ `/_rels/foo.rels`。
#[test]
fn n36_4_root_level_part_name() {
    assert_eq!(rels_name_of("/foo"), "/_rels/foo.rels");
}

/// N36-8 / 边界：深目录 `/a/b/c.xml` ⇒ `/a/b/_rels/c.xml.rels`。
#[test]
fn n36_8_deep_directory_part_name() {
    assert_eq!(rels_name_of("/a/b/c.xml"), "/a/b/_rels/c.xml.rels");
    assert_eq!(rels_name_of("/word/media/image1.png"), "/word/media/_rels/image1.png.rels");
}

/// N36-5 / 失败：入参本身是关系 part ⇒ Java `PackagingURIHelper.getRelationshipPartName` 抛
/// `InvalidOperationException("Can't be a relationship part")`（Rust 契约见 §1.7.1 ②：实测现状）。
#[test]
fn n36_5_relationship_part_input_is_rejected() {
    let p = PackagePart::new_with_string(None, name("/word/_rels/document.xml.rels"), RELS_CT);
    assert!(p.isRelationshipPart());
    let err = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        PackageRelationshipCollection::getRelationshipPartName(Some(&p)).map(|n| n.getName().to_string());
    }))
    .unwrap_err();
    let msg = panic_message(err);
    assert!(
        msg.contains("Can't be a relationship part") || msg.contains("relationship part"),
        "★实测 payload = {msg:?}（Java: InvalidOperationException(\"Can't be a relationship part\")）"
    );
}

/// N36-6 / 确定性：纯函数 ⇒ 同入参多次调用结果相同（含 null 分支）。
#[test]
fn n36_6_is_pure_and_repeatable() {
    let a = PackageRelationshipCollection::getRelationshipPartName(None).unwrap().getName().to_string();
    let b = PackageRelationshipCollection::getRelationshipPartName(None).unwrap().getName().to_string();
    assert_eq!(a, b);
    assert_eq!(rels_name_of("/word/document.xml"), rels_name_of("/word/document.xml"));
}

/// N36-7 / 登记性：`new_with_part(part)`（Java `PackageRelationshipCollection(PackagePart)` =
/// `this(part._container, part)`）在 `part._container == null` 时的两侧差异：
/// Java 解引用 `part._container` ⇒ **NPE**；Rust 传 `None` 容器 ⇒ `panic!("container")`（同为止损，文案不同源）。
#[test]
fn n36_7_part_ctor_without_container_fails_on_both_sides() {
    let p = part_named("/word/document.xml");
    let err = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // ★2026-09-20：`new_with_part` 的形参由 `Option<Box<dyn Any>>` 改为 Java 声明的
        //   真实类型别名 `Option<*const PackagePart>`（Java `this(part._container, part)` 不搬走对象）
        PackageRelationshipCollection::new_with_part(Some(&p as *const PackagePart));
    }))
    .unwrap_err();
    assert_eq!(panic_message(err), "container", "★Rust 文案；Java 侧为 NPE（§1.7.1 ③）");
}
