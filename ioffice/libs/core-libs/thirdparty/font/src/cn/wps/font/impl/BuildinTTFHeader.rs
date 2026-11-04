// Java: import cn.wps.font.FreeTypeJNI
use crate::cn::wps::font::FreeTypeJNI::TTFHeader;
// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::{OnceLock, Mutex};

// Java field: int FONT_NONE
const FONT_NONE: i32 = 0;
// Java field: int FONT_TIMES
const FONT_TIMES: i32 = 1;
// Java field: int FONT_CALIBRI
const FONT_CALIBRI: i32 = 2;
// Java field: int FONT_TIMESBD
const FONT_TIMESBD: i32 = 3;
// Java field: int FONT_TIMESBI
const FONT_TIMESBI: i32 = 4;
// Java field: int FONT_TIMESI
const FONT_TIMESI: i32 = 5;
// Java field: int FONT_CALIBRIB
const FONT_CALIBRIB: i32 = 6;
// Java field: int FONT_CALIBRII
const FONT_CALIBRII: i32 = 7;
// Java field: int FONT_CALIBRIZ
const FONT_CALIBRIZ: i32 = 8;
// Java field: int FONT_ARIALN
const FONT_ARIALN: i32 = 9;
// Java field: int FONT_ARIALNB
const FONT_ARIALNB: i32 = 10;
// Java field: int FONT_ARIALNBI
const FONT_ARIALNBI: i32 = 11;
// Java field: int FONT_ARIALNI
const FONT_ARIALNI: i32 = 12;
// Java field: int FONT_ARIALUNI
const FONT_ARIALUNI: i32 = 13;
// Java field: int FONT_ARIAL
const FONT_ARIAL: i32 = 14;
// Java field: int FONT_ARIALBD
const FONT_ARIALBD: i32 = 15;
// Java field: int FONT_ARIALBI
const FONT_ARIALBI: i32 = 16;
// Java field: int FONT_ARIALI
const FONT_ARIALI: i32 = 17;
// Java field: int FONT_SIMSUN
const FONT_SIMSUN: i32 = 18;
// Java field: int FONT_SIMFANG
const FONT_SIMFANG: i32 = 19;
// Java field: int FONT_SIMHEI
const FONT_SIMHEI: i32 = 20;
// Java field: int FONT_SIMKAI
const FONT_SIMKAI: i32 = 21;
// Java field: int FONT_FZXBS
const FONT_FZXBS: i32 = 22;
// Java field: int FONT_SIMLI
const FONT_SIMLI: i32 = 23;
// Java field: int FONT_MS_YAHEI
const FONT_MS_YAHEI: i32 = 24;
// Java field: int FONT_SIMYOU
const FONT_SIMYOU: i32 = 25;
// Java field: int FONT_SIXINGKA
const FONT_SIXINGKA: i32 = 26;
// Java field: int FONT_GARAMON
const FONT_GARAMON: i32 = 27;
// Java field: int FONT_GARAMONBD
const FONT_GARAMONBD: i32 = 28;
// Java field: int FONT_GARAMONI
const FONT_GARAMONI: i32 = 29;
// Java field: int FONT_CAMBRIA
const FONT_CAMBRIA: i32 = 30;
// Java field: int FONT_CAMBRIABD
const FONT_CAMBRIABD: i32 = 31;
// Java field: int FONT_CAMBRIAI
const FONT_CAMBRIAI: i32 = 32;
// Java field: int FONT_CAMBRIAZ
const FONT_CAMBRIAZ: i32 = 33;
// Java field: int FONT_COURIERNEW
const FONT_COURIERNEW: i32 = 34;
// Java field: int FONT_COURIERNEWBD
const FONT_COURIERNEWBD: i32 = 35;
// Java field: int FONT_COURIERNEWI
const FONT_COURIERNEWI: i32 = 36;
// Java field: int FONT_COURIERNEWBI
const FONT_COURIERNEWBI: i32 = 37;
// Java field: int FONT_VERDANA
const FONT_VERDANA: i32 = 38;
// Java field: int FONT_VERDANABD
const FONT_VERDANABD: i32 = 39;
// Java field: int FONT_VERDANAI
const FONT_VERDANAI: i32 = 40;
// Java field: int FONT_VERDANAZ
const FONT_VERDANAZ: i32 = 41;
// Java field: int FONT_TAHOMA
const FONT_TAHOMA: i32 = 42;
// Java field: int FONT_TAHOMABD
const FONT_TAHOMABD: i32 = 43;
// Java field: int FONT_COMIC_SANS_MS
const FONT_COMIC_SANS_MS: i32 = 44;
// Java field: int FONT_COMIC_SANS_MSBD
const FONT_COMIC_SANS_MSBD: i32 = 45;
// Java field: int FONT_COMIC_SANS_MSI
const FONT_COMIC_SANS_MSI: i32 = 46;
// Java field: int FONT_COMIC_SANS_MSZ
const FONT_COMIC_SANS_MSZ: i32 = 47;
// Java field: int FONT_THAI_SANS
const FONT_THAI_SANS: i32 = 48;
// Java field: int FONT_THAI_SERIF
const FONT_THAI_SERIF: i32 = 49;
// Java field: HashMap<String, int[]> msNameMap
static MS_NAME_MAP: OnceLock<Mutex<HashMap<String, Vec<i32>>>> = OnceLock::new();

fn get_ms_name_map() -> &'static Mutex<HashMap<String, Vec<i32>>> {
    MS_NAME_MAP.get_or_init(|| {
        let mut map = HashMap::new();
        put_font_init(&mut map, &[1,3,5,4], &["TIMES NEW ROMAN"]);
        put_font_init(&mut map, &[2,6,7,8], &["CALIBRI"]);
        put_font_init(&mut map, &[9,10,12,11], &["ARIAL NARROW"]);
        put_font_init(&mut map, &[13,13,13,13], &["ARIAL UNICODE MS"]);
        put_font_init(&mut map, &[14,15,17,16], &["ARIAL"]);
        put_font_init(&mut map, &[18,18,18,18], &["\u{5B8B}\u{4F53}", "SIMSUN"]);
        put_font_init(&mut map, &[19,19,19,19], &["\u{4EFF}\u{5B8B}", "\u{4EFF}\u{5B8B}_GB2312", "FANGSONG"]);
        put_font_init(&mut map, &[20,20,20,20], &["\u{9ED1}\u{4F53}"]);
        put_font_init(&mut map, &[23,23,23,23], &["\u{96B6}\u{4E66}"]);
        put_font_init(&mut map, &[21,21,21,21], &["\u{6977}\u{4F53}", "\u{6977}\u{4F53}_GB2312", "KAITI"]);
        put_font_init(&mut map, &[22,22,22,22], &["\u{65B9}\u{6B63}\u{5C0F}\u{6807}\u{5B8B}", "\u{65B9}\u{6B63}\u{5C0F}\u{6807}\u{5B8B}_GBK"]);
        put_font_init(&mut map, &[24,24,24,24], &["\u{5FAE}\u{8F6F}\u{96C5}\u{9ED1}"]);
        put_font_init(&mut map, &[25,25,25,25], &["\u{5E7C}\u{5706}"]);
        put_font_init(&mut map, &[26,26,26,26], &["\u{534E}\u{6587}\u{884C}\u{6977}"]);
        put_font_init(&mut map, &[27,28,29,29], &["GARAMOND"]);
        put_font_init(&mut map, &[30,31,32,33], &["CAMBRIA"]);
        put_font_init(&mut map, &[34,35,36,37], &["COURIER NEW"]);
        put_font_init(&mut map, &[38,39,40,41], &["VERDANA"]);
        put_font_init(&mut map, &[42,43,42,42], &["TAHOMA"]);
        put_font_init(&mut map, &[44,45,46,47], &["COMIC SANS MS"]);
        put_font_init(&mut map, &[48,48,48,48], &["NOTO SANS THAI"]);
        put_font_init(&mut map, &[49,49,49,49], &["NOTO SERIF THAI"]);
        Mutex::new(map)
    })
}

fn put_font_init(map: &mut HashMap<String, Vec<i32>>, styles: &[i32], names: &[&str]) {
    for name in names {
        map.insert(name.to_uppercase(), styles.to_vec());
    }
}

// Java: public class BuildinTTFHeader
pub struct BuildinTTFHeader {}

// Java: public class BuildinTTFHeader
impl BuildinTTFHeader {
    // Java: BuildinTTFHeader()
    pub fn new() -> Self {
        // Java: super();
        BuildinTTFHeader {}
    }

    // Java: static void putFont(int[] styles, String... alisNames)
    fn putFont(styles: &[i32], alis_names: &[&str]) {
        // Java: for(int i = 0; i < alisNames.length; ++i)
        let map = get_ms_name_map();
        let mut guard = map.lock().unwrap();
        for name in alis_names {
            // Java: msNameMap.put(alisNames[i], styles);
            guard.insert(name.to_uppercase(), styles.to_vec());
        }
    }

    // Java: static int[] getFont(String fontname)
    pub fn getFont(fontname: &str) -> Option<Vec<i32>> {
        // Java: if (fontname == null)
        if fontname.is_empty() {
            // Java: return null;
            return None;
        }
        // Java: else
        {
            // Java: String searchName = fontname.toUpperCase();
            let search_name = fontname.to_uppercase();
            // Java: return (int[])msNameMap.get(searchName);
            let map = get_ms_name_map().lock().unwrap();
            map.get(&search_name).cloned()
        }
    }

    // Java: static int getFontType(String fontname, int fontStyle)
    fn getFontType(fontname: &str, font_style: i32) -> i32 {
        // Java: if (fontStyle >= 0 && fontStyle < 4)
        if font_style >= 0 && font_style < 4 {
            // Java: int[] fontNames = getFont(fontname);
            let font_names = Self::getFont(fontname);
            // Java: if (null != fontNames)
            if let Some(names) = font_names {
                // Java: return fontNames[fontStyle];
                return names[font_style as usize];
            }
        }
        // Java: return 0;
        0
    }

    // Java: static FreeTypeJNI.TTFHeader getBuildinTTFHeader(String fontname, int fontStyle)
    pub fn getBuildinTTFHeader(fontname: &str, font_style: i32) -> Option<TTFHeader> {
        let mut ttf_header = TTFHeader::new();
        let font_type = Self::getFontType(fontname, font_style);
        if font_type == 0 {
            return None;
        }
        fill_ttf_header(&mut ttf_header, font_type);
        Some(ttf_header)
    }
}

fn fill_ttf_header(h: &mut TTFHeader, font_type: i32) {
    match font_type {
        1 => { h.codePageRange1=1073742335; h.codePageRange2=-65536; h.unicodeRange1=-536858881; h.unicodeRange2=-1073711013; h.unicodeRange3=9; h.unicodeRange4=0; h.underline_position=-273; h.underline_thickness=100; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1825.0; h.winDescent=443.0; h.tmAscent=1825.0; h.tmDescent=443.0; h.lineGap=87.0; h.avgCharWidth=821.0; h.subscriptYOffset=293.0; h.subscriptYSize=1331.0; h.superscriptYOffset=928.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        2 => { h.codePageRange1=536871327; h.codePageRange2=0; h.unicodeRange1=-536859905; h.unicodeRange2=-1073732485; h.unicodeRange3=9; h.unicodeRange4=0; h.underline_position=-299; h.underline_thickness=134; h.yStrikeoutPosition=512; h.yStrikeoutSize=134; h.winAscent=1950.0; h.winDescent=550.0; h.tmAscent=1536.0; h.tmDescent=512.0; h.lineGap=452.0; h.avgCharWidth=1067.0; h.subscriptYOffset=286.0; h.subscriptYSize=1331.0; h.superscriptYOffset=976.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        3 => { h.codePageRange1=1073742335; h.codePageRange2=-65536; h.unicodeRange1=-536858881; h.unicodeRange2=-1073711013; h.unicodeRange3=9; h.unicodeRange4=0; h.underline_position=-320; h.underline_thickness=195; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1825.0; h.winDescent=443.0; h.tmAscent=1825.0; h.tmDescent=443.0; h.lineGap=87.0; h.avgCharWidth=874.0; h.subscriptYOffset=293.0; h.subscriptYSize=1331.0; h.superscriptYOffset=928.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        4 => { h.codePageRange1=1073742271; h.codePageRange2=-537460736; h.unicodeRange1=-536867073; h.unicodeRange2=1073772635; h.unicodeRange3=1; h.unicodeRange4=0; h.underline_position=-320; h.underline_thickness=195; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1825.0; h.winDescent=443.0; h.tmAscent=1825.0; h.tmDescent=443.0; h.lineGap=87.0; h.avgCharWidth=844.0; h.subscriptYOffset=293.0; h.subscriptYSize=1331.0; h.superscriptYOffset=928.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        5 => { h.codePageRange1=1073742271; h.codePageRange2=-537460736; h.unicodeRange1=-536867073; h.unicodeRange2=1073772635; h.unicodeRange3=1; h.unicodeRange4=0; h.underline_position=-273; h.underline_thickness=100; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1825.0; h.winDescent=443.0; h.tmAscent=1825.0; h.tmDescent=443.0; h.lineGap=87.0; h.avgCharWidth=823.0; h.subscriptYOffset=293.0; h.subscriptYSize=1331.0; h.superscriptYOffset=928.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        6 => { h.codePageRange1=536871327; h.codePageRange2=0; h.unicodeRange1=-536859905; h.unicodeRange2=-1073732485; h.unicodeRange3=9; h.unicodeRange4=0; h.underline_position=-299; h.underline_thickness=134; h.yStrikeoutPosition=512; h.yStrikeoutSize=186; h.winAscent=1950.0; h.winDescent=550.0; h.tmAscent=1536.0; h.tmDescent=512.0; h.lineGap=452.0; h.avgCharWidth=1098.0; h.subscriptYOffset=286.0; h.subscriptYSize=1331.0; h.superscriptYOffset=976.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        7 => { h.codePageRange1=536871327; h.codePageRange2=0; h.unicodeRange1=-536868097; h.unicodeRange2=1073751163; h.unicodeRange3=1; h.unicodeRange4=0; h.underline_position=-299; h.underline_thickness=134; h.yStrikeoutPosition=512; h.yStrikeoutSize=134; h.winAscent=1950.0; h.winDescent=550.0; h.tmAscent=1536.0; h.tmDescent=512.0; h.lineGap=452.0; h.avgCharWidth=1067.0; h.subscriptYOffset=286.0; h.subscriptYSize=1331.0; h.superscriptYOffset=976.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        8 => { h.codePageRange1=536871327; h.codePageRange2=0; h.unicodeRange1=-536868097; h.unicodeRange2=1073751163; h.unicodeRange3=1; h.unicodeRange4=0; h.underline_position=-299; h.underline_thickness=134; h.yStrikeoutPosition=512; h.yStrikeoutSize=186; h.winAscent=1950.0; h.winDescent=550.0; h.tmAscent=1536.0; h.tmDescent=512.0; h.lineGap=452.0; h.avgCharWidth=1098.0; h.subscriptYOffset=286.0; h.subscriptYSize=1331.0; h.superscriptYOffset=976.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        9 => { h.codePageRange1=536871071; h.codePageRange2=-539557888; h.unicodeRange1=647; h.unicodeRange2=2048; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-292; h.underline_thickness=150; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1888.0; h.winDescent=431.0; h.tmAscent=1916.0; h.tmDescent=434.0; h.lineGap=0.0; h.avgCharWidth=741.0; h.subscriptYOffset=147.0; h.subscriptYSize=1331.0; h.superscriptYOffset=523.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        10 => { h.codePageRange1=536871071; h.codePageRange2=-539557888; h.unicodeRange1=647; h.unicodeRange2=2048; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-292; h.underline_thickness=150; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1910.0; h.winDescent=431.0; h.tmAscent=1916.0; h.tmDescent=434.0; h.lineGap=0.0; h.avgCharWidth=803.0; h.subscriptYOffset=147.0; h.subscriptYSize=1331.0; h.superscriptYOffset=523.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        11 => { h.codePageRange1=536871071; h.codePageRange2=-539557888; h.unicodeRange1=647; h.unicodeRange2=2048; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-324; h.underline_thickness=215; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1916.0; h.winDescent=434.0; h.tmAscent=1916.0; h.tmDescent=434.0; h.lineGap=0.0; h.avgCharWidth=803.0; h.subscriptYOffset=147.0; h.subscriptYSize=1331.0; h.superscriptYOffset=523.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        12 => { h.codePageRange1=536871071; h.codePageRange2=-539557888; h.unicodeRange1=647; h.unicodeRange2=2048; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-292; h.underline_thickness=150; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1873.0; h.winDescent=431.0; h.tmAscent=1916.0; h.tmDescent=434.0; h.lineGap=0.0; h.avgCharWidth=741.0; h.subscriptYOffset=147.0; h.subscriptYSize=1331.0; h.superscriptYOffset=523.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        13 => { h.codePageRange1=1614742015; h.codePageRange2=-65536; h.unicodeRange1=-1; h.unicodeRange2=-369098753; h.unicodeRange3=63; h.unicodeRange4=0; h.underline_position=-256; h.underline_thickness=102; h.yStrikeoutPosition=632; h.yStrikeoutSize=102; h.winAscent=2189.0; h.winDescent=555.0; h.tmAscent=2189.0; h.tmDescent=555.0; h.lineGap=0.0; h.avgCharWidth=904.0; h.subscriptYOffset=151.0; h.subscriptYSize=1331.0; h.superscriptYOffset=522.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        14 => { h.codePageRange1=1073742335; h.codePageRange2=-65536; h.unicodeRange1=-536858881; h.unicodeRange2=-1073711037; h.unicodeRange3=9; h.unicodeRange4=0; h.underline_position=-292; h.underline_thickness=150; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1854.0; h.winDescent=434.0; h.tmAscent=1854.0; h.tmDescent=434.0; h.lineGap=67.0; h.avgCharWidth=904.0; h.subscriptYOffset=283.0; h.subscriptYSize=1331.0; h.superscriptYOffset=977.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        15 => { h.codePageRange1=1073742335; h.codePageRange2=-65536; h.unicodeRange1=-536858881; h.unicodeRange2=-1073711037; h.unicodeRange3=9; h.unicodeRange4=0; h.underline_position=-324; h.underline_thickness=215; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1854.0; h.winDescent=434.0; h.tmAscent=1854.0; h.tmDescent=434.0; h.lineGap=67.0; h.avgCharWidth=980.0; h.subscriptYOffset=283.0; h.subscriptYSize=1331.0; h.superscriptYOffset=977.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        16 => { h.codePageRange1=1073742271; h.codePageRange2=-537460736; h.unicodeRange1=-536867073; h.unicodeRange2=30787; h.unicodeRange3=1; h.unicodeRange4=0; h.underline_position=-324; h.underline_thickness=215; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1854.0; h.winDescent=434.0; h.tmAscent=1854.0; h.tmDescent=434.0; h.lineGap=67.0; h.avgCharWidth=980.0; h.subscriptYOffset=283.0; h.subscriptYSize=1331.0; h.superscriptYOffset=977.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        17 => { h.codePageRange1=1073742271; h.codePageRange2=-537460736; h.unicodeRange1=-536867073; h.unicodeRange2=30787; h.unicodeRange3=1; h.unicodeRange4=0; h.underline_position=-292; h.underline_thickness=150; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1854.0; h.winDescent=434.0; h.tmAscent=1854.0; h.tmDescent=434.0; h.lineGap=67.0; h.avgCharWidth=904.0; h.subscriptYOffset=283.0; h.subscriptYSize=1331.0; h.superscriptYOffset=977.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        18 => { h.codePageRange1=262145; h.codePageRange2=0; h.unicodeRange1=3; h.unicodeRange2=680460288; h.unicodeRange3=6; h.unicodeRange4=0; h.underline_position=-28; h.underline_thickness=12; h.yStrikeoutPosition=65; h.yStrikeoutSize=12; h.winAscent=220.0; h.winDescent=36.0; h.tmAscent=220.0; h.tmDescent=36.0; h.lineGap=36.0; h.avgCharWidth=128.0; h.subscriptYOffset=16.0; h.subscriptYSize=128.0; h.superscriptYOffset=128.0; h.superscriptYSize=128.0; h.unitsPerEM=256.0; }
        19..=21 => { h.codePageRange1=262145; h.codePageRange2=0; h.unicodeRange1=-2147482945; h.unicodeRange2=953122042; h.unicodeRange3=22; h.unicodeRange4=0; h.underline_position=-28; h.underline_thickness=12; h.yStrikeoutPosition=65; h.yStrikeoutSize=12; h.winAscent=220.0; h.winDescent=36.0; h.tmAscent=220.0; h.tmDescent=36.0; h.lineGap=36.0; h.avgCharWidth=128.0; h.subscriptYOffset=16.0; h.subscriptYSize=128.0; h.superscriptYOffset=128.0; h.superscriptYSize=128.0; h.unitsPerEM=256.0; }
        22 => { h.codePageRange1=262144; h.codePageRange2=0; h.unicodeRange1=1; h.unicodeRange2=134217728; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-70; h.underline_thickness=12; h.yStrikeoutPosition=64; h.yStrikeoutSize=11; h.winAscent=222.0; h.winDescent=76.0; h.tmAscent=222.0; h.tmDescent=76.0; h.lineGap=0.0; h.avgCharWidth=128.0; h.subscriptYOffset=16.0; h.subscriptYSize=128.0; h.superscriptYOffset=128.0; h.superscriptYSize=128.0; h.unitsPerEM=256.0; }
        23 => { h.codePageRange1=262144; h.codePageRange2=0; h.unicodeRange1=1; h.unicodeRange2=135135232; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-65; h.underline_thickness=13; h.yStrikeoutPosition=65; h.yStrikeoutSize=12; h.winAscent=220.0; h.winDescent=36.0; h.tmAscent=220.0; h.tmDescent=36.0; h.lineGap=36.0; h.avgCharWidth=128.0; h.subscriptYOffset=16.0; h.subscriptYSize=128.0; h.superscriptYOffset=128.0; h.superscriptYSize=128.0; h.unitsPerEM=256.0; }
        24 => { h.codePageRange1=262175; h.codePageRange2=0; h.unicodeRange1=-2147483001; h.unicodeRange2=684670032; h.unicodeRange3=22; h.unicodeRange4=0; h.underline_position=-242; h.underline_thickness=129; h.yStrikeoutPosition=655; h.yStrikeoutSize=102; h.winAscent=2167.0; h.winDescent=536.0; h.tmAscent=2167.0; h.tmDescent=536.0; h.lineGap=0.0; h.avgCharWidth=987.0; h.subscriptYOffset=182.0; h.subscriptYSize=1331.0; h.superscriptYOffset=581.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        25 => { h.codePageRange1=262144; h.codePageRange2=0; h.unicodeRange1=1; h.unicodeRange2=135135232; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-28; h.underline_thickness=13; h.yStrikeoutPosition=65; h.yStrikeoutSize=12; h.winAscent=220.0; h.winDescent=36.0; h.tmAscent=210.0; h.tmDescent=46.0; h.lineGap=26.0; h.avgCharWidth=128.0; h.subscriptYOffset=16.0; h.subscriptYSize=128.0; h.superscriptYOffset=128.0; h.superscriptYSize=128.0; h.unitsPerEM=256.0; }
        26 => { h.codePageRange1=262144; h.codePageRange2=0; h.unicodeRange1=1; h.unicodeRange2=135200768; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-143; h.underline_thickness=20; h.yStrikeoutPosition=250; h.yStrikeoutSize=100; h.winAscent=761.0; h.winDescent=289.0; h.tmAscent=802.0; h.tmDescent=289.0; h.lineGap=0.0; h.avgCharWidth=313.0; h.subscriptYOffset=150.0; h.subscriptYSize=100.0; h.superscriptYOffset=650.0; h.superscriptYSize=100.0; h.unitsPerEM=1000.0; }
        27 => { h.codePageRange1=159; h.codePageRange2=-539557888; h.unicodeRange1=647; h.unicodeRange2=0; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-205; h.underline_thickness=102; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1765.0; h.winDescent=539.0; h.tmAscent=1765.0; h.tmDescent=539.0; h.lineGap=0.0; h.avgCharWidth=793.0; h.subscriptYOffset=189.0; h.subscriptYSize=1331.0; h.superscriptYOffset=469.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        28 => { h.codePageRange1=159; h.codePageRange2=-539557888; h.unicodeRange1=647; h.unicodeRange2=0; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-205; h.underline_thickness=102; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1765.0; h.winDescent=539.0; h.tmAscent=1765.0; h.tmDescent=539.0; h.lineGap=0.0; h.avgCharWidth=865.0; h.subscriptYOffset=179.0; h.subscriptYSize=1331.0; h.superscriptYOffset=465.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        29 => { h.codePageRange1=536871071; h.codePageRange2=-539557888; h.unicodeRange1=647; h.unicodeRange2=0; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-205; h.underline_thickness=102; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1765.0; h.winDescent=539.0; h.tmAscent=1765.0; h.tmDescent=539.0; h.lineGap=0.0; h.avgCharWidth=664.0; h.subscriptYOffset=176.0; h.subscriptYSize=1331.0; h.superscriptYOffset=460.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        30 => { h.codePageRange1=536871327; h.codePageRange2=0; h.unicodeRange1=-536870145; h.unicodeRange2=1073743103; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-183; h.underline_thickness=116; h.yStrikeoutPosition=510; h.yStrikeoutSize=102; h.winAscent=1946.0; h.winDescent=455.0; h.tmAscent=1595.0; h.tmDescent=455.0; h.lineGap=353.0; h.avgCharWidth=1260.0; h.subscriptYOffset=156.0; h.subscriptYSize=1331.0; h.superscriptYOffset=503.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        31 => { h.codePageRange1=536871327; h.codePageRange2=0; h.unicodeRange1=-536870145; h.unicodeRange2=1073742943; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-183; h.underline_thickness=156; h.yStrikeoutPosition=510; h.yStrikeoutSize=102; h.winAscent=1946.0; h.winDescent=455.0; h.tmAscent=1946.0; h.tmDescent=455.0; h.lineGap=0.0; h.avgCharWidth=1228.0; h.subscriptYOffset=156.0; h.subscriptYSize=1331.0; h.superscriptYOffset=503.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        32 => { h.codePageRange1=536871327; h.codePageRange2=0; h.unicodeRange1=-536870145; h.unicodeRange2=1073742943; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-183; h.underline_thickness=116; h.yStrikeoutPosition=510; h.yStrikeoutSize=102; h.winAscent=1946.0; h.winDescent=455.0; h.tmAscent=1946.0; h.tmDescent=455.0; h.lineGap=0.0; h.avgCharWidth=1111.0; h.subscriptYOffset=156.0; h.subscriptYSize=1331.0; h.superscriptYOffset=503.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        33 => { h.codePageRange1=536871327; h.codePageRange2=0; h.unicodeRange1=-536870145; h.unicodeRange2=1073742943; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-183; h.underline_thickness=156; h.yStrikeoutPosition=510; h.yStrikeoutSize=102; h.winAscent=1946.0; h.winDescent=455.0; h.tmAscent=1946.0; h.tmDescent=455.0; h.lineGap=0.0; h.avgCharWidth=1198.0; h.subscriptYOffset=156.0; h.subscriptYSize=1331.0; h.superscriptYOffset=503.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        34 => { h.codePageRange1=1073742335; h.codePageRange2=-65536; h.unicodeRange1=-536858881; h.unicodeRange2=-1073711037; h.unicodeRange3=9; h.unicodeRange4=0; h.underline_position=-519; h.underline_thickness=84; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1705.0; h.winDescent=615.0; h.tmAscent=1705.0; h.tmDescent=615.0; h.lineGap=0.0; h.avgCharWidth=1229.0; h.subscriptYOffset=285.0; h.subscriptYSize=1331.0; h.superscriptYOffset=865.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        35 => { h.codePageRange1=1073742335; h.codePageRange2=-65536; h.unicodeRange1=-536858881; h.unicodeRange2=-1073711037; h.unicodeRange3=9; h.unicodeRange4=0; h.underline_position=-579; h.underline_thickness=205; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1705.0; h.winDescent=615.0; h.tmAscent=1705.0; h.tmDescent=615.0; h.lineGap=0.0; h.avgCharWidth=1229.0; h.subscriptYOffset=285.0; h.subscriptYSize=1331.0; h.superscriptYOffset=865.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        36 => { h.codePageRange1=1073742271; h.codePageRange2=-537460736; h.unicodeRange1=-536867073; h.unicodeRange2=1073772611; h.unicodeRange3=1; h.unicodeRange4=0; h.underline_position=-519; h.underline_thickness=84; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1705.0; h.winDescent=615.0; h.tmAscent=1705.0; h.tmDescent=615.0; h.lineGap=0.0; h.avgCharWidth=1229.0; h.subscriptYOffset=285.0; h.subscriptYSize=1331.0; h.superscriptYOffset=865.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        37 => { h.codePageRange1=1073742271; h.codePageRange2=-537460736; h.unicodeRange1=-536867073; h.unicodeRange2=1073772611; h.unicodeRange3=1; h.unicodeRange4=0; h.underline_position=-579; h.underline_thickness=205; h.yStrikeoutPosition=530; h.yStrikeoutSize=102; h.winAscent=1705.0; h.winDescent=615.0; h.tmAscent=1705.0; h.tmDescent=615.0; h.lineGap=0.0; h.avgCharWidth=1229.0; h.subscriptYOffset=285.0; h.subscriptYSize=1331.0; h.superscriptYOffset=865.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        38 => { h.codePageRange1=536871327; h.codePageRange2=0; h.unicodeRange1=-1593833729; h.unicodeRange2=1073750107; h.unicodeRange3=16; h.unicodeRange4=0; h.underline_position=-240; h.underline_thickness=120; h.yStrikeoutPosition=679; h.yStrikeoutSize=120; h.winAscent=2059.0; h.winDescent=430.0; h.tmAscent=2059.0; h.tmDescent=430.0; h.lineGap=0.0; h.avgCharWidth=1041.0; h.subscriptYOffset=293.0; h.subscriptYSize=1331.0; h.superscriptYOffset=928.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        39 => { h.codePageRange1=536871327; h.codePageRange2=0; h.unicodeRange1=-1593833729; h.unicodeRange2=1073750107; h.unicodeRange3=16; h.unicodeRange4=0; h.underline_position=-244; h.underline_thickness=211; h.yStrikeoutPosition=773; h.yStrikeoutSize=211; h.winAscent=2059.0; h.winDescent=430.0; h.tmAscent=2059.0; h.tmDescent=430.0; h.lineGap=0.0; h.avgCharWidth=1163.0; h.subscriptYOffset=293.0; h.subscriptYSize=1331.0; h.superscriptYOffset=928.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        40 => { h.codePageRange1=536871327; h.codePageRange2=0; h.unicodeRange1=-1593833729; h.unicodeRange2=1073750107; h.unicodeRange3=16; h.unicodeRange4=0; h.underline_position=-240; h.underline_thickness=120; h.yStrikeoutPosition=679; h.yStrikeoutSize=120; h.winAscent=2059.0; h.winDescent=430.0; h.tmAscent=2059.0; h.tmDescent=430.0; h.lineGap=0.0; h.avgCharWidth=1041.0; h.subscriptYOffset=293.0; h.subscriptYSize=1331.0; h.superscriptYOffset=928.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        41 => { h.codePageRange1=536871327; h.codePageRange2=0; h.unicodeRange1=-1593833729; h.unicodeRange2=1073750107; h.unicodeRange3=16; h.unicodeRange4=0; h.underline_position=-244; h.underline_thickness=211; h.yStrikeoutPosition=773; h.yStrikeoutSize=211; h.winAscent=2059.0; h.winDescent=430.0; h.tmAscent=2059.0; h.tmDescent=430.0; h.lineGap=0.0; h.avgCharWidth=1163.0; h.subscriptYOffset=293.0; h.subscriptYSize=1331.0; h.superscriptYOffset=928.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        42 => { h.codePageRange1=536936959; h.codePageRange2=539492352; h.unicodeRange1=-520081665; h.unicodeRange2=-1073717157; h.unicodeRange3=41; h.unicodeRange4=0; h.underline_position=-235; h.underline_thickness=130; h.yStrikeoutPosition=689; h.yStrikeoutSize=130; h.winAscent=2049.0; h.winDescent=423.0; h.tmAscent=2049.0; h.tmDescent=423.0; h.lineGap=0.0; h.avgCharWidth=910.0; h.subscriptYOffset=293.0; h.subscriptYSize=1331.0; h.superscriptYOffset=928.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        43 => { h.codePageRange1=536936959; h.codePageRange2=539492352; h.unicodeRange1=-520081665; h.unicodeRange2=-1073717157; h.unicodeRange3=41; h.unicodeRange4=0; h.underline_position=-244; h.underline_thickness=201; h.yStrikeoutPosition=689; h.yStrikeoutSize=201; h.winAscent=2049.0; h.winDescent=423.0; h.tmAscent=2049.0; h.tmDescent=423.0; h.lineGap=0.0; h.avgCharWidth=1036.0; h.subscriptYOffset=293.0; h.subscriptYSize=1331.0; h.superscriptYOffset=928.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        44 => { h.codePageRange1=536871071; h.codePageRange2=0; h.unicodeRange1=647; h.unicodeRange2=19; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-359; h.underline_thickness=175; h.yStrikeoutPosition=630; h.yStrikeoutSize=175; h.winAscent=2257.0; h.winDescent=597.0; h.tmAscent=2257.0; h.tmDescent=597.0; h.lineGap=0.0; h.avgCharWidth=959.0; h.subscriptYOffset=293.0; h.subscriptYSize=1331.0; h.superscriptYOffset=928.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        45 => { h.codePageRange1=536871071; h.codePageRange2=0; h.unicodeRange1=647; h.unicodeRange2=19; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-359; h.underline_thickness=175; h.yStrikeoutPosition=630; h.yStrikeoutSize=175; h.winAscent=2257.0; h.winDescent=597.0; h.tmAscent=2257.0; h.tmDescent=597.0; h.lineGap=0.0; h.avgCharWidth=1014.0; h.subscriptYOffset=293.0; h.subscriptYSize=1331.0; h.superscriptYOffset=928.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        46 => { h.codePageRange1=536871071; h.codePageRange2=0; h.unicodeRange1=647; h.unicodeRange2=19; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-359; h.underline_thickness=175; h.yStrikeoutPosition=630; h.yStrikeoutSize=175; h.winAscent=2257.0; h.winDescent=597.0; h.tmAscent=2257.0; h.tmDescent=597.0; h.lineGap=0.0; h.avgCharWidth=1325.0; h.subscriptYOffset=293.0; h.subscriptYSize=1331.0; h.superscriptYOffset=928.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        47 => { h.codePageRange1=536871071; h.codePageRange2=0; h.unicodeRange1=647; h.unicodeRange2=19; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-359; h.underline_thickness=175; h.yStrikeoutPosition=630; h.yStrikeoutSize=175; h.winAscent=2257.0; h.winDescent=597.0; h.tmAscent=2257.0; h.tmDescent=597.0; h.lineGap=0.0; h.avgCharWidth=1343.0; h.subscriptYOffset=293.0; h.subscriptYSize=1331.0; h.superscriptYOffset=928.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        48 => { h.codePageRange1=1; h.codePageRange2=0; h.unicodeRange1=16777216; h.unicodeRange2=0; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-237; h.underline_thickness=119; h.yStrikeoutPosition=498; h.yStrikeoutSize=102; h.winAscent=2286.0; h.winDescent=524.0; h.tmAscent=2286.0; h.tmDescent=524.0; h.lineGap=0.0; h.avgCharWidth=1253.0; h.subscriptYOffset=286.0; h.subscriptYSize=1331.0; h.superscriptYOffset=976.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        49 => { h.codePageRange1=1; h.codePageRange2=0; h.unicodeRange1=16777216; h.unicodeRange2=0; h.unicodeRange3=0; h.unicodeRange4=0; h.underline_position=-205; h.underline_thickness=102; h.yStrikeoutPosition=749; h.yStrikeoutSize=102; h.winAscent=2286.0; h.winDescent=524.0; h.tmAscent=2286.0; h.tmDescent=524.0; h.lineGap=0.0; h.avgCharWidth=1238.0; h.subscriptYOffset=287.0; h.subscriptYSize=1331.0; h.superscriptYOffset=977.0; h.superscriptYSize=1331.0; h.unitsPerEM=2048.0; }
        _ => {}
    }
}
