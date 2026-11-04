use crate::{java_runtime::{Rc,RefCell},cn::wps::et::ss::formula::{Formula::Formula,ptg::Ptg::Ptg},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion};
use std::option::Option as Maybe;
pub type NameRecordRef=Rc<RefCell<NameRecord>>;
pub use crate::cn::wps::et::ss::formula::ptg::Ptg::PtgArray;
// java public final class NameRecord extends ContinuableRecord {
pub struct NameRecord{
    // java private short field_1_option_flag;private byte field_2_keyboard_shortcut;private short field_5_externSheetIndex_plus1;private int field_6_sheetNumber;
    field_1_option_flag:i16,field_2_keyboard_shortcut:i8,field_5_externSheetIndex_plus1:i16,field_6_sheetNumber:i32,
    // java private boolean field_11_nameIsMultibyte;private byte field_12_built_in_code;private String field_12_name_text;
    field_11_nameIsMultibyte:bool,field_12_built_in_code:i8,field_12_name_text:String,
    // java private Formula field_13_name_definition;private Ptg[] cached_fmls;
    field_13_name_definition:Maybe<Rc<RefCell<Formula>>>,cached_fmls:Maybe<PtgArray>,
    // java private String field_14_custom_menu_text;private String field_15_description_text;private String field_16_help_topic_text;private String field_17_status_bar_text;
    field_14_custom_menu_text:Maybe<String>,field_15_description_text:Maybe<String>,field_16_help_topic_text:Maybe<String>,field_17_status_bar_text:Maybe<String>,
    // java private final SpreadsheetVersion sv;
    sv:SpreadsheetVersion,
}
impl NameRecord{
    pub const sid:i16=24;pub const sid218:i16=536;
    pub const BUILTIN_CONSOLIDATE_AREA:i8=0;pub const BUILTIN_AUTO_OPEN:i8=1;pub const BUILTIN_AUTO_CLOSE:i8=2;pub const BUILTIN_EXTRACT:i8=3;pub const BUILTIN_DATABASE:i8=4;pub const BUILTIN_CRITERIA:i8=5;pub const BUILTIN_PRINT_AREA:i8=6;pub const BUILTIN_PRINT_TITLE:i8=7;pub const BUILTIN_RECORDER:i8=8;pub const BUILTIN_DATA_FORM:i8=9;pub const BUILTIN_AUTO_ACTIVATE:i8=10;pub const BUILTIN_AUTO_DEACTIVATE:i8=11;pub const BUILTIN_SHEET_TITLE:i8=12;pub const BUILTIN_FILTER_DB:i8=13;
    // java public NameRecord(SpreadsheetVersion sv){this.cached_fmls=null;this.sv=sv;this.field_13_name_definition=Formula.create(Ptg.EMPTY_PTG_ARRAY,sv);this.field_12_name_text="";this.field_14_custom_menu_text="";this.field_15_description_text="";this.field_16_help_topic_text="";this.field_17_status_bar_text="";}
    pub fn NameRecord(sv:SpreadsheetVersion)->Self{Self{field_1_option_flag:0,field_2_keyboard_shortcut:0,field_5_externSheetIndex_plus1:0,field_6_sheetNumber:0,field_11_nameIsMultibyte:false,field_12_built_in_code:0,field_12_name_text:String::new(),field_13_name_definition:Some(Rc::new(RefCell::new(Formula::create(&[],sv)))),cached_fmls:None,field_14_custom_menu_text:Some(String::new()),field_15_description_text:Some(String::new()),field_16_help_topic_text:Some(String::new()),field_17_status_bar_text:Some(String::new()),sv}}
    // java public NameRecord(byte builtin,int sheetNumber,SpreadsheetVersion sv){this(sv);this.field_12_built_in_code=builtin;this.setOptionFlag((short)(this.field_1_option_flag|32));this.field_6_sheetNumber=sheetNumber;}
    pub fn NameRecord__builtin(builtin:i8,sheetNumber:i32,sv:SpreadsheetVersion)->Self{let mut this=Self::NameRecord(sv);this.field_12_built_in_code=builtin;this.setOptionFlag(this.field_1_option_flag|32);this.field_6_sheetNumber=sheetNumber;this}
    // java public void setOptionFlag(short flag){this.field_1_option_flag=flag;}
    pub fn setOptionFlag(&mut self,flag:i16){self.field_1_option_flag=flag;}
    // java public void setKeyboardShortcut(byte shortcut){this.field_2_keyboard_shortcut=shortcut;}
    pub fn setKeyboardShortcut(&mut self,shortcut:i8){self.field_2_keyboard_shortcut=shortcut;}
    // java public int getSheetNumber(){return this.field_6_sheetNumber;}
    pub fn getSheetNumber(&self)->i32{self.field_6_sheetNumber}
    // java public byte getFnGroup(){int masked=this.field_1_option_flag&4032;return (byte)(masked>>4);}
    pub fn getFnGroup(&self)->i8{let masked=self.field_1_option_flag&4032;(masked>>4)as i8}
    // java public void setSheetNumber(int value){this.field_6_sheetNumber=value;}
    pub fn setSheetNumber(&mut self,value:i32){self.field_6_sheetNumber=value;}
    // Native non-null String view of the original setter.
    pub fn setNameText(&mut self,name:&str){self.setNameText__nullable(Some(name));}
    // java public void setNameText(String name){name=name.trim();this.field_12_name_text=name;this.field_11_nameIsMultibyte=StringUtil.hasMultibyte(name);}
    pub fn setNameText__nullable(&mut self,name:Maybe<&str>){let name=name.expect("NullPointerException").trim_matches(|c:char|c<='\u{20}');self.field_12_name_text=name.into();self.field_11_nameIsMultibyte=name.encode_utf16().any(|c|c>255);}
    // java public void setCustomMenuText(String text){this.field_14_custom_menu_text=text;}
    pub fn setCustomMenuText(&mut self,text:Maybe<String>){self.field_14_custom_menu_text=text;}
    // java public void setDescriptionText(String text){this.field_15_description_text=text;}
    pub fn setDescriptionText(&mut self,text:Maybe<String>){self.field_15_description_text=text;}
    // java public void setHelpTopicText(String text){this.field_16_help_topic_text=text;}
    pub fn setHelpTopicText(&mut self,text:Maybe<String>){self.field_16_help_topic_text=text;}
    // java public void setStatusBarText(String text){this.field_17_status_bar_text=text;}
    pub fn setStatusBarText(&mut self,text:Maybe<String>){self.field_17_status_bar_text=text;}
    // java public short getOptionFlag(){return this.field_1_option_flag;}
    pub fn getOptionFlag(&self)->i16{self.field_1_option_flag}
    // java public byte getKeyboardShortcut(){return this.field_2_keyboard_shortcut;}
    pub fn getKeyboardShortcut(&self)->i8{self.field_2_keyboard_shortcut}
    // java private int getNameTextLength(){return this.isBuiltInName()?1:this.field_12_name_text.length();}
    fn getNameTextLength(&self)->i32{if self.isBuiltInName(){1}else{self.field_12_name_text.encode_utf16().count()as i32}}
    // java public boolean isHiddenName(){return (this.field_1_option_flag&1)!=0;}
    pub fn isHiddenName(&self)->bool{self.field_1_option_flag&1!=0}
    // java public void setHidden(boolean b){if(b){this.field_1_option_flag=(short)(this.field_1_option_flag|1);}else{this.field_1_option_flag=(short)(this.field_1_option_flag&-2);}}
    pub fn setHidden(&mut self,b:bool){if b{self.field_1_option_flag|=1;}else{self.field_1_option_flag&= -2;}}
    // java public boolean isFunctionName(){return (this.field_1_option_flag&2)!=0;}
    pub fn isFunctionName(&self)->bool{self.field_1_option_flag&2!=0}
    // java public void setFunction(boolean function){if(function){this.field_1_option_flag=(short)(this.field_1_option_flag|2);}else{this.field_1_option_flag=(short)(this.field_1_option_flag&-3);}}
    pub fn setFunction(&mut self,function:bool){if function{self.field_1_option_flag|=2;}else{self.field_1_option_flag&= -3;}}
    // java public boolean hasFormula(){return NameRecord.Option.isFormula(this.field_1_option_flag)&&this.field_13_name_definition.getEncodedTokenSize()>0;}
    pub fn hasFormula(&self)->bool{Option::isFormula(self.field_1_option_flag as i32)&&self.field_13_name_definition.as_ref().expect("NullPointerException").borrow().getEncodedTokenSize()>0}
    // java public boolean isCommandName(){return (this.field_1_option_flag&4)!=0;}
    pub fn isCommandName(&self)->bool{self.field_1_option_flag&4!=0}
    // java public boolean isMacro(){return (this.field_1_option_flag&8)!=0;}
    pub fn isMacro(&self)->bool{self.field_1_option_flag&8!=0}
    // java public void setMacro(boolean macro){if(macro){this.field_1_option_flag=(short)(this.field_1_option_flag|8);}else{this.field_1_option_flag=(short)(this.field_1_option_flag&-9);}}
    pub fn setMacro(&mut self,r#macro:bool){if r#macro{self.field_1_option_flag|=8;}else{self.field_1_option_flag&= -9;}}
    // java public boolean isComplexFunction(){return (this.field_1_option_flag&16)!=0;}
    pub fn isComplexFunction(&self)->bool{self.field_1_option_flag&16!=0}
    // java public boolean isBuiltInName(){return (this.field_1_option_flag&32)!=0;}
    pub fn isBuiltInName(&self)->bool{self.field_1_option_flag&32!=0}
    // java public String getNameText(){return this.isBuiltInName()&&StringUtils.isEmpty(this.field_12_name_text)?translateBuiltInName(this.getBuiltInName()):this.field_12_name_text;}
    pub fn getNameText(&self)->&str{if self.isBuiltInName()&&self.field_12_name_text.is_empty(){Self::translateBuiltInName(self.getBuiltInName())}else{&self.field_12_name_text}}
    // java public byte getBuiltInName(){return this.field_12_built_in_code;}
    pub fn getBuiltInName(&self)->i8{self.field_12_built_in_code}
    // java public Ptg[] getNameDefinition(){
    pub fn getNameDefinition(&mut self)->PtgArray{
        // java if(this.cached_fmls==null){try{this.field_13_name_definition.setVersion(this.sv);this.cached_fmls=this.field_13_name_definition.getTokens(false);}
        if self.cached_fmls.is_none(){let r=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let f=self.field_13_name_definition.as_ref().expect("NullPointerException");f.borrow_mut().setVersion(Some(self.sv));let tokens=f.borrow().getTokens__external(false);self.cached_fmls=Some(crate::cn::wps::et::ss::formula::ptg::Ptg::ptgArray(tokens));}));if let Err(e)=r{
            // java catch(Exception e){e.printStackTrace();this.field_13_name_definition=Formula.EMPTY_FORMULA;return this.field_13_name_definition.getTokens();}
            let text=e.downcast_ref::<String>().map(String::as_str).or_else(||e.downcast_ref::<&str>().copied()).unwrap_or("");if !text.starts_with("NullPointerException")&&text!="Ptg array size mismatch"{std::panic::resume_unwind(e);}eprintln!("{text}");let f=Formula::EMPTY_FORMULA();self.field_13_name_definition=Some(f.clone());let tokens=f.borrow().getTokens();return crate::cn::wps::et::ss::formula::ptg::Ptg::ptgArray(tokens);
        }}
        // java return this.cached_fmls;
        self.cached_fmls.clone().unwrap()
    }
    // Native owned input transport; reference callers retain their original array.
    pub fn setNameDefinition(&mut self,ptgs:impl IntoPtgArray){self.setNameDefinition__array(ptgs.intoArray());}
    // java public void setNameDefinition(Ptg[] ptgs){this.cached_fmls=ptgs;this.field_13_name_definition=Formula.create(ptgs,this.sv);}
    pub fn setNameDefinition__array(&mut self,ptgs:Maybe<PtgArray>){self.cached_fmls=ptgs.clone();let formula=Formula::create__array(ptgs.as_ref(),Some(self.sv));self.field_13_name_definition=Some(Rc::new(RefCell::new(formula)));}
    // Original getRange/setRange and BIFF continuation serialization are separate dependencies.
    // java public String getCustomMenuText(){return this.field_14_custom_menu_text;}
    pub fn getCustomMenuText(&self)->Maybe<&str>{self.field_14_custom_menu_text.as_deref()}
    // java public String getDescriptionText(){return this.field_15_description_text;}
    pub fn getDescriptionText(&self)->Maybe<&str>{self.field_15_description_text.as_deref()}
    // java public String getHelpTopicText(){return this.field_16_help_topic_text;}
    pub fn getHelpTopicText(&self)->Maybe<&str>{self.field_16_help_topic_text.as_deref()}
    // java public String getStatusBarText(){return this.field_17_status_bar_text;}
    pub fn getStatusBarText(&self)->Maybe<&str>{self.field_17_status_bar_text.as_deref()}
    // java private int getNameRawSize(){if(this.isBuiltInName()){return 1;}else{int nChars=this.field_12_name_text.length();return this.field_11_nameIsMultibyte?2*nChars:nChars;}}
    fn getNameRawSize(&self)->i32{if self.isBuiltInName(){1}else{let nChars=self.field_12_name_text.encode_utf16().count()as i32;if self.field_11_nameIsMultibyte{2i32.wrapping_mul(nChars)}else{nChars}}}
    // java protected int getDataSize(){return 13+this.getNameRawSize()+this.field_14_custom_menu_text.length()+this.field_15_description_text.length()+this.field_16_help_topic_text.length()+this.field_17_status_bar_text.length()+this.field_13_name_definition.getEncodedSize();}
    pub(crate) fn getDataSize(&self)->i32{let len=|s:&Maybe<String>|s.as_ref().expect("NullPointerException").encode_utf16().count()as i32;13i32.wrapping_add(self.getNameRawSize()).wrapping_add(len(&self.field_14_custom_menu_text)).wrapping_add(len(&self.field_15_description_text)).wrapping_add(len(&self.field_16_help_topic_text)).wrapping_add(len(&self.field_17_status_bar_text)).wrapping_add(self.field_13_name_definition.as_ref().expect("NullPointerException").borrow().getEncodedSize())}
    // java public Formula getFormula(){return this.field_13_name_definition;}
    pub fn getFormula(&self)->Maybe<Rc<RefCell<Formula>>>{self.field_13_name_definition.clone()}
    // java public void setFormula(Formula formula){this.field_13_name_definition=formula;}
    pub fn setFormula(&mut self,formula:Maybe<Rc<RefCell<Formula>>>){self.field_13_name_definition=formula;}
    // java public short getSid(){return 24;}
    pub fn getSid(&self)->i16{24}
    // java private static String translateBuiltInName(byte name){switch(name){case 0:return "Consolidate_Area";case 1:return "Auto_Open";case 2:return "Auto_Close";case 3:return "Extract";case 4:return "Database";case 5:return "Criteria";case 6:return "Print_Area";case 7:return "Print_Titles";case 8:return "Recorder";case 9:return "Data_Form";case 10:return "Auto_Activate";case 11:return "Auto_Deactivate";case 12:return "Sheet_Title";case 13:return "_FilterDatabase";default:return "Unknown";}}
    fn translateBuiltInName(name:i8)->&'static str{match name{0=>"Consolidate_Area",1=>"Auto_Open",2=>"Auto_Close",3=>"Extract",4=>"Database",5=>"Criteria",6=>"Print_Area",7=>"Print_Titles",8=>"Recorder",9=>"Data_Form",10=>"Auto_Activate",11=>"Auto_Deactivate",12=>"Sheet_Title",13=>"_FilterDatabase",_=>"Unknown"}}
    // java public NameRecord copy(){NameRecord ret=new NameRecord(this.sv);ret.field_1_option_flag=this.field_1_option_flag;ret.field_2_keyboard_shortcut=this.field_2_keyboard_shortcut;ret.field_5_externSheetIndex_plus1=this.field_5_externSheetIndex_plus1;ret.field_6_sheetNumber=this.field_6_sheetNumber;ret.field_11_nameIsMultibyte=this.field_11_nameIsMultibyte;ret.field_12_built_in_code=this.field_12_built_in_code;ret.field_12_name_text=this.field_12_name_text;ret.field_13_name_definition=this.field_13_name_definition.copy();ret.cached_fmls=null;ret.field_14_custom_menu_text=this.field_14_custom_menu_text;ret.field_15_description_text=this.field_15_description_text;ret.field_16_help_topic_text=this.field_16_help_topic_text;ret.field_17_status_bar_text=this.field_17_status_bar_text;return ret;}
    pub fn copy(&self)->Self{let mut ret=Self::NameRecord(self.sv);ret.field_1_option_flag=self.field_1_option_flag;ret.field_2_keyboard_shortcut=self.field_2_keyboard_shortcut;ret.field_5_externSheetIndex_plus1=self.field_5_externSheetIndex_plus1;ret.field_6_sheetNumber=self.field_6_sheetNumber;ret.field_11_nameIsMultibyte=self.field_11_nameIsMultibyte;ret.field_12_built_in_code=self.field_12_built_in_code;ret.field_12_name_text=self.field_12_name_text.clone();ret.field_13_name_definition=Some(Rc::new(RefCell::new(self.field_13_name_definition.as_ref().expect("NullPointerException").borrow().copy())));ret.cached_fmls=None;ret.field_14_custom_menu_text=self.field_14_custom_menu_text.clone();ret.field_15_description_text=self.field_15_description_text.clone();ret.field_16_help_topic_text=self.field_16_help_topic_text.clone();ret.field_17_status_bar_text=self.field_17_status_bar_text.clone();ret}
    // Native view of original Formula.EMPTY_FORMULA.equals(record.getFormula()).
    pub fn formula_is_empty(&self)->bool{self.field_13_name_definition.as_ref().is_some_and(|f|Formula::EMPTY_FORMULA().borrow().equals(Some(&*f.borrow())))}
}
// java private static final class Option { public static final boolean isFormula(int optValue){return (optValue&14)==0;} }
struct Option;
impl Option{
    // java public static final int OPT_HIDDEN_NAME=1;
    const OPT_HIDDEN_NAME:i32=1;
    // java public static final int OPT_FUNCTION_NAME=2;
    const OPT_FUNCTION_NAME:i32=2;
    // java public static final int OPT_COMMAND_NAME=4;
    const OPT_COMMAND_NAME:i32=4;
    // java public static final int OPT_MACRO=8;
    const OPT_MACRO:i32=8;
    // java public static final int OPT_COMPLEX=16;
    const OPT_COMPLEX:i32=16;
    // java public static final int OPT_BUILTIN=32;
    const OPT_BUILTIN:i32=32;
    // java public static final int OPT_BINDATA=4096;
    const OPT_BINDATA:i32=4096;
    // java public static final boolean isFormula(int optValue){return(optValue&14)==0;}
    fn isFormula(optValue:i32)->bool{optValue&14==0}
}
// Native argument/reference transports; no name decisions are made here.
pub trait IntoPtgArray{fn intoArray(self)->Maybe<PtgArray>;}
impl IntoPtgArray for Vec<Ptg>{fn intoArray(self)->Maybe<PtgArray>{Some(crate::cn::wps::et::ss::formula::ptg::Ptg::ptgArray(self))}}
impl IntoPtgArray for PtgArray{fn intoArray(self)->Maybe<PtgArray>{Some(self)}}
impl IntoPtgArray for Maybe<PtgArray>{fn intoArray(self)->Maybe<PtgArray>{self}}
pub trait IntoNameRecordRef{fn intoRecord(self)->Maybe<NameRecordRef>;}
impl IntoNameRecordRef for NameRecord{fn intoRecord(self)->Maybe<NameRecordRef>{Some(Rc::new(RefCell::new(self)))}}
impl IntoNameRecordRef for NameRecordRef{fn intoRecord(self)->Maybe<NameRecordRef>{Some(self)}}
impl IntoNameRecordRef for Maybe<NameRecordRef>{fn intoRecord(self)->Maybe<NameRecordRef>{self}}
impl Clone for NameRecord{fn clone(&self)->Self{self.copy()}}
#[cfg(test)]
#[path="NameRecord.review.rs"]
pub(crate) mod review;
