use crate::cn::wps::et::ss::formula::{evaluator::{EvalResult,EvalFailure}};
pub struct LookupUtils;impl LookupUtils{
// java public static Pattern wildCardToPattern(String s, boolean caseInsensitive) {
// java int slen = s.length();
// java StringBuilder sb = new StringBuilder(slen * 3 / 2);
// java boolean hasWildCard = false;
// java
// java for(int i = 0; i < slen; ++i) {
// java char c = s.charAt(i);
// java if (c == '*') {
// java sb.append(".*");
// java hasWildCard = true;
// java } else if (c == '?') {
// java sb.append(".");
// java hasWildCard = true;
// java } else if (c != '~' && c != '～') {
// java if ((c & '\uff00') == 0) {
// java if ((c < 'a' || c > 'z') && (c < 'A' || c > 'Z') && (c < '0' || c > '9')) {
// java sb.append('\\').append(c);
// java } else {
// java sb.append(c);
// java }
// java } else {
// java sb.append(c);
// java }
// java } else if (i + 1 < slen) {
// java if (s.charAt(i + 1) == '*' || s.charAt(i + 1) == '?' || s.charAt(i + 1) == '~') {
// java sb.append("\\");
// java }
// java
// java ++i;
// java sb.append(s.charAt(i));
// java hasWildCard = true;
// java } else if (i + 1 >= slen) {
// java hasWildCard = true;
// java break;
// java }
// java }
// java
// java if (hasWildCard) {
// java return caseInsensitive ? Pattern.compile(sb.toString(), 34) : Pattern.compile(sb.toString(), 32);
// java } else {
// java return null;
// java }
// java }
pub fn wildCardToPattern(s:&[u16],caseInsensitive:bool)->EvalResult<Option<regex::Regex>> {
let mut pattern=String::new();let mut wild=false;let mut i=0;let dot="[^\\n\\r\\x{0085}\\x{2028}\\x{2029}]";while i<s.len(){let c=s[i];if c==42{pattern.push_str(dot);pattern.push('*');wild=true;}else if c==63{pattern.push_str(dot);wild=true;}else if c==126||c==0xff5e{wild=true;if i+1>=s.len(){break;}i+=1;let unit=s[i];let text=String::from_utf16(&[unit]).map_err(|_|EvalFailure::Unported("UTF16 regex unpaired surrogate"))?;if matches!(unit,42|63|126){pattern.push_str(&regex::escape(&text));}else{pattern.push_str(&text);}}else{let start=i;if (0xd800..=0xdbff).contains(&c)&&s.get(i+1).is_some_and(|v|(0xdc00..=0xdfff).contains(v)){i+=1;}let text=String::from_utf16(&s[start..=i]).map_err(|_|EvalFailure::Unported("UTF16 regex unpaired surrogate"))?;pattern.push_str(&regex::escape(&text));}i+=1;}if !wild{return Ok(None);}regex::RegexBuilder::new(&pattern).case_insensitive(caseInsensitive).unicode(true).build().map(Some).map_err(|_|EvalFailure::Runtime("PatternSyntaxException"))
}
}
