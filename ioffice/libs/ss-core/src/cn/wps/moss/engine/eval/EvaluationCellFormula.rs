// java package cn.wps.moss.engine.eval;

use crate::java_runtime::{Cell, RefCell};
use crate::java_runtime::Rc;
use std::any::Any;

// java import cn.wps.et.ss.formula.ptg.ExpPtg;
use crate::cn::wps::et::ss::formula::ptg::ExpPtg::ExpPtg;
// java import cn.wps.et.ss.formula.ptg.Ptg;
use crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg;
// java import cn.wps.et.ss.calcchain.FormulaHelper;
use crate::cn::wps::et::ss::calcchain::FormulaHelper::FormulaHelper;
// java import cn.wps.et.ss.interfaces.IEvalFormulaBase;
use crate::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;
// java import cn.wps.et.ss.interfaces.IEvalCellFormula;
use crate::cn::wps::et::ss::interfaces::IEvalCellFormula::IEvalCellFormula;
// java import cn.wps.et.ss.interfaces.IEvalSharedFormula;
use crate::cn::wps::et::ss::interfaces::IEvalSharedFormula::IEvalSharedFormula;
// java import cn.wps.et.ss.formula.Formula;
use crate::cn::wps::et::ss::formula::Formula::Formula;
// java import cn.wps.moss.app.KmoSheet;
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
// java import cn.wps.moss.engine.grid.KmoCellData;
use crate::cn::wps::moss::engine::grid::KmoCellData::KmoCellData;
// java import cn.wps.moss.engine.grid.KmoFormulaData;
use crate::cn::wps::moss::engine::grid::KmoFormulaData::KmoFormulaData;

use super::EvaluationPresent::EvaluationPresent;
use super::EvaluationSharedFormula::EvaluationSharedFormula;

// java public final class EvaluationCellFormula implements IEvalCellFormula
pub struct EvaluationCellFormula {
    // java private final KmoSheet ksheet;
    ksheet: Rc<RefCell<KmoSheet>>,
    // java private final int rowIndex;
    rowIndex: i32,
    // java private final int colIndex;
    colIndex: i32,
    // java private boolean isSharedFormula = false;
    isSharedFormula: bool,
    // java private byte[] encodedTokens = null;
    encodedTokens: Option<Vec<u8>>,
    // java private int encodedTokenLen = 0;
    encodedTokenLen: i32,
    // java private IEvalFormulaBase refFml;
    refFml: RefCell<Option<Rc<dyn IEvalFormulaBase>>>,
    // java public EvaluationPresent evaluationPresent;
    pub evaluationPresent: EvaluationPresent,
    // java private boolean isCycled;
    isCycled: Cell<bool>,
}

impl EvaluationCellFormula {
    // java public EvaluationCellFormula(KmoSheet sheet, int row, int col)
    pub fn EvaluationCellFormula(sheet: Rc<RefCell<KmoSheet>>, row: i32, col: i32) -> Self {
        let sheet_ref = Rc::clone(&sheet);
        // java this.ksheet = sheet; this.rowIndex = row; this.colIndex = col;
        // java this.evaluationPresent = new EvaluationPresent();
        let mut result = Self {
            ksheet: sheet,
            rowIndex: row,
            colIndex: col,
            isSharedFormula: false,
            encodedTokens: None,
            encodedTokenLen: 0,
            refFml: RefCell::new(None),
            evaluationPresent: EvaluationPresent::new(),
            isCycled: Cell::new(false),
        };
        // java KmoCellData cellBuf = SharedObjects.kmoCellDataPool.borrowObject();
        let mut cellBuf = KmoCellData::default();
        // java KmoFormulaData fmlbuf = SharedObjects.kmoFormulaDataPool.borrowObject();
        let mut fmlbuf = KmoFormulaData::KmoFormulaData();
        // java this.ksheet.getCellData(this.rowIndex, this.colIndex, cellBuf);
        let sheet = sheet_ref.borrow();
        sheet.getCellData__2(result.rowIndex, result.colIndex, &mut cellBuf);
        // java if (cellBuf.formulaType != 0 && cellBuf.formulaType != 2)
        if cellBuf.formulaType != 0 && cellBuf.formulaType != 2 {
            // java KmoCellValueStore cvs = this.ksheet.getCellValueStore();
            let cvs = sheet.getCellValueStore();
            // java cvs.formulaValue(cellBuf.id, fmlbuf);
            cvs.borrow().formulaValue(cellBuf.id, &mut fmlbuf);
            // java if (cellBuf.formulaType == 1)
            if cellBuf.formulaType == 1 {
                // java this.isSharedFormula = false;
                result.isSharedFormula = false;
                // java this.encodedTokens = cvs.formulaTokens(fmlbuf.getTokensId());
                result.encodedTokens = Some(cvs.borrow().formulaTokens(fmlbuf.getTokensId()));
                // java this.encodedTokenLen = fmlbuf.getTokensPlainSize();
                result.encodedTokenLen = fmlbuf.getTokensPlainSize();
                return result;
            }
            // java if (cellBuf.formulaType != 3) return;
            if cellBuf.formulaType != 3 {
                return result;
            }
            // java byte[] data = cvs.formulaTokens(fmlbuf.getTokensId());
            let data = cvs.borrow().formulaTokens(fmlbuf.getTokensId());
            // java if (data.length == ExpPtg.getSize(sheet.getVersion()) && data[0] == 1)
            if data.len() == ExpPtg::getSize(sheet.getVersion()) as usize && data[0] == 1 {
                // java int[] masterRowCol = ExpPtg.getRowCol(sheet.getVersion(), data);
                let masterRowCol = ExpPtg::getRowCol(sheet.getVersion(), &data);
                // java ExpFmlManager.ExpFml shrfml = this.ksheet.getExpFmlManager().query(masterRowCol[0], masterRowCol[1], true);
                let shrfml = sheet.getExpFmlManager().borrow().query(masterRowCol[0], masterRowCol[1], true);
                // java if (shrfml != null)
                if let Some(shrfml) = shrfml {
                    // java this.isSharedFormula = true;
                    result.isSharedFormula = true;
                    // java this.encodedTokens = shrfml.getEncodedTokens();
                    result.encodedTokens = Some(shrfml.getEncodedTokens());
                    // java this.encodedTokenLen = shrfml.getEncodedTokensLen();
                    result.encodedTokenLen = shrfml.getEncodedTokensLen();
                    return result;
                }
                // java return;
                return result;
            }
            // java return;
            return result;
        }
        // java SharedObjects.kmoCellDataPool.returnObject(cellBuf);
        // java SharedObjects.kmoFormulaDataPool.returnObject(fmlbuf);
        result
    }

    // java public int getFmlCol()
    pub fn getFmlCol(&self) -> i32 {
        // java return this.colIndex;
        self.colIndex
    }

    // java public int getFmlRow()
    pub fn getFmlRow(&self) -> i32 {
        // java return this.rowIndex;
        self.rowIndex
    }

    // java public int getSheetUid()
    pub fn getSheetUid(&self) -> i32 {
        // java return this.ksheet.getSheetUid();
        self.ksheet.borrow().getSheetUid()
    }

    // java public int getSheetIndex()
    pub fn getSheetIndex(&self) -> i32 {
        // java return this.ksheet.getSheetIndex();
        self.ksheet.borrow().getSheetIndex()
    }

    // java public static void loadBuffer(KmoSheet sheet, int row, int col, KmoCellData cellBuf, KmoFormulaData fmlbuf)
    pub fn loadBuffer(sheet: &KmoSheet, row: i32, col: i32, cellBuf: &mut KmoCellData, fmlbuf: &mut KmoFormulaData) {
        // java sheet.getCellData(row, col, cellBuf);
        sheet.getCellData__2(row, col, cellBuf);
        // java if (cellBuf.formulaType == 0) throw new IllegalStateException("Formula expacted");
        if cellBuf.formulaType == 0 { panic!("Formula expacted"); }
        // java sheet.getCellValueStore().formulaValue(cellBuf.id, fmlbuf);
        sheet.getCellValueStore().borrow().formulaValue(cellBuf.id, fmlbuf);
    }

    // java private static void storeBuffer(KmoSheet sheet, int row, int col, KmoCellData cellBuf, KmoFormulaData fmlbuf)
    fn storeBuffer(sheet: &KmoSheet, row: i32, col: i32, cellBuf: &mut KmoCellData, fmlbuf: &KmoFormulaData) {
        // java cellBuf.id = sheet.getCellValueStore().newFormulaValue(fmlbuf);
        cellBuf.id = sheet.getCellValueStore().borrow_mut().newFormulaValue(fmlbuf);
        // java sheet.setCellData(row, col, cellBuf);
        sheet.setCellData(row, col, cellBuf);
    }

    // java public byte[] getEncodedTokens()
    pub fn getEncodedTokens(&self) -> Option<&[u8]> {
        // java return this.encodedTokens;
        self.encodedTokens.as_deref()
    }

    // java public Ptg[] getTokens()
    pub fn getTokens(&self) -> Vec<Ptg> {
        // java if (this.encodedTokens == null) return Ptg.EMPTY_PTG_ARRAY;
        let Some(encoded) = &self.encodedTokens else { return Vec::new(); };
        let sheet = self.ksheet.borrow();
        // java Ptg[] ret = Ptg.readTokens(this.encodedTokenLen, this.encodedTokens, this.ksheet.getVersion());
        let ret = Ptg::readTokens(self.encodedTokenLen, encoded, sheet.getVersion());
        // java return this.isSharedFormula ? FormulaHelper.replaceNPtgs(ret, this.rowIndex, this.colIndex, this.ksheet.getBook().getSpreadsheetVersion()) : ret;
        if self.isSharedFormula {
            FormulaHelper::replaceNPtgs(&ret, self.rowIndex, self.colIndex, sheet.getBook().getSpreadsheetVersion())
        } else { ret }
    }

    // java public int getEncodedTokensLen()
    pub fn getEncodedTokensLen(&self) -> i32 {
        // java return this.encodedTokenLen;
        self.encodedTokenLen
    }

    // java public boolean isSharedFormula()
    pub fn isSharedFormula(&self) -> bool {
        // java return this.isSharedFormula;
        self.isSharedFormula
    }

    // java public boolean isMasterSharedFormula()
    pub fn isMasterSharedFormula(&self) -> bool {
        // java KmoCellData cellData = new KmoCellData();
        let mut cellData = KmoCellData::default();
        let sheet = self.ksheet.borrow();
        // java this.ksheet.getCellData(this.rowIndex, this.colIndex, cellData);
        sheet.getCellData__2(self.rowIndex, self.colIndex, &mut cellData);
        // java if (cellData.formulaType != 3) return false;
        if cellData.formulaType != 3 { return false; }
        // java KmoCellValueStore cvs = this.ksheet.getCellValueStore();
        let cvs = sheet.getCellValueStore();
        // java KmoFormulaData fmlData = new KmoFormulaData();
        let mut fmlData = KmoFormulaData::KmoFormulaData();
        // java cvs.formulaValue(cellData.id, fmlData);
        cvs.borrow().formulaValue(cellData.id, &mut fmlData);
        // java Formula fml = new Formula(cvs.formulaTokens(fmlData.getTokensId()), fmlData.getTokensPlainSize(), this.ksheet.getVersion());
        let fml = Formula::Formula(Some(Rc::new(RefCell::new(cvs.borrow().formulaTokens(fmlData.getTokensId()).into_boxed_slice()))),fmlData.getTokensPlainSize(),Some(sheet.getVersion()));
        // java CellReference cr = fml.getExpReference();
        let cr = fml.getExpReference();
        // java if (cr == null) return false;
        let Some(cr) = cr else { return false; };
        // java return cr.getRow() == this.rowIndex && cr.getCol() == this.colIndex;
        cr.getRow() == self.rowIndex && cr.getCol() as i32 == self.colIndex
    }

    // java public EvaluationSharedFormula getSharedFormula()
    pub fn getSharedFormula(&self) -> Option<EvaluationSharedFormula> {
        // java KmoCellData cellData = new KmoCellData();
        let mut cellData = KmoCellData::default();
        let sheet = self.ksheet.borrow();
        // java this.ksheet.getCellData(this.rowIndex, this.colIndex, cellData);
        sheet.getCellData__2(self.rowIndex, self.colIndex, &mut cellData);
        // java if (cellData.formulaType != 3) return null;
        if cellData.formulaType != 3 { return None; }
        // java KmoCellValueStore cvs = this.ksheet.getCellValueStore();
        let cvs = sheet.getCellValueStore();
        // java KmoFormulaData fmlData = new KmoFormulaData();
        let mut fmlData = KmoFormulaData::KmoFormulaData();
        // java cvs.formulaValue(cellData.id, fmlData);
        cvs.borrow().formulaValue(cellData.id, &mut fmlData);
        // java Formula fml = new Formula(cvs.formulaTokens(fmlData.getTokensId()), fmlData.getTokensPlainSize(), this.ksheet.getVersion());
        let fml = Formula::Formula(Some(Rc::new(RefCell::new(cvs.borrow().formulaTokens(fmlData.getTokensId()).into_boxed_slice()))),fmlData.getTokensPlainSize(),Some(sheet.getVersion()));
        // java CellReference cr = fml.getExpReference();
        let cr = fml.getExpReference()?;
        // java ExpFmlManager.ExpFml shrfml = this.ksheet.getExpFmlManager().query(cr.getRow(), cr.getCol(), true);
        let shrfml = sheet.getExpFmlManager().borrow().query(cr.getRow(), cr.getCol() as i32, true)?;
        // java return shrfml == null ? null : new EvaluationSharedFormula(this.ksheet, shrfml.firstRow, shrfml.lastRow, shrfml.firstCol, shrfml.lastCol);
        Some(EvaluationSharedFormula::EvaluationSharedFormula(Rc::clone(&self.ksheet), shrfml.firstRow, shrfml.lastRow, shrfml.firstCol, shrfml.lastCol))
    }

    // java public boolean isNeedRecalculationFlagSet()
    // java public EvaluationCellFormula updateTokens(Ptg[] ptgs, boolean unlinkSharedFml) {
    pub fn updateTokens(&self,ptgs:&[Ptg],unlinkSharedFml:bool)->Self{
        // java KmoCellData cell = SharedObjects.kmoCellDataPool.borrowObject();
        let mut cell=KmoCellData::default();
        // java if (unlinkSharedFml) { this.ksheet.getCellData(this.rowIndex, this.colIndex, cell); if (cell.formulaType == 3) { this.ksheet.getExpFmlManager().unlinkSharedFormulaByCellPos(this.ksheet, this.rowIndex, this.colIndex); } }
        if unlinkSharedFml{let sheet=self.ksheet.borrow();sheet.getCellData__2(self.rowIndex,self.colIndex,&mut cell);if cell.formulaType==3{sheet.getExpFmlManager().borrow().unlinkSharedFormulaByCellPos(&self.ksheet,self.rowIndex,self.colIndex);}}
        // java int encodedLen = Ptg.getEncodedSizeWithoutArrayData(ptgs); byte[] array = new byte[Ptg.getEncodedSize(ptgs)]; Ptg.serializePtgs(ptgs, array, 0);
        let encodedLen=crate::cn::wps::et::ss::formula::ptg::Ptg::getEncodedSizeWithoutArrayData(ptgs);let array=crate::cn::wps::et::ss::formula::ptg::Ptg::serializePtgs(ptgs);
        // java this.ksheet.getCellData(this.rowIndex, this.colIndex, cell); KmoCellValueStore cvs = this.ksheet.getCellValueStore();
        let sheet=self.ksheet.borrow();sheet.getCellData__2(self.rowIndex,self.colIndex,&mut cell);let cvs=sheet.getCellValueStore();
        // java KmoFormulaData fml = SharedObjects.kmoFormulaDataPool.borrowObject(); cvs.formulaValue(cell.id, fml);
        let mut fml=KmoFormulaData::KmoFormulaData();cvs.borrow().formulaValue(cell.id,&mut fml);
        // java fml.setTokens(cvs.newFormulaTokens(array), encodedLen); fml.setCalcFlags((byte)0);
        fml.setTokens(cvs.borrow_mut().newFormulaTokens(Some(&array)),encodedLen);fml.setCalcFlags(0);
        // java cell.formulaType = 1; cell.id = cvs.newFormulaValue(fml); this.ksheet.setCellData(this.rowIndex, this.colIndex, cell);
        cell.formulaType=1;cell.id=cvs.borrow_mut().newFormulaValue(&fml);sheet.setCellData(self.rowIndex,self.colIndex,&cell);
        // java cell = SharedObjects.kmoCellDataPool.returnObject(cell); fml = SharedObjects.kmoFormulaDataPool.returnObject(fml);
        drop(sheet);
        // java return new EvaluationCellFormula(this.ksheet, this.rowIndex, this.colIndex);
        Self::EvaluationCellFormula(self.ksheet.clone(),self.rowIndex,self.colIndex)
    }

    // java public boolean isNeedRecalculationFlagSet()
    pub fn isNeedRecalculationFlagSet(&self) -> bool {
        // java return EvaluationPresent.isNeedRecalculationFlagSet(this.ksheet, this.rowIndex, this.colIndex);
        EvaluationPresent::isNeedRecalculationFlagSet(&self.ksheet.borrow(), self.rowIndex, self.colIndex)
    }

    // java public void setNeedRecalculationFlag(boolean f)
    pub fn setNeedRecalculationFlag(&self, f: bool) {
        // java EvaluationPresent.setNeedRecalculationFlag(this.ksheet, this.rowIndex, this.colIndex, f);
        EvaluationPresent::setNeedRecalculationFlag(&self.ksheet.borrow(), self.rowIndex, self.colIndex, f);
    }

    // java public void setNumberResult(double v)
    pub fn setNumberResult(&self, v: f64) {
        // java EvaluationPresent.setNumberResult(this.ksheet, this.rowIndex, this.colIndex, v);
        EvaluationPresent::setNumberResult(&self.ksheet.borrow(), self.rowIndex, self.colIndex, v);
    }

    // java public void setStringResult(String s)
    pub fn setStringResult(&self, s: &str) {
        // java EvaluationPresent.setStringResult(this.ksheet, this.rowIndex, this.colIndex, s);
        EvaluationPresent::setStringResult(&self.ksheet.borrow(), self.rowIndex, self.colIndex, s);
    }

    // java public void setBoolResult(boolean b)
    pub fn setBoolResult(&self, b: bool) {
        // java EvaluationPresent.setBoolResult(this.ksheet, this.rowIndex, this.colIndex, b);
        EvaluationPresent::setBoolResult(&self.ksheet.borrow(), self.rowIndex, self.colIndex, b);
    }

    // java public void setErrorResult(byte e)
    pub fn setErrorResult(&self, e: i8) {
        // java EvaluationPresent.setErrorResult(this.ksheet, this.rowIndex, this.colIndex, e);
        EvaluationPresent::setErrorResult(&self.ksheet.borrow(), self.rowIndex, self.colIndex, e);
    }

    // java public void setZeroResultIfBlank()
    pub fn setZeroResultIfBlank(&self) {
        // java EvaluationPresent.setZeroResultIfBlank(this.ksheet, this.rowIndex, this.colIndex);
        EvaluationPresent::setZeroResultIfBlank(&self.ksheet.borrow(), self.rowIndex, self.colIndex);
    }

    // java public boolean isCycled()
    pub fn isCycled(&self) -> bool {
        // java return this.isCycled;
        self.isCycled.get()
    }

    // java public void setCycled(boolean iscycled)
    pub fn setCycled(&self, iscycled: bool) {
        // java this.isCycled = iscycled;
        self.isCycled.set(iscycled);
    }

    // java public boolean equals(Object o)
    pub fn equals(&self, other: &dyn IEvalFormulaBase) -> bool {
        // java if (!(o instanceof EvaluationCellFormula)) return false;
        let Some(other) = other.as_any().downcast_ref::<EvaluationCellFormula>() else { return false; };
        // java return fml.ksheet == this.ksheet && fml.rowIndex == this.rowIndex && fml.colIndex == this.colIndex;
        Rc::ptr_eq(&self.ksheet, &other.ksheet) && self.rowIndex == other.rowIndex && self.colIndex == other.colIndex
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
        // java int uid = this.ksheet.getSheetUid(); return uid << 8 ^ this.rowIndex << 16 ^ this.colIndex;
        (self.getSheetUid() << 8) ^ (self.rowIndex << 16) ^ self.colIndex
    }

    // java public void lock()
    pub fn lock(&self) { self.evaluationPresent.lock(); }
    // java public boolean islock()
    pub fn islock(&self) -> bool { self.evaluationPresent.isLock() }
    // java public void unlock()
    pub fn unlock(&self) { self.evaluationPresent.unlock(); }

    // java public void setRef(IEvalFormulaBase fml)
    pub fn setRef(&self, fml: Option<Rc<dyn IEvalFormulaBase>>) {
        // java this.refFml = fml;
        *self.refFml.borrow_mut() = fml;
    }

    // java public IEvalFormulaBase getRef()
    pub fn getRef(&self) -> Option<Rc<dyn IEvalFormulaBase>> {
        // java return this.refFml;
        self.refFml.borrow().clone()
    }
}

// java public final class EvaluationCellFormula implements IEvalCellFormula
impl IEvalFormulaBase for EvaluationCellFormula {
    fn getSheetUid(&self) -> i32 { self.getSheetUid() }
    fn getSheetIndex(&self) -> i32 { self.getSheetIndex() }
    fn getFmlRow(&self) -> i32 { self.getFmlRow() }
    fn getFmlCol(&self) -> i32 { self.getFmlCol() }
    fn getTokens(&self) -> Vec<Ptg> { self.getTokens() }
    fn getEncodedTokens(&self) -> Option<&[u8]> { self.getEncodedTokens() }
    fn getEncodedTokensLen(&self) -> i32 { self.getEncodedTokensLen() }
    fn setNeedRecalculationFlag(&self, value: bool) { self.setNeedRecalculationFlag(value); }
    fn isNeedRecalculationFlagSet(&self) -> bool { self.isNeedRecalculationFlagSet() }
    fn setZeroResultIfBlank(&self) { self.setZeroResultIfBlank(); }
    fn equals(&self, other: &dyn IEvalFormulaBase) -> bool { self.equals(other) }
    fn hashCode(&self) -> i32 { self.hashCode() }
    fn lock(&self) { self.lock(); }
    fn islock(&self) -> bool { self.islock() }
    fn unlock(&self) { self.unlock(); }
    fn isCycled(&self) -> bool { self.isCycled() }
    fn setCycled(&self, value: bool) { self.setCycled(value); }
    fn setRef(&self, value: Option<Rc<dyn IEvalFormulaBase>>) { self.setRef(value); }
    fn getRef(&self) -> Option<Rc<dyn IEvalFormulaBase>> { self.getRef() }
    fn as_cell_formula(&self) -> Option<&dyn IEvalCellFormula> { Some(self) }
    fn as_any(&self) -> &dyn Any { self }
}

// java public final class EvaluationCellFormula implements IEvalCellFormula
impl IEvalCellFormula for EvaluationCellFormula {
    fn setStringResult(&self, value: &str) { self.setStringResult(value); }
    fn setNumberResult(&self, value: f64) { self.setNumberResult(value); }
    fn setErrorResult(&self, value: i8) { self.setErrorResult(value); }
    fn setBoolResult(&self, value: bool) { self.setBoolResult(value); }
    fn isSharedFormula(&self) -> bool { self.isSharedFormula() }
    fn isMasterSharedFormula(&self) -> bool { self.isMasterSharedFormula() }
    fn getSharedFormula(&self) -> Option<Box<dyn IEvalSharedFormula>> {
        self.getSharedFormula().map(|formula| Box::new(formula) as Box<dyn IEvalSharedFormula>)
    }
}
