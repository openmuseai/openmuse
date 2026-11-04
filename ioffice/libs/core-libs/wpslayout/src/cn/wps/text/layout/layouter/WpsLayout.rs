// Java: import android.graphics.Canvas
// Java: import android.graphics.Matrix
// Java: import android.graphics.Paint
// Java: import android.graphics.Path
// Java: import android.text.TextPaint
// Java: import android.text.TextUtils
// Java: import android.text.method.TextKeyListener
// Java: import cn.wps.font.ITypeface
// Java: import cn.wps.language.Thai
// Java: import cn.wps.text.layout.data.ParaInfo
// Java: import cn.wps.text.layout.typo.WordSplitSpan
// Java: import cn.wps.text.layout.typo.WpsForegroundColorSpan
// Java: import cn.wps.text.layout.typo.WpsTabStop
// Java: import cn.wps.text.layout.typo.WpsTextOutlineSpan
// Java: import cn.wps.text.layout.typo.layout.ArrayUtils
// Java: import cn.wps.text.layout.typo.layout.Layout
// Java: import cn.wps.text.layout.typo.layout.Spannable
// Java: import cn.wps.text.layout.typo.layout.SpannableStringBuilder
// Java: import cn.wps.text.layout.typo.layout.Spanned
// Java: import cn.wps.text.layout.typo.layout.style.CharacterStyle
// Java: import cn.wps.text.layout.typo.layout.style.MetricAffectingSpan
// Java: import cn.wps.text.layout.util.MathUtil
// Java: import java.lang.reflect.Array
// Java: import java.lang.reflect.Field
// Java: import java.text.Bidi
// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.List
use crate::cn::wps::text::layout::data::ParaInfo::ParaInfo;
use crate::cn::wps::text::layout::typo::layout::Layout::Layout;

pub struct Directions;

// Java field: int LINEDATA_COUNT
pub(crate) const LINEDATA_COUNT: i32 = 2;
// Java field: int COLUMNS_NORMAL
pub(crate) const COLUMNS_NORMAL: i32 = 1;
// Java field: int COLUMNS_ELLIPSIZE
pub(crate) const COLUMNS_ELLIPSIZE: i32 = 5;
// Java field: int START
pub(crate) const START: i32 = 0;
// Java field: int DIR
pub(crate) const DIR: i32 = 0;
// Java field: int TAB
pub(crate) const TAB: i32 = 0;
// Java field: int TOP
pub(crate) const TOP: i32 = 0;
// Java field: int DESCENT
pub(crate) const DESCENT: i32 = 1;
// Java field: int ELLIPSIS_START
pub(crate) const ELLIPSIS_START: i32 = 3;
// Java field: int ELLIPSIS_COUNT
pub(crate) const ELLIPSIS_COUNT: i32 = 4;
// Java field: int START_MASK
pub(crate) const START_MASK: i32 = 536870911;
// Java field: int DIR_SHIFT
pub(crate) const DIR_SHIFT: i32 = 30;
// Java field: int TAB_MASK
pub(crate) const TAB_MASK: i32 = 536870912;
// Java field: char CHAR_FIRST_CJK
pub(crate) const CHAR_FIRST_CJK: char = '\u{2E80}';
// Java field: char NL
pub(crate) const NL: char = '\u{0085}';
// Java field: char LF
pub(crate) const LF: char = '\n';
// Java field: char CR
pub(crate) const CR: char = '\r';
// Java field: char[] BK
static BK: std::sync::OnceLock<Vec<char>> = std::sync::OnceLock::new();
// Java field: char CHAR_TAB
pub(crate) const CHAR_TAB: char = '\t';
// Java field: char CHAR_SPACE
pub(crate) const CHAR_SPACE: char = ' ';
// Java field: char CHAR_SLASH
pub(crate) const CHAR_SLASH: char = '/';
// Java field: char CHAR_HYPHEN
pub(crate) const CHAR_HYPHEN: char = '-';
// Java field: char CHAR_CJK_HYPHEN
pub(crate) const CHAR_CJK_HYPHEN: char = '\u{2013}';
// Java field: char CHAR_ZWSP
pub(crate) const CHAR_ZWSP: char = '\u{200b}';
// Java field: String EA_NON_BREAK_PUNCTUATIONS
pub(crate) const EA_NON_BREAK_PUNCTUATIONS: &str = "\u{FFE5}";

fn get_bk() -> &'static Vec<char> {
    BK.get_or_init(|| vec!['\u{000b}', '\u{000c}', '\u{2028}', '\u{2029}'])
}

// Java: public class WpsLayout extends Layout
pub struct WpsLayout {
    // Java field: int mFirstLead
    mFirstLead: i32,
    // Java field: int mRestLead
    mRestLead: i32,
    // Java field: int mLineCount
    mLineCount: i32,
    // Java field: int mTopPadding
    mTopPadding: i32,
    // Java field: int mBottomPadding
    mBottomPadding: i32,
    // Java field: int mColumns
    mColumns: i32,
    // Java field: float[] charWidths
    charWidths: Option<Vec<f32>>,
    // Java field: float[] originCharWidths
    originCharWidths: Option<Vec<f32>>,
    // Java field: float[] spacBefCharWidths
    spacBefCharWidths: Option<Vec<f32>>,
    // Java field: float[] spacings
    spacings: Option<Vec<f32>>,
    // Java field: int[] mLines
    mLines: Option<Vec<i32>>,
    // Java field: List<Float> mlineDatas
    mlineDatas: Option<Vec<f32>>,
    // Java field: Layout.Directions[] mLineDirections
    mLineDirections: Option<Vec<Directions>>,
    // Java field: int mMaximumVisibleLineCount
    mMaximumVisibleLineCount: i32,
    // Java field: ParaInfo mParaInfo
    mParaInfo: Option<ParaInfo>,
    // Java field: boolean hasParaInfo
    hasParaInfo: bool,
    // Java field: int[] invertMap
    invertMap: Option<Vec<i32>>,
    // Java field: int[] indexMap
    indexMap: Option<Vec<i32>>,
    pub(crate) layout: Layout,
}

// Java: public class WpsLayout extends Layout
impl WpsLayout {
    pub fn new() -> Self {
        WpsLayout {
            mFirstLead: 0,
            mRestLead: 0,
            mLineCount: 0,
            mTopPadding: 0,
            mBottomPadding: 0,
            mColumns: COLUMNS_NORMAL,
            charWidths: None,
            originCharWidths: None,
            spacBefCharWidths: None,
            spacings: None,
            mLines: None,
            mlineDatas: None,
            mLineDirections: None,
            mMaximumVisibleLineCount: i32::MAX,
            mParaInfo: None,
            hasParaInfo: false,
            invertMap: None,
            indexMap: None,
            layout: Layout::new(),
        }
    }

    pub fn getLineCount(&self) -> i32 { self.mLineCount }
}
