// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.font.IFontFamily
use crate::cn::wps::font::IFontFamily::IFontFamily;
// Java: import cn.wps.font.ITypeface
use crate::cn::wps::font::ITypeface::ITypeface;
// Java: import java.io.File
use std::path::Path;
// Java: import java.util.ArrayList
// Java: import java.util.Iterator
use std::sync::{Arc, Mutex};
use super::IFontDelayAdapter::IFontDelayAdapter;

struct Platform;
impl Platform {
    fn getSystemFontsDirectory() -> String {
        String::from("/system/fonts")
    }
}

struct FontNameDecoder {
    style: i32,
}
impl FontNameDecoder {
    fn decode(_name: &str) -> Self {
        FontNameDecoder { style: 0 }
    }
    fn getStyle(&self) -> i32 {
        self.style
    }
}

pub struct FontFamilyImpl {
    name: String,
    is_system_font: bool,
    id: i32,
    is_online_font: Mutex<bool>,
    font_files: Mutex<Vec<(i32, String)>>,
}

impl FontFamilyImpl {
    pub fn new(name: String, is_system_font: bool, id: i32) -> Self {
        FontFamilyImpl {
            name,
            is_system_font,
            id,
            is_online_font: Mutex::new(false),
            font_files: Mutex::new(Vec::new()),
        }
    }
}

impl IFontFamily for FontFamilyImpl {
    fn getName(&self) -> String { self.name.clone() }
    fn getId(&self) -> i32 { self.id }
    fn getTypeface(&self, _var1: i32) -> Option<Arc<dyn ITypeface>> { None }
    fn isSystemFont(&self) -> bool { self.is_system_font }
    fn isOnlineFont(&self) -> bool { *self.is_online_font.lock().unwrap() }
    fn fontFileExist(&self) -> bool { !self.font_files.lock().unwrap().is_empty() }
    fn setOnlineFont(&self, var1: bool) { *self.is_online_font.lock().unwrap() = var1; }
    fn getFontFiles(&self, _var1: i32) -> Vec<String> { Vec::new() }
    fn getFontFile(&self, _var1: i32) -> String { String::new() }
    fn getAllFontFiles(&self) -> Vec<String> { self.font_files.lock().unwrap().iter().map(|(_, f)| f.clone()).collect() }
    fn getNativeFreeTypeFontPointer(&self, _var1: i32) -> i64 { 0 }
    fn addTypefaceFromFile(&self, _var1: i32, _var2: String) -> Option<Arc<dyn ITypeface>> { None }
    fn addFontFile(&self, var1: i32, var2: String) { self.font_files.lock().unwrap().push((var1, var2)); }
    fn isBuildinHeader(&self) -> bool { false }
    fn getSourceFont(&self) -> Option<Arc<dyn IFontFamily>> { None }
    fn closeNativePtr(&self) {}
}

enum FontData {
    Single(String),
    Multiple(Vec<String>),
}

// Java: public class FontFamilyDealyAdapter implements IFontFamily, IFontDelayAdapter
pub struct FontFamilyDealyAdapter {
    // Java field: String mFamilyName
    pub(crate) mFamilyName: String,
    // Java field: int mId
    pub(crate) mId: i32,
    // Java field: Object mData
    mData: Mutex<Option<FontData>>,
    // Java field: IFontFamily mSourceFont
    mSourceFont: Mutex<Option<Arc<dyn IFontFamily>>>,
    // Java field: boolean mParsed
    mParsed: Mutex<bool>,
}

// Java: public class FontFamilyDealyAdapter implements IFontFamily, IFontDelayAdapter
impl FontFamilyDealyAdapter {
    // Java: FontFamilyDealyAdapter(String familyName, int id)
    pub fn new(family_name: String, id: i32) -> Self {
        // Java: super();
        // Java: this.mFamilyName = familyName;
        // Java: this.mId = id;
        FontFamilyDealyAdapter {
            mFamilyName: family_name,
            mId: id,
            mData: Mutex::new(None),
            mSourceFont: Mutex::new(None),
            mParsed: Mutex::new(false),
        }
    }

    // Java: void addLine(String line)
    pub fn addLine(&self, line: String) {
        let mut data = self.mData.lock().unwrap();
        // Java: if (this.mData == null)
        if data.is_none() {
            // Java: this.mData = line;
            *data = Some(FontData::Single(line));
        }
        // Java: else if (this.mData instanceof String)
        else if matches!(&*data, Some(FontData::Single(_))) {
            // Java: ArrayList<String> list = new ArrayList(2);
            let mut list = Vec::with_capacity(2);
            if let Some(FontData::Single(s)) = data.take() {
                // Java: list.add((String)this.mData);
                list.push(s);
            }
            // Java: list.add(line);
            list.push(line);
            // Java: this.mData = list;
            *data = Some(FontData::Multiple(list));
        }
        // Java: else if (this.mData instanceof ArrayList)
        else if let Some(FontData::Multiple(ref mut list)) = *data {
            // Java: ArrayList<String> list = (ArrayList)this.mData;
            // Java: list.add(line);
            list.push(line);
        }
    }

    // Java: IFontFamily validFont()
    pub fn validFont(&self) -> Option<Arc<dyn IFontFamily>> {
        // Java: IFontFamily font = this.mSourceFont;
        let font = self.mSourceFont.lock().unwrap().clone();
        // Java: if (font == null && !this.mParsed)
        if font.is_none() && !*self.mParsed.lock().unwrap() {
            // Java: synchronized(this)
            // Java: if (font == null)
            // Java: font = this.makeFont();
            let made = self.makeFont();
            // Java: this.mSourceFont = font;
            *self.mSourceFont.lock().unwrap() = made.clone();
            // Java: this.mParsed = true;
            *self.mParsed.lock().unwrap() = true;
            return made;
        }
        // Java: return font;
        font
    }

    // Java: IFontFamily fourceEmptySourceFont(boolean isSystemFont)
    pub fn fourceEmptySourceFont(&self, is_system_font: bool) -> Option<Arc<dyn IFontFamily>> {
        // Java: IFontFamily fontFamily = this.validFont();
        let mut font_family = self.validFont();
        // Java: if (fontFamily == null)
        if font_family.is_none() {
            // Java: fontFamily = new FontFamilyImpl(this.mFamilyName, isSystemFont, this.mId);
            font_family = Some(Arc::new(FontFamilyImpl::new(self.mFamilyName.clone(), is_system_font, self.mId)));
        }
        // Java: return fontFamily;
        font_family
    }

    // Java: IFontFamily makeFont()
    fn makeFont(&self) -> Option<Arc<dyn IFontFamily>> {
        // Java: IFontFamily fontFamily = null;
        let mut font_family: Option<Arc<dyn IFontFamily>> = None;
        let data = self.mData.lock().unwrap();
        // Java: if (this.mData instanceof String)
        if let Some(FontData::Single(ref s)) = *data {
            // Java: fontFamily = this.makeFont((String)this.mData, fontFamily);
            font_family = self.makeFont_line(s, font_family);
        }
        // Java: else if (this.mData instanceof ArrayList)
        else if let Some(FontData::Multiple(ref list)) = *data {
            // Java: for(Object lineObj : (ArrayList)this.mData)
            for line_obj in list {
                // Java: String line = (String)lineObj;
                // Java: fontFamily = this.makeFont(line, fontFamily);
                font_family = self.makeFont_line(line_obj, font_family);
            }
        }
        // Java: return fontFamily;
        font_family
    }

    // Java: IFontFamily makeFont(String line, IFontFamily fontFamily)
    fn makeFont_line(&self, line: &str, font_family: Option<Arc<dyn IFontFamily>>) -> Option<Arc<dyn IFontFamily>> {
        // Java: String[] names = line.split("\t");
        let names: Vec<&str> = line.split('\t').collect();
        // Java: if (names.length < 3)
        if names.len() < 3 {
            // Java: return null;
            return None;
        }
        // Java: else
        {
            // Java: String fontFullName = names[0];
            let font_full_name = names[0];
            // Java: String familyName = names[1];
            let family_name = names[1];
            // Java: String fontFileName = names[2];
            let font_file_name = names[2];
            // Java: if (familyName.equalsIgnoreCase("null"))
            if family_name.eq_ignore_ascii_case("null") {
                // Java: return null;
                return None;
            }
            // Java: else
            {
                // Java: File fontFile = new File(fontFileName);
                let font_file = Path::new(font_file_name);
                // Java: if (fontFile.exists())
                if font_file.exists() {
                    // Java: FontNameDecoder decoder = FontNameDecoder.decode(fontFullName);
                    let decoder = FontNameDecoder::decode(font_full_name);
                    // Java: int style = decoder.getStyle();
                    let style = decoder.getStyle();
                    // Java: String systemFontsDir = Platform.getSystemFontsDirectory();
                    let system_fonts_dir = Platform::getSystemFontsDirectory();
                    // Java: boolean isSystemFont = fontFileName.startsWith(systemFontsDir);
                    let is_system_font = font_file_name.starts_with(&system_fonts_dir);
                    // Java: boolean isOnlineFont = fontFileName.contains("KingsoftOffice/.font") && fontFileName.contains("cn.wps.moffice");
                    let is_online_font = font_file_name.contains("KingsoftOffice/.font") && font_file_name.contains("cn.wps.moffice");
                    // Java: if (fontFamily == null)
                    let mut result = font_family;
                    if result.is_none() {
                        // Java: fontFamily = new FontFamilyImpl(familyName, isSystemFont, this.mId);
                        result = Some(Arc::new(FontFamilyImpl::new(family_name.to_string(), is_system_font, self.mId)));
                    }
                    // Java: fontFamily.setOnlineFont(isOnlineFont);
                    if let Some(ref f) = result {
                        f.setOnlineFont(is_online_font);
                    }
                    // Java: fontFamily.addFontFile(style, fontFileName);
                    if let Some(ref f) = result {
                        f.addFontFile(style, font_file_name.to_string());
                    }
                    return result;
                }
                // Java: return fontFamily;
                return font_family;
            }
        }
    }

    // Java: String getName()
    fn getName_impl(&self) -> String {
        // Java: return this.mFamilyName;
        self.mFamilyName.clone()
    }

    // Java: int getId()
    fn getId_impl(&self) -> i32 {
        // Java: return this.mId;
        self.mId
    }

    // Java: ITypeface getTypeface(int i)
    fn getTypeface_impl(&self, i: i32) -> Option<Arc<dyn ITypeface>> {
        // Java: return this.validFont().getTypeface(i);
        self.validFont().and_then(|f| f.getTypeface(i))
    }

    // Java: boolean isSystemFont()
    fn isSystemFont_impl(&self) -> bool {
        // Java: IFontFamily validFont = this.validFont();
        let valid_font = self.validFont();
        // Java: return validFont != null && validFont.isSystemFont();
        valid_font.map_or(false, |f| f.isSystemFont())
    }

    // Java: boolean isOnlineFont()
    fn isOnlineFont_impl(&self) -> bool {
        // Java: IFontFamily validFont = this.validFont();
        let valid_font = self.validFont();
        // Java: return validFont != null && validFont.isOnlineFont();
        valid_font.map_or(false, |f| f.isOnlineFont())
    }

    // Java: boolean fontFileExist()
    fn fontFileExist_impl(&self) -> bool {
        // Java: IFontFamily validFont = this.validFont();
        let valid_font = self.validFont();
        // Java: return validFont != null && validFont.fontFileExist();
        valid_font.map_or(false, |f| f.fontFileExist())
    }

    // Java: void setOnlineFont(boolean b)
    fn setOnlineFont_impl(&self, b: bool) {
        // Java: IFontFamily validFont = this.validFont();
        let valid_font = self.validFont();
        // Java: if (validFont != null)
        if let Some(f) = valid_font {
            // Java: validFont.setOnlineFont(b);
            f.setOnlineFont(b);
        }
    }

    // Java: String[] getFontFiles(int i)
    fn getFontFiles_impl(&self, i: i32) -> Vec<String> {
        // Java: IFontFamily validFont = this.validFont();
        let valid_font = self.validFont();
        // Java: return validFont == null ? null : validFont.getFontFiles(i);
        valid_font.map_or(Vec::new(), |f| f.getFontFiles(i))
    }

    // Java: String getFontFile(int i)
    fn getFontFile_impl(&self, i: i32) -> String {
        // Java: IFontFamily validFont = this.validFont();
        let valid_font = self.validFont();
        // Java: return validFont == null ? null : validFont.getFontFile(i);
        valid_font.map_or(String::new(), |f| f.getFontFile(i))
    }

    // Java: String[] getAllFontFiles()
    fn getAllFontFiles_impl(&self) -> Vec<String> {
        // Java: IFontFamily validFont = this.validFont();
        let valid_font = self.validFont();
        // Java: return validFont == null ? null : validFont.getAllFontFiles();
        valid_font.map_or(Vec::new(), |f| f.getAllFontFiles())
    }

    // Java: long getNativeFreeTypeFontPointer(int i)
    fn getNativeFreeTypeFontPointer_impl(&self, i: i32) -> i64 {
        // Java: IFontFamily validFont = this.validFont();
        let valid_font = self.validFont();
        // Java: return validFont == null ? 0L : validFont.getNativeFreeTypeFontPointer(i);
        valid_font.map_or(0, |f| f.getNativeFreeTypeFontPointer(i))
    }

    // Java: ITypeface addTypefaceFromFile(int i, String s)
    fn addTypefaceFromFile_impl(&self, i: i32, s: String) -> Option<Arc<dyn ITypeface>> {
        // Java: IFontFamily validFont = this.validFont();
        let valid_font = self.validFont();
        // Java: return validFont == null ? null : validFont.addTypefaceFromFile(i, s);
        valid_font.and_then(|f| f.addTypefaceFromFile(i, s))
    }

    // Java: void addFontFile(int i, String s)
    fn addFontFile_impl(&self, i: i32, s: String) {
        // Java: IFontFamily validFont = this.validFont();
        let valid_font = self.validFont();
        // Java: if (validFont != null)
        if let Some(f) = valid_font {
            // Java: this.validFont().addFontFile(i, s);
            f.addFontFile(i, s);
        }
    }

    // Java: boolean isBuildinHeader()
    fn isBuildinHeader_impl(&self) -> bool {
        // Java: IFontFamily validFont = this.validFont();
        let valid_font = self.validFont();
        // Java: return validFont != null && validFont.isBuildinHeader();
        valid_font.map_or(false, |f| f.isBuildinHeader())
    }

    // Java: IFontFamily getSourceFont()
    fn getSourceFont_impl(&self) -> Option<Arc<dyn IFontFamily>> {
        // Java: IFontFamily validFont = this.validFont();
        let valid_font = self.validFont();
        // Java: return validFont == null ? null : validFont.getSourceFont();
        valid_font.and_then(|f| f.getSourceFont())
    }

    // Java: Iterator<ITypeface> iterator()
    pub fn iterator(&self) -> Vec<Arc<dyn ITypeface>> {
        // Java: IFontFamily validFont = this.validFont();
        // Java: return validFont == null ? null : validFont.iterator();
        Vec::new()
    }

    // Java: void closeNativePtr()
    fn closeNativePtr_impl(&self) {
        // Java: if (this.mSourceFont != null)
        let source = self.mSourceFont.lock().unwrap();
        if let Some(ref f) = *source {
            // Java: this.mSourceFont.closeNativePtr();
            f.closeNativePtr();
        }
    }
}

impl IFontFamily for FontFamilyDealyAdapter {
    fn getName(&self) -> String { self.getName_impl() }
    fn getId(&self) -> i32 { self.getId_impl() }
    fn getTypeface(&self, i: i32) -> Option<Arc<dyn ITypeface>> { self.getTypeface_impl(i) }
    fn isSystemFont(&self) -> bool { self.isSystemFont_impl() }
    fn isOnlineFont(&self) -> bool { self.isOnlineFont_impl() }
    fn fontFileExist(&self) -> bool { self.fontFileExist_impl() }
    fn setOnlineFont(&self, b: bool) { self.setOnlineFont_impl(b) }
    fn getFontFiles(&self, i: i32) -> Vec<String> { self.getFontFiles_impl(i) }
    fn getFontFile(&self, i: i32) -> String { self.getFontFile_impl(i) }
    fn getAllFontFiles(&self) -> Vec<String> { self.getAllFontFiles_impl() }
    fn getNativeFreeTypeFontPointer(&self, i: i32) -> i64 { self.getNativeFreeTypeFontPointer_impl(i) }
    fn addTypefaceFromFile(&self, i: i32, s: String) -> Option<Arc<dyn ITypeface>> { self.addTypefaceFromFile_impl(i, s) }
    fn addFontFile(&self, i: i32, s: String) { self.addFontFile_impl(i, s) }
    fn isBuildinHeader(&self) -> bool { self.isBuildinHeader_impl() }
    fn getSourceFont(&self) -> Option<Arc<dyn IFontFamily>> { self.getSourceFont_impl() }
    fn closeNativePtr(&self) { self.closeNativePtr_impl() }
}

impl IFontDelayAdapter for FontFamilyDealyAdapter {
    fn validFont(&self) -> Option<Arc<dyn IFontFamily>> {
        FontFamilyDealyAdapter::validFont(self)
    }
}
