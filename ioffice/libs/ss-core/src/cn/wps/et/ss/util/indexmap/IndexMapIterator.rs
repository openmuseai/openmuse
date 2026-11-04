use crate::java_runtime::{Rc,RefCell};
use super::SparseIndexMap::SparseIndexMap;
// java public class IndexMapIterator<T> implements Iterator<T> {
pub struct IndexMapIterator<T>{
    // java final SparseIndexMap<T> map;
    map:Rc<RefCell<SparseIndexMap<T>>>,
    // java final int last;
    last:i32,
    // java int nextIndex;
    nextIndex:i32,
    // java int currentIndex = -1;
    currentIndex:i32,
}
impl<T:Clone>IndexMapIterator<T>{
    // java public IndexMapIterator(SparseIndexMap<T> map, int first, int last) {
    pub fn new(map:Option<Rc<RefCell<SparseIndexMap<T>>>>,first:i32,last:i32)->Self{
        // java this.map = map; this.last = last; this.nextIndex = first - 1; this.advance();
        let mut it=Self{map:map.expect("NullPointerException"),last,nextIndex:first.wrapping_sub(1),currentIndex:-1};it.advance();it
    }
    // java private void advance() {
    fn advance(&mut self){
        // java ++this.nextIndex;
        self.nextIndex=self.nextIndex.wrapping_add(1);
        // java while(true) {
        loop{let map=self.map.borrow();
            // java if (this.nextIndex < this.map.firstIndex) { this.nextIndex = this.map.firstIndex; }
            if self.nextIndex<map.firstIndex{self.nextIndex=map.firstIndex;}
            // java if (this.nextIndex > this.map.lastIndex || this.nextIndex > this.last) { break; }
            if self.nextIndex>map.lastIndex||self.nextIndex>self.last{break;}
            // java int pdi = this.nextIndex >> this.map.BLOCK_SHIFT; if (this.map.blocks[pdi] == null) {
            let pdi=self.nextIndex>>map.BLOCK_SHIFT;let block=map.blocks.get(pdi as usize).expect("ArrayIndexOutOfBoundsException");if block.is_none(){
                // java this.nextIndex = pdi + 1 << this.map.BLOCK_SHIFT;
                self.nextIndex=pdi.wrapping_add(1).wrapping_shl(map.BLOCK_SHIFT as u32);
            }else{
                // java int pei = this.nextIndex & this.map.BLOCK_MASK; if (this.map.blocks[pdi][pei] != null) { break; }
                let pei=self.nextIndex&map.BLOCK_MASK;if block.as_ref().unwrap().get(pei as usize).expect("ArrayIndexOutOfBoundsException").is_some(){break;}
                // java ++this.nextIndex;
                self.nextIndex=self.nextIndex.wrapping_add(1);
            }
        }
    }
    // java public boolean hasNext() { return this.nextIndex <= this.map.lastIndex && this.nextIndex <= this.last; }
    pub fn hasNext(&self)->bool{self.nextIndex<=self.map.borrow().lastIndex&&self.nextIndex<=self.last}
    // java public T next() {
    pub fn next(&mut self)->Option<T>{
        // java if (!this.hasNext()) { throw new IllegalStateException(); } else {
        if !self.hasNext(){panic!("IllegalStateException");}
        // java this.currentIndex = this.nextIndex; this.advance(); return this.map.get(this.currentIndex);
        self.currentIndex=self.nextIndex;self.advance();self.map.borrow().get(self.currentIndex).cloned()
    }
    // java public int index() { return this.currentIndex; }
    pub fn index(&self)->i32{self.currentIndex}
    // java public T data() { return this.map.get(this.currentIndex); }
    pub fn data(&self)->Option<T>{self.map.borrow().get(self.currentIndex).cloned()}
    // java public void remove() { this.map.remove(this.currentIndex); }
    pub fn remove(&self){self.map.borrow_mut().remove(self.currentIndex);}
}
