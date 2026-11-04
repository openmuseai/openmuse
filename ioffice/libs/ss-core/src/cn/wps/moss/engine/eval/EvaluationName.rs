use crate::cn::wps::{moss::engine::linktb::NameRecord::{NameRecordRef,PtgArray},et::ss::{interfaces::{IEvalName::IEvalName,IEvalNameBase::IEvalNameBase},formula::ptg::Ptg::{Ptg,PtgKind,name_ptg,name_x_ptg}}};
// java public final class EvaluationName implements IEvalName {
pub struct EvaluationName{
    // java private final NameRecord nameRecord;
    nameRecord:Option<NameRecordRef>,
    // java private final int nameIndex;
    nameIndex:i32,
}
impl EvaluationName{
    // java public EvaluationName(NameRecord record,int nameIndex){this.nameRecord=record;this.nameIndex=nameIndex;}
    pub fn EvaluationName(record:Option<NameRecordRef>,nameIndex:i32)->Self{Self{nameRecord:record,nameIndex}}
    // java public Ptg[] getNameDefinition(){return this.nameRecord.getNameDefinition();}
    pub fn getNameDefinition(&self)->PtgArray{self.nameRecord.as_ref().expect("NullPointerException").borrow_mut().getNameDefinition()}
    // java public String getNameText(){return this.nameRecord.getNameText();}
    pub fn getNameText(&self)->String{self.nameRecord.as_ref().expect("NullPointerException").borrow().getNameText().into()}
    // java public boolean hasFormula(){return this.nameRecord.hasFormula();}
    pub fn hasFormula(&self)->bool{self.nameRecord.as_ref().expect("NullPointerException").borrow().hasFormula()}
    // java public boolean isFunctionName(){if(this.nameRecord.isFunctionName()){return true;}else if(!this.nameRecord.isHiddenName()){return false;}else if(!this.nameRecord.getNameText().toUpperCase().startsWith("_XLFN.")){return false;}else{Ptg[] ptgs=this.nameRecord.getNameDefinition();if(ptgs!=null&&ptgs.length==1&&ptgs[0].getSid()==28){return ((ErrPtg)ptgs[0]).getErrorCode()==29;}else{return false;}}}
    pub fn isFunctionName(&self)->bool{let record=self.nameRecord.as_ref().expect("NullPointerException");if record.borrow().isFunctionName(){true}else if !record.borrow().isHiddenName(){false}else if !record.borrow().getNameText().to_uppercase().starts_with("_XLFN."){false}else{let ptgs=record.borrow_mut().getNameDefinition();let ptgs=ptgs.borrow();if ptgs.len()==1{let ptg=ptgs[0].as_ref().expect("NullPointerException").borrow();if ptg.getSid()==28{let PtgKind::Err(error)=ptg.kind else{panic!("ClassCastException")};error==29}else{false}}else{false}}}
    // java public NamePtg createNamePtg(){return new NamePtg(this.nameIndex);}
    pub fn createNamePtg(&self)->Ptg{name_ptg(self.nameIndex)}
    // java public NameXPtg createNameXPtg(int externSheetRef){return new NameXPtg(externSheetRef,this.nameIndex);}
    pub fn createNameXPtg(&self,externSheetRef:i32)->Ptg{name_x_ptg(externSheetRef,self.nameIndex)}
    // java public int getSheetIndexOneBased(){return this.nameRecord.getSheetNumber();}
    pub fn getSheetIndexOneBased(&self)->i32{self.nameRecord.as_ref().expect("NullPointerException").borrow().getSheetNumber()}
    // java public String toString(){StringBuilder sb=new StringBuilder();sb.append(this.getClass().getSimpleName());sb.append(" [");sb.append(this.nameRecord.getNameText());sb.append("]");return sb.toString();}
    pub fn toString(&self)->String{let mut sb=String::new();sb.push_str("EvaluationName");sb.push_str(" [");sb.push_str(self.nameRecord.as_ref().expect("NullPointerException").borrow().getNameText());sb.push(']');sb}
}
impl IEvalNameBase for EvaluationName{
    fn getNameText(&self)->String{EvaluationName::getNameText(self)}
    fn getSheetIndexOneBased(&self)->i32{EvaluationName::getSheetIndexOneBased(self)}
    fn as_any(&self)->&dyn std::any::Any{self}
}
impl IEvalName for EvaluationName{
    fn isFunctionName(&self)->bool{EvaluationName::isFunctionName(self)}
    fn hasFormula(&self)->bool{EvaluationName::hasFormula(self)}
    fn getNameDefinition(&self)->PtgArray{EvaluationName::getNameDefinition(self)}
    fn createNamePtg(&self)->Ptg{EvaluationName::createNamePtg(self)}
    fn createNameXPtg(&self,e:i32)->Ptg{EvaluationName::createNameXPtg(self,e)}
}
