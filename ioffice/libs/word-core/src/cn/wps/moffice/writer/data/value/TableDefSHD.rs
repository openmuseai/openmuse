// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::SHD;

// java public abstract class TableDefSHD
// rust: ★T7 —— 本文件原为**自动生成的占位桩**（`// TODO: fields` + `todo!()`），
//   但其子类 `TableDefSHDBuilder` 真实使用 `mCount`/`shds` 两个字段
//   （Java `TableDefSHDBuilder.java:19,37-39`），`data::table::TableTool`
//   等亦依赖 `getCellSHD`。故按 `TableDefSHD.java` 完整补齐，
//   消除「桩 → 假编译面」链条。
#[derive(Clone)]
pub struct TableDefSHD {
    // java SHD[] shds = null;
    pub(crate) shds: Option<Vec<Option<SHD>>>,
    // java int mCount = 0;
    pub(crate) mCount: i32,
    // java int hash;
    pub(crate) hash: i32,
}

// java public abstract class TableDefSHD
impl TableDefSHD {
    // java public TableDefSHD(int nCount)
    // rust: Java 的 `shds = new SHD[nCount]` 会留 null 槽位 ⇒ `Vec<Option<SHD>>`。
    pub fn new(nCount: i32) -> Self {
        let mut this = Self { shds: None, mCount: nCount, hash: 0 };
        // java this.mCount = nCount;
        this.mCount = nCount;
        // java if (nCount >= 0)
        if nCount >= 0 {
            // java this.shds = new SHD[nCount];
            this.shds = Some(vec![None; nCount as usize]);
        }
        this
    }

    // java public int getCount()
    pub fn getCount(&self) -> i32 {
        // java return this.mCount;
        return self.mCount;
    }

    // java public SHD getCellSHD(int iCell)
    pub fn getCellSHD(&self, iCell: i32) -> Option<SHD> {
        // java return iCell >= 0 && iCell < this.mCount ? this.shds[iCell] : null;
        return if iCell >= 0 && iCell < self.mCount {
            self.shds.as_ref().and_then(|v| v[iCell as usize].clone())
        } else {
            None
        };
    }

    // java public void setCellSHD(int iCell, SHD shd)
    // rust: 非 Java 显式成员 —— Java 由子类直接写 `this.shds[iCell] = shd`
    //   （`TableDefSHDBuilder.java:39`）；Rust 字段为 `pub(crate)`，且需越界防护，
    //   故以该辅助方法承载同一次写操作，行为与 Java 一致。
    pub(crate) fn setCellSHD(&mut self, iCell: i32, shd: Option<SHD>) {
        // java this.shds[iCell] = shd;
        if let Some(v) = self.shds.as_mut() {
            if iCell >= 0 && (iCell as usize) < v.len() {
                v[iCell as usize] = shd;
            }
        }
    }

    // java public boolean equals(Object rhs)
    pub fn equals(&self, rhs: &dyn std::any::Any) -> bool {
        // java if (rhs == this) { return true; }
        // java else if (!(rhs instanceof TableDefSHD)) { return false; }
        let r = match rhs.downcast_ref::<TableDefSHD>() {
            None => {
                // java return false;
                return false;
            }
            Some(r) => r,
        };
        // java if (this.mCount != r.mCount)
        if self.mCount != r.mCount {
            // java return false;
            return false;
        }
        // java else
        {
            // java for(int i = 0; i < this.mCount; ++i)
            for i in 0..self.mCount {
                // java SHD shd1 = this.shds[i];
                let shd1 = self.getCellSHD(i);
                // java SHD shd2 = r.shds[i];
                let shd2 = r.getCellSHD(i);
                // java if (null != shd1 || null != shd2)
                if shd1.is_some() || shd2.is_some() {
                    // java if (null == shd1 && null != shd2) { return false; }
                    if shd1.is_none() && shd2.is_some() {
                        // java return false;
                        return false;
                    }
                    // java if (null != shd1 && !shd1.equals(shd2)) { return false; }
                    if let Some(s1) = shd1.as_ref() {
                        let equal = match shd2.as_ref() {
                            Some(s2) => s1.equals(s2),
                            None => false,
                        };
                        if !equal {
                            // java return false;
                            return false;
                        }
                    }
                }
            }
            // java return true;
            return true;
        }
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
        // java if (this.hash == 0)
        if self.hash == 0 {
            // java int h = this.mCount;
            let mut h: i32 = self.mCount;
            // java for(int i = 0; i < this.mCount; ++i)
            for i in 0..self.mCount {
                // java SHD shd = this.shds[i];
                let shd = self.getCellSHD(i);
                // java if (shd != null)
                if let Some(shd) = shd {
                    // java h += shd.hashCode();
                    h += shd.hashCode();
                }
            }
            // java this.hash = h;
            // rust: `hashCode(&self)` 只读，Java 在此做缓存写；缓存不影响返回值
            //   ⇒ 直接返回 `h`，与 Java 首次调用结果一致。
            return h;
        }
        // java return this.hash;
        return self.hash;
    }
}

// rust: 便于 `TableDefSHDBuilder::new()`（无参）构造空实例。
impl Default for TableDefSHD {
    fn default() -> Self {
        Self { shds: None, mCount: 0, hash: 0 }
    }
}
