// Java: import java.lang.Cloneable
use stubs::java::lang::Cloneable::*;
// Java: import cn.wps.moffice.writer.core.CloneNotSupportedException
use stubs::java::lang::CloneNotSupportedException::*;

// java public class LFOLVL implements Cloneable
#[derive(Clone, Default)]
pub struct LFOLVL {
    // java public int iStartAt;
    pub(crate) iStartAt: i32,
    // java public byte ilvl;
    pub(crate) ilvl: u8,
    // java public boolean fStartAt;
    pub(crate) fStartAt: bool,
    // java public boolean fFormatting;
    pub(crate) fFormatting: bool,
    // java public byte grfhic;
    pub(crate) grfhic: u8,
}

// java public class LFOLVL implements Cloneable
impl LFOLVL {
    // java protected LFOLVL clone()
    pub fn clone(&self) -> LFOLVL {
      // java LFOLVL lfolvl = (LFOLVL)super.clone();
      // rust: ★M-STAGE2 P1-D1 / T6 —— **修复模板误译**。
      //   Java `(LFO)super.clone()` 是 `Object.clone()` 浅拷贝后向下转型；
      //   模板却生成 `(self.super_.clone()).downcast_ref::<T>().unwrap()`：
      //   ① `Atom` 无 `downcast_ref`（非 `Any`）；② 即便有，克隆出的 `Atom`
      //   也不是子类实例。Rust 无继承，本结构以组合（`super_: Atom`）建模
      //   ⇒ 忠实等价为对**整个结构体**做浅克隆。
      let mut lfolvl: LFOLVL = Clone::clone(self);
      // java return lfolvl;
      return lfolvl;
    }

}
