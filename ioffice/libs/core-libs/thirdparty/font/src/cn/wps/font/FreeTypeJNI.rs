// Java: import android.graphics.Path
// Java: import cn.wps.graphics.RectF
// Java: import cn.wps.moffice.util.ISoUtil
// Java: import java.util.Arrays
use std::sync::{Mutex, OnceLock};

pub struct Path;
#[derive(Clone)]
pub struct RectF {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl RectF {
    pub fn setEmpty(&mut self) {
        self.left = 0.0;
        self.top = 0.0;
        self.right = 0.0;
        self.bottom = 0.0;
    }
}

// Java field: String TAG
pub(crate) const TAG: &str = "FreeTypeJNI";
// Java field: String LIBRARY_NAME
pub(crate) const LIBRARY_NAME: &str = "freetype-jni";
// Java field: boolean sIsLoadLibSucceed
static sIsLoadLibSucceed: OnceLock<Mutex<bool>> = OnceLock::new();
// Java field: float[] sArray
static sArray: OnceLock<Mutex<Vec<f32>>> = OnceLock::new();
// Java field: int[] sArrayInt
static sArrayInt: OnceLock<Mutex<Vec<i32>>> = OnceLock::new();

fn get_sIsLoadLibSucceed() -> &'static Mutex<bool> {
    sIsLoadLibSucceed.get_or_init(|| Mutex::new(false))
}

fn get_sArray() -> &'static Mutex<Vec<f32>> {
    sArray.get_or_init(|| Mutex::new(vec![0.0f32; 4]))
}

fn get_sArrayInt() -> &'static Mutex<Vec<i32>> {
    sArrayInt.get_or_init(|| Mutex::new(vec![0i32; 4]))
}

// Java: public class FreeTypeJNI
pub struct FreeTypeJNI {
}

// Java: public class TTFHeader
pub struct TTFHeader {
    // Java field: int codePageRange1
    pub codePageRange1: i32,
    // Java field: float winAscent
    pub winAscent: f32,
    // Java field: float winDescent
    pub winDescent: f32,
    // Java field: float tmAscent
    pub tmAscent: f32,
    // Java field: float tmDescent
    pub tmDescent: f32,
    // Java field: float lineGap
    pub lineGap: f32,
    // Java field: float avgCharWidth
    pub avgCharWidth: f32,
    // Java field: float subscriptYOffset
    pub subscriptYOffset: f32,
    // Java field: float subscriptYSize
    pub subscriptYSize: f32,
    // Java field: float superscriptYOffset
    pub superscriptYOffset: f32,
    // Java field: float superscriptYSize
    pub superscriptYSize: f32,
    // Java field: float unitsPerEM
    pub unitsPerEM: f32,
    // Java field: int codePageRange2
    pub codePageRange2: i32,
    // Java field: int unicodeRange1
    pub unicodeRange1: i32,
    // Java field: int unicodeRange2
    pub unicodeRange2: i32,
    // Java field: int unicodeRange3
    pub unicodeRange3: i32,
    // Java field: int unicodeRange4
    pub unicodeRange4: i32,
    // Java field: int underline_position
    pub underline_position: i32,
    // Java field: int underline_thickness
    pub underline_thickness: i32,
    // Java field: int yStrikeoutSize
    pub yStrikeoutSize: i32,
    // Java field: int yStrikeoutPosition
    pub yStrikeoutPosition: i32,
    // Java field: int fsSelection
    pub fsSelection: i32,
}

// Java: public class TypefaceInfo
pub struct TypefaceInfo {
    // Java field: int flags
    pub(crate) flags: i32,
    // Java field: float italicAngle
    pub(crate) italicAngle: f32,
    // Java field: float capHeight
    pub(crate) capHeight: f32,
    // Java field: float ascent
    pub(crate) ascent: f32,
    // Java field: float descent
    pub(crate) descent: f32,
    // Java field: float xMin
    pub(crate) xMin: f32,
    // Java field: float xMax
    pub(crate) xMax: f32,
    // Java field: float yMin
    pub(crate) yMin: f32,
    // Java field: float yMax
    pub(crate) yMax: f32,
    // Java field: float sTypoLineGap
    pub(crate) sTypoLineGap: f32,
}

extern "C" {
    fn native_initalize() -> bool;
    fn native_doGetVersion(var0: *mut i32) -> bool;
    fn native_doGetTTFHeader(var0: i64, var2: *mut TTFHeader) -> bool;
    fn native_doGetTypefaceInfo(var0: i64, var2: *mut TypefaceInfo) -> bool;
    fn native_doGetGlyphPaths(var0: i64, var2: *mut Path, var3: i32) -> bool;
    fn native_doGetGlyphIndexes(var0: i64, var2: *mut i32, var3: i32, var4: i32, var5: *mut i32, var6: i32) -> bool;
    fn native_doGetGlyphAdvances(var0: i64, var2: f32, var3: *const u16, var4: i32, var5: *mut f32, var6: i32, var7: i32) -> bool;
    fn native_doGetGlyphAdvance(var0: i64, var2: f32, var3: u16) -> f32;
    fn native_doGetGlyphAdvancesEM(var0: i64, var2: *const u16, var3: i32, var4: *mut i32, var5: i32, var6: i32) -> bool;
    fn native_doGetGlyphAdvanceEM(var0: i64, var2: u16) -> i32;
    fn native_doLoadGlyph(var0: i64, var2: i32) -> bool;
    fn native_doNewTypeface(var0: *const u8, var0_len: i32, var1: i32) -> i64;
    fn native_doDeleteTypeface(var0: i64) -> bool;
    fn native_doWriteTTF(var0: i64, var2: *const u8, var2_len: i32, var3: *const i32, var4: i32) -> bool;
    fn native_doGetGlyphNum(var0: i64) -> i64;
    fn native_doGetUnicode(var0: i64, var2: i32) -> u16;
    fn native_doGetHasKerning(var0: i64) -> bool;
    fn native_doGetKerning(var0: i64, var2: u16, var3: u16, var4: f32) -> f32;
    fn native_doGetKerningEM(var0: i64, var2: u16, var3: u16) -> i32;
    fn native_doGetTextRect(var0: i64, var2: f32, var3: u16, var4: *mut f32) -> bool;
    fn native_doGetTextRectEM(var0: i64, var2: u16, var3: *mut i32) -> bool;
}

// Java: public class FreeTypeJNI
impl FreeTypeJNI {
    // Java: FreeTypeJNI()
    pub fn new() -> Self {
        // Java: super();
        FreeTypeJNI {}
    }

    // Java: static private static synchronized native boolean initalize()
    // LLM Java synchronized: native method, Rust FFI is inherently single-threaded per call
    fn initalize() -> bool {
        let _lock = get_sIsLoadLibSucceed().lock().unwrap();
        unsafe { native_initalize() }
    }

    // Java: static private static native boolean doGetVersion(int[] var0)
    fn doGetVersion(var0: &mut [i32]) -> bool {
        unsafe { native_doGetVersion(var0.as_mut_ptr()) }
    }

    // Java: static private static native boolean doGetTTFHeader(long var0, TTFHeader var2)
    fn doGetTTFHeader(var0: i64, var2: &mut TTFHeader) -> bool {
        unsafe { native_doGetTTFHeader(var0, var2 as *mut TTFHeader) }
    }

    // Java: static private static native boolean doGetTypefaceInfo(long var0, TypefaceInfo var2)
    fn doGetTypefaceInfo(var0: i64, var2: &mut TypefaceInfo) -> bool {
        unsafe { native_doGetTypefaceInfo(var0, var2 as *mut TypefaceInfo) }
    }

    // Java: static private static native boolean doGetGlyphPaths(long var0, Path var2, int var3)
    fn doGetGlyphPaths(var0: i64, var2: &mut Path, var3: i32) -> bool {
        unsafe { native_doGetGlyphPaths(var0, var2 as *mut Path, var3) }
    }

    // Java: static private static native boolean doGetGlyphIndexes(long var0, int[] var2, int var3, int var4, int[] var5, int var6)
    fn doGetGlyphIndexes(var0: i64, var2: &mut [i32], var3: i32, var4: i32, var5: &mut [i32], var6: i32) -> bool {
        unsafe { native_doGetGlyphIndexes(var0, var2.as_mut_ptr(), var3, var4, var5.as_mut_ptr(), var6) }
    }

    // Java: static private static native boolean doGetGlyphAdvances(long var0, float var2, char[] var3, int var4, float[] var5, int var6, int var7)
    fn doGetGlyphAdvances(var0: i64, var2: f32, var3: &[u16], var4: i32, var5: &mut [f32], var6: i32, var7: i32) -> bool {
        unsafe { native_doGetGlyphAdvances(var0, var2, var3.as_ptr(), var4, var5.as_mut_ptr(), var6, var7) }
    }

    // Java: static private static native float doGetGlyphAdvance(long var0, float var2, char var3)
    fn doGetGlyphAdvance(var0: i64, var2: f32, var3: u16) -> f32 {
        unsafe { native_doGetGlyphAdvance(var0, var2, var3) }
    }

    // Java: static private static native boolean doGetGlyphAdvancesEM(long var0, char[] var2, int var3, int[] var4, int var5, int var6)
    fn doGetGlyphAdvancesEM(var0: i64, var2: &[u16], var3: i32, var4: &mut [i32], var5: i32, var6: i32) -> bool {
        unsafe { native_doGetGlyphAdvancesEM(var0, var2.as_ptr(), var3, var4.as_mut_ptr(), var5, var6) }
    }

    // Java: static private static native int doGetGlyphAdvanceEM(long var0, char var2)
    fn doGetGlyphAdvanceEM(var0: i64, var2: u16) -> i32 {
        unsafe { native_doGetGlyphAdvanceEM(var0, var2) }
    }

    // Java: static private static native boolean doLoadGlyph(long var0, int var2)
    fn doLoadGlyph(var0: i64, var2: i32) -> bool {
        unsafe { native_doLoadGlyph(var0, var2) }
    }

    // Java: static private static native long doNewTypeface(String var0, int var1)
    fn doNewTypeface(var0: &str, var1: i32) -> i64 {
        unsafe { native_doNewTypeface(var0.as_ptr(), var0.len() as i32, var1) }
    }

    // Java: static private static native boolean doDeleteTypeface(long var0)
    fn doDeleteTypeface(var0: i64) -> bool {
        unsafe { native_doDeleteTypeface(var0) }
    }

    // Java: static private static native boolean doWriteTTF(long var0, String var2, int[] var3, int var4)
    fn doWriteTTF(var0: i64, var2: &str, var3: &[i32], var4: i32) -> bool {
        unsafe { native_doWriteTTF(var0, var2.as_ptr(), var2.len() as i32, var3.as_ptr(), var4) }
    }

    // Java: static private static native long doGetGlyphNum(long var0)
    fn doGetGlyphNum(var0: i64) -> i64 {
        unsafe { native_doGetGlyphNum(var0) }
    }

    // Java: static private static native char doGetUnicode(long var0, int var2)
    fn doGetUnicode(var0: i64, var2: i32) -> u16 {
        unsafe { native_doGetUnicode(var0, var2) }
    }

    // Java: static private static native boolean doGetHasKerning(long var0)
    fn doGetHasKerning(var0: i64) -> bool {
        unsafe { native_doGetHasKerning(var0) }
    }

    // Java: static private static native float doGetKerning(long var0, char var2, char var3, float var4)
    fn doGetKerning(var0: i64, var2: u16, var3: u16, var4: f32) -> f32 {
        unsafe { native_doGetKerning(var0, var2, var3, var4) }
    }

    // Java: static private static native int doGetKerningEM(long var0, char var2, char var3)
    fn doGetKerningEM(var0: i64, var2: u16, var3: u16) -> i32 {
        unsafe { native_doGetKerningEM(var0, var2, var3) }
    }

    // Java: static private static native boolean doGetTextRect(long var0, float var2, char var3, float[] var4)
    fn doGetTextRect(var0: i64, var2: f32, var3: u16, var4: &mut [f32]) -> bool {
        unsafe { native_doGetTextRect(var0, var2, var3, var4.as_mut_ptr()) }
    }

    // Java: static private static native boolean doGetTextRectEM(long var0, char var2, int[] var3)
    fn doGetTextRectEM(var0: i64, var2: u16, var3: &mut [i32]) -> bool {
        unsafe { native_doGetTextRectEM(var0, var2, var3.as_mut_ptr()) }
    }

    // Java: static boolean getTTFHeader(long typefacePtr, TTFHeader header)
    // LLM Java synchronized: lock acquired via get_sIsLoadLibSucceed().lock()
    pub fn getTTFHeader_ptr(typefacePtr: i64, header: &mut TTFHeader) -> bool {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: return 0L != typefacePtr && null != header ? doGetTTFHeader(typefacePtr, header) : false;
            // LLM Rust Null-Safety: null != header check not needed, header is a reference
            if 0 != typefacePtr {
                Self::doGetTTFHeader(typefacePtr, header)
            } else {
                false
            }
        }
    }

    // Java: static boolean getTypefaceInfo(long typefacePtr, TypefaceInfo outInfo)
    pub fn getTypefaceInfo(typefacePtr: i64, outInfo: &mut TypefaceInfo) -> bool {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
        // Java: return 0L != typefacePtr && null != outInfo ? doGetTypefaceInfo(typefacePtr, outInfo) : false;
        // LLM Rust Null-Safety: null != outInfo check not needed
        if 0 != typefacePtr {
            Self::doGetTypefaceInfo(typefacePtr, outInfo)
        } else {
            false
        }
        }
    }

    // Java: static boolean getGlyphPaths(long typefacePtr, Path outPath, int glyph)
    pub fn getGlyphPaths(typefacePtr: i64, outPath: &mut Path, glyph: i32) -> bool {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
        // Java: return 0L != typefacePtr && null != outPath ? doGetGlyphPaths(typefacePtr, outPath, glyph) : false;
        // LLM Rust Null-Safety: null != outPath check not needed
        if 0 != typefacePtr {
            Self::doGetGlyphPaths(typefacePtr, outPath, glyph)
        } else {
            false
        }
        }
    }

    // Java: static boolean getGlyphIndexes(long typefacePtr, int[] unicodes, int unicodeStart, int len, int[] outGlyphIndexes, int glyphIdxStart)
    pub fn getGlyphIndexes(typefacePtr: i64, unicodes: &mut [i32], unicodeStart: i32, len: i32, outGlyphIndexes: &mut [i32], glyphIdxStart: i32) -> bool {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
        // Java: return 0L != typefacePtr && null != unicodes && null != outGlyphIndexes && unicodeStart >= 0 && glyphIdxStart >= 0 && unicodeStart + len <= unicodes.length && glyphIdxStart + len <= outGlyphIndexes.length ? doGetGlyphIndexes(typefacePtr, unicodes, unicodeStart, len, outGlyphIndexes, glyphIdxStart) : false;
        // LLM Rust Null-Safety: null checks not needed for slice references
        if 0 != typefacePtr
            && unicodeStart >= 0
            && glyphIdxStart >= 0
            && (unicodeStart + len) <= unicodes.len() as i32
            && (glyphIdxStart + len) <= outGlyphIndexes.len() as i32
        {
            Self::doGetGlyphIndexes(typefacePtr, unicodes, unicodeStart, len, outGlyphIndexes, glyphIdxStart)
        } else {
            false
        }
        }
    }

    // Java: static boolean getGlyphAdvances(long typefacePtr, float size, char[] text, int textIndex, float[] widths, int widthIndex, int length)
    pub fn getGlyphAdvances(typefacePtr: i64, size: f32, text: &[u16], textIndex: i32, widths: &mut [f32], widthIndex: i32, length: i32) -> bool {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
        // Java: boolean ret = false;
        let mut ret = false;
        // Java: if (0L != typefacePtr && !(size <= 0.0F) && null != text && null != widths && textIndex >= 0 && widthIndex >= 0 && textIndex + length <= text.length && widthIndex + length <= widths.length)
        // LLM Rust Null-Safety: null checks for text/widths not needed for slice references
        if 0 != typefacePtr
            && !(size <= 0.0f32)
            && true && true
            && textIndex >= 0
            && widthIndex >= 0
            && (textIndex + length) <= text.len() as i32
            && (widthIndex + length) <= widths.len() as i32
        {
            // Java: ret = doGetGlyphAdvances(typefacePtr, size, text, textIndex, widths, widthIndex, length);
            ret = Self::doGetGlyphAdvances(typefacePtr, size, text, textIndex, widths, widthIndex, length);
            // Java: if (!ret)
            if !ret {
                // Java: Arrays.fill(widths, widthIndex, widthIndex + length, 0.0F);
                // LLM Java Platform API: Arrays.fill replaced with Rust for-loop fill
                let start = widthIndex as usize;
                let end = (widthIndex + length) as usize;
                for i in start..end { widths[i] = 0.0f32; }
            }
        }
        // Java: else
        else {
            // Java: ret = false;
            ret = false;
        }
        // Java: return ret;
        ret
        }
    }

    // Java: static boolean getGlyphAdvancesEM(long typefacePtr, char[] text, int textIndex, int[] widths, int widthIndex, int length)
    pub fn getGlyphAdvancesEM(typefacePtr: i64, text: &[u16], textIndex: i32, widths: &mut [i32], widthIndex: i32, length: i32) -> bool {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
        // Java: boolean ret = false;
        let mut ret = false;
        // Java: if (0L != typefacePtr && null != text && null != widths && textIndex >= 0 && widthIndex >= 0 && textIndex + length <= text.length && widthIndex + length <= widths.length)
        // LLM Rust Null-Safety: null checks for text/widths not needed for slice references
        if 0 != typefacePtr
            && true && true
            && textIndex >= 0
            && widthIndex >= 0
            && (textIndex + length) <= text.len() as i32
            && (widthIndex + length) <= widths.len() as i32
        {
            // Java: ret = doGetGlyphAdvancesEM(typefacePtr, text, textIndex, widths, widthIndex, length);
            ret = Self::doGetGlyphAdvancesEM(typefacePtr, text, textIndex, widths, widthIndex, length);
            // Java: if (!ret)
            if !ret {
                // Java: Arrays.fill(widths, widthIndex, widthIndex + length, 0);
                // LLM Java Platform API: Arrays.fill replaced with Rust for-loop fill
                let start = widthIndex as usize;
                let end = (widthIndex + length) as usize;
                for i in start..end { widths[i] = 0; }
            }
        }
        // Java: else
        else {
            // Java: ret = false;
            ret = false;
        }
        // Java: return ret;
        ret
        }
    }

    // Java: static float getGlyphAdvance(long typefacePtr, float size, char text)
    pub fn getGlyphAdvance(typefacePtr: i64, size: f32, text: u16) -> f32 {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return -1.0F;
            return -1.0f32;
        }
        // Java: else
        else {
        // Java: return 0L != typefacePtr && !(size <= 0.0F) && 0 != text ? doGetGlyphAdvance(typefacePtr, size, text) : -1.0F;
        if 0 != typefacePtr && !(size <= 0.0f32) && 0 != text {
            Self::doGetGlyphAdvance(typefacePtr, size, text)
        } else {
            -1.0f32
        }
        }
    }

    // Java: static int getGlyphAdvanceEM(long typefacePtr, char text)
    pub fn getGlyphAdvanceEM(typefacePtr: i64, text: u16) -> i32 {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return -1;
            return -1;
        }
        // Java: else
        else {
        // Java: return 0L != typefacePtr && 0 != text ? doGetGlyphAdvanceEM(typefacePtr, text) : -1;
        if 0 != typefacePtr && 0 != text {
            Self::doGetGlyphAdvanceEM(typefacePtr, text)
        } else {
            -1
        }
        }
    }

    // Java: static boolean loadGlyph(long typefacePtr, int glyphIndex)
    pub fn loadGlyph(typefacePtr: i64, glyphIndex: i32) -> bool {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
        // Java: return 0L == typefacePtr ? false : doLoadGlyph(typefacePtr, glyphIndex);
        // LLM JavaLine Aggregation: 0L literal merged into Rust if-expression
        if 0 == typefacePtr { false } else { Self::doLoadGlyph(typefacePtr, glyphIndex) }
        }
    }

    // Java: static long newTypeface(String fullPath)
    pub fn newTypeface(fullPath: &str) -> i64 {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return 0L;
            return 0;
        }
        // Java: else
        else {
        // Java: return null == fullPath ? 0L : doNewTypeface(fullPath, 0);
        // LLM Rust Null-Safety: null check replaced with is_empty for &str
        if fullPath.is_empty() { 0 } else { Self::doNewTypeface(fullPath, 0) }
        }
    }

    // Java: static long newTypeface(String fullPath, int index)
    pub fn newTypeface_with_index(fullPath: &str, index: i32) -> i64 {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return 0L;
            return 0;
        }
        // Java: else
        else {
        // Java: return null == fullPath ? 0L : doNewTypeface(fullPath, index);
        // LLM Rust Null-Safety: null check replaced with is_empty for &str
        if fullPath.is_empty() { 0 } else { Self::doNewTypeface(fullPath, index) }
        }
    }

    // Java: static boolean deleteTypeface(long typefacePtr)
    pub fn deleteTypeface(typefacePtr: i64) -> bool {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
        // Java: return 0L == typefacePtr ? false : doDeleteTypeface(typefacePtr);
        // LLM JavaLine Aggregation: 0L literal merged into Rust if-expression
        if 0 == typefacePtr { false } else { Self::doDeleteTypeface(typefacePtr) }
        }
    }

    // Java: static boolean getTTFHeader(String fullPath, TTFHeader header)
    // LLM Java synchronized: lock acquired via get_sIsLoadLibSucceed().lock()
    pub fn getTTFHeader(fullPath: &str, header: &mut TTFHeader) -> bool {
        let _lock = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: long ptr = newTypeface(fullPath);
        let ptr = Self::newTypeface(fullPath);
        // Java: if (0L == ptr)
        if 0 == ptr {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
        // Java: boolean ret1 = getTTFHeader(ptr, header);
        let ret1 = Self::getTTFHeader_ptr(ptr, header);
        // Java: boolean ret2 = deleteTypeface(ptr);
        let ret2 = Self::deleteTypeface(ptr);
        // Java: return ret1 && ret2;
        ret1 && ret2
        }
    }

    // Java: static boolean writeTTF(long typefacePtr, String path, int[] glyphs)
    pub fn writeTTF(typefacePtr: i64, path: &str, glyphs: &[i32]) -> bool {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
        // Java: return 0L != typefacePtr && null != glyphs && glyphs.length >= 1 ? doWriteTTF(typefacePtr, path, glyphs, glyphs.length) : false;
        // LLM Rust Null-Safety: null != glyphs check not needed for slice reference
        if 0 != typefacePtr && true && glyphs.len() >= 1 {
            Self::doWriteTTF(typefacePtr, path, glyphs, glyphs.len() as i32)
        } else {
            false
        }
        }
    }

    // Java: static long getGlyphNum(long typefacePtr)
    pub fn getGlyphNum(typefacePtr: i64) -> i64 {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return 0L;
            return 0;
        }
        // Java: else
        else {
        // Java: return 0L == typefacePtr ? 0L : doGetGlyphNum(typefacePtr);
        // LLM JavaLine Aggregation: 0L literal merged into Rust if-expression
        if 0 == typefacePtr { 0 } else { Self::doGetGlyphNum(typefacePtr) }
        }
    }

    // Java: static char getUnicode(long typefacePtr, int glyph)
    pub fn getUnicode(typefacePtr: i64, glyph: i32) -> u16 {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return '\u0000';
            return 0u16;
        }
        // Java: else
        else {
        // Java: return 0L == typefacePtr ? '\u0000' : doGetUnicode(typefacePtr, glyph);
        // LLM JavaLine Aggregation: 0L literal merged into Rust if-expression
        if 0 == typefacePtr { 0u16 } else { Self::doGetUnicode(typefacePtr, glyph) }
        }
    }

    // Java: static boolean getTextRect(long typefacePtr, float size, char c, RectF rect)
    pub fn getTextRect(typefacePtr: i64, size: f32, c: u16, rect: &mut RectF) -> bool {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
        // Java: boolean ret = false;
        let mut ret = false;
        // Java: if (0L != typefacePtr && !(size <= 0.0F) && null != rect)
        // LLM Rust Null-Safety: null != rect check not needed, rect is a reference
        if 0 != typefacePtr && !(size <= 0.0f32) && true {
            let mut arr = get_sArray().lock().unwrap();
            // Java: ret = doGetTextRect(typefacePtr, size, c, sArray);
            ret = Self::doGetTextRect(typefacePtr, size, c, &mut arr);
            // Java: if (ret)
            if ret {
                // Java: rect.left = sArray[0];
                rect.left = arr[0];
                // Java: rect.top = sArray[1];
                rect.top = arr[1];
                // Java: rect.right = sArray[2];
                rect.right = arr[2];
                // Java: rect.bottom = sArray[3];
                rect.bottom = arr[3];
            }
            // Java: else
            else {
                // Java: rect.setEmpty();
                rect.setEmpty();
            }
        }
        // Java: else
        else {
            // Java: ret = false;
            ret = false;
        }
        // Java: return ret;
        ret
        }
    }

    // Java: static String getVersion()
    pub fn getVersion() -> Option<String> {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return null;
            return None;
        }
        // Java: else
        else {
        let mut arr = get_sArrayInt().lock().unwrap();
        // Java: return doGetVersion(sArrayInt) ? "" + sArrayInt[0] + "." + sArrayInt[1] + "." + sArrayInt[2] : null;
        if Self::doGetVersion(&mut arr) {
            Some(format!("{}.{}.{}", arr[0], arr[1], arr[2]))
        } else {
            None
        }
        }
    }

    // Java: static boolean getTextRectEM(long typefacePtr, char c, RectF rect)
    pub fn getTextRectEM(typefacePtr: i64, c: u16, rect: &mut RectF) -> bool {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
        // Java: boolean ret = false;
        let mut ret = false;
        // Java: if (0L != typefacePtr && null != rect)
        // LLM Rust Null-Safety: null != rect check not needed, rect is a reference
        if 0 != typefacePtr && true {
            let mut arr = get_sArrayInt().lock().unwrap();
            // Java: ret = doGetTextRectEM(typefacePtr, c, sArrayInt);
            ret = Self::doGetTextRectEM(typefacePtr, c, &mut arr);
            // Java: if (ret)
            if ret {
                // Java: rect.left = (float)sArrayInt[0];
                rect.left = arr[0] as f32;
                // Java: rect.top = (float)sArrayInt[1];
                rect.top = arr[1] as f32;
                // Java: rect.right = (float)sArrayInt[2];
                rect.right = arr[2] as f32;
                // Java: rect.bottom = (float)sArrayInt[3];
                rect.bottom = arr[3] as f32;
            }
            // Java: else
            else {
                // Java: rect.setEmpty();
                rect.setEmpty();
            }
        }
        // Java: else
        else {
            // Java: ret = false;
            ret = false;
        }
        // Java: return ret;
        ret
        }
    }

    // Java: static float getKerning(long typefacePtr, char left, char right, float calcFontSize)
    pub fn getKerning(typefacePtr: i64, left: u16, right: u16, calcFontSize: f32) -> f32 {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return 0.0F;
            return 0.0f32;
        }
        // Java: else
        else {
        // Java: return 0L == typefacePtr ? 0.0F : doGetKerning(typefacePtr, left, right, calcFontSize);
        // LLM JavaLine Aggregation: 0L literal merged into Rust if-expression
        if 0 == typefacePtr { 0.0f32 } else { Self::doGetKerning(typefacePtr, left, right, calcFontSize) }
        }
    }

    // Java: static int getKerningEM(long typefacePtr, char left, char right)
    pub fn getKerningEM(typefacePtr: i64, left: u16, right: u16) -> i32 {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return 0;
            return 0;
        }
        // Java: else
        else {
        // Java: return 0L == typefacePtr ? 0 : doGetKerningEM(typefacePtr, left, right);
        // LLM JavaLine Aggregation: 0L literal merged into Rust if-expression
        if 0 == typefacePtr { 0 } else { Self::doGetKerningEM(typefacePtr, left, right) }
        }
    }

    // Java: static boolean hasKerning(long typefacePtr)
    pub fn hasKerning(typefacePtr: i64) -> bool {
        let loaded = get_sIsLoadLibSucceed().lock().unwrap();
        // Java: if (!sIsLoadLibSucceed)
        if !*loaded {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
        // Java: return 0L == typefacePtr ? false : doGetHasKerning(typefacePtr);
        // LLM JavaLine Aggregation: 0L literal merged into Rust if-expression
        if 0 == typefacePtr { false } else { Self::doGetHasKerning(typefacePtr) }
        }
    }
}

// Java: public class TTFHeader
impl TTFHeader {
    // Java: TTFHeader()
    pub fn new() -> Self {
        // Java: super();
        TTFHeader {
            codePageRange1: 0,
            winAscent: 0.0,
            winDescent: 0.0,
            tmAscent: 0.0,
            tmDescent: 0.0,
            lineGap: 0.0,
            avgCharWidth: 0.0,
            subscriptYOffset: 0.0,
            subscriptYSize: 0.0,
            superscriptYOffset: 0.0,
            superscriptYSize: 0.0,
            unitsPerEM: 0.0,
            codePageRange2: 0,
            unicodeRange1: 0,
            unicodeRange2: 0,
            unicodeRange3: 0,
            unicodeRange4: 0,
            underline_position: 0,
            underline_thickness: 0,
            yStrikeoutSize: 0,
            yStrikeoutPosition: 0,
            fsSelection: 0,
        }
    }
}

// Java: public class TypefaceInfo
impl TypefaceInfo {
    // Java: TypefaceInfo()
    pub fn new() -> Self {
        // Java: super();
        TypefaceInfo {
            flags: 0,
            italicAngle: 0.0,
            capHeight: 0.0,
            ascent: 0.0,
            descent: 0.0,
            xMin: 0.0,
            xMax: 0.0,
            yMin: 0.0,
            yMax: 0.0,
            sTypoLineGap: 0.0,
        }
    }
}
