// Java: import java.util.HashMap
use std::collections::HashMap;

// Java field: short DBNUM0
pub(crate) const DBNUM0: i16 = 0;
// Java field: short DBNUM1
pub(crate) const DBNUM1: i16 = 1;
// Java field: short DBNUM2
pub(crate) const DBNUM2: i16 = 2;
// Java field: short DBNUM3
pub(crate) const DBNUM3: i16 = 3;
// Java field: int SC_CN
pub(crate) const SC_CN: i32 = 0;
// Java field: int TC_CN
pub(crate) const TC_CN: i32 = 1;
// Java field: int JP
pub(crate) const JP: i32 = 2;
// Java field: int KOREAN
pub(crate) const KOREAN: i32 = 3;
// Java field: HashMap<Short, Integer> mLocaleCodeMap
fn get_mLocaleCodeMap() -> &'static HashMap<i16, i32> {
    static MAP: std::sync::OnceLock<HashMap<i16, i32>> = std::sync::OnceLock::new();
    MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(2052, 0);
        m.insert(1028, 1);
        m.insert(1041, 2);
        m.insert(1042, 3);
        m
    })
}
// Java field: int LOCALE_COUNT
pub(crate) const LOCALE_COUNT: i32 = 23;
// Java field: int DBNUM_COUNT
pub(crate) const DBNUM_COUNT: i32 = 18;
// Java field: byte[][] DBNumLCID
fn get_DBNumLCID() -> &'static Vec<Vec<i8>> {
    static DATA: std::sync::OnceLock<Vec<Vec<i8>>> = std::sync::OnceLock::new();
    DATA.get_or_init(|| {
        let mut lcid = vec![vec![0i8; 18]; 23];
        lcid[0][1] = 30;
        lcid[0][2] = 31;
        lcid[0][3] = 32;
        lcid[1][1] = 33;
        lcid[1][2] = 34;
        lcid[1][3] = 35;
        lcid[2][1] = 27;
        lcid[2][2] = 28;
        lcid[2][3] = 29;
        lcid[3][1] = 36;
        lcid[3][2] = 37;
        lcid[3][3] = 38;
        lcid
    })
}

// Java: public class DBNumAccesser
pub struct DBNumAccesser {
}

// Java: public class DBNumAccesser
impl DBNumAccesser {
    // Java: DBNumAccesser()
    pub fn new() -> Self {
      // Java: super();
      DBNumAccesser {}
    }

    // Java: static byte transDBNumLocale(short lcid, String dbNum)
    pub fn transDBNumLocale(lcid: i16, db_num: &str) -> i8 {
      // Java: assert dbNum.matches("\\d+");
      debug_assert!(db_num.chars().all(|c| c.is_ascii_digit()));
      // Java: Integer rowIdx = (Integer)mLocaleCodeMap.get(lcid);
      let row_idx = get_mLocaleCodeMap().get(&lcid).copied();
      // Java: int colIdx = dbnum2int(dbNum);
      let col_idx = Self::dbnum2int(db_num);
      // Java: return rowIdx != null && rowIdx < 23 && colIdx <= 18 ? DBNumLCID[rowIdx][colIdx] : 0;
      match row_idx {
          Some(r) if r < 23 && col_idx <= 18 => get_DBNumLCID()[r as usize][col_idx as usize],
          _ => 0,
      }
    }

    // Java: static int dbnum2int(String dbnum)
    fn dbnum2int(dbnum: &str) -> i32 {
      // Java: return dbnum.charAt(0) - 48;
      dbnum.as_bytes()[0] as i32 - 48
    }

}
