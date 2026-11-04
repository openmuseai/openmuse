// java public final class NumberFormatFormatting implements Cloneable
#[derive(Clone)]
pub struct NumberFormatFormatting {
    isUser: bool,
    fmtCode: Option<String>,
    numFmtId: i32,
    cb: i32,
}
impl NumberFormatFormatting {
    // rust: this.numFmtId=numFmtId;this.fmtCode=fmtCode;this.isUser=fmtCode!=null;
    pub fn new(numFmtId: i32, fmtCode: Option<String>) -> Self {
        Self {
            numFmtId,
            isUser: fmtCode.is_some(),
            fmtCode,
            cb: 0,
        }
    }

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/NumberFormatFormatting.java:79
    // java public boolean isUser() {
    // java return this.isUser;
    // java }
    pub fn isUser(&self) -> bool {
        self.isUser
    }
    pub fn setIsUser(&mut self, isUser: bool) {
        self.isUser = isUser;
    }

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/NumberFormatFormatting.java:91
    // java public String fmtCode() {
    // java return this.fmtCode;
    // java }
    pub fn fmtCode(&self) -> Option<String> {
        self.fmtCode.clone()
    }
    pub fn setFmtCode(&mut self, fmtCode: Option<String>) {
        self.fmtCode = fmtCode;
    }

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/NumberFormatFormatting.java:99
    // java public int numFmtId() {
    // java return this.numFmtId;
    // java }
    pub fn numFmtId(&self) -> i32 {
        self.numFmtId
    }
    // java this.numFmtId=(byte)fmtId;

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/NumberFormatFormatting.java:95
    // java public void setNumFmtId(int fmtId) {
    // java this.numFmtId = (byte)fmtId;
    // java }
    pub fn setNumFmtId(&mut self, fmtId: i32) {
        self.numFmtId = fmtId as i8 as i32;
    }

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/NumberFormatFormatting.java:66
    // java public int getCB() {
    // java return this.cb;
    // java }
    pub fn getCB(&self) -> i32 {
        self.cb
    }
}

impl NumberFormatFormatting{// java public int hashCode() {
// java int prime = 31;
// java int result = 1;
// java result = 31 * result + this.cb;
// java result = 31 * result + (this.fmtCode == null ? 0 : this.fmtCode.hashCode());
// java result = 31 * result + (this.isUser ? 1231 : 1237);
// java result = 31 * result + this.numFmtId;
// java return result;
// java }
pub fn hashCode(&self)->i32 {
crate::cn::wps::moss::app::condfmt::java_values::fold([self.cb as i32,self.fmtCode.as_ref().map_or(0,|s|crate::cn::wps::moss::app::condfmt::java_values::string(s)),crate::cn::wps::moss::app::condfmt::java_values::boolean(self.isUser),self.numFmtId as i32])
}
// java public boolean equals(Object obj) {
// java if (this == obj) {
// java return true;
// java } else if (obj == null) {
// java return false;
// java } else if (this.getClass() != obj.getClass()) {
// java return false;
// java } else {
// java NumberFormatFormatting other = (NumberFormatFormatting)obj;
// java if (this.cb != other.cb) {
// java return false;
// java } else {
// java if (this.fmtCode == null) {
// java if (other.fmtCode != null) {
// java return false;
// java }
// java } else if (!this.fmtCode.equals(other.fmtCode)) {
// java return false;
// java }
// java
// java if (this.isUser != other.isUser) {
// java return false;
// java } else {
// java return this.numFmtId == other.numFmtId;
// java }
// java }
// java }
// java }
pub fn equals(&self,o:&Self)->bool {
std::ptr::eq(self,o)||(self.cb==o.cb&&self.fmtCode==o.fmtCode&&self.isUser==o.isUser&&self.numFmtId==o.numFmtId)
}
}
