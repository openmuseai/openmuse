// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.util.SearchUtil
use crate::cn::wps::moffice::util::SearchUtil::*;
// Java: import java.util.Arrays
use stubs::java::util::Arrays::*;
// Java: import cn.wps.moffice.writer.core.DocumentListener
use crate::cn::wps::moffice::writer::core::DocumentListener::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.DocumentEvent
use crate::cn::wps::moffice::writer::core::DocumentEvent::*;

// java static char[] PUNCTS = new char[]{'!', '%', ')', ',', '.', ':', ';', '>', '?', ']', '}', '¢', '¨', '°', '·', 'ˇ', 'ˉ', '―', '‖', '’', '”', '…', '‰', '′', '″', '›', '℃', '∶', '、', '。', '〃', '〉', '》', '」', '』', '】', '〕', '〗', '〞', '︶', '︺', '︾', '﹀', '﹄', '﹚', '﹜', '﹞', '！', '＂', '％', '＇', '）', '，', '．', '：', '；', '？', '］', '｀', '｜', '｝', '～', '￠', '$', '(', '[', '{', '£', '¥', '·', '‘', '“', '〈', '《', '「', '『', '【', '〔', '〖', '〝', '﹙', '﹛', '﹝', '＄', '（', '．', '［', '｛', '￡', '￥'};
pub(crate) static PUNCTS: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| { let mut __v = vec!['!', '%', ')', ',', '.', ':', ';', '>', '?', ']', '}', '¢', '¨', '°', '·', 'ˇ', 'ˉ', '―', '‖', '’', '”', '…', '‰', '′', '″', '›', '℃', '∶', '、', '。', '〃', '〉', '》', '」', '』', '】', '〕', '〗', '〞', '︶', '︺', '︾', '﹀', '﹄', '﹚', '﹜', '﹞', '！', '＂', '％', '＇', '）', '，', '．', '：', '；', '？', '］', '｀', '｜', '｝', '～', '￠', '$', '(', '[', '{', '£', '¥', '·', '‘', '“', '〈', '《', '「', '『', '【', '〔', '〖', '〝', '﹙', '﹛', '﹝', '＄', '（', '．', '［', '｛', '￡', '￥']; __v.sort(); __v });
// java static char[] ENDPUNCT_ZH_CN = new char[]{'!', '%', ')', ',', '.', ':', ';', '>', '?', ']', '}', '¢', '¨', '°', '·', 'ˇ', 'ˉ', '―', '‖', '’', '”', '…', '‰', '′', '″', '›', '℃', '∶', '、', '。', '〃', '〉', '》', '」', '』', '】', '〕', '〗', '〞', '︶', '︺', '︾', '﹀', '﹄', '﹚', '﹜', '﹞', '！', '＂', '％', '＇', '）', '，', '．', '：', '；', '？', '］', '｀', '｜', '｝', '～', '￠'};
pub(crate) static ENDPUNCT_ZH_CN: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| { let mut __v = vec!['!', '%', ')', ',', '.', ':', ';', '>', '?', ']', '}', '¢', '¨', '°', '·', 'ˇ', 'ˉ', '―', '‖', '’', '”', '…', '‰', '′', '″', '›', '℃', '∶', '、', '。', '〃', '〉', '》', '」', '』', '】', '〕', '〗', '〞', '︶', '︺', '︾', '﹀', '﹄', '﹚', '﹜', '﹞', '！', '＂', '％', '＇', '）', '，', '．', '：', '；', '？', '］', '｀', '｜', '｝', '～', '￠']; __v.sort(); __v });
// java static char[] STARTPUNCT_ZH_CN = new char[]{'$', '(', '[', '{', '£', '¥', '·', '‘', '“', '〈', '《', '「', '『', '【', '〔', '〖', '〝', '﹙', '﹛', '﹝', '＄', '（', '．', '［', '｛', '￡', '￥'};
pub(crate) static STARTPUNCT_ZH_CN: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| { let mut __v = vec!['$', '(', '[', '{', '£', '¥', '·', '‘', '“', '〈', '《', '「', '『', '【', '〔', '〖', '〝', '﹙', '﹛', '﹝', '＄', '（', '．', '［', '｛', '￡', '￥']; __v.sort(); __v });
// java static char[] ENDPUNCT_ZH_TW = new char[]{'!', ')', ',', '.', ':', ';', '?', ']', '}', '¢', '·', '–', '—', '’', '”', '•', '‥', '…', '‧', '′', '╴', '、', '。', '〉', '》', '」', '』', '】', '〕', '〞', '︰', '︱', '︳', '︴', '︶', '︸', '︺', '︼', '︾', '﹀', '﹂', '﹄', '﹏', '﹐', '﹑', '﹒', '﹔', '﹕', '﹖', '﹗', '﹚', '﹜', '﹞', '！', '）', '，', '．', '：', '；', '？', '］', '｜', '｝', '､'};
pub(crate) static ENDPUNCT_ZH_TW: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| { let mut __v = vec!['!', ')', ',', '.', ':', ';', '?', ']', '}', '¢', '·', '–', '—', '’', '”', '•', '‥', '…', '‧', '′', '╴', '、', '。', '〉', '》', '」', '』', '】', '〕', '〞', '︰', '︱', '︳', '︴', '︶', '︸', '︺', '︼', '︾', '﹀', '﹂', '﹄', '﹏', '﹐', '﹑', '﹒', '﹔', '﹕', '﹖', '﹗', '﹚', '﹜', '﹞', '！', '）', '，', '．', '：', '；', '？', '］', '｜', '｝', '､']; __v.sort(); __v });
// java static char[] STARTPUNCT_ZH_TW = new char[]{'(', '[', '{', '£', '¥', '‘', '“', '‵', '〈', '《', '「', '『', '【', '〔', '〝', '︵', '︷', '︹', '︻', '︽', '︿', '﹁', '﹃', '﹙', '﹛', '﹝', '（', '｛'};
pub(crate) static STARTPUNCT_ZH_TW: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| { let mut __v = vec!['(', '[', '{', '£', '¥', '‘', '“', '‵', '〈', '《', '「', '『', '【', '〔', '〝', '︵', '︷', '︹', '︻', '︽', '︿', '﹁', '﹃', '﹙', '﹛', '﹝', '（', '｛']; __v.sort(); __v });
// java static char[] ENDPUNCT_JA_JP = new char[]{'!', '%', ')', ',', '.', ':', ';', '?', ']', '}', '¢', '°', '’', '”', '‰', '′', '″', '℃', '、', '。', '々', '〉', '》', '」', '』', '】', '〕', '゛', '゜', 'ゝ', 'ゞ', '・', 'ヽ', 'ヾ', '！', '％', '）', '，', '．', '：', '；', '？', '］', '｝', '｡', '｣', '､', '･', 'ﾞ', 'ﾟ', '￠'};
pub(crate) static ENDPUNCT_JA_JP: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| { let mut __v = vec!['!', '%', ')', ',', '.', ':', ';', '?', ']', '}', '¢', '°', '’', '”', '‰', '′', '″', '℃', '、', '。', '々', '〉', '》', '」', '』', '】', '〕', '゛', '゜', 'ゝ', 'ゞ', '・', 'ヽ', 'ヾ', '！', '％', '）', '，', '．', '：', '；', '？', '］', '｝', '｡', '｣', '､', '･', 'ﾞ', 'ﾟ', '￠']; __v.sort(); __v });
// java static char[] STARTPUNCT_JA_JP = new char[]{'$', '(', '[', '\\', '{', '£', '¥', '‘', '“', '〈', '《', '「', '『', '【', '〔', '＄', '（', '［', '｛', '｢', '￡', '￥'};
pub(crate) static STARTPUNCT_JA_JP: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| { let mut __v = vec!['$', '(', '[', '\\', '{', '£', '¥', '‘', '“', '〈', '《', '「', '『', '【', '〔', '＄', '（', '［', '｛', '｢', '￡', '￥']; __v.sort(); __v });
// java static char[] ENDPUNCT_KO_KR = new char[]{'!', '%', ')', ',', '.', ':', ';', '?', ']', '}', '¢', '°', '’', '”', '′', '″', '℃', '〉', '》', '」', '』', '】', '〕', '！', '％', '）', '，', '．', '：', '；', '？', '］', '｝', '￠'};
pub(crate) static ENDPUNCT_KO_KR: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| { let mut __v = vec!['!', '%', ')', ',', '.', ':', ';', '?', ']', '}', '¢', '°', '’', '”', '′', '″', '℃', '〉', '》', '」', '』', '】', '〕', '！', '％', '）', '，', '．', '：', '；', '？', '］', '｝', '￠']; __v.sort(); __v });
// java static char[] STARTPUNCT_KO_KR = new char[]{'$', '(', '[', '\\', '{', '£', '¥', '‘', '“', '〈', '《', '「', '『', '【', '〔', '＄', '（', '［', '｛', '￡', '￥', '￦'};
pub(crate) static STARTPUNCT_KO_KR: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| { let mut __v = vec!['$', '(', '[', '\\', '{', '£', '¥', '‘', '“', '〈', '《', '「', '『', '【', '〔', '＄', '（', '［', '｛', '￡', '￥', '￦']; __v.sort(); __v });
// java static char[] ENDPUNCT_JA_JP_EX = new char[]{'!', '%', ')', ',', '.', ':', ';', '?', ']', '}', '¢', '°', '’', '”', '‰', '′', '″', '℃', '、', '。', '々', '〉', '》', '」', '』', '】', '〕', 'ぁ', 'ぃ', 'ぅ', 'ぇ', 'ぉ', 'っ', 'ゃ', 'ゅ', 'ょ', 'ゎ', '゛', '゜', 'ゝ', 'ゞ', 'ァ', 'ィ', 'ゥ', 'ェ', 'ォ', 'ッ', 'ャ', 'ュ', 'ョ', 'ヮ', 'ヵ', 'ヶ', '・', 'ー', 'ヽ', 'ヾ', '！', '％', '）', '，', '．', '：', '；', '？', '］', '｝', '｡', '｣', '､', '･', 'ｧ', 'ｨ', 'ｩ', 'ｪ', 'ｫ', 'ｬ', 'ｭ', 'ｮ', 'ｯ', 'ｰ', 'ﾞ', 'ﾟ', '￠'};
pub(crate) static ENDPUNCT_JA_JP_EX: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| { let mut __v = vec!['!', '%', ')', ',', '.', ':', ';', '?', ']', '}', '¢', '°', '’', '”', '‰', '′', '″', '℃', '、', '。', '々', '〉', '》', '」', '』', '】', '〕', 'ぁ', 'ぃ', 'ぅ', 'ぇ', 'ぉ', 'っ', 'ゃ', 'ゅ', 'ょ', 'ゎ', '゛', '゜', 'ゝ', 'ゞ', 'ァ', 'ィ', 'ゥ', 'ェ', 'ォ', 'ッ', 'ャ', 'ュ', 'ョ', 'ヮ', 'ヵ', 'ヶ', '・', 'ー', 'ヽ', 'ヾ', '！', '％', '）', '，', '．', '：', '；', '？', '］', '｝', '｡', '｣', '､', '･', 'ｧ', 'ｨ', 'ｩ', 'ｪ', 'ｫ', 'ｬ', 'ｭ', 'ｮ', 'ｯ', 'ｰ', 'ﾞ', 'ﾟ', '￠']; __v.sort(); __v });
// java static char[] STARTPUNCT_JA_JP_EX = new char[]{'$', '(', '[', '\\', '{', '£', '¥', '‘', '“', '〈', '《', '「', '『', '【', '〔', '＄', '（', '［', '｛', '｢', '￡', '￥'};
pub(crate) static STARTPUNCT_JA_JP_EX: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| { let mut __v = vec!['$', '(', '[', '\\', '{', '£', '¥', '‘', '“', '〈', '《', '「', '『', '【', '〔', '＄', '（', '［', '｛', '｢', '￡', '￥']; __v.sort(); __v });

// java public final class AsiaTypography implements DocumentListener
pub struct AsiaTypography {
    // java protected TextDocument mMainDocument = null;
    // rust: Java 对象引用**别名/回指**宿主文档；Rust 若按值/Box 持有一份副本，
    //   `init()` 读到的将是**陈旧属性**（文档是宿主 `TextDocument`）⇒ 必须裸指针。
    pub(crate) mMainDocument: Option<*mut TextDocument>,
    // java protected int mIJustification = 0;
    pub(crate) mIJustification: i32,
    // java protected int mILevelOfKinsoku = 0;
    pub(crate) mILevelOfKinsoku: i32,
    // java protected int mStartPunctLang = 2;
    pub(crate) mStartPunctLang: i32,
    // java protected int mEndPunctLang = 2;
    pub(crate) mEndPunctLang: i32,
    // java protected String mStrStartPunctCustom = null;
    pub(crate) mStrStartPunctCustom: Option<String>,
    // java protected String mStrEndPunctCustom = null;
    pub(crate) mStrEndPunctCustom: Option<String>,
    // java protected char[][] mStartPuncts = null;
    // rust: Java `char[][]` 的元素本身可为 null（`isLineStartPunct` 有 `mStartPuncts[2] == null` 判定）
    //   ⇒ 内层同样用 `Option`。
    pub(crate) mStartPuncts: Option<Vec<Option<Vec<char>>>>,
    // java protected char[][] mEndPuncts = null;
    pub(crate) mEndPuncts: Option<Vec<Option<Vec<char>>>>,
    // java protected char[] mPuncts = null;
    pub(crate) mPuncts: Option<Vec<char>>,
    // java protected int mStartLang = 0;
    pub(crate) mStartLang: i32,
    // java protected int mEndLang = 0;
    pub(crate) mEndLang: i32,
    // java protected boolean mDoNotUseEastAsianBreakRules = false;
    pub(crate) mDoNotUseEastAsianBreakRules: bool,
}

// java public interface DocumentListener
impl DocumentListener for AsiaTypography {
    // java public void update(DocumentEvent e)
    fn update(&mut self, e: &DocumentEvent) {
      // java if (4 == e.getType())
      if 4 == e.getType() {
        // java this.init();
        self.init();
      }
    }

}

// rust: ★ Java 只有 `AsiaTypography(TextDocument)` 一个构造器，无默认构造。本移植为
//   **测试与容器**提供 `Default`，语义 = 「Java ctor 完成、`init()` 之前」的状态：
//   `mStartPuncts`/`mEndPuncts` = `new char[5][]` + `initKinsoku()`，`mPuncts` = `PUNCTS`，
//   语言码取 Java 字段初值（`mStartPunctLang`/`mEndPunctLang` = 2），文档回指为空。
impl Default for AsiaTypography {
    fn default() -> Self {
        let mut me = Self {
            mMainDocument: None,
            mIJustification: 0,
            mILevelOfKinsoku: 0,
            mStartPunctLang: 2,
            mEndPunctLang: 2,
            mStrStartPunctCustom: None,
            mStrEndPunctCustom: None,
            mStartPuncts: Some(vec![None; 5usize]),
            mEndPuncts: Some(vec![None; 5usize]),
            mPuncts: Some(PUNCTS.clone()),
            mStartLang: 0,
            mEndLang: 0,
            mDoNotUseEastAsianBreakRules: false,
        };
        me.initKinsoku();
        me
    }
}

// java public final class AsiaTypography implements DocumentListener
impl AsiaTypography {
    // java public  AsiaTypography(TextDocument textDocument)
    pub fn AsiaTypography(textDocument: &TextDocument) -> Self {
      // java Arrays.sort(STARTPUNCT_ZH_CN);
      // java Arrays.sort(ENDPUNCT_ZH_CN);
      // java Arrays.sort(STARTPUNCT_ZH_TW);
      // java Arrays.sort(ENDPUNCT_ZH_TW);
      // java Arrays.sort(STARTPUNCT_JA_JP);
      // java Arrays.sort(ENDPUNCT_JA_JP);
      // java Arrays.sort(STARTPUNCT_KO_KR);
      // java Arrays.sort(ENDPUNCT_KO_KR);
      // java Arrays.sort(PUNCTS);
      // rust: ★ Java 这 9 次 `Arrays.sort` 是就地排序**静态数组**。Rust `static` 不可变，
      //   故把排序内联进各 `LazyLock` 的初始化闭包（进程内仅执行一次），语义等价且无数据竞争。
      // java this.mPuncts = PUNCTS;
      // java this.mStartPuncts = new char[5][];
      // java this.mEndPuncts = new char[5][];
      let mut this = Self { mMainDocument: Some(textDocument as *const TextDocument as *mut TextDocument), mIJustification: 0, mILevelOfKinsoku: 0, mStartPunctLang: 2, mEndPunctLang: 2, mStrStartPunctCustom: None, mStrEndPunctCustom: None, mStartPuncts: Some(vec![None; 5usize]), mEndPuncts: Some(vec![None; 5usize]), mPuncts: Some(PUNCTS.clone()), mStartLang: 0, mEndLang: 0, mDoNotUseEastAsianBreakRules: false };
      // java this.initKinsoku();
      this.initKinsoku();
      // java textDocument.getMainDocument().getDispatcher().addDocumentListener(this);
      // rust: ★ Java 在 ctor 内把 `this` 注册为文档监听器；Rust 无 ctor 内自引用 ⇒
      //   注册改由宿主 `TextDocument::getAsiaTypography()` 在装入字段后完成（见该函数）。
      // java this.mMainDocument = textDocument;
      this
    }

    // java public boolean getDoNotUseEastAsianBreakRules()
    pub fn getDoNotUseEastAsianBreakRules(&self) -> bool {
      // java return this.mDoNotUseEastAsianBreakRules;
      return self.mDoNotUseEastAsianBreakRules;
    }

    // java final void initKinsoku()
    pub fn initKinsoku(&mut self) {
      // java this.mStartPuncts[2] = STARTPUNCT_ZH_CN;
      self.mStartPuncts.as_mut().unwrap()[2usize] = Some(STARTPUNCT_ZH_CN.clone());
      // java this.mStartPuncts[4] = STARTPUNCT_ZH_TW;
      self.mStartPuncts.as_mut().unwrap()[4usize] = Some(STARTPUNCT_ZH_TW.clone());
      // java this.mStartPuncts[1] = STARTPUNCT_JA_JP;
      self.mStartPuncts.as_mut().unwrap()[1usize] = Some(STARTPUNCT_JA_JP.clone());
      // java this.mStartPuncts[3] = STARTPUNCT_KO_KR;
      self.mStartPuncts.as_mut().unwrap()[3usize] = Some(STARTPUNCT_KO_KR.clone());
      // java this.mEndPuncts[2] = ENDPUNCT_ZH_CN;
      self.mEndPuncts.as_mut().unwrap()[2usize] = Some(ENDPUNCT_ZH_CN.clone());
      // java this.mEndPuncts[4] = ENDPUNCT_ZH_TW;
      self.mEndPuncts.as_mut().unwrap()[4usize] = Some(ENDPUNCT_ZH_TW.clone());
      // java this.mEndPuncts[1] = ENDPUNCT_JA_JP;
      self.mEndPuncts.as_mut().unwrap()[1usize] = Some(ENDPUNCT_JA_JP.clone());
      // java this.mEndPuncts[3] = ENDPUNCT_KO_KR;
      self.mEndPuncts.as_mut().unwrap()[3usize] = Some(ENDPUNCT_KO_KR.clone());
    }

    // java private static boolean isInPunctsList(char[][] punctsList, char ch)
    pub fn isInPunctsList(punctsList: Option<&Vec<Option<Vec<char>>>>, ch: char) -> bool {
      // java if (null == punctsList)
      if punctsList.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java int i = 0;
        let mut i: i32 = 0;
        let n: i32 = (punctsList.as_ref().unwrap().len() as i32);
        // java for(int n = punctsList.length; i < n; ++i)
        while i < n {
          // java char[] puncts = punctsList[i];
          let puncts: Option<&Vec<char>> = punctsList.as_ref().unwrap()[(i) as usize].as_ref();
          // java if (null != puncts && Arrays.binarySearch(puncts, ch) >= 0)
          if puncts.is_some() && SearchUtil::binarySearch__chararr_char(puncts.unwrap(), ch) >= 0 {
            // java return true;
            return true;
          }
          i += 1;
        }
        // java return false;
        return false;
      }
    }

    // java public boolean isLineStartOrLineEndPunct(char ch)
    pub fn isLineStartOrLineEndPunct(&self, ch: char) -> bool {
      // java if (isInPunctsList(this.mStartPuncts, ch))
      if Self::isInPunctsList(self.mStartPuncts.as_ref(), ch) {
        // java return true;
        return true;
      }
      // java else
      else {
        // java return isInPunctsList(this.mEndPuncts, ch);
        return Self::isInPunctsList(self.mEndPuncts.as_ref(), ch);
      }
    }

    // java public void init()
    pub fn init(&mut self) {
      // java PropertySet docProp = this.mMainDocument.getProperty();
      // rust: `mMainDocument` 回指宿主文档（裸指针）⇒ 解引用读取**当前**属性，而非陈旧副本。
      let mut docProp: PropertySet = unsafe { (*self.mMainDocument.unwrap()).getProperty() };
      // java this.mIJustification = docProp.getInt(382, 0);
      self.mIJustification = docProp.getInt__int_int(382, 0);
      // java int iKinsoku = docProp.getInt(384, 0);
      let mut iKinsoku: i32 = docProp.getInt__int_int(384, 0);
      // java this.initKinsoku();
      self.initKinsoku();
      // java this.mStartLang = docProp.getInt(381, 0);
      self.mStartLang = docProp.getInt__int_int(381, 0);
      // java this.mEndLang = docProp.getInt(379, 0);
      self.mEndLang = docProp.getInt__int_int(379, 0);
      // java this.mDoNotUseEastAsianBreakRules = docProp.getBoolean(522, false);
      self.mDoNotUseEastAsianBreakRules = docProp.getBoolean__int_boolean(522, false);
      // java if (iKinsoku != 0)
      if iKinsoku != 0 {
        // java if (iKinsoku == 1)
        if iKinsoku == 1 {
          // java this.mStartPuncts[1] = STARTPUNCT_JA_JP_EX;
          self.mStartPuncts.as_mut().unwrap()[1usize] = Some(STARTPUNCT_JA_JP_EX.clone());
          // java this.mEndPuncts[1] = ENDPUNCT_JA_JP_EX;
          self.mEndPuncts.as_mut().unwrap()[1usize] = Some(ENDPUNCT_JA_JP_EX.clone());
        }
        // java else if (iKinsoku == 2)
        else if iKinsoku == 2 {
          // java String strStartPunct = docProp.getString(380, (String)null);
          // rust: Java 默认值传 `null` 表示「缺省即空」；Rust 门面 `getString__int_string` 取 `&str`，
          //   故把空串归一为 `None` 以保持 `null != strStartPunct` 判定语义。
          let mut strStartPunct: Option<String> = Some(docProp.getString__int_string(380, "")).filter(|v| !v.is_empty());
          // java String strEndPunct = docProp.getString(378, (String)null);
          let mut strEndPunct: Option<String> = Some(docProp.getString__int_string(378, "")).filter(|v| !v.is_empty());
          // java if (this.mStartPunctLang != this.mStartLang || null != strStartPunct)
          if self.mStartPunctLang != self.mStartLang || strStartPunct.is_some() {
            // java char[] _punct = null;
            let mut _punct: Option<Vec<char>> = None;
            // java if (strStartPunct != null && strStartPunct.length() > 0)
            if strStartPunct.is_some() && (strStartPunct.as_ref().unwrap().chars().count() as i32) > 0 {
              // java _punct = strStartPunct.toCharArray();
              _punct = Some(strStartPunct.as_ref().unwrap().chars().collect());
              // java Arrays.sort(_punct);
              Arrays::sort(_punct.as_mut().unwrap());
            }
            // java this.mStartPuncts[this.mStartLang] = _punct;
            self.mStartPuncts.as_mut().unwrap()[(self.mStartLang) as usize] = _punct;
          }
          // java this.mStartPunctLang = this.mStartLang;
          self.mStartPunctLang = self.mStartLang;
          // java this.mStrStartPunctCustom = strStartPunct;
          self.mStrStartPunctCustom = strStartPunct;
          // java if (this.mEndPunctLang != this.mEndLang || null != strEndPunct)
          if self.mEndPunctLang != self.mEndLang || strEndPunct.is_some() {
            // java char[] _punct = null;
            let mut _punct: Option<Vec<char>> = None;
            // java if (strEndPunct != null && strEndPunct.length() > 0)
            if strEndPunct.is_some() && (strEndPunct.as_ref().unwrap().chars().count() as i32) > 0 {
              // java _punct = strEndPunct.toCharArray();
              _punct = Some(strEndPunct.as_ref().unwrap().chars().collect());
              // java Arrays.sort(_punct);
              Arrays::sort(_punct.as_mut().unwrap());
            }
            // java this.mEndPuncts[this.mEndLang] = _punct;
            self.mEndPuncts.as_mut().unwrap()[(self.mEndLang) as usize] = _punct;
          }
          // java this.mEndPunctLang = this.mEndLang;
          self.mEndPunctLang = self.mEndLang;
          // java this.mStrEndPunctCustom = strEndPunct;
          self.mStrEndPunctCustom = strEndPunct;
        }
      }
    }

    // java public boolean isLineStartPunct(char ch)
    pub fn isLineStartPunct(&self, ch: char) -> bool {
      // java if (this.mStartPuncts[2] == null)
      if self.mStartPuncts.as_ref().unwrap()[2usize].is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java return SearchUtil.binarySearch(this.mStartPuncts[2], ch) >= 0;
        return SearchUtil::binarySearch__chararr_char(self.mStartPuncts.as_ref().unwrap()[2usize].as_ref().unwrap(), ch) >= 0;
      }
    }

    // java public boolean isLineEndPunct(char ch)
    pub fn isLineEndPunct(&self, ch: char) -> bool {
      // java if (this.mEndPuncts[2] == null)
      if self.mEndPuncts.as_ref().unwrap()[2usize].is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java return SearchUtil.binarySearch(this.mEndPuncts[2], ch) >= 0;
        return SearchUtil::binarySearch__chararr_char(self.mEndPuncts.as_ref().unwrap()[2usize].as_ref().unwrap(), ch) >= 0;
      }
    }

    // java public boolean isPunct(char ch)
    pub fn isPunct(&self, ch: char) -> bool {
      // java return SearchUtil.binarySearch(this.mPuncts, ch) >= 0;
      return SearchUtil::binarySearch__chararr_char(self.mPuncts.as_ref().unwrap(), ch) >= 0;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mMainDocument = null;
      self.mMainDocument = None;
      // java this.mStartPuncts = null;
      self.mStartPuncts = None;
      // java this.mEndPuncts = null;
      self.mEndPuncts = None;
      // java this.mPuncts = null;
      self.mPuncts = None;
    }

}
