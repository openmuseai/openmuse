// Java: import cn.wps.font.FreeTypeJNI
use crate::cn::wps::font::FreeTypeJNI::TTFHeader;
// Java: import cn.wps.font.ITypeface
use crate::cn::wps::font::ITypeface::ITypeface;
// Java: import cn.wps.graphics.Rect

#[derive(Clone, Default)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

use std::sync::{Arc, Mutex, OnceLock};

// Java: public interface IMetricsFactory
// LLM Java inner class (interface): translated to Rust trait + companion struct
pub struct IMetricsFactory;
pub trait IMetricsFactoryTrait: Send + Sync {
    // Java: abstract IMetricsGetter createMetrics(ITypeface var1)
    fn createMetrics(&self, var1: &dyn ITypeface) -> Option<Arc<dyn IMetricsGetterTrait>>;
}


// Java: public interface IMetricsGetter
pub struct IMetricsGetter;
pub trait IMetricsGetterTrait: Send + Sync {
    // Java: abstract boolean isBuildInHeader()
    fn isBuildInHeader(&self) -> bool;

    // Java: abstract FreeTypeJNI.TTFHeader getTTFHeader()
    fn getTTFHeader(&self) -> Option<TTFHeader>;

    // Java: abstract int getGlyphAdvanceEM(char var1)
    fn getGlyphAdvanceEM(&self, var1: u16) -> i32;

    // Java: abstract boolean cacheGlyphAdvanceEM(char var1, int var2)
    fn cacheGlyphAdvanceEM(&self, var1: u16, var2: i32) -> bool;

    // Java: abstract Rect getCharRect(char var1)
    fn getCharRect_char(&self, var1: u16) -> Option<Rect>;

    // Java: abstract boolean getCharRect(char var1, Rect var2)
    fn getCharRect_char_rect(&self, var1: u16, var2: &mut Rect) -> bool;

    // Java: abstract boolean cacheCharRect(char var1, Rect var2)
    fn cacheCharRect_rect(&self, var1: u16, var2: &Rect) -> bool;

    // Java: abstract boolean cacheCharRect(char var1, int var2, int var3, int var4, int var5)
    fn cacheCharRect_ints(&self, var1: u16, var2: i32, var3: i32, var4: i32, var5: i32) -> bool;

    // Java: abstract boolean hasKering()
    fn hasKering(&self) -> bool;

    // Java: abstract int getKerningPair(char var1, char var2)
    fn getKerningPair(&self, var1: u16, var2: u16) -> i32;

    // Java: abstract boolean cacheKerningPair(char var1, char var2, int var3)
    fn cacheKerningPair(&self, var1: u16, var2: u16, var3: i32) -> bool;
}

// Java field: IMetricsFactory mMetricsGettor

// Java: public class BuildinGlyphMetrics
pub struct BuildinGlyphMetrics {
    mMetricsGettor: Option<Arc<dyn IMetricsFactoryTrait>>,
}

static INSTANCE: OnceLock<Mutex<BuildinGlyphMetrics>> = OnceLock::new();

// Java: public class Holder
// LLM Java inner class: singleton holder pattern, translated to OnceLock above
pub struct Holder;
// Java field: BuildinGlyphMetrics instance
// LLM Java inner class: static field inlined into OnceLock<Mutex<BuildinGlyphMetrics>> INSTANCE above
// Java: Holder()
// LLM No inheritance in Rust: plain struct has no super constructor

// Java: public class BuildinGlyphMetrics
impl BuildinGlyphMetrics {
    // Java: static BuildinGlyphMetrics getInstance()
    pub fn getInstance() -> &'static Mutex<BuildinGlyphMetrics> {
        // Java: return BuildinGlyphMetrics.Holder.instance;
        INSTANCE.get_or_init(|| Mutex::new(BuildinGlyphMetrics::new()))
    }

    // Java: BuildinGlyphMetrics()
    pub fn new() -> Self {
        // Java: super();
        BuildinGlyphMetrics {
            mMetricsGettor: None,
        }
    }

    // Java: synchronized IMetricsFactory getMetricsFactory()
    // LLM synchronized: protected by Mutex wrapping INSTANCE (callers lock via getInstance())
    pub fn getMetricsFactory(&self) -> Option<Arc<dyn IMetricsFactoryTrait>> {
        // Java: return this.mMetricsGettor;
        self.mMetricsGettor.clone()
    }

    // Java: synchronized void setMetricsFactory(IMetricsFactory gettor)
    // LLM synchronized: protected by Mutex wrapping INSTANCE (callers lock via getInstance())
    pub fn setMetricsFactory(&mut self, gettor: Option<Arc<dyn IMetricsFactoryTrait>>) {
        // Java: this.mMetricsGettor = gettor;
        self.mMetricsGettor = gettor;
    }
}
