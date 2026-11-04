use std::sync::Arc;
use crate::cn::wps::font::ITypeface::ITypeface;

// Java field: String SERIF
pub const SERIF: &str = "serif";

// Java: public interface IFontFamily extends Iterable
pub trait IFontFamily: Send + Sync {
    // Java: abstract String getName()
    fn getName(&self) -> String;

    // Java: abstract int getId()
    fn getId(&self) -> i32;

    // Java: abstract ITypeface getTypeface(int var1)
    fn getTypeface(&self, var1: i32) -> Option<Arc<dyn ITypeface>>;

    // Java: abstract boolean isSystemFont()
    fn isSystemFont(&self) -> bool;

    // Java: abstract boolean isOnlineFont()
    fn isOnlineFont(&self) -> bool;

    // Java: abstract boolean fontFileExist()
    fn fontFileExist(&self) -> bool;

    // Java: abstract void setOnlineFont(boolean var1)
    fn setOnlineFont(&self, var1: bool);

    // Java: abstract String[] getFontFiles(int var1)
    fn getFontFiles(&self, var1: i32) -> Vec<String>;

    // Java: abstract String getFontFile(int var1)
    fn getFontFile(&self, var1: i32) -> String;

    // Java: abstract String[] getAllFontFiles()
    fn getAllFontFiles(&self) -> Vec<String>;

    // Java: abstract long getNativeFreeTypeFontPointer(int var1)
    fn getNativeFreeTypeFontPointer(&self, var1: i32) -> i64;

    // Java: abstract ITypeface addTypefaceFromFile(int var1, String var2)
    fn addTypefaceFromFile(&self, var1: i32, var2: String) -> Option<Arc<dyn ITypeface>>;

    // Java: abstract void addFontFile(int var1, String var2)
    fn addFontFile(&self, var1: i32, var2: String);

    // Java: abstract boolean isBuildinHeader()
    fn isBuildinHeader(&self) -> bool;

    // Java: abstract IFontFamily getSourceFont()
    fn getSourceFont(&self) -> Option<Arc<dyn IFontFamily>>;

    // Java: abstract void closeNativePtr()
    fn closeNativePtr(&self);
}
