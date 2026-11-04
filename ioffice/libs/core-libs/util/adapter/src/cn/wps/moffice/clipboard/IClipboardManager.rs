// Java: import java.util.HashMap
use std::collections::HashMap;

// Java: public interface IClipboardManager
pub trait IClipboardManager {
    // Java: abstract CharSequence getText()
    fn getText(&self) -> String;

    // Java: abstract boolean hasText()
    fn hasText(&self) -> bool;

    // Java: abstract void setText(CharSequence var1)
    fn setText(&self, var1: &str);

    // Java: abstract void setPrimaryClip(String var1, String var2, String[] var3)
    fn setPrimaryClip(&self, var1: &str, var2: &str, var3: &[String]);

    // Java: abstract boolean hasPrimaryClip()
    fn hasPrimaryClip(&self) -> bool;

    // Java: abstract IAssetFileDescriptor openTypedAssetFileDescriptor(String var1, HashMap<String, String> var2)
    fn openTypedAssetFileDescriptor(&self, var1: &str, var2: &HashMap<String, String>) -> Option<Box<dyn std::any::Any>>;

    // Java: abstract IClipDescription getClipDescription()
    fn getClipDescription(&self) -> Option<Box<dyn std::any::Any>>;

    // Java: abstract void setPlainTextPrimaryClip(String var1, String var2)
    fn setPlainTextPrimaryClip(&self, var1: &str, var2: &str);

}
