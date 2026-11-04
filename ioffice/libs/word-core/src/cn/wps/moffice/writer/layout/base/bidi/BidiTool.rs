// Java: cn.wps.moffice.writer.layout.base.bidi.BidiTool
#![allow(dead_code, non_snake_case, unused_variables)]

use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
use crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle;
use crate::cn::wps::moffice::writer::core::Document::Document;
use util::moffice::writer::data::value::FileFormat::FileFormat;
use crate::cn::wps::moffice::writer::data::value::Toggle::Toggle;
use crate::cn::wps::moffice::writer::layout::base::bidi::BidiItem::BidiItem;
use crate::cn::wps::moffice::writer::layout::base::bidi::BidiLevelCache::BidiLevelCache;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphAlignTool::GraphAlignTool;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;
use crate::cn::wps::moffice::writer::layout::base::line::TextOpCollector::TextOpCollector;
use crate::cn::wps::moffice::writer::layout::base::line::align::IHAlignHandler::IHAlignHandler;
use crate::cn::wps::moffice::writer::layout::base::line::style::BordersCollector::BordersCollector;
use crate::cn::wps::moffice::writer::layout::base::line::style::HighlightCollector::HighlightCollector;
use crate::cn::wps::moffice::writer::layout::base::line::style::SHDCollector::SHDCollector;
use crate::cn::wps::moffice::writer::layout::base::line::style::StrikeThroughCollector::StrikeThroughCollector;
use crate::cn::wps::moffice::writer::layout::base::line::style::UnderlineCollector::UnderlineCollector;
use thirdparty_font::cn::wps::font::ITypeface::ITypeface;

pub trait BidiStyleCollector {
    fn addRun(&mut self, run: &mut Graphs, runLeft: i32, runRight: i32, revisionAuthor: i32, hyperlinkId: i32);
    fn processTrailSpace(&mut self);
    fn setCollectTrailSpace(&mut self, val: bool);
    fn noForceInLineBoundary(&mut self);
}

macro_rules! impl_bidi_collector {
    ($t:ty) => {
        impl BidiStyleCollector for $t {
            fn addRun(&mut self, run: &mut Graphs, runLeft: i32, runRight: i32, revisionAuthor: i32, hyperlinkId: i32) {
                <$t>::addRun(self, run, runLeft, runRight, revisionAuthor, hyperlinkId);
            }
            fn processTrailSpace(&mut self) { <$t>::processTrailSpace(self); }
            fn setCollectTrailSpace(&mut self, val: bool) { <$t>::setCollectTrailSpace(self, val); }
            fn noForceInLineBoundary(&mut self) { <$t>::noForceInLineBoundary(self); }
        }
    };
}

impl_bidi_collector!(UnderlineCollector);
impl_bidi_collector!(StrikeThroughCollector);
impl_bidi_collector!(HighlightCollector);
impl_bidi_collector!(SHDCollector);
impl_bidi_collector!(BordersCollector);

// java public class BidiTool
pub struct BidiTool {
    mTextOpCollector: *mut TextOpCollector,
    mDocEnv: *mut DocumentEnv,
    mLastGraphs: *mut Graphs,
    mFinishCollect: bool,
    mLastConvert: *mut Graphs,
    mPrevGraphsLevel: i32,
    mExitToLevel: i32,
    mBidiLevelCache: BidiLevelCache,
    mLastCharsKey: Option<(usize, i32, i32)>,
    mStyleCollector: Vec<*mut dyn BidiStyleCollector>,
    bidiBuff: Vec<char>,
    bidiPos: Vec<i32>,
    bidisHead: usize,
    bidisTail: usize,
    pool: Vec<BidiItem>,
    mAlignLine: i32,
    mWordAlignFont: i32,
}

impl BidiTool {
    pub fn new(documentEnv: *mut DocumentEnv) -> Self {
        Self {
            mTextOpCollector: std::ptr::null_mut(),
            mDocEnv: documentEnv,
            mLastGraphs: std::ptr::null_mut(),
            mFinishCollect: false,
            mLastConvert: std::ptr::null_mut(),
            mPrevGraphsLevel: -1,
            mExitToLevel: -1,
            mBidiLevelCache: BidiLevelCache::new(),
            mLastCharsKey: None,
            mStyleCollector: Vec::new(),
            bidiBuff: Vec::new(),
            bidiPos: Vec::new(),
            bidisHead: 0,
            bidisTail: 0,
            pool: Vec::new(),
            mAlignLine: 0,
            mWordAlignFont: 2,
        }
    }

    pub fn dispose(&mut self) {
        self.mTextOpCollector = std::ptr::null_mut();
        self.mLastGraphs = std::ptr::null_mut();
        self.mLastConvert = std::ptr::null_mut();
        self.mStyleCollector.clear();
    }

    pub fn setTextOpCollector(&mut self, textOpCollector: *mut TextOpCollector) {
        self.mTextOpCollector = textOpCollector;
    }

    pub fn setStyleCollectors(&mut self, collectors: Vec<*mut dyn BidiStyleCollector>) {
        self.mStyleCollector = collectors;
    }

    pub fn setAlign(&mut self, alignLine: i32, wordAlignFont: i32) {
        self.mAlignLine = alignLine;
        self.mWordAlignFont = wordAlignFont;
    }

    // java public void calcBidi(...)
    pub fn calcBidi(&mut self, layoutState: &mut LayoutState, subLineEnv: &mut SubLineEnv, hAlign: &mut dyn IHAlignHandler, line: &mut TypoTextLine, lastGraphIndex: i32) {
        let Some(first) = subLineEnv.firstGraphs.clone() else { return; };
        let Some(last) = subLineEnv.lastGraphs.clone() else { return; };
        self.mLastCharsKey = None;
        let paraIsRTL = layoutState.paraEnv.isRTL;
        let state = layoutState as *mut LayoutState;
        let mut idx = find_index(state, first.graphIndex);
        if idx.is_none() {
            return;
        }
        let mut idx = idx.unwrap();
        self.mLastGraphs = node_at(state, find_index(state, last.graphIndex).unwrap_or(idx));
        self.mFinishCollect = false;
        self.mLastConvert = std::ptr::null_mut();
        self.mPrevGraphsLevel = -1;
        self.mExitToLevel = -1;
        let mut root = self.obtain();
        let first_ptr = node_at(state, idx);
        let first_auto = !first_ptr.is_null() && unsafe { (*first_ptr).isAutoNum() };
        let last_auto = !self.mLastGraphs.is_null() && unsafe { (*self.mLastGraphs).isAutoNum() };
        if paraIsRTL && first_auto && !last_auto {
            let mut last_auto_idx = idx;
            if let Some(p) = node_at_opt(state, idx) {
                last_auto_idx = idx;
                let mut i = idx;
                while let Some(g) = node_at_opt(state, i) {
                    if !unsafe { (*g).isAutoNum() } { break; }
                    last_auto_idx = i;
                    i += 1;
                }
                let _ = p;
            }
            idx = find_index(state, first.graphIndex).unwrap_or(idx);
            let mut auto_root = self.obtain();
            let last_node = node_at(state, last_auto_idx);
            let mut needProcessLastAutoNum = false;
            if !last_node.is_null() && unsafe { (*last_node).isSpace() || (*last_node).isAutoNum() } && first_ptr != last_node {
                needProcessLastAutoNum = true;
                if last_auto_idx > 0 {
                    self.mLastGraphs = node_at(state, last_auto_idx - 1);
                }
            } else {
                self.mLastGraphs = last_node;
            }
            self.collectBidi(state, &mut idx, &mut auto_root, 0);
            if needProcessLastAutoNum {
                let mut lastAutoNumBidi = self.obtain();
                self.mLastGraphs = last_node;
                self.mFinishCollect = false;
                self.mLastConvert = std::ptr::null_mut();
                self.mPrevGraphsLevel = -1;
                self.mExitToLevel = -1;
                idx = last_auto_idx;
                self.collectBidi(state, &mut idx, &mut lastAutoNumBidi, 0);
                auto_root.addFront(&mut lastAutoNumBidi.children);
                self.recycle(lastAutoNumBidi);
            }
            self.mLastGraphs = node_at(state, find_index(state, last.graphIndex).unwrap_or(0));
            self.mFinishCollect = false;
            self.mLastConvert = std::ptr::null_mut();
            self.mPrevGraphsLevel = -1;
            self.mExitToLevel = -1;
            self.collectBidi(state, &mut idx, &mut root, 0);
            root.addBackList(&mut auto_root.children);
            self.recycle(auto_root);
        } else {
            self.collectBidi(state, &mut idx, &mut root, 0);
        }
        if !self.mLastGraphs.is_null() {
            subLineEnv.lastGraphs = Some(Clone::clone(unsafe { &*self.mLastGraphs }));
        }
        hAlign.prepareRecalcForBidi();
        if paraIsRTL {
            for i in (0..root.children.len()).rev() {
                let bidi = &root.children[i];
                if !bidi.graphs.is_null() {
                    hAlign.onAddBidiRun(unsafe { &mut *bidi.graphs }, bidi.isRTL);
                }
            }
        } else {
            for bidi in &root.children {
                if !bidi.graphs.is_null() {
                    hAlign.onAddBidiRun(unsafe { &mut *bidi.graphs }, bidi.isRTL);
                }
            }
        }
        let mut rect = TypoRectCoreImpl::default();
        rect.setLeft(line.getLeft());
        rect.setWidth(line.getWidth());
        hAlign.adjustBidiLine(&mut rect);
        line.setLeft(rect.getLeft());
        line.setWidth(rect.super_.right - rect.super_.left);
        let mut offset = rect.getLeft();
        if paraIsRTL {
            offset = unsafe { (*state).pageEnv.getCoreWidth() } - rect.getLeft() - (rect.super_.right - rect.super_.left);
        }
        self.place_runs(state, &root, offset, lastGraphIndex, line, hAlign);
        self.recycle(root);
    }

    fn collectBidi(&mut self, state: *mut LayoutState, idx: &mut usize, parent: &mut BidiItem, level: i32) {
        loop {
            let run = node_at(state, *idx);
            if run.is_null() {
                return;
            }
            if self.mLastConvert != run {
                self.processForBidi(run, state);
                self.mLastConvert = run;
            }
            let mut bidi = self.obtain();
            let graphIndex = unsafe { (*run).graphIndex };
            let runLevel = self.mBidiLevelCache.getBidiLevel(graphIndex) as i32;
            if self.mPrevGraphsLevel >= 0 && runLevel != self.mPrevGraphsLevel {
                self.mExitToLevel = self.mPrevGraphsLevel.min(runLevel);
                let para_rtl = unsafe { (*state).paraEnv.isRTL };
                if para_rtl {
                    if self.mExitToLevel % 2 == 0 {
                        self.mExitToLevel -= 1;
                    }
                } else if self.mExitToLevel % 2 != 0 && self.mExitToLevel == self.mPrevGraphsLevel && runLevel % 2 != 0 {
                    self.mExitToLevel -= 1;
                }
                if self.mExitToLevel != self.mPrevGraphsLevel {
                    self.mExitToLevel = self.mExitToLevel.max(0);
                    self.recycle(bidi);
                    return;
                }
                self.mExitToLevel = -1;
                self.mPrevGraphsLevel = -1;
                let before = *idx;
                self.collectBidi(state, idx, &mut bidi, level + 1);
                if *idx == before {
                    *idx += 1;
                }
            } else if runLevel > level {
                let before = *idx;
                self.collectBidi(state, idx, &mut bidi, level + 1);
                if *idx == before {
                    *idx += 1;
                }
            } else {
                let sameLevelLength = self.mBidiLevelCache.getSameLevelLength(graphIndex);
                let graphLength = unsafe { (*run).graphLength };
                if sameLevelLength < graphLength {
                    let break_at = graphIndex + sameLevelLength;
                    let was_last = self.mLastGraphs == run;
                    unsafe { (*state).breakGraphs(&mut *run, true, break_at, false); }
                    if was_last {
                        self.mLastGraphs = next_node(state, run);
                    }
                    *idx += 1;
                    self.mLastConvert = node_at(state, *idx);
                } else {
                    *idx += 1;
                }
                if runLevel % 2 != 0 {
                    bidi.isRTL = true;
                    if self.mBidiLevelCache.isMirrored(graphIndex) && !self.isAutoNumsymbol(run) {
                        unsafe { (*run).runType = 19; }
                    }
                    let font_space = unsafe { (*run).fontStyle.as_ref().map(|f| f.fontSpace).unwrap_or(0) };
                    if font_space > 0 {
                        unsafe { (*state).addDrawPosOffsetX__int_int(unsafe { (*run).graphIndex }, -font_space); }
                    }
                }
                bidi.graphs = run;
                if self.mLastGraphs == run {
                    self.mFinishCollect = true;
                }
                self.mPrevGraphsLevel = runLevel;
            }
            if bidi.hasChildBidi() {
                parent.addReversionList(&mut bidi.children);
                self.recycle(bidi);
            } else {
                parent.addBack(bidi);
            }
            if self.mExitToLevel >= 0 {
                if self.mExitToLevel != level {
                    return;
                }
                self.mExitToLevel = -1;
                self.mPrevGraphsLevel = -1;
                if self.mFinishCollect {
                    return;
                }
            } else if self.mFinishCollect {
                return;
            }
        }
    }

    fn isAutoNumsymbol(&self, run: *mut Graphs) -> bool {
        if run.is_null() {
            return false;
        }
        unsafe {
            (*run).fontStyle.as_ref()
                .and_then(|fs| fs.typeface.as_ref())
                .and_then(|tf| tf.getFamily())
                .map(|fam| fam.getName() == "Wingdings")
                .unwrap_or(false)
        }
    }

    fn place_runs(&mut self, state: *mut LayoutState, parent: &BidiItem, mut offset: i32, lastGraphIndex: i32, line: &mut TypoTextLine, hAlign: &mut dyn IHAlignHandler) {
        if self.mTextOpCollector.is_null() {
            return;
        }
        unsafe { (*self.mTextOpCollector).initDrawPosHandler(&*state, lastGraphIndex); }
        let mut underLinePos = 0i32;
        let mut hasEmbed = false;
        for bidi in &parent.children {
            let run = bidi.graphs;
            if run.is_null() { continue; }
            let run = unsafe { &mut *run };
            hAlign.adjustRun(run);
            let runType = run.runType;
            if runType != 8 && runType != 9 {
                self.adjust_v(run);
                if runType == 6 {
                    let drawing = unsafe { (*state).getDrawing(run) };
                    if drawing != 0 && !line.mSnapshot.is_null() {
                        unsafe { (*line.mSnapshot).set__int_int(drawing + 2, line.mPointer); }
                    }
                    hasEmbed = true;
                }
            } else if let Some(style) = run.fontStyle.as_ref() {
                run.baseLine = style.maxAscent;
            }
            let isRTL = bidi.isRTL;
            if isRTL {
                unsafe { (*state).processRTLGraphWidth(run); }
                if run.hasBdr() {
                    run.mBdrInfo.setRtl(true);
                }
            }
            let graphsPos = offset;
            offset += run.width();
            let mut revisionAuthor = run.getRevisionInsAuthor();
            let revisionDelAuthor = run.getRevisionDelAuthor();
            if revisionDelAuthor != -1 {
                revisionAuthor = revisionDelAuthor;
            }
            let hyperlinkId = run.getHyperlinkId();
            let runLeft = graphsPos;
            let runRight = graphsPos + run.width();
            let run_ptr = run as *mut Graphs;
            unsafe { (*self.mTextOpCollector).addRun(&mut *run_ptr, &mut *state, graphsPos, runRight, isRTL); }
            for c in &self.mStyleCollector {
                if !c.is_null() {
                    unsafe { (**c).addRun(&mut *run_ptr, runLeft, runRight, revisionAuthor, hyperlinkId); }
                }
            }
            if let Some(style) = unsafe { (*run_ptr).fontStyle.as_ref() } {
                let org = style.getOrgFontStyle();
                underLinePos = underLinePos.max(underline_pos(unsafe { (*run_ptr).baseLine }, &org));
            }
        }
        for c in &self.mStyleCollector {
            if !c.is_null() {
                unsafe { (**c).processTrailSpace(); }
            }
        }
        if hasEmbed {
            line.setHasEmbedsDrawing(true);
        }
        line.setYPos(underLinePos);
    }

    fn adjust_v(&self, run: &mut Graphs) {
        let Some(font) = run.fontStyle.clone() else { return; };
        let align = self.mAlignLine;
        run.baseLine = match self.mWordAlignFont {
            0 => align + font.hpsPos + font.getAscentBeforeSupOrSubScript(false, false),
            1 => {
                let top = align - font.getHeightBeforeSupOrSubScript() / 2 + font.hpsPos;
                let ascent = font.getAscentBeforeSupOrSubScript(true, true);
                top + ascent - ((font.getExternalBeforeSupOrSubScript() / 2.0) as i32)
            }
            3 => {
                let factor = font.getSupOrSubFactor();
                let desPart = (((font.maxDescent as f32) - font.bdrWidth) * factor * (1.0 - GraphAlignTool::getReversedFactorForBottomAlign(font.isCJKFont()))) as i32;
                let ascent = font.getAscentBeforeSupOrSubScript(true, true);
                let top = align - (ascent + desPart) + font.hpsPos;
                top + ascent
            }
            _ => align + font.hpsPos,
        };
    }

    fn processForBidi(&mut self, graph: *mut Graphs, state: *mut LayoutState) {
        let graphIndex = unsafe { (*graph).graphIndex };
        while self.bidisTail != self.bidisHead && !self.bidiPos.is_empty() && graphIndex >= self.bidiPos[self.bidisHead] {
            let ch = self.removeFirstBidi();
            self.mBidiLevelCache.processBidiChar(ch);
        }
        let run = unsafe { &*graph };
        let in_math = run.charsRef.as_ref().map(|c| c.isInMath).unwrap_or(false);
        if in_math {
            self.mBidiLevelCache.addMath(run.graphIndex, run.graphLength, unsafe { (*state).paraEnv.isRTL });
            self.mLastCharsKey = None;
        } else if run.runType == 6 {
            self.mBidiLevelCache.addEmbed(run.graphIndex, unsafe { (*state).paraEnv.isRTL });
            self.mLastCharsKey = None;
        } else if let Some(charNode) = run.charsRef.as_ref() {
            let key = chars_key(charNode);
            if self.mLastCharsKey != Some(key) {
                let para_rtl = unsafe { (*state).paraEnv.isRTL };
                let src_type = charNode.srcRef.map(|p| unsafe { (*p).r#type }).unwrap_or(-1);
                let toggle = if src_type == 1 && para_rtl {
                    Some(Toggle::ON)
                } else {
                    charNode.finalRunProp.as_ref().and_then(|p| p.getValue(57)).and_then(|v| {
                        v.as_any().downcast_ref::<Toggle>().copied().or_else(|| v.as_any().downcast_ref::<i32>().map(|n| Toggle::from_value(*n)))
                    })
                };
                let lang_null = charNode.finalRunProp.as_ref().and_then(|p| p.getValue(43)).is_none();
                let is_doc = !self.mDocEnv.is_null() && unsafe {
                    (*self.mDocEnv).textDocument.as_ref().map(|t| t.getFileFormat() == FileFormat::FF_DOC).unwrap_or(false)
                };
                let glen = self.getGraphLen(graph, state);
                self.mBidiLevelCache.start(unsafe { &(*state).typoPara }, run.graphIndex, glen, para_rtl, toggle, is_doc, lang_null);
                self.mLastCharsKey = Some(key);
            }
        }
    }

    fn getGraphLen(&self, graph: *mut Graphs, state: *mut LayoutState) -> i32 {
        let startGraphIndex = unsafe { (*graph).graphIndex };
        let mut max = 0i32;
        let key = unsafe { (*graph).charsRef.as_ref().map(chars_key) };
        let mut cur = graph;
        while !cur.is_null() {
            let g = unsafe { &*cur };
            let same = g.charsRef.as_ref().map(chars_key) == key;
            if !same || g.runType == 6 {
                break;
            }
            max = g.graphIndex + g.graphLength;
            if cur == self.mLastGraphs {
                break;
            }
            cur = next_node(state, cur);
        }
        max - startGraphIndex
    }

    pub fn addBidi(&mut self, graphIndex: i32, ch: char) {
        if self.bidiBuff.is_empty() {
            self.bidiBuff.resize(4, '\0');
            self.bidiPos.resize(4, 0);
        }
        self.bidiBuff[self.bidisTail] = ch;
        self.bidiPos[self.bidisTail] = graphIndex;
        self.bidisTail = self.bidisTail.wrapping_add(1) & (self.bidiBuff.len() - 1);
        if self.bidisTail == self.bidisHead {
            self.doubleBidiBuff();
        }
    }

    fn doubleBidiBuff(&mut self) {
        let p = self.bidisHead;
        let n = self.bidiBuff.len();
        let r = n - p;
        let newCapacity = n << 1;
        if newCapacity == 0 || newCapacity > isize::MAX as usize {
            return;
        }
        let mut newBidiBuff = vec!['\0'; newCapacity];
        let mut newBidiPos = vec![0i32; newCapacity];
        newBidiBuff[..r].copy_from_slice(&self.bidiBuff[p..]);
        newBidiBuff[r..r + p].copy_from_slice(&self.bidiBuff[..p]);
        newBidiPos[..r].copy_from_slice(&self.bidiPos[p..]);
        newBidiPos[r..r + p].copy_from_slice(&self.bidiPos[..p]);
        self.bidiBuff = newBidiBuff;
        self.bidiPos = newBidiPos;
        self.bidisHead = 0;
        self.bidisTail = n;
    }

    fn removeFirstBidi(&mut self) -> char {
        if self.bidisTail == self.bidisHead || self.bidiBuff.is_empty() {
            return '\0';
        }
        let c = self.bidiBuff[self.bidisHead];
        self.bidisHead = self.bidisHead.wrapping_add(1) & (self.bidiBuff.len() - 1);
        c
    }

    pub fn clean(&mut self) {
        self.mBidiLevelCache.clean();
        self.bidisHead = 0;
        self.bidisTail = 0;
    }

    pub fn setNextLineStart(&mut self, cp: i32) {
        self.mBidiLevelCache.setNextLineStart(cp);
    }

    pub fn initEmbeddings(&mut self, document: &dyn Document, paraRangeStart: i32, lineStart: i32) {
        self.mBidiLevelCache.initEmbeddings(document, paraRangeStart, lineStart);
    }

    fn obtain(&mut self) -> BidiItem {
        let mut item = self.pool.pop().unwrap_or_else(BidiItem::new);
        let kids = std::mem::take(&mut item.children);
        for kid in kids {
            self.recycle(kid);
        }
        item.reset();
        item
    }

    fn recycle(&mut self, mut item: BidiItem) {
        let kids = std::mem::take(&mut item.children);
        item.reset();
        self.pool.push(item);
        for kid in kids {
            self.recycle(kid);
        }
    }
}

fn chars_key(node: &crate::cn::wps::moffice::writer::layout::base::graph::CharsNode::CharsNode) -> (usize, i32, i32) {
    let src = node.srcRef.map(|p| p as usize).unwrap_or(0);
    (src, node.offsetOfRef, node.lengthOfRef)
}

fn underline_pos(baseLine: i32, org: &TypoFontStyle) -> i32 {
    let Some(face) = org.typeface.as_ref() else { return baseLine; };
    if org.emSize == 0.0 {
        return baseLine;
    }
    ((baseLine as f32) - ((face.getUnderlinePosition() as f32) * (org.fontSize as f32)) / org.emSize + 0.5) as i32
}

fn find_index(state: *mut LayoutState, graphIndex: i32) -> Option<usize> {
    if state.is_null() { return None; }
    unsafe { (*state).graphList.nodes.iter().position(|n| n.graphIndex == graphIndex) }
}

fn node_at(state: *mut LayoutState, idx: usize) -> *mut Graphs {
    if state.is_null() { return std::ptr::null_mut(); }
    unsafe {
        let nodes = &mut (*state).graphList.nodes;
        if idx >= nodes.len() { std::ptr::null_mut() }
        else { &mut *nodes[idx] as *mut Graphs }
    }
}

fn node_at_opt(state: *mut LayoutState, idx: usize) -> Option<*mut Graphs> {
    let p = node_at(state, idx);
    if p.is_null() { None } else { Some(p) }
}

fn next_node(state: *mut LayoutState, cur: *mut Graphs) -> *mut Graphs {
    if state.is_null() || cur.is_null() { return std::ptr::null_mut(); }
    let idx = unsafe {
        (*state).graphList.nodes.iter().position(|n| &**n as *const Graphs == cur as *const Graphs)
    };
    match idx {
        Some(i) => node_at(state, i + 1),
        None => std::ptr::null_mut(),
    }
}
