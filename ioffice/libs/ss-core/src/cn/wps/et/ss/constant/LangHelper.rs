// java public final class LangHelper
pub struct LangHelper;pub struct CharW{pub val:u16}
// java private static final char[] fullWidthChars= new char[]{'。', '「', '」', '、', '・', 'ヲ', 'ァ', 'ィ', 'ゥ', 'ェ', 'ォ', 'ャ', 'ュ', 'ョ', 'ッ', 'ー', 'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ', 'ス', 'セ', 'ソ', 'タ', 'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ', 'マ', 'ミ', 'ム', 'メ', 'モ', 'ヤ', 'ユ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ワ', 'ン', '゛', '゜'};
const fullWidthChars:&[u16]=&[12290,12300,12301,12289,12539,12530,12449,12451,12453,12455,12457,12515,12517,12519,12483,12540,12450,12452,12454,12456,12458,12459,12461,12463,12465,12467,12469,12471,12473,12475,12477,12479,12481,12484,12486,12488,12490,12491,12492,12493,12494,12495,12498,12501,12504,12507,12510,12511,12512,12513,12514,12516,12518,12520,12521,12522,12523,12524,12525,12527,12531,12443,12444];
// java private static final char[] s_wchVoicedArr= new char[]{'ガ', 'ギ', 'グ', 'ゲ', 'ゴ', 'ザ', 'ジ', 'ズ', 'ゼ', 'ゾ', 'ダ', 'ヂ', 'ヅ', 'デ', 'ド', 'バ', 'ビ', 'ブ', 'ベ', 'ボ', 'ヴ', 'ヷ', 'ヺ'};
const s_wchVoicedArr:&[u16]=&[12460,12462,12464,12466,12468,12470,12472,12474,12476,12478,12480,12482,12485,12487,12489,12496,12499,12502,12505,12508,12532,12535,12538];
// java private static final char[] s_wchHalfVoicedArr= new char[]{'パ', 'ピ', 'プ', 'ペ', 'ポ'};
const s_wchHalfVoicedArr:&[u16]=&[12497,12500,12503,12506,12509];
// java private static final char[] pwNot= new char[]{'ガ', 'ギ', 'グ', 'ゲ', 'ゴ', 'ザ', 'ジ', 'ズ', 'ゼ', 'ゾ', 'ダ', 'ヂ', 'ヅ', 'デ', 'ド', 'バ', 'パ', 'ビ', 'ピ', 'ブ', 'プ', 'ベ', 'ペ', 'ボ', 'ポ', 'ヴ', '￥'};
const pwNot:&[u16]=&[12460,12462,12464,12466,12468,12470,12472,12474,12476,12478,12480,12482,12485,12487,12489,12496,12497,12499,12500,12502,12503,12505,12506,12508,12509,12532,65509];
impl LangHelper{
// java private static char checkKATAKANA(char c, boolean toHalf) {
// java if (!toHalf) {
// java return '｡' <= c && c < '｡' + fullWidthChars.length ? fullWidthChars[c - '｡'] : c;
// java } else {
// java for(int i = 0; i < fullWidthChars.length; ++i) {
// java if (fullWidthChars[i] == c) {
// java return (char)('｡' + i);
// java }
// java }
// java
// java return c;
// java }
// java }
pub fn checkKATAKANA(c:u16,toHalf:bool)->u16 {
if !toHalf{if c>=0xff61&&(c as usize)<0xff61+fullWidthChars.len(){fullWidthChars[(c-0xff61)as usize]}else{c}}else{fullWidthChars.iter().position(|x|*x==c).map_or(c,|i|0xff61+i as u16)}
}
// java public static boolean startsWithSingleQuote(String s) {
// java if (s != null && s.length() != 0) {
// java char c = s.charAt(0);
// java return c == '\'' || c == '＇' || c == 8217;
// java } else {
// java return false;
// java }
// java }
pub fn startsWithSingleQuote(s:&[u16])->bool {
s.first().is_some_and(|c|matches!(*c,39|0xff07|8217))
}
// java public static boolean isOprEqual(char c) {
// java return c == '=' || c == '＝' || c == 12307;
// java }
pub fn isOprEqual(c:u16)->bool {
matches!(c,61|0xff1d|12307)
}
// java public static char toHalfHornChar(char c) {
// java return toHalfHornChar(c, (CharW)null);
// java }
pub fn toHalfHornChar(c:u16)->u16 {
Self::toHalfHornChar__2(c,None)
}
// java static char toHalfHornChar(char c, CharW chFollow) {
// java if (c >= '！' && c <= '～') {
// java return (char)(c - 'ﻠ');
// java } else {
// java switch (c) {
// java case '“':
// java case '”':
// java return '"';
// java case '　':
// java return ' ';
// java case '－':
// java return '-';
// java case '￠':
// java return '¢';
// java case '￡':
// java return '£';
// java case '￢':
// java return '¬';
// java case '￣':
// java return '¯';
// java case '￤':
// java return '¦';
// java case '￥':
// java return '¥';
// java case '￦':
// java return '₩';
// java default:
// java if (c >= 12593 && c <= 12644) {
// java if (c == 12644) {
// java return 'ﾠ';
// java }
// java
// java if (c >= 12593 && c <= 12622) {
// java return (char)(c + '칰');
// java }
// java
// java if (c >= 12623 && c <= 12628) {
// java return (char)(c + '칳');
// java }
// java
// java if (c >= 12629 && c <= 12634) {
// java return (char)(c + '칵');
// java }
// java
// java if (c >= 12635 && c <= 12640) {
// java return (char)(c + '칷');
// java }
// java
// java if (c >= 12641 && c <= 12643) {
// java return (char)(c + '칹');
// java }
// java } else if (c >= 12289 && c <= 12540) {
// java if (chFollow != null) {
// java int find = binarySearch(s_wchVoicedArr, 0, s_wchVoicedArr.length - 3, c);
// java if (find >= 0) {
// java chFollow.val = 'ﾞ';
// java --c;
// java } else if (s_wchVoicedArr[20] == c) {
// java c = 12454;
// java } else {
// java int voice = Arrays.binarySearch(s_wchHalfVoicedArr, c);
// java if (voice >= 0) {
// java chFollow.val = 'ﾟ';
// java c = (char)(c - 2);
// java }
// java }
// java }
// java
// java return checkKATAKANA(c, true);
// java }
// java
// java return c;
// java }
// java }
// java }
pub fn toHalfHornChar__2(mut c:u16,follow:Option<&mut CharW>)->u16 {
if (0xff01..=0xff5e).contains(&c){return c-0xfee0;}match c{0x201c|0x201d=>34,0x3000=>32,0xff0d=>45,0xffe0=>0xa2,0xffe1=>0xa3,0xffe2=>0xac,0xffe3=>0xaf,0xffe4=>0xa6,0xffe5=>0xa5,0xffe6=>0x20a9,_=>{if(12593..=12644).contains(&c){if c==12644{return 0xffa0;}if(12593..=12622).contains(&c){return c.wrapping_add('칰' as u16);}if(12623..=12628).contains(&c){return c.wrapping_add('칳' as u16);}if(12629..=12634).contains(&c){return c.wrapping_add('칵' as u16);}if(12635..=12640).contains(&c){return c.wrapping_add('칷' as u16);}if(12641..=12643).contains(&c){return c.wrapping_add('칹' as u16);}}else if(12289..=12540).contains(&c){if let Some(f)=follow{if s_wchVoicedArr[..s_wchVoicedArr.len()-3].binary_search(&c).is_ok(){f.val=0xff9e;c-=1;}else if s_wchVoicedArr[20]==c{c=12454;}else if s_wchHalfVoicedArr.binary_search(&c).is_ok(){f.val=0xff9f;c-=2;}}return Self::checkKATAKANA(c,true);}c}}
}
// java public static String toHalfHornString(String s) {
// java if (s != null && s.length() != 0) {
// java int slen = s.length();
// java StringBuilder ret = new StringBuilder(slen);
// java
// java for(int i = 0; i < slen; ++i) {
// java char c = s.charAt(i);
// java int j = Arrays.binarySearch(pwNot, c);
// java if (j >= 0) {
// java ret.append(c);
// java } else {
// java ret.append(toHalfHornChar(c, (CharW)null));
// java }
// java }
// java
// java return ret.toString();
// java } else {
// java return s;
// java }
// java }
pub fn toHalfHornString(s:&[u16])->Vec<u16> {
s.iter().map(|c|if pwNot.binary_search(c).is_ok(){*c}else{Self::toHalfHornChar(*c)}).collect()
}
}
