use crate::java_runtime::Cell;
use super::CfBasedValue::CfBasedValue;
use super::super::rule::KmoIconSet::IconSetTypes;
// java public class CfIconSetValue extends CfBasedValue implements Cloneable
pub struct CfIconSetValue{base:CfBasedValue,itype:Cell<Option<IconSetTypes>>,index:Cell<i32>,showValue:Cell<bool>}
impl std::ops::Deref for CfIconSetValue{type Target=CfBasedValue;fn deref(&self)->&CfBasedValue{&self.base}}
impl CfIconSetValue{
pub fn new()->Self{Self{base:CfBasedValue::new(),itype:Cell::new(None),index:Cell::new(0),showValue:Cell::new(false)}}
// java public KmoIconSet.IconSetTypes getType() {
// java return this.type;
// java }
pub fn getType(&self)->Option<IconSetTypes> {
self.itype.get()
}
// java public void setType(KmoIconSet.IconSetTypes type) {
// java this.type = type;
// java }
pub fn setType(&self,v:Option<IconSetTypes>) {
self.itype.set(v);
}
// java public int getIndex() {
// java return this.index;
// java }
pub fn getIndex(&self)->i32 {
self.index.get()
}
// java public void setIndex(int index) {
// java this.index = index;
// java }
pub fn setIndex(&self,v:i32) {
self.index.set(v);
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
// java if (!(o instanceof CfIconSetValue)) {
// java return false;
// java } else {
// java CfIconSetValue result = (CfIconSetValue)o;
// java return result.type == this.type && result.index == this.index && result.showValue == this.showValue && result.priority == this.priority;
// java }
// java }
pub fn equals(&self,other:&Self)->bool {
self.getPriority()==other.getPriority()&&self.itype.get()==other.itype.get()&&self.index.get()==other.index.get()&&self.showValue.get()==other.showValue.get()
}
}
