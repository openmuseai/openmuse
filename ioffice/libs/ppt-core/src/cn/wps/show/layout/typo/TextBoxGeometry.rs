// Java: import android.graphics.Paint
// Java: import android.graphics.PointF
// Java: import android.graphics.RectF
// Java: import android.text.TextPaint
// Java: import cn.wps.text.layout.ITextLayout
// Java: import cn.wps.text.layout.data.ParaInfo
use wpslayout::cn::wps::text::layout::data::ParaInfo::ParaInfo;
// Java: import cn.wps.text.layout.data.TextBoxData
use wpslayout::cn::wps::text::layout::data::TextBoxData::TextBoxData;
// Java: import java.util.ArrayList
// Java: import java.util.List

use std::sync::atomic::{AtomicI32, Ordering};

// java public static final int NONE = 0;
pub const NONE: i32 = 0;
// java public static final int DOC = 1;
pub const DOC: i32 = 1;
// java public static final int DOCX = 2;
pub const DOCX: i32 = 2;
// java private static int ID_BASE = Integer.MIN_VALUE;
static ID_BASE: AtomicI32 = AtomicI32::new(i32::MIN);
// java protected static final float ITALIC_MARGIN_FACTOR = 0.2F;
pub(crate) const ITALIC_MARGIN_FACTOR: f32 = 0.2;
// java public static final String VOICE_KEY_WORD = "[voice]";
pub const VOICE_KEY_WORD: &str = "[voice]";
// java public static final String VOICE_STR = new String(new char[]{'\u0001'}) + "[voice]";
pub const VOICE_STR: &str = "\u{0001}[voice]";

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}

impl PointF {
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RectF {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl RectF {
    pub fn set(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        self.left = left;
        self.top = top;
        self.right = right;
        self.bottom = bottom;
    }
    pub fn setEmpty(&mut self) {
        self.set(0.0, 0.0, 0.0, 0.0);
    }
    pub fn width(&self) -> f32 {
        self.right - self.left
    }
}

pub struct TextPaint {}
pub struct Paint {}

pub struct TypoParam {
    pub mMaxWidth: f32,
    pub mMaxHeight: f32,
    pub mWidth: f32,
    pub mHeight: f32,
    pub mForceTypo: bool,
}

// java public abstract class BaseTextLayout implements ITextLayout
pub struct BaseTextLayout {
    // java private final int mID;
    mID: i32,
    // java protected List<PointF> mTypoOffsets;
    mTypoOffsets: Vec<PointF>,
    // java protected TextPaint mTextPaint;
    mTextPaint: TextPaint,
    // java protected TextBoxData mTextBoxData;
    mTextBoxData: Option<()>,
    // java protected RectF mTextRect;
    mTextRect: RectF,
    // java protected RectF mTypoRect;
    mTypoRect: RectF,
    // java protected float mTypoMaxWidth;
    mTypoMaxWidth: f32,
    // java protected float mTypoMaxHeight;
    mTypoMaxHeight: f32,
    // java protected int mTypoState;
    mTypoState: i32,
}

// java public abstract class BaseTextLayout implements ITextLayout
impl BaseTextLayout {
    // java public BaseTextLayout()
    pub fn BaseTextLayout() -> Self {
        // java super();
        // rust: no-stmt
        // java this.mID = ID_BASE++;
        let mID = ID_BASE.fetch_add(1, Ordering::Relaxed);
        // java this.mTypoOffsets = new ArrayList();
        let mTypoOffsets: Vec<PointF> = Vec::new();
        // java this.mTextPaint = new TextPaint();
        let mTextPaint = TextPaint {};
        // java this.mTextRect = new RectF();
        let mTextRect = RectF::default();
        // java this.mTypoRect = new RectF();
        let mTypoRect = RectF::default();
        // java this.mTypoMaxWidth = (float)Integer.MAX_VALUE;
        let mTypoMaxWidth = (i32::MAX as f32);
        // java this.mTypoMaxHeight = (float)Integer.MAX_VALUE;
        let mTypoMaxHeight = (i32::MAX as f32);
        // java this.mTypoState = -1;
        let mTypoState = -1;
        BaseTextLayout {
            mID,
            mTypoOffsets,
            mTextPaint,
            mTextBoxData: None,
            mTextRect,
            mTypoRect,
            mTypoMaxWidth,
            mTypoMaxHeight,
            mTypoState,
        }
    }

    // java public static String getSingleAudio(int id)
    pub fn getSingleAudio(id: i32) -> String {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return VOICE_STR + ";" + id + ";\n";
        return format!("{};{};\n", VOICE_STR, id);
        */
    }

    // java public static boolean isShape(String text)
    pub fn isShape(text: &str) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return text.startsWith(VOICE_STR);
        return text.starts_with(VOICE_STR);
        */
    }

    // java public int id()
    pub fn id(&self) -> i32 {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.mID;
        return self.mID;
        */
    }

    // java public boolean forceProcessCrlf()
    pub fn forceProcessCrlf(&self) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return false;
        return false;
        */
    }

    // java public void typo(TextBoxData textBoxData, ITextLayout.TypoParam param)
    pub fn typo__textboxdata_typoparam(&mut self, textBoxData: &TextBoxData, param: &TypoParam) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (textBoxData.isVertical())
        if textBoxData.isVertical() {
          // java this.mTypoMaxWidth = param.mMaxHeight;
          self.mTypoMaxWidth = param.mMaxHeight;
          // java this.mTypoMaxHeight = param.mMaxWidth;
          self.mTypoMaxHeight = param.mMaxWidth;
        } else {
          // java this.mTypoMaxWidth = param.mMaxWidth;
          self.mTypoMaxWidth = param.mMaxWidth;
          // java this.mTypoMaxHeight = param.mMaxHeight;
          self.mTypoMaxHeight = param.mMaxHeight;
        }
        // java this.typo(textBoxData, param.mWidth, param.mHeight, param.mForceTypo);
        self.typo(textBoxData, param.mWidth, param.mHeight, param.mForceTypo);
        */
    }

    // java public void typo(TextBoxData textBoxData, float width, float height, boolean forceTypo)
    pub fn typo(&mut self, textBoxData: &TextBoxData, width: f32, height: f32, forceTypo: bool) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        let mut width = width;
        let mut height = height;
        // java if (!textBoxData.isEmpty())
        if !textBoxData.isEmpty() {
          // java this.mTextBoxData = textBoxData;
          let _ = textBoxData;
          // java textBoxData.processBeforeTypo(this.forceProcessCrlf());
          // rust: no-stmt
          // java if (textBoxData.isVertical())
          if textBoxData.isVertical() {
            // java float temp = width;
            let temp = width;
            // java width = height;
            width = height;
            // java height = temp;
            height = temp;
          }
          // java this.mTypoRect.set(0.0F, 0.0F, width, height);
          self.mTypoRect.set(0.0, 0.0, width, height);
          // java this.setTextRect(textBoxData, width, height);
          self.setTextRect(textBoxData, width, height);
          // java this.doTypo(textBoxData, forceTypo);
          self.doTypo(textBoxData, forceTypo);
        }
        */
    }

    // java private void setTextRect(TextBoxData textBoxData, float width, float height)
    pub fn setTextRect(&mut self, textBoxData: &TextBoxData, width: f32, height: f32) {
        // java this.correctInset(textBoxData, width, height);
        let _ = self.correctInset(textBoxData, width, height);
        // java float left = 0.0F;
        let mut left: f32 = 0.0;
        // java float top = 0.0F;
        let mut top: f32 = 0.0;
        // java float right = width;
        let mut right: f32 = width;
        // java float bottom = height;
        let mut bottom: f32 = height;
        // java left += textBoxData.leftInset();
        left += textBoxData.leftInset();
        // java if (left > width)
        if left > width {
            // java right = left;
            right = left;
        }
        // java right -= textBoxData.rightInset();
        right -= textBoxData.rightInset();
        // java if (right < left)
        if right < left {
            // java left = right;
            left = right;
        }
        // java top += textBoxData.topInset();
        top += textBoxData.topInset();
        // java if (top > height)
        if top > height {
            // java bottom = top;
            bottom = top;
        }
        // java bottom -= textBoxData.bottomInset();
        bottom -= textBoxData.bottomInset();
        // java if (bottom < top)
        if bottom < top {
            // java top = bottom;
            top = bottom;
        }
        // java if (left > width)
        if left > width {
            // java float atten = (left - width) / 2.0F;
            let atten: f32 = (left - width) / 2.0;
            // java left -= atten;
            left -= atten;
            // java right -= atten;
            right -= atten;
        }
        // java if (right < 0.0F)
        if right < 0.0 {
            // java float atten = -right / 2.0F;
            let atten: f32 = -right / 2.0;
            // java left += atten;
            left += atten;
            // java right += atten;
            right += atten;
        }
        // java if (top > height)
        if top > height {
            // java float atten = (top - height) / 2.0F;
            let atten: f32 = (top - height) / 2.0;
            // java top -= atten;
            top -= atten;
            // java bottom -= atten;
            bottom -= atten;
        }
        // java if (bottom < 0.0F)
        if bottom < 0.0 {
            // java float atten = -bottom / 2.0F;
            let atten: f32 = -bottom / 2.0;
            // java top += atten;
            top += atten;
            // java bottom += atten;
            bottom += atten;
        }
        // java this.mTextRect.set(left, top, right, bottom);
        self.mTextRect.set(left, top, right, bottom);
    }

    // java public void destroy()
    pub fn destroy(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.mTextBoxData = null;
        self.mTextBoxData = None;
        // java this.mTextRect = null;
        self.mTextRect = RectF::default();
        */
    }

    // java public TextBoxData getTextBoxData()
    pub fn getTextBoxData(&self) -> Option<()> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.mTextBoxData;
        return self.mTextBoxData;
        */
    }

    // java public RectF getTextRect()
    pub fn getTextRect(&self) -> RectF {
        // java return this.mTextRect;
        return self.mTextRect;
    }

    // java public void clear()
    pub fn clear(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.mTextBoxData.clear();
        // rust: no-stmt
        // java this.mTypoOffsets.clear();
        self.mTypoOffsets.clear();
        // java this.mTextRect.setEmpty();
        self.mTextRect.setEmpty();
        // java this.mTypoRect.setEmpty();
        self.mTypoRect.setEmpty();
        // java this.mTypoState = -1;
        self.mTypoState = -1;
        */
    }

    // java public int state()
    pub fn state(&self) -> i32 {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.mTypoState;
        return self.mTypoState;
        */
    }

    // java protected PointF setTextAlignment(int paraIndex, ParaInfo paraInfo, RectF rect, TextPaint paint, float typoWidth, float preParaBottom)
    pub fn setTextAlignment(
        &self,
        paraIndex: i32,
        paraInfo: &ParaInfo,
        rect: &RectF,
        paint: &TextPaint,
        typoWidth: f32,
        preParaBottom: f32,
    ) -> PointF {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java PointF typoOffset = new PointF();
        let mut typoOffset = PointF { x: 0.0, y: 0.0 };
        // java typoOffset.set(rect.left, rect.top);
        typoOffset.set(rect.left, rect.top);
        // java float rectWidth = this.isAutoWidth() ? typoWidth : rect.width();
        let rectWidth = if self.isAutoWidth() { typoWidth } else { rect.width() };
        // java typoOffset.x += this.calcHorizAlign(paraIndex, paraInfo, paint, rectWidth, typoWidth);
        typoOffset.x += self.calcHorizAlign(paraIndex, paraInfo, paint, rectWidth, typoWidth);
        // java typoOffset.y = preParaBottom;
        typoOffset.y = preParaBottom;
        // java return typoOffset;
        return typoOffset;
        */
    }

    // java protected boolean isAutoWidth()
    pub fn isAutoWidth(&self) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.mTypoRect.width() <= 0.0F;
        return self.mTypoRect.width() <= 0.0;
        */
    }

    // java protected boolean isAutoHeight()
    pub fn isAutoHeight(&self) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.mTypoRect.height() <= 0.0F;
        return self.mTypoRect.width() <= 0.0;
        */
    }

    // java protected float calcHorizAlign(int paraIndex, ParaInfo paraInfo, Paint paint, float cellWidth, float typoWidth)
    pub fn calcHorizAlign(
        &self,
        paraIndex: i32,
        paraInfo: &ParaInfo,
        paint: &Paint,
        cellWidth: f32,
        typoWidth: f32,
    ) -> f32 {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return 0.0F;
        return 0.0;
        */
    }

    // java protected float calcVertAlign(int paraIndex, ParaInfo paraInfo, Paint paint, float cellHeight, float typoHeight)
    pub fn calcVertAlign(
        &self,
        paraIndex: i32,
        paraInfo: &ParaInfo,
        paint: &Paint,
        cellHeight: f32,
        typoHeight: f32,
    ) -> f32 {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return 0.0F;
        return 0.0;
        */
    }

    // java protected boolean correctInset(TextBoxData textBoxData, float width, float height)
    pub fn correctInset(&self, textBoxData: &TextBoxData, width: f32, height: f32) -> bool {
        let _ = (textBoxData, width, height);
        // java return false;
        return false;
    }

    // java protected abstract void doTypo(TextBoxData var1, boolean var2)
    pub fn doTypo(&mut self, var1: &TextBoxData, var2: bool) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        let _ = (var1, var2);
        */
    }
}

/// pl-05-02 门面：`BaseTextLayout.setTextRect` → 内缩矩形（不写 `TextBoxData`）。
pub fn text_box_geometry(textBoxData: &TextBoxData, width: f32, height: f32) -> RectF {
    let mut layout = BaseTextLayout::BaseTextLayout();
    layout.setTextRect(textBoxData, width, height);
    layout.getTextRect()
}
