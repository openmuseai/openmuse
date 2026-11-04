// function_id=35_function_rels_by_index
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 35_function_rels_by_index
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackageRelationshipCollection.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/PackageRelationshipCollection_by_index.rs
//
// 覆盖 `getRelationship(int index)`（Java `:120-136`）的**边界语义**：
//   Java: if (index >= 0 && index <= size()) { 循环找 i==index；找不到 ⇒ return retRel(=null) } else throw IAE("index")
//   ⇒ ★`index == size()` 返回 **null**（不抛）；`index > size()` 才抛。
//   Rust 改前 `index >= len` 就 `panic!("index")` ⇒ 空集合上 `getRelationship(0)`（**ZipHelper 的核心形态**）会炸；
//   本轮（2026-09-18）对齐为 Java 语义：`index == size()` ⇒ `None`。

use core_libs_io::io::opc::PackageRelationshipCollection::PackageRelationshipCollection;
use core_libs_io::io::opc::TargetMode::TargetMode;
use core_libs_io::io::util::URI::URI;

const IMAGE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
const OFFICE_DOC: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";

fn uri(s: &str) -> URI {
    URI::new(s).unwrap()
}

fn coll_of(n: usize) -> PackageRelationshipCollection {
    let mut c = PackageRelationshipCollection::new();
    for i in 0..n {
        c.addRelationship(
            uri(&format!("media/i{}.png", i + 1)),
            TargetMode::INTERNAL,
            IMAGE,
            Some(&format!("rId{}", i + 1)),
        );
    }
    c
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

/// I35-1 / 主路径：`size()==1` + `index 0` ⇒ 命中（C1 形态）。
#[test]
fn i35_1_single_relationship_by_index_zero() {
    let c = coll_of(1);
    assert_eq!(c.size(), 1);
    let rel = c.getRelationship(0).expect("index 0 必须命中");
    assert_eq!(rel.getId(), "rId1");
    assert_eq!(rel.getTargetURI().toString(), "/media/i1.png");
}

/// I35-2 / ★边界（本轮修复点）：`size()==0` + `index 0` ⇒ `None`（Java `null`），**不得 panic**。
#[test]
fn i35_2_empty_collection_index_zero_is_none() {
    let c = PackageRelationshipCollection::new();
    assert_eq!(c.size(), 0);
    assert!(c.getRelationship(0).is_none(), "★Java: 0 <= size() ⇒ 循环取不到 ⇒ 返回 null");
}

/// I35-3 / 边界：`size()==3` ⇒ `index 2` 命中；`index 3`（== size）⇒ `None`。
#[test]
fn i35_3_boundary_between_last_and_size() {
    let c = coll_of(3);
    assert!(c.getRelationship(2).is_some(), "index 2 是最后一条");
    assert!(c.getRelationship(3).is_none(), "★index == size ⇒ None（Java null）");
}

/// I35-4 / 失败：`index > size()` ⇒ Java `IllegalArgumentException("index")`（Rust `panic!("index")`）。
#[test]
fn i35_4_index_beyond_size_panics() {
    let c = coll_of(1);
    let err = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        c.getRelationship(5).map(|r| r.getId().to_string());
    }))
    .unwrap_err();
    assert_eq!(panic_message(err), "index");
}

/// I35-5 / 登记性：Java 的 `index < 0` 分支在 Rust 不可表达（签名 `usize`）⇒ 用 `usize::MAX` 表达「远越界」，
/// 行为与 Java 的越界一致（抛 `IllegalArgumentException("index")`）。负数入参的**类型级**差异登记在 §1.7.1 ②。
#[test]
fn i35_5_negative_index_is_unrepresentable_and_far_overflow_panics() {
    let c = coll_of(2);
    let err = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        c.getRelationship(usize::MAX).map(|r| r.getId().to_string());
    }))
    .unwrap_err();
    assert_eq!(panic_message(err), "index");
}

/// I35-6 / 确定性（★实测并登记）：两个集合用**同一插入序列**建出后，`index 0..size-1` 的 id 序列是否一致。
/// Java 的 `HashMap` 迭代顺序由键哈希决定（同插入序列 ⇒ 同顺序）；Rust 的 `HashMap` 每实例带随机种子。
/// ★本用例断言**集合层面**的确定性（索引全遍历拿到的 id 集合一致），顺序本身登记为「不构成契约」（§1.7.1 ③）。
#[test]
fn i35_6_index_walk_covers_all_entries() {
    let a = coll_of(4);
    let b = coll_of(4);
    let mut ids_a: Vec<String> = (0..a.size()).map(|i| a.getRelationship(i).unwrap().getId().to_string()).collect();
    let mut ids_b: Vec<String> = (0..b.size()).map(|i| b.getRelationship(i).unwrap().getId().to_string()).collect();
    ids_a.sort();
    ids_b.sort();
    assert_eq!(ids_a, vec!["rId1", "rId2", "rId3", "rId4"]);
    assert_eq!(ids_a, ids_b, "★索引全遍历必须覆盖同一集合（顺序本身不作契约）");
}

/// I35-7 / 顺序契约（产品用法 `IOHelper.java:1447-1448`：`for (i = 0; i < size(); i++) getRelationship(i)`）：
/// **不能有洞** —— 每个 `i in 0..size()` 都必须拿到 `Some`（Java/Rust 两侧同）。
#[test]
fn i35_7_full_index_walk_has_no_holes() {
    let c = coll_of(5);
    for i in 0..c.size() {
        assert!(c.getRelationship(i).is_some(), "★index {i} 不应为空（size={}）", c.size());
    }
    assert!(c.getRelationship(c.size()).is_none());
}

/// I35-8 / ★边界（`ZipHelper.getOPCNameFromZipItemName` 形态）：**过滤后的空集合** + `index 0` ⇒ `None` 而非 panic。
/// Java: `pkg.getRelationshipsByType(type).getRelationship(0)` 随后判 `!= null`。
#[test]
fn i35_8_filtered_empty_collection_index_zero_is_none() {
    let c = coll_of(2);
    let filtered = c.getRelationships(Some(OFFICE_DOC));
    assert_eq!(filtered.size(), 0, "该 type 无匹配 ⇒ 子集合为空");
    assert!(filtered.getRelationship(0).is_none(), "★ZipHelper 形态：必须 None，不得 panic");
    let filtered_some = c.getRelationships(Some(IMAGE));
    assert_eq!(filtered_some.size(), 2);
    assert!(filtered_some.getRelationship(1).is_some());
    assert!(filtered_some.getRelationship(2).is_none());
}

/// I35-9 / 回归门（D9）：越界请求产生的是**同一个** `panic!("index")`，且不影响后续正常取值
/// （Java 抛异常同样不改变集合状态）。
#[test]
fn i35_9_out_of_range_does_not_disturb_the_collection() {
    let c = coll_of(2);
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        c.getRelationship(99).map(|r| r.getId().to_string());
    }));
    assert_eq!(c.size(), 2);
    // ★不依赖索引顺序（两侧 `HashMap` 迭代顺序都不构成契约，§1.7.1 ③）⇒ 只要求能取到且 id 在集合内。
    let id = c.getRelationship(0).unwrap().getId().to_string();
    assert!(["rId1", "rId2"].contains(&id.as_str()), "index 0 的 id = {id}");
}
