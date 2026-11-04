// java package cn.wps.moss.engine.eval;

use crate::java_runtime::Rc;

use crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg;
use crate::cn::wps::et::ss::interfaces::IEvalExtSheet::IEvalExtSheet;
use crate::cn::wps::et::ss::interfaces::IEvalName::IEvalName;
use crate::cn::wps::et::ss::interfaces::IEvalNameBase::IEvalNameBase;
use crate::cn::wps::et::ss::interfaces::IEvalWorkbook::IEvalWorkbook;
use crate::cn::wps::moss::app::KmoBook::KmoBook;
use crate::cn::wps::moss::engine::eval::EvaluationExtName::EvaluationExtName;
use crate::cn::wps::moss::engine::eval::EvaluationExtSheet::EvaluationExtSheet;
use crate::cn::wps::moss::engine::eval::EvaluationName::EvaluationName;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

// java public class EvaluationWorkbook implements IEvalWorkbook
pub struct EvaluationWorkbook {
    // java private final KmoBook kbook;
    kbook: crate::java_runtime::Weak<KmoBook>,
    isInterruptedInCacl: crate::java_runtime::Cell<bool>,
    evalCallback: crate::java_runtime::RefCell<
        Option<Rc<dyn crate::cn::wps::et::ss::formula::IEvaluationCallback::IEvaluationCallback>>,
    >,
}

// java EvaluationWorkbook implements IEvalWorkbook.
impl IEvalWorkbook for EvaluationWorkbook {
    // java public IEvalWorkbook getOtherWorkbook(String name){if(name!=null&&name.length()!=0){try{return new SupEvaluationWorkbook(this.kbook.getSpreadsheetVersion(),this.kbook.getLinkTable(),this.kbook.getLinkTable().getExternalBookBlock(name),this.kbook.isUsing1904());}catch(RuntimeException var3){return null;}}else{return this;}}
    fn getOtherWorkbook(self:Rc<Self>,name:Option<&str>)->Option<Rc<dyn IEvalWorkbook>>{if name.is_some_and(|n|!n.is_empty()){let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let book=self.book();let linktable=book.getLinkTable();let external=linktable.borrow().getExternalBookBlock(name);Rc::new(super::sup::SupEvaluationWorkbook::SupEvaluationWorkbook::SupEvaluationWorkbook(book.getSpreadsheetVersion(),linktable,external,book.isUsing1904()))as Rc<dyn IEvalWorkbook>}));match result{Ok(value)=>Some(value),Err(e)=>{let s=e.downcast_ref::<String>().map(String::as_str).or_else(||e.downcast_ref::<&str>().copied()).unwrap_or("");if s.starts_with("RuntimeException")||s.starts_with("NullPointerException")||s.starts_with("IllegalStateException"){None}else{std::panic::resume_unwind(e)}}}}else{Some(self)}}
// java public IUDFFinder getUDFFinder(){return this.udffinder;} // java constructor leaves udffinder null.
fn getUDFFinder(&self)->Option<std::sync::Arc<dyn crate::cn::wps::et::ss::formula::udf::IUDFFinder::IUDFFinder>>{None}

    fn getDependent(
        &self,
    ) -> Option<
        Rc<crate::java_runtime::RefCell<crate::cn::wps::et::ss::dependent::Dependent::Dependent>>,
    > {
        EvaluationWorkbook::getDependent(self)
    }
    fn sheetIndex2SheetUid(&self, index: i32) -> i32 {
        EvaluationWorkbook::sheetIndex2SheetUid(self, index)
    }
    fn getNameByNamePtg(&self, index: i32) -> Option<Box<dyn IEvalName>> {
        EvaluationWorkbook::getNameByNamePtg(self, index)
            .map(|name| Box::new(name) as Box<dyn IEvalName>)
    }
    fn getNameByNameXPtg(&self, sheet: i32, name: i32) -> Option<Box<dyn IEvalNameBase>> {
        EvaluationWorkbook::getNameByNameXPtg(self, sheet, name)
    }
    fn getExtSheet(&self, index: i32) -> Option<Box<dyn IEvalExtSheet>> {
        Some(Box::new(EvaluationWorkbook::getExtSheet(self, index)))
    }
    fn getSpreadsheetVersion(&self) -> SpreadsheetVersion {
        EvaluationWorkbook::getSpreadsheetVersion(self)
    }
    fn getSheetName(&self,sheetIndex:i32)->Option<String>{Some(EvaluationWorkbook::getSheetName(self,sheetIndex))}
    fn getFileName(&self)->Option<String>{Some(EvaluationWorkbook::getFileName(self))}
    // java this.evalCallback = callback; return this.evalCallback;
    fn setEvaluateCallback(
        &self,
        callback: Rc<dyn crate::cn::wps::et::ss::formula::IEvaluationCallback::IEvaluationCallback>,
    ) {
        *self.evalCallback.borrow_mut() = Some(callback);
    }
    fn getEvaluateCallback(
        &self,
    ) -> Option<Rc<dyn crate::cn::wps::et::ss::formula::IEvaluationCallback::IEvaluationCallback>>
    {
        self.evalCallback.borrow().clone()
    }
    // java return this.isInterruptedInCacl;
    fn isInterruptCalc(&self) -> bool {
        self.isInterruptedInCacl.get()
    }
    // java this.isInterruptedInCacl = flag;
    fn setInterruptMark(&self, flag: bool) {
        self.isInterruptedInCacl.set(flag);
    }
    // java return new EvaluationWorksheet(kbook.sheetAt(sheetIndex));
    // java public CellRangeAddressBase getUseRange(int sheetIndex){return ExpandHelper.rect2range(this.kbook.sheetAt(sheetIndex).getUsedRange());}
    fn getUseRange(&self,sheet:i32)->Option<crate::cn::wps::moss::filefmt::biff8::util::CellRangeAddress::CellRangeAddress>{let r=self.book().sheetAt(sheet).borrow().getUsedRange();Some(crate::cn::wps::moss::filefmt::biff8::util::CellRangeAddress::CellRangeAddress::new(r.tl.row,r.br.row,r.tl.col,r.br.col))}
    // java public int getBookType(){return 0;}
    fn getBookType(&self)->i32{0}
    fn getSheet(
        &self,
        index: i32,
    ) -> Rc<dyn crate::cn::wps::et::ss::interfaces::IEvalWorksheet::IEvalWorksheet> {
        Rc::new(
            super::EvaluationWorksheet::EvaluationWorksheet::EvaluationWorksheet(
                self.book().sheetAt(index),
            ),
        )
    }
    // java return this.kbook.sheetCount();
    fn getSheetCount(&self) -> i32 {
        self.book().sheetCount()
    }
    // java return KmoBooks.getLanguageType();
    fn getLanguageType(&self) -> util::org::apache::poi::util::LanguageType::LanguageType {
        crate::cn::wps::moss::app::KmoBooks::KmoBooks::getLanguageType()
    }
    // java return this.kbook.isUsing1904();
    fn isUsing1904Date(&self) -> bool {
        self.book().isUsing1904()
    }
    // java return this.kbook.getCalcPr().isFullPrecision();
    fn isFullPrecision(&self) -> bool {
        self.book().getCalcPr().borrow().isFullPrecision()
    }
    // java public IValueEval getPrecisionValue(IValueEval valueEval,int sheetIndex,int row,int col)
    fn getPrecisionValue(&self,value:crate::cn::wps::et::ss::formula::eval::ValueRef,sheetIndex:i32,row:i32,col:i32)->crate::cn::wps::et::ss::formula::evaluator::EvalResult<crate::cn::wps::et::ss::formula::eval::ValueRef>{
        use crate::cn::wps::et::ss::formula::eval::NumberEval::NumberEval;
        // java only NumberEval enters formatting. Pool buffer lifetime is local to this branch.
        if let Some(number)=value.as_any().downcast_ref::<NumberEval>(){
            let book=self.book();let cell=book.sheetAt(sheetIndex).borrow().getCellData(row,col);
            let format=book.getXFManager().borrow().getXF(cell.ixf).expect("null XF").getNumFormat().expect("null format code");
            // java v=numFormatter.formatValue(v,kbook.isUsing1904(),xf.getNumFormat());valueEval=new NumberEval(v);
            let formatted=crate::cn::wps::moss::app::KmoApp::KmoApp::sharedApp().numFormatter(|formatter|formatter.expect("null numFormatter").formatValue(number.getNumberValue(),book.isUsing1904(),&format));
            Ok(Rc::new(NumberEval::NumberEval(formatted)))
        }else{Ok(value)}
    }
    // java getFormula(sheetIndex,row,col) returns null for nonformula/array cells.
    fn getFormula(
        &self,
        sheetIndex: i32,
        row: i32,
        col: i32,
    ) -> Option<Rc<dyn crate::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase>>
    {
        let sheet = self.book().sheetAt(sheetIndex);
        let cell = sheet.borrow().getCellData(row, col);
        if cell.formulaType != 0 && cell.formulaType != 2 {
            Some(Rc::new(
                super::EvaluationCellFormula::EvaluationCellFormula::EvaluationCellFormula(
                    sheet, row, col,
                ),
            ))
        } else {
            None
        }
    }
    // java writeDirty dispatches scalar value types to the original EvaluationPresent.
    fn writeDirty(
        &self,
        value: &crate::cn::wps::et::ss::formula::eval::ValueRef,
        uid: i32,
        row: i32,
        col: i32,
    ) {
        use super::EvaluationPresent::EvaluationPresent;
        use crate::cn::wps::et::ss::formula::eval::{
            BoolEval::BoolEval, ErrorEval::ErrorEval, NumberEval::NumberEval,
            StringEval::StringEval,
        };
        // java try { scalar dispatch } catch (IllegalStateException/RuntimeException) {}
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let Some(sheet) = self.book().sheetUid2Sheet(uid) else {
                return;
            };
            let sheet = sheet.borrow();
            if let Some(n) = value.as_any().downcast_ref::<NumberEval>() {
                EvaluationPresent::setNumberResult(&sheet, row, col, n.getNumberValue());
            } else if let Some(b) = value.as_any().downcast_ref::<BoolEval>() {
                EvaluationPresent::setBoolResult(&sheet, row, col, b.getBooleanValue());
            } else if let Some(s) = value.as_any().downcast_ref::<StringEval>() {
                // A Java String is converted only when representable by the current UTF-8 store.
                if let Ok(text) = String::from_utf16(s.getStringValue()) {
                    EvaluationPresent::setStringResult(&sheet, row, col, &text);
                }
            } else if let Some(e) = value.as_any().downcast_ref::<ErrorEval>() {
                EvaluationPresent::setErrorResult(&sheet, row, col, e.getErrorCode() as i8);
            }
            // java catch (IllegalStateException/RuntimeException) {} for unexpected values.
        }));
    }
}

impl EvaluationWorkbook {
    // java public Dependent getDependent()
    pub fn getDependent(
        &self,
    ) -> Option<
        Rc<crate::java_runtime::RefCell<crate::cn::wps::et::ss::dependent::Dependent::Dependent>>,
    > {
        // java return this.kbook.getDependent();
        self.book().getDependent()
    }

    // java public int sheetIndex2SheetUid(int sheetIndex)
    pub fn sheetIndex2SheetUid(&self, sheet_index: i32) -> i32 {
        // java return this.kbook.sheetAt(sheetIndex).getSheetUid();
        self.book().sheetAt(sheet_index).borrow().getSheetUid()
    }

    // java public EvaluationName getNameByNamePtg(int nameIndex)
    pub fn getNameByNamePtg(&self, name_index: i32) -> Option<EvaluationName> {
        // java try { return new EvaluationName(kbook.getLinkTable().getNameRecord(nameIndex), nameIndex); }
        // java catch (RuntimeException) { return null; }
        self.book()
            .getLinkTable()
            .borrow()
            .tryNameRecord(name_index).ok()
            .map(|record| EvaluationName::EvaluationName(record, name_index))
    }

    // java public IEvalNameBase getNameByNameXPtg(int extSheetIndex, int extNameIndex)
    pub fn getNameByNameXPtg(
        &self,
        ext_sheet_index: i32,
        ext_name_index: i32,
    ) -> Option<Box<dyn IEvalNameBase>> {
        // java LinkTable lt = this.kbook.getLinkTable();
        let link_table = self.book().getLinkTable();
        let table = link_table.borrow();
        // java ExternSheetRecord.RefSubRecord extSheetRef = lt.resolveExtSheetIndex(extSheetIndex);
        let ext_sheet_ref = table.resolveExtSheetIndex(ext_sheet_index)?;
        // java LinkTable.ExternalBookBlock extBookRef = lt.getExternalBookBlock(extSheetRef.getExtBookIndex());
        let ext_book_index = ext_sheet_ref.getExtBookIndex();
        // java SupBookRecord supBookRec = extBookRef.getExternalBookRecord();
        // java if (supBookRec == null || !supBookRec.isExternalReferences() && !supBookRec.isAddInFunctions())
        if !table.isExternalReferences(ext_book_index) && !table.isAddInFunctions(ext_book_index) {
            // java return new EvaluationName(lt.getNameRecord(extNameIndex), extNameIndex);
            let record = table.getNameRecord(ext_name_index);
            Some(Box::new(EvaluationName::EvaluationName(
                record,
                ext_name_index,
            )))
        } else {
            // java return new EvaluationExtName(extBookRef.getExternalNameRecord(extNameIndex));
            let record = table.getExternalNameRecord(ext_book_index, ext_name_index)?;
            Some(Box::new(EvaluationExtName::EvaluationExtName(record)))
        }
    }

    // java public String getSheetName(int sheetindex){return this.kbook.sheetAt(sheetindex).name();}
    pub fn getSheetName(&self,sheetindex:i32)->String{self.book().sheetAt(sheetindex).borrow().name()}

    // java public EvaluationExtSheet getExtSheet(int extSheetIndex)
    pub fn getExtSheet(&self, ext_sheet_index: i32) -> EvaluationExtSheet {
        // java return new EvaluationExtSheet(this.kbook, extSheetIndex);
        EvaluationExtSheet::EvaluationExtSheet(&self.book(), ext_sheet_index)
    }

    // java public SpreadsheetVersion getSpreadsheetVersion()
    pub fn getSpreadsheetVersion(&self) -> SpreadsheetVersion {
        // java return this.kbook.getSpreadsheetVersion();
        self.book().getSpreadsheetVersion()
    }

    // java public EvaluationWorkbook(KmoBook b)
    pub fn EvaluationWorkbook(b: &Rc<KmoBook>) -> Self {
        // java this.kbook = b;
        Self {
            kbook: Rc::downgrade(b),
            isInterruptedInCacl: crate::java_runtime::Cell::new(false),
            evalCallback: crate::java_runtime::RefCell::new(None),
        }
    }

    pub fn book(&self) -> Rc<KmoBook> {
        self.kbook
            .upgrade()
            .expect("EvaluationWorkbook outlived its KmoBook")
    }
    // java public String getFileName(){String ret=this.kbook.getFilePath();return ret==null?"":ret;}
    pub fn getFileName(&self)->String{let ret=self.book().getFilePath();ret.unwrap_or_default()}

    // java public EvaluationName getNameByText(String name, int sheetIndex)
    pub fn getNameByText(&self, name: &str, sheet_index: i32) -> Option<EvaluationName> {
        self.getNameByTextImpl(name, sheet_index)
    }

    // java public EvaluationName getNameByTextImpl(String name, int sheetIndex)
    pub fn getNameByTextImpl(&self, name: &str, sheet_index: i32) -> Option<EvaluationName> {
        let table = self.book().getLinkTable();
        let table = table.borrow();
        let count = table.getNumNames();
        let mut index = 0;
        while index < count {
            let record = table.getNameRecord(index).expect("NullPointerException");
            // java !EMPTY_FORMULA.equals(formula) && sheetNumber == sheetIndex + 1 && equalsIgnoreCase
            let matches={let value=record.borrow();!value.formula_is_empty()
                && value.getSheetNumber() == sheet_index.wrapping_add(1)
                && crate::java_runtime::Character::equalsIgnoreCase(&value.getNameText().encode_utf16().collect::<Vec<_>>(),&name.encode_utf16().collect::<Vec<_>>())};
            if matches {
                return Some(EvaluationName::EvaluationName(Some(record), index));
            }
            index += 1;
        }
        drop(table);
        // java return sheetIndex == -1 ? null : getNameByTextImpl(name, -1);
        if sheet_index == -1 {
            None
        } else {
            self.getNameByTextImpl(name, -1)
        }
    }

    // java public Ptg getUDFName(String name)
    pub fn getUDFName(&self, name: &str) -> Ptg {
        self.book().getLinkTable().borrow_mut().getUDFName(name)
    }

    // java public Ptg checkUDFName(String name)
    pub fn checkUDFName(&self, name: &str) -> Ptg {
        self.book().getLinkTable().borrow_mut().checkUDFName(name)
    }

    // java public int checkExternalSheetIndex(String sheetName, int formulaType)
    pub fn checkExternalSheetIndex(&self, sheet_name: &str, formula_type: i32) -> i32 {
        // java int sheetindex = 0;
        let mut sheetindex = 0;
        // java if (sheetName != null && sheetName.length() != 0)
        if !sheet_name.is_empty() {
            // java for (sheetcount = kbook.sheetCount(); sheetindex < sheetcount && !name.equalsIgnoreCase(sheetName); ++sheetindex)
            let sheetcount = self.book().sheetCount();
            while sheetindex < sheetcount
                && !self
                    .book()
                    .sheetAt(sheetindex)
                    .borrow()
                    .name()
                    .eq_ignore_ascii_case(sheet_name)
            {
                sheetindex += 1;
            }
            // java if (sheetindex >= sheetcount) return -1;
            if sheetindex >= sheetcount {
                return -1;
            }
        } else {
            // java sheetindex = -2;
            sheetindex = -2;
        }
        // java formulaType == 4 && sheetindex == -2 ? checkSameSheetReferSupBookExternSheet : checkNullSupBookExternSheet
        // SameSheetReferSupBook 还没有。Java 找不到时抛 IllegalStateException，这里收成 -1。
        if formula_type == 4 && sheetindex == -2 {
            return -1;
        }
        self.book()
            .getLinkTable()
            .borrow_mut()
            .checkNullSupBookExternSheet(sheetindex)
    }

    // java public int checkExternalSheetIndex(String workbookName, String sheetName)
    pub fn checkExternalSheetIndex__book(&self, _workbook_name: &str, _sheet_name: &str) -> i32 {
        // java return kbook.getLinkTable().checkExternalSheetIndex(workbookName, sheetName);
        // 外部工作簿块还没有。Java 捕获 RuntimeException 后返回 -1。
        -1
    }
}
