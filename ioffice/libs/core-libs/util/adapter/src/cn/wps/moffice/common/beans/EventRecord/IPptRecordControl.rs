// Java field: int FROM_ACTIVITY
pub const FROM_ACTIVITY: i32 = 1;
// Java field: int FROM_POPWINDOW
pub const FROM_POPWINDOW: i32 = 2;
// Java field: int FROM_DIALOG
pub const FROM_DIALOG: i32 = 3;
// Java field: int FROM_EDIT_TEXT
pub const FROM_EDIT_TEXT: i32 = 4;

// Java: public interface IPptRecordControl
pub trait IPptRecordControl {
    // Java: abstract void recordEditContent(String var1, int var2, int var3)
    fn recordEditContent(&self, _var1: &str, _var2: i32, _var3: i32);

}
