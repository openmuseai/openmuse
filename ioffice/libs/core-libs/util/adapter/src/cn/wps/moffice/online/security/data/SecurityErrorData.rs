// Java: public class SecurityErrorData
pub struct SecurityErrorData {
    // Java field: int errorCode
    pub(crate) errorCode: i32,
    // Java field: String errorMsg
    pub(crate) errorMsg: String,
}

// Java: public class SecurityErrorData
impl SecurityErrorData {
    // Java: SecurityErrorData(int errorCode, String errorMsg)
    pub fn new(_errorCode: i32, _errorMsg: &str) -> Self {
      // Java: super();
      // Java: this.errorCode = errorCode;
      // Java: this.errorMsg = errorMsg;
      SecurityErrorData { errorCode: _errorCode, errorMsg: _errorMsg.to_string() }
    }

}
