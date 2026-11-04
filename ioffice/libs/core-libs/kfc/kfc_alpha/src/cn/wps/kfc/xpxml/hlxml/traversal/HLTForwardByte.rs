use std::sync::{Arc,Mutex,atomic::{AtomicI32,AtomicBool,Ordering}};
use super::{HLTByteAccepter::HLTByteAccepter,HLBackupByteBuffer::HLBackupByteBuffer};
// java public final class HLTForwardByte
// rust: Separate fields allow callback getters/stop without a recursive borrow of the parser.
pub struct HLTForwardByte {accepter:Arc<dyn HLTByteAccepter>,backupBuffer:Mutex<HLBackupByteBuffer>,lastElementPos:AtomicI32,lastElementLen:AtomicI32,active:AtomicBool}
impl HLTForwardByte {
// java public HLTForwardByte(HLTByteAccepter accepter) {
// java super();
// java this.accepter = accepter;
// java this.backupBuffer = new HLBackupByteBuffer(8320);
// java }
pub fn new(accepter:Arc<dyn HLTByteAccepter>)->Self {
Self{accepter,backupBuffer:Mutex::new(HLBackupByteBuffer::new(8320)),lastElementPos:AtomicI32::new(0),lastElementLen:AtomicI32::new(0),active:AtomicBool::new(true)}
}
// java public void reset() {
// java this.backupBuffer.clear();
// java }
pub fn reset(&self) {
self.backupBuffer.lock().unwrap().clear();
}
// java public void parse(byte[] chars, int length, int pos) {
// java this.active = true;
// java if (length > chars.length) {
// java throw new ArrayIndexOutOfBoundsException();
// java } else {
// java int index = 0;
// java if (this.backupBuffer.top() != 0) {
// java this.backupBuffer.put(chars, 0, length);
// java int ret;
// java if (this.backupBuffer.get(0) == 60) {
// java ret = this.parseElement(this.backupBuffer.buffer(), 0, this.backupBuffer.top() - 1, this.backupBuffer.getPos());
// java } else {
// java ret = this.parseCharacters(this.backupBuffer.buffer(), 0, this.backupBuffer.top() - 1, this.backupBuffer.getPos());
// java }
// java
// java if (ret == -1) {
// java return;
// java }
// java
// java index = ret - (this.backupBuffer.top() - length);
// java this.backupBuffer.clear();
// java }
// java
// java int end = length - 1;
// java
// java while(index <= end) {
// java if (!this.active) {
// java return;
// java }
// java
// java int ret;
// java if (chars[index] == 60) {
// java ret = this.parseElement(chars, index, end, pos);
// java } else {
// java ret = this.parseCharacters(chars, index, end, pos);
// java }
// java
// java if (ret != -1) {
// java index = ret;
// java } else {
// java this.backupBuffer.clear();
// java this.backupBuffer.put(chars, index, end - index + 1);
// java this.backupBuffer.setPos(index + pos);
// java index = end + 1;
// java }
// java }
// java
// java }
// java }
pub fn parse(&self,chars:&[u8],length:usize,pos:i32) {
self.active.store(true,Ordering::Relaxed);assert!(length<=chars.len(),"ArrayIndexOutOfBoundsException");let mut index=0;let backup={let mut b=self.backupBuffer.lock().unwrap();if b.top()!=0{b.put(chars,0,length);Some((b.buffer().to_vec(),b.top(),b.getPos()))}else{None}};if let Some((bytes,top,bpos))=backup{let ret=if bytes[0]==60{self.parseElement(&bytes,0,top-1,bpos)}else{self.parseCharacters(&bytes,0,top-1,bpos)};if ret== -1{return;}index=ret as usize-(top-length);self.backupBuffer.lock().unwrap().clear();}let end=length as i32-1;while index as i32<=end{if !self.active.load(Ordering::Relaxed){return;}let ret=if chars[index]==60{self.parseElement(chars,index,end as usize,pos)}else{self.parseCharacters(chars,index,end as usize,pos)};if ret!= -1{index=ret as usize;}else{let mut b=self.backupBuffer.lock().unwrap();b.clear();b.put(chars,index,end as usize-index+1);b.setPos(index as i32+pos);index=end as usize+1;}}
}
// java private int parseElement(byte[] chars, int start, int end, int pos) {
// java if (chars[start] != 60) {
// java throw new IllegalStateException();
// java } else {
// java boolean bInSingleQuote = false;
// java boolean bInDoubleQuote = false;
// java
// java for(int index = start; index <= end; ++index) {
// java switch (chars[index]) {
// java case 34:
// java if (!bInSingleQuote) {
// java bInDoubleQuote = !bInDoubleQuote;
// java }
// java break;
// java case 39:
// java if (!bInDoubleQuote) {
// java bInSingleQuote = !bInSingleQuote;
// java }
// java break;
// java case 62:
// java if (!bInDoubleQuote && !bInSingleQuote) {
// java this.lastElementPos = pos + start;
// java this.lastElementLen = index - start + 1;
// java this.accepter.element(chars, start, index);
// java return index + 1;
// java }
// java }
// java }
// java
// java return -1;
// java }
// java }
pub fn parseElement(&self,chars:&[u8],start:usize,end:usize,pos:i32)->i32 {
assert_eq!(chars[start],60,"IllegalStateException");let(mut single,mut double)=(false,false);for index in start..=end{match chars[index]{34=>if !single{double=!double;},39=>if !double{single=!single;},62=>if !single&&!double{self.lastElementPos.store(pos+start as i32,Ordering::Relaxed);self.lastElementLen.store((index-start+1) as i32,Ordering::Relaxed);self.accepter.element(chars,start,index);return index as i32+1;},_=>{}}}-1
}
// java private int parseCharacters(byte[] chars, int start, int end, int pos) {
// java if (chars[start] == 60) {
// java throw new IllegalStateException();
// java } else {
// java for(int index = start; index <= end; ++index) {
// java if (chars[index] == 60) {
// java if (start < index) {
// java this.accepter.characters(chars, start, index - 1);
// java }
// java
// java return index;
// java }
// java }
// java
// java return -1;
// java }
// java }
pub fn parseCharacters(&self,chars:&[u8],start:usize,end:usize,_pos:i32)->i32 {
assert_ne!(chars[start],60,"IllegalStateException");for index in start..=end{if chars[index]==60{if start<index{self.accepter.characters(chars,start,index-1);}return index as i32;}}-1
}
// java public int getLastElemPos() {
// java return this.lastElementPos;
// java }
pub fn getLastElemPos(&self)->i32 {
self.lastElementPos.load(Ordering::Relaxed)
}
// java public int getLastElemLength() {
// java return this.lastElementLen;
// java }
pub fn getLastElemLength(&self)->i32 {
self.lastElementLen.load(Ordering::Relaxed)
}
// java public void stop() {
// java this.active = false;
// java }
pub fn stop(&self) {
self.active.store(false,Ordering::Relaxed);
}
}
