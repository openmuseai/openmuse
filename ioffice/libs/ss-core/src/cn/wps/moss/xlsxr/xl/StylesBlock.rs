// java package cn.wps.moss.xlsxr.xl;
// java
// java class StylesBlock {
// java int mBeginOffset;
// java int mEndOffset;
// java boolean bHasRead;
// java
// java StylesBlock(int beginOffset, int endOffset) {
// java this.mBeginOffset = beginOffset;
// java this.mEndOffset = endOffset;
// java this.bHasRead = false;
// java }
// java
// java public String toString() {
// java return "OFFSET: [" + this.mBeginOffset + ", " + this.mEndOffset + ")";
// java }
// java }
use crate::java_runtime::Cell;
pub struct StylesBlock {pub mBeginOffset:i32,pub mEndOffset:i32,pub bHasRead:Cell<bool>}
impl StylesBlock {pub fn new(beginOffset:i32,endOffset:i32)->Self{Self{mBeginOffset:beginOffset,mEndOffset:endOffset,bHasRead:Cell::new(false)}}pub fn toString(&self)->String{format!("OFFSET: [{}, {})",self.mBeginOffset,self.mEndOffset)}}
