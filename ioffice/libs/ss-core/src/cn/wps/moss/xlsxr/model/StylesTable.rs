use crate::cn::wps::moss::filefmt::biff8::record::{
    cf::{BorderFormatting::BorderFormatting, PatternFormatting::PatternFormatting},
    ExtendedFormatRecord::ExtendedFormatRecord,
    FontRecord::FontRecord,
};
use crate::java_runtime::{Cell, Rc, RefCell};
use std::collections::HashMap;

use crate::cn::wps::moss::app::xf::KmoXFManager::KmoXFManager;
use crate::cn::wps::moss::xlsxr::xl::XlsxrStyles::XlsxrStyles;

// java package cn.wps.moss.xlsxr.model;
// java import cn.wps.moss.app.xf.KmoXFManager;
// java import cn.wps.moss.filefmt.biff8.record.ExtendedFormatRecord;
// java import cn.wps.moss.filefmt.biff8.record.FontRecord;
// java import cn.wps.moss.filefmt.biff8.record.cf.BorderFormatting;
// java import cn.wps.moss.filefmt.biff8.record.cf.PatternFormatting;
// java import cn.wps.moss.xlsxr.xl.XlsxrStyles;
// java import java.util.HashMap;
// java import java.util.Map;
// java import java.util.Vector;

// java public class StylesTable {
pub struct StylesTable {
    dxfList: RefCell<Vec<Rc<super::DxfPropertyId::DxfPropertyId>>>,
    numFmts: RefCell<HashMap<i16, i16>>,
    mStartCoreXF: Cell<i32>,
    mEndCoreXF: Cell<i32>,
    styleXfList: RefCell<Vec<crate::java_runtime::Rc<RefCell<ExtendedFormatRecord>>>>,
    fontList: RefCell<Vec<i32>>,
    fontLis: RefCell<Vec<Rc<RefCell<FontRecord>>>>,
    patternFillList: RefCell<Vec<Option<Rc<RefCell<PatternFormatting>>>>>,
    borderList: RefCell<Vec<Rc<RefCell<BorderFormatting>>>>,
    styleIdList: RefCell<Vec<i32>>,
}

impl StylesTable {
    pub fn StylesTable() -> Self {
        Self {
            dxfList: RefCell::new(Vec::new()),
            numFmts: RefCell::new(HashMap::new()),
            fontList: RefCell::new(Vec::new()),
            fontLis: RefCell::new(Vec::new()),
            patternFillList: RefCell::new(Vec::new()),
            borderList: RefCell::new(Vec::new()),
            mStartCoreXF: Cell::new(0),
            mEndCoreXF: Cell::new(0),
            styleXfList: RefCell::new(Vec::new()),
            styleIdList: RefCell::new(Vec::new()),
        }
    }

    // java public void addnumFmt(short numFmtid, short formatCode) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:25
    // java public void addnumFmt(short numFmtid, short formatCode) {
    // java this.numFmts.put(numFmtid, formatCode);
    // java }
    pub fn addnumFmt(&self, numFmtid: i16, formatCode: i16) {
        // java this.numFmts.put(numFmtid, formatCode);
        self.numFmts.borrow_mut().insert(numFmtid, formatCode);
    }

    // java public Short getformatCode(short numFmtId) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:29
    // java public Short getformatCode(short numFmtId) {
    // java return this.numFmts.containsKey(numFmtId) ? (Short)this.numFmts.get(numFmtId) : numFmtId;
    // java }
    pub fn getformatCode(&self, numFmtId: i16) -> i16 {
        // java return this.numFmts.containsKey(numFmtId) ? (Short)this.numFmts.get(numFmtId) : numFmtId;
        self.numFmts
            .borrow()
            .get(&numFmtId)
            .copied()
            .unwrap_or(numFmtId)
    }

    // java public void initStartCoreXF(KmoXFManager xfs) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:33
    // java public void initStartCoreXF(KmoXFManager xfs) {
    // java this.mStartCoreXF = xfs.getNumExFormats();
    // java }
    pub fn initStartCoreXF(&self, xfs: &KmoXFManager) {
        // java this.mStartCoreXF = xfs.getNumExFormats();
        self.mStartCoreXF.set(xfs.getNumExFormats());
    }

    // java public int getStartCoreXF() {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:37
    // java public int getStartCoreXF() {
    // java return this.mStartCoreXF;
    // java }
    pub fn getStartCoreXF(&self) -> i32 {
        // java return this.mStartCoreXF;
        self.mStartCoreXF.get()
    }

    // java public void initEndCoreXF(KmoXFManager xfs) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:41
    // java public void initEndCoreXF(KmoXFManager xfs) {
    // java this.mEndCoreXF = xfs.getNumExFormats();
    // java }
    pub fn initEndCoreXF(&self, xfs: &KmoXFManager) {
        // java this.mEndCoreXF = xfs.getNumExFormats();
        self.mEndCoreXF.set(xfs.getNumExFormats());
    }

    // java public int convertDocumentXF2CoreXF(int ixf) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:45
    // java public int convertDocumentXF2CoreXF(int ixf) {
    // java if (ixf == 0) {
    // java XlsxrStyles.onXFIndex(this, ixf);
    // java return 15;
    // java } else if (ixf > 0 && ixf + this.mStartCoreXF - 1 < this.mEndCoreXF) {
    // java XlsxrStyles.onXFIndex(this, ixf);
    // java return ixf + this.mStartCoreXF - 1;
    // java } else {
    // java return 15;
    // java }
    // java }
    pub fn convertDocumentXF2CoreXF(&self, ixf: i32) -> i32 {
        // java if (ixf == 0) {
        // java XlsxrStyles.onXFIndex(this, ixf);
        // java return 15;
        // java } else if (ixf > 0 && ixf + this.mStartCoreXF - 1 < this.mEndCoreXF) {
        // java XlsxrStyles.onXFIndex(this, ixf);
        // java return ixf + this.mStartCoreXF - 1;
        // java return 15;
        if ixf == 0 {
            XlsxrStyles::onXFIndex(self, ixf);
            return 15;
        } else if ixf > 0 && ixf + self.mStartCoreXF.get() - 1 < self.mEndCoreXF.get() {
            XlsxrStyles::onXFIndex(self, ixf);
            return ixf + self.mStartCoreXF.get() - 1;
        } else {
            return 15;
        }
    }

    // java public void addFont(int fontId) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:57
    // java public void addFont(int fontId) {
    // java this.fontList.add(fontId);
    // java }
    pub fn addFont(&self, fontId: i32) {
        self.fontList.borrow_mut().push(fontId);
    }

    // java public int getFontAt(int index) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:61
    // java public int getFontAt(int index) {
    // java if (index >= 0 && index < this.fontList.size()) {
    // java return (Integer)this.fontList.get(index);
    // java } else {
    // java System.out.println("IO Error: OutOfIndexed FontId " + Integer.toString(index));
    // java return 0;
    // java }
    // java }
    pub fn getFontAt(&self, index: i32) -> i32 {
        self.fontList
            .borrow()
            .get(index as usize)
            .copied()
            .unwrap_or_else(|| {
                println!("IO Error: OutOfIndexed FontId {index}");
                0
            })
    }

    // java public void addPatternFill(PatternFormatting pattern) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:70
    // java public void addPatternFill(PatternFormatting pattern) {
    // java this.patternFillList.add(pattern);
    // java }
    pub fn addPatternFill(&self, pattern: Option<Rc<RefCell<PatternFormatting>>>) {
        self.patternFillList.borrow_mut().push(pattern);
    }

    // java public PatternFormatting getPatternFillAt(int index) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:74
    // java public PatternFormatting getPatternFillAt(int index) {
    // java return index >= 0 && index < this.patternFillList.size() ? (PatternFormatting)this.patternFillList.get(index) : null;
    // java }
    pub fn getPatternFillAt(&self, index: i32) -> Option<Rc<RefCell<PatternFormatting>>> {
        self.patternFillList
            .borrow()
            .get(index as usize)
            .cloned()
            .flatten()
    }

    // java public void addBorder(BorderFormatting border) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:78
    // java public void addBorder(BorderFormatting border) {
    // java this.borderList.add(border);
    // java }
    pub fn addBorder(&self, border: Rc<RefCell<BorderFormatting>>) {
        self.borderList.borrow_mut().push(border);
    }

    // java public BorderFormatting getBorderAt(int index) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:82
    // java public BorderFormatting getBorderAt(int index) {
    // java return index >= 0 && index < this.borderList.size() ? (BorderFormatting)this.borderList.get(index) : null;
    // java }
    pub fn getBorderAt(&self, index: i32) -> Option<Rc<RefCell<BorderFormatting>>> {
        self.borderList.borrow().get(index as usize).cloned()
    }

    // java public void addStyleXf(ExtendedFormatRecord style) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:86
    // java public void addStyleXf(ExtendedFormatRecord style) {
    // java this.styleXfList.add(style);
    // java }
    pub fn addStyleXf(&self, style: crate::java_runtime::Rc<RefCell<ExtendedFormatRecord>>) {
        self.styleXfList.borrow_mut().push(style);
    }

    // java public ExtendedFormatRecord getStyleXfAt(int index) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:90
    // java public ExtendedFormatRecord getStyleXfAt(int index) {
    // java return index >= 0 && index < this.styleXfList.size() ? (ExtendedFormatRecord)this.styleXfList.get(index) : null;
    // java }
    pub fn getStyleXfAt(
        &self,
        index: i32,
    ) -> Option<crate::java_runtime::Rc<RefCell<ExtendedFormatRecord>>> {
        self.styleXfList.borrow().get(index as usize).cloned()
    }

    // java public int getStyleXfSize() {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:94
    // java public int getStyleXfSize() {
    // java return this.styleXfList.size();
    // java }
    pub fn getStyleXfSize(&self) -> i32 {
        // java return this.styleXfList.size();
        self.styleXfList.borrow().len() as i32
    }

    // java public void addStyleId(int styleId) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:98
    // java public void addStyleId(int styleId) {
    // java this.styleIdList.add(styleId);
    // java }
    pub fn addStyleId(&self, styleId: i32) {
        // java this.styleIdList.add(styleId);
        self.styleIdList.borrow_mut().push(styleId);
    }

    // java public int getStyleId(int styleId) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:102
    // java public int getStyleId(int styleId) {
    // java return styleId < this.styleIdList.size() ? (Integer)this.styleIdList.get(styleId) : 0;
    // java }
    pub fn getStyleId(&self, styleId: i32) -> i32 {
        // java return styleId < this.styleIdList.size() ? (Integer)this.styleIdList.get(styleId) : 0;
        let list = self.styleIdList.borrow();
        if styleId >= 0 && (styleId as usize) < list.len() {
            list[styleId as usize]
        } else {
            0
        }
    }

    // java public void addFontRecord(FontRecord font) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:106
    // java public void addFontRecord(FontRecord font) {
    // java this.fontLis.add(font);
    // java }
    pub fn addFontRecord(&self, font: Rc<RefCell<FontRecord>>) {
        self.fontLis.borrow_mut().push(font);
    }

    // java public FontRecord getFontRecordAt(int index) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:110
    // java public FontRecord getFontRecordAt(int index) {
    // java return (FontRecord)this.fontLis.get(index);
    // java }
    pub fn getFontRecordAt(&self, index: i32) -> Rc<RefCell<FontRecord>> {
        self.fontLis.borrow()[index as usize].clone()
    }

    // java public int getborderListsize() {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:114
    // java public int getborderListsize() {
    // java return this.borderList.size();
    // java }
    pub fn getborderListsize(&self) -> i32 {
        self.borderList.borrow().len() as i32
    }

    // java public int getpatternFillListsize() {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:118
    // java public int getpatternFillListsize() {
    // java return this.patternFillList.size();
    // java }
    pub fn getpatternFillListsize(&self) -> i32 {
        self.patternFillList.borrow().len() as i32
    }

    // java public int getFontListsize() {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/StylesTable.java:122
    // java public int getFontListsize() {
    // java return this.fontLis.size();
    // java }
    pub fn getFontListsize(&self) -> i32 {
        self.fontLis.borrow().len() as i32
    }

    // java public void adddxf(DxfPropertyId dxf) {
    pub fn adddxf(&self, dxf: Rc<super::DxfPropertyId::DxfPropertyId>) {
        self.dxfList.borrow_mut().push(dxf);
    }

    // java public void removedxf(DxfPropertyId dxf) {
    pub fn removedxf(&self, dxf: &Rc<super::DxfPropertyId::DxfPropertyId>) {
        let mut list = self.dxfList.borrow_mut();
        if let Some(i) = list.iter().position(|item| Rc::ptr_eq(item, dxf)) {
            list.remove(i);
        }
    }

    // java public DxfPropertyId getdxf(int i) {
    pub fn getdxf(&self, index: i32) -> Option<Rc<super::DxfPropertyId::DxfPropertyId>> {
        let list = self.dxfList.borrow();
        if index < list.len() as i32 {
            Some(list[index as usize].clone())
        } else {
            None
        }
    }

    // java public int getDxfListSize() {
    pub fn getDxfListSize(&self) -> i32 {
        self.dxfList.borrow().len() as i32
    }
}
