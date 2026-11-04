// java package cn.wps.moss.engine.linktb;

use crate::java_runtime::Weak;

use crate::cn::wps::et::ss::formula::atp::AnalysisToolPak::AnalysisToolPak;
use crate::cn::wps::et::ss::formula::ptg::Ptg::{err_ptg, name_ptg, name_x_ptg, Ptg};
use crate::cn::wps::moss::app::KmoBook::KmoBook;
use crate::cn::wps::moss::engine::linktb::NameRecord::{NameRecord,NameRecordRef,IntoNameRecordRef};
use super::LinkInfo::LinkInfo;
use crate::cn::wps::moss::engine::linktb::ExternalNameRecord::ExternalNameRecord;

// java ExternSheetRecord.RefSubRecord
#[derive(Clone, Copy)]
pub struct RefSubRecord {
    ext_book: i32,
    first_sheet: i32,
    last_sheet: i32,
}

impl RefSubRecord {
    pub fn getExtBookIndex(&self) -> i32 {
        self.ext_book
    }
    pub fn getFirstSheetIndex(&self) -> i32 {
        self.first_sheet
    }
    pub fn getLastSheetIndex(&self) -> i32 {
        self.last_sheet
    }
}

// java public final class LinkTable
pub type ExternalBookBlocks=crate::java_runtime::Rc<crate::java_runtime::RefCell<Vec<Option<crate::java_runtime::Rc<ExternalBookBlock>>>>>;
pub struct LinkTable {
    book: Weak<KmoBook>,
    // java List<ExternalBookBlock> _externalBookBlocks
    blocks: Option<ExternalBookBlocks>,
    // java ExternSheetRecord _externSheetRecord
    refs: Vec<RefSubRecord>,
    // java private LinkInfo linkInfo = new LinkInfo();
    linkInfo: LinkInfo,
}

impl LinkTable {
    // java public LinkTable(int numberOfSheets, KmoBook book)
    pub fn LinkTable(number_of_sheets: i32, book: &Weak<KmoBook>) -> Self {
        let linkInfo=LinkInfo::LinkInfo();
        // java this._externalBookBlocks.add(new ExternalBookBlock(numberOfSheets));
        Self {
            book: book.clone(),
            blocks: Some(crate::java_runtime::Rc::new(crate::java_runtime::RefCell::new(vec![Some(crate::java_runtime::Rc::new(ExternalBookBlock::ExternalBookBlock__count(number_of_sheets)))]))),
            refs: Vec::new(),
            linkInfo,
        }
    }

    // java public int getNumNames()
    pub fn getNumNames(&self) -> i32 {
        self.linkInfo.getNamesCount()
    }

    // java public NameRecord getNameRecord(int index)
    pub fn getNameRecord(&self, index: i32) -> Option<NameRecordRef> {
        self.linkInfo.get(index)
    }
    // Native RuntimeException transport for original caller try/catch blocks.
    // Null NameRecord and an index exception remain distinct until that caller.
    pub(crate) fn tryNameRecord(&self,index:i32)->Result<Option<NameRecordRef>,()>{match std::panic::catch_unwind(std::panic::AssertUnwindSafe(||self.getNameRecord(index))){Ok(record)=>Ok(record),Err(e)=>{let s=e.downcast_ref::<String>().map(String::as_str).or_else(||e.downcast_ref::<&str>().copied()).unwrap_or("");if ["IndexOutOfBoundsException","NullPointerException","RuntimeException","IllegalStateException"].iter().any(|n|s.starts_with(n)){Err(())}else{std::panic::resume_unwind(e)}}}}

    // java public void addName(NameRecord nR)
    pub fn addName(&mut self, record: impl IntoNameRecordRef) {
        self.linkInfo.addNameInfo(record);
    }

    // java public Ptg getUDFName(String udfFuncName)
    pub fn getUDFName(&mut self, name: &str) -> Ptg {
        // java Ptg result = this.getUDFNameImpl(udfFuncName);
        if let Some(ptg) = self.get_udf_name_impl(name) {
            return ptg;
        }
        // java result = this.getUnKnowUDFName(udfFuncName);
        self.get_unknown_udf_name(name)
    }

    // java public Ptg checkUDFName(String udfFuncName)
    pub fn checkUDFName(&mut self, name: &str) -> Ptg {
        let name = Self::strip_xlfn(&name.to_ascii_uppercase());
        // java if (AnalysisToolPak.instance.isXLFN(udfFuncName))
        if AnalysisToolPak::isXLFN(&name) {
            let xlfn = format!("_xlfn.{name}");
            if let Some(index) = self.find_function_name(&xlfn) {
                return name_ptg(index);
            }
            self.add_hidden_function(&xlfn);
            return name_ptg(self.getNumNames() - 1);
        }
        // java int addInFuncExtBook = this.checkAddInFuncExternBook();
        let book_index = self.check_add_in_func_extern_book();
        let udf_index = self.index_of_external_name(book_index, &name);
        let udf_index = if udf_index < 0 {
            self.add_external_name(book_index, name)
        } else {
            udf_index
        };
        // java int extSheet = this._externSheetRecord.checkRefIx(addInFuncExtBook, -2, -2);
        let ext_sheet = self.check_ref_ix(book_index, -2, -2);
        name_x_ptg(ext_sheet, udf_index)
    }

    fn get_udf_name_impl(&mut self, name: &str) -> Option<Ptg> {
        let name = Self::strip_xlfn(&name.to_ascii_uppercase());
        if !AnalysisToolPak::isXLFN(&name) {
            return None;
        }
        let xlfn = format!("_xlfn.{name}");
        self.find_function_name(&xlfn).map(name_ptg)
    }

    fn get_unknown_udf_name(&mut self, name: &str) -> Ptg {
        let name = Self::strip_xlfn(&name.to_ascii_uppercase());
        let xlfn = format!("_xlfn.{name}");
        if let Some(index) = self.find_function_name(&xlfn) {
            return name_ptg(index);
        }
        self.add_hidden_function(&xlfn);
        name_ptg(self.getNumNames() - 1)
    }

    fn find_function_name(&self, xlfn: &str) -> Option<i32> {
        for index in 0..self.getNumNames() {
            let record=self.getNameRecord(index).expect("NullPointerException");let record=record.borrow();
            if record.isFunctionName() && record.getNameText().eq_ignore_ascii_case(xlfn) {
                return Some(index);
            }
        }
        None
    }

    fn add_hidden_function(&mut self, xlfn: &str) {
        // java NameRecord(book.getSpreadsheetVersion()); setNameDefinition(ErrPtg.NAME_INVALID)
        let sv = self
            .book
            .upgrade()
            .map(|book| book.getSpreadsheetVersion())
            .unwrap_or(crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion::EXCEL97);
        let mut record = NameRecord::NameRecord(sv);
        record.setNameText(xlfn);
        // java ErrPtg.NAME_INVALID error code 29
        record.setNameDefinition(vec![err_ptg(29)]);
        record.setMacro(true);
        record.setFunction(true);
        record.setHidden(true);
        self.linkInfo.addNameInfo(record);
    }

    fn check_add_in_func_extern_book(&mut self) -> i32 {
        for index in 0..self.blocks().borrow().len() {
            let book=self.block(index);
            let record=book.getExternalBookRecord().expect("NullPointerException");
            if record.isAddInFunctions() && !record.isNullSupBook() {
                return index as i32;
            }
        }
        self.blocks().borrow_mut().push(Some(crate::java_runtime::Rc::new(ExternalBookBlock::ExternalBookBlock__0())));
        self.blocks().borrow().len() as i32 - 1
    }

    fn index_of_external_name(&self, book_index: i32, name: &str) -> i32 {
        let blocks=self.blocks().borrow();
        let Some(book) = blocks.get(book_index as usize) else {
            return -1;
        };
        book.as_ref().expect("NullPointerException").getIndexOfName(Some(name))
    }

    fn add_external_name(&mut self, book_index: i32, name: String) -> i32 {
        let book = self.block(book_index as usize);
        // java ExternalNameRecord eNR = new ExternalNameRecord();
        let mut record = ExternalNameRecord::ExternalNameRecord();
        // java eNR.setText(udfFuncName);
        record.setText(&name);
        // java eNR.setParsedExpression(new Ptg[]{ErrPtg.REF_INVALID});
        record.setParsedExpression(vec![err_ptg(23)]);
        // java udfNameIndex = ebb.addExternalName(eNR);
        book.addExternalName(crate::java_runtime::Rc::new(record))
    }

    // java ExternalBookBlock.getExternalNameRecord(int ix)
    pub fn getExternalNameRecord(&self, ext_book: i32, index: i32) -> Option<crate::java_runtime::Rc<ExternalNameRecord>> {
        let blocks=self.blocks().borrow();let book=blocks.get(ext_book as usize)?.as_ref().expect("NullPointerException");if index<0||index>=book.getNumberOfNames(){None}else{Some(book.getExternalNameRecord(index))}
    }

    fn check_ref_ix(&mut self, ext_book: i32, first: i32, last: i32) -> i32 {
        let found = self.get_ref_ix(ext_book, first);
        if found >= 0 && self.refs[found as usize].last_sheet == last {
            found
        } else {
            self.add_ref(ext_book, first, last)
        }
    }

    // java public int checkNullSupBookExternSheet(int sheetIndex)
    pub fn checkNullSupBookExternSheet(&mut self, sheet_index: i32) -> i32 {
        // java int thisWbIndex = this.findNullSupBookIndex();
        let mut this_wb = self.find_null_sup_book();
        // java if (thisWbIndex < 0)
        if this_wb < 0 {
            // java this._externalBookBlocks.add(new ExternalBookBlock(this.book.sheetCount()));
            let count = self
                .book
                .upgrade()
                .map(|book| book.sheetCount())
                .unwrap_or(0);
            self.blocks().borrow_mut().push(Some(crate::java_runtime::Rc::new(ExternalBookBlock::ExternalBookBlock__count(count))));
            // java thisWbIndex = this.findNullSupBookIndex();
            this_wb = self.find_null_sup_book();
        }
        // java int i = this._externSheetRecord.getRefIxForSheet(thisWbIndex, sheetIndex);
        let found = self.get_ref_ix(this_wb, sheet_index);
        // java return i >= 0 ? i : this._externSheetRecord.addRef(...)
        if found >= 0 {
            found
        } else {
            self.add_ref(this_wb, sheet_index, sheet_index)
        }
    }

    // java public ExternSheetRecord.RefSubRecord resolveExtSheetIndex(int extSheetIndex)
    pub fn resolveExtSheetIndex(&self, ext_sheet_index: i32) -> Option<RefSubRecord> {
        // java return this._externSheetRecord.getRef(extSheetIndex);
        self.refs.get(ext_sheet_index as usize).copied()
    }

    pub fn isNullSupBook(&self, ext_book: i32) -> bool {
        self.blocks().borrow()
            .get(ext_book as usize)
            .map(|book| book.as_ref().expect("NullPointerException").getExternalBookRecord().expect("NullPointerException").isNullSupBook())
            .unwrap_or(false)
    }

    // java SupBookRecord.isExternalReferences()
    pub fn isExternalReferences(&self, ext_book: i32) -> bool {
        self.blocks().borrow().get(ext_book as usize)
            .map(|book| book.as_ref().expect("NullPointerException").getExternalBookRecord().expect("NullPointerException").isExternalReferences())
            .unwrap_or(false)
    }

    pub fn isAddInFunctions(&self, ext_book: i32) -> bool {
        self.blocks().borrow()
            .get(ext_book as usize)
            .map(|book| book.as_ref().expect("NullPointerException").getExternalBookRecord().expect("NullPointerException").isAddInFunctions())
            .unwrap_or(false)
    }

    // java SupBookRecord.getURL()
    pub fn getExternalBookUrl(&self, ext_book: i32) -> Option<String> {
        self.blocks().borrow().get(ext_book as usize)?.as_ref().expect("NullPointerException").getExternalBookRecord().map(|r|r.getURL())
    }

    // java SupBookRecord.getSheetNames()[sheetIndex]
    pub fn getExternalSheetName(&self, ext_book: i32, sheet_index: i32) -> Option<String> {
        self.blocks().borrow().get(ext_book as usize)?.as_ref().expect("NullPointerException").getExternalBookRecord()?.getSheetNames().get(sheet_index as usize).cloned().flatten()
    }

    fn find_null_sup_book(&self) -> i32 {
        // java for each ExternalBookBlock, ebr.isNullSupBook()
        for index in 0..self.blocks().borrow().len() {
            let book=self.block(index);
            if book.getExternalBookRecord().expect("NullPointerException").isNullSupBook() {
                return index as i32;
            }
        }
        -1
    }

    fn get_ref_ix(&self, ext_book: i32, sheet_index: i32) -> i32 {
        for (index, rec) in self.refs.iter().enumerate() {
            if rec.ext_book == ext_book
                && rec.first_sheet == sheet_index
                && rec.last_sheet == sheet_index
            {
                return index as i32;
            }
        }
        -1
    }

fn strip_xlfn(name: &str) -> String {
    if let Some(rest) = name.strip_prefix("_XLFN.") {
        rest.to_string()
    } else {
        name.to_string()
    }
}

    fn add_ref(&mut self, ext_book: i32, first: i32, last: i32) -> i32 {
        self.refs.push(RefSubRecord {
            ext_book,
            first_sheet: first,
            last_sheet: last,
        });
        self.refs.len() as i32 - 1
    }
}

// java public static final class CRNBlock {
// The original binary RecordInputStream constructor remains pending that reader.
pub struct CRNBlock{
    // java private final CRNCountRecord _countRecord;
    _countRecord:super::CRNCountRecord::CRNCountRecord,
    // java private final CRNRecord[] _crns;
    _crns:Vec<Option<crate::java_runtime::Rc<super::CRNRecord::CRNRecord>>>,
}
impl CRNBlock{
    // java private CRNBlock(int sheetId,List<CRNRecord> crnList){this._countRecord=new CRNCountRecord(sheetId,crnList.size());this._crns=new CRNRecord[crnList.size()];crnList.toArray(this._crns);}
    fn CRNBlock(sheetId:i32,crnList:Option<&[Option<crate::java_runtime::Rc<super::CRNRecord::CRNRecord>>]>)->Self{let crnList=crnList.expect("NullPointerException");let _countRecord=super::CRNCountRecord::CRNCountRecord::CRNCountRecord__2(sheetId,crnList.len()as i32);let _crns=crnList.to_vec();Self{_countRecord,_crns}}
    // java public static CRNBlock createCRNBlock(int sheetId,List<CRNRecord> crns){return new CRNBlock(sheetId,crns);}
    pub fn createCRNBlock(sheetId:i32,crns:Option<&[Option<crate::java_runtime::Rc<super::CRNRecord::CRNRecord>>]>)->Self{Self::CRNBlock(sheetId,crns)}
    // java public void serialize(LittleEndianOutput out){this._countRecord.serialize(out);int i=0;for(int len=this._crns.length;i<len;++i){this._crns[i].serialize(out);}}
    pub fn serialize(&self,out:&mut dyn util::org::apache::poi::util::LittleEndianOutput::LittleEndianOutput){use crate::cn::wps::moss::filefmt::biff8::record::StandardRecord::StandardRecord;self._countRecord.serialize(out);let mut i=0;let len=self._crns.len();while i<len{self._crns[i].as_ref().expect("NullPointerException").serialize(out);i+=1;}}
    // java public CRNRecord[] getCrns(){return (CRNRecord[])this._crns.clone();}
    pub fn getCrns(&self)->Vec<Option<crate::java_runtime::Rc<super::CRNRecord::CRNRecord>>>{self._crns.clone()}
    // java public int getSheetId(){return this._countRecord.getSheetTableIndex();}
    pub fn getSheetId(&self)->i32{self._countRecord.getSheetTableIndex()}
}

// java public static final class ExternalBookBlock {
// Binary RecordInputStream/ExternalNameRecord serialization overloads await
// their original record dependencies; the XML/cache constructors are real.
pub struct ExternalBookBlock{
    // java private SupBookRecord _externalBookRecord;
    _externalBookRecord:Option<crate::java_runtime::Rc<super::SupBookRecord::SupBookRecord>>,
    // java private List<ExternalNameRecord> _externalNameRecords;
    _externalNameRecords:crate::java_runtime::RefCell<Vec<crate::java_runtime::Rc<ExternalNameRecord>>>,
    // java private final CRNBlock[] _crnBlocks;
    _crnBlocks:crate::java_runtime::Rc<crate::java_runtime::RefCell<Vec<Option<crate::java_runtime::Rc<CRNBlock>>>>>,
}
impl ExternalBookBlock{
    // java public ExternalBookBlock(SupBookRecord bookRec,List<CRNBlock> crnBlockList){this._externalBookRecord=bookRec;this._externalNameRecords=new ArrayList(2);if(crnBlockList!=null){this._crnBlocks=new CRNBlock[crnBlockList.size()];crnBlockList.toArray(this._crnBlocks);}else{this._crnBlocks=new CRNBlock[0];}}
    pub fn ExternalBookBlock(bookRec:Option<crate::java_runtime::Rc<super::SupBookRecord::SupBookRecord>>,crnBlockList:Option<&[Option<crate::java_runtime::Rc<CRNBlock>>]>)->Self{Self{_externalBookRecord:bookRec,_externalNameRecords:crate::java_runtime::RefCell::new(Vec::with_capacity(2)),_crnBlocks:crate::java_runtime::Rc::new(crate::java_runtime::RefCell::new(crnBlockList.map_or_else(Vec::new,|v|v.to_vec())))}}
    // java public ExternalBookBlock(int numberOfSheets){this._externalBookRecord=SupBookRecord.createNullSupBook((short)numberOfSheets);this._externalNameRecords=new ArrayList(2);this._crnBlocks=new CRNBlock[0];}
    pub fn ExternalBookBlock__count(numberOfSheets:i32)->Self{Self{_externalBookRecord:Some(crate::java_runtime::Rc::new(super::SupBookRecord::SupBookRecord::createNullSupBook(numberOfSheets as i16))),_externalNameRecords:crate::java_runtime::RefCell::new(Vec::with_capacity(2)),_crnBlocks:crate::java_runtime::Rc::new(crate::java_runtime::RefCell::new(vec![]))}}
    // java public ExternalBookBlock(){this._externalBookRecord=SupBookRecord.createAddInFunctions();this._externalNameRecords=new ArrayList(2);this._crnBlocks=new CRNBlock[0];}
    pub fn ExternalBookBlock__0()->Self{Self{_externalBookRecord:Some(crate::java_runtime::Rc::new(super::SupBookRecord::SupBookRecord::createAddInFunctions())),_externalNameRecords:crate::java_runtime::RefCell::new(Vec::with_capacity(2)),_crnBlocks:crate::java_runtime::Rc::new(crate::java_runtime::RefCell::new(vec![]))}}
    // java public SupBookRecord getExternalBookRecord(){return this._externalBookRecord;}
    pub fn getExternalBookRecord(&self)->Option<crate::java_runtime::Rc<super::SupBookRecord::SupBookRecord>>{self._externalBookRecord.clone()}
    // java public String getNameText(int definedNameIndex){return this._externalNameRecords.get(definedNameIndex).getText();}
    pub fn getNameText(&self,definedNameIndex:i32)->String{self._externalNameRecords.borrow()[definedNameIndex as usize].getText().into()}
    // java public int getNameIx(int definedNameIndex){return this._externalNameRecords.get(definedNameIndex).getIx();}
    pub fn getNameIx(&self,definedNameIndex:i32)->i32{self._externalNameRecords.borrow()[definedNameIndex as usize].getIx()as i32}
    // java public ExternalNameRecord getExternalNameRecord(int ix){return this._externalNameRecords.get(ix);}
    pub fn getExternalNameRecord(&self,ix:i32)->crate::java_runtime::Rc<ExternalNameRecord>{self._externalNameRecords.borrow()[ix as usize].clone()}
    // java public CRNBlock getCRNBlock(int sheetIndex){int i=0;for(int len=this._crnBlocks.length;i<len;++i){CRNBlock b=this._crnBlocks[i];if(b._countRecord.getSheetTableIndex()==sheetIndex){return b;}}return null;}
    pub fn getCRNBlock(&self,sheetIndex:i32)->Option<crate::java_runtime::Rc<CRNBlock>>{let mut i=0;let len=self._crnBlocks.borrow().len();while i<len{let b=self._crnBlocks.borrow()[i].clone().expect("NullPointerException");if b._countRecord.getSheetTableIndex()==sheetIndex{return Some(b);}i+=1;}None}
    // java public CRNBlock[] getAllCRNBlock(){return this._crnBlocks;}
    pub fn getAllCRNBlock(&self)->crate::java_runtime::Rc<crate::java_runtime::RefCell<Vec<Option<crate::java_runtime::Rc<CRNBlock>>>>>{self._crnBlocks.clone()}
    // java public int getNumberOfBlock(){return this._crnBlocks.length;}
    pub fn getNumberOfBlock(&self)->i32{self._crnBlocks.borrow().len()as i32}
    // java public int getIndexOfName(String name){int i=0;for(int len=this._externalNameRecords.size();i<len;++i){if(this._externalNameRecords.get(i).getText().equalsIgnoreCase(name)){return i;}}return -1;}
    pub fn getIndexOfName(&self,name:Option<&str>)->i32{let mut i=0;let len=self._externalNameRecords.borrow().len();while i<len{let equal=name.is_some_and(|n|crate::java_runtime::Character::equalsIgnoreCase(&self._externalNameRecords.borrow()[i].getText().encode_utf16().collect::<Vec<_>>(),&n.encode_utf16().collect::<Vec<_>>()));if equal{return i as i32;}i+=1;}-1}
    // java public int getNumberOfNames(){return this._externalNameRecords.size();}
    pub fn getNumberOfNames(&self)->i32{self._externalNameRecords.borrow().len()as i32}
    // java public int addExternalName(ExternalNameRecord rec){this._externalNameRecords.add(rec);return this._externalNameRecords.size()-1;}
    pub fn addExternalName(&self,rec:crate::java_runtime::Rc<ExternalNameRecord>)->i32{self._externalNameRecords.borrow_mut().push(rec);self._externalNameRecords.borrow().len()as i32-1}
}
impl LinkTable{
    // java public ExternalBookBlock getExternalBookBlock(String bookname){int i=0;for(int len=this._externalBookBlocks.size();i<len;++i){SupBookRecord sbr=this._externalBookBlocks.get(i).getExternalBookRecord();if(sbr.isExternalReferences()&&sbr.getURL().equalsIgnoreCase(bookname)){return this._externalBookBlocks.get(i);}}return null;}
    pub fn getExternalBookBlock(&self,bookname:Option<&str>)->Option<crate::java_runtime::Rc<ExternalBookBlock>>{let mut i=0;let len=self.blocks().borrow().len();while i<len{let sbr=self.block(i).getExternalBookRecord().expect("NullPointerException");if sbr.isExternalReferences()&&bookname.is_some_and(|n|crate::java_runtime::Character::equalsIgnoreCase(&sbr.getURL().encode_utf16().collect::<Vec<_>>(),&n.encode_utf16().collect::<Vec<_>>())){return Some(self.block(i));}i+=1;}None}
    // java public ExternalBookBlock getExternalBookBlock(int extBookIndex){return this._externalBookBlocks.get(extBookIndex);}
    pub fn getExternalBookBlock__index(&self,extBookIndex:i32)->crate::java_runtime::Rc<ExternalBookBlock>{self.block(extBookIndex as usize)}
    // java public int getExternalBookIndex(String bookName){int i=0;for(int len=this._externalBookBlocks.size();i<len;++i){SupBookRecord sbr=this._externalBookBlocks.get(i).getExternalBookRecord();if(sbr.isExternalReferences()&&sbr.getURL().equalsIgnoreCase(bookName)){return i;}}return -1;}
    pub fn getExternalBookIndex(&self,bookName:Option<&str>)->i32{let mut i=0;let len=self.blocks().borrow().len();while i<len{let sbr=self.block(i).getExternalBookRecord().expect("NullPointerException");if sbr.isExternalReferences()&&bookName.is_some_and(|n|crate::java_runtime::Character::equalsIgnoreCase(&sbr.getURL().encode_utf16().collect::<Vec<_>>(),&n.encode_utf16().collect::<Vec<_>>())){return i as i32;}i+=1;}-1}
    // java public int checkExternalSheetIndex(String workbookName,String sheetName) {
    pub fn checkExternalSheetIndex(&mut self,workbookName:Option<&str>,sheetName:Option<&str>)->i32{
        // java SupBookRecord ebrTarget=null;int externalBookIndex=-1;int i=0;for(int len=this._externalBookBlocks.size();i<len;++i){SupBookRecord ebr=this._externalBookBlocks.get(i).getExternalBookRecord();if(ebr.isExternalReferences()&&workbookName.equals(ebr.getURL())){ebrTarget=ebr;externalBookIndex=i;break;}}
        let mut ebrTarget=None;let mut externalBookIndex=-1;let mut i=0;let len=self.blocks().borrow().len();while i<len{let ebr=self.block(i).getExternalBookRecord().expect("NullPointerException");if ebr.isExternalReferences()&&workbookName.expect("NullPointerException")==ebr.getURL(){ebrTarget=Some(ebr);externalBookIndex=i as i32;break;}i+=1;}
        // java if(ebrTarget==null){return -1;}else{i=getSheetIndex(ebrTarget.getSheetNames(),sheetName);return i<0?-1:this._externSheetRecord.checkRefIx(externalBookIndex,i,i);}
        if let Some(ebrTarget)=ebrTarget{let sheets=ebrTarget.getSheetNames();let mut index=-1;for (j,s) in sheets.iter().enumerate(){if sheetName.is_some_and(|n|crate::java_runtime::Character::equalsIgnoreCase(&s.as_ref().expect("NullPointerException").encode_utf16().collect::<Vec<_>>(),&n.encode_utf16().collect::<Vec<_>>())){index=j as i32;break;}}if index<0{-1}else{self.check_ref_ix(externalBookIndex,index,index)}}else{-1}
    }
    // java public void resetExternalLink(ExternalBookBlock[] extBooks){this._externalBookBlocks=new ArrayList(extBooks.length);int i=0;for(int count=extBooks.length;i<count;++i){this._externalBookBlocks.add(extBooks[i]);}i=0;for(int count=extBooks.length;i<count;++i){this._externSheetRecord.addRef(i,0,0);}}
    pub fn resetExternalLink(&mut self,extBooks:Option<&[crate::java_runtime::Rc<ExternalBookBlock>]>){let extBooks=extBooks.expect("NullPointerException");self.blocks=Some(crate::java_runtime::Rc::new(crate::java_runtime::RefCell::new(Vec::with_capacity(extBooks.len()))));let mut i=0;let count=extBooks.len();while i<count{self.blocks().borrow_mut().push(Some(extBooks[i].clone()));i+=1;}i=0;let count=extBooks.len();while i<count{self.add_ref(i as i32,0,0);i+=1;}}
    // java public List<ExternalBookBlock> getAllExternalBookBlocks() { return this._externalBookBlocks; }
    pub fn getAllExternalBookBlocks(&self)->Option<ExternalBookBlocks>{self.blocks.clone()}
    // java public String[] getExternalURLs(){List<String> targetUrls=new ArrayList();int i=0;for(int count=this._externalBookBlocks.size();i<count;++i){SupBookRecord supBookRec=this._externalBookBlocks.get(i).getExternalBookRecord();if(supBookRec!=null&&supBookRec.isExternalReferences()){String targetUrl=supBookRec.getURL();if(targetUrl!=null&&targetUrl.length()!=0&&!targetUrl.equals(" ")){targetUrls.add(targetUrl);}}}String[] extTargetUrls=new String[targetUrls.size()];return targetUrls.toArray(extTargetUrls);}
    pub fn getExternalURLs(&self)->Vec<String>{let mut targetUrls=vec![];let mut i=0;let count=self.blocks().borrow().len();while i<count{let supBookRec=self.block(i).getExternalBookRecord();if let Some(supBookRec)=supBookRec{if supBookRec.isExternalReferences(){let targetUrl=supBookRec.getURL();if !targetUrl.is_empty()&&targetUrl!=" "{targetUrls.push(targetUrl);}}}i+=1;}targetUrls}
    // java public void resetExternalBookBlocks(List<ExternalBookBlock> extBookRecord) { this._externalBookBlocks = extBookRecord; }
    pub fn resetExternalBookBlocks(&mut self,extBookRecord:Option<ExternalBookBlocks>){self.blocks=extBookRecord;}
    // java public void resetExternalLinks(ExternalBookBlock[] extBooks){int i=0;for(int count=extBooks.length;i<count;++i){this._externalBookBlocks.add(extBooks[i]);}}
    pub fn resetExternalLinks(&mut self,extBooks:Option<&[crate::java_runtime::Rc<ExternalBookBlock>]>){let mut i=0;let extBooks=extBooks.expect("NullPointerException");let count=extBooks.len();while i<count{self.blocks().borrow_mut().push(Some(extBooks[i].clone()));i+=1;}}
    // Native reference adapter for the existing weak book field.
    pub(crate) fn ownerBook(&self)->Option<crate::java_runtime::Rc<KmoBook>>{self.book.upgrade()}
    // Native dereference adapters; the original List and its nullable entries retain identity.
    fn blocks(&self)->&crate::java_runtime::RefCell<Vec<Option<crate::java_runtime::Rc<ExternalBookBlock>>>>{self.blocks.as_ref().expect("NullPointerException")}
    fn block(&self,index:usize)->crate::java_runtime::Rc<ExternalBookBlock>{self.blocks().borrow()[index].clone().expect("NullPointerException")}
}
#[cfg(test)]
#[path="LinkTable.review.rs"]
pub(crate) mod review;
