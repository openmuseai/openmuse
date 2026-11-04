#![allow(non_snake_case, dead_code, unused_imports)]
// Java: cn.wps.moffice.writer.data.autonum.AutoNumCount
use std::collections::HashMap;
use std::ptr;

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::PropertySet;
use util::moffice::writer::data::value::FileFormat::FF_DOCX;

use crate::cn::wps::moffice::writer::core::Document::{Document, SoftTextDocument};
use crate::cn::wps::moffice::writer::core::RangeTool::RangeTool;
use crate::cn::wps::moffice::writer::core::property::IPropertyCalculate::IPropertyCalculate;
use crate::cn::wps::moffice::writer::core::property::PropertyCalculateImpl::PropertyCalculateImpl;
use crate::cn::wps::moffice::writer::core::property::StyleCache::StyleCache;
use crate::cn::wps::moffice::writer::data::IChpxTable::{Entry as ChpxEntry, IChpxTable};
use crate::cn::wps::moffice::writer::data::IPapxTable::{Entry as PapxEntry, IPapxTable};
use crate::cn::wps::moffice::writer::data::PropertyUtil::PropertyUtil;
use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;
use crate::cn::wps::moffice::writer::data::TableTool::TableTool;
use crate::cn::wps::moffice::writer::data::autonum::AutoNumTable::AutoNumTable;
use crate::cn::wps::moffice::writer::data::autonum::LstLfoTool::LstLfoTool;
use crate::cn::wps::moffice::writer::data::table::TableInfos::TableInfos;
use crate::cn::wps::moffice::writer::data::value::Revision::Revision;

const PLACEHOLDER: i32 = -2;
const LVLEND: i32 = -3;

// Java: AutoNumCount.AutoNum
#[derive(Clone)]
pub struct AutoNum {
    pub cp: i32,
    pub mILvl: i8,
    pub mILfo: i32,
    pub mCounts: Vec<i32>,
}

struct CPRecord {
    cp: i32,
    count: i32,
    lfo: i32,
    record: Option<Box<LvlRecord>>,
}

impl CPRecord {
    fn fresh() -> Self {
        Self { cp: 0, count: 0, lfo: i32::MIN, record: None }
    }
}

struct LvlRecord {
    nIlvl: i32,
    size: i32,
    cps: Vec<CPRecord>,
}

impl LvlRecord {
    fn new(i_lvl: i32) -> Self {
        Self { nIlvl: i_lvl, size: 0, cps: Vec::new() }
    }

    // Java: LvlRecord.getLvlCount
    fn getLvlCount(&self, cp: i32, count: &mut [i32]) -> i32 {
        let mut i = self.size - 1;
        while i >= 0 {
            let slot = &self.cps[i as usize];
            if slot.cp == cp {
                count[self.nIlvl as usize] = slot.count;
                return self.nIlvl + 1;
            }
            if slot.cp < cp {
                if let Some(record) = slot.record.as_ref() {
                    count[self.nIlvl as usize] = slot.count;
                    return record.getLvlCount(cp, count);
                }
            }
            i -= 1;
        }
        0
    }

    // Java: LvlRecord.append
    fn append(&mut self, ilvl: i32, cp: i32, tool: &LstLfoTool) {
        self.setInvalid(cp);
        if self.size == 0 || ilvl == self.nIlvl {
            self.extendArray();
        }
        let cur_index = (self.size - 1) as usize;
        if ilvl > self.nIlvl {
            if self.cps[cur_index].cp == LVLEND {
                self.cps[cur_index].cp = PLACEHOLDER;
                self.cps[cur_index].count = tool.getLevelStartAt(self.nIlvl);
                self.cps[cur_index].lfo = tool.getLfo();
            }
            if self.cps[cur_index].record.is_none() {
                self.cps[cur_index].record = Some(Box::new(LvlRecord::new(self.nIlvl + 1)));
            }
            if let Some(record) = self.cps[cur_index].record.as_mut() {
                record.append(ilvl, cp, tool);
            }
        } else {
            self.cps[cur_index].cp = cp;
            let set_start_at = if cur_index == 0 {
                true
            } else if tool.isFStartAt(self.nIlvl) {
                self.cps[cur_index - 1].lfo != tool.getLfo()
            } else {
                false
            };
            self.cps[cur_index].lfo = tool.getLfo();
            if set_start_at {
                self.cps[cur_index].count = tool.getLevelStartAt(self.nIlvl);
            } else {
                self.cps[cur_index].count = self.cps[cur_index - 1].count + 1;
            }
        }
    }

    // Java: LvlRecord.setInvalid
    fn setInvalid(&mut self, invalid_cp: i32) {
        if self.cps.is_empty() {
            return;
        }
        let mut i = self.size - 1;
        while i >= 0 {
            if self.cps[i as usize].cp >= invalid_cp {
                self.cps[i as usize].cp = LVLEND;
                self.cps[i as usize].record = None;
                self.cps[i as usize].count = 0;
                self.size -= 1;
            } else if self.cps[i as usize].cp != PLACEHOLDER {
                if let Some(record) = self.cps[i as usize].record.as_mut() {
                    record.setInvalid(invalid_cp);
                }
                break;
            } else {
                if let Some(record) = self.cps[i as usize].record.as_mut() {
                    record.setInvalid(invalid_cp);
                    if record.size == 0 {
                        self.cps[i as usize].record = None;
                    }
                }
                if self.cps[i as usize].record.is_none() {
                    self.cps[i as usize].cp = LVLEND;
                    self.cps[i as usize].count = 0;
                    self.size -= 1;
                }
            }
            i -= 1;
        }
    }

    // Java: LvlRecord.extendArray
    fn extendArray(&mut self) {
        if self.cps.is_empty() {
            self.cps.push(CPRecord::fresh());
        } else if self.size + 1 > self.cps.len() as i32 {
            self.cps.push(CPRecord::fresh());
        }
        let index = self.size as usize;
        self.cps[index].cp = LVLEND;
        self.cps[index].record = None;
        self.cps[index].count = 0;
        self.size += 1;
    }
}

// Java: public class AutoNumCount
pub struct AutoNumCount {
    mCurCp: i32,
    mLsidCounts: HashMap<i32, LvlRecord>,
    mLstLfoTool: *mut LstLfoTool,
    mDocument: Option<*mut (dyn Document + 'static)>,
    mAutoNumCache: Vec<AutoNum>,
    mAutoNumTable: *mut AutoNumTable,
    tmpCount: [i32; 9],
}

impl Default for AutoNumCount {
    fn default() -> Self {
        Self {
            mCurCp: 0,
            mLsidCounts: HashMap::new(),
            mLstLfoTool: ptr::null_mut(),
            mDocument: None,
            mAutoNumCache: Vec::new(),
            mAutoNumTable: ptr::null_mut(),
            tmpCount: [0; 9],
        }
    }
}

impl AutoNumCount {
    // Java: public AutoNumCount(Document document)
    pub fn AutoNumCount(document: &dyn Document) -> Self {
        let mut count = Self::default();
        count.mDocument = Some(unsafe {
            std::mem::transmute::<*const dyn Document, *mut (dyn Document + 'static)>(
                document as *const dyn Document,
            )
        });
        count
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn dispose(&mut self) {
        self.mLstLfoTool = ptr::null_mut();
        self.mDocument = None;
        self.mAutoNumTable = ptr::null_mut();
        self.mLsidCounts.clear();
        self.mAutoNumCache.clear();
    }

    fn document(&self) -> Option<&dyn Document> {
        self.mDocument.map(|document| unsafe { &*document })
    }

    fn document_mut(&mut self) -> Option<&mut dyn Document> {
        self.mDocument.map(|document| unsafe { &mut *document })
    }

    // Java: public void init()
    pub fn init(&mut self) {
        if !self.mLstLfoTool.is_null() {
            return;
        }
        let Some(document) = self.document() else { return };
        let Some(text) = document.getTextDocument() else { return };
        let Some(table) = text.getAutoNumTable() else { return };
        let table = table as *const AutoNumTable as *mut AutoNumTable;
        self.mAutoNumTable = table;
        self.mLstLfoTool = unsafe { (*table).getLstLfoTool() as *mut LstLfoTool };
    }

    fn tool(&self) -> Option<&LstLfoTool> {
        if self.mLstLfoTool.is_null() { None } else { Some(unsafe { &*self.mLstLfoTool }) }
    }

    fn tool_mut(&mut self) -> Option<&mut LstLfoTool> {
        if self.mLstLfoTool.is_null() { None } else { Some(unsafe { &mut *self.mLstLfoTool }) }
    }

    // Java: public synchronized void constructTo(int cp)
    pub fn constructTo(&mut self, cp: i32) {
        if self.mCurCp > cp {
            return;
        }
        self.init();
        let Some(document) = self.mDocument else { return };
        let document = unsafe { &mut *document };
        let Some(papx_ptr) = document.getPapxTable() else { return };
        let papx = unsafe { &*papx_ptr };
        let Some(mut para) = IPapxTable::seek(papx, self.mCurCp) else { return };
        while self.mCurCp < cp {
            if self.shouldShowAutonum(para.as_ref()) && para.position() == self.mCurCp {
                let prop = merged_pap(document, para.as_ref());
                if prop.contains(235) {
                    let mut show = true;
                    let para_break = para.endCp() - 1;
                    if is_invisible_para(document, para.as_ref()) {
                        show = false;
                    } else if RangeTool::isRevisionDelete__document_int_int(document, para_break, para_break + 1) {
                        let mut next_prop = MutablePropertySet::new();
                        let next = PapxEntry::getNext(para.as_ref());
                        show = next.as_ref().is_some_and(|entry| self.nextValidParaHasNumbring(entry.as_ref(), &mut next_prop));
                        if show {
                            let frozen = next_prop.freeze();
                            self.record_numbering(&frozen);
                            show = false;
                        }
                    } else if !same_papx(papx, para.as_ref(), IPapxTable::begin(papx).as_deref()) && !self.prevValidRunIsParaEnd(para.as_ref()) {
                        show = false;
                    }
                    if show {
                        self.record_numbering(&prop);
                    }
                }
            }
            match PapxEntry::getNext(para.as_ref()) {
                Some(next) if !next.isHead() => {
                    self.mCurCp = next.position();
                    para = next;
                }
                _ => {
                    let still_loading = document.getTextDocument().is_some_and(|text| text.isInIOState());
                    if !still_loading {
                        self.mCurCp = document.getLength();
                    }
                    break;
                }
            }
        }
    }

    fn record_numbering(&mut self, prop: &PropertySet) {
        let lfo = prop.getInt(235);
        let lvl = prop.getInt__int_int(234, 0);
        let tool_ptr = self.mLstLfoTool;
        if tool_ptr.is_null() {
            return;
        }
        let tool = unsafe { &mut *tool_ptr };
        if !tool.init(lfo, lvl) {
            return;
        }
        let lsid = tool.getLsid();
        let cp = self.mCurCp;
        let record = self.mLsidCounts.entry(lsid).or_insert_with(|| LvlRecord::new(0));
        record.append(lvl, cp, tool);
    }

    // Java: public synchronized AutoNum getAutoNum(int nILfo, int nILvl, int cp)
    pub fn getAutoNum(&mut self, mut n_ilfo: i32, mut n_ilvl: i32, cp: i32) -> Option<AutoNum> {
        let index = self.getAutoNumIndex(cp);
        if index >= 0 {
            return self.mAutoNumCache.get(index as usize).cloned();
        }
        let Some(document) = self.mDocument else { return None };
        let document = unsafe { &mut *document };
        if let Some(papx) = document.getPapxTable() {
            if let Some(para) = IPapxTable::seek(unsafe { &*papx }, cp) {
                let para_break = para.endCp() - 1;
                if RangeTool::isRevisionDelete__document_int_int(document, para_break, para_break + 1) {
                    let mut next_prop = MutablePropertySet::new();
                    if let Some(next) = PapxEntry::getNext(para.as_ref()) {
                        if self.nextValidParaHasNumbring(next.as_ref(), &mut next_prop) {
                            n_ilfo = next_prop.getInt(235);
                            n_ilvl = next_prop.getInt_with_default(234, 0);
                        }
                    }
                }
            }
        }
        let mut counts = self.getCount(n_ilfo, n_ilvl, cp);
        if counts.is_empty() {
            self.mCurCp = self.mCurCp.min(cp);
            self.constructTo(cp);
            counts = self.getCount(n_ilfo, n_ilvl, cp);
            if counts.is_empty() {
                return None;
            }
        }
        let auto_num = AutoNum {
            cp,
            mILfo: n_ilfo,
            mILvl: n_ilvl as i8,
            mCounts: counts,
        };
        let insert_at = (-index - 1).max(0) as usize;
        let insert_at = insert_at.min(self.mAutoNumCache.len());
        self.mAutoNumCache.insert(insert_at, auto_num.clone());
        self.updateAutoNumCache(insert_at as i32);
        Some(auto_num)
    }

    // Java: private void updateAutoNumCache(int index)
    fn updateAutoNumCache(&mut self, index: i32) {
        let Some(start) = self.mAutoNumCache.get(index as usize).cloned() else { return };
        let level = start.mILvl as usize;
        let Some(mut count) = start.mCounts.get(level).copied() else { return };
        let cp = start.cp;
        let lfo = start.mILfo;
        let mut i = index as usize + 1;
        while i < self.mAutoNumCache.len() {
            let item = &mut self.mAutoNumCache[i];
            if cp < item.cp && item.mILfo == lfo && item.mILvl as usize == level {
                count += 1;
                if level < item.mCounts.len() {
                    item.mCounts[level] = count;
                }
            }
            i += 1;
        }
    }

    // Java: public int[] getCount(int nILfo, int nILvl, int cp)
    pub fn getCount(&mut self, n_ilfo: i32, _n_ilvl: i32, cp: i32) -> Vec<i32> {
        if self.mAutoNumTable.is_null() {
            self.init();
        }
        if self.mAutoNumTable.is_null() {
            return Vec::new();
        }
        let lfo = unsafe { (*self.mAutoNumTable).mLfoTable.as_ref() }
            .and_then(|table| table.getDataByIlfo(n_ilfo));
        let Some(lfo) = lfo else { return Vec::new() };
        let Some(record) = self.mLsidCounts.get(&lfo.getLsid()) else { return Vec::new() };
        let n = record.getLvlCount(cp, &mut self.tmpCount);
        if n <= 0 {
            return Vec::new();
        }
        self.tmpCount[..n as usize].to_vec()
    }

    // Java: protected int getAutoNumIndex(int cp)
    fn getAutoNumIndex(&self, cp: i32) -> i32 {
        let mut low = 0i32;
        let mut mid = self.mAutoNumCache.len() as i32;
        let mut high = mid - 1;
        let mut result = -1i32;
        while low <= high {
            mid = ((low as u32).wrapping_add(high as u32) >> 1) as i32;
            result = cp - self.mAutoNumCache[mid as usize].cp;
            if result > 0 {
                low = mid + 1;
            } else if result == 0 {
                return mid;
            } else {
                high = mid - 1;
            }
        }
        -mid - if result < 0 { 1 } else { 2 }
    }

    // Java: private boolean nextValidParaHasNumbring
    fn nextValidParaHasNumbring(&self, mut para: &dyn PapxEntry, next_prop: &mut MutablePropertySet) -> bool {
        let Some(document) = self.document() else { return false };
        loop {
            if para.isEnd() {
                return false;
            }
            if is_invisible_para(document, para) {
                match PapxEntry::getNext(para) {
                    Some(next) => {
                        // The boxed entry must stay alive for the next iteration.
                        return self.next_from_box(next, next_prop);
                    }
                    None => return false,
                }
            }
            let Some(chpx) = document.getChpxTable() else { return false };
            let run = IChpxTable::seek(unsafe { &*chpx }, para.endCp() - 1);
            if run.as_ref().is_some_and(|entry| is_invisible_run(entry.as_ref())) {
                match PapxEntry::getNext(para) {
                    Some(next) => return self.next_from_box(next, next_prop),
                    None => return false,
                }
            }
            let prop = merged_pap(document, para);
            if prop.contains(235) {
                let lfo = prop.getInt(235);
                let lvl = prop.getInt__int_int(234, 0);
                let exists = document.getTextDocument()
                    .and_then(|text| text.getAutoNumTable())
                    .is_some_and(|table| table.level_exists(lfo, lvl));
                if exists {
                    next_prop.add(&prop);
                    return true;
                }
            }
            return false;
        }
    }

    fn next_from_box(&self, para: Box<dyn PapxEntry>, next_prop: &mut MutablePropertySet) -> bool {
        self.nextValidParaHasNumbring(para.as_ref(), next_prop)
    }

    // Java: private boolean prevValidRunIsParaEnd
    fn prevValidRunIsParaEnd(&self, para: &dyn PapxEntry) -> bool {
        let Some(document_ptr) = self.mDocument else { return true };
        let document = unsafe { &*document_ptr };
        let Some(chpx) = document.getChpxTable() else { return true };
        let chpx = unsafe { &*chpx };
        let Some(mut run) = IChpxTable::seek(chpx, para.position() - 1) else { return true };
        while !run.isHead() {
            let prop = merged_chp(document, run.as_ref());
            let run_end = run.endCp();
            if TableTool::isCellEnd__document_int(unsafe { &mut *document_ptr }, run_end - 1) {
                return true;
            }
            if !prop.contains(45) {
                let Some(papx) = document.getPapxTable() else { return true };
                let last = IPapxTable::seek(unsafe { &*papx }, run_end - 1);
                if last.as_ref().is_some_and(|entry| entry.endCp() != run_end) {
                    return false;
                }
                break;
            }
            match ChpxEntry::getPrev(run.as_ref()) {
                Some(prev) => run = prev,
                None => break,
            }
        }
        true
    }

    // Java: protected boolean shouldShowAutonum
    fn shouldShowAutonum(&mut self, paragraph: &dyn PapxEntry) -> bool {
        let Some(document_ptr) = self.mDocument else { return false };
        let document = unsafe { &mut *document_ptr };
        let range = paragraph.range();
        let start = RANGEFactory::getStart(range);
        let end = RANGEFactory::getEnd(range);
        let len = end - start;
        let first = document.charAt(start);
        if first == '\u{000c}' {
            if len == 1 {
                return false;
            }
            let docx = document.getTextDocument().and_then(|text| text.getFileFormat()) == Some(FF_DOCX);
            if docx && len == 2 {
                return false;
            }
        }
        if TableTool::isCellEnd__document_int(document, end - 1) {
            let cell_end = end - 1;
            if let Some(infos) = document.getTableInfos() {
                if let Some(mut table) = unsafe { &*infos }.getTableInfoByRange(cell_end, cell_end) {
                    if let Some(row) = table.getRowByCP(cell_end) {
                        if let Some(cell) = row.getCellByCP(cell_end) {
                            return !cell.isVertMergeCell() || cell.isFirstVertMergeCell();
                        }
                    }
                }
            }
        }
        start != end || start != 0
    }
}

fn same_papx(papx: &dyn IPapxTable, para: &dyn PapxEntry, begin: Option<&dyn PapxEntry>) -> bool {
    let _ = papx;
    begin.is_some_and(|begin| begin.position() == para.position() && begin.isHead() == para.isHead() && begin.endCp() == para.endCp())
}

fn merged_pap(document: &dyn Document, para: &dyn PapxEntry) -> PropertySet {
    let Some(text) = document.getTextDocument() else {
        return PapxEntry::getProperty(para).cloned().unwrap_or_else(PropertySet::empty);
    };
    let cache = std::rc::Rc::new(std::cell::RefCell::new(StyleCache::StyleCache(text)));
    let calc = PropertyCalculateImpl::PropertyCalculateImpl(document, cache);
    IPropertyCalculate::getPap__entry_int(&calc, para, 3)
}

fn merged_chp(document: &dyn Document, run: &dyn ChpxEntry) -> PropertySet {
    let Some(text) = document.getTextDocument() else {
        return ChpxEntry::getProperty(run).cloned().unwrap_or_else(PropertySet::empty);
    };
    let cache = std::rc::Rc::new(std::cell::RefCell::new(StyleCache::StyleCache(text)));
    let calc = PropertyCalculateImpl::PropertyCalculateImpl(document, cache);
    IPropertyCalculate::getChp__entry_int(&calc, run, 3)
}

// Java: KRange.is_invisible_run
fn is_invisible_run(entry: &dyn ChpxEntry) -> bool {
    let Some(prop) = ChpxEntry::getProperty(entry) else { return false };
    let revision = prop.getValue(45).and_then(|value| value.as_ref().as_any().downcast_ref::<Revision>());
    if revision.is_some() {
        return true;
    }
    PropertyUtil::getFinalRunFormat(prop).getBoolean__int_boolean(18, false)
}

fn is_invisible_para(document: &dyn Document, para: &dyn PapxEntry) -> bool {
    let range = para.range();
    let start = RANGEFactory::getStart(range);
    let end = RANGEFactory::getEnd(range);
    let Some(chpx) = document.getChpxTable() else { return true };
    let chpx = unsafe { &*chpx };
    let Some(mut run) = IChpxTable::seek(chpx, start) else { return true };
    while !run.isHead() {
        if run.position() >= end {
            break;
        }
        if !is_invisible_run(run.as_ref()) {
            return false;
        }
        match ChpxEntry::getNext(run.as_ref()) {
            Some(next) => run = next,
            None => break,
        }
    }
    true
}
