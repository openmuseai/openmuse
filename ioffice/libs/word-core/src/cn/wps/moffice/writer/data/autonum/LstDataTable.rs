// Java: import cn.wps.moffice.transaction.Atom
use util::moffice::transaction::Atom::*;
// Java: import cn.wps.moffice.transaction.AtomMember
use util::moffice::transaction::AtomMember::*;
// Java: import cn.wps.moffice.transaction.Transaction
use util::moffice::transaction::Transaction::*;
// Java: import java.util.HashMap
// Java: import cn.wps.moffice.writer.data.autonum.LstData
use crate::cn::wps::moffice::writer::data::autonum::LstData::*;
use std::cell::RefCell;
use std::collections::{HashMap};
use std::rc::Rc;

// java public static final int NOT_IN_LST_INDEX = 0;
pub(crate) const NOT_IN_LST_INDEX: i32 = 0;
// java public static final int START_INDEX = 1;
pub(crate) const START_INDEX: i32 = 1;

// java public class LstDataTable extends Atom
#[derive(Default)]
pub struct LstDataTable {
    // java Atom super_
    pub(crate) super_: Atom,
    // java @AtomMember(1) public HashMap<Integer, LstData> mLstMap = new HashMap();
    pub(crate) mLstMap: Option<HashMap<i32, LstData>>,
}

// java public class LstDataTable extends Atom
impl LstDataTable {
    // java public  LstDataTable(Transaction transaction)
    pub fn LstDataTable(transaction: Option<Rc<RefCell<Transaction>>>) -> Self {
      // java this.setTransaction(transaction, false);
      let mut this = Self {
        super_: Atom::new(),
        mLstMap: Some(HashMap::new()),
      };
      this.super_.setTransaction_recursive(transaction, false);
      this
    }

    // java public HashMap<Integer, LstData> getLstMap()
    pub fn getLstMap(&self) -> HashMap<i32, LstData> {
      // java return this.mLstMap;
      self.mLstMap.clone().unwrap_or_default()
    }

    // java public void add(LstData data)
    pub fn add(&mut self, data: &mut LstData) {
      // java data.setTransaction(this.mTransaction, false);
      data.setTransaction(self.super_.getTransaction(), false);
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLstMap.put(data.getlsid(), data);
      let id = data.getlsid();
      self.mLstMap.get_or_insert_with(HashMap::new).insert(id, data.clone());
    }

    // java public LstData remove(int lsid)
    pub fn remove(&mut self, lsid: i32) -> Option<LstData> {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java return (LstData)this.mLstMap.remove(lsid);
      self.mLstMap.as_mut().and_then(|m| m.remove(&lsid))
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mLstMap.clear();
      if let Some(m) = self.mLstMap.as_mut() {
        m.clear();
      }
      // java this.mLstMap = null;
      self.mLstMap = None;
    }

    // java public LstData getDataByLsid(int lsid)
    pub fn getDataByLsid(&self, lsid: i32) -> Option<LstData> {
      // java return (LstData)this.mLstMap.get(lsid);
      self.mLstMap.as_ref().and_then(|m| m.get(&lsid).cloned())
    }

    // java public int getIndex(int lsid)
    pub fn getIndex(&self, lsid: i32) -> i32 {
      // java int i = 1;
      let mut i: i32 = 1;
      // java for(Integer curLsid : this.mLstMap.keySet())
      let Some(map) = self.mLstMap.as_ref() else {
        return 0;
      };
      for curLsid in map.keys() {
        // java if (curLsid == lsid)
        if *curLsid == lsid {
          // java return i;
          return i;
        }
        // java ++i;
        i += 1;
      }
      // java return 0;
      0
    }

}

impl std::ops::Deref for LstDataTable {
    type Target = Atom;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for LstDataTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
