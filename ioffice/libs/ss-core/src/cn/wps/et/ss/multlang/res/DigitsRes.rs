// java public final class DigitsRes {
pub struct DigitsRes;
impl DigitsRes{
    // java public static final String COMMON_DIGITS = "0123456789";
    pub const COMMON_DIGITS:&str="0123456789";
    // java public static final String ARABIC_DIGITS = "٠١٢٣٤٥٦٧٨٩";
    pub const ARABIC_DIGITS:&str="٠١٢٣٤٥٦٧٨٩";
    // java public static final String TAI_DIGITS = "๐๑๒๓๔๕๖๗๘๙";
    pub const TAI_DIGITS:&str="๐๑๒๓๔๕๖๗๘๙";
    // java public static final String HINDI_DIGITS = "०१२३४५६७८९";
    pub const HINDI_DIGITS:&str="०१२३४५६७८९";
    // java public static final String KOREAN_DIGITS = "영일이삼사오육칠팔구";
    pub const KOREAN_DIGITS:&str="영일이삼사오육칠팔구";
    // java public static final int COMMON_INDEX = 0; ARABIC_INDEX = 1; TAI_INDEX = 2; HINDI_INDEX = 3; KOREAN_INDEX = 4;
    pub const COMMON_INDEX:i32=0;pub const ARABIC_INDEX:i32=1;pub const TAI_INDEX:i32=2;pub const HINDI_INDEX:i32=3;pub const KOREAN_INDEX:i32=4;
    // java public static final String[] DIGITS = new String[]{"0123456789", "٠١٢٣٤٥٦٧٨٩", "๐๑๒๓๔๕๖๗๘๙", "०१२३४५६७८९", "영일이삼사오육칠팔구"};
    pub const DIGITS:[&str;5]=[Self::COMMON_DIGITS,Self::ARABIC_DIGITS,Self::TAI_DIGITS,Self::HINDI_DIGITS,Self::KOREAN_DIGITS];
    // java public static final int COMMON = 1; ARABIC = 2; TAI = 4; HINDI = 8; KOREAN = 16;
    pub const COMMON:i32=1;pub const ARABIC:i32=2;pub const TAI:i32=4;pub const HINDI:i32=8;pub const KOREAN:i32=16;
    // java public static boolean isMix(int flag) {
    pub fn isMix(flag:i32)->bool{
        // java int cur = flag; int typeCount = 0;
        let mut cur=flag;let mut typeCount=0;
        // java while(cur != 0) {
        while cur!=0{
            // java cur >>= 1;
            cur>>=1;
            // java if ((cur & 1) == 1) { ++typeCount; }
            if cur&1==1{typeCount+=1;}
            // java if (typeCount >= 2) { return true; }
            if typeCount>=2{return true;}
        }
        // java return false;
        false
    }
    // java public static boolean isCommon(int flag) { return flag == 1; }
    pub fn isCommon(flag:i32)->bool{flag==1}
    // java public static boolean isArabic(int flag) { return flag == 2; }
    pub fn isArabic(flag:i32)->bool{flag==2}
    // java public static boolean isTai(int flag) { return flag == 4; }
    pub fn isTai(flag:i32)->bool{flag==4}
    // java public static boolean isHindi(int flag) { return flag == 8; }
    pub fn isHindi(flag:i32)->bool{flag==8}
    // java public static boolean isKorean(int flag) { return flag == 16; }
    pub fn isKorean(flag:i32)->bool{flag==16}
    // java public static boolean containCommon(int flag) { return (flag & 1) > 0; }
    pub fn containCommon(flag:i32)->bool{flag&1>0}
    // java public static boolean containArabic(int flag) { return (flag & 2) > 0; }
    pub fn containArabic(flag:i32)->bool{flag&2>0}
    // java public static boolean containTai(int flag) { return (flag & 4) > 0; }
    pub fn containTai(flag:i32)->bool{flag&4>0}
    // java public static boolean containHindi(int flag) { return (flag & 8) > 0; }
    pub fn containHindi(flag:i32)->bool{flag&8>0}
    // java public static boolean containKorean(int flag) { return (flag & 16) > 0; }
    pub fn containKorean(flag:i32)->bool{flag&16>0}
    // java public static int addCommon(int flag) { return flag | 1; }
    pub fn addCommon(flag:i32)->i32{flag|1}
    // java public static int addFlag(int index, int flag) {
    pub fn addFlag(index:i32,flag:i32)->i32{
        // java switch(index) { case 0: return flag | 1; case 1: return flag | 2; case 2: return flag | 4; case 3: return flag | 8; case 4: return flag | 16; default: return flag; }
        match index{0=>flag|1,1=>flag|2,2=>flag|4,3=>flag|8,4=>flag|16,_=>flag}
    }
    // java public static String translateDigits(String s, String digits) {
    pub fn translateDigits(s:&[u16],digits:&[u16])->Vec<u16>{
        // java assert digits.length() == 10;
        // rust: the original reference APK enables this Java assertion; retain
        // its UTF-16 length condition and unchecked AssertionError boundary.
        assert_eq!(digits.len(),10,"AssertionError");
        // java if ("0123456789".equals(digits)) { return s; }
        if digits.iter().copied().eq(Self::COMMON_DIGITS.encode_utf16()){return s.to_vec();}
        // java else if ("٠١٢٣٤٥٦٧٨٩".equals(digits)) { return processDigit(s, digits); }
        else if digits.iter().copied().eq(Self::ARABIC_DIGITS.encode_utf16()){return Self::processDigit(s,digits);}
        // java else if ("๐๑๒๓๔๕๖๗๘๙".equals(digits)) { return processDigit(s, digits); }
        else if digits.iter().copied().eq(Self::TAI_DIGITS.encode_utf16()){return Self::processDigit(s,digits);}
        // java else if ("०१२३४५६७८९".equals(digits)) { return processDigit(s, digits); }
        else if digits.iter().copied().eq(Self::HINDI_DIGITS.encode_utf16()){return Self::processDigit(s,digits);}
        // java int len = s.length(); StringBuilder sb = new StringBuilder(len);
        let len=s.len();let mut sb=Vec::with_capacity(len);
        // java for(int i = 0; i < len; ++i) {
        for i in 0..len{
            // java char c = s.charAt(i); int index = digits.indexOf(c);
            let c=s[i];let index=digits.iter().position(|d|*d==c);
            // java if (index >= 0) { sb.append((char)(48 + index)); } else { sb.append(c); }
            if let Some(index)=index{sb.push(48u16.wrapping_add(index as u16));}else{sb.push(c);}
        }
        // java return sb.toString();
        sb
    }
    // java private static String processDigit(String s, String digits) {
    fn processDigit(s:&[u16],digits:&[u16])->Vec<u16>{
        // java int len = s.length(); char zero = digits.charAt(0); char nine = digits.charAt(9); StringBuilder sb = new StringBuilder(len);
        let len=s.len();let zero=digits[0];let nine=digits[9];let mut sb=Vec::with_capacity(len);
        // java for(int i = 0; i < len; ++i) { char c = s.charAt(i); if (zero <= c && c <= nine) { sb.append((char)(48 + (c - zero))); } else { sb.append(c); } }
        for i in 0..len{let c=s[i];if zero<=c&&c<=nine{sb.push(48+(c-zero));}else{sb.push(c);}}
        // java return sb.toString();
        sb
    }
}
