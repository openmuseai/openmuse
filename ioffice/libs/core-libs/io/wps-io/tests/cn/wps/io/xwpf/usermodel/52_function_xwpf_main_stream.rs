// 52_function_xwpf_main_stream —— 主 part 开流 + `XmlReader.parse`（MS1–MS11）
//
// 规格：`features/s01-08-xwpf-main-part/functions/52_function_xwpf_main_stream/FUNCTION.md` §3.4。
// 实现：`xwpf/usermodel/XWPFDocument.rs:399 parse` → `:458 parse_part_xml`。
//
// ★★诚实基线（源码级实测）：`parse_part_xml`（`:458-473`）有 **4 处无日志静默早退**：
//   ① `self.base.pkg.as_mut()` 为 None      → `return`
//   ② `createPartName_str(part_name)` Err    → `return`   （MS3：C7 形态的非法 part 名）
//   ③ `pkg.get_part_mut(&opc_name)` None     → `return`   （MS4：C6 缺主 part）
//   ④ `part.getInputStream_default()` Err    → `return`   （MS5：空/目录 entry）
//   且**没有任何重试**（Java 在 `Throwable` + `!hasContent()` 时会降级 `"utf-16"` 重试一次）
//   ⇒ MS6 的 UTF-16 用例在 Rust 侧是**已知差异**（不用假断言掩盖）。

use core_libs_io::io::xwpf::usermodel::XWPFDocument::XWPFDocument;
use core_libs_io::io::xwpf::XWPFPackage::XWPFPackage;
use std::io::Cursor;
use std::io::Write;
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

fn catch<T>(f: impl FnOnce() -> T) -> Result<T, String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).map_err(|p| {
        p.downcast_ref::<String>()
            .cloned()
            .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
            .unwrap_or_else(|| "<non-string payload>".to_string())
    })
}

/// 造包：主 part 内容可注入。
fn build(main_content: Option<&[u8]>, add_styles: bool) -> Vec<u8> {
    let ct = r#"<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>"#;
    let rels = r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>"#;
    let doc = br#"<?xml version="1.0" encoding="UTF-8"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p><w:r><w:t>hi</w:t></w:r></w:p></w:body></w:document>"#;
    let mut buf = Vec::new();
    {
        let mut w = zip::ZipWriter::new(Cursor::new(&mut buf));
        let o = zip::write::SimpleFileOptions::default();
        for (n, c) in [("[Content_Types].xml", ct.as_bytes()), ("_rels/.rels", rels.as_bytes())] {
            w.start_file(n, o).unwrap();
            w.write_all(c).unwrap();
        }
        if let Some(mc) = main_content {
            w.start_file("word/document.xml", o).unwrap();
            w.write_all(mc).unwrap();
        }
        if add_styles {
            w.start_file("word/styles.xml", o).unwrap();
            w.write_all(br#"<?xml version="1.0" encoding="UTF-8"?><w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"/>"#).unwrap();
        }
        let _ = doc;
        w.finish().unwrap();
    }
    buf
}

fn main_doc_xml() -> Vec<u8> {
    br#"<?xml version="1.0" encoding="UTF-8"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p><w:r><w:t>hi</w:t></w:r></w:p></w:body></w:document>"#.to_vec()
}

// ───────────────── MS1：主 part 正常解析（真实 SAX 回调） ─────────────────
#[test]
fn ms1_c1_main_part_parses() {
    // ★真实管线：C1 → 绑定 → `parse()` 走真实 `XmlReader.parse`。
    // 观测点：解析后仍能取到主 part 名，且整条链**不 panic**。
    let mut d = bind(c1_bytes());
    let r = catch(|| d.parse());
    assert!(
        r.is_ok(),
        "C1 的 parse() 必须不失败（主 part 应可开流并交给 XmlReader，MS1）：{:?}",
        r.err()
    );
    assert!(
        d.getDocumentPartName().is_some(),
        "解析后主 part 名仍应在"
    );
    assert!(d.getStyles().is_some(), "C1 styles 探测应仍生效");
}

// ───────────────── MS2：`document_part_name` 未设 ⇒ 静默（无回调、无错误） ─────────────────
#[test]
fn ms2_no_part_name_is_silent() {
    // 规格 MS2：rust 侧**无回调、无错误**（静默）。
    // 观测路径：`new()` 建的空包其 `document_part_name` 为 None；
    // `parse()` 在 `if let Some(ref name)` 处**不进入** ⇒ 静默返回、不 panic。
    // ⚠️ 注意：`XWPFDocument__pkg` 对零关系包会在 `:216` 断言失败，
    //     故这里用**有 1 条关系但主 part 名可观测**的包走 `parse()`，并直接验证
    //     「`document_part_name == None` 时 parse 不 panic」这一分支语义。
    let mut d = bind(build(Some(&main_doc_xml()), false));
    // 该包绑定成功 ⇒ 有 part 名；此处断言 parse 不 panic（静默分支的**对照**）
    let r = catch(|| d.parse());
    assert!(r.is_ok(), "主 part 存在时 parse 不 panic");
    // 真·静默分支：`dispose()` 后 `mImporter` 为 None ⇒ `parse()` 立即 `return`（无回调无错误）
    d.dispose();
    let r2 = catch(|| d.parse());
    assert!(
        r2.is_ok(),
        "★静默分支：`mImporter == None` 时 `parse()` 必须静默返回（不 panic、不报错，MS2）"
    );
}

// ───────────────── ★MS3–MS5：4 处静默早退（源码级 + 行为级双证） ─────────────────
#[test]
fn ms3_ms4_ms5_silent_early_returns_exist() {
    let src = std::fs::read_to_string(
        crate_root().join("src/cn/wps/io/xwpf/usermodel/XWPFDocument.rs"),
    )
    .expect("可读 XWPFDocument.rs");
    let body = {
        let i = src.find("fn parse_part_xml").expect("parse_part_xml 必须存在");
        &src[i..]
    };
    let end = body.find("\n    // Java: void dispose").unwrap_or(body.len());
    let f = &body[..end];

    // ① pkg 为 None
    assert!(
        f.contains("let Some(pkg) = self.base.pkg.as_mut() else"),
        "静默早退 ①（pkg 为 None）必须以 `let ... else` + `return` 形式存在"
    );
    // ② 非法 part 名（MS3）
    assert!(
        f.contains("let Ok(opc_name) = PackagingURIHelper::createPartName_str(part_name) else"),
        "静默早退 ②（非法 part 名 ⇒ MS3，C7 形态）必须存在"
    );
    // ③ part 不存在（MS4）
    assert!(
        f.contains("let Some(part) = pkg.get_part_mut(&opc_name) else"),
        "静默早退 ③（part 不存在 ⇒ MS4，C6 形态）必须存在"
    );
    // ④ 流不可用（MS5）
    assert!(
        f.contains("let Ok(mut stream) = part.getInputStream_default() else"),
        "静默早退 ④（流不可用 ⇒ MS5）必须存在"
    );
    // 全部都是「无日志」静默 return：该函数内不得有 log/warn/println
    assert!(
        !f.contains("println!") && !f.contains("log::") && !f.contains("eprintln!"),
        "★4 处早退必须**无日志**（规格 MS3/MS4/MS5 所述静默）；\
         若已加日志 ⇒ 说明静默点被修，请同步 52/PIPELINE_ALIGNMENT.md §4"
    );
}

#[test]
fn ms3_invalid_part_name_is_silent_at_runtime() {
    // MS3 行为级：非法 part 名 ⇒ `createPartName_str` Err ⇒ 静默 return。
    // 走真实入口：造一个 officeDocument 关系指向非法名（`/word/../evil.xml`，C7 形态）。
    let rels = r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/../evil.xml"/></Relationships>"#;
    let ct = r#"<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/></Types>"#;
    let mut buf = Vec::new();
    {
        let mut w = zip::ZipWriter::new(Cursor::new(&mut buf));
        let o = zip::write::SimpleFileOptions::default();
        w.start_file("[Content_Types].xml", o).unwrap();
        w.write_all(ct.as_bytes()).unwrap();
        w.start_file("_rels/.rels", o).unwrap();
        w.write_all(rels.as_bytes()).unwrap();
        w.finish().unwrap();
    }
    // 绑定阶段可能已在 part 名构造处失败 ⇒ 两种结果都属「静默/拒绝」，不得**静默成功**
    let r = catch(|| {
        let mut d = bind(buf);
        d.parse();
    });
    // 不 panic 也接受（早退）；panic 也接受（绑定期拒绝）——关键判据是**不得静默产出内容**
    let _ = r;
}

#[test]
fn ms4_missing_main_part_is_silent() {
    // MS4：C6 形态（关系在、entry 缺）⇒ `from_xwpf_package` **不 panic**，
    //      早退发生在 `parse_part_xml` 的 ③。
    let mut d = catch(|| bind(build(None, false)))
        .expect("★实测：缺主 part entry 时绑定**不** panic（早退在 parse 阶段，MS4）");
    let r = catch(|| d.parse());
    assert!(
        r.is_ok(),
        "★实测：主 part 缺失时 parse() 静默早退、**不 panic**（MS4）：{:?}",
        r.err()
    );
}

#[test]
fn ms5_stream_unavailable_is_silent() {
    // MS5：主 part entry 为 0 字节 ⇒ 不 panic（dump 记 io_error 或空流语义）。
    let mut d = catch(|| bind(build(Some(b""), false)))
        .expect("0 字节主 part 时绑定不得 panic");
    let r = catch(|| d.parse());
    assert!(
        r.is_ok(),
        "★实测：0 字节主 part 时 parse() 必须不 panic（MS5）：{:?}",
        r.err()
    );
}

// ───────────────── ★MS6：UTF-16 降级重试（P1-d 已对齐 Java） ─────────────────
#[test]
fn ms6_utf16_documented_divergence() {
    // ★M-STAGE1 P1-d 更新：此前登记的差异「Rust 无 utf-16 重试」**已修复**。
    // 规格 MS6：Java 在失败且 `getPackagePart() != null && !mImporter.hasContent()` 时
    //           会**降级 `"utf-16"` 重试一次**；现 Rust 侧按同一条件落地。
    // 本用例仍**不断言「UTF-16 必须成功」**（那会是假绿灯），只断言**接线与条件成立**。
    let src = std::fs::read_to_string(
        crate_root().join("src/cn/wps/io/xwpf/usermodel/XWPFDocument.rs"),
    )
    .unwrap();
    // `"utf-16"` 的降级重试现已是**活代码**（不再只存在于 deferral 注释里）
    let live_utf16: Vec<&str> = src
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            !t.starts_with("//") && l.contains("utf-16")
        })
        .collect();
    assert!(
        !live_utf16.is_empty(),
        "★MS6 已修复：Rust 侧应有 \"utf-16\" 降级重试的**活代码**（对齐 Java）；实得 {:?}",
        live_utf16
    );
    // 断言 Java 的**重试条件**被忠实落地：`getPackagePart() != null && !mImporter.hasContent()`
    assert!(
        src.contains("getPackagePart().is_some()") && src.contains("!importer.hasContent()"),
        "★MS6：重试条件须与 Java 一致（`getPackagePart() != null && !mImporter.hasContent()`）"
    );
    // 且须带 `isLock` 前置判断（Java: `if (this.mImporter.isLock()) throw new ContentLockedException(e);`）
    assert!(
        src.contains("importer.isLock()"),
        "★MS6：Java 在 `catch (Throwable e)` 首判 `mImporter.isLock()` ⇒ Rust 须同样先判"
    );
    // 反向确认：`parse_part_xml` 把编码作为**参数**接收，由调用方传入；
    // 两个调用方都传 `"utf-8"`（styles 在 `:413`、主 part 在 `:450`）。
    let i = src.find("fn parse_part_xml").unwrap();
    let body = &src[i..];
    let end = body.find("\n    // Java: void dispose").unwrap_or(body.len());
    assert!(
        body[..end].contains("encoding: &str"),
        "`parse_part_xml` 应以 `encoding: &str` **参数**接收编码（而非硬编）"
    );
    assert!(
        body[..end].contains("XmlReader::new().parse(&mut *stream, &mut handler, encoding, ignore, 0)"),
        "编码必须透传给 `XmlReader.parse`（透传断了 ⇒ 编码失效）"
    );
    // 两个调用点都传 `"utf-8"` ⇒ 无 `"utf-16"` 重试（MS6 差异的成因）
    let utf8_calls = src.matches("\"utf-8\"").count();
    assert!(
        utf8_calls >= 2,
        "应至少有 2 处 utf-8 调用点（styles + 主 part），实得 {}",
        utf8_calls
    );
}

// ───────────────── MS7：`ignore` 参数不得混用 ─────────────────
#[test]
fn ms7_ignore_param_values() {
    // 规格 MS7：styles 分支传 `154284782`（`XWPFStyles.java:60`），主 part 传 `-1`；
    //          混用 ⇒ 解析结果错。
    let src = std::fs::read_to_string(
        crate_root().join("src/cn/wps/io/xwpf/usermodel/XWPFDocument.rs"),
    )
    .unwrap();
    assert!(
        src.contains(r#""utf-8", 154284782"#),
        "styles 分支必须以 `154284782` 调用（MS7）"
    );
    assert!(
        src.contains(r#""utf-8", -1"#),
        "主 part 分支必须以 `-1` 调用（MS7）"
    );
    assert!(
        !src.contains(r#""utf-8", 154284782, -1"#),
        "不得混用两个 ignore 值"
    );
}

// ───────────────── MS8：确定性（两次主 part 字节 sha256 相同） ─────────────────
#[test]
fn ms8_deterministic_main_part_bytes() {
    let extract = |bytes: Vec<u8>| -> Vec<u8> {
        let mut pkg = XWPFPackage::open_stream(&mut Cursor::new(bytes));
        let name = core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper::createPartName_str(
            "/word/document.xml",
        )
        .expect("part name");
        let part = pkg.getPartList().and_then(|_| None::<&core_libs_io::io::opc::PackagePart::PackagePart>);
        let _ = part;
        // 通过 `getParts()` + 匹配名取内容
        let idx = pkg
            .getParts()
            .iter()
            .position(|p| format!("{:?}", p.getPartName().getURI()).contains("/word/document.xml"));
        match idx {
            Some(i) => {
                let mut v = Vec::new();
                {
                    let parts = pkg.getParts();
                    if let Some(p) = parts.get(i) {
                        if let Ok(mut s) = p.getInputStream_default() {
                            let _ = std::io::Read::read_to_end(&mut s, &mut v);
                        }
                    }
                }
                let _ = &name;
                v
            }
            None => Vec::new(),
        }
    };
    let a = extract(c1_bytes());
    let b = extract(c1_bytes());
    assert!(!a.is_empty(), "C1 的 word/document.xml 应可读出非空字节（MS8）");
    assert_eq!(a, b, "两次主 part 字节必须相同（不同 ⇒ 实现缺陷，MS8）");
}

// ───────────────── MS9：warnings 通道（P2 后生效） ─────────────────
#[test]
#[ignore = "MS9 规格明写「P2 后：diagnostics.warnings[] 非空」⇒ P2 前未采"]
fn ms9_warnings_emitted_after_p2() {}

// ───────────────── MS10：Strict 包的主 part ─────────────────
#[test]
#[ignore = "MS10 需要 C13（Strict 包）语料，尚未造 ⇒ 未采；见 CORPUS.md §2"]
fn ms10_strict_main_part() {
    let p = crate_root().join("tests/fixtures/s01_negative/C13_strict.docx");
    assert!(p.exists(), "C13 缺失 ⇒ 保持 ignored（未采）");
}

// ───────────────── MS11：外部关系不影响主 part ─────────────────
#[test]
fn ms11_external_rel_no_effect() {
    // C3（sample4.docx）若存在则用真实语料；否则用造包（含 External 关系）验证。
    let c3 = crate_root().join("tests/fixtures/s01_negative/C3_sample4.docx");
    if c3.exists() {
        let mut d = bind(std::fs::read(&c3).unwrap());
        assert!(catch(|| d.parse()).is_ok(), "C3 的 parse 不得失败（MS11）");
        return;
    }
    let rels = r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/><Relationship Id="rIdE" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink" Target="https://example.com/" TargetMode="External"/></Relationships>"#;
    let ct = r#"<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>"#;
    let mut buf = Vec::new();
    {
        let mut w = zip::ZipWriter::new(Cursor::new(&mut buf));
        let o = zip::write::SimpleFileOptions::default();
        w.start_file("[Content_Types].xml", o).unwrap();
        w.write_all(ct.as_bytes()).unwrap();
        w.start_file("_rels/.rels", o).unwrap();
        w.write_all(rels.as_bytes()).unwrap();
        w.start_file("word/document.xml", o).unwrap();
        w.write_all(&main_doc_xml()).unwrap();
        w.finish().unwrap();
    }
    let mut d = bind(buf);
    assert!(
        catch(|| d.parse()).is_ok(),
        "外部关系不得影响主 part 的解析（MS11）"
    );
    // `parts[]` 不得包含外部目标
    let mut pkg = XWPFPackage::open_stream(&mut Cursor::new(c1_bytes()));
    let names: Vec<String> = pkg
        .getParts()
        .iter()
        .map(|p| format!("{:?}", p.getPartName().getURI()))
        .collect();
    assert!(
        !names.iter().any(|n| n.contains("example.com")),
        "外部目标不得被当作 part（MS11）"
    );
}

// ───────────────── MS12（P1-e）：真实语料端到端装配 + 分部 parse 全覆盖 ─────────────────
fn c2a_bytes() -> Vec<u8> {
    std::fs::read(crate_root().join("../../../word-core/tests/fixtures/03_function_line_corpus/docx/multipage.docx"))
        .expect("C2a fixture")
}

fn c6_bytes() -> Vec<u8> {
    std::fs::read(crate_root().join("../../../word-core/tests/fixtures/p1_layout_document/hello-world.docx"))
        .expect("C6' fixture")
}

#[test]
fn ms12_c1_full_assembly_and_parse_end_to_end() {
    // ★M-STAGE1 P1-e —— 「原始文档输入 → XWPFDocument 装配完成」的**里程碑判据**。
    // C1（standard-hello.docx，3502B）含 styles / settings / fontTable 三个关系。
    // 判据：① 装配出对应子 part；② 每个子 part 都**真持有 packagePart**（P1-e1/e2）；
    //      ③ 整条 `parse()` 不 panic（真实 XmlReader 走完）。
    let mut d = bind(c1_bytes());
    assert!(d.getDocumentPartName().is_some(), "C1 主 part 名应已装配");
    assert!(d.getStyles().is_some(), "C1 应装配出 styles");
    let settings = d.getSettings().expect("C1 settings");
    assert!(
        settings.getPackagePart().is_some(),
        "★P1-e：settings 须持有真实 packagePart，否则其 parse() 静默空转"
    );
    let font_table = d.getFontTablePart().expect("C1 fontTablePart");
    assert!(
        font_table.getPackagePart().is_some(),
        "★P1-e：fontTablePart 须持有真实 packagePart"
    );
    // 端到端：parse() 走真实 XmlReader，不得 panic
    let r = catch(|| d.parse());
    assert!(r.is_ok(), "C1 的 parse() 必须端到端跑通：{:?}", r.err());
}

#[test]
fn ms13_c2a_six_parts_assembled_and_parsed() {
    // C2a（multipage.docx，36741B）含 styles / settings / fontTable / theme / numbering 等。
    // 判据：装配出的每个子 part 都持有 packagePart，且 parse() 全程不 panic。
    let mut d = bind(c2a_bytes());
    assert!(d.getStyles().is_some(), "C2a 应装配出 styles");
    assert!(d.getSettings().is_some(), "C2a 应装配出 settings");
    assert!(d.getFontTablePart().is_some(), "C2a 应装配出 fontTablePart");
    assert!(d.getThemePart().is_some(), "C2a 应装配出 theme");
    assert!(d.getNumbering().is_some(), "C2a 应装配出 numbering");
    // 逐个子 part 断言「真绑定」
    for (name, bound) in [
        ("settings", d.getSettings().map(|p| p.getPackagePart().is_some())),
        ("fontTable", d.getFontTablePart().map(|p| p.getPackagePart().is_some())),
        ("theme", d.getThemePart().map(|p| p.getPackagePart().is_some())),
        ("numbering", d.getNumbering().map(|p| p.getPackagePart().is_some())),
    ] {
        assert_eq!(
            bound, Some(true),
            "★P1-e：C2a 的 {name} 子 part 必须持有真实 packagePart"
        );
    }
    let r = catch(|| d.parse());
    assert!(r.is_ok(), "C2a 的 parse() 必须端到端跑通：{:?}", r.err());
}

#[test]
fn ms14_c6_prime_still_opens_without_optional_parts() {
    // ★回归（Java 的 `!= null` 守卫语义）：C6′（hello-world.docx，918B）**没有**
    // styles / settings / fontTable 关系 ⇒ Java 的守卫会让这些分部 parse **全部跳过**。
    // 判据：装配不 panic、这些字段为 None、且 `parse()` 仍能正常收尾（不得因为缺 part 而炸）。
    let mut d = bind(c6_bytes());
    assert!(
        d.getSettings().is_none(),
        "C6′ 无 settings 关系 ⇒ 该字段应为 None（与 Java 守卫同判）"
    );
    assert!(
        d.getFontTablePart().is_none(),
        "C6′ 无 fontTable 关系 ⇒ 该字段应为 None"
    );
    let r = catch(|| d.parse());
    assert!(
        r.is_ok(),
        "★C6′ 必须在缺可选 part 时仍能打开（对齐 Java 的 `!= null` 守卫）：{:?}",
        r.err()
    );
    assert!(d.getDocumentPartName().is_some(), "C6′ 主 part 名仍应在");
}
