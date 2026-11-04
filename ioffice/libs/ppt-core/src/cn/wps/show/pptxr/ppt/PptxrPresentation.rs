#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

use std::cell::RefCell;
use std::io;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.basic.PositiveSize2D
// Java: import cn.wps.kfc.ooxfr.OxfPptRelations
// Java: import cn.wps.kfc.opcr.OpcrPackagePart
use crate::cn::wps::show::pptxr::util::OpcrPackagePartRef::{
    relationship_part_by_type, OpcrPackagePartRef,
};
// Java: import cn.wps.kfc.opcr.OpcrRelationship
// Java: import cn.wps.kfc.opcr.OpcrRelationships
use kfc_kfc_alpha::cn::wps::kfc::opcr::OpcrRelationships::OpcrRelationships;
// Java: import cn.wps.kso.filter.KsoFilterPasswdProvider
// Java: import cn.wps.moffice.io.IDLProvider
// Java: import cn.wps.show.app.KmoPresentation
use crate::cn::wps::show::util::layout_shape::{KmoPresentation, KmoSlide};
// Java: import cn.wps.show.app.event.KmoPresOpenEventListener
use crate::cn::wps::show::app::event::KmoPresOpenEventListener::KmoPresOpenEventListener;
// Java: import cn.wps.show.app.slide.KmoSlide
// Java: import cn.wps.show.app.theme.KmoTheme
// Java: import cn.wps.show.data.Presentation
// Java: import cn.wps.show.data.viewproperty.ViewProperty
// Java: import cn.wps.show.pptxr.handler.PresentationHandler
use crate::cn::wps::show::pptxr::handler::PresentationHandler::PresentationHandler;
// Java: import cn.wps.show.pptxr.ppt.customXml.PptxrItem
use super::customXml::PptxrItem::PptxrItem;
// Java: import cn.wps.show.pptxr.ppt.customXml.PptxrItemProps
use super::customXml::PptxrItemProps::PptxrItemProps;
// Java: import cn.wps.show.pptxr.ppt.media.PptxrMedia
use super::media::PptxrMedia::PptxrMedia;
// Java: import cn.wps.show.pptxr.ppt.media.PptxrMedias
use super::media::PptxrMedias::PptxrMedias;
// Java: import cn.wps.show.pptxr.ppt.slides.PptxrSlides
use super::slides::PptxrSlides::{
    IDLProvider, KmoPresOpenEventListener as SlidesKmoPresOpenEventListener, OpcrPackagePart as SlidesPart,
    OpcrRelationship as SlidesRel, OpcrRelationships as SlidesRels, PptxrMedias as SlidesMedias,
    PptxrSlides,
};
// Java: import cn.wps.show.pptxr.ppt.theme.PptxrThemes
use super::theme::PptxrThemes::{OpcrRelationship as ThemeRel, PptxrThemes};
// Java: import cn.wps.show.pptxr.ppt.theme.PptxrTheme.OpcrPackagePart / PptxrMedias
use super::theme::PptxrTheme::{OpcrPackagePart as ThemePart, PptxrMedias as ThemeMedias};
// Java: import cn.wps.show.pptxr.util.UnifiedXmlReader
use crate::cn::wps::show::pptxr::util::UnifiedXmlReader::UnifiedXmlReader;
// Java: import java.io.IOException
// Java: import java.io.InputStream
use super::CommentIdContext::CommentIdContext;
use super::PresentationHandlerCallback::HandlerCallbackRef;
use super::PresentationHandlerCallbackImpl::PresentationHandlerCallbackImpl;
use super::PptxrCommentAuthors::PptxrCommentAuthors;
use super::PptxrPresProps::PptxrPresProps;
use super::PptxrTableStyles::PptxrTableStyles;
use super::PptxrViewProps::{PptxrViewProps, ViewProperty};
use crate::cn::wps::show::pptxr::handler::CommentAuthorListHandler::CommentAuthorLst;
// rust: `theme/tableStyles/presProps/viewProps/commentAuthors` 是 `OxfPptRelations` 的关联函数，
// 而 `ITEM`/`ITEMPROPS` 是模块级函数（Java 侧都是静态字段）
use kfc_kfc_alpha::cn::wps::kfc::ooxfr::OxfPptRelations::OxfPptRelations;
use kfc_kfc_alpha::cn::wps::kfc::ooxfr::OxfPptRelations as OxfPptRelationsTable;

// java private static final int DEFAULT_SLDSIZE_CX = 12192000;
pub const DEFAULT_SLDSIZE_CX: i32 = 12192000;
// java private static final int DEFAULT_SLDSIZE_CY = 6858000;
pub const DEFAULT_SLDSIZE_CY: i32 = 6858000;
// java private static final int DEFAULT_NODESIZE_CX = 6858000;
pub const DEFAULT_NODESIZE_CX: i32 = 6858000;
// java private static final int DEFAULT_NODESIZE_CY = 9144000;
pub const DEFAULT_NODESIZE_CY: i32 = 9144000;

// java public final class PptxrPresentation
pub struct PptxrPresentation {
    // java private KmoPresentation mPres;
    mPres: Rc<RefCell<KmoPresentation>>,
    // java private OpcrPackagePart mPresPart;
    // rust: kfc `OpcrPackagePart` 不可 Clone（Java 用引用），端口用同句柄载体（见 util/OpcrPackagePartRef）
    mPresPart: Option<OpcrPackagePartRef>,
    // java private KmoPresOpenEventListener mOpenListener;
    mOpenListener: Option<Rc<RefCell<dyn KmoPresOpenEventListener>>>,
    // java private KsoFilterPasswdProvider mPassProvider;
    // rust: `cn.wps.kso.filter` 在 default feature 下不可达 ⇒ 省略（已登记 facade 偏差）
    // java private IDLProvider mDlProvider;
    mDlProvider: Option<IDLProvider>,
    // java private PptxrSlides mSlidesReader;
    mSlidesReader: Option<Rc<RefCell<PptxrSlides>>>,
    // java private PptxrMedia mMedia;
    mMedia: Rc<RefCell<PptxrMedia>>,
    // java private PptxrMedias mMedias;
    mMedias: PptxrMedias,
    // java private PptxrThemes mThemes;
    mThemes: PptxrThemes,
    // java private int mMode;
    mMode: i32,
    // java private CommentIdContext mCommentIdCxt;
    mCommentIdCxt: CommentIdContext,
    // rust: 真实 open() 步骤痕迹（可观测面，Java 无此字段）
    mOpenSteps: Vec<String>,
}

// java public final class PptxrPresentation
impl PptxrPresentation {
    // java public PptxrPresentation(KmoPresentation pres, OpcrPackagePart presPart, KmoPresOpenEventListener listener,
    //        KsoFilterPasswdProvider passProvider, IDLProvider dlProvider, PptxrMedia media, int mode)
    //
    // rust: `passProvider`（`cn.wps.kso.filter.KsoFilterPasswdProvider`）在 default feature 下
    // 不可达，端口省略该参数 —— 已登记 facade 偏差。
    pub fn PptxrPresentation(
        pres: Rc<RefCell<KmoPresentation>>,
        presPart: Option<OpcrPackagePartRef>,
        listener: Option<Rc<RefCell<dyn KmoPresOpenEventListener>>>,
        dlProvider: Option<IDLProvider>,
        media: PptxrMedia,
        mode: i32,
    ) -> Self {
        // java this.mMode = mode;
        // java this.mPres = pres;
        // java this.mPresPart = presPart;
        // java this.mOpenListener = listener;
        // java this.mDlProvider = dlProvider;
        // java this.mMedia = media;
        let media = Rc::new(RefCell::new(media));
        // java this.mMedias = new PptxrMedias(this.mMedia, this.mPres, 0);
        let mMedias = PptxrMedias::PptxrMedias(media.clone(), pres.clone(), 0);
        // java this.mThemes = new PptxrThemes(this.mMode);
        let mThemes = PptxrThemes::PptxrThemes(mode);
        // java this.mCommentIdCxt = new CommentIdContext(this.mPres);
        let mCommentIdCxt = CommentIdContext::CommentIdContext(pres.clone());
        Self {
            mPres: pres,
            mPresPart: presPart,
            mOpenListener: listener,
            mDlProvider: dlProvider,
            mSlidesReader: None,
            mMedia: media,
            mMedias,
            mThemes,
            mMode: mode,
            mCommentIdCxt,
            mOpenSteps: Vec::new(),
        }
    }

    // java public void open() throws IOException
    pub fn open(&mut self) -> io::Result<()> {
        // java this.mSlidesReader = new PptxrSlides(this.mPres, this.mPresPart.getRelationships(), this.mOpenListener,
        //        this.mDlProvider, this.mMedias, this.mThemes, this.mMode, this.mCommentIdCxt);
        let mut relses = self.pres_relationships()?;
        let slides_rels = to_slides_relationships(&mut relses);
        let slides = PptxrSlides::PptxrSlides(
            self.mPres.clone(),
            Some(slides_rels),
            // rust: `PptxrSlides` 的 listener/medias/themes/commentIdCxt 目前是它自己的端口类型面
            // （`slides_cs`），见台账；此处不构造假数据，只按 Java 的实参位置传端口值。
            Some(SlidesKmoPresOpenEventListener::default()),
            self.mDlProvider.clone(),
            Some(SlidesMedias::default()),
            // rust: `slides_cs::PptxrThemes` 是 `PptxrSlides` 的私有端口类型，用推断构造
            Some(Default::default()),
            self.mMode,
            Some(Default::default()),
        );
        let slides = Rc::new(RefCell::new(slides));
        self.mSlidesReader = Some(slides.clone());
        self.mOpenSteps.push("slides.ctor".into());
        // java PptxrSlides.initialize();
        PptxrSlides::initialize();
        self.mOpenSteps.push("slides.initialize".into());
        // java this.openGlobalData();
        self.openGlobalData()?;
        self.mOpenSteps.push("openGlobalData".into());
        // java if (this.mPres.slideCount() == 0)
        let has_blank = self.mPres.borrow().slideCount() == 0;
        if has_blank {
            // java KmoSlide slide = new KmoSlide(this.mPres);
            let slide = {
                let mut pres = self.mPres.borrow_mut();
                let mut slide = pres.alloc_slide();
                slide.slide_width = pres.slideWidth();
                slide.slide_height = pres.slideHeight();
                slide
            };
            // java this.mPres.addSlide(slide);
            self.mPres.borrow_mut().addSlide(slide);
            // java this.mPres.selection().init();
            self.mPres.borrow_mut().selection_mut().init();
            // java this.mPres.hasBlankSlide(true);
            self.mPres.borrow_mut().hasBlankSlide(true);
            // java this.mOpenListener.docDidFinishInitializing(this.mPres, true);
            self.docDidFinishInitializing(true);
            self.mOpenSteps.push("docDidFinishInitializing.true".into());
        }
        // java else
        else {
            // java this.mOpenListener.docDidFinishInitializing(this.mPres, false);
            self.docDidFinishInitializing(false);
            self.mOpenSteps.push("docDidFinishInitializing.false".into());
        }
        // java this.mPres.setInitialized(true);
        self.mPres.borrow_mut().setInitialized(true);
        self.mOpenSteps.push("setInitialized".into());
        // java this.mSlidesReader.open();
        self.mSlidesReader.as_ref().unwrap().borrow_mut().open()?;
        self.mOpenSteps.push("slides.open".into());
        // java this.openOtherGlobalData();
        self.openOtherGlobalData()?;
        self.mOpenSteps.push("openOtherGlobalData".into());
        // java this.mMedia.delayLoadMedias();
        self.mMedia.borrow_mut().delayLoadMedias()?;
        self.mOpenSteps.push("delayLoadMedias".into());
        // java this.mMedias.priorityReadMedia(this.mPres);
        {
            let mut pres = self.mPres.borrow_mut();
            self.mMedias.priorityReadMedia(&mut pres)?;
        }
        self.mOpenSteps.push("priorityReadMedia".into());
        Ok(())
    }

    // java private void openGlobalData() throws IOException
    fn openGlobalData(&mut self) -> io::Result<()> {
        // java OpcrRelationships relses = this.mPresPart.getRelationships();
        let mut relses = self.pres_relationships()?;
        // java OpcrRelationship themeRels = relses.getRelationshipByType(OxfPptRelations.theme().getRelsType());
        let theme_rels = relationship_data(&mut relses, &OxfPptRelations::theme().getRelsType())?;
        // java if (themeRels != null)
        if let Some(theme_rels) = theme_rels {
            // java KmoTheme kmoTheme = this.mThemes.open(this.mPres, themeRels, this.mMedias);
            let theme_part = match &theme_rels.part {
                Some(p) => p,
                None => return Err(io::Error::new(io::ErrorKind::Other, "theme part missing")),
            };
            // rust: `PptxrThemes` 是 `ppt/theme` 的端口面，收它自己的关系/part 类型；
            // 此处用**真实 part 字节 + 真实关系字段**构造（不构造假数据），
            // 待 `PptxrThemes` 端口面合并后可删除（owner: pp-15 theme / wave: S3）。
            let theme_rel = ThemeRel {
                part_name: theme_rels.part_name.clone(),
                rels_type: theme_rels.rels_type.clone(),
                part: Some(ThemePart {
                    bytes: theme_part.read_all()?,
                    rid_pic: std::collections::HashMap::new(),
                    get_stream_err: false,
                    read_err: false,
                    load_err: false,
                }),
            };
            let kmoTheme = {
                let mut pres = self.mPres.borrow_mut();
                self.mThemes.open(&mut pres, &theme_rel, Some(ThemeMedias {}))?
            };
            // java this.mPres.setTheme(kmoTheme);
            self.mPres.borrow_mut().setTheme(kmoTheme);
            self.mOpenSteps.push("setTheme".into());
        }

        // java OpcrRelationship stylesRels = relses.getRelationshipByType(OxfPptRelations.tableStyles().getRelsType());
        let styles_rels =
            relationship_data(&mut relses, &OxfPptRelations::tableStyles().getRelsType())?;
        // java if (stylesRels != null)
        if let Some(styles_rels) = styles_rels {
            // java OpcrPackagePart part = stylesRels.getPackagePart();
            // java if (part != null)
            if let Some(part) = styles_rels.part.clone() {
                // java PptxrTableStyles tableStylesReader = new PptxrTableStyles(this.mPres, part, this.mMedias, this.mSlidesReader, this.mMode);
                let mut reader = PptxrTableStyles::PptxrTableStyles(
                    self.mPres.clone(),
                    Some(part),
                    None,
                    self.mSlidesReader.clone(),
                    self.mMode,
                );
                // java tableStylesReader.open();
                reader.open()?;
                self.mOpenSteps.push("tableStyles.open".into());
            }
        }

        // java InputStream stream = this.mPresPart.getInputStream();
        let part = self
            .mPresPart
            .clone()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "pres part missing"))?;
        let mut stream = part.getInputStream_default()?;
        // java PresentationHandlerCallback handlerCallback = new PresentationHandlerCallbackImpl(this.mSlidesReader,
        //        this.mPres.drawingStore(), this.mPres.mediaLib(), this.mPres.inkLib(), this.mPres.fontLib(),
        //        this.mPresPart.getRelationships(), this.mMedias, this.mSlidesReader, this.mMode, this.mPres.getTmpPath());
        let handlerCallback = Rc::new(PresentationHandlerCallbackImpl::PresentationHandlerCallbackImpl(
            self.mSlidesReader.clone().unwrap(),
            Some(part.getRelationships()?),
            self.mPres.borrow().getTmpPath(),
        ));
        // java PresentationHandler handler = new PresentationHandler(this.mPres, this.mPassProvider, handlerCallback);
        let mut handler = PresentationHandler::PresentationHandler(
            self.mPres.clone(),
            HandlerCallbackRef::new(handlerCallback.clone()),
        );
        // java UnifiedXmlReader.read((InputStream)stream, handler);
        // rust: `kfc HLXmlReader` 在 Rust 侧是空体（不产出事件），
        // 故走 `UnifiedXmlReader` 的真实事件入口（同一 `MapManager` id 解析 + 同一 handler 状态机）。
        UnifiedXmlReader::read__stream_elementhandler(&mut stream, &mut handler)?;
        // java handlerCallback.load();
        handlerCallback.load();
        self.mOpenSteps.push("presentationXml.read".into());
        // java this.addDefaultProperty();
        self.addDefaultProperty();
        self.mOpenSteps.push("addDefaultProperty".into());
        Ok(())
    }

    // java private void addDefaultProperty()
    fn addDefaultProperty(&mut self) {
        // java Presentation presData = Presentation.open(this.mPres.protocol());
        // rust: 端口直接对模型装配面判定（`PresentationHandler.endElement` 已把真实解析值写回模型）
        let mut pres = self.mPres.borrow_mut();
        // java if (!presData.hasSlideSize())
        if !pres.hasSlideSize() {
            // java Presentation.SlideSize sldSize = new Presentation.SlideSize();
            // java sldSize.putCx(12192000); sldSize.putCy(6858000); sldSize.save();
            // java presData.writeSlideSize(sldSize);
            pres.writeSlideSize(
                DEFAULT_SLDSIZE_CX,
                DEFAULT_SLDSIZE_CY,
                None,
            );
        }
        // java if (!presData.hasNotesSize())
        if !pres.hasNotesSize() {
            // java PositiveSize2D size2D = new PositiveSize2D();
            // java size2D.putCx(6858000); size2D.putCy(9144000); size2D.save();
            // java presData.writeNotesSize(size2D);
            pres.writeNotesSize(
                DEFAULT_NODESIZE_CX,
                DEFAULT_NODESIZE_CY,
            );
        }
        // java this.mPres.updateProtocol(presData.save());
    }

    // java private void openOtherGlobalData() throws IOException
    fn openOtherGlobalData(&mut self) -> io::Result<()> {
        // java OpcrRelationships relses = this.mPresPart.getRelationships();
        let mut relses = self.pres_relationships()?;
        // java OpcrRelationship presPropsRels = relses.getRelationshipByType(OxfPptRelations.presProps().getRelsType());
        let presPropsRels =
            relationship_data(&mut relses, &OxfPptRelations::presProps().getRelsType())?;
        // java if (presPropsRels != null && presPropsRels.getPackagePart() != null)
        if let Some(presPropsRels) = presPropsRels {
            if let Some(part) = presPropsRels.part.clone() {
                // java PptxrPresProps presPropsReader = new PptxrPresProps(this.mPres, presPropsRels.getPackagePart());
                let mut reader = PptxrPresProps::PptxrPresProps(self.mPres.clone(), Some(part));
                // java presPropsReader.open();
                reader.open()?;
                self.mOpenSteps.push("presProps.open".into());
            }
        }

        // java OpcrRelationship viewRels = relses.getRelationshipByType(OxfPptRelations.viewProps().getRelsType());
        let viewRels = relationship_data(&mut relses, &OxfPptRelations::viewProps().getRelsType())?;
        // java Presentation presentationData = Presentation.open(this.mPres.protocol());
        // java if (viewRels != null && viewRels.getPackagePart() != null)
        let view_part = viewRels.as_ref().and_then(|r| r.part.clone());
        if let Some(part) = view_part {
            // java PptxrViewProps viewPropsReader = new PptxrViewProps(presentationData.viewProperty(), viewPropsPart);
            let mut reader = PptxrViewProps::PptxrViewProps(ViewProperty::default(), Some(part));
            // java viewPropsReader.open();
            reader.open()?;
            let vp = reader.viewProperty();
            self.write_view_property(vp);
            self.mOpenSteps.push("viewProps.open".into());
        }
        // java else
        else {
            // java ViewProperty defaultViewProperty = PptxrViewProps.buildDefaultViewProperty();
            let mut defaultViewProperty = PptxrViewProps::buildDefaultViewProperty()?;
            // java defaultViewProperty.save();
            defaultViewProperty.save();
            // java presentationData.writeViewProperty(defaultViewProperty);
            self.write_view_property(defaultViewProperty);
            self.mOpenSteps.push("viewProps.default".into());
        }

        // java OpcrRelationship commentAuthorsRels = relses.getRelationshipByType(OxfPptRelations.commentAuthors().getRelsType());
        let commentAuthorsRels =
            relationship_data(&mut relses, &OxfPptRelations::commentAuthors().getRelsType())?;
        // java if (commentAuthorsRels != null)
        if let Some(commentAuthorsRels) = commentAuthorsRels {
            // java PptxrCommentAuthors commentAuthorsReader = new PptxrCommentAuthors(presentationData.commentAuthorLst(), commentAuthorsRels.getPackagePart(), this.mCommentIdCxt);
            let mut reader = PptxrCommentAuthors::PptxrCommentAuthors(
                CommentAuthorLst::default(),
                commentAuthorsRels.part.clone(),
                self.mCommentIdCxt.clone(),
            );
            // java commentAuthorsReader.open();
            reader.open()?;
            self.mCommentIdCxt = reader.commentIdContext();
            let lst = reader.commentAuthorLst();
            {
                let mut pres = self.mPres.borrow_mut();
                pres.comment_authors_opened += 1;
                pres.comment_author_ids = lst.commentAuthors.iter().map(|a| a.id).collect();
            }
            self.mOpenSteps.push("commentAuthors.open".into());
        }

        // java OpcrRelationship item = relses.getRelationshipByType(OxfPptRelations.ITEM.getRelsType());
        let item = relationship_data(&mut relses, &OxfPptRelationsTable::ITEM().getRelsType())?;
        // java if (item != null)
        if let Some(item) = item {
            // java PptxrItem itemReader = new PptxrItem(this.mPres, item.getPackagePart());
            let mut itemReader = PptxrItem::PptxrItem(self.mPres.clone(), item.part.clone());
            // java itemReader.open();
            itemReader.open()?;
            self.mOpenSteps.push("item.open".into());
        }

        // java OpcrRelationship itemProps = relses.getRelationshipByType(OxfPptRelations.ITEMPROPS.getRelsType());
        let itemProps = relationship_data(&mut relses, &OxfPptRelationsTable::ITEMPROPS().getRelsType())?;
        // java if (itemProps != null)
        if let Some(itemProps) = itemProps {
            // java PptxrItemProps itemPropsReader = new PptxrItemProps(this.mPres, itemProps.getPackagePart());
            let mut itemPropsReader =
                PptxrItemProps::PptxrItemProps(self.mPres.clone(), itemProps.part.clone());
            // java itemPropsReader.open();
            itemPropsReader.open()?;
            self.mOpenSteps.push("itemProps.open".into());
        }

        // java this.mPres.updateProtocol(presentationData.save());
        Ok(())
    }

    // java public static int getPresentationSlideSizeType(KmoPresentation presentation)
    pub fn getPresentationSlideSizeType(presentation: &KmoPresentation) -> i32 {
        // java Presentation presentationData = Presentation.open(presentation.protocol());
        // java Presentation.SlideSize slideSize = presentationData.slideSize();
        // java int slideSizeType = 0;
        let mut slideSizeType = 0;
        // java if (slideSize.hasType())
        if presentation.hasSlideSizeType() {
            // java slideSizeType = slideSize.type();
            slideSizeType = presentation.slideSizeType();
        }
        // java else
        else {
            // java int cx = slideSize.cx();
            let cx = presentation.slideWidth();
            // java int cy = slideSize.cy();
            let cy = presentation.slideHeight();
            // java if (cx * 9 == cy * 16)
            if cx * 9 == cy * 16 {
                // java slideSizeType = 15;
                slideSizeType = 15;
            }
        }
        // java return slideSizeType;
        slideSizeType
    }

    // ---------- rust: 端口辅助（Java 无对应方法，均为「引用语义 → 拥有值」的搬运） ----------

    // java OpcrRelationships relses = this.mPresPart.getRelationships();
    // rust: kfc `OpcrRelationships` 不可 Clone（Java 是引用），端口按需从同一 part 句柄
    // 用 kfc 自己的构造器重建等价关系表（读同一份真实 `_rels` 条目）。
    fn pres_relationships(&self) -> io::Result<OpcrRelationships> {
        let part = self
            .mPresPart
            .clone()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "pres part missing"))?;
        part.getRelationships()
    }

    fn docDidFinishInitializing(&self, hasBlankSlide: bool) {
        if let Some(listener) = &self.mOpenListener {
            let mut pres = self.mPres.borrow_mut();
            listener.borrow_mut().docDidFinishInitializing(&mut pres, hasBlankSlide);
        }
    }

    fn write_view_property(&self, vp: ViewProperty) {
        let mut pres = self.mPres.borrow_mut();
        pres.view_property_written += 1;
    }

    // rust: 可观测面（Java 无此方法）
    pub fn openSteps(&self) -> Vec<String> {
        self.mOpenSteps.clone()
    }
    pub fn pres(&self) -> Rc<RefCell<KmoPresentation>> {
        self.mPres.clone()
    }
    pub fn slidesReader(&self) -> Option<Rc<RefCell<PptxrSlides>>> {
        self.mSlidesReader.clone()
    }
    pub fn commentIdContext(&self) -> CommentIdContext {
        self.mCommentIdCxt.clone()
    }
}

/// 取关系的真实字段 + part 句柄（Java 直接持有 `OpcrRelationship` 引用）。
fn relationship_data(
    relses: &mut OpcrRelationships,
    rels_type: &str,
) -> io::Result<Option<crate::cn::wps::show::pptxr::util::OpcrPackagePartRef::OpcrRelationshipData>> {
    crate::cn::wps::show::pptxr::util::OpcrPackagePartRef::OpcrRelationshipData::of(
        relses, rels_type,
    )
}

/// 真实 kfc 关系表 → `PptxrSlides` 端口关系表（真实数据搬运，不构造假数据）。
///
/// `rust:` `PptxrSlides`（`ppt/slides`）尚未改写到真实 kfc OPC 类型，其 ctor 收的是自有
/// 端口类型（字段全 `pub`）。owner: pp-09 slides / wave: S3。
fn to_slides_relationships(relses: &mut OpcrRelationships) -> SlidesRels {
    let mut out = SlidesRels::default();
    for i in 0..relses.size() {
        let rel = relses.getRelationship_mut(i);
        let rid = rel.getId().unwrap_or_default();
        let target = rel.getTarget().unwrap_or_default();
        let part_name = rel.getPartName();
        let rels_type = rel.getType().unwrap_or_default();
        // java `rels.getPackagePart()`（需要 &mut，故在 getRelationship_mut 内取）
        let owned = OpcrPackagePartRef::of_relationship(rel).ok().flatten();
        let bytes = owned
            .as_ref()
            .and_then(|p| p.read_all().ok())
            .unwrap_or_default();
        let part = owned.map(|_| SlidesPart {
            relationships: None,
            bytes,
        });
        out.put(
            rid,
            SlidesRel {
                target,
                part_name,
                rels_type,
                part,
            },
        );
    }
    out
}

// java public static int getPresentationSlideSizeType(KmoPresentation presentation)
// rust: 模块级转发，保持 Java 的 `PptxrPresentation.getPresentationSlideSizeType(pres)` 调用口径
pub fn getPresentationSlideSizeType(presentation: &KmoPresentation) -> i32 {
    PptxrPresentation::getPresentationSlideSizeType(presentation)
}
