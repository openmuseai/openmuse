// java public class SparseIndexMap<T> implements Iterable<T> {
pub struct SparseIndexMap<T>{
    // java protected final int BLOCK_SHIFT;
    pub(crate) BLOCK_SHIFT:i32,
    // java protected final int BLOCK_MASK;
    pub(crate) BLOCK_MASK:i32,
    // java protected final int BLOCK_SIZE;
    pub(crate) BLOCK_SIZE:i32,
    // java protected int firstIndex;
    pub(crate) firstIndex:i32,
    // java protected int lastIndex;
    pub(crate) lastIndex:i32,
    // java protected final Object[][] blocks;
    pub(crate) blocks:Vec<Option<Vec<Option<T>>>>,
}
impl<T>SparseIndexMap<T>{
    // java public SparseIndexMap(int maxIndex) {
    pub fn SparseIndexMap(maxIndex:i32)->Self{
        // java if (maxIndex <= 0) { throw new IllegalArgumentException(); } else {
        if maxIndex<=0{panic!("IllegalArgumentException");}
        // java if (maxIndex < 256) {
        let(BLOCK_SHIFT,BLOCK_MASK,BLOCK_SIZE,blocks)=if maxIndex<256{
            // java this.BLOCK_SHIFT = 8; this.BLOCK_MASK = 255; this.BLOCK_SIZE = 256; this.blocks = new Object[1][]; this.blocks[0] = new Object[maxIndex + 1];
            let mut row=vec![];row.resize_with((maxIndex+1)as usize,||None);(8,255,256,vec![Some(row)])
        }else{
            // java int bitLength = 32 - Integer.numberOfLeadingZeros(maxIndex - 1);
            let bitLength=32-(maxIndex.wrapping_sub(1)as u32).leading_zeros()as i32;
            // java this.BLOCK_SHIFT = bitLength / 2; this.BLOCK_MASK = ~(-1 << this.BLOCK_SHIFT); this.BLOCK_SIZE = 1 << this.BLOCK_SHIFT;
            let shift=bitLength/2;let mask=!(-1i32.wrapping_shl(shift as u32));let size=1i32.wrapping_shl(shift as u32);
            // java int blockCount = (maxIndex + this.BLOCK_SIZE - 1) / this.BLOCK_SIZE; this.blocks = new Object[blockCount][];
            let blockCount=maxIndex.wrapping_add(size).wrapping_sub(1)/size;if blockCount<0{panic!("NegativeArraySizeException");}let mut blocks=vec![];blocks.resize_with(blockCount as usize,||None);(shift,mask,size,blocks)
        };
        // java this.firstIndex = -1; this.lastIndex = -2;
        Self{BLOCK_SHIFT,BLOCK_MASK,BLOCK_SIZE,firstIndex:-1,lastIndex:-2,blocks}
    }
    // java public T get(int index) {
    pub fn get(&self,index:i32)->Option<&T>{
        // java if (index < 0) { throw new IllegalArgumentException(); } else {
        if index<0{panic!("IllegalArgumentException");}
        // java int pdi = index >> this.BLOCK_SHIFT; if (pdi < this.blocks.length && this.blocks[pdi] != null) {
        let pdi=index>>self.BLOCK_SHIFT;if pdi<(self.blocks.len()as i32){if let Some(block)=&self.blocks[pdi as usize]{
            // java int pei = index & this.BLOCK_MASK; return (T)this.blocks[pdi][pei];
            let pei=index&self.BLOCK_MASK;return block.get(pei as usize).expect("ArrayIndexOutOfBoundsException").as_ref();
        }}
        // java else { return null; }
        None
    }
    // Native mutable primitive-list adapter for existing bounded LineLevelMap users.
    pub fn get_mut(&mut self,index:i32)->Option<&mut T>{if index<0{panic!("IllegalArgumentException");}let block=self.blocks.get_mut((index>>self.BLOCK_SHIFT)as usize)?.as_mut()?;block.get_mut((index&self.BLOCK_MASK)as usize).expect("ArrayIndexOutOfBoundsException").as_mut()}
    // Native non-null argument transport; T is an Rc for Java mutable references.
    pub fn put(&mut self,index:i32,data:T)->Option<T>where T:Clone{self.putNullable(index,Some(data))}
    // java public T put(int index, T data) {
    pub fn putNullable(&mut self,index:i32,data:Option<T>)->Option<T>where T:Clone{
        // java if (index >= 0 && data != null) {
        if index>=0&&data.is_some(){
            // java if (this.firstIndex < 0 || index < this.firstIndex) { this.firstIndex = index; }
            if self.firstIndex<0||index<self.firstIndex{self.firstIndex=index;}
            // java if (this.lastIndex < 0 || index > this.lastIndex) { this.lastIndex = index; }
            if self.lastIndex<0||index>self.lastIndex{self.lastIndex=index;}
            // java int pdi = index >> this.BLOCK_SHIFT; int pei = index & this.BLOCK_MASK; Object ret = null;
            let pdi=index>>self.BLOCK_SHIFT;let pei=index&self.BLOCK_MASK;let ret;
            // java if (pdi >= this.blocks.length) {
            if pdi>=self.blocks.len()as i32{
                // java if (this.blocks[this.blocks.length - 1] == null) { this.blocks[this.blocks.length - 1] = new Object[this.BLOCK_SIZE]; }
                let last=self.blocks.len().checked_sub(1).expect("ArrayIndexOutOfBoundsException");if self.blocks[last].is_none(){let mut block=vec![];block.resize_with(self.BLOCK_SIZE as usize,||None);self.blocks[last]=Some(block);}
                // java ret = this.blocks[this.blocks.length - 1][pei];
                ret=self.blocks[last].as_ref().unwrap().get(pei as usize).expect("ArrayIndexOutOfBoundsException").clone();
            }else{
                // java if (this.blocks[pdi] == null) { this.blocks[pdi] = new Object[this.BLOCK_SIZE]; }
                if self.blocks[pdi as usize].is_none(){let mut block=vec![];block.resize_with(self.BLOCK_SIZE as usize,||None);self.blocks[pdi as usize]=Some(block);}
                // java ret = this.blocks[pdi][pei];
                ret=self.blocks[pdi as usize].as_ref().unwrap().get(pei as usize).expect("ArrayIndexOutOfBoundsException").clone();
            }
            // java if (pdi < this.blocks.length) { this.blocks[pdi][pei] = data; }
            if pdi<self.blocks.len()as i32{*self.blocks[pdi as usize].as_mut().unwrap().get_mut(pei as usize).expect("ArrayIndexOutOfBoundsException")=data;}
            // java return (T)ret;
            ret
        // java } else { throw new IllegalArgumentException(); }
        }else{panic!("IllegalArgumentException");}
    }
    // java public T remove(int index) {
    pub fn remove(&mut self,index:i32)->Option<T>{
        // java if (index < 0) { throw new IllegalArgumentException(); } else {
        if index<0{panic!("IllegalArgumentException");}
        // java int pdi = index >> this.BLOCK_SHIFT; if (pdi < this.blocks.length && this.blocks[pdi] != null) {
        let pdi=index>>self.BLOCK_SHIFT;if pdi<self.blocks.len()as i32{if let Some(block)=&mut self.blocks[pdi as usize]{
            // java int pei = index & this.BLOCK_MASK; Object ret = this.blocks[pdi][pei]; this.blocks[pdi][pei] = null; return (T)ret;
            let pei=index&self.BLOCK_MASK;return block.get_mut(pei as usize).expect("ArrayIndexOutOfBoundsException").take();
        }}
        // java else { return null; }
        None
    }
    // java public int getFirstIndex() { return this.firstIndex; }
    pub fn getFirstIndex(&self)->i32{self.firstIndex}
    // java public int getLastIndex() { return this.lastIndex; }
    pub fn getLastIndex(&self)->i32{self.lastIndex}
    // Read-only borrowed Rust iterator adapters for existing LineLevelMap callers.
    // Java's live iterator facade is IndexMapIterator, retained by reference-valued maps.
    pub fn iteratorRange(&self,first:i32,last:i32)->impl Iterator<Item=(i32,&T)>{let first=first.max(self.firstIndex).max(0);let last=last.min(self.lastIndex);(first..=last).filter_map(move|i|self.get(i).map(|v|(i,v)))}
    pub fn iterator(&self)->impl Iterator<Item=(i32,&T)>{self.iteratorRange(self.firstIndex,self.lastIndex)}
}
