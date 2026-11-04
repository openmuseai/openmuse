// Java: import android.graphics.Paint
// Java: import android.graphics.Path
// Java: import android.graphics.Typeface
// Java: import android.text.TextPaint
// Java: import cn.wps.font.FontMetrics
use crate::cn::wps::font::FontMetrics::FontMetrics;
// Java: import cn.wps.font.FreeTypeJNI
use crate::cn::wps::font::FreeTypeJNI::{self, RectF, TTFHeader, TypefaceInfo};
// Java: import cn.wps.font.IFontFamily
use crate::cn::wps::font::IFontFamily::IFontFamily;
// Java: import cn.wps.font.ITypeface
// Java: import cn.wps.graphics.Rect
// Java: import cn.wps.graphics.RectF
// Java: import cn.wps.moffice.define.FontName
// Java: import cn.wps.util.PrimitivePackTool
use crate::cn::wps::util::PrimitivePackTool;
// Java: import java.util.LinkedList
// Java: import java.util.List

use super::AbstractTypeface::AbstractTypeface;
use super::BucketWidthCache::BucketWidthCache;
use super::BucketRectCache::BucketRectCache;
use super::KerningPairCache::KerningPairCache;
use super::BuildinGlyphMetrics;
use std::sync::{Arc, Mutex, OnceLock};

pub struct TextPaint {
    text_size: f32,
    typeface: i64,
    anti_alias: bool,
    flags: i32,
}
impl TextPaint {
    pub fn new() -> Self { TextPaint { text_size: 0.0, typeface: 0, anti_alias: false, flags: 0 } }
    pub fn setTextSize(&mut self, _size: f32) {}
    pub fn setTypeface(&mut self, _tf: i64) {}
    pub fn setAntiAlias(&mut self, _aa: bool) {}
    pub fn setFlags(&mut self, _flags: i32) {}
    pub fn getFontMetrics(&self, _fm: &mut PaintFontMetrics) {}
    pub fn measureText_slice(&self, _text: &[u16], _index: usize, _count: usize) -> f32 { 0.0 }
    pub fn getTextPath(&self, _text: &[u16], _index: usize, _count: usize, _x: f32, _y: f32, _path: &mut Path) {}
}

pub struct PaintFontMetrics {
    pub top: f32,
    pub ascent: f32,
    pub descent: f32,
    pub bottom: f32,
    pub leading: f32,
}
impl PaintFontMetrics { pub fn new() -> Self { PaintFontMetrics { top: 0.0, ascent: 0.0, descent: 0.0, bottom: 0.0, leading: 0.0 } } }

pub struct Path;
impl Path { pub fn new() -> Self { Path } pub fn computeBounds(&self, _rect: &mut RectF, _exact: bool) {} }

#[derive(Clone, Default)]
pub struct Rect { pub left: i32, pub top: i32, pub right: i32, pub bottom: i32 }

#[derive(Clone)]
pub struct BlockItem { pub start: i32, pub end: i32 }
impl BlockItem { pub fn contains(&self, c: i32) -> bool { c >= self.start && c <= self.end } }

struct UnicodeBlockMap;
impl UnicodeBlockMap {
    fn parseRangeToList_name(_supportlangs: &mut Vec<BlockItem>, _name: &str) {}
    fn parseRangeToList_range(_range: i32, _start: i32, _end: i32, _supportlangs: &mut Vec<BlockItem>) {}
}

struct BuildinTTFHeader;
impl BuildinTTFHeader {
    fn getBuildinTTFHeader(_name: &str, _style: i32) -> Option<TTFHeader> { None }
}

struct KerningCacheFactory;
impl KerningCacheFactory {
    fn createCache(_name: &str) -> Option<KerningPairCache> { None }
}

struct FontName;
impl FontName {
    fn isSpecialFontName(_name: &str) -> bool { false }
}

// Java field: int MAX_STYLE_COUNT
const MAX_STYLE_COUNT: i32 = 4;
// Java field: float CALC_FONT_SIZE
const CALC_FONT_SIZE: f32 = 100.0;
// Java field: int BUFF_SIZE
const BUFF_SIZE: usize = 128;
// Java field: int FLAG_FSSELECTION
const FLAG_FSSELECTION: i32 = 128;
// Java field: FreeTypeJNI.TTFHeader sTTFHeader
static S_TTF_HEADER: OnceLock<Mutex<TTFHeader>> = OnceLock::new();
fn get_s_ttf_header() -> &'static Mutex<TTFHeader> {
    S_TTF_HEADER.get_or_init(|| Mutex::new(TTFHeader::new()))
}

// Java: public class AndroidTypeface extends AbstractTypeface implements Cloneable
pub struct AndroidTypeface {
    pub base: AbstractTypeface,
    // Java field: FreeTypeJNI.TypefaceInfo mTypefaceInfo
    mTypefaceInfo: Option<TypefaceInfo>,
    // Java field: Typeface mImpl
    pub(crate) mImpl: i64,
    // Java field: TextPaint mPaint
    mPaint: TextPaint,
    // Java field: float[] mWidthBuf
    mWidthBuf: Vec<f32>,
    // Java field: char[] mTextBuf
    mTextBuf: Vec<u16>,
    // Java field: BucketWidthCache mWidthCache
    mWidthCache: Option<BucketWidthCache>,
    // Java field: BucketRectCache mRectCache
    mRectCache: Option<BucketRectCache>,
    // Java field: LayoutMetricsInfo mMetricsInfo
    mMetricsInfo: Option<LayoutMetricsInfo>,
    // Java field: WppLayoutMetricInfo mWppLayoutMetricInfo
    mWppLayoutMetricInfo: Option<WppLayoutMetricInfo>,
    // Java field: float mFarEastWidth
    pub(crate) mFarEastWidth: f32,
    // Java field: KerningPairCache mKerningCache
    pub(crate) mKerningCache: Option<KerningPairCache>,
    // Java field: boolean mInitedKerning
    mInitedKerning: bool,
    // Java field: BuildinGlyphMetrics.IMetricsGetter mMetricsGetter
    pub(crate) mMetricsGetter: Option<Box<dyn BuildinGlyphMetrics::IMetricsGetterTrait>>,
    // Java field: Rect mRect
    mRect: Rect,
}

// Java: public class LayoutMetricsInfo
#[derive(Clone)]
pub struct LayoutMetricsInfo {
    // Java field: boolean isCJKFont
    pub(crate) isCJKFont: bool,
    // Java field: boolean isLatinFont
    pub(crate) isLatinFont: bool,
    // Java field: boolean isComplexFont
    pub(crate) isComplexFont: bool,
    // Java field: float ascent
    pub(crate) ascent: f32,
    // Java field: float descent
    pub(crate) descent: f32,
    // Java field: float latinExternal
    pub(crate) latinExternal: f32,
    // Java field: float charSpaceRatio
    pub(crate) charSpaceRatio: f32,
    // Java field: float subscriptFactor
    pub(crate) subscriptFactor: f32,
    // Java field: float superscriptFactor
    pub(crate) superscriptFactor: f32,
    // Java field: float srcEmSize
    pub(crate) srcEmSize: f32,
    // Java field: float emSize
    pub(crate) emSize: f32,
    // Java field: int underline_position
    pub(crate) underline_position: i32,
    // Java field: int underline_thickness
    pub(crate) underline_thickness: i32,
    // Java field: int yStrikeoutSize
    pub(crate) yStrikeoutSize: i32,
    // Java field: int yStrikeoutPosition
    pub(crate) yStrikeoutPosition: i32,
    // Java field: float lineGap
    pub(crate) lineGap: f32,
    // Java field: float hheaAscent
    pub(crate) hheaAscent: f32,
    // Java field: float hheaDescent
    pub(crate) hheaDescent: f32,
    // Java field: float winAscent
    pub(crate) winAscent: f32,
    // Java field: float winDescent
    pub(crate) winDescent: f32,
    // Java field: int fsSelection
    pub(crate) fsSelection: i32,
    // Java field: List<UnicodeBlockMap.BlockItem> supportlangs
    pub(crate) supportlangs: Vec<BlockItem>,
}

// Java: public class WppLayoutMetricInfo
#[derive(Clone)]
pub struct WppLayoutMetricInfo {
    // Java field: float height
    pub(crate) height: f32,
    // Java field: float stakckedHeight
    pub(crate) stakckedHeight: f32,
}

// Java: public class AndroidTypeface extends AbstractTypeface implements Cloneable
impl AndroidTypeface {
    // Java: AndroidTypeface(IFontFamily family, int style, Typeface impl)
    pub fn new(family: Option<Arc<dyn IFontFamily>>, style: i32, type_impl: i64) -> Self {
        // Java: this(family, style, impl, false);
        Self::new_with(family, style, type_impl, false)
    }

    // Java: AndroidTypeface(IFontFamily family, int style, Typeface impl, boolean createFromFontFile)
    pub fn new_with(family: Option<Arc<dyn IFontFamily>>, style: i32, type_impl: i64, create_from_font_file: bool) -> Self {
        // Java: super(family, style, createFromFontFile);
        let base = AbstractTypeface::new_with(family, style, create_from_font_file);
        // Java: this.mPaint = new TextPaint();
        // Java: this.mWidthBuf = new float[128];
        // Java: this.mTextBuf = new char[128];
        // Java: this.mWidthCache = null;
        // Java: this.mRectCache = null;
        // Java: this.mMetricsInfo = null;
        // Java: this.mWppLayoutMetricInfo = null;
        // Java: this.mFarEastWidth = -1.0F;
        // Java: this.mInitedKerning = false;
        // Java: this.mRect = new Rect();
        // Java: this.mImpl = impl;
        let mut at = AndroidTypeface {
            base,
            mTypefaceInfo: None,
            mImpl: type_impl,
            mPaint: TextPaint::new(),
            mWidthBuf: vec![0.0f32; BUFF_SIZE],
            mTextBuf: vec![0u16; BUFF_SIZE],
            mWidthCache: None,
            mRectCache: None,
            mMetricsInfo: None,
            mWppLayoutMetricInfo: None,
            mFarEastWidth: -1.0,
            mKerningCache: None,
            mInitedKerning: false,
            mMetricsGetter: None,
            mRect: Rect::default(),
        };
        // Java: this.initMetricsGetter();
        at.initMetricsGetter();
        // Java: this.initFontMetrics();
        at.initFontMetrics();
        at
    }

    // Java: BuildinGlyphMetrics.IMetricsGetter getMetricsGetter()
    pub fn getMetricsGetter(&self) -> &Option<Box<dyn BuildinGlyphMetrics::IMetricsGetterTrait>> {
        // Java: return this.mMetricsGetter;
        &self.mMetricsGetter
    }

    // Java: void initMetricsGetter()
    fn initMetricsGetter(&mut self) {
        // Java: BuildinGlyphMetrics.IMetricsFactory metricsFactory = BuildinGlyphMetrics.getInstance().getMetricsFactory();
        // Java: if (metricsFactory != null)
        // Java: this.mMetricsGetter = metricsFactory.createMetrics(this);
    }

    // Java: void initFontMetrics()
    fn initFontMetrics(&mut self) {
        // Java: Paint.FontMetrics fontMetrics = new Paint.FontMetrics();
        let mut font_metrics = PaintFontMetrics::new();
        // Java: this.mPaint.setTextSize(100.0F);
        self.mPaint.setTextSize(CALC_FONT_SIZE);
        // Java: this.mPaint.setTypeface(this.mImpl);
        self.mPaint.setTypeface(self.mImpl);
        // Java: this.mPaint.setAntiAlias(true);
        self.mPaint.setAntiAlias(true);
        // Java: this.mPaint.setFlags(128);
        self.mPaint.setFlags(FLAG_FSSELECTION);
        // Java: this.mPaint.getFontMetrics(fontMetrics);
        self.mPaint.getFontMetrics(&mut font_metrics);
        // Java: this.mFontMetrics.setMaxDescent(fontMetrics.descent);
        self.base.mFontMetrics.setMaxDescent(font_metrics.descent);
        // Java: this.mFontMetrics.setMaxAscent(-fontMetrics.ascent);
        self.base.mFontMetrics.setMaxAscent(-font_metrics.ascent);
        // Java: this.mFontMetrics.setBaseline(-fontMetrics.top + fontMetrics.leading);
        self.base.mFontMetrics.setBaseline(-font_metrics.top + font_metrics.leading);
        // Java: this.mFontMetrics.setHeight(fontMetrics.bottom - fontMetrics.top);
        self.base.mFontMetrics.setHeight(font_metrics.bottom - font_metrics.top);
    }

    // Java: float getSrcEmSize()
    pub fn getSrcEmSize(&mut self) -> f32 {
        // Java: return this.ensureLayoutMetricsInfo().srcEmSize;
        self.ensureLayoutMetricsInfo();
        self.mMetricsInfo.as_ref().unwrap().srcEmSize
    }

    // Java: void setSrcEmSize(float emSize)
    pub fn setSrcEmSize(&mut self, em_size: f32) {
        // Java: this.ensureLayoutMetricsInfo().srcEmSize = emSize;
        self.ensureLayoutMetricsInfo();
        self.mMetricsInfo.as_mut().unwrap().srcEmSize = em_size;
    }

    // Java: void getFontMetrics(float size, FontMetrics m)
    pub fn getFontMetrics(&self, size: f32, m: &mut FontMetrics) {
        // Java: float scal = size / 100.0F;
        let scal = size / CALC_FONT_SIZE;
        // Java: m.setBaseline(this.mFontMetrics.getBaseline() * scal);
        m.setBaseline(self.base.mFontMetrics.getBaseline() * scal);
        // Java: m.setMaxAscent(this.mFontMetrics.getMaxAscent() * scal);
        m.setMaxAscent(self.base.mFontMetrics.getMaxAscent() * scal);
        // Java: m.setMaxDescent(this.mFontMetrics.getMaxDescent() * scal);
        m.setMaxDescent(self.base.mFontMetrics.getMaxDescent() * scal);
        // Java: m.setHeight(this.mFontMetrics.getHeight() * scal);
        m.setHeight(self.base.mFontMetrics.getHeight() * scal);
    }

    // Java: BucketWidthCache ensureBucketWidthCache()
    fn ensureBucketWidthCache(&mut self) -> &mut BucketWidthCache {
        // Java: if (null == this.mWidthCache)
        if self.mWidthCache.is_none() {
            // Java: this.mWidthCache = new BucketWidthCache();
            self.mWidthCache = Some(BucketWidthCache::new());
        }
        // Java: return this.mWidthCache;
        self.mWidthCache.as_mut().unwrap()
    }

    // Java: BucketRectCache ensureBucketRectCache()
    fn ensureBucketRectCache(&mut self) -> &mut BucketRectCache {
        // Java: if (null == this.mRectCache)
        if self.mRectCache.is_none() {
            // Java: this.mRectCache = new BucketRectCache();
            self.mRectCache = Some(BucketRectCache::new());
        }
        // Java: return this.mRectCache;
        self.mRectCache.as_mut().unwrap()
    }

    // Java: void getTextWidths(float size, char[] text, int textIndex, float[] widths, int widthIndex, int length)
    pub fn getTextWidths_float(&mut self, size: f32, text: &[u16], text_index: usize, widths: &mut [f32], width_index: usize, length: usize) {
        // Java: float scal = size / 100.0F;
        let scal = size / CALC_FONT_SIZE;
        // Java: BucketWidthCache widthCache = this.ensureBucketWidthCache();
        // Java: char[] textBuf = null;
        let mut text_buf: Option<Vec<u16>> = None;
        // Java: float[] widthBuf = null;
        let mut width_buf: Option<Vec<f32>> = None;
        // Java: int j = 0;
        let mut j: usize = 0;
        // Java: int posWidth = widthIndex;
        let mut pos_width = width_index;
        // Java: int i = textIndex;
        // Java: for(int n = textIndex + length; i < n; ++i)
        for i in text_index..(text_index + length) {
            // Java: float width = widthCache.findWidth(text[i]);
            let width = self.mWidthCache.as_ref().map_or(-1.0, |c| c.findWidth(text[i]));
            // Java: if (width >= 0.0F)
            if width >= 0.0 {
                // Java: widths[posWidth++] = width * scal;
                widths[pos_width] = width * scal;
                pos_width += 1;
            }
            // Java: else
            else {
                // Java: this.mPaint.setTextSize(100.0F);
                self.mPaint.setTextSize(CALC_FONT_SIZE);
                // Java: this.mPaint.setTypeface(this.mImpl);
                self.mPaint.setTypeface(self.mImpl);
                // Java: width = this.mPaint.measureText(text, i, 1);
                let width = self.mPaint.measureText_slice(text, i, 1);
                // Java: widths[posWidth++] = width * scal;
                widths[pos_width] = width * scal;
                pos_width += 1;
                // Java: if (textBuf == null)
                if text_buf.is_none() {
                    // Java: textBuf = length > 128 ? new char[length] : this.mTextBuf;
                    text_buf = Some(vec![0u16; length]);
                    // Java: widthBuf = length > 128 ? new float[length] : this.mWidthBuf;
                    width_buf = Some(vec![0.0f32; length]);
                }
                // Java: textBuf[j] = text[i];
                if let Some(ref mut tb) = text_buf { tb[j] = text[i]; }
                // Java: widthBuf[j++] = width;
                if let Some(ref mut wb) = width_buf { wb[j] = width; }
                j += 1;
            }
        }
        // Java: if (j > 0)
        if j > 0 {
            // Java: widthCache.addWidth(textBuf, 0, widthBuf, 0, j);
            if let (Some(tb), Some(wb)) = (text_buf, width_buf) {
                if let Some(ref mut wc) = self.mWidthCache {
                    wc.addWidth_batch(&tb, 0, &wb, 0, j);
                }
            }
        }
    }

    // Java: void getTextWidths(int size, char[] text, int textIndex, int[] widths, int widthIndex, int length)
    pub fn getTextWidths_int(&mut self, size: i32, text: &[u16], text_index: usize, widths: &mut [i32], width_index: usize, length: usize) {
        // Java: float scal = (float)size / 100.0F;
        let scal = size as f32 / CALC_FONT_SIZE;
        // Java: BucketWidthCache widthCache = this.ensureBucketWidthCache();
        // Java: char[] textBuf = null;
        let mut text_buf: Option<Vec<u16>> = None;
        // Java: float[] widthBuf = null;
        let mut width_buf: Option<Vec<f32>> = None;
        // Java: int j = 0;
        let mut j: usize = 0;
        // Java: int posWidth = widthIndex;
        let mut pos_width = width_index;
        // Java: int i = textIndex;
        // Java: for(int n = textIndex + length; i < n; ++i)
        for i in text_index..(text_index + length) {
            // Java: float width = widthCache.findWidth(text[i]);
            let width = self.mWidthCache.as_ref().map_or(-1.0, |c| c.findWidth(text[i]));
            // Java: if (width >= 0.0F)
            if width >= 0.0 {
                // Java: widths[posWidth++] = (int)(width * scal + 0.5F);
                widths[pos_width] = (width * scal + 0.5) as i32;
                pos_width += 1;
            }
            // Java: else
            else {
                // Java: this.mPaint.setTextSize(100.0F);
                self.mPaint.setTextSize(CALC_FONT_SIZE);
                // Java: this.mPaint.setTypeface(this.mImpl);
                self.mPaint.setTypeface(self.mImpl);
                // Java: width = this.mPaint.measureText(text, i, 1);
                let width = self.mPaint.measureText_slice(text, i, 1);
                // Java: widths[posWidth++] = (int)(width * scal + 0.5F);
                widths[pos_width] = (width * scal + 0.5) as i32;
                pos_width += 1;
                // Java: if (textBuf == null)
                if text_buf.is_none() {
                    // Java: textBuf = length > 128 ? new char[length] : this.mTextBuf;
                    text_buf = Some(vec![0u16; length]);
                    // Java: widthBuf = length > 128 ? new float[length] : this.mWidthBuf;
                    width_buf = Some(vec![0.0f32; length]);
                }
                // Java: textBuf[j] = text[i];
                if let Some(ref mut tb) = text_buf { tb[j] = text[i]; }
                // Java: widthBuf[j++] = (float)((int)(width + 0.5F));
                if let Some(ref mut wb) = width_buf { wb[j] = (width + 0.5) as i32 as f32; }
                j += 1;
            }
        }
        // Java: if (j > 0)
        if j > 0 {
            // Java: widthCache.addWidth(textBuf, 0, widthBuf, 0, j);
            if let (Some(tb), Some(wb)) = (text_buf, width_buf) {
                if let Some(ref mut wc) = self.mWidthCache {
                    wc.addWidth_batch(&tb, 0, &wb, 0, j);
                }
            }
        }
    }

    // Java: void getTextWidths(int size, int[] text, int textIndex, int[] widths, int widthIndex, int length)
    pub fn getTextWidths_int_arr(&mut self, size: i32, text: &[i32], text_index: usize, widths: &mut [i32], width_index: usize, length: usize) {
        // Java: float scal = (float)size / 100.0F;
        let scal = size as f32 / CALC_FONT_SIZE;
        // Java: BucketWidthCache widthCache = this.ensureBucketWidthCache();
        // Java: char[] textBuf = null;
        let mut text_buf: Option<Vec<u16>> = None;
        // Java: float[] widthBuf = null;
        let mut width_buf: Option<Vec<f32>> = None;
        // Java: char[] tmpBuf = new char[1];
        let mut tmp_buf: Vec<u16> = vec![0u16; 1];
        // Java: int j = 0;
        let mut j: usize = 0;
        // Java: int posWidth = widthIndex;
        let mut pos_width = width_index;
        // Java: int i = textIndex;
        // Java: for(int n = textIndex + length; i < n; ++i)
        for i in text_index..(text_index + length) {
            // Java: float width = widthCache.findWidth((char)text[i]);
            let ch = text[i] as u16;
            let width = self.mWidthCache.as_ref().map_or(-1.0, |c| c.findWidth(ch));
            // Java: if (width >= 0.0F)
            if width >= 0.0 {
                // Java: widths[posWidth++] = (int)(width * scal + 0.5F);
                widths[pos_width] = (width * scal + 0.5) as i32;
                pos_width += 1;
            }
            // Java: else
            else {
                // Java: this.mPaint.setTextSize(100.0F);
                self.mPaint.setTextSize(CALC_FONT_SIZE);
                // Java: this.mPaint.setTypeface(this.mImpl);
                self.mPaint.setTypeface(self.mImpl);
                // Java: tmpBuf[0] = (char)text[i];
                tmp_buf[0] = ch;
                // Java: width = this.mPaint.measureText(tmpBuf, 0, 1);
                let width = self.mPaint.measureText_slice(&tmp_buf, 0, 1);
                // Java: widths[posWidth++] = (int)(width * scal + 0.5F);
                widths[pos_width] = (width * scal + 0.5) as i32;
                pos_width += 1;
                // Java: if (textBuf == null)
                if text_buf.is_none() {
                    // Java: textBuf = length > 128 ? new char[length] : this.mTextBuf;
                    text_buf = Some(vec![0u16; length]);
                    // Java: widthBuf = length > 128 ? new float[length] : this.mWidthBuf;
                    width_buf = Some(vec![0.0f32; length]);
                }
                // Java: textBuf[j] = (char)text[i];
                if let Some(ref mut tb) = text_buf { tb[j] = ch; }
                // Java: widthBuf[j++] = (float)((int)(width + 0.5F));
                if let Some(ref mut wb) = width_buf { wb[j] = (width + 0.5) as i32 as f32; }
                j += 1;
            }
        }
        // Java: if (j > 0)
        if j > 0 {
            // Java: widthCache.addWidth(textBuf, 0, widthBuf, 0, j);
            if let (Some(tb), Some(wb)) = (text_buf, width_buf) {
                if let Some(ref mut wc) = self.mWidthCache {
                    wc.addWidth_batch(&tb, 0, &wb, 0, j);
                }
            }
        }
    }

    // Java: void getTextWidthsEM(char[] text, int textIndex, int[] widths, int widthIndex, int length)
    pub fn getTextWidthsEM(&mut self, text: &[u16], text_index: usize, widths: &mut [i32], width_index: usize, length: usize) {
        // Java: BucketWidthCache widthCache = this.ensureBucketWidthCache();
        // Java: char[] textBuf = null;
        let mut text_buf: Option<Vec<u16>> = None;
        // Java: float[] widthBuf = null;
        let mut width_buf: Option<Vec<f32>> = None;
        // Java: int j = 0;
        let mut j: usize = 0;
        // Java: int posWidth = widthIndex;
        let mut pos_width = width_index;
        // Java: float emsize = this.ensureLayoutMetricsInfo().emSize;
        self.ensureLayoutMetricsInfo();
        let emsize = self.mMetricsInfo.as_ref().unwrap().emSize;
        // Java: int i = textIndex;
        // Java: for(int n = textIndex + length; i < n; ++i)
        for i in text_index..(text_index + length) {
            // Java: float width = widthCache.findWidth(text[i]);
            let width = self.mWidthCache.as_ref().map_or(-1.0, |c| c.findWidth(text[i]));
            // Java: if (width >= 0.0F)
            if width >= 0.0 {
                // Java: widths[posWidth++] = (int)(width + 0.5F);
                widths[pos_width] = (width + 0.5) as i32;
                pos_width += 1;
            }
            // Java: else
            else {
                // Java: this.mPaint.setTextSize(emsize);
                self.mPaint.setTextSize(emsize);
                // Java: this.mPaint.setTypeface(this.mImpl);
                self.mPaint.setTypeface(self.mImpl);
                // Java: width = this.mPaint.measureText(text, i, 1);
                let width = self.mPaint.measureText_slice(text, i, 1);
                // Java: widths[posWidth++] = (int)(width + 0.5F);
                widths[pos_width] = (width + 0.5) as i32;
                pos_width += 1;
                // Java: if (textBuf == null)
                if text_buf.is_none() {
                    // Java: textBuf = length > 128 ? new char[length] : this.mTextBuf;
                    text_buf = Some(vec![0u16; length]);
                    // Java: widthBuf = length > 128 ? new float[length] : this.mWidthBuf;
                    width_buf = Some(vec![0.0f32; length]);
                }
                // Java: textBuf[j] = text[i];
                if let Some(ref mut tb) = text_buf { tb[j] = text[i]; }
                // Java: widthBuf[j++] = width;
                if let Some(ref mut wb) = width_buf { wb[j] = width; }
                j += 1;
            }
        }
        // Java: if (j > 0)
        if j > 0 {
            // Java: widthCache.addWidth(textBuf, 0, widthBuf, 0, j);
            if let (Some(tb), Some(wb)) = (text_buf, width_buf) {
                if let Some(ref mut wc) = self.mWidthCache {
                    wc.addWidth_batch(&tb, 0, &wb, 0, j);
                }
            }
        }
    }

    // Java: float getTextWidth(float size, char[] text, int textIndex, int length)
    pub fn getTextWidth(&mut self, size: f32, text: &[u16], text_index: usize, length: usize) -> f32 {
        // Java: this.mPaint.setTextSize(100.0F);
        self.mPaint.setTextSize(CALC_FONT_SIZE);
        // Java: float result = this.mPaint.measureText(text, textIndex, length);
        let result = self.mPaint.measureText_slice(text, text_index, length);
        // Java: return result * size / 100.0F;
        result * size / CALC_FONT_SIZE
    }

    // Java: int getTextWidthEM(char[] text, int textIndex, int length)
    pub fn getTextWidthEM(&mut self, text: &[u16], text_index: usize, length: usize) -> i32 {
        // Java: this.mPaint.setTextSize(this.ensureLayoutMetricsInfo().emSize);
        self.ensureLayoutMetricsInfo();
        let emsize = self.mMetricsInfo.as_ref().unwrap().emSize;
        self.mPaint.setTextSize(emsize);
        // Java: float result = this.mPaint.measureText(text, textIndex, length);
        let result = self.mPaint.measureText_slice(text, text_index, length);
        // Java: return (int)(result + 0.5F);
        (result + 0.5) as i32
    }

    // Java: RectF getTextRect(float size, char[] text, int textIndex, int length)
    pub fn getTextRect_text(&mut self, size: f32, text: &[u16], text_index: usize, length: usize) -> RectF {
        // Java: float scal = size / 100.0F;
        let scal = size / CALC_FONT_SIZE;
        // Java: Path path = this.getTextPath(text, textIndex, length);
        // Java: android.graphics.RectF rectF = new android.graphics.RectF();
        let mut rect_f = RectF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 };
        // Java: path.computeBounds(rectF, false);
        // Java: return new RectF(rectF.left * scal, rectF.top * scal, rectF.right * scal, rectF.bottom * scal);
        RectF { left: rect_f.left * scal, top: rect_f.top * scal, right: rect_f.right * scal, bottom: rect_f.bottom * scal }
    }

    // Java: RectF getTextRectEM(char[] text, int textIndex, int length)
    pub fn getTextRectEM_text(&mut self, text: &[u16], text_index: usize, length: usize) -> RectF {
        // Java: Path path = this.getTextPathEM(text, textIndex, length);
        // Java: android.graphics.RectF rectF = new android.graphics.RectF();
        let rect_f = RectF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 };
        // Java: path.computeBounds(rectF, false);
        // Java: return new RectF(rectF.left, rectF.top, rectF.right, rectF.bottom);
        rect_f
    }

    // Java: Path getTextPath(char[] text, int textIndex, int length)
    fn getTextPath(&mut self, text: &[u16], text_index: usize, length: usize) -> Path {
        // Java: this.mPaint.setTextSize(100.0F);
        self.mPaint.setTextSize(CALC_FONT_SIZE);
        // Java: this.mPaint.setTypeface(this.mImpl);
        self.mPaint.setTypeface(self.mImpl);
        // Java: Path path = new Path();
        let mut path = Path::new();
        // Java: this.mPaint.getTextPath(text, textIndex, length, 0.0F, 0.0F, path);
        self.mPaint.getTextPath(text, text_index, length, 0.0, 0.0, &mut path);
        // Java: return path;
        path
    }

    // Java: Path getTextPathEM(char[] text, int textIndex, int length)
    fn getTextPathEM(&mut self, text: &[u16], text_index: usize, length: usize) -> Path {
        // Java: this.mPaint.setTextSize(this.ensureLayoutMetricsInfo().emSize);
        self.ensureLayoutMetricsInfo();
        let emsize = self.mMetricsInfo.as_ref().unwrap().emSize;
        self.mPaint.setTextSize(emsize);
        // Java: this.mPaint.setTypeface(this.mImpl);
        self.mPaint.setTypeface(self.mImpl);
        // Java: Path path = new Path();
        let mut path = Path::new();
        // Java: this.mPaint.getTextPath(text, textIndex, length, 0.0F, 0.0F, path);
        self.mPaint.getTextPath(text, text_index, length, 0.0, 0.0, &mut path);
        // Java: return path;
        path
    }

    // Java: RectF getTextRect(float size, char c)
    pub fn getTextRect_char(&mut self, size: f32, c: u16) -> RectF {
        // Java: if (this.mMetricsGetter != null && this.mMetricsGetter.getCharRect(c, this.mRect))
        {
            // Java: float scal = size / this.ensureLayoutMetricsInfo().srcEmSize;
            // Java: return new RectF((float)this.mRect.left * scal, (float)this.mRect.top * scal, (float)this.mRect.right * scal, (float)this.mRect.bottom * scal);
        }
        // Java: else
        {
            // Java: BucketRectCache rectCache = this.ensureBucketRectCache();
            // Java: RectF rectInCache = rectCache.findRect(c);
            // Java: if (null == rectInCache)
            {
                // Java: rectInCache = new RectF();
                let mut rect_in_cache = RectF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 };
                // Java: long typefacePtr = this.getNativeFreeTypeFontPointer(this.mStyle);
                let typeface_ptr = self.getNativeFreeTypeFontPointer(self.base.mStyle);
                // Java: boolean ret = typefacePtr != 0L ? FreeTypeJNI.getTextRect(typefacePtr, 100.0F, c, rectInCache) : false;
                let ret = if typeface_ptr != 0 { FreeTypeJNI::FreeTypeJNI::getTextRect(typeface_ptr, CALC_FONT_SIZE, c, &mut rect_in_cache) } else { false };
                // Java: if (!ret)
                if !ret {
                    // Java: Path path = new Path();
                    // Java: this.mTextBuf[0] = c;
                    self.mTextBuf[0] = c;
                    // Java: this.mPaint.setTextSize(100.0F);
                    self.mPaint.setTextSize(CALC_FONT_SIZE);
                    // Java: this.mPaint.setTypeface(this.mImpl);
                    self.mPaint.setTypeface(self.mImpl);
                    // Java: this.mPaint.getTextPath(this.mTextBuf, 0, 1, 0.0F, 0.0F, path);
                    // Java: android.graphics.RectF rectF = new android.graphics.RectF();
                    // Java: path.computeBounds(rectF, false);
                    // Java: rectInCache.left = rectF.left;
                    // Java: rectInCache.top = rectF.top;
                    // Java: rectInCache.right = rectF.right;
                    // Java: rectInCache.bottom = rectF.bottom;
                }
                // Java: rectCache.addRect(c, rectInCache);
                if let Some(ref mut rc) = self.mRectCache {
                    rc.addRect(c, &rect_in_cache);
                }
                // Java: float scal = size / 100.0F;
                let scal = size / CALC_FONT_SIZE;
                // Java: RectF rect = new RectF(rectInCache.left * scal, rectInCache.top * scal, rectInCache.right * scal, rectInCache.bottom * scal);
                // Java: return rect;
                return RectF { left: rect_in_cache.left * scal, top: rect_in_cache.top * scal, right: rect_in_cache.right * scal, bottom: rect_in_cache.bottom * scal };
            }
        }
    }

    // Java: RectF getTextRectEM(char c)
    pub fn getTextRectEM_char(&mut self, c: u16) -> RectF {
        // Java: if (this.mMetricsGetter != null && this.mMetricsGetter.getCharRect(c, this.mRect))
        {
            // Java: return new RectF((float)this.mRect.left, (float)this.mRect.top, (float)this.mRect.right, (float)this.mRect.bottom);
        }
        // Java: else
        {
            // Java: BucketRectCache rectCache = this.ensureBucketRectCache();
            // Java: RectF rectInCache = rectCache.findRect(c);
            // Java: if (null == rectInCache)
            {
                // Java: rectInCache = new RectF();
                let mut rect_in_cache = RectF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 };
                // Java: long typefacePtr = this.getNativeFreeTypeFontPointer(this.mStyle);
                let typeface_ptr = self.getNativeFreeTypeFontPointer(self.base.mStyle);
                // Java: boolean ret = FreeTypeJNI.getTextRectEM(typefacePtr, c, rectInCache);
                let ret = FreeTypeJNI::FreeTypeJNI::getTextRectEM(typeface_ptr, c, &mut rect_in_cache);
                // Java: if (!ret)
                if !ret {
                    // Java: Path path = new Path();
                    // Java: this.mTextBuf[0] = c;
                    self.mTextBuf[0] = c;
                    // Java: this.mPaint.setTextSize(this.ensureLayoutMetricsInfo().emSize);
                    self.ensureLayoutMetricsInfo();
                    let emsize = self.mMetricsInfo.as_ref().unwrap().emSize;
                    self.mPaint.setTextSize(emsize);
                    // Java: this.mPaint.setTypeface(this.mImpl);
                    self.mPaint.setTypeface(self.mImpl);
                    // Java: this.mPaint.getTextPath(this.mTextBuf, 0, 1, 0.0F, 0.0F, path);
                    // Java: android.graphics.RectF rectF = new android.graphics.RectF();
                    // Java: path.computeBounds(rectF, false);
                    // Java: rectInCache.left = rectF.left;
                    // Java: rectInCache.top = rectF.top;
                    // Java: rectInCache.right = rectF.right;
                    // Java: rectInCache.bottom = rectF.bottom;
                }
                // Java: rectCache.addRect(c, rectInCache);
                if let Some(ref mut rc) = self.mRectCache {
                    rc.addRect(c, &rect_in_cache);
                }
                // Java: RectF rect = new RectF(rectInCache.left, rectInCache.top, rectInCache.right, rectInCache.bottom);
                // Java: return rect;
                return RectF { left: rect_in_cache.left, top: rect_in_cache.top, right: rect_in_cache.right, bottom: rect_in_cache.bottom };
            }
        }
    }

    // Java: Object getNativeTypeface()
    pub fn getNativeTypeface(&self) -> i64 {
        // Java: return this.mImpl;
        self.mImpl
    }

    // Java: AbstractTypeface clone()
    pub fn clone_typeface(&self) -> AndroidTypeface {
        // Java: AndroidTypeface face = new AndroidTypeface(this.mParent, this.mStyle, this.mImpl, this.mCreateFromFontFile);
        let mut face = AndroidTypeface::new_with(self.base.mParent.clone(), self.base.mStyle, self.mImpl, self.base.mCreateFromFontFile);
        // Java: face.setNeedBold(this.needBold());
        face.base.needBold = self.base.needBold;
        // Java: face.setNeedItalic(this.needItalic());
        face.base.needItalic = self.base.needItalic;
        // Java: if (null != this.mMetricsInfo)
        if let Some(ref mi) = self.mMetricsInfo {
            // Java: face.mMetricsInfo = this.mMetricsInfo.clone();
            face.mMetricsInfo = Some(mi.clone());
        }
        // Java: if (null != this.mWppLayoutMetricInfo)
        if let Some(ref wmi) = self.mWppLayoutMetricInfo {
            // Java: face.mWppLayoutMetricInfo = this.mWppLayoutMetricInfo.clone();
            face.mWppLayoutMetricInfo = Some(wmi.clone());
        }
        // Java: return face;
        face
    }

    // Java: ITypeface deriveFallbackTypeface(IFontFamily fontFamily)
    pub fn deriveFallbackTypeface(&self, font_family: Option<Arc<dyn IFontFamily>>) -> AndroidTypeface {
        // Java: AndroidTypeface typeface = (AndroidTypeface)this.clone();
        let mut typeface = self.clone_typeface();
        // Java: typeface.mParent = fontFamily;
        typeface.base.mParent = font_family;
        // Java: typeface.mMetricsInfo = null;
        typeface.mMetricsInfo = None;
        // Java: typeface.mWppLayoutMetricInfo = null;
        typeface.mWppLayoutMetricInfo = None;
        // Java: return typeface;
        typeface
    }

    // Java: void initLalayoutMetricsInfo()
    fn initLalayoutMetricsInfo(&mut self) {
        // Java: if (null == this.mMetricsInfo)
        if self.mMetricsInfo.is_none() {
            // Java: LayoutMetricsInfo metrics = new LayoutMetricsInfo();
            let mut metrics = LayoutMetricsInfo::new();
            // Java: FreeTypeJNI.TTFHeader ttfHeader = sTTFHeader;
            // Java: synchronized(ttfHeader)
            {
                let ttf_header_guard = get_s_ttf_header().lock().unwrap();
                // Java: FreeTypeJNI.TTFHeader buildInHeader = null;
                // Java: if (this.mMetricsGetter != null && (buildInHeader = this.mMetricsGetter.getTTFHeader()) != null)
                {
                    // Java: this.doCalcLayoutMetricsInfo(buildInHeader, metrics);
                }
                // Java: else
                {
                    // Java: long typefacePtr = this.mParent.getNativeFreeTypeFontPointer(this.mStyle);
                    let typeface_ptr = self.base.mParent.as_ref().map_or(0i64, |p| p.getNativeFreeTypeFontPointer(self.base.mStyle));
                    // Java: if (typefacePtr != 0L && FreeTypeJNI.getTTFHeader(typefacePtr, ttfHeader))
                    {
                        // Java: this.doCalcLayoutMetricsInfo(ttfHeader, metrics);
                    }
                    // Java: else if (null != (buildInHeader = BuildinTTFHeader.getBuildinTTFHeader(this.getFamily().getName(), this.mStyle)))
                    {
                        // Java: this.doCalcLayoutMetricsInfo(buildInHeader, metrics);
                    }
                    // Java: else
                    {
                        // Java: metrics.isCJKFont = false;
                        metrics.isCJKFont = false;
                        // Java: metrics.isLatinFont = false;
                        metrics.isLatinFont = false;
                        // Java: metrics.isComplexFont = false;
                        metrics.isComplexFont = false;
                        // Java: metrics.latinExternal = 0.0F;
                        metrics.latinExternal = 0.0;
                        // Java: metrics.charSpaceRatio = 4.0F;
                        metrics.charSpaceRatio = 4.0;
                        // Java: metrics.ascent = this.mFontMetrics.getMaxAscent();
                        metrics.ascent = self.base.mFontMetrics.getMaxAscent();
                        // Java: metrics.descent = this.mFontMetrics.getMaxDescent();
                        metrics.descent = self.base.mFontMetrics.getMaxDescent();
                        // Java: metrics.subscriptFactor = 0.6F;
                        metrics.subscriptFactor = 0.6;
                        // Java: metrics.superscriptFactor = 0.6F;
                        metrics.superscriptFactor = 0.6;
                        // Java: metrics.emSize = 100.0F;
                        metrics.emSize = CALC_FONT_SIZE;
                        // Java: metrics.underline_position = -10;
                        metrics.underline_position = -10;
                        // Java: metrics.underline_thickness = 5;
                        metrics.underline_thickness = 5;
                        // Java: metrics.yStrikeoutPosition = 24;
                        metrics.yStrikeoutPosition = 24;
                        // Java: metrics.yStrikeoutSize = 5;
                        metrics.yStrikeoutSize = 5;
                    }
                }
            }
            // Java: this.mMetricsInfo = metrics;
            // Java: this.mMetricsInfo.srcEmSize = this.mMetricsInfo.emSize;
            metrics.srcEmSize = metrics.emSize;
            self.mMetricsInfo = Some(metrics);
        }
    }

    // Java: LayoutMetricsInfo ensureLayoutMetricsInfo()
    fn ensureLayoutMetricsInfo(&mut self) {
        // Java: if (null == this.mMetricsInfo)
        if self.mMetricsInfo.is_none() {
            // Java: this.initLalayoutMetricsInfo();
            self.initLalayoutMetricsInfo();
        }
        // Java: return this.mMetricsInfo;
    }

    // Java: void doCalcLayoutMetricsInfo(FreeTypeJNI.TTFHeader ttfHeader, LayoutMetricsInfo metrics)
    fn doCalcLayoutMetricsInfo(&self, ttf_header: &TTFHeader, metrics: &mut LayoutMetricsInfo) {
        // Java: boolean isCJKFont = isCJKCodePageRange(ttfHeader.codePageRange1);
        let is_cjk_font = AbstractTypeface::isCJKCodePageRange(ttf_header.codePageRange1);
        // Java: boolean isLatinFont = isLatinCodePageRange(ttfHeader.codePageRange1, ttfHeader.codePageRange2);
        let is_latin_font = AbstractTypeface::isLatinCodePageRange(ttf_header.codePageRange1, ttf_header.codePageRange2);
        // Java: boolean isComplexFont = isComplexUnicodeRange(ttfHeader.unicodeRange1, ttfHeader.unicodeRange2, ttfHeader.unicodeRange3);
        let is_complex_font = AbstractTypeface::isComplexUnicodeRange(ttf_header.unicodeRange1, ttf_header.unicodeRange2, ttf_header.unicodeRange3);
        // Java: float winAscent = ttfHeader.winAscent;
        let win_ascent = ttf_header.winAscent;
        // Java: float winDescent = ttfHeader.winDescent;
        let win_descent = ttf_header.winDescent;
        // Java: float tmAscent = ttfHeader.tmAscent;
        let tm_ascent = ttf_header.tmAscent;
        // Java: float tmDescent = ttfHeader.tmDescent;
        let tm_descent = ttf_header.tmDescent;
        // Java: float lineGap = ttfHeader.lineGap;
        let line_gap = ttf_header.lineGap;
        // Java: long ascentAndDescent = doGetLatinAscentAndDescent(winAscent, winDescent, tmAscent, tmDescent, isCJKFont);
        let ascent_and_descent = AbstractTypeface::doGetLatinAscentAndDescent(win_ascent, win_descent, tm_ascent, tm_descent, is_cjk_font);
        // Java: float ascent = PrimitivePackTool.unpackFirstFloat(ascentAndDescent);
        let ascent = PrimitivePackTool::PrimitivePackTool::unpackFirstFloat(ascent_and_descent);
        // Java: float descent = PrimitivePackTool.unpackSecondFloat(ascentAndDescent);
        let descent = PrimitivePackTool::PrimitivePackTool::unpackSecondFloat(ascent_and_descent);
        // Java: float externalLeading = doGetLatinExternal(winAscent, winDescent, tmAscent, tmDescent, lineGap, isCJKFont);
        let external_leading = AbstractTypeface::doGetLatinExternal(win_ascent, win_descent, tm_ascent, tm_descent, line_gap, is_cjk_font);
        // Java: metrics.isCJKFont = isCJKFont;
        metrics.isCJKFont = is_cjk_font;
        // Java: metrics.isLatinFont = isLatinFont;
        metrics.isLatinFont = is_latin_font;
        // Java: metrics.isComplexFont = isComplexFont;
        metrics.isComplexFont = is_complex_font;
        // Java: metrics.latinExternal = externalLeading;
        metrics.latinExternal = external_leading;
        // Java: metrics.ascent = ascent;
        metrics.ascent = ascent;
        // Java: metrics.descent = descent;
        metrics.descent = descent;
        // Java: metrics.subscriptFactor = doGetScriptFactor(ttfHeader.subscriptYOffset, ttfHeader.subscriptYSize, ttfHeader.superscriptYOffset, ttfHeader.superscriptYSize, ttfHeader.unitsPerEM, true);
        metrics.subscriptFactor = AbstractTypeface::doGetScriptFactor(ttf_header.subscriptYOffset, ttf_header.subscriptYSize, ttf_header.superscriptYOffset, ttf_header.superscriptYSize, ttf_header.unitsPerEM, true);
        // Java: metrics.superscriptFactor = doGetScriptFactor(ttfHeader.subscriptYOffset, ttfHeader.subscriptYSize, ttfHeader.superscriptYOffset, ttfHeader.superscriptYSize, ttfHeader.unitsPerEM, false);
        metrics.superscriptFactor = AbstractTypeface::doGetScriptFactor(ttf_header.subscriptYOffset, ttf_header.subscriptYSize, ttf_header.superscriptYOffset, ttf_header.superscriptYSize, ttf_header.unitsPerEM, false);
        // Java: metrics.charSpaceRatio = doGetCharSpaceRatio(ttfHeader.avgCharWidth, ttfHeader.unitsPerEM);
        metrics.charSpaceRatio = AbstractTypeface::doGetCharSpaceRatio(ttf_header.avgCharWidth, ttf_header.unitsPerEM);
        // Java: metrics.emSize = ttfHeader.unitsPerEM;
        metrics.emSize = ttf_header.unitsPerEM;
        // Java: metrics.underline_position = ttfHeader.underline_position;
        metrics.underline_position = ttf_header.underline_position;
        // Java: metrics.underline_thickness = ttfHeader.underline_thickness;
        metrics.underline_thickness = ttf_header.underline_thickness;
        // Java: metrics.yStrikeoutPosition = ttfHeader.yStrikeoutPosition;
        metrics.yStrikeoutPosition = ttf_header.yStrikeoutPosition;
        // Java: metrics.yStrikeoutSize = ttfHeader.yStrikeoutSize;
        metrics.yStrikeoutSize = ttf_header.yStrikeoutSize;
        // Java: metrics.lineGap = ttfHeader.lineGap;
        metrics.lineGap = ttf_header.lineGap;
        // Java: metrics.hheaAscent = ttfHeader.tmAscent;
        metrics.hheaAscent = ttf_header.tmAscent;
        // Java: metrics.hheaDescent = ttfHeader.tmDescent;
        metrics.hheaDescent = ttf_header.tmDescent;
        // Java: metrics.winAscent = ttfHeader.winAscent;
        metrics.winAscent = ttf_header.winAscent;
        // Java: metrics.winDescent = ttfHeader.winDescent;
        metrics.winDescent = ttf_header.winDescent;
        // Java: metrics.fsSelection = ttfHeader.fsSelection;
        metrics.fsSelection = ttf_header.fsSelection;
        // Java: String name = this.getFamily().getName();
        let name = self.base.mParent.as_ref().map_or(String::new(), |p| p.getName());
        // Java: if (FontName.isSpecialFontName(name))
        if FontName::isSpecialFontName(&name) {
            // Java: metrics.isLatinFont = true;
            metrics.isLatinFont = true;
            // Java: UnicodeBlockMap.parseRangeToList(metrics.supportlangs, name);
            UnicodeBlockMap::parseRangeToList_name(&mut metrics.supportlangs, &name);
        }
        // Java: else
        else {
            // Java: UnicodeBlockMap.parseRangeToList(ttfHeader.unicodeRange1, 0, 31, metrics.supportlangs);
            UnicodeBlockMap::parseRangeToList_range(ttf_header.unicodeRange1, 0, 31, &mut metrics.supportlangs);
            // Java: UnicodeBlockMap.parseRangeToList(ttfHeader.unicodeRange2, 32, 63, metrics.supportlangs);
            UnicodeBlockMap::parseRangeToList_range(ttf_header.unicodeRange2, 32, 63, &mut metrics.supportlangs);
            // Java: UnicodeBlockMap.parseRangeToList(ttfHeader.unicodeRange3, 64, 95, metrics.supportlangs);
            UnicodeBlockMap::parseRangeToList_range(ttf_header.unicodeRange3, 64, 95, &mut metrics.supportlangs);
            // Java: UnicodeBlockMap.parseRangeToList(ttfHeader.unicodeRange4, 96, 127, metrics.supportlangs);
            UnicodeBlockMap::parseRangeToList_range(ttf_header.unicodeRange4, 96, 127, &mut metrics.supportlangs);
        }
    }

    // Java: WppLayoutMetricInfo ensureWppLayoutMetricsInfo()
    fn ensureWppLayoutMetricsInfo(&mut self) {
        // Java: if (null == this.mWppLayoutMetricInfo)
        if self.mWppLayoutMetricInfo.is_none() {
            // Java: this.initWppLayoutMetricsInfo();
            self.initWppLayoutMetricsInfo();
        }
        // Java: return this.mWppLayoutMetricInfo;
    }

    // Java: void initWppLayoutMetricsInfo()
    fn initWppLayoutMetricsInfo(&mut self) {
        // Java: this.ensureLayoutMetricsInfo();
        self.ensureLayoutMetricsInfo();
        // Java: FreeTypeJNI.TypefaceInfo typefaceInfo = new FreeTypeJNI.TypefaceInfo();
        let mut typeface_info = TypefaceInfo::new();
        // Java: WppLayoutMetricInfo wppMetricInfo = new WppLayoutMetricInfo();
        let mut wpp_metric_info = WppLayoutMetricInfo::new();
        // Java: synchronized(sTTFHeader)
        {
            let _guard = get_s_ttf_header().lock().unwrap();
            // Java: long typefacePtr = this.mParent.getNativeFreeTypeFontPointer(this.mStyle);
            let typeface_ptr = self.base.mParent.as_ref().map_or(0i64, |p| p.getNativeFreeTypeFontPointer(self.base.mStyle));
            // Java: if (typefacePtr != 0L && FreeTypeJNI.getTypefaceInfo(typefacePtr, typefaceInfo))
            if typeface_ptr != 0 && FreeTypeJNI::FreeTypeJNI::getTypefaceInfo(typeface_ptr, &mut typeface_info) {
                // Java: this.doCalcWppLayoutMetricsInfo(this.mMetricsInfo, typefaceInfo, wppMetricInfo);
                if let Some(ref mi) = self.mMetricsInfo {
                    let ascent = mi.ascent;
                    let descent = mi.descent;
                    let line_gap = mi.lineGap;
                    let win_ascent = mi.winAscent;
                    let win_descent = mi.winDescent;
                    let hhea_ascent = mi.hheaAscent;
                    let hhea_descent = mi.hheaDescent;
                    let is_cjk = mi.isCJKFont;
                    Self::do_calc_wpp_static(is_cjk, win_ascent, win_descent, hhea_ascent, hhea_descent, line_gap, &typeface_info, &mut wpp_metric_info);
                }
                // Java: this.mTypefaceInfo = typefaceInfo;
                self.mTypefaceInfo = Some(typeface_info);
            }
            // Java: else
            else {
                // Java: wppMetricInfo.height = this.mMetricsInfo.ascent + this.mMetricsInfo.descent;
                if let Some(ref mi) = self.mMetricsInfo {
                    wpp_metric_info.height = mi.ascent + mi.descent;
                    // Java: wppMetricInfo.stakckedHeight = this.mMetricsInfo.ascent + this.mMetricsInfo.descent;
                    wpp_metric_info.stakckedHeight = mi.ascent + mi.descent;
                }
            }
        }
        // Java: this.mWppLayoutMetricInfo = wppMetricInfo;
        self.mWppLayoutMetricInfo = Some(wpp_metric_info);
    }

    // Java: void doCalcWppLayoutMetricsInfo(LayoutMetricsInfo metricsInfo, FreeTypeJNI.TypefaceInfo typefaceInfo, WppLayoutMetricInfo wppMetricInfo)
    fn do_calc_wpp_static(is_cjk: bool, win_ascent: f32, win_descent: f32, hhea_ascent: f32, hhea_descent: f32, line_gap: f32, typeface_info: &TypefaceInfo, wpp_metric_info: &mut WppLayoutMetricInfo) {
        // Java: float hheadLineGap = Math.max(0.0F, metricsInfo.lineGap - (metricsInfo.winAscent + metricsInfo.winDescent - metricsInfo.hheaAscent - metricsInfo.hheaDescent));
        let hhead_line_gap = f32::max(0.0, line_gap - (win_ascent + win_descent - hhea_ascent - hhea_descent));
        // Java: metricsInfo.lineGap = (float)Math.max((double)hheadLineGap * (double)1.5F, (double)metricsInfo.winAscent * 0.2);
        let adjusted_line_gap = f64::max((hhead_line_gap as f64) * 1.5, (win_ascent as f64) * 0.2) as f32;
        // Java: if (this.isCJKFont())
        if is_cjk {
            // Java: float ascent = metricsInfo.winAscent;
            let ascent = win_ascent;
            // Java: float descent = metricsInfo.winDescent;
            let descent = win_descent;
            // Java: float descentPadding = (float)(0.2996 * (double)((ascent + descent) / 2.0F));
            let descent_padding = (0.2996 * ((ascent + descent) / 2.0) as f64) as f32;
            // Java: wppMetricInfo.height = ascent + descent + descentPadding / 4.0F;
            wpp_metric_info.height = ascent + descent + descent_padding / 4.0;
        }
        // Java: else
        else {
            // Java: float ascent = typefaceInfo.ascent;
            let ascent = typeface_info.ascent;
            // Java: float descent = typefaceInfo.descent;
            let descent = typeface_info.descent;
            // Java: wppMetricInfo.height = ascent + descent + typefaceInfo.sTypoLineGap;
            wpp_metric_info.height = ascent + descent + typeface_info.sTypoLineGap;
        }
        // Java: wppMetricInfo.stakckedHeight = metricsInfo.hheaAscent + metricsInfo.hheaDescent + metricsInfo.lineGap;
        wpp_metric_info.stakckedHeight = hhea_ascent + hhea_descent + adjusted_line_gap;
    }

    // Java: boolean isCJKFont()
    pub fn isCJKFont(&mut self) -> bool {
        // Java: return this.ensureLayoutMetricsInfo().isCJKFont;
        self.ensureLayoutMetricsInfo();
        self.mMetricsInfo.as_ref().unwrap().isCJKFont
    }

    // Java: boolean isLatinFont()
    pub fn isLatinFont(&mut self) -> bool {
        // Java: return this.ensureLayoutMetricsInfo().isLatinFont;
        self.ensureLayoutMetricsInfo();
        self.mMetricsInfo.as_ref().unwrap().isLatinFont
    }

    // Java: boolean isComplexFont()
    pub fn isComplexFont(&mut self) -> bool {
        // Java: return this.ensureLayoutMetricsInfo().isComplexFont;
        self.ensureLayoutMetricsInfo();
        self.mMetricsInfo.as_ref().unwrap().isComplexFont
    }

    // Java: boolean hasUnicodeBlock(int c)
    pub fn hasUnicodeBlock(&mut self, c: i32) -> bool {
        // Java: for(UnicodeBlockMap.BlockItem item : this.ensureLayoutMetricsInfo().supportlangs)
        self.ensureLayoutMetricsInfo();
        if let Some(ref mi) = self.mMetricsInfo {
            for item in &mi.supportlangs {
                // Java: if (item.contains(c))
                if item.contains(c) {
                    // Java: return true;
                    return true;
                }
            }
        }
        // Java: return false;
        false
    }

    // Java: float getWriterLayoutAscent()
    pub fn getWriterLayoutAscent(&mut self) -> f32 {
        // Java: return this.ensureLayoutMetricsInfo().ascent;
        self.ensureLayoutMetricsInfo();
        self.mMetricsInfo.as_ref().unwrap().ascent
    }

    // Java: float getWriterLayoutDescent()
    pub fn getWriterLayoutDescent(&mut self) -> f32 {
        // Java: return this.ensureLayoutMetricsInfo().descent;
        self.ensureLayoutMetricsInfo();
        self.mMetricsInfo.as_ref().unwrap().descent
    }

    // Java: float getWriterLayoutLatinExternal()
    pub fn getWriterLayoutLatinExternal(&mut self) -> f32 {
        // Java: return this.ensureLayoutMetricsInfo().latinExternal;
        self.ensureLayoutMetricsInfo();
        self.mMetricsInfo.as_ref().unwrap().latinExternal
    }

    // Java: float getWriterLayoutEMSize()
    pub fn getWriterLayoutEMSize(&mut self) -> f32 {
        // Java: return this.ensureLayoutMetricsInfo().emSize;
        self.ensureLayoutMetricsInfo();
        self.mMetricsInfo.as_ref().unwrap().emSize
    }

    // Java: float getDefaultFareastCharWidth(float size)
    pub fn getDefaultFareastCharWidth(&mut self, size: f32) -> f32 {
        // Java: float scal = size / 100.0F;
        let scal = size / CALC_FONT_SIZE;
        // Java: if (this.mFarEastWidth >= 0.0F)
        if self.mFarEastWidth >= 0.0 {
            // Java: return this.mFarEastWidth * scal;
            return self.mFarEastWidth * scal;
        }
        // Java: else
        {
            // Java: long typefacePtr = this.mParent.getNativeFreeTypeFontPointer(this.mStyle);
            let typeface_ptr = self.base.mParent.as_ref().map_or(0i64, |p| p.getNativeFreeTypeFontPointer(self.base.mStyle));
            // Java: float width = FreeTypeJNI.getGlyphAdvance(typefacePtr, 100.0F, '我');
            let mut width = FreeTypeJNI::FreeTypeJNI::getGlyphAdvance(typeface_ptr, CALC_FONT_SIZE, 0x6211);
            // Java: if (width == -1.0F)
            if width == -1.0 {
                // Java: this.mPaint.setTextSize(100.0F);
                self.mPaint.setTextSize(CALC_FONT_SIZE);
                // Java: this.mPaint.setTypeface(this.mImpl);
                self.mPaint.setTypeface(self.mImpl);
                // Java: width = this.mPaint.measureText(DEF_FAREAST_CHAR_ARRAY, 0, 1);
                width = 0.0; // stub
                // Java: if (width <= 0.0F)
                if width <= 0.0 {
                    // Java: return size;
                    return size;
                }
            }
            // Java: this.mFarEastWidth = width;
            self.mFarEastWidth = width;
            // Java: return this.mFarEastWidth * scal;
            self.mFarEastWidth * scal
        }
    }

    // Java: int getDefaultFareastCharWidthEM()
    pub fn getDefaultFareastCharWidthEM(&mut self) -> i32 {
        // Java: if (this.mFarEastWidth >= 0.0F)
        if self.mFarEastWidth >= 0.0 {
            // Java: return (int)(this.mFarEastWidth + 0.5F);
            return (self.mFarEastWidth + 0.5) as i32;
        }
        // Java: else
        {
            // Java: long typefacePtr = this.mParent.getNativeFreeTypeFontPointer(this.mStyle);
            let typeface_ptr = self.base.mParent.as_ref().map_or(0i64, |p| p.getNativeFreeTypeFontPointer(self.base.mStyle));
            // Java: float emsize = this.ensureLayoutMetricsInfo().emSize;
            self.ensureLayoutMetricsInfo();
            let emsize = self.mMetricsInfo.as_ref().unwrap().emSize;
            // Java: int width = FreeTypeJNI.getGlyphAdvanceEM(typefacePtr, '我');
            let mut width = FreeTypeJNI::FreeTypeJNI::getGlyphAdvanceEM(typeface_ptr, 0x6211);
            // Java: if (width == -1)
            if width == -1 {
                // Java: this.mPaint.setTextSize(emsize);
                self.mPaint.setTextSize(emsize);
                // Java: this.mPaint.setTypeface(this.mImpl);
                self.mPaint.setTypeface(self.mImpl);
                // Java: width = (int)(this.mPaint.measureText(DEF_FAREAST_CHAR_ARRAY, 0, 1) + 0.5F);
                width = 0; // stub
                // Java: if ((float)width <= 0.0F)
                if (width as f32) <= 0.0 {
                    // Java: return (int)(emsize + 0.5F);
                    return (emsize + 0.5) as i32;
                }
            }
            // Java: this.mFarEastWidth = (float)width;
            self.mFarEastWidth = width as f32;
            // Java: return (int)(this.mFarEastWidth + 0.5F);
            (self.mFarEastWidth + 0.5) as i32
        }
    }

    // Java: void getWriterLayoutAdavances(float size, char[] text, int textIndex, float[] widths, int widthIndex, int length)
    pub fn getWriterLayoutAdavances_float(&mut self, size: f32, text: &[u16], text_index: usize, widths: &mut [f32], width_index: usize, length: usize) {
        // Java: float scal = size / 100.0F;
        let scal = size / CALC_FONT_SIZE;
        // Java: BucketWidthCache widthCache = this.ensureBucketWidthCache();
        self.ensureBucketWidthCache();
        // Java: int posWidth = widthIndex;
        let mut pos_width = width_index;
        // Java: boolean usePaintOnced = false;
        let mut use_paint_onced = false;
        // Java: if (null == this.mNativePointer)
        if self.base.mNativePointer.is_none() {
            // Java: this.mNativePointer = this.getNativeFreeTypeFontPointer(this.mStyle);
            self.base.mNativePointer = Some(self.getNativeFreeTypeFontPointer(self.base.mStyle));
        }
        let native_ptr = self.base.mNativePointer.unwrap_or(0);
        // Java: int i = textIndex;
        // Java: for(int n = textIndex + length; i < n; ++i)
        for i in text_index..(text_index + length) {
            // Java: float width = widthCache.findWidth(text[i]);
            let mut width = self.mWidthCache.as_ref().map_or(-1.0, |c| c.findWidth(text[i]));
            // Java: if (width >= 0.0F)
            if width >= 0.0 {
                // Java: widths[posWidth++] = width * scal;
                widths[pos_width] = width * scal;
                pos_width += 1;
            }
            // Java: else
            else {
                // Java: if (usePaintOnced)
                if use_paint_onced {
                    // Java: width = this.mPaint.measureText(text, i, 1);
                    width = self.mPaint.measureText_slice(text, i, 1);
                }
                // Java: else
                else {
                    // Java: width = this.mNativePointer != 0L ? FreeTypeJNI.getGlyphAdvance(this.mNativePointer, 100.0F, text[i]) : -1.0F;
                    width = if native_ptr != 0 { FreeTypeJNI::FreeTypeJNI::getGlyphAdvance(native_ptr, CALC_FONT_SIZE, text[i]) } else { -1.0 };
                    // Java: if (width == -1.0F)
                    if width == -1.0 {
                        // Java: this.mPaint.setTextSize(100.0F);
                        self.mPaint.setTextSize(CALC_FONT_SIZE);
                        // Java: this.mPaint.setTypeface(this.mImpl);
                        self.mPaint.setTypeface(self.mImpl);
                        // Java: width = this.mPaint.measureText(text, i, 1);
                        width = self.mPaint.measureText_slice(text, i, 1);
                        // Java: if (width <= 0.0F)
                        if width <= 0.0 {
                            // Java: widths[posWidth++] = size;
                            widths[pos_width] = size;
                            pos_width += 1;
                            // Java: continue;
                            continue;
                        }
                        // Java: usePaintOnced = true;
                        use_paint_onced = true;
                    }
                }
                // Java: widthCache.addWidth(text[i], width);
                if let Some(ref mut wc) = self.mWidthCache { wc.addWidth(text[i], width); }
                // Java: widths[posWidth++] = width * scal;
                widths[pos_width] = width * scal;
                pos_width += 1;
            }
        }
    }

    // Java: void getWriterLayoutAdavances(int size, char[] text, int textIndex, int[] widths, int widthIndex, int length)
    pub fn getWriterLayoutAdavances_int(&mut self, size: i32, text: &[u16], text_index: usize, widths: &mut [i32], width_index: usize, length: usize) {
        // Java: float scal = (float)size / 100.0F;
        let scal = size as f32 / CALC_FONT_SIZE;
        // Java: BucketWidthCache widthCache = this.ensureBucketWidthCache();
        self.ensureBucketWidthCache();
        // Java: if (null == this.mNativePointer)
        if self.base.mNativePointer.is_none() {
            // Java: this.mNativePointer = this.getNativeFreeTypeFontPointer(this.mStyle);
            self.base.mNativePointer = Some(self.getNativeFreeTypeFontPointer(self.base.mStyle));
        }
        let native_ptr = self.base.mNativePointer.unwrap_or(0);
        // Java: int posWidth = widthIndex;
        let mut pos_width = width_index;
        // Java: boolean usePaintOnced = false;
        let mut use_paint_onced = false;
        // Java: int i = textIndex;
        // Java: for(int n = textIndex + length; i < n; ++i)
        for i in text_index..(text_index + length) {
            // Java: float width = widthCache.findWidth(text[i]);
            let mut width = self.mWidthCache.as_ref().map_or(-1.0, |c| c.findWidth(text[i]));
            // Java: if (width >= 0.0F)
            if width >= 0.0 {
                // Java: widths[posWidth++] = (int)(width * scal + 0.5F);
                widths[pos_width] = (width * scal + 0.5) as i32;
                pos_width += 1;
            }
            // Java: else
            else {
                // Java: if (usePaintOnced)
                if use_paint_onced {
                    // Java: width = this.mPaint.measureText(text, i, 1);
                    width = self.mPaint.measureText_slice(text, i, 1);
                }
                // Java: else
                else {
                    // Java: width = this.mNativePointer != 0L ? FreeTypeJNI.getGlyphAdvance(this.mNativePointer, 100.0F, text[i]) : -1.0F;
                    width = if native_ptr != 0 { FreeTypeJNI::FreeTypeJNI::getGlyphAdvance(native_ptr, CALC_FONT_SIZE, text[i]) } else { -1.0 };
                    // Java: if (width == -1.0F)
                    if width == -1.0 {
                        // Java: this.mPaint.setTextSize(100.0F);
                        self.mPaint.setTextSize(CALC_FONT_SIZE);
                        // Java: this.mPaint.setTypeface(this.mImpl);
                        self.mPaint.setTypeface(self.mImpl);
                        // Java: width = this.mPaint.measureText(text, i, 1);
                        width = self.mPaint.measureText_slice(text, i, 1);
                        // Java: if (width <= 0.0F)
                        if width <= 0.0 {
                            // Java: widths[posWidth++] = (int)((float)size + 0.5F);
                            widths[pos_width] = (size as f32 + 0.5) as i32;
                            pos_width += 1;
                            // Java: continue;
                            continue;
                        }
                        // Java: usePaintOnced = true;
                        use_paint_onced = true;
                    }
                }
                // Java: widthCache.addWidth(text[i], width);
                if let Some(ref mut wc) = self.mWidthCache { wc.addWidth(text[i], width); }
                // Java: widths[posWidth++] = (int)(width * scal + 0.5F);
                widths[pos_width] = (width * scal + 0.5) as i32;
                pos_width += 1;
            }
        }
    }

    // Java: void getWriterLayoutAdavances(int size, int[] text, int textIndex, int[] widths, int widthIndex, int length)
    pub fn getWriterLayoutAdavances_int_arr(&mut self, size: i32, text: &[i32], text_index: usize, widths: &mut [i32], width_index: usize, length: usize) {
        // Java: float scal = (float)size / 100.0F;
        let scal = size as f32 / CALC_FONT_SIZE;
        // Java: BucketWidthCache widthCache = this.ensureBucketWidthCache();
        self.ensureBucketWidthCache();
        // Java: if (null == this.mNativePointer)
        if self.base.mNativePointer.is_none() {
            // Java: this.mNativePointer = this.getNativeFreeTypeFontPointer(this.mStyle);
            self.base.mNativePointer = Some(self.getNativeFreeTypeFontPointer(self.base.mStyle));
        }
        let native_ptr = self.base.mNativePointer.unwrap_or(0);
        // Java: int posWidth = widthIndex;
        let mut pos_width = width_index;
        // Java: boolean usePaintOnced = false;
        let mut use_paint_onced = false;
        // Java: char[] tmpBuff = null;
        let mut tmp_buff: Vec<u16> = vec![0u16; 1];
        // Java: int i = textIndex;
        // Java: for(int n = textIndex + length; i < n; ++i)
        for i in text_index..(text_index + length) {
            // Java: float width = widthCache.findWidth((char)text[i]);
            let ch = text[i] as u16;
            let mut width = self.mWidthCache.as_ref().map_or(-1.0, |c| c.findWidth(ch));
            // Java: if (width >= 0.0F)
            if width >= 0.0 {
                // Java: widths[posWidth++] = (int)(width * scal + 0.5F);
                widths[pos_width] = (width * scal + 0.5) as i32;
                pos_width += 1;
            }
            // Java: else
            else {
                // Java: if (usePaintOnced)
                if use_paint_onced {
                    // Java: tmpBuff[0] = (char)text[i];
                    tmp_buff[0] = ch;
                    // Java: width = this.mPaint.measureText(tmpBuff, 0, 1);
                    width = self.mPaint.measureText_slice(&tmp_buff, 0, 1);
                }
                // Java: else
                else {
                    // Java: width = this.mNativePointer != null ? FreeTypeJNI.getGlyphAdvance(this.mNativePointer, 100.0F, (char)text[i]) : -1.0F;
                    width = if native_ptr != 0 { FreeTypeJNI::FreeTypeJNI::getGlyphAdvance(native_ptr, CALC_FONT_SIZE, ch) } else { -1.0 };
                    // Java: if (width == -1.0F)
                    if width == -1.0 {
                        // Java: this.mPaint.setTextSize(100.0F);
                        self.mPaint.setTextSize(CALC_FONT_SIZE);
                        // Java: this.mPaint.setTypeface(this.mImpl);
                        self.mPaint.setTypeface(self.mImpl);
                        // Java: tmpBuff = new char[]{(char)text[i]};
                        tmp_buff = vec![ch];
                        // Java: width = this.mPaint.measureText(tmpBuff, 0, 1);
                        width = self.mPaint.measureText_slice(&tmp_buff, 0, 1);
                        // Java: if (width <= 0.0F)
                        if width <= 0.0 {
                            // Java: widths[posWidth++] = (int)((float)size + 0.5F);
                            widths[pos_width] = (size as f32 + 0.5) as i32;
                            pos_width += 1;
                            // Java: continue;
                            continue;
                        }
                        // Java: usePaintOnced = true;
                        use_paint_onced = true;
                    }
                }
                // Java: widthCache.addWidth((char)text[i], width);
                if let Some(ref mut wc) = self.mWidthCache { wc.addWidth(ch, width); }
                // Java: widths[posWidth++] = (int)(width * scal + 0.5F);
                widths[pos_width] = (width * scal + 0.5) as i32;
                pos_width += 1;
            }
        }
    }

    // Java: void getWriterLayoutAdavancesEM(char[] text, int textIndex, int[] widths, int widthIndex, int length)
    pub fn getWriterLayoutAdavancesEM(&mut self, text: &[u16], text_index: usize, widths: &mut [i32], width_index: usize, length: usize) {
        // Java: BucketWidthCache widthCache = this.ensureBucketWidthCache();
        self.ensureBucketWidthCache();
        // Java: int posWidth = widthIndex;
        let mut pos_width = width_index;
        // Java: boolean usePaintOnced = false;
        let mut use_paint_onced = false;
        // Java: float emsize = this.ensureLayoutMetricsInfo().srcEmSize;
        self.ensureLayoutMetricsInfo();
        let emsize = self.mMetricsInfo.as_ref().unwrap().srcEmSize;
        // Java: int i = textIndex;
        // Java: for(int n = textIndex + length; i < n; ++i)
        for i in text_index..(text_index + length) {
            // Java: float width = widthCache.findWidth(text[i]);
            let mut width = self.mWidthCache.as_ref().map_or(-1.0, |c| c.findWidth(text[i]));
            // Java: char ch = text[i];
            let ch = text[i];
            // Java: if (width == -1.0F && this.mMetricsGetter != null)
            // Java: width = (float)this.mMetricsGetter.getGlyphAdvanceEM(ch);
            // Java: if (width >= 0.0F)
            if width >= 0.0 {
                // Java: widths[posWidth++] = (int)(width + 0.5F);
                widths[pos_width] = (width + 0.5) as i32;
                pos_width += 1;
            }
            // Java: else
            else {
                // Java: if (usePaintOnced)
                if use_paint_onced {
                    // Java: width = this.mPaint.measureText(text, i, 1);
                    width = self.mPaint.measureText_slice(text, i, 1);
                }
                // Java: else
                else {
                    // Java: if (null == this.mNativePointer)
                    if self.base.mNativePointer.is_none() {
                        // Java: this.mNativePointer = this.getNativeFreeTypeFontPointer(this.mStyle);
                        self.base.mNativePointer = Some(self.getNativeFreeTypeFontPointer(self.base.mStyle));
                    }
                    let native_ptr = self.base.mNativePointer.unwrap_or(0);
                    // Java: width = this.mNativePointer != 0L ? (float)FreeTypeJNI.getGlyphAdvanceEM(this.mNativePointer, text[i]) : -1.0F;
                    width = if native_ptr != 0 { FreeTypeJNI::FreeTypeJNI::getGlyphAdvanceEM(native_ptr, text[i]) as f32 } else { -1.0 };
                    // Java: if (Float.compare(width, -1.0F) == 0)
                    if width == -1.0 {
                        // Java: this.mPaint.setTextSize(emsize);
                        self.mPaint.setTextSize(emsize);
                        // Java: this.mPaint.setTypeface(this.mImpl);
                        self.mPaint.setTypeface(self.mImpl);
                        // Java: width = this.mPaint.measureText(text, i, 1);
                        width = self.mPaint.measureText_slice(text, i, 1);
                        // Java: if (width <= 0.0F)
                        if width <= 0.0 {
                            // Java: widths[posWidth++] = (int)(emsize + 0.5F);
                            widths[pos_width] = (emsize + 0.5) as i32;
                            pos_width += 1;
                            // Java: continue;
                            continue;
                        }
                        // Java: usePaintOnced = true;
                        use_paint_onced = true;
                    }
                }
                // Java: widthCache.addWidth(ch, width);
                if let Some(ref mut wc) = self.mWidthCache { wc.addWidth(ch, width); }
                // Java: widths[posWidth++] = (int)(width + 0.5F);
                widths[pos_width] = (width + 0.5) as i32;
                pos_width += 1;
            }
        }
    }

    // Java: int getCharWidth(char c, boolean useAndroidPaint)
    pub fn getCharWidth(&mut self, c: u16, use_android_paint: bool) -> i32 {
        // Java: if (null == this.mNativePointer)
        if self.base.mNativePointer.is_none() {
            // Java: this.mNativePointer = this.getNativeFreeTypeFontPointer(this.mStyle);
            self.base.mNativePointer = Some(self.getNativeFreeTypeFontPointer(self.base.mStyle));
        }
        let native_ptr = self.base.mNativePointer.unwrap_or(0);
        // Java: int ret = this.mNativePointer != 0L ? FreeTypeJNI.getGlyphAdvanceEM(this.mNativePointer, c) : -1;
        let mut ret = if native_ptr != 0 { FreeTypeJNI::FreeTypeJNI::getGlyphAdvanceEM(native_ptr, c) } else { -1 };
        // Java: if (ret == -1 && useAndroidPaint)
        if ret == -1 && use_android_paint {
            // Java: float emsize = this.ensureLayoutMetricsInfo().srcEmSize;
            self.ensureLayoutMetricsInfo();
            let emsize = self.mMetricsInfo.as_ref().unwrap().srcEmSize;
            // Java: this.mPaint.setTextSize(emsize);
            self.mPaint.setTextSize(emsize);
            // Java: this.mPaint.setTypeface(this.mImpl);
            self.mPaint.setTypeface(self.mImpl);
            // Java: char[] tmpBuff = new char[1];
            let mut tmp_buff: Vec<u16> = vec![0u16; 1];
            // Java: tmpBuff[0] = c;
            tmp_buff[0] = c;
            // Java: ret = (int)this.mPaint.measureText(tmpBuff, 0, 1);
            ret = self.mPaint.measureText_slice(&tmp_buff, 0, 1) as i32;
        }
        // Java: return ret;
        ret
    }

    // Java: RectF getCharRect(char c, boolean useAndroidPaint)
    pub fn getCharRect(&mut self, c: u16, use_android_paint: bool) -> Option<RectF> {
        // Java: RectF rectInCache = new RectF();
        let mut rect_in_cache = RectF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 };
        // Java: long typefacePtr = this.getNativeFreeTypeFontPointer(this.mStyle);
        let typeface_ptr = self.getNativeFreeTypeFontPointer(self.base.mStyle);
        // Java: boolean ret = typefacePtr != 0L ? FreeTypeJNI.getTextRectEM(typefacePtr, c, rectInCache) : false;
        let mut ret = if typeface_ptr != 0 { FreeTypeJNI::FreeTypeJNI::getTextRectEM(typeface_ptr, c, &mut rect_in_cache) } else { false };
        // Java: if (!ret && useAndroidPaint)
        if !ret && use_android_paint {
            // Java: Path path = new Path();
            // Java: this.mTextBuf[0] = c;
            self.mTextBuf[0] = c;
            // Java: this.mPaint.setTextSize(this.ensureLayoutMetricsInfo().srcEmSize);
            self.ensureLayoutMetricsInfo();
            let em = self.mMetricsInfo.as_ref().unwrap().srcEmSize;
            self.mPaint.setTextSize(em);
            // Java: this.mPaint.setTypeface(this.mImpl);
            self.mPaint.setTypeface(self.mImpl);
            // Java: this.mPaint.getTextPath(this.mTextBuf, 0, 1, 0.0F, 0.0F, path);
            // Java: android.graphics.RectF rectF = new android.graphics.RectF();
            // Java: path.computeBounds(rectF, false);
            // Java: rectInCache.left = rectF.left;
            // Java: rectInCache.top = rectF.top;
            // Java: rectInCache.right = rectF.right;
            // Java: rectInCache.bottom = rectF.bottom;
            // Java: ret = true;
            ret = true;
        }
        // Java: return ret ? rectInCache : null;
        if ret { Some(rect_in_cache) } else { None }
    }

    // Java: FreeTypeJNI.TTFHeader getTTFHeader(boolean useBuildHeader)
    pub fn getTTFHeader(&self, _use_build_header: bool) -> Option<TTFHeader> {
        // Java: long typefacePtr = this.mParent.getNativeFreeTypeFontPointer(this.mStyle);
        // Java: return ttfHeader;
        None
    }

    // Java: void getWriterLayoutAdavancesEM(int[] text, int textIndex, int[] widths, int widthIndex, int length)
    pub fn getWriterLayoutAdavancesEM_int(&mut self, text: &[i32], text_index: usize, widths: &mut [i32], width_index: usize, length: usize) {
        // Java: BucketWidthCache widthCache = this.ensureBucketWidthCache();
        self.ensureBucketWidthCache();
        // Java: int posWidth = widthIndex;
        let mut pos_width = width_index;
        // Java: boolean usePaintOnced = false;
        let mut use_paint_onced = false;
        // Java: char[] tmpBuff = null;
        let mut tmp_buff: Vec<u16> = vec![0u16; 1];
        // Java: float emsize = this.ensureLayoutMetricsInfo().srcEmSize;
        self.ensureLayoutMetricsInfo();
        let emsize = self.mMetricsInfo.as_ref().unwrap().srcEmSize;
        // Java: int i = textIndex;
        // Java: for(int n = textIndex + length; i < n; ++i)
        for i in text_index..(text_index + length) {
            // Java: char ch = (char)text[i];
            let ch = text[i] as u16;
            // Java: float width = widthCache.findWidth(ch);
            let mut width = self.mWidthCache.as_ref().map_or(-1.0, |c| c.findWidth(ch));
            // Java: if (width == -1.0F && this.mMetricsGetter != null)
            // Java: width = (float)this.mMetricsGetter.getGlyphAdvanceEM(ch);
            // Java: if (width >= 0.0F)
            if width >= 0.0 {
                // Java: widths[posWidth++] = (int)(width + 0.5F);
                widths[pos_width] = (width + 0.5) as i32;
                pos_width += 1;
            }
            // Java: else
            else {
                // Java: if (usePaintOnced)
                if use_paint_onced {
                    // Java: tmpBuff[0] = ch;
                    tmp_buff[0] = ch;
                    // Java: width = this.mPaint.measureText(tmpBuff, 0, 1);
                    width = self.mPaint.measureText_slice(&tmp_buff, 0, 1);
                }
                // Java: else
                else {
                    // Java: if (null == this.mNativePointer)
                    if self.base.mNativePointer.is_none() {
                        // Java: this.mNativePointer = this.getNativeFreeTypeFontPointer(this.mStyle);
                        self.base.mNativePointer = Some(self.getNativeFreeTypeFontPointer(self.base.mStyle));
                    }
                    let native_ptr = self.base.mNativePointer.unwrap_or(0);
                    // Java: width = this.mNativePointer != 0L ? (float)FreeTypeJNI.getGlyphAdvanceEM(this.mNativePointer, ch) : -1.0F;
                    width = if native_ptr != 0 { FreeTypeJNI::FreeTypeJNI::getGlyphAdvanceEM(native_ptr, ch) as f32 } else { -1.0 };
                    // Java: if (Float.compare(width, -1.0F) == 0)
                    if width == -1.0 {
                        // Java: this.mPaint.setTextSize(emsize);
                        self.mPaint.setTextSize(emsize);
                        // Java: this.mPaint.setTypeface(this.mImpl);
                        self.mPaint.setTypeface(self.mImpl);
                        // Java: tmpBuff = new char[]{(char)text[i]};
                        tmp_buff = vec![ch];
                        // Java: width = this.mPaint.measureText(tmpBuff, 0, 1);
                        width = self.mPaint.measureText_slice(&tmp_buff, 0, 1);
                        // Java: if (width <= 0.0F)
                        if width <= 0.0 {
                            // Java: widths[posWidth++] = (int)(emsize + 0.5F);
                            widths[pos_width] = (emsize + 0.5) as i32;
                            pos_width += 1;
                            // Java: continue;
                            continue;
                        }
                        // Java: usePaintOnced = true;
                        use_paint_onced = true;
                    }
                }
                // Java: widthCache.addWidth(ch, width);
                if let Some(ref mut wc) = self.mWidthCache { wc.addWidth(ch, width); }
                // Java: widths[posWidth++] = (int)(width + 0.5F);
                widths[pos_width] = (width + 0.5) as i32;
                pos_width += 1;
            }
        }
    }

    // Java: long getNativeFreeTypeFontPointer(int style)
    fn getNativeFreeTypeFontPointer(&self, _style: i32) -> i64 {
        // Java: long typefacePtr = this.mParent.getNativeFreeTypeFontPointer(this.mStyle);
        let typeface_ptr = self.base.mParent.as_ref().map_or(0i64, |p| p.getNativeFreeTypeFontPointer(self.base.mStyle));
        // Java: if (0L == typefacePtr && this.mParent.isBuildinHeader())
        if typeface_ptr == 0 {
            if let Some(ref parent) = self.base.mParent {
                if parent.isBuildinHeader() {
                    // Java: IFontFamily srcFamily = this.mParent.getSourceFont();
                    let src_family = parent.getSourceFont();
                    // Java: if (null != srcFamily)
                    if let Some(ref sf) = src_family {
                        // Java: typefacePtr = srcFamily.getNativeFreeTypeFontPointer(this.mStyle);
                        return sf.getNativeFreeTypeFontPointer(self.base.mStyle);
                    }
                }
            }
        }
        // Java: return typefacePtr;
        typeface_ptr
    }

    // Java: float getWriterLayoutCharSpaceRatio()
    pub fn getWriterLayoutCharSpaceRatio(&mut self) -> f32 {
        // Java: return this.ensureLayoutMetricsInfo().charSpaceRatio;
        self.ensureLayoutMetricsInfo();
        self.mMetricsInfo.as_ref().unwrap().charSpaceRatio
    }

    // Java: float getWriterLayoutSubscriptFactor()
    pub fn getWriterLayoutSubscriptFactor(&mut self) -> f32 {
        // Java: return this.ensureLayoutMetricsInfo().subscriptFactor;
        self.ensureLayoutMetricsInfo();
        self.mMetricsInfo.as_ref().unwrap().subscriptFactor
    }

    // Java: float getWriterLayoutSuperscriptFactor()
    pub fn getWriterLayoutSuperscriptFactor(&mut self) -> f32 {
        // Java: return this.ensureLayoutMetricsInfo().superscriptFactor;
        self.ensureLayoutMetricsInfo();
        self.mMetricsInfo.as_ref().unwrap().superscriptFactor
    }

    // Java: int getUnderlinePosition()
    pub fn getUnderlinePosition(&mut self) -> i32 {
        // Java: return this.ensureLayoutMetricsInfo().underline_position;
        self.ensureLayoutMetricsInfo();
        self.mMetricsInfo.as_ref().unwrap().underline_position
    }

    // Java: int getUnderlineThickness()
    pub fn getUnderlineThickness(&mut self) -> i32 {
        // Java: return this.ensureLayoutMetricsInfo().underline_thickness;
        self.ensureLayoutMetricsInfo();
        self.mMetricsInfo.as_ref().unwrap().underline_thickness
    }

    // Java: int getYStrikeoutSize()
    pub fn getYStrikeoutSize(&mut self) -> i32 {
        // Java: return this.ensureLayoutMetricsInfo().yStrikeoutSize;
        self.ensureLayoutMetricsInfo();
        self.mMetricsInfo.as_ref().unwrap().yStrikeoutSize
    }

    // Java: int getYStrikeoutPosition()
    pub fn getYStrikeoutPosition(&mut self) -> i32 {
        // Java: return this.ensureLayoutMetricsInfo().yStrikeoutPosition;
        self.ensureLayoutMetricsInfo();
        self.mMetricsInfo.as_ref().unwrap().yStrikeoutPosition
    }

    // Java: int getKerningEM(char left, char right)
    pub fn getKerningEM(&mut self, left: u16, right: u16) -> i32 {
        // Java: if (!this.mInitedKerning)
        if !self.mInitedKerning {
            // Java: this.initKeringPair();
            self.initKeringPair();
        }
        // Java: if (this.mMetricsGetter != null)
        {
            // Java: return this.mMetricsGetter.getKerningPair(left, right);
        }
        // Java: else if (null == this.mKerningCache)
        if self.mKerningCache.is_none() {
            // Java: return 0;
            return 0;
        }
        // Java: else
        {
            // Java: int kerning = this.mKerningCache.find(left, right);
            let kerning = self.mKerningCache.as_mut().unwrap().find(left, right);
            // Java: if (kerning != Integer.MIN_VALUE)
            if kerning != i32::MIN {
                // Java: return kerning;
                return kerning;
            }
            // Java: else
            {
                // Java: long typefacePtr = this.mParent.getNativeFreeTypeFontPointer(this.mStyle);
                let typeface_ptr = self.base.mParent.as_ref().map_or(0i64, |p| p.getNativeFreeTypeFontPointer(self.base.mStyle));
                // Java: kerning = FreeTypeJNI.getKerningEM(typefacePtr, left, right);
                let kerning = FreeTypeJNI::FreeTypeJNI::getKerningEM(typeface_ptr, left, right);
                // Java: this.mKerningCache.put(kerning);
                self.mKerningCache.as_mut().unwrap().put(kerning);
                // Java: return kerning;
                kerning
            }
        }
    }

    // Java: void initKeringPair()
    fn initKeringPair(&mut self) {
        // Java: if (!this.mInitedKerning)
        if !self.mInitedKerning {
            // Java: if (this.mMetricsGetter == null)
            if self.mMetricsGetter.is_none() {
                // Java: long ptr = this.mParent.getNativeFreeTypeFontPointer(this.mStyle);
                let ptr = self.base.mParent.as_ref().map_or(0i64, |p| p.getNativeFreeTypeFontPointer(self.base.mStyle));
                // Java: if (ptr != 0L && FreeTypeJNI.hasKerning(ptr))
                if ptr != 0 && FreeTypeJNI::FreeTypeJNI::hasKerning(ptr) {
                    // Java: this.mKerningCache = KerningCacheFactory.createCache(this.mParent.getName());
                    let name = self.base.mParent.as_ref().map_or(String::new(), |p| p.getName());
                    self.mKerningCache = KerningCacheFactory::createCache(&name);
                }
            }
            // Java: this.mInitedKerning = true;
            self.mInitedKerning = true;
        }
    }

    // Java: boolean hasKerning()
    pub fn hasKerning(&mut self) -> bool {
        // Java: if (!this.mInitedKerning)
        if !self.mInitedKerning {
            // Java: this.initKeringPair();
            self.initKeringPair();
        }
        // Java: return this.mKerningCache != null;
        self.mKerningCache.is_some()
    }

    // Java: float getWppLayoutHeight()
    pub fn getWppLayoutHeight(&mut self) -> f32 {
        // Java: return this.ensureWppLayoutMetricsInfo().height;
        self.ensureWppLayoutMetricsInfo();
        self.mWppLayoutMetricInfo.as_ref().unwrap().height
    }

    // Java: float getWppLayoutStakckedHeight()
    pub fn getWppLayoutStakckedHeight(&mut self) -> f32 {
        // Java: return this.ensureWppLayoutMetricsInfo().stakckedHeight;
        self.ensureWppLayoutMetricsInfo();
        self.mWppLayoutMetricInfo.as_ref().unwrap().stakckedHeight
    }
}

// Java: public class LayoutMetricsInfo
impl LayoutMetricsInfo {
    // Java: LayoutMetricsInfo()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.supportlangs = new LinkedList();
        LayoutMetricsInfo {
            isCJKFont: false, isLatinFont: false, isComplexFont: false,
            ascent: 0.0, descent: 0.0, latinExternal: 0.0,
            charSpaceRatio: 0.0, subscriptFactor: 0.0, superscriptFactor: 0.0,
            srcEmSize: 0.0, emSize: 0.0,
            underline_position: 0, underline_thickness: 0,
            yStrikeoutSize: 0, yStrikeoutPosition: 0,
            lineGap: 0.0, hheaAscent: 0.0, hheaDescent: 0.0,
            winAscent: 0.0, winDescent: 0.0, fsSelection: 0,
            supportlangs: Vec::new(),
        }
    }

    // Java: LayoutMetricsInfo clone()
    pub fn clone_metrics(&self) -> LayoutMetricsInfo {
        // Java: LayoutMetricsInfo ret = new LayoutMetricsInfo();
        let mut ret = LayoutMetricsInfo::new();
        // Java: ret.isCJKFont = this.isCJKFont;
        ret.isCJKFont = self.isCJKFont;
        // Java: ret.isLatinFont = this.isLatinFont;
        ret.isLatinFont = self.isLatinFont;
        // Java: ret.isComplexFont = this.isComplexFont;
        ret.isComplexFont = self.isComplexFont;
        // Java: ret.ascent = this.ascent;
        ret.ascent = self.ascent;
        // Java: ret.descent = this.descent;
        ret.descent = self.descent;
        // Java: ret.latinExternal = this.latinExternal;
        ret.latinExternal = self.latinExternal;
        // Java: ret.subscriptFactor = this.subscriptFactor;
        ret.subscriptFactor = self.subscriptFactor;
        // Java: ret.superscriptFactor = this.superscriptFactor;
        ret.superscriptFactor = self.superscriptFactor;
        // Java: ret.emSize = this.emSize;
        ret.emSize = self.emSize;
        // Java: ret.srcEmSize = this.srcEmSize;
        ret.srcEmSize = self.srcEmSize;
        // Java: ret.supportlangs.addAll(this.supportlangs);
        ret.supportlangs = self.supportlangs.clone();
        // Java: ret.underline_position = this.underline_position;
        ret.underline_position = self.underline_position;
        // Java: ret.underline_thickness = this.underline_thickness;
        ret.underline_thickness = self.underline_thickness;
        // Java: ret.yStrikeoutSize = this.yStrikeoutSize;
        ret.yStrikeoutSize = self.yStrikeoutSize;
        // Java: ret.yStrikeoutPosition = this.yStrikeoutPosition;
        ret.yStrikeoutPosition = self.yStrikeoutPosition;
        // Java: return ret;
        ret
    }
}

// Java: public class WppLayoutMetricInfo
impl WppLayoutMetricInfo {
    // Java: WppLayoutMetricInfo()
    pub fn new() -> Self {
        // Java: super();
        WppLayoutMetricInfo { height: 0.0, stakckedHeight: 0.0 }
    }

    // Java: WppLayoutMetricInfo clone()
    pub fn clone_metrics(&self) -> WppLayoutMetricInfo {
        // Java: WppLayoutMetricInfo ret = new WppLayoutMetricInfo();
        let mut ret = WppLayoutMetricInfo::new();
        // Java: ret.height = this.height;
        ret.height = self.height;
        // Java: ret.stakckedHeight = this.stakckedHeight;
        ret.stakckedHeight = self.stakckedHeight;
        // Java: return ret;
        ret
    }
}
