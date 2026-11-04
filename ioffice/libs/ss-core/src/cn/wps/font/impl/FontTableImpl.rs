use super::{FontFamilyImpl::FontFamilyImpl, TypefaceFactory::TypefaceFactory};
use crate::java_runtime::Rc;
use std::collections::HashMap;
// java class FontTableImpl implements IFontTable
pub struct FontTableImpl {
    pub mArray: Vec<Rc<FontFamilyImpl>>,
    pub mMap: HashMap<String, Rc<FontFamilyImpl>>,
    pub aliases: HashMap<String, Vec<String>>,
}
impl FontTableImpl {
    // java public FontTableImpl(boolean createDefault) {
    // java super();
    // java this.mArray = new ArrayList(127);
    // java this.mMap = new HashMap(127);
    // java if (createDefault) {
    // java FontFamilyImpl serif = (FontFamilyImpl)this.register("serif", true);
    // java ITypeface face = TypefaceFactory.createTypeface(serif, 0);
    // java serif.addTypeface(0, face);
    // java face = TypefaceFactory.createTypeface(serif, 0);
    // java ((AbstractTypeface)face).setNeedBold(true);
    // java serif.addTypeface(1, face);
    // java face = TypefaceFactory.createTypeface(serif, 0);
    // java ((AbstractTypeface)face).setNeedItalic(true);
    // java serif.addTypeface(2, face);
    // java face = TypefaceFactory.createTypeface(serif, 0);
    // java ((AbstractTypeface)face).setNeedBold(true);
    // java ((AbstractTypeface)face).setNeedItalic(true);
    // java serif.addTypeface(3, face);
    // java }
    // java
    // java }
    pub fn new(createDefault: bool) -> Self {
        let mut table = Self {
            mArray: vec![],
            mMap: HashMap::new(),
            aliases: Self::fontAliases(),
        };
        if createDefault {
            let serif = table.register("serif", true);
            for style in 0..4 {
                serif.addTypeface(style, TypefaceFactory::createTypeface(0, None));
            }
        }
        table
    }
    // java public IFontFamily register(String familyName, boolean isSystemFont) {
    // java if (familyName != null && familyName.length() != 0) {
    // java IFontFamily family = (IFontFamily)this.mMap.get(familyName);
    // java if (family != null) {
    // java return family;
    // java } else {
    // java int id = this.mArray.size();
    // java family = new FontFamilyImpl(familyName, isSystemFont, id);
    // java this.mArray.add(family);
    // java this.mMap.put(familyName, family);
    // java return family;
    // java }
    // java } else {
    // java return this.getDefault();
    // java }
    // java }
    pub fn register(&mut self, name: &str, system: bool) -> Rc<FontFamilyImpl> {
        if name.is_empty() {
            return self.getDefault();
        }
        if let Some(f) = self.mMap.get(name) {
            return f.clone();
        }
        let family = Rc::new(FontFamilyImpl::new(
            name.into(),
            system,
            self.mArray.len() as i32,
        ));
        self.mArray.push(family.clone());
        self.mMap.insert(name.into(), family.clone());
        family
    }
    // java public IFontFamily get(String familyName, boolean defaultIfNotExist, boolean replaceIfNotExist) {
    // java if (familyName.length() <= 0) {
    // java return this.getDefault();
    // java } else {
    // java IFontFamily fontFamily = this.doGetFromMap(familyName);
    // java if (fontFamily == null) {
    // java String s = FontNameDecoder.removeEncodingSuffix(familyName);
    // java fontFamily = this.doGetFromMap(s);
    // java }
    // java
    // java if (fontFamily != null) {
    // java return fontFamily;
    // java } else {
    // java String[] aliasNames = (String[])FONT_ALIAS_NAMES.get(familyName);
    // java if (null != aliasNames) {
    // java FontReplaceBindHelper helper = FontReplaceBindHelperFactory.getImpl();
    // java if (helper != null) {
    // java helper.sortArrayByBind(familyName, aliasNames);
    // java }
    // java
    // java int i = 0;
    // java
    // java for(int n = aliasNames.length; i < n; ++i) {
    // java fontFamily = this.doGetFromMap(aliasNames[i]);
    // java if (null != fontFamily) {
    // java return fontFamily;
    // java }
    // java }
    // java }
    // java
    // java return defaultIfNotExist ? this.getDefault() : null;
    // java }
    // java }
    // java }
    pub fn get(
        &self,
        name: &str,
        defaultIfNotExist: bool,
        _replaceIfNotExist: bool,
    ) -> Option<Rc<FontFamilyImpl>> {
        if name.is_empty() {
            return Some(self.getDefault());
        }
        let simple = super::FontNameDecoder::FontNameDecoder::removeEncodingSuffix(name);
        if let Some(f) = self.mMap.get(name).or_else(|| self.mMap.get(&simple)) {
            return Some(f.clone());
        }
        if let Some(aliases) = self.aliases.get(name) {
            for a in aliases {
                if let Some(f) = self.mMap.get(a) {
                    return Some(f.clone());
                }
            }
        }
        if defaultIfNotExist {
            Some(self.getDefault())
        } else {
            None
        }
    }
    // java public IFontFamily getDefault() {
    // java if (this.mArray.size() <= 0) {
    // java throw new RuntimeException("No fonts!!");
    // java } else {
    // java return (IFontFamily)this.mArray.get(0);
    // java }
    // java }
    pub fn getDefault(&self) -> Rc<FontFamilyImpl> {
        self.mArray.first().expect("No fonts!!").clone()
    }
    // java private static void resolveMultipleFontNames(HashMap<String, String[]> nameMap, String mainName, String... aliasNames) {
    // java if (null != nameMap && null != mainName && null != aliasNames && 0 != aliasNames.length) {
    // java String[] mappedNames = (String[])nameMap.get(mainName);
    // java int count = 0;
    // java count += aliasNames.length;
    // java count += null != mappedNames ? mappedNames.length : 0;
    // java String[] values = new String[count];
    // java System.arraycopy(aliasNames, 0, values, 0, aliasNames.length);
    // java if (null != mappedNames) {
    // java int len = mappedNames.length;
    // java System.arraycopy(mappedNames, 0, values, aliasNames.length, mappedNames.length);
    // java }
    // java
    // java nameMap.put(mainName, values);
    // java int i = 0;
    // java
    // java for(int n = aliasNames.length; i < n; ++i) {
    // java int idx = 0;
    // java values = new String[count];
    // java values[idx++] = mainName;
    // java if (null != mainName) {
    // java if (i > 0) {
    // java System.arraycopy(aliasNames, 0, values, idx, i);
    // java idx += i;
    // java }
    // java
    // java int tailLen = n - i - 1;
    // java if (tailLen > 0) {
    // java System.arraycopy(aliasNames, i + 1, values, idx, tailLen);
    // java idx += tailLen;
    // java }
    // java }
    // java
    // java if (null != mappedNames) {
    // java int len = mappedNames.length;
    // java System.arraycopy(mappedNames, 0, values, idx, len);
    // java int var10000 = idx + len;
    // java }
    // java
    // java nameMap.put(aliasNames[i], values);
    // java }
    // java
    // java }
    // java }
    fn resolveMultipleFontNames(
        map: &mut HashMap<String, Vec<String>>,
        name: &str,
        aliases: &[&str],
    ) {
        if aliases.is_empty() {
            return;
        }
        let mapped = map.get(name).cloned().unwrap_or_default();
        let mut values = aliases.iter().map(|v| v.to_string()).collect::<Vec<_>>();
        values.extend(mapped.clone());
        map.insert(name.into(), values);
        for (i, a) in aliases.iter().enumerate() {
            let mut values = vec![name.to_string()];
            values.extend(aliases[..i].iter().map(|v| v.to_string()));
            values.extend(aliases[i + 1..].iter().map(|v| v.to_string()));
            values.extend(mapped.clone());
            map.insert((*a).into(), values);
        }
    }
    fn fontAliases() -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();
        // java FONT_ALIAS_NAMES.put("Symbol", new String[]{"Kingsoft Sign"});
        map.insert("Symbol".into(), vec!["Kingsoft Sign".into()]);
        // java FONT_ALIAS_NAMES.put("MT Extra", new String[]{"Kingsoft Extra"});
        map.insert("MT Extra".into(), vec!["Kingsoft Extra".into()]);
        // java FONT_ALIAS_NAMES.put("Wingdings", new String[]{"Kingsoft Confetti"});
        map.insert("Wingdings".into(), vec!["Kingsoft Confetti".into()]);
        // java FONT_ALIAS_NAMES.put("Cambria", new String[]{"Kingsoft Math"});
        map.insert("Cambria".into(), vec!["Kingsoft Math".into()]);
        // java FONT_ALIAS_NAMES.put("Cambria Math", new String[]{"Kingsoft Math"});
        map.insert("Cambria Math".into(), vec!["Kingsoft Math".into()]);
        // java FONT_ALIAS_NAMES.put("Webdings", new String[]{"Kingsoft Mark"});
        map.insert("Webdings".into(), vec!["Kingsoft Mark".into()]);
        // java FONT_ALIAS_NAMES.put("Impact", new String[]{"Kingsoft Stress"});
        map.insert("Impact".into(), vec!["Kingsoft Stress".into()]);
        // java FONT_ALIAS_NAMES.put("宋体", simSun);
        map.insert(
            "宋体".into(),
            vec![
                "方正宋一".into(),
                "方正宋一简体".into(),
                "方正宋一_GBK".into(),
                "方正宋一_GB18030".into(),
                "方正书宋".into(),
                "方正书宋简体".into(),
                "方正书宋_GBK".into(),
                "方正书宋_GB18030".into(),
                "方正宋体S-超大字符集".into(),
            ],
        );
        // java FONT_ALIAS_NAMES.put("黑体", new String[]{"汉仪中黑简", "方正黑体", "方正黑体简体", "方正黑体_GBK", "方正黑体_GB18030", "方正兰亭黑", "方正兰亭黑简体", "方正兰亭黑_GBK", "方正兰亭黑_GB18030"});
        map.insert(
            "黑体".into(),
            vec![
                "汉仪中黑简".into(),
                "方正黑体".into(),
                "方正黑体简体".into(),
                "方正黑体_GBK".into(),
                "方正黑体_GB18030".into(),
                "方正兰亭黑".into(),
                "方正兰亭黑简体".into(),
                "方正兰亭黑_GBK".into(),
                "方正兰亭黑_GB18030".into(),
            ],
        );
        // java FONT_ALIAS_NAMES.put("仿宋", fangSong);
        map.insert(
            "仿宋".into(),
            vec![
                "汉仪仿宋简".into(),
                "方正仿宋".into(),
                "方正仿宋简体".into(),
                "方正仿宋_GBK".into(),
                "方正仿宋_GB18030".into(),
            ],
        );
        // java FONT_ALIAS_NAMES.put("楷体", kaiTi);
        map.insert(
            "楷体".into(),
            vec![
                "汉仪中楷简".into(),
                "方正楷体".into(),
                "方正楷体简体".into(),
                "方正楷体_GBK".into(),
                "方正楷体_GB18030".into(),
                "汉仪楷体简".into(),
            ],
        );
        // java FONT_ALIAS_NAMES.put("隶书", new String[]{"汉仪中隶书简", "方正隶书", "方正隶书简体", "方正隶书_GBK", "方正隶书_GB18030", "汉仪小隶书简", "汉仪中楷简"});
        map.insert(
            "隶书".into(),
            vec![
                "汉仪中隶书简".into(),
                "方正隶书".into(),
                "方正隶书简体".into(),
                "方正隶书_GBK".into(),
                "方正隶书_GB18030".into(),
                "汉仪小隶书简".into(),
                "汉仪中楷简".into(),
            ],
        );
        // java FONT_ALIAS_NAMES.put("微软雅黑", new String[]{"汉仪旗黑-55简"});
        map.insert("微软雅黑".into(), vec!["汉仪旗黑-55简".into()]);
        // java FONT_ALIAS_NAMES.put("新宋体", new String[]{"汉仪报宋简"});
        map.insert("新宋体".into(), vec!["汉仪报宋简".into()]);
        // java FONT_ALIAS_NAMES.put("幼圆", new String[]{"汉仪细圆简"});
        map.insert("幼圆".into(), vec!["汉仪细圆简".into()]);
        // java FONT_ALIAS_NAMES.put("华文宋体", new String[]{"汉仪报宋简", "宋体-方正超大字符集", "方正宋体S-超大字符集(SIP)"});
        map.insert(
            "华文宋体".into(),
            vec![
                "汉仪报宋简".into(),
                "宋体-方正超大字符集".into(),
                "方正宋体S-超大字符集(SIP)".into(),
            ],
        );
        // java FONT_ALIAS_NAMES.put("华文细黑", new String[]{"汉仪中等线简"});
        map.insert("华文细黑".into(), vec!["汉仪中等线简".into()]);
        // java FONT_ALIAS_NAMES.put("华文楷体", new String[]{"汉仪楷体简", "方正楷体", "方正楷体简体", "方正楷体_GB2312", "方正楷体_GB18030", "方正楷体_GBK"});
        map.insert(
            "华文楷体".into(),
            vec![
                "汉仪楷体简".into(),
                "方正楷体".into(),
                "方正楷体简体".into(),
                "方正楷体_GB2312".into(),
                "方正楷体_GB18030".into(),
                "方正楷体_GBK".into(),
            ],
        );
        // java FONT_ALIAS_NAMES.put("华文仿宋", new String[]{"汉仪仿宋简", "方正仿宋", "方正仿宋简体", "方正仿宋_GB2312", "方正仿宋_GB18030", "方正仿宋_GBK"});
        map.insert(
            "华文仿宋".into(),
            vec![
                "汉仪仿宋简".into(),
                "方正仿宋".into(),
                "方正仿宋简体".into(),
                "方正仿宋_GB2312".into(),
                "方正仿宋_GB18030".into(),
                "方正仿宋_GBK".into(),
            ],
        );
        // java FONT_ALIAS_NAMES.put("华文中宋", new String[]{"汉仪中宋简"});
        map.insert("华文中宋".into(), vec!["汉仪中宋简".into()]);
        // java FONT_ALIAS_NAMES.put("华文新魏", new String[]{"汉仪魏碑简", "方正魏碑", "方正魏碑简体", "方正魏碑_GB2312", "方正魏碑_GB18030", "方正魏碑_GBK"});
        map.insert(
            "华文新魏".into(),
            vec![
                "汉仪魏碑简".into(),
                "方正魏碑".into(),
                "方正魏碑简体".into(),
                "方正魏碑_GB2312".into(),
                "方正魏碑_GB18030".into(),
                "方正魏碑_GBK".into(),
            ],
        );
        // java FONT_ALIAS_NAMES.put("Courier", new String[]{"Courier New"});
        map.insert("Courier".into(), vec!["Courier New".into()]);
        // java FONT_ALIAS_NAMES.put("Basemic Symbol", new String[]{"Liberation Mono"});
        map.insert("Basemic Symbol".into(), vec!["Liberation Mono".into()]);
        // java FONT_ALIAS_NAMES.put("DFBiaoSong-B5", new String[]{"DFPBiaoSong-B5"});
        map.insert("DFBiaoSong-B5".into(), vec!["DFPBiaoSong-B5".into()]);
        // java FONT_ALIAS_NAMES.put("ＭＳ 明朝", new String[]{"MS Mincho"});
        map.insert("ＭＳ 明朝".into(), vec!["MS Mincho".into()]);
        // java FONT_ALIAS_NAMES.put("细明体", new String[]{"MingLiU", "汉仪报宋简"});
        map.insert("细明体".into(), vec!["MingLiU".into(), "汉仪报宋简".into()]);
        // java FONT_ALIAS_NAMES.put("新细明体", new String[]{"PMingLiU", "汉仪报宋简"});
        map.insert(
            "新细明体".into(),
            vec!["PMingLiU".into(), "汉仪报宋简".into()],
        );
        // java FONT_ALIAS_NAMES.put("Times", new String[]{"Times New Roman"});
        map.insert("Times".into(), vec!["Times New Roman".into()]);
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "仿宋", "仿宋_GB2312", "FangSong", "方正仿宋", "方正仿宋简体", "方正仿宋_GB2312", "方正仿宋_GB18030", "方正仿宋_GBK");
        Self::resolveMultipleFontNames(
            &mut map,
            "仿宋",
            &[
                "仿宋_GB2312",
                "FangSong",
                "方正仿宋",
                "方正仿宋简体",
                "方正仿宋_GB2312",
                "方正仿宋_GB18030",
                "方正仿宋_GBK",
            ],
        );
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "楷体", "楷体_GB2312", "KaiTi", "标楷体", "標楷體", "华文楷体", "方正楷体", "方正楷体简体", "方正楷体_GB2312", "方正楷体_GB18030", "方正楷体_GBK", "方正新楷体_GBK");
        Self::resolveMultipleFontNames(
            &mut map,
            "楷体",
            &[
                "楷体_GB2312",
                "KaiTi",
                "标楷体",
                "標楷體",
                "华文楷体",
                "方正楷体",
                "方正楷体简体",
                "方正楷体_GB2312",
                "方正楷体_GB18030",
                "方正楷体_GBK",
                "方正新楷体_GBK",
            ],
        );
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "宋体", "SimSun");
        Self::resolveMultipleFontNames(&mut map, "宋体", &["SimSun"]);
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "新宋体", "NSimSun");
        Self::resolveMultipleFontNames(&mut map, "新宋体", &["NSimSun"]);
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "幼圆", "YouYuan");
        Self::resolveMultipleFontNames(&mut map, "幼圆", &["YouYuan"]);
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "华文宋体", "STSong");
        Self::resolveMultipleFontNames(&mut map, "华文宋体", &["STSong"]);
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "华文细黑", "STXihei");
        Self::resolveMultipleFontNames(&mut map, "华文细黑", &["STXihei"]);
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "华文楷体", "STKaiti");
        Self::resolveMultipleFontNames(&mut map, "华文楷体", &["STKaiti"]);
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "华文仿宋", "STFangsong");
        Self::resolveMultipleFontNames(&mut map, "华文仿宋", &["STFangsong"]);
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "华文中宋", "STZhongsong");
        Self::resolveMultipleFontNames(&mut map, "华文中宋", &["STZhongsong"]);
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "华文新魏", "STXinWei");
        Self::resolveMultipleFontNames(&mut map, "华文新魏", &["STXinWei"]);
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "细明体", "細明體");
        Self::resolveMultipleFontNames(&mut map, "细明体", &["細明體"]);
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "新细明体", "新細明體");
        Self::resolveMultipleFontNames(&mut map, "新细明体", &["新細明體"]);
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "微软雅黑", "微软雅黑体", "微軟正黑體");
        Self::resolveMultipleFontNames(&mut map, "微软雅黑", &["微软雅黑体", "微軟正黑體"]);
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "书宋", "文鼎书宋", "方正书宋", "方正书宋简体", "方正书宋_GB2312", "方正书宋_GB18030", "方正书宋_GBK");
        Self::resolveMultipleFontNames(
            &mut map,
            "书宋",
            &[
                "文鼎书宋",
                "方正书宋",
                "方正书宋简体",
                "方正书宋_GB2312",
                "方正书宋_GB18030",
                "方正书宋_GBK",
            ],
        );
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "方正小标宋", "方正小标宋简体", "方正小标宋_GB2312", "方正小标宋_GB18030", "方正小标宋_GBK");
        Self::resolveMultipleFontNames(
            &mut map,
            "方正小标宋",
            &[
                "方正小标宋简体",
                "方正小标宋_GB2312",
                "方正小标宋_GB18030",
                "方正小标宋_GBK",
            ],
        );
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "方正兰亭黑", "方正兰亭黑简体", "方正兰亭黑_GB2312", "方正兰亭黑_GB18030", "方正兰亭黑_GBK");
        Self::resolveMultipleFontNames(
            &mut map,
            "方正兰亭黑",
            &[
                "方正兰亭黑简体",
                "方正兰亭黑_GB2312",
                "方正兰亭黑_GB18030",
                "方正兰亭黑_GBK",
            ],
        );
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "方正兰亭细黑", "方正兰亭细黑简体", "方正兰亭细黑_GB2312", "方正兰亭细黑_GB18030", "方正兰亭细黑_GBK");
        Self::resolveMultipleFontNames(
            &mut map,
            "方正兰亭细黑",
            &[
                "方正兰亭细黑简体",
                "方正兰亭细黑_GB2312",
                "方正兰亭细黑_GB18030",
                "方正兰亭细黑_GBK",
            ],
        );
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "方正兰亭大黑", "方正兰亭大黑简体", "方正兰亭大黑_GB2312", "方正兰亭大黑_GB18030", "方正兰亭大黑_GBK");
        Self::resolveMultipleFontNames(
            &mut map,
            "方正兰亭大黑",
            &[
                "方正兰亭大黑简体",
                "方正兰亭大黑_GB2312",
                "方正兰亭大黑_GB18030",
                "方正兰亭大黑_GBK",
            ],
        );
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "方正行楷", "方正行楷简体", "方正行楷_GB2312", "方正行楷_GB18030", "方正行楷_GBK");
        Self::resolveMultipleFontNames(
            &mut map,
            "方正行楷",
            &[
                "方正行楷简体",
                "方正行楷_GB2312",
                "方正行楷_GB18030",
                "方正行楷_GBK",
            ],
        );
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "方正黑体", "方正黑体简体", "方正黑体_GB2312", "方正黑体_GB18030", "方正黑体_GBK");
        Self::resolveMultipleFontNames(
            &mut map,
            "方正黑体",
            &[
                "方正黑体简体",
                "方正黑体_GB2312",
                "方正黑体_GB18030",
                "方正黑体_GBK",
            ],
        );
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "方正隶书", "方正隶书简体", "方正隶书_GB2312", "方正隶书_GB18030", "方正隶书_GBK");
        Self::resolveMultipleFontNames(
            &mut map,
            "方正隶书",
            &[
                "方正隶书简体",
                "方正隶书_GB2312",
                "方正隶书_GB18030",
                "方正隶书_GBK",
            ],
        );
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "方正魏碑", "方正魏碑简体", "方正魏碑_GB2312", "方正魏碑_GB18030", "方正魏碑_GBK");
        Self::resolveMultipleFontNames(
            &mut map,
            "方正魏碑",
            &[
                "方正魏碑简体",
                "方正魏碑_GB2312",
                "方正魏碑_GB18030",
                "方正魏碑_GBK",
            ],
        );
        // java resolveMultipleFontNames(FONT_ALIAS_NAMES, "方正宋体S-超大字符集", "宋体-方正超大字符集", "方正宋体S-超大字符集(SIP)");
        Self::resolveMultipleFontNames(
            &mut map,
            "方正宋体S-超大字符集",
            &["宋体-方正超大字符集", "方正宋体S-超大字符集(SIP)"],
        );
        map
    }
}
