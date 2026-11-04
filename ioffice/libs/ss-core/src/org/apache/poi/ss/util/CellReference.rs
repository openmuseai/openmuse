// java package org.apache.poi.ss.util;
use crate::{org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion,java_runtime::Character};
use std::sync::LazyLock;
use super::CellRangeAddressBase::CellRangeAddressBase;
use crate::cn::wps::et::ss::formula::SheetNameFormatter::SheetNameFormatter;
// java private static final char ABSOLUTE_REFERENCE_MARKER = '$';
const ABSOLUTE_REFERENCE_MARKER:char='$';
// java private static final char SHEET_NAME_DELIMITER = '!';
const SHEET_NAME_DELIMITER:char='!';
// java private static final char SPECIAL_NAME_DELIMITER = '\'';
const SPECIAL_NAME_DELIMITER:char='\'';
// java private static final Pattern CELL_REF_PATTERN=Pattern.compile("\\$?([A-Za-z]+)\\$?([0-9]+)");
static CELL_REF_PATTERN:LazyLock<regex::Regex>=LazyLock::new(||regex::Regex::new(r"\A\$?([A-Za-z]+)\$?([0-9]+)\z").unwrap());
// java private static final Pattern COLUMN_REF_PATTERN=Pattern.compile("\\$?([A-Za-z]+)");
static COLUMN_REF_PATTERN:LazyLock<regex::Regex>=LazyLock::new(||regex::Regex::new(r"\A\$?([A-Za-z]+)\z").unwrap());
// java private static final Pattern ROW_REF_PATTERN=Pattern.compile("\\$?([0-9]+)");
static ROW_REF_PATTERN:LazyLock<regex::Regex>=LazyLock::new(||regex::Regex::new(r"\A\$?([0-9]+)\z").unwrap());
// java private static final Pattern NAMED_RANGE_NAME_PATTERN=Pattern.compile("[_A-Za-z][_.A-Za-z0-9]*");
static NAMED_RANGE_NAME_PATTERN:LazyLock<regex::Regex>=LazyLock::new(||regex::Regex::new(r"\A[_A-Za-z][_.A-Za-z0-9]*\z").unwrap());

// java public class CellReference
pub struct CellReference {
    // java private final int _rowIndex;
    _rowIndex: i32,
    // java private final int _colIndex;
    _colIndex: i32,
    // java private final String _sheetName;
    _sheetName:Option<String>,
    // java private final boolean _isRowAbs;
    _isRowAbs:bool,
    // java private final boolean _isColAbs;
    _isColAbs:bool,
}

impl CellReference {
    // java public CellReference(String cellRef) {
    pub fn fromString(cellRef:&str)->Result<Self,String>{
        // java if (cellRef.endsWith("#REF!")) { throw new IllegalArgumentException("Cell reference invalid: " + cellRef); }
        if cellRef.ends_with("#REF!"){return Err(format!("IllegalArgumentException: Cell reference invalid: {cellRef}"));}
        // java String[] parts=separateRefParts(cellRef,false); this._sheetName=parts[0]; String colRef=parts[1];
        let(_sheetName,mut colRef,mut rowRef)=Self::separateRefParts__rc(cellRef,false)?;
        // java if (colRef.length()<1) { throw new IllegalArgumentException("Invalid Formula cell reference: '" + cellRef + "'"); }
        if colRef.is_empty(){return Err(format!("IllegalArgumentException: Invalid Formula cell reference: '{cellRef}'"));}
        // java this._isColAbs=colRef.charAt(0)=='$'; if(this._isColAbs){colRef=colRef.substring(1);}
        let _isColAbs=colRef.starts_with(ABSOLUTE_REFERENCE_MARKER);if _isColAbs{colRef=colRef[1..].into();}
        // java this._colIndex=convertColStringToIndex(colRef); String rowRef=parts[2];
        let _colIndex=Self::convertColStringToIndex(&colRef)?;
        // java if(rowRef.length()<1){throw new IllegalArgumentException("Invalid Formula cell reference: '"+cellRef+"'");}
        if rowRef.is_empty(){return Err(format!("IllegalArgumentException: Invalid Formula cell reference: '{cellRef}'"));}
        // java this._isRowAbs=rowRef.charAt(0)=='$'; if(this._isRowAbs){rowRef=rowRef.substring(1);}
        let _isRowAbs=rowRef.starts_with(ABSOLUTE_REFERENCE_MARKER);if _isRowAbs{rowRef=rowRef[1..].into();}
        // java this._rowIndex=Integer.parseInt(rowRef)-1;
        let _rowIndex=crate::java_runtime::DecimalInteger::parseInt(&rowRef.encode_utf16().collect::<Vec<_>>()).ok_or_else(||"NumberFormatException".to_string())?.wrapping_sub(1);
        Ok(Self{_rowIndex,_colIndex,_sheetName,_isRowAbs,_isColAbs})
    }
    // java public CellReference(int pRow, int pCol)
    pub fn CellReference(pRow: i32, pCol: i32) -> Self {
        // java this(pRow, pCol, false, false);
        Self::CellReference__absolute(pRow,pCol,false,false)
    }
    // java public CellReference(int pRow,short pCol){this(pRow,pCol&'\uffff',false,false);}
    pub fn CellReference__short(pRow:i32,pCol:i16)->Self{Self::CellReference__absolute(pRow,(pCol as u16)as i32,false,false)}
    // java public CellReference(int pRow,int pCol,boolean pAbsRow,boolean pAbsCol){this((String)null,pRow,pCol,pAbsRow,pAbsCol);}
    pub fn CellReference__absolute(pRow:i32,pCol:i32,pAbsRow:bool,pAbsCol:bool)->Self{Self::CellReference__sheet(None,pRow,pCol,pAbsRow,pAbsCol)}
    // java public CellReference(String pSheetName,int pRow,int pCol,boolean pAbsRow,boolean pAbsCol){
    pub fn CellReference__sheet(pSheetName:Option<String>,pRow:i32,pCol:i32,pAbsRow:bool,pAbsCol:bool)->Self{
        // java if(pRow<-1){throw new IllegalArgumentException("row index may not be negative");}else if(pCol<-1){throw new IllegalArgumentException("column index may not be negative");}
        assert!(pRow>=-1,"IllegalArgumentException: row index may not be negative");assert!(pCol>=-1,"IllegalArgumentException: column index may not be negative");
        // java this._sheetName=pSheetName;this._rowIndex=pRow;this._colIndex=pCol;this._isRowAbs=pAbsRow;this._isColAbs=pAbsCol;
        Self{_sheetName:pSheetName,_rowIndex:pRow,_colIndex:pCol,_isRowAbs:pAbsRow,_isColAbs:pAbsCol}
    }

    // java public int getRow()
    pub fn getRow(&self) -> i32 { self._rowIndex }

    // java public short getCol()
    pub fn getCol(&self) -> i16 { self._colIndex as i16 }
    // java public boolean isRowAbsolute(){return this._isRowAbs;}
    pub fn isRowAbsolute(&self)->bool{self._isRowAbs}
    // java public boolean isColAbsolute(){return this._isColAbs;}
    pub fn isColAbsolute(&self)->bool{self._isColAbs}
    // java public String getSheetName(){return this._sheetName;}
    pub fn getSheetName(&self)->Option<&str>{self._sheetName.as_deref()}
    // java public static boolean isPartAbsolute(String part){return part.charAt(0)=='$';}
    pub fn isPartAbsolute(part:Option<&str>)->bool{part.expect("NullPointerException").encode_utf16().next().expect("StringIndexOutOfBoundsException")==36}
// java public static int convertColStringToIndex(String ref) {
// java int pos = 0;
// java int retval = 0;
// java
// java for(int k = ref.length() - 1; k >= 0; --k) {
// java char thechar = ref.charAt(k);
// java if (thechar == '$') {
// java if (k != 0) {
// java throw new IllegalArgumentException("Bad col ref format '" + ref + "'");
// java }
// java break;
// java }
// java
// java int shift = (int)Math.pow((double)26.0F, (double)pos);
// java retval += (Character.getNumericValue(thechar) - 9) * shift;
// java ++pos;
// java }
// java
// java return retval - 1;
// java }
pub fn convertColStringToIndex(reference:&str)->Result<i32,String> {
    // java int pos=0;int retval=0;
    let mut pos=0;let mut retval=0i32;let chars:Vec<_>=reference.encode_utf16().collect();
    // java for(int k=ref.length()-1;k>=0;--k){char thechar=ref.charAt(k);
    for k in (0..chars.len()).rev(){let thechar=chars[k];
        // java if(thechar=='$'){if(k!=0){throw new IllegalArgumentException("Bad col ref format '"+ref+"'");}break;}
        if thechar==36{if k!=0{return Err("IllegalArgumentException: Bad col ref format".into());}break;}
        // java int shift=(int)Math.pow(26.0,pos);retval+=(Character.getNumericValue(thechar)-9)*shift;++pos;
        let shift=26.0f64.powi(pos)as i32;retval=retval.wrapping_add(Character::getNumericValue(thechar).wrapping_sub(9).wrapping_mul(shift));pos+=1;
    }
    // java return retval-1;
    Ok(retval.wrapping_sub(1))
}
    // java public static NameType classifyCellReference(String str,SpreadsheetVersion ssVersion){
    pub fn classifyCellReference(str:Option<&str>,ssVersion:SpreadsheetVersion)->NameType{
        let str=str.expect("NullPointerException");
        // java int len=str.length();if(len<1){throw new IllegalArgumentException("Empty string not allowed");}
        let units:Vec<_>=str.encode_utf16().collect();let len=units.len();assert!(len>=1,"IllegalArgumentException: Empty string not allowed");
        // java char firstChar=str.charAt(0);switch(firstChar){default:if(!Character.isLetter(firstChar)&&!Character.isDigit(firstChar)){throw new IllegalArgumentException(...);}case '$':case '.':case '_':
        let firstChar=units[0];if !matches!(firstChar,36|46|95)&&!Character::isLetter(firstChar)&&!Character::isDigit(firstChar){panic!("IllegalArgumentException: invalid first char");}
        // java if(!Character.isDigit(str.charAt(len-1))){return validateNamedRangeName(str,ssVersion);}
        if !Character::isDigit(units[len-1]){return Self::validateNamedRangeName(str,ssVersion);}
        // java Matcher cellRefPatternMatcher=CELL_REF_PATTERN.matcher(str);if(!cellRefPatternMatcher.matches()){return validateNamedRangeName(str,ssVersion);}
        let Some(cellRefPatternMatcher)=CELL_REF_PATTERN.captures(str)else{return Self::validateNamedRangeName(str,ssVersion);};
        // java String lettersGroup=cellRefPatternMatcher.group(1);String digitsGroup=cellRefPatternMatcher.group(2);
        let lettersGroup=&cellRefPatternMatcher[1];let digitsGroup=&cellRefPatternMatcher[2];
        // java if(cellReferenceIsWithinRange(lettersGroup,digitsGroup,ssVersion)){return NameType.CELL;}else{return str.indexOf(36)>=0?NameType.BAD_CELL_OR_NAMED_RANGE:NameType.NAMED_RANGE;}
        if Self::cellReferenceIsWithinRange(lettersGroup,digitsGroup,ssVersion){NameType::CELL}else if str.contains('$'){NameType::BAD_CELL_OR_NAMED_RANGE}else{NameType::NAMED_RANGE}
    }
    // java private static NameType validateNamedRangeName(String str,SpreadsheetVersion ssVersion){
    fn validateNamedRangeName(str:&str,ssVersion:SpreadsheetVersion)->NameType{
        // java Matcher colMatcher=COLUMN_REF_PATTERN.matcher(str);if(colMatcher.matches()){String colStr=colMatcher.group(1);if(isColumnWithnRange(colStr,ssVersion)){return NameType.COLUMN;}}
        if let Some(colMatcher)=COLUMN_REF_PATTERN.captures(str){let colStr=&colMatcher[1];if Self::isColumnWithnRange(colStr,ssVersion){return NameType::COLUMN;}}
        // java Matcher rowMatcher=ROW_REF_PATTERN.matcher(str);if(rowMatcher.matches()){String rowStr=rowMatcher.group(1);if(isRowWithnRange(rowStr,ssVersion)){return NameType.ROW;}}
        if let Some(rowMatcher)=ROW_REF_PATTERN.captures(str){let rowStr=&rowMatcher[1];if Self::isRowWithnRange(rowStr,ssVersion){return NameType::ROW;}}
        // java return !NAMED_RANGE_NAME_PATTERN.matcher(str).matches()?NameType.BAD_CELL_OR_NAMED_RANGE:NameType.NAMED_RANGE;
        if !NAMED_RANGE_NAME_PATTERN.is_match(str){NameType::BAD_CELL_OR_NAMED_RANGE}else{NameType::NAMED_RANGE}
    }
    // java public static boolean cellReferenceIsWithinRange(String colStr,String rowStr,SpreadsheetVersion ssVersion){return !isColumnWithnRange(colStr,ssVersion)?false:isRowWithnRange(rowStr,ssVersion);}
    pub fn cellReferenceIsWithinRange(colStr:&str,rowStr:&str,ssVersion:SpreadsheetVersion)->bool{if !Self::isColumnWithnRange(colStr,ssVersion){false}else{Self::isRowWithnRange(rowStr,ssVersion)}}
    // java public static boolean isColumnWithnRange(String colStr,SpreadsheetVersion ssVersion){
    pub fn isColumnWithnRange(colStr:&str,ssVersion:SpreadsheetVersion)->bool{
        // java String lastCol=ssVersion.getLastColumnName();int lastColLength=lastCol.length();int numberOfLetters=colStr.length();
        let lastCol=ssVersion.getLastColumnName();let lastColLength=lastCol.encode_utf16().count();let numberOfLetters=colStr.encode_utf16().count();
        // java if(numberOfLetters>lastColLength){return false;}else{return numberOfLetters!=lastColLength||colStr.toUpperCase().compareTo(lastCol)<=0;}
        if numberOfLetters>lastColLength{false}else{numberOfLetters!=lastColLength||colStr.to_uppercase().encode_utf16().cmp(lastCol.encode_utf16())!=std::cmp::Ordering::Greater}
    }
    // java public static boolean isRowWithnRange(String rowStr,SpreadsheetVersion ssVersion){
    pub fn isRowWithnRange(rowStr:&str,ssVersion:SpreadsheetVersion)->bool{
        // java int rowNum=Integer.parseInt(rowStr);
        let rowNum=crate::java_runtime::DecimalInteger::parseInt(&rowStr.encode_utf16().collect::<Vec<_>>()).expect("NumberFormatException");
        // java if(rowNum<0){throw new IllegalStateException(...);}else if(rowNum==0){return false;}else{return rowNum<=ssVersion.getMaxRows();}
        assert!(rowNum>=0,"IllegalStateException: Invalid rowStr");if rowNum==0{false}else{rowNum<=ssVersion.getMaxRows()}
    }
// java private static String[] separateRefParts(String reference, boolean isRC) {
// java int plingPos = reference.lastIndexOf(33);
// java String sheetName = parseSheetName(reference, plingPos);
// java int start = plingPos + 1;
// java int length = reference.length();
// java int loc = start;
// java if (reference.charAt(start) == '$') {
// java loc = start + 1;
// java }
// java
// java for(; loc < length; ++loc) {
// java char ch = reference.charAt(loc);
// java if (isRC) {
// java if (ch == 'C') {
// java break;
// java }
// java } else if (Character.isDigit(ch) || ch == '$') {
// java break;
// java }
// java }
// java
// java return new String[]{sheetName, reference.substring(start, loc), reference.substring(loc)};
// java }
pub fn separateRefParts(reference:&str)->Result<(Option<String>,String,String),String> {
    Self::separateRefParts__rc(reference,false)
}
// rust: boolean overload preserves the original private method; legacy wrapper above supplies false.
pub(crate) fn separateRefParts__rc(reference:&str,isRC:bool)->Result<(Option<String>,String,String),String>{
    // java int plingPos=reference.lastIndexOf(33);String sheetName=parseSheetName(reference,plingPos);
    let pling=reference.rfind(SHEET_NAME_DELIMITER);let sheetName=Self::parseSheetName(reference,pling)?;
    // java int start=plingPos+1;int length=reference.length();int loc=start;
    // rust: ASCII delimiter byte position is converted to the same UTF16 index before slicing.
    let chars:Vec<_>=reference.encode_utf16().collect();let start=pling.map_or(0,|p|reference[..p+1].encode_utf16().count());let length=chars.len();let mut loc=start;
    // java if(reference.charAt(start)=='$'){loc=start+1;}
    let ch=chars.get(start).ok_or("StringIndexOutOfBoundsException")?;if *ch==36{loc=start+1;}
    // java for(;loc<length;++loc){char ch=reference.charAt(loc);if(isRC){if(ch=='C'){break;}}else if(Character.isDigit(ch)||ch=='$'){break;}}
    while loc<length{let ch=chars[loc];if isRC{if ch==67{break;}}else if Character::isDigit(ch)||ch==36{break;}loc+=1;}
    // java return new String[]{sheetName,reference.substring(start,loc),reference.substring(loc)};
    Ok((sheetName,String::from_utf16(&chars[start..loc]).expect("native valid String boundary"),String::from_utf16(&chars[loc..]).expect("native valid String boundary")))
}
// java private static String parseSheetName(String reference, int indexOfSheetNameDelimiter) {
// java if (indexOfSheetNameDelimiter < 0) {
// java return null;
// java } else {
// java boolean isQuoted = reference.charAt(0) == '\'';
// java if (!isQuoted) {
// java return reference.substring(0, indexOfSheetNameDelimiter);
// java } else {
// java int lastQuotePos = indexOfSheetNameDelimiter - 1;
// java if (reference.charAt(lastQuotePos) != '\'') {
// java throw new RuntimeException("Mismatched quotes: (" + reference + ")");
// java } else {
// java StringBuffer sb = new StringBuffer(indexOfSheetNameDelimiter);
// java
// java for(int i = 1; i < lastQuotePos; ++i) {
// java char ch = reference.charAt(i);
// java if (ch != '\'') {
// java sb.append(ch);
// java } else {
// java if (i >= lastQuotePos || reference.charAt(i + 1) != '\'') {
// java throw new RuntimeException("Bad sheet name quote escaping: (" + reference + ")");
// java }
// java
// java ++i;
// java sb.append(ch);
// java }
// java }
// java
// java return sb.toString();
// java }
// java }
// java }
// java }
pub fn parseSheetName(reference:&str,pling:Option<usize>)->Result<Option<String>,String> {
    // java if(indexOfSheetNameDelimiter<0){return null;}
    let Some(p)=pling else{return Ok(None);};
    // java boolean isQuoted=reference.charAt(0)=='\'';if(!isQuoted){return reference.substring(0,indexOfSheetNameDelimiter);}
    if !reference.starts_with(SPECIAL_NAME_DELIMITER){return Ok(Some(reference[..p].into()));}
    // java int lastQuotePos=indexOfSheetNameDelimiter-1;if(reference.charAt(lastQuotePos)!='\''){throw new RuntimeException("Mismatched quotes: ("+reference+")");}
    let text:Vec<_>=reference[..p].encode_utf16().collect();let Some(lastQuotePos)=text.len().checked_sub(1)else{return Err("StringIndexOutOfBoundsException".into());};if text[lastQuotePos]!=39{return Err("RuntimeException: Mismatched quotes".into());}
    // java StringBuffer sb=new StringBuffer(indexOfSheetNameDelimiter);for(int i=1;i<lastQuotePos;++i){char ch=reference.charAt(i);
    let mut sb=vec![];let mut i=1;while i<lastQuotePos{let ch=text[i];
        // java if(ch!='\''){sb.append(ch);}else{if(i>=lastQuotePos||reference.charAt(i+1)!='\''){throw new RuntimeException("Bad sheet name quote escaping: ("+reference+")");}++i;sb.append(ch);}
        if ch!=39{sb.push(ch);}else{if i>=lastQuotePos||text[i+1]!=39{return Err("RuntimeException: Bad sheet name quote escaping".into());}i+=1;sb.push(ch);}i+=1;
    }
    // java return sb.toString();
    Ok(Some(String::from_utf16(&sb).expect("native valid String boundary")))
}
    // java public static String convertNumToColString(int col) {
    pub fn convertNumToColString(col:i32)->String{
        // java int excelColNum = col + 1; String colRef = "";
        let excelColNum=col.wrapping_add(1);let mut colRef=String::new();
        // java char colChar; for(int colRemain=excelColNum;colRemain>0;colRef=colChar+colRef){
        let mut colRemain=excelColNum;while colRemain>0{
            // java int thisPart=colRemain%26;if(thisPart==0){thisPart=26;}
            let mut thisPart=colRemain%26;if thisPart==0{thisPart=26;}
            // java colRemain=(colRemain-thisPart)/26;colChar=(char)(thisPart+64);
            colRemain=(colRemain-thisPart)/26;let colChar=char::from_u32((thisPart+64)as u32).unwrap();
            // java colRef=colChar+colRef;
            colRef.insert(0,colChar);
        }
        // java return colRef;
        colRef
    }

    // java public String formatAsString() {
    pub fn formatAsString(&self)->String{
        // java StringBuilder sb=new StringBuilder(32);if(this._sheetName!=null){SheetNameFormatter.appendFormat(sb,this._sheetName);sb.append('!');}
        let mut sb=String::with_capacity(32);if let Some(sheet)=self._sheetName.as_deref(){SheetNameFormatter::appendFormat(&mut sb,Some(sheet));sb.push('!');}
        // java this.appendCellReference(sb);return sb.toString();
        self.appendCellReference(&mut sb);sb
    }
    // java public String formatAsRCString(CellRangeAddressBase formulaAddress,CellRangeAddressBase originAddress){
    pub fn formatAsRCString(&self,formulaAddress:Option<&CellRangeAddressBase>,originAddress:Option<&CellRangeAddressBase>)->String{
        // java StringBuilder sb=new StringBuilder(32);if(this._sheetName!=null){SheetNameFormatter.appendFormat(sb,this._sheetName);sb.append('!');}
        let mut sb=String::with_capacity(32);if let Some(sheet)=self._sheetName.as_deref(){SheetNameFormatter::appendFormat(&mut sb,Some(sheet));sb.push('!');}
        // java this.appendRCCellReference(sb,formulaAddress,originAddress);return sb.toString();
        self.appendRCCellReference(&mut sb,formulaAddress,originAddress);sb
    }
    // java public String toString(){StringBuffer sb=new StringBuffer(64);sb.append(this.getClass().getName()).append(" [");sb.append(this.formatAsString());sb.append("]");return sb.toString();}
    pub fn toString(&self)->String{let mut sb=String::with_capacity(64);sb.push_str("org.apache.poi.ss.util.CellReference [");sb.push_str(&self.formatAsString());sb.push(']');sb}
    // java public String[] getCellRefParts(){return new String[]{this._sheetName,Integer.toString(this._rowIndex+1),convertNumToColString(this._colIndex)};}
    pub fn getCellRefParts(&self)->[Option<String>;3]{[self._sheetName.clone(),Some(self._rowIndex.wrapping_add(1).to_string()),Some(Self::convertNumToColString(self._colIndex))]}
    // java void appendCellReference(StringBuilder sb){
    pub(crate) fn appendCellReference(&self,sb:&mut String){
        // java if(this._isColAbs){sb.append('$');}sb.append(convertNumToColString(this._colIndex));
        if self._isColAbs{sb.push('$');}sb.push_str(&Self::convertNumToColString(self._colIndex));
        // java if(this._isRowAbs){sb.append('$');}sb.append(this._rowIndex+1);
        if self._isRowAbs{sb.push('$');}sb.push_str(&self._rowIndex.wrapping_add(1).to_string());
    }
    // java void appendRCCellReference(StringBuilder sb,CellRangeAddressBase formulaAddress,CellRangeAddressBase originAddress){
    pub(crate) fn appendRCCellReference(&self,sb:&mut String,formulaAddress:Option<&CellRangeAddressBase>,originAddress:Option<&CellRangeAddressBase>){
        // java if(!this._isRowAbs){sb.append('R');if(this._rowIndex-formulaAddress.getFirstRow()!=0){sb.append('[');sb.append(this._rowIndex-formulaAddress.getFirstRow());sb.append(']');}}
        if !self._isRowAbs{sb.push('R');if self._rowIndex.wrapping_sub(formulaAddress.expect("NullPointerException").getFirstRow())!=0{sb.push('[');sb.push_str(&self._rowIndex.wrapping_sub(formulaAddress.unwrap().getFirstRow()).to_string());sb.push(']');}}
        // java else{sb.append('R');sb.append(this._rowIndex-originAddress.getFirstRow()+1);}
        else{sb.push('R');sb.push_str(&self._rowIndex.wrapping_sub(originAddress.expect("NullPointerException").getFirstRow()).wrapping_add(1).to_string());}
        // java if(!this._isColAbs){sb.append('C');if(this._colIndex-formulaAddress.getFirstColumn()!=0){sb.append('[');sb.append(this._colIndex-formulaAddress.getFirstColumn());sb.append(']');}}
        if !self._isColAbs{sb.push('C');if self._colIndex.wrapping_sub(formulaAddress.expect("NullPointerException").getFirstColumn())!=0{sb.push('[');sb.push_str(&self._colIndex.wrapping_sub(formulaAddress.unwrap().getFirstColumn()).to_string());sb.push(']');}}
        // java else{sb.append('C');sb.append(this._colIndex-originAddress.getFirstColumn()+1);}
        else{sb.push('C');sb.push_str(&self._colIndex.wrapping_sub(originAddress.expect("NullPointerException").getFirstColumn()).wrapping_add(1).to_string());}
    }
    // java public int hashCode(){int prime=31;int result=1;
    pub fn hashCode(&self)->i32{
        let mut result=1i32;
        // java result=31*result+this._colIndex;
        result=result.wrapping_mul(31).wrapping_add(self._colIndex);
        // java result=31*result+(this._isColAbs?1231:1237);
        result=result.wrapping_mul(31).wrapping_add(if self._isColAbs{1231}else{1237});
        // java result=31*result+(this._isRowAbs?1231:1237);
        result=result.wrapping_mul(31).wrapping_add(if self._isRowAbs{1231}else{1237});
        // java result=31*result+this._rowIndex;
        result=result.wrapping_mul(31).wrapping_add(self._rowIndex);
        // java result=31*result+(this._sheetName==null?0:this._sheetName.hashCode());return result;
        result=result.wrapping_mul(31).wrapping_add(self._sheetName.as_ref().map_or(0,|s|s.encode_utf16().fold(0i32,|h,c|h.wrapping_mul(31).wrapping_add(c as i32))));result
    }
    // java public boolean equals(Object o){if(!(o instanceof CellReference)){return false;}else{CellReference cr=(CellReference)o;return this._rowIndex==cr._rowIndex&&this._colIndex==cr._colIndex&&this._isRowAbs==cr._isColAbs&&this._isColAbs==cr._isColAbs;}}
    pub fn equals(&self,o:Option<&Self>)->bool{let Some(cr)=o else{return false;};self._rowIndex==cr._rowIndex&&self._colIndex==cr._colIndex&&self._isRowAbs==cr._isColAbs&&self._isColAbs==cr._isColAbs}
    // rust: Object boundary performs the Java instanceof check before the typed method above.
    pub fn equals__object(&self,o:Option<&dyn std::any::Any>)->bool{self.equals(o.and_then(|o|o.downcast_ref::<Self>()))}

}
// java public static enum NameType{CELL,NAMED_RANGE,COLUMN,ROW,BAD_CELL_OR_NAMED_RANGE;}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum NameType{CELL,NAMED_RANGE,COLUMN,ROW,BAD_CELL_OR_NAMED_RANGE}
#[cfg(test)]
#[path="CellReference.review.rs"]
mod review;
