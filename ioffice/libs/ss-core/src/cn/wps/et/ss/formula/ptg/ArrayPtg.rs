// java public final class ArrayPtg extends Ptg {
// rust: original dimension/value fields borrow the existing serialized token carrier.
use super::Ptg::{Ptg,PtgKind};
use crate::cn::wps::et::ss::formula::constant::{ConstantValueParser::ConstantValue,ErrorConstant::ErrorConstant};
use crate::org::apache::poi::ss::util::NumberToTextConverter::NumberToTextConverter;
impl Ptg{
    // java public String toString() {
    pub fn toArrayDebugString(&self)->String{
        // java StringBuffer sb = new StringBuffer("[ArrayPtg]\n");
        let mut sb=String::from("[ArrayPtg]\n");
        // java sb.append("nRows = ").append(this.getRowCount()).append("\n");
        sb.push_str(&format!("nRows = {}\n",self.getRowCount()));
        // java sb.append("nCols = ").append(this.getColumnCount()).append("\n");
        sb.push_str(&format!("nCols = {}\n",self.getColumnCount()));
        // java if (this._arrayValues == null) { sb.append("  #values#uninitialised#\n"); } else { sb.append("  ").append(this.toFormulaString('.', ',', ';')); }
        let PtgKind::Array{values,..}=&self.kind else{panic!("ClassCastException: ArrayPtg")};
        if values.is_none(){sb.push_str("  #values#uninitialised#\n");}else{sb.push_str("  ");sb.push_str(&self.toArrayFormulaString__separators('.',',',';'));}
        // java return sb.toString();
        sb
    }
    // java int getValueIndex(int colIx,int rowIx){
    pub fn getValueIndex(&self,colIx:i32,rowIx:i32)->i32{
        // java if(colIx>=0&&colIx<this._nColumns){if(rowIx>=0&&rowIx<this._nRows){return rowIx*this._nColumns+colIx;}else{throw new IllegalArgumentException(...);}}else{throw new IllegalArgumentException(...);}
        assert!(colIx>=0&&colIx<self.getColumnCount(),"IllegalArgumentException: colIx");assert!(rowIx>=0&&rowIx<self.getRowCount(),"IllegalArgumentException: rowIx");rowIx.wrapping_mul(self.getColumnCount()).wrapping_add(colIx)
    }
    // java public int getRowCount(){return this._nRows;}
    pub fn getRowCount(&self)->i32{match self.kind{PtgKind::Array{rows,..}=>rows as i32,_=>panic!("ClassCastException: ArrayPtg")}}
    // java public int getColumnCount(){return this._nColumns;}
    pub fn getColumnCount(&self)->i32{match self.kind{PtgKind::Array{columns,..}=>columns as i32,_=>panic!("ClassCastException: ArrayPtg")}}
    // java public String toFormulaString(){throw new RuntimeException("don't call this method");}
    pub fn toArrayFormulaString(&self)->String{panic!("RuntimeException: don't call this method")}
    // java public String toFormulaString(char decimalSeparator,char constArraySeparator,char constArrayRowSeparator){
    pub fn toArrayFormulaString__separators(&self,decimalSeparator:char,constArraySeparator:char,constArrayRowSeparator:char)->String{
        // java StringBuffer b=new StringBuffer();b.append("{");
        let mut b=String::new();b.push('{');
        // java for(int y=0;y<this.getRowCount();++y){if(y>0){b.append(constArrayRowSeparator);}
        for y in 0..self.getRowCount(){if y>0{b.push(constArrayRowSeparator);}
            // java for(int x=0;x<this.getColumnCount();++x){if(x>0){b.append(constArraySeparator);}
            for x in 0..self.getColumnCount(){if x>0{b.push(constArraySeparator);}
                // java Object o=this._arrayValues[this.getValueIndex(x,y)];b.append(getConstantText(o,decimalSeparator));
                let PtgKind::Array{values,..}=&self.kind else{unreachable!()};let o=values.as_ref().expect("NullPointerException").get(self.getValueIndex(x,y)as usize).expect("ArrayIndexOutOfBoundsException");b.push_str(&Self::getConstantText(o,decimalSeparator));
            }
        }
        // java b.append("}");return b.toString();
        b.push('}');b
    }
    // java private static String getConstantText(Object o,char decimalSeparator){
    fn getConstantText(o:&ConstantValue,decimalSeparator:char)->String{match o{
        // java if(o==null){throw new RuntimeException("Array item cannot be null");}
        ConstantValue::Empty=>panic!("RuntimeException: Array item cannot be null"),
        // java else if(o instanceof String){return "\""+(String)o+"\"";}
        ConstantValue::String(s)=>format!("\"{s}\""),ConstantValue::StringUtf16(s)=>format!("\"{}\"",String::from_utf16(s).expect("native UTF16 String output boundary: isolated surrogate")),
        // java else if(o instanceof Double){return NumberToTextConverter.toText((Double)o,decimalSeparator);}
        ConstantValue::Number(n)=>NumberToTextConverter::toText(*n,decimalSeparator),
        // java else if(o instanceof Boolean){return (Boolean)o?"TRUE":"FALSE";}
        ConstantValue::Boolean(b)=>if *b{"TRUE"}else{"FALSE"}.into(),
        // java else if(o instanceof ErrorConstant){return ((ErrorConstant)o).getText();}
        ConstantValue::Error(c)=>ErrorConstant::valueOf(*c as i32).getText(),
        // rust: unexpected Object subclasses remain outside the existing typed ConstantValue transport.
    }}
}
