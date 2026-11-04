// Java: public interface FontReplaceBindHelper
pub trait FontReplaceBindHelper: Send + Sync {
    // Java: abstract void sortArrayByBind(String var1, String[] var2)
    fn sortArrayByBind(&self, var1: &str, var2: &mut Vec<String>);
}
