// java public class NFFunDefine
#[derive(Default)]
pub struct NFFunDefine {
}

// java  class NFSTYLE
#[derive(Default)]
pub struct NFSTYLE {
}

// java  class NF_FORMAT_PARAM
#[derive(Default)]
pub struct NF_FORMAT_PARAM {
    // java public long lMaxcch = -1L;
    pub(crate) lMaxcch: i64,
    // java public long dwOpt = 0L;
    pub(crate) dwOpt: i64,
    // java public long lLanguage = -1L;
    pub(crate) lLanguage: i64,
}

// java public class NFFunDefine
impl NFFunDefine {
    // java public static int addGen(int a)
    pub const fn addGen(a: i32) -> i32 {
      // java return 16384 | a;
      return 16384 | a;
    }

    // java public static int addZeroCount(int a, int n)
    pub const fn addZeroCount(a: i32, n: i32) -> i32 {
      // java return a | n << 11;
      return a | n << 11;
    }

    // java public static int pickDBNum(int a)
    pub const fn pickDBNum(a: i32) -> i32 {
      // java return 1023 & a;
      return 1023 & a;
    }

    // java public static int pickGen(int a)
    pub const fn pickGen(a: i32) -> i32 {
      // java return 16384 & a;
      return 16384 & a;
    }

    // java public static int pickZeroCount(int a)
    pub const fn pickZeroCount(a: i32) -> i32 {
      // java return (14336 & a) >> 11;
      return (14336 & a) >> 11;
    }

    // java public static int pickBullet(int a)
    pub const fn pickBullet(a: i32) -> i32 {
      // java return 1024 & a;
      return 1024 & a;
    }

    // java public static int getDbNum(int nfc)
    pub const fn getDbNum(nfc: i32) -> i32 {
      // java return pickDBNum(nfc);
      return Self::pickDBNum(nfc);
    }

    // java public static boolean isAddGen(int nfc)
    pub const fn isAddGen(nfc: i32) -> bool {
      // java return pickGen(nfc) != 0;
      return Self::pickGen(nfc) != 0;
    }

    // java public static int getZeroCount(int nfc)
    pub const fn getZeroCount(nfc: i32) -> i32 {
      // java return pickZeroCount(nfc);
      return Self::pickZeroCount(nfc);
    }

    // java public static boolean isBullet(int nfc)
    pub const fn isBullet(nfc: i32) -> bool {
      // java return pickBullet(nfc) != 0;
      return Self::pickBullet(nfc) != 0;
    }

}

// java  class NFSTYLE
impl NFSTYLE {
}

// java  class NF_FORMAT_PARAM
impl NF_FORMAT_PARAM {
    // java public  NF_FORMAT_PARAM()
    pub fn NF_FORMAT_PARAM() -> Self {
      Self { lMaxcch: -1, dwOpt: 0, lLanguage: -1 }
    }

}
