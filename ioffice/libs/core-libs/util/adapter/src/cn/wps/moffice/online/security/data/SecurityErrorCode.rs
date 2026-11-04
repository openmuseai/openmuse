// Java field: int ErrorCodeUnknownLocal
pub(crate) const ERROR_CODE_UNKNOWN_LOCAL: i32 = -1;
// Java field: int ErrorCodeVersionMismatch
pub(crate) const ERROR_CODE_VERSION_MISMATCH: i32 = -2;
// Java field: int ErrorCodeAuthError
pub(crate) const ERROR_CODE_AUTH_ERROR: i32 = 400000;
// Java field: int ErrorCodeNoFullControlPermission
pub(crate) const ERROR_CODE_NO_FULL_CONTROL_PERMISSION: i32 = 400001;
// Java field: int ErrorCodeNoAuthPermission
pub(crate) const ERROR_CODE_NO_AUTH_PERMISSION: i32 = 400002;
// Java field: int ErrorCodeNoReadPermission
pub(crate) const ERROR_CODE_NO_READ_PERMISSION: i32 = 400003;
// Java field: int ErrorCodeNoEditPermission
pub(crate) const ERROR_CODE_NO_EDIT_PERMISSION: i32 = 400004;
// Java field: int ErrorCodeNoSaveasPermission
pub(crate) const ERROR_CODE_NO_SAVEAS_PERMISSION: i32 = 400005;
// Java field: int ErrorCodeNoCopyPermission
pub(crate) const ERROR_CODE_NO_COPY_PERMISSION: i32 = 400006;
// Java field: int ErrorCodeNoPrintPermission
pub(crate) const ERROR_CODE_NO_PRINT_PERMISSION: i32 = 400007;
// Java field: int ErrorCodeNoPrintScreenPermission
pub(crate) const ERROR_CODE_NO_PRINT_SCREEN_PERMISSION: i32 = 400008;
// Java field: int ErrorCodePersonNoPermissionError
pub(crate) const ERROR_CODE_PERSON_NO_PERMISSION_ERROR: i32 = 40300200;
// Java field: int ErrorCodeGroupError
pub(crate) const ERROR_CODE_GROUP_ERROR: i32 = 40000700;
// Java field: int ErrorCodeParamFormatError
pub(crate) const ERROR_CODE_PARAM_FORMAT_ERROR: i32 = 410000;
// Java field: int ErrorCodeDocumentNotExist
pub(crate) const ERROR_CODE_DOCUMENT_NOT_EXIST: i32 = 410001;
// Java field: int ErrorCodeInvalidEncData
pub(crate) const ERROR_CODE_INVALID_ENC_DATA: i32 = 410002;
// Java field: int ErrorCodeInternalServerError
pub(crate) const ERROR_CODE_INTERNAL_SERVER_ERROR: i32 = 500000;
// Java field: int ErrorCodeParseJsonError
pub(crate) const ERROR_CODE_PARSE_JSON_ERROR: i32 = 500001;
// Java field: int ErrorCodeCreateDocument
pub(crate) const ERROR_CODE_CREATE_DOCUMENT: i32 = 500002;
// Java field: int ErrorCodeDeleteDocument
pub(crate) const ERROR_CODE_DELETE_DOCUMENT: i32 = 500003;
// Java field: int ErrorCodeUpdateRights
pub(crate) const ERROR_CODE_UPDATE_RIGHTS: i32 = 500004;
// Java field: int ErrorCodeUnknownError
pub(crate) const ERROR_CODE_UNKNOWN_ERROR: i32 = 999999;

// Java: public class SecurityErrorCode
pub struct SecurityErrorCode {
}

// Java: public class SecurityErrorCode
impl SecurityErrorCode {
    // Java: SecurityErrorCode()
    pub fn new() -> Self {
      // Java: super();
      SecurityErrorCode {}
    }

}
