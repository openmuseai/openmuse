// Java: import android.graphics.Bitmap
// Java: import android.graphics.Canvas
// Java: import android.graphics.Matrix
// Java: import android.graphics.Paint
// Java: import android.graphics.Rect
// Java: import android.graphics.Bitmap.Config
// Java: import android.graphics.PorterDuff.Mode
// Java: import android.text.DynamicLayout
// Java: import android.text.GetChars
// Java: import android.text.SpannableStringBuilder
// Java: import android.text.TextPaint
// Java: import android.text.Layout.Alignment
// Java: import android.text.style.MetricAffectingSpan

use super::IEmojiService::IEmojiService;

// Java field: float MAX_TEXTSIZE
const MAX_TEXTSIZE: f32 = 180.0;
// Java field: int BITMAP_WIDTH
const BITMAP_WIDTH: i32 = 100;
// Java field: int BITMAP_HEIGHT
const BITMAP_HEIGHT: i32 = 100;

// Java: public class EmojiServiceImpl implements IEmojiService
pub struct EmojiServiceImpl {
    // Java field: DynamicLayout mLayout
    // Java field: SpannableStringBuilder mText
    // Java field: TextSizeSpan mSpan
    mSpan: TextSizeSpan,
    // Java field: CharBufWrapper mBufWrapper
    mBufWrapper: CharBufWrapper,
    // Java field: float mScale
    pub(crate) mScale: f32,
    // Java field: float mActualTextSize
    mActualTextSize: f32,
    // Java field: boolean mIsBmpCleared
    mIsBmpCleared: bool,
    // Java field: char[] mCharBuf
    mCharBuf: Vec<u16>,
    // Java field: Matrix mMatrix
    // Java field: float[] mMatrixValues
    mMatrixValues: Vec<f32>,
    // Java field: Bitmap mBitmap
    // Java field: Canvas mBmpCanvas
    // Java field: Rect mSrcRect
    // Java field: Rect mDstRect
}

// Java: public class CharBufWrapper
pub struct CharBufWrapper {
    // Java field: char[] mBuf
    pub(crate) mBuf: Vec<u16>,
    // Java field: int mStartIdx
    pub(crate) mStartIdx: i32,
    // Java field: int mLength
    pub(crate) mLength: i32,
}

// Java: public class TextSizeSpan
pub struct TextSizeSpan {
    // Java field: float mTextSize
    pub(crate) mTextSize: f32,
}

// Java: public class EmojiServiceImpl implements IEmojiService
impl EmojiServiceImpl {
    // Java: EmojiServiceImpl()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.mText.setSpan(this.mSpan, 0, 0, 18);
        // Java: this.mLayout = new DynamicLayout(this.mText, new TextPaint(), Integer.MAX_VALUE, Alignment.ALIGN_NORMAL, 1.0F, 0.0F, false);
        EmojiServiceImpl {
            mSpan: TextSizeSpan::new(),
            mBufWrapper: CharBufWrapper::new(),
            mScale: 1.0,
            mActualTextSize: 0.0,
            mIsBmpCleared: true,
            mCharBuf: vec![0u16; 2],
            mMatrixValues: vec![0.0f32; 9],
        }
    }

    // Java: void prepareToDrawBitmap()
    fn prepareToDrawBitmap(&mut self) {
        // Java: this.mIsBmpCleared = false;
        self.mIsBmpCleared = false;
        // Java: if (null == this.mBitmap)
        {
            // Java: this.mBitmap = Bitmap.createBitmap(100, 100, Config.ARGB_8888);
            // Java: this.mBmpCanvas = new Canvas(this.mBitmap);
            // Java: this.mSrcRect = new Rect();
            // Java: this.mDstRect = new Rect();
        }
    }

    // Java: void setEmoji(char ch, float textSize)
    pub fn setEmoji_char(&mut self, ch: u16, text_size: f32) {
        // Java: char[] buf = this.mCharBuf;
        // Java: buf[0] = ch;
        self.mCharBuf[0] = ch;
        // Java: this.doSetText(buf, 0, 1, textSize);
        self.doSetText(0, 1, text_size);
    }

    // Java: void setEmoji(char highSurrogate, char lowSurrogate, float textSize)
    pub fn setEmoji_char_char(&mut self, high_surrogate: u16, low_surrogate: u16, text_size: f32) {
        // Java: char[] buf = this.mCharBuf;
        // Java: buf[0] = highSurrogate;
        self.mCharBuf[0] = high_surrogate;
        // Java: buf[1] = lowSurrogate;
        self.mCharBuf[1] = low_surrogate;
        // Java: this.doSetText(buf, 0, 2, textSize);
        self.doSetText(0, 2, text_size);
    }

    // Java: void setEmoji(int codePoint, float textSize)
    pub fn setEmoji_int(&mut self, code_point: i32, text_size: f32) {
        // Java: char[] buf = this.mCharBuf;
        // Java: int count = Character.toChars(codePoint, this.mCharBuf, 0);
        let ch = char::from_u32(code_point as u32).unwrap_or('\0');
        let mut buf = [0u16; 2];
        let count = ch.encode_utf16(&mut buf).len();
        self.mCharBuf[..count].copy_from_slice(&buf[..count]);
        // Java: this.doSetText(buf, 0, count, textSize);
        self.doSetText(0, count as i32, text_size);
    }

    // Java: void setEmoji(char[] buff, int buffIndex, int len, float fontSize)
    pub fn setEmoji_chars(&mut self, _buff: &[u16], buff_index: i32, len: i32, font_size: f32) {
        // Java: this.doSetText(buff, buffIndex, len, fontSize);
        self.doSetText(buff_index, len, font_size);
    }

    // Java: void doSetText(char[] buf, int bufStartIdx, int bufLen, float textSize)
    fn doSetText(&mut self, _buf_start_idx: i32, _buf_len: i32, text_size: f32) {
        // Java: boolean needScale = textSize > 180.0F;
        let need_scale = text_size > MAX_TEXTSIZE;
        // Java: this.mActualTextSize = needScale ? 180.0F : textSize;
        self.mActualTextSize = if need_scale { MAX_TEXTSIZE } else { text_size };
        // Java: this.mScale = needScale ? textSize / 180.0F : 1.0F;
        self.mScale = if need_scale { text_size / MAX_TEXTSIZE } else { 1.0 };
        // Java: this.mSpan.setTextSize(this.mActualTextSize);
        self.mSpan.setTextSize(self.mActualTextSize);
        // Java: this.mBufWrapper.init(buf, bufStartIdx, bufLen);
        // Java: this.mText.replace(0, this.mText.length(), this.mBufWrapper);
        // Java: this.clearBitmap();
        self.clearBitmap();
    }

    // Java: void clearBitmap()
    fn clearBitmap(&mut self) {
        // Java: if (null != this.mBitmap && !this.mIsBmpCleared)
        if !self.mIsBmpCleared {
            // Java: this.mIsBmpCleared = true;
            self.mIsBmpCleared = true;
            // Java: this.mBmpCanvas.drawColor(0, Mode.CLEAR);
        }
    }

    // Java: void draw(Object nativeCanvas)
    pub fn draw(&self, _native_canvas: i64) {
        // Java: Canvas canvas = (Canvas)nativeCanvas;
        // Java: DynamicLayout layout = this.mLayout;
        // Java: if (this.shouldDrawCanvasFromBitmap(canvas))
        if self.shouldDrawCanvasFromBitmap() {
            // Java: this.prepareToDrawBitmap();
            // Java: float layoutWidth = layout.getLineWidth(0);
            // Java: float layoutHeight = (float)layout.getHeight();
            // Java: Bitmap bitmap = this.mBitmap;
            // Java: float bmpWidth = (float)bitmap.getWidth();
            // Java: float bmpHeight = (float)bitmap.getHeight();
            // Java: float bmpScale = Math.min(bmpWidth / layoutWidth, bmpHeight / layoutHeight);
            // Java: Canvas bmpCanvas = this.mBmpCanvas;
            // Java: bmpCanvas.save();
            // Java: bmpCanvas.scale(bmpScale, bmpScale);
            // Java: layout.draw(bmpCanvas);
            // Java: bmpCanvas.restore();
            // Java: Rect srcRect = this.mSrcRect;
            // Java: int srcRectW = Math.round(bmpScale * layoutWidth);
            // Java: int srcRectH = Math.round(bmpScale * layoutHeight);
            // Java: srcRect.set(0, 0, srcRectW, srcRectH);
            // Java: Rect dstRect = this.mDstRect;
            // Java: int dstRectW = Math.round(this.mScale * layoutWidth);
            // Java: int dstRectH = Math.round(this.mScale * layoutHeight);
            // Java: dstRect.set(0, 0, dstRectW, dstRectH);
            // Java: canvas.save();
            // Java: canvas.drawBitmap(bitmap, srcRect, dstRect, (Paint)null);
            // Java: canvas.restore();
        }
        // Java: else
        else {
            // Java: layout.draw(canvas);
        }
    }

    // Java: boolean shouldDrawCanvasFromBitmap(Canvas canvas)
    fn shouldDrawCanvasFromBitmap(&self) -> bool {
        // Java: if (this.mScale > 1.0F)
        if self.mScale > 1.0 {
            // Java: return true;
            return true;
        }
        // Java: else
        {
            // Java: Matrix matrix = this.mMatrix;
            // Java: float[] matrixValues = this.mMatrixValues;
            // Java: canvas.getMatrix(matrix);
            // Java: matrix.getValues(matrixValues);
            // Java: float scaleX = matrixValues[0];
            let scale_x = self.mMatrixValues[0];
            // Java: float scaleY = matrixValues[4];
            let scale_y = self.mMatrixValues[4];
            // Java: if (scaleX != scaleY)
            if scale_x != scale_y {
                // Java: return true;
                return true;
            }
            // Java: else if (0.0F == matrixValues[1] && 0.0F == matrixValues[3])
            else if self.mMatrixValues[1] == 0.0 && self.mMatrixValues[3] == 0.0 {
                // Java: if (!(scaleX <= 0.0F) && !(scaleY <= 0.0F))
                if scale_x > 0.0 && scale_y > 0.0 {
                    // Java: return this.mScale * this.mActualTextSize * scaleX > 180.0F;
                    return self.mScale * self.mActualTextSize * scale_x > MAX_TEXTSIZE;
                }
                // Java: else
                else {
                    // Java: return true;
                    return true;
                }
            }
            // Java: else
            else {
                // Java: return true;
                true
            }
        }
    }

    // Java: float getAscent()
    pub fn getAscent(&self) -> f32 {
        // Java: return this.mScale * (float)(-this.mLayout.getLineAscent(0));
        self.mScale * 0.0
    }

    // Java: float getDescent()
    pub fn getDescent(&self) -> f32 {
        // Java: return this.mScale * (float)this.mLayout.getLineDescent(0);
        self.mScale * 0.0
    }

    // Java: float getHeight()
    pub fn getHeight(&self) -> f32 {
        // Java: return this.mScale * (float)this.mLayout.getHeight();
        self.mScale * 0.0
    }

    // Java: float getWidth()
    pub fn getWidth(&self) -> f32 {
        // Java: return this.mScale * this.mLayout.getLineWidth(0);
        self.mScale * 0.0
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this.mCharBuf = null;
        self.mCharBuf.clear();
        // Java: this.mSpan = null;
        // Java: this.mLayout = null;
        // Java: this.mBmpCanvas = null;
        // Java: this.mMatrix = null;
        // Java: this.mMatrixValues = null;
        self.mMatrixValues.clear();
        // Java: this.mSrcRect = null;
        // Java: this.mDstRect = null;
        // Java: if (null != this.mText)
        {
            // Java: this.mText.clear();
            // Java: this.mText.clearSpans();
            // Java: this.mText = null;
        }
        // Java: if (null != this.mBufWrapper)
        {
            // Java: this.mBufWrapper.dispose();
            self.mBufWrapper.dispose();
            // Java: this.mBufWrapper = null;
        }
        // Java: if (null != this.mBitmap)
        {
            // Java: this.mBitmap.recycle();
            // Java: this.mBitmap = null;
        }
    }
}

// Java: public class CharBufWrapper
impl CharBufWrapper {
    // Java: CharBufWrapper()
    pub fn new() -> Self {
        // Java: super();
        CharBufWrapper {
            mBuf: Vec::new(),
            mStartIdx: 0,
            mLength: 0,
        }
    }

    // Java: void init(char[] buf, int start, int len)
    pub fn init(&mut self, buf: &[u16], start: i32, len: i32) {
        // Java: this.mBuf = buf;
        self.mBuf = buf.to_vec();
        // Java: this.mStartIdx = start;
        self.mStartIdx = start;
        // Java: this.mLength = len;
        self.mLength = len;
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this.mBuf = null;
        self.mBuf.clear();
        // Java: this.mStartIdx = Integer.MIN_VALUE;
        self.mStartIdx = i32::MIN;
        // Java: this.mLength = Integer.MIN_VALUE;
        self.mLength = i32::MIN;
    }

    // Java: int length()
    pub fn length(&self) -> i32 {
        // Java: return this.mLength;
        self.mLength
    }

    // Java: char charAt(int index)
    pub fn charAt(&self, index: i32) -> u16 {
        // Java: return this.mBuf[this.mStartIdx + index];
        self.mBuf[(self.mStartIdx + index) as usize]
    }

    // Java: CharSequence subSequence(int start, int end)
    pub fn subSequence(&self, start: i32, end: i32) -> String {
        // Java: int subStrLen = end - start;
        let from = (self.mStartIdx + start) as usize;
        let to = (self.mStartIdx + end) as usize;
        // Java: return String.valueOf(this.mBuf, this.mStartIdx + start, subStrLen);
        String::from_utf16_lossy(&self.mBuf[from..to])
    }

    // Java: void getChars(int start, int end, char[] dest, int destPos)
    pub fn getChars(&self, start: i32, end: i32, dest: &mut [u16], dest_pos: i32) {
        // Java: int copyBufStart = this.mStartIdx + start;
        let copy_buf_start = (self.mStartIdx + start) as usize;
        // Java: int copyBufLen = end - start;
        let copy_buf_len = (end - start) as usize;
        // Java: System.arraycopy(this.mBuf, copyBufStart, dest, destPos, copyBufLen);
        let dp = dest_pos as usize;
        dest[dp..dp + copy_buf_len].copy_from_slice(&self.mBuf[copy_buf_start..copy_buf_start + copy_buf_len]);
    }
}

// Java: public class TextSizeSpan
impl TextSizeSpan {
    // Java: TextSizeSpan()
    pub fn new() -> Self {
        // Java: super();
        TextSizeSpan { mTextSize: 0.0 }
    }

    // Java: void setTextSize(float size)
    pub fn setTextSize(&mut self, size: f32) {
        // Java: this.mTextSize = size;
        self.mTextSize = size;
    }

    // Java: void applyTextSize(TextPaint paint)
    fn applyTextSize(&self, _paint: i64) {
        // Java: if (null != paint)
        {
            // Java: paint.setTextSize(this.mTextSize);
        }
    }

    // Java: void updateMeasureState(TextPaint p)
    pub fn updateMeasureState(&self, p: i64) {
        // Java: this.applyTextSize(p);
        self.applyTextSize(p);
    }

    // Java: void updateDrawState(TextPaint tp)
    pub fn updateDrawState(&self, tp: i64) {
        // Java: this.applyTextSize(tp);
        self.applyTextSize(tp);
    }
}

impl IEmojiService for EmojiServiceImpl {
    fn setEmoji_char(&self, _var1: u16, _var2: f32) {}
    fn setEmoji_char_char(&self, _var1: u16, _var2: u16, _var3: f32) {}
    fn setEmoji_int(&self, _var1: i32, _var2: f32) {}
    fn setEmoji_chars(&self, _var1: &[u16], _var2: i32, _var3: i32, _var4: f32) {}
    fn draw(&self, _var1: i64) {}
    fn getAscent(&self) -> f32 { self.getAscent() }
    fn getDescent(&self) -> f32 { self.getDescent() }
    fn getHeight(&self) -> f32 { self.getHeight() }
    fn getWidth(&self) -> f32 { self.getWidth() }
    fn dispose(&self) {}
}
