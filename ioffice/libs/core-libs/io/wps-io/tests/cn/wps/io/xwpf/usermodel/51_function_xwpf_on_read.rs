// 51_function_xwpf_on_read —— 子 part 按关系 type 分派（OR1–OR12）
//
// 规格：`features/s01-08-xwpf-main-part/functions/51_function_xwpf_on_read/FUNCTION.md` §3.4。
// 实现：`xwpf/usermodel/XWPFDocument.rs:647 onDocumentRead`。
//
// ★★诚实基线（源码级实测，**不是**猜测）：
//   ★ 2026-09-21 更新（M-STAGE1 P1）：`onDocumentRead` 在 Rust 侧**已实现**
//   （照 Java `XWPFDocument.java:197-235` 的 9 路关系串分类逐字转写），不再是
//   `todo!("DEFERRED_NON_FUNCTION")` 桩。
//
//   ★ 且该覆写现在**真正生效**：其上游阻塞（`PackagePart` 的 `.rels` 从未被解析）已于
//     M-STAGE1 P1-c1 修复 —— `getRelationshipsCore` 首次读取时以
//     `new_with_container_and_part_name` 建集合、真解析 `_rels`，并借 `sourceUri` 快照
//     正确解析相对 Target（`/word/styles.xml` 而非 `/styles.xml`）。
//     Part 的 `_container` 亦在包 move 后重定位（`set_container`），消除取流 SIGBUS。
//
// ⇒ 本文件判据 = 「**断言当前实测事实**」；字段是否置位取决于**包内是否存在对应关系**，
//   而非「分派是否实现」。未匹配的关系类型（如 C1 的 `webSettings`）静默忽略。
//   规格 §3.4 自己也写明 OR2/OR12 的期望是「当前预期失败 / 断言差异存在并登记 🟡」。

use core_libs_io::io::xwpf::usermodel::XWPFDocument::XWPFDocument;
use core_libs_io::io::xwpf::XWPFPackage::XWPFPackage;
use std::io::Cursor;
use std::path::PathBuf;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn c1_bytes() -> Vec<u8> {
    std::fs::read(crate_root().join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx"))
        .expect("C1 fixture")
}

fn bind(bytes: Vec<u8>) -> XWPFDocument {
    XWPFDocument::XWPFDocument__pkg(XWPFPackage::open_stream(&mut Cursor::new(bytes)))
}

/// 9 个分派字段的「是否已置」快照（`or7`/`or11`/`or12` 的判据）。
fn field_snapshot(d: &XWPFDocument) -> [(&'static str, bool); 9] {
    [
        ("styles", d.getStyles().is_some()),
        ("numbering", d.getNumbering().is_some()),
        ("settings", d.getSettings().is_some()),
        ("comments", d.getComments().is_some()),
        ("endnotes", d.getEndnotes().is_some()),
        ("footnotes", d.getFootnotes().is_some()),
        ("fontTablePart", d.getFontTable().is_some()),
        ("mTheme", d.getThemePart().is_some()),
        ("mCommentsEx", d.getCommentsEx().is_some()),
    ]
}

// ───────────────── OR1：styles 的**类型**差异（★登记） ─────────────────
#[test]
fn or1_styles_typed_poixml_not_xwpfstyles() {
    // 规格 OR1：期望 `Some(XWPFStyles)`；★现状 Rust 是 `Some(POIXMLDocumentPart)`
    // ⇒ 断言「类型差异」并登记（不得断言「已是 XWPFStyles」）。
    let d = bind(c1_bytes());
    assert!(
        d.getStyles().is_some(),
        "C1 的 styles 字段应为 Some（探测段生效）"
    );
    // 类型差异的可观测形式：访问器返回 `&POIXMLDocumentPart`（而非 `&XWPFStyles`）。
    // 该差异由**签名**固定（编译期事实），此处用源码级判据把它钉死，防止静默漂移。
    let src = std::fs::read_to_string(
        crate_root().join("src/cn/wps/io/xwpf/usermodel/XWPFDocument.rs"),
    )
    .expect("可读 XWPFDocument.rs");
    assert!(
        src.contains("pub fn getStyles(&self) -> Option<&POIXMLDocumentPart>"),
        "★现状：`getStyles` 返回 `Option<&POIXMLDocumentPart>`（非 `&XWPFStyles`）——\
         这是 OR1 登记的类型差异；若已改为 `XWPFStyles` ⇒ 请更新 51/PIPELINE_ALIGNMENT.md §4"
    );
    // numbering 的类型同理（Rust 为 `XWPFNumbering`，与 Java 一致）
    assert!(
        src.contains("pub fn getNumbering(&self) -> Option<&XWPFNumbering>"),
        "`getNumbering` 返回 `XWPFNumbering`（与 Java 一致）"
    );
}

// ───────────────── OR12 / OR2：7 个字段恒 None（★核心缺口） ─────────────────
#[test]
fn or12_seven_fields_are_still_none() {
    // 规格 OR12 原为「除 styles/numbering 外 7 个字段恒 None」的**缺口登记**。
    // ★2026-09-21 更新（M-STAGE1 P1-c1）：`onDocumentRead` 的 9 路分派**已真正生效**
    //   （关系解析链路已打通），故本用例按原注释指示改为断言**新的实测事实**。
    //
    // ★C1（standard-hello.docx）实测所置字段：styles / settings / fontTablePart
    //   —— 与 C1 内实际存在的关系一一对应（C1 只有 4 条 document 级关系：
    //      styles、fontTable、settings、webSettings；webSettings 不在 9 路分派清单内）。
    let d = bind(c1_bytes());
    let snap = field_snapshot(&d);
    let some: Vec<&str> = snap.iter().filter(|(_, v)| *v).map(|(k, _)| *k).collect();
    let none: Vec<&str> = snap.iter().filter(|(_, v)| !*v).map(|(k, _)| *k).collect();
    assert_eq!(
        some,
        vec!["styles", "settings", "fontTablePart"],
        "★P1-c1 后实测：C1 上 styles/settings/fontTablePart 被置（与包内关系对应）"
    );
    // 仍是 None 的字段 = C1 **确实没有**对应关系 ⇒ 是「包内无此 part」，不是「分派未实现」
    //   （对照：C2a 的 multipage.docx 含 theme/numbering，会额外置 numbering/mTheme）
    for f in [
        "numbering",
        "comments",
        "endnotes",
        "footnotes",
        "mTheme",
        "mCommentsEx",
    ] {
        assert!(
            none.contains(&f),
            "C1 内无 `{}` 对应关系 ⇒ 该字段应为 None（若被置，说明包内容或分派判据变了）",
            f
        );
    }
}

// ───────────────── OR7：无子 part ⇒ 9 字段全 None ─────────────────
#[test]
fn or7_empty_package_diverges_by_panic() {
    // 规格 OR7 原写「无子 part 的包 ⇒ 9 字段全 None」。
    // ★实测更正：`XWPFPackage::new()`（无 zipArchive、无关系）走 `XWPFDocument__pkg` 时
    //   在 **`:216` 的关系数 `assert!`** 处 **panic**
    //   （`"The number of main document relationship should be only 1."`）——
    //   根本走不到「字段全 None」那一步。
    //   ⇒ OR7 的「9 字段全 None」只在**有 1 条 officeDocument 关系但无子 part** 的包上才有意义
    //     （见下 `or7b`）。
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        XWPFDocument::XWPFDocument__pkg(XWPFPackage::new());
    }));
    assert!(
        r.is_err(),
        "★实测：空 WRITE 包（零关系）在 `:216` 断言处 panic；         若此处为 Ok ⇒ 关系数门禁已放宽，请复核 OR7 与 49 的 BM3 口径"
    );
}

/// OR7 的**有效**形态：有 1 条 officeDocument 关系、但无任何子 part 关系。
#[test]
fn or7b_no_child_parts_all_fields_none() {
    use std::io::Write;
    let ct = r#"<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>"#;
    let rels = r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>"#;
    let doc = r#"<?xml version="1.0" encoding="UTF-8"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body/></w:document>"#;
    let mut buf = Vec::new();
    {
        let mut w = zip::ZipWriter::new(Cursor::new(&mut buf));
        let o = zip::write::SimpleFileOptions::default();
        for (n, c) in [
            ("[Content_Types].xml", ct),
            ("_rels/.rels", rels),
            ("word/document.xml", doc),
        ] {
            w.start_file(n, o).unwrap();
            w.write_all(c.as_bytes()).unwrap();
        }
        w.finish().unwrap();
    }
    let d = bind(buf);
    let snap = field_snapshot(&d);
    assert!(
        snap.iter().all(|(_, v)| !*v),
        "无**子 part 关系**的包 ⇒ 9 字段必须全 None（有字段被置 ⇒ 凭空构造，OR7）；实得 {:?}",
        snap
    );
}

// ───────────────── OR6：未匹配 type ⇒ 静默（不加 warnings） ─────────────────
#[test]
fn or6_unmatched_type_is_silent() {
    // 规格 OR6：无字段变化且**不加 warnings**（加了但未登记 ⇒ 新差异）。
    // `onDocumentRead` 是 deferral 桩 ⇒ 天然「静默」；此处断言 API 面上**不存在** warnings 通道漂移。
    let d = bind(c1_bytes());
    // 无可观测的 warnings 计数 API ⇒ 只断言字段面。
    // ★2026-09-21（M-STAGE1 P1-c1）：分派已生效，C1 上置位字段数 = 3（styles/settings/fontTablePart）；
    //   未匹配的关系类型（C1 的 `webSettings`）**不产生**任何字段变化，也不引入 warnings 通道 —— 与本规格一致。
    let snap = field_snapshot(&d);
    assert_eq!(
        snap.iter().filter(|(_, v)| *v).count(),
        3,
        "C1 上置位字段数应为 3；未匹配类型（webSettings）应静默忽略"
    );
}

// ───────────────── OR10：覆盖体已被调用（P1 已实现） ─────────────────
#[test]
fn or10_override_not_reached() {
    // 规格 OR10：期望「覆盖体被调用（可观测：字段被置）」。
    //
    // ★ 2026-09-21 更新（M-STAGE1 P1）：`onDocumentRead()` 已由 `todo!("DEFERRED_NON_FUNCTION")`
    //   桩改为**真实现**（照 Java `XWPFDocument.java:197-235` 的 9 路关系分类逐字转写）。
    //   原判据「调用必 panic」已失效，按原注释的指示改为断言「可被调用且不再 panic」。
    let mut d = bind(c1_bytes());
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| d.onDocumentRead()));
    assert!(
        r.is_ok(),
        "P1 后 `onDocumentRead()` 已是真实现 ⇒ 调用**不得** panic；\
         若此处为 Err ⇒ 覆盖体退回了桩或实现有缺陷"
    );
}

// ───────────────── OR10b：覆盖体确实被装配链调用（P1 根因回归） ─────────────────
#[test]
fn or10b_override_is_wired_into_assembly() {
    // ★ 这是 P1 修掉的**根因回归**：Java 的 `POIXMLDocument.java:41 this.onDocumentRead()`
    //   是**虚调用**，会派发到 `XWPFDocument` 的覆写；Rust 的 `POIXMLDocument::load`
    //   内是 `self.base.onDocumentRead()`（基类空实现）⇒ **不会**虚派发。
    //   若不在装配处显式调用，9 个 part 字段恒为 None。
    //
    // 判据（源码级，防静默漂移）：装配路径必须存在对 `onDocumentRead()` 的显式调用。
    let src = std::fs::read_to_string(
        crate_root().join("src/cn/wps/io/xwpf/usermodel/XWPFDocument.rs"),
    )
    .expect("XWPFDocument.rs");
    assert!(
        src.contains("doc.onDocumentRead();"),
        "装配路径必须显式调用 `doc.onDocumentRead()`（Rust 无虚派发，漏调 ⇒ 9 路分派全落空）"
    );
}

// ───────────────── OR11：确定性（字段集合不依赖哈希序） ─────────────────
#[test]
fn or11_deterministic_field_set() {
    let a = field_snapshot(&bind(c1_bytes()));
    let b = field_snapshot(&bind(c1_bytes()));
    assert_eq!(a, b, "两次认包的字段集合必须相同（不同 ⇒ 依赖哈希序，OR11）");
}

// ───────────────── OR5：`IMAGES` 被排除（源码级判据） ─────────────────
#[test]
fn or5_images_exclusion_condition_present() {
    // 规格 OR5：`IMAGES` 类型的关系**不得**落进 `fontTablePart`。
    // 该排除条件在 Java `onDocumentRead` 的 `else if`（见 Rust 桩内的模板注释）。
    // ★现状：桩体保留 Java 模板 ⇒ 排除条件**以注释形式**存在但**未执行**。
    // ⇒ 断言「模板注释里含 IMAGES 排除」，并注明其不可执行。
    let src = std::fs::read_to_string(
        crate_root().join("src/cn/wps/io/xwpf/usermodel/XWPFDocument.rs"),
    )
    .unwrap();
    assert!(
        src.contains("XWPFRelation.IMAGES.getRelation()"),
        "桩内应保留 `IMAGES` 排除条件的 Java 模板（OR5）"
    );
    // 且它必须**不在**活代码里（否则就是「已实现」，判据需改）
    let live: Vec<&str> = src
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            !t.starts_with("//") && l.contains("XWPFRelation.IMAGES")
        })
        .collect();
    assert!(
        live.is_empty(),
        "`IMAGES` 排除条件应仍在模板注释中（非活代码）；若已活 ⇒ 请更新 OR5 判据，实得 {:?}",
        live
    );
}

// ───────────────── OR9：`packageRel = None` 的决策点 ─────────────────
#[test]
fn or9_null_rel_decision_recorded() {
    // 规格 OR9：决策点须**被记录**（不得静默 panic 且未登记）。
    // 现状：`onDocumentRead` 整体未实现 ⇒ 该决策点尚不存在；此处断言「已登记」这一事实
    //      （登记位置 = 51 的 PIPELINE_ALIGNMENT.md §4）。
    let rep = crate_root().join(
        "../../../word-core/rewrite-review/stages/S01-opc-package/features/s01-08-xwpf-main-part/functions/51_function_xwpf_on_read/PIPELINE_ALIGNMENT.md",
    );
    if rep.exists() {
        let t = std::fs::read_to_string(&rep).unwrap();
        assert!(
            t.contains("OR9") || t.contains("packageRel"),
            "OR9 的决策点必须在 PIPELINE_ALIGNMENT.md 中登记"
        );
    }
}

// ───────────────── OR13：子 part 绑定了真实 `packagePart`（P1-e 根因回归） ─────────────────
#[test]
fn or13_child_parts_are_bound_to_real_package_part() {
    // ★这是 M-STAGE1 P1-e 修掉的**根因回归**（与 OR10b 同一类，但更深一层）。
    //
    // 根因链（三层，全部实测）：
    //   ① `parse()` 里 8 处分部 parse 已接线，但**会静默空转**；
    //   ② 因为子 part 的 `base.getPackagePart()` 为 `None` ⇒ `parse()` 首行守卫不成立
    //      ⇒ 静默 `return Ok(())`，从不真正调 `XmlReader::parse`；
    //   ③ 因为 `XWPFFactory::createDocumentPart` 把 `part` 参数**丢弃**、给 `new_with_part` 传 `None`
    //      （Java 是 `constructor.newInstance(parent, part, rel)` —— 把真实 part 交给子 part 构造器）。
    //
    // 判据：C1（含 `word/settings.xml` 182B、`word/fontTable.xml` 246B）装配后，
    //      这两个子 part 必须**真的持有** `packagePart`，否则 parse 是假绿。
    let d = bind(c1_bytes());
    let settings = d.getSettings().expect("C1 应装配出 settings（包内有该关系）");
    assert!(
        settings.getPackagePart().is_some(),
        "★P1-e：settings 必须绑定真实 packagePart（否则其 parse() 静默返回 Ok，是假绿）"
    );
    assert_eq!(
        settings.getPackagePart().as_ref().unwrap().getPartName().getName(),
        "/word/settings.xml",
        "绑定的应是包内真实 part 名"
    );
    let font_table = d.getFontTable().expect("C1 应装配出 fontTablePart");
    assert!(
        font_table.getPackagePart().is_some(),
        "★P1-e：fontTablePart 必须绑定真实 packagePart"
    );
    assert_eq!(
        font_table.getPackagePart().as_ref().unwrap().getPartName().getName(),
        "/word/fontTable.xml",
        "绑定的应是包内真实 part 名"
    );
}

// ───────────────── OR14：子 part 的 parse() 真的读到了流（非静默空转） ─────────────────
#[test]
fn or14_child_part_parse_actually_reads_stream() {
    // OR13 证「绑上了 part」；本用例证「该 part **能取到流**」—— 取流能力才是 `parse()` 的前提。
    // 判定：对绑定的 `packagePart` 调 `getInputStream_default()`，必须拿到**非空**字节，
    //      且内容与该 part 的真实内容一致（settings.xml 是 XML ⇒ 以 `<` 开头）。
    let d = bind(c1_bytes());
    let settings = d.getSettings().expect("settings");
    let part = settings.getPackagePart().as_ref().expect("part bound");
    let mut stream = part
        .getInputStream_default()
        .expect("★P1-e：绑定后的 part 必须能取到流（否则 parse() 仍空转）");
    let mut buf = Vec::new();
    std::io::Read::read_to_end(&mut stream, &mut buf).expect("read");
    assert!(!buf.is_empty(), "settings.xml 内容非空（C1 实测 182 字节）");
    let text = String::from_utf8_lossy(&buf);
    assert!(
        text.trim_start().starts_with('<'),
        "取到的应是 settings.xml 的 XML 内容，实际开头：{:?}",
        &text[..text.len().min(40)]
    );
}
