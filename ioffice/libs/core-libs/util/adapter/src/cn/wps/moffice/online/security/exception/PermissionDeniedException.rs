// Java field: String MSG_PERMISSION_DENIED_NEED_LOGIN
pub(crate) const MSG_PERMISSION_DENIED_NEED_LOGIN: &str = "MSG_PERMISSION_DENIED_NEED_LOGIN";

// Java: public class PermissionDeniedException extends OnlineSecurityException
pub struct PermissionDeniedException {
}

// Java: public class PermissionDeniedException extends OnlineSecurityException
impl PermissionDeniedException {
    // Java: PermissionDeniedException()
    pub fn new() -> Self {
      // Java: super();
      PermissionDeniedException {}
    }

    // Java: PermissionDeniedException(String msg)
    pub fn new_with_msg(_msg: &str) -> Self {
      // Java: super(msg);
      PermissionDeniedException {}
    }

}
