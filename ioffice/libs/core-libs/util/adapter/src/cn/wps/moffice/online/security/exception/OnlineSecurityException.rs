// Java: public class OnlineSecurityException extends RuntimeException
pub struct OnlineSecurityException {
    // Java field: Integer mSecurityErrorCode
    pub(crate) mSecurityErrorCode: Option<i32>,
    // Java field: String docGuid
    pub(crate) docGuid: Option<String>,
}

// Java: public class OnlineSecurityException extends RuntimeException
impl OnlineSecurityException {
    // Java: OnlineSecurityException()
    pub fn new() -> Self {
        // Java: super();
        OnlineSecurityException { mSecurityErrorCode: None, docGuid: None }
    }

    // Java: OnlineSecurityException(String msg, int securityErrorCode)
    pub fn new_with_msg_and_code(_msg: &str, securityErrorCode: i32) -> Self {
        // Java: super(msg);
        // Java: this.mSecurityErrorCode = securityErrorCode;
        OnlineSecurityException { mSecurityErrorCode: Some(securityErrorCode), docGuid: None }
    }

    // Java: OnlineSecurityException(String msg)
    pub fn new_with_msg(_msg: &str) -> Self {
        // Java: super(msg);
        OnlineSecurityException { mSecurityErrorCode: None, docGuid: None }
    }

    // Java: OnlineSecurityException(int securityErrorCode)
    pub fn new_with_code(securityErrorCode: i32) -> Self {
        // Java: super();
        // Java: this.mSecurityErrorCode = securityErrorCode;
        OnlineSecurityException { mSecurityErrorCode: Some(securityErrorCode), docGuid: None }
    }

    // Java: Integer getSecurityErrorCode()
    pub fn getSecurityErrorCode(&self) -> Option<i32> {
        // Java: return this.mSecurityErrorCode;
        self.mSecurityErrorCode
    }

    // Java: void setDocGuid(String guid)
    pub fn setDocGuid(&mut self, guid: String) {
        // Java: this.docGuid = guid;
        self.docGuid = Some(guid);
    }

    // Java: String getDocGuid()
    pub fn getDocGuid(&self) -> Option<&str> {
        // Java: return this.docGuid;
        self.docGuid.as_deref()
    }

}
