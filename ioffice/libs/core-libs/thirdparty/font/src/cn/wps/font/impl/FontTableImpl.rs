// Java: import cn.wps.base.log.Log
// Java: import cn.wps.font.IFontFamily
// Java: import cn.wps.font.IFontTable
// Java: import cn.wps.font.IMathFontFamily
// Java: import cn.wps.font.ITypeface
// Java: import cn.wps.moffice.define.UILanguage
// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.HashMap
// Java: import java.util.HashSet
// Java: import java.util.LinkedHashMap
// Java: import java.util.List
// Java: import java.util.Map
// Java: import java.util.Set
use std::sync::{Arc, Mutex, OnceLock};
use std::collections::HashMap;
use crate::cn::wps::font::IFontFamily::IFontFamily;
use crate::cn::wps::font::IFontTable::IFontTable;
use crate::cn::wps::font::IMathFontFamily::IMathFontFamily;

pub type UILanguage = i32;

// Java field: HashMap<String, String[]> FONT_ALIAS_NAMES
static FONT_ALIAS_NAMES: OnceLock<Mutex<HashMap<String, Vec<String>>>> = OnceLock::new();
// Java field: Map<String, String[]> FONT_REPLACE_CLOUD
static FONT_REPLACE_CLOUD: OnceLock<Mutex<HashMap<String, Vec<String>>>> = OnceLock::new();
// Java field: Map<String, String[]> FONT_SERVER_REPLACE_CLOUD
static FONT_SERVER_REPLACE_CLOUD: OnceLock<Mutex<HashMap<String, Vec<String>>>> = OnceLock::new();
// Java field: HashMap<String, String[]> FONT_REPLACE_NAMES
static FONT_REPLACE_NAMES: OnceLock<Mutex<HashMap<String, Vec<String>>>> = OnceLock::new();

fn get_font_alias_names() -> &'static Mutex<HashMap<String, Vec<String>>> {
    FONT_ALIAS_NAMES.get_or_init(|| Mutex::new(HashMap::new()))
}
fn get_font_replace_cloud() -> &'static Mutex<HashMap<String, Vec<String>>> {
    FONT_REPLACE_CLOUD.get_or_init(|| Mutex::new(HashMap::new()))
}
fn get_font_server_replace_cloud() -> &'static Mutex<HashMap<String, Vec<String>>> {
    FONT_SERVER_REPLACE_CLOUD.get_or_init(|| Mutex::new(HashMap::new()))
}
fn get_font_replace_names() -> &'static Mutex<HashMap<String, Vec<String>>> {
    FONT_REPLACE_NAMES.get_or_init(|| Mutex::new(HashMap::new()))
}

// Java: public class FontTableImpl implements IFontTable
pub struct FontTableImpl {
    // Java field: ArrayList<IFontFamily> mArray
    pub mArray: Vec<Arc<dyn IFontFamily>>,
    // Java field: Map<String, IFontFamily> mMap
    pub mMap: HashMap<String, Arc<dyn IFontFamily>>,
}
// Java field: int DEFAULT_FONT_ID
pub const DEFAULT_FONT_ID: i32 = 0;

// Java: public class FontTableImpl
impl FontTableImpl {
    // Java: static void resolveMultipleFontNames(HashMap<String, String[]> nameMap, String mainName, String... aliasNames)
    fn resolveMultipleFontNames(name_map: &mut HashMap<String, Vec<String>>, main_name: &str, alias_names: &[&str]) {
        // Java: if (null != nameMap && null != mainName && null != aliasNames && 0 != aliasNames.length)
        if alias_names.is_empty() {
            return;
        }
        // Java: String[] mappedNames = (String[])nameMap.get(mainName);
        let mapped_names = name_map.get(main_name).cloned();
        // Java: int count = 0;
        let mut count = 0usize;
        // Java: count += aliasNames.length;
        count += alias_names.len();
        // Java: count += null != mappedNames ? mappedNames.length : 0;
        count += mapped_names.as_ref().map_or(0, |v| v.len());
        // Java: String[] values = new String[count];
        let mut values: Vec<String> = Vec::with_capacity(count);
        // Java: System.arraycopy(aliasNames, 0, values, 0, aliasNames.length);
        for a in alias_names {
            values.push(a.to_string());
        }
        // Java: if (null != mappedNames)
        if let Some(ref mn) = mapped_names {
            // Java: int len = mappedNames.length;
            // Java: System.arraycopy(mappedNames, 0, values, aliasNames.length, mappedNames.length);
            for m in mn {
                values.push(m.clone());
            }
        }
        // Java: nameMap.put(mainName, values);
        name_map.insert(main_name.to_string(), values);
        // Java: int i = 0;
        // Java: for(int n = aliasNames.length; i < n; ++i)
        for i in 0..alias_names.len() {
            // Java: int idx = 0;
            // Java: values = new String[count];
            let mut vals: Vec<String> = Vec::with_capacity(count);
            // Java: values[idx++] = mainName;
            vals.push(main_name.to_string());
            // Java: if (null != mainName)
            {
                // Java: if (i > 0)
                if i > 0 {
                    // Java: System.arraycopy(aliasNames, 0, values, idx, i);
                    for j in 0..i {
                        vals.push(alias_names[j].to_string());
                    }
                    // Java: idx += i;
                }
                // Java: int tailLen = n - i - 1;
                let tail_len = alias_names.len() - i - 1;
                // Java: if (tailLen > 0)
                if tail_len > 0 {
                    // Java: System.arraycopy(aliasNames, i + 1, values, idx, tailLen);
                    for j in (i + 1)..alias_names.len() {
                        vals.push(alias_names[j].to_string());
                    }
                    // Java: idx += tailLen;
                }
            }
            // Java: if (null != mappedNames)
            if let Some(ref mn) = mapped_names {
                // Java: int len = mappedNames.length;
                // Java: System.arraycopy(mappedNames, 0, values, idx, len);
                for m in mn {
                    vals.push(m.clone());
                }
                // Java: int var10000 = idx + len;
            }
            // Java: nameMap.put(aliasNames[i], values);
            name_map.insert(alias_names[i].to_string(), vals);
        }
    }

    // Java: FontTableImpl()
    pub fn new() -> Self {
        // Java: this(true);
        Self::new_with(true)
    }

    // Java: FontTableImpl(boolean createDefault)
    pub fn new_with(create_default: bool) -> Self {
        // Java: super();
        // Java: this.mArray = new ArrayList(127);
        let mut inst = FontTableImpl {
            mArray: Vec::with_capacity(127),
            // Java: this.mMap = new HashMap(127);
            mMap: HashMap::with_capacity(127),
        };
        // Java: if (createDefault)
        if create_default {
            // Java: FontFamilyImpl serif = (FontFamilyImpl)this.register("serif", true);
            // Java: ITypeface face = TypefaceFactory.createTypeface(serif, 0);
            // Java: serif.addTypeface(0, face);
            // Java: face = TypefaceFactory.createTypeface(serif, 0);
            // Java: ((AbstractTypeface)face).setNeedBold(true);
            // Java: serif.addTypeface(1, face);
            // Java: face = TypefaceFactory.createTypeface(serif, 0);
            // Java: ((AbstractTypeface)face).setNeedItalic(true);
            // Java: serif.addTypeface(2, face);
            // Java: face = TypefaceFactory.createTypeface(serif, 0);
            // Java: ((AbstractTypeface)face).setNeedBold(true);
            // Java: ((AbstractTypeface)face).setNeedItalic(true);
            // Java: serif.addTypeface(3, face);
            let _ = create_default;
        }
        inst
    }

    // Java: IFontFamily register(String familyName, boolean isSystemFont)
    pub fn register_impl(&mut self, _family_name: &str, _is_system_font: bool) -> Option<Arc<dyn IFontFamily>> {
        // Java: if (familyName != null && familyName.length() != 0)
        // Java: IFontFamily family = (IFontFamily)this.mMap.get(familyName);
        // Java: if (family != null)
        // Java: return family;
        // Java: else
        // Java: int id = this.mArray.size();
        // Java: family = new FontFamilyImpl(familyName, isSystemFont, id);
        // Java: this.mArray.add(family);
        // Java: this.mMap.put(familyName, family);
        // Java: return family;
        // Java: else
        // Java: return this.getDefault();
        None
    }

    // Java: IFontFamily get(int id)
    pub fn get_by_id_impl(&self, id: i32) -> Option<Arc<dyn IFontFamily>> {
        // Java: return this.get(id, false);
        self.get_by_id_bool_impl(id, false)
    }

    // Java: IFontFamily get(int id, boolean defaultIfNotExist)
    pub fn get_by_id_bool_impl(&self, id: i32, default_if_not_exist: bool) -> Option<Arc<dyn IFontFamily>> {
        // Java: if (id >= 0 && id < this.mArray.size())
        if id >= 0 && (id as usize) < self.mArray.len() {
            // Java: IFontFamily fontFamily = (IFontFamily)this.mArray.get(id);
            let font_family = self.mArray.get(id as usize).cloned();
            // Java: if (fontFamily != null)
            if font_family.is_some() {
                // Java: return fontFamily;
                return font_family;
            }
            // Java: else
            // Java: return defaultIfNotExist ? this.getDefault() : null;
            if default_if_not_exist { self.getDefault_impl() } else { None }
        }
        // Java: else
        // Java: throw new IndexOutOfBoundsException("Invalid family id: " + id);
        else {
            None
        }
    }

    // Java: IFontFamily get(String familyName)
    pub fn get_by_name_impl(&self, family_name: &str) -> Option<Arc<dyn IFontFamily>> {
        // Java: return this.get(familyName, false);
        self.get_by_name_bool_impl(family_name, false)
    }

    // Java: IFontFamily get(String familyName, boolean defaultIfNotExist)
    pub fn get_by_name_bool_impl(&self, family_name: &str, default_if_not_exist: bool) -> Option<Arc<dyn IFontFamily>> {
        // Java: return this.get(familyName, defaultIfNotExist, true);
        self.get_by_name_bool_bool_impl(family_name, default_if_not_exist, true)
    }

    // Java: IFontFamily doGetFromMap(String familyName)
    fn doGetFromMap(&self, family_name: &str) -> Option<Arc<dyn IFontFamily>> {
        // Java: return (IFontFamily)this.mMap.get(familyName);
        self.mMap.get(family_name).cloned()
    }

    // Java: IFontFamily get(String familyName, boolean defaultIfNotExist, boolean replaceIfNotExist)
    pub fn get_by_name_bool_bool_impl(&self, family_name: &str, default_if_not_exist: bool, _replace_if_not_exist: bool) -> Option<Arc<dyn IFontFamily>> {
        // Java: if (familyName.length() <= 0)
        if family_name.is_empty() {
            // Java: return this.getDefault();
            return self.getDefault_impl();
        }
        // Java: else
        // Java: IFontFamily fontFamily = this.doGetFromMap(familyName);
        let mut font_family = self.doGetFromMap(family_name);
        // Java: if (fontFamily == null)
        if font_family.is_none() {
            // Java: String s = FontNameDecoder.removeEncodingSuffix(familyName);
            // Java: fontFamily = this.doGetFromMap(s);
            let _ = family_name;
        }
        // Java: if (fontFamily != null)
        if font_family.is_some() {
            // Java: return fontFamily;
            return font_family;
        }
        // Java: else
        // Java: String[] aliasNames = (String[])FONT_ALIAS_NAMES.get(familyName);
        let alias_guard = get_font_alias_names().lock().unwrap();
        let alias_names = alias_guard.get(family_name).cloned();
        drop(alias_guard);
        // Java: if (null != aliasNames)
        if let Some(ref aliases) = alias_names {
            // Java: FontReplaceBindHelper helper = FontReplaceBindHelperFactory.getImpl();
            // Java: if (helper != null)
            // Java: helper.sortArrayByBind(familyName, aliasNames);
            // Java: int i = 0;
            // Java: for(int n = aliasNames.length; i < n; ++i)
            for alias in aliases {
                // Java: fontFamily = this.doGetFromMap(aliasNames[i]);
                font_family = self.doGetFromMap(alias);
                // Java: if (null != fontFamily)
                if font_family.is_some() {
                    // Java: return fontFamily;
                    return font_family;
                }
            }
        }
        // Java: return defaultIfNotExist ? this.getDefault() : null;
        if default_if_not_exist { self.getDefault_impl() } else { None }
    }

    // Java: IFontFamily getDefault()
    pub fn getDefault_impl(&self) -> Option<Arc<dyn IFontFamily>> {
        // Java: if (this.mArray.size() <= 0)
        if self.mArray.is_empty() {
            // Java: throw new RuntimeException("No fonts!!");
            return None;
        }
        // Java: else
        // Java: return (IFontFamily)this.mArray.get(0);
        self.mArray.first().cloned()
    }

    // Java: IFontFamily getDefault(UILanguage uiLanguage)
    pub fn getDefault_by_language_impl(&self, _ui_language: UILanguage) -> Option<Arc<dyn IFontFamily>> {
        // Java: if (uiLanguage == UILanguage.UILanguage_Thai)
        // Java: IFontFamily iFontFamily = this.get("Noto Sans Thai");
        // Java: if (null != iFontFamily)
        // Java: return iFontFamily;
        // Java: iFontFamily = this.get("Noto Serif Thai");
        // Java: if (null != iFontFamily)
        // Java: return iFontFamily;
        // Java: else if (uiLanguage == UILanguage.UILanguage_Arabic)
        // Java: IFontFamily iFontFamily = this.get("Scheherazade");
        // Java: if (iFontFamily != null)
        // Java: return iFontFamily;
        // Java: return this.getDefault();
        self.getDefault_impl()
    }

    // Java: IFontFamily[] getFontFamilys()
    pub fn getFontFamilys(&self) -> Vec<Arc<dyn IFontFamily>> {
        // Java: IFontFamily[] fontFamilys = new IFontFamily[this.mArray.size()];
        // Java: return (IFontFamily[])this.mArray.toArray(fontFamilys);
        self.mArray.clone()
    }

    // Java: IMathFontFamily getMathFontFamily(String familyName, boolean defaultIfNotExist)
    pub fn getMathFontFamily_impl(&self, _family_name: &str, _default_if_not_exist: bool) -> Option<Arc<dyn IMathFontFamily>> {
        // Java: if (familyName.length() <= 0)
        // Java: throw new IllegalArgumentException("Invalid familyName: " + familyName);
        // Java: else
        // Java: IFontFamily fontFamily = this.doGetFromMap(familyName);
        // Java: if (fontFamily == null)
        // Java: String[] aliasNames = (String[])FONT_ALIAS_NAMES.get(familyName);
        // Java: if (null != aliasNames)
        // Java: FontReplaceBindHelper helper = FontReplaceBindHelperFactory.getImpl();
        // Java: if (helper != null)
        // Java: helper.sortArrayByBind(familyName, aliasNames);
        // Java: int i = 0;
        // Java: for(int n = aliasNames.length; i < n; ++i)
        // Java: fontFamily = this.doGetFromMap(aliasNames[i]);
        // Java: if (null != fontFamily)
        // Java: break;
        // Java: if (fontFamily != null)
        // Java: return new MathFontFamily(fontFamily);
        // Java: else
        // Java: return defaultIfNotExist ? new MathFontFamily(this.getDefault()) : null;
        None
    }

    // Java: Map<String, IFontFamily> getFontMap()
    pub fn getFontMap_impl(&self) -> HashMap<String, Arc<dyn IFontFamily>> {
        // Java: return this.mMap;
        self.mMap.clone()
    }

    // Java: ArrayList<String> getCJKName()
    pub fn getCJKName_impl(&self) -> Vec<String> {
        // Java: ArrayList<String> cjks = new ArrayList();
        let cjks: Vec<String> = Vec::new();
        // Java: for(String str : this.mMap.keySet())
        // Java: if (((IFontFamily)this.mMap.get(str)).getTypeface(0).isCJKFont())
        // Java: cjks.add(str);
        // Java: return cjks;
        cjks
    }

    // Java: IFontFamily getBuildinFontFamily(String familyName, IFontFamily sourceFontFamily)
    pub fn getBuildinFontFamily_impl(&self, _family_name: &str, source_font_family: Option<Arc<dyn IFontFamily>>) -> Option<Arc<dyn IFontFamily>> {
        // Java: if (!TypefaceFactory.hasBuildInFamily())
        // Java: return sourceFontFamily;
        // Java: else if (!this.needBuilInHerderForHReplaceFont(sourceFontFamily != null ? sourceFontFamily.getName() : null, familyName))
        // Java: return sourceFontFamily;
        // Java: else
        // Java: IFontFamily fontFamily = this.register(familyName, true);
        // Java: FontFamilyImpl buildInFontFamily = null;
        // Java: if (fontFamily != null && fontFamily instanceof FontFamilyImpl)
        // Java: buildInFontFamily = (FontFamilyImpl)fontFamily;
        // Java: if (null == sourceFontFamily)
        // Java: sourceFontFamily = this.getDefault();
        // Java: if (buildInFontFamily != null)
        // Java: buildInFontFamily.setBuildInfo(sourceFontFamily, true);
        // Java: return buildInFontFamily;
        source_font_family
    }

    // Java: String getReplaceCloudFontId(String fontName)
    pub fn getReplaceCloudFontId_impl(&self, font_name: &str) -> String {
        // Java: String[] strings = (String[])FONT_REPLACE_CLOUD.get(fontName);
        let guard = get_font_replace_cloud().lock().unwrap();
        let strings = guard.get(font_name).cloned();
        drop(guard);
        // Java: return strings != null && strings.length > 0 ? strings[0] : "";
        strings.and_then(|s| s.first().cloned()).unwrap_or_default()
    }

    // Java: String getReplaceCloudFontName(String fontName)
    pub fn getReplaceCloudFontName_impl(&self, font_name: &str) -> String {
        // Java: String[] strings = (String[])FONT_SERVER_REPLACE_CLOUD.get(fontName);
        let server_guard = get_font_server_replace_cloud().lock().unwrap();
        let server_strings = server_guard.get(font_name).cloned();
        drop(server_guard);
        // Java: if (strings != null && strings.length > 0)
        if let Some(ref ss) = server_strings {
            if !ss.is_empty() {
                // Java: return strings[0];
                return ss[0].clone();
            }
        }
        // Java: else
        // Java: strings = (String[])FONT_REPLACE_CLOUD.get(fontName);
        let guard = get_font_replace_cloud().lock().unwrap();
        let strings = guard.get(font_name).cloned();
        drop(guard);
        // Java: return strings != null && strings.length > 0 ? strings[1] : "";
        strings.and_then(|s| s.get(1).cloned()).unwrap_or_default()
    }

    // Java: void updateServerFontName(Map<String, String[]> serverCacheFontNames)
    pub fn updateServerFontName_impl(&self, server_cache_font_names: HashMap<String, Vec<String>>) {
        // Java: FONT_SERVER_REPLACE_CLOUD.clear();
        let mut server_guard = get_font_server_replace_cloud().lock().unwrap();
        server_guard.clear();
        // Java: FONT_SERVER_REPLACE_CLOUD.putAll(serverCacheFontNames);
        server_guard.extend(server_cache_font_names.clone());
        drop(server_guard);
        // Java: for(Map.Entry<String, String[]> entry : serverCacheFontNames.entrySet())
        let mut alias_guard = get_font_alias_names().lock().unwrap();
        for (main_name, server_alias_names) in &server_cache_font_names {
            // Java: String mainName = (String)entry.getKey();
            // Java: String[] serverAliaNames = (String[])entry.getValue();
            // Java: String[] localAliaNames = (String[])FONT_ALIAS_NAMES.get(mainName);
            let local_alias_names = alias_guard.get(main_name).cloned();
            // Java: if (localAliaNames != null && localAliaNames.length != 0)
            if let Some(ref local) = local_alias_names {
                if !local.is_empty() {
                    // Java: Set<String> tempAliaNames = new HashSet(Arrays.asList(localAliaNames));
                    let mut temp: std::collections::HashSet<String> = local.iter().cloned().collect();
                    // Java: tempAliaNames.addAll(Arrays.asList(serverAliaNames));
                    for s in server_alias_names {
                        temp.insert(s.clone());
                    }
                    // Java: FONT_ALIAS_NAMES.put(mainName, (String[])tempAliaNames.toArray(new String[tempAliaNames.size()]));
                    alias_guard.insert(main_name.clone(), temp.into_iter().collect());
                } else {
                    // Java: else
                    // Java: FONT_ALIAS_NAMES.put(mainName, serverAliaNames);
                    alias_guard.insert(main_name.clone(), server_alias_names.clone());
                }
            } else {
                // Java: else
                // Java: FONT_ALIAS_NAMES.put(mainName, serverAliaNames);
                alias_guard.insert(main_name.clone(), server_alias_names.clone());
            }
        }
    }

    // Java: List<String> getFontAliasNames(String fontName)
    pub fn getFontAliasNames_impl(&self, font_name: &str) -> Vec<String> {
        // Java: String[] alias = (String[])FONT_SERVER_REPLACE_CLOUD.get(fontName);
        let server_guard = get_font_server_replace_cloud().lock().unwrap();
        let alias = server_guard.get(font_name).cloned();
        drop(server_guard);
        // Java: if (alias != null)
        if let Some(a) = alias {
            // Java: return Arrays.asList(alias);
            return a;
        }
        // Java: else
        // Java: alias = (String[])FONT_REPLACE_CLOUD.get(fontName);
        let cloud_guard = get_font_replace_cloud().lock().unwrap();
        let alias2 = cloud_guard.get(font_name).cloned();
        drop(cloud_guard);
        // Java: if (alias != null && alias.length > 1)
        if let Some(ref a) = alias2 {
            if a.len() > 1 {
                // Java: return Arrays.asList(alias[1]);
                return vec![a[1].clone()];
            }
        }
        // Java: else
        // Java: alias = (String[])FONT_ALIAS_NAMES.get(fontName);
        let alias_guard = get_font_alias_names().lock().unwrap();
        let alias3 = alias_guard.get(font_name).cloned();
        drop(alias_guard);
        // Java: return alias != null ? Arrays.asList(alias) : null;
        alias3.unwrap_or_default()
    }

    // Java: Set<String> getNoSupportFonts()
    pub fn getNoSupportFonts_impl(&self) -> std::collections::HashSet<String> {
        // Java: return FONT_REPLACE_CLOUD.keySet();
        let guard = get_font_replace_cloud().lock().unwrap();
        guard.keys().cloned().collect()
    }

    // Java: boolean needBuilInHerderForHReplaceFont(String replaceName, String srcName)
    fn needBuilInHerderForHReplaceFont(&self, _replace_name: Option<&str>, _src_name: &str) -> bool {
        // Java: if (replaceName != null && srcName != null && replaceName.compareToIgnoreCase(srcName) == 0)
        // Java: return false;
        // Java: else if (BuildinTTFHeader.getFont(srcName) == null)
        // Java: return false;
        // Java: else if (replaceName == null)
        // Java: return true;
        // Java: else
        // Java: String[] replaceNames = (String[])FONT_REPLACE_NAMES.get(srcName);
        // Java: if (null != replaceNames)
        // Java: int i = 0;
        // Java: for(int n = replaceNames.length; i < n; ++i)
        // Java: if (replaceNames[i].compareToIgnoreCase(replaceName) == 0)
        // Java: return true;
        // Java: return false;
        false
    }

    // Java: void closeAllNativePtr()
    pub fn closeAllNativePtr_impl(&self) {
        // Java: long start = 0L;
        // Java: try
        // Java: IFontFamily[] fontFamilys = this.getFontFamilys();
        // Java: for(int i = 0; i < fontFamilys.length; ++i)
        // Java: IFontFamily fontFamily = fontFamilys[i];
        // Java: if (fontFamily != null)
        // Java: fontFamily.closeNativePtr();
        // Java: catch (Throwable var6)
        // Java: Log.d("FontTableImpl", "closeNativePtr exeption！");
    }
}

impl IFontTable for FontTableImpl {
    fn get_by_id(&self, var1: i32) -> Option<Arc<dyn IFontFamily>> { self.get_by_id_impl(var1) }
    fn get_by_id_bool(&self, var1: i32, var2: bool) -> Option<Arc<dyn IFontFamily>> { self.get_by_id_bool_impl(var1, var2) }
    fn get_by_name(&self, var1: &str) -> Option<Arc<dyn IFontFamily>> { self.get_by_name_impl(var1) }
    fn get_by_name_bool(&self, var1: &str, var2: bool) -> Option<Arc<dyn IFontFamily>> { self.get_by_name_bool_impl(var1, var2) }
    fn get_by_name_bool_bool(&self, var1: &str, var2: bool, var3: bool) -> Option<Arc<dyn IFontFamily>> { self.get_by_name_bool_bool_impl(var1, var2, var3) }
    fn getDefault(&self) -> Option<Arc<dyn IFontFamily>> { self.getDefault_impl() }
    fn getDefault_by_language(&self, var1: UILanguage) -> Option<Arc<dyn IFontFamily>> { self.getDefault_by_language_impl(var1) }
    fn getMathFontFamily(&self, var1: &str, var2: bool) -> Option<Arc<dyn IMathFontFamily>> { self.getMathFontFamily_impl(var1, var2) }
    fn register(&self, _var1: &str, _var2: bool) -> Option<Arc<dyn IFontFamily>> { None }
    fn getFontMap(&self) -> HashMap<String, Arc<dyn IFontFamily>> { self.getFontMap_impl() }
    fn getCJKName(&self) -> Vec<String> { self.getCJKName_impl() }
    fn getBuildinFontFamily(&self, var1: &str, var2: Arc<dyn IFontFamily>) -> Option<Arc<dyn IFontFamily>> { self.getBuildinFontFamily_impl(var1, Some(var2)) }
    fn getReplaceCloudFontId(&self, var1: &str) -> String { self.getReplaceCloudFontId_impl(var1) }
    fn getReplaceCloudFontName(&self, var1: &str) -> String { self.getReplaceCloudFontName_impl(var1) }
    fn getNoSupportFonts(&self) -> std::collections::HashSet<String> { self.getNoSupportFonts_impl() }
    fn updateServerFontName(&self, _var1: HashMap<String, Vec<String>>) {}
    fn getFontAliasNames(&self, var1: &str) -> Vec<String> { self.getFontAliasNames_impl(var1) }
    fn closeAllNativePtr(&self) { self.closeAllNativePtr_impl() }
}
