// Java field: Pattern utfPtrn
#[allow(non_upper_case_globals)]
pub(crate) static utfPtrn: &str = "_x([0-9A-Fa]{4})_";
// Java field: String SUFFIX
pub const SUFFIX: &str = "_x";
// Java field: String[] CONTROL_CHARS
pub static CONTROL_CHARS: &[Option<&str>] = &[
    None, Some("_x0001_"), Some("_x0002_"), Some("_x0003_"), Some("_x0004_"),
    Some("_x0005_"), Some("_x0006_"), Some("_x0007_"), Some("_x0008_"), None,
    None, Some("_x000B_"), Some("_x000C_"), None, Some("_x000E_"), Some("_x000F_"),
    Some("_x0010_"), Some("_x0011_"), Some("_x0012_"), Some("_x0013_"), Some("_x0014_"),
    Some("_x0015_"), Some("_x0016_"), Some("_x0017_"), Some("_x0018_"), Some("_x0019_"),
    Some("_x001A_"), Some("_x001B_"), Some("_x001C_"), Some("_x001D_"), Some("_x001E_"),
    Some("_x001F_"),
];
// Java field: String[] CONTROL_CHARS_V2
pub static CONTROL_CHARS_V2: &[Option<&str>] = &[
    None, Some("_x0001_"), Some("_x0002_"), Some("_x0003_"), Some("_x0004_"),
    Some("_x0005_"), Some("_x0006_"), Some("_x0007_"), Some("_x0008_"), None,
    None, Some("_x000B_"), Some("_x000C_"), Some("_x000D_"), Some("_x000E_"),
    Some("_x000F_"), Some("_x0010_"), Some("_x0011_"), Some("_x0012_"), Some("_x0013_"),
    Some("_x0014_"), Some("_x0015_"), Some("_x0016_"), Some("_x0017_"), Some("_x0018_"),
    Some("_x0019_"), Some("_x001A_"), Some("_x001B_"), Some("_x001C_"), Some("_x001D_"),
    Some("_x001E_"), Some("_x001F_"),
];

fn is_hex_char(c: u8) -> bool {
    matches!(c, b'0'..=b'9' | b'A'..=b'F' | b'a')
}

fn find_utf_pattern(value: &str, start: usize) -> Option<(usize, usize, &str)> {
    let bytes = value.as_bytes();
    let mut i = start;
    while i + 6 < bytes.len() {
        if bytes[i] == b'_' && bytes[i + 1] == b'x'
            && is_hex_char(bytes[i + 2])
            && is_hex_char(bytes[i + 3])
            && is_hex_char(bytes[i + 4])
            && is_hex_char(bytes[i + 5])
            && bytes[i + 6] == b'_'
        {
            return Some((i, i + 7, &value[i + 2..i + 6]));
        }
        i += 1;
    }
    None
}

// Java: public class EscapeUtil
pub struct EscapeUtil;

// Native String.replaceAll of the original literal single-code-unit patterns.
fn replaceUnit(value:&[u16],unit:u16,replacement:&str)->Vec<u16>{let mut out=vec![];for &v in value{if v==unit{out.extend(replacement.encode_utf16());}else{out.push(v);}}out}

// Java: public class EscapeUtil
impl EscapeUtil {
    // Java String code-unit transport of escapeCharacter, preserving its guard
    // (U+001E alone does not trigger needEscapeCharacter) and replacement order.
    pub fn escapeCharacterUtf16(value:&[u16])->Vec<u16>{
        // java if (!needEscapeCharacter(str)) { return str; }
        if !value.iter().any(|v|matches!(*v,38|60|62|39|34|0)){return value.to_vec();}
        // java String value = str;
        let mut value=value.to_vec();
        // java if (str.indexOf(38) != -1) { value = str.replaceAll("&", "&amp;"); }
        if value.contains(&38){value=replaceUnit(&value,38,"&amp;");}
        // java if (value.indexOf(60) != -1) { value = value.replaceAll("<", "&lt;"); }
        if value.contains(&60){value=replaceUnit(&value,60,"&lt;");}
        // java if (value.indexOf(62) != -1) { value = value.replaceAll(">", "&gt;"); }
        if value.contains(&62){value=replaceUnit(&value,62,"&gt;");}
        // java if (value.indexOf(39) != -1) { value = value.replaceAll("'", "&apos;"); }
        if value.contains(&39){value=replaceUnit(&value,39,"&apos;");}
        // java if (value.indexOf(34) != -1) { value = value.replaceAll("\"", "&quot;"); }
        if value.contains(&34){value=replaceUnit(&value,34,"&quot;");}
        // java if (value.indexOf(0) != -1) { value = value.replaceAll("\u0000", "_x0000_"); }
        if value.contains(&0){value=replaceUnit(&value,0,"_x0000_");}
        // java if (value.indexOf(30) != -1) { value = value.replaceAll("\u001e", ""); }
        if value.contains(&30){value=replaceUnit(&value,30,"");}
        // java return value;
        value
    }
    // Native lossless String code-unit transport of the original control escape.
    pub fn escapeControlCharacterUtf16(value:&[u16],controlChars:&[Option<&str>])->Vec<u16>{
        // java boolean needEscape = false; ... if (ch < controlChars.length && controlChars[ch] != null) { needEscape = true; break; }
        let mut needEscape=false;for &ch in value{if (ch as usize)<controlChars.len()&&controlChars[ch as usize].is_some(){needEscape=true;break;}}
        // java if (!needEscape) { return value; }
        if !needEscape{return value.to_vec();}
        // java StringBuilder ret = new StringBuilder(value.length());
        let mut ret=Vec::with_capacity(value.len());
        // java for (...) { int ch = value.charAt(i) & '\uffff'; if (ch < controlChars.length && controlChars[ch] != null) { ret.append(controlChars[ch]); } else { ret.append((char)ch); } }
        for &ch in value{if (ch as usize)<controlChars.len(){if let Some(replacement)=controlChars[ch as usize]{ret.extend(replacement.encode_utf16());continue;}}ret.push(ch);}
        // java return ret.toString();
        ret
    }
    // Java: EscapeUtil()
    pub fn new() -> Self {
        // Java: super();
        EscapeUtil
    }

    // Java: static boolean needEscapeCharacter(String str)
    pub fn need_escape_character(str: &str) -> bool {
        // Java: if (str != null && str.length() != 0)
        // LLM Rust Null-Safety: str is always non-null reference
        if true && str.len() != 0 {
            // Java: int i = 0;
            let mut i = 0;
            // Java: for(int size = str.length(); i < size; ++i)
            for ch in str.chars() {
                // Java: char ch = str.charAt(i);
                let ch = ch;
                // Java: if (ch == '&' || ch == '<' || ch == '>' || ch == '\'' || ch == '"' || ch == 0)
                if ch == '&' || ch == '<' || ch == '>' || ch == '\'' || ch == '"' || ch == '\0' {
                    // Java: return true;
                    return true;
                }
                i += 1;
            }
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: return false;
            return false;
        }
    }

    // Java: static String escapeCharacter(String str)
    pub fn escape_character(str: &str) -> String {
        // Java: if (!needEscapeCharacter(str))
        if !Self::need_escape_character(str) {
            // Java: return str;
            return str.to_string();
        }
        // Java: else
        else {
        // Java: String value = str;
        let mut value = str.to_string();
        // Java: if (str.indexOf(38) != -1)
        if value.contains('&') {
            // Java: value = str.replaceAll("&", "&amp;");
            value = value.replace("&", "&amp;");
        }
        // Java: if (value.indexOf(60) != -1)
        if value.contains('<') {
            // Java: value = value.replaceAll("<", "&lt;");
            value = value.replace("<", "&lt;");
        }
        // Java: if (value.indexOf(62) != -1)
        if value.contains('>') {
            // Java: value = value.replaceAll(">", "&gt;");
            value = value.replace(">", "&gt;");
        }
        // Java: if (value.indexOf(39) != -1)
        if value.contains('\'') {
            // Java: value = value.replaceAll("'", "&apos;");
            value = value.replace("'", "&apos;");
        }
        // Java: if (value.indexOf(34) != -1)
        if value.contains('"') {
            // Java: value = value.replaceAll("\"", "&quot;");
            value = value.replace("\"", "&quot;");
        }
        // Java: if (value.indexOf(0) != -1)
        if value.contains('\0') {
            // Java: value = value.replaceAll("\u0000", "_x0000_");
            value = value.replace("\u{0000}", "_x0000_");
        }
        // Java: if (value.indexOf(30) != -1)
        if value.contains('\u{001e}') {
            // Java: value = value.replaceAll("\u001e", "");
            value = value.replace("\u{001e}", "");
        }
        // Java: return value;
        value
        }
    }

    // Java: static String unEscapeControlCharacter(String value)
    pub fn un_escape_control_character(value: &str) -> String {
        // Java: if (value != null && value.length() >= 7)
        // LLM Rust Null-Safety: value is always non-null reference
        if true && value.len() >= 7 {
            // Java: if (value.indexOf("_x") == -1)
            if !value.contains("_x") {
                // Java: return value;
                return value.to_string();
            }
            // Java: else
            else {
            // Java: StringBuffer buf = new StringBuffer();
            let mut buf = String::new();
            // Java: Matcher m = utfPtrn.matcher(value);
            // LLM Java Platform API: Pattern/Matcher replaced by find_utf_pattern helper
            let m = true;
            // Java: int idx;
            let mut idx = 0;
            // Java: for(idx = 0; m.find(); idx = m.end())
            while let Some((pos, end, code)) = find_utf_pattern(value, idx) {
                // Java: int pos = m.start();
                let pos = pos;
                // Java: if (pos > idx)
                if pos > idx {
                    // Java: buf.append(value.substring(idx, pos));
                    buf.push_str(&value[idx..pos]);
                }
                // Java: String code = m.group(1);
                let code = code;
                // Java: int icode = Integer.decode("0x" + code);
                let icode = u32::from_str_radix(code, 16).unwrap_or(0);
                // Java: buf.append((char)icode);
                if let Some(c) = char::from_u32(icode) { buf.push(c); }
                else { return String::from_utf16(&Self::unEscapeControlCharacter(&value.encode_utf16().collect::<Vec<_>>())).expect("unpaired Java UTF-16 outside String facade"); }
                idx = end;
            }
            // Java: buf.append(value.substring(idx));
            buf.push_str(&value[idx..]);
            // Java: return buf.toString();
            return buf;
            }
        }
        // Java: else
        else {
        // Java: return value;
        return value.to_string();
        }
    }

    // Java: static String escapeControlCharacter(String value, String[] controlChars)
    pub fn escape_control_character(value: &str, control_chars: &[Option<&str>]) -> String {
        // Java: boolean needEscape = false;
        let mut needEscape = false;
        // Java: int i = 0;
        let mut i = 0;
        // Java: for(int count = value.length(); i < count; ++i)
        for c in value.chars() {
            // Java: int ch = value.charAt(i) & '\uffff';
            let ch = c as usize;
            // Java: if (ch < controlChars.length && controlChars[ch] != null)
            if ch < control_chars.len() && control_chars[ch].is_some() {
                // Java: needEscape = true;
                needEscape = true;
                // Java: break;
                break;
            }
            i += 1;
        }
        // Java: if (!needEscape)
        if !needEscape {
            // Java: return value;
            return value.to_string();
        }
        // Java: else
        else {
        // Java: StringBuilder ret = new StringBuilder(value.length());
        let mut ret = String::with_capacity(value.len());
        // Java: i = 0;
        i = 0;
        // Java: for(int count = value.length(); i < count; ++i)
        for c in value.chars() {
            // Java: int ch = value.charAt(i) & '\uffff';
            let ch = c as usize;
            // Java: if (ch < controlChars.length && controlChars[ch] != null)
            if ch < control_chars.len() && control_chars[ch].is_some() {
                // Java: ret.append(controlChars[ch]);
                ret.push_str(control_chars[ch].unwrap());
            }
            // Java: else
            else {
                // Java: ret.append((char)ch);
                ret.push(c);
            }
        }
        // Java: return ret.toString();
        ret
        }
    }
}

impl EscapeUtil {
    // java public static String unEscapeControlCharacter(String value) {
    // rust: UTF-16 overload retains Java char casts, including surrogate halves.
    pub fn unEscapeControlCharacter(value:&[u16])->Vec<u16>{
        // java if (value != null && value.length() >= 7) {
        if value.len()>=7{
            // java if (value.indexOf("_x") == -1) { return value; }
            if !value.windows(2).any(|p|p==[95,120]){return value.to_vec();}
            // java StringBuffer buf = new StringBuffer(); Matcher m = utfPtrn.matcher(value);
            let mut buf=vec![];let mut search=0;
            // java int idx; for(idx = 0; m.find(); idx = m.end()) {
            let mut idx=0;
            while search+7<=value.len(){
                // rust: native Pattern.find for the original _x([0-9A-Fa]{4})_.
                let p=&value[search..search+7];
                if p[0]!=95||p[1]!=120||p[6]!=95||!p[2..6].iter().all(|c|*c<=127&&is_hex_char(*c as u8)){search+=1;continue;}
                // java int pos = m.start(); if (pos > idx) { buf.append(value.substring(idx, pos)); }
                let pos=search;if pos>idx{buf.extend_from_slice(&value[idx..pos]);}
                // java String code = m.group(1); int icode = Integer.decode("0x" + code); buf.append((char)icode);
                let code:String=p[2..6].iter().map(|c|char::from_u32(*c as u32).unwrap()).collect();
                let icode=u16::from_str_radix(&code,16).unwrap();buf.push(icode);
                // java idx = m.end();
                search+=7;idx=search;
            }
            // java buf.append(value.substring(idx)); return buf.toString();
            buf.extend_from_slice(&value[idx..]);buf
        // java } else { return value; }
        }else{value.to_vec()}
    }
    // java public static boolean needEscapeCharacter(String str) {
    pub fn needEscapeCharacter(value:Option<&str>)->bool{value.is_some_and(Self::need_escape_character)}
    // java public static String escapeCharacter(String str) {
    pub fn escapeCharacter(value:Option<&str>)->Option<String>{value.map(Self::escape_character)}
    // java public static String escapeControlCharacter(String value, String[] controlChars) {
    pub fn escapeControlCharacter(value:&str,chars:&[Option<&str>])->String{Self::escape_control_character(value,chars)}
}
