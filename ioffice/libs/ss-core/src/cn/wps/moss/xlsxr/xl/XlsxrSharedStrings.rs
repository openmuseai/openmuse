use crate::{java_runtime::{Rc,RefCell},cn::wps::moss::{app::KmoBook::KmoBook,engine::grid::MemoryMixSQLStringList::OoxmlStr,xlsxr::{handler::SharedStringsHandler::SharedStringsHandler,util::{UnifiedXmlReader::UnifiedXmlReader,XlsxrFileCache::XlsxrFileCache}}}};
use super::StringsTraversal::{StringsTraversal,Blocks};
use kfc_kfc_alpha::kfc::{opcr::OpcrRelationship::OpcrRelationship,xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr}};
use std::{io,fs};
// java public final class XlsxrSharedStrings {
pub struct XlsxrSharedStrings;
// java public static final int ERROR_SI_INDEX=-1;public static final int LARGE_STRINGS=16384;
pub const ERROR_SI_INDEX:i32=-1;pub const LARGE_STRINGS:i32=16384;
struct SstStatic{
    // java private static ArrayList<StringBlock> mBlocks;
    mBlocks:Option<Blocks>,
    // java private static String mStringPath;
    mStringPath:Option<String>,
    // java private static int mUniqueCount=0;
    mUniqueCount:i32,
}
static SST_STATIC:std::sync::LazyLock<RefCell<SstStatic>>=std::sync::LazyLock::new(||RefCell::new(SstStatic{mBlocks:None,mStringPath:None,mUniqueCount:0}));
// Native view retaining the same reusable handler between original XML calls.
struct HandlerRef(Rc<SharedStringsHandler>);
impl ElementHandler for HandlerRef{
    fn startElement(&self,e:i32,a:&dyn XmlRoAttr){self.0.startElement(e,a);}
    fn endElement(&self,e:i32){self.0.endElement(e);}
    fn characters(&self,s:&str){self.0.characters(s);}
    fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{self.0.enterSubElement(e)}
}
impl XlsxrSharedStrings{
    // java private static void clearCache(){if(mBlocks!=null){mBlocks.clear();mBlocks=null;}if(mStringPath!=null){(new File(mStringPath)).delete();mStringPath=null;}}
    fn clearCache(){let mut state=SST_STATIC.borrow_mut();if let Some(blocks)=&state.mBlocks{blocks.borrow_mut().clear();state.mBlocks=None;}if let Some(path)=&state.mStringPath{let _=fs::remove_file(path);state.mStringPath=None;}}
    // java public static void open(KmoBook book,OpcrRelationship stringsRels)throws IOException{
    pub fn open(book:&Rc<KmoBook>,stringsRels:Option<&mut OpcrRelationship>)->io::Result<()>{
        // java mUniqueCount=0;clearCache();if(stringsRels!=null){OpcrPackagePart part=stringsRels.getPackagePart();if(part!=null){InputStream stream=part.getInputStream();if(stream!=null){if(stream.available()<=16384){openAllStrings(book,stream);}else{
        SST_STATIC.borrow_mut().mUniqueCount=0;Self::clearCache();if let Some(stringsRels)=stringsRels{if let Some(part)=stringsRels.getPackagePart(){if let Some(mut stream)=part.getInputStream_available(false)?{let available=stream.get_ref().len()as u64-stream.position();if available<=16384{Self::openAllStrings(book,&mut stream)?;}else{
            // java mStringPath=XlsxrFileCache.dumpTempFile(stream,new File(book.getTmpPath()));File file=new File(mStringPath);if(null==mStringPath||0L==file.length()){return;}
            let path=XlsxrFileCache::dumpTempFile(&mut stream,&book.getTmpPath())?;SST_STATIC.borrow_mut().mStringPath=Some(path.clone());let length=fs::metadata(&path).map(|m|m.len()).unwrap_or(0);if length==0{return Ok(());}
            // java mBlocks=new ArrayList();StringsTraversal traversal=new StringsTraversal(mStringPath,mBlocks);traversal.traversal();mUniqueCount=traversal.getUniqueCount();
            let blocks=Rc::new(RefCell::new(vec![]));SST_STATIC.borrow_mut().mBlocks=Some(blocks.clone());let traversal=StringsTraversal::StringsTraversal(Some(path.clone()),Some(blocks));traversal.traversal()?;SST_STATIC.borrow_mut().mUniqueCount=traversal.getUniqueCount();
            // java if(mUniqueCount==0){openAllStrings(book,mStringPath);mBlocks=null;return;}
            if Self::getUniqueCount()==0{Self::openAllStrings__2(book,&path)?;SST_STATIC.borrow_mut().mBlocks=None;return Ok(());}
            // java ArrayList<UnicodeString> strs=new ArrayList();book.getCellValueStore().addStrsByOOXML(strs.iterator(),0,mUniqueCount,false);
            let strs:Vec<OoxmlStr>=vec![];book.getCellValueStore().borrow_mut().addStrsByOOXML(&strs,0,Self::getUniqueCount(),false);
        }}}}Ok(())
    }
    // java public static void onActiveSheetBefore(KmoBook book)throws IOException{
    pub fn onActiveSheetBefore(book:&Rc<KmoBook>)->io::Result<()>{
        // java if(mBlocks!=null){ArrayList<UnicodeString> strs=null;SharedStringsHandler handler=new SharedStringsHandler(strs,book);for(StringBlock block:mBlocks){if(block.bNeedReadFirst&&!block.bHasRead){
        let blocks=SST_STATIC.borrow().mBlocks.clone();if let Some(blocks)=blocks{let mut strs:Option<Rc<RefCell<Vec<OoxmlStr>>>>=None;let handler=Rc::new(SharedStringsHandler::SharedStringsHandler(strs.clone(),book.clone()));for block in blocks.borrow().iter(){if block.bNeedReadFirst.get()&&!block.bHasRead.get(){
            // java if(strs==null){strs=new ArrayList(block.mEndIndex-block.mBeginIndex+1);}handler.reset(strs);UnifiedXmlReader.read(mStringPath,block.mBeginOffset,block.mEndOffset,handler,false);
            if strs.is_none(){strs=Some(Rc::new(RefCell::new(Vec::with_capacity(block.mEndIndex.get().wrapping_sub(block.mBeginIndex.get()).wrapping_add(1)as usize))));}handler.reset(strs.clone());let path=SST_STATIC.borrow().mStringPath.clone().expect("NullPointerException");UnifiedXmlReader::read__5(&path,block.mBeginOffset.get(),block.mEndOffset.get(),Box::new(HandlerRef(handler.clone())),false)?;
            // java book.getCellValueStore().addStrsByOOXML(strs.iterator(),block.mBeginIndex,mUniqueCount,false);strs.clear();block.bHasRead=true;
            let strs=strs.as_ref().unwrap();book.getCellValueStore().borrow_mut().addStrsByOOXML(&strs.borrow(),block.mBeginIndex.get(),Self::getUniqueCount(),false);strs.borrow_mut().clear();block.bHasRead.set(true);
        }}}Ok(())
    }
    // java public static void onActiveSheetAfter(KmoBook book)throws IOException{
    pub fn onActiveSheetAfter(book:&Rc<KmoBook>)->io::Result<()>{
        // java if(mBlocks!=null){ArrayList<UnicodeString> strs=null;SharedStringsHandler handler=new SharedStringsHandler(strs,book);for(StringBlock block:mBlocks){if(!block.bHasRead){
        let blocks=SST_STATIC.borrow().mBlocks.clone();if let Some(blocks)=blocks{let mut strs:Option<Rc<RefCell<Vec<OoxmlStr>>>>=None;let handler=Rc::new(SharedStringsHandler::SharedStringsHandler(strs.clone(),book.clone()));for block in blocks.borrow().iter(){if !block.bHasRead.get(){
            // java if(strs==null){strs=new ArrayList(block.mEndIndex-block.mBeginIndex+1);}handler.reset(strs);UnifiedXmlReader.read(mStringPath,block.mBeginOffset,block.mEndOffset,handler,false);
            if strs.is_none(){strs=Some(Rc::new(RefCell::new(Vec::with_capacity(block.mEndIndex.get().wrapping_sub(block.mBeginIndex.get()).wrapping_add(1)as usize))));}handler.reset(strs.clone());let path=SST_STATIC.borrow().mStringPath.clone().expect("NullPointerException");UnifiedXmlReader::read__5(&path,block.mBeginOffset.get(),block.mEndOffset.get(),Box::new(HandlerRef(handler.clone())),false)?;
            // java book.getCellValueStore().addStrsByOOXML(strs.iterator(),block.mBeginIndex,mUniqueCount,true);strs.clear();block.bHasRead=true;
            let strs=strs.as_ref().unwrap();book.getCellValueStore().borrow_mut().addStrsByOOXML(&strs.borrow(),block.mBeginIndex.get(),Self::getUniqueCount(),true);strs.borrow_mut().clear();block.bHasRead.set(true);
        }}
        // java clearCache();
        Self::clearCache();}Ok(())
    }
    // java public static void onSheetStringIndex(int si){if(mBlocks!=null){for(StringBlock block:mBlocks){if(block.mBeginIndex<=si&&block.mEndIndex>=si){block.bNeedReadFirst=true;}}}}
    pub fn onSheetStringIndex(si:i32){let blocks=SST_STATIC.borrow().mBlocks.clone();if let Some(blocks)=blocks{for block in blocks.borrow().iter(){if block.mBeginIndex.get()<=si&&block.mEndIndex.get()>=si{block.bNeedReadFirst.set(true);}}}}
    // java public static int getUniqueCount(){return mUniqueCount;}
    pub fn getUniqueCount()->i32{SST_STATIC.borrow().mUniqueCount}
    // java private static void openAllStrings(KmoBook book,InputStream stream)throws IOException{ArrayList<UnicodeString> strs=new ArrayList();SharedStringsHandler handler=new SharedStringsHandler(strs,book);UnifiedXmlReader.read((InputStream)stream,handler);book.getCellValueStore().addStrsByOOXML(strs.iterator(),0,strs.size(),true);mUniqueCount=strs.size();strs.clear();}
    fn openAllStrings(book:&Rc<KmoBook>,stream:&mut dyn io::Read)->io::Result<()>{let strs=Rc::new(RefCell::new(vec![]));let handler=SharedStringsHandler::SharedStringsHandler(Some(strs.clone()),book.clone());UnifiedXmlReader::read(stream,Box::new(handler))?;book.getCellValueStore().borrow_mut().addStrsByOOXML(&strs.borrow(),0,strs.borrow().len()as i32,true);SST_STATIC.borrow_mut().mUniqueCount=strs.borrow().len()as i32;strs.borrow_mut().clear();Ok(())}
    // java private static void openAllStrings(KmoBook book,String path)throws IOException{ArrayList<UnicodeString> strs=new ArrayList();SharedStringsHandler handler=new SharedStringsHandler(strs,book);UnifiedXmlReader.read((String)path,handler);book.getCellValueStore().addStrsByOOXML(strs.iterator(),0,strs.size(),true);mUniqueCount=strs.size();strs.clear();}
    fn openAllStrings__2(book:&Rc<KmoBook>,path:&str)->io::Result<()>{let strs=Rc::new(RefCell::new(vec![]));let handler=SharedStringsHandler::SharedStringsHandler(Some(strs.clone()),book.clone());UnifiedXmlReader::read__4(path,Box::new(handler))?;book.getCellValueStore().borrow_mut().addStrsByOOXML(&strs.borrow(),0,strs.borrow().len()as i32,true);SST_STATIC.borrow_mut().mUniqueCount=strs.borrow().len()as i32;strs.borrow_mut().clear();Ok(())}
}
#[cfg(test)]
#[path="XlsxrSharedStrings.review.rs"]
pub(crate) mod review;
