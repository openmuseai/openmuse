#![allow(dead_code, non_snake_case, unused_variables, unused_mut, unused_imports)]
// Java: import cn.wps.moffice.transaction.Transaction
use util::moffice::transaction::Transaction::*;
use std::cell::RefCell;
use std::rc::Rc;
// Java: import cn.wps.moffice.writer.data.autonum.LstDataTable
use super::LstDataTable::LstDataTable;
// Java: import cn.wps.moffice.writer.data.autonum.LfoDataTable
use super::LfoDataTable::LfoDataTable;
// Java: import cn.wps.moffice.writer.data.autonum.LstLfoTool
use super::LstLfoTool::LstLfoTool;

// java public class AutoNumTable
#[derive(Default)]
pub struct AutoNumTable {
    // java public LstDataTable mLstTable;
    pub(crate) mLstTable: Option<LstDataTable>,
    // java public LfoDataTable mLfoTable;
    pub(crate) mLfoTable: Option<LfoDataTable>,
    // java public LstLfoTool mLstLfoTool;
    pub(crate) mLstLfoTool: Option<LstLfoTool>,
}

// java public class AutoNumTable
impl AutoNumTable {
    // java public  AutoNumTable(Transaction transaction)
    pub fn AutoNumTable(transaction: Option<Rc<RefCell<Transaction>>>) -> Self {
      // java this.mLstTable = new LstDataTable(transaction);
      let mLstTable = Some(LstDataTable::LstDataTable(transaction.clone()));
      // java this.mLfoTable = new LfoDataTable(transaction);
      let mLfoTable = Some(LfoDataTable::LfoDataTable(transaction));
      // java this.mLstLfoTool = new LstLfoTool(this);
      let mut this = Self { mLstTable, mLfoTable, mLstLfoTool: None };
      let ptr = &mut this as *mut Self;
      this.mLstLfoTool = Some(LstLfoTool::LstLfoTool(ptr));
      this
    }

    // java public LstDataTable getLstDataTable()
    pub fn getLstDataTable(&mut self) -> &mut LstDataTable {
      // java return this.mLstTable;
      self.mLstTable.as_mut().expect("mLstTable")
    }

    // java public LfoDataTable getLfoDataTable()
    pub fn getLfoDataTable(&mut self) -> &mut LfoDataTable {
      // java return this.mLfoTable;
      self.mLfoTable.as_mut().expect("mLfoTable")
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mLstLfoTool.dispose();
      if let Some(t) = self.mLstLfoTool.as_mut() { t.dispose(); }
      // java this.mLstTable.dispose();
      if let Some(t) = self.mLstTable.as_mut() { t.dispose(); }
      // java this.mLfoTable.dispose();
      if let Some(t) = self.mLfoTable.as_mut() { t.dispose(); }
      // java this.mLstTable = null;
      self.mLstTable = None;
      // java this.mLfoTable = null;
      self.mLfoTable = None;
      // java this.mLstLfoTool = null;
      self.mLstLfoTool = None;
    }

    // java public LstLfoTool getLstLfoTool()
    pub fn getLstLfoTool(&mut self) -> &mut LstLfoTool {
      // java return this.mLstLfoTool;
      let ptr = self as *mut Self;
      if self.mLstLfoTool.is_none() {
        self.mLstLfoTool = Some(LstLfoTool::LstLfoTool(ptr));
      } else if let Some(tool) = self.mLstLfoTool.as_mut() {
        tool.setTable(ptr);
      }
      self.mLstLfoTool.as_mut().expect("mLstLfoTool")
    }

    pub fn level_exists(&self, nILfo: i32, nILvl: i32) -> bool {
      if nILvl < 0 || nILvl > 8 {
        return false;
      }
      let lfo_data = self.mLfoTable.as_ref().and_then(|t| t.getDataByIlfo(nILfo));
      let lst_data = lfo_data.as_ref().and_then(|d| self.mLstTable.as_ref().and_then(|t| t.getDataByLsid(d.getLsid())));
      LstLfoTool::existing_from(lfo_data.as_ref(), lst_data.as_ref(), nILvl)
    }

}
