use crate::{cn::wps::{et::ss::{constant::LangHelper::LangHelper,formula::parser::FormulaParser},moss::{app::KmoBook::KmoBook,engine::grid::KmoCellValueStore::KmoCellValueStore,util::KmoRect::KmoRect}},org::apache::poi::ss::{SpreadsheetVersion::SpreadsheetVersion,util::CellReference::{CellReference,NameType}},java_runtime::Character};
use std::panic::{catch_unwind,AssertUnwindSafe};
// java public class KmoNameManager {
pub struct KmoNameManager;
impl KmoNameManager{
    // java public static final int VALID=0,EXCEED_255_CHARACTERS=1,NT_BEG_LETTER_UNDERSCORE=2,INC_SPACE_INVALID_CHARS=3;
    pub const VALID:i32=0;pub const EXCEED_255_CHARACTERS:i32=1;pub const NT_BEG_LETTER_UNDERSCORE:i32=2;pub const INC_SPACE_INVALID_CHARS:i32=3;
    // java public static int judgeValidName(String kmoName,SpreadsheetVersion version){
    pub fn judgeValidName(kmoName:Option<&str>,version:Option<SpreadsheetVersion>)->i32{
        // java if(kmoName==null){return 2;}
        let Some(kmoName)=kmoName else{return 2;};
        // java kmoName=kmoName.trim();if(kmoName.length()==0){return 2;}else if(kmoName.length()>255){return 1;}
        let kmoName=kmoName.trim_matches(|c:char|c<='\u{20}');let units:Vec<_>=kmoName.encode_utf16().collect();if units.is_empty(){return 2;}else if units.len()>255{return 1;}
        // java int _pointer=0;char look=kmoName.charAt(_pointer++);
        let mut _pointer=0;let mut look=units[_pointer];_pointer+=1;
        // java if(!FormulaParser.isValidDefinedNameFirstChar(look)){return 2;}
        if !FormulaParser::isValidDefinedNameFirstChar(look){return 2;}
        // java for(StringBuilder sb=new StringBuilder();FormulaParser.isValidDefinedNameChar(look);look=kmoName.charAt(_pointer++)){
        let mut sb=vec![];while FormulaParser::isValidDefinedNameChar(look){
            // java sb.append(look);if(kmoName.length()<=_pointer){
            sb.push(look);if units.len()<=_pointer{
                // java if(kmoName.length()>_pointer){return 3;}
                if units.len()>_pointer{return 3;}
                // java String name=sb.toString();String half_horn_name=LangHelper.toHalfHornString(name);
                let half_horn_name=LangHelper::toHalfHornString(&sb);
                // java if(!half_horn_name.equalsIgnoreCase("TRUE")&&!half_horn_name.equalsIgnoreCase("FALSE")){
                if !Character::equalsIgnoreCaseAscii(&half_horn_name,b"TRUE")&&!Character::equalsIgnoreCaseAscii(&half_horn_name,b"FALSE"){
                    // java try{if(CellReference.classifyCellReference(kmoName,version)==NameType.CELL){return 3;}}catch(Exception var8){}
                    let classification=catch_unwind(AssertUnwindSafe(||CellReference::classifyCellReference(Some(kmoName),version.expect("NullPointerException"))));
                    match classification{Ok(NameType::CELL)=>return 3,Ok(_)=>{},Err(e)=>{let text=e.downcast_ref::<String>().map(String::as_str).or_else(||e.downcast_ref::<&str>().copied()).unwrap_or("");if !["IllegalArgumentException","IllegalStateException","NumberFormatException","NullPointerException"].iter().any(|v|text.starts_with(v)){std::panic::resume_unwind(e);}}}
                    // java return 0;
                    return 0;
                }
                // java return 3;
                return 3;
            }
            look=units[_pointer];_pointer+=1;
        }
        // java return 3;
        3
    }
    // java public static String getTopLeftName(KmoBook book,KmoRect rect){
    pub fn getTopLeftName(book:Option<&KmoBook>,rect:Option<&KmoRect>)->Option<String>{
        let rect=rect.expect("NullPointerException");
        // java if(rect.isSingleCell()){String validCellName=getCellString(book,rect.tl.row-1,rect.tl.col);return validCellName!=null?validCellName:getCellString(book,rect.tl.row,rect.tl.col-1);}
        if rect.isSingleCell(){let validCellName=Self::getCellString(book,rect.tl.row.wrapping_sub(1),rect.tl.col);if validCellName.is_some(){validCellName}else{Self::getCellString(book,rect.tl.row,rect.tl.col.wrapping_sub(1))}}
        // java else if(rect.height()>rect.width()){return getCellString(book,rect.tl.row-1,rect.tl.col);}
        else if rect.height()>rect.width(){Self::getCellString(book,rect.tl.row.wrapping_sub(1),rect.tl.col)}
        // java else if(rect.height()<rect.width()){return getCellString(book,rect.tl.row,rect.tl.col-1);}
        else if rect.height()<rect.width(){Self::getCellString(book,rect.tl.row,rect.tl.col.wrapping_sub(1))}
        // java else{String validCellName=getCellString(book,rect.tl.row-1,rect.tl.col);return validCellName!=null?validCellName:getCellString(book,rect.tl.row,rect.tl.col-1);}
        else{let validCellName=Self::getCellString(book,rect.tl.row.wrapping_sub(1),rect.tl.col);if validCellName.is_some(){validCellName}else{Self::getCellString(book,rect.tl.row,rect.tl.col.wrapping_sub(1))}}
    }
    // java private static String getCellString(KmoBook book,int row,int col){KmoSheet kmoSheet=book.getActiveSheet();if((new KmoRect(row,col,row,col)).isValid()){int id=kmoSheet.getCellId(row,col);return fetchPossibleCellName(book,id,row,col);}else{return null;}}
    fn getCellString(book:Option<&KmoBook>,row:i32,col:i32)->Option<String>{let kmoSheet=book.expect("NullPointerException").getActiveSheet();if KmoRect::KmoRect__bounds(row,col,row,col).isValid(){let id=kmoSheet.borrow().getCellId(row,col);Self::fetchPossibleCellName(book,id,row,col)}else{None}}
    // java private static String fetchPossibleCellName(KmoBook book,int id,int row,int col){if(KmoCellValueStore.identifyTypeByID(id)!=2){return null;}else{KmoSheet kmoSheet=book.getActiveSheet();String strNeedBeValid=kmoSheet.getCellStringValue(row,col);return changeStrValid(strNeedBeValid);}}
    fn fetchPossibleCellName(book:Option<&KmoBook>,id:i32,row:i32,col:i32)->Option<String>{if KmoCellValueStore::identifyTypeByID(id)!=2{None}else{let kmoSheet=book.expect("NullPointerException").getActiveSheet();let strNeedBeValid=kmoSheet.borrow().getCellStringValue(row,col);Self::changeStrValid(Some(&strNeedBeValid))}}
    // java private static String changeStrValid(String strNeedBeValid){
    fn changeStrValid(strNeedBeValid:Option<&str>)->Option<String>{
        // java strNeedBeValid=strNeedBeValid.trim();StringBuilder sb=new StringBuilder();int strlength=strNeedBeValid.length();if(strlength<=0){return null;}
        let text=strNeedBeValid.expect("NullPointerException").trim_matches(|c:char|c<='\u{20}');let units:Vec<_>=text.encode_utf16().collect();let mut sb=vec![];let strlength=units.len();if strlength==0{return None;}
        // java boolean lastCharChanged=false;int _pointer=0;char look=strNeedBeValid.charAt(_pointer++);
        let mut lastCharChanged=false;let mut _pointer=0;let mut look=units[_pointer];_pointer+=1;
        // java if(!FormulaParser.isValidDefinedNameFirstChar(look)){sb.append("_");if(Character.isDigit(look)){lastCharChanged=false;}else{lastCharChanged=true;}}
        if !FormulaParser::isValidDefinedNameFirstChar(look){sb.push(95);if Character::isDigit(look){lastCharChanged=false;}else{lastCharChanged=true;}}
        // java while(true){
        loop{
            // java if(FormulaParser.isValidDefinedNameChar(look)){sb.append(look);lastCharChanged=false;}else if(!lastCharChanged){sb.append("_");lastCharChanged=true;}
            if FormulaParser::isValidDefinedNameChar(look){sb.push(look);lastCharChanged=false;}else if !lastCharChanged{sb.push(95);lastCharChanged=true;}
            // java if(strlength<=_pointer){if(lastCharChanged&&sb.length()!=1){sb.deleteCharAt(sb.length()-1);}return sb.toString();}
            if strlength<=_pointer{if lastCharChanged&&sb.len()!=1{sb.pop();}return Some(String::from_utf16(&sb).expect("native UTF16 String facade boundary"));}
            // java look=strNeedBeValid.charAt(_pointer++);
            look=units[_pointer];_pointer+=1;
        }
    }
}
#[cfg(test)]
#[path="KmoNameManager.review.rs"]
mod review;
