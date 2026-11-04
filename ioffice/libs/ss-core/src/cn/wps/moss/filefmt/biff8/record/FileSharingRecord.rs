use crate::java_runtime::{Cell,RefCell,Rc};
use io_poi_compact::cn::wps::encrypt::ModifyVerifier::ModifyVerifier;
use util::org::apache::poi::util::{LittleEndianOutput::LittleEndianOutput,StringUtil::StringUtil};
use super::StandardRecord::StandardRecord;
// java public static final short sid = 91;
pub const sid:i16=91;
// java public final class FileSharingRecord extends StandardRecord
pub struct FileSharingRecord {
    // java private short field_1_readonly;
    field_1_readonly:Cell<i16>,
    // java private short field_2_password;
    field_2_password:Cell<i16>,
    // java private ModifyVerifier field_2_password_modify_verifier;
    field_2_password_modify_verifier:RefCell<Option<Rc<RefCell<ModifyVerifier>>>>,
    // java private byte field_3_username_unicode_options;
    field_3_username_unicode_options:Cell<i8>,
    // java private String field_3_username_value;
    field_3_username_value:RefCell<Option<String>>,
}
impl FileSharingRecord {
    // java public FileSharingRecord() { }
    pub fn new()->Self{Self{field_1_readonly:Cell::new(0),field_2_password:Cell::new(0),field_2_password_modify_verifier:RefCell::new(None),field_3_username_unicode_options:Cell::new(0),field_3_username_value:RefCell::new(None)}}
    // rust: The two RecordInputStream constructors await the real BIFF reader graph.
    // java public void setValue(String value) {
    pub fn setValue(&self,value:Option<&str>){
        // java if (value.getBytes(StandardCharsets.UTF_8).length == value.length()) {
        let value=value.expect("NullPointerException");
        if value.as_bytes().len()==value.encode_utf16().count(){
            // java this.field_3_username_unicode_options = 0;
            self.field_3_username_unicode_options.set(0);
        }else{
            // java this.field_3_username_unicode_options = 1;
            self.field_3_username_unicode_options.set(1);
        }
        // java this.field_3_username_value = value;
        *self.field_3_username_value.borrow_mut()=Some(value.to_owned());
    }
    // java public static short hashPassword(String password) {
    pub fn hashPassword(password:Option<&str>)->i16{
        // java byte[] passwordCharacters = password.getBytes(StandardCharsets.UTF_8);
        let password=password.expect("NullPointerException");let passwordCharacters=password.as_bytes();
        // java if (password.length() != passwordCharacters.length) { return hashPassword_Chinaese(password); }
        if password.encode_utf16().count()!=passwordCharacters.len(){Self::hashPassword_Chinaese(Some(password))}else{
            // java int hash = 0;
            let mut hash=0i32;
            // java if (passwordCharacters.length > 0) {
            if !passwordCharacters.is_empty(){
                // java for(int charIndex = passwordCharacters.length; charIndex-- > 0; hash ^= passwordCharacters[charIndex]) {
                for charIndex in (0..passwordCharacters.len()).rev(){
                    // java hash = hash >> 14 & 1 | hash << 1 & 32767;
                    hash=(hash>>14&1)|(hash.wrapping_shl(1)&32767);
                    // java hash ^= passwordCharacters[charIndex];
                    hash^=passwordCharacters[charIndex]as i8 as i32;
                }
                // java hash = hash >> 14 & 1 | hash << 1 & 32767;
                hash=(hash>>14&1)|(hash.wrapping_shl(1)&32767);
                // java hash ^= passwordCharacters.length; hash ^= 52811;
                hash^=passwordCharacters.len()as i32;hash^=52811;
            }
            // java return (short)hash;
            hash as i16
        }
    }
    // java public static short hashPassword_Chinaese(String password) {
    pub fn hashPassword_Chinaese(password:Option<&str>)->i16{
        // java if (password != null && password.length() != 0) {
        if let Some(password)=password.filter(|s|!s.is_empty()){
            // java byte[] passwordCharacters; try { passwordCharacters = password.getBytes("gb2312"); }
            // rust: Android Charset transport; the supported charset cannot throw UnsupportedEncodingException.
            let passwordCharacters=util::java_runtime::Charset::gb2312Bytes(password);
            // java short nHash = (short)passwordCharacters.length;
            let mut nHash=passwordCharacters.len()as i16;
            // java nHash = (short)(nHash ^ '칋');
            nHash=((nHash as i32)^0xce4b)as i16;
            // java for(int nIndex = 0; nIndex < passwordCharacters.length; ++nIndex) {
            for nIndex in 0..passwordCharacters.len(){
                // java short b = (short)passwordCharacters[nIndex];
                let mut b=passwordCharacters[nIndex]as i8 as i16;
                // java if (b < 0) { b = (short)(256 + b); }
                if b<0{b=(256i32+b as i32)as i16;}
                // java b = lclRotateLeft(b, nIndex + 1, 15);
                b=Self::lclRotateLeft(b,(nIndex as i32).wrapping_add(1),15);
                // java nHash ^= b;
                nHash^=b;
            }
            // java return nHash;
            nHash
        }else{
            // java return 0;
            0
        }
    }
    // java private static short lclRotateLeft(short b, int nBits, int nWidth) {
    fn lclRotateLeft(b:i16,nBits:i32,nWidth:i32)->i16{
        // java short nMash = (short)((1 << nWidth) - 1);
        let nMash=1i32.wrapping_shl(nWidth as u32).wrapping_sub(1)as i16;
        // java return (short)(b << nBits | (b & nMash) >> nWidth - nBits);
        ((b as i32).wrapping_shl(nBits as u32)|((b&nMash)as i32).wrapping_shr(nWidth.wrapping_sub(nBits)as u32))as i16
    }
    // java public void setReadOnly(short readonly) { this.field_1_readonly = readonly; }
    pub fn setReadOnly(&self,readonly:i16){self.field_1_readonly.set(readonly);}
    // java public short getReadOnly() { return this.field_1_readonly; }
    pub fn getReadOnly(&self)->i16{self.field_1_readonly.get()}
    // java public void setPassword(short password) { this.field_2_password = password; }
    pub fn setPassword(&self,password:i16){self.field_2_password.set(password);}
    // java public void setPassword(ModifyVerifier password) { this.field_2_password_modify_verifier = password; }
    pub fn setPassword__verifier(&self,password:Option<Rc<RefCell<ModifyVerifier>>>){*self.field_2_password_modify_verifier.borrow_mut()=password;}
    // java public short getPassword() { return this.field_2_password; }
    pub fn getPassword(&self)->i16{self.field_2_password.get()}
    // java public ModifyVerifier getPasswordModifyVerifier() { return this.field_2_password_modify_verifier; }
    pub fn getPasswordModifyVerifier(&self)->Option<Rc<RefCell<ModifyVerifier>>>{self.field_2_password_modify_verifier.borrow().clone()}
    // java public String getUsername() { return this.field_3_username_value; }
    pub fn getUsername(&self)->Option<String>{self.field_3_username_value.borrow().clone()}
    // java public void setUsername(String username) { this.field_3_username_value = username; }
    pub fn setUsername(&self,username:Option<&str>){*self.field_3_username_value.borrow_mut()=username.map(str::to_owned);}
    // java public String toString() {
    pub fn toString(&self)->String{
        // java StringBuffer buffer = new StringBuffer(); buffer.append("[FILESHARING]\n");
        let mut buffer=String::from("[FILESHARING]\n");
        // java buffer.append("    .readonly       = ").append(this.getReadOnly() == 1 ? "true" : "false").append("\n");
        buffer.push_str(&format!("    .readonly       = {}\n",self.getReadOnly()==1));
        // java buffer.append("    .password       = ").append(Integer.toHexString(this.getPassword())).append("\n");
        buffer.push_str(&format!("    .password       = {:x}\n",self.getPassword()as i32));
        // java buffer.append("    .username       = ").append(this.getUsername()).append("\n");
        buffer.push_str(&format!("    .username       = {}\n",self.getUsername().as_deref().unwrap_or("null")));
        // java buffer.append("[/FILESHARING]\n"); return buffer.toString();
        buffer.push_str("[/FILESHARING]\n");buffer
    }
    // java public void serializez(LittleEndianOutput out) {
    pub fn serializez(&self,out:&mut dyn LittleEndianOutput){
        // java out.writeShort(this.getReadOnly());
        out.writeShort(self.getReadOnly()as i32);
        // java out.writeShort(this.getPassword());
        out.writeShort(self.getPassword()as i32);
        // java out.writeShort(this.field_3_username_value.length());
        let length=self.field_3_username_value.borrow().as_ref().expect("NullPointerException").encode_utf16().count()as i32;out.writeShort(length);
        // java if (this.field_3_username_value.length() > 0) {
        if !self.field_3_username_value.borrow().as_ref().expect("NullPointerException").is_empty(){
            // java out.writeByte(this.field_3_username_unicode_options);
            out.writeByte(self.field_3_username_unicode_options.get()as i32);
            // java if (this.field_3_username_unicode_options == 0) {
            if self.field_3_username_unicode_options.get()==0{
                // java StringUtil.putCompressedUnicode(this.getUsername(), out);
                StringUtil::putCompressedUnicode_out(&self.getUsername().expect("NullPointerException"),out);
            }else{
                // java StringUtil.putUnicodeLE(this.getUsername(), out);
                StringUtil::putUnicodeLE_out(&self.getUsername().expect("NullPointerException"),out);
            }
        }
    }
    // java protected int getDataSize() {
    pub fn getDataSize(&self)->i32{
        // java int nameLen = this.field_3_username_value.length();
        let mut nameLen=self.field_3_username_value.borrow().as_ref().expect("NullPointerException").encode_utf16().count()as i32;
        // java if (nameLen < 1) { return 6; }
        if nameLen<1{6}else{
            // java if (this.field_3_username_unicode_options != 0) { nameLen *= 2; }
            if self.field_3_username_unicode_options.get()!=0{nameLen=nameLen.wrapping_mul(2);}
            // java return 7 + nameLen;
            7i32.wrapping_add(nameLen)
        }
    }
    // java public short getSid() { return 91; }
    pub fn getSid(&self)->i16{91}
    // java public Object clone() {
    pub fn clone(&self)->Self{
        // java FileSharingRecord clone = new FileSharingRecord();
        let clone=Self::new();
        // java clone.setReadOnly(this.field_1_readonly);
        clone.setReadOnly(self.field_1_readonly.get());
        // java clone.setPassword(this.field_2_password);
        clone.setPassword(self.field_2_password.get());
        // java clone.setUsername(this.field_3_username_value);
        clone.setUsername(self.field_3_username_value.borrow().as_deref());
        // java return clone;
        clone
    }
    // java protected void serializeData(LittleEndianOutput out) {
    pub fn serializeData(&self,out:&mut dyn LittleEndianOutput){
        // java out.writeShort(this.field_1_readonly);
        out.writeShort(self.field_1_readonly.get()as i32);
        // java out.writeShort(this.field_2_password);
        out.writeShort(self.field_2_password.get()as i32);
        // java int nameLen = this.field_3_username_value.length();
        let nameLen=self.field_3_username_value.borrow().as_ref().expect("NullPointerException").encode_utf16().count()as i32;
        // java out.writeShort(nameLen);
        out.writeShort(nameLen);
        // java if (nameLen > 0) {
        if nameLen>0{
            // java out.writeByte(this.field_3_username_unicode_options);
            out.writeByte(self.field_3_username_unicode_options.get()as i32);
            // java if (this.field_3_username_unicode_options == 0) {
            if self.field_3_username_unicode_options.get()==0{
                // java StringUtil.putCompressedUnicode(this.field_3_username_value, out);
                let value=self.field_3_username_value.borrow().clone().expect("NullPointerException");StringUtil::putCompressedUnicode_out(&value,out);
            }else{
                // java StringUtil.putUnicodeLE(this.field_3_username_value, out);
                let value=self.field_3_username_value.borrow().clone().expect("NullPointerException");StringUtil::putUnicodeLE_out(&value,out);
            }
        }
    }
}
// rust: Native StandardRecord dispatch to the original methods above.
impl StandardRecord for FileSharingRecord{
    fn getDataSize(&self)->i32{FileSharingRecord::getDataSize(self)}
    fn getSid(&self)->i16{FileSharingRecord::getSid(self)}
    fn serializeData(&self,out:&mut dyn LittleEndianOutput){FileSharingRecord::serializeData(self,out)}
}
#[cfg(test)]
#[path="FileSharingRecord.review.rs"]
mod review;
#[cfg(test)]
#[path="ModifyPassword.review.rs"]
mod modify_password_review;
