// java package cn.wps.et.ss.formula.functions.util.matcher;
// java
// java public final class CmpOp {
#[derive(Clone,Copy,PartialEq,Eq)]pub struct CmpOp{code:i32,rep:&'static str}impl CmpOp{pub const NONE:i32=0;pub const OP_NONE:Self=Self{code:0,rep:""};
pub const EQ:i32=1;pub const OP_EQ:Self=Self{code:1,rep:"="};
pub const NE:i32=2;pub const OP_NE:Self=Self{code:2,rep:"<>"};
pub const LE:i32=3;pub const OP_LE:Self=Self{code:3,rep:"<="};
pub const LT:i32=4;pub const OP_LT:Self=Self{code:4,rep:"<"};
pub const GT:i32=5;pub const OP_GT:Self=Self{code:5,rep:">"};
pub const GE:i32=6;pub const OP_GE:Self=Self{code:6,rep:">="};
// java public int getLength() {
// java return this._representation.length();
// java }
pub fn getLength(&self)->usize {
self.rep.len()
}
// java public int getCode() {
// java return this._code;
// java }
pub fn getCode(&self)->i32 {
self.code
}
// java public String getRepresentation() {
// java return this._representation;
// java }
pub fn getRepresentation(&self)->&str {
self.rep
}
// java public static CmpOp getOperator(String value) {
// java int len = value.length();
// java if (len < 1) {
// java return OP_NONE;
// java } else {
// java char firstChar = value.charAt(0);
// java switch (firstChar) {
// java case '<':
// java if (len > 1) {
// java switch (value.charAt(1)) {
// java case '=':
// java return OP_LE;
// java case '>':
// java return OP_NE;
// java }
// java }
// java
// java return OP_LT;
// java case '=':
// java return OP_EQ;
// java case '>':
// java if (len > 1) {
// java switch (value.charAt(1)) {
// java case '=':
// java return OP_GE;
// java }
// java }
// java
// java return OP_GT;
// java default:
// java return OP_NONE;
// java }
// java }
// java }
pub fn getOperator(v:&[u16])->Self {
match v.first(){Some(60)=>match v.get(1){Some(61)=>Self::OP_LE,Some(62)=>Self::OP_NE,_=>Self::OP_LT},Some(61)=>Self::OP_EQ,Some(62)=>if v.get(1)==Some(&61){Self::OP_GE}else{Self::OP_GT},_=>Self::OP_NONE}
}
// java public boolean evaluate(boolean cmpResult) {
// java switch (this._code) {
// java case 0:
// java case 1:
// java return cmpResult;
// java case 2:
// java return !cmpResult;
// java default:
// java throw new RuntimeException("Cannot call boolean evaluate on non-equality operator '" + this._representation + "'");
// java }
// java }
pub fn evaluateBool(&self,b:bool)->bool {
match self.code{0|1=>b,2=>!b,_=>panic!("Cannot call boolean evaluate on non-equality operator")}
}
// java public boolean evaluate(int cmpResult) {
// java switch (this._code) {
// java case 0:
// java case 1:
// java return cmpResult == 0;
// java case 2:
// java return cmpResult != 0;
// java case 3:
// java return cmpResult <= 0;
// java case 4:
// java return cmpResult < 0;
// java case 5:
// java return cmpResult > 0;
// java case 6:
// java return cmpResult >= 0;
// java default:
// java throw new RuntimeException("Cannot call boolean evaluate on non-equality operator '" + this._representation + "'");
// java }
// java }
pub fn evaluate(&self,c:i32)->bool {
match self.code{0|1=>c==0,2=>c!=0,3=>c<=0,4=>c<0,5=>c>0,6=>c>=0,_=>panic!("CmpOp")}
}
}