use crate::java_runtime::Cell;
use super::CfBasedValue::CfBasedValue;
use super::super::rule::KmoIconSet::IconSetTypes;
// java public class CfColorScaleValue extends CfBasedValue implements Cloneable
pub struct CfColorScaleValue{base:CfBasedValue,color:Cell<i32>}
impl std::ops::Deref for CfColorScaleValue{type Target=CfBasedValue;fn deref(&self)->&CfBasedValue{&self.base}}
impl CfColorScaleValue{
pub fn new()->Self{Self{base:CfBasedValue::new(),color:Cell::new(0)}}
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
// java public boolean equals(Object o) {
// java if (!(o instanceof CfColorScaleValue)) {
// java return false;
// java } else {
// java CfColorScaleValue result = (CfColorScaleValue)o;
// java return result.color == this.color && result.priority == this.priority;
// java }
// java }
pub fn equals(&self,other:&Self)->bool {
self.getPriority()==other.getPriority()&&self.color.get()==other.color.get()
}
}
