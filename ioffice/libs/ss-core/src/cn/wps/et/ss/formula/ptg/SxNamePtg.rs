// java public class SxNamePtg extends ScalarConstantPtg {
use super::Ptg::{Ptg,PtgKind};
impl Ptg{
    // java public String getFieldName(){return this.fieldName;}
    pub fn getFieldName(&self)->Option<&str>{let PtgKind::SxName{field_name,..}=&self.kind else{panic!("ClassCastException: SxNamePtg")};field_name.as_deref()}
    // java public void setFieldName(String fieldName){this.fieldName=fieldName;}
    pub fn setFieldName(&mut self,fieldName:Option<String>){let PtgKind::SxName{field_name,..}=&mut self.kind else{panic!("ClassCastException: SxNamePtg")};*field_name=fieldName;}
    // java public String getItemName(){return this.itemName;}
    pub fn getItemName(&self)->Option<&str>{let PtgKind::SxName{item_name,..}=&self.kind else{panic!("ClassCastException: SxNamePtg")};item_name.as_deref()}
    // java public void setItemName(String itemName){this.itemName=itemName;}
    pub fn setItemName(&mut self,itemName:Option<String>){let PtgKind::SxName{item_name,..}=&mut self.kind else{panic!("ClassCastException: SxNamePtg")};*item_name=itemName;}
    // java public int getSxIndex(){return this.sxIndex;}
    pub fn getSxIndex(&self)->i32{let PtgKind::SxName{index,..}=self.kind else{panic!("ClassCastException: SxNamePtg")};index}
    // java public void setSxIndex(int sxIndex){this.sxIndex=sxIndex;}
    pub fn setSxIndex(&mut self,sxIndex:i32){let PtgKind::SxName{index,..}=&mut self.kind else{panic!("ClassCastException: SxNamePtg")};*index=sxIndex;}
    // java public String toFormulaString(){
    pub fn toSxNameFormulaString(&self)->String{
        // java if(this.fieldName!=null){String fieldName=this.fieldName.replaceAll("'","''");return this.itemName!=null?this.getWrapString(fieldName)+"["+this.getWrapString(this.itemName)+"]":this.getWrapString(fieldName);}else{return "#NAME?";}
        if let Some(fieldName)=self.getFieldName(){let fieldName=fieldName.replace('\'',"''");if let Some(itemName)=self.getItemName(){format!("{}[{}]",Self::getWrapString(&fieldName),Self::getWrapString(itemName))}else{Self::getWrapString(&fieldName)}}else{"#NAME?".into()}
    }
    // java private String getWrapString(String str){return "'"+str+"'";}
    fn getWrapString(str:&str)->String{format!("'{str}'")}
}
