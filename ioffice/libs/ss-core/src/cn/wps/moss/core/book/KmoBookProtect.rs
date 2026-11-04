use crate::java_runtime::{Cell,RefCell,Rc,Weak};
use crate::cn::wps::moss::{app::KmoBook::KmoBook,filefmt::biff8::record::FileSharingRecord::FileSharingRecord};
use io_poi_compact::cn::wps::encrypt::{CryptTool::CryptTool,ModifyVerifier::ModifyVerifier};
use util_adapter::cn::wps::moffice::io::IKmoBookProtect::IKmoBookProtect;
// java public final class KmoBookProtect implements IKmoBookProtect
pub struct KmoBookProtect {
    // java private boolean fLockWn = false;
    fLockWn:Cell<bool>,
    // java private boolean fLock = false;
    fLock:Cell<bool>,
    // java private int wPassword = 0;
    wPassword:Cell<i32>,
    // java private ModifyVerifier mSha512Password = null;
    mSha512Password:RefCell<Option<Rc<RefCell<ModifyVerifier>>>>,
    // java public boolean fRevLock = false;
    pub fRevLock:Cell<bool>,
    // java public int protPwdRev = 0;
    pub protPwdRev:Cell<i32>,
    // java private final KmoBook book;
    // rust: same book back-reference convention as the existing transaction graph.
    book:Weak<KmoBook>,
}
impl KmoBookProtect {
    // java public KmoBookProtect(KmoBook book) { this.book = book; }
    pub fn new(book:Weak<KmoBook>)->Self{Self{fLockWn:Cell::new(false),fLock:Cell::new(false),wPassword:Cell::new(0),mSha512Password:RefCell::new(None),fRevLock:Cell::new(false),protPwdRev:Cell::new(0),book}}
    // java public boolean isDefaultBookProtect() { return this.wPassword == 0 && this.mSha512Password == null && !this.fLock && !this.fLockWn && !this.fRevLock && this.protPwdRev == 0; }
    pub fn isDefaultBookProtect(&self)->bool{self.wPassword.get()==0&&self.mSha512Password.borrow().is_none()&&!self.fLock.get()&&!self.fLockWn.get()&&!self.fRevLock.get()&&self.protPwdRev.get()==0}
    // java public void setLockWindow(boolean b) { this.fLockWn = b; }
    pub fn setLockWindow(&self,b:bool){self.fLockWn.set(b);}
    // java public boolean getLockWindow() { return this.fLockWn; }
    pub fn getLockWindow(&self)->bool{self.fLockWn.get()}
    // java public void setLockStructure(boolean b) { this.fLock = b; }
    pub fn setLockStructure(&self,b:bool){self.fLock.set(b);}
    // java public boolean getLockStructure() { return this.fLock; }
    pub fn getLockStructure(&self)->bool{self.fLock.get()}
    // java public boolean checkPassword(String passwd) {
    pub fn checkPassword(&self,passwd:Option<&str>)->bool{
        // java if (this.mSha512Password == null) {
        if self.mSha512Password.borrow().is_none(){
            // java short hash = (short)(this.wPassword & '\uffff');
            let hash=(self.wPassword.get()&0xffff)as i16;
            // java short hash2 = FileSharingRecord.hashPassword(passwd);
            let hash2=FileSharingRecord::hashPassword(passwd);
            // java return hash == hash2;
            hash==hash2
        }else{
            // java return CryptTool.checkModifyPassword(this.mSha512Password, passwd);
            let password=self.mSha512Password.borrow().clone().unwrap();let value=CryptTool::checkModifyPassword__nullable(Some(&password.borrow()),passwd);value
        }
    }
    // java public void setPassword(String passwd) {
    pub fn setPassword__string(&self,passwd:Option<&str>){
        // java short hash = FileSharingRecord.hashPassword(passwd);
        let hash=FileSharingRecord::hashPassword(passwd);
        // java this.wPassword = hash & '\uffff';
        self.wPassword.set(hash as i32&0xffff);
        // java this.mSha512Password = CryptTool.createModifyVerifierSHA512(passwd);
        *self.mSha512Password.borrow_mut()=Some(Rc::new(RefCell::new(CryptTool::createModifyVerifierSHA512__nullable(passwd))));
        // java this.fLock = true;
        self.fLock.set(true);
        // java this.book.setDirty(true);
        self.book.upgrade().expect("NullPointerException").setDirty(true);
        // java this.book.changeListenerGroup().bookSettingsDidFinishChanging();
        self.book.upgrade().expect("NullPointerException").changeListenerGroup().bookSettingsDidFinishChanging();
    }
    // java public void setPassword(int passwd) { this.wPassword = passwd; }
    pub fn setPassword(&self,passwd:i32){self.wPassword.set(passwd);}
    // java public void setPassword(ModifyVerifier passwd) { this.mSha512Password = passwd; }
    pub fn setPassword__verifier(&self,passwd:Option<Rc<RefCell<ModifyVerifier>>>){*self.mSha512Password.borrow_mut()=passwd;}
    // java public int getPassword() { return this.wPassword; }
    pub fn getPassword(&self)->i32{self.wPassword.get()}
    // java public ModifyVerifier getSha512Password() { return this.mSha512Password; }
    pub fn getSha512Password(&self)->Option<Rc<RefCell<ModifyVerifier>>>{self.mSha512Password.borrow().clone()}
    // java public boolean isBookProtected() { return this.fLock || this.fLockWn; }
    pub fn isBookProtected(&self)->bool{self.fLock.get()||self.fLockWn.get()}
    // java public void cancelBookProtected() {
    pub fn cancelBookProtected(&self){
        // java this.wPassword = 0;
        self.wPassword.set(0);
        // java this.mSha512Password = null;
        *self.mSha512Password.borrow_mut()=None;
        // java this.fLock = false;
        self.fLock.set(false);
        // java this.fLockWn = false;
        self.fLockWn.set(false);
        // java this.book.setDirty(true);
        self.book.upgrade().expect("NullPointerException").setDirty(true);
        // java this.book.changeListenerGroup().bookSettingsDidFinishChanging();
        self.book.upgrade().expect("NullPointerException").changeListenerGroup().bookSettingsDidFinishChanging();
    }
    // java public boolean isCorrectPassword(String ps) {
    pub fn isCorrectPassword(&self,ps:Option<&str>)->bool{
        // java if (this.mSha512Password == null) {
        if self.mSha512Password.borrow().is_none(){
            // java short hash = FileSharingRecord.hashPassword(ps);
            let hash=FileSharingRecord::hashPassword(ps);
            // java int p = hash & '\uffff';
            let p=hash as i32&0xffff;
            // java return p == this.wPassword;
            p==self.wPassword.get()
        }else{
            // java return CryptTool.checkModifyPassword(this.mSha512Password, ps);
            let password=self.mSha512Password.borrow().clone().unwrap();let value=CryptTool::checkModifyPassword__nullable(Some(&password.borrow()),ps);value
        }
    }
    // java public boolean isLockRevion() { return this.fRevLock; }
    pub fn isLockRevion(&self)->bool{self.fRevLock.get()}
    // java public void setLockRevion(boolean isRevLock) { this.fRevLock = isRevLock; }
    pub fn setLockRevion(&self,isRevLock:bool){self.fRevLock.set(isRevLock);}
    // java public int getRevisionPassword() { return this.protPwdRev; }
    pub fn getRevisionPassword(&self)->i32{self.protPwdRev.get()}
    // java public void setRevisionPassword(int psd) { this.protPwdRev = psd; }
    pub fn setRevisionPassword(&self,psd:i32){self.protPwdRev.set(psd);}
    // java public boolean isCorrectRevisionPassword(String s) {
    pub fn isCorrectRevisionPassword(&self,s:Option<&str>)->bool{
        // java short hash = FileSharingRecord.hashPassword(s);
        let hash=FileSharingRecord::hashPassword(s);
        // java int p = hash & '\uffff';
        let p=hash as i32&0xffff;
        // java return p == this.protPwdRev;
        p==self.protPwdRev.get()
    }
}
// rust: Native Java interface dispatch; original bodies remain above.
impl IKmoBookProtect for KmoBookProtect{
 fn isDefaultBookProtect(&self)->bool{Self::isDefaultBookProtect(self)}
 fn getLockWindow(&self)->bool{Self::getLockWindow(self)}
 fn getLockStructure(&self)->bool{Self::getLockStructure(self)}
 fn checkPassword(&self,s:Option<&str>)->bool{Self::checkPassword(self,s)}
 fn setPassword_from_str(&self,s:Option<&str>){Self::setPassword__string(self,s)}
 fn setPassword(&self,p:i32){Self::setPassword(self,p)}
 fn getPassword(&self)->i32{Self::getPassword(self)}
 fn isBookProtected(&self)->bool{Self::isBookProtected(self)}
 fn cancelBookProtected(&self){Self::cancelBookProtected(self)}
 fn isCorrectPassword(&self,s:Option<&str>)->bool{Self::isCorrectPassword(self,s)}
 fn isLockRevion(&self)->bool{Self::isLockRevion(self)}
 fn setLockRevion(&self,b:bool){Self::setLockRevion(self,b)}
 fn getRevisionPassword(&self)->i32{Self::getRevisionPassword(self)}
 fn setRevisionPassword(&self,p:i32){Self::setRevisionPassword(self,p)}
 fn isCorrectRevisionPassword(&self,s:Option<&str>)->bool{Self::isCorrectRevisionPassword(self,s)}
}
#[cfg(test)]
#[path="KmoBookProtect.review.rs"]
mod review;
