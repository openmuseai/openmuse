use crate::{java_runtime::{Rc,RefCell},cn::wps::moss::util::KmoPair::KmoPair};
#[cfg(test)]
#[path="KmoRowRange.review.rs"]
mod review;
// java public class KmoRowRange implements Cloneable {
pub struct KmoRowRange{
    // java private ILongList ranges = new WLongArrayList();
    // Native primitive-list storage retains the same Java mutable List identity.
    ranges:Rc<RefCell<Vec<i64>>>,
}
impl KmoRowRange{
    pub fn new()->Self{Self{ranges:Rc::new(RefCell::new(vec![]))}}
    // java public KmoRowRange clone() {
    pub fn clone(&self)->Self{
        // java KmoRowRange rowRange = new KmoRowRange();
        let mut rowRange=Self::new();
        // java rowRange.ranges = new WLongArrayList(this.ranges); return rowRange;
        rowRange.ranges=Rc::new(RefCell::new(self.ranges.borrow().clone()));rowRange
    }
    // java public synchronized void addRow(int row) {
    pub fn addRow(&self,row:i32){
        // java int size = this.ranges.size(); if (size == 0) {
        let mut ranges=self.ranges.borrow_mut();let size=ranges.len()as i32;if size==0{
            // java this.ranges.add(makePair(row, row));
            ranges.push(Self::makePair(row,row));
        }else{
            // java int pos = this.findRowInsert(row); int mode = pos >> 20; if (mode != 1) {
            let pos=Self::findRowInsertIn(&ranges,row);let mode=pos>>20;if mode!=1{
                // java int insertPos = pos & 1048575;
                let insertPos=pos&1048575;
                // java if (insertPos == 0) { prevFirst = -2; prevSecond = -2; } else { long prevPair = this.ranges.get(insertPos - 1); prevFirst = getFirst(prevPair); prevSecond = getSecond(prevPair); }
                let(prevFirst,prevSecond)=if insertPos==0{(-2,-2)}else{let prevPair=Self::get(&ranges,insertPos-1);(Self::getFirst(prevPair),Self::getSecond(prevPair))};
                // java if (insertPos == size) { nextFirst = -2; nextSecond = -2; } else { long nextPair = this.ranges.get(insertPos); nextFirst = getFirst(nextPair); nextSecond = getSecond(nextPair); }
                let(nextFirst,nextSecond)=if insertPos==size{(-2,-2)}else{let nextPair=Self::get(&ranges,insertPos);(Self::getFirst(nextPair),Self::getSecond(nextPair))};
                // java if (row == prevSecond + 1) {
                if row==prevSecond.wrapping_add(1){
                    // java if (row == nextFirst - 1) { this.ranges.set(insertPos - 1, makePair(prevFirst, nextSecond)); this.ranges.removeAt(insertPos); }
                    if row==nextFirst.wrapping_sub(1){Self::set(&mut ranges,insertPos-1,Self::makePair(prevFirst,nextSecond));Self::removeAt(&mut ranges,insertPos);}
                    // java else { this.ranges.set(insertPos - 1, makePair(prevFirst, row)); }
                    else{Self::set(&mut ranges,insertPos-1,Self::makePair(prevFirst,row));}
                // java } else if (row == nextFirst - 1) { this.ranges.set(insertPos, makePair(row, nextSecond)); }
                }else if row==nextFirst.wrapping_sub(1){Self::set(&mut ranges,insertPos,Self::makePair(row,nextSecond));}
                // java else { this.ranges.insert(insertPos, makePair(row, row)); }
                else{Self::insert(&mut ranges,insertPos,Self::makePair(row,row));}
            }
        }
    }
    // java public boolean equals(Object o) {
    pub fn equals(&self,o:Option<&dyn std::any::Any>)->bool{
        // java if (this == o) { return true; } else if (!(o instanceof KmoRowRange)) { return false; }
        let Some(rowRange)=o.and_then(|o|o.downcast_ref::<Self>())else{return false;};if std::ptr::eq(self,rowRange){return true;}
        // java else { KmoRowRange rowRange = (KmoRowRange)o; return Objects.equals(this.ranges, rowRange.ranges); }
        *self.ranges.borrow()==*rowRange.ranges.borrow()
    }
    // java public int hashCode() { return Objects.hash(new Object[]{this.ranges}); }
    // WLongArrayList.hashCode sums HashCode.hash(long), then Objects.hash adds 31.
    pub fn hashCode(&self)->i32{self.ranges.borrow().iter().rev().fold(0i32,|h,v|h.wrapping_add((*v as u64^(*v as u64>>32))as i32)).wrapping_add(31)}
    // java public synchronized boolean containsRow(int row) { return this.findRowInsert(row) >> 20 == 1; }
    pub fn containsRow(&self,row:i32)->bool{self.findRowInsert(row)>>20==1}
    // java public synchronized void removeRow(int row) {
    pub fn removeRow(&self,row:i32){
        // java int pos = this.findRowInsert(row); int mode = pos >> 20; if (mode != 0) {
        let mut ranges=self.ranges.borrow_mut();let pos=Self::findRowInsertIn(&ranges,row);let mode=pos>>20;if mode!=0{
            // java int findPos = pos & 1048575; long pair = this.ranges.get(findPos); int first = getFirst(pair); int second = getSecond(pair);
            let findPos=pos&1048575;let pair=Self::get(&ranges,findPos);let first=Self::getFirst(pair);let second=Self::getSecond(pair);
            // java if (first == row) {
            if first==row{
                // java if (row == second) { this.ranges.removeAt(findPos); } else { this.ranges.set(findPos, makePair(row + 1, second)); }
                if row==second{Self::removeAt(&mut ranges,findPos);}else{Self::set(&mut ranges,findPos,Self::makePair(row.wrapping_add(1),second));}
            // java } else if (row == second) { this.ranges.set(findPos, makePair(first, row - 1)); }
            }else if row==second{Self::set(&mut ranges,findPos,Self::makePair(first,row.wrapping_sub(1)));}
            // java else { this.ranges.set(findPos, makePair(first, row - 1)); this.ranges.insert(findPos + 1, makePair(row + 1, second)); }
            else{Self::set(&mut ranges,findPos,Self::makePair(first,row.wrapping_sub(1)));Self::insert(&mut ranges,findPos+1,Self::makePair(row.wrapping_add(1),second));}
        }
    }
    // java public String toString() {
    pub fn toString(&self)->String{
        // java StringBuilder stringBuilder = new StringBuilder(); int size = this.ranges.size(); for(int i = 0; i < size; ++i) {
        let mut stringBuilder=String::new();let ranges=self.ranges.borrow();let size=ranges.len();for i in 0..size{
            // java long pair = this.ranges.get(i); stringBuilder.append("[").append(getFirst(pair)).append(",").append(getSecond(pair)).append("]");
            let pair=ranges[i];stringBuilder.push_str(&format!("[{},{}]",Self::getFirst(pair),Self::getSecond(pair)));
        }
        // java return stringBuilder.toString();
        stringBuilder
    }
    // java private static long makePair(int first, int second) { return ((long)first << 32) + (long)second; }
    fn makePair(first:i32,second:i32)->i64{((first as i64)<<32).wrapping_add(second as i64)}
    // java private static int getFirst(long pair) { return (int)(pair >> 32); }
    fn getFirst(pair:i64)->i32{(pair>>32)as i32}
    // java private static int getSecond(long pair) { return (int)(pair & 1048575L); }
    fn getSecond(pair:i64)->i32{(pair&1048575)as i32}
    // java private int findRowInsert(int row) {
    fn findRowInsert(&self,row:i32)->i32{Self::findRowInsertIn(&self.ranges.borrow(),row)}
    // Native lock adapter: same original method body while synchronized callers own ranges.
    fn findRowInsertIn(ranges:&[i64],row:i32)->i32{
        // java int left = 0; int right = this.ranges.size(); while(left < right) {
        let mut left=0i32;let mut right=ranges.len()as i32;while left<right{
            // java int mid = left + right >> 1; long pair = this.ranges.get(mid); int first = getFirst(pair); int second = getSecond(pair);
            let mid=left.wrapping_add(right)>>1;let pair=Self::get(ranges,mid);let first=Self::getFirst(pair);let second=Self::getSecond(pair);
            // java if (row < first) { right = mid; } else {
            if row<first{right=mid;}else{
                // java if (row <= second) { return 1048576 | mid & 1048575; } left = mid + 1;
                if row<=second{return 1048576|mid&1048575;}left=mid+1;
            }
        }
        // java return left & 1048575;
        left&1048575
    }
    // java public int getRowCount(int rowIndex) {
    pub fn getRowCount(&self,rowIndex:i32)->i32{
        // java int count = 0; int size = this.ranges.size(); for(int i = 0; i < size; ++i) {
        let mut count=0i32;let ranges=self.ranges.borrow();let size=ranges.len();for i in 0..size{
            // java long pair = this.ranges.get(i); int first = getFirst(pair); int second = getSecond(pair);
            let pair=ranges[i];let first=Self::getFirst(pair);let second=Self::getSecond(pair);
            // java if (second < rowIndex) { count = count + (second - first) + 1; } else {
            if second<rowIndex{count=count.wrapping_add(second.wrapping_sub(first)).wrapping_add(1);}else{
                // java if (first > rowIndex) { break; } count = count + (rowIndex - first) + 1;
                if first>rowIndex{break;}count=count.wrapping_add(rowIndex.wrapping_sub(first)).wrapping_add(1);
            }
        }
        // java return count;
        count
    }
    // java public boolean isEmpty() { return this.ranges.isEmpty(); }
    pub fn isEmpty(&self)->bool{self.ranges.borrow().is_empty()}
    // java public int getRangeSize() { return this.ranges.size(); }
    pub fn getRangeSize(&self)->i32{self.ranges.borrow().len()as i32}
    // java public Iterator<KmoPair<Integer, Integer>> iterator() { return new Iterator<KmoPair<Integer, Integer>>() {
    pub fn iterator(&self)->KmoRowRangeIterator{KmoRowRangeIterator{ranges:self.ranges.clone(),curIndex:0}}
    // Native primitive List adapters preserve WLongArrayList bounds exceptions.
    fn get(ranges:&[i64],i:i32)->i64{*ranges.get(i as usize).unwrap_or_else(||panic!("ArrayIndexOutOfBoundsException"))}
    fn set(ranges:&mut[i64],i:i32,v:i64){*ranges.get_mut(i as usize).unwrap_or_else(||panic!("ArrayIndexOutOfBoundsException"))=v;}
    fn removeAt(ranges:&mut Vec<i64>,i:i32){if i<0||i as usize>=ranges.len(){panic!("ArrayIndexOutOfBoundsException");}ranges.remove(i as usize);}
    fn insert(ranges:&mut Vec<i64>,i:i32,v:i64){if i<0||i as usize>ranges.len(){panic!("ArrayIndexOutOfBoundsException");}ranges.insert(i as usize,v);}
}
// Native receiver for the original anonymous Iterator; retains the live List.
pub struct KmoRowRangeIterator{ranges:Rc<RefCell<Vec<i64>>>,curIndex:i32}
impl KmoRowRangeIterator{
    // java public boolean hasNext() { return this.curIndex < KmoRowRange.this.ranges.size(); }
    pub fn hasNext(&self)->bool{self.curIndex<(self.ranges.borrow().len()as i32)}
    // java public KmoPair<Integer, Integer> next() {
    pub fn next(&mut self)->KmoPair<i32,i32>{
        // java long pair = KmoRowRange.this.ranges.get(this.curIndex++);
        let i=self.curIndex;self.curIndex=self.curIndex.wrapping_add(1);let pair=KmoRowRange::get(&self.ranges.borrow(),i);
        // java return KmoPair.<Integer, Integer>of(KmoRowRange.getFirst(pair), KmoRowRange.getSecond(pair));
        KmoPair::of(KmoRowRange::getFirst(pair),KmoRowRange::getSecond(pair))
    }
}
