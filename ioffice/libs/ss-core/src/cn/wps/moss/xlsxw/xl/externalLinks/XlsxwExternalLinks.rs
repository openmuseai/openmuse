use crate::{java_runtime::{Rc,RefCell,JavaObject::JavaObject},cn::wps::{moss::{app::KmoBook::KmoBook,engine::linktb::{LinkTable::{ExternalBookBlock,CRNBlock},SupBookRecord::SupBookRecord,CRNRecord::CRNRecord},xlsxw::util::{ValueHelper::ValueHelper,XlsxwPackage::XlsxwPackage}},et::ss::formula::{constant::ErrorConstant::ErrorConstant,ptg::Ptg::PtgKind}},org::apache::poi::ss::util::CellReference::CellReference};
use kfc_kfc_alpha::kfc::{opcw::{OpcwRelationships::OpcwRelationships,native::RelationRef},opcr::OpcTargetMode::OpcTargetMode,ooxfr::OxfXlRelations,xpxml::writer::{XmlWriter::XmlWriter,r#impl::BufferXmlWriter::BufferXmlWriter}};
// java public class XlsxwExternalLinks {
pub struct XlsxwExternalLinks{
    // java private KmoBook mBook;
    mBook:Rc<KmoBook>,
    // java private OpcwRelationships mRelses;
    mRelses:Option<std::rc::Rc<std::cell::RefCell<OpcwRelationships>>>,
    // java private List<String> rids = new ArrayList();
    rids:Rc<RefCell<Vec<String>>>,
}
#[cfg(test)]
#[path="XlsxwExternalLinks.review.rs"]
mod review;
impl XlsxwExternalLinks{
    // java public XlsxwExternalLinks(KmoBook book,OpcwRelationships relses){this.mBook=book;this.mRelses=relses;}
    pub fn XlsxwExternalLinks(book:Rc<KmoBook>,relses:Option<std::rc::Rc<std::cell::RefCell<OpcwRelationships>>>)->Self{Self{mBook:book,mRelses:relses,rids:Rc::new(RefCell::new(vec![]))}}
    // java public boolean write() {
    pub fn write(&self)->bool{
        // java KmoLog.logPoint();List<LinkTable.ExternalBookBlock> blocks=this.mBook.getLinkTable().getAllExternalBookBlocks();int index=0;int i=0;
        let blocks=self.mBook.getLinkTable().borrow().getAllExternalBookBlocks();let mut index=0;let mut i=0;
        // java for(int count=blocks.size();i<count;++i){
        let blocks=blocks.expect("NullPointerException");let count=blocks.borrow().len();while i<count{
            // java if(this.writeExternalLink((LinkTable.ExternalBookBlock)blocks.get(i),index)){++index;}
            let block=blocks.borrow()[i].clone();if self.writeExternalLink(block.as_deref(),index){index+=1;}i+=1;
        }
        // java KmoLog.logPoint();return true;
        true
    }
    // java public final List<String> getExternalIds(){return this.rids;}
    pub fn getExternalIds(&self)->Rc<RefCell<Vec<String>>>{self.rids.clone()}
    // java private boolean writeExternalLink(LinkTable.ExternalBookBlock bookLink,int index){
    fn writeExternalLink(&self,bookLink:Option<&ExternalBookBlock>,index:i32)->bool{
        // java if(bookLink==null){return false;}else if(bookLink.getExternalBookRecord()==null){return false;}else if(!bookLink.getExternalBookRecord().isExternalReferences()){return false;}else{
        let Some(bookLink)=bookLink else{return false;};let Some(record)=bookLink.getExternalBookRecord()else{return false;};if !record.isExternalReferences(){return false;}
        // java String targetUrl=bookLink.getExternalBookRecord().getURL();if(targetUrl!=null&&targetUrl.length()!=0&&!targetUrl.equals(" ")){
        let mut targetUrl=record.getURL();if !targetUrl.is_empty()&&targetUrl!=" "{
            // java targetUrl=targetUrl.replaceAll(" ","%20");OpcwRelationship rels=this.mRelses.addRelationship(OxfXlRelations.EXTERNAL_LINK,index+1);
            targetUrl=targetUrl.replace(' ',"%20");let rels=self.mRelses.as_ref().expect("NullPointerException").borrow_mut().addRelationship__2(RelationRef::Static(&*OxfXlRelations::EXTERNAL_LINK),index.wrapping_add(1));
            // java OpcwPackagePart linkPart=rels.createPart();OpcwRelationships relses2=linkPart.getRelationships();
            let linkPart=rels.borrow_mut().createPart();let relses2=linkPart.getRelationships();
            // java relses2.addRelationship(OxfXlRelations.EXTERNAL_LINK_PATH,targetUrl,OpcTargetMode.EXTERNAL);this.rids.add(rels.getRId());
            relses2.borrow_mut().addRelationship__4(RelationRef::Static(&*OxfXlRelations::EXTERNAL_LINK_PATH),Some(targetUrl),Some(OpcTargetMode::EXTERNAL));self.rids.borrow_mut().push(rels.borrow().getRId().into());
            // java OutputStream stream=XlsxwPackage.getOutputStream(linkPart);XmlWriter writer=new BufferXmlWriter(stream);
            let stream=linkPart.withBaseMut(XlsxwPackage::getOutputStream);let mut writer=BufferXmlWriter::new(Box::new(stream.expect("NullPointerException")));
            // java writer.startDocument();writer.startElement("externalLink");writer.addNS((String)null,"http://schemas.openxmlformats.org/spreadsheetml/2006/main");
            writer.startDocument();writer.startElement("externalLink");writer.addNS(None,"http://schemas.openxmlformats.org/spreadsheetml/2006/main");
            // java writer.startElement("externalBook");writer.addNS("r","http://schemas.openxmlformats.org/officeDocument/2006/relationships");writer.addAttr("r","id","rId1");
            writer.startElement("externalBook");writer.addNS(Some("r"),"http://schemas.openxmlformats.org/officeDocument/2006/relationships");writer.addAttr__ns(Some("r"),"id","rId1");
            // java this.writeSheetNames(writer,bookLink.getExternalBookRecord());this.writeDefinedNames(writer,bookLink);this.writeSheetDataSet(writer,bookLink);
            self.writeSheetNames(&mut writer,Some(&record));self.writeDefinedNames(&mut writer,Some(bookLink));self.writeSheetDataSet(&mut writer,Some(bookLink));
            // java writer.endElement("externalBook");writer.endElement("externalLink");writer.endDocument();return true;
            writer.endElement("externalBook");writer.endElement("externalLink");writer.endDocument();true
        // java }else{return false;}
        }else{false}
    }
    // java private void writeSheetNames(XmlWriter writer,SupBookRecord bookRec){
    fn writeSheetNames(&self,writer:&mut dyn XmlWriter,bookRec:Option<&SupBookRecord>){
        // java String[] sheetNames=bookRec.getSheetNames();if(sheetNames.length!=0){writer.startElement("sheetNames");
        let sheetNames=bookRec.expect("NullPointerException").getSheetNames();if !sheetNames.is_empty(){writer.startElement("sheetNames");
            // java for(String sheetName:sheetNames){writer.startElement("sheetName");writer.addAttr("val",sheetName);writer.endElement("sheetName");}
            for sheetName in sheetNames{writer.startElement("sheetName");writer.addAttrNullable(Some("val"),sheetName.as_deref());writer.endElement("sheetName");}
            // java writer.endElement("sheetNames");
            writer.endElement("sheetNames");
        }
    }
    // java private void writeDefinedNames(XmlWriter writer,LinkTable.ExternalBookBlock bookLink){
    fn writeDefinedNames(&self,writer:&mut dyn XmlWriter,bookLink:Option<&ExternalBookBlock>){
        // java if(bookLink.getNumberOfNames()>=1){writer.startElement("definedNames");for(int i=0;i<bookLink.getNumberOfNames();++i){
        let bookLink=bookLink.expect("NullPointerException");if bookLink.getNumberOfNames()>=1{writer.startElement("definedNames");let mut i=0;while i<bookLink.getNumberOfNames(){
            // java writer.startElement("definedName");ExternalNameRecord exName=bookLink.getExternalNameRecord(i);writer.addAttr("name",exName.getText());Ptg[] ptgs=exName.getParsedExpression();
            writer.startElement("definedName");let exName=bookLink.getExternalNameRecord(i);writer.addAttr("name",exName.getText());let ptgs=exName.getParsedExpression();
            // java if(ptgs!=null&&ptgs.length>0){if(ptgs[0].getSid()==58){
            if let Some(ptgs)=ptgs.filter(|p|!p.is_empty()){if ptgs[0].getSid()==58{
                // java int sheetIndex=((Ref3DExtPtg)ptgs[0]).getSheetIndex();String[] sheetNames=bookLink.getExternalBookRecord().getSheetNames();
                let PtgKind::Ref3DExt{sheet_index1,row,col_raw,..}=ptgs[0].kind else{panic!("ClassCastException")};let sheetIndex=sheet_index1 as i32;let sheetNames=bookLink.getExternalBookRecord().expect("NullPointerException").getSheetNames();
                // java if(sheetIndex>0&&sheetIndex<sheetNames.length){writer.addAttr("refersTo","='"+sheetNames[sheetIndex]+"'!$"+CellReference.convertNumToColString(((Ref3DExtPtg)ptgs[0]).getColumn())+"$"+(((Ref3DExtPtg)ptgs[0]).getRow()+1));}
                if sheetIndex>0&&(sheetIndex as usize)<sheetNames.len(){writer.addAttr("refersTo",&format!("='{}'!${}${}",sheetNames[sheetIndex as usize].as_deref().unwrap_or("null"),CellReference::convertNumToColString((col_raw&0x3fff)as i32),row.wrapping_add(1)));}
            // java }else if(ptgs[0].getSid()==59){writer.addAttr("refersTo",((Area3DExtPtg)ptgs[0]).getSheetIndex());}
            }else if ptgs[0].getSid()==59{let PtgKind::Area3DExt{sheet_index1,..}=ptgs[0].kind else{panic!("ClassCastException")};writer.addAttr__i32("refersTo",sheet_index1 as i32);}}
            // java if(exName.getIx()>0){writer.addAttr("sheetId",exName.getIx()-1);}writer.endElement("definedName");
            if exName.getIx()>0{writer.addAttr__i32("sheetId",exName.getIx()as i32-1);}writer.endElement("definedName");i+=1;
        }
        // java writer.endElement("definedNames");
        writer.endElement("definedNames");}
    }
    // java private void writeSheetDataSet(XmlWriter writer,LinkTable.ExternalBookBlock bookLink){
    fn writeSheetDataSet(&self,writer:&mut dyn XmlWriter,bookLink:Option<&ExternalBookBlock>){
        // java LinkTable.CRNBlock[] block=bookLink.getAllCRNBlock();if(block.length!=0){writer.startElement("sheetDataSet");for(int i=0;i<block.length;++i){this.writeSheetData(writer,block[i]);}writer.endElement("sheetDataSet");}
        let block=bookLink.expect("NullPointerException").getAllCRNBlock();if !block.borrow().is_empty(){writer.startElement("sheetDataSet");let mut i=0;while i<block.borrow().len(){let b=block.borrow()[i].clone();self.writeSheetData(writer,b.as_deref());i+=1;}writer.endElement("sheetDataSet");}
    }
    // java private void writeSheetData(XmlWriter writer,LinkTable.CRNBlock crnBlock){
    fn writeSheetData(&self,writer:&mut dyn XmlWriter,crnBlock:Option<&CRNBlock>){
        // java writer.startElement("sheetData");writer.addAttr("sheetId",crnBlock.getSheetId());int lastRow=-1;CRNRecord[] crnRecs=crnBlock.getCrns();
        writer.startElement("sheetData");let crnBlock=crnBlock.expect("NullPointerException");writer.addAttr__i32("sheetId",crnBlock.getSheetId());let mut lastRow=-1;let crnRecs=crnBlock.getCrns();
        // java for(CRNRecord crnRec:crnRecs){if(!this.isNullRow(crnRec)){
        for crnRec in crnRecs{if !self.isNullRow(crnRec.as_deref()){let crnRec=crnRec.as_ref().unwrap();
            // java if(lastRow!=-1){if(lastRow!=crnRec.getRowIndex()){lastRow=crnRec.getRowIndex();writer.endElement("row");writer.startElement("row");writer.addAttr("r",lastRow+1);}this.writeCells(writer,crnRec);
            if lastRow != -1{if lastRow!=crnRec.getRowIndex(){lastRow=crnRec.getRowIndex();writer.endElement("row");writer.startElement("row");writer.addAttr__i32("r",lastRow.wrapping_add(1));}self.writeCells(writer,crnRec);
            // java }else{lastRow=crnRec.getRowIndex();writer.startElement("row");writer.addAttr("r",lastRow+1);this.writeCells(writer,crnRec);}
            }else{lastRow=crnRec.getRowIndex();writer.startElement("row");writer.addAttr__i32("r",lastRow.wrapping_add(1));self.writeCells(writer,crnRec);}
        }}
        // java if(lastRow!=-1){writer.endElement("row");}writer.endElement("sheetData");
        if lastRow != -1{writer.endElement("row");}writer.endElement("sheetData");
    }
    // java private void writeCells(XmlWriter writer,CRNRecord crnRec){int row=crnRec.getRowIndex();int column=crnRec.getFirstColumn();for(int end=crnRec.getLastColumn();column<=end;++column){this.writCell(writer,row,column,crnRec.getConstantValue(column));}}
    fn writeCells(&self,writer:&mut dyn XmlWriter,crnRec:&CRNRecord){let row=crnRec.getRowIndex();let mut column=crnRec.getFirstColumn();let end=crnRec.getLastColumn();while column<=end{self.writCell(writer,row,column,crnRec.getConstantValue(column).as_deref());column=column.wrapping_add(1);}}
    // java private boolean isNullRow(CRNRecord crnRec){int column=crnRec.getFirstColumn();for(int end=crnRec.getLastColumn();column<=end;++column){if(crnRec.getConstantValue(column)!=null){return false;}}return true;}
    fn isNullRow(&self,crnRec:Option<&CRNRecord>)->bool{let crnRec=crnRec.expect("NullPointerException");let mut column=crnRec.getFirstColumn();let end=crnRec.getLastColumn();while column<=end{if crnRec.getConstantValue(column).is_some(){return false;}column=column.wrapping_add(1);}true}
    // java private void writCell(XmlWriter writer,int row,int column,Object obj){
    fn writCell(&self,writer:&mut dyn XmlWriter,row:i32,column:i32,obj:Option<&dyn JavaObject>){
        // java if(obj!=null){writer.startElement("cell");String tmp=CellReference.convertNumToColString(column);tmp=tmp+ValueHelper.toString(row+1);writer.addAttr("r",tmp);String type=this.getType(obj);if(type!=null){writer.addAttr("t",type);}writer.startElement("v");
        if let Some(obj)=obj{writer.startElement("cell");let mut tmp=CellReference::convertNumToColString(column);tmp+=&ValueHelper::toString(row.wrapping_add(1));writer.addAttr("r",&tmp);let r#type=self.getType(obj);if let Some(t)=r#type{writer.addAttr("t",t);}writer.startElement("v");
            // java if(type!=null&&!type.equals("str")){char ch=type.charAt(0);if(ch=='b'){Boolean b=(Boolean)obj;if(b){writer.addText("1");}else{writer.addText("0");}}else if(ch=='e'){ErrorConstant error=(ErrorConstant)obj;writer.addText(error.getText());}}
            if r#type.is_some_and(|t|t!="str"){let ch=r#type.unwrap().as_bytes()[0];if ch==b'b'{let b=obj.as_any().downcast_ref::<bool>().expect("ClassCastException");if *b{writer.addText("1");}else{writer.addText("0");}}else if ch==b'e'{let error=obj.as_any().downcast_ref::<ErrorConstant>().expect("ClassCastException");writer.addText(&error.getText());}
            // java }else{writer.addText(obj.toString());}writer.endElement("v");writer.endElement("cell");
            }else{writer.addText(&obj.toString());}writer.endElement("v");writer.endElement("cell");
        }
    }
    // java private String getType(Object obj){if(obj instanceof Double){return null;}else if(obj instanceof ErrorConstant){return "e";}else{return obj instanceof Boolean?"b":"str";}}
    fn getType(&self,obj:&dyn JavaObject)->Option<&'static str>{if obj.as_any().is::<f64>(){None}else if obj.as_any().is::<ErrorConstant>(){Some("e")}else{Some(if obj.as_any().is::<bool>(){"b"}else{"str"})}}
}
