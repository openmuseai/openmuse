use crate::{java_runtime::{Rc,RefCell,Cell},cn::wps::{et::ss::{dependent::Dependent::Dependent,formula::{eval::ValueRef,evaluator::EvalResult,ptg::Ptg::Ptg,udf::IUDFFinder::IUDFFinder,IEvaluationCallback::IEvaluationCallback},interfaces::{IEvalWorkbook::IEvalWorkbook,IEvalWorksheet::IEvalWorksheet,IEvalExtSheet::IEvalExtSheet,IEvalName::IEvalName,IEvalNameBase::IEvalNameBase,IEvalFormulaBase::IEvalFormulaBase}},moss::{app::{KmoApp::KmoApp,KmoBook::KmoBook},engine::linktb::LinkTable::{LinkTable,ExternalBookBlock},filefmt::biff8::util::CellRangeAddress::CellRangeAddress}},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion};
use super::SupEvaluationSheet::SupEvaluationSheet;
// java public final class SupEvaluationWorkbook implements IEvalWorkbook {
pub struct SupEvaluationWorkbook{
    // java boolean using1904;
    using1904:bool,
    // java LinkTable linktable;
    linktable:Rc<RefCell<LinkTable>>,
    // java LinkTable.ExternalBookBlock externalBookBlock;
    externalBookBlock:Option<Rc<ExternalBookBlock>>,
    // java SpreadsheetVersion ssVersion;
    ssVersion:SpreadsheetVersion,
    // java boolean isInterruptedInCacl;
    isInterruptedInCacl:Cell<bool>,
    // Native root reference preserves the original LinkTable.book graph.
    _owner:Option<Rc<KmoBook>>,
}
impl SupEvaluationWorkbook{
    // java public SupEvaluationWorkbook(SpreadsheetVersion version,LinkTable lt,LinkTable.ExternalBookBlock ebb,boolean use1904){this.ssVersion=version;this.linktable=lt;this.externalBookBlock=ebb;this.using1904=use1904;}
    pub fn SupEvaluationWorkbook(version:SpreadsheetVersion,lt:Rc<RefCell<LinkTable>>,ebb:Option<Rc<ExternalBookBlock>>,use1904:bool)->Self{let _owner=lt.borrow().ownerBook();Self{using1904:use1904,linktable:lt,externalBookBlock:ebb,ssVersion:version,isInterruptedInCacl:Cell::new(false),_owner}}
    // java public IEvalWorksheet getSheet(int sheetIndex){LinkTable.CRNBlock crn=this.externalBookBlock.getCRNBlock(sheetIndex);return new SupEvaluationSheet(sheetIndex,crn);}
    pub fn getSheet(&self,sheetIndex:i32)->Rc<SupEvaluationSheet>{let crn=self.externalBookBlock.as_ref().expect("NullPointerException").getCRNBlock(sheetIndex);SupEvaluationSheet::SupEvaluationSheet(sheetIndex,crn)}
    // java public int getSheetUniqueSign(int sheetIndex){LinkTable.CRNBlock crn=this.externalBookBlock.getCRNBlock(sheetIndex);return crn.hashCode();}
    pub fn getSheetUniqueSign(&self,sheetIndex:i32)->i32{let crn=self.externalBookBlock.as_ref().expect("NullPointerException").getCRNBlock(sheetIndex).expect("NullPointerException");Rc::as_ptr(&crn)as usize as i32}
    // java public int getSheetIndex(String sheetName){String[] sheets=this.externalBookBlock.getExternalBookRecord().getSheetNames();int i=0;for(int len=sheets.length;i<len;++i){if(sheets[i].equalsIgnoreCase(sheetName)){return i;}}return -1;}
    pub fn getSheetIndex(&self,sheetName:Option<&str>)->i32{let sheets=self.externalBookBlock.as_ref().expect("NullPointerException").getExternalBookRecord().expect("NullPointerException").getSheetNames();let mut i=0;let len=sheets.len();while i<len{let s=sheets[i].as_ref().expect("NullPointerException");if sheetName.is_some_and(|n|crate::java_runtime::Character::equalsIgnoreCase(&s.encode_utf16().collect::<Vec<_>>(),&n.encode_utf16().collect::<Vec<_>>())){return i as i32;}i+=1;}-1}
    // java public String getSheetName(int sheetIndex){return this.externalBookBlock.getExternalBookRecord().getSheetNames()[sheetIndex];}
    pub fn getSheetName(&self,sheetIndex:i32)->Option<String>{self.externalBookBlock.as_ref().expect("NullPointerException").getExternalBookRecord().expect("NullPointerException").getSheetNames()[sheetIndex as usize].clone()}
    // java public IEvalExtSheet getExtSheet(int extSheetIndex){throw new RuntimeException("unimpl");}
    pub fn getExtSheet(&self,_extSheetIndex:i32)->Option<Box<dyn IEvalExtSheet>>{panic!("RuntimeException unimpl")}
    // java public IEvalWorkbook getOtherWorkbook(String name){throw new RuntimeException("unimpl");}
    pub fn getOtherWorkbook(&self,_name:Option<&str>)->Option<Rc<dyn IEvalWorkbook>>{panic!("RuntimeException unimpl")}
    // java public IUDFFinder getUDFFinder(){return null;}
    pub fn getUDFFinder(&self)->Option<Rc<dyn IUDFFinder>>{None}
    // java public SpreadsheetVersion getSpreadsheetVersion(){return this.ssVersion;}
    pub fn getSpreadsheetVersion(&self)->SpreadsheetVersion{self.ssVersion}
    // java public boolean isInterruptCalc(){return this.isInterruptedInCacl;}
    pub fn isInterruptCalc(&self)->bool{self.isInterruptedInCacl.get()}
    // java public void setInterruptMark(boolean flag){this.isInterruptedInCacl=flag;}
    pub fn setInterruptMark(&self,flag:bool){self.isInterruptedInCacl.set(flag);}
    // java public void writeDirty(IValueEval dirtyValue,int uid,int r,int c){throw new RuntimeException("unimpl");}
    pub fn writeDirty(&self,_dirtyValue:&ValueRef,_uid:i32,_r:i32,_c:i32){panic!("RuntimeException unimpl")}
    // java public boolean isFullPrecision(){return false;}
    pub fn isFullPrecision(&self)->bool{false}
    // java public IValueEval getPrecisionValue(IValueEval valueEval,int sheetIndex,int row,int col){return valueEval;}
    pub fn getPrecisionValue(&self,valueEval:ValueRef,_sheetIndex:i32,_row:i32,_col:i32)->ValueRef{valueEval}
    // java public boolean isUsing1904Date(){return this.using1904;}
    pub fn isUsing1904Date(&self)->bool{self.using1904}
    // java public String formatNumber(double value,String numfmt){INumFormatter formatter=KmoApp.sharedApp().numFormatter();String formatCode=formatter.compile(numfmt);if(formatCode!=null){NumFmtResult rs=new NumFmtResult();formatter.format(value,formatCode,Integer.MAX_VALUE,this.isUsing1904Date(),rs);return rs.numfmtStr();}else{return numfmt;}}
    pub fn formatNumber(&self,value:f64,numfmt:&str)->String{KmoApp::sharedApp().numFormatter(|formatter|{let formatter=formatter.expect("NullPointerException");let formatCode=formatter.compile(numfmt);if let Some(formatCode)=formatCode{let mut rs=kfc::cn::wps::kfc::numfmt::format::NumFmtResult::NumFmtResult::new();formatter.format_num(value,&formatCode,i32::MAX,self.isUsing1904Date(),&mut rs);rs.numfmtStr().into()}else{numfmt.into()}})}
    // java public String getFileName(){return this.externalBookBlock.getExternalBookRecord().getURL();}
    pub fn getFileName(&self)->String{self.externalBookBlock.as_ref().expect("NullPointerException").getExternalBookRecord().expect("NullPointerException").getURL()}
    // java public int getActiveSheetIndex(){return 0;}
    pub fn getActiveSheetIndex(&self)->i32{0}
    // java public int getSheetCount(){return this.externalBookBlock.getExternalBookRecord().getNumberOfSheets();}
    pub fn getSheetCount(&self)->i32{self.externalBookBlock.as_ref().expect("NullPointerException").getExternalBookRecord().expect("NullPointerException").getNumberOfSheets()as i32}
    // java public IEvalName getNameByText(String name,int sheetIndex){return null;}
    pub fn getNameByText(&self,_name:Option<&str>,_sheetIndex:i32)->Option<Box<dyn IEvalName>>{None}
    // java public IEvalName getNameByNamePtg(int nameIndex){return null;}
    pub fn getNameByNamePtg(&self,_nameIndex:i32)->Option<Box<dyn IEvalName>>{None}
    // java public IEvalNameBase getNameByNameXPtg(int extSheetIndex,int extNameIndex){return null;}
    pub fn getNameByNameXPtg(&self,_extSheetIndex:i32,_extNameIndex:i32)->Option<Box<dyn IEvalNameBase>>{None}
    // java public Dependent getDependent(){throw new RuntimeException("unimpl");}
    pub fn getDependent(&self)->Option<Rc<RefCell<Dependent>>>{panic!("RuntimeException unimpl")}
    // java public int sheetIndex2SheetUid(int sheetIndex){throw new RuntimeException("unimpl");}
    pub fn sheetIndex2SheetUid(&self,_sheetIndex:i32)->i32{panic!("RuntimeException unimpl")}
    // java public IEvalFormulaBase getFormula(int sheetIndex,int row,int col){throw new RuntimeException("unimpl");}
    pub fn getFormula(&self,_sheetIndex:i32,_row:i32,_col:i32)->Option<Rc<dyn IEvalFormulaBase>>{panic!("RuntimeException unimpl")}
    // java public IValueEval evaluateOutside(int uid,int r,int c,boolean needCalc){return null;}
    pub fn evaluateOutside(&self,_uid:i32,_r:i32,_c:i32,_needCalc:bool)->Option<ValueRef>{None}
    // java public void addRef(int uid,int r,int c) { }
    pub fn addRef(&self,_uid:i32,_r:i32,_c:i32){}
    // java public void setEvaluateCallback(IEvaluationCallback callback) { }
    pub fn setEvaluateCallback(&self,_callback:Option<Rc<dyn IEvaluationCallback>>){}
    // java public int checkExternalSheetIndex(String sheetName,int formulaType){throw new RuntimeException("unimpl");}
    pub fn checkExternalSheetIndex(&self,_sheetName:Option<&str>,_formulaType:i32)->i32{panic!("RuntimeException unimpl")}
    // java public int checkExternalSheetIndex(String workbookName,String sheetName){throw new RuntimeException("unimpl");}
    pub fn checkExternalSheetIndex__book(&self,_workbookName:Option<&str>,_sheetName:Option<&str>)->i32{panic!("RuntimeException unimpl")}
    // java public int checkExternalSheetIndex(int extBookIndex,String sheetName){throw new RuntimeException("unimpl");}
    pub fn checkExternalSheetIndex__index(&self,_extBookIndex:i32,_sheetName:Option<&str>)->i32{panic!("RuntimeException unimpl")}
    // java public Ptg getUDFName(String name){throw new RuntimeException("unimpl");}
    pub fn getUDFName(&self,_name:Option<&str>)->Ptg{panic!("RuntimeException unimpl")}
    // java public Ptg checkUDFName(String name){throw new RuntimeException("unimpl");}
    pub fn checkUDFName(&self,_name:Option<&str>)->Ptg{panic!("RuntimeException unimpl")}
    // java public Ptg transformPtg(Ptg ptg){return null;}
    pub fn transformPtg(&self,_ptg:Option<&Ptg>)->Option<Ptg>{None}
    // java public IEvaluationCallback getEvaluateCallback(){return null;}
    pub fn getEvaluateCallback(&self)->Option<Rc<dyn IEvaluationCallback>>{None}
    // java public CellRangeAddressBase getUseRange(int sheetIndex){throw new RuntimeException("unimpl");}
    pub fn getUseRange(&self,_sheetIndex:i32)->Option<CellRangeAddress>{panic!("RuntimeException unimpl")}
    // java public LanguageType getLanguageType(){throw new RuntimeException("unimpl");}
    pub fn getLanguageType(&self)->util::org::apache::poi::util::LanguageType::LanguageType{panic!("RuntimeException unimpl")}
    // java public int getBookType(){return 1;}
    pub fn getBookType(&self)->i32{1}
    // java public int[] getLastModify(){throw new RuntimeException("unimpl");}
    pub fn getLastModify(&self)->Vec<i32>{panic!("RuntimeException unimpl")}
}
impl IEvalWorkbook for SupEvaluationWorkbook{
 fn getSheetName(&self,sheetIndex:i32)->Option<String>{Self::getSheetName(self,sheetIndex)}
 fn getFileName(&self)->Option<String>{Some(Self::getFileName(self))}
 fn getOtherWorkbook(self:Rc<Self>,name:Option<&str>)->Option<Rc<dyn IEvalWorkbook>>{SupEvaluationWorkbook::getOtherWorkbook(&self,name)}
 fn getDependent(&self)->Option<Rc<RefCell<Dependent>>>{Self::getDependent(self)}fn sheetIndex2SheetUid(&self,s:i32)->i32{Self::sheetIndex2SheetUid(self,s)}fn getNameByNamePtg(&self,n:i32)->Option<Box<dyn IEvalName>>{Self::getNameByNamePtg(self,n)}fn getNameByNameXPtg(&self,s:i32,n:i32)->Option<Box<dyn IEvalNameBase>>{Self::getNameByNameXPtg(self,s,n)}fn getExtSheet(&self,s:i32)->Option<Box<dyn IEvalExtSheet>>{Self::getExtSheet(self,s)}fn getSpreadsheetVersion(&self)->SpreadsheetVersion{Self::getSpreadsheetVersion(self)}fn getSheet(&self,s:i32)->Rc<dyn IEvalWorksheet>{Self::getSheet(self,s)}fn getFormula(&self,s:i32,r:i32,c:i32)->Option<Rc<dyn IEvalFormulaBase>>{Self::getFormula(self,s,r,c)}fn setEvaluateCallback(&self,c:Rc<dyn IEvaluationCallback>){Self::setEvaluateCallback(self,Some(c))}fn getEvaluateCallback(&self)->Option<Rc<dyn IEvaluationCallback>>{Self::getEvaluateCallback(self)}fn evaluateOutside(&self,u:i32,r:i32,c:i32,n:bool)->EvalResult<Option<ValueRef>>{Ok(Self::evaluateOutside(self,u,r,c,n))}fn addRef(&self,u:i32,r:i32,c:i32){Self::addRef(self,u,r,c)}fn isInterruptCalc(&self)->bool{Self::isInterruptCalc(self)}fn setInterruptMark(&self,f:bool){Self::setInterruptMark(self,f)}fn getSheetCount(&self)->i32{Self::getSheetCount(self)}fn getLanguageType(&self)->util::org::apache::poi::util::LanguageType::LanguageType{Self::getLanguageType(self)}fn isUsing1904Date(&self)->bool{Self::isUsing1904Date(self)}fn isFullPrecision(&self)->bool{Self::isFullPrecision(self)}fn getPrecisionValue(&self,v:ValueRef,s:i32,r:i32,c:i32)->EvalResult<ValueRef>{Ok(Self::getPrecisionValue(self,v,s,r,c))}fn writeDirty(&self,v:&ValueRef,u:i32,r:i32,c:i32){Self::writeDirty(self,v,u,r,c)}fn getUDFFinder(&self)->Option<Rc<dyn IUDFFinder>>{Self::getUDFFinder(self)}fn getUseRange(&self,s:i32)->Option<CellRangeAddress>{Self::getUseRange(self,s)}fn getBookType(&self)->i32{Self::getBookType(self)}
}
