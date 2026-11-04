// Java: import cn.wps.moffice.transaction.Atom
use util::moffice::transaction::Atom::*;
// Java: import cn.wps.moffice.transaction.AtomMember
use util::moffice::transaction::AtomMember::*;
// Java: import java.lang.Cloneable
use stubs::java::lang::Cloneable::*;
// Java: import cn.wps.moffice.writer.core.CloneNotSupportedException
use stubs::java::lang::CloneNotSupportedException::*;

// java public static final int MAX_LEVEL_COUNT = 9;
pub(crate) const MAX_LEVEL_COUNT: i32 = 9;
// java public static final int INVALID_TEMPLATE_CODE = -1;
pub(crate) const INVALID_TEMPLATE_CODE: i32 = -1;
// java public static final int INVALID_ISTD = 4095;
pub(crate) const INVALID_ISTD: i32 = 4095;

// java public class LSTF extends Atom implements Cloneable
#[derive(Clone, Default)]
pub struct LSTF {
    // java Atom super_
    pub(crate) super_: Atom,
    // java @AtomMember(1) private int[] rgtplc = new int[0];
    rgtplc: Option<Vec<i32>>,
    // java @AtomMember private int lsid;
    lsid: i32,
    // java @AtomMember private int tplc = -1;
    tplc: i32,
    // java @AtomMember(1) private short[] rgistd = new short[9];
    rgistd: Option<Vec<i16>>,
    // java @AtomMember private boolean fSimpleList;
    fSimpleList: bool,
    // java @AtomMember private boolean fRestartHdn;
    fRestartHdn: bool,
    // java @AtomMember private boolean fAutoNum;
    fAutoNum: bool,
    // java @AtomMember private boolean fHybrid;
    fHybrid: bool,
    // java @AtomMember private byte grfhic;
    grfhic: u8,
    // java @AtomMember private String name;
    name: Option<String>,
}

// java public class LSTF extends Atom implements Cloneable
impl LSTF {
    // java public LSTF clone()
    pub fn clone(&self) -> LSTF {
      // java LSTF lstf = (LSTF)super.clone();
      // rust: ★M-STAGE2 P1-D1 / T6 —— **修复模板误译**。
      //   Java `(LFO)super.clone()` 是 `Object.clone()` 浅拷贝后向下转型；
      //   模板却生成 `(self.super_.clone()).downcast_ref::<T>().unwrap()`：
      //   ① `Atom` 无 `downcast_ref`（非 `Any`）；② 即便有，克隆出的 `Atom`
      //   也不是子类实例。Rust 无继承，本结构以组合（`super_: Atom`）建模
      //   ⇒ 忠实等价为对**整个结构体**做浅克隆。
      let mut lstf: LSTF = Clone::clone(self);
      // java lstf.rgistd = (short[])this.rgistd.clone();
      lstf.rgistd = todo!("cast → Vec<i16>");
      // java lstf.rgtplc = (int[])this.rgtplc.clone();
      lstf.rgtplc = todo!("cast → Vec<i32>");
      // java return lstf;
      return lstf;
    }

    // java public void initRgTplc()
    pub fn initRgTplc(&mut self) {
      // java this.rgtplc = new int[9];
      self.rgtplc = Some(vec![Default::default(); 9usize]);
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java int i = 0;
      let mut i: i32 = 0;
      let mut len: i32 = (self.rgtplc.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java for(int len = this.rgtplc.length; i < len; ++i)
      while i < len {
        // java this.rgtplc[i] = -1;
        self.rgtplc.as_mut().unwrap()[(i) as usize] = -1;
        i += 1;
      }
    }

    // java public void initRgIstd()
    pub fn initRgIstd(&mut self) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java short[] rgistd = new short[9]  （字段初始化器；模板漏掉了分配）
      if self.rgistd.as_ref().map(|v| v.is_empty()).unwrap_or(true) {
        self.rgistd = Some(vec![0; 9]);
      }
      // java int i = 0;
      let mut i: i32 = 0;
      let mut len: i32 = (self.rgistd.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java for(int len = this.rgistd.length; i < len; ++i)
      while i < len {
        // java this.rgistd[i] = 4095;
        self.rgistd.as_mut().unwrap()[(i) as usize] = 4095;
        i += 1;
      }
    }

    // java public void initIstd(int level)
    pub fn initIstd(&mut self, level: i32) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.rgistd[level] = 4095;
      self.rgistd.as_mut().unwrap()[(level) as usize] = 4095;
    }

    // java public int getRgtplc(int level)
    pub fn getRgtplc__int(&self, level: i32) -> i32 {
      // java return this.rgtplc[level];
      return self.rgtplc.as_ref().unwrap()[(level) as usize];
    }

    // java public int[] getRgtplc()
    // rust: Java 返回可空引用 ⇒ Rust 返回 `Option<Vec<i32>>`。
    pub fn getRgtplc(&self) -> Vec<i32> {
      // java return this.rgtplc;
      return self.rgtplc.clone().unwrap_or_default();
    }

    // java public void setRgtplc(int[] rgtplc)
    pub fn setRgtplc__intarr(&mut self, rgtplc: &Vec<i32>) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.rgtplc = rgtplc;
      self.rgtplc = Some(rgtplc.clone());
    }

    // java public void setRgtplc(int index, int val)
    pub fn setRgtplc__int_int(&mut self, index: i32, val: i32) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.rgtplc[index] = val;
      self.rgtplc.as_mut().unwrap()[(index) as usize] = val;
    }

    // java public int getLsid()
    pub fn getLsid(&self) -> i32 {
      // java return this.lsid;
      return self.lsid;
    }

    // java public void setLsid(int lsid)
    pub fn setLsid(&mut self, lsid: i32) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.lsid = lsid;
      self.lsid = lsid;
    }

    // java public int getTplc()
    pub fn getTplc(&self) -> i32 {
      // java return this.tplc;
      return self.tplc;
    }

    // java public void setTplc(int tplc)
    pub fn setTplc(&mut self, tplc: i32) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.tplc = tplc;
      self.tplc = tplc;
    }

    // java public short getRgistd(int index)
    pub fn getRgistd__int(&self, index: i32) -> i16 {
      // java return this.rgistd[index];
      return self.rgistd.as_ref().unwrap()[(index) as usize];
    }

    // java public short[] getRgistd()
    pub fn getRgistd(&self) -> Vec<i16> {
      // java return this.rgistd;
      return self.rgistd.clone().unwrap_or_default();
    }

    // java public void setRgistd(short[] rgistd)
    pub fn setRgistd__shortarr(&mut self, rgistd: &Vec<i16>) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.rgistd = rgistd;
      self.rgistd = Some(rgistd.clone());
    }

    // java public void setRgistd(int index, short rgistd)
    pub fn setRgistd__int_short(&mut self, index: i32, rgistd: i16) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.rgistd[index] = rgistd;
      self.rgistd.as_mut().unwrap()[(index) as usize] = rgistd;
    }

    // java public boolean isfSimpleList()
    pub fn isfSimpleList(&self) -> bool {
      // java return this.fSimpleList;
      return self.fSimpleList;
    }

    // java public void setfSimpleList(boolean fSimpleList)
    pub fn setfSimpleList(&mut self, fSimpleList: bool) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.fSimpleList = fSimpleList;
      self.fSimpleList = fSimpleList;
    }

    // java public boolean isfRestartHdn()
    pub fn isfRestartHdn(&self) -> bool {
      // java return this.fRestartHdn;
      return self.fRestartHdn;
    }

    // java public void setfRestartHdn(boolean fRestartHdn)
    pub fn setfRestartHdn(&mut self, fRestartHdn: bool) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.fRestartHdn = fRestartHdn;
      self.fRestartHdn = fRestartHdn;
    }

    // java public boolean isfAutoNum()
    pub fn isfAutoNum(&self) -> bool {
      // java return this.fAutoNum;
      return self.fAutoNum;
    }

    // java public void setfAutoNum(boolean fAutoNum)
    pub fn setfAutoNum(&mut self, fAutoNum: bool) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.fAutoNum = fAutoNum;
      self.fAutoNum = fAutoNum;
    }

    // java public boolean isfHybrid()
    pub fn isfHybrid(&self) -> bool {
      // java return this.fHybrid;
      return self.fHybrid;
    }

    // java public void setfHybrid(boolean fHybrid)
    pub fn setfHybrid(&mut self, fHybrid: bool) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.fHybrid = fHybrid;
      self.fHybrid = fHybrid;
    }

    // java public byte getGrfhic()
    pub fn getGrfhic(&self) -> u8 {
      // java return this.grfhic;
      return self.grfhic;
    }

    // java public void setGrfhic(byte grfhic)
    pub fn setGrfhic(&mut self, grfhic: u8) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.grfhic = grfhic;
      self.grfhic = grfhic;
    }

    // java public String getName()
    pub fn getName(&self) -> String {
      // java return this.name;
      return self.name.clone().unwrap_or_default();
    }

    // java public void setName(String name)
    pub fn setName(&mut self, name: &str) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.name = name;
      self.name = Some(name.to_string());
    }

}

impl std::ops::Deref for LSTF {
    type Target = Atom;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for LSTF {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
