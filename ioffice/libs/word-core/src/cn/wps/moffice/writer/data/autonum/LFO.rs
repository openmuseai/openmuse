// Java: import java.lang.Cloneable
use stubs::java::lang::Cloneable::*;
// Java: import cn.wps.moffice.writer.core.CloneNotSupportedException
use stubs::java::lang::CloneNotSupportedException::*;

// java public class LFO implements Cloneable
#[derive(Clone, Default)]
pub struct LFO {
    // java public int lsid;
    pub(crate) lsid: i32,
    // java public byte clfolvl;
    pub(crate) clfolvl: u8,
    // java public byte ibstFltAutoNum;
    pub(crate) ibstFltAutoNum: u8,
    // java public byte grfhic;
    pub(crate) grfhic: u8,
}

// java public class LFO implements Cloneable
impl LFO {
    // java public LFO clone()
    pub fn clone(&self) -> LFO {
      // java LFO lfo = (LFO)super.clone();
      // rust: ★M-STAGE2 P1-D1 / T6 —— **修复模板误译**。
      //   Java `(LFO)super.clone()` 是 `Object.clone()` 浅拷贝后向下转型；
      //   模板却生成 `(self.super_.clone()).downcast_ref::<T>().unwrap()`：
      //   ① `Atom` 无 `downcast_ref`（非 `Any`）；② 即便有，克隆出的 `Atom`
      //   也不是子类实例。Rust 无继承，本结构以组合（`super_: Atom`）建模
      //   ⇒ 忠实等价为对**整个结构体**做浅克隆。
      let mut lfo: LFO = Clone::clone(self);
      // java return lfo;
      return lfo;
    }

}
