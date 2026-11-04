use crate::{java_runtime::{parseDouble,Rc},cn::wps::et::ss::{calcchain::RecalcLock,constant::RecogniseResult::RecogniseResult,statemachine::RecogniseResultRl::Value}};
// java public class DigitsResResult {
pub struct DigitsResResult{
    // java private String content;
    content:Option<Vec<u16>>,
    // java public int flag;
    pub flag:i32,
    monitor:Rc<RecalcLock>,
}
impl DigitsResResult{
    // java public static final String NUMFMT_THAI = "[$-D00041E]0";
    pub const NUMFMT_THAI:&str="[$-D00041E]0";
    // java public static final String NUMFMT_ARABIC = "[$-2000401]0";
    pub const NUMFMT_ARABIC:&str="[$-2000401]0";
    // java implicit constructor: content=null; flag=0.
    pub fn new()->Self{Self{content:None,flag:0,monitor:Rc::new(RecalcLock::new())}}
    // java public synchronized void reset() {
    pub fn reset(&mut self){
        // rust: exclusive receiver plus the same reentrant monitor.
        let monitor=self.monitor.clone();let _m=monitor.lock();
        // java this.setContent((String)null); this.flag = 0;
        self.setContent(None);self.flag=0;
    }
    // java public synchronized void setContent(String content) { this.content = content; }
    pub fn setContent(&mut self,content:Option<&[u16]>){let _m=self.monitor.lock();self.content=content.map(<[u16]>::to_vec);}
    // java public synchronized boolean completeDiff(String str) {
    pub fn completeDiff(&self,s:Option<&[u16]>)->bool{
        let _m=self.monitor.lock();
        // java if (this.content != null && str != null) {
        if let(Some(content),Some(s))=(&self.content,s){
            // java if (this.content.length() != str.length()) { return false; }
            if content.len()!=s.len(){return false;}
            // java int len = this.content.length(); for(int i = 0; i < len; ++i) { if (this.content.charAt(i) == str.charAt(i)) { return false; } }
            let len=content.len();for i in 0..len{if content[i]==s[i]{return false;}}
            // java return true;
            true
        // java } else { return false; }
        }else{false}
    }
    // java public synchronized boolean diff(String str) {
    pub fn diff(&self,s:Option<&[u16]>)->bool{
        let _m=self.monitor.lock();
        // java if (this.content != null && str != null) {
        if let(Some(content),Some(s))=(&self.content,s){
            // java if (this.content.length() != str.length()) { return true; }
            if content.len()!=s.len(){return true;}
            // java int len = this.content.length(); for(int i = 0; i < len; ++i) { if (this.content.charAt(i) != str.charAt(i)) { return true; } }
            let len=content.len();for i in 0..len{if content[i]!=s[i]{return true;}}
            // java return false;
            false
        // java } else { return true; }
        }else{true}
    }
    // java public synchronized RecogniseResult getResult() {
    pub fn getResult(&self)->Option<RecogniseResult>{
        let _m=self.monitor.lock();
        // java if (this.content == null) { return null; }
        let content=self.content.as_ref()?;
        // java RecogniseResult ret = null; switch(this.flag) {
        let mut ret=None;match self.flag{
            // java case 2: case 3: ret = new RecogniseResult(Double.valueOf(this.content)); ret.setSuggestedNumfmt("[$-2000401]0"); break;
            2|3=>{let value=parseDouble(&String::from_utf16(content).expect("NumberFormatException")).expect("NumberFormatException");let mut r=RecogniseResult::fromValue(Value::Number(value));r.setSuggestedNumfmt(Some(Self::NUMFMT_ARABIC.into()));ret=Some(r);}
            // java case 4: case 5: ret = new RecogniseResult(Double.valueOf(this.content)); ret.setSuggestedNumfmt("[$-D00041E]0");
            4|5=>{let value=parseDouble(&String::from_utf16(content).expect("NumberFormatException")).expect("NumberFormatException");let mut r=RecogniseResult::fromValue(Value::Number(value));r.setSuggestedNumfmt(Some(Self::NUMFMT_THAI.into()));ret=Some(r);}
            _=>{},
        }
        // java return ret;
        ret
    }
    // java public static boolean isLocalNumberFmt(String numfmt) {
    pub fn isLocalNumberFmt(numfmt:Option<&str>)->bool{
        // java boolean flag = false; if ("[$-D00041E]0".equals(numfmt) || "[$-2000401]0".equals(numfmt)) { flag = true; } return flag;
        let mut flag=false;if numfmt==Some(Self::NUMFMT_THAI)||numfmt==Some(Self::NUMFMT_ARABIC){flag=true;}flag
    }
}
