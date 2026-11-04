// java public abstract class AbstractFunctionPtg extends OperationPtg {
// rust: original immutable function fields are read from the existing Ptg carrier.
use super::{Ptg::{Ptg,PtgKind},ValueOperatorPtg::operand};
use crate::cn::wps::et::ss::formula::function::FunctionMetadataRegistry;
impl Ptg{
    // java public final String toString() {
    pub fn toFunctionDebugString(&self)->String{
        // java StringBuilder sb = new StringBuilder(64);
        let mut sb=String::with_capacity(64);
        // java sb.append(this.getClass().getName()).append(" [");
        sb.push_str(&self.javaClassName());sb.push_str(" [");
        // java sb.append(this.lookupName(this._functionIndex));
        sb.push_str(&self.lookupFunctionName(self.getFunctionIndex()));
        // java sb.append(" nArgs=").append(this._numberOfArgs);
        sb.push_str(" nArgs=");sb.push_str(&self.getFunctionOperandCount().to_string());
        // java sb.append("]"); return sb.toString();
        sb.push(']');sb
    }
    // java public final short getFunctionIndex(){return this._functionIndex;}
    pub fn getFunctionIndex(&self)->i16{match self.kind{PtgKind::Func{meta}|PtgKind::FuncVar{meta,..}=>meta.getIndex()as i16,PtgKind::FuncVarExt{index,..}=>index as i16,_=>panic!("ClassCastException: AbstractFunctionPtg")}}
    // java public final int getNumberOfOperands(){return this._numberOfArgs;}
    pub fn getFunctionOperandCount(&self)->i32{match self.kind{PtgKind::Func{meta}=>meta.getMinParams()as i8 as i32,PtgKind::FuncVar{nargs,..}|PtgKind::FuncVarExt{nargs,..}=>nargs as i8 as i32,_=>panic!("ClassCastException: AbstractFunctionPtg")}}
    // java public final String getName(){return this.lookupName(this._functionIndex);}
    pub fn getFunctionName(&self)->String{self.lookupFunctionName(self.getFunctionIndex())}
    // java public final boolean isExternalFunction(){return this._functionIndex==255;}
    pub fn isExternalFunction(&self)->bool{self.getFunctionIndex()==255}
    // java public final String toFormulaString(){return this.getName();}
    pub fn toFunctionFormulaString(&self)->String{self.getFunctionName()}
    // java public String toFormulaString(String[] operands,char argSeparator,char unionSeparator){
    pub fn toFunctionFormulaString__operands(&self,operands:Option<&[Option<String>]>,argSeparator:char,_unionSeparator:char)->String{
        // java StringBuilder buf=new StringBuilder();if(this.isExternalFunction()){buf.append(operands[0]);appendArgs(buf,1,operands,argSeparator);}
        let mut buf=String::new();if self.isExternalFunction(){buf.push_str(operand(operands,0));Self::appendArgs(&mut buf,1,operands,argSeparator);}
        // java else{buf.append(this.getName());appendArgs(buf,0,operands,argSeparator);}return buf.toString();
        else{buf.push_str(&self.getFunctionName());Self::appendArgs(&mut buf,0,operands,argSeparator);}buf
    }
    // java private static void appendArgs(StringBuilder buf,int firstArgIx,String[] operands,char argSeparator){
    fn appendArgs(buf:&mut String,firstArgIx:usize,operands:Option<&[Option<String>]>,argSeparator:char){
        // java buf.append('(');for(int i=firstArgIx;i<operands.length;++i){if(i>firstArgIx){buf.append(argSeparator);}buf.append(operands[i]);}buf.append(")");
        buf.push('(');let operands=operands.expect("NullPointerException");for i in firstArgIx..operands.len(){if i>firstArgIx{buf.push(argSeparator);}buf.push_str(operands[i].as_deref().unwrap_or("null"));}buf.push(')');
    }
    // java protected final String lookupName(short index){
    fn lookupFunctionName(&self,index:i16)->String{
        // java if(index==255){return "#external#";}
        if index==255{return "#external#".into();}
        // java FunctionMetadata fm=FunctionMetadataRegistry.getFunctionByIndex(index);
        assert!(index>=0&&(index as usize)<368,"ArrayIndexOutOfBoundsException");let fm=FunctionMetadataRegistry::getFunctionByIndex(index as i32);
        // java if(fm==null){throw new RuntimeException("bad function index ("+index+")");}else{return fm.getName();}
        fm.unwrap_or_else(||panic!("RuntimeException: bad function index ({index})")).getName().into()
    }
}
