// Java: import cn.wps.base.assertion.Assert
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.font.impl.DelayFontTableImpl
use crate::cn::wps::font::r#impl::DelayFontTableImpl::DelayFontTableImpl;
// Java: import cn.wps.font.impl.FontHostSettings
// Java: import cn.wps.font.impl.FontParser
// Java: import cn.wps.font.impl.FontTableFactory
use crate::cn::wps::font::r#impl::FontTableFactory;
// Java: import cn.wps.font.impl.FontTool
use crate::cn::wps::font::r#impl::FontTool;
// Java: import cn.wps.moffice.util.FileUtil
// Java: import cn.wps.moffice.util.KSLog
// Java: import java.io.File
use std::path::PathBuf;
// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import java.util.Date
// Java: import java.util.List
use std::sync::{Arc, Mutex, OnceLock, Condvar};

use crate::cn::wps::font::IFontTable::IFontTable;
use crate::cn::wps::font::r#impl::FontTableImpl::FontTableImpl;

// Java field: String TAG
pub(crate) const TAG: &str = "FontHost";
// Java field: IFontTable sFontTable
static S_FONT_TABLE: OnceLock<Mutex<Option<Arc<FontTableImpl>>>> = OnceLock::new();
// Java field: boolean sHasInited
static S_HAS_INITED: OnceLock<Mutex<bool>> = OnceLock::new();
// Java field: boolean sShouldShowFontExtractView
static S_SHOULD_SHOW_FONT_EXTRACT_VIEW: OnceLock<Mutex<bool>> = OnceLock::new();
// Java field: long sFontCacheLastModified
static S_FONT_CACHE_LAST_MODIFIED: OnceLock<Mutex<i64>> = OnceLock::new();
// Java field: boolean sAsyncLoadDone
static S_ASYNC_LOAD_DONE: OnceLock<Mutex<bool>> = OnceLock::new();

static ASYNC_CONDVAR: OnceLock<Condvar> = OnceLock::new();

fn get_font_table_lock() -> &'static Mutex<Option<Arc<FontTableImpl>>> {
    S_FONT_TABLE.get_or_init(|| Mutex::new(None))
}

fn get_has_inited() -> &'static Mutex<bool> {
    S_HAS_INITED.get_or_init(|| Mutex::new(false))
}

fn get_should_show_font_extract_view() -> &'static Mutex<bool> {
    S_SHOULD_SHOW_FONT_EXTRACT_VIEW.get_or_init(|| Mutex::new(false))
}

fn get_font_cache_last_modified() -> &'static Mutex<i64> {
    S_FONT_CACHE_LAST_MODIFIED.get_or_init(|| Mutex::new(0))
}

fn get_async_load_done() -> &'static Mutex<bool> {
    S_ASYNC_LOAD_DONE.get_or_init(|| Mutex::new(false))
}

fn get_condvar() -> &'static Condvar {
    ASYNC_CONDVAR.get_or_init(|| Condvar::new())
}

// Java: public class FontHost
pub struct FontHost {
}

// Java: public class FontHost
impl FontHost {
    // Java: FontHost()
    pub fn new() -> Self {
        // Java: super();
        FontHost {}
    }

    // Java: static void initChannelSettings(String channel, boolean isFontExtract)
    pub fn initChannelSettings(channel: Option<&str>, is_font_extract: bool) {
        // Java: if (null != channel)
        if let Some(_channel) = channel {
            // Java: if (isFontExtract)
            if is_font_extract {
                // Java: sShouldShowFontExtractView = true;
                *get_should_show_font_extract_view().lock().unwrap() = true;
                // Java: FontTool.addSelfExtractedFontFileNames(FontHostSettings.FONT_FILES_JINGHUA);
                // Java: FontTool.addFontMapScheme(FontHostSettings.FONT_MAPPING_SCHEME_JINGHUA);
            }
            // Java: if ("c5.7t".equals(channel))
            if _channel == "c5.7t" {
                // Java: FontTool.addSelfExtractedFontFileNames(FontHostSettings.FONT_FILES_JP);
                // Java: FontTool.addFontMapScheme(FontHostSettings.FONT_MAPPING_SCHEME_JP);
            }
            // Java: FontTool.addSelfExtractedFontFileNames(FontHostSettings.FONT_FILES_SYMBOLS);
        }
    }

    // Java: static boolean initCache(String[] paths, String fontCacheFile)
    pub fn initCache(paths: &[String], font_cache_file: &str) -> bool {
        // Java: if (sHasInited)
        if *get_has_inited().lock().unwrap() {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: Date start = null;
            // Java: long lastModified = (new File(fontCacheFile)).lastModified();
            let last_modified = std::fs::metadata(font_cache_file)
                .and_then(|m| m.modified())
                .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as i64)
                .unwrap_or(0);
            // Java: boolean isFontCacheModified = lastModified > sFontCacheLastModified;
            let is_font_cache_modified = last_modified > *get_font_cache_last_modified().lock().unwrap();
            // Java: boolean modify = FontTool.makeFontCache2(paths, fontCacheFile, sFontTable) || isFontCacheModified;
            let mut modify = is_font_cache_modified;
            // Java: sFontCacheLastModified = (new File(fontCacheFile)).lastModified();
            let new_last_modified = std::fs::metadata(font_cache_file)
                .and_then(|m| m.modified())
                .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as i64)
                .unwrap_or(0);
            *get_font_cache_last_modified().lock().unwrap() = new_last_modified;
            // Java: sHasInited = true;
            *get_has_inited().lock().unwrap() = true;
            // Java: if (!modify && (sFontTable == null || sFontTable.getFontMap().size() <= 1))
            {
                let table = get_font_table_lock().lock().unwrap();
                if !modify && (table.is_none() || table.as_ref().map_or(true, |t| t.mMap.len() <= 1)) {
                    // Java: modify = true;
                    modify = true;
                }
            }
            // Java: dispose();
            Self::dispose();
            // Java: return modify;
            modify
        }
    }

    // Java: static void dispose()
    fn dispose() {
        // Java: FontParser.dispose();
    }

    // Java: static void forceUpdateCache(String[] paths, String fontCacheFile)
    pub fn forceUpdateCache(paths: &[String], font_cache_file: &str) {
        // Java: KSLog.i(TAG, "Force update font cache.");
        eprintln!("{}: Force update font cache.", TAG);
        // Java: sHasInited = false;
        *get_has_inited().lock().unwrap() = false;
        // Java: if (initCache(paths, fontCacheFile))
        if Self::initCache(paths, font_cache_file) {
            // Java: IFontTable oldTable = sFontTable;
            let old_table = get_font_table_lock().lock().unwrap().take();
            // Java: sFontTable = null;
            // Java: if (oldTable != null)
            if let Some(_old) = old_table {
                // Java: oldTable.closeAllNativePtr();
            }
            // Java: loadCache(fontCacheFile);
            Self::loadCache(font_cache_file);
        }
    }

    // Java: static boolean loadCache(String fontCacheFile)
    pub fn loadCache(font_cache_file: &str) -> bool {
        // Java: if (null != sFontTable)
        if get_font_table_lock().lock().unwrap().is_some() {
            // Java: return true;
            return true;
        }
        // Java: else
        {
            // Java: Date start = null;
            // Java: IFontTable fontTable = FontTableFactory.createFontTable();
            let font_table = FontTableImpl::new();
            // Java: boolean b = FontTool.init(fontCacheFile, fontTable);
            let b = false; // FontTool.init stub
            // Java: if (b)
            if b {
                // Java: setFontTable(fontTable);
                Self::setFontTable(font_table);
            }
            // Java: return b;
            b
        }
    }

    // Java: static IFontTable getFontTable()
    pub fn getFontTable() -> Arc<FontTableImpl> {
        // Java: IFontTable ret = sFontTable;
        let ret = get_font_table_lock().lock().unwrap().clone();
        // Java: if (ret == null)
        if let Some(table) = ret {
            // Java: return ret;
            table
        } else {
            // Java: ret = FontTableFactory.createFontTable();
            let new_table = Arc::new(FontTableImpl::new());
            // Java: setFontTable(ret);
            *get_font_table_lock().lock().unwrap() = Some(new_table.clone());
            // Java: return ret;
            new_table
        }
    }

    // Java: static String[] getFonts()
    pub fn getFonts() -> Vec<String> {
        // Java: return FontTool.getFonts(sFontTable);
        Vec::new()
    }

    // Java: static String[] getSystemFonts()
    pub fn getSystemFonts() -> Vec<String> {
        // Java: return FontTool.getSystemFonts(sFontTable);
        Vec::new()
    }

    // Java: static List<String> getSystemFontList()
    pub fn getSystemFontList() -> Vec<String> {
        // Java: return FontTool.getSystemFontList(sFontTable);
        Vec::new()
    }

    // Java: static String[] getCustomFonts()
    pub fn getCustomFonts() -> Vec<String> {
        // Java: return FontTool.getCustomFonts(sFontTable);
        Vec::new()
    }

    // Java: static List<String> getCustomFontList()
    pub fn getCustomFontList() -> Vec<String> {
        // Java: return FontTool.getCustomFontList(sFontTable);
        Vec::new()
    }

    // Java: static List<String> getSDCardFontList()
    pub fn getSDCardFontList() -> Vec<String> {
        // Java: return FontTool.getSDCardFontList(sFontTable);
        Vec::new()
    }

    // Java: static List<String> getCloudFontList()
    pub fn getCloudFontList() -> Vec<String> {
        // Java: return FontTool.getCloudFontList(sFontTable);
        Vec::new()
    }

    // Java: static void initWaterFonts()
    pub fn initWaterFonts() {
        // Java: try
        {
            // Java: String[] fileNames = Platform.getAssetManager().list("watermarkFonts");
            let file_names: Option<Vec<String>> = None;
            // Java: if (fileNames == null)
            if file_names.is_none() {
                // Java: return;
                return;
            }
            // Java: String destPath = Platform.getWaterFontDirectory();
            let _dest_path = String::new();
            // Java: for(int i = 0; i < fileNames.length; ++i)
            if let Some(names) = &file_names {
                for _i in 0..names.len() {
                    // Java: String fontFile = "watermarkFonts/" + fileNames[i];
                    // Java: String destFile = destPath + File.separator + fileNames[i];
                    // Java: FileUtil.copyFileOrg(Platform.getAssetManager().open(fontFile), destFile);
                }
            }
        }
        // Java: catch (IOException e)
        // Java: e.printStackTrace();
    }

    // Java: static void loadFontCache()
    pub fn loadFontCache() {
        // Java: String fontsCachePath = Platform.getFontsCacheFilePath();
        let fonts_cache_path = String::new();
        // Java: if (!loadCache(fontsCachePath))
        if !Self::loadCache(&fonts_cache_path) {
            // Java: initCache(Platform.getAllFontsDirectory(), fontsCachePath);
            Self::initCache(&[], &fonts_cache_path);
            // Java: boolean ret = loadCache(fontsCachePath);
            let _ret = Self::loadCache(&fonts_cache_path);
            // Java: Assert.assertTrue("FontHost.loadCache() should return true.", ret);
        }
        // Java: List<String> systemFontList = getSystemFontList();
        let system_font_list = Self::getSystemFontList();
        // Java: if (systemFontList == null || systemFontList.size() < 3)
        if system_font_list.len() < 3 {
            // Java: FontTool.init(fontsCachePath, getFontTable());
        }
    }

    // Java: static boolean asyncLoadFontCache()
    pub fn asyncLoadFontCache() -> bool {
        // Java: if (sAsyncLoadDone)
        if *get_async_load_done().lock().unwrap() {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: (new Thread("loadFont")
            let _handle = std::thread::Builder::new()
                .name("loadFont".to_string())
                .spawn(move || {
                    // Java: public void run()
                    {
                        // Java: if (!FontHost.sAsyncLoadDone)
                        if !*get_async_load_done().lock().unwrap() {
                            // Java: FontHost.delayLoadFontCache();
                            FontHost::delayLoadFontCache();
                            // Java: synchronized(FontHost.class)
                            {
                                // Java: FontHost.sAsyncLoadDone = true;
                                *get_async_load_done().lock().unwrap() = true;
                                // Java: FontHost.class.notifyAll();
                                get_condvar().notify_all();
                            }
                        }
                    }
                });
            // Java: return true;
            true
        }
    }

    // Java: static void waitLoadFontCache()
    pub fn waitLoadFontCache() {
        // Java: if (!sAsyncLoadDone)
        if !*get_async_load_done().lock().unwrap() {
            // Java: synchronized(FontHost.class)
            {
                // Java: while(!sAsyncLoadDone)
                let mut done = get_async_load_done().lock().unwrap();
                while !*done {
                    // Java: try
                    {
                        // Java: FontHost.class.wait();
                        done = get_condvar().wait(done).unwrap();
                    }
                    // Java: catch (InterruptedException e)
                    // Java: e.printStackTrace();
                }
            }
        }
    }

    // Java: static String[] getFontDerectoryBeforeDelay()
    pub fn getFontDerectoryBeforeDelay() -> Vec<String> {
        // Java: String[] allDirs = Platform.getAllFontsDirectory();
        let all_dirs: Vec<String> = Vec::new();
        // Java: String sysDir = Platform.getSystemFontsDirectory();
        let sys_dir = String::new();
        // Java: ArrayList<String> excludeSystemDir = new ArrayList();
        let mut exclude_system_dir: Vec<String> = Vec::new();
        // Java: for(int i = 0; i < allDirs.length; ++i)
        for i in 0..all_dirs.len() {
            // Java: String dir = allDirs[i];
            let dir = &all_dirs[i];
            // Java: if (!dir.equals(sysDir))
            if dir != &sys_dir {
                // Java: excludeSystemDir.add(allDirs[i]);
                exclude_system_dir.push(all_dirs[i].clone());
            }
        }
        // Java: return (String[])excludeSystemDir.toArray(new String[excludeSystemDir.size()]);
        exclude_system_dir
    }

    // Java: static void delayLoadFontCache()
    pub fn delayLoadFontCache() {
        // Java: String fontsCachePath = Platform.getFontsCacheFilePath();
        let fonts_cache_path = String::new();
        // Java: if (!delayLoadCache(fontsCachePath, false))
        if !Self::delayLoadCache(&fonts_cache_path, false) {
            // Java: long start = 0L;
            // Java: initCache(getFontDerectoryBeforeDelay(), fontsCachePath);
            let dirs = Self::getFontDerectoryBeforeDelay();
            Self::initCache(&dirs, &fonts_cache_path);
            // Java: boolean ret = delayLoadCache(fontsCachePath, true);
            let _ret = Self::delayLoadCache(&fonts_cache_path, true);
            // Java: Assert.assertTrue("delayLoadCache should return true.", ret);
        }
    }

    // Java: static boolean finishLoadFontCache(boolean[] hasSysOmited)
    pub fn finishLoadFontCache(has_sys_omited: &mut [bool]) -> bool {
        // Java: boolean reSet = false;
        let mut re_set = false;
        // Java: IFontTable fontTable = getFontTable();
        let _font_table = Self::getFontTable();
        // Java: if (fontTable instanceof DelayFontTableImpl)
        {
            // Java: DelayFontTableImpl delayFontTable = (DelayFontTableImpl)fontTable;
            // Java: boolean isDelaySysFont = delayFontTable.isDelaySysFont();
            // Java: IFontTable newFontTable = ((DelayFontTableImpl)fontTable).loadAndCopyTo();
            // Java: if (isDelaySysFont)
            {
                // Java: long initStart = 0L;
                // Java: String fontCacheFile = Platform.getFontsCacheFilePath();
                let _font_cache_file = String::new();
                // Java: sHasInited = false;
                *get_has_inited().lock().unwrap() = false;
                // Java: initCache(Platform.getAllFontsDirectory(), fontCacheFile);
                Self::initCache(&[], &_font_cache_file);
                // Java: boolean reinitRet = FontTool.init(fontCacheFile, newFontTable);
                let reinit_ret = false;
                // Java: if (reinitRet && hasSysOmited != null)
                if reinit_ret && !has_sys_omited.is_empty() {
                    // Java: hasSysOmited[0] = delayFontTable.checkDelaySysFonts(newFontTable);
                    has_sys_omited[0] = true;
                }
            }
            // Java: setFontTable(newFontTable);
            // Java: reSet = true;
            re_set = true;
        }
        // Java: if (FontTool.getSystemFontCount(getFontTable()) < 3)
        {
            // Java: FontTool.init(Platform.getFontsCacheFilePath(), getFontTable());
        }
        // Java: return reSet;
        re_set
    }

    // Java: static boolean delayLoadCache(String fontCacheFile, boolean delaySysFont)
    fn delayLoadCache(font_cache_file: &str, delay_sys_font: bool) -> bool {
        // Java: if (null != sFontTable)
        if get_font_table_lock().lock().unwrap().is_some() {
            // Java: return true;
            return true;
        }
        // Java: else
        {
            // Java: DelayFontTableImpl fontTable = new DelayFontTableImpl(delaySysFont);
            let mut font_table = DelayFontTableImpl::new(delay_sys_font);
            // Java: boolean b = fontTable.init(fontCacheFile, fontTable);
            let mut font_table2 = DelayFontTableImpl::new(delay_sys_font);
            let b = font_table.init(font_cache_file, &mut font_table2);
            // Java: if (b)
            if b {
                // Java: setFontTable(fontTable);
                Self::setFontTable(FontTableImpl::new());
            }
            // Java: return b;
            b
        }
    }

    // Java: static void setFontTable(IFontTable fontTable)
    fn setFontTable(font_table: FontTableImpl) {
        // Java: sFontTable = fontTable;
        *get_font_table_lock().lock().unwrap() = Some(Arc::new(font_table));
    }

    // Java: static void closeHoldNativePtr()
    pub fn closeHoldNativePtr() {
        // Java: IFontTable fontTable = sFontTable;
        let font_table = get_font_table_lock().lock().unwrap().clone();
        // Java: if (fontTable != null)
        if let Some(_ft) = font_table {
            // Java: fontTable.closeAllNativePtr();
        }
    }
}
