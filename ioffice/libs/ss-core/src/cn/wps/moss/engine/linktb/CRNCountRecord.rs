use util::org::apache::poi::util::LittleEndianOutput::LittleEndianOutput;
// java public final class CRNCountRecord extends StandardRecord {
// Original binary RecordInputStream overloads await their reader dependency.
pub struct CRNCountRecord{
    // java private int field_1_number_crn_records;
    field_1_number_crn_records:i32,
    // java private int field_2_sheet_table_index;
    field_2_sheet_table_index:i32,
}
impl CRNCountRecord{
    // java public static final short sid=89;private static final short DATA_SIZE=4;
    pub const sid:i16=89;const DATA_SIZE:i16=4;
    // java public CRNCountRecord(){throw new RuntimeException("incomplete code");}
    pub fn CRNCountRecord()->Self{panic!("RuntimeException incomplete code")}
    // java public int getNumberOfCRNs(){return this.field_1_number_crn_records;}
    pub fn getNumberOfCRNs(&self)->i32{self.field_1_number_crn_records}
    // java public CRNCountRecord(int sheetId,int numberCrn){this.field_1_number_crn_records=numberCrn;this.field_2_sheet_table_index=sheetId;}
    pub fn CRNCountRecord__2(sheetId:i32,numberCrn:i32)->Self{Self{field_1_number_crn_records:numberCrn,field_2_sheet_table_index:sheetId}}
    // java public int getSheetTableIndex(){return this.field_2_sheet_table_index;}
    pub fn getSheetTableIndex(&self)->i32{self.field_2_sheet_table_index}
    // java public String toString(){StringBuffer sb=new StringBuffer();sb.append(this.getClass().getName()).append(" [XCT");sb.append(" nCRNs=").append(this.field_1_number_crn_records);sb.append(" sheetIx=").append(this.field_2_sheet_table_index);sb.append("]");return sb.toString();}
    pub fn toString(&self)->String{let mut sb=String::new();sb.push_str("cn.wps.moss.engine.linktb.CRNCountRecord [XCT");sb.push_str(&format!(" nCRNs={}",self.field_1_number_crn_records));sb.push_str(&format!(" sheetIx={}",self.field_2_sheet_table_index));sb.push(']');sb}
    // java public void serializeData(LittleEndianOutput out){out.writeShort((short)this.field_1_number_crn_records);out.writeShort((short)this.field_2_sheet_table_index);}
    pub fn serializeData(&self,out:&mut dyn LittleEndianOutput){out.writeShort(self.field_1_number_crn_records as i16 as i32);out.writeShort(self.field_2_sheet_table_index as i16 as i32);}
    // java protected int getDataSize(){return 4;}
    pub(crate) fn getDataSize(&self)->i32{4}
    // java public short getSid(){return 89;}
    pub fn getSid(&self)->i16{89}
}
impl crate::cn::wps::moss::filefmt::biff8::record::StandardRecord::StandardRecord for CRNCountRecord{fn getDataSize(&self)->i32{Self::getDataSize(self)}fn getSid(&self)->i16{Self::getSid(self)}fn serializeData(&self,out:&mut dyn LittleEndianOutput){Self::serializeData(self,out)}}
