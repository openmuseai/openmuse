use crate::java_runtime::{Cell,RefCell,Rc,Weak};
use crate::cn::wps::moss::{app::KmoBook::KmoBook,filefmt::biff8::record::FileSharingRecord::FileSharingRecord};
use io_poi_compact::cn::wps::{encrypt::{CryptTool::CryptTool,ModifyVerifier::ModifyVerifier},crypt::provider::ProviderInfo::{ProviderInfo,ENCRYPT_STD}};
// java public final class KmoBookCrypt
pub struct KmoBookCrypt {
    // java private KmoBook book;
    book:Weak<KmoBook>,
    // java private String mPasswd;
    mPasswd:RefCell<Option<String>>,
    // java private int nEncryptType;
    nEncryptType:Cell<i32>,
    // java private ProviderInfo mProviderInfo;
    mProviderInfo:RefCell<Option<Rc<RefCell<ProviderInfo>>>>,
    // java private FileSharingRecord sharingRec;
    sharingRec:RefCell<Option<Rc<FileSharingRecord>>>,
    // java public byte[] backupOtherEncryptHeadInfo = null;
    pub backupOtherEncryptHeadInfo:RefCell<Option<Rc<RefCell<Vec<u8>>>>>,
}
#[cfg(test)]
#[path="KmoBookCrypt.review.rs"]
mod review;
impl KmoBookCrypt {
    // java public KmoBookCrypt(KmoBook book) {
    pub fn new(book:Weak<KmoBook>)->Self{
        // java this.book = book; this.nEncryptType = ProviderInfo.ENCRYPT_STD; this.mPasswd = null; this.sharingRec = null;
        Self{book,mPasswd:RefCell::new(None),nEncryptType:Cell::new(ENCRYPT_STD),mProviderInfo:RefCell::new(None),sharingRec:RefCell::new(None),backupOtherEncryptHeadInfo:RefCell::new(None)}
    }
    // java public void setPasswd(String passwd) { this.setPasswd(passwd, ProviderInfo.ENCRYPT_STD, true); }
    pub fn setPasswd(&self,passwd:Option<&str>){self.setPasswd__3(passwd,ENCRYPT_STD,true);}
    // java public void setPasswd(String passwd, int encryptType, boolean bDirty) {
    pub fn setPasswd__3(&self,mut passwd:Option<&str>,encryptType:i32,bDirty:bool){
        // java if (passwd != null && passwd.length() == 0) { passwd = null; }
        if passwd.is_some_and(str::is_empty){passwd=None;}
        // java if (bDirty) {
        if bDirty{
            // java if (passwd == null && this.mPasswd != null && this.mPasswd.length() > 0) {
            let current=self.mPasswd.borrow().clone();
            if passwd.is_none()&&current.as_ref().is_some_and(|s|!s.is_empty()){
                // java this.book.setDirty(true);
                self.book.upgrade().expect("NullPointerException").setDirty(true);
            // java } else if (passwd == null || this.mPasswd != null && passwd.equals(this.mPasswd)) {
            }else if passwd.is_none()||current.as_ref().is_some_and(|s|passwd==Some(s.as_str())){
                // java if (encryptType != this.nEncryptType) { this.book.setDirty(true); }
                if encryptType!=self.nEncryptType.get(){self.book.upgrade().expect("NullPointerException").setDirty(true);}
            }else{
                // java this.book.setDirty(true);
                self.book.upgrade().expect("NullPointerException").setDirty(true);
            }
        }
        // java this.mPasswd = passwd;
        *self.mPasswd.borrow_mut()=passwd.map(str::to_owned);
        // java this.nEncryptType = encryptType;
        self.nEncryptType.set(encryptType);
    }
    // java public boolean hasPasswd() { return this.mPasswd != null && this.mPasswd.length() > 0; }
    pub fn hasPasswd(&self)->bool{self.mPasswd.borrow().as_ref().is_some_and(|s|!s.is_empty())}
    // java public String getPasswd() { return this.mPasswd; }
    pub fn getPasswd(&self)->Option<String>{self.mPasswd.borrow().clone()}
    // java public int getEncryptType() { return this.nEncryptType; }
    pub fn getEncryptType(&self)->i32{self.nEncryptType.get()}
    // java public boolean hasModifyPasswd() { return this.sharingRec != null; }
    pub fn hasModifyPasswd(&self)->bool{self.sharingRec.borrow().is_some()}
    // java public boolean isModifyPassword(String passwd) {
    pub fn isModifyPassword(&self,passwd:Option<&str>)->bool{
        // java if (this.sharingRec == null) { return false; }
        if self.sharingRec.borrow().is_none(){false}else{
            // java ModifyVerifier passwordModifyVerifier = this.sharingRec.getPasswordModifyVerifier();
            let passwordModifyVerifier=self.sharingRec.borrow().clone().unwrap().getPasswordModifyVerifier();
            // java if (passwordModifyVerifier != null) { return CryptTool.checkModifyPassword(passwordModifyVerifier, passwd); }
            if let Some(passwordModifyVerifier)=passwordModifyVerifier{let result=CryptTool::checkModifyPassword__nullable(Some(&passwordModifyVerifier.borrow()),passwd);result}else{
                // java short hash = FileSharingRecord.hashPassword(passwd);
                let hash=FileSharingRecord::hashPassword(passwd);
                // java return hash == this.sharingRec.getPassword();
                hash==self.sharingRec.borrow().clone().unwrap().getPassword()
            }
        }
    }
    // java public void setModifyPasswd(String passwd) {
    pub fn setModifyPasswd(&self,passwd:Option<&str>){
        // java if (passwd != null && passwd.length() > 15) { throw new IllegalArgumentException("密码不能超过15个字符"); }
        if passwd.is_some_and(|s|s.encode_utf16().count()>15){panic!("IllegalArgumentException: 密码不能超过15个字符");}else{
            // java short hash = FileSharingRecord.hashPassword(passwd);
            let hash=FileSharingRecord::hashPassword(passwd);
            // java if (hash != 0) {
            if hash!=0{
                // java if (this.sharingRec == null) {
                if self.sharingRec.borrow().is_none(){
                    // java this.sharingRec = new FileSharingRecord();
                    *self.sharingRec.borrow_mut()=Some(Rc::new(FileSharingRecord::new()));
                    // java this.book.setDirty(true);
                    self.book.upgrade().expect("NullPointerException").setDirty(true);
                }
                // java if (this.sharingRec.getPassword() != hash) {
                let old=self.sharingRec.borrow().clone().unwrap().getPassword();if old!=hash{
                    // java this.sharingRec.setPassword(hash);
                    self.sharingRec.borrow().clone().unwrap().setPassword(hash);
                    // java this.book.setDirty(true);
                    self.book.upgrade().expect("NullPointerException").setDirty(true);
                }
                // java this.handleSha512Password(passwd);
                self.handleSha512Password(passwd);
            }else{
                // java if (this.sharingRec != null && this.sharingRec.getPassword() != 0) { this.book.setDirty(true); }
                let record=self.sharingRec.borrow().clone();if record.is_some_and(|r|r.getPassword()!=0){self.book.upgrade().expect("NullPointerException").setDirty(true);}
                // java this.sharingRec = null;
                *self.sharingRec.borrow_mut()=None;
            }
        }
    }
    // java private void handleSha512Password(String passwd) {
    fn handleSha512Password(&self,passwd:Option<&str>){
        // java if (this.sharingRec != null && this.book != null && passwd != null) {
        if self.sharingRec.borrow().is_some()&&self.book.upgrade().is_some()&&passwd.is_some(){
            // java ModifyVerifier passwordModifyVerifier = this.sharingRec.getPasswordModifyVerifier();
            let passwordModifyVerifier=self.sharingRec.borrow().clone().unwrap().getPasswordModifyVerifier();
            // java if (passwordModifyVerifier == null || !CryptTool.checkModifyPassword(passwordModifyVerifier, passwd)) {
            let replace=match passwordModifyVerifier{None=>true,Some(p)=>!CryptTool::checkModifyPassword__nullable(Some(&p.borrow()),passwd)};
            if replace{
                // java ModifyVerifier modifyVerifierSHA512 = CryptTool.createModifyVerifierSHA512(passwd);
                let modifyVerifierSHA512=Rc::new(RefCell::new(CryptTool::createModifyVerifierSHA512__nullable(passwd)));
                // java this.sharingRec.setPassword(modifyVerifierSHA512);
                self.sharingRec.borrow().clone().unwrap().setPassword__verifier(Some(modifyVerifierSHA512));
                // java this.book.setDirty(true);
                self.book.upgrade().expect("NullPointerException").setDirty(true);
            }
        }
    }
    // java public void setFileSharing(FileSharingRecord rec) { this.sharingRec = rec; }
    pub fn setFileSharing(&self,rec:Option<Rc<FileSharingRecord>>){*self.sharingRec.borrow_mut()=rec;}
    // java public FileSharingRecord getFileSharing() { return this.sharingRec; }
    pub fn getFileSharing(&self)->Option<Rc<FileSharingRecord>>{self.sharingRec.borrow().clone()}
    // java public ProviderInfo getProviderInfo() { return this.mProviderInfo; }
    pub fn getProviderInfo(&self)->Option<Rc<RefCell<ProviderInfo>>>{self.mProviderInfo.borrow().clone()}
    // java public void setProviderInfo(ProviderInfo providerInfo) { this.mProviderInfo = providerInfo; }
    pub fn setProviderInfo(&self,providerInfo:Option<Rc<RefCell<ProviderInfo>>>){*self.mProviderInfo.borrow_mut()=providerInfo;}
}
