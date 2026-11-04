//! `03_function_pres_open` —— **真实 C1（`Sample_ppt.pptx`）驱动的完整测试矩阵（A 组：全量 + 鲁棒 + 错误面 + 产品入口）**。
//!
//! ★ 设计：整个矩阵只从**一份真实 `.pptx`（C1）**出发。
//! - A/B/C/D 组直接跑 C1 原始字节（23 张真实幻灯片、真实 theme/tableStyles/presProps/viewProps、
//!   真实 slideMaster、真实 tags 关系）；
//! - 分支用例（E 组，见 `PptxrPresentation.rs`）用**真实 zip 字节手术**从 C1 派生变体，
//!   派生物仍是真实 OPC 包（真实 `[Content_Types].xml`、真实 `_rels/.rels`、真实 part 关系）；
//! - **不再使用任何合成最小包**。真实存在的残缺语料 `hello.pptx`（无 `_rels/.rels`）仅在 F 组
//!   作为「必须被拒绝」的负例。
use std::cell::RefCell;
use std::rc::Rc;

#[path = "_c1_real.rs"]
mod c1;
use c1::*;

use kfc_kfc_alpha::cn::wps::kfc::ooxfr::OxfrPackage::OxfrPackage;
use ppt_core::cn::wps::show::pptxr::ppt::PptxrPresentation::{
    getPresentationSlideSizeType, DEFAULT_NODESIZE_CX, DEFAULT_NODESIZE_CY,
};
use ppt_core::cn::wps::show::app::event::KmoPresOpenEventListener::KmoPresOpenEventListener;
use ppt_core::cn::wps::show::pptxr::PptxReader::{getPresPart, PptxReader};
use ppt_core::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use ppt_core::cn::wps::show::util::layout_shape::KmoPresentation;

/// CORPUS C2 —— 真实存在的**残缺**语料（不是完整 OPC 包）。
const C2: &str = "../../samples/ppt-render/flutter/assets/hello.pptx";

// ═══════════════════════════════════════════════════════════════
// A 组：C1 全量装配 —— slideSize / notesSize / theme / 页数 / 初始化 / 回调
// ═══════════════════════════════════════════════════════════════

// java public void open() —— C1 真实的 slideSize 与 notesSize 必须被**真实解析**（非默认值路径）
#[test]
fn a01_c1_real_slide_and_notes_size() {
    let o = open_c1();
    let p = o.pres.borrow();
    assert_eq!((p.slideWidth(), p.slideHeight()), (C1_SLD_CX, C1_SLD_CY));
    assert_eq!((p.notesWidth(), p.notesHeight()), (C1_NOTES_CX, C1_NOTES_CY));
    assert!(p.hasSlideSize(), "C1 有 p:sldSz ⇒ 必须走真实解析值");
    assert!(p.hasNotesSize(), "C1 有 p:notesSz ⇒ 必须走真实解析值");
    // java this.mPres.setInitialized(true)
    assert!(p.hasInitialized());
}

// java KmoPresentation.getTheme() —— C1 的 theme 关系必须真实装载
#[test]
fn a02_c1_real_theme_loaded() {
    let o = open_c1();
    assert!(
        o.pres.borrow().getTheme().is_some(),
        "C1 有 theme 关系 ⇒ setTheme 必须被真实调用"
    );
    assert!(o.reader.openSteps().contains(&"setTheme".to_string()));
}

// java PresentationHandlerCallback.onSlideIds → mPres.addSlide —— C1 真实 23 张
#[test]
fn a03_c1_real_slide_count_23() {
    let o = open_c1();
    let p = o.pres.borrow();
    assert_eq!(p.slideCount(), C1_SLIDES, "C1 的 p:sldIdLst 有 23 条");
    assert!(!p.isHasBlankSlide(), "有真实幻灯片 ⇒ 不得建空白页");
}

// java this.mOpenListener.docDidFinishInitializing(this.mPres, false) —— 真实回调入参
#[test]
fn a04_c1_listener_receives_real_assembly() {
    let o = open_c1();
    let l = o.listener.borrow();
    assert_eq!(l.calls.len(), 1, "docDidFinishInitializing 必须恰好回调一次");
    let (blank, count, w, h) = l.calls[0];
    assert!(!blank);
    // java `PptxrSlides.UnReadSlides.getCreateSlideCount(mPres, 23)` = min(23,10) = 10
    // ⇒ 回调时**热装配** 10 张；其余 13 张由随后的 `slidesReader.open()` 真实补齐（见 a06）
    assert_eq!(count, C1_EAGER_SLIDES, "回调时已热装配的页数必须是 Java 的 min(n,10)");
    assert_eq!((w, h), (C1_SLD_CX, C1_SLD_CY));
    assert_eq!(l.doc_done, 0, "docDidFinishReadingEverything 不在 open() 方法内");
}

// java `slidesReader.open()` 补齐剩余幻灯片 —— 终态必须是 C1 的真实 23 张
#[test]
fn a06_c1_lazy_slides_completed_by_slides_open() {
    let o = open_c1();
    assert_eq!(
        o.pres.borrow().slideCount(),
        C1_SLIDES,
        "open() 返回时 23 张必须全部到位（前 10 张热装配 + 其余由 slides.open() 补齐）"
    );
    assert!(o.reader.openSteps().contains(&"slides.open".to_string()));
}

// java this.mPres.addSlide / selection().init() —— C1 有 23 张 ⇒ selection 必须被初始化
#[test]
fn a05_c1_selection_initialized_with_real_slides() {
    let o = open_c1();
    let p = o.pres.borrow();
    assert_eq!(p.slideCount(), C1_SLIDES);
    assert_eq!(p.selection().getHistorySlideIndex(), 0);
    assert!(p.slideSizeType() == 0 || p.hasSlideSizeType());
}

// ═══════════════════════════════════════════════════════════════
// B 组：openGlobalData 的真实分支（C1 全部命中「有 part」侧）+ 步骤顺序
// ═══════════════════════════════════════════════════════════════

// java PptxrTableStyles —— C1 真实 tableStyles.xml：`@def` 有值、style 列表为空
#[test]
fn b01_c1_real_table_styles() {
    let o = open_c1();
    let p = o.pres.borrow();
    assert_eq!(p.tableStylesOpened(), 1, "C1 有 tableStyles 关系 ⇒ 必须真实解析");
    assert_eq!(p.tableStyleDefault().as_deref(), Some(C1_TBL_DEF));
    assert_eq!(p.tableStyleCount(), 0, "C1 的 a:tblStyleLst 内没有 style 条目");
}

// java openGlobalData() 步骤顺序：setTheme → tableStyles.open → presentationXml.read → addDefaultProperty
#[test]
fn b02_c1_real_step_order() {
    let o = open_c1();
    let steps = o.reader.openSteps();
    let idx = |n: &str| {
        steps
            .iter()
            .position(|s| s == n)
            .unwrap_or_else(|| panic!("openSteps 缺少 {n}: {steps:?}"))
    };
    assert!(idx("slides.ctor") < idx("slides.initialize"));
    assert!(idx("slides.initialize") < idx("openGlobalData"));
    assert!(idx("setTheme") < idx("tableStyles.open"));
    assert!(idx("tableStyles.open") < idx("presentationXml.read"));
    assert!(idx("presentationXml.read") < idx("addDefaultProperty"));
    assert!(idx("addDefaultProperty") < idx("docDidFinishInitializing.false"));
}

// ═══════════════════════════════════════════════════════════════
// C 组：openOtherGlobalData —— presProps / viewProps / commentAuthors / items / media
// ═══════════════════════════════════════════════════════════════

// java PptxrPresProps / PptxrViewProps —— C1 的两个真实 part 必须被读取与解析
#[test]
fn c01_c1_real_pres_and_view_props_parts() {
    let o = open_c1();
    let steps = o.reader.openSteps();
    assert!(steps.contains(&"presProps.open".to_string()));
    assert!(steps.contains(&"viewProps.open".to_string()));
    assert!(
        !steps.contains(&"viewProps.default".to_string()),
        "C1 有真实 viewProps part ⇒ 不得走默认值分支"
    );
    assert_eq!(o.pres.borrow().view_property_written, 1);
}

// java `commentAuthorsRels` —— C1 **没有** commentAuthors part
#[test]
fn c02_c1_no_comment_authors_part() {
    let o = open_c1();
    let steps = o.reader.openSteps();
    assert!(!steps.contains(&"commentAuthors.open".to_string()));
    assert_eq!(o.reader.commentIdContext().mapped_len(), 0);
}

// java `ITEM()/ITEMPROPS()` —— C1 **没有** item/itemProps 关系（有 tags 关系，但不是 item）
#[test]
fn c03_c1_no_item_parts() {
    let o = open_c1();
    let p = o.pres.borrow();
    assert!(p.items().is_empty(), "C1 无 ITEM 关系");
    assert!(p.itemProps().is_empty(), "C1 无 ITEMPROPS 关系");
}

// java `mMedias.openMedia(...)` —— C1 的 presentation 级无媒体关系 ⇒ 全局媒体表为空
#[test]
fn c04_c1_no_presentation_level_media() {
    let o = open_c1();
    let steps = o.reader.openSteps();
    assert!(
        !steps.iter().any(|s| s.starts_with("media.")),
        "presentation 级无媒体关系：{steps:?}"
    );
    assert!(o.pres.borrow().mediaPriorityReader().is_none());
}

// ═══════════════════════════════════════════════════════════════
// D 组：getPresentationSlideSizeType + addDefaultProperty 的真实语义
// ═══════════════════════════════════════════════════════════════

// java static getPresentationSlideSizeType(KmoPresentation) —— C1 是 16:9 ⇒ 15
#[test]
fn d01_c1_slide_size_type_16_9() {
    let o = open_c1();
    assert_eq!(getPresentationSlideSizeType(&o.pres.borrow()), 15);
}

// java addDefaultProperty()：真实值在场时**不得**被默认值覆盖
#[test]
fn d02_c1_defaults_do_not_override_real_values() {
    let o = open_c1();
    let p = o.pres.borrow();
    assert!(p.hasSlideSize() && p.hasNotesSize());
    // C1 的真实值与 Java 默认值常量恰好相同 ⇒ 双向锚定：任一侧漂移本断言即失败
    assert_eq!((p.slideWidth(), p.slideHeight()), (C1_SLD_CX, C1_SLD_CY));
    assert_eq!((p.notesWidth(), p.notesHeight()), (DEFAULT_NODESIZE_CX, DEFAULT_NODESIZE_CY));
}

// ═══════════════════════════════════════════════════════════════
// E 组：幂等 / 语料固定
// ═══════════════════════════════════════════════════════════════

// 同一份真实文件装配两次 ⇒ 装配面逐项一致（无跨次污染）
#[test]
fn e01_c1_open_twice_is_deterministic() {
    let snap = |o: &Opened| {
        let p = o.pres.borrow();
        (
            p.slideCount(),
            p.slideWidth(),
            p.slideHeight(),
            p.notesWidth(),
            p.notesHeight(),
            p.tableStylesOpened(),
            p.tableStyleCount(),
            p.view_property_written,
            p.getTheme().is_some(),
            p.hasInitialized(),
            o.reader.openSteps(),
        )
    };
    assert_eq!(snap(&open_c1()), snap(&open_c1()));
}

// CORPUS 冻结：C1 的字节长度 / 条目数 / 23 页 / 真实 sldSz / notesSz / presentation 属性
#[test]
fn e02_c1_corpus_is_pinned() {
    let bytes = c1_bytes();
    assert_eq!(bytes.len() as u64, C1_LEN);
    let mut z = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
    assert_eq!(z.len(), C1_ENTRIES);
    let mut xml = String::new();
    std::io::Read::read_to_string(&mut z.by_name("ppt/presentation.xml").unwrap(), &mut xml).unwrap();
    assert_eq!(xml.matches("<p:sldId ").count() as i32, C1_SLIDES);
    assert!(xml.contains(&format!("<p:sldSz cx=\"{C1_SLD_CX}\" cy=\"{C1_SLD_CY}\"/>")));
    assert!(xml.contains(&format!("<p:notesSz cx=\"{C1_NOTES_CX}\" cy=\"{C1_NOTES_CY}\"/>")));
    assert!(xml.contains(&format!("{}=\"{}\"", C1_PRES_ATTR.0, C1_PRES_ATTR.1)));
}

// ═══════════════════════════════════════════════════════════════
// F 组：错误面（真实残缺语料 / 字节损坏）
// ═══════════════════════════════════════════════════════════════

// 真实残缺语料 C2（无 `_rels/.rels`）⇒ 真实 OPC 路线必须报 FileDamaged
#[test]
fn f01_real_incomplete_package_is_rejected() {
    let mut pack = OxfrPackage::OxfrPackage(C2).expect("hello.pptx 仍是可打开的 zip");
    assert!(
        getPresPart(&mut pack).is_err(),
        "hello.pptx 无 _rels/.rels ⇒ 不得被当作完整 OPC 包"
    );
}

// C1 字节截断 ⇒ 真实错误面（不是 panic、不是静默成功）
#[test]
fn f02_c1_truncated_bytes_rejected() {
    let bytes = c1_bytes();
    let mut trunc = bytes;
    trunc.truncate(trunc.len() / 2);
    let bad = std::env::temp_dir().join(format!("m_ppt_pres_trunc-{}.pptx", std::process::id()));
    std::fs::write(&bad, &trunc).unwrap();
    let mut pack = OxfrPackage::OxfrPackage(&bad.to_string_lossy()).expect("zip 头仍在");
    assert!(getPresPart(&mut pack).is_err(), "截断包必须被拒绝");
}

// ═══════════════════════════════════════════════════════════════
// G 组：产品入口（真实接线，无旁路）
// ═══════════════════════════════════════════════════════════════

// 产品入口 `PptxReader.open()` 打开**真实 C1** ⇒ 真 Importer 装配进真实模型，正文真的落到模型里
struct FinishListener {
    events: Rc<RefCell<Vec<&'static str>>>,
}

impl KmoPresOpenEventListener for FinishListener {
    fn docDidFinishInitializing(&mut self, _pres: &mut KmoPresentation, _has_blank: bool) {
        self.events.borrow_mut().push("docDidFinishInitializing");
    }
    fn slideDidFinishReadingEverything(&mut self, _slide_index: i32) {}
    fn docDidFinishReadingEverything(&mut self) {
        self.events.borrow_mut().push("docDidFinishReadingEverything");
    }
}

#[test]
fn g01_product_entry_opens_c1_through_real_importer() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let model = Rc::new(RefCell::new(KmoPresentation::default()));
    let listener = Rc::new(RefCell::new(FinishListener { events: events.clone() }));
    let mut reader = PptxReader::PptxReader(Some(model.clone()), C1, Some(listener), false, None);
    reader.open().expect("PptxReader.open on real C1");
    let assembled = reader.assembled_pres().expect("real importer ran");
    assert!(Rc::ptr_eq(&model, &assembled), "open() 必须写回调用方传入的同一份模型");
    let p = model.borrow();
    assert!(p.hasInitialized());
    assert_eq!(p.file_format, 32);
    assert!(!p.read_only);
    assert_eq!(p.app_application.as_deref(), Some("Microsoft Office PowerPoint"));
    assert_eq!(p.core_title.as_deref(), Some("PowerPoint 演示文稿"));
    assert_eq!(p.core_creator.as_deref(), Some("墨香 大人"));
    assert_eq!(p.slideCount(), C1_SLIDES);
    assert_eq!((p.slideWidth(), p.slideHeight()), (C1_SLD_CX, C1_SLD_CY));
    assert!(p.getTheme().is_some());
    // 真实 slide XML 里的正文必须落进 `KmoShowShape.getTextContent`
    let has_title = p.slides.iter().any(|slide| {
        slide
            .kmo_sp_root
            .as_ref()
            .is_some_and(|root| tree_contains(root, "年终") || tree_contains(root, "蓝色"))
    });
    assert!(has_title, "slide xml 正文未落进装配模型");
    assert_eq!(
        events.borrow().as_slice(),
        ["docDidFinishInitializing", "docDidFinishReadingEverything"]
    );
    assert!(reader.mPackage.is_none(), "finally 必须关掉包");
}

fn tree_contains(root: &KmoShowShape, needle: &str) -> bool {
    if root.getTextContent().contains(needle) {
        return true;
    }
    (0..root.childCount()).any(|i| tree_contains(root.childAt(i), needle))
}

fn count_nodes(root: &KmoShowShape) -> usize {
    (0..root.childCount())
        .map(|i| 1 + count_nodes(root.childAt(i)))
        .sum()
}

fn count_ph(root: &KmoShowShape) -> usize {
    (0..root.childCount())
        .map(|i| {
            let child = root.childAt(i);
            usize::from(child.isPlaceHolder()) + count_ph(child)
        })
        .sum()
}

/// C1 逐页形状树必须与独立 oracle 的 depth1 / recursive 一致，组不能被拍平。
#[test]
fn g02_c1_shape_tree_matches_oracle_nesting() {
    let model = Rc::new(RefCell::new(KmoPresentation::default()));
    let mut reader = PptxReader::PptxReader(Some(model.clone()), C1, None, false, None);
    reader.open().expect("PptxReader.open");
    let p = model.borrow();
    const DEPTH1: [usize; 23] = [
        7, 31, 12, 24, 23, 37, 39, 13, 12, 9, 11, 20, 9, 16, 12, 31, 11, 30, 12, 26, 13, 12, 7,
    ];
    const RECURSIVE: [usize; 23] = [
        9, 41, 23, 30, 34, 49, 39, 66, 23, 25, 23, 32, 11, 38, 23, 31, 21, 35, 23, 26, 25, 36, 9,
    ];
    assert_eq!(p.slides.len(), 23);
    for (i, slide) in p.slides.iter().enumerate() {
        let root = slide.kmo_sp_root.as_ref().expect("kmo shape tree");
        assert_eq!(root.r#type(), 0, "slide{} spTree is a group", i + 1);
        assert_eq!(root.childCount() as usize, DEPTH1[i], "slide{} depth1", i + 1);
        assert_eq!(count_nodes(root), RECURSIVE[i], "slide{} recursive", i + 1);
    }
    let slide8 = p.slides[7].kmo_sp_root.as_ref().expect("slide8 tree");
    let mut groups = 0usize;
    let mut nested = false;
    for i in 0..slide8.childCount() {
        let child = slide8.childAt(i);
        if child.r#type() == 0 {
            groups += 1;
            nested |= child.childCount() > 0;
        }
    }
    assert_eq!(groups, 8, "slide8 顶层 grpSp");
    assert!(nested, "slide8 的组必须持有子形状");
    const PH: [usize; 23] = [
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5,
    ];
    const CNVPR: [usize; 23] = [
        10, 42, 24, 31, 35, 50, 40, 67, 24, 26, 24, 33, 12, 39, 24, 32, 22, 36, 24, 27, 26, 37, 10,
    ];
    for (i, slide) in p.slides.iter().enumerate() {
        let root = slide.kmo_sp_root.as_ref().expect("kmo shape tree");
        assert_eq!(count_ph(root), PH[i], "slide{} ph", i + 1);
        assert_eq!(slide.cnvpr_count as usize, CNVPR[i], "slide{} cNvPr id", i + 1);
    }
}

/// `KmoFilm.protocol` / `updateProtocol` 经同一份 `FilmAtomData.cSlide` 往返名字。
#[test]
fn g03_kmo_film_protocol_roundtrips_name() {
    use ppt_core::cn::wps::show::app::slide::KmoFilm::KmoFilm;
    use ppt_core::cn::wps::show::pptxr::handler::shape::SwGroupShapeHandler::ImportHost;

    let host = ImportHost::leak();
    let mut film = KmoFilm::KmoFilm(host);
    let tree_a = film.spTree() as *mut _;
    let tree_b = film.spTree() as *mut _;
    assert_eq!(tree_a, tree_b, "两次 spTree() 必须是同一个 KmoShapeTree");
    film.setName("年终".to_string());
    assert!(film.hasName());
    assert_eq!(film.name(), "年终");
    let bin = film.protocol();
    let mut loaded = KmoFilm::KmoFilm(host);
    loaded.updateProtocol(bin);
    assert_eq!(loaded.name(), "年终");
    assert!(loaded.hasName());
}

/// `custDataLst` 经 `cSlide.forceWriteCustDataLst` 写入，`protocol` 往返后标签还在。
#[test]
fn g04_kmo_film_cust_data_roundtrips() {
    use ppt_core::cn::wps::show::app::slide::KmoFilm::KmoFilm;
    use ppt_core::cn::wps::show::data::custdatalst::CustDataLst::CustDataLst;
    use ppt_core::cn::wps::show::data::custdatalst::Tag::Tag;
    use ppt_core::cn::wps::show::pptxr::handler::shape::SwGroupShapeHandler::ImportHost;

    let host = ImportHost::leak();
    let mut film = KmoFilm::KmoFilm(host);
    assert!(!film.hasCusDataLst());
    assert!(!film.hasBackgroud());
    let _ = film.background();
    film.clearFilmBackground();
    assert!(!film.hasBackgroud());

    let mut tag = Tag::create();
    tag.putName("KSO");
    tag.putVal("年终");
    let _ = tag.save();
    let mut lst = CustDataLst::create();
    lst.tags().write(std::slice::from_ref(&tag));
    let _ = lst.save();
    film.setCustDataLst(lst);
    assert!(film.hasCusDataLst());
    let mut tags = Vec::new();
    film.custDataLst().tags().read(&mut tags);
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].name(), "KSO");
    assert_eq!(tags[0].val(), "年终");

    let bin = film.protocol();
    let mut loaded = KmoFilm::KmoFilm(host);
    loaded.updateProtocol(bin);
    assert!(loaded.hasCusDataLst());
    tags.clear();
    loaded.custDataLst().tags().read(&mut tags);
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].name(), "KSO");
    assert_eq!(tags[0].val(), "年终");
}

#[test]
fn g05_kmo_film_kso_wm_info_reads_tags() {
    use ppt_core::cn::wps::show::app::KsoWMInfo::KsoWMInfo;
    use ppt_core::cn::wps::show::app::slide::KmoFilm::KmoFilm;
    use ppt_core::cn::wps::show::pptxr::handler::shape::SwGroupShapeHandler::ImportHost;

    let host = ImportHost::leak();
    let mut film = KmoFilm::KmoFilm(host);
    let mut info = KsoWMInfo::new();
    info.put("KSO".to_string(), "年终".to_string());
    film.addTags(info);
    assert!(film.hasCusDataLst());
    assert_eq!(film.ksoWMInfo().get("KSO"), Some("年终"));

    let bin = film.protocol();
    let mut loaded = KmoFilm::KmoFilm(host);
    loaded.updateProtocol(bin);
    assert_eq!(loaded.ksoWMInfo().get("KSO"), Some("年终"));
}
