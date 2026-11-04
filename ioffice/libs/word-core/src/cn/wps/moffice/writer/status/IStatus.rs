// java public interface IStatus
pub trait IStatus {
    // java  IStatus[] getChildren()
    fn getChildren(&self) -> &(dyn IStatus);

    // java  int getCode()
    fn getCode(&self) -> i32;

    // java  Throwable getException()
    fn getException(&self) -> String;

    // java  String getMessage()
    fn getMessage(&self) -> String;

    // java  String getPlugin()
    fn getPlugin(&self) -> String;

    // java  int getSeverity()
    fn getSeverity(&self) -> i32;

    // java  boolean isMultiStatus()
    fn isMultiStatus(&self) -> bool;

    // java  boolean isOK()
    fn isOK(&self) -> bool;

    // java  boolean matches(int var1)
    fn matches(&mut self, var1: i32) -> bool;

}
