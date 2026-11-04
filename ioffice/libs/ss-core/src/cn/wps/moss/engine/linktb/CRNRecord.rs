use crate::{java_runtime::{Rc,RefCell,JavaObject::NullableObject},cn::wps::et::ss::formula::constant::ConstantValueParser::ConstantValueParser};
use util::org::apache::poi::util::LittleEndianOutput::LittleEndianOutput;
pub type ConstantObjects=Rc<RefCell<Vec<NullableObject>>>;
// java public final class CRNRecord extends StandardRecord {
// RecordInputStream constructor remains with its original binary dependency.
pub struct CRNRecord{
    // java private int field_1_last_column_index;
    field_1_last_column_index:i32,
    // java private int field_2_first_column_index;
    field_2_first_column_index:i32,
    // java private int field_3_row_index;
    field_3_row_index:i32,
    // java private Object[] field_4_constant_values;
    field_4_constant_values:Option<ConstantObjects>,
}
impl CRNRecord{
    // java public static final short sid=90;
    pub const sid:i16=90;
    // java public CRNRecord(){throw new RuntimeException("incomplete code");}
    pub fn CRNRecord()->Self{panic!("RuntimeException incomplete code")}
    // java public int getNumberOfCRNs(){return this.field_1_last_column_index;}
    pub fn getNumberOfCRNs(&self)->i32{self.field_1_last_column_index}
    // java private CRNRecord(int beginColumn,int endColumn,int row,Object[] objs){this.field_2_first_column_index=beginColumn;this.field_1_last_column_index=endColumn;this.field_3_row_index=row;this.field_4_constant_values=objs;}
    fn CRNRecord__4(beginColumn:i32,endColumn:i32,row:i32,objs:Option<ConstantObjects>)->Self{Self{field_1_last_column_index:endColumn,field_2_first_column_index:beginColumn,field_3_row_index:row,field_4_constant_values:objs}}
    // java public static CRNRecord createCRNRecord(int beginColumn,int endColumn,int row,Object[] objs){return new CRNRecord(beginColumn,endColumn,row,objs);}
    pub fn createCRNRecord(beginColumn:i32,endColumn:i32,row:i32,objs:Option<ConstantObjects>)->Self{Self::CRNRecord__4(beginColumn,endColumn,row,objs)}
    // java public int getFirstColumn(){return this.field_2_first_column_index;}
    pub fn getFirstColumn(&self)->i32{self.field_2_first_column_index}
    // java public int getLastColumn(){return this.field_1_last_column_index;}
    pub fn getLastColumn(&self)->i32{self.field_1_last_column_index}
    // java public int getRowIndex(){return this.field_3_row_index;}
    pub fn getRowIndex(&self)->i32{self.field_3_row_index}
    // java public Object getConstantValue(int col){return this.field_4_constant_values[col-this.field_2_first_column_index];}
    pub fn getConstantValue(&self,col:i32)->NullableObject{self.field_4_constant_values.as_ref().expect("NullPointerException").borrow()[col.wrapping_sub(self.field_2_first_column_index)as usize].clone()}
    // java public Object[] getConstantValues(){return this.field_4_constant_values;}
    pub fn getConstantValues(&self)->Option<ConstantObjects>{self.field_4_constant_values.clone()}
    // java public String toString(){StringBuffer sb=new StringBuffer();sb.append(this.getClass().getName()).append(" [CRN");sb.append(" rowIx=").append(this.field_3_row_index);sb.append(" firstColIx=").append(this.field_2_first_column_index);sb.append(" lastColIx=").append(this.field_1_last_column_index);sb.append("]");return sb.toString();}
    pub fn toString(&self)->String{let mut sb=String::new();sb.push_str("cn.wps.moss.engine.linktb.CRNRecord [CRN");sb.push_str(&format!(" rowIx={}",self.field_3_row_index));sb.push_str(&format!(" firstColIx={}",self.field_2_first_column_index));sb.push_str(&format!(" lastColIx={}",self.field_1_last_column_index));sb.push(']');sb}
    // java protected int getDataSize(){return 4+ConstantValueParser.getEncodedSize(this.field_4_constant_values);}
    pub(crate) fn getDataSize(&self)->i32{4i32.wrapping_add(ConstantValueParser::getEncodedSize__objects(&self.field_4_constant_values.as_ref().expect("NullPointerException").borrow()))}
    // java public void serializeData(LittleEndianOutput out){out.writeByte(this.field_1_last_column_index);out.writeByte(this.field_2_first_column_index);out.writeShort(this.field_3_row_index);ConstantValueParser.encode(out,this.field_4_constant_values);}
    pub fn serializeData(&self,out:&mut dyn LittleEndianOutput){out.writeByte(self.field_1_last_column_index);out.writeByte(self.field_2_first_column_index);out.writeShort(self.field_3_row_index);ConstantValueParser::encode__objects(out,&self.field_4_constant_values.as_ref().expect("NullPointerException").borrow());}
    // java public short getSid(){return 90;}
    pub fn getSid(&self)->i16{90}
    // java public int hashCode(){int prime=31;int result=1;result=31*result+this.field_1_last_column_index;result=31*result+this.field_2_first_column_index;result=31*result+this.field_3_row_index;result=31*result+Arrays.hashCode(this.field_4_constant_values);return result;}
    pub fn hashCode(&self)->i32{let _prime=31;let mut result=1i32;result=result.wrapping_mul(31).wrapping_add(self.field_1_last_column_index);result=result.wrapping_mul(31).wrapping_add(self.field_2_first_column_index);result=result.wrapping_mul(31).wrapping_add(self.field_3_row_index);let arrayHash=if let Some(a)=&self.field_4_constant_values{a.borrow().iter().fold(1i32,|h,v|h.wrapping_mul(31).wrapping_add(v.as_ref().map_or(0,|v|v.hashCode())))}else{0};result=result.wrapping_mul(31).wrapping_add(arrayHash);result}
    // java public boolean equals(Object obj) {
    pub fn equals(&self,obj:Option<&dyn std::any::Any>)->bool{
        // java if(obj==null){return false;}else{if(obj instanceof CRNRecord){CRNRecord record=(CRNRecord)obj;boolean posEquals=this.field_2_first_column_index==record.getFirstColumn()&&this.field_1_last_column_index==record.getLastColumn()&&this.field_3_row_index==record.getRowIndex();if(posEquals){
        let Some(obj)=obj else{return false;};if let Some(record)=obj.downcast_ref::<CRNRecord>(){let posEquals=self.field_2_first_column_index==record.getFirstColumn()&&self.field_1_last_column_index==record.getLastColumn()&&self.field_3_row_index==record.getRowIndex();if posEquals{
            // java Object[] recordConstants=record.getConstantValues();int len=this.field_4_constant_values.length;for(int i=0;i<len;++i){if(!this.field_4_constant_values[i].equals(recordConstants[i])){return false;}}return true;
            let recordConstants=record.getConstantValues().expect("NullPointerException");let len=self.field_4_constant_values.as_ref().expect("NullPointerException").borrow().len();for i in 0..len{let lhs=self.field_4_constant_values.as_ref().unwrap().borrow()[i].clone().expect("NullPointerException");let rhs=recordConstants.borrow()[i].clone();if !rhs.as_ref().is_some_and(|rhs|lhs.equals(rhs.as_ref())){return false;}}return true;
        }}
        // java return false;
        false
    }
}
impl crate::cn::wps::moss::filefmt::biff8::record::StandardRecord::StandardRecord for CRNRecord{fn getDataSize(&self)->i32{Self::getDataSize(self)}fn getSid(&self)->i16{Self::getSid(self)}fn serializeData(&self,out:&mut dyn LittleEndianOutput){Self::serializeData(self,out)}}
