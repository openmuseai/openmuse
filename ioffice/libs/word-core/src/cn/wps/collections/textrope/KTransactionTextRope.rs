// Java: import cn.wps.collections.IntervalMap
use crate::cn::wps::collections::IntervalMap::Segment;
// Java: import cn.wps.moffice.transaction.Command
use util::moffice::transaction::Command::Command;
// Java: import cn.wps.moffice.transaction.Transaction
use util::moffice::transaction::Transaction::Transaction;
use util::moffice::transaction::Action::ActionCommand;
// Java: import java.util.ArrayList
// Java: import cn.wps.collections.textrope.KTextRope
use crate::cn::wps::collections::textrope::KTextRope::KTextRope;
use crate::cn::wps::collections::textrope::ITextRope::ITextRope;
// Java: import cn.wps.moffice.drawing.smartshape.Segment

// java public class KTransactionTextRope extends KTextRope
pub struct KTransactionTextRope {
    // java KTextRope super_
    pub(crate) super_: KTextRope,
    // java Transaction mTransaction;
    // fill: shared non-owned ref (Java); *mut matches PLC pattern (typeoracle: Weak)
    pub(crate) mTransaction: *mut Transaction,
}

impl Default for KTransactionTextRope {
    fn default() -> Self {
        Self {
            super_: KTextRope::default(),
            mTransaction: std::ptr::null_mut(),
        }
    }
}

// java  class CmdErase extends Command
pub struct CmdErase {
    // java Command super_
    pub(crate) super_: Command,
    // Outer.this backref (Java inner class); glue — matches PLC *mut pattern
    pub(crate) outer: *mut KTransactionTextRope,
    // java IntervalMap.Segment[] segments;
    pub(crate) segments: Vec<Segment>,
}

// java  class CmdInsert extends Command
pub struct CmdInsert {
    // java Command super_
    pub(crate) super_: Command,
    pub(crate) outer: *mut KTransactionTextRope,
    // java int cp;
    pub(crate) cp: i32,
    // java int fc;
    pub(crate) fc: i32,
    // java int length;
    pub(crate) length: i32,
}

// java  class CmdModify_F0 extends Command
pub struct CmdModify_F0 {
    // java Command super_
    pub(crate) super_: Command,
    pub(crate) outer: *mut KTransactionTextRope,
    // java int cp;
    pub(crate) cp: i32,
    // java int len;
    pub(crate) len: i32,
    // java boolean fAddF0;
    pub(crate) fAddF0: bool,
}

// java public class KTransactionTextRope extends KTextRope
impl KTransactionTextRope {
    // java public void modifyF0Impl(int cp, int len, boolean fAddF0)
    pub fn modifyF0Impl(&mut self, cp: i32, len: i32, fAddF0: bool) {
      // java super.modifyF0Impl(cp, len, fAddF0);
      ITextRope::modifyF0Impl(&mut self.super_, cp, len, fAddF0);
      // java if (this.mTransaction != null && this.mTransaction.getState() == 1)
      if !self.mTransaction.is_null() {
        let mTransaction = unsafe { &mut *self.mTransaction };
        if mTransaction.getState() == 1 {
          // java this.mTransaction.submitCommand(new CmdModify_F0(cp, len, fAddF0));
          let this_ptr = self as *mut KTransactionTextRope;
          let cmd = CmdModify_F0::CmdModify_F0(this_ptr, cp, len, fAddF0);
          mTransaction.submitCommand(Some(Box::new(cmd)));
        }
      }
    }

    // java protected void eraseImpl(int start, int end)
    pub fn eraseImpl(&mut self, start: i32, end: i32) {
      // java ArrayList<IntervalMap.Segment> segments = null;
      let mut segments: Option<Vec<Segment>> = None;
      // java boolean bBackup = this.mTransaction != null && this.mTransaction.getState() == 1;
      let bBackup: bool = !self.mTransaction.is_null()
        && unsafe { (*self.mTransaction).getState() == 1 };
      // java if (bBackup)
      if bBackup {
        // java segments = new ArrayList();
        segments = Some(Vec::new());
      }
      // java super.eraseMapping(start, end, segments);
      self.super_.eraseMapping(start, end, segments.as_mut());
      // java if (bBackup)
      if bBackup {
        // java IntervalMap.Segment[] p = new IntervalMap.Segment[segments.size()];
        // java segments.toArray(p);
        let p: Vec<Segment> = segments.take().unwrap_or_default();
        // java this.mTransaction.submitCommand(new CmdErase(p));
        let this_ptr = self as *mut KTransactionTextRope;
        let cmd = CmdErase::CmdErase(this_ptr, p);
        unsafe { (*self.mTransaction).submitCommand(Some(Box::new(cmd))); }
      }
    }

    // java protected void insertImpl(int cp, int fc, int length)
    pub fn insertImpl(&mut self, cp: i32, fc: i32, length: i32) {
      // java super.insertImpl(cp, fc, length);
      self.super_.insertImpl(cp, fc, length);
      // java if (this.mTransaction != null && this.mTransaction.getState() == 1)
      if !self.mTransaction.is_null() {
        let mTransaction = unsafe { &mut *self.mTransaction };
        if mTransaction.getState() == 1 {
          // java this.mTransaction.submitCommand(new CmdInsert(cp, fc, length));
          let this_ptr = self as *mut KTransactionTextRope;
          let cmd = CmdInsert::CmdInsert(this_ptr, cp, fc, length);
          mTransaction.submitCommand(Some(Box::new(cmd)));
        }
      }
    }

    // java public final void setTransaction(Transaction transaction)
    pub fn setTransaction(&mut self, transaction: Option<&Transaction>) {
      // java this.mTransaction = transaction;
      self.mTransaction = match transaction {
        Some(t) => t as *const Transaction as *mut Transaction,
        None => std::ptr::null_mut(),
      };
    }

}

impl std::ops::Deref for KTransactionTextRope {
    type Target = KTextRope;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for KTransactionTextRope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java  class CmdErase extends Command
impl CmdErase {
    // java   CmdErase(IntervalMap.Segment[] segments)
    pub fn CmdErase(outer: *mut KTransactionTextRope, segments: Vec<Segment>) -> Self {
      // java this.segments = segments;
      Self { super_: Command::new(), outer, segments }
    }

    // java public void execute()
    pub fn execute(&self) {
      // java int n = this.segments.length;
      let n: i32 = self.segments.len() as i32;
      let mut i: i32 = n - 1;
      // java for(int i = n - 1; i >= 0 --i)
      while i >= 0 {
        // java IntervalMap.Segment segment = this.segments[i];
        let segment: Segment = self.segments[i as usize];
        // java KTransactionTextRope.this.eraseImpl(segment.position1, segment.position1 + segment.length);
        if !self.outer.is_null() {
          unsafe { (*self.outer).eraseImpl(segment.position1, segment.position1 + segment.length); }
        }
        i -= 1;
      }
    }

    // java public void unexecute()
    pub fn unexecute(&self) {
      // java int n = this.segments.length;
      let n: i32 = self.segments.len() as i32;
      let mut i: i32 = 0;
      // java for(int i = 0; i < n; ++i)
      while i < n {
        // java IntervalMap.Segment segment = this.segments[i];
        let segment: Segment = self.segments[i as usize];
        // java KTransactionTextRope.this.insertImpl(segment.position1, segment.position2, segment.length);
        if !self.outer.is_null() {
          unsafe { (*self.outer).insertImpl(segment.position1, segment.position2, segment.length); }
        }
        i += 1;
      }
    }

}

impl ActionCommand for CmdErase {
    fn execute(&self) { CmdErase::execute(self); }
    fn unexecute(&self) { CmdErase::unexecute(self); }
    fn dispose(&self) {}
    fn get_level(&self) -> i32 { self.super_.level }
}

impl std::ops::Deref for CmdErase {
    type Target = Command;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CmdErase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java  class CmdInsert extends Command
impl CmdInsert {
    // java   CmdInsert(int cp, int fc, int length)
    pub fn CmdInsert(outer: *mut KTransactionTextRope, cp: i32, fc: i32, length: i32) -> Self {
      // java this.cp = cp;
      Self { super_: Command::new(), outer, cp, fc, length }
      // java this.fc = fc;
      // java this.length = length;
    }

    // java public void execute()
    pub fn execute(&self) {
      // java KTransactionTextRope.this.insertImpl(this.cp, this.fc, this.length);
      if !self.outer.is_null() {
        unsafe { (*self.outer).insertImpl(self.cp, self.fc, self.length); }
      }
    }

    // java public void unexecute()
    pub fn unexecute(&self) {
      // java KTransactionTextRope.this.eraseImpl(this.cp, this.cp + this.length);
      if !self.outer.is_null() {
        unsafe { (*self.outer).eraseImpl(self.cp, self.cp + self.length); }
      }
    }

}

impl ActionCommand for CmdInsert {
    fn execute(&self) { CmdInsert::execute(self); }
    fn unexecute(&self) { CmdInsert::unexecute(self); }
    fn dispose(&self) {}
    fn get_level(&self) -> i32 { self.super_.level }
}

impl std::ops::Deref for CmdInsert {
    type Target = Command;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CmdInsert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java  class CmdModify_F0 extends Command
impl CmdModify_F0 {
    // java   CmdModify_F0(int cp, int len, boolean fAddF0)
    pub fn CmdModify_F0(outer: *mut KTransactionTextRope, cp: i32, len: i32, fAddF0: bool) -> Self {
      // java this.cp = cp;
      Self { super_: Command::new(), outer, cp, len, fAddF0 }
      // java this.len = len;
      // java this.fAddF0 = fAddF0;
    }

    // java public void execute()
    pub fn execute(&self) {
      // java KTransactionTextRope.this.modifyF0Impl(this.cp, this.len, this.fAddF0);
      if !self.outer.is_null() {
        unsafe { (*self.outer).modifyF0Impl(self.cp, self.len, self.fAddF0); }
      }
    }

    // java public void unexecute()
    pub fn unexecute(&self) {
      // java KTransactionTextRope.this.modifyF0Impl(this.cp, this.len, !this.fAddF0);
      if !self.outer.is_null() {
        unsafe { (*self.outer).modifyF0Impl(self.cp, self.len, !self.fAddF0); }
      }
    }

}

impl ActionCommand for CmdModify_F0 {
    fn execute(&self) { CmdModify_F0::execute(self); }
    fn unexecute(&self) { CmdModify_F0::unexecute(self); }
    fn dispose(&self) {}
    fn get_level(&self) -> i32 { self.super_.level }
}

impl std::ops::Deref for CmdModify_F0 {
    type Target = Command;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CmdModify_F0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
