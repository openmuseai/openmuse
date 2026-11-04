#![allow(dead_code, unused_imports, unused_variables, non_snake_case, unused_mut)]

// Java: import cn.wps.font.FontHost
use thirdparty_font::cn::wps::font::FontHost::FontHost;
// Java: import cn.wps.font.FontMetrics
use thirdparty_font::cn::wps::font::ITypeface::FontMetrics;
// Java: import cn.wps.font.IFontFamily
use thirdparty_font::cn::wps::font::IFontFamily::IFontFamily;
// Java: import cn.wps.font.IFontTable
use thirdparty_font::cn::wps::font::IFontTable::IFontTable;
// Java: import cn.wps.font.ITypeface
use thirdparty_font::cn::wps::font::ITypeface::ITypeface;
// Java: import cn.wps.graphics.RectF
use thirdparty_font::cn::wps::font::FreeTypeJNI::RectF;
use thirdparty_font::cn::wps::font::r#impl::FontTableImpl::FontTableImpl;
use std::sync::{Arc, Mutex, OnceLock};

fn typeface_tool_slot() -> &'static Mutex<Option<Arc<TypefaceTool>>> {
    static SLOT: OnceLock<Mutex<Option<Arc<TypefaceTool>>>> = OnceLock::new();
    SLOT.get_or_init(|| Mutex::new(None))
}

fn chars_as_utf16(text: &[char]) -> Vec<u16> {
    text.iter().map(|c| *c as u32 as u16).collect()
}

fn rect_width(rect: &RectF) -> f32 {
    rect.right - rect.left
}

// java static volatile TypefaceTool sTypefaceTool = null;
// java public class TypefaceTool
pub struct TypefaceTool {
    // java IFontTable mFontTable = FontHost.getFontTable();
    pub(crate) mFontTable: Arc<FontTableImpl>,
}

impl Default for TypefaceTool {
    fn default() -> Self {
        Self::TypefaceTool()
    }
}

// java public class TypefaceTool
impl TypefaceTool {
    fn n_font_style(bold: bool, italic: bool) -> i32 {
        let mut nFontStyle: i32 = 0;
        if bold {
            nFontStyle += 1;
        }
        if italic {
            nFontStyle += 2;
        }
        nFontStyle
    }

    fn typeface(&self, font_name: &str, bold: bool, italic: bool) -> Arc<dyn ITypeface> {
        let fontFamily = self
            .mFontTable
            .get_by_name_bool(font_name, true)
            .expect("IFontFamily");
        fontFamily
            .getTypeface(Self::n_font_style(bold, italic))
            .expect("ITypeface")
    }

    // java private  TypefaceTool()
    pub fn TypefaceTool() -> Self {
        Self {
            mFontTable: FontHost::getFontTable(),
        }
    }

    // java public static TypefaceTool getInstance()
    pub fn getInstance() -> Arc<TypefaceTool> {
        let mut slot = typeface_tool_slot().lock().unwrap();
        if slot.is_none() {
            *slot = Some(Arc::new(TypefaceTool::TypefaceTool()));
        }
        slot.as_ref().unwrap().clone()
    }

    // java public static synchronized void dispose()
    pub fn dispose() {
        *typeface_tool_slot().lock().unwrap() = None;
    }

    // java public float[] getCharWidths(String text, String fontName, float fontSize, boolean bold, boolean italic, float fontSpace)
    pub fn getCharWidths__string_string_float_boolean_boolean_float(
        &self,
        text: &str,
        fontName: &str,
        fontSize: f32,
        bold: bool,
        italic: bool,
        fontSpace: f32,
    ) -> Vec<f32> {
        let chars: Vec<char> = text.chars().collect();
        self.getCharWidths__chararr_string_float_boolean_boolean_float(
            &chars, fontName, fontSize, bold, italic, fontSpace,
        )
    }

    // java public float[] getCharWidths(char[] text, String fontName, float fontSize, boolean bold, boolean italic, float fontSpace)
    pub fn getCharWidths__chararr_string_float_boolean_boolean_float(
        &self,
        text: &Vec<char>,
        fontName: &str,
        fontSize: f32,
        bold: bool,
        italic: bool,
        fontSpace: f32,
    ) -> Vec<f32> {
        self.getCharWidths__chararr_int_int_string_float_boolean_boolean_float(
            text,
            0,
            text.len() as i32,
            fontName,
            fontSize,
            bold,
            italic,
            fontSpace,
        )
    }

    // java public float[] getCharWidths(char[] text, int charsPos, int charsLen, String fontName, float fontSize, boolean bold, boolean italic, float fontSpace)
    pub fn getCharWidths__chararr_int_int_string_float_boolean_boolean_float(
        &self,
        text: &Vec<char>,
        charsPos: i32,
        charsLen: i32,
        fontName: &str,
        fontSize: f32,
        bold: bool,
        italic: bool,
        fontSpace: f32,
    ) -> Vec<f32> {
        let mut widths: Vec<f32> = vec![0.0; charsLen as usize];
        self.getCharWidths__chararr_int_int_string_float_boolean_boolean_floatarr(
            text, charsPos, charsLen, fontName, fontSize, bold, italic, &mut widths,
        );
        let mut i: i32 = 0;
        while i < charsLen {
            widths[i as usize] += fontSpace;
            i += 1;
        }
        widths
    }

    // java public float[] getCharWidths(String text, String fontName, float fontSize, boolean bold, boolean italic)
    pub fn getCharWidths__string_string_float_boolean_boolean(
        &self,
        text: &str,
        fontName: &str,
        fontSize: f32,
        bold: bool,
        italic: bool,
    ) -> Vec<f32> {
        self.getCharWidths__string_string_float_boolean_boolean_float(
            text, fontName, fontSize, bold, italic, 0.0,
        )
    }

    // java public float[] getCharWidths(char[] text, String fontName, float fontSize, boolean bold, boolean italic)
    pub fn getCharWidths__chararr_string_float_boolean_boolean(
        &self,
        text: &Vec<char>,
        fontName: &str,
        fontSize: f32,
        bold: bool,
        italic: bool,
    ) -> Vec<f32> {
        self.getCharWidths__chararr_string_float_boolean_boolean_float(
            text, fontName, fontSize, bold, italic, 0.0,
        )
    }

    // java public float[] getCharWidths(char[] text, int charsPos, int charsLen, String fontName, float fontSize, boolean bold, boolean italic)
    pub fn getCharWidths__chararr_int_int_string_float_boolean_boolean(
        &self,
        text: &Vec<char>,
        charsPos: i32,
        charsLen: i32,
        fontName: &str,
        fontSize: f32,
        bold: bool,
        italic: bool,
    ) -> Vec<f32> {
        self.getCharWidths__chararr_int_int_string_float_boolean_boolean_float(
            text, charsPos, charsLen, fontName, fontSize, bold, italic, 0.0,
        )
    }

    // java public void getCharWidths(char[] text, int charsPos, int charsLen, String fontName, float fontSize, boolean bold, boolean italic, float[] widths)
    pub fn getCharWidths__chararr_int_int_string_float_boolean_boolean_floatarr(
        &self,
        text: &Vec<char>,
        charsPos: i32,
        charsLen: i32,
        fontName: &str,
        fontSize: f32,
        bold: bool,
        italic: bool,
        widths: &mut [f32],
    ) {
        let units = chars_as_utf16(text);
        let face = self.typeface(fontName, bold, italic);
        face.getWriterLayoutAdavances_float(fontSize, &units, 0, widths, charsPos, charsLen);
    }

    // java public float getTextWidth(String text, String fontname, float fontSize, boolean bold, boolean italic, float fontSpace)
    pub fn getTextWidth__string_string_float_boolean_boolean_float(
        &self,
        text: &str,
        fontname: &str,
        fontSize: f32,
        bold: bool,
        italic: bool,
        fontSpace: f32,
    ) -> f32 {
        let chars: Vec<char> = text.chars().collect();
        self.getTextWidth__chararr_string_float_boolean_boolean_float(
            &chars, fontname, fontSize, bold, italic, fontSpace,
        )
    }

    // java public float getTextWidth(char[] text, String fontname, float fontSize, boolean bold, boolean italic, float fontSpace)
    pub fn getTextWidth__chararr_string_float_boolean_boolean_float(
        &self,
        text: &Vec<char>,
        fontname: &str,
        fontSize: f32,
        bold: bool,
        italic: bool,
        fontSpace: f32,
    ) -> f32 {
        self.getTextWidth__chararr_int_int_string_float_boolean_boolean_float(
            text,
            0,
            text.len() as i32,
            fontname,
            fontSize,
            bold,
            italic,
            fontSpace,
        )
    }

    // java public float getTextWidth(char[] text, int charsPos, int charsLen, String fontname, float fontSize, boolean bold, boolean italic, float fontSpace)
    pub fn getTextWidth__chararr_int_int_string_float_boolean_boolean_float(
        &self,
        text: &Vec<char>,
        charsPos: i32,
        charsLen: i32,
        fontname: &str,
        fontSize: f32,
        bold: bool,
        italic: bool,
        _fontSpace: f32,
    ) -> f32 {
        let units = chars_as_utf16(text);
        let face = self.typeface(fontname, bold, italic);
        let rect = face.getTextRect_chars(fontSize, &units, charsPos, charsPos + charsLen);
        rect_width(&rect) + rect.left
    }

    // java public void getFontMetrics(FontMetrics fontMetrics, String fontName, float fontSize, boolean bold, boolean italic)
    pub fn getFontMetrics__fontmetrics_string_float_boolean_boolean(
        &self,
        fontMetrics: &mut FontMetrics,
        fontName: &str,
        fontSize: f32,
        bold: bool,
        italic: bool,
    ) {
        let face = self.typeface(fontName, bold, italic);
        face.getFontMetrics(fontSize, fontMetrics);
    }

    // java public FontMetrics getFontMetrics(String fontName, float fontSize, boolean bold, boolean italic)
    pub fn getFontMetrics__string_float_boolean_boolean(
        &self,
        fontName: &str,
        fontSize: f32,
        bold: bool,
        italic: bool,
    ) -> FontMetrics {
        let mut fontMetrics = FontMetrics::new();
        self.getFontMetrics__fontmetrics_string_float_boolean_boolean(
            &mut fontMetrics,
            fontName,
            fontSize,
            bold,
            italic,
        );
        fontMetrics
    }

    // java public float getAdjCalcFactor(boolean bDiffSBMB)
    pub fn getAdjCalcFactor(&self, bDiffSBMB: bool) -> f32 {
      // java float adjCalcFactor = 1.0F;
      let mut adjCalcFactor: f32 = 1.0;
      // java if (bDiffSBMB)
      if bDiffSBMB {
        // java adjCalcFactor = 4.0F;
        adjCalcFactor = 4.0;
      }
      // java return adjCalcFactor;
      return adjCalcFactor;
    }

}

// ============================================================================
// rust: ★wr-29 04 —— WriterTypeface：Java `cn.wps.font.ITypeface` 的 Rust 真实实现。
//       度量源 = FontStack（FontHost 静态表 / wr-11 后端）。EM 语义 = ttf font units，
//       srcEmSize := units_per_em（与 FontStyleFactory.createFontStyle 的
//       `宽 = advance × fontSize / srcEmSize` 同式）。
// ============================================================================
use crate::cn::wps::moffice::writer::layout::text::FontStack::FontStack;

// java class AndroidTypeface implements ITypeface —— Rust 对等（FontStack 承载）
pub struct WriterTypeface {
    // java family/name
    pub(crate) font_name: Option<String>,
    stack: Arc<FontStack>,
    bold: bool,
    italic: bool,
    cjk: bool,
    em_size: f32,
    src_em_size: f32,
    ascent: f32,
    descent: f32,
    latin_external: f32,
    char_space_ratio: f32,
    subscript_factor: f32,
    superscript_factor: f32,
}

impl WriterTypeface {
    // rust: ★wr-29 04 —— 由 FontStyleFactory.createTypeface 调用（Java family.getTypeface(fontStyle)）
    //       font_name 空串 = 未解析名 → 用默认面（Java get(fontName,false)==null → 默认族）
    pub fn new(font_name: String, bold: bool, italic: bool, cjk: bool, use_builtin_header: bool) -> Arc<dyn thirdparty_font::cn::wps::font::ITypeface::ITypeface> {
        let name = if font_name.is_empty() { None } else { Some(font_name) };
        let stack = crate::cn::wps::moffice::writer::layout::text::FontHost::get_font_table()
            .unwrap_or_else(|| Arc::new(FontStack::default().unwrap_or_else(|_| FontStack::from_font_bytes(vec![0u8; 16]).unwrap())));
        // Java AndroidTypeface.initLalayoutMetricsInfo: a named family whose
        // native FreeType pointer is unavailable uses BuildinTTFHeader.
        // FontStack's named file supplies fallback glyphs; it does not turn
        // that substitute file into the document's declared font family.
        let font_style = i32::from(bold) | (i32::from(italic) << 1);
        let header = name.as_deref().filter(|_| use_builtin_header).and_then(|n|
            thirdparty_font::cn::wps::font::r#impl::BuildinTTFHeader::BuildinTTFHeader::getBuildinTTFHeader(n, font_style));
        let has_builtin_header = header.is_some();
        let (em_size, ascent, descent, latin_external, char_space_ratio, cjk, subscript_factor, superscript_factor) = if let Some(h) = header {
            // Java AndroidTypeface.doCalcLayoutMetricsInfo /
            // AbstractTypeface.doGetLatinAscentAndDescent/doGetLatinExternal.
            use thirdparty_font::cn::wps::font::r#impl::AbstractTypeface::AbstractTypeface;
            use thirdparty_font::cn::wps::util::PrimitivePackTool::PrimitivePackTool;
            let is_cjk = AbstractTypeface::isCJKCodePageRange(h.codePageRange1);
            let packed = AbstractTypeface::doGetLatinAscentAndDescent(
                h.winAscent, h.winDescent, h.tmAscent, h.tmDescent, is_cjk);
            let ascent = PrimitivePackTool::unpackFirstFloat(packed);
            let descent = PrimitivePackTool::unpackSecondFloat(packed);
            let external = AbstractTypeface::doGetLatinExternal(
                h.winAscent, h.winDescent, h.tmAscent, h.tmDescent, h.lineGap, is_cjk);
            let ratio = AbstractTypeface::doGetCharSpaceRatio(h.avgCharWidth, h.unitsPerEM);
            let sub = AbstractTypeface::doGetScriptFactor(h.subscriptYOffset, h.subscriptYSize, h.superscriptYOffset, h.superscriptYSize, h.unitsPerEM, true);
            let sup = AbstractTypeface::doGetScriptFactor(h.subscriptYOffset, h.subscriptYSize, h.superscriptYOffset, h.superscriptYSize, h.unitsPerEM, false);
            (h.unitsPerEM, ascent, descent, external, ratio, is_cjk, sub, sup)
        } else if name.as_deref().map_or(true, |n| !stack.name_hits_file(n)) {
            // Java AndroidTypeface.initLalayoutMetricsInfo: no native
            // FreeType pointer and no built-in TTF header. Its mFontMetrics
            // came from the Android source Typeface (Paint's sans-serif),
            // while glyph lookup may use a different fallback font.
            let (ascent, descent) = stack.paint_source_metrics().unwrap_or((0.0, 0.0));
            (100.0, ascent, descent, 0.0, 4.0, false, 0.6, 0.6)
        } else {
            let em = stack.units_per_em(name.as_deref()).unwrap_or(1000.0);
            let metrics = stack.writer_layout_metrics(name.as_deref()).ok();
            let (sub, sup) = stack.writer_script_factors(name.as_deref()).unwrap_or((0.6, 0.6));
            (em, metrics.as_ref().map_or(0.0, |m| m.ascent),
             metrics.as_ref().map_or(0.0, |m| m.descent),
             metrics.as_ref().map_or(0.0, |m| m.latin_external),
             stack.writer_char_space_ratio(name.as_deref()).unwrap_or(4.0),
             metrics.as_ref().map_or(cjk, |m| m.is_cjk_font), sub, sup)
        };
        // Java: ITypeface typeFace = this.mSourceFontFamily.getTypeface(style);
        // Java: face = TypefaceFactory.createFromSourceFace(this, typeFace, style);
        // Java: face.setSrcEmSize(typeFace.getSrcEmSize());
        // Java: this.mMetricsInfo.srcEmSize = this.mMetricsInfo.emSize;
        // The built-in family supplies layout metrics, while the source face
        // supplies the glyph EM. For a family without a native font file the
        // Android source face takes initLalayoutMetricsInfo's 100 EM fallback.
        // The injected FontStack face is a glyph substitute, not that source
        // family; its unitsPerEm must not replace the Java source EM.
        let src_em_size = if name.as_deref().map_or(true, |n| !stack.name_hits_file(n)) {
            100.0
        } else {
            stack.units_per_em(name.as_deref()).unwrap_or(em_size)
        };
        if std::env::var_os("WORD_PARITY_TYPEFACE_TRACE").is_some() {
            eprintln!("typeface_new name={:?} header={} file={} em={} srcEm={} ascent={} descent={}",
                name, has_builtin_header, name.as_deref().is_some_and(|n| stack.name_hits_file(n)),
                em_size, src_em_size, ascent, descent);
        }
        Arc::new(WriterTypeface {
            font_name: name,
            stack,
            bold,
            italic,
            cjk,
            em_size,
            src_em_size,
            ascent,
            descent,
            latin_external,
            char_space_ratio,
            subscript_factor,
            superscript_factor,
        })
    }

    fn advance_of(&self, u: u16) -> i32 {
        // Java GraphFactory classifies LF/VT as runType 3. AndroidTypeface's
        // FreeType/Paint control-glyph fallback returns srcEmSize when Paint
        // measures no visible glyph; a substitute font's .notdef advance is
        // not the width of these controls.
        if u == 0x000a || u == 0x000b {
            return (self.src_em_size + 0.5) as i32;
        }
        let ch = char::from_u32(u as u32).unwrap_or('\u{FFFD}');
        // Java: AndroidTypeface.getWriterLayoutAdavancesEM — FreeTypeJNI.getGlyphAdvanceEM,
        // then Paint.measureText at srcEmSize when the source face has no glyph.
        match self.stack.glyph_hmtx_units_for_em(self.font_name.as_deref(), ch, self.src_em_size) {
            Ok(advance) => {
                // Some substitute fonts map form feed to .notdef with a zero
                // advance. Android's Paint fallback yields srcEmSize here.
                if u == 0x000c && advance == 0 {
                    (self.src_em_size + 0.5) as i32
                } else {
                    advance
                }
            },
            Err(err)
                if err.kind == crate::cn::wps::moffice::writer::layout::text::FontStack::FontErrorKind::MissingGlyph =>
            {
                // AndroidTypeface.getWriterLayoutAdavancesEM falls back to
                // Paint.measureText and then srcEmSize for a zero-width
                // control glyph. The section form feed at CP1034 takes this
                // path on Android (180 layout units at font size 180).
                if u == 0x000c {
                    return (self.src_em_size + 0.5) as i32;
                }
                // Java AndroidTypeface falls through to Paint.measureText for
                // a missing paragraph mark. The PKM110 Java capture reports
                // 25 at srcEmSize 100 for both Times New Roman and 宋体;
                // using the substitute font's .notdef width here adds an
                // entire false glyph to every paragraph-ending line.
                if u == 0x000d {
                    return (self.src_em_size * 0.25 + 0.5) as i32;
                }
                // Java: SimsunMetrics.getWidthForSpecial. 宋体 controls are absent from the
                // substitute cmap; '\r' is 25 at EM 100 (device Paint raw=25.0).
                if self.font_name.as_deref().is_some_and(is_simsun_family) {
                    let special = crate::cn::wps::moffice::writer::layout::base::env::buildinfont::metrics::SimsunMetrics::SimsunMetrics::getWidthForSpecial(u);
                    if special >= 0 {
                        return special;
                    }
                }
                // Java AndroidTypeface falls through FreeType's missing glyph
                // to Paint.measureText. Paint measures the selected face's
                // .notdef glyph; for the device SysFont this is 994/1000 EM.
                self.stack.missing_glyph_hmtx_units_for_em(self.font_name.as_deref(), self.src_em_size).unwrap_or(0)
            }
            Err(_) => 0,
        }
    }
}

fn is_simsun_family(name: &str) -> bool {
    name == "宋体" || name.eq_ignore_ascii_case("simsun")
}

impl thirdparty_font::cn::wps::font::ITypeface::ITypeface for WriterTypeface {
    // java IFontFamily getFamily()
    fn getFamily(&self) -> Option<Arc<dyn thirdparty_font::cn::wps::font::IFontFamily::IFontFamily>> {
        // rust: FontHost 表内无 IFontFamily 层级（Rust 用名字解析）；null 对等
        None
    }

    // java int getStyle()
    fn getStyle(&self) -> i32 {
        // java return (bold ? 1 : 0) | (italic ? 2 : 0);
        (if self.bold { 1 } else { 0 }) | (if self.italic { 2 } else { 0 })
    }

    // java void getFontMetrics(float fontSize, FontMetrics metrics)
    fn getFontMetrics(&self, fontSize: f32, metrics: &mut thirdparty_font::cn::wps::font::ITypeface::FontMetrics) {
        // java metrics.setBaseline(ascent * fontSize / emSize); metrics.setMaxDescent(-descent * fontSize / emSize);
        let scale = fontSize / self.em_size;
        metrics.mBaseline = self.ascent * scale;
        metrics.mMaxDescent = (-self.descent).max(0.0) * scale;
        metrics.mMaxAscent = metrics.mBaseline;
        metrics.mHeight = metrics.mMaxAscent + metrics.mMaxDescent;
    }

    // java void getTextWidths(float fontSize, char[] ch, int start, float[] widths, int offset, int count)
    fn getTextWidths_float(&self, _var1: f32, var2: &[u16], var3: i32, var4: &mut [f32], var5: i32, var6: i32) {
        for i in 0..var6 {
            var4[(var5 + i) as usize] = (self.advance_of(var2[(var3 + i) as usize]) as f32) * _var1 / self.em_size;
        }
    }

    // java void getTextWidths(int fontSize, char[] ch, int start, int[] widths, int offset, int count)
    fn getTextWidths_int(&self, _var1: i32, var2: &[u16], var3: i32, var4: &mut [i32], var5: i32, var6: i32) {
        for i in 0..var6 {
            var4[(var5 + i) as usize] = self.advance_of(var2[(var3 + i) as usize]) * _var1 / self.em_size as i32;
        }
    }

    // java void getTextWidths(int fontSize, int[] ch, int start, int[] widths, int offset, int count)
    fn getTextWidths_int_int(&self, _var1: i32, var2: &[i32], var3: i32, var4: &mut [i32], var5: i32, var6: i32) {
        for i in 0..var6 {
            let u = var2[(var3 + i) as usize].clamp(0, 0xFFFF) as u16;
            var4[(var5 + i) as usize] = self.advance_of(u) * _var1 / self.em_size as i32;
        }
    }

    // java void getTextWidthsEM(char[] ch, int start, int[] widths, int offset, int count)
    fn getTextWidthsEM(&self, var1: &[u16], var2: i32, var3: &mut [i32], var4: i32, var5: i32) {
        for i in 0..var5 {
            var3[(var4 + i) as usize] = self.advance_of(var1[(var2 + i) as usize]);
        }
    }

    // java float getTextWidth(float fontSize, char[] ch, int start, int len)
    fn getTextWidth(&self, var1: f32, var2: &[u16], var3: i32, var4: i32) -> f32 {
        let mut w = 0.0;
        for i in 0..var4 {
            w += (self.advance_of(var2[(var3 + i) as usize]) as f32) * var1 / self.em_size;
        }
        w
    }

    // java int getTextWidthEM(char[] ch, int start, int len)
    fn getTextWidthEM(&self, var1: &[u16], var2: i32, var3: i32) -> i32 {
        let mut w = 0i32;
        for i in 0..var3 {
            w += self.advance_of(var1[(var2 + i) as usize]);
        }
        w
    }

    // java RectF getTextRect(float fontSize, char ch)
    fn getTextRect_char(&self, var1: f32, var2: u16) -> thirdparty_font::cn::wps::font::FreeTypeJNI::RectF {
        self.getTextRect_chars(var1, &[var2], 0, 1)
    }

    // java RectF getTextRectEM(char ch)
    fn getTextRectEM_char(&self, var2: u16) -> thirdparty_font::cn::wps::font::FreeTypeJNI::RectF {
        thirdparty_font::cn::wps::font::FreeTypeJNI::RectF {
            left: 0.0,
            top: 0.0,
            right: self.advance_of(var2) as f32,
            bottom: 0.0,
        }
    }

    // java RectF getTextRect(float fontSize, char[] ch, int start, int len)
    fn getTextRect_chars(&self, var1: f32, var2: &[u16], var3: i32, var4: i32) -> thirdparty_font::cn::wps::font::FreeTypeJNI::RectF {
        let mut w = 0.0;
        for i in 0..var4 {
            w += (self.advance_of(var2[(var3 + i) as usize]) as f32) * var1 / self.em_size;
        }
        thirdparty_font::cn::wps::font::FreeTypeJNI::RectF {
            left: 0.0,
            top: 0.0,
            right: w,
            bottom: (-self.descent) * var1 / self.em_size,
        }
    }

    // java RectF getTextRectEM(char[] ch, int start, int len)
    fn getTextRectEM_chars(&self, var1: &[u16], var2: i32, var3: i32) -> thirdparty_font::cn::wps::font::FreeTypeJNI::RectF {
        let mut w = 0.0;
        for i in 0..var3 {
            w += self.advance_of(var1[(var2 + i) as usize]) as f32;
        }
        thirdparty_font::cn::wps::font::FreeTypeJNI::RectF {
            left: 0.0,
            top: 0.0,
            right: w,
            bottom: (-self.descent).max(0.0),
        }
    }

    // java Object getNativeTypeface()
    fn getNativeTypeface(&self) -> i64 {
        0
    }

    // java boolean needBold()
    fn needBold(&self) -> bool {
        self.bold
    }

    // java boolean needItalic()
    fn needItalic(&self) -> bool {
        self.italic
    }

    // java float getDefaultFareastCharWidth(float fontSize)
    fn getDefaultFareastCharWidth(&self, var1: f32) -> f32 {
        // java 全角默认宽 = 1em
        var1
    }

    // java int getDefaultFareastCharWidthEM()
    fn getDefaultFareastCharWidthEM(&self) -> i32 {
        self.em_size as i32
    }

    // java RectF getDefaultFareastCharRect(float fontSize)
    fn getDefaultFareastCharRect(&self, var1: f32) -> thirdparty_font::cn::wps::font::FreeTypeJNI::RectF {
        thirdparty_font::cn::wps::font::FreeTypeJNI::RectF {
            left: 0.0,
            top: 0.0,
            right: var1,
            bottom: 0.0,
        }
    }

    // java ITypeface deriveFallbackTypeface(IFontFamily family)
    fn deriveFallbackTypeface(&self, _var1: Arc<dyn thirdparty_font::cn::wps::font::IFontFamily::IFontFamily>) -> Option<Arc<dyn thirdparty_font::cn::wps::font::ITypeface::ITypeface>> {
        // rust: 无 fallback 链（MVP 单默认面）
        None
    }

    // java boolean isCJKFont()
    fn isCJKFont(&self) -> bool {
        self.cjk
    }

    // java boolean isLatinFont()
    fn isLatinFont(&self) -> bool {
        !self.cjk
    }

    // java boolean isComplexFont()
    fn isComplexFont(&self) -> bool {
        false
    }

    // java boolean hasUnicodeBlock(int unicodeBlock)
    fn hasUnicodeBlock(&self, _var1: i32) -> bool {
        true
    }

    // java float getWriterLayoutEMSize()
    fn getWriterLayoutEMSize(&self) -> f32 {
        self.em_size
    }

    // java float getSrcEmSize()
    fn getSrcEmSize(&self) -> f32 {
        self.src_em_size
    }

    // java void setSrcEmSize(float srcEmSize)
    fn setSrcEmSize(&self, _var1: f32) {}

    // java float getWriterLayoutAscent()
    fn getWriterLayoutAscent(&self) -> f32 {
        self.ascent
    }

    // java float getWriterLayoutDescent()
    fn getWriterLayoutDescent(&self) -> f32 {
        self.descent
    }

    // java float getWriterLayoutLatinExternal()
    fn getWriterLayoutLatinExternal(&self) -> f32 {
        // java AndroidTypeface.getWriterLayoutLatinExternal() returns metrics.latinExternal;
        self.latin_external
    }

    // java float getWriterLayoutCharSpaceRatio()
    // Java AbstractTypeface.doGetCharSpaceRatio: emSize * 2.0F / avgCharWidth.
    fn getWriterLayoutCharSpaceRatio(&self) -> f32 {
        // java AndroidTypeface.getWriterLayoutCharSpaceRatio() returns metrics.charSpaceRatio;
        self.char_space_ratio
    }

    // java float getWriterLayoutSubscriptFactor()
    fn getWriterLayoutSubscriptFactor(&self) -> f32 {
        self.subscript_factor
    }

    // java float getWriterLayoutSuperscriptFactor()
    fn getWriterLayoutSuperscriptFactor(&self) -> f32 {
        self.superscript_factor
    }

    // java void getWriterLayoutAdavances(float fontSize, char[] ch, int start, float[] widths, int offset, int count)
    fn getWriterLayoutAdavances_float(&self, var1: f32, var2: &[u16], var3: i32, var4: &mut [f32], var5: i32, var6: i32) {
        self.getTextWidths_float(var1, var2, var3, var4, var5, var6)
    }

    // java void getWriterLayoutAdavances(int fontSize, char[] ch, int start, int[] widths, int offset, int count)
    fn getWriterLayoutAdavances_int(&self, var1: i32, var2: &[u16], var3: i32, var4: &mut [i32], var5: i32, var6: i32) {
        self.getTextWidths_int(var1, var2, var3, var4, var5, var6)
    }

    // java void getWriterLayoutAdavances(int fontSize, int[] ch, int start, int[] widths, int offset, int count)
    fn getWriterLayoutAdavances_int_int(&self, var1: i32, var2: &[i32], var3: i32, var4: &mut [i32], var5: i32, var6: i32) {
        self.getTextWidths_int_int(var1, var2, var3, var4, var5, var6)
    }

    // java void getWriterLayoutAdavancesEM(char[] ch, int start, int[] widths, int offset, int count)
    fn getWriterLayoutAdavancesEM_chars(&self, var1: &[u16], var2: i32, var3: &mut [i32], var4: i32, var5: i32) {
        self.getTextWidthsEM(var1, var2, var3, var4, var5)
    }

    // java int getCharWidth(char ch, boolean null_bool)
    fn getCharWidth(&self, var1: u16, _var2: bool) -> i32 {
        self.advance_of(var1)
    }

    // java RectF getCharRect(char ch, boolean null_bool)
    fn getCharRect(&self, var1: u16, _var2: bool) -> thirdparty_font::cn::wps::font::FreeTypeJNI::RectF {
        self.getTextRectEM_char(var1)
    }

    // java FreeTypeJNI.TTFHeader getTTFHeader(boolean null_bool)
    fn getTTFHeader(&self, _var1: bool) -> Option<thirdparty_font::cn::wps::font::FreeTypeJNI::TTFHeader> {
        None
    }

    // java void getWriterLayoutAdavancesEM(int[] ch, int start, int[] widths, int offset, int count)
    fn getWriterLayoutAdavancesEM_ints(&self, var1: &[i32], var2: i32, var3: &mut [i32], var4: i32, var5: i32) {
        for i in 0..var5 {
            // Java AndroidTypeface: char ch = (char)text[i] (low 16 bits).
            let u = var1[(var2 + i) as usize] as u16;
            // Java AndroidTypeface's int[] overload uses srcEmSize if the
            // source face has no native glyph and Paint measures the control
            // U+0007 as zero. The substitute face reports zero for it, while
            // Java's Sample 3 trace records raw 100 EM for Calibri.
            let source_has_file = self.font_name.as_deref().is_some_and(|name| self.stack.name_hits_file(name));
            var3[(var4 + i) as usize] = if u == 0x0007 && !source_has_file {
                (self.src_em_size + 0.5) as i32
            } else {
                self.advance_of(u)
            };
        }
    }

    // java int getUnderlinePosition()
    fn getUnderlinePosition(&self) -> i32 {
        0
    }

    // java int getUnderlineThickness()
    fn getUnderlineThickness(&self) -> i32 {
        0
    }

    // java int getYStrikeoutSize()
    fn getYStrikeoutSize(&self) -> i32 {
        0
    }

    // java int getYStrikeoutPosition()
    fn getYStrikeoutPosition(&self) -> i32 {
        0
    }

    // java boolean hasKerning()
    fn hasKerning(&self) -> bool {
        false
    }

    // java int getKerningEM(char ch1, char ch2)
    fn getKerningEM(&self, _var1: u16, _var2: u16) -> i32 {
        0
    }

    // java BuildinGlyphMetrics.IMetricsGetter getMetricsGetter()
    fn getMetricsGetter(&self) -> Option<std::sync::Arc<dyn thirdparty_font::cn::wps::font::r#impl::BuildinGlyphMetrics::IMetricsGetterTrait>> {
        // rust: FontStack（ttf-parser）无 BuildinGlyphMetrics 层 → None
        None
    }

    // java boolean isCreateFromFontFile()
    fn isCreateFromFontFile(&self) -> bool {
        false
    }

    // java float getWppLayoutHeight()
    fn getWppLayoutHeight(&self) -> f32 {
        0.0
    }

    // java float getWppLayoutStakckedHeight()
    fn getWppLayoutStakckedHeight(&self) -> f32 {
        0.0
    }
}

impl WriterTypeface {
    pub fn default_font_name(&self) -> Option<&str> {
        self.font_name.as_deref()
    }
}
