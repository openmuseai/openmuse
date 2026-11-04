// java package cn.wps.moss.filefmt.biff8.record;
use crate::java_runtime::{Cell,Rc,RefCell};
use std::sync::LazyLock;
use crate::{cn::wps::et::ss::formula::{Formula::{Formula,ByteArray},ptg::Ptg::{Ptg,PtgArray,ptgArray}},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion};
use util::org::apache::poi::util::{BitField::BitField,BitFieldFactory::BitFieldFactory};
use super::{CellRecord::{CellRecord,CellRecordValue,CellRecordText},CellValueRecordInterface::CellValueRecordInterface,StandardRecord::StandardRecord};
use crate::cn::wps::et::ss::formula::eval::ErrorEval::ErrorEval;
use util::org::apache::poi::util::{LittleEndianOutput::LittleEndianOutput,HexDump::HexDump};
// java public static final short sid = 6; public static final short sid406 = 1030; public static final short sid206 = 518;
pub const sid:i16=6;pub const sid406:i16=1030;pub const sid206:i16=518;
// java private static int FIXED_SIZE = 14;
static FIXED_SIZE:Cell<i32>=Cell::new(14);
// java private static final BitField alwaysCalc = BitFieldFactory.getInstance(1);
static alwaysCalc:LazyLock<BitField>=LazyLock::new(||BitFieldFactory::getInstance(1));
// java private static final BitField calcOnLoad = BitFieldFactory.getInstance(2);
static calcOnLoad:LazyLock<BitField>=LazyLock::new(||BitFieldFactory::getInstance(2));
// java private static final BitField sharedFormula = BitFieldFactory.getInstance(8);
static sharedFormula:LazyLock<BitField>=LazyLock::new(||BitFieldFactory::getInstance(8));
// java static byte[] buffer = new byte[22];
static buffer:LazyLock<ByteArray>=LazyLock::new(||Rc::new(RefCell::new(vec![0;22].into_boxed_slice())));
// java public final class FormulaRecord extends CellRecord implements ReusableRecord {
pub struct FormulaRecord{
    // rust: original superclass fields remain in their CellRecord owner.
    base:CellRecord,
    // java private double field_4_value;
    field_4_value:Cell<f64>,
    // java private short field_5_options;
    field_5_options:Cell<i16>,
    // java private int field_6_zero;
    field_6_zero:Cell<i32>,
    // java private Formula field_8_parsed_expr;
    field_8_parsed_expr:RefCell<Option<Rc<RefCell<Formula>>>>,
    // java private SpecialCachedValue specialCachedValue;
    specialCachedValue:RefCell<Option<Rc<SpecialCachedValue>>>,
    // java private boolean isBIFF2ExternalLinkFormula = false;
    isBIFF2ExternalLinkFormula:Cell<bool>,
}
impl FormulaRecord{
    // java public FormulaRecord() { this.field_8_parsed_expr = Formula.create(Ptg.EMPTY_PTG_ARRAY, SpreadsheetVersion.EXCEL97); }
    pub fn new()->Self{Self{base:CellRecord::new(),field_4_value:Cell::new(0.0),field_5_options:Cell::new(0),field_6_zero:Cell::new(0),field_8_parsed_expr:RefCell::new(Some(Rc::new(RefCell::new(Formula::create__array(Some(&Ptg::EMPTY_PTG_ARRAY()),Some(SpreadsheetVersion::EXCEL97)))))),specialCachedValue:RefCell::new(None),isBIFF2ExternalLinkFormula:Cell::new(false)}}
    // java public FormulaRecord(byte[] byteEncoding) { this.field_8_parsed_expr = new Formula(byteEncoding, byteEncoding.length, SpreadsheetVersion.EXCEL97); }
    pub fn new__bytes(byteEncoding:Option<ByteArray>)->Self{let length=byteEncoding.as_ref().expect("NullPointerException").borrow().len()as i32;Self{base:CellRecord::new(),field_4_value:Cell::new(0.0),field_5_options:Cell::new(0),field_6_zero:Cell::new(0),field_8_parsed_expr:RefCell::new(Some(Rc::new(RefCell::new(Formula::Formula(byteEncoding,length,Some(SpreadsheetVersion::EXCEL97)))))),specialCachedValue:RefCell::new(None),isBIFF2ExternalLinkFormula:Cell::new(false)}}
    // rust: BIFF RecordInputStream constructors/reuse and BIFF5 getREFRanges
    // wait for their original reader/reviser dependency graph; no empty overload.
    // java public boolean BIFF2ExternalLinkFormula() { return this.isBIFF2ExternalLinkFormula; }
    pub fn BIFF2ExternalLinkFormula(&self)->bool{self.isBIFF2ExternalLinkFormula.get()}
    // java public void setValue(double value) { this.field_4_value = value; this.specialCachedValue = null; }
    pub fn setValue(&self,value:f64){self.field_4_value.set(value);*self.specialCachedValue.borrow_mut()=None;}
    // java public void setCachedResultTypeEmptyString() { this.specialCachedValue = FormulaRecord.SpecialCachedValue.createCachedEmptyValue(); }
    pub fn setCachedResultTypeEmptyString(&self){*self.specialCachedValue.borrow_mut()=Some(SpecialCachedValue::createCachedEmptyValue());}
    // java public void setCachedResultTypeString() { this.specialCachedValue = FormulaRecord.SpecialCachedValue.createForString(); }
    pub fn setCachedResultTypeString(&self){*self.specialCachedValue.borrow_mut()=Some(SpecialCachedValue::createForString());}
    // java public void setCachedResultErrorCode(int errorCode) { this.specialCachedValue = FormulaRecord.SpecialCachedValue.createCachedErrorCode(errorCode); }
    pub fn setCachedResultErrorCode(&self,errorCode:i32){*self.specialCachedValue.borrow_mut()=Some(SpecialCachedValue::createCachedErrorCode(errorCode));}
    // java public void setCachedResultBoolean(boolean value) { this.specialCachedValue = FormulaRecord.SpecialCachedValue.createCachedBoolean(value); }
    pub fn setCachedResultBoolean(&self,value:bool){*self.specialCachedValue.borrow_mut()=Some(SpecialCachedValue::createCachedBoolean(value));}
    // java public boolean hasCachedResultString() {
    pub fn hasCachedResultString(&self)->bool{
        // java if (this.specialCachedValue == null) { return false; } else { return this.specialCachedValue.getTypeCode() == 0; }
        if self.specialCachedValue.borrow().is_none(){false}else{self.specialCachedValue.borrow().as_ref().unwrap().getTypeCode()==0}
    }
    // java public int getCachedResultType() { return this.specialCachedValue == null ? 0 : this.specialCachedValue.getValueType(); }
    pub fn getCachedResultType(&self)->i32{self.specialCachedValue.borrow().as_ref().map_or(0,|v|v.getValueType())}
    // java public boolean getCachedBooleanValue() { return this.specialCachedValue.getBooleanValue(); }
    pub fn getCachedBooleanValue(&self)->bool{self.specialCachedValue.borrow().as_ref().expect("NullPointerException").getBooleanValue()}
    // java public int getCachedErrorValue() { return this.specialCachedValue.getErrorValue(); }
    pub fn getCachedErrorValue(&self)->i32{self.specialCachedValue.borrow().as_ref().expect("NullPointerException").getErrorValue()}
    // java public void setOptions(short options) { this.field_5_options = options; }
    pub fn setOptions(&self,options:i16){self.field_5_options.set(options);}
    // java public double getValue() { return this.field_4_value; }
    pub fn getValue(&self)->f64{self.field_4_value.get()}
    // java public short getOptions() { return this.field_5_options; }
    pub fn getOptions(&self)->i16{self.field_5_options.get()}
    // java public boolean isSharedFormula() { return sharedFormula.isSet(this.field_5_options); }
    pub fn isSharedFormula(&self)->bool{sharedFormula.isSet(self.field_5_options.get()as i32)}
    // java public void setSharedFormula(boolean flag) { this.field_5_options = sharedFormula.setShortBoolean(this.field_5_options, flag); }
    pub fn setSharedFormula(&self,flag:bool){self.field_5_options.set(sharedFormula.setShortBoolean(self.field_5_options.get(),flag));}
    // java public boolean isAlwaysCalc() { return alwaysCalc.isSet(this.field_5_options); }
    pub fn isAlwaysCalc(&self)->bool{alwaysCalc.isSet(self.field_5_options.get()as i32)}
    // java public void setAlwaysCalc(boolean flag) { this.field_5_options = alwaysCalc.setShortBoolean(this.field_5_options, flag); }
    pub fn setAlwaysCalc(&self,flag:bool){self.field_5_options.set(alwaysCalc.setShortBoolean(self.field_5_options.get(),flag));}
    // java public boolean isCalcOnLoad() { return calcOnLoad.isSet(this.field_5_options); }
    pub fn isCalcOnLoad(&self)->bool{calcOnLoad.isSet(self.field_5_options.get()as i32)}
    // java public void setCalcOnLoad(boolean flag) { this.field_5_options = calcOnLoad.setShortBoolean(this.field_5_options, flag); }
    pub fn setCalcOnLoad(&self,flag:bool){self.field_5_options.set(calcOnLoad.setShortBoolean(self.field_5_options.get(),flag));}
    // java public Ptg[] getParsedExpression() { return this.field_8_parsed_expr.getTokens(); }
    pub fn getParsedExpression(&self)->PtgArray{self.field_8_parsed_expr.borrow().as_ref().expect("NullPointerException").borrow().getTokens__reference()}
    // java public Formula getFormula() { return this.field_8_parsed_expr; }
    pub fn getFormula(&self)->Option<Rc<RefCell<Formula>>>{self.field_8_parsed_expr.borrow().clone()}
    // java public void setFormula(Formula fml) { this.field_8_parsed_expr = fml; }
    pub fn setFormula(&self,fml:Option<Rc<RefCell<Formula>>>){*self.field_8_parsed_expr.borrow_mut()=fml;}
    // java public void setParsedExpression(Ptg[] ptgs) { this.field_8_parsed_expr = Formula.create(ptgs, SpreadsheetVersion.EXCEL97); }
    pub fn setParsedExpression(&self,ptgs:Option<PtgArray>){*self.field_8_parsed_expr.borrow_mut()=Some(Rc::new(RefCell::new(Formula::create__array(ptgs.as_ref(),Some(SpreadsheetVersion::EXCEL97)))));}
    // java public short getSid() { return 6; }
    pub fn getSid(&self)->i16{6}
    // java protected int getValueDataSize() { return FIXED_SIZE + this.field_8_parsed_expr.getEncodedSize(); }
    pub fn getValueDataSize(&self)->i32{FIXED_SIZE.get().wrapping_add(self.field_8_parsed_expr.borrow().as_ref().expect("NullPointerException").borrow().getEncodedSize())}
    // java protected void serializeValue(LittleEndianOutput out) {
    pub fn serializeValue(&self,out:&mut dyn LittleEndianOutput){
        // java if (this.specialCachedValue == null) { out.writeDouble(this.field_4_value); } else { this.specialCachedValue.serialize(out); }
        let special=self.specialCachedValue.borrow().clone();if special.is_none(){out.writeDouble(self.field_4_value.get());}else{special.unwrap().serialize(out);}
        // java out.writeShort(this.getOptions());
        out.writeShort(self.getOptions()as i32);
        // java out.writeInt(this.field_6_zero);
        out.writeInt(self.field_6_zero.get());
        // java this.field_8_parsed_expr.serialize(out);
        self.field_8_parsed_expr.borrow().as_ref().expect("NullPointerException").borrow().serialize(out);
    }
    // java protected String getRecordName() { return "FORMULA"; }
    pub fn getRecordName(&self)->String{"FORMULA".into()}
    // java protected void appendValueText(StringBuilder sb) {
    pub fn appendValueText(&self,sb:&mut String){
        // java sb.append("  .value\t = ");
        sb.push_str("  .value\t = ");
        // java if (this.specialCachedValue == null) {
        let special=self.specialCachedValue.borrow().clone();if special.is_none(){
            // java sb.append(this.field_4_value).append("\n");
            sb.push_str(&kfc::java_runtime::FloatingDecimal::FloatingDecimal::toJavaFormatString(self.field_4_value.get()));sb.push('\n');
        }else{
            // java sb.append(this.specialCachedValue.formatDebugString()).append("\n");
            sb.push_str(&special.unwrap().formatDebugString());sb.push('\n');
        }
        // java sb.append("  .options   = ").append(HexDump.shortToHex(this.getOptions())).append("\n");
        sb.push_str("  .options   = ");sb.extend(HexDump::shortToHex(self.getOptions()as i32));sb.push('\n');
        // java sb.append("    .alwaysCalc= ").append(this.isAlwaysCalc()).append("\n");
        sb.push_str(&format!("    .alwaysCalc= {}\n",self.isAlwaysCalc()));
        // java sb.append("    .calcOnLoad= ").append(this.isCalcOnLoad()).append("\n");
        sb.push_str(&format!("    .calcOnLoad= {}\n",self.isCalcOnLoad()));
        // java sb.append("    .shared    = ").append(this.isSharedFormula()).append("\n");
        sb.push_str(&format!("    .shared    = {}\n",self.isSharedFormula()));
        // java sb.append("  .zero      = ").append(HexDump.intToHex(this.field_6_zero)).append("\n");
        sb.push_str("  .zero      = ");sb.extend(HexDump::intToHex(self.field_6_zero.get()));sb.push('\n');
        // java Ptg[] ptgs = this.field_8_parsed_expr.getTokens();
        let formula=self.field_8_parsed_expr.borrow().clone().expect("NullPointerException");let ptgs=formula.borrow().getTokens__reference();
        // java for(int k = 0; k < ptgs.length; ++k) {
        for k in 0..ptgs.borrow().len(){
            // java if (k > 0) { sb.append("\n"); }
            if k>0{sb.push('\n');}
            // java sb.append("    Ptg[").append(k).append("]=");
            sb.push_str(&format!("    Ptg[{}]=",k));
            // java Ptg ptg = ptgs[k];
            let ptg=ptgs.borrow()[k].clone().expect("NullPointerException");let ptg=ptg.borrow();
            // java sb.append(ptg.toString()).append(ptg.getRVAType());
            sb.push_str(&ptg.toString());sb.push(ptg.getRVAType());
        }
    }
    // rust: inherited final CellRecord.toString with this derived receiver.
    pub fn toString(&self)->String{self.base.toString(self)}
    // java public Object clone() {
    pub fn clone(&self)->Self{
        // java FormulaRecord rec = new FormulaRecord(); this.copyBaseFields(rec);
        let rec=Self::new();self.base.copyBaseFields(Some(&rec.base));
        // java rec.field_4_value = this.field_4_value;
        rec.field_4_value.set(self.field_4_value.get());
        // java rec.field_5_options = this.field_5_options;
        rec.field_5_options.set(self.field_5_options.get());
        // java rec.field_6_zero = this.field_6_zero;
        rec.field_6_zero.set(self.field_6_zero.get());
        // java rec.field_8_parsed_expr = this.field_8_parsed_expr;
        *rec.field_8_parsed_expr.borrow_mut()=self.field_8_parsed_expr.borrow().clone();
        // java rec.specialCachedValue = this.specialCachedValue; return rec;
        *rec.specialCachedValue.borrow_mut()=self.specialCachedValue.borrow().clone();rec
    }
}
// rust: inherited final CellRecord methods and StandardRecord virtual dispatch.
impl std::ops::Deref for FormulaRecord{type Target=CellRecord;fn deref(&self)->&CellRecord{&self.base}}
impl CellRecordValue for FormulaRecord{fn serializeValue(&self,out:&mut dyn LittleEndianOutput){Self::serializeValue(self,out)}fn getValueDataSize(&self)->i32{Self::getValueDataSize(self)}}
impl CellRecordText for FormulaRecord{fn appendValueText(&self,sb:&mut String){Self::appendValueText(self,sb)}fn getRecordName(&self)->Option<String>{Some(Self::getRecordName(self))}}
impl StandardRecord for FormulaRecord{fn getSid(&self)->i16{Self::getSid(self)}fn getDataSize(&self)->i32{self.base.getDataSize(self)}fn serializeData(&self,out:&mut dyn LittleEndianOutput){self.base.serializeData(self,out)}}
impl CellValueRecordInterface for FormulaRecord{fn getRow(&self)->i32{self.base.getRow()}fn getColumn(&self)->i16{self.base.getColumn()}fn setRow(&self,row:i32){self.base.setRow(row)}fn setColumn(&self,col:i16){self.base.setColumn(col)}fn setXFIndex(&self,xf:i32){self.base.setXFIndex(xf)}fn getXFIndex(&self)->i32{self.base.getXFIndex()}}
// java private static final class SpecialCachedValue {
struct SpecialCachedValue{
    // java private final byte[] _variableData;
    _variableData:Option<ByteArray>,
}
#[cfg(test)]
#[path="FormulaRecord.review.rs"]
mod review;
#[cfg(test)]
#[path="FormulaRecordPublic.review.rs"]
mod public_review;
impl SpecialCachedValue{
    // java private static final long BIT_MARKER = -281474976710656L;
    const BIT_MARKER:i64= -281474976710656;
    // java private static final int VARIABLE_DATA_LENGTH = 6;
    const VARIABLE_DATA_LENGTH:i32=6;
    // java private static final int DATA_INDEX = 2;
    const DATA_INDEX:usize=2;
    // java public static final int STRING = 0;
    const STRING:i32=0;
    // java public static final int BOOLEAN = 1;
    const BOOLEAN:i32=1;
    // java public static final int ERROR_CODE = 2;
    const ERROR_CODE:i32=2;
    // java public static final int EMPTY = 3;
    const EMPTY:i32=3;
    // java private SpecialCachedValue(byte[] data) { this._variableData = data; }
    fn new(data:Option<ByteArray>)->Self{Self{_variableData:data}}
    // java public int getTypeCode() { return this._variableData[0]; }
    fn getTypeCode(&self)->i32{*self._variableData.as_ref().expect("NullPointerException").borrow().get(0).expect("ArrayIndexOutOfBoundsException")as i8 as i32}
    // java public static SpecialCachedValue create(long valueLongBits) {
    fn create__bits(valueLongBits:i64)->Option<Rc<Self>>{
        // java if ((-281474976710656L & valueLongBits) != -281474976710656L) { return null; }
        if Self::BIT_MARKER&valueLongBits!=Self::BIT_MARKER{None}else{
            // java byte[] result = new byte[6]; long x = valueLongBits;
            let mut result=vec![0u8;6].into_boxed_slice();let mut x=valueLongBits;
            // java for(int i = 0; i < 6; ++i) { result[i] = (byte)((int)x); x >>= 8; }
            for i in 0..6{result[i]=x as i32 as u8;x>>=8;}
            // java switch (result[0]) { case 0: case 1: case 2: case 3: return new SpecialCachedValue(result);
            match result[0]as i8{0|1|2|3=>Some(Rc::new(Self::new(Some(Rc::new(RefCell::new(result)))))),
                // java default: throw new RecordFormatException("Bad special value code (" + result[0] + ")");
                code=>panic!("RecordFormatException: Bad special value code ({code})")}
        }
    }
    // java public void serialize(LittleEndianOutput out) {
    fn serialize(&self,out:&mut dyn LittleEndianOutput){
        // java out.write(this._variableData);
        let bytes=self._variableData.as_ref().expect("NullPointerException").borrow().clone();out.write(&bytes);
        // java out.writeShort(65535);
        out.writeShort(65535);
    }
    // java public String formatDebugString() { return this.formatValue() + ' ' + HexDump.toHex(this._variableData); }
    fn formatDebugString(&self)->String{
        let value=self.formatValue();let data=self._variableData.as_ref().expect("NullPointerException").borrow();
        format!("{} {}",value,HexDump::toHex_bytes(&data.iter().map(|b|*b as i8).collect::<Vec<_>>()))
    }
    // java private String formatValue() {
    fn formatValue(&self)->String{
        // java int typeCode = this.getTypeCode(); switch (typeCode) {
        let typeCode=self.getTypeCode();match typeCode{
            // java case 0: return "<string>";
            0=>"<string>".into(),
            // java case 1: return this.getDataValue() == 0 ? "FALSE" : "TRUE";
            1=>if self.getDataValue()==0{"FALSE"}else{"TRUE"}.into(),
            // java case 2: return ErrorEval.getText(this.getDataValue());
            2=>ErrorEval::getText(self.getDataValue()),
            // java case 3: return "<empty>";
            3=>"<empty>".into(),
            // java default: return "#error(type=" + typeCode + ")#";
            _=>format!("#error(type={typeCode})#"),
        }
    }
    // java private int getDataValue() { return this._variableData[2]; }
    fn getDataValue(&self)->i32{*self._variableData.as_ref().expect("NullPointerException").borrow().get(Self::DATA_INDEX).expect("ArrayIndexOutOfBoundsException")as i8 as i32}
    // java public static SpecialCachedValue createCachedEmptyValue() { return create(3, 0); }
    fn createCachedEmptyValue()->Rc<Self>{Self::create__code(3,0)}
    // java public static SpecialCachedValue createForString() { return create(0, 0); }
    fn createForString()->Rc<Self>{Self::create__code(0,0)}
    // java public static SpecialCachedValue createCachedBoolean(boolean b) { return create(1, b ? 1 : 0); }
    fn createCachedBoolean(b:bool)->Rc<Self>{Self::create__code(1,if b{1}else{0})}
    // java public static SpecialCachedValue createCachedErrorCode(int errorCode) { return create(2, errorCode); }
    fn createCachedErrorCode(errorCode:i32)->Rc<Self>{Self::create__code(2,errorCode)}
    // java private static SpecialCachedValue create(int code, int data) {
    fn create__code(code:i32,data:i32)->Rc<Self>{
        // java byte[] vd = new byte[]{(byte)code, 0, (byte)data, 0, 0, 0};
        let vd=vec![code as u8,0,data as u8,0,0,0].into_boxed_slice();
        // java return new SpecialCachedValue(vd);
        Rc::new(Self::new(Some(Rc::new(RefCell::new(vd)))))
    }
    // java public String toString() {
    fn toString(&self)->String{
        // java StringBuffer sb = new StringBuffer(64); sb.append(this.getClass().getName());
        let mut sb=String::from("cn.wps.moss.filefmt.biff8.record.FormulaRecord$SpecialCachedValue");
        // java sb.append('[').append(this.formatValue()).append(']'); return sb.toString();
        sb.push('[');sb.push_str(&self.formatValue());sb.push(']');sb
    }
    // java public int getValueType() {
    fn getValueType(&self)->i32{
        // java int typeCode = this.getTypeCode(); switch (typeCode) {
        let typeCode=self.getTypeCode();match typeCode{
            // java case 0: return 1; case 1: return 4; case 2: return 5; case 3: return 1;
            0=>1,1=>4,2=>5,3=>1,
            // java default: throw new IllegalStateException("Unexpected type id (" + typeCode + ")");
            _=>panic!("IllegalStateException: Unexpected type id ({typeCode})"),
        }
    }
    // java public boolean getBooleanValue() {
    fn getBooleanValue(&self)->bool{
        // java if (this.getTypeCode() != 1) { throw new IllegalStateException("Not a boolean cached value - " + this.formatValue()); }
        if self.getTypeCode()!=1{panic!("IllegalStateException: Not a boolean cached value - {}",self.formatValue());}else{
            // java return this.getDataValue() != 0;
            self.getDataValue()!=0
        }
    }
    // java public int getErrorValue() {
    fn getErrorValue(&self)->i32{
        // java if (this.getTypeCode() != 2) { throw new IllegalStateException("Not an error cached value - " + this.formatValue()); }
        if self.getTypeCode()!=2{panic!("IllegalStateException: Not an error cached value - {}",self.formatValue());}else{
            // java return this.getDataValue();
            self.getDataValue()
        }
    }
}
