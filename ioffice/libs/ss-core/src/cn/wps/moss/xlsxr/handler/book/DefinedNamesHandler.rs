use crate::{java_runtime::{Rc,RefCell},cn::wps::{moss::{app::KmoBook::KmoBook,engine::{eval::EvaluationWorkbook::EvaluationWorkbook,linktb::LinkTable::LinkTable}},et::ss::formula::parser::{FormulaParser,ParserOption::ParserOption}}};
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
use super::DefinedNameHandler::{DefinedNameHandler,NameRecordCache};
use std::panic::{catch_unwind,AssertUnwindSafe};
// java public class DefinedNamesHandler extends DefaultElementHandler {
pub struct DefinedNamesHandler{
    // java private LinkTable linkTable;
    linkTable:Rc<RefCell<LinkTable>>,
    // java private KmoBook mBook;
    mBook:Rc<KmoBook>,
    // java private List<DefinedNameHandler.NameRecordCache> mRecordCaches;
    mRecordCaches:RefCell<Vec<Rc<RefCell<NameRecordCache>>>>,
}
// java private static final ParserOption parserOption=ParserOption.createOOXMLParserOption();
static parserOption:std::sync::LazyLock<ParserOption>=std::sync::LazyLock::new(ParserOption::createOOXMLParserOption);
impl DefinedNamesHandler{
    // java public DefinedNamesHandler(KmoBook book) {
    pub fn DefinedNamesHandler(book:Rc<KmoBook>)->Self{
        // java this.mBook=book; this.linkTable=this.mBook.getLinkTable(); this.mRecordCaches=new ArrayList();
        let linkTable=book.getLinkTable();Self{mBook:book,linkTable,mRecordCaches:RefCell::new(vec![])}
    }
    // java public ElementHandler enterSubElement(int subElement) {
    pub fn enterSubElement(&self,subElement:i32)->Option<Box<dyn ElementHandler>>{
        // java if(subElement==5202){
        if subElement==5202{
            // java NameRecordCache recordCache=new NameRecordCache();
            let recordCache=Rc::new(RefCell::new(NameRecordCache::new()));
            // java this.mRecordCaches.add(recordCache);
            self.mRecordCaches.borrow_mut().push(recordCache.clone());
            // java return new DefinedNameHandler(recordCache);
            Some(Box::new(DefinedNameHandler::DefinedNameHandler(recordCache)))
        }else{
            // java return null;
            None
        }
    }
    // java public void endElement(int element) {
    pub fn endElement(&self,_element:i32){
        // java boolean hasRecordUnImported=true;
        let mut hasRecordUnImported=true;
        // java while(hasRecordUnImported){
        while hasRecordUnImported{
            // java hasRecordUnImported=false; int i=0;
            hasRecordUnImported=false;let mut i=0;
            // java for(int count=this.mRecordCaches.size();i<count;++i){
            let count=self.mRecordCaches.borrow().len();while i<count{
                // java NameRecordCache recordCache=this.mRecordCaches.get(i);
                let recordCache=self.mRecordCaches.borrow()[i].clone();
                // java if(!recordCache.bImported){
                if !recordCache.borrow().bImported{
                    // java NameRecord nameRec=recordCache.nameRec;
                    let nameRec=recordCache.borrow().nameRec.clone();
                    // java try {Ptg[] ptgs=FormulaParser.parse(recordCache.formula,new EvaluationWorkbook(this.mBook),7,nameRec.getSheetNumber()-1,this.mBook.getSpreadsheetVersion(),parserOption); nameRec.setNameDefinition(ptgs);}
                    let parsed=catch_unwind(AssertUnwindSafe(||{
                        let formula=recordCache.borrow().formula.clone().expect("NullPointerException");
                        let nameRec=nameRec.as_ref().expect("NullPointerException");
                        let sheet=nameRec.borrow().getSheetNumber().wrapping_sub(1);
                        let ptgs=FormulaParser::parse(&formula,&EvaluationWorkbook::EvaluationWorkbook(&self.mBook),7,sheet,self.mBook.getSpreadsheetVersion(),&parserOption)?;
                        nameRec.borrow_mut().setNameDefinition(ptgs);
                        Ok::<(),FormulaParser::FormulaParseException>(())
                    }));
                    // java catch(FormulaParseException e){e.printStackTrace();continue;} catch(Throwable e){e.printStackTrace();continue;}
                    match parsed{Ok(Ok(()))=>{},Ok(Err(e))=>{eprintln!("FormulaParseException: {}",e.message);i+=1;continue;},Err(e)=>{eprintln!("DefinedNamesHandler caught Throwable: {:?}",e.type_id());i+=1;continue;}}
                    // java hasRecordUnImported=true; recordCache.bImported=true; this.linkTable.addName(nameRec);
                    hasRecordUnImported=true;recordCache.borrow_mut().bImported=true;self.linkTable.borrow_mut().addName(nameRec);
                }
                i+=1;
            }
        }
    }
}
impl ElementHandler for DefinedNamesHandler{
    fn startElement(&self,_e:i32,_a:&dyn XmlRoAttr){}
    fn endElement(&self,e:i32){self.endElement(e);}
    fn characters(&self,_s:&str){}
    fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{self.enterSubElement(e)}
}
