// Java: import android.graphics.Paint
// (no direct Rust equivalent)
// Java: import android.graphics.RectF
// (mapped to [f32; 4] or custom struct)
// Java: import cn.wps.font.IFontFamily
// (not available in this crate)

// Java field: int Type1_Font
#[allow(non_upper_case_globals)]
pub(crate) const Type1_Font: i32 = 1;
// Java field: int Type1CID_Font
#[allow(non_upper_case_globals)]
pub(crate) const Type1CID_Font: i32 = 2;
// Java field: int CFF_Font
#[allow(non_upper_case_globals)]
pub(crate) const CFF_Font: i32 = 3;
// Java field: int TrueType_Font
#[allow(non_upper_case_globals)]
pub(crate) const TrueType_Font: i32 = 4;
// Java field: int Ohter_Font
#[allow(non_upper_case_globals)]
pub(crate) const Ohter_Font: i32 = 5;
// Java field: int NotEmbeddable_Font
#[allow(non_upper_case_globals)]
pub(crate) const NotEmbeddable_Font: i32 = 6;

// Java: public class KAdvancedTypefaceMetrics
pub struct KAdvancedTypefaceMetrics {
    // Java field: IFontFamily mFamily
    // mFamily: not available (IFontFamily is external)
    // Java field: String mFontName
    pub(crate) mFontName: Option<String>,
    // Java field: int mStyle
    pub(crate) mStyle: i32,
    // Java field: int mItalicAngle
    pub(crate) mItalicAngle: i32,
    // Java field: float mAscent
    pub(crate) mAscent: f32,
    // Java field: float mDescent
    pub(crate) mDescent: f32,
    // Java field: int mStemV
    pub(crate) mStemV: i32,
    // Java field: int mCapHeight
    pub(crate) mCapHeight: i32,
    // Java field: int mEmSize
    pub(crate) mEmSize: i32,
    // Java field: RectF mBBox
    pub(crate) mBBox: Option<[f32; 4]>,
    // Java field: int mFontInfo
    pub(crate) mFontInfo: i32,
}

// Java: public class KAdvancedTypefaceMetrics
#[allow(non_snake_case)]
impl KAdvancedTypefaceMetrics {
    // Java: KAdvancedTypefaceMetrics()
    pub fn new() -> Self {
      // Java: super();
      Self {
          mFontName: None,
          mStyle: 0,
          mItalicAngle: 0,
          mAscent: 0.0,
          mDescent: 0.0,
          mStemV: 0,
          mCapHeight: 0,
          mEmSize: 0,
          mBBox: None,
          mFontInfo: 0,
      }
    }

    // Java: static KAdvancedTypefaceMetrics getAdvancedTypefaceMetrics(IFontFamily family, Paint paint)
    pub fn getAdvancedTypefaceMetrics(_family_name: &str, _paint_ascent: Option<f32>, _paint_descent: Option<f32>) -> KAdvancedTypefaceMetrics {
      // Java: KAdvancedTypefaceMetrics info = new KAdvancedTypefaceMetrics();
      let mut info = KAdvancedTypefaceMetrics::new();
      // Java: info.mFamily = family;
      // (IFontFamily not available)
      // Java: info.mFontName = family.getName();
      info.mFontName = Some(_family_name.to_string());
      // Java: if (paint != null)
        if _paint_ascent.is_some()
      {
         // Java: Paint.FontMetrics metrics = paint.getFontMetrics();
         // Java: info.mStyle = 0;
         info.mStyle = 0;
         // Java: info.mItalicAngle = 0;
         info.mItalicAngle = 0;
         // Java: info.mAscent = metrics.ascent;
         info.mAscent = _paint_ascent.unwrap_or(0.0);
         // Java: info.mDescent = metrics.descent;
         info.mDescent = _paint_descent.unwrap_or(0.0);
         // Java: info.mStemV = 0;
         info.mStemV = 0;
         // Java: info.mCapHeight = 0;
         info.mCapHeight = 0;
      }
      // Java: info.mFontInfo = 4;
      info.mFontInfo = 4;
      // Java: return info;
      info
    }

}
