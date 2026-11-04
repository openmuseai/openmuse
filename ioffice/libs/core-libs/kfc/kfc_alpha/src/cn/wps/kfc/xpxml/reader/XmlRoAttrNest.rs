use super::{XmlRoAttr::XmlRoAttr,XmlRoAttrValue::XmlRoAttrValue};
// java public final class XmlRoAttrNest implements XmlRoAttr
pub struct XmlRoAttrNest {mName:i32,mText:Option<String>,mAttrList:Vec<Box<dyn XmlRoAttr>>}
impl XmlRoAttrNest {
    // java this.mName=name; this.mAttrList=new ArrayList(); this.mText=null;
    pub fn new(name:i32)->Self {Self{mName:name,mText:None,mAttrList:Vec::new()}}
    // java this.mAttrList.add(attr);
    pub fn addAttr(&mut self,attr:Box<dyn XmlRoAttr>){self.mAttrList.push(attr);}
    // java this.mText=chars;
    pub fn characters(&mut self,chars:&str){self.mText=Some(chars.into());}
}
impl XmlRoAttrValue for XmlRoAttrNest {
    // java builder.append("\n[class XmlRoAttrNest]\n\n"); builder.append(this.mAttrList.size()); builder.append("\n"); return builder.toString();
    fn to_string(&self)->String {format!("\n[class XmlRoAttrNest]\n\n{}\n",self.mAttrList.len())}
    // java throw new RuntimeException("Not implemented");
    fn to_int(&self)->i32 {panic!("Not implemented")}
    fn to_byte(&self)->i8 {panic!("Not implemented")}
    fn to_short(&self)->i16 {panic!("Not implemented")}
    fn to_boolean(&self)->bool {panic!("Not implemented")}
    fn to_float(&self)->f32 {panic!("Not implemented")}
    fn to_double(&self)->f64 {panic!("Not implemented")}
    fn to_long(&self)->i64 {panic!("Not implemented")}
    fn to_percentage(&self)->f64 {panic!("Not implemented")}
}
impl XmlRoAttr for XmlRoAttrNest {
    // java for(...) if(roAttr.getName()==name) return roAttr; return null;
    fn get_by_name(&self,name:i32)->Option<&dyn XmlRoAttr>{self.mAttrList.iter().find(|a|a.get_name()==name).map(|a|a.as_ref())}
    // java return this.mAttrList.get(index);
    fn get_attr_at(&self,index:i32)->Option<&dyn XmlRoAttr>{Some(self.mAttrList[index as usize].as_ref())}
    fn get_name_at(&self,index:i32)->i32 {self.mAttrList[index as usize].get_name()}
    fn count(&self)->i32 {self.mAttrList.len() as i32}
    fn get_characters(&self)->String {self.mText.clone().unwrap_or_default()}
    fn get_name(&self)->i32 {self.mName}
    // java if(roAttr.getName()==name && roAttr.isLeaf()==bLeaf) return roAttr;
    fn get_by_name_with_flag(&self,name:i32,bLeaf:bool)->Option<&dyn XmlRoAttr>{self.mAttrList.iter().find(|a|a.get_name()==name&&a.is_leaf()==bLeaf).map(|a|a.as_ref())}
    fn is_leaf(&self)->bool {false}
}
