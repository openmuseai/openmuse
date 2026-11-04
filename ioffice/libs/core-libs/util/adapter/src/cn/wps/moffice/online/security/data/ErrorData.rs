// Java field: String ERROR_TYPE_NO_PERMISSION
pub(crate) const ERROR_TYPE_NO_PERMISSION: &str = "ERROR_NO_PERMISSION";

// Java: public class ErrorData
pub struct ErrorData {
    // Java field: String errorType
    pub(crate) errorType: String,
}

// Java: public class ErrorData
impl ErrorData {
    // Java: static ErrorData parseError(int httpStatus)
    pub fn parseError(_httpStatus: i32) -> ErrorData {
      // Java: return 403 == httpStatus ? new ErrorData("ERROR_NO_PERMISSION") : new ErrorData("unknown");
      if 403 == _httpStatus {
          ErrorData::new("ERROR_NO_PERMISSION")
      } else {
          ErrorData::new("unknown")
      }
    }

    // Java: ErrorData(String errorType)
    pub fn new(_errorType: &str) -> Self {
      // Java: super();
      // Java: this.errorType = errorType;
      ErrorData { errorType: _errorType.to_string() }
    }

}
