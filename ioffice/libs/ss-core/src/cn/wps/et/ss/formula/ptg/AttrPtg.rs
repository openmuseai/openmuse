// java public final class AttrPtg extends ControlPtg {
use super::{Ptg::{Ptg,PtgKind},ValueOperatorPtg::operand};
impl Ptg{
    // rust: borrow the same original AttrPtg._options field through the carrier.
    fn attrOptions(&self)->u8{match self.kind{PtgKind::Attr{options,..}=>options,_=>panic!("ClassCastException: AttrPtg")}}
    // java public boolean isSemiVolatile(){return semiVolatile.isSet(this._options);}
    pub fn isSemiVolatile(&self)->bool{self.attrOptions()&1!=0}
    // java public boolean isOptimizedIf(){return (this._options&2)!=0;}
    pub fn isOptimizedIf(&self)->bool{self.attrOptions()&2!=0}
    // java public boolean isOptimizedChoose(){return (this._options&4)!=0;}
    pub fn isOptimizedChoose(&self)->bool{self.attrOptions()&4!=0}
    // java public boolean isSum(){return (this._options&16)!=0;}
    pub fn isSum(&self)->bool{self.attrOptions()&16!=0}
    // java public boolean isSkip(){return (this._options&8)!=0;}
    pub fn isSkip(&self)->bool{self.attrOptions()&8!=0}
    // java private boolean isBaxcel(){return baxcel.isSet(this._options);}
    fn isBaxcel(&self)->bool{self.attrOptions()&32!=0}
    // java public boolean isSpace(){return space.isSet(this._options);}
    pub fn isSpace(&self)->bool{self.attrOptions()&64!=0}
    // java public short getData(){return this._data;}
    pub fn getAttrData(&self)->i16{match self.kind{PtgKind::Attr{data,..}=>data,_=>panic!("ClassCastException: AttrPtg")}}
    // java public int[] getJumpTable(){return (int[])this._jumpTable.clone();}
    pub fn getJumpTable(&self)->Vec<i32>{let PtgKind::Attr{jump_table,..}=&self.kind else{panic!("ClassCastException: AttrPtg")};jump_table.as_ref().expect("NullPointerException").clone()}
    // java public int getChooseFuncOffset(){if(this._jumpTable==null){throw new IllegalStateException("Not tAttrChoose");}else{return this._chooseFuncOffset;}}
    pub fn getChooseFuncOffset(&self)->i32{let PtgKind::Attr{jump_table,choose_offset,..}=&self.kind else{panic!("ClassCastException: AttrPtg")};assert!(jump_table.is_some(),"IllegalStateException: Not tAttrChoose");*choose_offset}
    // java public boolean isNull(){return this._options==0;}
    pub fn isNull(&self)->bool{self.attrOptions()==0}
    // java public String toString(){StringBuffer sb=new StringBuffer(64);sb.append(this.getClass().getName()).append(" [");
    pub fn attrToString(&self)->String{
        let mut sb=String::with_capacity(64);sb.push_str("cn.wps.et.ss.formula.ptg.AttrPtg [");
        // java if(this.isSemiVolatile()){sb.append("volatile ");}
        if self.isSemiVolatile(){sb.push_str("volatile ");}
        // java if(this.isSpace()){sb.append("space count=").append(this._data>>8&255);sb.append(" type=").append(this._data&255).append(" ");}
        if self.isSpace(){sb.push_str(&format!("space count={} type={} ",(self.getAttrData()as i32>>8)&255,self.getAttrData()as i32&255));}
        // java if(this.isOptimizedIf()){sb.append("if dist=").append(this._data);}else if(this.isOptimizedChoose()){sb.append("choose nCases=").append(this._data);}else if(this.isSkip()){sb.append("skip dist=").append(this._data);}else if(this.isSum()){sb.append("sum ");}else if(this.isBaxcel()){sb.append("assign ");}
        if self.isOptimizedIf(){sb.push_str(&format!("if dist={}",self.getAttrData()));}else if self.isOptimizedChoose(){sb.push_str(&format!("choose nCases={}",self.getAttrData()));}else if self.isSkip(){sb.push_str(&format!("skip dist={}",self.getAttrData()));}else if self.isSum(){sb.push_str("sum ");}else if self.isBaxcel(){sb.push_str("assign ");}
        // java sb.append("]");return sb.toString();
        sb.push(']');sb
    }
    // java public String toFormulaString(String[] operands){
    pub fn toAttrFormulaString__operands(&self,operands:Option<&[Option<String>]>)->Option<String>{
        // java if(space.isSet(this._options)){return operands[0];}
        if self.isSpace(){return operands.expect("NullPointerException").get(0).expect("ArrayIndexOutOfBoundsException").clone();}
        // java else if((this._options&2)!=0){return this.toFormulaString()+"("+operands[0]+")";}
        if self.isOptimizedIf(){Some(format!("{}({})",self.toAttrFormulaString(),operand(operands,0)))}
        // java else{return (this._options&8)!=0?this.toFormulaString()+operands[0]:this.toFormulaString()+"("+operands[0]+")";}
        else if self.isSkip(){Some(format!("{}{}",self.toAttrFormulaString(),operand(operands,0)))}else{Some(format!("{}({})",self.toAttrFormulaString(),operand(operands,0)))}
    }
    // java public int getNumberOfOperands(){return 1;}
    pub fn getAttrOperandCount(&self)->i32{1}
    // java public int getType(){return -1;}
    pub fn getAttrType(&self)->i32{-1}
    // java public String toFormulaString(){
    pub fn toAttrFormulaString(&self)->String{
        // java if(semiVolatile.isSet(this._options)){return "ATTR(semiVolatile)";}else if((this._options&2)!=0){return "IF";}else if((this._options&4)!=0){return "CHOOSE";}else if((this._options&8)!=0){return "";}else if((this._options&16)!=0){return "SUM";}else if(baxcel.isSet(this._options)){return "ATTR(baxcel)";}else{return space.isSet(this._options)?"":"UNKNOWN ATTRIBUTE";}
        if self.isSemiVolatile(){"ATTR(semiVolatile)"}else if self.isOptimizedIf(){"IF"}else if self.isOptimizedChoose(){"CHOOSE"}else if self.isSkip(){""}else if self.isSum(){"SUM"}else if self.isBaxcel(){"ATTR(baxcel)"}else if self.isSpace(){""}else{"UNKNOWN ATTRIBUTE"}.into()
    }
}
