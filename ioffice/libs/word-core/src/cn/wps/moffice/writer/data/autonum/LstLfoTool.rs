// Java: import cn.wps.moffice.writer.data.autonum.AutoNumTable
use crate::cn::wps::moffice::writer::data::autonum::AutoNumTable::*;
// Java: import cn.wps.moffice.writer.data.autonum.LfoData
use crate::cn::wps::moffice::writer::data::autonum::LfoData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LstData
use crate::cn::wps::moffice::writer::data::autonum::LstData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LvlfData
use crate::cn::wps::moffice::writer::data::autonum::LvlfData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LfoLvlData
use crate::cn::wps::moffice::writer::data::autonum::LfoLvlData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LfoDataTable
use crate::cn::wps::moffice::writer::data::autonum::LfoDataTable::*;
// Java: import cn.wps.moffice.writer.data.autonum.LstDataTable
use crate::cn::wps::moffice::writer::data::autonum::LstDataTable::*;

// java public class LstLfoTool
pub struct LstLfoTool {
    // java private AutoNumTable mTable = null;
    // rust: Java 持有表的引用。`AutoNumTable` 又持有本工具，按值嵌入会无限大，故用指针，并在 `getLstLfoTool` 里按当前地址重绑。
    mTable: *mut AutoNumTable,
    // java private LfoData mLfoDataRef = null;
    mLfoDataRef: Option<LfoData>,
    // java private LstData mLstDataRef = null;
    mLstDataRef: Option<LstData>,
    // java protected LvlfData mLvlfDataTmp = null;
    pub(crate) mLvlfDataTmp: Option<LvlfData>,
    // java protected int mLfo = -1;
    pub(crate) mLfo: i32,
    // java protected int mLvl = -1;
    pub(crate) mLvl: i32,
}

// java public class LstLfoTool
impl LstLfoTool {
    // java public  LstLfoTool(AutoNumTable autoNumTable)
    pub fn LstLfoTool(autoNumTable: *mut AutoNumTable) -> Self {
      // java this.mTable = autoNumTable;
      Self { mTable: autoNumTable, mLfoDataRef: None, mLstDataRef: None, mLvlfDataTmp: None, mLfo: -1, mLvl: -1 }
    }

    pub fn setTable(&mut self, autoNumTable: *mut AutoNumTable) {
      self.mTable = autoNumTable;
    }

    fn lfo_table(&self) -> Option<&LfoDataTable> {
      if self.mTable.is_null() {
        None
      } else {
        unsafe { (*self.mTable).mLfoTable.as_ref() }
      }
    }

    fn lst_table(&self) -> Option<&LstDataTable> {
      if self.mTable.is_null() {
        None
      } else {
        unsafe { (*self.mTable).mLstTable.as_ref() }
      }
    }

    pub(crate) fn existing_from(lfo_data: Option<&LfoData>, lst_data: Option<&LstData>, nILvl: i32) -> bool {
      if nILvl < 0 || nILvl > 8 {
        return false;
      }
      let Some(lfo_data) = lfo_data else {
        return false;
      };
      if let Some(lfo_lvl) = lfo_data.getLfoLvlByIlvl(nILvl) {
        if lfo_lvl.hasSetFormat() {
          return lfo_lvl.getLvlfData().is_some();
        }
      }
      let Some(lst) = lst_data else {
        return false;
      };
      lst.getLvlfDataByIlvl(nILvl).is_some()
    }

    // java public void clear()
    pub fn clear(&mut self) {
      // java this.mLfo = -1;
      self.mLfo = -1;
      // java this.mLvl = -1;
      self.mLvl = -1;
    }

    // java public void setLstData(LstData data)
    pub fn setLstData(&mut self, data: &LstData) {
      // java this.mLstDataRef = data;
      self.mLstDataRef = Some(data.clone());
    }

    // java public void setLfoData(LfoData data)
    pub fn setLfoData(&mut self, data: &LfoData) {
      // java this.mLfoDataRef = data;
      self.mLfoDataRef = Some(data.clone());
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mTable = null;
      self.mTable = std::ptr::null_mut();
      // java this.mLfoDataRef = null;
      self.mLfoDataRef = None;
      // java this.mLstDataRef = null;
      self.mLstDataRef = None;
    }

    // java public LfoData getLfoData()
    pub fn getLfoData(&self) -> Option<LfoData> {
      // java return this.mLfoDataRef;
      return self.mLfoDataRef.clone();
    }

    // java public LstData getLstData()
    pub fn getLstData(&self) -> Option<LstData> {
      // java return this.mLstDataRef;
      return self.mLstDataRef.clone();
    }

    // java public int getLevelCount()
    pub fn getLevelCount(&self) -> i32 {
      // java return this.mLstDataRef.getLevelCount();
      return self.mLstDataRef.as_ref().map(|d| d.getLevelCount()).unwrap_or(0);
    }

    // java public LvlfData getLvlfData()
    pub fn getLvlfData(&self) -> Option<LvlfData> {
      // java return this.mLvlfDataTmp;
      return self.mLvlfDataTmp.clone();
    }

    // java public LvlfData getLvlfData(int nILvl)
    pub fn getLvlfData__int(&self, nILvl: i32) -> Option<LvlfData> {
      // java if (this.mLvl == nILvl)
      if self.mLvl == nILvl {
        // java return this.mLvlfDataTmp;
        return self.mLvlfDataTmp.clone();
      }
      // java else
      else {
        // java LfoLvlData lfoLvlData = null;
        let mut lfoLvlData: Option<LfoLvlData> = None;
        // java if (null != this.mLfoDataRef)
        if let Some(mLfoDataRef) = self.mLfoDataRef.as_ref() {
          // java lfoLvlData = this.mLfoDataRef.getLfoLvlByIlvl(nILvl);
          lfoLvlData = mLfoDataRef.getLfoLvlByIlvl(nILvl);
        }
        // java if (lfoLvlData != null && lfoLvlData.hasSetFormat())
        if lfoLvlData.as_ref().is_some_and(|d| d.hasSetFormat()) {
          // java return lfoLvlData.getLvlfData();
          return lfoLvlData.and_then(|d| d.getLvlfData());
        }
        // java else
        else {
          // java return null != this.mLstDataRef ? this.mLstDataRef.getLvlfDataByIlvl(nILvl) : null;
          return self.mLstDataRef.as_ref().and_then(|d| d.getLvlfDataByIlvl(nILvl));
        }
      }
    }

    // java public int getLfo()
    pub fn getLfo(&self) -> i32 {
      // java return this.mLfo;
      return self.mLfo;
    }

    // java public int getLsid()
    pub fn getLsid(&self) -> i32 {
      // java return this.mLfoDataRef.getLsid();
      return self.mLfoDataRef.as_ref().map(|d| d.getLsid()).unwrap_or(0);
    }

    // java public boolean isFStartAt(int nILvl)
    pub fn isFStartAt(&self, nILvl: i32) -> bool {
      // java LfoLvlData lfoLvlData = null;
      let mut lfoLvlData: Option<LfoLvlData> = None;
      // java if (null != this.mLfoDataRef)
      if let Some(mLfoDataRef) = self.mLfoDataRef.as_ref() {
        // java lfoLvlData = this.mLfoDataRef.getLfoLvlByIlvl(nILvl);
        lfoLvlData = mLfoDataRef.getLfoLvlByIlvl(nILvl);
      }
      // java if (lfoLvlData == null)
      if lfoLvlData.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java return lfoLvlData.hasSetStartAt() && !lfoLvlData.hasSetFormat();
        let d = lfoLvlData.as_ref().unwrap();
        return d.hasSetStartAt() && !d.hasSetFormat();
      }
    }

    // java public int getLevelStartAt(int nILvl)
    pub fn getLevelStartAt(&self, nILvl: i32) -> i32 {
      // java LfoLvlData lfoLvlData = null;
      let mut lfoLvlData: Option<LfoLvlData> = None;
      // java if (null != this.mLfoDataRef)
      if let Some(mLfoDataRef) = self.mLfoDataRef.as_ref() {
        // java lfoLvlData = this.mLfoDataRef.getLfoLvlByIlvl(nILvl);
        lfoLvlData = mLfoDataRef.getLfoLvlByIlvl(nILvl);
      }
      // java if (lfoLvlData != null && lfoLvlData.hasSetStartAt() && !lfoLvlData.hasSetFormat())
      if let Some(d) = lfoLvlData.as_ref() {
        if d.hasSetStartAt() && !d.hasSetFormat() {
          // java return lfoLvlData.getStartAt();
          return d.getStartAt();
        }
      }
      // java else
      // java return null != this.mLstDataRef ? this.mLstDataRef.getLvlfDataByIlvl(nILvl).getStartAt() : 0;
      return self.mLstDataRef.as_ref().and_then(|d| d.getLvlfDataByIlvl(nILvl)).map(|v| v.getStartAt()).unwrap_or(0);
    }

    // java public int getLevelIstd(int nILvl)
    pub fn getLevelIstd(&self, nILvl: i32) -> i32 {
      // java return this.mLstDataRef.getLSTF().getRgistd(nILvl);
      return self.mLstDataRef.as_ref().and_then(|d| d.getLSTF()).map(|f| f.getRgistd__int(nILvl) as i32).unwrap_or(0);
    }

    // java public boolean isExisting(int nILfo, int nILvl)
    pub fn isExisting(&self, nILfo: i32, nILvl: i32) -> bool {
      // java if (nILvl >= 0 && nILvl <= 8)
      if nILvl >= 0 && nILvl <= 8 {
        // java LfoData lfoDataRef = this.mTable.mLfoTable.getDataByIlfo(nILfo);
        let lfoDataRef = self.lfo_table().and_then(|t| t.getDataByIlfo(nILfo));
        // java if (lfoDataRef == null)
        if lfoDataRef.is_none() {
          // java return false;
          return false;
        }
        // java else
        else {
          let lsid = lfoDataRef.as_ref().map(|d| d.getLsid()).unwrap_or(0);
          // java LstData lstDataRef = this.mTable.mLstTable.getDataByLsid(lfoDataRef.getLsid());
          let lstDataRef = self.lst_table().and_then(|t| t.getDataByLsid(lsid));
          return Self::existing_from(lfoDataRef.as_ref(), lstDataRef.as_ref(), nILvl);
        }
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public boolean init(int nILfo, int nILvl)
    pub fn init(&mut self, nILfo: i32, nILvl: i32) -> bool {
      // java if (nILvl >= 0 && nILvl <= 8)
      if nILvl >= 0 && nILvl <= 8 {
        // java if (this.mLfo != nILfo)
        if self.mLfo != nILfo {
          // java this.mLfoDataRef = this.mTable.mLfoTable.getDataByIlfo(nILfo);
          self.mLfoDataRef = self.lfo_table().and_then(|t| t.getDataByIlfo(nILfo));
          // java if (this.mLfoDataRef == null)
          if self.mLfoDataRef.is_none() {
            // java this.mLfo = Integer.MIN_VALUE;
            self.mLfo = i32::MIN;
            // java return false;
            return false;
          }
          // java this.mLstDataRef = this.mTable.mLstTable.getDataByLsid(this.mLfoDataRef.getLsid());
          let lsid = self.mLfoDataRef.as_ref().map(|d| d.getLsid()).unwrap_or(0);
          self.mLstDataRef = self.lst_table().and_then(|t| t.getDataByLsid(lsid));
        }
        // java if (this.mLfo != nILfo || this.mLvl != nILvl)
        if self.mLfo != nILfo || self.mLvl != nILvl {
          // java this.mLvl = Integer.MIN_VALUE;
          self.mLvl = i32::MIN;
          // java this.mLvlfDataTmp = this.getLvlfData(nILvl);
          self.mLvlfDataTmp = self.getLvlfData__int(nILvl);
          // java if (this.mLvlfDataTmp == null)
          if self.mLvlfDataTmp.is_none() {
            // java this.mLvl = Integer.MIN_VALUE;
            self.mLvl = i32::MIN;
            // java return false;
            return false;
          }
        }
        // java this.mLfo = nILfo;
        self.mLfo = nILfo;
        // java this.mLvl = nILvl;
        self.mLvl = nILvl;
        // java return true;
        return true;
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public boolean isSimpleList()
    pub fn isSimpleList(&self) -> bool {
      // java return this.mLstDataRef.getLSTF().isfSimpleList();
      return self.mLstDataRef.as_ref().and_then(|d| d.getLSTF()).is_some_and(|f| f.isfSimpleList());
    }

}
