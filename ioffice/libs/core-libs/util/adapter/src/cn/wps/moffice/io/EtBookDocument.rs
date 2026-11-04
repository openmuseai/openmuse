// Java: public interface EtBookDocument extends IDocument
pub trait EtBookDocument {
    // Java: abstract EtSheet sheetAt(int var1)
    fn sheetAt(&self, _var1: i32) -> Box<dyn std::any::Any>;

    // Java: abstract int sheetCount()
    fn sheetCount(&self) -> i32;

    // Java: abstract String getFilePath()
    fn getFilePath(&self) -> String;

    // Java: abstract int sheetMergeIndex(EtSheet var1)
    fn sheetMergeIndex(&self, _var1: &dyn std::any::Any) -> i32;

    // Java: abstract int whichSheetIsActivated()
    fn whichSheetIsActivated(&self) -> i32;

    // Java: abstract void setAutoCalcOn(boolean var1)
    fn setAutoCalcOn(&mut self, _var1: bool);

    // Java: abstract boolean isAutoCalcOn()
    fn isAutoCalcOn(&self) -> bool;

}
