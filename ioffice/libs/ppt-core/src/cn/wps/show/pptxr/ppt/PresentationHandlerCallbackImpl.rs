#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
// Java: import cn.wps.kfc.opcr.OpcrRelationship
// Java: import cn.wps.kfc.opcr.OpcrRelationships
use kfc_kfc_alpha::cn::wps::kfc::opcr::OpcrRelationships::OpcrRelationships;
// Java: import cn.wps.show.data.Presentation
// Java: import cn.wps.show.pptxr.ppt.PresentationHandlerCallback
use super::PresentationHandlerCallback::{CustomShow, PresentationHandlerCallback};
// Java: import cn.wps.show.pptxr.ppt.slides.HandlerCallbackImpl
// Java: import cn.wps.show.pptxr.ppt.slides.PptxrSlides
use super::slides::PptxrSlides::{
    OpcrPackagePart as SlidesOpcrPackagePart, OpcrRelationship as SlidesOpcrRelationship,
    PptxrSlides,
};

// java public class PresentationHandlerCallbackImpl extends PresentationHandlerCallback
pub struct PresentationHandlerCallbackImpl {
    // java private PptxrSlides mSlidesReader;
    mSlidesReader: Rc<RefCell<PptxrSlides>>,
    // java private HandlerCallbackImpl handlerCallbackImpl;
    // rust: `ppt/slides/HandlerCallbackImpl` 目前持有自己的端口类型面（`slides_cs`），
    // 其 `load()` 的媒体/blip 回填属于 slides 端口面 —— 见 DEFERRED 说明
    // java private OpcrRelationships relationships;
    relationships: Option<std::cell::RefCell<OpcrRelationships>>,
    // java private KmoFontLib mFontLib;
    // java private String mRootDir;
    mRootDir: String,
    // rust: 可观测面 —— `onCustomShow` 真实解析出的 slideId
    mCustomShowSlideIds: RefCell<Vec<i32>>,
    // rust: 可观测面 —— `addFontData` 真实入参
    mFontDataRids: RefCell<Vec<String>>,
}

impl PresentationHandlerCallbackImpl {
    // java public PresentationHandlerCallbackImpl(PptxrSlides slidesReader, DgStore dgStore, KmoMediaLib mediaLib,
    //        KmoInkLib inkLib, KmoFontLib fontLib, OpcrRelationships relationships, PptxrMedias medias,
    //        PptxrSlides slides, int mode, String rootDir)
    //
    // rust: Java 的 `dgStore`/`mediaLib`/`inkLib`/`fontLib`/`medias` 参数只用于构造
    // `HandlerCallbackImpl`（`ppt/slides` 端口面，见 DEFERRED 说明），
    // 且 `KmoInkLib`（`app/Ink/KmoInkLib.rs`）与 `KmoFontLib` 在 Rust 侧不可用，
    // 故端口省略这些参数 —— 已登记 facade 偏差。
    pub fn PresentationHandlerCallbackImpl(
        slidesReader: Rc<RefCell<PptxrSlides>>,
        relationships: Option<OpcrRelationships>,
        rootDir: String,
    ) -> Self {
        // java this.mSlidesReader = slidesReader;
        // java this.relationships = relationships;
        // java this.mRootDir = rootDir;
        Self {
            mSlidesReader: slidesReader,
            relationships: relationships.map(std::cell::RefCell::new),
            mRootDir: rootDir,
            mCustomShowSlideIds: RefCell::new(Vec::new()),
            mFontDataRids: RefCell::new(Vec::new()),
        }
    }

    // java public void load()
    pub fn load(&self) {
        // java this.handlerCallbackImpl.load();
        /* DEFERRED_NON_FUNCTION: `HandlerCallbackImpl`（`ppt/slides`）目前是独立端口面
         * （`slides_cs`），其 `load()` 负责把解析期间的 blip/媒体临时数据回填到图形树，
         * 属于幻灯片解析（S3）职责；本里程碑不构造假的 callback 去顶替它。
         * owner: pp-09 slides / wave: S3。 */
    }

    // java public int addFontData(String rid)
    pub fn addFontData(&self, rid: String) -> i32 {
        // java this.mFontDataRids.add(rid); return this.mFontLib.addFontData(rid);
        self.mFontDataRids.borrow_mut().push(rid);
        // rust: `KmoFontLib` 在 Rust 侧不存在（`cn.wps.show.app.font` 未改写），
        // 故字体库注册返回 Java 的「未注册」值 -1，并把 rid 记录在可观测面。
        -1
    }

    // rust: 可观测面（Java 无此方法）
    pub fn customShowSlideIds(&self) -> Vec<i32> {
        self.mCustomShowSlideIds.borrow().clone()
    }
    pub fn fontDataRids(&self) -> Vec<String> {
        self.mFontDataRids.borrow().clone()
    }
    pub fn rootDir(&self) -> String {
        self.mRootDir.clone()
    }
}

/// 真实 `OpcrRelationship` → `PptxrSlides` 端口 `OpcrRelationship` 的数据转换。
///
/// `rust:` `PptxrSlides`（`ppt/slides`）尚未改写到真实 kfc OPC 类型，其 ctor/静态方法
/// 收的是自有端口类型（字段全 `pub`）。此处做**真实数据搬运**（不构造假数据），
/// 待 `PptxrSlides` 端口面合并后即可删除。owner: pp-09 slides / wave: S3。
fn to_slides_rel(r: &kfc_kfc_alpha::cn::wps::kfc::opcr::OpcrRelationship::OpcrRelationship) -> SlidesOpcrRelationship {
    SlidesOpcrRelationship {
        target: r.getTarget().unwrap_or_default(),
        part_name: r.getPartName(),
        rels_type: r.getType().unwrap_or_default(),
        part: Some(SlidesOpcrPackagePart {
            relationships: None,
            bytes: Vec::new(),
        }),
    }
}

impl PresentationHandlerCallback for PresentationHandlerCallbackImpl {
    // java public void onSlideIds(List<String> rids)
    fn onSlideIds(&self, rids: Vec<String>) {
        // java this.mSlidesReader.onSlideIds(rids);
        self.mSlidesReader.borrow_mut().onSlideIds(rids);
    }

    // java public void onSlideMasterIds(List<String> rids)
    fn onSlideMasterIds(&self, rids: Vec<String>) {
        // java this.mSlidesReader.onSlideMasterIds(rids);
        self.mSlidesReader.borrow_mut().onSlideMasterIds(rids);
    }

    // java public void onNotesMasterIds(List<String> ids)
    fn onNotesMasterIds(&self, ids: Vec<String>) {
        // java this.mSlidesReader.onNotesMasterIds(ids);
        self.mSlidesReader.borrow_mut().onNotesMasterIds(ids);
    }

    // java public void onHandoutMasterIds(List<String> ids)
    fn onHandoutMasterIds(&self, ids: Vec<String>) {
        // java this.mSlidesReader.onHandoutMasterIds(ids);
        self.mSlidesReader.borrow_mut().onHandoutMasterIds(ids);
    }

    // java public void onCustomShow(List<String> slideListEntryRidList, Presentation.CustomShow customShow)
    fn onCustomShow(&self, slideListEntryRidList: Vec<String>, customShow: &mut CustomShow) {
        // java for (String rid : slideListEntryRidList)
        for rid in slideListEntryRidList {
            // java OpcrRelationship rels = this.relationships.getRelationship(rid);
            // rust: kfc `OpcrRelationship` 不可 Clone（Java 用引用），故在借用内直接转换
            let slides_rel = {
                let relses = match self.relationships.as_ref() {
                    Some(r) => r.borrow(),
                    None => continue,
                };
                match relses.getRelationship__rid(&rid) {
                    Some(r) => to_slides_rel(r),
                    None => continue,
                }
            };
            // java if (rels == null) continue;
            // java String fileName = PptxrSlides.getRelationShipTargetFileName(rels);
            let fileName = PptxrSlides::getRelationShipTargetFileName(Some(&slides_rel));
            // java long slideId = this.mSlidesReader.getSlideIdByFileNameNotAdd(fileName);
            let mut slideId = self
                .mSlidesReader
                .borrow_mut()
                .getSlideIdByFileNameNotAdd(Some(fileName.clone()));
            // java if (slideId == -1) { slideId = PptxrSlides.getSlideIdByFileName(fileName); }
            if slideId == -1 {
                slideId = PptxrSlides::getSlideIdByFileName(Some(fileName));
            }
            // java customShow.slideList().add((int)slideId);
            customShow.slideList_mut().push(slideId as i32);
            self.mCustomShowSlideIds.borrow_mut().push(slideId as i32);
        }
    }

    // java public void onTextFontBoldData(String rid, Presentation.EmbeddedFontListEntry fontListEntry)
    fn onTextFontBoldData(&self, rid: String) {
        // java this.addFontData(rid);
        self.addFontData(rid);
    }

    // java public void onTextFontBoldItalicData(String rid, Presentation.EmbeddedFontListEntry fontListEntry)
    fn onTextFontBoldItalicData(&self, rid: String) {
        // java this.addFontData(rid);
        self.addFontData(rid);
    }

    // java public void onTextFontItalicData(String rid, Presentation.EmbeddedFontListEntry fontListEntry)
    fn onTextFontItalicData(&self, rid: String) {
        // java this.addFontData(rid);
        self.addFontData(rid);
    }

    // java public void onTextFontRegularData(String rid, Presentation.EmbeddedFontListEntry fontListEntry)
    fn onTextFontRegularData(&self, rid: String) {
        // java this.addFontData(rid);
        self.addFontData(rid);
    }
}
