// Java: cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ThemeFontTool
// The SCRIPT_LANGUAGES_MAP entries below are transcribed in Java insertion order.
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use crate::cn::wps::moffice::writer::data::theme::font::FontCollection::FontCollection;
use crate::cn::wps::moffice::writer::data::value::FontInfo::FontInfo;
use crate::cn::wps::moffice::writer::data::value::LID::BCP2LIDMAP;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::java_string_hash_code;
use util::moffice::property::PropertySet::PropertySet;

// java public final class ThemeFontTool
pub struct ThemeFontTool;

// java private static final WIntObjectHashMap<int[]> SCRIPT_LANGUAGES_MAP = ...;
const SCRIPT_LANGUAGES_MAP: &[(i32, &[i32])] = &[
    // Java: this.put(15370, new int[]{2361039});
    (15370, &[2361039]),
    // Java: this.put(1100, new int[]{2401555});
    (1100, &[2401555]),
    // Java: this.put(2117, new int[]{2066780});
    (2117, &[2066780]),
    // Java: this.put(1064, new int[]{2115920});
    (1064, &[2115920]),
    // Java: this.put(1122, new int[]{2361039});
    (1122, &[2361039]),
    // Java: this.put(20490, new int[]{2361039});
    (20490, &[2361039]),
    // Java: this.put(1061, new int[]{2361039});
    (1061, &[2361039]),
    // Java: this.put(1136, new int[]{2361039});
    (1136, &[2361039]),
    // Java: this.put(2108, new int[]{2361039});
    (2108, &[2361039]),
    // Java: this.put(4122, new int[]{2361039});
    (4122, &[2361039]),
    // Java: this.put(12289, new int[]{2049074});
    (12289, &[2049074]),
    // Java: this.put(9217, new int[]{2049074});
    (9217, &[2049074]),
    // Java: this.put(1087, new int[]{2115920});
    (1087, &[2115920]),
    // Java: this.put(7169, new int[]{2049074});
    (7169, &[2049074]),
    // Java: this.put(11265, new int[]{2049074});
    (11265, &[2049074]),
    // Java: this.put(1034, new int[]{2361039});
    (1034, &[2361039]),
    // Java: this.put(1094, new int[]{2231249});
    (1094, &[2231249]),
    // Java: this.put(1057, new int[]{2361039});
    (1057, &[2361039]),
    // Java: this.put(2068, new int[]{2361039});
    (2068, &[2361039]),
    // Java: this.put(4097, new int[]{2049074});
    (4097, &[2049074]),
    // Java: this.put(1096, new int[]{2466891});
    (1096, &[2466891]),
    // Java: this.put(1044, new int[]{2361039});
    (1044, &[2361039]),
    // Java: this.put(1144, new int[]{2755664});
    (1144, &[2755664]),
    // Java: this.put(1103, new int[]{2126604});
    (1103, &[2126604]),
    // Java: this.put(1052, new int[]{2361039});
    (1052, &[2361039]),
    // Java: this.put(6153, new int[]{2361039});
    (6153, &[2361039]),
    // Java: this.put(2060, new int[]{2361039});
    (2060, &[2361039]),
    // Java: this.put(12298, new int[]{2361039});
    (12298, &[2361039]),
    // Java: this.put(4105, new int[]{2361039});
    (4105, &[2361039]),
    // Java: this.put(1110, new int[]{2361039});
    (1110, &[2361039]),
    // Java: this.put(2057, new int[]{2361039});
    (2057, &[2361039]),
    // Java: this.put(1047, new int[]{2361039});
    (1047, &[2361039]),
    // Java: this.put(2074, new int[]{2361039});
    (2074, &[2361039]),
    // Java: this.put(4108, new int[]{2361039});
    (4108, &[2361039]),
    // Java: this.put(1042, new int[]{2241682, 2241684, 2344631});
    (1042, &[2241682, 2241684, 2344631]),
    // Java: this.put(3098, new int[]{2115920});
    (3098, &[2115920]),
    // Java: this.put(8193, new int[]{2049074});
    (8193, &[2049074]),
    // Java: this.put(17417, new int[]{2361039});
    (17417, &[2361039]),
    // Java: this.put(1030, new int[]{2361039});
    (1030, &[2361039]),
    // Java: this.put(1028, new int[]{2241684, 2241695});
    (1028, &[2241684, 2241695]),
    // Java: this.put(2064, new int[]{2361039});
    (2064, &[2361039]),
    // Java: this.put(1083, new int[]{2361039});
    (1083, &[2361039]),
    // Java: this.put(3076, new int[]{2241684, 2241695});
    (3076, &[2241684, 2241695]),
    // Java: this.put(1054, new int[]{2605500});
    (1054, &[2605500]),
    // Java: this.put(5127, new int[]{2361039});
    (5127, &[2361039]),
    // Java: this.put(16393, new int[]{2361039});
    (16393, &[2361039]),
    // Java: this.put(1078, new int[]{2361039});
    (1078, &[2361039]),
    // Java: this.put(1117, new int[]{2092739});
    (1117, &[2092739]),
    // Java: this.put(1105, new int[]{2606503});
    (1105, &[2606503]),
    // Java: this.put(1111, new int[]{2126604});
    (1111, &[2126604]),
    // Java: this.put(1088, new int[]{2115920});
    (1088, &[2115920]),
    // Java: this.put(1029, new int[]{2361039});
    (1029, &[2361039]),
    // Java: this.put(1152, new int[]{2049074});
    (1152, &[2049074]),
    // Java: this.put(1130, new int[]{2361039});
    (1130, &[2361039]),
    // Java: this.put(5129, new int[]{2361039});
    (5129, &[2361039]),
    // Java: this.put(2055, new int[]{2361039});
    (2055, &[2361039]),
    // Java: this.put(2128, new int[]{2404091});
    (2128, &[2404091]),
    // Java: this.put(4106, new int[]{2361039});
    (4106, &[2361039]),
    // Java: this.put(1108, new int[]{2360885});
    (1108, &[2360885]),
    // Java: this.put(1050, new int[]{2361039});
    (1050, &[2361039]),
    // Java: this.put(4103, new int[]{2361039});
    (4103, &[2361039]),
    // Java: this.put(5124, new int[]{2241684, 2241695});
    (5124, &[2241684, 2241695]),
    // Java: this.put(12297, new int[]{2361039});
    (12297, &[2361039]),
    // Java: this.put(1076, new int[]{2361039});
    (1076, &[2361039]),
    // Java: this.put(6154, new int[]{2361039});
    (6154, &[2361039]),
    // Java: this.put(5130, new int[]{2361039});
    (5130, &[2361039]),
    // Java: this.put(1024, new int[]{2241694});
    (1024, &[2241694]),
    // Java: this.put(1027, new int[]{2361039});
    (1027, &[2361039]),
    // Java: this.put(1058, new int[]{2115920});
    (1058, &[2115920]),
    // Java: this.put(7178, new int[]{2361039});
    (7178, &[2361039]),
    // Java: this.put(1080, new int[]{2361039});
    (1080, &[2361039]),
    // Java: this.put(2070, new int[]{2361039});
    (2070, &[2361039]),
    // Java: this.put(1074, new int[]{2361039});
    (1074, &[2361039]),
    // Java: this.put(1059, new int[]{2115920});
    (1059, &[2115920]),
    // Java: this.put(1124, new int[]{2361039});
    (1124, &[2361039]),
    // Java: this.put(1115, new int[]{2577072});
    (1115, &[2577072]),
    // Java: this.put(1104, new int[]{2115920});
    (1104, &[2115920]),
    // Java: this.put(1037, new int[]{2245165});
    (1037, &[2245165]),
    // Java: this.put(1077, new int[]{2361039});
    (1077, &[2361039]),
    // Java: this.put(3084, new int[]{2361039});
    (3084, &[2361039]),
    // Java: this.put(13322, new int[]{2361039});
    (13322, &[2361039]),
    // Java: this.put(1039, new int[]{2361039});
    (1039, &[2361039]),
    // Java: this.put(1121, new int[]{2126604});
    (1121, &[2126604]),
    // Java: this.put(1038, new int[]{2361039});
    (1038, &[2361039]),
    // Java: this.put(3073, new int[]{2049074});
    (3073, &[2049074]),
    // Java: this.put(1153, new int[]{2361039});
    (1153, &[2361039]),
    // Java: this.put(1089, new int[]{2361039});
    (1089, &[2361039]),
    // Java: this.put(2052, new int[]{2241684, 2241694});
    (2052, &[2241684, 2241694]),
    // Java: this.put(8201, new int[]{2361039});
    (8201, &[2361039]),
    // Java: this.put(1068, new int[]{2361039});
    (1068, &[2361039]),
    // Java: this.put(1049, new int[]{2115920});
    (1049, &[2115920]),
    // Java: this.put(1048, new int[]{2361039});
    (1048, &[2361039]),
    // Java: this.put(1031, new int[]{2361039});
    (1031, &[2361039]),
    // Java: this.put(1106, new int[]{2361039});
    (1106, &[2361039]),
    // Java: this.put(1025, new int[]{2049074});
    (1025, &[2049074]),
    // Java: this.put(1123, new int[]{2049074});
    (1123, &[2049074]),
    // Java: this.put(1132, new int[]{2361039});
    (1132, &[2361039]),
    // Java: this.put(5121, new int[]{2049074});
    (5121, &[2049074]),
    // Java: this.put(1065, new int[]{2049074});
    (1065, &[2049074]),
    // Java: this.put(6145, new int[]{2049074});
    (6145, &[2049074]),
    // Java: this.put(1067, new int[]{2049458});
    (1067, &[2049458]),
    // Java: this.put(16394, new int[]{2361039});
    (16394, &[2361039]),
    // Java: this.put(2143, new int[]{2361039});
    (2143, &[2361039]),
    // Java: this.put(1097, new int[]{2599148});
    (1097, &[2599148]),
    // Java: this.put(1114, new int[]{2592567});
    (1114, &[2592567]),
    // Java: this.put(3179, new int[]{2361039});
    (3179, &[2361039]),
    // Java: this.put(1131, new int[]{2361039});
    (1131, &[2361039]),
    // Java: this.put(19466, new int[]{2361039});
    (19466, &[2361039]),
    // Java: this.put(3081, new int[]{2361039});
    (3081, &[2361039]),
    // Java: this.put(13313, new int[]{2049074});
    (13313, &[2049074]),
    // Java: this.put(1101, new int[]{2066780});
    (1101, &[2066780]),
    // Java: this.put(1033, new int[]{2361039});
    (1033, &[2361039]),
    // Java: this.put(1128, new int[]{2361039});
    (1128, &[2361039]),
    // Java: this.put(1045, new int[]{2361039});
    (1045, &[2361039]),
    // Java: this.put(1086, new int[]{2361039});
    (1086, &[2361039]),
    // Java: this.put(2092, new int[]{2115920});
    (2092, &[2115920]),
    // Java: this.put(1071, new int[]{2115920});
    (1071, &[2115920]),
    // Java: this.put(11273, new int[]{2361039});
    (11273, &[2361039]),
    // Java: this.put(1092, new int[]{2115920});
    (1092, &[2115920]),
    // Java: this.put(2110, new int[]{2361039});
    (2110, &[2361039]),
    // Java: this.put(6156, new int[]{2361039});
    (6156, &[2361039]),
    // Java: this.put(1041, new int[]{2241684, 2249488, 2315283, 2331049});
    (1041, &[2241684, 2249488, 2315283, 2331049]),
    // Java: this.put(1091, new int[]{2361039});
    (1091, &[2361039]),
    // Java: this.put(1099, new int[]{2343232});
    (1099, &[2343232]),
    // Java: this.put(10249, new int[]{2361039});
    (10249, &[2361039]),
    // Java: this.put(1102, new int[]{2126604});
    (1102, &[2126604]),
    // Java: this.put(5146, new int[]{2361039});
    (5146, &[2361039]),
    // Java: this.put(9225, new int[]{2361039});
    (9225, &[2361039]),
    // Java: this.put(13321, new int[]{2361039});
    (13321, &[2361039]),
    // Java: this.put(1107, new int[]{2337762});
    (1107, &[2337762]),
    // Java: this.put(14346, new int[]{2361039});
    (14346, &[2361039]),
    // Java: this.put(1056, new int[]{2049074});
    (1056, &[2049074]),
    // Java: this.put(2115, new int[]{2115920});
    (2115, &[2115920]),
    // Java: this.put(18441, new int[]{2361039});
    (18441, &[2361039]),
    // Java: this.put(2077, new int[]{2361039});
    (2077, &[2361039]),
    // Java: this.put(5132, new int[]{2361039});
    (5132, &[2361039]),
    // Java: this.put(1093, new int[]{2066780});
    (1093, &[2066780]),
    // Java: this.put(1070, new int[]{2361039});
    (1070, &[2361039]),
    // Java: this.put(10250, new int[]{2361039});
    (10250, &[2361039]),
    // Java: this.put(1066, new int[]{2361039});
    (1066, &[2361039]),
    // Java: this.put(1060, new int[]{2361039});
    (1060, &[2361039]),
    // Java: this.put(2049, new int[]{2049074});
    (2049, &[2049074]),
    // Java: this.put(11274, new int[]{2361039});
    (11274, &[2361039]),
    // Java: this.put(1125, new int[]{2605492});
    (1125, &[2605492]),
    // Java: this.put(21514, new int[]{2361039});
    (21514, &[2361039]),
    // Java: this.put(1090, new int[]{2361039});
    (1090, &[2361039]),
    // Java: this.put(1118, new int[]{2170384});
    (1118, &[2170384]),
    // Java: this.put(3082, new int[]{2361039});
    (3082, &[2361039]),
    // Java: this.put(1051, new int[]{2361039});
    (1051, &[2361039]),
    // Java: this.put(17418, new int[]{2361039});
    (17418, &[2361039]),
    // Java: this.put(1081, new int[]{2126604});
    (1081, &[2126604]),
    // Java: this.put(2155, new int[]{2361039});
    (2155, &[2361039]),
    // Java: this.put(9226, new int[]{2361039});
    (9226, &[2361039]),
    // Java: this.put(1069, new int[]{2361039});
    (1069, &[2361039]),
    // Java: this.put(1035, new int[]{2361039});
    (1035, &[2361039]),
    // Java: this.put(1095, new int[]{2230998});
    (1095, &[2230998]),
    // Java: this.put(1036, new int[]{2361039});
    (1036, &[2361039]),
    // Java: this.put(1079, new int[]{2215777});
    (1079, &[2215777]),
    // Java: this.put(2058, new int[]{2361039});
    (2058, &[2361039]),
    // Java: this.put(1046, new int[]{2361039});
    (1046, &[2361039]),
    // Java: this.put(16385, new int[]{2049074});
    (16385, &[2049074]),
    // Java: this.put(1082, new int[]{2361039});
    (1082, &[2361039]),
    // Java: this.put(1040, new int[]{2361039});
    (1040, &[2361039]),
    // Java: this.put(2067, new int[]{2361039});
    (2067, &[2361039]),
    // Java: this.put(15361, new int[]{2049074});
    (15361, &[2049074]),
    // Java: this.put(1032, new int[]{2227953});
    (1032, &[2227953]),
    // Java: this.put(1098, new int[]{2602970});
    (1098, &[2602970]),
    // Java: this.put(7177, new int[]{2361039});
    (7177, &[2361039]),
    // Java: this.put(1053, new int[]{2361039});
    (1053, &[2361039]),
    // Java: this.put(10241, new int[]{2049074});
    (10241, &[2049074]),
    // Java: this.put(3079, new int[]{2361039});
    (3079, &[2361039]),
    // Java: this.put(1026, new int[]{2115920});
    (1026, &[2115920]),
    // Java: this.put(1043, new int[]{2361039});
    (1043, &[2361039]),
    // Java: this.put(1062, new int[]{2361039});
    (1062, &[2361039]),
    // Java: this.put(1063, new int[]{2361039});
    (1063, &[2361039]),
    // Java: this.put(14337, new int[]{2049074});
    (14337, &[2049074]),
    // Java: this.put(18442, new int[]{2361039});
    (18442, &[2361039]),
    // Java: this.put(8202, new int[]{2361039});
    (8202, &[2361039]),
    // Java: this.put(4100, new int[]{2241684, 2241694});
    (4100, &[2241684, 2241694]),
    // Java: this.put(1055, new int[]{2361039});
    (1055, &[2361039]),
];

impl ThemeFontTool {
    // java public static String toFontsTheme(String fontTheme, TextDocument doc)
    pub fn toFontsTheme(fontTheme: &str, doc: &mut TextDocument) -> Option<String> {
        // java IOAssert.assertNotNull("fontTheme should not be null!", fontTheme);
        // java int langType = 0; int themeFontPropID = 571; boolean isMajor = false;
        // java switch (fontTheme.hashCode())
        let (isMajor, langType, themeFontPropID) = match java_string_hash_code(fontTheme) {
            // java case -422987968: isMajor = true; langType = 1; themeFontPropID = 572;
            -422987968 => (true, 1, 572),
            // java case -359287528: case -354301550: isMajor = true; langType = 0; themeFontPropID = 571;
            -359287528 | -354301550 => (true, 0, 571),
            // java case 681166949: isMajor = true; langType = 2; themeFontPropID = 573;
            681166949 => (true, 2, 573),
            // java case 742252700: case 747238678: isMajor = false; langType = 0; themeFontPropID = 571;
            742252700 | 747238678 => (false, 0, 571),
            // java case 993795169: isMajor = false; langType = 2; themeFontPropID = 573;
            993795169 => (false, 2, 573),
            // java case 2011802940: isMajor = false; langType = 1; themeFontPropID = 572;
            2011802940 => (false, 1, 572),
            // java default: IOAssert.fail("It should not reach here!"); return null;
            _ => panic!("unsupported theme font: {fontTheme}"),
        };
        // java PropertySet docProp = doc.getProperty();
        let docProp = doc.getProperty();
        // java int themeFontLID = getThemeFontLID(docProp, themeFontPropID);
        let themeFontLID = Self::getThemeFontLID(&docProp, themeFontPropID);
        // java if (1024 != themeFontLID) { String fontName = getThemeFontNameByLanguageID(doc, isMajor, themeFontLID); if (null != fontName) return fontName; }
        if themeFontLID != 1024 {
            if let Some(fontName) = Self::getThemeFontNameByLanguageID(doc, isMajor, themeFontLID) {
                return Some(fontName);
            }
        }
        // java return getThemeDefaultFont(doc, isMajor, langType);
        Self::getThemeDefaultFont(doc, isMajor, langType)
    }

    // java private static String getThemeFontNameByLanguageID(TextDocument textDoc, boolean isMajor, int lid)
    fn getThemeFontNameByLanguageID(textDoc: &mut TextDocument, isMajor: bool, lid: i32) -> Option<String> {
        // java FontCollection fontCollection = getFontCollection(textDoc, isMajor);
        let fontCollection = Self::getFontCollection(textDoc, isMajor)?;
        // java ArrayList<SupplementalFont> fonts = fontCollection.getSupplementalFont();
        let fonts = fontCollection.getSupplementalFont()?;
        // java int[] scriptLanguages = (int[])SCRIPT_LANGUAGES_MAP.get(lid);
        let scriptLanguages = SCRIPT_LANGUAGES_MAP.iter().find(|(key, _)| *key == lid)?.1;
        // java for each scriptLanguage, for each font: if (script == scriptLanguage) { addFontByName(fontTable, typeface); return typeface; }
        for scriptLanguage in scriptLanguages.iter() {
            for font in &fonts {
                if font.getScript() == *scriptLanguage {
                    let typeface = font.getTypeface()?;
                    let fontTable = textDoc.getFontTable_mut().expect("fontTable should not be null!");
                    Self::addFontByName(fontTable, &typeface);
                    return Some(typeface);
                }
            }
        }
        // java return null;
        None
    }

    // java private static FontCollection getFontCollection(TextDocument textDoc, boolean isMajor)
    fn getFontCollection(textDoc: &mut TextDocument, isMajor: bool) -> Option<FontCollection> {
        // java Theme theme = textDoc.getTheme(); if (null == theme) return null;
        let theme = textDoc.getTheme_mut()?;
        // java ThemeFontScheme themeFontScheme = theme.getThemeFontScheme();
        let scheme = unsafe { &mut *(*theme).getThemeFontScheme_mut() };
        // java return isMajor ? themeFontScheme.getMajorFont() : themeFontScheme.getMinorFont();
        if isMajor { scheme.getMajorFont() } else { scheme.getMinorFont() }
    }

    // java private static String getThemeDefaultFont(TextDocument textDoc, boolean isMajor, int langType)
    fn getThemeDefaultFont(textDoc: &mut TextDocument, isMajor: bool, langType: i32) -> Option<String> {
        // java FontCollection fontCollection = getFontCollection(textDoc, isMajor); if (null == fontCollection) return null;
        let fontCollection = Self::getFontCollection(textDoc, isMajor)?;
        // java FontTable fontTable = textDoc.getFontTable();
        let fontTable = textDoc.getFontTable_mut().expect("fontTable should not be null!");
        // java if (0 == langType) { ret = fontCollection.getLatinFont(); ... return fontName != null ? fontName : "Times New Roman"; }
        if langType == 0 {
            let ret = fontCollection.getLatinFont();
            return Some(Self::addFontByFontInfo(fontTable, ret.as_ref())
                .unwrap_or_else(|| "Times New Roman".to_string()));
        }
        // java if (1 == langType) ret = fontCollection.getEastAsian(); else if (2 == langType) ret = fontCollection.getComplexScript();
        let ret = if langType == 1 { fontCollection.getEastAsian() } else { fontCollection.getComplexScript() };
        // java return fontName != null ? fontName : "宋体";
        Some(Self::addFontByFontInfo(fontTable, ret.as_ref()).unwrap_or_else(|| "宋体".to_string()))
    }

    // java private static int getThemeFontLID(PropertySet prop, int propID)
    fn getThemeFontLID(prop: &PropertySet, propID: i32) -> i32 {
        // java String languageString = prop.getString(propID, (String)null);
        if prop.contains(propID) {
            let languageString = prop.getString(propID);
            // java Integer lid = (Integer)LID.BCP2LIDMAP.get(languageString.hashCode()); if (null != lid) return lid;
            if let Some(lid) = BCP2LIDMAP.get(&java_string_hash_code(&languageString)) {
                return *lid;
            }
        }
        // java return 1024;
        1024
    }

    // java private static String addFontByFontInfo(FontTable fontTable, FontInfo themeFontInfo)
    fn addFontByFontInfo(fontTable: &mut crate::cn::wps::moffice::writer::core::FontTable::FontTable, themeFontInfo: Option<&FontInfo>) -> Option<String> {
        // java if (null == themeFontInfo) return null;
        let themeFontInfo = themeFontInfo?;
        // java String name = themeFontInfo.getFontName(); if (null != name && name.length() > 0)
        let name = themeFontInfo.getFontName();
        if name.is_empty() { return None; }
        // java FontInfo fontInfo = fontTable.get(name); if (null == fontInfo) fontTable.add(themeFontInfo);
        if fontTable.get__string_opt(&name).is_none() { fontTable.add__fontinfo(themeFontInfo); }
        // java return name;
        Some(name)
    }

    // java private static void addFontByName(FontTable fontTable, String typeface)
    fn addFontByName(fontTable: &mut crate::cn::wps::moffice::writer::core::FontTable::FontTable, typeface: &str) {
        // java FontInfo fontInfo = fontTable.get(typeface); if (null == fontInfo)
        if fontTable.get__string_opt(typeface).is_none() {
            // java FontInfo themeFontInfo = new FontInfo(); themeFontInfo.setFontName(typeface);
            let mut themeFontInfo = FontInfo::default();
            themeFontInfo.setFontName(typeface.to_string());
            // java themeFontInfo.setFontSignature(timesNewRomanSig); fontTable.add(themeFontInfo);
            themeFontInfo.setFontSignature(vec![-1, 42, 0, -32, 65, 120, 0, -64, 9, 0, 0, 0, 0, 0, 0, 0, -1, 1, 0, 0, 0, 0, 0, 0]);
            fontTable.add__fontinfo(&themeFontInfo);
        }
    }
}
