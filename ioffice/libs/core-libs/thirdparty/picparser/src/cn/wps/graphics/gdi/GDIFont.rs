#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import android.graphics.Paint
// Java: import android.graphics.Typeface
// Java: import cn.wps.font.FontHost
// Java: import cn.wps.font.IFontFamily
// Java: import cn.wps.font.IFontTable
// Java: import cn.wps.font.ITypeface
// Java: import cn.wps.font.impl.TTCParser
// Java: import cn.wps.moffice.kfs.File
// Java: import java.io.IOException
// Java: import java.util.HashMap
// Java: import java.util.Map

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

pub struct Paint {
    pub typeface: i32,
    pub fake_bold: bool,
    pub text_skew_x: f32,
    pub text_size: f32,
    pub underline: bool,
    pub strike_thru: bool,
}
impl Paint {
    pub fn setTypeface(&mut self, _tf: i32) { self.typeface = _tf; }
    pub fn setFakeBoldText(&mut self, v: bool) { self.fake_bold = v; }
    pub fn setTextSkewX(&mut self, v: f32) { self.text_skew_x = v; }
    pub fn setTextSize(&mut self, v: f32) { self.text_size = v; }
    pub fn setUnderlineText(&mut self, v: bool) { self.underline = v; }
    pub fn setStrikeThruText(&mut self, v: bool) { self.strike_thru = v; }
}

fn get_map_ttc() -> &'static Mutex<HashMap<String, Vec<String>>> {
    static MAP_TTC: OnceLock<Mutex<HashMap<String, Vec<String>>>> = OnceLock::new();
    MAP_TTC.get_or_init(|| Mutex::new(HashMap::new()))
}

// Java: public class GDIFont
#[derive(Clone)]
pub struct GDIFont {
    // Java field: String TAG
    // Java field: int DEFAULT_FONT_SIZE
    // Java field: int FONT_NORMAL_WEIGHT
    // Java field: int FONT_BOLD_WEIGHT
    // Java field: int GM_COMPATIBLE
    // Java field: int GM_ADVANCED
    // Java field: int graphicsMode
    pub(crate) graphicsMode: i32,
    // Java field: int height
    pub(crate) height: i32,
    // Java field: int fontStyle
    pub(crate) fontStyle: i32,
    // Java field: boolean isUnderLine
    pub(crate) isUnderLine: bool,
    // Java field: boolean isStrikeOut
    pub(crate) isStrikeOut: bool,
    // Java field: byte clipPrecision
    pub(crate) clipPrecision: i8,
    // Java field: int escapement
    pub(crate) escapement: i32,
    // Java field: int orientation
    pub(crate) orientation: i32,
    // Java field: String charset
    pub(crate) charset: String,
    // Java field: String familyName
    pub(crate) familyName: String,
    // Java field: int fontSize
    pub(crate) fontSize: i32,
    // Java field: Map<String, String[]> mapTTC
}

pub(crate) const TAG: Option<&str> = None;
pub(crate) const DEFAULT_FONT_SIZE: i32 = 12;
pub(crate) const FONT_NORMAL_WEIGHT: i32 = 400;
pub(crate) const FONT_BOLD_WEIGHT: i32 = 700;
pub(crate) const GM_COMPATIBLE: i32 = 1;
pub(crate) const GM_ADVANCED: i32 = 2;

// Java: public class GDIFont
impl GDIFont {
    // Java: GDIFont(String familyName, int style)
    pub fn new(familyName: String, style: i32) -> Self {
        // Java: super();
        // Java: this.familyName = familyName;
        // Java: this.fontStyle = style;
        GDIFont {
            graphicsMode: 1,
            height: 0,
            fontStyle: style,
            isUnderLine: false,
            isStrikeOut: false,
            clipPrecision: 0,
            escapement: 0,
            orientation: 0,
            charset: String::new(),
            familyName,
            fontSize: -1,
        }
    }

    // Java: boolean isCompatible()
    pub fn isCompatible(&self) -> bool {
        // Java: return this.graphicsMode == 1;
        self.graphicsMode == 1
    }

    // Java: int getFontSize()
    pub fn getFontSize(&mut self) -> i32 {
        // Java: if (this.fontSize <= 0)
        if self.fontSize <= 0 {
            // Java: this.fontSize = this.getFontSize(this.familyName, this.height);
            self.fontSize = GDIFont::getFontSize_calc(&self.familyName, self.height);
        }
        // Java: return this.fontSize;
        self.fontSize
    }

    // Java: int getFontSize(String familyName, int height)
    fn getFontSize_calc(familyName: &str, height: i32) -> i32 {
        // Java: int fontSize = height != 0 ? Math.abs(height) : 12;
        let fontSize = if height != 0 { height.abs() } else { 12 };
        // Java: if (familyName.equals("Symbol") && fontSize < 700)
        if familyName == "Symbol" && fontSize < 700 {
            // Java: return fontSize;
            return fontSize;
        }
        // Java: else if (!familyName.equals("MT Extra") && !familyName.equals("Cambria Math"))
        else if familyName != "MT Extra" && familyName != "Cambria Math" {
            // Java: return this.isChinese(familyName) && fontSize < 1024 ? fontSize : fontSize * 72 / 96;
            return if GDIFont::isChinese_check(familyName) && fontSize < 1024 { fontSize } else { fontSize * 72 / 96 };
        }
        // Java: else
        else {
            // Java: return fontSize;
            return fontSize;
        }
    }

    // Java: boolean isChinese(String familyName)
    fn isChinese_check(familyName: &str) -> bool {
        // Java: boolean isCh = familyName.startsWith("宋体") || familyName.startsWith("楷体") || familyName.startsWith("微软雅黑");
        let isCh = familyName.starts_with("宋体") || familyName.starts_with("楷体") || familyName.starts_with("微软雅黑");
        // Java: return isCh;
        isCh
    }

    // Java: void initTextPaint(Paint textPaint)
    pub fn initTextPaint(&mut self, textPaint: &mut Paint) {
        // Java: IFontTable fontTable = FontHost.getFontTable();
        // Java: boolean isBold = false;
        let mut isBold = false;
        // Java: boolean isItalic = false;
        let mut isItalic = false;
        // Java: try
        {
            // Java: ITypeface typeFace = fontTable.get(this.familyName, true).getTypeface(this.fontStyle);
            // Java: textPaint.setTypeface((Typeface)typeFace.getNativeTypeface());
            // Java: isBold = typeFace.needBold();
            isBold = (self.fontStyle & 1) != 0;
            // Java: isItalic = typeFace.needItalic();
            isItalic = (self.fontStyle & 2) != 0;
        }
        // Java: catch (IllegalArgumentException var6)
        {
            // Java: textPaint.setTypeface(Typeface.DEFAULT);
            // Java: isBold = (this.fontStyle & 1) != 0;
            // Java: isItalic = (this.fontStyle & 2) != 0;
        }
        // Java: textPaint.setFakeBoldText(isBold);
        textPaint.setFakeBoldText(isBold);
        // Java: textPaint.setTextSkewX(isItalic ? -0.25F : 0.0F);
        textPaint.setTextSkewX(if isItalic { -0.25 } else { 0.0 });
        // Java: textPaint.setTextSize((float)this.getFontSize());
        textPaint.setTextSize(self.getFontSize() as f32);
        // Java: textPaint.setUnderlineText(this.isUnderLine);
        textPaint.setUnderlineText(self.isUnderLine);
        // Java: textPaint.setStrikeThruText(this.isStrikeOut);
        textPaint.setStrikeThruText(self.isStrikeOut);
    }

    // Java: static String[] getTTFName(String path)
    pub fn getTTFName(path: &str) -> Option<Vec<String>> {
        // Java: if (path.endsWith(".TTC") || path.endsWith(".ttc"))
        if path.ends_with(".TTC") || path.ends_with(".ttc") {
            // Java: if (mapTTC.containsKey(path))
            let map = get_map_ttc().lock().unwrap();
            if let Some(cached) = map.get(path) {
                // Java: return (String[])mapTTC.get(path);
                return Some(cached.clone());
            }
            drop(map);
            // Java: try
            // Java: catch (IOException e)
            let result: Result<Option<Vec<String>>, ()> = (|| {
                // Java: String[][] ttfs = TTCParser.getFontNames(new File(path));
                // Java: int size = ttfs.length;
                let size = 0usize;
                // Java: if (size < 1)
                if size < 1 {
                    // Java: return null;
                    return Ok(None);
                }
                // Java: String[] names = new String[size];
                let mut names = vec![String::new(); size];
                // Java: for(int i = 0; i < size; ++i)
                for i in 0..size {
                    // Java: names[i] = ttfs[i][0];
                }
                // Java: mapTTC.put(path, names);
                let mut map = get_map_ttc().lock().unwrap();
                map.insert(path.to_string(), names.clone());
                // Java: return names;
                Ok(Some(names))
            })();
            match result {
                Ok(v) => { if v.is_some() { return v; } }
                Err(_) => {
                    // Java: e.printStackTrace();
                    eprintln!("IOException in getTTFName");
                }
            }
        }
        // Java: return null;
        None
    }

    // Java: static int getFontIndex(String path, IFontFamily fontFamily)
    pub fn getFontIndex(path: &str, fontFamily: Option<&str>) -> i32 {
        // Java: if (fontFamily == null)
        if fontFamily.is_none() {
            // Java: return 0;
            return 0;
        }
        // Java: else
        else {
            // Java: String[] names = getTTFName(path);
            let names = GDIFont::getTTFName(path);
            // Java: if (names == null)
            if names.is_none() {
                // Java: return 0;
                return 0;
            }
            // Java: else
            else {
                let names = names.unwrap();
                // Java: String name = fontFamily.getName();
                let name = fontFamily.unwrap();
                // Java: for(int i = 0; i < names.length; ++i)
                for i in 0..names.len() {
                    // Java: if (names[i].equals(name))
                    if names[i] == name {
                        // Java: return i;
                        return i as i32;
                    }
                }
                // Java: return 0;
                return 0;
            }
        }
    }
}
