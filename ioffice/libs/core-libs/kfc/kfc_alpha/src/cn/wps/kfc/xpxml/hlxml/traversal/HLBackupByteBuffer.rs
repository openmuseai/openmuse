// java public final class HLBackupByteBuffer
pub struct HLBackupByteBuffer {_buffer:Vec<u8>,_size:usize,_top:usize,_pos:i32}
impl HLBackupByteBuffer {
// java public HLBackupByteBuffer(int size) {
// java super();
// java this._buffer = new byte[size];
// java this._size = size;
// java this._top = 0;
// java }
pub fn new(size:usize)->Self {
Self{_buffer:vec![0;size],_size:size,_top:0,_pos:0}
}
// java public void put(byte[] chars, int start, int count) {
// java if (chars.length < start + count) {
// java throw new ArrayIndexOutOfBoundsException();
// java } else {
// java if (this._top + count >= this._size) {
// java int newSize = (this._top + count) * 2;
// java byte[] tmp = new byte[newSize];
// java System.arraycopy(this._buffer, 0, tmp, 0, this._top);
// java this._buffer = tmp;
// java this._size = newSize;
// java }
// java
// java System.arraycopy(chars, start, this._buffer, this._top, count);
// java this._top += count;
// java }
// java }
pub fn put(&mut self,chars:&[u8],start:usize,count:usize) {
assert!(chars.len()>=start+count,"ArrayIndexOutOfBoundsException");if self._top+count>=self._size{let new_size=(self._top+count)*2;self._buffer.resize(new_size,0);self._size=new_size;}self._buffer[self._top..self._top+count].copy_from_slice(&chars[start..start+count]);self._top+=count;
}
// java public void putAtBegin(byte[] chars, int start, int count) {
// java if (count != 0) {
// java if (this._top == 0) {
// java this.put(chars, start, count);
// java } else {
// java if (this._top + count >= this._size) {
// java int newSize = (this._top + count) * 2;
// java byte[] tmp = new byte[newSize];
// java System.arraycopy(this._buffer, 0, tmp, count, this._top);
// java this._buffer = tmp;
// java this._size = newSize;
// java } else {
// java System.arraycopy(this._buffer, 0, this._buffer, count, this._top);
// java }
// java
// java this._top += count;
// java System.arraycopy(chars, start, this._buffer, 0, count);
// java }
// java }
// java }
pub fn putAtBegin(&mut self,chars:&[u8],start:usize,count:usize) {
if count!=0{if self._top==0{self.put(chars,start,count);}else{if self._top+count>=self._size{self._size=(self._top+count)*2;self._buffer.resize(self._size,0);}self._buffer.copy_within(0..self._top,count);self._top+=count;self._buffer[..count].copy_from_slice(&chars[start..start+count]);}}
}
// java public byte get(int index) {
// java if (index >= this._top) {
// java throw new ArrayIndexOutOfBoundsException();
// java } else {
// java return this._buffer[index];
// java }
// java }
pub fn get(&self,index:usize)->u8 {
assert!(index<self._top,"ArrayIndexOutOfBoundsException");self._buffer[index]
}
// java public void clear() {
// java this._top = 0;
// java }
pub fn clear(&mut self) {
self._top=0;
}
// java public byte[] buffer() {
// java return this._buffer;
// java }
pub fn buffer(&self)->&[u8] {
&self._buffer
}
// java public int top() {
// java return this._top;
// java }
pub fn top(&self)->usize {
self._top
}
// java public void setTop(int top) {
// java this._top = top;
// java }
pub fn setTop(&mut self,top:usize) {
self._top=top;
}
// java public int getPos() {
// java return this._pos;
// java }
pub fn getPos(&self)->i32 {
self._pos
}
// java public void setPos(int pos) {
// java this._pos = pos;
// java }
pub fn setPos(&mut self,pos:i32) {
self._pos=pos;
}
}
