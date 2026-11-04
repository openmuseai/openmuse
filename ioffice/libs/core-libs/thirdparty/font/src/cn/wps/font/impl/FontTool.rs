// Java: import cn.wps.base.assertion.Assert
// Java: import cn.wps.base.log.Log
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.font.IFontFamily
// Java: import cn.wps.font.IFontTable
// Java: import cn.wps.moffice.util.FileUtil
// Java: import cn.wps.moffice.util.StreamUtil
// Java: import cn.wps.moffice.util.StringUtil
// Java: import java.io.BufferedReader
// Java: import java.io.BufferedWriter
// Java: import java.io.File
// Java: import java.io.FileOutputStream
// Java: import java.io.FileReader
// Java: import java.io.FilenameFilter
// Java: import java.io.IOException
// Java: import java.io.OutputStreamWriter
// Java: import java.io.PrintWriter
// Java: import java.io.Writer
// Java: import java.nio.charset.StandardCharsets
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.HashSet
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.Set
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use super::FontParser::FontParser;
use super::FontHostSettings::SchemaItem;

// Java field: String VERSION_TAG
pub(crate) const VERSION_TAG: &str = "Version";
// Java field: float curVersion
pub(crate) const curVersion: f32 = 5.0;
// Java field: boolean needOverwrite
static needOverwrite: OnceLock<Mutex<bool>> = OnceLock::new();
// Java field: String TAG
static TAG: OnceLock<Mutex<String>> = OnceLock::new();
// Java field: HashMap<String, String> customJPFontMap
static customJPFontMap: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
// Java field: HashMap<String, String> customFonts
static customFonts: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
// Java field: HashSet<FontHostSettings.SchemaItem> msFontMappingSchemas
static msFontMappingSchemas: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
// Java field: HashSet<String> msSelfExtractedFontNames
static msSelfExtractedFontNames: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
// Java field: String FONT_FZ_FSJW
pub(crate) const FONT_FZ_FSJW: &str = "FZFSJW.TTF";
// Java field: String FONT_FZ_FSK
pub(crate) const FONT_FZ_FSK: &str = "FZFSK.TTF";
// Java field: String FONT_FZ_HTJW
pub(crate) const FONT_FZ_HTJW: &str = "FZHTJW.TTF";
// Java field: String FONT_FZ_HTK
pub(crate) const FONT_FZ_HTK: &str = "FZHTK.TTF";
// Java field: String FONT_FZ_KTJW
pub(crate) const FONT_FZ_KTJW: &str = "FZKTJW.TTF";
// Java field: String FONT_FZ_KTK
pub(crate) const FONT_FZ_KTK: &str = "FZKTK.TTF";
// Java field: String FONT_FZ_XBSJW
pub(crate) const FONT_FZ_XBSJW: &str = "FZXBSJW.TTF";
// Java field: String FONT_FZ_XBSK
pub(crate) const FONT_FZ_XBSK: &str = "FZXBSK.TTF";
// Java field: boolean ENABLE_FZ_FONT
pub(crate) const ENABLE_FZ_FONT: bool = false;
// Java field: String FONT_KINGSOFT_WINDINGS
pub(crate) const FONT_KINGSOFT_WINDINGS: &str = "wps_Kingsoft_Windings.ttf";
// Java field: String FONT_KINGSOFT_EXTRA
pub(crate) const FONT_KINGSOFT_EXTRA: &str = "wps_Kingsoft_Extra.ttf";
// Java field: String FONT_KINGSOFT_SYMBOL
pub(crate) const FONT_KINGSOFT_SYMBOL: &str = "wps_Kingsoft_Symbol.ttf";
// Java field: String FONT_KINGSOFT_MATH
pub(crate) const FONT_KINGSOFT_MATH: &str = "wps_Kingsoft_Math.ttf";
// Java field: String FONT_KINGSOFT_MARK
pub(crate) const FONT_KINGSOFT_MARK: &str = "wps_Kingsoft_Mark.ttf";
// Java field: String FONT_KINGSOFT_STRESS
pub(crate) const FONT_KINGSOFT_STRESS: &str = "wps_Kingsoft_Stress.ttf";
// Java field: String FONT_SCHEHERAZADE
pub(crate) const FONT_SCHEHERAZADE: &str = "Scheherazade.ttf";
// Java field: String SCHEHERAZADE_NAME
pub(crate) const SCHEHERAZADE_NAME: &str = "Scheherazade";
// Java field: String[] OLD_FONT_FILES
static OLD_FONT_FILES: OnceLock<Vec<String>> = OnceLock::new();
// Java field: HashMap<String, String[]> splitFileMap
static splitFileMap: OnceLock<Mutex<HashMap<String, Vec<String>>>> = OnceLock::new();
// Java field: String[] unsport
static unsport: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

fn get_needOverwrite() -> &'static Mutex<bool> {
    needOverwrite.get_or_init(|| Mutex::new(false))
}

fn get_customJPFontMap() -> &'static Mutex<HashMap<String, String>> {
    customJPFontMap.get_or_init(|| Mutex::new(HashMap::new()))
}

fn get_customFonts() -> &'static Mutex<HashMap<String, String>> {
    customFonts.get_or_init(|| Mutex::new(HashMap::new()))
}

fn get_unsport() -> &'static Mutex<Vec<String>> {
    unsport.get_or_init(|| Mutex::new(Vec::new()))
}

// Java: public class FontTool
pub struct FontTool {
}

// Java: public class FontTool
impl FontTool {
    // Java: FontTool()
    pub fn new() -> Self {
        // Java: super();
        FontTool {}
    }

    // Java: static boolean makeFontCache2(String[] paths, String cacheFile, IFontTable fontTable)
    pub fn makeFontCache2(paths: &[String], cacheFile: &str, _fontTable: &dyn crate::cn::wps::font::IFontTable::IFontTable) -> bool {
        // Java: boolean modify = false;
        let mut modify = false;
        // Java: try
        {
            // Java: List<File> fontFiles = getAllFontFiles(paths);
            let mut fontFiles = Self::getAllFontFiles(paths);
            // Java: StringBuilder cacheTxt = new StringBuilder();
            let mut cacheTxt = String::new();
            // Java: File oldCache = new File(cacheFile);
            let old_cache = Path::new(cacheFile);
            // Java: float oldVersion = 0.0F;
            let mut old_version: f32 = 0.0;
            // Java: File newCache = new File(cacheFile + ".tmp");
            let new_cache_path = format!("{}.tmp", cacheFile);
            // Java: Writer writer = new PrintWriter(new BufferedWriter(new OutputStreamWriter(new FileOutputStream(newCache), StandardCharsets.UTF_8)));
            let mut writer = match File::create(&new_cache_path) {
                Ok(f) => f,
                Err(_) => return false,
            };
            // Java: writer.write("Version5.0\r");
            let _ = writer.write_all(b"Version5.0\r");
            // Java: Set<File> removeLater = new HashSet();
            let mut remove_later: HashSet<PathBuf> = HashSet::new();
            // Java: if (oldCache.exists())
            if old_cache.exists() {
                // Java: BufferedReader br = new BufferedReader(new FileReader(oldCache));
                if let Ok(file) = File::open(old_cache) {
                    let reader = BufReader::new(file);
                    let mut lines = reader.lines();
                    // Java: String line = br.readLine();
                    let mut line = lines.next().and_then(|l| l.ok());
                    // Java: if (null != line)
                    if let Some(ref l) = line {
                        // Java: if (line.length() > 0)
                        if !l.is_empty() {
                            // Java: if (line.startsWith("Version"))
                            if l.starts_with("Version") {
                                // Java: oldVersion = Float.parseFloat(line.substring("Version".length()));
                                old_version = l["Version".len()..].parse::<f32>().unwrap_or(0.0);
                                // Java: line = br.readLine();
                                line = lines.next().and_then(|l| l.ok());
                            }
                        }
                        // Java: else
                        else {
                            // Java: line = br.readLine();
                            line = lines.next().and_then(|l| l.ok());
                        }
                    }
                    // Java: while(null != line)
                    while let Some(ref l) = line {
                        // Java: if (line.length() <= 0)
                        if l.is_empty() {
                            // Java: line = br.readLine();
                            line = lines.next().and_then(|l| l.ok());
                        }
                        // Java: else
                        else {
                            // Java: String[] names = line.split("\t");
                            let names: Vec<&str> = l.split('\t').collect();
                            // Java: if (null != names && names.length >= 3)
                            if names.len() >= 3 {
                                // Java: String fontFileName = names[2];
                                let font_file_name = names[2];
                                // Java: boolean findSame = false;
                                let mut find_same = false;
                                // Java: for(int i = 0; i < fontFiles.size(); ++i)
                                let mut i = 0;
                                while i < fontFiles.len() {
                                    // Java: if (fontFileName.equals(((File)fontFiles.get(i)).getAbsolutePath()))
                                    if font_file_name == fontFiles[i].to_str().unwrap_or("") {
                                        // Java: String subFix = fontFileName.substring(fontFileName.lastIndexOf(".") + 1);
                                        let sub_fix = font_file_name.rsplit('.').next().unwrap_or("");
                                        // Java: if (subFix.equalsIgnoreCase("ttf"))
                                        if sub_fix.eq_ignore_ascii_case("ttf") {
                                            // Java: fontFiles.remove(i);
                                            fontFiles.remove(i);
                                        }
                                        // Java: else
                                        else {
                                            // Java: removeLater.add((File)fontFiles.get(i));
                                            remove_later.insert(fontFiles[i].clone());
                                            i += 1;
                                        }
                                        // Java: cacheTxt.append(line);
                                        cacheTxt.push_str(l);
                                        // Java: cacheTxt.append("\r");
                                        cacheTxt.push('\r');
                                        // Java: findSame = true;
                                        find_same = true;
                                        // Java: break;
                                        break;
                                    }
                                    i += 1;
                                }
                                // Java: if (!findSame)
                                if !find_same {
                                    // Java: modify = true;
                                    modify = true;
                                }
                            }
                            // Java: line = br.readLine();
                            line = lines.next().and_then(|l| l.ok());
                        }
                    }
                    // Java: writer.write(cacheTxt.toString());
                    let _ = writer.write_all(cacheTxt.as_bytes());
                    // Java: br.close();
                }
                // Java: oldCache.delete();
                let _ = fs::remove_file(old_cache);
            }
            // Java: if (!removeLater.isEmpty())
            if !remove_later.is_empty() {
                // Java: Iterator<File> it = removeLater.iterator();
                // Java: while(it.hasNext())
                for item in &remove_later {
                    // Java: fontFiles.remove(it.next());
                    fontFiles.retain(|f| f != item);
                }
            }
            // Java: needOverwrite = oldVersion < 5.0F;
            *get_needOverwrite().lock().unwrap() = old_version < 5.0;
            // Java: if (parseFontAndMakeCache((File[])fontFiles.toArray(new File[0]), writer))
            if Self::parseFontAndMakeCache(&fontFiles, &mut writer) {
                // Java: modify = true;
                modify = true;
            }
            // Java: writer.flush();
            let _ = writer.flush();
            // Java: writer.close();
            drop(writer);
            // Java: newCache.renameTo(oldCache);
            let _ = fs::rename(&new_cache_path, cacheFile);
        }
        // Java: catch (Throwable e)
        // Java: Log.e(TAG, "", e);
        // Java: deleteOldFontFiles();
        Self::deleteOldFontFiles();
        // Java: return modify;
        modify
    }

    // Java: static List<File> getAllFontFiles(String[] paths)
    pub fn getAllFontFiles(paths: &[String]) -> Vec<PathBuf> {
        // Java: List<File> fontFiles = new ArrayList();
        let mut fontFiles: Vec<PathBuf> = Vec::new();
        // Java: if (null == paths)
        if paths.is_empty() {
            // Java: Assert.fail("Font paths should not be null!");
            // Java: return fontFiles;
            return fontFiles;
        }
        // Java: else
        // Java: for(int i = 0; i < paths.length; ++i)
        for path_str in paths {
            // Java: File dir = new File(paths[i]);
            let dir = Path::new(path_str);
            // Java: if (dir.exists() && dir.isDirectory())
            if dir.exists() && dir.is_dir() {
                // Java: File[] files = dir.listFiles(new FilenameFilter()
                // Java: public boolean accept(File dir, String name)
                // Java: return FontParser.hasFontFileSuffix(name);
                if let Ok(entries) = fs::read_dir(dir) {
                    // Java: if (null != files)
                    // Java: for(int j = 0; j < files.length; ++j)
                    for entry in entries.flatten() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if FontParser::hasFontFileSuffix(&name) {
                            // Java: fontFiles.add(files[j]);
                            fontFiles.push(entry.path());
                        }
                    }
                }
            }
        }
        // Java: return fontFiles;
        fontFiles
    }

    // Java: static void makeFontCache(String[] paths, String cacheFile)
    pub fn makeFontCache(paths: &[String], cacheFile: &str) {
        // Java: List<File> fontFiles = getAllFontFiles(paths);
        let fontFiles = Self::getAllFontFiles(paths);
        // Java: if (fontFiles.size() != 0)
        if !fontFiles.is_empty() {
            // Java: try
            if let Ok(mut writer) = File::create(cacheFile) {
                // Java: Writer writer = new PrintWriter(new FileOutputStream(cacheFile));
                // Java: parseFontAndMakeCache((File[])fontFiles.toArray(new File[0]), writer);
                Self::parseFontAndMakeCache(&fontFiles, &mut writer);
                // Java: writer.flush();
                let _ = writer.flush();
                // Java: writer.close();
            }
            // Java: catch (Throwable e)
            // Java: Log.e(TAG, "", e);
        }
    }

    // Java: static boolean parseFontAndMakeCache(File[] files, Writer writer)
    pub fn parseFontAndMakeCache(files: &[PathBuf], writer: &mut dyn Write) -> bool {
        // Java: boolean added = false;
        let mut added = false;
        // Java: try
        // Java: int j = 0;
        // Java: for(int size = files.length; j < size; ++j)
        for file in files {
            // Java: File file = files[j];
            // Java: if (!file.isDirectory())
            if !file.is_dir() {
                // Java: try
                // Java: if (!file.getName().startsWith("wps_"))
                let file_name = file.file_name().unwrap_or_default().to_string_lossy();
                if !file_name.starts_with("wps_") {
                    // Java: String[][] fontNames = FontParser.getFontNames(file);
                    let font_names = FontParser::getFontNames(file.as_path());
                    // Java: if (null != fontNames && fontNames.length != 0)
                    if let Some(ref names) = font_names {
                        if !names.is_empty() {
                            // Java: String fileName = file.getAbsolutePath();
                            let file_path = file.to_string_lossy();
                            // Java: for(int n = 0; n < fontNames.length; ++n)
                            for font_name in names {
                                // Java: String[] fontName = fontNames[n];
                                // Java: if (null != fontName && fontName.length >= 2)
                                if font_name.len() >= 2 {
                                    // Java: writer.write(fontName[0] + "\t" + fontName[1] + "\t" + fileName + "\r\n");
                                    let _ = write!(writer, "{}\t{}\t{}\r\n", font_name[0], font_name[1], file_path);
                                    // Java: added = true;
                                    added = true;
                                }
                            }
                        }
                    }
                }
                // Java: catch (Throwable e)
                // Java: Log.e(TAG, "", e);
            }
        }
        // Java: if (Platform.getLanguageCode() != null && Platform.getLanguageCode().startsWith("ja-JP"))
        {
            // Java: for(String fontName : customJPFontMap.keySet())
            let jp_fonts = get_customJPFontMap().lock().unwrap();
            for (font_name, custom_font_name) in jp_fonts.iter() {
                // Java: String fileName = saveCustomFont((String)customJPFontMap.get(fontName));
                let file_name = Self::saveCustomFont(custom_font_name);
                // Java: if (null != fileName)
                if let Some(ref f) = file_name {
                    // Java: writer.write(fontName + "\t" + fontName + "\t" + fileName + "\r\n");
                    let _ = write!(writer, "{}\t{}\t{}\r\n", font_name, font_name, f);
                    // Java: added = true;
                    added = true;
                }
            }
        }
        // Java: for(String fontName : customFonts.keySet())
        let fonts = get_customFonts().lock().unwrap();
        for (font_name, custom_font_name) in fonts.iter() {
            // Java: String fileName = saveCustomFont((String)customFonts.get(fontName));
            let file_name = Self::saveCustomFont(custom_font_name);
            // Java: if (null != fileName)
            if let Some(ref f) = file_name {
                // Java: writer.write(fontName + "\t" + fontName + "\t" + fileName + "\r\n");
                let _ = write!(writer, "{}\t{}\t{}\r\n", font_name, font_name, f);
                // Java: added = true;
                added = true;
            }
        }
        // Java: catch (IOException e)
        // Java: Log.e(TAG, "", e);
        // Java: catch (Throwable e)
        // Java: Log.e(TAG, "", e);
        // Java: return added;
        added
    }

    // Java: static void exportAssetsFonts(Writer writer)
    pub fn exportAssetsFonts(writer: &mut dyn Write) {
        // Java: if (null != Platform.getAssetManager())
        {
            // Java: String[] fontFiles = Platform.getAssetManager().list("fonts");
            let font_files: Vec<String> = Vec::new();
            // Java: if (null != fontFiles)
            if !font_files.is_empty() {
                // Java: for(int i = 0; i < fontFiles.length; ++i)
                for font_file_entry in &font_files {
                    // Java: String name = fontFiles[i];
                    let name = font_file_entry.clone();
                    // Java: String tar = Platform.getCustomFontsDirectory() + name;
                    let tar = name.clone();
                    // Java: FileUtil.copyFileOrg(Platform.getAssetManager().open("fonts/" + name), tar);
                    // Java: File file = new File(tar);
                    let file = Path::new(&tar);
                    // Java: String[][] fontNames = FontParser.getFontNames(file);
                    let font_names = FontParser::getFontNames(file);
                    // Java: if (null != fontNames && fontNames.length != 0)
                    if let Some(ref names) = font_names {
                        if !names.is_empty() {
                            // Java: String fileName = file.getAbsolutePath();
                            let file_name = file.to_string_lossy();
                            // Java: for(int n = 0; n < fontNames.length; ++n)
                            for font_name in names {
                                // Java: String[] fontName = fontNames[n];
                                // Java: if (null != fontName && fontName.length >= 2)
                                if font_name.len() >= 2 {
                                    // Java: writer.write(fontName[0] + "\t" + fontName[1] + "\t" + fileName + "\r\n");
                                    let _ = write!(writer, "{}\t{}\t{}\r\n", font_name[0], font_name[1], file_name);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Java: static void deleteOldFontFiles()
    pub fn deleteOldFontFiles() {
        // Java: for(String fileName : OLD_FONT_FILES)
        if let Some(old_files) = OLD_FONT_FILES.get() {
            for file_name in old_files.iter() {
                // Java: String fontFileName = Platform.getCustomFontsDirectory() + fileName;
                let font_file_name = file_name.clone();
                // Java: String oldFontFileName = Platform.getOldCustomFontsDirectory() + fileName;
                let old_font_file_name = file_name.clone();
                // Java: try
                {
                    // Java: File fontFile = new File(fontFileName);
                    let font_file = Path::new(&font_file_name);
                    // Java: if (fontFile.exists())
                    if font_file.exists() {
                        // Java: fontFile.delete();
                        let _ = fs::remove_file(font_file);
                    }
                    // Java: File oldFontFile = new File(oldFontFileName);
                    let old_font_file = Path::new(&old_font_file_name);
                    // Java: if (oldFontFile.exists())
                    if old_font_file.exists() {
                        // Java: oldFontFile.delete();
                        let _ = fs::remove_file(old_font_file);
                    }
                }
                // Java: catch (Throwable e)
                // Java: Log.e(TAG, "", e);
            }
        }
    }

    // Java: static boolean init(String fontCacheFile, IFontTable fontTable)
    pub fn init(font_cache_file: &str, font_table: &dyn crate::cn::wps::font::IFontTable::IFontTable) -> bool {
        // Java: File cacheFile = new File(fontCacheFile);
        let cache_file = Path::new(font_cache_file);
        // Java: if (!cacheFile.exists())
        if !cache_file.exists() {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: boolean b = doLoadCache(cacheFile, fontTable);
            let b = Self::doLoadCache(cache_file, font_table);
            // Java: return b;
            b
        }
    }

    // Java: static boolean doLoadCache(File cacheFile, IFontTable fontTable)
    pub fn doLoadCache(cache_file: &Path, _font_table: &dyn crate::cn::wps::font::IFontTable::IFontTable) -> bool {
        // Java: String systemFontsDir = Platform.getSystemFontsDirectory();
        let system_fonts_dir = String::new();
        // Java: BufferedReader br = null;
        // Java: try
        let file = match File::open(cache_file) {
            Ok(f) => f,
            Err(_) => {
                // Java: catch (Throwable e)
                // Java: Log.e(TAG, "", e);
                return false;
            }
        };
        // Java: br = new BufferedReader(new FileReader(cacheFile));
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        // Java: String line = br.readLine();
        let first_line = lines.next().and_then(|l| l.ok());
        // Java: if (line == null || !line.startsWith("Version"))
        if first_line.is_none() || !first_line.as_ref().unwrap().starts_with("Version") {
            // Java: boolean var26 = false;
            // Java: return var26;
            return false;
        }
        // Java: else
        {
            // Java: try
            let line = first_line.unwrap();
            // Java: float oldVersion = Float.parseFloat(line.substring("Version".length()));
            let old_version = line["Version".len()..].parse::<f32>().unwrap_or(0.0);
            // Java: if (oldVersion < 5.0F)
            if old_version < 5.0 {
                // Java: boolean var27 = false;
                // Java: return var27;
                return false;
            }
            // Java: catch (Exception var20)
        }
        // Java: while(null != (line = br.readLine()) && line.length() > 0)
        for line_result in lines {
            let line = match line_result {
                Ok(l) => l,
                Err(_) => break,
            };
            if line.is_empty() { break; }
            // Java: String[] names = line.split("\t");
            let names: Vec<&str> = line.split('\t').collect();
            // Java: if (null != names && names.length >= 3)
            if names.len() >= 3 {
                // Java: String fontFullName = names[0];
                let font_full_name = names[0];
                // Java: String familyName = names[1];
                let family_name = names[1];
                // Java: String fontFileName = names[2];
                let font_file_name = names[2];
                // Java: File fontFile = new File(fontFileName);
                let font_file = Path::new(font_file_name);
                // Java: if (fontFile.exists())
                if font_file.exists() {
                    // Java: FontNameDecoder decoder = FontNameDecoder.decode(fontFullName);
                    let _decoder_style = 0i32;
                    // Java: int style = decoder.getStyle();
                    let _style = _decoder_style;
                    // Java: if (familyName != null && familyName.length() != 0 && !familyName.equalsIgnoreCase("null"))
                    if !family_name.is_empty() && !family_name.eq_ignore_ascii_case("null") {
                        // Java: boolean isSystemFont = fontFileName.startsWith(systemFontsDir);
                        let _is_system_font = font_file_name.starts_with(&*system_fonts_dir);
                        // Java: boolean isOnlineFont = fontFileName.contains("KingsoftOffice/.font") && fontFileName.contains("cn.wps.moffice");
                        let _is_online_font = font_file_name.contains("KingsoftOffice/.font") && font_file_name.contains("cn.wps.moffice");
                        // Java: FontFamilyImpl fontFamily = (FontFamilyImpl)((FontTableImpl)fontTable).register(familyName, isSystemFont);
                        // Java: fontFamily.setOnlineFont(isOnlineFont);
                        // Java: fontFamily.addFontFile(style, fontFileName);
                        // Java: if (familyName.startsWith("Cambria") && style != 0)
                        if family_name.starts_with("Cambria") && _style != 0 {
                            // Java: FontFamilyImpl cambriaMath = (FontFamilyImpl)((FontTableImpl)fontTable).register("Cambria Math", isSystemFont);
                            // Java: cambriaMath.addFontFile(style, fontFileName);
                        }
                    }
                }
            }
        }
        // Java: boolean var25 = true;
        // Java: return var25;
        true
        // Java: catch (Throwable e)
        // Java: Log.e(TAG, "", e);
        // Java: return false;
        // Java: finally
        // Java: StreamUtil.close(br);
    }

    // Java: static String saveCustomFont(String customFontName)
    pub fn saveCustomFont(custom_font_name: &str) -> Option<String> {
        // Java: boolean added = false;
        let mut added = false;
        // Java: if (null == Platform.getAssetManager())
        // Java: return null;
        // Java: else
        {
            // Java: String customFontPath = Platform.getCustomFontsDirectory() + customFontName;
            let custom_font_path = custom_font_name.to_string();
            // Java: File file = new File(customFontPath);
            let file = PathBuf::from(&custom_font_path);
            // Java: if (!file.exists() || file.length() <= 0L || needOverwrite)
            if !file.exists() || *get_needOverwrite().lock().unwrap() {
                // Java: added = true;
                added = true;
                // Java: if (file.exists())
                if file.exists() {
                    // Java: file.delete();
                    let _ = fs::remove_file(&file);
                }
                // Java: String[] customFontFileNames = (String[])splitFileMap.get(customFontName);
                let split_map = splitFileMap.get_or_init(|| Mutex::new(HashMap::new()));
                let split_guard = split_map.lock().unwrap();
                let custom_font_file_names = split_guard.get(custom_font_name);
                // Java: if (customFontFileNames != null)
                if let Some(file_names) = custom_font_file_names {
                    // Java: int nCount = customFontFileNames.length;
                    let n_count = file_names.len();
                    // Java: for(int i = 0; i < nCount; ++i)
                    for i in 0..n_count {
                        // Java: String cutFileName = "custom/" + customFontFileNames[i];
                        let _cut_file_name = format!("custom/{}", file_names[i]);
                        // Java: try
                        // Java: FileUtil.writeToFile(customFontPath, Platform.getAssetManager().open(cutFileName), true);
                        // Java: catch (Throwable e)
                        // Java: Log.e(TAG, "", e);
                        let _ = i;
                    }
                }
                // Java: else
                else {
                    // Java: try
                    // Java: FileUtil.copyFileOrg(Platform.getAssetManager().open("custom/" + customFontName), customFontPath);
                    // Java: catch (Throwable e)
                    // Java: Log.e(TAG, "", e);
                }
                drop(split_guard);
            }
            // Java: return added && file.exists() && file.length() > 0L ? customFontPath : null;
            if added && file.exists() {
                Some(custom_font_path)
            } else {
                None
            }
        }
    }

    // Java: static String[] getFonts(IFontTable fontTable)
    pub fn getFonts(_font_table: &dyn crate::cn::wps::font::IFontTable::IFontTable) -> Vec<String> {
        // Java: if (null == fontTable)
        // Java: return null;
        // Java: else
        {
            // Java: IFontFamily[] fontFamilys = ((FontTableImpl)fontTable).getFontFamilys();
            // Java: List<String> allFonts = new ArrayList();
            let mut all_fonts: Vec<String> = Vec::new();
            // Java: for(int i = 0; i < fontFamilys.length; ++i)
            // Java: String name = fontFamilys[i].getName();
            // Java: if (isAllowFont(name) && !StringUtil.isEmpty(name))
            // Java: allFonts.add(name);
            let _ = &mut all_fonts;
            // Java: String[] fonts = new String[allFonts.size()];
            // Java: return (String[])allFonts.toArray(fonts);
            all_fonts
        }
    }

    // Java: static String[] getSystemFonts(IFontTable fontTable)
    pub fn getSystemFonts(_font_table: &dyn crate::cn::wps::font::IFontTable::IFontTable) -> Vec<String> {
        // Java: if (null == fontTable)
        // Java: return null;
        // Java: else
        {
            // Java: List<String> systemFonts = getSystemFontList(fontTable);
            let system_fonts = Self::getSystemFontList(_font_table);
            // Java: String[] fonts = new String[systemFonts.size()];
            // Java: return (String[])systemFonts.toArray(fonts);
            system_fonts
        }
    }

    // Java: static List<String> getSystemFontList(IFontTable fontTable)
    pub fn getSystemFontList(_font_table: &dyn crate::cn::wps::font::IFontTable::IFontTable) -> Vec<String> {
        // Java: if (null == fontTable)
        // Java: return null;
        // Java: else
        {
            // Java: IFontFamily[] fontFamilys = ((FontTableImpl)fontTable).getFontFamilys();
            // Java: List<String> systemFonts = new ArrayList();
            let system_fonts: Vec<String> = Vec::new();
            // Java: for(int i = 0; i < fontFamilys.length; ++i)
            // Java: IFontFamily f = fontFamilys[i];
            // Java: if (f.isSystemFont() && !f.isBuildinHeader() && !f.getName().equals("serif") && isAllowFont(f.getName()) && !StringUtil.isEmpty(f.getName()))
            // Java: systemFonts.add(f.getName());
            // Java: return systemFonts;
            system_fonts
        }
    }

    // Java: static int getSystemFontCount(IFontTable fontTable)
    pub fn getSystemFontCount(_font_table: &dyn crate::cn::wps::font::IFontTable::IFontTable) -> i32 {
        // Java: int count = 0;
        let count = 0;
        // Java: if (null == fontTable)
        // Java: return count;
        // Java: else
        {
            // Java: ArrayList<IFontFamily> fontFamilys = ((FontTableImpl)fontTable).mArray;
            // Java: int i = 1;
            // Java: for(int size = fontFamilys.size(); i < size; ++i)
            // Java: IFontFamily f = (IFontFamily)fontFamilys.get(i);
            // Java: if (f.isSystemFont() && !f.isBuildinHeader() && isAllowFont(f.getName()) && !StringUtil.isEmpty(f.getName()))
            // Java: ++count;
            // Java: return count;
            count
        }
    }

    // Java: static String[] getCustomFonts(IFontTable fontTable)
    pub fn getCustomFonts(_font_table: &dyn crate::cn::wps::font::IFontTable::IFontTable) -> Vec<String> {
        // Java: if (null == fontTable)
        // Java: return null;
        // Java: else
        {
            // Java: List<String> customFonts = getCustomFontList(fontTable);
            let custom_fonts = Self::getCustomFontList(_font_table);
            // Java: String[] fonts = new String[customFonts.size()];
            // Java: return (String[])customFonts.toArray(fonts);
            custom_fonts
        }
    }

    // Java: static List<String> getCustomFontList(IFontTable fontTable)
    pub fn getCustomFontList(_font_table: &dyn crate::cn::wps::font::IFontTable::IFontTable) -> Vec<String> {
        // Java: if (null == fontTable)
        // Java: return null;
        // Java: else
        {
            // Java: IFontFamily[] fontFamilys = ((FontTableImpl)fontTable).getFontFamilys();
            // Java: List<String> customFonts = new ArrayList();
            let custom_fonts: Vec<String> = Vec::new();
            // Java: for(int i = 0; i < fontFamilys.length; ++i)
            // Java: IFontFamily f = fontFamilys[i];
            // Java: if (!f.isSystemFont() && !f.isBuildinHeader() && isAllowFont(f.getName()) && f.fontFileExist() && !StringUtil.isEmpty(f.getName()))
            // Java: customFonts.add(f.getName());
            // Java: return customFonts;
            custom_fonts
        }
    }

    // Java: static List<String> getSDCardFontList(IFontTable fontTable)
    pub fn getSDCardFontList(_font_table: &dyn crate::cn::wps::font::IFontTable::IFontTable) -> Vec<String> {
        // Java: if (null == fontTable)
        // Java: return null;
        // Java: else
        {
            // Java: IFontFamily[] fontFamilys = ((FontTableImpl)fontTable).getFontFamilys();
            // Java: List<String> customFonts = new ArrayList();
            let custom_fonts: Vec<String> = Vec::new();
            // Java: for(int i = 0; i < fontFamilys.length; ++i)
            // Java: IFontFamily f = fontFamilys[i];
            // Java: if (!f.isSystemFont() && isAllowFont(f.getName()) && !f.isOnlineFont() && !f.isBuildinHeader() && f.fontFileExist() && !StringUtil.isEmpty(f.getName()))
            // Java: customFonts.add(f.getName());
            // Java: return customFonts;
            custom_fonts
        }
    }

    // Java: static List<String> getCloudFontList(IFontTable fontTable)
    pub fn getCloudFontList(_font_table: &dyn crate::cn::wps::font::IFontTable::IFontTable) -> Vec<String> {
        // Java: if (null == fontTable)
        // Java: return null;
        // Java: else
        {
            // Java: IFontFamily[] fontFamilys = ((FontTableImpl)fontTable).getFontFamilys();
            // Java: List<String> customFonts = new ArrayList();
            let custom_fonts: Vec<String> = Vec::new();
            // Java: for(int i = 0; i < fontFamilys.length; ++i)
            // Java: IFontFamily f = fontFamilys[i];
            // Java: if (!f.isSystemFont() && isAllowFont(f.getName()) && f.isOnlineFont() && !f.isBuildinHeader() && f.fontFileExist() && !StringUtil.isEmpty(f.getName()))
            // Java: customFonts.add(f.getName());
            // Java: return customFonts;
            custom_fonts
        }
    }

    // Java: static boolean isAllowFont(String fontName)
    pub fn isAllowFont(font_name: &str) -> bool {
        // Java: if (null == fontName)
        if font_name.is_empty() {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: int i = 0;
            let unsport_list = get_unsport().lock().unwrap();
            // Java: for(int size = unsport.length; i < size; ++i)
            for item in unsport_list.iter() {
                // Java: if (unsport[i].equals(fontName))
                if item == font_name {
                    // Java: return false;
                    return false;
                }
            }
            // Java: return !fontName.equals("WPS Special 1") && !fontName.equals("WPS Special 2") && !fontName.equals("WPS Special 3");
            font_name != "WPS Special 1" && font_name != "WPS Special 2" && font_name != "WPS Special 3"
        }
    }

    // Java: static private static <T> void doAddArrayToSet(Set<T> destSet, T[] array)
    fn doAddArrayToSet(dest_set: &mut HashSet<String>, array: &[String]) {
        // Java: int i = 0;
        // Java: for(int n = array.length; i < n; ++i)
        for item in array {
            // Java: destSet.add(array[i]);
            dest_set.insert(item.clone());
        }
    }

    // Java: static void addFontMapScheme(FontHostSettings.SchemaItem[] schemes)
    pub fn addFontMapScheme(_schemes: &[SchemaItem]) {
        // Java: doAddArrayToSet(msFontMappingSchemas, schemes);
    }

    // Java: static void addSelfExtractedFontFileNames(String[] fontFileNames)
    pub fn addSelfExtractedFontFileNames(font_file_names: &[String]) {
        // Java: doAddArrayToSet(msSelfExtractedFontNames, fontFileNames);
        if let Some(set) = msSelfExtractedFontNames.get() {
            let mut guard = set.lock().unwrap();
            Self::doAddArrayToSet(&mut guard, font_file_names);
        }
    }
}
