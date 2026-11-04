use super::super::statemachine::{RecogniseResultRl::{RecogniseResultRl,Value},ValueType::ValueType};
// java public class RecogniseResult
pub struct RecogniseResult{flags:i16,suggestedNumfmt:Option<String>,builtinNumfmtIdx:i32,value:Option<Value>}
impl RecogniseResult{pub fn fromValue(value:Value)->Self{Self{flags:0,suggestedNumfmt:None,builtinNumfmtIdx:-1,value:Some(value)}}
// java public RecogniseResult(RecogniseResultRl rl) {
// java this.value = rl.getValue();
// java this.suggestedNumfmt = rl.getSuggestedNumfmt();
// java this.builtinNumfmtIdx = rl.getBuiltInNumfmtIdx();
// java switch (rl.getValueType()) {
// java case BOOLEAN:
// java this.setHyperlinkFlag();
// java break;
// java case ERROR:
// java this.setErrorFlag();
// java break;
// java case FRACTION:
// java this.setFractionFlag();
// java break;
// java case PERCENT:
// java this.setPercentageFlage();
// java break;
// java case SENTIC:
// java this.setSenticFlag();
// java case NUMBER:
// java break;
// java case CURRENCY:
// java this.setCurrencyFlag();
// java break;
// java case DATE:
// java this.setDateFlag();
// java break;
// java case TIME:
// java this.setTimeFlag();
// java break;
// java case DATETIME:
// java this.setDateFlag();
// java this.setTimeFlag();
// java break;
// java case STRING:
// java if (rl.hasSqleading()) {
// java this.setSqleadingFlag();
// java }
// java break;
// java case HYPERLINK:
// java this.setHyperlinkFlag();
// java break;
// java default:
// java throw new RuntimeException("Unexpected");
// java }
// java
// java }
pub fn fromState(rl:RecogniseResultRl)->Self {
let mut ret=Self{flags:0,value:rl.getValue(),suggestedNumfmt:rl.getSuggestedNumfmt(),builtinNumfmtIdx:rl.getBuiltInNumfmtIdx()};match rl.getValueType().unwrap(){ValueType::BOOLEAN|ValueType::HYPERLINK=>ret.setHyperlinkFlag(),ValueType::ERROR=>ret.setErrorFlag(),ValueType::FRACTION=>ret.setFractionFlag(),ValueType::PERCENT=>ret.setPercentageFlage(),ValueType::SENTIC=>ret.setSenticFlag(),ValueType::NUMBER=>{},ValueType::CURRENCY=>ret.setCurrencyFlag(),ValueType::DATE=>ret.setDateFlag(),ValueType::TIME=>ret.setTimeFlag(),ValueType::DATETIME=>{ret.setDateFlag();ret.setTimeFlag();},ValueType::STRING=>{if rl.sqleading{ret.setSqleadingFlag();}}}ret
}
// java public boolean hasGroupedFlag() {
// java return grouped.isSet(this.flags);
// java }
pub fn hasGroupedFlag(&self)->bool {
self.flags&1!=0
}
// java public void setGroupedFlag() {
// java this.flags = grouped.setShortBoolean(this.flags, true);
// java }
pub fn setGroupedFlag(&mut self) {
self.flags|=1;
}
// java public boolean hasDateFlag() {
// java return date.isSet(this.flags);
// java }
pub fn hasDateFlag(&self)->bool {
self.flags&2!=0
}
// java public void setDateFlag() {
// java this.flags = date.setShortBoolean(this.flags, true);
// java }
pub fn setDateFlag(&mut self) {
self.flags|=2;
}
// java public boolean hasTimeFlag() {
// java return time.isSet(this.flags);
// java }
pub fn hasTimeFlag(&self)->bool {
self.flags&4!=0
}
// java public void setTimeFlag() {
// java this.flags = time.setShortBoolean(this.flags, true);
// java }
pub fn setTimeFlag(&mut self) {
self.flags|=4;
}
// java public boolean hasCurrencyFlag() {
// java return currency.isSet(this.flags);
// java }
pub fn hasCurrencyFlag(&self)->bool {
self.flags&8!=0
}
// java public void setCurrencyFlag() {
// java this.flags = currency.setShortBoolean(this.flags, true);
// java }
pub fn setCurrencyFlag(&mut self) {
self.flags|=8;
}
// java public boolean hasErrorFlag() {
// java return error.isSet(this.flags);
// java }
pub fn hasErrorFlag(&self)->bool {
self.flags&16!=0
}
// java void setErrorFlag() {
// java this.flags = error.setShortBoolean(this.flags, true);
// java }
pub fn setErrorFlag(&mut self) {
self.flags|=16;
}
// java public boolean hasSqleadingFlag() {
// java return sqleading.isSet(this.flags);
// java }
pub fn hasSqleadingFlag(&self)->bool {
self.flags&32!=0
}
// java void setSqleadingFlag() {
// java this.flags = sqleading.setShortBoolean(this.flags, true);
// java }
pub fn setSqleadingFlag(&mut self) {
self.flags|=32;
}
// java public boolean hasPercentageFlag() {
// java return percentage.isSet(this.flags);
// java }
pub fn hasPercentageFlag(&self)->bool {
self.flags&64!=0
}
// java void setPercentageFlage() {
// java this.flags = percentage.setShortBoolean(this.flags, true);
// java }
pub fn setPercentageFlage(&mut self) {
self.flags|=64;
}
// java public boolean hasFractionFlag() {
// java return fraction.isSet(this.flags);
// java }
pub fn hasFractionFlag(&self)->bool {
self.flags&128!=0
}
// java public void setFractionFlag() {
// java this.flags = fraction.setShortBoolean(this.flags, true);
// java }
pub fn setFractionFlag(&mut self) {
self.flags|=128;
}
// java public boolean hasHyperlinkFlag() {
// java return hyperlink.isSet(this.flags);
// java }
pub fn hasHyperlinkFlag(&self)->bool {
self.flags&256!=0
}
// java void setHyperlinkFlag() {
// java this.flags = hyperlink.setShortBoolean(this.flags, true);
// java }
pub fn setHyperlinkFlag(&mut self) {
self.flags|=256;
}
// java public boolean hasSenticFlag() {
// java return sentic.isSet(this.flags);
// java }
pub fn hasSenticFlag(&self)->bool {
self.flags&512!=0
}
// java public void setSenticFlag() {
// java this.flags = sentic.setShortBoolean(this.flags, true);
// java }
pub fn setSenticFlag(&mut self) {
self.flags|=512;
}
// java public Object getValue() {
// java return this.value;
// java }
pub fn getValue(&self)->Option<Value> {
self.value.clone()
}
// java public Double getDblValue() {
// java return (Double)this.value;
// java }
pub fn getDblValue(&self)->Option<f64> {
match self.value.as_ref(){None=>None,Some(Value::Number(v))=>Some(*v),_=>panic!("ClassCastException Double")}
}
// java public void setSuggestedNumfmt(int numfmt) {
// java this.builtinNumfmtIdx = numfmt;
// java }
pub fn setSuggestedNumfmtI(&mut self,v:i32) {
self.builtinNumfmtIdx=v;
}

// rust: Java overloaded constructors share the same field initializer and assignment.
// java public RecogniseResult(String v) { this.value = v; }
pub fn RecogniseResult(v: Option<Vec<u16>>) -> Self {
    Self { flags: 0, suggestedNumfmt: None, builtinNumfmtIdx: -1, value: v.map(Value::Text) }
}
// java public RecogniseResult(Double v) { this.value = v; }
pub fn RecogniseResult__2(v: Option<f64>) -> Self {
    Self { flags: 0, suggestedNumfmt: None, builtinNumfmtIdx: -1, value: v.map(Value::Number) }
}
// java public RecogniseResult(Boolean v) { this.value = v; }
pub fn RecogniseResult__3(v: Option<bool>) -> Self {
    Self { flags: 0, suggestedNumfmt: None, builtinNumfmtIdx: -1, value: v.map(Value::Bool) }
}
// java public RecogniseResult(RecogniseResultRl rl) { ... }
pub fn RecogniseResult__4(rl: RecogniseResultRl) -> Self { Self::fromState(rl) }

// java public String getSuggestedNumfmt() { return this.suggestedNumfmt; }
pub fn getSuggestedNumfmt(&self) -> Option<String> { self.suggestedNumfmt.clone() }
// java public int getBuiltInNumfmtIdx() { return this.builtinNumfmtIdx; }
pub fn getBuiltInNumfmtIdx(&self) -> i32 { self.builtinNumfmtIdx }
// java public String getStrValue() { return (String)this.value; }
pub fn getStrValue(&self) -> Option<Vec<u16>> {
    match &self.value { None => None, Some(Value::Text(v)) => Some(v.clone()), _ => panic!("ClassCastException String") }
}
// java public Boolean getBoolValue() { return (Boolean)this.value; }
pub fn getBoolValue(&self) -> Option<bool> {
    match &self.value { None => None, Some(Value::Bool(v)) => Some(*v), _ => panic!("ClassCastException Boolean") }
}
// java public void setSuggestedNumfmt(String fmt) { this.suggestedNumfmt = fmt; }
pub fn setSuggestedNumfmt(&mut self, fmt: Option<String>) { self.suggestedNumfmt = fmt; }
// java public void setSuggestedNumfmt(int numfmt) { this.builtinNumfmtIdx = numfmt; }
pub fn setSuggestedNumfmt__2(&mut self, numfmt: i32) { self.builtinNumfmtIdx = numfmt; }
// java public void setValue(String s) { this.value = s; }
pub fn setValue(&mut self, s: Option<Vec<u16>>) { self.value = s.map(Value::Text); }
// java public void setValue(Double d) { this.value = d; }
pub fn setValue__2(&mut self, d: Option<f64>) { self.value = d.map(Value::Number); }
// java public void setValue(Boolean b) { this.value = b; }
pub fn setValue__3(&mut self, b: Option<bool>) { self.value = b.map(Value::Bool); }
// java public void clear() {
pub fn clear(&mut self) {
    // java this.flags = 0;
    self.flags = 0;
    // java this.value = null;
    self.value = None;
    // java this.suggestedNumfmt = null;
    self.suggestedNumfmt = None;
    // java this.builtinNumfmtIdx = -1;
    self.builtinNumfmtIdx = -1;
}
}
