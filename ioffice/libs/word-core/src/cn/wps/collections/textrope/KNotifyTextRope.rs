// Java: import cn.wps.collections.textrope.KTransactionTextRope
use crate::cn::wps::collections::textrope::KTransactionTextRope::*;
use crate::cn::wps::collections::textrope::KTransactionTextRope::KTransactionTextRope;
// Java: import cn.wps.collections.textrope.INotifyTextRope
use crate::cn::wps::collections::textrope::INotifyTextRope::*;
use crate::cn::wps::collections::textrope::INotifyTextRope::{INotifyTextRope, Listener};
use crate::cn::wps::collections::textrope::ITextRope::ITextRope;
use crate::cn::wps::collections::textrope::ITextPiece::ITextPiece;
use crate::cn::wps::collections::textrope::ITextPool::ITextPool;

// java public class KNotifyTextRope extends KTransactionTextRope implements INotifyTextRope
#[derive(Default)]
pub struct KNotifyTextRope {
    // java KTransactionTextRope super_
    pub(crate) super_: KTransactionTextRope,
    // java INotifyTextRope.Listener mListener;
    pub(crate) mListener: Option<Box<dyn Listener>>,
}

// java public interface INotifyTextRope
impl INotifyTextRope for KNotifyTextRope {
    // java public final INotifyTextRope.Listener getListener()
    fn getListener(&self) -> Option<&dyn Listener> {
      // java return this.mListener;
      self.mListener.as_deref()
    }

    // java  void setListener(Listener var1)
    fn setListener(&mut self, var1: Option<Box<dyn Listener>>) {
      self.mListener = var1;
    }

}

impl KNotifyTextRope {
    // Java: ((TextRopeListeners) this.getListener()).add(listener)
    pub fn addTextListener(&mut self, listener: Box<dyn Listener>) {
        if let Some(existing) = self.mListener.as_mut() {
            existing.addListener(listener);
        }
    }
}

// ITextRope required by INotifyTextRope; insert/erase go through overrides (Java virtual this)
impl ITextRope for KNotifyTextRope {
    fn append__char(&mut self, c: char) {
      let fc = unsafe { (*self.super_.super_.active_pool()).append__char(c) };
      self.insertImpl(self.super_.super_.mLength, fc, 1);
    }
    fn append__chararr_int_int(&mut self, buf: &Vec<char>, start: i32, end: i32) {
      let fc = unsafe { (*self.super_.super_.active_pool()).append__chararr_int_int(buf, start, end) };
      self.insertImpl(self.super_.super_.mLength, fc, end - start);
    }
    fn append__string_int_int(&mut self, s: &str, start: i32, end: i32) {
      let fc = unsafe { (*self.super_.super_.active_pool()).append__charsequence_int_int(Some(s), start, end) };
      self.insertImpl(self.super_.super_.mLength, fc, end - start);
    }
    fn begin(&mut self) -> Option<Box<dyn ITextPiece>> {
      ITextRope::begin(&mut self.super_.super_)
    }
    fn charAt(&mut self, cp: i32) -> char {
      ITextRope::charAt(&mut self.super_.super_, cp)
    }
    fn cp2fc(&mut self, cp: i32) -> i32 {
      ITextRope::cp2fc(&mut self.super_.super_, cp)
    }
    fn end(&mut self) -> Option<Box<dyn ITextPiece>> {
      ITextRope::end(&mut self.super_.super_)
    }
    fn erase(&mut self, start: i32, end: i32) {
      self.eraseImpl(start, end);
    }
    fn fc2cp(&mut self, fc: i32) -> i32 {
      ITextRope::fc2cp(&mut self.super_.super_, fc)
    }
    fn getText(&self, start: i32, end: i32, buffer: &mut [char], bufStart: i32) {
      ITextRope::getText(&self.super_.super_, start, end, buffer, bufStart)
    }
    fn getTextPool(&self) -> Option<*mut dyn ITextPool> {
      ITextRope::getTextPool(&self.super_.super_)
    }
    fn insert__int_char(&mut self, cp: i32, c: char) {
      let fc = unsafe { (*self.super_.super_.active_pool()).append__char(c) };
      self.insertImpl(cp, fc, 1);
    }
    fn insert__int_chararr_int_int(&mut self, cp: i32, buf: &Vec<char>, start: i32, end: i32) {
      let fc = unsafe { (*self.super_.super_.active_pool()).append__chararr_int_int(buf, start, end) };
      self.insertImpl(cp, fc, end - start);
    }
    fn insert__int_string_int_int(&mut self, cp: i32, s: &str, start: i32, end: i32) {
      let fc = unsafe { (*self.super_.super_.active_pool()).append__charsequence_int_int(Some(s), start, end) };
      self.insertImpl(cp, fc, end - start);
    }
    fn length(&self) -> i32 {
      ITextRope::length(&self.super_.super_)
    }
    fn seek(&mut self, cp: i32) -> Option<Box<dyn ITextPiece>> {
      ITextRope::seek(&mut self.super_.super_, cp)
    }
    fn addF0(&mut self, ranges: &Vec<i64>) {
      ITextRope::addF0(&mut self.super_.super_, ranges)
    }
    fn removeF0(&mut self, ranges: &Vec<i64>) {
      ITextRope::removeF0(&mut self.super_.super_, ranges)
    }
    fn modifyF0Impl(&mut self, cp: i32, len: i32, fAddF0: bool) {
      self.super_.modifyF0Impl(cp, len, fAddF0);
    }
}

// java public class KNotifyTextRope extends KTransactionTextRope implements INotifyTextRope
impl KNotifyTextRope {
    // java public final void setListener(INotifyTextRope.Listener listener)
    pub fn setListener(&mut self, listener: Option<Box<dyn Listener>>) {
      // java this.mListener = listener;
      self.mListener = listener;
    }

    // java protected void insertImpl(int cp, int fc, int length)
    pub fn insertImpl(&mut self, cp: i32, fc: i32, length: i32) {
      // java if (this.mListener != null)
      if let Some(mListener) = self.mListener.as_mut() {
        // java this.mListener.beforeInsertText(cp, fc, length);
        mListener.beforeInsertText(cp, fc, length);
      }
      // java super.insertImpl(cp, fc, length);
      self.super_.insertImpl(cp, fc, length);
      // java if (this.mListener != null)
      if let Some(mListener) = self.mListener.as_mut() {
        // java this.mListener.afterInsertText(cp, fc, length);
        mListener.afterInsertText(cp, fc, length);
      }
    }

    // java protected void eraseImpl(int start, int end)
    pub fn eraseImpl(&mut self, start: i32, end: i32) {
      // java if (this.mListener != null)
      if let Some(mListener) = self.mListener.as_mut() {
        // java this.mListener.beforeRemoveText(start, end);
        mListener.beforeRemoveText(start, end);
      }
      // java super.eraseImpl(start, end);
      self.super_.eraseImpl(start, end);
      // java if (this.mListener != null)
      if let Some(mListener) = self.mListener.as_mut() {
        // java this.mListener.afterRemoveText(start, end);
        mListener.afterRemoveText(start, end);
      }
    }

}

impl std::ops::Deref for KNotifyTextRope {
    type Target = KTransactionTextRope;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for KNotifyTextRope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
