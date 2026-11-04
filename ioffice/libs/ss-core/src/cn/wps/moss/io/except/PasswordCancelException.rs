// Native Throwable transport for the original RuntimeException subclass.
// java public final class PasswordCancelException extends RuntimeException {
#[derive(Debug)]
pub struct PasswordCancelException{detailMessage:Option<String>,cause:Option<std::sync::Arc<dyn std::error::Error+Send+Sync>>}
impl PasswordCancelException {
    // java public PasswordCancelException() { }
    pub fn PasswordCancelException()->Self{Self{detailMessage:None,cause:None}}
    // java public PasswordCancelException(String detailMessage) { super(detailMessage); }
    pub fn PasswordCancelException__message(detailMessage:Option<String>)->Self{Self{detailMessage,cause:None}}
    // java public PasswordCancelException(String detailMessage, Throwable throwable) { super(detailMessage, throwable); }
    pub fn PasswordCancelException__message_throwable(detailMessage:Option<String>,throwable:Option<std::sync::Arc<dyn std::error::Error+Send+Sync>>)->Self{Self{detailMessage,cause:throwable}}
    // java public PasswordCancelException(Throwable throwable) { super(throwable); }
    pub fn PasswordCancelException__throwable(throwable:Option<std::sync::Arc<dyn std::error::Error+Send+Sync>>)->Self{Self{detailMessage:throwable.as_ref().map(|e|e.to_string()),cause:throwable}}
}
impl std::fmt::Display for PasswordCancelException{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str(self.detailMessage.as_deref().unwrap_or("PasswordCancelException"))}}
impl std::error::Error for PasswordCancelException{fn source(&self)->Option<&(dyn std::error::Error+'static)>{self.cause.as_deref().map(|e|e as _)}}
