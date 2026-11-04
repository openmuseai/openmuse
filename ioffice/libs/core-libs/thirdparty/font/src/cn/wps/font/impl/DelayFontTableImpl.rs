// Java: import cn.wps.base.log.Log
// Java: import cn.wps.font.IFontFamily
use crate::cn::wps::font::IFontFamily::IFontFamily;
// Java: import cn.wps.font.IFontTable
use crate::cn::wps::font::IFontTable::IFontTable;
// Java: import cn.wps.moffice.util.StreamUtil
// Java: import java.io.BufferedReader
use std::io::BufRead;
// Java: import java.io.File
use std::path::PathBuf;
// Java: import java.io.FileReader
use std::io::BufReader;
// Java: import java.util.HashSet
use std::collections::HashSet;
// Java: import java.util.Set
use std::sync::Arc;

use super::FontTableImpl::FontTableImpl;

// Java field: String TAG
static TAG: std::sync::OnceLock<String> = std::sync::OnceLock::new();

fn get_tag() -> &'static str {
    TAG.get_or_init(|| "DelayFontTableImpl".to_string())
}

// Java: public class DelayFontTableImpl extends FontTableImpl
pub struct DelayFontTableImpl {
    pub base: FontTableImpl,
    // Java field: boolean mDelaySysFont
    pub(crate) mDelaySysFont: bool,
    // Java field: Set<String> mGetFailedList
    mGetFailedList: HashSet<String>,
}

// Java: public class DelayFontTableImpl extends FontTableImpl
impl DelayFontTableImpl {
    // Java: DelayFontTableImpl(boolean delaySysFont)
    pub fn new(delay_sys_font: bool) -> Self {
        // Java: super();
        let base = FontTableImpl::new_with(false);
        // Java: this.mDelaySysFont = delaySysFont;
        DelayFontTableImpl {
            base,
            mDelaySysFont: delay_sys_font,
            mGetFailedList: HashSet::new(),
        }
    }

    // Java: boolean isDelaySysFont()
    pub fn isDelaySysFont(&self) -> bool {
        // Java: return this.mDelaySysFont;
        self.mDelaySysFont
    }

    // Java: IFontTable loadAndCopyTo()
    pub fn loadAndCopyTo(&self) -> FontTableImpl {
        // Java: FontTableImpl fontTable = new FontTableImpl(false);
        let mut font_table = FontTableImpl::new_with(false);
        // Java: IFontFamily defaut = (IFontFamily)this.mArray.get(0);
        let defaut = self.base.mArray.get(0).cloned();
        // Java: fontTable.mArray.add(defaut);
        if let Some(ref d) = defaut {
            font_table.mArray.push(d.clone());
            // Java: fontTable.mMap.put(defaut.getName(), defaut);
            font_table.mMap.insert(d.getName(), d.clone());
        }
        // Java: int i = 1;
        // Java: for(int size = this.mArray.size(); i < size; ++i)
        for i in 1..self.base.mArray.len() {
            // Java: IFontFamily font = (IFontFamily)this.mArray.get(i);
            let font = self.base.mArray[i].clone();
            // Java: if (font instanceof IFontDelayAdapter)
            // Java: font = ((IFontDelayAdapter)font).validFont();
            // Java: if (font instanceof FontFamilyImpl)
            {
                // Java: int id = this.mArray.size();
                let _id = font_table.mArray.len() as i32;
                // Java: ((FontFamilyImpl)font).setId(id);
                // Java: fontTable.mArray.add(font);
                font_table.mArray.push(font.clone());
                // Java: fontTable.mMap.put(font.getName(), font);
                font_table.mMap.insert(font.getName(), font);
            }
        }
        // Java: return fontTable;
        font_table
    }

    // Java: IFontFamily doGetFromMap(String familyName)
    pub fn doGetFromMap(&self, family_name: &str) -> Option<Arc<dyn IFontFamily>> {
        // Java: IFontFamily fontFamily = (IFontFamily)this.mMap.get(familyName);
        let font_family = self.base.mMap.get(family_name).cloned();
        // Java: if (fontFamily instanceof FontFamilyDealyAdapter)
        // Java: fontFamily = ((FontFamilyDealyAdapter)fontFamily).validFont();
        // Java: return fontFamily;
        font_family
    }

    // Java: IFontFamily get(String familyName, boolean defaultIfNotExist, boolean replaceIfNotExist)
    pub fn get_by_name_bool_bool_impl(&mut self, family_name: &str, default_if_not_exist: bool, replace_if_not_exist: bool) -> Option<Arc<dyn IFontFamily>> {
        // Java: IFontFamily fontFamily = super.get(familyName, defaultIfNotExist, replaceIfNotExist);
        let font_family = self.base.get_by_name_bool_bool_impl(family_name, default_if_not_exist, replace_if_not_exist);
        // Java: if (fontFamily == null)
        if font_family.is_none() {
            // Java: this.mGetFailedList.add(familyName);
            self.mGetFailedList.insert(family_name.to_string());
        }
        // Java: return fontFamily;
        font_family
    }

    // Java: boolean checkDelaySysFonts(IFontTable fontTable)
    pub fn checkDelaySysFonts(&self, font_table: &dyn IFontTable) -> bool {
        // Java: for(String fontName : this.mGetFailedList)
        for font_name in &self.mGetFailedList {
            // Java: IFontFamily fontFamily = fontTable.get(fontName);
            let font_family = font_table.get_by_name(font_name);
            // Java: if (fontFamily != null && fontFamily.isSystemFont() && !fontFamily.isBuildinHeader())
            if let Some(ff) = font_family {
                if ff.isSystemFont() && !ff.isBuildinHeader() {
                    // Java: return true;
                    return true;
                }
            }
        }
        // Java: return true;
        true
    }

    // Java: boolean init(String fontCacheFile, DelayFontTableImpl fontTable)
    pub fn init(&mut self, font_cache_file: &str, _font_table: &mut DelayFontTableImpl) -> bool {
        // Java: File cacheFile = new File(fontCacheFile);
        let cache_file = PathBuf::from(font_cache_file);
        // Java: return !cacheFile.exists() ? false : this.doLoadCache(cacheFile, fontTable);
        if !cache_file.exists() {
            return false;
        }
        self.doLoadCache(&cache_file)
    }

    // Java: boolean doLoadCache(File cacheFile, DelayFontTableImpl fontTable)
    fn doLoadCache(&mut self, cache_file: &PathBuf) -> bool {
        // Java: BufferedReader br = null;
        // Java: try
        let result = (|| -> Result<bool, std::io::Error> {
            // Java: br = new BufferedReader(new FileReader(cacheFile));
            let file = std::fs::File::open(cache_file)?;
            let reader = BufReader::new(file);
            let mut lines_iter = reader.lines();
            // Java: String line = br.readLine();
            let line = match lines_iter.next() {
                Some(Ok(l)) => l,
                Some(Err(e)) => return Err(e),
                None => return Ok(false),
            };
            // Java: if (line == null || !line.startsWith("Version"))
            if !line.starts_with("Version") {
                // Java: boolean var21 = false;
                // Java: return var21;
                return Ok(false);
            }
            // Java: else
            {
                // Java: try
                {
                    // Java: float oldVersion = Float.parseFloat(line.substring("Version".length()));
                    let version_str = &line["Version".len()..];
                    if let Ok(old_version) = version_str.parse::<f32>() {
                        // Java: if (oldVersion < 5.0F)
                        if old_version < 5.0 {
                            // Java: boolean var23 = false;
                            // Java: return var23;
                            return Ok(false);
                        }
                    }
                }
                // Java: catch (Exception var16)

                // Java: FontFamilyDealyAdapter fontFamily = null;
                let mut current_family_name: Option<String> = None;
                // Java: while(null != (line = br.readLine()))
                for line_result in lines_iter {
                    let line = match line_result {
                        Ok(l) => l,
                        Err(_) => break,
                    };
                    // Java: int len = line.length();
                    let len = line.len();
                    // Java: if (len > 0)
                    if len > 0 {
                        // Java: int start = line.indexOf(9) + 1;
                        let tab_pos = line.find('\t');
                        let start = tab_pos.map_or(0, |p| p + 1);
                        // Java: if (start > 1)
                        if start > 1 {
                            // Java: int end = line.indexOf(9, start);
                            let end = line[start..].find('\t').map(|p| p + start);
                            // Java: if (end > start && end < len - 2)
                            if let Some(end_pos) = end {
                                if end_pos > start && end_pos < len - 2 {
                                    // Java: String familyName = line.substring(start, end);
                                    let family_name = line[start..end_pos].to_string();
                                    // Java: if (fontFamily == null || !familyName.equals(fontFamily.getName()))
                                    let need_register = match &current_family_name {
                                        None => true,
                                        Some(name) => family_name != *name,
                                    };
                                    if need_register {
                                        // Java: fontFamily = (FontFamilyDealyAdapter)this.registerDelayFont(familyName);
                                        self.registerDelayFont(&family_name);
                                        current_family_name = Some(family_name);
                                    }
                                    // Java: fontFamily.addLine(line);
                                }
                            }
                        }
                    }
                }
                // Java: FontFamilyDealyAdapter cambriaFont = (FontFamilyDealyAdapter)this.mMap.get("Cambria");
                let _cambria_font = self.base.mMap.get("Cambria").cloned();
                // Java: if (cambriaFont != null)
                if let Some(_cambria) = _cambria_font {
                    // Java: IFontFamily sourceCambria = cambriaFont.validFont();
                    // Java: if (sourceCambria != null)
                    {
                        // Java: boolean isSystemFont = sourceCambria.isSystemFont();
                        // Java: for(int style = 1; style < 4; ++style)
                        for _style in 1..4i32 {
                            // Java: String file = cambriaFont.getFontFile(style);
                            // Java: if (file != null)
                            {
                                // Java: IFontFamily mathFont = ((FontFamilyDealyAdapter)this.registerDelayFont("Cambria Math")).fourceEmptySourceFont(isSystemFont);
                                self.registerDelayFont("Cambria Math");
                                // Java: mathFont.addFontFile(style, file);
                            }
                        }
                    }
                }
                // Java: boolean var25 = true;
                // Java: return var25;
                Ok(true)
            }
        })();
        match result {
            Ok(v) => v,
            // Java: catch (Throwable e)
            Err(_e) => {
                // Java: Log.e(TAG, "", e);
                eprintln!("{}: {}", get_tag(), _e);
                // Java: return false;
                false
            }
        }
        // Java: finally
        // Java: StreamUtil.close(br);
    }

    // Java: IFontFamily registerDelayFont(String familyName)
    fn registerDelayFont(&mut self, family_name: &str) -> Option<Arc<dyn IFontFamily>> {
        // Java: IFontFamily family = (IFontFamily)this.mMap.get(familyName);
        let family = self.base.mMap.get(family_name).cloned();
        // Java: if (family != null)
        if family.is_some() {
            // Java: return family;
            return family;
        }
        // Java: else
        {
            // Java: int id = this.mArray.size();
            let _id = self.base.mArray.len() as i32;
            // Java: family = new FontFamilyDealyAdapter(familyName, id);
            // Java: this.mArray.add(family);
            // Java: this.mMap.put(familyName, family);
            // Java: return family;
            None
        }
    }
}
