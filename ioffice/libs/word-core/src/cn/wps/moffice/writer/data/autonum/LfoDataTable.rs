// Java: import cn.wps.moffice.transaction.Atom
use util::moffice::transaction::Atom::*;
// Java: import cn.wps.moffice.transaction.AtomMember
use util::moffice::transaction::AtomMember::*;
// Java: import cn.wps.moffice.transaction.Transaction
use util::moffice::transaction::Transaction::*;
// Java: import java.util.HashMap
// Java: import cn.wps.moffice.writer.data.autonum.LfoData
use crate::cn::wps::moffice::writer::data::autonum::LfoData::*;
use std::cell::RefCell;
use std::collections::{HashMap};
use std::rc::Rc;

// java public static final int NOT_IN_NUM_INDEX = 0;
pub(crate) const NOT_IN_NUM_INDEX: i32 = 0;
// java public static final int START_INDEX = 1;
pub(crate) const START_INDEX: i32 = 1;

// java public class LfoDataTable extends Atom
#[derive(Default)]
pub struct LfoDataTable {
    // java Atom super_
    pub(crate) super_: Atom,
    // java @AtomMember(1) public HashMap<Integer, LfoData> mLfoMap = new HashMap();
    pub(crate) mLfoMap: Option<HashMap<i32, LfoData>>,
}

// java public class LfoDataTable extends Atom
impl LfoDataTable {
    // java public  LfoDataTable(Transaction transaction)
    pub fn LfoDataTable(transaction: Option<Rc<RefCell<Transaction>>>) -> Self {
      // java this.setTransaction(transaction, false);
      let mut this = Self {
        super_: Atom::new(),
        mLfoMap: Some(HashMap::new()),
      };
      this.super_.setTransaction_recursive(transaction, false);
      this
    }

    // java public HashMap<Integer, LfoData> getLfoDataMap()
    pub fn getLfoDataMap(&self) -> HashMap<i32, LfoData> {
      // java return this.mLfoMap;
      self.mLfoMap.clone().unwrap_or_default()
    }

    // java public void clear()
    pub fn clear(&mut self) {
      if let Some(m) = self.mLfoMap.as_mut() {
        m.clear();
      }
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mLfoMap.clear();
      if let Some(m) = self.mLfoMap.as_mut() {
        m.clear();
      }
      // java this.mLfoMap = null;
      self.mLfoMap = None;
    }

    // java public void add(Integer lfo, LfoData data)
    pub fn add(&mut self, lfo: i32, data: &mut LfoData) {
      // java data.setTransaction(this.mTransaction, false);
      data.super_.setTransaction_recursive(self.super_.getTransaction(), false);
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLfoMap.put(lfo, data);
      self.mLfoMap.get_or_insert_with(HashMap::new).insert(lfo, data.clone());
    }

    // java public LfoData remove(Integer lfo)
    pub fn remove(&mut self, lfo: i32) -> Option<LfoData> {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java return (LfoData)this.mLfoMap.remove(lfo);
      self.mLfoMap.as_mut().and_then(|m| m.remove(&lfo))
    }

    // java public LfoData getDataByIlfo(Integer nILfo)
    pub fn getDataByIlfo(&self, nILfo: i32) -> Option<LfoData> {
      // java return (LfoData)this.mLfoMap.get(nILfo);
      self.mLfoMap.as_ref().and_then(|m| m.get(&nILfo).cloned())
    }

    // java public int getIndex(int lfo)
    pub fn getIndex(&self, lfo: i32) -> i32 {
      // java int i = 1;
      let mut i: i32 = 1;
      // java for(Integer curLfo : this.mLfoMap.keySet())
      let Some(map) = self.mLfoMap.as_ref() else {
        return 0;
      };
      for curLfo in map.keys() {
        // java if (curLfo == lfo)
        if *curLfo == lfo {
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

impl std::ops::Deref for LfoDataTable {
    type Target = Atom;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for LfoDataTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
