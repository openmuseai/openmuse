// java package cn.wps.et.ss.formula.eval;
// java
// java public final class UDFNameEval implements IValueEval {
// java private final String _functionName;
// java
// java public UDFNameEval(String functionName) {
// java this._functionName = functionName;
// java }
// java
// java public String getFunctionName() {
// java return this._functionName;
// java }
// java
// java public String toString() {
// java StringBuffer sb = new StringBuffer(64);
// java sb.append(this.getClass().getName()).append(" [");
// java sb.append(this._functionName);
// java sb.append("]");
// java return sb.toString();
// java }
// java }
use super::IValueEval;pub struct UDFNameEval{functionName:String}impl UDFNameEval{pub fn new(name:String)->Self{Self{functionName:name}}pub fn getFunctionName(&self)->&str{&self.functionName}}impl IValueEval for UDFNameEval{fn as_any(&self)->&dyn std::any::Any{self}}
