use std::sync::Mutex;
use crate::java_runtime::{Rc,RefCell};
use kfc_kfc_alpha::kfc::opcr::OpcrRelationship::OpcrRelationship;
use crate::cn::wps::moss::{app::KmoBook::KmoBook,xlsxr::{handler::style::StylesHandler::StylesHandler,model::StylesTable::StylesTable,util::{UnifiedXmlReader::UnifiedXmlReader,XlsxrFileCache::XlsxrFileCache}}};
use super::{StylesTraversal::StylesTraversal,StylesBlock::StylesBlock};
struct StylesStatic {mStringPath:Option<String>,mTraversal:Option<Rc<StylesTraversal>>,mStylesHandler:Option<Rc<StylesHandler>>}
static STYLES_STATIC:Mutex<StylesStatic>=Mutex::new(StylesStatic{mStringPath:None,mTraversal:None,mStylesHandler:None});
// java public final class XlsxrStyles
pub struct XlsxrStyles;
pub const LARGE_STYLES:i32=102400;
struct HandlerRef(Rc<StylesHandler>);
impl kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler for HandlerRef {
fn startElement(&self,e:i32,a:&dyn kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr){self.0.startElement(e,a);}fn endElement(&self,e:i32){self.0.endElement(e);}fn characters(&self,s:&str){self.0.characters(s);}fn enterSubElement(&self,e:i32)->Option<Box<dyn kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler>>{self.0.enterSubElement(e)}}
impl XlsxrStyles {
// java private static void clear() {
// java mStringPath = null;
// java mTraversal = null;
// java mStylesHandler = null;
// java }
pub fn clear() {
let mut s=STYLES_STATIC.lock().unwrap();s.mStringPath=None;s.mTraversal=None;s.mStylesHandler=None;
}
// java public static void open(KmoBook book, OpcrRelationship stylesRels, StylesTable stylesTable) throws IOException {
// java clear();
// java KmoXFManager xfs = book.getXFManager();
// java stylesTable.initStartCoreXF(xfs);
// java stylesTable.initEndCoreXF(xfs);
// java if (stylesRels != null) {
// java OpcrPackagePart part = stylesRels.getPackagePart();
// java if (part != null) {
// java InputStream stream = part.getInputStream();
// java if (stream != null) {
// java if (stream.available() <= 102400) {
// java StylesHandler handler = new StylesHandler(book, stylesTable);
// java UnifiedXmlReader.read((InputStream)stream, handler);
// java } else {
// java mStringPath = XlsxrFileCache.dumpTempFile(stream, new File(book.getTmpPath()));
// java StylesTraversal traversal = new StylesTraversal(mStringPath);
// java if (traversal.needBlocks()) {
// java StylesHandler handler = new StylesHandler(book, stylesTable);
// java UnifiedXmlReader.read(mStringPath, 0, traversal.getFirstXfStartOffset(), handler, true);
// java UnifiedXmlReader.read(mStringPath, traversal.getCellXfsEndOffset() + 10, traversal.getFileSize() - 13, handler, false);
// java xfs.createNullXFs(traversal.getXFCount() - 1);
// java mStylesHandler = handler;
// java mTraversal = traversal;
// java onXFIndex(stylesTable, 0);
// java } else {
// java stream = part.getInputStream();
// java StylesHandler handler = new StylesHandler(book, stylesTable);
// java UnifiedXmlReader.read((InputStream)stream, handler);
// java }
// java }
// java
// java stylesTable.initEndCoreXF(book.getXFManager());
// java }
// java }
// java }
// java }
pub fn open(book:&Rc<KmoBook>,stylesRels:Option<&mut OpcrRelationship>,stylesTable:&Rc<RefCell<StylesTable>>)->std::io::Result<()> {
Self::clear();stylesTable.borrow().initStartCoreXF(&book.getXFManager().borrow());stylesTable.borrow().initEndCoreXF(&book.getXFManager().borrow());if let Some(rels)=stylesRels{if let Some(part)=rels.getPackagePart(){let mut stream=part.getInputStream_default()?;let size=part.getPartSize()?;if size<=102400{UnifiedXmlReader::read(&mut stream,Box::new(StylesHandler::StylesHandler(book.clone(),stylesTable.clone())))?;}else{let path=XlsxrFileCache::dumpTempFile(&mut stream,&book.getTmpPath())?;let traversal=StylesTraversal::new(&path)?;if traversal.needBlocks(){let handler=Rc::new(StylesHandler::StylesHandler(book.clone(),stylesTable.clone()));UnifiedXmlReader::read__5(&path,0,traversal.getFirstXfStartOffset(),Box::new(HandlerRef(handler.clone())),true)?;UnifiedXmlReader::read__5(&path,traversal.getCellXfsEndOffset()+10,traversal.getFileSize()-13,Box::new(HandlerRef(handler.clone())),false)?;book.getXFManager().borrow().createNullXFs(traversal.getXFCount()-1);{let mut state=STYLES_STATIC.lock().unwrap();state.mStringPath=Some(path);state.mTraversal=Some(traversal);state.mStylesHandler=Some(handler);}Self::onXFIndex(&stylesTable.borrow(),0);}else{let mut stream=part.getInputStream_default()?;UnifiedXmlReader::read(&mut stream,Box::new(StylesHandler::StylesHandler(book.clone(),stylesTable.clone())))?;}}stylesTable.borrow().initEndCoreXF(&book.getXFManager().borrow());}}Ok(())
}
// rust: Clone Java references before parsing; callbacks may re-enter static getters.
fn state()->Option<(String,Rc<StylesTraversal>,Rc<StylesHandler>)>{let s=STYLES_STATIC.lock().unwrap();Some((s.mStringPath.clone()?,s.mTraversal.clone()?,s.mStylesHandler.clone()?))}
// java public static void onXFIndex(StylesTable stylesTable, int ixf) {
// java if (mTraversal != null) {
// java ArrayList<StylesBlock> blocks = mTraversal.getXfsOffset();
// java int index = ixf / 100;
// java StylesBlock block = (StylesBlock)blocks.get(index);
// java if (!block.bHasRead) {
// java block.bHasRead = true;
// java StylesHandler.CellXFormatsHandler handler = mStylesHandler.getCellXFormatsHandler();
// java int currentXF = stylesTable.getStartCoreXF() + index * 100 - 1;
// java handler.setCurrentXF(currentXF);
// java
// java try {
// java UnifiedXmlReader.read(mStringPath, block.mBeginOffset, block.mEndOffset, handler, false);
// java } catch (IOException e) {
// java throw new RuntimeException(e);
// java }
// java }
// java }
// java }
pub fn onXFIndex(stylesTable:&StylesTable,ixf:i32) {
if let Some((path,traversal,styles))=Self::state(){let blocks=traversal.getXfsOffset();let index=ixf/100;let block=blocks.borrow()[index as usize].clone();if !block.bHasRead.get(){block.bHasRead.set(true);let handler=styles.getCellXFormatsHandler();let current=stylesTable.getStartCoreXF()+index*100-1;handler.setCurrentXF(current);UnifiedXmlReader::read__5(&path,block.mBeginOffset,block.mEndOffset,Box::new(handler),false).unwrap_or_else(|e|panic!("RuntimeException: {e}"));}}
}
// java private static int openBlocks(ArrayList<StylesBlock> blocks, int beginIndex, StylesTable stylesTable) {
// java int size = blocks.size();
// java int beginOffset = -1;
// java int beginOffsetIndex = -1;
// java
// java int endOffset;
// java for(endOffset = -1; beginIndex < size; ++beginIndex) {
// java StylesBlock block = (StylesBlock)blocks.get(beginIndex);
// java if (!block.bHasRead) {
// java beginOffset = block.mBeginOffset;
// java endOffset = block.mEndOffset;
// java beginOffsetIndex = beginIndex++;
// java block.bHasRead = true;
// java break;
// java }
// java }
// java
// java if (beginOffset == -1) {
// java return -1;
// java } else {
// java while(beginIndex < size) {
// java StylesBlock block = (StylesBlock)blocks.get(beginIndex);
// java if (block.bHasRead) {
// java ++beginIndex;
// java break;
// java }
// java
// java endOffset = block.mEndOffset;
// java block.bHasRead = true;
// java ++beginIndex;
// java }
// java
// java StylesHandler.CellXFormatsHandler handler = mStylesHandler.getCellXFormatsHandler();
// java int currentXF = stylesTable.getStartCoreXF() + beginOffsetIndex * 100 - 1;
// java handler.setCurrentXF(currentXF);
// java
// java try {
// java UnifiedXmlReader.read(mStringPath, beginOffset, endOffset, handler, false);
// java } catch (IOException e) {
// java throw new RuntimeException(e);
// java }
// java
// java return beginIndex >= size ? -1 : beginIndex;
// java }
// java }
pub fn openBlocks(blocks:&[Rc<StylesBlock>],mut beginIndex:i32,stylesTable:&StylesTable)->i32 {
let size=blocks.len() as i32;let(mut begin_offset,mut end_offset,mut begin_offset_index)=(-1,-1,-1);while beginIndex<size{let block=&blocks[beginIndex as usize];if !block.bHasRead.get(){begin_offset=block.mBeginOffset;end_offset=block.mEndOffset;begin_offset_index=beginIndex;beginIndex+=1;block.bHasRead.set(true);break;}beginIndex+=1;}if begin_offset== -1{return -1;}while beginIndex<size{let block=&blocks[beginIndex as usize];if block.bHasRead.get(){beginIndex+=1;break;}end_offset=block.mEndOffset;block.bHasRead.set(true);beginIndex+=1;}let(path,_,styles)=Self::state().unwrap();let handler=styles.getCellXFormatsHandler();handler.setCurrentXF(stylesTable.getStartCoreXF()+begin_offset_index*100-1);UnifiedXmlReader::read__5(&path,begin_offset,end_offset,Box::new(handler),false).unwrap_or_else(|e|panic!("RuntimeException: {e}"));if beginIndex>=size{-1}else{beginIndex}
}
// java public static void onActiveSheetAfter(StylesTable stylesTable) throws IOException {
// java if (mTraversal != null) {
// java ArrayList<StylesBlock> blocks = mTraversal.getXfsOffset();
// java int beginIndex = 0;
// java
// java while((beginIndex = openBlocks(blocks, beginIndex, stylesTable)) != -1) {
// java }
// java
// java }
// java }
pub fn onActiveSheetAfter(stylesTable:&StylesTable)->std::io::Result<()> {
if let Some((_,traversal,_))=Self::state(){let blocks=traversal.getXfsOffset().borrow().clone();let mut begin=0;loop{begin=Self::openBlocks(&blocks,begin,stylesTable);if begin== -1{break;}}}Ok(())
}
}
