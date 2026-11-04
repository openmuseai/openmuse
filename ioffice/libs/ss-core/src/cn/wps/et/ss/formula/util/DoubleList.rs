// java public class DoubleList implements Cloneable
#[derive(Clone)]pub struct DoubleList{array:Vec<f64>,count:usize}impl DoubleList{
// java public DoubleList() {
// java this._array = EMPTY_DOUBLE_ARRAY;
// java this._count = 0;
// java }
pub fn new()->Self {
Self{array:vec![],count:0}
}
// java public DoubleList(int initCap) {
// java if (initCap > 0) {
// java this._array = new double[initCap];
// java } else {
// java this._array = EMPTY_DOUBLE_ARRAY;
// java }
// java
// java this._count = 0;
// java }
pub fn withCapacity(cap:i32)->Self {
Self{array:vec![0.0;cap.max(0)as usize],count:0}
}
// java private void ensureCapacity(int reqSize) {
// java if (reqSize > this._array.length) {
// java int newCap = this._array.length * 3 / 2;
// java if (reqSize > newCap) {
// java newCap = reqSize;
// java }
// java
// java double[] newArr = new double[newCap];
// java System.arraycopy(this._array, 0, newArr, 0, this._count);
// java this._array = newArr;
// java }
// java }
pub fn ensureCapacity(&mut self,n:usize) {
if n>self.array.len(){let cap=(self.array.len()*3/2).max(n);self.array.resize(cap,0.0);}
}
// java public void add(double value) {
// java this.ensureCapacity(this._count + 1);
// java this._array[this._count++] = value;
// java }
pub fn add(&mut self,v:f64) {
self.ensureCapacity(self.count+1);self.array[self.count]=v;self.count+=1;
}
// java public double get(int index) {
// java if (index >= 0 && index < this._count) {
// java return this._array[index];
// java } else {
// java throw new IndexOutOfBoundsException("index " + index + " size " + this._count);
// java }
// java }
pub fn get(&self,i:i32)->f64 {
assert!(i>=0&&(i as usize)<self.count,"IndexOutOfBoundsException");self.array[i as usize]
}
// java public void set(int index, double value) {
// java if (index >= 0 && index < this._count) {
// java this._array[index] = value;
// java } else {
// java throw new IndexOutOfBoundsException("index " + index + " size " + this._count);
// java }
// java }
pub fn set(&mut self,i:i32,v:f64) {
assert!(i>=0&&(i as usize)<self.count,"IndexOutOfBoundsException");self.array[i as usize]=v;
}
// java public int size() {
// java return this._count;
// java }
pub fn size(&self)->i32 {
self.count as i32
}
// java public double[] innerArray() {
// java return this._array;
// java }
pub fn innerArray(&mut self)->&mut [f64] {
&mut self.array
}
// java public double[] toArray() {
// java if (this._count < 1) {
// java return EMPTY_DOUBLE_ARRAY;
// java } else {
// java double[] result = new double[this._count];
// java System.arraycopy(this._array, 0, result, 0, this._count);
// java return result;
// java }
// java }
pub fn toArray(&self)->Vec<f64> {
self.array[..self.count].to_vec()
}
}
impl DoubleList{// java public void addAll(DoubleList dl) {
// java this.ensureCapacity(this._count + dl._count);
// java System.arraycopy(dl._array, 0, this._array, this._count, dl._count);
// java this._count += dl._count;
// java }
pub fn addAll(&mut self,other:&Self) {
self.ensureCapacity(self.count+other.count);self.array[self.count..self.count+other.count].copy_from_slice(&other.array[..other.count]);self.count+=other.count;
}
}
