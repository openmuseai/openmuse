use crate::java_runtime::Cell;
use super::CfBasedValue::CfBasedValue;
use super::super::rule::KmoIconSet::IconSetTypes;
// java public class CfDataBarValue extends CfBasedValue implements Cloneable
pub struct CfDataBarValue{base:CfBasedValue,length:Cell<i32>,color:Cell<i32>,dir:Cell<i32>,showValue:Cell<bool>}
impl std::ops::Deref for CfDataBarValue{type Target=CfBasedValue;fn deref(&self)->&CfBasedValue{&self.base}}
impl CfDataBarValue{
pub fn new()->Self{Self{base:CfBasedValue::new(),length:Cell::new(0),color:Cell::new(0),dir:Cell::new(0),showValue:Cell::new(false)}}
// java public int getLength() {
// java return this.length;
// java }
pub fn getLength(&self)->i32 {
self.length.get()
}
// java public void setLength(int length) {
// java this.length = length;
// java }
pub fn setLength(&self,v:i32) {
self.length.set(v);
}
// java public int getColor() {
// java return this.color;
// java }
pub fn getColor(&self)->i32 {
self.color.get()
}
// java public void setColor(int color) {
// java this.color = color;
// java }
pub fn setColor(&self,v:i32) {
self.color.set(v);
}
// java public int getDir() {
// java return this.dir;
// java }
pub fn getDir(&self)->i32 {
self.dir.get()
}
// java public void setDir(int dir) {
// java this.dir = dir;
// java }
pub fn setDir(&self,v:i32) {
self.dir.set(v);
}
// java public boolean isShowValue() {
// java return this.showValue;
// java }
pub fn isShowValue(&self)->bool {
self.showValue.get()
}
// java public void setShowValue(boolean showValue) {
// java this.showValue = showValue;
// java }
pub fn setShowValue(&self,v:bool) {
self.showValue.set(v);
}
// java public boolean equals(Object o) {
// java if (!(o instanceof CfDataBarValue)) {
// java return false;
// java } else {
// java CfDataBarValue result = (CfDataBarValue)o;
// java return result.length == this.length && result.color == this.color && result.dir == this.dir && result.showValue == this.showValue && result.priority == this.priority;
// java }
// java }
pub fn equals(&self,other:&Self)->bool {
self.getPriority()==other.getPriority()&&self.length.get()==other.length.get()&&self.color.get()==other.color.get()&&self.dir.get()==other.dir.get()&&self.showValue.get()==other.showValue.get()
}
}
