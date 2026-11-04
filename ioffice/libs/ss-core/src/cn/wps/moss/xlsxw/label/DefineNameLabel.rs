// java package cn.wps.moss.xlsxw.label;
use crate::{java_runtime::{Rc,RefCell},cn::wps::{et::ss::{calcchain::FormulaHelper::FormulaHelper,formula::renderer::{FormulaRenderer::{FormulaRenderer,javaException},RendererOption::RendererOption}},moss::{app::{KmoBook::KmoBook,nameManager::KmoNameManager::KmoNameManager},engine::{eval::EvaluationWorkbook::EvaluationWorkbook,linktb::{LinkTable::LinkTable,NameRecord::NameRecordRef}},xlsxw::label::Label::Label}}};
use kfc_kfc_alpha::kfc::xpxml::writer::XmlWriter::XmlWriter;
use std::panic::{catch_unwind,AssertUnwindSafe,resume_unwind};
// java public final class DefineNameLabel implements Label {
pub struct DefineNameLabel{
    // java private KmoBook book;
    book:Rc<KmoBook>,
    // java private LinkTable linkTable;
    linkTable:Rc<RefCell<LinkTable>>,
}
#[cfg(test)]
#[path="DefineNameLabel.review.rs"]
mod review;
impl DefineNameLabel{
    // java public DefineNameLabel(KmoBook book){this.book=book;this.linkTable=book.getLinkTable();}
    pub fn DefineNameLabel(book:Option<Rc<KmoBook>>)->Self{let book=book.expect("NullPointerException");let linkTable=book.getLinkTable();Self{book,linkTable}}
    // java public void write(XmlWriter writer){
    pub fn write(&self,writer:&mut dyn XmlWriter){
        // java if(this.linkTable.getNumNames()>0){writer.startElement("definedNames");int index=0;
        if self.linkTable.borrow().getNumNames()>0{writer.startElement("definedNames");let mut index=0;
            // java for(int count=this.linkTable.getNumNames();index<count;++index){this.writeDefinedName(writer,this.linkTable.getNameRecord(index));}
            let count=self.linkTable.borrow().getNumNames();while index<count{let record=self.linkTable.borrow().getNameRecord(index);self.writeDefinedName(writer,record.as_ref());index=index.wrapping_add(1);}
            // java writer.endElement("definedNames");
            writer.endElement("definedNames");
        }
    }
    // java private void writeDefinedName(XmlWriter writer,NameRecord nameRec){
    fn writeDefinedName(&self,writer:&mut dyn XmlWriter,nameRec:Option<&NameRecordRef>){
        // java if(!nameRec.isFunctionName()){Ptg[] ptgs=nameRec.getNameDefinition();if(ptgs!=null&&ptgs.length!=0){String str=null;
        let nameRec=nameRec.expect("NullPointerException");if !nameRec.borrow().isFunctionName(){let ptgs=nameRec.borrow_mut().getNameDefinition();if !ptgs.borrow().is_empty(){
            // java try{str=FormulaRenderer.toFormulaString(new EvaluationWorkbook(this.book),FormulaHelper.convertFromExcel97ToExcel2007(ptgs,4,this.book.getSpreadsheetVersion()),RendererOption.createOOXMLRendererOption(this.book.getLinkTable().getExternalURLs()));}
            let result=catch_unwind(AssertUnwindSafe(||{let evaluation=EvaluationWorkbook::EvaluationWorkbook(&self.book);let converted=FormulaHelper::convertFromExcel97ToExcel2007(Some(ptgs.clone()),4,Some(self.book.getSpreadsheetVersion()));let urls=self.book.getLinkTable().borrow().getExternalURLs();let urls=Rc::new(RefCell::new(urls.into_iter().map(Some).collect::<Vec<_>>().into_boxed_slice()));let option=RendererOption::createOOXMLRendererOption(Some(urls));let value=FormulaRenderer::toFormulaString(Some(&evaluation),converted.as_ref(),Some(&option.borrow()));value}));
            // java catch(Exception e){System.out.println("在写workbook数据时 解析公式函数不能解析出String");e.printStackTrace();}
            let str=match result{Ok(value)=>value,Err(e)=>{if !javaException(e.as_ref()){resume_unwind(e);}eprintln!("在写workbook数据时 解析公式函数不能解析出String");None}};
            // java if(str!=null&&str.length()!=0){this.writeDefinedNameImpl(writer,nameRec,str);}
            if let Some(str)=str.filter(|s|!s.is_empty()){self.writeDefinedNameImpl(writer,Some(nameRec),&str);}
        }}
    }
    // java private void writeDefinedNameImpl(XmlWriter writer,NameRecord nameRec,String nameDefine){
    fn writeDefinedNameImpl(&self,writer:&mut dyn XmlWriter,nameRec:Option<&NameRecordRef>,nameDefine:&str){
        // java writer.startElement("definedName");String nameName=nameRec.getNameText();
        writer.startElement("definedName");let nameRec=nameRec.expect("NullPointerException");let mut nameName=nameRec.borrow().getNameText().to_string();
        // java if(nameRec.isBuiltInName()){switch(nameRec.getBuiltInName()){
        if nameRec.borrow().isBuiltInName(){match nameRec.borrow().getBuiltInName(){
            // java case 0:nameName="_xlnm.Consolidate_Area";case 1:case 2:case 8:case 9:case 10:case 11:default:break;
            0=>nameName="_xlnm.Consolidate_Area".into(),
            // java case 3:nameName="_xlnm.Extract";break;case 4:nameName="_xlnm.Database";break;case 5:nameName="_xlnm.Criteria";break;
            3=>nameName="_xlnm.Extract".into(),4=>nameName="_xlnm.Database".into(),5=>nameName="_xlnm.Criteria".into(),
            // java case 6:nameName="_xlnm.Print_Area";break;case 7:nameName="_xlnm.Print_Titles";break;case 12:nameName="_xlnm.Sheet_Title";break;case 13:nameName="_xlnm._FilterDatabase";
            6=>nameName="_xlnm.Print_Area".into(),7=>nameName="_xlnm.Print_Titles".into(),12=>nameName="_xlnm.Sheet_Title".into(),13=>nameName="_xlnm._FilterDatabase".into(),_=>{}
        }}
        // java int type=KmoNameManager.judgeValidName(nameName,this.book.getSpreadsheetVersion());if(0!=type){nameName=this.correctDefineName(nameName);}else{nameName=nameName.trim();}
        let r#type=KmoNameManager::judgeValidName(Some(&nameName),Some(self.book.getSpreadsheetVersion()));if r#type!=0{nameName=self.correctDefineName(&nameName);}else{nameName=nameName.trim_matches(|c:char|c<='\u{20}').into();}
        // java writer.addAttr("name",nameName);
        writer.addAttr("name",&nameName);
        // java if(nameRec.getSheetNumber()-1!=-1){writer.addAttr("localSheetId",nameRec.getSheetNumber()-1);}
        if nameRec.borrow().getSheetNumber().wrapping_sub(1)!=-1{let sheetNumber=nameRec.borrow().getSheetNumber().wrapping_sub(1);writer.addAttr__i32("localSheetId",sheetNumber);}
        // java if(nameRec.isHiddenName()){writer.addAttr("hidden",nameRec.isHiddenName());}
        if nameRec.borrow().isHiddenName(){let hidden=nameRec.borrow().isHiddenName();writer.addAttr__bool("hidden",hidden);}
        // java nameDefine=this.encodeLinkNameDefine(nameDefine);writer.addText(nameDefine);writer.endElement("definedName");
        let nameDefine=self.encodeLinkNameDefine(nameDefine);writer.addText(&nameDefine);writer.endElement("definedName");
    }
    // java private String correctDefineName(String nameName){nameName=nameName.replaceAll("[^A-z0-9\\._]","_");return nameName;}
    fn correctDefineName(&self,nameName:&str)->String{let nameName=regex::Regex::new(r"[^A-z0-9\._]").unwrap().replace_all(nameName,"_").into_owned();nameName}
    // java private String encodeLinkNameDefine(String nameDefine){return nameDefine;}
    fn encodeLinkNameDefine(&self,nameDefine:&str)->String{nameDefine.into()}
}
impl Label for DefineNameLabel{fn write(&self,writer:&mut dyn XmlWriter){Self::write(self,writer)}}
