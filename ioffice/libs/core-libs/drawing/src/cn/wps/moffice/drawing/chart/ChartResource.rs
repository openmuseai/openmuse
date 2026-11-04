#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables)]

use std::sync::OnceLock;

// Java field: int CHART_COLOR_STYLE1
pub(crate) const CHART_COLOR_STYLE1: i32 = 0;
// Java field: int CHART_COLOR_STYLE2
pub(crate) const CHART_COLOR_STYLE2: i32 = 1;
// Java field: int CHART_COLOR_STYLE3
pub(crate) const CHART_COLOR_STYLE3: i32 = 2;
// Java field: int CHART_COLOR_STYLE4
pub(crate) const CHART_COLOR_STYLE4: i32 = 3;
// Java field: int CHART_COLOR_STYLE5
pub(crate) const CHART_COLOR_STYLE5: i32 = 4;
// Java field: int DEF_COLOR_STYLE_INDEX
pub(crate) const DEF_COLOR_STYLE_INDEX: i32 = 0;
// Java field: int[] mColorStyles
static mColorStyles: OnceLock<Vec<i32>> = OnceLock::new();
fn get_mColorStyles() -> &'static Vec<i32> { mColorStyles.get_or_init(|| vec![10, 11, 12, 13, 14]) }
// Java field: int DEF_COLOR_STYLE_ID
pub(crate) const DEF_COLOR_STYLE_ID: i32 = 10;
// Java field: int[] mBarTypes
static mBarTypes: OnceLock<Vec<i32>> = OnceLock::new();
fn get_mBarTypes() -> &'static Vec<i32> { mBarTypes.get_or_init(|| vec![17, 19, 20]) }
// Java field: int[] mBarStyles
static mBarStyles: OnceLock<Vec<i32>> = OnceLock::new();
fn get_mBarStyles() -> &'static Vec<i32> { mBarStyles.get_or_init(|| vec![216, 297, 297]) }
// Java field: int[] mColumnTypes
static mColumnTypes: OnceLock<Vec<i32>> = OnceLock::new();
fn get_mColumnTypes() -> &'static Vec<i32> { mColumnTypes.get_or_init(|| vec![23, 24, 25]) }
// Java field: int[] mColumnStyles
static mColumnStyles: OnceLock<Vec<i32>> = OnceLock::new();
fn get_mColumnStyles() -> &'static Vec<i32> { mColumnStyles.get_or_init(|| vec![201, 297, 297]) }
// Java field: int[] mLineTypes
static mLineTypes: OnceLock<Vec<i32>> = OnceLock::new();
fn get_mLineTypes() -> &'static Vec<i32> { mLineTypes.get_or_init(|| vec![42, 46, 47, 43, 44, 45]) }
// Java field: int[] mLineStyles
static mLineStyles: OnceLock<Vec<i32>> = OnceLock::new();
fn get_mLineStyles() -> &'static Vec<i32> { mLineStyles.get_or_init(|| vec![227, 227, 227, 332, 332, 332]) }
// Java field: int[] mPieTypes
static mPieTypes: OnceLock<Vec<i32>> = OnceLock::new();
fn get_mPieTypes() -> &'static Vec<i32> { mPieTypes.get_or_init(|| vec![48, 49, 50, 18, 40, 41]) }
// Java field: int[] mPieStyles
static mPieStyles: OnceLock<Vec<i32>> = OnceLock::new();
fn get_mPieStyles() -> &'static Vec<i32> { mPieStyles.get_or_init(|| vec![251, 251, 333, 333, 251, 251]) }
// Java field: int[] mAreaTypes
static mAreaTypes: OnceLock<Vec<i32>> = OnceLock::new();
fn get_mAreaTypes() -> &'static Vec<i32> { mAreaTypes.get_or_init(|| vec![14, 15, 16]) }
// Java field: int[] mAreaStyles
static mAreaStyles: OnceLock<Vec<i32>> = OnceLock::new();
fn get_mAreaStyles() -> &'static Vec<i32> { mAreaStyles.get_or_init(|| vec![276, 276, 276]) }
// Java field: int[] mXYScatterTypes
static mXYScatterTypes: OnceLock<Vec<i32>> = OnceLock::new();
fn get_mXYScatterTypes() -> &'static Vec<i32> { mXYScatterTypes.get_or_init(|| vec![69, 72, 73, 70, 71]) }
// Java field: int[] mXYScatterStyles
static mXYScatterStyles: OnceLock<Vec<i32>> = OnceLock::new();
fn get_mXYScatterStyles() -> &'static Vec<i32> { mXYScatterStyles.get_or_init(|| vec![240, 240, 240, 240, 240]) }
// Java field: int[] mStockTypes
static mStockTypes: OnceLock<Vec<i32>> = OnceLock::new();
fn get_mStockTypes() -> &'static Vec<i32> { mStockTypes.get_or_init(|| vec![61, 62, 63, 64]) }
// Java field: int[] mRadarTypes
static mRadarTypes: OnceLock<Vec<i32>> = OnceLock::new();
fn get_mRadarTypes() -> &'static Vec<i32> { mRadarTypes.get_or_init(|| vec![58, 60, 59]) }
// Java field: int[] mRadarStyles
static mRadarStyles: OnceLock<Vec<i32>> = OnceLock::new();
fn get_mRadarStyles() -> &'static Vec<i32> { mRadarStyles.get_or_init(|| vec![317, 317, 317]) }

// Java: public class ChartResource
pub struct ChartResource {
}

// Java: public enum UIChartType
pub struct UIChartType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UIChartTypeEnum {
    BAR,
    COLUMN,
    LINE,
    PIE,
    AREA,
    XY,
    STOCK,
    RADAR,
    NONE,
}

// Java: public class ChartResource
impl ChartResource {
    // Java: ChartResource()
    pub fn new() -> Self {
      // Java: super();
      ChartResource {}
    }

    // Java: static int findColorStyleIndex(int colorStyle)
    pub fn findColorStyleIndex(colorStyle: i32) -> i32 {
      // Java: return findIndex(mColorStyles, colorStyle);
      Self::findIndex(get_mColorStyles(), colorStyle)
    }

    // Java: static int findColorStyle(int index)
    pub fn findColorStyle(index: i32) -> i32 {
      // Java: return index >= 0 && index < mColorStyles.length ? mColorStyles[index] : -1;
      let styles = get_mColorStyles();
      if index >= 0 && (index as usize) < styles.len() { styles[index as usize] } else { -1 }
    }

    // Java: static int findIndex(int[] arr, int val)
    fn findIndex(arr: &[i32], val: i32) -> i32 {
      // Java: for(int i = 0; i < arr.length; ++i)
      for i in 0..arr.len() {
         // Java: if (arr[i] == val)
         if arr[i] == val {
            // Java: return i;
            return i as i32;
         }
      }
      // Java: return -1;
      -1
    }

    // Java: static int styleFromType(int[] styles, int[] types, int type)
    fn styleFromType(styles: &[i32], types: &[i32], r#type: i32) -> i32 {
      // Java: int index = findIndex(types, type);
      let index = Self::findIndex(types, r#type);
      // Java: return styles[index];
      styles[index as usize]
    }

    // Java: static int barStyleFromType(int type)
    pub fn barStyleFromType(r#type: i32) -> i32 {
      // Java: return styleFromType(mBarStyles, mBarTypes, type);
      Self::styleFromType(get_mBarStyles(), get_mBarTypes(), r#type)
    }

    // Java: static int columnStyleFromType(int type)
    pub fn columnStyleFromType(r#type: i32) -> i32 {
      // Java: return styleFromType(mColumnStyles, mColumnTypes, type);
      Self::styleFromType(get_mColumnStyles(), get_mColumnTypes(), r#type)
    }

    // Java: static int lineStyleFromType(int type)
    pub fn lineStyleFromType(r#type: i32) -> i32 {
      // Java: return styleFromType(mLineStyles, mLineTypes, type);
      Self::styleFromType(get_mLineStyles(), get_mLineTypes(), r#type)
    }

    // Java: static int pieStyleFromType(int type)
    pub fn pieStyleFromType(r#type: i32) -> i32 {
      // Java: return styleFromType(mPieStyles, mPieTypes, type);
      Self::styleFromType(get_mPieStyles(), get_mPieTypes(), r#type)
    }

    // Java: static int areaStyleFromType(int type)
    pub fn areaStyleFromType(r#type: i32) -> i32 {
      // Java: return styleFromType(mAreaStyles, mAreaTypes, type);
      Self::styleFromType(get_mAreaStyles(), get_mAreaTypes(), r#type)
    }

}

// Java: public enum UIChartType
impl UIChartType {
    // Java: UIChartType()
    pub fn new() -> Self {
      UIChartType
    }
}
