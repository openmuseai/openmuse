// Java: import cn.wps.font.FontHost
use crate::cn::wps::font::FontHost::FontHost;
// Java: import cn.wps.font.FreeTypeJNI
use crate::cn::wps::font::FreeTypeJNI;
// Java: import cn.wps.font.IFontFamily
use crate::cn::wps::font::IFontFamily::IFontFamily;
// Java: import cn.wps.font.ITypeface
use crate::cn::wps::font::ITypeface::ITypeface;
// Java: import java.io.File
use std::path::{Path, PathBuf};
// Java: import java.util.ArrayList
// Java: import java.util.Iterator
use std::sync::{Arc, Mutex};

// Java field: String TAG
const TAG: Option<&str> = None;
// Java field: ArrayList<FontFamilyImpl> s_hold_native_list
// Java field: int MAX_STYLE_COUNT
const MAX_STYLE_COUNT: usize = 4;

enum FontFileEntry {
    Single(PathBuf),
    Multiple(Vec<PathBuf>),
}

// Java: public class FontFiles
pub struct FontFiles {
    // Java field: Object[] mFontFileMap
    mFontFileMap: Vec<Option<FontFileEntry>>,
}

// Java: public class FontFiles
impl FontFiles {
    // Java: FontFiles()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.mFontFileMap = new Object[4];
        FontFiles {
            mFontFileMap: vec![None, None, None, None],
        }
    }

    // Java: void addFontFilePath(int style, String path)
    pub fn addFontFilePath(&mut self, style: i32, path: &str) {
        // Java: if (style < 4)
        if (style as usize) < MAX_STYLE_COUNT {
            // Java: File file = new File(path);
            let file = PathBuf::from(path);
            // Java: if (file.exists())
            if file.exists() {
                // Java: ArrayList<File> files = null;
                // Java: Object[] fileMap = this.mFontFileMap;
                // Java: Object obj = fileMap[style];
                let idx = style as usize;
                let entry = self.mFontFileMap[idx].take();
                // Java: if (null == obj)
                if entry.is_none() {
                    // Java: fileMap[style] = file;
                    self.mFontFileMap[idx] = Some(FontFileEntry::Single(file));
                }
                // Java: else if (obj instanceof File)
                else if let Some(FontFileEntry::Single(org_file)) = entry {
                    // Java: File orgFile = (File)obj;
                    // Java: if (!orgFile.equals(file))
                    if org_file != file {
                        // Java: files = new ArrayList();
                        let mut files = Vec::new();
                        // Java: files.add(orgFile);
                        files.push(org_file);
                        // Java: files.add(file);
                        files.push(file);
                        // Java: fileMap[style] = files;
                        self.mFontFileMap[idx] = Some(FontFileEntry::Multiple(files));
                    } else {
                        self.mFontFileMap[idx] = Some(FontFileEntry::Single(org_file));
                    }
                }
                // Java: else if (obj instanceof ArrayList)
                else if let Some(FontFileEntry::Multiple(mut files)) = entry {
                    // Java: files = (ArrayList)obj;
                    // Java: if (!files.contains(files))
                    if !files.contains(&file) {
                        // Java: files.add(file);
                        files.push(file);
                    }
                    self.mFontFileMap[idx] = Some(FontFileEntry::Multiple(files));
                }
            }
        }
    }

    // Java: File doGetFilePathsByStyle(int style, ArrayList<File> outFiles)
    fn doGetFilePathsByStyle(&self, style: i32, out_files: Option<&mut Vec<PathBuf>>) -> Option<PathBuf> {
        // Java: Object obj = this.mFontFileMap[style];
        let idx = style as usize;
        if idx >= self.mFontFileMap.len() {
            return None;
        }
        let obj = &self.mFontFileMap[idx];
        // Java: if (null == obj)
        if obj.is_none() {
            // Java: return null;
            return None;
        }
        // Java: else if (obj instanceof File)
        else if let Some(FontFileEntry::Single(ref ret)) = obj {
            // Java: File ret = (File)obj;
            // Java: if (null != outFiles)
            if let Some(out) = out_files {
                // Java: outFiles.add(ret);
                out.push(ret.clone());
            }
            // Java: return ret;
            return Some(ret.clone());
        }
        // Java: else if (obj instanceof ArrayList)
        else if let Some(FontFileEntry::Multiple(ref files)) = obj {
            // Java: ArrayList<File> files = (ArrayList)obj;
            // Java: if (null != outFiles)
            if let Some(out) = out_files {
                // Java: outFiles.addAll(files);
                out.extend(files.iter().cloned());
            }
            // Java: return files.isEmpty() ? null : (File)files.get(0);
            return if files.is_empty() { None } else { Some(files[0].clone()) };
        }
        // Java: else
        {
            // Java: return null;
            None
        }
    }

    // Java: File getFontFile(int style)
    pub fn getFontFile(&self, style: i32) -> Option<PathBuf> {
        // Java: return this.doGetFilePathsByStyle(style, (ArrayList)null);
        self.doGetFilePathsByStyle(style, None)
    }

    // Java: String getFontFilePath(int style)
    pub fn getFontFilePath(&self, style: i32) -> Option<String> {
        // Java: File file = this.getFontFile(style);
        let file = self.getFontFile(style);
        // Java: return null != file ? file.getAbsolutePath() : null;
        file.map(|f| f.to_string_lossy().to_string())
    }

    // Java: void getFontFilePaths(int style, ArrayList<File> outFiles)
    pub fn getFontFilePaths(&self, style: i32, out_files: &mut Vec<PathBuf>) {
        // Java: outFiles.clear();
        out_files.clear();
        // Java: this.doGetFilePathsByStyle(style, outFiles);
        self.doGetFilePathsByStyle(style, Some(out_files));
    }

    // Java: void getAllFontFilePaths(ArrayList<File> outFiles)
    pub fn getAllFontFilePaths(&self, out_files: &mut Vec<PathBuf>) {
        // Java: outFiles.clear();
        out_files.clear();
        // Java: int i = 0;
        // Java: for(int n = this.mFontFileMap.length; i < n; ++i)
        for i in 0..self.mFontFileMap.len() {
            // Java: this.doGetFilePathsByStyle(i, outFiles);
            self.doGetFilePathsByStyle(i as i32, Some(out_files));
        }
    }
}

// Java: public class FontFamilyImpl implements IFontFamily
pub struct FontFamilyImpl {
    // Java field: boolean mIsSystemFont
    pub(crate) mIsSystemFont: bool,
    // Java field: boolean isOnlineFont
    pub(crate) isOnlineFont: Mutex<bool>,
    // Java field: String mFamilyName
    pub(crate) mFamilyName: String,
    // Java field: int mId
    pub(crate) mId: Mutex<i32>,
    // Java field: Long[] mNativePointers
    mNativePointers: Mutex<[Option<i64>; 4]>,
    // Java field: ArrayList<ITypeface> mTypefaces
    pub(crate) mTypefaces: Mutex<Vec<Option<Arc<dyn ITypeface>>>>,
    // Java field: FontFiles mFontFiles
    pub(crate) mFontFiles: Mutex<FontFiles>,
    // Java field: ArrayList<File> mTmpList
    // Java field: IFontFamily mSourceFontFamily
    pub(crate) mSourceFontFamily: Mutex<Option<Arc<dyn IFontFamily>>>,
    // Java field: boolean mIsBuildinHeader
    pub(crate) mIsBuildinHeader: Mutex<bool>,
}

// Java: public class FontFamilyImpl implements IFontFamily
impl FontFamilyImpl {
    // Java: FontFamilyImpl(String familyName, boolean isSystemFont, int id)
    pub fn new(family_name: String, is_system_font: bool, id: i32) -> Self {
        // Java: super();
        // Java: this.mFamilyName = familyName;
        // Java: this.mIsSystemFont = isSystemFont;
        // Java: this.mId = id;
        FontFamilyImpl {
            mFamilyName: family_name,
            mIsSystemFont: is_system_font,
            isOnlineFont: Mutex::new(false),
            mId: Mutex::new(id),
            mNativePointers: Mutex::new([None; 4]),
            mTypefaces: Mutex::new(Vec::with_capacity(4)),
            mFontFiles: Mutex::new(FontFiles::new()),
            mSourceFontFamily: Mutex::new(None),
            mIsBuildinHeader: Mutex::new(false),
        }
    }

    // Java: Iterator<ITypeface> iterator()
    pub fn iterator(&self) -> Vec<Option<Arc<dyn ITypeface>>> {
        // Java: return this.mTypefaces.iterator();
        self.mTypefaces.lock().unwrap().clone()
    }

    // Java: String getName()
    pub fn getName_impl(&self) -> String {
        // Java: return this.mFamilyName;
        self.mFamilyName.clone()
    }

    // Java: int getId()
    pub fn getId_impl(&self) -> i32 {
        // Java: return this.mId;
        *self.mId.lock().unwrap()
    }

    // Java: void setId(int id)
    pub fn setId(&self, id: i32) {
        // Java: this.mId = id;
        *self.mId.lock().unwrap() = id;
    }

    // Java: ITypeface getTypeface(int style)
    pub fn getTypeface_impl(&self, style: i32) -> Option<Arc<dyn ITypeface>> {
        // Java: if (style < 0)
        if style < 0 {
            // Java: return null;
            return None;
        }
        // Java: else
        {
            // Java: ITypeface face = null;
            // Java: face = this.createTypeface(style);
            let face = self.createTypeface(style);
            // Java: if (face == null)
            if face.is_none() {
                // Java: face = FontHost.getFontTable().getDefault().getTypeface(style);
                return None;
            }
            // Java: return face;
            face
        }
    }

    // Java: void addTypeface(int style, ITypeface face)
    pub fn addTypeface(&self, style: i32, face: Arc<dyn ITypeface>) {
        // Java: if (face == null)
        // Java: throw new IllegalArgumentException("Invalid typeface");
        // Java: else
        {
            let mut typefaces = self.mTypefaces.lock().unwrap();
            // Java: for(int i = this.mTypefaces.size(); i < style + 1; ++i)
            while typefaces.len() < (style + 1) as usize {
                // Java: this.mTypefaces.add(null);
                typefaces.push(None);
            }
            // Java: this.mTypefaces.set(style, face);
            typefaces[style as usize] = Some(face);
        }
    }

    // Java: ITypeface createTypeface(int style)
    pub fn createTypeface(&self, style: i32) -> Option<Arc<dyn ITypeface>> {
        // Java: if (style < 0)
        if style < 0 {
            // Java: return null;
            return None;
        }
        // Java: else
        {
            // Java: ITypeface face = null;
            let mut face: Option<Arc<dyn ITypeface>> = None;
            // Java: if (style < this.mTypefaces.size())
            {
                let typefaces = self.mTypefaces.lock().unwrap();
                if (style as usize) < typefaces.len() {
                    // Java: face = (ITypeface)this.mTypefaces.get(style);
                    face = typefaces[style as usize].clone();
                }
            }
            // Java: if (face == null)
            if face.is_none() {
                // Java: if (this.mIsBuildinHeader)
                if *self.mIsBuildinHeader.lock().unwrap() {
                    // Java: if (null != this.mSourceFontFamily)
                    let source = self.mSourceFontFamily.lock().unwrap().clone();
                    if source.is_some() {
                        // Java: ITypeface typeFace = this.mSourceFontFamily.getTypeface(style);
                        // Java: face = TypefaceFactory.createFromSourceFace(this, typeFace, style);
                        // Java: ((AbstractTypeface)face).setNeedBold(typeFace.needBold());
                        // Java: ((AbstractTypeface)face).setNeedItalic(typeFace.needItalic());
                    }
                    // Java: else
                    else {
                        // Java: face = TypefaceFactory.createTypeface(this, 0, (String)null);
                        // Java: ((AbstractTypeface)face).setNeedBold((style & 1) == 1);
                        // Java: ((AbstractTypeface)face).setNeedItalic((style & 2) == 2);
                    }
                }
                // Java: else
                else {
                    // Java: String fileName = null;
                    // Java: FontFiles paths = this.mFontFiles;
                    let font_files = self.mFontFiles.lock().unwrap();
                    // Java: if (null != (fileName = paths.getFontFilePath(style)))
                    if let Some(_file_name) = font_files.getFontFilePath(style) {
                        // Java: face = TypefaceFactory.createTypeface(this, style, fileName);
                    }
                    // Java: else if (null != (fileName = paths.getFontFilePath(0)))
                    else if let Some(_file_name) = font_files.getFontFilePath(0) {
                        // Java: face = TypefaceFactory.createTypeface(this, style, fileName);
                        // Java: ((AbstractTypeface)face).setNeedBold((style & 1) == 1);
                        // Java: ((AbstractTypeface)face).setNeedItalic((style & 2) == 2);
                    }
                    // Java: else if (null != (fileName = paths.getFontFilePath(1)))
                    else if let Some(_file_name) = font_files.getFontFilePath(1) {
                        // Java: face = TypefaceFactory.createTypeface(this, style, fileName);
                        // Java: ((AbstractTypeface)face).setNeedItalic((style & 2) == 2);
                    }
                    // Java: else if (null != (fileName = paths.getFontFilePath(2)))
                    else if let Some(_file_name) = font_files.getFontFilePath(2) {
                        // Java: face = TypefaceFactory.createTypeface(this, style, fileName);
                        // Java: ((AbstractTypeface)face).setNeedBold((style & 1) == 1);
                    }
                    // Java: else if (null != (fileName = paths.getFontFilePath(3)))
                    else if let Some(_file_name) = font_files.getFontFilePath(3) {
                        // Java: face = TypefaceFactory.createTypeface(this, style, fileName);
                    }
                }
                // Java: if (face != null)
                if let Some(ref f) = face {
                    // Java: this.addTypeface(style, face);
                    self.addTypeface(style, f.clone());
                }
            }
            // Java: return face;
            face
        }
    }

    // Java: void addFontFile(int style, String fileName)
    pub fn addFontFile_impl(&self, style: i32, file_name: &str) {
        // Java: this.mFontFiles.addFontFilePath(style, fileName);
        self.mFontFiles.lock().unwrap().addFontFilePath(style, file_name);
    }

    // Java: ITypeface addTypefaceFromFile(int style, String file)
    pub fn addTypefaceFromFile_impl(&self, style: i32, file: &str) -> Option<Arc<dyn ITypeface>> {
        // Java: ITypeface face = TypefaceFactory.createTypeface(this, style, file);
        let face: Option<Arc<dyn ITypeface>> = None;
        // Java: this.mFontFiles.addFontFilePath(style, file);
        self.mFontFiles.lock().unwrap().addFontFilePath(style, file);
        // Java: this.addTypeface(style, face);
        // Java: return face;
        face
    }

    // Java: String toString()
    pub fn to_string_impl(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: if (this.mIsSystemFont)
        if self.mIsSystemFont {
            // Java: sb.append("isSystemFont\r");
            sb.push_str("isSystemFont\r");
        }
        // Java: for(int i = 0; i < 4; ++i)
        let font_files = self.mFontFiles.lock().unwrap();
        for i in 0..4 {
            // Java: String path = this.mFontFiles.getFontFilePath(i);
            let path = font_files.getFontFilePath(i).unwrap_or_default();
            // Java: sb.append(i + ":" + path + " === " + this.mTypefaces.get(i) + "\r");
            sb.push_str(&format!("{}:{} === typeface\r", i, path));
        }
        // Java: return sb.toString();
        sb
    }

    // Java: boolean isSystemFont()
    pub fn isSystemFont_impl(&self) -> bool {
        // Java: return this.mIsSystemFont;
        self.mIsSystemFont
    }

    // Java: boolean isOnlineFont()
    pub fn isOnlineFont_impl(&self) -> bool {
        // Java: return this.isOnlineFont;
        *self.isOnlineFont.lock().unwrap()
    }

    // Java: void setOnlineFont(boolean isOnlineFont)
    pub fn setOnlineFont_impl(&self, is_online_font: bool) {
        // Java: this.isOnlineFont = isOnlineFont;
        *self.isOnlineFont.lock().unwrap() = is_online_font;
    }

    // Java: boolean fontFileExist()
    pub fn fontFileExist_impl(&self) -> bool {
        // Java: ArrayList<File> filePaths = this.mTmpList;
        // Java: this.mFontFiles.getAllFontFilePaths(filePaths);
        let mut file_paths = Vec::new();
        self.mFontFiles.lock().unwrap().getAllFontFilePaths(&mut file_paths);
        // Java: int i = 0;
        // Java: for(int n = filePaths.size(); i < n; ++i)
        for file in &file_paths {
            // Java: File file = (File)filePaths.get(i);
            // Java: if (file != null && file.exists())
            if file.exists() {
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: String[] convertFilesToStrings(ArrayList<File> files)
    fn convertFilesToStrings(files: &[PathBuf]) -> Vec<String> {
        // Java: int fileCount = files.size();
        // Java: String[] ret = new String[fileCount];
        let mut ret = Vec::with_capacity(files.len());
        // Java: for(int i = 0; i < fileCount; ++i)
        for file in files {
            // Java: ret[i] = ((File)files.get(i)).getAbsolutePath();
            ret.push(file.to_string_lossy().to_string());
        }
        // Java: return ret;
        ret
    }

    // Java: String[] getFontFiles(int style)
    pub fn getFontFiles_impl(&self, style: i32) -> Vec<String> {
        // Java: ArrayList<File> tmpList = this.mTmpList;
        let mut tmp_list = Vec::new();
        // Java: this.mFontFiles.getFontFilePaths(style, tmpList);
        self.mFontFiles.lock().unwrap().getFontFilePaths(style, &mut tmp_list);
        // Java: return this.convertFilesToStrings(tmpList);
        Self::convertFilesToStrings(&tmp_list)
    }

    // Java: String getFontFile(int style)
    pub fn getFontFile_impl(&self, style: i32) -> String {
        // Java: return this.mFontFiles.getFontFilePath(style);
        self.mFontFiles.lock().unwrap().getFontFilePath(style).unwrap_or_default()
    }

    // Java: String[] getAllFontFiles()
    pub fn getAllFontFiles_impl(&self) -> Vec<String> {
        // Java: ArrayList<File> tmpList = this.mTmpList;
        let mut tmp_list = Vec::new();
        // Java: this.mFontFiles.getAllFontFilePaths(tmpList);
        self.mFontFiles.lock().unwrap().getAllFontFilePaths(&mut tmp_list);
        // Java: return this.convertFilesToStrings(tmpList);
        Self::convertFilesToStrings(&tmp_list)
    }

    // Java: Long ensureNativeFontPointerByFullPath(File fontFile)
    fn ensureNativeFontPointerByFullPath(&self, font_file: &Path) -> Option<i64> {
        // Java: if (null == fontFile)
        // Java: return 0L;
        // Java: else
        {
            // Java: Long ptr = null;
            let mut ptr: Option<i64> = None;
            // Java: FontFiles files = this.mFontFiles;
            let font_files = self.mFontFiles.lock().unwrap();
            // Java: Long[] pointers = this.mNativePointers;
            let pointers = self.mNativePointers.lock().unwrap();
            // Java: int i = 0;
            // Java: for(int n = 4; i < n; ++i)
            for i in 0..MAX_STYLE_COUNT {
                // Java: if (fontFile.equals(files.getFontFile(i)))
                if let Some(ref f) = font_files.getFontFile(i as i32) {
                    if f.as_path() == font_file {
                        // Java: Long p = pointers[i];
                        // Java: if (null != p)
                        if let Some(p) = pointers[i] {
                            // Java: ptr = p;
                            ptr = Some(p);
                            // Java: break;
                            break;
                        }
                    }
                }
            }
            drop(pointers);
            // Java: if (null == ptr)
            if ptr.is_none() {
                // Java: ptr = FreeTypeJNI.newTypeface(fontFile.getAbsolutePath());
                ptr = Some(FreeTypeJNI::FreeTypeJNI::newTypeface(&font_file.to_string_lossy()));
            }
            // Java: i = 0;
            // Java: for(int n = 4; i < n; ++i)
            let mut pointers = self.mNativePointers.lock().unwrap();
            for i in 0..MAX_STYLE_COUNT {
                // Java: if (null == pointers[i] && fontFile.equals(files.getFontFile(i)))
                if pointers[i].is_none() {
                    if let Some(ref f) = font_files.getFontFile(i as i32) {
                        if f.as_path() == font_file {
                            // Java: pointers[i] = ptr;
                            pointers[i] = ptr;
                        }
                    }
                }
            }
            // Java: return ptr;
            ptr
        }
    }

    // Java: void closeNativePtr()
    pub fn closeNativePtr_impl(&self) {
        // Java: int i = 0;
        // Java: for(int size = this.mTypefaces.size(); i < size; ++i)
        {
            // Java: ITypeface typeface = (ITypeface)this.mTypefaces.get(i);
            // Java: if (typeface instanceof AndroidTypeface)
            // Java: ((AndroidTypeface)typeface).resetNative();
        }
        // Java: for(i = 0; i < 4; ++i)
        let mut pointers = self.mNativePointers.lock().unwrap();
        for i in 0..MAX_STYLE_COUNT {
            // Java: Long ptr = this.mNativePointers[i];
            let ptr = pointers[i];
            // Java: if (ptr != null && ptr != 0L)
            if let Some(p) = ptr {
                if p != 0 {
                    // Java: FreeTypeJNI.deleteTypeface(ptr);
                    FreeTypeJNI::FreeTypeJNI::deleteTypeface(p);
                    // Java: this.mNativePointers[i] = null;
                    pointers[i] = None;
                    // Java: for(int j = i + 1; j < 4; ++j)
                    for j in (i + 1)..MAX_STYLE_COUNT {
                        // Java: if (this.mNativePointers[j] != null && this.mNativePointers[j] == ptr)
                        if pointers[j] == Some(p) {
                            // Java: this.mNativePointers[j] = null;
                            pointers[j] = None;
                        }
                    }
                }
            }
        }
    }

    // Java: long getNativeFreeTypeFontPointer(int style)
    pub fn getNativeFreeTypeFontPointer_impl(&self, style: i32) -> i64 {
        // Java: Long ptr = this.mNativePointers[style];
        let idx = style as usize;
        let ptr = self.mNativePointers.lock().unwrap()[idx];
        // Java: if (null != ptr)
        if let Some(p) = ptr {
            // Java: return ptr;
            return p;
        }
        // Java: else
        {
            // Java: FontFiles fontFilePaths = this.mFontFiles;
            let font_files = self.mFontFiles.lock().unwrap();
            // Java: File fontFile = fontFilePaths.getFontFile(style);
            let font_file = font_files.getFontFile(style);
            let mut result_ptr: Option<i64> = None;
            // Java: if (null != fontFile)
            if let Some(ref ff) = font_file {
                // Java: ptr = this.ensureNativeFontPointerByFullPath(fontFile);
                result_ptr = self.ensureNativeFontPointerByFullPath(ff);
            }
            // Java: if (null == ptr)
            if result_ptr.is_none() {
                // Java: int i = 0;
                // Java: for(int n = 4; i < n; ++i)
                for i in 0..MAX_STYLE_COUNT {
                    // Java: File file = fontFilePaths.getFontFile(i);
                    let file = font_files.getFontFile(i as i32);
                    // Java: if (null != file && file.exists())
                    if let Some(ref f) = file {
                        if f.exists() {
                            // Java: ptr = this.ensureNativeFontPointerByFullPath(file);
                            result_ptr = self.ensureNativeFontPointerByFullPath(f);
                            // Java: if (null != ptr)
                            if result_ptr.is_some() {
                                // Java: break;
                                break;
                            }
                        }
                    }
                }
            }
            // Java: if (null == ptr)
            if result_ptr.is_none() {
                // Java: ptr = 0L;
                result_ptr = Some(0);
            }
            // Java: this.mNativePointers[style] = ptr;
            self.mNativePointers.lock().unwrap()[idx] = result_ptr;
            // Java: return ptr;
            result_ptr.unwrap_or(0)
        }
    }

    // Java: void setBuildInfo(IFontFamily sourceFontFamily, boolean isBuildinHeader)
    pub fn setBuildInfo(&self, source_font_family: Option<Arc<dyn IFontFamily>>, is_buildin_header: bool) {
        // Java: this.mSourceFontFamily = sourceFontFamily;
        *self.mSourceFontFamily.lock().unwrap() = source_font_family;
        // Java: this.mIsBuildinHeader = isBuildinHeader;
        *self.mIsBuildinHeader.lock().unwrap() = is_buildin_header;
    }

    // Java: boolean isBuildinHeader()
    pub fn isBuildinHeader_impl(&self) -> bool {
        // Java: return this.mIsBuildinHeader;
        *self.mIsBuildinHeader.lock().unwrap()
    }

    // Java: IFontFamily getSourceFont()
    pub fn getSourceFont_impl(&self) -> Option<Arc<dyn IFontFamily>> {
        // Java: return this.mSourceFontFamily;
        self.mSourceFontFamily.lock().unwrap().clone()
    }
}

impl IFontFamily for FontFamilyImpl {
    fn getName(&self) -> String { self.getName_impl() }
    fn getId(&self) -> i32 { self.getId_impl() }
    fn getTypeface(&self, style: i32) -> Option<Arc<dyn ITypeface>> { self.getTypeface_impl(style) }
    fn isSystemFont(&self) -> bool { self.isSystemFont_impl() }
    fn isOnlineFont(&self) -> bool { self.isOnlineFont_impl() }
    fn fontFileExist(&self) -> bool { self.fontFileExist_impl() }
    fn setOnlineFont(&self, b: bool) { self.setOnlineFont_impl(b) }
    fn getFontFiles(&self, style: i32) -> Vec<String> { self.getFontFiles_impl(style) }
    fn getFontFile(&self, style: i32) -> String { self.getFontFile_impl(style) }
    fn getAllFontFiles(&self) -> Vec<String> { self.getAllFontFiles_impl() }
    fn getNativeFreeTypeFontPointer(&self, style: i32) -> i64 { self.getNativeFreeTypeFontPointer_impl(style) }
    fn addTypefaceFromFile(&self, style: i32, file: String) -> Option<Arc<dyn ITypeface>> { self.addTypefaceFromFile_impl(style, &file) }
    fn addFontFile(&self, style: i32, file_name: String) { self.addFontFile_impl(style, &file_name) }
    fn isBuildinHeader(&self) -> bool { self.isBuildinHeader_impl() }
    fn getSourceFont(&self) -> Option<Arc<dyn IFontFamily>> { self.getSourceFont_impl() }
    fn closeNativePtr(&self) { self.closeNativePtr_impl() }
}
