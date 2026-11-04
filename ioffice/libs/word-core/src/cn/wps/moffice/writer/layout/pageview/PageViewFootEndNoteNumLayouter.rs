// Java: cn.wps.moffice.writer.layout.pageview.PageViewFootEndNoteNumLayouter
#![allow(dead_code, non_snake_case)]

use crate::cn::wps::collections::textrope::ITextRope::ITextRope;
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
use crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::IChpxTable::{Entry as ChpxEntry, IChpxTable};
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode;
use crate::cn::wps::moffice::writer::data::PLCSection::SectionNode;
use crate::cn::wps::moffice::writer::layout::base::SectionTool::SectionTool;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::FootEndNoteEnv::FootEndNoteEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::footendnote::FootEndNotePropCalc::FootEndNotePropCalc;
use crate::cn::wps::moffice::writer::layout::base::footendnote::IFootEndNoteNumLayouter::IFootEndNoteNumLayouter;
use crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::StateSnapshot;
use crate::cn::wps::moffice::writer::layout::numformat::Numformat::Numformat;
use util::moffice::property::PropertySet::PropertySet;

// java public class PageViewFootEndNoteNumLayouter implements IFootEndNoteNumLayouter
pub struct PageViewFootEndNoteNumLayouter {
    // java private MainState mMainState;
    mMainState: *mut MainState,
    // java private DocumentEnv mDocEnv;
    mDocEnv: *mut DocumentEnv,
    // java private ViewEnv mViewEnv;
    mViewEnv: *mut ViewEnv,
    // java private FootEndNoteEnv mFootEndNoteEnv;
    mFootEndNoteEnv: *mut FootEndNoteEnv,
    // java private int mLastFootNoteNum;
    mLastFootNoteNum: i32,
    // java private int mLastEndNoteNum;
    mLastEndNoteNum: i32,
    // java private PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode mLastFootNoteNode;
    mLastFootNoteNode: Option<FootnoteEndnoteRefNode>,
    // java private PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode mLastEndNoteNode;
    mLastEndNoteNode: Option<FootnoteEndnoteRefNode>,
    // java private PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode mFootNoteRef;
    mFootNoteRef: Option<FootnoteEndnoteRefNode>,
    // java private String mFootNoteNumStr;
    mFootNoteNumStr: Option<String>,
    // java private PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode mEndNoteRef;
    mEndNoteRef: Option<FootnoteEndnoteRefNode>,
    // java private StateSnapshot mPageViewSnapshot;
    mPageViewSnapshot: *mut StateSnapshot,
    // java private boolean mMeetBreakCellWhenSearchMinCp;
    mMeetBreakCellWhenSearchMinCp: bool,
}

impl PageViewFootEndNoteNumLayouter {
    // java public PageViewFootEndNoteNumLayouter(MainState mainState, DocumentEnv documentEnv, ViewEnv viewEnv, FootEndNoteEnv footEndNoteEnv)
    pub fn new(main_state: *mut MainState, document_env: *mut DocumentEnv, view_env: *mut ViewEnv, foot_end_note_env: *mut FootEndNoteEnv) -> Self {
        Self {
            mMainState: main_state,
            mDocEnv: document_env,
            mViewEnv: view_env,
            mFootEndNoteEnv: foot_end_note_env,
            mLastFootNoteNum: 0,
            mLastEndNoteNum: 0,
            mLastFootNoteNode: None,
            mLastEndNoteNode: None,
            mFootNoteRef: None,
            mFootNoteNumStr: None,
            mEndNoteRef: None,
            mPageViewSnapshot: std::ptr::null_mut(),
            mMeetBreakCellWhenSearchMinCp: false,
        }
    }

    // java public void setCurFootEndNoteNumStr(String numStr)
    pub fn setCurFootEndNoteNumStr(&mut self, num_str: Option<String>) {
        // java this.mFootNoteNumStr = numStr;
        self.mFootNoteNumStr = num_str;
    }

    // java public void setCurFootNoteRef(PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode footNoteRef)
    pub fn setCurFootNoteRef(&mut self, foot_note_ref: Option<FootnoteEndnoteRefNode>) {
        // java this.mFootNoteRef = footNoteRef;
        self.mFootNoteRef = foot_note_ref;
    }

    // java public void setCurEndNoteRefNode(PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode endNoteRef)
    pub fn setCurEndNoteRefNode(&mut self, end_note_ref: Option<FootnoteEndnoteRefNode>) {
        // java this.mEndNoteRef = endNoteRef;
        self.mEndNoteRef = end_note_ref;
    }

    // java private void init()
    fn init_env(&mut self) {
        if self.mFootEndNoteEnv.is_null() || self.mMainState.is_null() {
            return;
        }
        // java if (!this.mFootEndNoteEnv.hasInitSecProp)
        if unsafe { (*self.mFootEndNoteEnv).hasInitSecProp } {
            return;
        }
        let main = unsafe { &*self.mMainState };
        let section = main.sectionNode;
        let prop = main.secPropSet.clone().unwrap_or_else(PropertySet::empty);
        // java FootEndNotePropCalc.calc(this.mFootEndNoteEnv, mainState.sectionNode, mainState.secPropSet);
        FootEndNotePropCalc::calc(unsafe { &mut *self.mFootEndNoteEnv }, section, &prop);
        self.clear_last();
    }

    // java public void init(PLCSection.SectionNode sectionNode)
    pub fn init(&mut self, section_node: Option<SectionNode>) {
        if self.mFootEndNoteEnv.is_null() {
            return;
        }
        let same = section_same(unsafe { (*self.mFootEndNoteEnv).curSectionNode }, section_node);
        // java if (this.mFootEndNoteEnv.curSectionNode != sectionNode)
        if !same {
            if self.mViewEnv.is_null() {
                return;
            }
            // java PropertySet propertySet = SectionTool.calcSectionPropertySet(this.mViewEnv, sectionNode);
            let property_set = SectionTool::calcSectionPropertySet(unsafe { &*self.mViewEnv }, section_node);
            FootEndNotePropCalc::calc(unsafe { &mut *self.mFootEndNoteEnv }, section_node, &property_set);
        } else {
            // java if (this.mFootEndNoteEnv.hasInitSecProp) return;
            if unsafe { (*self.mFootEndNoteEnv).hasInitSecProp } {
                return;
            }
            if self.mMainState.is_null() {
                return;
            }
            let main = unsafe { &*self.mMainState };
            let prop = main.secPropSet.clone().unwrap_or_else(PropertySet::empty);
            FootEndNotePropCalc::calc(unsafe { &mut *self.mFootEndNoteEnv }, main.sectionNode, &prop);
        }
        self.clear_last();
    }

    fn clear_last(&mut self) {
        // java this.mLastFootNoteNum = 0; this.mLastEndNoteNum = 0; nodes = null;
        self.mLastFootNoteNum = 0;
        self.mLastEndNoteNum = 0;
        self.mLastFootNoteNode = None;
        self.mLastEndNoteNode = None;
    }

    // java private int getNum(...)
    fn get_num(&mut self, node: &FootnoteEndnoteRefNode, foot_note_restart: i32, foot_note_nfc_start: i32, is_foot_note: bool) -> i32 {
        if is_foot_note {
            // java if (null != this.mLastFootNoteNode && this.mLastFootNoteNode.getCP() < footEndNoteRefNode.getCP())
            if let Some(last) = self.mLastFootNoteNode.clone() {
                if last.node.getCP() < node.node.getCP() {
                    return self.get_num_from_last(node, &last, self.mLastFootNoteNum);
                }
            }
        } else if let Some(last) = self.mLastEndNoteNode.clone() {
            if last.node.getCP() < node.node.getCP() {
                return self.get_num_from_last(node, &last, self.mLastEndNoteNum);
            }
        }
        let mut find_cp = 0;
        let mut need_strict = false;
        // java switch (footNoteRestart)
        match foot_note_restart {
            0 => find_cp = 0,
            1 => {
                if !self.mMainState.is_null() {
                    find_cp = unsafe { (*self.mMainState).secStart };
                }
            }
            2 => {
                find_cp = self.get_min_cp();
                need_strict = self.mMeetBreakCellWhenSearchMinCp;
            }
            _ => {}
        }
        let mut count = 0;
        // java for (PLC.Node curNode = footEndNoteRefNode.getPrev(); !curNode.isHead(); curNode = curNode.getPrev())
        let mut cur = node.node.getPrev();
        while !cur.isHead() && !cur.ptr.is_null() {
            let cp = cur.getCP();
            if cp < find_cp {
                break;
            }
            let page_index = if self.mMainState.is_null() { 0 } else { unsafe { (*self.mMainState).curPageIndex } };
            if !need_strict || !self.check_cp_in_pre_page(page_index - 1, cp, find_cp) {
                let ref_node = FootnoteEndnoteRefNode::fromNode(cur);
                if ref_node.getNumStart() != 0 && !self.is_vanish(cp) {
                    count += 1;
                }
            }
            cur = cur.getPrev();
        }
        count + foot_note_nfc_start
    }

    // java private int getNumFromLastNode(...)
    fn get_num_from_last(&mut self, des: &FootnoteEndnoteRefNode, last: &FootnoteEndnoteRefNode, last_num: i32) -> i32 {
        let mut num = last_num;
        let mut cur = des.node;
        loop {
            let cp = cur.getCP();
            let cur_ref = FootnoteEndnoteRefNode::fromNode(cur);
            if cur_ref.getNumStart() != 0 && !self.is_vanish(cp) {
                num += 1;
            }
            cur = cur.getPrev();
            if cur.ptr == last.node.ptr || cur.ptr.is_null() {
                break;
            }
        }
        num
    }

    // java private boolean checkCpInPrePage(int prePageIndex, int cp, int minCp, TypoSnapshot snapshot)
    fn check_cp_in_pre_page(&self, mut pre_page_index: i32, cp: i32, min_cp: i32) -> bool {
        if pre_page_index < 0 || self.mMainState.is_null() {
            return false;
        }
        let snapshot = unsafe { &mut (*self.mMainState).snapshot };
        let p_page_list = snapshot.getLayoutPageList();
        let size = TypoList::size__int_typosnapshot(p_page_list, snapshot);
        if pre_page_index >= size {
            return false;
        }
        while pre_page_index >= 0 {
            let p_page = TypoList::quickGet__int_int_typosnapshot(pre_page_index, p_page_list, snapshot);
            if TypoPage::getMaxEndCP__int_typosnapshot(p_page, snapshot) < min_cp {
                return false;
            }
            if TypoPage::contains__int_int_typosnapshot(cp, p_page, snapshot) {
                return true;
            }
            pre_page_index -= 1;
        }
        false
    }

    // java private boolean isVanish(int refCP)
    fn is_vanish(&self, ref_cp: i32) -> bool {
        if self.mViewEnv.is_null() || !unsafe { (*self.mViewEnv).isRevisionFinal() } {
            return false;
        }
        if self.mDocEnv.is_null() {
            return false;
        }
        let Some(document) = main_document(unsafe { &*self.mDocEnv }) else { return false; };
        let Some(table) = document.getChpxTable() else { return false; };
        if table.is_null() {
            return false;
        }
        let Some(chpx) = (unsafe { IChpxTable::seek(&*table, ref_cp) }) else { return false; };
        let Some(prop) = ChpxEntry::getProperty(chpx.as_ref()) else { return false; };
        // java return prop.contains(45) || prop.getBoolean(18, false);
        prop.contains(45) || prop.getBoolean__int_boolean(18, false)
    }

    // java private int getMinCP()
    fn get_min_cp(&mut self) -> i32 {
        // java this.mMeetBreakCellWhenSearchMinCp = false;
        self.mMeetBreakCellWhenSearchMinCp = false;
        if self.mPageViewSnapshot.is_null() {
            return 0;
        }
        // java return Math.max(0, this.getMinCP(this.mPageViewSnapshot));
        0.max(self.min_cp_of(unsafe { &*self.mPageViewSnapshot }))
    }

    // java private int getMinCP(StateSnapshot stateSnapshot)
    fn min_cp_of(&mut self, state_snapshot: &StateSnapshot) -> i32 {
        let mut min_cp = state_snapshot.getStartCp();
        if let Some(first) = state_snapshot.getFirstMergeCell() {
            min_cp = self.min_cp_of(first).min(min_cp);
            self.mMeetBreakCellWhenSearchMinCp = true;
        }
        for cell in &state_snapshot.cells {
            self.mMeetBreakCellWhenSearchMinCp = true;
            min_cp = self.min_cp_of(cell).min(min_cp);
        }
        min_cp
    }
}

fn section_same(left: Option<SectionNode>, right: Option<SectionNode>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(a), Some(b)) => a.ptr == b.ptr,
        _ => false,
    }
}

fn main_document(env: &DocumentEnv) -> Option<&dyn Document> {
    if !env.textDocumentAlias.is_null() {
        return unsafe { (*env.textDocumentAlias).getMainDocument().map(|doc| doc as &dyn Document) };
    }
    env.textDocument.as_ref().and_then(|doc| doc.getMainDocument().map(|sub| sub as &dyn Document))
}

fn main_char(env: &DocumentEnv, cp: i32) -> char {
    let Some(document) = main_document(env) else { return '\0'; };
    let Some(rope) = document.getTextRope() else { return '\0'; };
    if rope.is_null() {
        return '\0';
    }
    unsafe { (*rope).charAt(cp) }
}

impl IFootEndNoteNumLayouter for PageViewFootEndNoteNumLayouter {
    // java public String getFootNoteNum(PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode footNoteRefNode)
    fn getFootNoteNum(&mut self, foot_note_ref_node: &FootnoteEndnoteRefNode) -> String {
        self.init_env();
        if self.mDocEnv.is_null() || self.mFootEndNoteEnv.is_null() {
            return String::new();
        }
        let ref_cp = foot_note_ref_node.node.getCP();
        // java char ch = document.getTextRope().charAt(refCP);
        let ch = main_char(unsafe { &*self.mDocEnv }, ref_cp);
        // java if (2 != ch) return String.valueOf(ch);
        if ch != '\u{2}' {
            return ch.to_string();
        }
        let (restart, start, nfc) = {
            let env = unsafe { &*self.mFootEndNoteEnv };
            (env.footNoteRestart, env.footNoteStart, env.footNoteRefMsoNfc)
        };
        let num = self.get_num(foot_note_ref_node, restart, start, true);
        self.mLastFootNoteNode = Some(foot_note_ref_node.clone());
        self.mLastFootNoteNum = num;
        // java return Numformat.txFormatNum(num, this.mFootEndNoteEnv.footNoteRefMsoNfc);
        Numformat::txFormatNum__int_int(num, nfc)
    }

    // java public String getEndNoteNum(PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode endNoteRefNode)
    fn getEndNoteNum(&mut self, end_note_ref_node: &FootnoteEndnoteRefNode) -> String {
        if self.mDocEnv.is_null() || self.mFootEndNoteEnv.is_null() {
            return String::new();
        }
        let ref_cp = end_note_ref_node.node.getCP();
        let section = main_document(unsafe { &*self.mDocEnv }).and_then(|doc| {
            let plc = doc.getPLCSection().unwrap_or(std::ptr::null_mut());
            if plc.is_null() { None } else { unsafe { (*plc).getSection(ref_cp) } }
        });
        // java this.init(sectionNode);
        self.init(section);
        let ch = main_char(unsafe { &*self.mDocEnv }, ref_cp);
        if ch != '\u{2}' {
            return ch.to_string();
        }
        let (restart, start, nfc) = {
            let env = unsafe { &*self.mFootEndNoteEnv };
            (env.endNoteRestart, env.endNoteStart, env.endNoteRefMsoNfc)
        };
        let num = self.get_num(end_note_ref_node, restart, start, false);
        self.mLastEndNoteNode = Some(end_note_ref_node.clone());
        self.mLastEndNoteNum = num;
        Numformat::txFormatNum__int_int(num, nfc)
    }

    // java public String getNumByCP(Document document, int cp)
    fn getNumByCP(&mut self, document: &dyn Document, _cp: i32) -> Option<String> {
        self.init_env();
        // java if (document.getType() == 1)
        if document.getType__on_IDocument() == 1 {
            if let Some(num) = self.mFootNoteNumStr.clone() {
                return Some(num);
            }
            if let Some(node) = self.mFootNoteRef.clone() {
                return Some(self.getFootNoteNum(&node));
            }
        }
        // java return null != this.mEndNoteRef && document.getType() == 4 ? this.getEndNoteNum(this.mEndNoteRef) : null;
        if document.getType__on_IDocument() == 4 {
            if let Some(node) = self.mEndNoteRef.clone() {
                return Some(self.getEndNoteNum(&node));
            }
        }
        None
    }

    // java public void dispose()
    fn dispose(&mut self) {
        self.mMainState = std::ptr::null_mut();
        self.mDocEnv = std::ptr::null_mut();
        self.mViewEnv = std::ptr::null_mut();
        self.mFootEndNoteEnv = std::ptr::null_mut();
        self.mLastFootNoteNode = None;
        self.mLastEndNoteNode = None;
        self.mFootNoteNumStr = None;
        self.mEndNoteRef = None;
    }

    fn rebind_ptrs(&mut self, main: *mut (), doc: *mut (), view: *mut (), snap: *mut ()) {
        self.mMainState = main as *mut MainState;
        self.mDocEnv = doc as *mut DocumentEnv;
        self.mViewEnv = view as *mut ViewEnv;
        self.mPageViewSnapshot = snap as *mut StateSnapshot;
    }

    // java public void reset(StateSnapshot stateSnapshot)
    fn reset(&mut self) {
        if self.mFootEndNoteEnv.is_null() {
            return;
        }
        let mut reset_foot = true;
        let mut reset_end = true;
        let env = unsafe { &*self.mFootEndNoteEnv };
        // java if (this.mFootEndNoteEnv.hasInitSecProp)
        if env.hasInitSecProp {
            if 2 != env.footNoteRestart {
                reset_foot = false;
            }
            reset_end = false;
        }
        if reset_foot {
            self.mLastFootNoteNum = 0;
            self.mLastFootNoteNode = None;
        }
        if reset_end {
            self.mLastEndNoteNum = 0;
            self.mLastEndNoteNode = None;
        }
    }
}
