#![allow(non_snake_case)]

use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use crate::cn::wps::moffice::writer::data::DefaultSectionProperties::DefaultSectionProperties;
use crate::cn::wps::moffice::writer::data::PLCSection::as_section;
use crate::cn::wps::moffice::writer::data::value::BRC::BRC;
use crate::cn::wps::moffice::writer::data::value::PageBorderProp::PageBorderProp;
use crate::cn::wps::moffice::writer::data::value::PageSetup::PageSetup;
use crate::cn::wps::moffice::writer::layout::base::SectionTool::SectionTool;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use util::moffice::property::PropertySet::PropertySet;
use util::moffice::writer::data::value::FileFormat::FileFormat;

// java public class PageViewSectionLayouter extends SectionLayouter
pub struct PageViewSectionLayouter {
    mMainState: *mut MainState,
    mDocEnv: *mut DocumentEnv,
    mViewEnv: *mut ViewEnv,
    bStarted: bool,
    mStartCurSection: bool,
    hasTitlePage: bool,
    headerPos: i32,
    footerPos: i32,
    evenHeaderRange: i64,
    oddHeaderRange: i64,
    evenFooterRange: i64,
    oddFooterRange: i64,
    firstHeaderRange: i64,
    firstFooterRange: i64,
}

impl PageViewSectionLayouter {
    // Java: PageViewSectionLayouter.getSecPropSet reads the current section
    // and SectionTool.calcSectionPropertySet resolves its properties.
    pub fn getSecPropSet(textDocument: &TextDocument, cp: i32) -> PropertySet {
        let section = textDocument.getMainDocument()
            .and_then(|main| main.getPLCSection())
            .and_then(|sections| sections.getSection(cp));
        match section {
            // Java: final-revision view merges section revision properties.
            Some(node) => textDocument.getMergedSepx(&node.getPropertySet()),
            // Java: null section uses DefaultSectionProperties.getProperties().
            None => DefaultSectionProperties::getProperties(),
        }
    }

    // Java: PageViewSectionLayouter.calcPageSetup gets property 676 from
    // mMainState.secPropSet and copies its paper and margins to PageEnv.
    pub fn calcPageSetup(secPropSet: &PropertySet, pageEnv: &mut PageEnv) {
        let pageSetup = Self::page_setup(secPropSet);
        pageEnv.width = pageSetup.getWidth();
        pageEnv.height = pageSetup.getHeight();
        pageEnv.marginLeft = pageSetup.getMarginLeft();
        pageEnv.marginRight = pageSetup.getMarginRight();
        pageEnv.marginTop = pageSetup.getMarginTop();
        pageEnv.marginBottom = pageSetup.getMarginBottom();
        // Java: pageEnv.sectionMarginOffset = marginLeft - orgPageMarginLeft.
        pageEnv.sectionMarginOffset = pageEnv.marginLeft - pageEnv.orgPageMarginLeft;
        pageEnv.allowOverCoreHeight = false;
        // Java: mSecPropSet.getInt(714, DefaultValue.section_TextFlow).
        pageEnv.textFlow = secPropSet.getInt__int_int(714, 0);
        pageEnv.maxLayoutHeight = pageEnv.getCoreHeight();
    }

    // Java: PageViewSectionLayouter.getBreakCode(nextSecPropSet).
    pub fn getBreakCode(nextSecPropSet: Option<&PropertySet>) -> i32 {
        nextSecPropSet.map_or(2, |p| p.getInt__int_int(681, 2))
    }

    // java public PageViewSectionLayouter(MainState, DocumentEnv, ViewEnv, ILayouterWhileLoading)
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self {
            mMainState: main,
            mDocEnv: doc,
            mViewEnv: view,
            bStarted: false,
            mStartCurSection: true,
            hasTitlePage: false,
            headerPos: 0,
            footerPos: 0,
            evenHeaderRange: 0,
            oddHeaderRange: 0,
            evenFooterRange: 0,
            oddFooterRange: 0,
            firstHeaderRange: 0,
            firstFooterRange: 0,
        }
    }

    pub fn hasStart(&self) -> bool { self.bStarted }

    // Java fields alias the PageViewLayouter envs. Rebind after that owner moves.
    pub fn rebind(&mut self, main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) {
        self.mMainState = main;
        self.mDocEnv = doc;
        self.mViewEnv = view;
    }

    // java public void start(SectionLayouter.IStateEnvoy stateEnvoy)
    pub fn start(&mut self, state: &mut LayoutState) {
        if self.bStarted {
            if unsafe { (*self.mDocEnv).mirrorMargins } {
                self.calc_page_setup(state);
            }
        } else {
            self.on_start(state);
            self.bStarted = true;
        }
    }

    // java public void startForNoBreakPage(SectionLayouter.IStateEnvoy stateEnvoy)
    pub fn startForNoBreakPage(&mut self, state: &mut LayoutState) {
        self.mStartCurSection = false;
        self.start(state);
        self.mStartCurSection = true;
    }

    pub fn reset(&mut self) { self.bStarted = false; }

    // java LayouterBase.end: onEnd is empty, then bStarted = false.
    pub fn end(&mut self) { self.bStarted = false; }

    fn on_start(&mut self, state: &mut LayoutState) {
        self.get_sec_prop_set(state);
        // java getHeaderFooterRange calls HeaderFooterRangeCalcTool. The tool is not
        // on this compile surface, so the six ranges stay 0 and getTemplateIndex
        // returns the section slot without flowing a header/footer story.
        self.calc_grid(state);
        self.calc_page_brc(state);
        self.calc_columns();
        let prop = unsafe { (*self.mMainState).secPropSet.clone() }.unwrap_or_else(DefaultSectionProperties::getProperties);
        let nfc = prop.getInt__int_int(687, unsafe { (*self.mViewEnv).getDefSectionNfcPgn() });
        let sec_index = unsafe { (*self.mMainState).secIndex };
        let sec_start = unsafe { (*self.mMainState).secStart };
        let section_count = unsafe { (*self.mMainState).snapshot.sectionListSize() };
        if sec_start == state.cp || sec_index >= section_count {
            let mut restart = false;
            if prop.getBoolean__int_boolean(691, false) {
                unsafe { (*self.mMainState).curSecPageIndex = prop.getInt__int_int(690, 0); }
                restart = true;
            } else if state.cp == 0 {
                unsafe { (*self.mMainState).curSecPageIndex = 1; }
                restart = true;
            }
            let page_index = unsafe { (*self.mMainState).curPageIndex };
            let sec_page = unsafe { (*self.mMainState).curSecPageIndex };
            let (page_index, sec_page) = if self.mStartCurSection {
                (page_index, sec_page)
            } else {
                (page_index + 1, sec_page + 1)
            };
            let sec_count = self.section_count(state);
            unsafe {
                (*self.mMainState).snapshot.startSection(sec_index, sec_count, page_index, restart, sec_page, nfc);
            }
        }
        let page_type = prop.getInt__int_int(681, 2);
        unsafe {
            if page_type == 3 && (*self.mMainState).curSecPageIndex % 2 != 0 {
                (*self.mMainState).curSecPageIndex += 1;
            } else if page_type == 4 && (*self.mMainState).curSecPageIndex % 2 == 0 {
                (*self.mMainState).curSecPageIndex += 1;
            }
            (*self.mMainState).curSecMsoNfc = nfc;
        }
        self.apply_view_flags();
    }

    fn apply_view_flags(&mut self) {
        let doc = unsafe { (*self.mDocEnv).textDocument_mut() };
        let Some(doc) = doc else { return; };
        let split = doc.getProperty().getBoolean__int_boolean(535, false);
        unsafe { (*self.mViewEnv).setEnableFixSectionBreak(!split); }
        doc.getAsiaTypography().init();
        let document_prop = doc.getProperty();
        let view = unsafe { &mut *self.mViewEnv };
        if document_prop.contains(489) {
            view.setUlTrailSpace(document_prop.getBoolean__int_boolean(489, false));
        } else if doc.getFileFormat() == FileFormat::FF_DOCX {
            view.setUlTrailSpace(false);
        } else {
            view.setUlTrailSpace(true);
        }
    }

    fn get_sec_prop_set(&mut self, state: &mut LayoutState) {
        let cp = state.cp;
        let range_end = state.getRangeEnd();
        let plc = unsafe {
            (*self.mDocEnv).textDocument_mut()
                .and_then(|text| text.getMainDocument())
                .and_then(|main| main.getPLCSection())
                .map(|sections| sections as *const _ as *mut crate::cn::wps::moffice::writer::data::PLCSection::PLCSection)
        };
        unsafe {
            let main = &mut *self.mMainState;
            if let Some(plc) = plc {
                let sections = &*plc;
                if let Some(search) = sections.getSection(cp) {
                    main.secStart = sections.getSectionStart(&search);
                    main.secEnd = sections.getSectionEnd(&search);
                    main.sectionNode = Some(search);
                    if main.curPageIndex == 0 {
                        main.firstUseDefaultSec = false;
                    }
                } else {
                    main.secStart = 0;
                    main.secEnd = range_end;
                    main.sectionNode = None;
                    if main.curPageIndex == 0 {
                        main.firstUseDefaultSec = true;
                    }
                }
            } else {
                main.secStart = 0;
                main.secEnd = range_end;
                main.sectionNode = None;
            }
            let node = main.sectionNode;
            let view = &*self.mViewEnv;
            main.secPropSet = Some(SectionTool::calcSectionPropertySet(view, node));
            main.secIndex = match node {
                Some(node) if !node.ptr.is_null() => {
                    if let Some(plc) = plc { (&*plc).getIndex(&node) } else { 0 }
                }
                _ => 0,
            };
        }
        let node = unsafe { (*self.mMainState).sectionNode };
        let next = self.next_sec_prop(node);
        let break_code = Self::getBreakCode(next.as_ref());
        let can_continue = if break_code == 0 {
            self.can_continue(next.as_ref(), &state.pageEnv)
        } else {
            false
        };
        unsafe {
            (*self.mMainState).secBreakCode = break_code;
            (*self.mDocEnv).canContinueFlowNextSec = can_continue;
            let prop = (*self.mMainState).secPropSet.clone().unwrap_or_else(DefaultSectionProperties::getProperties);
            self.headerPos = prop.getInt__int_int(682, 720);
            self.footerPos = prop.getInt__int_int(683, 720);
            self.hasTitlePage = prop.getBoolean__int_boolean(684, false);
        }
        state.hasInitSecProp = false;
        self.calc_page_setup(state);
    }

    fn next_sec_prop(&self, node: Option<crate::cn::wps::moffice::writer::data::PLCSection::SectionNode>) -> Option<PropertySet> {
        let node = node.filter(|node| !node.ptr.is_null())?;
        let next = node.getNext();
        let plc = unsafe {
            (*self.mDocEnv).textDocument_mut()
                .and_then(|text| text.getMainDocument())
                .and_then(|main| main.getPLCSection())
        }?;
        if plc.isHead(next) {
            return None;
        }
        let view = unsafe { &*self.mViewEnv };
        Some(SectionTool::calcSectionPropertySet(view, Some(as_section(next))))
    }

    fn can_continue(&self, next: Option<&PropertySet>, page_env: &PageEnv) -> bool {
        let page_setup = next.map(Self::page_setup).unwrap_or_else(|| PageSetup::PageSetup(11906, 16838, 1800, 1800, 1440, 1440));
        if page_env.width != page_setup.getWidth() || page_env.height != page_setup.getHeight() {
            return false;
        }
        let text_flow = next.map(|prop| prop.getInt__int_int(714, 0)).unwrap_or(0);
        page_env.textFlow == text_flow
    }

    fn calc_page_setup(&mut self, state: &mut LayoutState) {
        let prop = unsafe { (*self.mMainState).secPropSet.clone() }.unwrap_or_else(DefaultSectionProperties::getProperties);
        let page_setup = Self::page_setup(&prop);
        let portrait = page_setup.getHeight() >= page_setup.getWidth();
        let swap = unsafe { SectionTool::checkPageMirrorMargins(portrait, &*self.mDocEnv, &mut *self.mMainState) };
        {
            let page_env = &mut state.pageEnv;
            page_env.width = page_setup.getWidth();
            page_env.height = page_setup.getHeight();
            if swap {
                page_env.marginLeft = page_setup.getMarginRight();
                page_env.marginRight = page_setup.getMarginLeft();
            } else {
                page_env.marginLeft = page_setup.getMarginLeft();
                page_env.marginRight = page_setup.getMarginRight();
            }
            page_env.marginTop = page_setup.getMarginTop();
            page_env.marginBottom = page_setup.getMarginBottom();
            page_env.sectionMarginOffset = page_env.marginLeft - page_env.orgPageMarginLeft;
            page_env.allowOverCoreHeight = false;
            page_env.textFlow = prop.getInt__int_int(714, 0);
        }
        self.set_snapshot_max_size(state);
    }

    fn set_snapshot_max_size(&mut self, state: &mut LayoutState) {
        let mut max_width = 0;
        let mut max_height = 0;
        let plc = unsafe {
            (*self.mDocEnv).textDocument_mut()
                .and_then(|text| text.getMainDocument())
                .and_then(|main| main.getPLCSection())
                .map(|sections| sections as *const _ as *mut crate::cn::wps::moffice::writer::data::PLCSection::PLCSection)
        };
        if let Some(plc) = plc {
            let mut node = unsafe { (*plc).getFirstSection() };
            while let Some(current) = node {
                let prop = SectionTool::calcSectionPropertySet(unsafe { &*self.mViewEnv }, Some(current));
                if let Some(setup) = prop.getValue(676).and_then(|value| value.as_ref().as_any().downcast_ref::<PageSetup>().copied()) {
                    max_width = max_width.max(setup.getWidth());
                    max_height = max_height.max(setup.getHeight());
                }
                let next = current.getNext();
                if unsafe { (*plc).isHead(next) } {
                    break;
                }
                node = Some(as_section(next));
            }
        }
        if max_width <= 0 {
            let props = DefaultSectionProperties::getProperties();
            if let Some(setup) = props.getValue(676).and_then(|value| value.as_ref().as_any().downcast_ref::<PageSetup>().copied()) {
                max_width = setup.getWidth();
                max_height = setup.getHeight();
            }
        }
        let _ = state;
        unsafe {
            (*self.mMainState).snapshot.setDocumentWidth(max_width);
            (*self.mMainState).snapshot.setMaxPageHeight(max_height);
        }
    }

    fn calc_grid(&self, state: &mut LayoutState) {
        let prop = unsafe { (*self.mMainState).secPropSet.clone() }.unwrap_or_else(DefaultSectionProperties::getProperties);
        let text = unsafe { (*self.mDocEnv).textDocument_mut() };
        SectionTool::calcGrid(&mut state.pageEnv, &prop, text.map(|doc| &*doc));
    }

    fn calc_page_brc(&self, state: &mut LayoutState) {
        let prop = unsafe { (*self.mMainState).secPropSet.clone() }.unwrap_or_else(DefaultSectionProperties::getProperties);
        state.brcTop = Self::brc_of(&prop, 717);
        state.brcLeft = Self::brc_of(&prop, 718);
        state.brcRight = Self::brc_of(&prop, 720);
        state.brcBottom = Self::brc_of(&prop, 719);
        state.pageBorderProp = prop.getValue(715)
            .and_then(|value| value.as_ref().as_any().downcast_ref::<PageBorderProp>().cloned());
    }

    fn calc_columns(&self) {
        let prop = unsafe { (*self.mMainState).secPropSet.clone() }.unwrap_or_else(DefaultSectionProperties::getProperties);
        unsafe { (*self.mMainState).columns = prop.getInt__int_int(680, 0) + 1; }
    }

    // java public int getTemplateIndex(SectionLayouter.IStateEnvoy stateEnvoy)
    pub fn getTemplateIndex(&self, state: &LayoutState) -> i32 {
        let mut header_range = 0i64;
        let mut footer_range = 0i64;
        let mut template_index = unsafe { (*self.mMainState).secIndex } * 3;
        if self.hasTitlePage && state.pageEnv.isFirstPageOfSection {
            header_range = self.firstHeaderRange;
            footer_range = self.firstFooterRange;
        } else if unsafe { (*self.mMainState).curSecPageIndex } % 2 == 0 {
            header_range = self.evenHeaderRange;
            footer_range = self.evenFooterRange;
            template_index += 2;
        } else {
            header_range = self.oddHeaderRange;
            footer_range = self.oddFooterRange;
            template_index += 1;
        }
        let _ = (header_range, footer_range);
        // java RANGEFactory.getLength == 0 for both ranges returns the index.
        // Header/footer ranges are still 0 until HeaderFooterRangeCalcTool lands.
        template_index
    }

    fn section_count(&self, _state: &LayoutState) -> i32 {
        unsafe {
            (*self.mDocEnv).textDocument_mut()
                .and_then(|text| text.getMainDocument())
                .and_then(|main| main.getPLCSection())
                .map(|sections| sections.size())
                .unwrap_or(0)
        }
    }

    fn page_setup(prop: &PropertySet) -> PageSetup {
        prop.getValue(676)
            .and_then(|value| value.as_ref().as_any().downcast_ref::<PageSetup>().copied())
            .unwrap_or_else(|| PageSetup::PageSetup(11906, 16838, 1800, 1800, 1440, 1440))
    }

    fn brc_of(prop: &PropertySet, id: i32) -> Option<BRC> {
        prop.getValue(id).and_then(|value| value.as_ref().as_any().downcast_ref::<BRC>().cloned())
    }
}
