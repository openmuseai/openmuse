#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};
use std::ops::Deref;

// Java: import cn.wps.core.runtime.Platform
// use util_adapter::core::runtime::Platform;
// Java: import cn.wps.drawing.app.KmoProtocol
use crate::cn::wps::drawing::app::KmoProtocol::KmoProtocol;
// Java: import cn.wps.drawing.data.fill.Fill
use crate::cn::wps::drawing::data::fill::Fill::Fill;
// Java: import cn.wps.drawing.data.outline.Outline
use crate::cn::wps::drawing::data::outline::Outline::Outline;
// Java: import cn.wps.drawing.data.style.ColorMapping
use crate::cn::wps::drawing::data::style::ColorMapping::ColorMapping;
// Java: import cn.wps.drawing.data.style.ColorScheme
use crate::cn::wps::drawing::data::style::ColorScheme::ColorScheme;
// Java: import cn.wps.drawing.data.style.EffectStyle
use crate::cn::wps::drawing::data::style::EffectStyle::EffectStyle;
// Java: import cn.wps.drawing.data.style.FontCollection
use crate::cn::wps::drawing::data::style::FontCollection::{FontCollection, SupplementalFont};
// Java: import cn.wps.drawing.data.style.FontScheme
use crate::cn::wps::drawing::data::style::FontScheme::FontScheme;
// Java: import cn.wps.drawing.data.style.StyleMatrix
use crate::cn::wps::drawing::data::style::StyleMatrix::StyleMatrix;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::{KoBin as KoBinTrait, Factory, Null};
// Java: import cn.wps.memor4j.KmoAtom
// use kfc::cn::wps::memor4j::KmoAtom::KmoAtom;
// Java: import cn.wps.moffice.util.FileUtil
// use util_adapter::cn::wps::moffice::util::FileUtil;
// Java: import java.util.ArrayList
// LLM Rust No Need: using Vec instead
// Java: import java.util.HashMap
// LLM Rust No Need: using std::collections::HashMap
// Java: import java.util.Map
// LLM Rust No Need: using HashMap
// Java: import java.util.concurrent.atomic.AtomicInteger
// LLM Rust No Need: using std::sync::atomic::AtomicI32

// Java field: AtomicInteger curId
static CUR_ID: AtomicI32 = AtomicI32::new(0);

// Java: public interface ThemePathUpdater
pub struct ThemePathUpdater;

pub trait ThemePathUpdaterTrait {
    // Java: abstract void update(KmoTheme var1)
    fn update(&self, theme: &mut KmoTheme);
}

// Java: public class KmoTheme extends KmoAtom implements KmoProtocol
pub struct KmoTheme {
    // parent: KmoAtom composition for extends
    pub(crate) parent: KmoAtom,
    // Java field: ArrayList<Fill> mFills
    mFills: Option<Vec<Fill>>,
    // Java field: ArrayList<Fill> mBackgroundFills
    mBackgroundFills: Option<Vec<Fill>>,
    // Java field: ArrayList<Outline> mOutlines
    pub(crate) mOutlines: Option<Vec<Outline>>,
    // Java field: ArrayList<EffectStyle> effectStyleLst
    pub(crate) effectStyleLst: Option<Vec<EffectStyle>>,
    // Java field: ArrayList<FontCollection.SupplementalFont> minorFonts
    minorFonts: Option<Vec<SupplementalFont>>,
    // Java field: ArrayList<FontCollection.SupplementalFont> majorFonts
    majorFonts: Option<Vec<SupplementalFont>>,
    // Java field: ArrayList<String> minorFontScript
    minorFontScript: Option<Vec<String>>,
    // Java field: ArrayList<String> majorFontScript
    majorFontScript: Option<Vec<String>>,
    // Java field: Map<String, String> minorFontTypeface
    minorFontTypeface: Option<HashMap<String, String>>,
    // Java field: Map<String, String> majorFontTypeface
    majorFontTypeface: Option<HashMap<String, String>>,
    // Java field: String mThemePath
    pub(crate) mThemePath: Option<String>,
    // Java field: Map<String, Integer> mRid2MediaId
    pub(crate) mRid2MediaId: Option<HashMap<String, i32>>,
    // Java field: ThemePathUpdater mThemePathUpdater
    mThemePathUpdater: Option<Box<dyn ThemePathUpdaterTrait>>,
    // Java field: boolean mThemePathIsOld
    mThemePathIsOld: bool,
    // Java field: int mId
    pub(crate) mId: i32,
}

impl Deref for KmoTheme {
    type Target = KmoAtom;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

// Java: public class IdGenerator
pub struct IdGenerator;

// Java: public class KmoTheme extends KmoAtom implements KmoProtocol
impl KmoTheme {
    // Java: KmoTheme()
    pub fn new() -> Self {
        // Java: super(new ThemeAtomData());
        let parent = KmoAtom::new_with_data(ThemeAtomData::new());
        KmoTheme {
            parent,
            mFills: None,
            mBackgroundFills: None,
            mOutlines: None,
            effectStyleLst: None,
            minorFonts: None,
            majorFonts: None,
            minorFontScript: None,
            majorFontScript: None,
            minorFontTypeface: None,
            majorFontTypeface: None,
            mThemePath: None,
            mRid2MediaId: None,
            mThemePathUpdater: None,
            mThemePathIsOld: false,
            mId: IdGenerator::createId(),
        }
    }

    // Java: KoBin protocol()
    pub fn protocol(&self) -> Box<dyn KoBinTrait> {
        // Java: return ((ThemeAtomData)this.rawData()).theme.save();
        self.parent.rawData().as_theme_atom_data().theme.save()
    }

    // Java: void updateProtocol(KoBin data)
    pub fn updateProtocol(&mut self, data: Box<dyn KoBinTrait>) {
        // Java: ((ThemeAtomData)this.rawData()).theme.load(data);
        self.parent.rawData_mut().as_theme_atom_data_mut().theme.load(data.as_ref());
        // Java: if (((ThemeAtomData)this.rawData()).theme.hasFormatScheme())
        if self.parent.rawData().as_theme_atom_data().theme.hasFormatScheme() {
            // Java: StyleMatrix format = ((ThemeAtomData)this.rawData()).theme.formatScheme();
            let format = self.parent.rawData().as_theme_atom_data().theme.formatScheme();
            // Java: if (format.hasFillLst())
            if format.hasFillLst() {
                // Java: this.mFills = new ArrayList();
                self.mFills = Some(Vec::new());
                // Java: format.fillLst().read(this.mFills);
                format.fillLst().read(self.mFills.as_mut().unwrap());
            }
            // Java: if (format.hasBgFillLst())
            if format.hasBgFillLst() {
                // Java: this.mBackgroundFills = new ArrayList();
                self.mBackgroundFills = Some(Vec::new());
                // Java: format.bgFillLst().read(this.mBackgroundFills);
                format.bgFillLst().read(self.mBackgroundFills.as_mut().unwrap());
            }
            // Java: if (format.hasOutlineLst())
            if format.hasOutlineLst() {
                // Java: this.mOutlines = new ArrayList();
                self.mOutlines = Some(Vec::new());
                // Java: format.outlineLst().read(this.mOutlines);
                format.outlineLst().read(self.mOutlines.as_mut().unwrap());
            }
            // Java: if (format.hasEffectLst())
            if format.hasEffectLst() {
                // Java: this.effectStyleLst = new ArrayList();
                self.effectStyleLst = Some(Vec::new());
                // Java: format.effectLst().read(this.effectStyleLst);
                format.effectLst().read(self.effectStyleLst.as_mut().unwrap());
            }
            // Java: FontScheme fontScheme = this.getFontScheme();
            let fontScheme = self.getFontScheme();
            // Java: if (fontScheme.hasMinor() && fontScheme.minor().hasExtLst())
            if fontScheme.hasMinor() && fontScheme.minor().hasExtLst() {
                // Java: this.minorFonts = new ArrayList();
                self.minorFonts = Some(Vec::new());
                // Java: fontScheme.minor().extLst().read(this.minorFonts);
                fontScheme.minor().extLst().read(self.minorFonts.as_mut().unwrap());
                // Java: this.minorFontScript = new ArrayList();
                self.minorFontScript = Some(Vec::new());
                // Java: this.minorFontTypeface = new HashMap();
                self.minorFontTypeface = Some(HashMap::new());
                // Java: for(int i = 0; i < this.minorFonts.size(); ++i)
                for i in 0..self.minorFonts.as_ref().unwrap().len() {
                    // Java: FontCollection.SupplementalFont minorFont = (FontCollection.SupplementalFont)this.minorFonts.get(i);
                    let minorFont = &self.minorFonts.as_ref().unwrap()[i];
                    // Java: String script = minorFont.script();
                    let script = minorFont.script();
                    // Java: this.minorFontScript.add(script);
                    self.minorFontScript.as_mut().unwrap().push(script.clone());
                    // Java: this.minorFontTypeface.put(script, minorFont.typeface());
                    self.minorFontTypeface.as_mut().unwrap().insert(script, minorFont.typeface());
                }
            }
            // Java: if (fontScheme.hasMajor() && fontScheme.major().hasExtLst())
            if fontScheme.hasMajor() && fontScheme.major().hasExtLst() {
                // Java: this.majorFonts = new ArrayList();
                self.majorFonts = Some(Vec::new());
                // Java: fontScheme.major().extLst().read(this.majorFonts);
                fontScheme.major().extLst().read(self.majorFonts.as_mut().unwrap());
                // Java: this.majorFontScript = new ArrayList();
                self.majorFontScript = Some(Vec::new());
                // Java: this.majorFontTypeface = new HashMap();
                self.majorFontTypeface = Some(HashMap::new());
                // Java: for(int i = 0; i < this.majorFonts.size(); ++i)
                for i in 0..self.majorFonts.as_ref().unwrap().len() {
                    // Java: FontCollection.SupplementalFont majorFont = (FontCollection.SupplementalFont)this.majorFonts.get(i);
                    let majorFont = &self.majorFonts.as_ref().unwrap()[i];
                    // Java: String script = majorFont.script();
                    let script = majorFont.script();
                    // Java: this.majorFontScript.add(script);
                    self.majorFontScript.as_mut().unwrap().push(script.clone());
                    // Java: this.majorFontTypeface.put(script, majorFont.typeface());
                    self.majorFontTypeface.as_mut().unwrap().insert(script, majorFont.typeface());
                }
            }
        }
    }

    // Java: FontScheme getFontScheme()
    pub fn getFontScheme(&self) -> FontScheme {
        // Java: return ((ThemeAtomData)this.rawData()).theme.fontScheme();
        self.parent.rawData().as_theme_atom_data().theme.fontScheme()
    }

    // Java: ColorScheme getColorScheme()
    pub fn getColorScheme(&self) -> ColorScheme {
        // Java: return ((ThemeAtomData)this.rawData()).theme.colorScheme();
        self.parent.rawData().as_theme_atom_data().theme.colorScheme()
    }

    // Java: ColorMapping getColorMapping()
    pub fn getColorMapping(&self) -> ColorMapping {
        // Java: return ((ThemeAtomData)this.rawData()).theme.colorMapping();
        self.parent.rawData().as_theme_atom_data().theme.colorMapping()
    }

    // Java: Fill fillAt(int index)
    pub fn fillAt(&self, mut index: i32) -> Option<&Fill> {
        // Java: if (index != 0 && index != 1000)
        if index != 0 && index != 1000 {
            // Java: ArrayList<Fill> fills;
            let fills: &Option<Vec<Fill>>;
            // Java: if (index >= 1001)
            if index >= 1001 {
                // Java: fills = this.mBackgroundFills;
                fills = &self.mBackgroundFills;
                // Java: index -= 1001;
                index -= 1001;
            // Java: else
            } else {
                // Java: fills = this.mFills;
                fills = &self.mFills;
                // Java: --index;
                index -= 1;
            }
            // Java: return fills != null && 0 <= index && index < fills.size() ? (Fill)fills.get(index) : null;
            match fills {
                Some(f) if 0 <= index && (index as usize) < f.len() => Some(&f[index as usize]),
                _ => None,
            }
        // Java: else
        } else {
            // Java: return null;
            None
        }
    }

    // Java: Outline outlineAt(int index)
    pub fn outlineAt(&self, mut index: i32) -> Option<&Outline> {
        // Java: --index;
        index -= 1;
        // Java: return this.mOutlines != null && 0 <= index && index < this.mOutlines.size() ? (Outline)this.mOutlines.get(index) : null;
        match &self.mOutlines {
            Some(o) if 0 <= index && (index as usize) < o.len() => Some(&o[index as usize]),
            _ => None,
        }
    }

    // Java: int effectStyleSize()
    pub fn effectStyleSize(&self) -> i32 {
        // Java: return this.effectStyleLst == null ? 0 : this.effectStyleLst.size();
        self.effectStyleLst.as_ref().map_or(0, |v| v.len() as i32)
    }

    // Java: EffectStyle effectStyleAt(int index)
    pub fn effectStyleAt(&self, mut index: i32) -> Option<&EffectStyle> {
        // Java: --index;
        index -= 1;
        // Java: return this.effectStyleLst != null && 0 <= index && index < this.effectStyleLst.size() ? (EffectStyle)this.effectStyleLst.get(index) : null;
        match &self.effectStyleLst {
            Some(e) if 0 <= index && (index as usize) < e.len() => Some(&e[index as usize]),
            _ => None,
        }
    }

    // Java: int minorFontSize()
    pub fn minorFontSize(&self) -> i32 {
        // Java: return null == this.minorFonts ? 0 : this.minorFonts.size();
        self.minorFonts.as_ref().map_or(0, |v| v.len() as i32)
    }

    // Java: String minorFontScriptAt(int index)
    pub fn minorFontScriptAt(&self, mut index: i32) -> Option<String> {
        // Java: --index;
        index -= 1;
        // Java: return null != this.minorFontScript && 0 <= index && index < this.minorFontScript.size() ? (String)this.minorFontScript.get(index) : null;
        match &self.minorFontScript {
            Some(s) if 0 <= index && (index as usize) < s.len() => Some(s[index as usize].clone()),
            _ => None,
        }
    }

    // Java: String minorFontTypefaceAt(String script)
    pub fn minorFontTypefaceAt(&self, script: &str) -> String {
        // Java: return null != this.minorFontTypeface && this.minorFontTypeface.containsKey(script) ? (String)this.minorFontTypeface.get(script) : "";
        self.minorFontTypeface.as_ref().and_then(|m| m.get(script)).cloned().unwrap_or_default()
    }

    // Java: int majorFontSize()
    pub fn majorFontSize(&self) -> i32 {
        // Java: return null == this.majorFonts ? 0 : this.majorFonts.size();
        self.majorFonts.as_ref().map_or(0, |v| v.len() as i32)
    }

    // Java: String majorFontScriptAt(int index)
    pub fn majorFontScriptAt(&self, mut index: i32) -> Option<String> {
        // Java: --index;
        index -= 1;
        // Java: return null != this.majorFontScript && 0 <= index && index < this.majorFontScript.size() ? (String)this.majorFontScript.get(index) : null;
        match &self.majorFontScript {
            Some(s) if 0 <= index && (index as usize) < s.len() => Some(s[index as usize].clone()),
            _ => None,
        }
    }

    // Java: String majorFontTypefaceAt(String script)
    pub fn majorFontTypefaceAt(&self, script: &str) -> String {
        // Java: return null != this.majorFontTypeface && this.majorFontTypeface.containsKey(script) ? (String)this.majorFontTypeface.get(script) : "";
        self.majorFontTypeface.as_ref().and_then(|m| m.get(script)).cloned().unwrap_or_default()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.mId;
        self.mId
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: Option<&KmoTheme>) -> bool {
        // Java: if (null != o && o instanceof KmoTheme)
        if o.is_some() && true {
            // Java: if (this == o)
            if std::ptr::eq(self, o.unwrap()) {
                // Java: return true;
                return true;
            // Java: else
            } else {
                // Java: KmoTheme dstTheme = (KmoTheme)o;
                let dstTheme = o.unwrap();
                // Java: return ((ThemeAtomData)this.rawData()).theme.equals(((ThemeAtomData)dstTheme.rawData()).theme);
                return self.parent.rawData().as_theme_atom_data().theme.equals_other(
                    &dstTheme.parent.rawData().as_theme_atom_data().theme
                );
            }
        // Java: else
        } else {
            // Java: return false;
            return false;
        }
    }

    // Java: String getThemePath()
    pub fn getThemePath(&mut self) -> Option<&str> {
        // Java: if (this.mThemePathIsOld)
        if self.mThemePathIsOld {
            // Java: this.updateThemePath();
            self.updateThemePath();
            // Java: this.mThemePathIsOld = false;
            self.mThemePathIsOld = false;
        }
        // Java: return this.mThemePath;
        self.mThemePath.as_deref()
    }

    // Java: void updateThemePath()
    pub fn updateThemePath(&mut self) {
        // Java: if (null != this.mThemePathUpdater)
        if self.mThemePathUpdater.is_some() {
            // Java: this.mThemePathUpdater.update(this);
            // LLM Rust Null-Safety: cannot call updater with &mut self due to borrow, deferred
        }
    }

    // Java: void setThemePath(String mThemePath)
    pub fn setThemePath(&mut self, mThemePath: String) {
        // Java: this.mThemePath = mThemePath;
        self.mThemePath = Some(mThemePath);
    }

    // Java: public Map<String, Integer> getRid2MediaId()
    pub fn getRid2MediaId(&self) -> &Option<HashMap<String, i32>> {
        // Java: return this.mRid2MediaId;
        &self.mRid2MediaId
    }

    // Java: void setRid2MediaId(Map<String, Integer> mRid2MediaId)
    pub fn setRid2MediaId(&mut self, mRid2MediaId: HashMap<String, i32>) {
        // Java: this.mRid2MediaId = mRid2MediaId;
        self.mRid2MediaId = Some(mRid2MediaId);
    }

    // Java: void setThemePathUpdater(ThemePathUpdater themePathUpdater)
    pub fn setThemePathUpdater(&mut self, themePathUpdater: Box<dyn ThemePathUpdaterTrait>) {
        // Java: this.mThemePathUpdater = themePathUpdater;
        self.mThemePathUpdater = Some(themePathUpdater);
    }

    // Java: void setPathIsOld(boolean isOld)
    pub fn setPathIsOld(&mut self, isOld: bool) {
        // Java: this.mThemePathIsOld = isOld;
        self.mThemePathIsOld = isOld;
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: String tempDirectory = Platform.getCurrentPidTmpDir();
        let tempDirectory = Platform::getCurrentPidTmpDir();
        // Java: if (null != this.mThemePath && this.mThemePath.startsWith(tempDirectory))
        if self.mThemePath.is_some() && self.mThemePath.as_ref().unwrap().starts_with(&tempDirectory) {
            // Java: FileUtil.delFile(this.mThemePath);
            FileUtil::delFile(self.mThemePath.as_ref().unwrap());
        }
    }

    // Java: static int createId()
    pub fn createId() -> i32 {
        // Java: return curId.getAndIncrement();
        CUR_ID.fetch_add(1, Ordering::SeqCst)
    }
}

// Java: public class IdGenerator
impl IdGenerator {
    // Java: static int createId()
    pub fn createId() -> i32 {
        // Java: return curId.getAndIncrement();
        CUR_ID.fetch_add(1, Ordering::SeqCst)
    }
}

struct ThemeAtomData {
    pub theme: crate::cn::wps::drawing::data::style::Theme::Theme,
}

impl ThemeAtomData {
    fn new() -> Self {
        ThemeAtomData {
            theme: crate::cn::wps::drawing::data::style::Theme::Theme::create(),
        }
    }
}

impl KmoProtocol for KmoTheme {
}
