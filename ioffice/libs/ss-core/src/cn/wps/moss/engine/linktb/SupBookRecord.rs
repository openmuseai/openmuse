use crate::java_runtime::{Rc,RefCell};
use util::org::apache::poi::util::{LittleEndianOutput::LittleEndianOutput,StringUtil::StringUtil};
// Java array reference adapter: constructor retains the array; getSheetNames clones it.
pub type SheetNames=Rc<RefCell<Vec<Option<String>>>>;
// java public final class SupBookRecord extends StandardRecord {
// The original RecordInputStream/ContinueRecord overloads remain pending their
// binary reader dependencies; no substitute input stream or empty serializer.
pub struct SupBookRecord{
    // java private short field_1_number_of_sheets;
    field_1_number_of_sheets:crate::java_runtime::Cell<i16>,
    // java private String field_2_encoded_url;
    field_2_encoded_url:Option<String>,
    // java private String[] field_3_sheet_names;
    field_3_sheet_names:Option<SheetNames>,
    // java private boolean _isAddInFunctions;
    _isAddInFunctions:bool,
    // java private boolean isSameSheetRefer;
    isSameSheetRefer:bool,
}
impl SupBookRecord{
    // java public static final short sid=430;private static final short SMALL_RECORD_SIZE=4;private static final short TAG_INTERNAL_REFERENCES=1025;private static final short TAG_ADD_IN_FUNCTIONS=14849;
    pub const sid:i16=430;const SMALL_RECORD_SIZE:i16=4;const TAG_INTERNAL_REFERENCES:i16=1025;const TAG_ADD_IN_FUNCTIONS:i16=14849;
    // java public static SupBookRecord createAddInFunctions(){return new SupBookRecord(true,(short)1,(String)null,(String[])null);}
    pub fn createAddInFunctions()->Self{Self::SupBookRecord(true,1,None,None)}
    // java public static SupBookRecord createNullSupBook(short numberOfSheets){return new SupBookRecord(false,numberOfSheets,(String)null,(String[])null);}
    pub fn createNullSupBook(numberOfSheets:i16)->Self{Self::SupBookRecord(false,numberOfSheets,None,None)}
    // java public static SupBookRecord createEmptySupBook(){return new SupBookRecord(false,(short)0,"",new String[0]);}
    pub fn createEmptySupBook()->Self{Self::SupBookRecord(false,0,Some(String::new()),Some(Rc::new(RefCell::new(vec![]))))}
    // java public static SupBookRecord createSameSheetRef(){SupBookRecord supBook=new SupBookRecord(false,(short)0,"",(String[])null);supBook.isSameSheetRefer=true;return supBook;}
    pub fn createSameSheetRef()->Self{let mut supBook=Self::SupBookRecord(false,0,Some(String::new()),None);supBook.isSameSheetRefer=true;supBook}
    // java public static SupBookRecord createExternalReferences(String url,String[] sheetNames){return new SupBookRecord(false,(short)sheetNames.length,encodeFileName(url),sheetNames);}
    pub fn createExternalReferences(url:Option<&str>,sheetNames:Option<SheetNames>)->Self{let count=sheetNames.as_ref().expect("NullPointerException").borrow().len()as i16;Self::SupBookRecord(false,count,Some(Self::encodeFileName(url)),sheetNames)}
    // java private SupBookRecord(boolean isAddInFuncs,short numberOfSheets,String url,String[] sheetNames){this.field_1_number_of_sheets=numberOfSheets;this.field_2_encoded_url=url;this.field_3_sheet_names=sheetNames;this._isAddInFunctions=isAddInFuncs;}
    fn SupBookRecord(isAddInFuncs:bool,numberOfSheets:i16,url:Option<String>,sheetNames:Option<SheetNames>)->Self{Self{field_1_number_of_sheets:crate::java_runtime::Cell::new(numberOfSheets),field_2_encoded_url:url,field_3_sheet_names:sheetNames,_isAddInFunctions:isAddInFuncs,isSameSheetRefer:false}}
    // java public boolean isAddInFunctions(){return this._isAddInFunctions;}
    pub fn isAddInFunctions(&self)->bool{self._isAddInFunctions}
    // java public boolean isExternalReferences(){return !this._isAddInFunctions&&this.field_2_encoded_url!=null&&this.field_2_encoded_url.length()>0&&this.field_3_sheet_names!=null&&this.field_3_sheet_names.length>0;}
    pub fn isExternalReferences(&self)->bool{!self._isAddInFunctions&&self.field_2_encoded_url.as_ref().is_some_and(|s|!s.is_empty())&&self.field_3_sheet_names.as_ref().is_some_and(|s|!s.borrow().is_empty())}
    // java public boolean isNullSupBook(){return !this._isAddInFunctions&&this.field_2_encoded_url==null&&this.field_3_sheet_names==null;}
    pub fn isNullSupBook(&self)->bool{!self._isAddInFunctions&&self.field_2_encoded_url.is_none()&&self.field_3_sheet_names.is_none()}
    // java public boolean isEmptySupBook(){return !this._isAddInFunctions&&this.field_2_encoded_url!=null&&this.field_2_encoded_url.length()==0&&this.field_3_sheet_names!=null&&this.field_3_sheet_names.length==0;}
    pub fn isEmptySupBook(&self)->bool{!self._isAddInFunctions&&self.field_2_encoded_url.as_ref().is_some_and(String::is_empty)&&self.field_3_sheet_names.as_ref().is_some_and(|s|s.borrow().is_empty())}
    // java public boolean isInternalReferences(){return !this._isAddInFunctions&&(this.field_2_encoded_url==null||this.field_2_encoded_url.length()==0)&&(this.field_3_sheet_names==null||this.field_3_sheet_names.length==0);}
    pub fn isInternalReferences(&self)->bool{!self._isAddInFunctions&&self.field_2_encoded_url.as_ref().is_none_or(String::is_empty)&&self.field_3_sheet_names.as_ref().is_none_or(|s|s.borrow().is_empty())}
    // java public boolean isSameSheetRef(){return !this._isAddInFunctions&&this.isSameSheetRefer;}
    pub fn isSameSheetRef(&self)->bool{!self._isAddInFunctions&&self.isSameSheetRefer}
    // java public String toString(){StringBuffer sb=new StringBuffer();sb.append(this.getClass().getName()).append(" [SUPBOOK ");if(this.isExternalReferences()){sb.append("External References");sb.append(" nSheets=").append(this.field_1_number_of_sheets);sb.append(" url=").append(this.field_2_encoded_url);}else if(this._isAddInFunctions){sb.append("Add-In Functions");}else{sb.append("Internal References ");sb.append(" nSheets= ").append(this.field_1_number_of_sheets);}sb.append("]");return sb.toString();}
    pub fn toString(&self)->String{let mut sb=String::from("cn.wps.moss.engine.linktb.SupBookRecord [SUPBOOK ");if self.isExternalReferences(){sb.push_str("External References");sb.push_str(&format!(" nSheets={}",self.field_1_number_of_sheets.get()));sb.push_str(" url=");sb.push_str(self.field_2_encoded_url.as_deref().unwrap_or("null"));}else if self._isAddInFunctions{sb.push_str("Add-In Functions");}else{sb.push_str("Internal References ");sb.push_str(&format!(" nSheets= {}",self.field_1_number_of_sheets.get()));}sb.push(']');sb}
    // java protected int getDataSize(){if(this.isSameSheetRefer){return 6;}else if(!this.isExternalReferences()){return 4;}else{int sum=2;sum+=StringUtil.getEncodedSize(this.field_2_encoded_url);for(int i=0;i<this.field_3_sheet_names.length;++i){sum+=StringUtil.getEncodedSize(this.field_3_sheet_names[i]);}return sum;}}
    pub(crate) fn getDataSize(&self)->i32{if self.isSameSheetRefer{6}else if !self.isExternalReferences(){4}else{let mut sum=2;sum+=StringUtil::getEncodedSize(self.field_2_encoded_url.as_deref().expect("NullPointerException"));let names=self.field_3_sheet_names.as_ref().expect("NullPointerException").borrow();for i in 0..names.len(){sum=sum.wrapping_add(StringUtil::getEncodedSize(names[i].as_deref().expect("NullPointerException")));}sum}}
    // java public void serializeData(LittleEndianOutput out){out.writeShort(this.field_1_number_of_sheets);if(this.isSameSheetRefer){out.writeShort(1);out.writeShort(0);}else{if(this.isExternalReferences()){StringUtil.writeUnicodeString(out,this.field_2_encoded_url);for(int i=0;i<this.field_3_sheet_names.length;++i){StringUtil.writeUnicodeString(out,this.field_3_sheet_names[i]);}}else{int field2val=this._isAddInFunctions?14849:1025;out.writeShort(field2val);}}}
    pub fn serializeData(&self,out:&mut dyn LittleEndianOutput){out.writeShort(self.field_1_number_of_sheets.get()as i32);if self.isSameSheetRefer{out.writeShort(1);out.writeShort(0);}else{if self.isExternalReferences(){StringUtil::writeUnicodeString(out,self.field_2_encoded_url.as_deref().expect("NullPointerException"));let names=self.field_3_sheet_names.as_ref().expect("NullPointerException").borrow();for i in 0..names.len(){StringUtil::writeUnicodeString(out,names[i].as_deref().expect("NullPointerException"));}}else{let field2val=if self._isAddInFunctions{14849}else{1025};out.writeShort(field2val);}}}
    // java public void setNumberOfSheets(short number){this.field_1_number_of_sheets=number;}
    pub fn setNumberOfSheets(&self,number:i16){self.field_1_number_of_sheets.set(number);}
    // java public short getNumberOfSheets(){return this.field_1_number_of_sheets;}
    pub fn getNumberOfSheets(&self)->i16{self.field_1_number_of_sheets.get()}
    // java public short getSid(){return 430;}
    pub fn getSid(&self)->i16{430}
    // java public String getURL(){String encodedUrl=this.field_2_encoded_url;switch(encodedUrl.charAt(0)){case '\u0000':return encodedUrl.substring(1);case '\u0001':return decodeFileName(encodedUrl);case '\u0002':return encodedUrl.substring(1);default:return encodedUrl;}}
    pub fn getURL(&self)->String{let encodedUrl=self.field_2_encoded_url.as_ref().expect("NullPointerException");let units:Vec<_>=encodedUrl.encode_utf16().collect();let first=*units.first().expect("StringIndexOutOfBoundsException");match first{0|2=>String::from_utf16(&units[1..]).expect("UTF16 string facade"),1=>Self::decodeFileName(encodedUrl),_=>encodedUrl.clone()}}
    // java private static String decodeFileName(String encodedUrl) {
    fn decodeFileName(encodedUrl:&str)->String{
        let encodedUrl:Vec<_>=encodedUrl.encode_utf16().collect();
        // java if(encodedUrl.charAt(0)==1&&encodedUrl.length()>3){StringBuilder ret=new StringBuilder(encodedUrl.length());switch(encodedUrl.charAt(1)){
        if encodedUrl[0]==1&&encodedUrl.len()>3{let mut ret=vec![];match encodedUrl[1]{
            // java case '\u0001':{char ch=encodedUrl.charAt(2);if(ch!='@'){ret.append(ch);ret.append(":/");}else{ret.append("//");}int i=3;for(int count=encodedUrl.length();i<count;++i){ch=encodedUrl.charAt(i);if(ch!=3){ret.append(ch);}else{ret.append('/');}}break;}
            1=>{let mut ch=encodedUrl[2];if ch!=64{ret.push(ch);ret.extend(":/".encode_utf16());}else{ret.extend("//".encode_utf16());}let mut i=3;let count=encodedUrl.len();while i<count{ch=encodedUrl[i];if ch!=3{ret.push(ch);}else{ret.push(47);}i+=1;}},
            // java case '\u0002':{ret.append("/");int j=2;for(int count=encodedUrl.length();j<count;++j){char ch2=encodedUrl.charAt(j);if(ch2!=3){ret.append(ch2);}else{ret.append('/');}}break;}
            2=>{ret.push(47);let mut j=2;let count=encodedUrl.len();while j<count{let ch2=encodedUrl[j];if ch2!=3{ret.push(ch2);}else{ret.push(47);}j+=1;}},
            // java case '\u0005':{char ch5=encodedUrl.charAt(2);if(ch5!='='){ret.append(encodedUrl.substring(2));}else{ret.append(encodedUrl.substring(3));}break;}
            5=>{let ch5=encodedUrl[2];if ch5!=61{ret.extend_from_slice(&encodedUrl[2..]);}else{ret.extend_from_slice(&encodedUrl[3..]);}},
            // java case '\u0003':case '\u0004':default:{ret.append(encodedUrl.substring(1));for(int k=0;k<ret.length();++k){int _0x3=encodedUrl.charAt(k);if(_0x3==3){ret.replace(k-1,k,"/");}}break;}
            _=>{ret.extend_from_slice(&encodedUrl[1..]);for k in 0..ret.len(){let _0x3=encodedUrl[k];if _0x3==3{let start=k.checked_sub(1).expect("StringIndexOutOfBoundsException");ret.splice(start..k,[47]);}}},
        }
        // java return ret.toString();}else{return encodedUrl;}
        String::from_utf16(&ret).expect("UTF16 string facade")}else{String::from_utf16(&encodedUrl).expect("UTF16 string facade")}
    }
    // java public static String encodeFileName(String url) {
    pub fn encodeFileName(url:Option<&str>)->String{
        let url=url.expect("NullPointerException");
        // java if(url.length()==0){return " ";}else{int index=0;StringBuilder ret=new StringBuilder(url.length());ret.append('\u0001');
        if url.is_empty(){return " ".into();}let units:Vec<_>=url.encode_utf16().collect();let mut index=0;let mut ret=vec![1u16];
        // java if(!url.startsWith("file:///")){
        if !url.starts_with("file:///"){
            // java if(url.charAt(0)=='/'){ret.append('\u0002');index=1;for(int count=url.length();index<count;++index){char ch=url.charAt(index);if(ch!='/'&&ch!='\\'){ret.append(ch);}else{ret.append('\u0003');}}}
            if units[0]==47{ret.push(2);index=1;let count=units.len();while index<count{let ch=units[index];if ch!=47&&ch!=92{ret.push(ch);}else{ret.push(3);}index+=1;}}
            // java else if(!url.startsWith("http://")&&!url.startsWith("ftp://")){ret.append(url);}else{ret.append('\u0005');ret.append(url);}
            else if !url.starts_with("http://")&&!url.starts_with("ftp://"){ret.extend_from_slice(&units);}else{ret.push(5);ret.extend_from_slice(&units);}
        // java }else{index=8;ret.append('\u0001');if(url.charAt(index+1)!=':'||url.charAt(index+2)!='/'&&url.charAt(index+2)!='\\'){
        }else{index=8;ret.push(1);if *units.get(index+1).expect("StringIndexOutOfBoundsException")!=58||(*units.get(index+2).expect("StringIndexOutOfBoundsException")!=47&&units[index+2]!=92){
            // java if((url.charAt(index)!='/'||url.charAt(index+1)!='/')&&(url.charAt(index)!='\\'||url.charAt(index+1)!='\\')){throw new IllegalStateException("未考虑的用况: "+url);}ret.append('@');index+=2;
            if (units[index]!=47||units[index+1]!=47)&&(units[index]!=92||units[index+1]!=92){panic!("IllegalStateException 未考虑的用况: {url}");}ret.push(64);index+=2;
        // java }else{ret.append(url.charAt(index));index+=3;}for(int count=url.length();index<count;++index){char ch=url.charAt(index);if(ch!='/'&&ch!='\\'){ret.append(ch);}else{ret.append('\u0003');}}}return ret.toString();
        }else{ret.push(units[index]);index+=3;}let count=units.len();while index<count{let ch=units[index];if ch!=47&&ch!=92{ret.push(ch);}else{ret.push(3);}index+=1;}}String::from_utf16(&ret).expect("UTF16 string facade")
    }
    // java public String[] getSheetNames(){return (String[])this.field_3_sheet_names.clone();}
    pub fn getSheetNames(&self)->Vec<Option<String>>{self.field_3_sheet_names.as_ref().expect("NullPointerException").borrow().clone()}
}
