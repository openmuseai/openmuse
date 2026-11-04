use super::FormatRun::FormatRun;
// java public class UnicodeString implements Comparable<UnicodeString>, Cloneable
#[derive(Clone,Debug)]pub struct UnicodeString{pub text:String,pub format_run_count:i32,pub parsed:bool,pub runs:Vec<FormatRun>,charCount:i16,optionFlags:i8}
impl UnicodeString{
// java public UnicodeString(String str) {
// java this.setString(str);
// java }
pub fn new(value:String)->Self{let mut s=Self{text:String::new(),format_run_count:0,parsed:true,runs:vec![],charCount:0,optionFlags:0};s.setString(value);s}
// java public void setString(String string) {
// java this.field_3_string = string;
// java this.setCharCount((short)this.field_3_string.length());
// java boolean useUTF16 = false;
// java int strlen = string.length();
// java
// java for(int j = 0; j < strlen; ++j) {
// java if (string.charAt(j) > 255) {
// java useUTF16 = true;
// java break;
// java }
// java }
// java
// java if (useUTF16) {
// java this.field_2_optionflags = highByte.setByte(this.field_2_optionflags);
// java } else {
// java this.field_2_optionflags = highByte.clearByte(this.field_2_optionflags);
// java }
// java
// java }
pub fn setString(&mut self,value:String){self.text=value;self.charCount=self.text.encode_utf16().count()as i16;if self.text.encode_utf16().any(|c|c>255){self.optionFlags|=1;}else{self.optionFlags&=!1;}}
// java public String getString() {
// java if (!this.isStrParsed()) {
// java this.parsedBytesToStr();
// java }
// java
// java return this.field_3_string;
// java }
pub fn getString(&self)->&str{assert!(self.parsed,"UnicodeString.parsedBytesToStr outside XLSX parsed-string closure");&self.text}
// java public int getFormatRunCount() {
// java return this.field_4_format_runs == null ? 0 : this.field_4_format_runs.size();
// java }
pub fn getFormatRunCount(&self)->i32{self.runs.len()as i32}
// java public List<FormatRun> getFormatRuns() {
// java return this.field_4_format_runs;
// java }
pub fn getFormatRuns(&self)->&[FormatRun]{&self.runs}
// java public FormatRun getFormatRun(int index) {
// java if (this.field_4_format_runs == null) {
// java return null;
// java } else {
// java return index >= 0 && index < this.field_4_format_runs.size() ? (FormatRun)this.field_4_format_runs.get(index) : null;
// java }
// java }
pub fn getFormatRun(&self,i:i32)->Option<&FormatRun>{if i<0{None}else{self.runs.get(i as usize)}}
// java public void resetFormatRuns(List<FormatRun> runs) {
// java this.field_4_format_runs = runs;
// java this.field_2_optionflags = richText.setByte(this.field_2_optionflags);
// java }
pub fn resetFormatRuns(&mut self,runs:Vec<FormatRun>){self.runs=runs;self.format_run_count=self.runs.len()as i32;self.optionFlags|=8;}
}
