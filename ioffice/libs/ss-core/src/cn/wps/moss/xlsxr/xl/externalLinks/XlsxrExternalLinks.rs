use crate::{java_runtime::{Rc,RefCell},cn::wps::moss::{app::KmoBook::KmoBook,engine::linktb::LinkTable::ExternalBookBlock,xlsxr::{handler::externalLink::ExternalLinkHandler::ExternalLinkHandler,util::UnifiedXmlReader::UnifiedXmlReader}}};
use kfc_kfc_alpha::kfc::opcr::OpcrRelationships::OpcrRelationships;
use std::{collections::HashMap,io};
use kfc_kfc_alpha::kfc::opcr::OpcrPackagePart::OpcrPackagePart;
// Native reference transport: retain the owner of the same cached relationships.
enum RelationshipsRef<'a>{Borrowed(&'a mut OpcrRelationships),PartOwner(Rc<RefCell<Option<OpcrPackagePart>>>)}
impl RelationshipsRef<'_>{fn with_mut<T>(&mut self,f:impl FnOnce(&mut OpcrRelationships)->io::Result<T>)->io::Result<T>{match self{Self::Borrowed(r)=>f(r),Self::PartOwner(p)=>{let mut p=p.borrow_mut();f(p.as_mut().expect("NullPointerException").getRelationships_mut()?.expect("NullPointerException"))}}}}
// java public final class XlsxrExternalLinks {
pub struct XlsxrExternalLinks<'a>{
    // java private KmoBook mBook;
    mBook:Rc<KmoBook>,
    // java private OpcrRelationships mRelses;
    mRelses:Option<RelationshipsRef<'a>>,
    // java private List<String> mRids;
    mRids:Option<Rc<RefCell<Vec<Option<String>>>>>,
    // java private Map<String,LinkTable.ExternalBookBlock> mExtBookMap;
    mExtBookMap:HashMap<Option<String>,Option<Rc<ExternalBookBlock>>>,
}
impl<'a>XlsxrExternalLinks<'a>{
    // java public XlsxrExternalLinks(KmoBook book,OpcrRelationships relses,List<String> rids){this.mBook=book;this.mRelses=relses;this.mRids=rids;this.mExtBookMap=new HashMap();}
    pub fn XlsxrExternalLinks(book:Rc<KmoBook>,relses:Option<&'a mut OpcrRelationships>,rids:Option<Rc<RefCell<Vec<Option<String>>>>>)->Self{Self{mBook:book,mRelses:relses.map(RelationshipsRef::Borrowed),mRids:rids,mExtBookMap:HashMap::new()}}
    // java public void open()throws IOException{for(String rid:this.mRids){this.openExternalLink(rid);}this.addExternalLinkToCore();}
    pub fn open(&mut self)->io::Result<()>{let rids=self.mRids.clone().expect("NullPointerException");for rid in rids.borrow().iter(){self.openExternalLink(rid.as_deref())?;}self.addExternalLinkToCore();Ok(())}
    // java private void openExternalLink(String rid)throws IOException{
    fn openExternalLink(&mut self,rid:Option<&str>)->io::Result<()>{
        let mExtBookMap=&mut self.mExtBookMap;
        self.mRelses.as_mut().expect("NullPointerException").with_mut(|mRelses|{
        // java OpcrRelationship rels=this.mRelses.getRelationship(rid);if(rels!=null){OpcrPackagePart part=rels.getPackagePart();if(part!=null){OpcrRelationships relses=part.getRelationships();if(relses!=null){OpcrRelationship rId1Rels=relses.getRelationship("rId1");if(rId1Rels!=null){String targetUri=rId1Rels.getTarget();if(targetUri!=null){
        if let Some(rels)=mRelses.getRelationship__rid_mut(rid){if let Some(part)=rels.getPackagePart_mut(){if let Some(relses)=part.getRelationships()?{if let Some(rId1Rels)=relses.getRelationship__rid("rId1"){if let Some(mut targetUri)=rId1Rels.getTarget(){
            // java targetUri=targetUri.replaceAll("%20"," ");InputStream stream=part.getInputStream();ExternalLinkHandler handler=new ExternalLinkHandler(targetUri);UnifiedXmlReader.read((InputStream)stream,handler);this.mExtBookMap.put(rid,handler.getBookBlock());
            targetUri=targetUri.replace("%20"," ");let mut stream=part.getInputStream_default()?;let handler=ExternalLinkHandler::ExternalLinkHandler(Some(targetUri));UnifiedXmlReader::read(&mut stream,handler.asElementHandler())?;mExtBookMap.insert(rid.map(str::to_string),handler.getBookBlock());
        }}}}}Ok(())})
    }
    // java private void addExternalLinkToCore(){
    fn addExternalLinkToCore(&self){
        // java ArrayList<LinkTable.ExternalBookBlock> blockList=new ArrayList();for(String rid:this.mRids){LinkTable.ExternalBookBlock block=this.mExtBookMap.get(rid);if(block!=null){blockList.add(block);}}
        let mut blockList=vec![];for rid in self.mRids.as_ref().expect("NullPointerException").borrow().iter(){if let Some(Some(block))=self.mExtBookMap.get(rid){blockList.push(block.clone());}}
        // java int numOfSheet=this.mBook.sheetCount();blockList.add(new LinkTable.ExternalBookBlock(numOfSheet));LinkTable.ExternalBookBlock[] extBooks=new LinkTable.ExternalBookBlock[blockList.size()];blockList.toArray(extBooks);this.mBook.getLinkTable().resetExternalLink(extBooks);
        let numOfSheet=self.mBook.sheetCount();blockList.push(Rc::new(ExternalBookBlock::ExternalBookBlock__count(numOfSheet)));let extBooks=blockList;self.mBook.getLinkTable().borrow_mut().resetExternalLink(Some(&extBooks));
    }
}
impl XlsxrExternalLinks<'static>{
    pub(crate) fn fromPart(book:Rc<KmoBook>,part:Rc<RefCell<Option<OpcrPackagePart>>>,rids:Option<Rc<RefCell<Vec<Option<String>>>>>)->io::Result<Self>{
        // java OpcrRelationships relses=this.mBookPart.getRelationships();
        let present=part.borrow_mut().as_mut().expect("NullPointerException").getRelationships_mut()?.is_some();
        Ok(Self{mBook:book,mRelses:if present{Some(RelationshipsRef::PartOwner(part))}else{None},mRids:rids,mExtBookMap:HashMap::new()})
    }
}
