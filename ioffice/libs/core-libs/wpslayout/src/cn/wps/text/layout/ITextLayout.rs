// Java: import android.graphics.Canvas
// Java: import android.graphics.RectF
// Java: import cn.wps.text.layout.data.TextBoxData
// Java: import cn.wps.text.layout.typo.ArtTransform

// Java field: int STATE_TYPO_NONE
pub const STATE_TYPO_NONE: i32 = -1;
// Java field: int STATE_TYPO_FAIL
pub const STATE_TYPO_FAIL: i32 = 0;
// Java field: int STATE_TYPO_SUCCESSS
pub const STATE_TYPO_SUCCESSS: i32 = 1;
// Java field: int STATE_TYPO_RESZIE
pub const STATE_TYPO_RESZIE: i32 = 2;
// Java field: int STATE_TYPO_REAUTOFIT
pub const STATE_TYPO_REAUTOFIT: i32 = 3;

// Java: public interface ITextLayout extends ITextEdit
pub trait ITextLayout: super::ITextEdit::ITextEdit {
    fn id(&self) -> i32;
    fn state(&self) -> i32;
    fn clear(&mut self);
    fn destroy(&mut self);
}

// Java: public static class TypoParam
pub struct TypoParam {
    // Java field: float mWidth
    pub mWidth: f32,
    // Java field: float mHeight
    pub mHeight: f32,
    // Java field: float mMaxWidth
    pub mMaxWidth: f32,
    // Java field: float mMaxHeight
    pub mMaxHeight: f32,
    // Java field: boolean mForceTypo
    pub mForceTypo: bool,
}

pub const TYPO_AUTO_ADJUST_SIZE: i32 = 0;

impl TypoParam {
    pub fn new() -> Self {
        TypoParam {
            mWidth: 0.0,
            mHeight: 0.0,
            mMaxWidth: i32::MAX as f32,
            mMaxHeight: i32::MAX as f32,
            mForceTypo: true,
        }
    }

    pub fn isAutoAdjustSize(size: f32) -> bool {
        size <= 0.0
    }
}
