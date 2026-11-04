use crate::java_runtime::Cell;
// java class StringBlock {
pub(super) struct StringBlock{
    // java int mBeginIndex; int mEndIndex; int mBeginOffset; int mEndOffset;
    pub mBeginIndex:Cell<i32>,pub mEndIndex:Cell<i32>,pub mBeginOffset:Cell<i32>,pub mEndOffset:Cell<i32>,
    // java boolean bNeedReadFirst; boolean bHasRead;
    pub bNeedReadFirst:Cell<bool>,pub bHasRead:Cell<bool>,
}
impl StringBlock{
    // java StringBlock(int beginIndex,int endIndex,int beginOffset,int endOffset){this.mBeginIndex=beginIndex;this.mEndIndex=endIndex;this.mBeginOffset=beginOffset;this.mEndOffset=endOffset;this.bNeedReadFirst=false;this.bHasRead=false;}
    pub fn StringBlock(beginIndex:i32,endIndex:i32,beginOffset:i32,endOffset:i32)->Self{Self{mBeginIndex:Cell::new(beginIndex),mEndIndex:Cell::new(endIndex),mBeginOffset:Cell::new(beginOffset),mEndOffset:Cell::new(endOffset),bNeedReadFirst:Cell::new(false),bHasRead:Cell::new(false)}}
    // java public String toString(){return "INDEX: ["+this.mBeginIndex+", "+this.mEndIndex+"], OFFSET: ["+this.mBeginOffset+", "+this.mEndOffset+")";}
    pub fn toString(&self)->String{format!("INDEX: [{}, {}], OFFSET: [{}, {})",self.mBeginIndex.get(),self.mEndIndex.get(),self.mBeginOffset.get(),self.mEndOffset.get())}
}
