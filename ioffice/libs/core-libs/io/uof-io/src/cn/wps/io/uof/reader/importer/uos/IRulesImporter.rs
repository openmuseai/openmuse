// Java: public interface IRulesImporter
pub trait IRulesImporter {
    // Java: abstract void setSheetIndex(int var1)
    fn setSheetIndex(&self, var1: i32);

    // Java: abstract void importAllConditionFormat()
    fn importAllConditionFormat(&self);

}
