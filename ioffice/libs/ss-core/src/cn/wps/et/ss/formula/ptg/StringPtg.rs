// java public final class StringPtg extends ScalarConstantPtg {
use super::Ptg::{Ptg,PtgKind};
impl Ptg{
    // java public String getValue(){return this.field_3_string;}
    pub fn getStringValue(&self)->String{match &self.kind{PtgKind::Str(s)=>s.clone(),PtgKind::StrRaw{units,..}=>String::from_utf16(units).expect("native UTF16 String output boundary: isolated surrogate"),_=>panic!("ClassCastException: StringPtg")}}
    // java public String toFormulaString(){
    pub fn toStringFormulaString(&self)->String{
        // java String value=this.field_3_string;int len=value.length();StringBuffer sb=new StringBuffer(len+4);sb.append('"');
        let value=self.getStringValue();let len=value.encode_utf16().count();let mut sb=String::with_capacity(len+4);sb.push('"');
        // java for(int i=0;i<len;++i){char c=value.charAt(i);if(c=='"'){sb.append('"');}sb.append(c);}
        // rust: valid String scalars append exactly the original UTF16 units; delimiter is BMP.
        for c in value.chars(){if c=='"'{sb.push('"');}sb.push(c);}
        // java sb.append('"');return sb.toString();
        sb.push('"');sb
    }
}
