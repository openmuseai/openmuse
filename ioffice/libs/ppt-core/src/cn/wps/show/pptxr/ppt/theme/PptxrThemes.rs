use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::io;
use std::rc::Rc;

// Java: import cn.wps.base.assertion.IOAssert
use std::convert::Infallible as IOAssert;
// Java: import cn.wps.kfc.ooxfr.OxfPptRelations
use std::marker::PhantomData as OxfPptRelations;
// Java: import cn.wps.kfc.opcr.OpcrPackagePart
use crate::cn::wps::show::pptxr::ppt::theme::PptxrTheme::OpcrPackagePart;
// Java: import cn.wps.kfc.opcr.OpcrRelationship
use std::marker::PhantomData as OpcrRelationshipMarker;
// Java: import cn.wps.show.PptFileCache
use std::marker::PhantomData as PptFileCacheMarker;
// Java: import cn.wps.show.app.KmoPresentation
use crate::cn::wps::show::util::layout_shape::KmoPresentation;
// Java: import cn.wps.show.app.theme.KmoTheme
use crate::cn::wps::show::util::layout_shape::ThemeObj;
pub type KmoTheme = ThemeObj;
// Java: import cn.wps.show.pptxr.ppt.media.PptxrMedias
use crate::cn::wps::show::pptxr::ppt::theme::PptxrTheme::PptxrMedias;
// Java: import java.io.IOException
use std::io::Error as IOException;
// Java: import java.util.HashMap
use std::collections::HashMap as JavaHashMap;
// Java: import java.util.Map
use std::collections::HashMap as JavaMap;
// Java: import cn.wps.show.pptxr.ppt.theme.PptxrTheme
use crate::cn::wps::show::pptxr::ppt::theme::PptxrTheme::PptxrTheme;
// Java: import cn.wps.show.pptxr.ppt.theme.PptxrThemeOverride
use std::marker::PhantomData as PptxrThemeOverride;

pub const THEME_RELS_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme";
pub const THEME_OVERRIDE_RELS_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride";

thread_local! {
    static DUMP_FILE_CALLS: Cell<i32> = Cell::new(0);
}

pub struct OpcrRelationship {
    pub part_name: String,
    pub rels_type: String,
    pub part: Option<OpcrPackagePart>,
}

impl OpcrRelationship {
    pub fn getPartName(&self) -> String {
        self.part_name.clone()
    }
    pub fn getType(&self) -> String {
        self.rels_type.clone()
    }
    pub fn getPackagePart(&self) -> Option<OpcrPackagePart> {
        self.part.clone()
    }
}

pub struct PptFileCache {}

impl PptFileCache {
    pub fn dumpFile(part: Option<&OpcrPackagePart>, pptRootTmpDir: &str) -> io::Result<String> {
        DUMP_FILE_CALLS.with(|c| c.set(c.get() + 1));
        let _ = part.ok_or_else(|| io::Error::new(io::ErrorKind::Other, "dumpFile null part"))?;
        Ok(format!("{}/theme.xml", pptRootTmpDir))
    }
    pub fn dumpFileCallCount() -> i32 {
        DUMP_FILE_CALLS.with(|c| c.get())
    }
    pub fn resetDumpFileCallCount() {
        DUMP_FILE_CALLS.with(|c| c.set(0));
    }
}

fn IOAssert_assertNotNull<T>(_v: &T) {
    let _ = (
        Option::<IOAssert>::None,
        OxfPptRelations::<()>,
        OpcrRelationshipMarker::<()>,
        PptFileCacheMarker::<()>,
        PptxrThemeOverride::<()>,
        JavaHashMap::<String, i32>::new(),
        JavaMap::<String, String>::new(),
        IOException::from(io::ErrorKind::Other),
    );
}

// java public final class PptxrThemes
pub struct PptxrThemes {
    // java private Map<String, KmoTheme> mGlobalThemes = new HashMap();
    mGlobalThemes: HashMap<String, Rc<RefCell<KmoTheme>>>,
    // java private Map<String, String> mGlobalThemesPath = new HashMap();
    mGlobalThemesPath: HashMap<String, String>,
    // java private int mMode;
    mMode: i32,
}

// java public final class PptxrThemes
impl PptxrThemes {
    // java public  PptxrThemes(int mode)
    pub fn PptxrThemes(mode: i32) -> Self {
        // java this.mMode = mode;
        Self {
            mGlobalThemes: HashMap::new(),
            mGlobalThemesPath: HashMap::new(),
            mMode: mode,
        }
    }

    // java public KmoTheme open(KmoPresentation pres, OpcrRelationship themeRels, PptxrMedias medias)
    pub fn open(
        &mut self,
        pres: &mut KmoPresentation,
        themeRels: &OpcrRelationship,
        medias: Option<PptxrMedias>,
    ) -> io::Result<Rc<RefCell<KmoTheme>>> {
        // java return this.open(pres, themeRels, medias, true);
        self.open_addTheme(pres, themeRels, medias, true)
    }

    // java public KmoTheme open(KmoPresentation pres, OpcrRelationship themeRels, PptxrMedias medias, boolean addTheme)
    pub fn open_addTheme(
        &mut self,
        pres: &mut KmoPresentation,
        themeRels: &OpcrRelationship,
        medias: Option<PptxrMedias>,
        addTheme: bool,
    ) -> io::Result<Rc<RefCell<KmoTheme>>> {
        // java String partName = themeRels.getPartName();
        let partName = themeRels.getPartName();
        // java KmoTheme theme = (KmoTheme)this.mGlobalThemes.get(partName);
        let cached = self.mGlobalThemes.get(&partName).cloned();
        // java if (theme != null)
        if let Some(theme) = cached {
            // java return theme;
            return Ok(theme);
        }
        // java else
        else {
            // java OpcrPackagePart part = themeRels.getPackagePart();
            let part = themeRels.getPackagePart();
            // java HashMap<String, Integer> rid2MediaIds = null;
            let mut rid2MediaIds: Option<HashMap<String, i32>> = None;
            let theme: Rc<RefCell<KmoTheme>>;
            // java if (part != null)
            if part.is_some() {
                // java theme = new KmoTheme();
                theme = Rc::new(RefCell::new(KmoTheme::create()));
                // java String pptRootDir = pres.getTmpPath();
                let pptRootDir = pres.getTmpPath();
                // java if (themeRels.getType().equals(OxfPptRelations.theme().getRelsType()))
                if themeRels.getType() == THEME_RELS_TYPE {
                    // java PptxrTheme themeReader = new PptxrTheme(theme, part, pres.mediaLib(), medias, this.mMode);
                    let mut themeReader = PptxrTheme::PptxrTheme(
                        theme.clone(),
                        part.clone(),
                        Some(pres.mediaLib()),
                        medias,
                        self.mMode,
                    );
                    // java rid2MediaIds = themeReader.open();
                    rid2MediaIds = themeReader.open();
                    // java theme.setRid2MediaId(rid2MediaIds);
                    theme.borrow_mut().setRid2MediaId(rid2MediaIds.clone());
                    // java theme.setThemePath(this.dump(themeRels, pptRootDir));
                    theme
                        .borrow_mut()
                        .setThemePath(Some(self.dump(themeRels, pptRootDir)?));
                }
                // java else if (themeRels.getType().equals(OxfPptRelations.themeOverride().getRelsType()))
                else if themeRels.getType() == THEME_OVERRIDE_RELS_TYPE {
                    todo!("DEFERRED_NON_FUNCTION");
                    /* DEFERRED_NON_FUNCTION
                    // java PptxrThemeOverride themeOverrideReader = new PptxrThemeOverride(theme, themeRels.getPackagePart(), pres.mediaLib(), medias, this.mMode);
                    let mut themeOverrideReader: PptxrThemeOverride = PptxrThemeOverride::new(theme, themeRels.getPackagePart(), pres.mediaLib(), medias, self.mMode);
                    // java rid2MediaIds = themeOverrideReader.open();
                    rid2MediaIds = themeOverrideReader.open();
                    // java theme.setRid2MediaId(rid2MediaIds);
                    theme.setRid2MediaId(rid2MediaIds);
                    // java theme.setThemePath(this.dump(themeRels, pptRootDir));
                    theme.setThemePath(self.dump(&themeRels, pptRootDir));
                    */
                }
            }
            // java else
            else {
                // java theme = PptxrTheme.buildDefaultTheme();
                theme = PptxrTheme::buildDefaultTheme()?;
            }
            // java this.mGlobalThemes.put(partName, theme);
            self.mGlobalThemes.insert(partName.clone(), theme.clone());
            // java if (addTheme)
            if addTheme {
                // java pres._addTheme(theme);
                pres._addTheme(theme.clone());
            }
            // java return theme;
            return Ok(theme);
        }
    }

    // java public String dump(OpcrRelationship themeRels, String pptRootTmpDir)
    pub fn dump(&mut self, themeRels: &OpcrRelationship, pptRootTmpDir: String) -> io::Result<String> {
        // java IOAssert.assertNotNull(themeRels);
        IOAssert_assertNotNull(themeRels);
        // java String partName = themeRels.getPartName();
        let partName = themeRels.getPartName();
        // java String themePath = (String)this.mGlobalThemesPath.get(partName);
        let mut themePath = self.mGlobalThemesPath.get(&partName).cloned();
        // java if (themePath != null)
        if let Some(themePath) = themePath {
            // java return themePath;
            return Ok(themePath);
        }
        // java else
        else {
            // java OpcrPackagePart part = themeRels.getPackagePart();
            let part = themeRels.getPackagePart();
            // java themePath = PptFileCache.dumpFile(part, pptRootTmpDir);
            let dumped = PptFileCache::dumpFile(part.as_ref(), &pptRootTmpDir)?;
            themePath = Some(dumped);
            // java this.mGlobalThemesPath.put(partName, themePath);
            self.mGlobalThemesPath
                .insert(partName, themePath.clone().unwrap());
            // java return themePath;
            return Ok(themePath.unwrap());
        }
    }
}
