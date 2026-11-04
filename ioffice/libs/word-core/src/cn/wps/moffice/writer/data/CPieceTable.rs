#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]
// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::*;
// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::*;
// Java: import cn.wps.moffice.transaction.Transaction
use util::moffice::transaction::Transaction::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::{
    Entry as IChpxEntry, IChpxTable,
};
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{
    Entry as IPapxEntry, IPapxTable,
};
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;

// java public class CPieceTable
#[derive(Default)]
pub struct CPieceTable {
    // java Document document;
    pub(crate) document: Option<*mut dyn Document>,
    // java ITextRope mTextRope;
    pub(crate) mTextRope: Option<*mut dyn ITextRope>,
    // java IChpxTable mChpxTable;
    pub(crate) mChpxTable: Option<*mut dyn IChpxTable>,
    // java IPapxTable mPapxTable;
    pub(crate) mPapxTable: Option<*mut dyn IPapxTable>,
    // java int mGuessedTotalLength = 0;
    pub(crate) mGuessedTotalLength: i32,
}

// java public class CPieceTable
impl CPieceTable {
    // java public  CPieceTable(Document document)
    pub fn CPieceTable(document: *mut dyn Document) -> Self {
      // java Transaction transaction = document.getTextDocument().getTransaction();
      let transaction: Option<&Transaction> = if !document.is_null() {
            unsafe {
                (*document)
                    .getTextDocument()
                    .and_then(|td| td.getTransaction())
            }
        } else {
            None
        };
      // java KSAssert.assertNotNull(transaction);
      KSAssert::assertNotNull(transaction);
      // java this.document = document;
      let mut this = Self::default();
      this.document = if document.is_null() {
            None
        } else {
            Some(document)
        };
      // java this.mTextRope = document.getTextRope();
      this.mTextRope = if document.is_null() {
            None
        } else {
            unsafe { (*document).getTextRope() }
        };
      // java this.mChpxTable = document.getChpxTable();
      this.mChpxTable = if document.is_null() {
            None
        } else {
            unsafe { (*document).getChpxTable() }
        };
      // java this.mPapxTable = document.getPapxTable();
      this.mPapxTable = if document.is_null() {
            None
        } else {
            unsafe { (*document).getPapxTable() }
        };
      this
    }

    // java public  CPieceTable()
    pub fn CPieceTable__empty() -> Self {
      Self::default()
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.document = null;
      self.document = None;
      // java this.mTextRope = null;
      self.mTextRope = None;
      // java this.mChpxTable = null;
      self.mChpxTable = None;
      // java this.mPapxTable = null;
      self.mPapxTable = None;
    }

    // java public ITextRope getTextRope()
    pub fn getTextRope(&self) -> Option<*mut dyn ITextRope> {
      self.mTextRope
    }

    // java public IChpxTable getChpxTable()
    pub fn getChpxTable(&self) -> Option<*mut dyn IChpxTable> {
      self.mChpxTable
    }

    // java public IPapxTable getPapxTable()
    pub fn getPapxTable(&self) -> Option<*mut dyn IPapxTable> {
      self.mPapxTable
    }

    // java public char charAt(int cp)
    pub fn charAt(&mut self, cp: i32) -> char {
      // java return this.mTextRope.charAt(cp);
      match self.mTextRope {
            Some(r) if !r.is_null() => unsafe { (*r).charAt(cp) },
            _ => '\0',
        }
    }

    // java public final long getParagraphRange(int start, int end)
    pub fn getParagraphRange__int_int(&self, start: i32, end: i32) -> i64 {
      let mut s: Option<Box<dyn IPapxEntry>> = match self.mPapxTable {
            Some(t) if !t.is_null() => unsafe { IPapxTable::seek(&*t, start) },
            _ => None,
        };
      let mut e: Option<Box<dyn IPapxEntry>> = match self.mPapxTable {
            Some(t) if !t.is_null() => unsafe { IPapxTable::seek(&*t, end) },
            _ => None,
        };
      if let Some(ref ee) = e {
        if !ee.isHead() && ee.position() != end {
          e = IPapxEntry::getNext(ee.as_ref());
        }
      }
      let same = match (&e, &s) {
            (Some(ee), Some(ss)) => ee.position() == ss.position() && ee.isHead() == ss.isHead(),
            _ => false,
        };
      if same {
        if let Some(ref ee) = e {
                e = IPapxEntry::getNext(ee.as_ref());
            }
      }
      let para_end: i32;
      if e.as_ref().map(|ee| ee.isHead()).unwrap_or(true) {
        para_end = match self.mTextRope {
                Some(r) if !r.is_null() => unsafe { (*r).length() },
                _ => 0,
            };
      } else {
        para_end = e.as_ref().map(|ee| ee.position()).unwrap_or(0);
      }
      let s_pos = s.as_ref().map(|ss| ss.position()).unwrap_or(0);
      RANGEFactory::getRANGE(s_pos, para_end)
    }

    // java public final long getParagraphRange(int cp)
    pub fn getParagraphRange__int(&self, cp: i32) -> i64 {
      // java return this.mPapxTable.seek(cp).range();
      match self.mPapxTable {
            Some(t) if !t.is_null() => unsafe {
                IPapxTable::seek(&*t, cp).map(|e| e.range()).unwrap_or(0)
            },
            _ => 0,
        }
    }

    // java public int getGuessedTotalLength()
    pub fn getGuessedTotalLength(&self) -> i32 {
      self.mGuessedTotalLength
    }

    // java public void setGuessedTotalLength(int length)
    pub fn setGuessedTotalLength(&mut self, length: i32) {
      self.mGuessedTotalLength = length;
    }

    // java public final int getCP(IChpxTable.Entry entry)
    pub fn getCP(&self, entry: &dyn IChpxEntry) -> i32 {
      entry.position()
    }

    // java public final int position(IChpxTable.Entry entry)
    pub fn position(&self, entry: &dyn IChpxEntry) -> i32 {
      self.getCP(entry)
    }
}
