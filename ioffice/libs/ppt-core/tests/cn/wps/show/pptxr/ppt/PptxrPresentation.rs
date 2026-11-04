//! `02_function_pres_open_global` —— **E 组矩阵：从真实 C1 用「真实 zip 字节手术」派生变体，
//! 覆盖 `openGlobalData()` / `openOtherGlobalData()` / `addDefaultProperty()` 的每一个分支**。
//!
//! ★ 每个变体都源自同一份真实 `Sample_ppt.pptx`：真实 `[Content_Types].xml`、真实 `_rels/.rels`、
//! 真实 159 个条目（只对目标条目做增删改），因此仍是**真实 OPC 包**，只是把某个分支推到另一端。
//! 没有任何合成最小包。
#[path = "_c1_real.rs"]
mod c1;
use c1::*;

use ppt_core::cn::wps::show::pptxr::ppt::PptxrPresentation::DEFAULT_SLDSIZE_CX;
use ppt_core::cn::wps::show::pptxr::PptxReader::getPresPart;
use ppt_core::cn::wps::show::pptxr::util::OpcrPackagePartRef::{
    relationship_part_by_type, OpcrPackagePartRef,
};

// ═══════════════════════════════════════════════════════════════
// E1：addDefaultProperty() 的默认值分支（删掉 C1 真实存在的那一个元素）
// ═══════════════════════════════════════════════════════════════

// java if (!presData.hasSlideSize()) ⇒ 写 12192000×6858000
#[test]
fn e01_drop_real_sldsz_falls_back_to_defaults() {
    let o = open_variant(
        "drop-sldsz",
        &[Edit::Text("ppt/presentation.xml", |x| {
            drop_self_closing(x, "p:sldSz")
        })],
    );
    let p = o.pres.borrow();
    // java addDefaultProperty()：缺失时写默认值 ⇒ open() 之后必然 hasSlideSize() == true，
    // 且值必须是 Java 默认值（12192000×6858000）
    assert!(p.hasSlideSize(), "默认值必须被 addDefaultProperty() 真实写入");
    assert_eq!((p.slideWidth(), p.slideHeight()), (DEFAULT_SLDSIZE_CX, 6_858_000));
    assert!(o.reader.openSteps().contains(&"addDefaultProperty".to_string()));
    // notesSz 仍在 ⇒ 不得被默认值覆盖（两个默认值分支互相独立）
    assert!(p.hasNotesSize());
    assert_eq!((p.notesWidth(), p.notesHeight()), (C1_NOTES_CX, C1_NOTES_CY));
}

// 把 C1 的真实 p:sldSz 改成 4:3 ⇒ 必须用**解析值**，不得被默认值覆盖
#[test]
fn e01b_real_sldsz_4_3_wins_over_defaults() {
    let o = open_variant(
        "sldsz-4x3",
        &[Edit::Text("ppt/presentation.xml", |x| {
            set_self_closing(x, "p:sldSz", r#"<p:sldSz cx="9144000" cy="6858000"/>"#)
        })],
    );
    let p = o.pres.borrow();
    assert!(p.hasSlideSize());
    assert_eq!(
        (p.slideWidth(), p.slideHeight()),
        (9_144_000, 6_858_000),
        "真实解析值必须胜过默认值"
    );
    // java getPresentationSlideSizeType：9144000×6858000 既非 16:9 也非 4:3 名单内 ⇒ 0
    assert_eq!(
        ppt_core::cn::wps::show::pptxr::ppt::PptxrPresentation::getPresentationSlideSizeType(&p),
        0
    );
}

// java if (!presData.hasNotesSize()) ⇒ 写 6858000×9144000
#[test]
fn e02_drop_real_notessz_falls_back_to_defaults() {
    let o = open_variant(
        "drop-notessz",
        &[Edit::Text("ppt/presentation.xml", |x| {
            drop_self_closing(x, "p:notesSz")
        })],
    );
    let p = o.pres.borrow();
    assert!(p.hasNotesSize(), "默认值必须被 addDefaultProperty() 真实写入");
    assert_eq!((p.notesWidth(), p.notesHeight()), (C1_NOTES_CX, C1_NOTES_CY));
    assert!(p.hasSlideSize(), "sldSz 仍在 ⇒ 不得被默认值覆盖");
    assert_eq!((p.slideWidth(), p.slideHeight()), (C1_SLD_CX, C1_SLD_CY));
}

// 把 C1 的真实 p:notesSz 改成非默认值 ⇒ 必须用**解析值**
#[test]
fn e02b_real_notessz_wins_over_defaults() {
    let o = open_variant(
        "notessz-custom",
        &[Edit::Text("ppt/presentation.xml", |x| {
            set_self_closing(x, "p:notesSz", r#"<p:notesSz cx="1234567" cy="7654321"/>"#)
        })],
    );
    let p = o.pres.borrow();
    assert!(p.hasNotesSize());
    assert_eq!((p.notesWidth(), p.notesHeight()), (1_234_567, 7_654_321));
}

// ═══════════════════════════════════════════════════════════════
// E2：openGlobalData() 的「关系不存在 / part 不存在」分支
// ═══════════════════════════════════════════════════════════════

// java `if (themeRels != null)` 的 else 侧
#[test]
fn e03_drop_theme_rel_skips_set_theme() {
    let o = open_variant("drop-theme", &[Edit::DropRelType("theme")]);
    assert!(
        !o.reader.openSteps().contains(&"setTheme".to_string()),
        "无 theme 关系 ⇒ 不得 setTheme"
    );
    assert!(o.pres.borrow().getTheme().is_none());
    // 其余真实全局数据不受影响
    assert_eq!(o.pres.borrow().tableStylesOpened(), 1);
    assert_eq!(o.pres.borrow().slideCount(), C1_SLIDES);
}

// java `if (stylesRels != null) { if (part != null) ... }` 的 else 侧
#[test]
fn e04_drop_table_styles_rel_skips_parse() {
    let o = open_variant("drop-tablestyles", &[Edit::DropRelType("tableStyles")]);
    let p = o.pres.borrow();
    assert_eq!(p.tableStylesOpened(), 0);
    assert_eq!(p.tableStyleDefault(), None);
    assert!(!o.reader.openSteps().contains(&"tableStyles.open".to_string()));
    // theme 仍在 ⇒ setTheme 不受影响
    assert!(p.getTheme().is_some());
}

// 关系在、但目标条目不在包里 ⇒ Java `part == null` 侧（跳过，不崩、不静默成功）
#[test]
fn e05_drop_table_styles_part_keeps_rel() {
    let path = variant("drop-tablestyles-part", &[Edit::DropEntry("ppt/tableStyles.xml")]);

    // 关系本身仍在（真实 rels 里 tableStyles 关系未被删）
    let mut pack =
        kfc_kfc_alpha::cn::wps::kfc::ooxfr::OxfrPackage::OxfrPackage::OxfrPackage(&path)
            .expect("variant is a real OPC package");
    let part = getPresPart(&mut pack).expect("pres part");
    let handle = OpcrPackagePartRef::of(part).expect("handle");
    let mut rels = handle.getRelationships().expect("rels");
    let ty = kfc_kfc_alpha::cn::wps::kfc::ooxfr::OxfPptRelations::OxfPptRelations::tableStyles()
        .getRelsType();
    let found = rels.getRelationshipByType(&ty).and_then(|r| r.getTarget());
    assert!(found.is_some(), "tableStyles 关系必须仍在（只删了 part 条目）");

    // 目标条目缺失 ⇒ part 解析不出来 ⇒ Importer 走「part == null」侧，不得报错也不得假成功
    let o = open_path(&path);
    assert_eq!(o.pres.borrow().tableStylesOpened(), 0);
    assert!(!o.reader.openSteps().contains(&"tableStyles.open".to_string()));
    assert_eq!(o.pres.borrow().slideCount(), C1_SLIDES, "其余装配不受影响");
}

// ═══════════════════════════════════════════════════════════════
// E3：openOtherGlobalData() 的 presProps / viewProps 两侧分支
// ═══════════════════════════════════════════════════════════════

// java `if (presPropsRels != null)` 的 else 侧
#[test]
fn e06_drop_pres_props_rel_skips() {
    let o = open_variant("drop-presprops", &[Edit::DropRelType("presProps")]);
    let steps = o.reader.openSteps();
    assert!(!steps.contains(&"presProps.open".to_string()));
    // viewProps 仍在 ⇒ 不受影响
    assert!(steps.contains(&"viewProps.open".to_string()));
}

// java `if (viewPropsRels != null) ... else { buildDefaultViewProperty() }` 的 else 侧
#[test]
fn e07_drop_view_props_rel_takes_default_branch() {
    let o = open_variant("drop-viewprops", &[Edit::DropRelType("viewProps")]);
    let steps = o.reader.openSteps();
    assert!(steps.contains(&"viewProps.default".to_string()), "{steps:?}");
    assert!(!steps.contains(&"viewProps.open".to_string()));
    assert_eq!(o.pres.borrow().view_property_written, 1, "默认 viewProps 仍必须写回模型");
    assert!(steps.contains(&"presProps.open".to_string()));
}

// java PptxrPresProps —— 把真实属性注入 C1 自己的 presProps.xml：
// 真实 part 必须仍被完整读取且不报错（★取值解析本身是登记 DEFERRED D3：
// `PresPropsHandler` 的 id↔name 表在 Rust 侧仍是 `PMap` 桩，owner: pp-03 / wave: S4）
#[test]
fn e08_inject_real_pres_props_values() {
    const INJECTED: &str = " serverZoom=\"50\" showSpecialPlsOnTitleSld=\"1\" rtl=\"1\"";
    fn inject_pres(x: String) -> String {
        inject_attrs(x, "p:presentationPr", INJECTED)
    }
    let o = open_variant(
        "inject-presprops",
        &[Edit::Text("ppt/presProps.xml", inject_pres)],
    );
    assert!(
        o.reader.openSteps().contains(&"presProps.open".to_string()),
        "真实 presProps part 必须被读取"
    );
    // 真实 part 被真实读取（不是空文件）：注入后 C1 的 presProps 仍被读流
    let path = variant(
        "inject-presprops-bytes",
        &[Edit::Text("ppt/presProps.xml", inject_pres)],
    );
    let mut pack =
        kfc_kfc_alpha::cn::wps::kfc::ooxfr::OxfrPackage::OxfrPackage::OxfrPackage(&path).unwrap();
    let part = getPresPart(&mut pack).unwrap();
    let handle = OpcrPackagePartRef::of(part).unwrap();
    let mut rels = handle.getRelationships().unwrap();
    let ty = kfc_kfc_alpha::cn::wps::kfc::ooxfr::OxfPptRelations::OxfPptRelations::presProps()
        .getRelsType();
    let target = relationship_part_by_type(&mut rels, &ty)
        .unwrap()
        .expect("presProps rel resolves");
    let bytes = target.read_all().expect("real presProps stream readable");
    assert!(
        String::from_utf8_lossy(&bytes).contains("serverZoom=\"50\""),
        "注入的真实属性必须在真实读到的字节里"
    );
}

// java PptxrViewProps 的真实**取值**分支：把真实属性注入 C1 自己的 viewProps.xml。
//
// ★ Java 元素归属（`ViewPropsHandler`）：
//   - `showComments`(3211659) 在 `p:viewPr`（根）上读；
//   - `snapToGrid`(3211647)/`snapToObjects`(3211648) 在 `p:cSldViewPr` 上读
//     （`ViewPropsHandler.CommonSlideViewPrHandler`），**不是** `p:normalViewPr`。
// 本用例把属性注入到 Java 归属的那个真实元素上。
#[test]
fn e09_inject_real_view_props_values() {
    let path = variant(
        "inject-viewprops",
        &[Edit::Text("ppt/viewProps.xml", |x| {
            let x = inject_attrs(x, "p:viewPr", " showComments=\"1\"");
            set_attrs(x, "p:cSldViewPr", " snapToGrid=\"1\" snapToObjects=\"1\"")
        })],
    );
    // Importer 侧：真实 part 走 viewProps.open 分支
    let o = open_path(&path);
    assert!(o.reader.openSteps().contains(&"viewProps.open".to_string()));
    assert!(!o.reader.openSteps().contains(&"viewProps.default".to_string()));
    assert_eq!(o.pres.borrow().view_property_written, 1);

    // 端口侧：用真实 part 解析出真实值（与 Importer 同一构造方式）
    let mut reader = ppt_core::cn::wps::show::pptxr::ppt::PptxrViewProps::PptxrViewProps::PptxrViewProps(
        Default::default(),
        Some(view_props_part(&path)),
    );
    reader.open().expect("real viewProps open");
    let vp = reader.viewProperty();
    assert_eq!(vp.showComments, Some(true), "真实 showComments 未被解析");
    assert_eq!(vp.snapToGrid, Some(true), "真实 p:cSldViewPr/@snapToGrid 未被解析");
    assert_eq!(vp.snapToObjects, Some(true));
    assert!(reader.bytesRead() > 0);
}

/// 取变体里真实 `ppt/viewProps.xml` 的 part 句柄（与 Importer 定位方式一致）。
fn view_props_part(path: &str) -> OpcrPackagePartRef {
    let mut pack =
        kfc_kfc_alpha::cn::wps::kfc::ooxfr::OxfrPackage::OxfrPackage::OxfrPackage(path).unwrap();
    let part = getPresPart(&mut pack).unwrap();
    let handle = OpcrPackagePartRef::of(part).unwrap();
    let mut rels = handle.getRelationships().unwrap();
    let ty = kfc_kfc_alpha::cn::wps::kfc::ooxfr::OxfPptRelations::OxfPptRelations::viewProps()
        .getRelsType();
    relationship_part_by_type(&mut rels, &ty)
        .unwrap()
        .expect("viewProps rel resolves")
}

// 真实 C1（未改）的 viewProps 真实取值：`p:cSldViewPr snapToGrid="0" showGuides="1"`
#[test]
fn e09b_real_c1_view_props_value() {
    let mut reader = ppt_core::cn::wps::show::pptxr::ppt::PptxrViewProps::PptxrViewProps::PptxrViewProps(
        Default::default(),
        Some(view_props_part(C1)),
    );
    reader.open().expect("real C1 viewProps open");
    let vp = reader.viewProperty();
    assert_eq!(vp.snapToGrid, Some(false), "C1 真实 p:cSldViewPr/@snapToGrid=0");
    // C1 的 p:viewPr 上没有 showComments ⇒ 保持未设置（不得被默认值顶替）
    assert_eq!(vp.showComments, None);
    assert!(reader.bytesRead() > 0);
}

// ═══════════════════════════════════════════════════════════════
// E4：open() 的 blank-slide 分支（改 C1 自己的 p:sldIdLst）
// ═══════════════════════════════════════════════════════════════

// java `if (this.mPres.slideCount() == 0)` 的 then 侧：清空真实 sldIdLst
#[test]
fn e10_empty_sld_id_lst_creates_blank_slide() {
    let o = open_variant(
        "empty-sldidlst",
        &[Edit::Text("ppt/presentation.xml", |x| set_sld_id_lst(x, ""))],
    );
    let p = o.pres.borrow();
    assert_eq!(p.slideCount(), 1, "无真实幻灯片 ⇒ 建且仅建 1 张空白页");
    assert!(p.isHasBlankSlide());
    assert!(o.listener.borrow().calls[0].0, "必须回调 hasBlankSlide=true");
    // 空白页继承真实 slideSize
    assert_eq!((p.slideWidth(), p.slideHeight()), (C1_SLD_CX, C1_SLD_CY));
    let s = p.slides.first().expect("blank slide exists");
    assert_eq!((s.slide_width, s.slide_height), (C1_SLD_CX, C1_SLD_CY));
}

// java else 侧：只保留 1 条真实 sldId
#[test]
fn e11_single_sld_id_no_blank_slide() {
    let o = open_variant(
        "single-sldid",
        &[Edit::Text("ppt/presentation.xml", |x| {
            set_sld_id_lst(x, r#"<p:sldId id="257" r:id="rId2"/>"#)
        })],
    );
    let p = o.pres.borrow();
    assert_eq!(p.slideCount(), 1);
    assert!(!p.isHasBlankSlide());
    assert!(!o.listener.borrow().calls[0].0);
}

// `onSlideIds` 的 rId 顺序必须决定装配顺序（真实 sldIdLst 改成逆序）
#[test]
fn e12_reversed_sld_id_order_is_followed() {
    let o = open_variant(
        "reversed-sldid",
        &[Edit::Text("ppt/presentation.xml", |x| {
            set_sld_id_lst(
                x,
                r#"<p:sldId id="263" r:id="rId24"/><p:sldId id="264" r:id="rId23"/><p:sldId id="259" r:id="rId22"/>"#,
            )
        })],
    );
    {
        let p = o.pres.borrow();
        assert_eq!(p.slideCount(), 3, "3 条 rId 都指向真实存在的 slide");
        assert!(!p.isHasBlankSlide());
    }
    // 装配顺序 = sldIdLst 顺序：`mSlideIdMap` 的 slideId 按创建顺序递增
    // ⇒ slide24.xml 必须是第 1 张（id=1），依次 slide23.xml=2、slide22.xml=3
    use ppt_core::cn::wps::show::pptxr::ppt::slides::PptxrSlides::PptxrSlides;
    let id_of = |f: &str| PptxrSlides::getSlideIdByFileName(Some(f.to_string()));
    // C1 真实映射：rId24→slides/slide23.xml、rId23→slides/slide22.xml、rId22→slides/slide21.xml
    assert_eq!(id_of("slide23.xml"), 1, "sldIdLst 顺序未被遵循");
    assert_eq!(id_of("slide22.xml"), 2);
    assert_eq!(id_of("slide21.xml"), 3);
}

// rId 指向不存在的关系 ⇒ 该条被跳过（Java `if (rels != null)`），不 panic
#[test]
fn e13_dangling_rid_is_skipped() {
    let o = open_variant(
        "dangling-rid",
        &[Edit::Text("ppt/presentation.xml", |x| {
            set_sld_id_lst(
                x,
                r#"<p:sldId id="257" r:id="rId2"/><p:sldId id="999" r:id="rId999"/>"#,
            )
        })],
    );
    assert_eq!(o.pres.borrow().slideCount(), 1, "悬空 rId 必须被跳过");
}

// 删掉真实 slideMaster 关系后仍需装配成功（master 属 F3 端口面）
#[test]
fn e14_drop_slide_master_rel_still_assembles() {
    let o = open_variant("drop-slidemaster", &[Edit::DropRelType("slideMaster")]);
    let p = o.pres.borrow();
    assert_eq!(p.slideCount(), C1_SLIDES, "幻灯片装配不受 master 关系影响");
    assert!(p.hasInitialized());
    assert!(p.getTheme().is_some());
}

// ═══════════════════════════════════════════════════════════════
// E5：变体的交叉不变量（保证手术没有把包改坏）
// ═══════════════════════════════════════════════════════════════

// 每个变体都必须是**真实、可打开、可定位 pres part** 的 OPC 包
#[test]
fn e15_every_variant_is_a_real_openable_package() {
    let cases: Vec<(&str, Vec<Edit>)> = vec![
        (
            "v-drop-sldsz",
            vec![Edit::Text("ppt/presentation.xml", |x| {
                drop_self_closing(x, "p:sldSz")
            })],
        ),
        ("v-drop-notessz", vec![Edit::DropRelType("tags")]),
        ("v-drop-theme", vec![Edit::DropRelType("theme")]),
        ("v-drop-tablestyles", vec![Edit::DropRelType("tableStyles")]),
        ("v-drop-presprops", vec![Edit::DropRelType("presProps")]),
        ("v-drop-viewprops", vec![Edit::DropRelType("viewProps")]),
        (
            "v-empty-sldidlst",
            vec![Edit::Text("ppt/presentation.xml", |x| set_sld_id_lst(x, ""))],
        ),
    ];
    for (tag, edits) in cases {
        let path = variant(tag, &edits);
        let mut pack =
            kfc_kfc_alpha::cn::wps::kfc::ooxfr::OxfrPackage::OxfrPackage::OxfrPackage(&path)
                .unwrap_or_else(|e| panic!("{tag} 不是真实 OPC 包: {e:?}"));
        getPresPart(&mut pack).unwrap_or_else(|e| panic!("{tag} 定位 pres part 失败: {e:?}"));
        // 真实条目数必须仍在同一量级（确认只做了局部手术）
        let z = zip::ZipArchive::new(std::io::Cursor::new(std::fs::read(&path).unwrap())).unwrap();
        assert!(z.len() >= C1_ENTRIES - 1, "{tag} 条目数异常: {}", z.len());
    }
}
