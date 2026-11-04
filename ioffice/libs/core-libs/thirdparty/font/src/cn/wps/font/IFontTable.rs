// Java: import cn.wps.moffice.define.UILanguage
pub type UILanguage = i32;
// Java: import java.util.ArrayList
use std::collections::HashMap;
// Java: import java.util.List
use std::collections::HashSet;
// Java: import java.util.Map
use std::sync::Arc;
// Java: import java.util.Set
use crate::cn::wps::font::IFontFamily::IFontFamily;
use crate::cn::wps::font::IMathFontFamily::IMathFontFamily;

// Java: public interface IFontTable
pub trait IFontTable {
    // Java: abstract IFontFamily get(int var1)
    fn get_by_id(&self, var1: i32) -> Option<Arc<dyn IFontFamily>>;

    // Java: abstract IFontFamily get(int var1, boolean var2)
    fn get_by_id_bool(&self, var1: i32, var2: bool) -> Option<Arc<dyn IFontFamily>>;

    // Java: abstract IFontFamily get(String var1)
    fn get_by_name(&self, var1: &str) -> Option<Arc<dyn IFontFamily>>;

    // Java: abstract IFontFamily get(String var1, boolean var2)
    fn get_by_name_bool(&self, var1: &str, var2: bool) -> Option<Arc<dyn IFontFamily>>;

    // Java: abstract IFontFamily get(String var1, boolean var2, boolean var3)
    fn get_by_name_bool_bool(&self, var1: &str, var2: bool, var3: bool) -> Option<Arc<dyn IFontFamily>>;

    // Java: abstract IFontFamily getDefault()
    fn getDefault(&self) -> Option<Arc<dyn IFontFamily>>;

    // Java: abstract IFontFamily getDefault(UILanguage var1)
    fn getDefault_by_language(&self, var1: UILanguage) -> Option<Arc<dyn IFontFamily>>;

    // Java: abstract IMathFontFamily getMathFontFamily(String var1, boolean var2)
    fn getMathFontFamily(&self, var1: &str, var2: bool) -> Option<Arc<dyn IMathFontFamily>>;

    // Java: abstract IFontFamily register(String var1, boolean var2)
    fn register(&self, var1: &str, var2: bool) -> Option<Arc<dyn IFontFamily>>;

    // Java: abstract Map<String, IFontFamily> getFontMap()
    fn getFontMap(&self) -> HashMap<String, Arc<dyn IFontFamily>>;

    // Java: abstract ArrayList<String> getCJKName()
    fn getCJKName(&self) -> Vec<String>;

    // Java: abstract IFontFamily getBuildinFontFamily(String var1, IFontFamily var2)
    fn getBuildinFontFamily(&self, var1: &str, var2: Arc<dyn IFontFamily>) -> Option<Arc<dyn IFontFamily>>;

    // Java: abstract String getReplaceCloudFontId(String var1)
    fn getReplaceCloudFontId(&self, var1: &str) -> String;

    // Java: abstract String getReplaceCloudFontName(String var1)
    fn getReplaceCloudFontName(&self, var1: &str) -> String;

    // Java: abstract Set<String> getNoSupportFonts()
    fn getNoSupportFonts(&self) -> HashSet<String>;

    // Java: abstract void updateServerFontName(Map<String, String[]> var1)
    fn updateServerFontName(&self, var1: HashMap<String, Vec<String>>);

    // Java: abstract List<String> getFontAliasNames(String var1)
    fn getFontAliasNames(&self, var1: &str) -> Vec<String>;

    // Java: abstract void closeAllNativePtr()
    fn closeAllNativePtr(&self);
}
