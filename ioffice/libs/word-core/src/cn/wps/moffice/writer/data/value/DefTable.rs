// Java: import cn.wps.moffice.writer.data.value.TableCell
use crate::cn::wps::moffice::writer::data::value::TableCell::TableCell;
// Java: import cn.wps.moffice.writer.data.value.TableCellBuilder
use crate::cn::wps::moffice::writer::data::value::TableCellBuilder::TableCellBuilder;
// Java: import cn.wps.moffice.writer.data.value.DefTableBuilder
use crate::cn::wps::moffice::writer::data::value::DefTableBuilder::DefTableBuilder;
use std::sync::LazyLock;

// java public static final DefTable empty = new DefTableBuilder((DefTable)null);
pub static empty: LazyLock<DefTableBuilder> = LazyLock::new(|| DefTableBuilder::obtain());

// java public abstract class DefTable
#[derive(Clone)]
pub struct DefTable {
    // java int itc;
    pub(crate) itc: i32,
    // java int itMax;
    pub(crate) itMax: i32,
    // java int[] rgdxaCenter = null;
    pub(crate) rgdxaCenter: Option<Vec<i32>>,
    // java TableCellBuilder[] rgtc = null;
    pub(crate) rgtc: Option<Vec<Option<TableCellBuilder>>>,
    // java int hash = 0;
    pub(crate) hash: i32,
}

impl Default for DefTable {
    fn default() -> Self {
        Self::new()
    }
}

// java public abstract class DefTable
impl DefTable {
    // java public  DefTable()
    pub fn new() -> Self {
      Self {
          itc: 0,
          itMax: 0,
          rgdxaCenter: None,
          rgtc: None,
          hash: 0,
      }
    }

    // java protected void initDefault()
    pub fn initDefault(&mut self) {
      // java this.itc = 0;
      self.itc = 0;
      // java this.itMax = 0;
      self.itMax = 0;
      // java if (this.rgdxaCenter != null)
      if let Some(rgdxaCenter) = self.rgdxaCenter.as_mut() {
        // java Arrays.fill(this.rgdxaCenter, 0);
        rgdxaCenter.fill(0);
      } else {
        // java this.rgdxaCenter = new int[0];
        self.rgdxaCenter = Some(Vec::new());
      }
      // java if (this.rgtc != null)
      if let Some(rgtc) = self.rgtc.as_mut() {
        // java Arrays.fill(this.rgtc, (Object)null);
        for cell in rgtc.iter_mut() {
          *cell = None;
        }
      } else {
        // java this.rgtc = new TableCellBuilder[0];
        self.rgtc = Some(Vec::new());
      }
      // java this.hash = 0;
      self.hash = 0;
    }

    // java public  DefTable(DefTable defTables)
    pub fn new__deftable(defTables: Option<&DefTable>) -> Self {
      // java this.init(defTables, (int[])null);
      let mut this = Self::new();
      this.init(defTables, None);
      this
    }

    // java public  DefTable(DefTable defTables, int[] rgdxaCenter)
    pub fn new__deftable_intarr(defTables: Option<&DefTable>, rgdxaCenter: Option<&Vec<i32>>) -> Self {
      // java this.init(defTables, rgdxaCenter);
      let mut this = Self::new();
      this.init(defTables, rgdxaCenter);
      this
    }

    // java public void init(DefTable defTables, int[] rgdxaCenter)
    pub fn init(&mut self, defTables: Option<&DefTable>, rgdxaCenter: Option<&Vec<i32>>) {
      // java if (defTables == null)
      let Some(defTables) = defTables else {
        // java this.initDefault();
        self.initDefault();
        return;
      };
      let src_lines: Vec<i32> = if rgdxaCenter.map(|v| v.len() as i32).unwrap_or(0) < defTables.itMax {
        // java rgdxaCenter = defTables.rgdxaCenter;
        defTables.rgdxaCenter.clone().unwrap_or_default()
      } else {
        rgdxaCenter.cloned().unwrap_or_default()
      };
      // java this.itc = defTables.itc;
      self.itc = defTables.itc;
      // java this.itMax = defTables.itMax;
      self.itMax = defTables.itMax;
      // java if (this.rgdxaCenter == null || this.rgdxaCenter.length < defTables.itMax)
      if self.rgdxaCenter.as_ref().map(|v| v.len() as i32).unwrap_or(0) < defTables.itMax {
        // java this.rgdxaCenter = new int[defTables.itMax];
        self.rgdxaCenter = Some(vec![0; defTables.itMax.max(0) as usize]);
      }
      // java System.arraycopy(rgdxaCenter, 0, this.rgdxaCenter, 0, defTables.itMax);
      if let Some(dst) = self.rgdxaCenter.as_mut() {
        let n = (defTables.itMax.max(0) as usize).min(src_lines.len()).min(dst.len());
        dst[..n].copy_from_slice(&src_lines[..n]);
      }
      // java if (this.rgtc == null || this.rgtc.length < defTables.itc)
      if self.rgtc.as_ref().map(|v| v.len() as i32).unwrap_or(0) < defTables.itc {
        // java this.rgtc = new TableCellBuilder[defTables.itc];
        self.rgtc = Some(vec![None; defTables.itc.max(0) as usize]);
      }
      // java int i = 0;
      let mut i: i32 = 0;
      // java for(int size = defTables.itc; i < size; ++i)
      let size = defTables.itc;
      while i < size {
        // java this.rgtc[i] = TableCellBuilder.obtain(defTables.rgtc[i]);
        let src_cell = defTables
            .rgtc
            .as_ref()
            .and_then(|v| v.get(i as usize))
            .and_then(|c| c.as_ref());
        let built = if let Some(cell) = src_cell {
          TableCellBuilder::obtain__tablecell(cell)
        } else {
          TableCellBuilder::obtain()
        };
        if let Some(rgtc) = self.rgtc.as_mut() {
          if (i as usize) < rgtc.len() {
            rgtc[i as usize] = Some(built);
          }
        }
        i += 1;
      }
      // java this.hash = 0;
      self.hash = 0;
    }

    // java public int getItMax()
    pub fn getItMax(&self) -> i32 {
      // java return this.itMax;
      self.itMax
    }

    // java public int getCellCount()
    pub fn getCellCount(&self) -> i32 {
      // java return this.itc;
      self.itc
    }

    // java public int getRgdxa(int it)
    pub fn getRgdxa(&self, it: i32) -> i32 {
      // java return it >= 0 && it < this.itMax ? this.rgdxaCenter[it] : -108;
      if it >= 0 && it < self.itMax {
        self.rgdxaCenter
            .as_ref()
            .and_then(|v| v.get(it as usize).copied())
            .unwrap_or(-108)
      } else {
        -108
      }
    }

    // java public TableCell getTableCell(int it)
    pub fn getTableCell(&self, it: i32) -> Option<&TableCell> {
      // java return it >= 0 && it < this.itc ? this.rgtc[it] : null;
      if it >= 0 && it < self.itc {
        self.rgtc
            .as_ref()
            .and_then(|v| v.get(it as usize))
            .and_then(|c| c.as_ref())
            .map(|b| &b.super_)
      } else {
        None
      }
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java StringBuilder sb = new StringBuilder();
      let mut sb = String::new();
      // java sb.append(this.getClass().getSimpleName());
      sb.push_str("DefTable");
      // java sb.append("\nitc = " + this.itc);
      sb.push_str(&format!("\nitc = {}", self.itc));
      // java sb.append("\nitcMax = " + this.itMax);
      sb.push_str(&format!("\nitcMax = {}", self.itMax));
      // java if (this.rgdxaCenter != null && this.rgdxaCenter.length >= this.itMax)
      if let Some(rgdxaCenter) = self.rgdxaCenter.as_ref() {
        if rgdxaCenter.len() as i32 >= self.itMax && self.itMax > 0 {
          // java sb.append("\nrgdxaCenter = {");
          sb.push_str("\nrgdxaCenter = {");
          // java sb.append(this.rgdxaCenter[0]);
          sb.push_str(&rgdxaCenter[0].to_string());
          let mut i: i32 = 1;
          // java for(int i = 1; i < this.itMax; ++i)
          while i < self.itMax {
            // java sb.append(", " + this.rgdxaCenter[i]);
            sb.push_str(&format!(", {}", rgdxaCenter[i as usize]));
            i += 1;
          }
          // java sb.append("}");
          sb.push('}');
        }
      }
      // java return sb.toString();
      sb
    }

    pub fn same_table(&self, other: &DefTable) -> bool {
      if self.itc != other.itc || self.itMax != other.itMax {
        return false;
      }
      let (Some(a), Some(b)) = (self.rgdxaCenter.as_ref(), other.rgdxaCenter.as_ref()) else {
        return false;
      };
      if (a.len() as i32) < self.itMax || (b.len() as i32) < self.itMax {
        return false;
      }
      for i in 0..self.itMax as usize {
        if a[i] != b[i] {
          return false;
        }
      }
      let (Some(ca), Some(cb)) = (self.rgtc.as_ref(), other.rgtc.as_ref()) else {
        return false;
      };
      if (ca.len() as i32) < self.itc || (cb.len() as i32) < self.itc {
        return false;
      }
      for i in 0..self.itc as usize {
        if ca[i] != cb[i] {
          return false;
        }
      }
      true
    }

    // java public int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // java if (this.hash == 0)
      if self.hash == 0 {
        // java int h = 0;
        let mut h: i32 = 0;
        // java h += this.itc + this.itMax;
        h += self.itc + self.itMax;
        // java if (this.rgdxaCenter != null && this.rgdxaCenter.length >= this.itMax)
        if let Some(rgdxaCenter) = self.rgdxaCenter.as_ref() {
          if rgdxaCenter.len() as i32 >= self.itMax {
            let mut i: i32 = 0;
            // java for(int i = 0; i < this.itMax; ++i)
            while i < self.itMax {
              // java h += this.rgdxaCenter[i] * 20;
              h += rgdxaCenter[i as usize] * 20;
              i += 1;
            }
          }
        }
        // java if (this.rgtc != null && this.rgtc.length >= this.itc)
        if let Some(rgtc) = self.rgtc.as_ref() {
          if rgtc.len() as i32 >= self.itc {
            let mut i: i32 = 0;
            // java for(int i = 0; i < this.itc; ++i)
            while i < self.itc {
              // java TableCellBuilder c = this.rgtc[i];
              if let Some(c) = rgtc[i as usize].as_ref() {
                // java if (c != null)
                // java h += c.hashCode();
                let mut cell = c.super_.clone();
                h += cell.hashCode();
              }
              i += 1;
            }
          }
        }
        // java this.hash = h;
        self.hash = h;
      }
      // java return this.hash;
      self.hash
    }
}
