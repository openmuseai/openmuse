// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.language.Thai
use util::language::Thai::*;
// Java: import java.util.Stack
use stubs::java::util::Stack::*;
use stubs::java::lang::StringBuffer::StringBuffer;
use super::NFFunDefine::NFFunDefine;
use super::NFStrDefine::NFStrDefine;
use super::NFStrDefine::*;
use super::NFIntDefine::*;
use std::sync::OnceLock;

fn jchar_at(s: &str, i: i32) -> char {
    s.chars().nth(i as usize).unwrap_or('\0')
}


// java private static CodePage instanceCodePage = null;
static instanceCodePage: OnceLock<CodePage> = OnceLock::new();
// java static final int ChineseZero = 0;
pub(crate) const ChineseZero: i32 = 0;
// java static final int ChineseTen = 10;
pub(crate) const ChineseTen: i32 = 10;
// java static final int ChineseHundred = 11;
pub(crate) const ChineseHundred: i32 = 11;
// java static final int ChineseThousand = 12;
pub(crate) const ChineseThousand: i32 = 12;
// java static final int ChineseMyriad = 13;
pub(crate) const ChineseMyriad: i32 = 13;
// java static final int ChineseHundredMillion = 14;
pub(crate) const ChineseHundredMillion: i32 = 14;
// java static final int ChineseMega = 15;
pub(crate) const ChineseMega: i32 = 15;
// java static final int[][] chNumTraits = new int[][]{{0, 10, 1}, {10, 100, 10}, {11, 1000, 100}, {12, 10000, 1000}};
pub(crate) static chNumTraits: &[[i32; 3]; 4] =
                    &[[0, 10, 1], [10, 100, 10], [11, 1000, 100], [12, 10000, 1000]];
// java static final int RomaThousand = 27;
pub(crate) const RomaThousand: i32 = 27;
// java static final int RomaHundred = 18;
pub(crate) const RomaHundred: i32 = 18;
// java static final int RomaTen = 9;
pub(crate) const RomaTen: i32 = 9;

// java public class CodePage
#[derive(Default)]
pub struct CodePage {
    // java private CODEPAGE[] mCodePages = null;
    mCodePages: Vec<Option<CODEPAGE>>,
    // java private String[][] mRomas = null;
    mRomas: Vec<Vec<Option<String>>>,
}

// java  class CODEPAGE
#[derive(Default, Clone)]
pub struct CODEPAGE {
    // java public String m_strPage;
    pub(crate) m_strPage: Option<String>,
    // java public int m_uiPower;
    pub(crate) m_uiPower: i32,
}

// java public class CodePage
impl CodePage {
    // java public static CodePage getInstance()
    pub fn getInstance() -> &'static CodePage {
      // java if (instanceCodePage == null)
      // java instanceCodePage = new CodePage();
      // java return instanceCodePage;
      instanceCodePage.get_or_init(|| CodePage::CodePage())
    }

    // java private  CodePage()
    pub fn CodePage() -> Self {
      // java this.mCodePages = new CODEPAGE[31];
      let mut this = Self {
        mCodePages: vec![None; 31usize],
        mRomas: vec![vec![None; 28usize]; 2usize],
      };
      // java this.mCodePages[0] = new CODEPAGE("0123456789", 10);
      this.mCodePages[0usize] = Some(CODEPAGE::CODEPAGE(Some("0123456789"), 10));
      // java this.mCodePages[1] = new CODEPAGE("○一二三四五六七八九十百千万亿兆正负点", 11);
      this.mCodePages[1usize] = Some(CODEPAGE::CODEPAGE(Some("○一二三四五六七八九十百千万亿兆正负点"), 11));
      // java this.mCodePages[2] = new CODEPAGE("零壹贰叁肆伍陆柒捌玖拾佰仟万亿兆正负点", 11);
      this.mCodePages[2usize] = Some(CODEPAGE::CODEPAGE(Some("零壹贰叁肆伍陆柒捌玖拾佰仟万亿兆正负点"), 11));
      // java this.mCodePages[3] = new CODEPAGE("０１２３４５６７８９", 10);
      this.mCodePages[3usize] = Some(CODEPAGE::CODEPAGE(Some("０１２３４５６７８９"), 10));
      // java this.mCodePages[4] = null;
      this.mCodePages[4usize] = None;
      // java this.mCodePages[5] = null;
      this.mCodePages[5usize] = None;
      // java this.mCodePages[6] = new CODEPAGE("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 26);
      this.mCodePages[6usize] = Some(CODEPAGE::CODEPAGE(Some("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), 26));
      // java this.mCodePages[7] = new CODEPAGE("abcdefghijklmnopqrstuvwxyz", 26);
      this.mCodePages[7usize] = Some(CODEPAGE::CODEPAGE(Some("abcdefghijklmnopqrstuvwxyz"), 26));
      // java this.mCodePages[8] = null;
      this.mCodePages[8usize] = None;
      // java this.mCodePages[9] = null;
      this.mCodePages[9usize] = None;
      // java this.mCodePages[10] = null;
      this.mCodePages[10usize] = None;
      // java this.mCodePages[11] = new CODEPAGE("0123456789ABCDEF", 16);
      this.mCodePages[11usize] = Some(CODEPAGE::CODEPAGE(Some("0123456789ABCDEF"), 16));
      // java this.mCodePages[12] = new CODEPAGE("甲乙丙丁戊己庚辛壬癸", 10);
      this.mCodePages[12usize] = Some(CODEPAGE::CODEPAGE(Some("甲乙丙丁戊己庚辛壬癸"), 10));
      // java this.mCodePages[13] = new CODEPAGE("子丑寅卯辰巳午未申酉戌亥", 12);
      this.mCodePages[13usize] = Some(CODEPAGE::CODEPAGE(Some("子丑寅卯辰巳午未申酉戌亥"), 12));
      // java this.mCodePages[14] = new CODEPAGE("⒈⒉⒊⒋⒌⒍⒎⒏⒐⒑⒒⒓⒔⒕⒖⒗⒘⒙⒚⒛", 20);
      this.mCodePages[14usize] = Some(CODEPAGE::CODEPAGE(Some("⒈⒉⒊⒋⒌⒍⒎⒏⒐⒑⒒⒓⒔⒕⒖⒗⒘⒙⒚⒛"), 20));
      // java this.mCodePages[15] = new CODEPAGE("⑴⑵⑶⑷⑸⑹⑺⑻⑼⑽⑾⑿⒀⒁⒂⒃⒄⒅⒆⒇", 20);
      this.mCodePages[15usize] = Some(CODEPAGE::CODEPAGE(Some("⑴⑵⑶⑷⑸⑹⑺⑻⑼⑽⑾⑿⒀⒁⒂⒃⒄⒅⒆⒇"), 20));
      // java this.mCodePages[16] = new CODEPAGE("①②③④⑤⑥⑦⑧⑨⑩⑪⑫⑬⑭⑮⑯⑰⑱⑲⑳", 20);
      this.mCodePages[16usize] = Some(CODEPAGE::CODEPAGE(Some("①②③④⑤⑥⑦⑧⑨⑩⑪⑫⑬⑭⑮⑯⑰⑱⑲⑳"), 20));
      // java this.mCodePages[17] = new CODEPAGE("㈠㈡㈢㈣㈤㈥㈦㈧㈨㈩", 10);
      this.mCodePages[17usize] = Some(CODEPAGE::CODEPAGE(Some("㈠㈡㈢㈣㈤㈥㈦㈧㈨㈩"), 10));
      // java this.mCodePages[18] = new CODEPAGE("ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ", 24);
      this.mCodePages[18usize] = Some(CODEPAGE::CODEPAGE(Some("ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ"), 24));
      // java this.mCodePages[19] = new CODEPAGE("αβγδεζηθικλμνξοπρστυφχψω", 24);
      this.mCodePages[19usize] = Some(CODEPAGE::CODEPAGE(Some("αβγδεζηθικλμνξοπρστυφχψω"), 24));
      // java this.mCodePages[20] = new CODEPAGE("零壹贰叁肆伍陆柒捌玖拾佰仟万亿兆正负点", 11);
      this.mCodePages[20usize] = Some(CODEPAGE::CODEPAGE(Some("零壹贰叁肆伍陆柒捌玖拾佰仟万亿兆正负点"), 11));
      // java this.mCodePages[21] = new CODEPAGE("アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン", 46);
      this.mCodePages[21usize] = Some(CODEPAGE::CODEPAGE(Some("アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン"), 46));
      // java this.mCodePages[22] = new CODEPAGE("イロハニホヘトチリヌルヲワカヨタレソツネナラムウヰノオクヤマケフコエテアサキユメミシヱヒモセスン", 48);
      this.mCodePages[22usize] = Some(CODEPAGE::CODEPAGE(Some("イロハニホヘトチリヌルヲワカヨタレソツネナラムウヰノオクヤマケフコエテアサキユメミシヱヒモセスン"), 48));
      // java this.mCodePages[23] = new CODEPAGE("〇壱弐参四伍六七八九拾百阡萬億兆-.", 11);
      this.mCodePages[23usize] = Some(CODEPAGE::CODEPAGE(Some("〇壱弐参四伍六七八九拾百阡萬億兆-."), 11));
      // java this.mCodePages[24] = new CODEPAGE("〇一二三四五六七八九十百千万億兆-.", 11);
      this.mCodePages[24usize] = Some(CODEPAGE::CODEPAGE(Some("〇一二三四五六七八九十百千万億兆-."), 11));
      // java this.mCodePages[25] = new CODEPAGE("０１２３４５６７８９十百千万億兆", 11);
      this.mCodePages[25usize] = Some(CODEPAGE::CODEPAGE(Some("０１２３４５６７８９十百千万億兆"), 11));
      // java this.mCodePages[26] = new CODEPAGE("零壹貳參肆伍陸柒捌玖拾佰仟萬億兆正-.", 11);
      this.mCodePages[26usize] = Some(CODEPAGE::CODEPAGE(Some("零壹貳參肆伍陸柒捌玖拾佰仟萬億兆正-."), 11));
      // java this.mCodePages[27] = new CODEPAGE("○一二三四五六七八九十百千萬億兆正-.", 11);
      this.mCodePages[27usize] = Some(CODEPAGE::CODEPAGE(Some("○一二三四五六七八九十百千萬億兆正-."), 11));
      // java this.mCodePages[28] = new CODEPAGE("กขคงจฉชซฌญฎฏฐฑฒณดตถทธนบปผฝพฟภมยรลวศษสหฬอฮ", 41);
      this.mCodePages[28usize] = Some(CODEPAGE::CODEPAGE(Some("กขคงจฉชซฌญฎฏฐฑฒณดตถทธนบปผฝพฟภมยรลวศษสหฬอฮ"), 41));
      // java this.mCodePages[29] = new CODEPAGE("๐๑๒๓๔๕๖๗๘๙", 10);
      this.mCodePages[29usize] = Some(CODEPAGE::CODEPAGE(Some("๐๑๒๓๔๕๖๗๘๙"), 10));
      // java this.mCodePages[30] = new CODEPAGE(NFStrDefine.DBNum30Info, 10);
      this.mCodePages[30usize] = Some(CODEPAGE::CODEPAGE(DBNum30Info, 10));
      // java this.mRomas = new String[2][28];
      // java this.mRomas[0][0] = "I";
      this.mRomas[0usize][0usize] = Some("I".to_string());
      // java this.mRomas[0][1] = "II";
      this.mRomas[0usize][1usize] = Some("II".to_string());
      // java this.mRomas[0][2] = "III";
      this.mRomas[0usize][2usize] = Some("III".to_string());
      // java this.mRomas[0][3] = "IV";
      this.mRomas[0usize][3usize] = Some("IV".to_string());
      // java this.mRomas[0][4] = "V";
      this.mRomas[0usize][4usize] = Some("V".to_string());
      // java this.mRomas[0][5] = "VI";
      this.mRomas[0usize][5usize] = Some("VI".to_string());
      // java this.mRomas[0][6] = "VII";
      this.mRomas[0usize][6usize] = Some("VII".to_string());
      // java this.mRomas[0][7] = "VIII";
      this.mRomas[0usize][7usize] = Some("VIII".to_string());
      // java this.mRomas[0][8] = "IX";
      this.mRomas[0usize][8usize] = Some("IX".to_string());
      // java this.mRomas[0][9] = "X";
      this.mRomas[0usize][9usize] = Some("X".to_string());
      // java this.mRomas[0][10] = "XX";
      this.mRomas[0usize][10usize] = Some("XX".to_string());
      // java this.mRomas[0][11] = "XXX";
      this.mRomas[0usize][11usize] = Some("XXX".to_string());
      // java this.mRomas[0][12] = "XL";
      this.mRomas[0usize][12usize] = Some("XL".to_string());
      // java this.mRomas[0][13] = "L";
      this.mRomas[0usize][13usize] = Some("L".to_string());
      // java this.mRomas[0][14] = "LX";
      this.mRomas[0usize][14usize] = Some("LX".to_string());
      // java this.mRomas[0][15] = "LXX";
      this.mRomas[0usize][15usize] = Some("LXX".to_string());
      // java this.mRomas[0][16] = "LXXX";
      this.mRomas[0usize][16usize] = Some("LXXX".to_string());
      // java this.mRomas[0][17] = "XC";
      this.mRomas[0usize][17usize] = Some("XC".to_string());
      // java this.mRomas[0][18] = "C";
      this.mRomas[0usize][18usize] = Some("C".to_string());
      // java this.mRomas[0][19] = "CC";
      this.mRomas[0usize][19usize] = Some("CC".to_string());
      // java this.mRomas[0][20] = "CCC";
      this.mRomas[0usize][20usize] = Some("CCC".to_string());
      // java this.mRomas[0][21] = "CD";
      this.mRomas[0usize][21usize] = Some("CD".to_string());
      // java this.mRomas[0][22] = "D";
      this.mRomas[0usize][22usize] = Some("D".to_string());
      // java this.mRomas[0][23] = "DC";
      this.mRomas[0usize][23usize] = Some("DC".to_string());
      // java this.mRomas[0][24] = "DCC";
      this.mRomas[0usize][24usize] = Some("DCC".to_string());
      // java this.mRomas[0][25] = "DCCC";
      this.mRomas[0usize][25usize] = Some("DCCC".to_string());
      // java this.mRomas[0][26] = "CM";
      this.mRomas[0usize][26usize] = Some("CM".to_string());
      // java this.mRomas[0][27] = "M";
      this.mRomas[0usize][27usize] = Some("M".to_string());
      // java this.mRomas[1][0] = "i";
      this.mRomas[1usize][0usize] = Some("i".to_string());
      // java this.mRomas[1][1] = "ii";
      this.mRomas[1usize][1usize] = Some("ii".to_string());
      // java this.mRomas[1][2] = "iii";
      this.mRomas[1usize][2usize] = Some("iii".to_string());
      // java this.mRomas[1][3] = "iv";
      this.mRomas[1usize][3usize] = Some("iv".to_string());
      // java this.mRomas[1][4] = "v";
      this.mRomas[1usize][4usize] = Some("v".to_string());
      // java this.mRomas[1][5] = "vi";
      this.mRomas[1usize][5usize] = Some("vi".to_string());
      // java this.mRomas[1][6] = "vii";
      this.mRomas[1usize][6usize] = Some("vii".to_string());
      // java this.mRomas[1][7] = "viii";
      this.mRomas[1usize][7usize] = Some("viii".to_string());
      // java this.mRomas[1][8] = "ix";
      this.mRomas[1usize][8usize] = Some("ix".to_string());
      // java this.mRomas[1][9] = "x";
      this.mRomas[1usize][9usize] = Some("x".to_string());
      // java this.mRomas[1][10] = "xx";
      this.mRomas[1usize][10usize] = Some("xx".to_string());
      // java this.mRomas[1][11] = "xxx";
      this.mRomas[1usize][11usize] = Some("xxx".to_string());
      // java this.mRomas[1][12] = "xl";
      this.mRomas[1usize][12usize] = Some("xl".to_string());
      // java this.mRomas[1][13] = "l";
      this.mRomas[1usize][13usize] = Some("l".to_string());
      // java this.mRomas[1][14] = "lx";
      this.mRomas[1usize][14usize] = Some("lx".to_string());
      // java this.mRomas[1][15] = "lxx";
      this.mRomas[1usize][15usize] = Some("lxx".to_string());
      // java this.mRomas[1][16] = "lxxx";
      this.mRomas[1usize][16usize] = Some("lxxx".to_string());
      // java this.mRomas[1][17] = "xc";
      this.mRomas[1usize][17usize] = Some("xc".to_string());
      // java this.mRomas[1][18] = "c";
      this.mRomas[1usize][18usize] = Some("c".to_string());
      // java this.mRomas[1][19] = "cc";
      this.mRomas[1usize][19usize] = Some("cc".to_string());
      // java this.mRomas[1][20] = "ccc";
      this.mRomas[1usize][20usize] = Some("ccc".to_string());
      // java this.mRomas[1][21] = "cd";
      this.mRomas[1usize][21usize] = Some("cd".to_string());
      // java this.mRomas[1][22] = "d";
      this.mRomas[1usize][22usize] = Some("d".to_string());
      // java this.mRomas[1][23] = "dc";
      this.mRomas[1usize][23usize] = Some("dc".to_string());
      // java this.mRomas[1][24] = "dcc";
      this.mRomas[1usize][24usize] = Some("dcc".to_string());
      // java this.mRomas[1][25] = "dccc";
      this.mRomas[1usize][25usize] = Some("dccc".to_string());
      // java this.mRomas[1][26] = "cm";
      this.mRomas[1usize][26usize] = Some("cm".to_string());
      // java this.mRomas[1][27] = "m";
      this.mRomas[1usize][27usize] = Some("m".to_string());
      this
    }

    // java public void serialNumber(long ulValue, int ksonfc, StringBuffer strResult)
    pub fn serialNumber(&self, ulValue: i64, ksonfc: i32, strResult: &mut StringBuffer) {
      // java int iDBNum = NFFunDefine.pickDBNum(ksonfc);
      let mut iDBNum: i32 = NFFunDefine::pickDBNum(ksonfc);
      // java if (ulValue >= 0L && iDBNum < this.mCodePages.length)
      if ulValue >= 0 && iDBNum < (self.mCodePages.len() as i32) {
        // java switch (iDBNum)
        match iDBNum {
          // java case 0: case 3: case 6: case 7: case 11: case 29:
          0 | 3 | 6 | 7 | 11 | 29 => {
            // java this.convertSeries(iDBNum, ulValue, strResult);
            self.convertSeries(iDBNum, ulValue, strResult);
          }
          // java case 1: case 20: case 27:
          1 | 20 | 27 => {
            // java this.tongue_chinese(iDBNum, ulValue, strResult, true);
            self.tongue_chinese(iDBNum, ulValue, strResult, true);
          }
          // java case 2: case 26:
          2 | 26 => {
            // java this.tongue_chinese(iDBNum, ulValue, strResult, false);
            self.tongue_chinese(iDBNum, ulValue, strResult, false);
          }
          // java case 4: case 5:
          4 | 5 => {
            // java this.tongue_Roma(iDBNum, ulValue, strResult);
            self.tongue_Roma(iDBNum, ulValue, strResult);
          }
          // java case 8:
          8 => {
            // java this.serialNumber_En_th(ulValue, strResult);
            self.serialNumber_En_th(ulValue, strResult);
          }
          // java case 9:
          9 => {
            // java this.serialNumber_En_One(ulValue, true, strResult);
            self.serialNumber_En_One(ulValue, true, strResult);
          }
          // java case 10:
          10 => {
            // java this.serialNumber_En_First(ulValue, strResult);
            self.serialNumber_En_First(ulValue, strResult);
          }
          // java case 12: case 13: case 14: case 15: case 16: case 17: case 18: case 19: case 21: case 22:
          12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 21 | 22 => {
            // java this.convertNum(iDBNum, ulValue, strResult);
            self.convertNum(iDBNum, ulValue, strResult);
          }
          // java case 23: case 24: case 25:
          23 | 24 | 25 => {
            // java this.tongue_japanese(iDBNum, ulValue, strResult);
            self.tongue_japanese(iDBNum, ulValue, strResult);
          }
          // java case 28:
          28 => {
            // java this.thaiLetter(iDBNum, ulValue, strResult);
            self.thaiLetter(iDBNum, ulValue, strResult);
          }
          // java case 30:
          30 => {
            // java this.thaiCount(iDBNum, ulValue, strResult);
            self.thaiCount(iDBNum, ulValue, strResult);
          }
          // java default:
          _ => {
          }
        }
        // java if (strResult.length() == 0)
        if (strResult.length() as i32) == 0 {
          // java strResult.append(Long.toString(ulValue));
          strResult.append_str(&ulValue.to_string());
        }
        // java int zeros = NFFunDefine.pickZeroCount(ksonfc);
        let mut zeros: i32 = NFFunDefine::pickZeroCount(ksonfc);
        // java String strPage = this.mCodePages[iDBNum] != null ? this.mCodePages[iDBNum].m_strPage : null;
      let mut strPage: Option<String> = self.mCodePages[(iDBNum) as usize].as_ref().and_then(|c| c.m_strPage.clone());
        // java while (zeros > strResult.length() && strPage != null)
        while zeros > (strResult.length() as i32) && strPage.is_some() {
          // java strResult.insert(0, strPage.charAt(0));
          strResult.insert(0, jchar_at(strPage.as_deref().unwrap_or(""), 0));
        }
      }
      // java else
      else {
        // java strResult.append(Long.toString(ulValue));
        strResult.append_str(&ulValue.to_string());
      }
    }

    // java private void serialNumber_En_First(long ulValue, StringBuffer strResult)
    pub fn serialNumber_En_First(&self, ulValue: i64, strResult: &mut StringBuffer) {
      // java if (ulValue <= 999999L)
      if ulValue <= 999999 {
        // java this.serialNumber_En_One(ulValue, false, strResult);
        self.serialNumber_En_One(ulValue, false, strResult);
        // java long ulTemp = 0L;
        let mut ulTemp: i64 = 0;
        // java label26: switch (strResult.charAt(strResult.length() - 1)) { case 'e': ulTemp = ulValue % 10L; switch ((int)ulTemp) { case 1: strResult.replace(strResult.length() - 3, strResult.length(), "first"); break label26; case 2: case 5: strResult.replace(strResult.length() - 2, strResult.length(), "fth"); break label26; case 3: strResult.replace(strResult.length() - 5, strResult.length(), "third"); case 4: case 6: case 7: case 8: default: break label26; case 9: strResult.replace(strResult.length() - 1, strResult.length(), NFStrDefine.EnglishNumber_th[0]); break label26; } case 'o': ulTemp = ulValue % 10L; if (ulTemp == 2L) { strResult.replace(strResult.length() - 3, strResult.length(), "second"); } else if (ulTemp == 0L) { strResult.append(NFStrDefine.EnglishNumber_th[0]); } break; case 't': strResult.replace(strResult.length() - 1, strResult.length(), NFStrDefine.EnglishNumber_th[0]); break; case 'y': strResult.replace(strResult.length() - 1, strResult.length(), "ieth"); break; default: strResult.append(NFStrDefine.EnglishNumber_th[0]);
        match strResult.charAt((strResult.length() as i32) - 1) {
  'e' => {
    ulTemp = ulValue % 10;
    match ((ulTemp) as i32) {
  1 => { strResult.replace((strResult.length() as i32) - 3, (strResult.length() as i32), "first"); },
  2 => {},
  5 => { strResult.replace((strResult.length() as i32) - 2, (strResult.length() as i32), "fth"); },
  3 => { strResult.replace((strResult.length() as i32) - 5, (strResult.length() as i32), "third"); },
  4 => {},
  6 => {},
  7 => {},
  8 => {},
  _ => {},
  9 => { strResult.replace((strResult.length() as i32) - 1, (strResult.length() as i32), EnglishNumber_th[0usize]); }
}
  },
  'o' => {
    ulTemp = ulValue % 10;
    if ulTemp == 2 { strResult.replace((strResult.length() as i32) - 3, (strResult.length() as i32), "second"); } else { if ulTemp == 0 { strResult.append_str(EnglishNumber_th[0usize]); } }
  },
  't' => { strResult.replace((strResult.length() as i32) - 1, (strResult.length() as i32), EnglishNumber_th[0usize]); },
  'y' => { strResult.replace((strResult.length() as i32) - 1, (strResult.length() as i32), "ieth"); },
  _ => { strResult.append_str(EnglishNumber_th[0usize]); }
}
        // java strResult.setCharAt(0, (char)(strResult.charAt(0) - 32));
        strResult.setCharAt(0, todo!("cast → char"));
      }
    }

    // java private void serialNumber_En_One(long ulValue, boolean bCapital, StringBuffer strResult)
    pub fn serialNumber_En_One(&self, ulValue: i64, bCapital: bool, strResult: &mut StringBuffer) {
      // java if (ulValue <= 999999L)
      if ulValue <= 999999 {
        // java if (ulValue == 0L)
        if ulValue == 0 {
          // java strResult.append(NFStrDefine.EnglishNumber_D0[0]);
          strResult.append_str(EnglishNumber_D0[0usize]);
        }
        // java else
        else {
          // java this.serialNumber_En_1000(ulValue, strResult);
          self.serialNumber_En_1000(ulValue, strResult);
        }
        // java if (bCapital)
        if bCapital {
          // java strResult.setCharAt(0, (char)(strResult.charAt(0) - 32));
          strResult.setCharAt(0, todo!("cast → char"));
        }
      }
    }

    // java private void serialNumber_En_99(long ulValue, StringBuffer strResult)
    pub fn serialNumber_En_99(&self, mut ulValue: i64, strResult: &mut StringBuffer) {
      // java if (ulValue != 0L)
      if ulValue != 0 {
        // java if (ulValue < 10L)
        if ulValue < 10 {
          // java strResult.append(NFStrDefine.EnglishNumber_D0[(int)ulValue]);
          strResult.append_str(EnglishNumber_D0[((ulValue) as i32) as usize]);
        }
        // java else
        else {
          // java if (ulValue / 10L != 1L)
          if ulValue / 10 != 1 {
            // java strResult.append(NFStrDefine.EnglishNumber_Tens[(int)(ulValue / 10L)]);
            strResult.append_str(EnglishNumber_Tens[((ulValue / 10) as i32) as usize]);
            // java ulValue %= 10L;
            ulValue = ulValue % 10;
            // java if (ulValue != 0L)
            if ulValue != 0 {
              // java strResult.append('-');
              strResult.append('-');
              // java strResult.append(NFStrDefine.EnglishNumber_D0[(int)(ulValue % 10L)]);
              strResult.append_str(EnglishNumber_D0[((ulValue % 10) as i32) as usize]);
            }
          }
          // java else
          else {
            // java strResult.append(NFStrDefine.EnglishNumber_1x[(int)(ulValue % 10L)]);
            strResult.append_str(EnglishNumber_1x[((ulValue % 10) as i32) as usize]);
          }
        }
      }
    }

    // java private void serialNumber_En_999(long ulValue, StringBuffer strResult)
    pub fn serialNumber_En_999(&self, mut ulValue: i64, strResult: &mut StringBuffer) {
      // java if (ulValue != 0L)
      if ulValue != 0 {
        // java if (ulValue < 100L)
        if ulValue < 100 {
          // java this.serialNumber_En_99(ulValue, strResult);
          self.serialNumber_En_99(ulValue, strResult);
        }
        // java else
        else {
          // java strResult.append(NFStrDefine.EnglishNumber_D0[(int)(ulValue / 100L)]);
          strResult.append_str(EnglishNumber_D0[((ulValue / 100) as i32) as usize]);
          // java strResult.append(' ');
          strResult.append(' ');
          // java strResult.append("hundred");
          strResult.append_str("hundred");
          // java ulValue %= 100L;
          ulValue = ulValue % 100;
          // java if (ulValue != 0L)
          if ulValue != 0 {
            // java strResult.append(' ');
            strResult.append(' ');
            // java this.serialNumber_En_99(ulValue, strResult);
            self.serialNumber_En_99(ulValue, strResult);
          }
        }
      }
    }

    // java private void serialNumber_En_1000(long ulValue, StringBuffer strResult)
    pub fn serialNumber_En_1000(&self, mut ulValue: i64, strResult: &mut StringBuffer) {
      // java if (ulValue < 1000L)
      if ulValue < 1000 {
        // java this.serialNumber_En_999(ulValue, strResult);
        self.serialNumber_En_999(ulValue, strResult);
      }
      // java else
      else {
        // java this.serialNumber_En_999(ulValue / 1000L, strResult);
        self.serialNumber_En_999(ulValue / 1000, strResult);
        // java strResult.append(' ');
        strResult.append(' ');
        // java strResult.append("thousand");
        strResult.append_str("thousand");
        // java ulValue %= 1000L;
        ulValue = ulValue % 1000;
        // java if (ulValue != 0L)
        if ulValue != 0 {
          // java strResult.append(' ');
          strResult.append(' ');
          // java this.serialNumber_En_999(ulValue, strResult);
          self.serialNumber_En_999(ulValue, strResult);
        }
      }
    }

    // java  void serialNumber_En_th(long ulValue, StringBuffer strResult)
    pub fn serialNumber_En_th(&self, mut ulValue: i64, strResult: &mut StringBuffer) {
      // java this.convertSeries(0, ulValue, strResult);
      self.convertSeries(0, ulValue, strResult);
      // java switch ((int)(ulValue % 10L))
      'sn_en_th: {
      match ((ulValue % 10) as i32) {
        // java case 1: case 2: case 3:
        1 | 2 | 3 => {
          // java if (ulValue % 100L / 10L != 1L)
          if ulValue % 100 / 10 != 1 {
            // java ulValue %= 10L;
            ulValue = ulValue % 10;
            // java break;
            break 'sn_en_th;
          }
        }
        // java default:
        _ => {
          // java ulValue = 0L;
          ulValue = 0;
        }
      }
      }
      // java strResult.append(NFStrDefine.EnglishNumber_th[(int)ulValue]);
      strResult.append_str(EnglishNumber_th[((ulValue) as i32) as usize]);
    }

    // java private void tongue_chinese(int iDBNum, long ulValue, StringBuffer strResult, boolean bEraseTen)
    pub fn tongue_chinese(&self, iDBNum: i32, mut ulValue: i64, strResult: &mut StringBuffer, bEraseTen: bool) {
      // java strResult.setLength(0);
      strResult.setLength(0);
      // java Stack<StringBuffer> stack = new Stack();
      let mut stack: Vec<StringBuffer> = Vec::new();
      // java String strPage = this.mCodePages[iDBNum].m_strPage;
      let mut strPage: Option<String> = self.mCodePages[(iDBNum) as usize].as_ref().and_then(|c| c.m_strPage.clone());
      // java if (ulValue == 0L)
      if ulValue == 0 {
        // java strResult.append(strPage.charAt(0));
        strResult.append(jchar_at(strPage.as_deref().unwrap_or(""), 0));
      }
      // java else
      else {
        // java int sects = 0;
        let mut sects: i32 = 0;
        // java int n9999 = 0;
        let mut n9999: i32 = 0;
        let mut bLastIsZero: bool = false;
        // java for(boolean bLastIsZero = false; ulValue > 0L ++sects)
        while ulValue > 0 {
          // java n9999 = (int)(ulValue % 10000L);
          n9999 = ((ulValue % 10000) as i32);
          // java ulValue /= 10000L;
          ulValue = ulValue / 10000;
          // java StringBuffer _strRes = new StringBuffer();
          let mut _strRes: StringBuffer = StringBuffer::new();
          // java if (this.tongue_chinese_9999(n9999, strPage, _strRes))
          if self.tongue_chinese_9999(n9999, strPage.as_deref().unwrap_or(""), &mut _strRes) {
            // java switch (sects % 4)
            match sects % 4 {
              // java case 0:
              0 => {
                // java if (sects > 0)
                if sects > 0 {
                  // java _strRes.append(strPage.charAt(15));
                  _strRes.append(jchar_at(strPage.as_deref().unwrap_or(""), 15));
                }
              }
              // java case 1:
              1 => {
                // java _strRes.append(strPage.charAt(13));
                _strRes.append(jchar_at(strPage.as_deref().unwrap_or(""), 13));
              }
              // java case 2:
              2 => {
                // java _strRes.append(strPage.charAt(14));
                _strRes.append(jchar_at(strPage.as_deref().unwrap_or(""), 14));
              }
              // java default:
              _ => {
              }
            }
            // java stack.push(_strRes);
            stack.push(_strRes);
          }
          // java if (n9999 < 1000 && !bLastIsZero && stack.size() > 0 && ulValue > 0L)
          if n9999 < 1000 && !bLastIsZero && (stack.len() as i32) > 0 && ulValue > 0 {
            // java stack.push(new StringBuffer("" + strPage.charAt(0)));
            stack.push(StringBuffer::new__string(&format!("{}", jchar_at(strPage.as_deref().unwrap_or(""), 0))));
          }
          // java bLastIsZero = n9999 < 1000;
          bLastIsZero = n9999 < 1000;
          sects += 1;
        }
        // java while (stack.size() > 0)
        while (stack.len() as i32) > 0 {
          // java strResult.append((StringBuffer)stack.pop());
          if let Some(sb) = stack.pop() { strResult.append_str(&sb.toString()); }
        }
        // java if (bEraseTen && strResult.length() >= 2 && strResult.charAt(1) == strPage.charAt(10) && strResult.charAt(0) == strPage.charAt(1))
        if bEraseTen && (strResult.length() as i32) >= 2 && strResult.charAt(1) == jchar_at(strPage.as_deref().unwrap_or(""), 10) && strResult.charAt(0) == jchar_at(strPage.as_deref().unwrap_or(""), 1) {
          // java strResult.deleteCharAt(0);
          strResult.deleteCharAt(0);
        }
      }
    }

    // java private boolean tongue_chinese_9999(int n9999, String strPage, StringBuffer strResult)
    pub fn tongue_chinese_9999(&self, n9999: i32, strPage: &str, strResult: &mut StringBuffer) -> bool {
      // java Assert.assertTrue("n9999 <= 9999 should be true.", n9999 <= 9999);
      Assert::assertTrue_msg(&"n9999 <= 9999 should be true.", n9999 <= 9999);
      // java strResult.setLength(0);
      strResult.setLength(0);
      // java int[] bLastIsZero = new int[]{0};
      let mut bLastIsZero: Vec<i32> = vec![0];
      // java int i = 4;
      let mut i: i32 = 4;
      // java do (i >= 0 && this._tongue_chinese_9999_help(n9999, strPage, strResult, i, bLastIsZero))
      loop {
          // java --i;
          { let _tmp = i; i -= 1; _tmp };
          if !(i >= 0 && self._tongue_chinese_9999_help(n9999, strPage, strResult, i, &mut bLastIsZero)) {
              break;
          }
      }
      // java return strResult.length() > 0;
      return (strResult.length() as i32) > 0;
    }

    // java private boolean _tongue_chinese_9999_help(int n9999, String strPage, StringBuffer strResult, int trait, int[] bLastIsZero)
    pub fn _tongue_chinese_9999_help(&self, n9999: i32, strPage: &str, strResult: &mut StringBuffer, r#trait: i32, bLastIsZero: &mut Vec<i32>) -> bool {
      // java int n = 0;
      let mut n: i32 = 0;
      // java int[] row = chNumTraits[trait];
      let row: [i32; 3] = chNumTraits[(r#trait) as usize];
      // java if (n9999 % row[1] == 0)
      if n9999 % row[1usize] == 0 {
        // java return false;
        return false;
      }
      // java else if (row[2] == 1)
      else if row[2usize] == 1 {
        // java strResult.append(strPage.charAt(n9999 % row[1]));
        strResult.append(jchar_at(strPage, n9999 % row[1usize]));
        // java return false;
        return false;
      }
      // java else
      else {
        // java if ((n = n9999 % row[1] / row[2]) > 0)
        if { n = n9999 % row[1usize] / row[2usize]; n } > 0 {
          // java strResult.append(strPage.charAt(n));
          strResult.append(jchar_at(strPage, n));
          // java strResult.append(strPage.charAt(row[0]));
          strResult.append(jchar_at(strPage, row[0usize]));
          // java bLastIsZero[0] = 0;
          bLastIsZero[0usize] = 0;
        }
        // java else if (strResult.length() > 0 && bLastIsZero[0] == 0)
        else if (strResult.length() as i32) > 0 && bLastIsZero[0usize] == 0 {
          // java strResult.append(strPage.charAt(0));
          strResult.append(jchar_at(strPage, 0));
          // java bLastIsZero[0] = 1;
          bLastIsZero[0usize] = 1;
        }
        // java return true;
        return true;
      }
    }

    // java private void tongue_japanese(int iDBNum, long ulValue, StringBuffer strResult)
    pub fn tongue_japanese(&self, iDBNum: i32, mut ulValue: i64, strResult: &mut StringBuffer) {
      // java strResult.setLength(0);
      strResult.setLength(0);
      // java Stack<StringBuffer> stack = new Stack();
      let mut stack: Vec<StringBuffer> = Vec::new();
      // java String strPage = this.mCodePages[iDBNum].m_strPage;
      let mut strPage: Option<String> = self.mCodePages[(iDBNum) as usize].as_ref().and_then(|c| c.m_strPage.clone());
      // java int sects = 0;
      let mut sects: i32 = 0;
      let mut n9999: i32 = 0;
      // java for(int n9999 = 0; ulValue > 0L ++sects)
      while ulValue > 0 {
        // java n9999 = (int)(ulValue % 10000L);
        n9999 = ((ulValue % 10000) as i32);
        // java ulValue /= 10000L;
        ulValue = ulValue / 10000;
        // java StringBuffer _strRes = new StringBuffer();
        let mut _strRes: StringBuffer = StringBuffer::new();
        // java if (this.tongue_japanese_9999(n9999, strPage, _strRes))
        if self.tongue_japanese_9999(n9999, strPage.as_deref().unwrap_or(""), &mut _strRes) {
          // java switch (sects % 4)
          match sects % 4 {
            // java case 0:
            0 => {
              // java if (sects > 0)
              if sects > 0 {
                // java _strRes.append(strPage.charAt(15));
                _strRes.append(jchar_at(strPage.as_deref().unwrap_or(""), 15));
              }
            }
            // java case 1:
            1 => {
              // java _strRes.append(strPage.charAt(13));
              _strRes.append(jchar_at(strPage.as_deref().unwrap_or(""), 13));
            }
            // java case 2:
            2 => {
              // java _strRes.append(strPage.charAt(14));
              _strRes.append(jchar_at(strPage.as_deref().unwrap_or(""), 14));
            }
            // java default:
            _ => {
            }
          }
          // java stack.push(_strRes);
          stack.push(_strRes);
        }
        sects += 1;
      }
      // java while (stack.size() > 0)
      while (stack.len() as i32) > 0 {
        // java strResult.append((StringBuffer)stack.pop());
        if let Some(sb) = stack.pop() { strResult.append_str(&sb.toString()); }
      }
    }

    // java private boolean tongue_japanese_9999(int n9999, String strPage, StringBuffer strResult)
    pub fn tongue_japanese_9999(&self, n9999: i32, strPage: &str, strResult: &mut StringBuffer) -> bool {
      // java Assert.assertTrue("n9999 <= 9999 should be true.", n9999 <= 9999);
      Assert::assertTrue_msg(&"n9999 <= 9999 should be true.", n9999 <= 9999);
      // java strResult.setLength(0);
      strResult.setLength(0);
      // java int i = 4;
      let mut i: i32 = 4;
      // java do (i >= 0 && this._tongue_japanese_9999_help(n9999, strPage, strResult, i))
      loop {
          // java --i;
          { let _tmp = i; i -= 1; _tmp };
          if !(i >= 0 && self._tongue_japanese_9999_help(n9999, strPage, strResult, i)) {
              break;
          }
      }
      // java return strResult.length() > 0;
      return (strResult.length() as i32) > 0;
    }

    // java private boolean _tongue_japanese_9999_help(int n9999, String strPage, StringBuffer strResult, int trait)
    pub fn _tongue_japanese_9999_help(&self, n9999: i32, strPage: &str, strResult: &mut StringBuffer, r#trait: i32) -> bool {
      // java int n = 0;
      let mut n: i32 = 0;
      // java int[] row = chNumTraits[trait];
      let row: [i32; 3] = chNumTraits[(r#trait) as usize];
      // java if (n9999 % row[1] == 0)
      if n9999 % row[1usize] == 0 {
        // java return false;
        return false;
      }
      // java else if (row[2] == 1)
      else if row[2usize] == 1 {
        // java strResult.append(strPage.charAt(n9999 % row[1]));
        strResult.append(jchar_at(strPage, n9999 % row[1usize]));
        // java return false;
        return false;
      }
      // java else
      else {
        // java if ((n = n9999 % row[1] / row[2]) > 0)
        if { n = n9999 % row[1usize] / row[2usize]; n } > 0 {
          // java if (n != 1)
          if n != 1 {
            // java strResult.append(strPage.charAt(n));
            strResult.append(jchar_at(strPage, n));
          }
          // java strResult.append(strPage.charAt(row[0]));
          strResult.append(jchar_at(strPage, row[0usize]));
        }
        // java return true;
        return true;
      }
    }

    // java private void convertNum(int iDBNum, long ulValue, StringBuffer strResult)
    pub fn convertNum(&self, iDBNum: i32, ulValue: i64, strResult: &mut StringBuffer) {
      // java if (1L <= ulValue && ulValue <= (long)this.mCodePages[iDBNum].m_uiPower)
      if 1 <= ulValue && ulValue <= ((self.mCodePages[(iDBNum) as usize].as_ref().map(|c| c.m_uiPower).unwrap_or(0)) as i64) {
        // java strResult.append(this.mCodePages[iDBNum].m_strPage.charAt((int)(ulValue - 1L)));
        strResult.append(jchar_at(self.mCodePages[(iDBNum) as usize].as_ref().and_then(|c| c.m_strPage.as_deref()).unwrap_or(""), ((ulValue - 1) as i32)));
      }
    }

    // java private void convertSeries(int iDBNum, long ulValue, StringBuffer strResult)
    pub fn convertSeries(&self, iDBNum: i32, mut ulValue: i64, strResult: &mut StringBuffer) {
      // java long ulPower = (long)this.mCodePages[iDBNum].m_uiPower;
      let mut ulPower: i64 = ((self.mCodePages[(iDBNum) as usize].as_ref().map(|c| c.m_uiPower).unwrap_or(0)) as i64);
      // java String strPage = this.mCodePages[iDBNum].m_strPage;
      let mut strPage: Option<String> = self.mCodePages[(iDBNum) as usize].as_ref().and_then(|c| c.m_strPage.clone());
      // java if (0L == ulValue)
      if 0 == ulValue {
        // java strResult.append(strPage.charAt(0));
        strResult.append(jchar_at(strPage.as_deref().unwrap_or(""), 0));
      }
      // java else
      else {
        // java int iNum = 0;
        let mut iNum: i32 = 0;
        // java if (29 != iDBNum && 3 != iDBNum && 11 != iDBNum && 0 != iDBNum)
        if 29 != iDBNum && 3 != iDBNum && 11 != iDBNum && 0 != iDBNum {
          // java iNum = 1;
          iNum = 1;
        }
        // java long ulDigit = 0L;
        let mut ulDigit: i64 = 0;
        // java while (ulValue >= ulPower)
        while ulValue >= ulPower {
          // java ulValue -= (long)iNum;
          ulValue = ulValue - ((iNum) as i64);
          // java ulDigit = ulValue % ulPower;
          ulDigit = ulValue % ulPower;
          // java ulValue /= ulPower;
          ulValue = ulValue / ulPower;
          // java strResult.append(strPage.charAt((int)ulDigit));
          strResult.append(jchar_at(strPage.as_deref().unwrap_or(""), ((ulDigit) as i32)));
        }
        // java if (ulValue != 0L)
        if ulValue != 0 {
          // java strResult.append(strPage.charAt((int)(ulValue - (long)iNum)));
          strResult.append(jchar_at(strPage.as_deref().unwrap_or(""), ((ulValue - ((iNum) as i64)) as i32)));
        }
      }
      // java strResult.reverse();
      strResult.reverse();
    }

    // java private void tongue_Roma(int iDBNum, long ulValue, StringBuffer strResult)
    pub fn tongue_Roma(&self, iDBNum: i32, mut ulValue: i64, strResult: &mut StringBuffer) {
      // java if (ulValue <= 1000L && ulValue > 0L)
      if ulValue <= 1000 && ulValue > 0 {
        // java String[] ppwszRomaPage = this.mRomas[iDBNum - 4];
        let ppwszRomaPage: &Vec<Option<String>> = &self.mRomas[(iDBNum - 4) as usize];
        // java int uDigit = (int)(ulValue / 1000L);
        let mut uDigit: i32 = ((ulValue / 1000) as i32);
        // java if (uDigit != 0)
        if uDigit != 0 {
          // java strResult.append_str(ppwszRomaPage[uDigit + 27 - 1].as_deref().unwrap_or(""));
          strResult.append_str(ppwszRomaPage[(uDigit + 27 - 1) as usize].as_deref().unwrap_or(""));
        }
        // java ulValue %= 1000L;
        ulValue = ulValue % 1000;
        // java uDigit = (int)(ulValue / 100L);
        uDigit = ((ulValue / 100) as i32);
        // java if (uDigit != 0)
        if uDigit != 0 {
          // java strResult.append_str(ppwszRomaPage[uDigit + 18 - 1].as_deref().unwrap_or(""));
          strResult.append_str(ppwszRomaPage[(uDigit + 18 - 1) as usize].as_deref().unwrap_or(""));
        }
        // java ulValue %= 100L;
        ulValue = ulValue % 100;
        // java uDigit = (int)(ulValue / 10L);
        uDigit = ((ulValue / 10) as i32);
        // java if (uDigit != 0)
        if uDigit != 0 {
          // java strResult.append_str(ppwszRomaPage[uDigit + 9 - 1].as_deref().unwrap_or(""));
          strResult.append_str(ppwszRomaPage[(uDigit + 9 - 1) as usize].as_deref().unwrap_or(""));
        }
        // java uDigit = (int)(ulValue % 10L);
        uDigit = ((ulValue % 10) as i32);
        // java if (uDigit != 0)
        if uDigit != 0 {
          // java strResult.append_str(ppwszRomaPage[uDigit - 1].as_deref().unwrap_or(""));
          strResult.append_str(ppwszRomaPage[(uDigit - 1) as usize].as_deref().unwrap_or(""));
        }
      }
    }

    // java private void thaiLetter(int iDBNum, long ulValue, StringBuffer strResult)
    pub fn thaiLetter(&self, iDBNum: i32, mut ulValue: i64, strResult: &mut StringBuffer) {
      // java String strPage = this.mCodePages[iDBNum].m_strPage;
      let mut strPage: Option<String> = self.mCodePages[(iDBNum) as usize].as_ref().and_then(|c| c.m_strPage.clone());
      // java --ulValue;
      { let _tmp = ulValue; ulValue -= 1; _tmp };
      // java int len = (int)(ulValue / (long)strPage.length() + 1L);
      let mut len: i32 = ((ulValue / (((strPage.as_deref().unwrap_or("").chars().count() as i32)) as i64) + 1) as i32);
      // java char letter = strPage.charAt((int)(ulValue % (long)strPage.length()));
      let mut letter: char = jchar_at(strPage.as_deref().unwrap_or(""), ((ulValue % (((strPage.as_deref().unwrap_or("").chars().count() as i32)) as i64)) as i32));
      let mut i: i32 = 0;
      // java for(int i = 0; i < len; ++i)
      while i < len {
        // java strResult.append(letter);
        strResult.append(letter);
        i += 1;
      }
    }

    // java private void thaiCount(int iDBNum, long ulValue, StringBuffer strResult)
    pub fn thaiCount(&self, iDBNum: i32, ulValue: i64, strResult: &mut StringBuffer) {
      // java if (ulValue <= 10000L)
      if ulValue <= 10000 {
        // java String result = Thai.getCount((int)ulValue);
        let mut result: String = Thai::getCount(((ulValue) as i32)).unwrap_or_default();
        // java strResult.append_str(&result);
        strResult.append_str(&result);
      }
    }

}

// java  class CODEPAGE
impl CODEPAGE {
    // java   CODEPAGE(String strPage, int uiPower)
    pub fn CODEPAGE(strPage: Option<&str>, uiPower: i32) -> Self {
      // java this.m_strPage = strPage;
      Self { m_strPage: strPage.map(|s| s.to_string()), m_uiPower: uiPower }
      // java this.m_uiPower = uiPower;
    }

}
