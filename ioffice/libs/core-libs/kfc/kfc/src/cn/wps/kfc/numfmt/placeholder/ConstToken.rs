#![allow(
    non_snake_case,
    non_camel_case_types,
    unused_variables,
    unused_imports,
    dead_code,
    unreachable_code,
    unused_mut
)]

// Java: import java.util.Hashtable
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// Java field: char TOKEN_W_X
pub(crate) const TOKEN_W_X: char = 'x';
// Java field: char TOKEN_W_SQUATATION
pub(crate) const TOKEN_W_SQUATATION: char = '\'';
// Java field: char TOKEN_W_APOSTROPHE
pub(crate) const TOKEN_W_APOSTROPHE: char = '`';
// Java field: char TOKEN_W_SUB
pub(crate) const TOKEN_W_SUB: char = '-';
// Java field: char TOKEN_W_ADD
pub(crate) const TOKEN_W_ADD: char = '+';
// Java field: char TOKEN_W_O
pub(crate) const TOKEN_W_O: char = 'O';
// Java field: char TOKEN_W_N
pub(crate) const TOKEN_W_N: char = 'n';
// Java field: char TOKEN_W_E
pub(crate) const TOKEN_W_E: char = 'E';
// Java field: char TOKEN_W_B
pub(crate) const TOKEN_W_B: char = 'B';
// Java field: char TOKEN_W_R
pub(crate) const TOKEN_W_R: char = 'r';
// Java field: char TOKEN_W_M
pub(crate) const TOKEN_W_M: char = 'M';
// Java field: char TOKEN_W_A
pub(crate) const TOKEN_W_A: char = 'A';
// Java field: char TOKEN_W_W
pub(crate) const TOKEN_W_W: char = 'W';
// Java field: char[] CASE_SENSITIVE
static CASE_SENSITIVE: OnceLock<Vec<char>> = OnceLock::new();
// Java field: char TOKEN_SHARP
pub(crate) const TOKEN_SHARP: char = '#';
// Java field: char TOKEN_ZERO
pub(crate) const TOKEN_ZERO: char = '0';
// Java field: char TOKEN_ASK
pub(crate) const TOKEN_ASK: char = '?';
// Java field: char TOKEN_PERCENT
pub(crate) const TOKEN_PERCENT: char = '%';
// Java field: char TOKEN_FRACTION
pub(crate) const TOKEN_FRACTION: char = '/';
// Java field: char TOKEN_ESCAPE
pub(crate) const TOKEN_ESCAPE: char = '\\';
// Java field: char TOKEN_ESCAPE_EX
pub(crate) const TOKEN_ESCAPE_EX: char = '!';
// Java field: char TOKEN_DOUBLE_QUATATION
pub(crate) const TOKEN_DOUBLE_QUATATION: char = '"';
// Java field: char TOKEN_SINGLE_QUATATION
pub(crate) const TOKEN_SINGLE_QUATATION: char = '\'';
// Java field: char TOKEN_ASTERISK
pub(crate) const TOKEN_ASTERISK: char = '*';
// Java field: char TOKEN_UNDERLINE
pub(crate) const TOKEN_UNDERLINE: char = '_';
// Java field: char TOKEN_TEXT
pub(crate) const TOKEN_TEXT: char = '@';
// Java field: char TOKEN_PERIOD
pub(crate) const TOKEN_PERIOD: char = '.';
// Java field: char TOKEN_COMMA
pub(crate) const TOKEN_COMMA: char = ',';
// Java field: char TOKEN_COLON
pub(crate) const TOKEN_COLON: char = ':';
// Java field: char Token_1
pub(crate) const Token_1: char = '1';
// Java field: char Token_2
pub(crate) const Token_2: char = '2';
// Java field: char Token_3
pub(crate) const Token_3: char = '3';
// Java field: char Token_4
pub(crate) const Token_4: char = '4';
// Java field: char Token_5
pub(crate) const Token_5: char = '5';
// Java field: char Token_6
pub(crate) const Token_6: char = '6';
// Java field: char Token_7
pub(crate) const Token_7: char = '7';
// Java field: char Token_8
pub(crate) const Token_8: char = '8';
// Java field: char Token_9
pub(crate) const Token_9: char = '9';
// Java field: char TOKEN_y
pub(crate) const TOKEN_y: char = 'y';
// Java field: char TOKEN_m
pub(crate) const TOKEN_m: char = 'm';
// Java field: char TOKEN_d
pub(crate) const TOKEN_d: char = 'd';
// Java field: char TOKEN_h
pub(crate) const TOKEN_h: char = 'h';
// Java field: char TOKEN_s
pub(crate) const TOKEN_s: char = 's';
// Java field: char TOKEN_CH_AP
pub(crate) const TOKEN_CH_AP: char = '\u{4E0A}';
// Java field: String TOKEN_CH_AMPM
pub(crate) const TOKEN_CH_AMPM: &str = "\u{4E0A}\u{5348}/\u{4E0B}\u{5348}";
// Java field: String[] TOKEN_CH_AP_CHILDREN
pub(crate) static TOKEN_CH_AP_CHILDREN: OnceLock<Vec<String>> = OnceLock::new();
// Java field: char TOKEN_a
pub(crate) const TOKEN_a: char = 'a';
// Java field: String TOKEN_a_ap
pub(crate) const TOKEN_a_ap: &str = "a/p";
// Java field: String TOKEN_a_ampm
pub(crate) const TOKEN_a_ampm: &str = "am/pm";
// Java field: String[] TOKEN_a_CHILDREN
pub(crate) static TOKEN_a_CHILDREN: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String TOKEN_w_AMPM
pub(crate) const TOKEN_w_AMPM: &str = "AMPM";
// Java field: String[] TOKEN_W_a_CHILDREN
pub(crate) static TOKEN_W_a_CHILDREN: OnceLock<Vec<String>> = OnceLock::new();
// Java field: char TOKEN_b
pub(crate) const TOKEN_b: char = 'b';
// Java field: String TOKEN_B1
pub(crate) const TOKEN_B1: &str = "B1";
// Java field: String TOKEN_B2
pub(crate) const TOKEN_B2: &str = "B2";
// Java field: String[] TOKEN_b_CHILDREN
pub(crate) static TOKEN_b_CHILDREN: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
// Java field: char TOKEN_TH_DAY
pub(crate) const TOKEN_TH_DAY: char = '\u{0E27}';
// Java field: char TOKEN_TH_MONTH
pub(crate) const TOKEN_TH_MONTH: char = '\u{0E14}';
// Java field: char TOKEN_TH_YEAR
pub(crate) const TOKEN_TH_YEAR: char = '\u{0E1B}';
// Java field: char TOKEN_TH_HOUR
pub(crate) const TOKEN_TH_HOUR: char = '\u{0E21}';
// Java field: char TOKEN_TH_MINUTE
pub(crate) const TOKEN_TH_MINUTE: char = '\u{0E19}';
// Java field: char TOKEN_TH_SECOND
pub(crate) const TOKEN_TH_SECOND: char = '\u{0E17}';
// Java field: char TOKEN_e
pub(crate) const TOKEN_e: char = 'e';
// Java field: String TOKEN_SCIENCE_E_ADD
pub(crate) const TOKEN_SCIENCE_E_ADD: &str = "E+";
// Java field: String TOKEN_SCIENCE_E_SUB
pub(crate) const TOKEN_SCIENCE_E_SUB: &str = "E-";
// Java field: String TOKEN_SCIENCE_E
pub(crate) const TOKEN_SCIENCE_E: &str = "E";
// Java field: String[] TOKEN_e_CHILDREN
static TOKEN_e_CHILDREN: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
// Java field: String[] TOKEN_SUPER_e_CHILDREN
pub(crate) static TOKEN_SUPER_e_CHILDREN: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
// Java field: char TOKEN_g
pub(crate) const TOKEN_g: char = 'g';
// Java field: String TOKEN_g_General
pub(crate) const TOKEN_g_General: &str = "General";
// Java field: String[] TOKEN_g_CHILDREN
pub(crate) static TOKEN_g_CHILDREN: OnceLock<Vec<String>> = OnceLock::new();
// Java field: char TOKEN_BRACKET
pub(crate) const TOKEN_BRACKET: char = '[';
// Java field: String TOKEN_NCL
pub(crate) const TOKEN_NCL: &str = "\\$(?:[^-]*)(?:-([0-9A-F]+))?";
// Java field: String TOKEN_COLOR
pub(crate) const TOKEN_COLOR: &str = "Black|Blue|Cyan|Green|Magenta|Red|White|Yellow|Color(\\d+)";
// Java field: String TOKEN_DBNUM
pub(crate) const TOKEN_DBNUM: &str = "DBNum(\\d+)";
// Java field: String TOKEN_CONDITION
pub(crate) const TOKEN_CONDITION: &str = "=|<>|>|<|>=|<=";
// Java field: String TOKEN_h_ELAPSED
pub(crate) const TOKEN_h_ELAPSED: &str = "h+";
// Java field: String TOKEN_m_ELAPSED
pub(crate) const TOKEN_m_ELAPSED: &str = "m+";
// Java field: String TOKEN_s_ELAPSED
pub(crate) const TOKEN_s_ELAPSED: &str = "s+";
// Java field: int TOKEN_AREA_ID
pub(crate) const TOKEN_AREA_ID: i32 = 0;
// Java field: int TOKEN_COLOR_ID
pub(crate) const TOKEN_COLOR_ID: i32 = 1;
// Java field: int TOKEN_DBNUM_ID
pub(crate) const TOKEN_DBNUM_ID: i32 = 2;
// Java field: int TOKEN_CONDITION_ID
pub(crate) const TOKEN_CONDITION_ID: i32 = 3;
// Java field: int TOKEN_h_ELAPSED_ID
pub(crate) const TOKEN_h_ELAPSED_ID: i32 = 4;
// Java field: int TOKEN_m_ELAPSED_ID
pub(crate) const TOKEN_m_ELAPSED_ID: i32 = 5;
// Java field: int TOKEN_s_ELAPSED_ID
pub(crate) const TOKEN_s_ELAPSED_ID: i32 = 6;
// Java field: int TOKEN_BRACKET_COUNT
pub(crate) const TOKEN_BRACKET_COUNT: i32 = 7;
// Java field: String[] TOKEN_BRACKET_CHILDREN
pub(crate) static TOKEN_BRACKET_CHILDREN: OnceLock<Vec<String>> = OnceLock::new();
// Java field: TokenChildMap TokenChildtoHolder
pub(crate) static TOKEN_CHILD_TO_HOLDER: OnceLock<TokenChildMap> = OnceLock::new();
// Java field: TokenChildMap WTokenChildtoHolder
pub(crate) static W_TOKEN_CHILD_TO_HOLDER: OnceLock<TokenChildMap> = OnceLock::new();

// Java: public class ConstToken
pub struct ConstToken {}

// Java: public class TokenChildMap
#[derive(Clone)]
pub struct TokenChildMap {
    // Java field: Hashtable<String, Short> mChildtoHolder
    pub(crate) mChildtoHolder: SharedChildTable,
}
// Java Hashtable reference adapter: cloned native references retain the same
// table so already retained locale pieces see the original reset mutations.
#[derive(Clone,Default)]
pub(crate) struct SharedChildTable(std::sync::Arc<Mutex<HashMap<String,i16>>>);
impl SharedChildTable{
    pub fn insert(&self,key:String,value:i16)->Option<i16>{self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(key,value)}
    pub fn clear(&self){self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clear();}
    pub fn get(&self,key:&str)->Option<i16>{self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(key).copied()}
}

// Java: public class ConstToken
impl ConstToken {
    // Java: ConstToken()
    pub fn new() -> Self {
        // Java: super();
        ConstToken {}
    }
}

// Java: public class TokenChildMap
impl TokenChildMap {
    // Java: TokenChildMap()
    pub fn new() -> Self {
        // Java: super();
        TokenChildMap {
            mChildtoHolder: SharedChildTable::default(),
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.mChildtoHolder.clear();
        self.mChildtoHolder.clear();
    }

    // Java: short tokenChildToHolder(String token)
    pub fn tokenChildToHolder(&self, token: Option<&str>) -> i16 {
        // Java: Short id = token != null ? (Short)this.mChildtoHolder.get(token) : null;
        // Java: return id != null ? id : -1;
        token
            .and_then(|t| self.mChildtoHolder.get(t))
            .unwrap_or(-1)
    }
}

impl TokenChildMap {
    pub fn excel() -> Self {
        let mut map = Self::new();
        map.mChildtoHolder.insert("General".into(), 1);
        map.mChildtoHolder.insert("a/p".into(), 34);
        map.mChildtoHolder.insert("am/pm".into(), 34);
        map.mChildtoHolder.insert("E+".into(), 7);
        map.mChildtoHolder.insert("E-".into(), 7);
        map.mChildtoHolder.insert("E".into(), 7);
        map.mChildtoHolder.insert("B1".into(), 22);
        map.mChildtoHolder.insert("B2".into(), 22);
        map.mChildtoHolder
            .insert("\\$(?:[^-]*)(?:-([0-9A-F]+))?".into(), 18);
        map.mChildtoHolder.insert(
            "Black|Blue|Cyan|Green|Magenta|Red|White|Yellow|Color(\\d+)".into(),
            16,
        );
        map.mChildtoHolder.insert("DBNum(\\d+)".into(), 19);
        map.mChildtoHolder.insert("=|<>|>|<|>=|<=".into(), 17);
        map.mChildtoHolder.insert("h+".into(), 29);
        map.mChildtoHolder.insert("m+".into(), 30);
        map.mChildtoHolder.insert("s+".into(), 31);
        map.mChildtoHolder.insert("上午/下午".into(), 34);
        map
    }
}
impl TokenChildMap {
// java WTokenChildtoHolder = new TokenChildMap(); WTokenChildtoHolder.mChildtoHolder.put("a/p",(short)55); WTokenChildtoHolder.mChildtoHolder.put("am/pm",(short)55); WTokenChildtoHolder.mChildtoHolder.put("AMPM",(short)54);
pub fn word()->Self{let mut map=Self::new();map.mChildtoHolder.insert("a/p".into(),55);map.mChildtoHolder.insert("am/pm".into(),55);map.mChildtoHolder.insert("AMPM".into(),54);map}}
