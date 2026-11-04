use crate::cn::wps::moss::filefmt::biff8::record::common::{UnicodeString::UnicodeString,FormatRun::FormatRun};use kfc_kfc_alpha::kfc::xpxml::util::EscapeUtil::EscapeUtil;
// java class RichTextBuilder
#[derive(Default)]pub struct RichTextBuilder{textList:Vec<(String,i32)>}
impl RichTextBuilder{
// java public void add(String text, int fontId) {
// java if (text != null && text.length() > 0) {
// java this.textList.add(new RichText(text, fontId));
// java }
// java
// java }
pub fn add(&mut self,text:Option<String>,fontId:i32){if let Some(text)=text{if !text.is_empty(){self.textList.push((text,fontId));}}}
// java public UnicodeString build() {
// java StringBuilder strValue = new StringBuilder();
// java
// java for(RichText item : this.textList) {
// java strValue.append(item.text);
// java }
// java
// java int idx = 0;
// java ArrayList<FormatRun> runs = new ArrayList();
// java
// java for(RichText item : this.textList) {
// java if (item.fontId != -1) {
// java runs.add(new FormatRun((short)idx, (short)item.fontId));
// java } else if (runs.size() > 0) {
// java runs.add(new FormatRun((short)idx, (short)0));
// java }
// java
// java idx += item.text.length();
// java }
// java
// java String newValue = EscapeUtil.unEscapeControlCharacter(strValue.toString());
// java UnicodeString unicodeStr = new UnicodeString(newValue);
// java unicodeStr.resetFormatRuns(runs);
// java return unicodeStr;
// java }
pub fn build(&self)->UnicodeString{let mut text=String::new();for(item,_)in &self.textList{text.push_str(item);}let mut idx=0i32;let mut runs=vec![];for(item,fontId)in &self.textList{if *fontId!= -1{runs.push(FormatRun::new(idx as i16,*fontId as i16));}else if !runs.is_empty(){runs.push(FormatRun::new(idx as i16,0));}idx=idx.wrapping_add(item.encode_utf16().count()as i32);}let mut unicode=UnicodeString::new(EscapeUtil::un_escape_control_character(&text));unicode.resetFormatRuns(runs);unicode}
// java public boolean empty() {
// java return this.textList.size() == 0;
// java }
pub fn empty(&self)->bool{self.textList.is_empty()}
// java public void Clear() {
// java this.textList.clear();
// java }
pub fn Clear(&mut self){self.textList.clear();}
}
