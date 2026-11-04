// java package cn.wps.et.ss.formula.evaluator;
use super::WorkbookEvaluator::WorkbookEvaluator;
use crate::cn::wps::et::ss::interfaces::IEvalWorkbook::IEvalWorkbook;
use std::sync::Arc;
// java public final class EvaluationContext
pub struct EvaluationContext {
    _bookEvaluator: Arc<WorkbookEvaluator>,
    _workbook: Arc<dyn IEvalWorkbook>,
    _sheetIndex: i32,
    _rowIndex: i32,
    _columnIndex: i32,
    _sharedFormula: bool,
    _arrayFormula: bool,
    _lastRowIndex: i32,
    _lastColIndex: i32,
    isEndOfCalc: bool,condFormula:bool,
}
impl EvaluationContext {
    // java this(bookEvaluator, workbook, sheetIndex, srcRowNum, srcColNum, srcRowNum, srcColNum, false, sharedFml);
    pub fn EvaluationContext(
        be: Arc<WorkbookEvaluator>,
        wb: Arc<dyn IEvalWorkbook>,
        sheet: i32,
        row: i32,
        col: i32,
        shared: bool,
    ) -> Self {
        Self {
            _bookEvaluator: be,
            _workbook: wb,
            _sheetIndex: sheet,
            _rowIndex: row,
            _columnIndex: col,
            _sharedFormula: shared,
            _arrayFormula: false,
            _lastRowIndex: row,
            _lastColIndex: col,
            isEndOfCalc: false,condFormula:false,
        }
    }
    // java array constructor delegates with arrayFormula=true, sharedFormula=false.
    pub fn EvaluationContext__array(
        be: Arc<WorkbookEvaluator>,
        wb: Arc<dyn IEvalWorkbook>,
        sheet: i32,
        row: i32,
        col: i32,
        lr: i32,
        lc: i32,
    ) -> Self {
        let mut ec = Self::EvaluationContext(be, wb, sheet, row, col, false);
        ec._arrayFormula = true;
        ec._lastRowIndex = lr;
        ec._lastColIndex = lc;
        ec
    }
    pub fn getLastRowIndex(&self) -> i32 {
        self._lastRowIndex
    }
    pub fn getLastColIndex(&self) -> i32 {
        self._lastColIndex
    }
    // java private SheetRefEvaluator createExternSheetRefEvaluator(int externSheetIndex) {
    fn createExternSheetRefEvaluator(&self, externSheetIndex: i32) -> super::EvalResult<Option<Arc<super::SheetRefEvaluator::SheetRefEvaluator>>> {
        // java IEvalExtSheet extSheet = this._workbook.getExtSheet(externSheetIndex);
        let extSheet = self._workbook.getExtSheet(externSheetIndex).ok_or(super::EvalFailure::Runtime("NullPointerException"))?;
        // java if (extSheet.getBookType() == 4) {
        if extSheet.getBookType() == 4 {
            // java String workbookName = extSheet.getBookName(); IEvalWorkbook targetBook = this._workbook.getOtherWorkbook(workbookName);
            // java if (targetBook == null) { return null; } else { int otherSheetIndex = extSheet.getFirstSheetIndex(); return otherSheetIndex >= 0 && otherSheetIndex <= extSheet.getLastSheetIndex() ? new SheetRefEvaluator(targetBook, otherSheetIndex) : null; }
            let workbookName=extSheet.getBookName();let targetBook=self._workbook.clone().getOtherWorkbook(workbookName.as_deref());
            let Some(targetBook)=targetBook else{return Ok(None);};let otherSheetIndex=extSheet.getFirstSheetIndex();
            Ok(if otherSheetIndex>=0&&otherSheetIndex<=extSheet.getLastSheetIndex(){Some(Arc::new(super::SheetRefEvaluator::SheetRefEvaluator::SheetRefEvaluator(targetBook,otherSheetIndex)))}else{None})
        // java } else {
        } else {
            // java int otherSheetIndex = extSheet.getFirstSheetIndex();
            let mut otherSheetIndex = extSheet.getFirstSheetIndex();
            // java if (otherSheetIndex == -2) { otherSheetIndex = this._sheetIndex; }
            if otherSheetIndex == -2 { otherSheetIndex = self._sheetIndex; }
            // java return otherSheetIndex < 0 ? null : new SheetRefEvaluator(this._workbook, otherSheetIndex);
            Ok(if otherSheetIndex < 0 { None } else { Some(Arc::new(super::SheetRefEvaluator::SheetRefEvaluator::SheetRefEvaluator(self._workbook.clone(), otherSheetIndex))) })
        }
    }
    // java SheetRefEvaluator sre=getRefEvaluatorForCurrentSheet();return new LazyRefEval(rowIndex,columnIndex,sre);
    pub fn getRefEval(
        &self,
        row: i32,
        col: i32,
    ) -> crate::cn::wps::et::ss::formula::eval::ValueRef {
        Arc::new(super::super::eval::LazyRefEval::LazyRefEval::LazyRefEval(
            row,
            col,
            Arc::new(
                super::SheetRefEvaluator::SheetRefEvaluator::SheetRefEvaluator(
                    self._workbook.clone(),
                    self._sheetIndex,
                ),
            ),
        ))
    }
    // java return this._sheetIndex;
    pub fn getSheetIndex(&self) -> i32 {
        self._sheetIndex
    }
    // java return this._rowIndex;
    pub fn getRowIndex(&self) -> i32 {
        self._rowIndex
    }
    // java return this._columnIndex;
    pub fn getColumnIndex(&self) -> i32 {
        self._columnIndex
    }
    // java return this._workbook;
    pub fn getEvaluationWorkbook(&self) -> &Arc<dyn IEvalWorkbook> {
        &self._workbook
    }
    // java return this._sharedFormula;
    pub fn isSharedFormula(&self) -> bool {
        self._sharedFormula
    }
    // java this.isEndOfCalc = isEndOfCalc;
    pub fn setEndOfCalc(&mut self, value: bool) {
        self.isEndOfCalc = value;
    }
    // java public IValueEval getRef3DEval(int rowIndex, int columnIndex, int extSheetIndex) {
    pub fn getRef3DEval(&self, rowIndex: i32, columnIndex: i32, extSheetIndex: i32) -> super::EvalResult<super::super::eval::ValueRef> {
        // java SheetRefEvaluator sre = this.createExternSheetRefEvaluator(extSheetIndex);
        let sre = self.createExternSheetRefEvaluator(extSheetIndex)?;
        // java return (IValueEval)(sre == null ? ErrorEval.REF_INVALID : new LazyRefEval(rowIndex, columnIndex, sre));
        Ok(match sre { None => Arc::new(super::super::eval::ErrorEval::ErrorEval::REF_INVALID), Some(sre) => Arc::new(super::super::eval::LazyRefEval::LazyRefEval::LazyRefEval(rowIndex, columnIndex, sre)) })
    }
// java public IValueEval getAreaEval(int firstRowIndex, int firstColumnIndex, int lastRowIndex, int lastColumnIndex, byte rav) {
// java SheetRefEvaluator sre = this.getRefEvaluatorForCurrentSheet();
// java return new LazyAreaEval(firstRowIndex, firstColumnIndex, lastRowIndex, lastColumnIndex, rav, sre);
// java }
pub fn getAreaEval(&self,fr:i32,fc:i32,lr:i32,lc:i32,rav:i8)->super::super::eval::ValueRef {
Arc::new(super::super::eval::LazyAreaEval::LazyAreaEval::new(fr,fc,lr,lc,rav,Arc::new(super::SheetRefEvaluator::SheetRefEvaluator::SheetRefEvaluator(self._workbook.clone(),self._sheetIndex))))
}
    // java public IValueEval getArea3DEval(int firstRowIndex, int firstColumnIndex, int lastRowIndex, int lastColumnIndex, int extSheetIndex, byte rav) {
    pub fn getArea3DEval(&self, firstRowIndex: i32, firstColumnIndex: i32, lastRowIndex: i32, lastColumnIndex: i32, extSheetIndex: i32, rav: i8) -> super::EvalResult<super::super::eval::ValueRef> {
        // java SheetRefEvaluator sre = this.createExternSheetRefEvaluator(extSheetIndex);
        let sre = self.createExternSheetRefEvaluator(extSheetIndex)?;
        // java return (IValueEval)(sre == null ? ErrorEval.REF_INVALID : new LazyAreaEval(firstRowIndex, firstColumnIndex, lastRowIndex, lastColumnIndex, rav, sre));
        Ok(match sre { None => Arc::new(super::super::eval::ErrorEval::ErrorEval::REF_INVALID), Some(sre) => Arc::new(super::super::eval::LazyAreaEval::LazyAreaEval::new(firstRowIndex, firstColumnIndex, lastRowIndex, lastColumnIndex, rav, sre)) })
    }
// java public void setArrayFormula(boolean b) {
// java this._arrayFormula = b;
// java }
pub fn setArrayFormula(&mut self,v:bool) {
self._arrayFormula=v;
}
// java public void setCondFormula(boolean b) {
// java this._condFormula = b;
// java }
pub fn setCondFormula(&mut self,v:bool) {
self.condFormula=v;
}
// java public boolean isCondFormula() {
// java return this._condFormula;
// java }
pub fn isCondFormula(&self)->bool {
self.condFormula
}
// java public boolean isArrayFormula() {
// java return this._arrayFormula;
// java }
pub fn isArrayFormula(&self)->bool {
self._arrayFormula
}
// java public boolean isEndOfCalc(){return this.isEndOfCalc;}
pub fn isEndOfCalc(&self)->bool{self.isEndOfCalc}
}
impl EvaluationContext{// java public boolean isUsing1904() {
// java return this._workbook.isUsing1904Date();
// java }
pub fn isUsing1904(&self)->bool {
self._workbook.isUsing1904Date()
}
}
impl EvaluationContext{// java public IFunction findUserDefinedFunction(String functionName) {
// java IFunction ret = AggregatingUDFFinder.DEFAULT.findFunction(functionName);
// java if (ret != null) {
// java return ret;
// java } else {
// java IUDFFinder udfinder = this._workbook.getUDFFinder();
// java return udfinder != null ? udfinder.findFunction(functionName) : null;
// java }
// java }
pub fn findUserDefinedFunction(&self,name:&str)->super::EvalResult<Option<Arc<dyn super::super::functions::base::IFunction::IFunction>>> {
use super::super::udf::IUDFFinder::IUDFFinder;if let Some(f)=super::super::udf::AggregatingUDFFinder::DEFAULT.findFunction(name)?{Ok(Some(f))}else if let Some(finder)=self._workbook.getUDFFinder(){finder.findFunction(name)}else{Ok(None)}
}
// java public IValueEval getNameXEval(int extSheetIndex, int extNameIndex, byte ptgClass) {
// java IEvalExtSheet extSheet = this._workbook.getExtSheet(extSheetIndex);
// java IEvalNameBase aname = this._workbook.getNameByNameXPtg(extSheetIndex, extNameIndex);
// java if (extSheet != null && aname != null) {
// java if (extSheet.getBookType() == 3) {
// java return new UDFNameEval(aname.getNameText());
// java } else if (aname instanceof IEvalName) {
// java IEvalName name = (IEvalName)aname;
// java if (name.isFunctionName()) {
// java return new UDFNameEval(name.getNameText());
// java } else {
// java return (IValueEval)(name.hasFormula() ? this._bookEvaluator.evaluateNameFormula(name.getNameDefinition(), ptgClass, this) : ErrorEval.NAME_INVALID);
// java }
// java } else {
// java IEvalExtName ename = (IEvalExtName)aname;
// java if (extSheet.getBookType() != 4) {
// java return ErrorEval.NAME_INVALID;
// java } else {
// java IEvalWorkbook targetBook = this._workbook.getOtherWorkbook(extSheet.getBookName());
// java if (targetBook == null) {
// java return ErrorEval.REF_INVALID;
// java } else {
// java Ptg[] ptgs = ename.getNameDefinition();
// java if (ptgs.length > 1) {
// java throw new RuntimeException("Complex cross-book name formulas not supported yet");
// java } else {
// java ptgs = FormulaHelper.replaceRelativeNamePtgs(ptgs, this._rowIndex, this._columnIndex, targetBook.getSpreadsheetVersion());
// java if (ptgs.length == 0) {
// java return ErrorEval.REF_INVALID;
// java } else {
// java Ptg ptg = ptgs[0];
// java if (ptg.getSid() == 58) {
// java Ref3DExtPtg ref3D = (Ref3DExtPtg)ptg;
// java SheetRefEvaluator sre = new SheetRefEvaluator(targetBook, ref3D.getSheetIndex());
// java return new LazyRefEval(ref3D.getRow(), ref3D.getColumn(), sre);
// java } else if (ptg.getSid() == 59) {
// java Area3DExtPtg area3D = (Area3DExtPtg)ptg;
// java SheetRefEvaluator sre = new SheetRefEvaluator(targetBook, area3D.getSheetIndex());
// java return new LazyAreaEval(area3D.getFirstRow(), area3D.getFirstColumn(), area3D.getLastRow(), area3D.getLastColumn(), area3D.getPtgClass(), sre);
// java } else {
// java return ErrorEval.REF_INVALID;
// java }
// java }
// java }
// java }
// java }
// java }
// java } else {
// java return ErrorEval.NAME_INVALID;
// java }
// java }
pub fn getNameXEval(&mut self,sheet:i32,nameIndex:i32,ptgClass:i8)->super::EvalResult<crate::cn::wps::et::ss::formula::eval::ValueRef> {
use super::super::eval::{UDFNameEval::UDFNameEval,ErrorEval::ErrorEval};let ext=self._workbook.getExtSheet(sheet);let name=self._workbook.getNameByNameXPtg(sheet,nameIndex);let(Some(ext),Some(name))=(ext,name)else{return Ok(Arc::new(ErrorEval::NAME_INVALID));};if ext.getBookType()==3{return Ok(Arc::new(UDFNameEval::new(name.getNameText().to_string())));}if let Some(n)=name.as_any().downcast_ref::<crate::cn::wps::moss::engine::eval::EvaluationName::EvaluationName>(){if n.isFunctionName(){return Ok(Arc::new(UDFNameEval::new(n.getNameText().to_string())));}if n.hasFormula(){let be=self._bookEvaluator.clone();return be.evaluateNameFormula(&crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&n.getNameDefinition()),ptgClass,self)?.ok_or(super::EvalFailure::Runtime("null name result"));}return Ok(Arc::new(ErrorEval::NAME_INVALID));}if ext.getBookType()!=4{return Ok(Arc::new(ErrorEval::NAME_INVALID));}Err(super::EvalFailure::Unported("cross-book external name"))
}
}
