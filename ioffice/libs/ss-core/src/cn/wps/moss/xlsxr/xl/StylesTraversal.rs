use std::{io::{self,Read},fs::File};
use crate::java_runtime::{Rc,Weak,RefCell,Cell};
use kfc_kfc_alpha::kfc::xpxml::hlxml::traversal::{HLTByteAccepter::HLTByteAccepter,HLTForwardByte::HLTForwardByte};
use super::StylesBlock::StylesBlock;
// java class StylesTraversal implements HLTByteAccepter
pub struct StylesTraversal {bNeedBlocks:Cell<bool>,mFirstXfStartOffset:Cell<i32>,mCellXfsEndOffset:Cell<i32>,mXFCount:Cell<i32>,mXfsOffset:Rc<RefCell<Vec<Rc<StylesBlock>>>>,mFileSize:Cell<i32>,mForward:RefCell<Option<Rc<HLTForwardByte>>>,mBeginTmpOffset:Cell<i32>}
// rust: The Java accepter points to its enclosing traversal without retaining a parser cycle.
struct Accepter(Weak<StylesTraversal>);
impl HLTByteAccepter for Accepter {fn element(&self,b:&[u8],s:usize,e:usize){self.0.upgrade().unwrap().element(b,s,e);}fn characters(&self,b:&[u8],s:usize,e:usize){self.0.upgrade().unwrap().characters(b,s,e);}}
impl StylesTraversal {
// java StylesTraversal(String xmlPath) throws IOException {
// java FileInputStream input = null;
// java
// java try {
// java input = new FileInputStream(xmlPath);
// java this.mFileSize = input.available();
// java if (this.mFileSize >= 102400) {
// java this.bNeedBlocks = true;
// java this.traversal(input);
// java return;
// java }
// java } finally {
// java if (input != null) {
// java input.close();
// java }
// java
// java }
// java
// java }
pub fn new(xmlPath:&str)->io::Result<Rc<Self>> {
let mut input=File::open(xmlPath)?;let this=Rc::new(Self{bNeedBlocks:Cell::new(false),mFirstXfStartOffset:Cell::new(-1),mCellXfsEndOffset:Cell::new(-1),mXFCount:Cell::new(0),mXfsOffset:Rc::new(RefCell::new(Vec::new())),mFileSize:Cell::new(input.metadata()?.len() as i32),mForward:RefCell::new(None),mBeginTmpOffset:Cell::new(0)});if this.mFileSize.get()>=102400{this.bNeedBlocks.set(true);Self::traversal(&this,&mut input)?;}Ok(this)
}
// java public boolean needBlocks() {
// java return this.bNeedBlocks;
// java }
pub fn needBlocks(&self)->bool {
self.bNeedBlocks.get()
}
// java public int getFirstXfStartOffset() {
// java return this.mFirstXfStartOffset;
// java }
pub fn getFirstXfStartOffset(&self)->i32 {
self.mFirstXfStartOffset.get()
}
// java public int getCellXfsEndOffset() {
// java return this.mCellXfsEndOffset;
// java }
pub fn getCellXfsEndOffset(&self)->i32 {
self.mCellXfsEndOffset.get()
}
// java public int getXFCount() {
// java return this.mXFCount;
// java }
pub fn getXFCount(&self)->i32 {
self.mXFCount.get()
}
// java public int getFileSize() {
// java return this.mFileSize;
// java }
pub fn getFileSize(&self)->i32 {
self.mFileSize.get()
}
// java public ArrayList<StylesBlock> getXfsOffset() {
// java return this.mXfsOffset;
// java }
pub fn getXfsOffset(&self)->Rc<RefCell<Vec<Rc<StylesBlock>>>> {
self.mXfsOffset.clone()
}
// java private void traversal(InputStream input) throws IOException {
// java int bufferSize = 8192;
// java byte[] buffer = new byte[bufferSize];
// java int pos = 0;
// java int length = 0;
// java this.mForward = new HLTForwardByte(this);
// java
// java while((length = input.read(buffer, 0, bufferSize)) > 0) {
// java this.mForward.parse(buffer, length, pos);
// java pos += length;
// java if (this.mCellXfsEndOffset != -1) {
// java break;
// java }
// java }
// java
// java this.mForward = null;
// java if (this.mFirstXfStartOffset == -1 || this.mCellXfsEndOffset == -1 || this.mXFCount < 300) {
// java this.bNeedBlocks = false;
// java }
// java
// java }
pub fn traversal(this:&Rc<Self>,input:&mut dyn Read)->io::Result<()> {
let mut buffer=[0u8;8192];let mut pos=0;let forward=Rc::new(HLTForwardByte::new(Rc::new(Accepter(Rc::downgrade(this)))));*this.mForward.borrow_mut()=Some(forward.clone());loop{let length=input.read(&mut buffer)?;if length==0{break;}forward.parse(&buffer,length,pos);pos+=length as i32;if this.mCellXfsEndOffset.get()!= -1{break;}}*this.mForward.borrow_mut()=None;if this.mFirstXfStartOffset.get()== -1||this.mCellXfsEndOffset.get()== -1||this.mXFCount.get()<300{this.bNeedBlocks.set(false);}Ok(())
}
// java public void characters(byte[] chars, int start, int end) {
// java }
pub fn characters(&self,_chars:&[u8],_start:usize,_end:usize) {

}
// java public void element(byte[] chars, int start, int end) {
// java if (this.mCellXfsEndOffset == -1) {
// java if (this.mFirstXfStartOffset != -1) {
// java if (end - start + 1 == 5 && chars[start + 1] == 47 && chars[start + 2] == 120 && chars[start + 3] == 102 && chars[start + 4] == 62) {
// java ++this.mXFCount;
// java if (this.mXFCount % 100 == 0) {
// java int offset = this.mForward.getLastElemPos() + this.mForward.getLastElemLength();
// java this.mXfsOffset.add(new StylesBlock(this.mBeginTmpOffset, offset));
// java this.mBeginTmpOffset = offset;
// java }
// java } else if (end - start + 1 >= 5 && chars[start + 1] == 120 && chars[start + 2] == 102 && chars[end - 1] == 47 && chars[end] == 62) {
// java ++this.mXFCount;
// java if (this.mXFCount % 100 == 0) {
// java int offset = this.mForward.getLastElemPos() + this.mForward.getLastElemLength();
// java this.mXfsOffset.add(new StylesBlock(this.mBeginTmpOffset, offset));
// java this.mBeginTmpOffset = offset;
// java }
// java } else if (end - start + 1 == 10 && chars[start + 1] == 47 && chars[start + 2] == 99 && chars[start + 3] == 101 && chars[start + 4] == 108 && chars[start + 5] == 108 && chars[start + 6] == 88 && chars[start + 7] == 102 && chars[start + 8] == 115 && chars[start + 9] == 62) {
// java int offset = this.mForward.getLastElemPos();
// java this.mCellXfsEndOffset = offset;
// java if (this.mXFCount % 100 != 0) {
// java this.mXfsOffset.add(new StylesBlock(this.mBeginTmpOffset, offset));
// java this.mBeginTmpOffset = offset;
// java }
// java }
// java } else if (end - start + 1 > 8 && chars[start + 1] == 99 && chars[start + 2] == 101 && chars[start + 3] == 108 && chars[start + 4] == 108 && chars[start + 5] == 88 && chars[start + 6] == 102 && chars[start + 7] == 115) {
// java int offset = this.mForward.getLastElemPos() + this.mForward.getLastElemLength();
// java this.mFirstXfStartOffset = offset;
// java this.mBeginTmpOffset = this.mFirstXfStartOffset;
// java }
// java
// java }
// java }
pub fn element(&self,chars:&[u8],start:usize,end:usize) {
if self.mCellXfsEndOffset.get()!= -1{return;}let token=&chars[start..=end];let f=self.mForward.borrow().as_ref().unwrap().clone();if self.mFirstXfStartOffset.get()!= -1{if token==b"</xf>"||(token.len()>=5&&token[1]==b'x'&&token[2]==b'f'&&token[token.len()-2]==b'/'&&token[token.len()-1]==b'>'){self.mXFCount.set(self.mXFCount.get()+1);if self.mXFCount.get()%100==0{let offset=f.getLastElemPos()+f.getLastElemLength();self.mXfsOffset.borrow_mut().push(Rc::new(StylesBlock::new(self.mBeginTmpOffset.get(),offset)));self.mBeginTmpOffset.set(offset);}}else if token==b"</cellXfs>"{let offset=f.getLastElemPos();self.mCellXfsEndOffset.set(offset);if self.mXFCount.get()%100!=0{self.mXfsOffset.borrow_mut().push(Rc::new(StylesBlock::new(self.mBeginTmpOffset.get(),offset)));self.mBeginTmpOffset.set(offset);}}}else if token.len()>8&&token[1..8]==*b"cellXfs"{let offset=f.getLastElemPos()+f.getLastElemLength();self.mFirstXfStartOffset.set(offset);self.mBeginTmpOffset.set(offset);}
}
}
