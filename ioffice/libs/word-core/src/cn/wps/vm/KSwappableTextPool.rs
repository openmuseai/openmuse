// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::*;
// Java: import cn.wps.collections.textrope.ITextPool
use crate::cn::wps::collections::textrope::ITextPool::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import cn.wps.vm.KMemoryPool
use crate::cn::wps::vm::KMemoryPool::*;
// Java: import java.lang.CharSequence
use stubs::java::lang::CharSequence::*;

// java public class KSwappableTextPool implements ITextPool
#[derive(Default)]
pub struct KSwappableTextPool {
    // java int mLength;
    pub(crate) mLength: i32,
    // java KMemoryPool mMemoryPool = new KMemoryPool();
    pub(crate) mMemoryPool: Option<KMemoryPool>,
    // java KMemoryPool.PageRecord mCurrentPage;
    pub(crate) mCurrentPage: Option<crate::cn::wps::vm::KMemoryPool::PageRecord>,
}

// java public interface ITextPool
impl ITextPool for KSwappableTextPool {
    // java public int append(char c)
    fn append__char(&mut self, c: char) -> i32 {
        // java (this)
        self;
          // java int len = this.mLength++;
          let mut len: i32 = { let _tmp = self.mLength; self.mLength += 1; _tmp };
          // java this.set(this.mLength - 1, c);
          self.set(self.mLength - 1, c);
          // java return len;
          return len;
    }

    // java public int append(char[] s, int start, int end)
    fn append__chararr_int_int(&mut self, s: &Vec<char>, start: i32, end: i32) -> i32 {
      // java int len = this.length();
      let mut len: i32 = self.length();
        // java (this)
        self;
              // java while (start < end)
              while start < end {
                // java int index = this.mLength << 1;
                let mut index: i32 = self.mLength << 1;
                // java int nPageNo = index >> KMemoryPool.PAGE_SIZE_SHIFT;
                let mut nPageNo: i32 = index >> KMemoryPool::PAGE_SIZE_SHIFT;
                // java int off = index & KMemoryPool.PAGE_SIZE - 1;
                let mut off: i32 = index & KMemoryPool::PAGE_SIZE - 1;
                // java int nbytesRemain = KMemoryPool.PAGE_SIZE - off;
                let mut nbytesRemain: i32 = KMemoryPool::PAGE_SIZE - off;
                // java int nCharWrite = Math.min(nbytesRemain >> 1, end - start);
                let mut nCharWrite: i32 = Math::min(nbytesRemain >> 1, end - start);
                // java KMemoryPool.PageRecord page = this.getPage(nPageNo);
                let mut page: KMemoryPool::PageRecord = self.getPage(nPageNo);
                // java page.write(off, s, start, nCharWrite);
                page.write(off, s, start, nCharWrite);
                // java this.putPage(page);
                self.putPage(&page);
                // java this.mLength += nCharWrite;
                self.mLength = self.mLength + nCharWrite;
                // java start += nCharWrite;
                start = start + nCharWrite;
              }
              // java IOException e
              todo!();
                // java e.printStackTrace();
                e.printStackTrace();
                // java throw new RuntimeException(e.getMessage());
                panic!("{}", RuntimeException::new(e.getMessage()));
          // java return len;
          return len;
    }

    // java public int append(CharSequence s)
    fn append__charsequence(&mut self, s: &CharSequence) -> i32 {
      // java return this.append((CharSequence)s, 0, s.length());
      return self.append__charsequence_int_int(&(s).downcast_ref::<CharSequence>().unwrap(), 0, s.length());
    }

    // java public int append(CharSequence s, int start, int end)
    fn append__charsequence_int_int(&mut self, s: &CharSequence, start: i32, end: i32) -> i32 {
      // java int len = this.length();
      let mut len: i32 = self.length();
        // java (this)
        self;
              // java while (start < end)
              while start < end {
                // java int index = this.mLength << 1;
                let mut index: i32 = self.mLength << 1;
                // java int nPageNo = index >> KMemoryPool.PAGE_SIZE_SHIFT;
                let mut nPageNo: i32 = index >> KMemoryPool::PAGE_SIZE_SHIFT;
                // java int off = index & KMemoryPool.PAGE_SIZE - 1;
                let mut off: i32 = index & KMemoryPool::PAGE_SIZE - 1;
                // java int nbytesRemain = KMemoryPool.PAGE_SIZE - off;
                let mut nbytesRemain: i32 = KMemoryPool::PAGE_SIZE - off;
                // java int nCharWrite = Math.min(nbytesRemain >> 1, end - start);
                let mut nCharWrite: i32 = Math::min(nbytesRemain >> 1, end - start);
                // java KMemoryPool.PageRecord page = this.getPage(nPageNo);
                let mut page: KMemoryPool::PageRecord = self.getPage(nPageNo);
                // java page.write(off, s, start, nCharWrite);
                page.write(off, s, start, nCharWrite);
                // java this.putPage(page);
                self.putPage(&page);
                // java this.mLength += nCharWrite;
                self.mLength = self.mLength + nCharWrite;
                // java start += nCharWrite;
                start = start + nCharWrite;
              }
              // java IOException e
              todo!();
                // java e.printStackTrace();
                e.printStackTrace();
                // java throw new RuntimeException(e.getMessage());
                panic!("{}", RuntimeException::new(e.getMessage()));
          // java return len;
          return len;
    }

    // java public void dispose()
    fn dispose(&mut self) {
      // java if (this.mCurrentPage != null)
      if let Some(mCurrentPage) = self.mCurrentPage.as_mut() {
        // java this.mCurrentPage = null;
        mCurrentPage = None;
      }
      // java if (this.mMemoryPool != null)
      if let Some(mMemoryPool) = self.mMemoryPool.as_mut() {
        // java this.mMemoryPool.dispose();
        mMemoryPool.dispose();
      }
      // java this.mMemoryPool = null;
      self.mMemoryPool = None;
    }

    // java @Deprecated public char[] getChars(int start, int end)
    fn getChars__int_int(&self, start: i32, end: i32) -> Vec<char> {
      // java KSAssert.assertTrue(start >= 0 && end < this.mLength && start <= end);
      KSAssert::assertTrue(start >= 0 && end < self.mLength && start <= end);
      // java char[] buf = new char[end - start];
      let mut buf: Vec<char> = vec![Default::default(); (end - start) as usize];
      // java this.getChars(start, end, buf, 0);
      self.getChars__int_int_chararr_int(start, end, &buf, 0);
      // java return buf;
      return buf;
    }

    // java public void getChars(int start, int end, char[] buffer, int bufStart)
    fn getChars__int_int_chararr_int(&mut self, start: i32, end: i32, buffer: &Vec<char>, bufStart: i32) {
        // java (this)
        self;
              // java while (start < end)
              while start < end {
                // java int index = start << 1;
                let mut index: i32 = start << 1;
                // java int nPageNo = index >> KMemoryPool.PAGE_SIZE_SHIFT;
                let mut nPageNo: i32 = index >> KMemoryPool::PAGE_SIZE_SHIFT;
                // java int off = index & KMemoryPool.PAGE_SIZE - 1;
                let mut off: i32 = index & KMemoryPool::PAGE_SIZE - 1;
                // java int nbytesRemain = KMemoryPool.PAGE_SIZE - off;
                let mut nbytesRemain: i32 = KMemoryPool::PAGE_SIZE - off;
                // java int nCharRead = Math.min(nbytesRemain >> 1, end - start);
                let mut nCharRead: i32 = Math::min(nbytesRemain >> 1, end - start);
                // java KMemoryPool.PageRecord page = this.getPage(nPageNo);
                let mut page: KMemoryPool::PageRecord = self.getPage(nPageNo);
                // java page.read(off, nCharRead, buffer, bufStart);
                page.read(off, nCharRead, buffer, bufStart);
                // java this.putPage(page);
                self.putPage(&page);
                // java start += nCharRead;
                start = start + nCharRead;
                // java bufStart += nCharRead;
                bufStart = bufStart + nCharRead;
              }
              // java IOException e
              todo!();
                // java e.printStackTrace();
                e.printStackTrace();
                // java throw new RuntimeException(e.getMessage());
                panic!("{}", RuntimeException::new(e.getMessage()));
    }

    // java public void set(int index, char c)
    fn set(&mut self, index: i32, c: char) {
        // java (this)
        self;
          // java KSAssert.assertTrue(index <= this.mLength);
          KSAssert::assertTrue(index <= self.mLength);
          // java index <<= 1;
          index = index << 1;
              // java int nPageNo = index >> KMemoryPool.PAGE_SIZE_SHIFT;
              let mut nPageNo: i32 = index >> KMemoryPool::PAGE_SIZE_SHIFT;
              // java int off = index & KMemoryPool.PAGE_SIZE - 1;
              let mut off: i32 = index & KMemoryPool::PAGE_SIZE - 1;
              // java KMemoryPool.PageRecord page = this.getPage(nPageNo);
              let mut page: KMemoryPool::PageRecord = self.getPage(nPageNo);
              // java page.write(off, (byte)(c & 255));
              page.write(off, ((c & 255) as u8));
              // java page.write(off + 1, (byte)(c >> 8));
              page.write(off + 1, ((c >> 8) as u8));
              // java this.putPage(page);
              self.putPage(&page);
              // java IOException e
              todo!();
                // java e.printStackTrace();
                e.printStackTrace();
                // java throw new RuntimeException(e.getMessage());
                panic!("{}", RuntimeException::new(e.getMessage()));
    }

}

// java public class KSwappableTextPool implements ITextPool
impl KSwappableTextPool {
    // java public  KSwappableTextPool()
    pub fn KSwappableTextPool() -> Self {
      Self { mLength: Default::default(), mMemoryPool: Default::default(), mCurrentPage: Default::default() }
    }

    // java public void setPoolSize(int nbytes)
    pub fn setPoolSize(&mut self, nbytes: i32) {
      // java this.mMemoryPool.setPoolSize(nbytes);
      self.mMemoryPool.setPoolSize(nbytes);
    }

    // java public int length()
    pub fn length(&self) -> i32 {
      // java return this.mLength;
      return self.mLength;
    }

    // java private KMemoryPool.PageRecord getPage(int nPageNo)
    pub fn getPage(&mut self, nPageNo: i32) -> KMemoryPool::PageRecord {
      // java if (this.mCurrentPage != null)
      if let Some(mCurrentPage) = self.mCurrentPage.as_mut() {
        // java if (this.mCurrentPage.mPageNo == nPageNo)
        if mCurrentPage.mPageNo == nPageNo {
          // java return this.mCurrentPage;
          return mCurrentPage;
        }
        // java this.mMemoryPool.putPage(this.mCurrentPage);
        self.mMemoryPool.putPage(&mut mCurrentPage);
        // java this.mCurrentPage = null;
        mCurrentPage = None;
      }
      // java this.mCurrentPage = this.mMemoryPool.getPage(nPageNo);
      self.mCurrentPage = self.mMemoryPool.getPage(nPageNo);
      // java return this.mCurrentPage;
      return self.mCurrentPage;
    }

    // java private void putPage(KMemoryPool.PageRecord page)
    pub fn putPage(&mut self, page: &KMemoryPool::PageRecord) {
    }

    // java public char charAt(int index)
    pub fn charAt(&mut self, index: i32) -> char {
        // java (this)
        self;
          // java KSAssert.assertTrue(index < this.mLength);
          KSAssert::assertTrue(index < self.mLength);
          // java index <<= 1;
          index = index << 1;
          // java char var10000;
          let mut var10000: char;
              // java int nPageNo = index >> KMemoryPool.PAGE_SIZE_SHIFT;
              let mut nPageNo: i32 = index >> KMemoryPool::PAGE_SIZE_SHIFT;
              // java int off = index & KMemoryPool.PAGE_SIZE - 1;
              let mut off: i32 = index & KMemoryPool::PAGE_SIZE - 1;
              // java KMemoryPool.PageRecord page = this.getPage(nPageNo);
              let mut page: KMemoryPool::PageRecord = self.getPage(nPageNo);
              // java this.putPage(page);
              self.putPage(&page);
              // java int low = page.read(off) & 255;
              let mut low: i32 = page.read(off) & 255;
              // java int high = page.read(off + 1) & 255;
              let mut high: i32 = page.read(off + 1) & 255;
              // java var10000 = (char)(low + (high << 8));
              var10000 = todo!("cast → char");
              // java IOException e
              todo!();
                // java e.printStackTrace();
                e.printStackTrace();
                // java throw new RuntimeException(e.getMessage());
                panic!("{}", RuntimeException::new(e.getMessage()));
          // java return var10000;
          return var10000;
    }

    // java public CharSequence subSequence(int start, int end)
    pub fn subSequence(&mut self, start: i32, end: i32) -> Option<CharSequence> {
      // java return null;
      return None;
    }

}
