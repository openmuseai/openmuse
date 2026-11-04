#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused_variables, unused_imports, dead_code)]

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::cn::wps::kfc::numfmt::i18n::KmoLang::KmoLang;

// Java: public class GlobalEnv
pub struct GlobalEnv;

static DEF_LOCALE_CODE_ID: Mutex<i16> = Mutex::new(0);
static M_RULE_LCID_BASE: OnceLock<Mutex<HashMap<i16, i16>>> = OnceLock::new();

// Java field: short LAST_CHOICE_LCID
pub(crate) const LAST_CHOICE_LCID: i16 = 0;

// Java field: static final LocaleStrManager localeStrManager
pub static localeStrManager: std::sync::LazyLock<Mutex<super::LocaleStrManager::LocaleStrManager>>=std::sync::LazyLock::new(||Mutex::new(super::LocaleStrManager::LocaleStrManager::new()));

impl GlobalEnv {
    // Java field: short deflocaleCodeID
    // LLM JavaLine Aggregation: declared as static DEF_LOCALE_CODE_ID above
    // Java field: LocaleStrManager localeStrManager
    // LLM JavaLine Aggregation: declared as static localeStrManager above
    // Java field: HashMap<Short, Short> mRuleLCIDBase
    // LLM JavaLine Aggregation: declared as static M_RULE_LCID_BASE above

    // Java: GlobalEnv()
    pub fn new() -> Self {
        // Java: super();
        GlobalEnv
    }

    // Java: static boolean setLCID(short lcid)
    pub fn setLCID(lcid: i16) -> bool {
        // Java: assert lcid != 0;
        debug_assert!(lcid != 0);
        let mut def = DEF_LOCALE_CODE_ID.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        // Java: if (deflocaleCodeID == lcid)
        if *def == lcid {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: localeStrManager.loadDefaultRes(lcid);
            localeStrManager.lock().unwrap_or_else(std::sync::PoisonError::into_inner).loadDefaultRes(lcid);
            // Java: deflocaleCodeID = lcid;
            *def = lcid;
            // Java: return true;
            return true;
        }
    }

    // Java: static short getDefaultLCID()
    pub fn getDefaultLCID() -> i16 {
        // Java: return deflocaleCodeID;
        *DEF_LOCALE_CODE_ID.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    // Java: static short getValidLCID(short lcid)
    pub fn getValidLCID(lcid: i16) -> i16 {
        // Java: return lcid == 0 ? deflocaleCodeID : lcid;
        let lcid = lcid;
        let deflocaleCodeID = Self::getDefaultLCID();
        if lcid == 0 { deflocaleCodeID } else { lcid }
    }

    // Java: static short matchLCID(short langID)
    pub fn matchLCID(lang_id: i16) -> i16 {
        // Java: initRuleLCID();
        Self::initRuleLCID();
        let map = M_RULE_LCID_BASE.get().unwrap().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        // Java: Short matcher = (Short)mRuleLCIDBase.get(KmoLang.getPrimaryLang(langID));
        let primary = KmoLang::getPrimaryLang(lang_id);
        let matcher = map.get(&primary).copied();
        // Java: return matcher != null ? matcher : 1033;
        matcher.unwrap_or(1033)
    }

    // Java: static void initRuleLCID()
    fn initRuleLCID() {
        // Java: if (mRuleLCIDBase == null)
        // LLM Rust Null-Safety: OnceLock::get_or_init handles null/init check
        M_RULE_LCID_BASE.get_or_init(|| {
            // Java: mRuleLCIDBase = new HashMap();
            let mut map = HashMap::new();
            // Java: short lang = 1049;
            let mut lang: i16 = 1049;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 3082;
            lang = 3082;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1025;
            lang = 1025;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1058;
            lang = 1058;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1063;
            lang = 1063;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1026;
            lang = 1026;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1027;
            lang = 1027;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 2052;
            lang = 2052;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1029;
            lang = 1029;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1030;
            lang = 1030;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1031;
            lang = 1031;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1032;
            lang = 1032;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1033;
            lang = 1033;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1034;
            lang = 1034;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1035;
            lang = 1035;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1036;
            lang = 1036;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1037;
            lang = 1037;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1037;
            lang = 1037;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1040;
            lang = 1040;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1041;
            lang = 1041;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1042;
            lang = 1042;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1043;
            lang = 1043;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1044;
            lang = 1044;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1045;
            lang = 1045;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1046;
            lang = 1046;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1047;
            lang = 1047;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1048;
            lang = 1048;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1049;
            lang = 1049;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1050;
            lang = 1050;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1051;
            lang = 1051;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1053;
            lang = 1053;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1054;
            lang = 1054;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1055;
            lang = 1055;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1057;
            lang = 1057;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1058;
            lang = 1058;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1060;
            lang = 1060;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1061;
            lang = 1061;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1062;
            lang = 1062;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1063;
            lang = 1063;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1065;
            lang = 1065;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1066;
            lang = 1066;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1071;
            lang = 1071;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1081;
            lang = 1081;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            // Java: lang = 1086;
            lang = 1086;
            // Java: mRuleLCIDBase.put(KmoLang.getPrimaryLang(lang), lang);
            map.insert(KmoLang::getPrimaryLang(lang), lang);
            Mutex::new(map)
        });
    }
}
