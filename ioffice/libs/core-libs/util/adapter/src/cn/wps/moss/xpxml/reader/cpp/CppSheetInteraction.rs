// Java: public interface CppSheetInteraction
pub trait CppSheetInteraction {
    // Java: abstract void startSheetDataOffset(int var1)
    fn startSheetDataOffset(&self, _var1: i32);

    // Java: abstract void endSheetDataOffset(int var1)
    fn endSheetDataOffset(&self, _var1: i32);

    // Java: abstract void addRowRange(int var1, int var2, int var3, int var4)
    fn addRowRange(&self, _var1: i32, _var2: i32, _var3: i32, _var4: i32);

    // Java: abstract void setFileSize(int var1)
    fn setFileSize(&self, _var1: i32);

}
