// Java: public interface EtSheet
pub trait EtSheet {
    // Java: abstract boolean isSheetHidden()
    fn isSheetHidden(&self) -> bool;

    // Java: abstract boolean isRowHidden(int var1)
    fn isRowHidden(&self, _var1: i32) -> bool;

    // Java: abstract boolean isColHidden(int var1)
    fn isColHidden(&self, _var1: i32) -> bool;

    // Java: abstract String name()
    fn name(&self) -> String;

}
