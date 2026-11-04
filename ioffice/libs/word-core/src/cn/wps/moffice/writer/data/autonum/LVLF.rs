// Java: import java.lang.Cloneable
use stubs::java::lang::Cloneable::*;
// Java: import cn.wps.moffice.writer.core.CloneNotSupportedException
use stubs::java::lang::CloneNotSupportedException::*;

// java public static final byte IXCHFOLLOW_TAB = 0;
pub(crate) const IXCHFOLLOW_TAB: u8 = 0;
// java public static final byte IXCHFOLLOW_SPACING = 1;
pub(crate) const IXCHFOLLOW_SPACING: u8 = 1;
// java public static final byte IXCHFOLLOW_NOTHING = 2;
pub(crate) const IXCHFOLLOW_NOTHING: u8 = 2;

// java public class LVLF implements Cloneable
#[derive(Clone, Default)]
pub struct LVLF {
    // java public int iStartAt;
    pub(crate) iStartAt: i32,
    // java public byte nfc;
    pub(crate) nfc: u8,
    // java public byte jc;
    pub(crate) jc: u8,
    // java public boolean fLegal;
    pub(crate) fLegal: bool,
    // java public boolean fNoRestart;
    pub(crate) fNoRestart: bool,
    // java public boolean fIndentSav;
    pub(crate) fIndentSav: bool,
    // java public boolean fConverted;
    pub(crate) fConverted: bool,
    // java public boolean fTentative;
    pub(crate) fTentative: bool,
    // java public byte[] rgbxchNums = new byte[9];
    pub(crate) rgbxchNums: Option<Vec<u8>>,
    // java public byte ixchFollow;
    pub(crate) ixchFollow: u8,
    // java public int dxaIndentSav;
    pub(crate) dxaIndentSav: i32,
    // java public short cbGrpprlChpx;
    pub(crate) cbGrpprlChpx: i16,
    // java public short cbGrpprlPapx;
    pub(crate) cbGrpprlPapx: i16,
    // java public byte iRestartAfterLevel;
    pub(crate) iRestartAfterLevel: u8,
    // java public byte grfhic;
    pub(crate) grfhic: u8,
}

// java public class LVLF implements Cloneable
impl LVLF {
    // java public LVLF clone()
    pub fn clone(&self) -> LVLF {
      // java return (LVLF)super.clone();
      // rust: ★M-STAGE2 P1-D1 / T6 —— `LVLF` **不继承 `Atom`**（Java 为
      //   `public class LVLF implements Cloneable`），模板却生成 `self.super_`。
      //   忠实等价为对整个结构体做浅克隆。
      return Clone::clone(self);
    }

}
