// function_id=29_function_parts_scan_ct
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 29_function_parts_scan_ct
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/OPCPackage.rs（`getPartsImpl` 第一遍 `:151-164`）
//       libs/core-libs/io/wps-io/src/cn/wps/io/opc/internal/ContentTypeManager.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/OPCPackage_scan_ct.rs
//
// 本 function = `getPartsImpl` 的**第一遍**（Java `OPCPackage.java:527-546`）：
//   ① 逐 entry：`entry.getSize() > 1073741824L` ⇒ `FileDamagedException`
//   ② `checkEntryNameLegal(entryName)`（`entryName.contains("../")` ⇒ `FileDamagedException`）——**先于** CT 判定
//   ③ 若**尚未**找到 CT 且 `isPartNameContentTypes(entryName)`（19 字节按位掩码比 `[Content_Types].xml`，大小写不敏感）
//      ⇒ `contentTypeManager = new ZipContentTypeManager(getZipArchive().getInputStream(entry), this)`；IOException ⇒ InvalidFormatException
//   ④ 否则 entry 进 `exceptContentType`（候选集）★CT entry **自己不进**候选集
// ⇒ 输出面 = `contentTypeManager`（可为 null）+ 候选集（顺序 = zip 物理序）

use core_libs_io::io::opc::internal::ContentTypeManager::{ContentTypeManager, TYPES_NAMESPACE_URI};
use core_libs_io::io::opc::internal::ZipContentTypeManager::ZipContentTypeManager;
use core_libs_io::io::opc::OPCPackage::{OPCPackage, MAX_SIZE_ENTRY};
use core_libs_io::io::opc::PackagePartName::PackagePartName;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;
use std::io::{Cursor, Write};

const MAIN_CT: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const ALT_CT: &str = "application/x-alt-main";
const XML_CT: &str = "application/xml";
const RELS_CT: &str = "application/vnd.openxmlformats-package.relationships+xml";

/// 生成 `[Content_Types].xml`：`override` 为 `(PartName, ContentType)` 列表。
fn types_xml_with(overrides: &[(&str, &str)]) -> String {
    let mut body = String::new();
    for (part, ct) in overrides {
        body.push_str(&format!(
            r#"<Override PartName="{part}" ContentType="{ct}"/>"#
        ));
    }
    format!(
        r#"<?xml version="1.0"?><Types xmlns="{TYPES_NAMESPACE_URI}"><Default Extension="xml" ContentType="{XML_CT}"/><Default Extension="rels" ContentType="{RELS_CT}"/>{body}</Types>"#
    )
}

fn zip_bytes(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(cursor);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    for (name, bytes) in entries {
        writer.start_file(*name, options).unwrap();
        writer.write_all(bytes).unwrap();
    }
    writer.finish().unwrap().into_inner()
}

/// ★返回 `Box<OPCPackage>`：part 会持有指向所属包的共享引用指针（见 `05` 的 §1.7.1 ③）
/// ⇒ 包在 part 物化之后**不得再被 move**；`Box` 保证被指对象地址稳定。
fn open(bytes: Vec<u8>) -> Box<OPCPackage> {
    let mut input = Cursor::new(bytes);
    Box::new(OPCPackage::from_stream(&mut input))
}

fn doc_name() -> PackagePartName {
    PackagingURIHelper::createPartName_str("/word/document.xml").unwrap()
}

/// 取 part 的 CT 文本（`None` = 该 part 不在表里）。
fn ct_of(pkg: &mut OPCPackage, name: &PackagePartName) -> Option<String> {
    pkg.getPart(name).map(|p| p.getContentType().toString())
}

/// 表里所有 part 名（排序后，便于两侧比较；`HashMap` 序不保证）。
/// ★与 dump 同口径：Java `partList.values()`（Rust 集合只暴露 `values()`，无 `keySet()` 等价物）
/// ⇒ 从 part 自己取 `getPartName().getName()`，不新增 src 访问器。
fn part_names(pkg: &mut OPCPackage) -> Vec<String> {
    let mut names: Vec<String> = match pkg.getPartList() {
        Some(list) => list.values().map(|p| p.getPartName().getName()).collect(),
        None => Vec::new(),
    };
    names.sort();
    names
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

/// SC-01 / 主路径：第一条 entry 就是 CT ⇒ manager 被建、**从 entry 字节**解析（override 生效），
/// 且 CT entry **自己不是**一个 part。
#[test]
fn sc01_ct_manager_is_built_from_the_ct_entry() {
    let types = types_xml_with(&[("/word/document.xml", MAIN_CT)]);
    let mut pkg = open(zip_bytes(&[
        ("[Content_Types].xml", types.as_bytes()),
        ("word/document.xml", b"<w:document/>"),
    ]));
    pkg.getParts();
    // override 生效 ⇒ CT 来自 entry 内容（仅靠扩展名猜测只会得到 `application/xml`）
    assert_eq!(ct_of(&mut pkg, &doc_name()).as_deref(), Some(MAIN_CT));
    // CT entry 不进候选集 ⇒ 不在 part 表里
    assert_eq!(part_names(&mut pkg), vec!["/word/document.xml".to_string()]);
}

/// SC-01 的补充证据（§1.7.1 ③）：Java 第一遍用**子类** `new ZipContentTypeManager(stream, this)` 建 manager，
/// Rust 直接用基类 `ContentTypeManager::new(…)`。此处的等价性**可验证**：Java 子类构造器只有一句
/// `super(in, pkg);`（`ZipContentTypeManager.java:23-25`，增量只在写侧 `saveImpl`）⇒ 同一份 CT 字节喂两条
/// 构造路径，读侧 `getContentType` 必须逐项相同。
#[test]
fn sc01b_zip_ct_manager_ctor_is_equivalent_to_the_base_class() {
    let types = types_xml_with(&[("/word/document.xml", MAIN_CT), ("/word/media/a.png", "image/png")]);
    let base = ContentTypeManager::new(
        Some(&mut Cursor::new(types.as_bytes().to_vec())),
        None,
    );
    let sub = ZipContentTypeManager::new(
        Some(&mut Cursor::new(types.as_bytes().to_vec())),
        None,
    );
    let name = doc_name();
    assert_eq!(base.getContentType(&name).as_deref(), Some(MAIN_CT));
    assert_eq!(sub.getContentType(&name), base.getContentType(&name));
    // Default 推断（`xml`/`rels`）也必须一致
    let rels = PackagingURIHelper::createPartName_str("/word/_rels/document.xml.rels").unwrap();
    assert_eq!(sub.getContentType(&rels), base.getContentType(&rels));
    assert_eq!(base.getContentType(&rels).as_deref(), Some(RELS_CT));
}

/// SC-02 / 边界：`isPartNameContentTypes` 是**按位掩码的大小写不敏感**比较 ⇒ 大小写混写仍命中。
#[test]
fn sc02_ct_name_match_is_case_insensitive() {
    for name in ["[Content_Types].xml", "[content_types].XML", "[CONTENT_TYPES].Xml"] {
        assert!(
            OPCPackage::isPartNameContentTypes(name),
            "{name:?} 应按 19 字节掩码规则命中"
        );
        let types = types_xml_with(&[("/word/document.xml", MAIN_CT)]);
        let mut pkg = open(zip_bytes(&[
            (name, types.as_bytes()),
            ("word/document.xml", b"<w:document/>"),
        ]));
        pkg.getParts();
        assert_eq!(
            ct_of(&mut pkg, &doc_name()).as_deref(),
            Some(MAIN_CT),
            "{name:?} 应被当作 CT 消费"
        );
        assert_eq!(part_names(&mut pkg), vec!["/word/document.xml".to_string()], "{name:?}");
    }
}

/// SC-03 / 边界：长度或字符不符 ⇒ **不**命中（落候选集）⇒ manager 未建，CT 走扩展名猜测。
#[test]
fn sc03_length_and_charset_guards_fall_through_to_candidates() {
    let good = "[Content_Types].xml";
    assert!(OPCPackage::isPartNameContentTypes(good));
    for bad in [
        "[Content_Types].xml ", // 20 字节（尾空格）
        "[Content_Types]xml",   // 18 字节
        "(Content_Types).xml",  // 首字符不符
        "[Content_Types]xml.",  // `]`/`.` 顺序不符
        "Content_Types].xml",   // 少 `[`
        "[Content_Types].xm_",  // 尾字符不符
    ] {
        assert!(!OPCPackage::isPartNameContentTypes(bad), "{bad:?} 不应命中 CT 规则");
    }
    // ★实测（两侧一致）：CT 未被消费 ⇒ manager 未建 ⇒ 第二遍 `getContentType` 为 null，
    //   退回 `ContentTypes.guessContentTypeByExtension(extension)` —— 它**只认图片扩展名**
    //   （Java `PictureType.getPictureType(extension)`）⇒ `.xml` 猜不出 CT ⇒ 该 part **根本不进表**。
    //   （旧文档 §3.4 写的「`part_count = 1`，CT 来自扩展名猜测 application/xml」是**错的**，已按实测改正。）
    let types = types_xml_with(&[("/word/document.xml", MAIN_CT)]);
    let mut pkg = open(zip_bytes(&[
        ("[Content_Types].xml ", types.as_bytes()),
        ("word/document.xml", b"<w:document/>"),
    ]));
    pkg.getParts();
    assert_eq!(ct_of(&mut pkg, &doc_name()), None, "无 CT ⇒ `.xml` 猜不出 CT ⇒ 无该 part");
    assert_eq!(
        part_names(&mut pkg),
        Vec::<String>::new(),
        "CT entry 名尾带空格 ⇒ 连它自己也不是合法 part 名 ⇒ 整表为空"
    );
    // 对照：名字合法时它确实会（在 CT 未命中时）走完全不同的分支
    assert!(pkg.getPartList().is_some(), "`partList` 仍被建立（只是为空）");
}

/// SC-04 / 畸形：没有 CT entry ⇒ **不报错**，`contentTypeManager` 保持 `None`（CT 全靠猜测）。
#[test]
fn sc04_missing_ct_entry_is_not_an_error() {
    let mut pkg = open(zip_bytes(&[("word/document.xml", b"<w:document/>")]));
    pkg.getParts(); // 不 panic = 不报错（Java 侧同样「静默」）
    assert_eq!(
        ct_of(&mut pkg, &doc_name()),
        None,
        "★实测：无 CT manager ⇒ CT 为 null；`.xml` 不是图片扩展名 ⇒ 猜不出 ⇒ part 不入表（Java 同）"
    );
    assert_eq!(part_names(&mut pkg), Vec::<String>::new());
    // 图片扩展名才猜得出 CT ⇒ 用它反证「null CT ⇒ 丢 part」这条规则本身在跑
    let mut png = open(zip_bytes(&[("word/media/a.png", b"\x89PNG")]));
    png.getParts();
    assert_eq!(
        ct_of(&mut png, &PackagingURIHelper::createPartName_str("/word/media/a.png").unwrap()).as_deref(),
        Some("image/png"),
        "PNG 走 `PictureType` 猜测 ⇒ 无 CT manager 也能入表"
    );
}

/// SC-05 / 畸形（C9 变体）：两个 CT entry ⇒ **只消费第一个**；第二个按普通 entry 处理
/// （它的名字不合法 ⇒ `buildPartName` 返回 null ⇒ 不进 part 表）。
#[test]
fn sc05_first_ct_entry_wins() {
    // ★实测：`zip` crate 的 `start_file` 拒**重名**（`Err("duplicate filename")`）⇒ 无法在本 crate 里
    //   造出「字节完全相同名字」的两条 entry（真重复是 `30_function_parts_dup_guard` 的面，
    //   需要手写 zip 字节）。这里用**两个都被 CT 规则命中**的不同名（`contentTypePartFound` 同一判据）。
    let first = types_xml_with(&[("/word/document.xml", MAIN_CT)]);
    let second = types_xml_with(&[("/word/document.xml", ALT_CT)]);
    let mut pkg = open(zip_bytes(&[
        ("[Content_Types].xml", first.as_bytes()),
        ("[CONTENT_TYPES].XML", second.as_bytes()),
        ("word/document.xml", b"<w:document/>"),
    ]));
    pkg.getParts();
    assert_eq!(
        ct_of(&mut pkg, &doc_name()).as_deref(),
        Some(MAIN_CT),
        "★`contentTypePartFound` 一旦置真 ⇒ 后面的同名 entry 不再覆盖 manager"
    );
    assert_eq!(part_names(&mut pkg), vec!["/word/document.xml".to_string()]);
}

/// SC-06 / 失败：entry 名含 `../` ⇒ `checkEntryNameLegal` 抛 `FileDamagedException`，
/// 且该判定在 **CT 判定之前**（即便名字长得像 CT 也一样）。
#[test]
fn sc06_parent_dir_entry_name_panics_before_ct_detection() {
    // ① 判据本体（Java `OPCPackage.checkEntryNameLegal`，`:63-67`）：`contains("../")` ⇒ FileDamagedException
    let err = std::panic::catch_unwind(|| OPCPackage::checkEntryNameLegal("../[Content_Types].xml"))
        .unwrap_err();
    assert_eq!(panic_message(err), "FileDamagedException");
    // 合法名不 panic
    OPCPackage::checkEntryNameLegal("word/document.xml");
    OPCPackage::checkEntryNameLegal("a..b.xml"); // 只有连续 `../` 才命中（Java `contains`，非路径规范化）
    // ② ★端到端实测偏差（已登记 §1.7.1）：Rust 的 **zip 源构造**自己就会先拒该名并返回
    //    `Err("FileDamagedException")`，而 `OPCPackage::from_stream` 把**任何**源错误统一换成
    //    `panic!("Can't open the specified stream")` ⇒ Java 侧此时抛的是 `FileDamagedException`
    //    （`ZipInputStreamZipEntrySource:35` 调 `XWPFPackage.checkEntryNameLegal`）⇒ 消息/类型丢失。
    let bytes = zip_bytes(&[
        ("../[Content_Types].xml", b"<Types/>"),
        ("word/document.xml", b"<w:document/>"),
    ]);
    let err = std::panic::catch_unwind(move || {
        let mut pkg = open(bytes);
        pkg.getParts();
    })
    .unwrap_err();
    assert_eq!(
        panic_message(err),
        "Can't open the specified stream",
        "★当前实测：错误在源构造阶段就发生，Java 的 `FileDamagedException` 被换成该字面量（偏差，见 §1.7.1 ③）"
    );
}

/// SC-07 / 失败：1 GiB 尺寸门禁在**第一遍的最前面**（`entry.getSize() > 1073741824L`）。
/// ★zip 里无法凭空伪造 >1 GiB 的 entry（要么真写 1 GiB 数据）⇒ 本用例钉住**判据本身**与
/// `MAX_SIZE_ENTRY` 常量，循环级触发由 §1.7.1 登记（与既有 `p6_max_size_entry_gate` 同一判据）。
#[test]
fn sc07_size_guard_boundary_and_constants() {
    assert_eq!(MAX_SIZE_ENTRY, 1073741824);
    assert!(!OPCPackage::isEntryTooLarge(MAX_SIZE_ENTRY as i64), "边界值本身不算超限（Java `>` 不是 `>=`）");
    assert!(OPCPackage::isEntryTooLarge(MAX_SIZE_ENTRY as i64 + 1));
    // 结构断言：尺寸门禁在 CT 判定之前（源码顺序 = Java `:529` 先于 `:535`）
    let src = include_str!("../../../../../src/cn/wps/io/opc/OPCPackage.rs");
    // ★调用点用 `Self::` 限定并且局部绑定为 `entryName`（与 Java 变量名对齐，见 §3.8 D0）；
    //   断言的是**三者的相对源码顺序**，与限定形式无关。
    let size_pos = src.find("isEntryTooLarge(entry.getSize())").expect("尺寸门禁调用点");
    assert!(src.find("Self::isEntryTooLarge(entry.getSize())").is_some(), "尺寸门禁调用点形态");
    let ct_pos = src
        .find("isPartNameContentTypes(&entryName)")
        .or_else(|| src.find("isPartNameContentTypes(&entry_name)"))
        .expect("CT 判定调用点");
    assert!(size_pos < ct_pos, "★尺寸门禁必须排在 CT 判定之前（Java `:529` → `:535`）");
    let name_pos = src
        .find("checkEntryNameLegal(&entryName)")
        .or_else(|| src.find("checkEntryNameLegal(&entry_name)"))
        .expect("名字门禁调用点");
    assert!(size_pos < name_pos && name_pos < ct_pos, "★顺序必须是 尺寸 → 名字 → CT");
}

/// SC-08 / 确定性：entry 的 zip 物理序不同 ⇒ **结果相同**（part 表与 CT 一致）。
/// （候选集自身的顺序在 Java 侧也只喂给按下标对齐的两个数组 ⇒ 对外不可观测；
///  真正可观测的是「结果与物理序无关」。）
#[test]
fn sc08_results_are_independent_of_zip_physical_order() {
    let types = types_xml_with(&[("/word/document.xml", MAIN_CT)]);
    let forward = zip_bytes(&[
        ("[Content_Types].xml", types.as_bytes()),
        ("word/document.xml", b"<w:document/>"),
        ("word/media/a.png", b"\x89PNG"),
    ]);
    let reversed = zip_bytes(&[
        ("word/media/a.png", b"\x89PNG"),
        ("word/document.xml", b"<w:document/>"),
        ("[Content_Types].xml", types.as_bytes()),
    ]);
    let mut a = open(forward);
    let mut b = open(reversed);
    a.getParts();
    b.getParts();
    assert_eq!(part_names(&mut a), part_names(&mut b));
    assert_eq!(ct_of(&mut a, &doc_name()), ct_of(&mut b, &doc_name()));
    assert_eq!(ct_of(&mut a, &doc_name()).as_deref(), Some(MAIN_CT));
}

/// SC-09 / 畸形：CT entry 是 **0 字节** ⇒ 两侧行为必须分别记录（Java：`new ZipContentTypeManager`
/// 解析空 XML ⇒ 抛异常 ⇒ `getPartsImpl` 以 `InvalidFormatException` 失败）。
/// ★本用例钉住**当前 Rust 实测**行为，并在 §1.7.1 记录与 Java 的差异（若有）。
#[test]
fn sc09_empty_ct_stream_behavior_is_recorded() {
    let outcome = std::panic::catch_unwind(|| {
        let mut pkg = open(zip_bytes(&[
            ("[Content_Types].xml", b""),
            ("word/document.xml", b"<w:document/>"),
        ]));
        pkg.getParts();
        ct_of(&mut pkg, &doc_name())
    });
    // ★实测：Rust 在 CT 解析处直接 panic，载荷 = Java 的字面量
    //   （`ContentTypeManager.rs`：`throw new InvalidFormatException("Can't read content types part !")` 的归一化分支）。
    //   Java 侧同一输入：`new ZipContentTypeManager(空流, this)` ⇒ `super(in, pkg)` 解析失败 ⇒
    //   `InvalidFormatException("Can't read content types part !")` —— 它是 **checked** 异常，
    //   `getPartsImpl` 只捕获 `IOException` ⇒ 异常**穿透** `getParts()` 传出去 ⇒ 打开包失败。
    //   ⇒ 两侧「失败 + 同一文案」一致；差别只在「异常通道 vs panic」（本仓既定约定，见 §1.7.1 ②）。
    let err = outcome.unwrap_err();
    assert_eq!(panic_message(err), "Can't read content types part !");
}

/// SC-10 / 确定性：同一份字节跑两次 ⇒ part 表与 CT 完全一致。
#[test]
fn sc10_same_input_twice_same_result() {
    let types = types_xml_with(&[("/word/document.xml", MAIN_CT)]);
    let bytes = zip_bytes(&[
        ("[Content_Types].xml", types.as_bytes()),
        ("word/document.xml", b"<w:document/>"),
        ("word/media/a.png", b"\x89PNG"),
    ]);
    let mut first = open(bytes.clone());
    let mut second = open(bytes);
    first.getParts();
    second.getParts();
    assert_eq!(part_names(&mut first), part_names(&mut second));
    assert_eq!(ct_of(&mut first, &doc_name()), ct_of(&mut second, &doc_name()));
}

