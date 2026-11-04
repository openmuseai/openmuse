#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use std::sync::{Mutex, OnceLock};

// Java: import cn.wps.kfc.numfmt.global.GlobalEnv
use crate::cn::wps::kfc::numfmt::global::GlobalEnv;
// Java: import cn.wps.kfc.numfmt.resource.IResourceParser
use crate::cn::wps::kfc::numfmt::resource::IResourceParser;
// Java: import cn.wps.kfc.numfmt.resource.ResourceDef
use crate::cn::wps::kfc::numfmt::resource::ResourceDef;
// Java: import cn.wps.kfc.numfmt.resource.ResourceLoader
use crate::cn::wps::kfc::numfmt::resource::ResourceLoader;
// Java: import java.util.List

// Java field: int FIRST_USER_DEFINED_FORMAT_INDEX
pub(crate) const FIRST_USER_DEFINED_FORMAT_INDEX: i32 = 164;
// Java field: String GENERAL
pub(crate) const GENERAL: &str = "General";
// Java field: String[] builtinFormats
static builtinFormats: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
// Java field: short latestLoaded
static latestLoaded: OnceLock<Mutex<i16>> = OnceLock::new();

fn get_latestLoaded() -> &'static Mutex<i16> {
    latestLoaded.get_or_init(|| Mutex::new(0))
}

fn get_builtinFormats() -> &'static Mutex<Vec<String>> {
    builtinFormats.get_or_init(|| Mutex::new(Vec::new()))
}

// Java: public class BuiltinFormats
pub struct BuiltinFormats {
}

// Java: public class BuiltinFormats
impl BuiltinFormats {
    // Java: BuiltinFormats()
    pub fn new() -> Self {
        // Java: super();
        BuiltinFormats {}
    }

    // Java: static void loadFormatCodes(short lcid)
    pub fn loadFormatCodes(lcid:i16) {
// java assert lcid!=0;
assert!(lcid!=0);let mut latest=get_latestLoaded().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
// java if(latestLoaded==0||latestLoaded!=lcid) {
if *latest==0||*latest!=lcid {
use crate::cn::wps::kfc::numfmt::resource::{ResourceLoader::ResourceLoader,ResourceDef::ResourceDef,IResourceParser::IResourceParser};
// java ResourceLoader.getSaxParser(ResourceDef.getBuiltinFormatsName(lcid)); fallback matchLCID; fallback 1033.
let parser=ResourceLoader::getSaxParser(&ResourceDef::getBuiltinFormatsName(lcid)).or_else(||ResourceLoader::getSaxParser(&ResourceDef::getBuiltinFormatsName(crate::cn::wps::kfc::numfmt::global::GlobalEnv::GlobalEnv::matchLCID(lcid)))).or_else(||ResourceLoader::getSaxParser(&ResourceDef::getBuiltinFormatsName(1033))).expect("builtin formats resource");
// java List<String> mFormats=parser.getArray("builtin-formats"); assert mFormats.size()==builtinFormats.length;mFormats.toArray(builtinFormats);latestLoaded=lcid;
let formats=parser.getArray("builtin-formats");assert_eq!(formats.len(),59);*get_builtinFormats().lock().unwrap_or_else(std::sync::PoisonError::into_inner)=formats;*latest=lcid;
}
    }

    // Java: static String[] getAll()
    pub fn getAll() -> Vec<String> {
        // Java: return builtinFormats;
        get_builtinFormats().lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
    }

    // Java: static boolean isBuiltinFormat(int formatIdx)
    pub fn isBuiltinFormat(formatIdx: i32) -> bool {
        // Java: return formatIdx < 164;
        formatIdx < 164
    }
}
