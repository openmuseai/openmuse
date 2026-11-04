// java package cn.wps.et.ss.formula;
use crate::{java_runtime::Character,org::apache::poi::ss::{SpreadsheetVersion::CUR_VERSION,util::CellReference::CellReference}};
use super::parser::FormulaParser;
use std::sync::LazyLock;
// java public final class SheetNameFormatter {
pub struct SheetNameFormatter;
// java private static final char DELIMITER = '\'';
const DELIMITER:char='\'';
// java private static final Pattern CELL_REF_PATTERN = Pattern.compile("([A-Za-z]+)([0-9]+)");
static CELL_REF_PATTERN:LazyLock<regex::Regex>=LazyLock::new(||regex::Regex::new(r"\A([A-Za-z]+)([0-9]+)\z").unwrap());
impl SheetNameFormatter{
    // java private SheetNameFormatter() {}
    fn SheetNameFormatter()->Self{Self}
    // java public static void appendFormat(StringBuilder out, String rawSheetName) {
    pub fn appendFormat(out:&mut String,rawSheetName:Option<&str>){
        // java if (rawSheetName == null) { rawSheetName = "#REF"; }
        let rawSheetName=rawSheetName.unwrap_or("#REF");
        // java boolean needsQuotes = needsDelimiting(rawSheetName);
        let needsQuotes=Self::needsDelimiting(Some(rawSheetName));
        // java if (needsQuotes) { out.append('\''); appendAndEscape(out, rawSheetName); out.append('\''); }
        if needsQuotes{out.push(DELIMITER);Self::appendAndEscape(out,rawSheetName);out.push(DELIMITER);}
        // java else { out.append(rawSheetName); }
        else{out.push_str(rawSheetName);}
    }
    // java public static void appendFormat(StringBuilder out, String workbookName, String rawSheetName) {
    pub fn appendFormat__workbook(out:&mut String,workbookName:Option<&str>,rawSheetName:Option<&str>){
        // java if (rawSheetName == null) { rawSheetName = "#REF"; }
        let rawSheetName=rawSheetName.unwrap_or("#REF");
        // java boolean needsQuotes = needsDelimiting(workbookName) || needsDelimiting(rawSheetName);
        let needsQuotes=Self::needsDelimiting(workbookName)||Self::needsDelimiting(Some(rawSheetName));
        // java int sperator = workbookName.lastIndexOf(47); String dirPath; String fileName;
        let workbookName=workbookName.expect("NullPointerException");let sperator=workbookName.rfind('/');let(dirPath,fileName);
        // java if (sperator >= 0) { dirPath = workbookName.substring(0, sperator + 1); fileName = workbookName.substring(sperator + 1); }
        if let Some(sperator)=sperator{dirPath=&workbookName[..sperator+1];fileName=&workbookName[sperator+1..];}
        // java else { dirPath = ""; fileName = workbookName; }
        else{dirPath="";fileName=workbookName;}
        // java if (needsQuotes) {
        if needsQuotes{
            // java out.append('\''); appendAndEscape(out, dirPath.replace('[', '(').replace(']', ')'));
            out.push(DELIMITER);Self::appendAndEscape(out,&dirPath.replace('[',"(").replace(']',")"));
            // java out.append('['); appendAndEscape(out, fileName.replace('[', '(').replace(']', ')'));
            out.push('[');Self::appendAndEscape(out,&fileName.replace('[',"(").replace(']',")"));
            // java out.append(']'); appendAndEscape(out, rawSheetName); out.append('\'');
            out.push(']');Self::appendAndEscape(out,rawSheetName);out.push(DELIMITER);
        }else{
            // java out.append(dirPath); out.append('['); out.append(fileName); out.append(']'); out.append(rawSheetName);
            out.push_str(dirPath);out.push('[');out.push_str(fileName);out.push(']');out.push_str(rawSheetName);
        }
    }
    // java private static void appendAndEscape(StringBuilder sb, String rawSheetName) {
    fn appendAndEscape(sb:&mut String,rawSheetName:&str){
        // java int len = rawSheetName.length(); for (int i = 0; i < len; ++i) { char ch = rawSheetName.charAt(i);
        // rust: valid native String scalars preserve the same UTF16 output units; apostrophes are BMP.
        for ch in rawSheetName.chars(){
            // java if (ch == '\'') { sb.append('\''); } sb.append(ch);
            if ch==DELIMITER{sb.push(DELIMITER);}sb.push(ch);
        }
    }
    // java public static boolean needsDelimiting(String rawSheetName) {
    pub fn needsDelimiting(rawSheetName:Option<&str>)->bool{
        // java if (rawSheetName == null) { return false; }
        let Some(rawSheetName)=rawSheetName else{return false;};
        // java int len = rawSheetName.length(); if (len < 1) { throw new RuntimeException("Zero length string is an invalid sheet name"); }
        let chars:Vec<_>=rawSheetName.encode_utf16().collect();let len=chars.len();assert!(len>=1,"RuntimeException: Zero length string is an invalid sheet name");
        // java char ch = rawSheetName.charAt(0); if (!Character.isDigit(ch) && ch != '.') {
        let mut ch=chars[0];if !Character::isDigit(ch)&&ch!=46{
            // java if (len <= 1 || !Character.isDigit(rawSheetName.charAt(1)) || ch != 'r' && ch != 'R' && ch != 'c' && ch != 'C') {
            if len<=1||!Character::isDigit(chars[1])||!matches!(ch,114|82|99|67){
                // java for (int i = 0; i < len; ++i) { ch = rawSheetName.charAt(i);
                for i in 0..len{ch=chars[i];
                    // java if (ch != '[' && ch != ']' && ch != ':' && isSpecialChar(ch)) { return true; }
                    if !matches!(ch,91|93|58)&&Self::isSpecialChar(ch){return true;}
                }
                // java if (Character.isLetter(rawSheetName.charAt(0)) && Character.isDigit(rawSheetName.charAt(len - 1)) && nameLooksLikePlainCellReference(rawSheetName)) { return true; }
                if Character::isLetter(chars[0])&&Character::isDigit(chars[len-1])&&Self::nameLooksLikePlainCellReference(rawSheetName){true}
                // java else if (nameLooksLikeBooleanLiteral(rawSheetName)) { return true; } else { return false; }
                else if Self::nameLooksLikeBooleanLiteral(rawSheetName){true}else{false}
            // java else { return true; }
            }else{true}
        // java else { return true; }
        }else{true}
    }
    // java private static boolean nameLooksLikeBooleanLiteral(String rawSheetName) {
    fn nameLooksLikeBooleanLiteral(rawSheetName:&str)->bool{
        // java switch (rawSheetName.charAt(0)) { case 'F': case 'f': return "FALSE".equalsIgnoreCase(rawSheetName); case 'T': case 't': return "TRUE".equalsIgnoreCase(rawSheetName); default: return false; }
        let chars:Vec<_>=rawSheetName.encode_utf16().collect();match chars[0]{70|102=>Character::equalsIgnoreCaseAscii(&chars,b"FALSE"),84|116=>Character::equalsIgnoreCaseAscii(&chars,b"TRUE"),_=>false}
    }
    // java static boolean isSpecialChar(char ch) {
    pub(crate) fn isSpecialChar(mut ch:u16)->bool{
        // java if (!Character.isLetterOrDigit(ch) && ch != '.' && ch != '_') {
        if !Character::isLetter(ch)&&!Character::isDigit(ch)&&!matches!(ch,46|95){
            // java if (ch != '\n' && ch != '\r' && ch != '\t') {
            if !matches!(ch,10|13|9){
                // java if ((ch & '\uff00') == 0) { return true; }
                if ch&0xff00==0{return true;}
                // java ch = FormulaParser.toStdHalfHorn(ch);
                ch=FormulaParser::toStdHalfHorn(ch);
                // java if (!Character.isLetterOrDigit(ch) && ch != '.' && ch != '_') { return (ch & '\uff00') == 0; } else { return false; }
                if !Character::isLetter(ch)&&!Character::isDigit(ch)&&!matches!(ch,46|95){ch&0xff00==0}else{false}
            // java else { throw new RuntimeException("Illegal character (0x" + Integer.toHexString(ch) + ") found in sheet name"); }
            }else{panic!("RuntimeException: Illegal character (0x{ch:x}) found in sheet name");}
        // java else { return false; }
        }else{false}
    }
    // java static boolean cellReferenceIsWithinRange(String lettersPrefix, String numbersSuffix) {
    pub(crate) fn cellReferenceIsWithinRange(lettersPrefix:&str,numbersSuffix:&str)->bool{
        // java return CellReference.cellReferenceIsWithinRange(lettersPrefix, numbersSuffix, SpreadsheetVersion.CUR_VERSION);
        let version=*CUR_VERSION.lock().unwrap();CellReference::cellReferenceIsWithinRange(lettersPrefix,numbersSuffix,version)
    }
    // java static boolean nameLooksLikePlainCellReference(String rawSheetName) {
    pub(crate) fn nameLooksLikePlainCellReference(rawSheetName:&str)->bool{
        // java Matcher matcher = CELL_REF_PATTERN.matcher(rawSheetName); if (!matcher.matches()) { return false; }
        let Some(matcher)=CELL_REF_PATTERN.captures(rawSheetName)else{return false;};
        // java String lettersPrefix = matcher.group(1); String numbersSuffix = matcher.group(2); return cellReferenceIsWithinRange(lettersPrefix, numbersSuffix);
        Self::cellReferenceIsWithinRange(&matcher[1],&matcher[2])
    }
}
