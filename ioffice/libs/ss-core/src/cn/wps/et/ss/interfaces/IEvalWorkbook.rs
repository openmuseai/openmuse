// java package cn.wps.et.ss.interfaces;

// java import org.apache.poi.ss.SpreadsheetVersion;
use crate::cn::wps::et::ss::dependent::Dependent::Dependent;
use crate::java_runtime::Rc;
use crate::java_runtime::RefCell;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;
// java import cn.wps.et.ss.interfaces.IEvalExtSheet/IEvalName/IEvalNameBase;
use super::IEvalExtSheet::IEvalExtSheet;
use super::IEvalName::IEvalName;
use super::IEvalNameBase::IEvalNameBase;

// java public interface IEvalWorkbook
// rust: methods required by Static*RefMap are present here; the remaining
// Java evaluation, callback, formatting and dirty-write methods are tracked
// in the M1 class plan before the full evaluator uses this interface.
pub trait IEvalWorkbook: Send + Sync {
    // java IEvalWorkbook getOtherWorkbook(String var1);
    // Arc receiver carries Java `return this` without allocating a new workbook.
    fn getOtherWorkbook(self:Rc<Self>,name:Option<&str>)->Option<Rc<dyn IEvalWorkbook>>;
// java IUDFFinder getUDFFinder();
fn getUDFFinder(&self)->Option<std::sync::Arc<dyn crate::cn::wps::et::ss::formula::udf::IUDFFinder::IUDFFinder>>{panic!("IEvalWorkbook.getUDFFinder")}
    // java CellRangeAddressBase getUseRange(int sheetIndex); int getBookType();
    fn getUseRange(&self,_sheet:i32)->Option<crate::cn::wps::moss::filefmt::biff8::util::CellRangeAddress::CellRangeAddress>{panic!("IEvalWorkbook.getUseRange")}
    fn getBookType(&self)->i32{panic!("IEvalWorkbook.getBookType")}
    // java Dependent getDependent();
    fn getDependent(&self) -> Option<Rc<RefCell<Dependent>>>;
    // java int sheetIndex2SheetUid(int var1);
    fn sheetIndex2SheetUid(&self, sheet_index: i32) -> i32;
    // java IEvalName getNameByNamePtg(int var1);
    fn getNameByNamePtg(&self, name_index: i32) -> Option<Box<dyn IEvalName>>;
    // java IEvalNameBase getNameByNameXPtg(int var1, int var2);
    fn getNameByNameXPtg(
        &self,
        ext_sheet_index: i32,
        ext_name_index: i32,
    ) -> Option<Box<dyn IEvalNameBase>>;
    // java IEvalExtSheet getExtSheet(int var1);
    fn getExtSheet(&self, ext_sheet_index: i32) -> Option<Box<dyn IEvalExtSheet>>;
    // java SpreadsheetVersion getSpreadsheetVersion();
    fn getSpreadsheetVersion(&self) -> SpreadsheetVersion;
    // java String getSheetName(int sheetIndex);
    fn getSheetName(&self,sheetIndex:i32)->Option<String>;
    // java String getFileName();
    fn getFileName(&self)->Option<String>;
    // java IEvalWorksheet getSheet(int sheetIndex);
    fn getSheet(&self, _sheet: i32) -> Rc<dyn super::IEvalWorksheet::IEvalWorksheet> {
        panic!("IEvalWorkbook.getSheet")
    }
    // java IEvalFormulaBase getFormula(int sheet,int row,int col);
    fn getFormula(
        &self,
        _sheet: i32,
        _row: i32,
        _col: i32,
    ) -> Option<Rc<dyn super::IEvalFormulaBase::IEvalFormulaBase>> {
        panic!("IEvalWorkbook.getFormula")
    }
    // java void setEvaluateCallback(IEvaluationCallback callback); IEvaluationCallback getEvaluateCallback();
    fn setEvaluateCallback(
        &self,
        _callback: Rc<
            dyn crate::cn::wps::et::ss::formula::IEvaluationCallback::IEvaluationCallback,
        >,
    ) {
        panic!("IEvalWorkbook.setEvaluateCallback")
    }
    fn getEvaluateCallback(
        &self,
    ) -> Option<Rc<dyn crate::cn::wps::et::ss::formula::IEvaluationCallback::IEvaluationCallback>>
    {
        None
    }
    // java IValueEval evaluateOutside(int uid,int r,int c,boolean needCalc);
    fn evaluateOutside(
        &self,
        uid: i32,
        row: i32,
        col: i32,
        needCalc: bool,
    ) -> crate::cn::wps::et::ss::formula::evaluator::EvalResult<
        Option<crate::cn::wps::et::ss::formula::eval::ValueRef>,
    > {
        self.getEvaluateCallback()
            .expect("NullPointerException evalCallback")
            .evaluation(uid, row, col, needCalc)
    }
    // java void addRef(int uid,int r,int c);
    fn addRef(&self, uid: i32, row: i32, col: i32) {
        self.getEvaluateCallback()
            .expect("NullPointerException evalCallback")
            .addRef(uid, row, col);
    }
    // java boolean isInterruptCalc(); void setInterruptMark(boolean flag);
    fn isInterruptCalc(&self) -> bool {
        panic!("IEvalWorkbook.isInterruptCalc")
    }
    fn setInterruptMark(&self, _flag: bool) {
        panic!("IEvalWorkbook.setInterruptMark")
    }
    // java int getSheetCount();
    fn getSheetCount(&self) -> i32 {
        panic!("IEvalWorkbook.getSheetCount")
    }
    // java boolean isUsing1904Date();
    fn isUsing1904Date(&self) -> bool {
        panic!("IEvalWorkbook.isUsing1904Date")
    }
    // java LanguageType getLanguageType();
    fn getLanguageType(&self) -> util::org::apache::poi::util::LanguageType::LanguageType {
        panic!("IEvalWorkbook.getLanguageType")
    }
    // java boolean isFullPrecision();
    fn isFullPrecision(&self) -> bool {
        panic!("IEvalWorkbook.isFullPrecision")
    }
    // java IValueEval getPrecisionValue(IValueEval value, int sheet, int row, int col);
    fn getPrecisionValue(
        &self,
        _value: crate::cn::wps::et::ss::formula::eval::ValueRef,
        _sheet: i32,
        _row: i32,
        _col: i32,
    ) -> crate::cn::wps::et::ss::formula::evaluator::EvalResult<
        crate::cn::wps::et::ss::formula::eval::ValueRef,
    > {
        Err(
            crate::cn::wps::et::ss::formula::evaluator::EvalFailure::Unported(
                "getPrecisionValue/numFormatter",
            ),
        )
    }
    // java void writeDirty(IValueEval value, int uid, int row, int col);
    fn writeDirty(
        &self,
        _value: &crate::cn::wps::et::ss::formula::eval::ValueRef,
        _uid: i32,
        _row: i32,
        _col: i32,
    ) {
        panic!("IEvalWorkbook.writeDirty")
    }
}
