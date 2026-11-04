#![allow(dead_code, non_snake_case, unused_variables)]
// Java: cn.wps.moffice.writer.layout.base.balloons.content.BalloonContentLayouter
use thirdparty_font::cn::wps::font::IFontFamily::IFontFamily;
use thirdparty_font::cn::wps::font::ITypeface::ITypeface;
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingResizeTool::DrawingResizeTool;
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
use crate::cn::wps::moffice::writer::cache::TypoTextBox::TypoTextBox;
use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::IPapxTable::{Entry as PapxEntry, IPapxTable};
use crate::cn::wps::moffice::writer::data::ITable::Entry as ITableEntry;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::FontStyleFactory::FontStyleFactory;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationLayouter::EquationLayouter;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphBuff::EmbedArg;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphFactory::GraphFactory;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::MathParaSubLineTool::MathParaSubLineTool;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineLayouterBase::SubLineLayouterBase;
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignLeftHandler::AlignLeftHandler;
use crate::cn::wps::moffice::writer::layout::base::line::align::IHAlignHandler::IHAlignHandler;
use crate::cn::wps::moffice::writer::layout::base::math::AlignOMathParaHandler::AlignOMathParaHandler;
use crate::cn::wps::moffice::writer::layout::base::math::MathLayouter::MathLayouter;

// java public class BalloonContentLayouter extends SubLineLayouterBase
pub struct BalloonContentLayouter {
    // java MainState mMainState;
    main: *mut MainState,
    // java DocumentEnv mDocEnv;
    doc: *mut DocumentEnv,
    // java ViewEnv mViewEnv;
    view: *mut ViewEnv,
    // java GraphBuff mGraphBuff;
    graph: GraphFactory,
    // java SubLineEnv mSubLineEnv;
    sub_line: SubLineEnv,
    // java IHAlignHandler mHAlignHandler;
    align: AlignLeftHandler,
    // java TypoTextLine mTextLine;
    text_line: TypoTextLine,
    excess: SubLineLayouterBase,
    // java int mCurLayoutHeight;
    cur_layout_height: i32,
    // java int mCoreWidth;
    core_width: i32,
    // java int mMaxAscentForBaseLine;
    max_ascent: i32,
    // java int mMaxDescentForBaseLine;
    max_descent: i32,
    // java int mBaseLine;
    base_line: i32,
    // java int mLineHeight;
    line_height: i32,
    // java EquationContentLayouter.vAlignRun replaces BalloonContentLayouter.vAlignRun
    equation_valign: bool,
    need_adjust_draw_pos_offset_x: bool,
    // java MathContentLayouter extends BalloonContentLayouter
    math_mode: bool,
    max_embed_height: i32,
    use_align_omath: bool,
    omath_active: bool,
    omath: AlignOMathParaHandler,
}

impl BalloonContentLayouter {
    // java public BalloonContentLayouter(MainState mainState, DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn BalloonContentLayouter(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self::new(main, doc, view, false)
    }

    pub fn for_equation(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self::new(main, doc, view, true)
    }

    fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv, equation_valign: bool) -> Self {
        // java this.mGraphBuff = GraphBuffFactory.createGraphBuff(mainState, docEnv, viewEnv);
        // The factory shares the caller's MainState. Default owned envs are not read.
        Self {
            main,
            doc,
            view,
            graph: GraphFactory::from_env_alias(main, doc, view),
            sub_line: SubLineEnv::default(),
            align: AlignLeftHandler::AlignLeftHandler(),
            text_line: TypoTextLine::newInstance(),
            excess: SubLineLayouterBase::SubLineLayouterBase(MainState::default(), DocumentEnv::default(), ViewEnv::default()),
            cur_layout_height: 0,
            core_width: 0,
            max_ascent: 0,
            max_descent: 0,
            base_line: 0,
            line_height: 0,
            equation_valign,
            need_adjust_draw_pos_offset_x: false,
            math_mode: false,
            max_embed_height: 0,
            use_align_omath: false,
            omath_active: false,
            omath: AlignOMathParaHandler::AlignOMathParaHandler(doc),
        }
    }

    // java MathContentLayouter constructor: mGraphBuff.setCalcGraphHeight(true)
    pub fn enable_math(&mut self) {
        self.math_mode = true;
        self.graph.setCalcGraphHeight(true);
    }

    // java public void setUseAlignOMathParaHandler(boolean value)
    pub fn setUseAlignOMathParaHandler(&mut self, value: bool) {
        self.use_align_omath = value;
    }

    // java public int getAlignType()
    pub fn getAlignType(&self) -> i32 {
        if self.max_ascent == 0 && self.max_descent == 0 { 0 } else { 1 }
    }

    // java TypoPage.hasLines(textBox, snapshot)
    pub fn has_lines(&self, text_box: i32) -> bool {
        if text_box == 0 || self.main.is_null() {
            return false;
        }
        let snapshot = unsafe { &(*self.main).snapshot };
        let lines = TypoTextBox::getLines__int_typosnapshot(text_box, snapshot);
        TypoList::size__int_typosnapshot(lines, snapshot) > 0
    }

    // java MathContentLayouter.flow sets these before super.flow
    pub fn begin_math_flow(&mut self, layout_state: &mut LayoutState) {
        self.graph.super_.setIsInMath(true, layout_state);
        self.graph.super_.setIgnoreHpsAndVAlignRun(true);
        self.graph.super_.setIgnoreCapsAndSmallCaps(true);
    }

    // java public void setIgnoreLeading(boolean ingoreLeading)
    pub fn setIgnoreLeading(&mut self, value: bool) {
        self.graph.setIgnoreLeading(value);
    }

    // java public void setMathContent(boolean mathContent)
    pub fn setMathContent(&mut self, value: bool) {
        self.graph.setMathContent(value);
    }

    // java GraphBuff.addFunctionSepField → GraphReader.addFunctionSepField
    pub fn addFunctionSepField(&mut self, sep_cp: i32) {
        self.graph.super_.addFunctionSepField(sep_cp);
    }

    fn rebind(&mut self) {
        // GraphFactory dispatch stores this address. Rebind after the value has moved.
        self.graph.bind_factory_dispatch();
    }

    // java public int getBaseLine()
    pub fn getBaseLine(&self) -> i32 {
        self.base_line
    }

    // java public void setCalcGraphHeight(boolean value)
    pub fn setCalcGraphHeight(&mut self, value: bool) {
        self.graph.setCalcGraphHeight(value);
    }

    // java public void initFilter(Document document, int start, int end, LayoutState layoutState)
    pub fn initFilter(&mut self, document: Option<*mut dyn Document>, start: i32, end: i32, layout_state: &mut LayoutState) {
        self.graph.super_.initFilter(document, start, end, layout_state);
    }

    // java public int flow(LayoutState layoutState)
    pub fn flow(&mut self, layout_state: &mut LayoutState) -> i32 {
        self.rebind();
        let pointer = {
            let snapshot = unsafe { &mut (*self.main).snapshot };
            let mut text_box = snapshot.getTypoTools().obtainTypoTextBox();
            let pointer = text_box.alloc(snapshot);
            text_box.clear();
            pointer
        };
        let mut page = TypoPage::default();
        page.snapshot_pointer = pointer;
        page.snapshot = unsafe { &mut (*self.main).snapshot as *mut TypoSnapshot };
        self.flow_page(layout_state, page)
    }

    // java public int flow(LayoutState layoutState, TypoPage page)
    fn flow_page(&mut self, layout_state: &mut LayoutState, page: TypoPage) -> i32 {
        // java layoutState.linePos = 0;
        layout_state.linePos = 0;
        // java TypoPage restorePage = layoutState.curPage;
        let restore = layout_state.curPage.take();
        layout_state.curPage = Some(page);
        // java page.setDocumentType(layoutState.document.getType());
        if let Some(document) = layout_state.document.filter(|p| !p.is_null()) {
            let doc_type = unsafe { (*document).getType__on_IDocument() };
            if let Some(page) = layout_state.curPage.as_mut() {
                page.setDocumentType(doc_type);
            }
        }
        // java page.setMargin(...); page.setWidth(pageEnv.width);
        let (top, left, bottom, right, width, core) = {
            let env = &layout_state.pageEnv;
            (env.marginTop, env.marginLeft, env.marginBottom, env.marginRight, env.width, env.getCoreWidth())
        };
        if let Some(page) = layout_state.curPage.as_mut() {
            page.setMargin(top, left, bottom, right);
            page.setWidth(width);
        }
        self.cur_layout_height = 0;
        self.core_width = core;
        self.init_para_env(&mut layout_state.paraEnv);
        self.init_para_prop(layout_state);
        FontStyleFactory::setSmartFontSize(false);
        FontStyleFactory::setIsInEmbedBalloonMode(layout_state.isInEmbedBalloonMode);
        FontStyleFactory::setEmebedFontSize(layout_state.mEmbedBallooFontSize);
        if self.graph.super_.prepareGraphs(2, layout_state) {
            self.prepare_embeds(2, layout_state);
        }
        while layout_state.hasContents() && !layout_state.getGraphList().isEmpty() {
            if !self.flow_text_line(layout_state) {
                break;
            }
            if self.graph.super_.prepareGraphs(2, layout_state) {
                self.prepare_embeds(2, layout_state);
            }
        }
        FontStyleFactory::setIsInEmbedBalloonMode(false);
        let snap = unsafe { &mut (*self.main).snapshot as *mut TypoSnapshot };
        if let Some(page) = layout_state.curPage.as_mut() {
            let lines = page.getLines();
            if lines != 0 && TypoList::size__int_typosnapshot(lines, unsafe { &*snap }) > 0 {
                let last = TypoList::last__int_typosnapshot(lines, unsafe { &mut *snap });
                let bottom = TypoObjBase::getBottom__int_typosnapshot(last, unsafe { &*snap });
                let height = bottom + layout_state.pageEnv.marginTop + layout_state.pageEnv.marginBottom;
                page.setHeight(height);
            }
        }
        let pointer = layout_state.curPage.as_ref().map(|page| page.getPointer()).unwrap_or(0);
        layout_state.curPage = restore;
        pointer
    }

    // java private void prepareEmbeds(int wordAlingFont, LayoutState layoutState)
    fn prepare_embeds(&mut self, word_align_font: i32, layout_state: &mut LayoutState) {
        let use_word97 = unsafe { (*self.doc).useWord97LineBreakRules };
        let mut cur = layout_state.getEmbedSize();
        loop {
            let args_size = self.graph.super_.super_.getArgsForEmbedSize();
            if cur >= args_size {
                break;
            }
            let arg = self.graph.super_.super_.getArgsForEmbed(cur).cloned();
            let graphs = self.graph.super_.super_.getGraphsForEmbed(cur).unwrap_or(std::ptr::null_mut());
            let Some(arg) = arg else { break; };
            let main = self.main;
            let doc = self.doc;
            let view = self.view;
            let drawing = match &arg {
                EmbedArg::Shape(shape_id) => self.graph.super_.super_.shape_embed_pointer(*shape_id, layout_state),
                EmbedArg::Math(node) => MathLayouter::getMath(
                    unsafe { &mut *main },
                    unsafe { &mut *doc },
                    unsafe { &mut *view },
                    layout_state,
                    node,
                ),
                EmbedArg::Eq(eq) => {
                    match EquationLayouter::getEquation__mainstate_documentenv_viewenv_document_int_int_boolean_boolean(
                        unsafe { &mut *main },
                        unsafe { &mut *doc },
                        unsafe { &mut *view },
                        layout_state.document,
                        eq.start,
                        eq.end,
                        eq.enableRevision,
                        eq.revisionFinal,
                    ) {
                        Ok(pointer) => pointer,
                        Err(_) => 0,
                    }
                }
            };
            if drawing != 0 {
                if let EmbedArg::Shape(shape_id) = &arg {
                    let kind = layout_state.document.filter(|doc| !doc.is_null()).map(|doc| self.graph.super_.super_.comment_shape_kind(*shape_id, doc)).unwrap_or(0);
                    let snapshot = unsafe { &mut (*main).snapshot };
                    if kind == 1 {
                        let layout_width = TypoDrawing::layoutWidth__int_typosnapshot(drawing, snapshot) as f32;
                        let layout_height = TypoDrawing::layoutHeight__int_typosnapshot(drawing, snapshot) as f32;
                        let min_size = layout_width.min(layout_height);
                        if min_size > 0.0 {
                            let pic = unsafe { (*view).getCommentPicDisplaySize() };
                            DrawingResizeTool::scaleDrawing(drawing, pic / min_size, true, true, snapshot);
                        }
                    } else if kind == 2 {
                        let layout_width = TypoDrawing::layoutWidth__int_typosnapshot(drawing, snapshot) as f32;
                        let core = layout_state.pageEnv.getCoreWidth() as f32;
                        if layout_width > core && layout_width > 0.0 {
                            DrawingResizeTool::scaleDrawing(drawing, core / layout_width, true, true, snapshot);
                        }
                    }
                }
                if !graphs.is_null() {
                    layout_state.setDrawing(unsafe { &mut *graphs }, drawing, word_align_font, use_word97);
                    let chars = unsafe { &*graphs }.charsRef.as_ref();
                    let revision_final = chars.and_then(|c| c.srcRef).map(|src| unsafe { (*src).revisionFinal }).unwrap_or(false);
                    let has_del = chars.and_then(|c| c.finalRunProp.as_ref()).map(|p| p.contains(45)).unwrap_or(false);
                    if revision_final && has_del {
                        let author = chars.map(|c| c.revisionDelAuthor).unwrap_or(-1);
                        let snapshot = unsafe { &mut (*main).snapshot };
                        TypoDrawing::setIsRevisionDelEmbed__boolean_int_typosnapshot(true, drawing, snapshot);
                        TypoDrawing::setRevisionDelId(author, drawing, snapshot);
                    }
                }
                cur += 1;
            } else {
                self.graph.super_.super_.removeEmbedArgsAt(cur);
                if !graphs.is_null() {
                    let next = layout_state.getGraphList().getNext(graphs);
                    if layout_state.getGraphList().firstElement() == graphs {
                        layout_state.push_ptr(next);
                    }
                    layout_state.getGraphList().remove(graphs, graphs);
                }
            }
        }
    }

    // java protected void initParaEnv(ParaEnv paraEnv)
    fn init_para_env(&self, para_env: &mut crate::cn::wps::moffice::writer::layout::base::env::ParaEnv::ParaEnv) {
        para_env.alignment = 0;
        para_env.wordAlignFont = 4;
        para_env.isRTL = false;
        para_env.compressLine = false;
        para_env.overflowPunct = false;
    }

    // java protected void initParaProp(LayoutState layoutState)
    fn init_para_prop(&mut self, layout_state: &mut LayoutState) {
        let node = layout_state.getFirstCoreSrc();
        let Some(doc_ptr) = node.doc.filter(|p| !p.is_null()) else { return; };
        if node.cp < 0 || node.cp >= unsafe { (*doc_ptr).getLength() } {
            return;
        }
        unsafe { &mut (*self.doc).pools };
        layout_state.paraEnv.init(unsafe { &mut (*self.doc).pools });
        let Some(table) = (unsafe { (*doc_ptr).getPapxTable() }).filter(|p| !p.is_null()) else { return; };
        let Some(entry) = (unsafe { IPapxTable::seek(&*table, node.cp) }) else { return; };
        let mut para_prop = unsafe { (*self.doc).pools.obtainParaPropEnv() };
        let prop = PapxEntry::getProperty(entry.as_ref()).cloned();
        para_prop.paraProperty = prop.clone();
        para_prop.firstNormalParaProperty = prop;
        para_prop.start = ITableEntry::position(entry.as_ref());
        let end_cp = ITableEntry::endCp(entry.as_ref());
        if let Some(document) = layout_state.document.filter(|p| !p.is_null()) {
            para_prop.setLayoutRange(layout_state.cp, end_cp, unsafe { &*document });
        }
        layout_state.paraEnv.curParaPropEnv = Some(para_prop);
    }

    // java protected void initSubLineEnv(LayoutState layoutState)
    fn init_sub_line(&mut self, layout_state: &mut LayoutState) {
        self.sub_line.clear();
        if self.omath_active {
            self.omath.setSubLineEnv(&mut self.sub_line, layout_state);
        }
        IHAlignHandler::setSubLineEnv(&mut self.align, &mut self.sub_line, layout_state);
        self.core_width = layout_state.pageEnv.getCoreWidth();
        self.sub_line.right = self.core_width;
        self.sub_line.layoutRight = self.core_width;
        self.max_ascent = 0;
        self.max_descent = 0;
        if self.math_mode {
            // java MathContentLayouter.initSubLineEnv after super
            self.sub_line.layoutLeft = self.sub_line.left;
            self.sub_line.layoutRight = self.sub_line.right;
            self.core_width = self.sub_line.layoutRight - self.sub_line.layoutLeft;
            self.max_embed_height = 0;
        }
    }

    // java protected void calcLineHeight()
    fn calc_line_height(&mut self) {
        if self.math_mode {
            // java MathContentLayouter.calcLineHeight
            self.base_line = self.max_ascent.max(self.max_embed_height);
            self.line_height = self.base_line + self.max_descent;
            return;
        }
        self.base_line = self.max_ascent;
        self.line_height = self.max_ascent + self.max_descent;
    }

    // java protected void collectLineHeight(Graphs run, LayoutState layoutState)
    fn collect_line_height(&mut self, run: &Graphs, layout_state: &mut LayoutState) {
        if self.math_mode {
            self.collect_math_line_height(run, layout_state);
            return;
        }
        let Some(style) = run.fontStyle.as_ref() else { return; };
        self.max_ascent = self.max_ascent.max((style.maxAscent - style.hpsPos).max(0));
        self.max_descent = self.max_descent.max((style.maxDescent + style.hpsPos).max(0));
    }

    // java MathContentLayouter.collectLineHeight
    fn collect_math_line_height(&mut self, run: &Graphs, layout_state: &mut LayoutState) {
        let drawing = if run.isEmbed() { layout_state.getDrawing(run) } else { 0 };
        if drawing != 0 {
            let snapshot = unsafe { &(*self.main).snapshot };
            if TypoDrawing::isMath__int_typosnapshot(drawing, snapshot) {
                let _align_type = TypoDrawing::getAlignType__int_typosnapshot(drawing, snapshot);
                let align_line = TypoDrawing::getAlignLine__int_typosnapshot(drawing, snapshot);
                let max_descent = TypoDrawing::layoutHeight__int_typosnapshot(drawing, snapshot) - align_line;
                self.max_ascent = self.max_ascent.max(align_line);
                self.max_descent = self.max_descent.max(max_descent);
            } else {
                let height = TypoDrawing::layoutHeight__int_typosnapshot(drawing, snapshot);
                self.max_embed_height = self.max_embed_height.max(height);
            }
            return;
        }
        let Some(style) = run.fontStyle.as_ref() else { return; };
        self.max_ascent = self.max_ascent.max(style.maxAscent);
        self.max_descent = self.max_descent.max(style.maxDescent);
    }

    // java EquationContentLayouter.setNeedAdjustDrawPosOffsetX
    pub fn setNeedAdjustDrawPosOffsetX(&mut self, value: bool) {
        self.need_adjust_draw_pos_offset_x = value;
    }

    // java EquationContentLayouter.addRun when mNeedAdjustDrawPosOffsetX
    fn adjust_draw_pos_offset_x(&mut self, run: *mut Graphs, layout_state: &mut LayoutState) {
        let (face, font_size, graph_index, char_width) = {
            let run = unsafe { &*run };
            let Some(style) = run.fontStyle.as_ref() else { return };
            let Some(face) = style.typeface.clone() else { return };
            (face, style.fontSize, run.graphIndex, run.width())
        };
        let ch = layout_state.typoPara.getChar(graph_index);
        let rect = ITypeface::getTextRect_char(face.as_ref(), font_size as f32, ch as u16);
        let offset = (-rect.left.min(char_width as f32 / 2.0)) as i32;
        let extend_width = (offset as f32 - (char_width as f32 - rect.right)) as i32;
        layout_state.typoPara.extendWidths(graph_index, 1, extend_width);
        unsafe {
            (*run).offsetX(offset);
            (*run).mWidth += extend_width;
        }
        layout_state.addDrawPosOffsetX__int_int(graph_index, offset);
    }

    // java protected boolean addRun(Graphs run, LayoutState layoutState)
    fn add_run(&mut self, run: *mut Graphs, layout_state: &mut LayoutState) -> bool {
        if self.need_adjust_draw_pos_offset_x {
            self.adjust_draw_pos_offset_x(run, layout_state);
        }
        if self.math_mode {
            self.collect_line_height(unsafe { &*run }, layout_state);
        }
        let run_pos = if self.sub_line.hasGraphs() {
            self.sub_line.lastGraphs.as_ref().map(|g| g.getRight()).unwrap_or(0)
        } else {
            0
        };
        unsafe { (*run).offsetXTo(run_pos); }
        self.sub_line.addGraphs(run);
        if TypoRun::isLineBreak(unsafe { (*run).runType }) {
            if !self.math_mode {
                self.collect_line_height(unsafe { &*run }, layout_state);
            }
            IHAlignHandler::onAddRun(&mut self.align, unsafe { &mut *run });
            return false;
        }
        if unsafe { (*run).getRight() } > self.core_width {
            if let Some(env) = self.sub_line.lineCompressEnv.as_mut() {
                env.reset();
            }
            if self.math_mode {
                return self.rollback_while_in_math(run, layout_state);
            }
            let sub = &mut self.sub_line as *mut SubLineEnv;
            self.excess.super_.attachSubLineEnv(sub);
            let cont = self.excess.doAddExcessGraphs(&mut self.align, None, run, self.core_width, layout_state);
            if let Some(last) = self.sub_line.lastGraphs.clone() {
                self.collect_line_height(&last, layout_state);
            }
            cont
        } else {
            IHAlignHandler::onAddRun(&mut self.align, unsafe { &mut *run });
            if !self.math_mode {
                self.collect_line_height(unsafe { &*run }, layout_state);
            }
            true
        }
    }

    // java SubLineLayouterBase.rollbackWhileInMathPara
    fn rollback_while_in_math(&mut self, excess: *mut Graphs, layout_state: &mut LayoutState) -> bool {
        let first = MathParaSubLineTool::getFirstRollbackGraphs(layout_state, &self.sub_line, excess);
        if first.is_null() {
            let sub = &mut self.sub_line as *mut SubLineEnv;
            self.excess.super_.attachSubLineEnv(sub);
            return self.excess.doAddExcessGraphs(&mut self.align, None, excess, self.core_width, layout_state);
        }
        if !std::ptr::eq(excess, first) {
            let mut cur = layout_state.getGraphList().getPrev(excess);
            while !cur.is_null() && !std::ptr::eq(cur, first) {
                IHAlignHandler::onRemoveRun(&mut self.align, unsafe { &mut *cur });
                cur = layout_state.getGraphList().getPrev(cur);
            }
            IHAlignHandler::onRemoveRun(&mut self.align, unsafe { &mut *first });
        }
        let prev = layout_state.getGraphList().getPrev(first);
        if prev.is_null() {
            self.sub_line.lastGraphs = None;
            self.sub_line.lastGraphNode = None;
        } else {
            self.sub_line.lastGraphs = Some(unsafe { (*prev).clone() });
            self.sub_line.lastGraphNode = Some(prev);
        }
        layout_state.push_ptr(first);
        false
    }

    fn align_run(&self, run: *mut Graphs, drawing: i32) {
        let run = unsafe { &mut *run };
        if self.math_mode {
            // java MathContentLayouter.vAlignRun — no hpsPos
            if run.isEmbed() && drawing != 0 {
                let snapshot = unsafe { &(*self.main).snapshot };
                if TypoDrawing::isMath__int_typosnapshot(drawing, snapshot) {
                    let _align_type = TypoDrawing::getAlignType__int_typosnapshot(drawing, snapshot);
                    let align_line = TypoDrawing::getAlignLine__int_typosnapshot(drawing, snapshot);
                    let ascent = run.fontStyle.as_ref().map(|style| style.maxAscent).unwrap_or(0);
                    run.baseLine = self.base_line - align_line + ascent;
                } else {
                    run.baseLine = self.base_line;
                }
            } else {
                run.baseLine = self.base_line;
            }
            return;
        }
        if self.equation_valign {
            run.baseLine = self.base_line;
            if let Some(style) = run.fontStyle.as_ref() {
                run.baseLine += style.hpsPos;
            }
            return;
        }
        if run.isEmbed() && drawing != 0 {
            let snapshot = unsafe { &(*self.main).snapshot };
            if TypoDrawing::isMath__int_typosnapshot(drawing, snapshot) {
                let align_line = TypoDrawing::getAlignLine__int_typosnapshot(drawing, snapshot);
                let ascent = run.fontStyle.as_ref().map(|style| style.maxAscent).unwrap_or(0);
                run.baseLine = self.base_line - align_line + ascent;
            } else {
                run.baseLine = self.base_line;
            }
        } else {
            let hps = run.fontStyle.as_ref().map(|style| style.hpsPos).unwrap_or(0);
            run.baseLine = self.base_line + hps;
        }
    }

    // java private void flowTextLine(LayoutState layoutState)
    fn flow_text_line(&mut self, layout_state: &mut LayoutState) -> bool {
        self.omath_active = self.math_mode && self.use_align_omath && layout_state.isInMathPara();
        self.init_sub_line(layout_state);
        {
            let snapshot = unsafe { &mut (*self.main).snapshot };
            self.text_line.alloc(snapshot);
        }
        let mut has_embed = false;
        loop {
            let run = layout_state.get();
            if run.is_null() {
                break;
            }
            if unsafe { (*run).isEmbed() } {
                has_embed = true;
                let drawing = layout_state.getDrawing(unsafe { &*run });
                if drawing != 0 {
                    let mut holder = TypoDrawing::new();
                    holder.assignment(drawing, unsafe { &mut (*self.main).snapshot as *mut TypoSnapshot });
                    holder.setParent(self.text_line.mPointer);
                }
            }
            if !self.add_run(run, layout_state) {
                break;
            }
        }
        let Some(last) = self.sub_line.lastGraphs.clone() else { return false; };
        let last_graph_index = last.getEndGraphIndex();
        let last_run_type = last.runType;
        self.calc_line_height();
        let line_height = self.line_height;
        let parent = layout_state.curPage.as_ref().map(|page| page.getPointer()).unwrap_or(0);
        self.text_line.setPara__int(layout_state.typoPara.getPointer());
        self.text_line.setParent(parent);
        self.text_line.setTop(self.cur_layout_height);
        self.text_line.setTextRectTop(0);
        self.text_line.setTextRectHeight(line_height);
        self.text_line.setAscDesPos(0, line_height);
        self.text_line.setHeight(line_height);
        let start_index = if self.sub_line.firstGraphIndexInCP > 0 {
            self.sub_line.firstGraphIndexInCP
        } else {
            self.sub_line.firstGraphs.as_ref().map(|g| g.graphIndex).unwrap_or(0)
        };
        self.text_line.setGraphIndexStart(start_index);
        self.text_line.setGraphIndexEnd(last_graph_index);
        self.text_line.setHasEmbedsDrawing(has_embed);
        self.cur_layout_height = self.text_line.top + self.text_line.height;
        let ltr = !layout_state.hasBidiGraph() && !layout_state.paraEnv.isRTL;
        if ltr {
            let mut rect = TypoRectCoreImpl::default();
            rect.setLeft(self.text_line.getLeft());
            rect.setWidth(self.text_line.getWidth());
            if self.omath_active {
                self.omath.adjustLine(&mut rect);
            } else {
                IHAlignHandler::adjustLine(&mut self.align, &mut rect);
            }
            self.text_line.setLeft(rect.getLeft());
            self.text_line.setWidth(rect.super_.right - rect.super_.left);
            self.adjust_italic_tail(layout_state);
            let first_index = self.sub_line.firstGraphs.as_ref().map(|g| g.graphIndex).unwrap_or(0);
            let last_index = self.sub_line.lastGraphs.as_ref().map(|g| g.graphIndex).unwrap_or(first_index);
            let count = layout_state.graphList.nodes.len();
            for i in 0..count {
                let run = layout_state.graphList.get_at(i);
                if run.is_null() {
                    continue;
                }
                let index = unsafe { (*run).graphIndex };
                if index < first_index {
                    continue;
                }
                if index > last_index {
                    break;
                }
                let run_type = unsafe { (*run).runType };
                if run_type != 8 && run_type != 9 {
                    let drawing = if unsafe { (*run).isEmbed() } { layout_state.getDrawing(unsafe { &*run }) } else { 0 };
                    self.align_run(run, drawing);
                } else {
                    let ascent = unsafe { (*run).fontStyle.as_ref().map(|style| style.maxAscent).unwrap_or(0) };
                    unsafe { (*run).baseLine = ascent; }
                }
                IHAlignHandler::adjustRun(&mut self.align, unsafe { &mut *run });
                if index == last_index {
                    break;
                }
            }
        } else if let Some(last) = self.sub_line.lastGraphs.as_ref() {
            // BidiTool is not compiled. Runs stay at the positions addRun wrote.
            self.text_line.setWidth(last.getRight().max(0));
        }
        let mem_end = unsafe { (*self.main).snapshot.getAllocSize() } as i32;
        self.text_line.setMemEnd(mem_end);
        let line_ptr = self.text_line.mPointer;
        let line_pos = layout_state.linePos;
        layout_state.linePos = line_pos + 1;
        if let Some(page) = layout_state.curPage.as_mut() {
            page.insertLine(line_ptr, line_pos);
        }
        if let Some(last) = self.sub_line.lastGraphs.clone() {
            layout_state.consumeGraphsBefore(&last, false);
        }
        if layout_state.paraEnv.isRTL {
            self.text_line.setRTL();
        }
        match last_run_type {
            4 | 5 | 10 | 11 | 12 | 13 | 14 => self.init_para_prop(layout_state),
            _ => {}
        }
        true
    }

    // java italic / negative fontSpace tail on the last normal run
    fn adjust_italic_tail(&mut self, layout_state: &mut LayoutState) {
        let Some(last) = self.sub_line.lastGraphs.as_ref() else { return; };
        if last.runType != 1 {
            return;
        }
        let Some(style) = last.fontStyle.as_ref() else { return; };
        if let Some(typeface) = style.typeface.as_ref() {
            let italic = (typeface.getStyle() & 2) != 0 || typeface.needItalic() || Self::is_italic_font(&typeface.getFamily().map(|family| family.getName()).unwrap_or_default());
            if italic && last.graphLength > 0 {
                let ch = layout_state.typoPara.getChar(last.graphIndex + last.graphLength - 1);
                let rect = typeface.getTextRect_char(style.fontSize as f32, ch as u16);
                let extra = ((rect.bottom - rect.top) * -0.25) as i32;
                self.text_line.setWidth(self.text_line.getWidth() - extra);
            }
        }
        if style.fontSpace < 0 {
            let mut extend = style.fontSpace;
            if last.language == 2 && unsafe { (*self.doc).balanceSingleByteDoubleByteWidth } {
                extend *= 2;
            }
            self.text_line.setWidth(self.text_line.getWidth() - extend);
        }
    }

    fn is_italic_font(name: &str) -> bool {
        name == "华文隶书" || name == "华文行楷"
    }
}
