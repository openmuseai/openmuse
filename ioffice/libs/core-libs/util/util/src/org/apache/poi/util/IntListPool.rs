#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.base.assertion.KSAssert
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import cn.wps.moffice.util.KSLog
// Java: import cn.wps.moffice.util.PlatformAdapter
// Java: import java.io.IOException
// Java: import java.util.HashMap
use std::collections::HashMap;
use std::io::{Read, Write, Seek, SeekFrom};

use super::LittleEndian::LittleEndian;

// Java field: String TAG
const TAG: &str = "IntListPool";

// Java: public class IntListPool
pub struct IntListPool {
    // Java field: HashMap<Integer, PageRecord> mPageMap
    mPageMap: HashMap<i32, usize>,
    // Java field: RandomAccessFile mFile
    mFile: std::fs::File,
    // Java field: PageRecord mFreePages
    mFreePages: usize,
    // Java field: IntListPool mInstance
    // Java field: int mMaxPageAllowed
    mMaxPageAllowed: i32,
    // Java field: PageRecord mCurrentPage
    mCurrentPage: Option<usize>,
    // Java field: int PAGE_SIZE
    pages: Vec<PageRecord>,
}

pub const PAGE_SIZE: i32 = 8192;

// Java: public class SwappedIntList
pub struct SwappedIntList {
    // Java field: long mOffset
    pub mOffset: i64,
    // Java field: int mLength
    pub mLength: i32,
    // Java field: IntListPool pool
}

// Java: public class Builder
pub struct Builder {
    // Java field: long offset
    pub offset: i64,
    // Java field: _buf: &[u8]
    pub buf: Vec<u8>,
    // Java field: int mIndex
    pub mIndex: i32,
    // Java field: IntListPool pool
    // Java field: int mLength
    pub mLength: i32,
}

// Java: public class PageRecord
pub struct PageRecord {
    // Java field: int mPageNo
    pub mPageNo: i32,
    // Java field: _buf: &[u8]
    pub buf: Vec<i8>,
    // Java field: int mRef
    pub mRef: i32,
    // Java field: PageRecord mPrevFree
    pub mPrevFree: Option<usize>,
    // Java field: PageRecord mNextFree
    pub mNextFree: Option<usize>,
}

// Java: public class IntListPool
impl IntListPool {
    // Java: static IntListPool getInstance()
    pub fn getInstance() -> Self {
        // Java: if (null == mInstance)
        // Java: mInstance = new IntListPool();
        // Java: return mInstance;
        Self::new()
    }

    // Java: IntListPool()
    pub fn new() -> Self {
        // Java: super();
        // Java: File mFileHandle = PlatformAdapter.createTempFile("intListp", (String)null);
        // Java: if (mFileHandle == null)
        // Java: mFileHandle = File.createTempFile("mmp", (String)null);
        let temp_path = std::env::temp_dir().join(format!("intListp-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()));
        // Java: mFile = new RandomAccessFile(mFileHandle, "rw");
        let file = std::fs::OpenOptions::new().read(true).write(true).create(true).open(&temp_path).expect("Failed to create temp file");
        // Java: this.mFreePages = new PageRecord(-1, 0);
        let sentinel = PageRecord::new_with_size(-1, 0);
        let mut pool = Self {
            mPageMap: HashMap::new(),
            mFreePages: 0,
            mMaxPageAllowed: 10,
            mCurrentPage: None,
            pages: vec![sentinel],
            mFile: file,
        };
        // Java: this.mFreePages.mNextFree = this.mFreePages.mPrevFree = this.mFreePages;
        pool.pages[0].mNextFree = Some(0);
        pool.pages[0].mPrevFree = Some(0);
        pool
    }

    // Java: PageRecord getPage(int nPageNo)
    fn getPage(&mut self, n_page_no: i32) -> usize {
        // Java: if (this.mCurrentPage != null)
        if let Some(cur_idx) = self.mCurrentPage
        {
            // Java: if (this.mCurrentPage.mPageNo == nPageNo)
            if self.pages[cur_idx].mPageNo == n_page_no
            {
                // Java: return this.mCurrentPage;
                return cur_idx;
            }
            // Java: this.putPage(this.mCurrentPage);
            self.pages[cur_idx].unlock();
            if self.pages[cur_idx].mRef == 0 {
                self.freePage_push_back(cur_idx);
            }
            // Java: this.mCurrentPage = null;
            self.mCurrentPage = None;
        }
        // Java: PageRecord page = (PageRecord)this.mPageMap.get(nPageNo);
        let page_idx = self.mPageMap.get(&n_page_no).copied();
        // Java: if (page == null)
        let page_idx = if page_idx.is_none()
        {
            let idx;
            // Java: if (this.mPageMap.size() < this.mMaxPageAllowed)
            if (self.mPageMap.len() as i32) < self.mMaxPageAllowed
            {
                // Java: page = new PageRecord(nPageNo);
                let page = PageRecord::new(n_page_no);
                self.pages.push(page);
                idx = self.pages.len() - 1;
            }
            // Java: else
            else
            {
                // Java: page = this.freePage_pop_front();
                idx = self.freePage_pop_front().expect("No free pages");
                // Java: this.mPageMap.remove(page.mPageNo);
                let old_page_no = self.pages[idx].mPageNo;
                self.mPageMap.remove(&old_page_no);
                // Java: page.mPageNo = nPageNo;
                self.pages[idx].mPageNo = n_page_no;
                // Java: page.buf = new byte[8192];
                self.pages[idx].buf = vec![0i8; 8192];
            }
            // Java: if (nPageNo >= 0)
            if n_page_no >= 0
            {
                // Java: this.readPage(page);
                self.readPage(idx);
            }
            // Java: this.mPageMap.put(page.mPageNo, page);
            self.mPageMap.insert(n_page_no, idx);
            idx
        }
        // Java: else if (page.mRef == 0)
        else
        {
            let idx = page_idx.unwrap();
            if self.pages[idx].mRef == 0
            {
                // Java: KSAssert.assertTrue(page.mNextFree != null);
                assert!(self.pages[idx].mNextFree.is_some());
                // Java: this.freePage_remove(page);
                self.freePage_remove(idx);
            }
            idx
        };
        // Java: page.lock();
        self.pages[page_idx].lock();
        // Java: this.mCurrentPage = page;
        self.mCurrentPage = Some(page_idx);
        // Java: return page;
        page_idx
    }

    // Java: void readPage(PageRecord page)
    fn readPage(&mut self, page_idx: usize) {
        // Java: int off = page.mPageNo * 8192;
        let off = self.pages[page_idx].mPageNo as u64 * 8192;
        // Java: mFile.seek((long)off);
        let _ = self.mFile.seek(SeekFrom::Start(off));
        // Java: mFile.read(page.buf);
        let buf_len = self.pages[page_idx].buf.len();
        let buf_ptr = self.pages[page_idx].buf.as_mut_ptr() as *mut u8;
        let buf_slice = unsafe { std::slice::from_raw_parts_mut(buf_ptr, buf_len) };
        let _ = self.mFile.read(buf_slice);
        // Java: KSLog.d(TAG, "**********************read page newoffset = " + page.mPageNo + "   size = " + page.buf.length);
    }

    // Java: PageRecord freePage_pop_front()
    fn freePage_pop_front(&mut self) -> Option<usize> {
        let sentinel = self.mFreePages;
        // Java: if (this.mFreePages.mNextFree == this.mFreePages)
        if self.pages[sentinel].mNextFree == Some(sentinel)
        {
            // Java: return null;
            return None;
        }
        // Java: else
        {
            // Java: PageRecord page = this.mFreePages.mNextFree;
            let page_idx = self.pages[sentinel].mNextFree.unwrap();
            // Java: this.mFreePages.mNextFree = page.mNextFree;
            let page_next = self.pages[page_idx].mNextFree;
            self.pages[sentinel].mNextFree = page_next;
            // Java: page.mNextFree.mPrevFree = this.mFreePages;
            if let Some(next) = page_next {
                self.pages[next].mPrevFree = Some(sentinel);
            }
            // Java: page.mPrevFree = page.mNextFree = null;
            self.pages[page_idx].mPrevFree = None;
            self.pages[page_idx].mNextFree = None;
            // Java: return page;
            Some(page_idx)
        }
    }

    // Java: void freePage_push_back(PageRecord page)
    fn freePage_push_back(&mut self, page_idx: usize) {
        let sentinel = self.mFreePages;
        // Java: page.mNextFree = this.mFreePages;
        self.pages[page_idx].mNextFree = Some(sentinel);
        // Java: page.mPrevFree = this.mFreePages.mPrevFree;
        let sentinel_prev = self.pages[sentinel].mPrevFree;
        self.pages[page_idx].mPrevFree = sentinel_prev;
        // Java: this.mFreePages.mPrevFree.mNextFree = page;
        if let Some(prev) = sentinel_prev {
            self.pages[prev].mNextFree = Some(page_idx);
        }
        // Java: this.mFreePages.mPrevFree = page;
        self.pages[sentinel].mPrevFree = Some(page_idx);
    }

    // Java: void freePage_remove(PageRecord page)
    fn freePage_remove(&mut self, page_idx: usize) {
        // Java: page.mPrevFree.mNextFree = page.mNextFree;
        let prev = self.pages[page_idx].mPrevFree;
        let next = self.pages[page_idx].mNextFree;
        if let Some(p) = prev {
            self.pages[p].mNextFree = next;
        }
        // Java: page.mNextFree.mPrevFree = page.mPrevFree;
        if let Some(n) = next {
            self.pages[n].mPrevFree = prev;
        }
        // Java: page.mPrevFree = null;
        self.pages[page_idx].mPrevFree = None;
        // Java: page.mNextFree = null;
        self.pages[page_idx].mNextFree = None;
    }

    // Java: void putPage(PageRecord page)
    fn putPage(&mut self, page_idx: usize) {
        // Java: page.unlock();
        self.pages[page_idx].unlock();
        // Java: if (page.mRef == 0)
        if self.pages[page_idx].mRef == 0
        {
            // Java: this.freePage_push_back(page);
            self.freePage_push_back(page_idx);
        }
    }

    // Java: void write(_buffer: &[u8])
    fn write_buf(&mut self, buffer: &[u8]) {
        // Java: mFile.seek(mFile.length());
        let _ = self.mFile.seek(SeekFrom::End(0));
        // Java: mFile.write(buffer);
        let _ = self.mFile.write_all(buffer);
    }

    // Java: long length()
    pub fn length(&self) -> i64 {
        // Java: return mFile.length();
        self.mFile.metadata().map(|m| m.len() as i64).unwrap_or(0)
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this.mPageMap.clear();
        self.mPageMap.clear();
        // Java: this.mFreePages = null;
        // Java: this.mCurrentPage = null;
        self.mCurrentPage = None;
        // Java: try
        {
            // Java: mFile.close();
        }
        // Java: catch (IOException e)
        {
            // Java: KSLog.e(TAG, "Failed to close RandomAccessFile.", e);
        }
    }

    // Java: SwappedIntList createSwappedIntList(long offset, int length)
    fn createSwappedIntList(&self, offset: i64, length: i32) -> SwappedIntList {
        // Java: return new SwappedIntList(offset, length, this);
        SwappedIntList::new(offset, length)
    }
}

// Java: public class SwappedIntList
impl SwappedIntList {
    // Java: SwappedIntList(long offset, int length, IntListPool pool)
    pub fn new(offset: i64, length: i32) -> Self {
        // Java: super(0);
        // Java: this.mOffset = offset;
        // Java: this.mLength = length;
        // Java: this.pool = pool;
        Self {
            mOffset: offset,
            mLength: length,
        }
    }

    // Java: int get(int blockIndex)
    pub fn get(&self, pool: &mut IntListPool, block_index: i32) -> i32 {
        // Java: long offset = this.mOffset + (long)(blockIndex << 2);
        let offset = self.mOffset + ((block_index as i64) << 2);
        // Java: int nPageNo = (int)(offset / 8192L);
        let n_page_no = (offset / 8192) as i32;
        // Java: PageRecord page;
        // Java: try
        // Java: page = this.pool.getPage(nPageNo);
        let page_idx = pool.getPage(n_page_no);
        // Java: catch (IOException e)
        // Java: e.printStackTrace();
        // Java: throw new RuntimeException(e.getMessage());
        // Java: return page.get(offset - (long)(page.mPageNo * 8192));
        let page_no = pool.pages[page_idx].mPageNo;
        pool.pages[page_idx].get(offset - (page_no as i64 * 8192))
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.mLength;
        self.mLength
    }
}

// Java: public class Builder
impl Builder {
    // Java: Builder(IntListPool pool)
    pub fn new(pool: &IntListPool) -> Self {
        // Java: super();
        // Java: this.pool = pool;
        // Java: this.offset = pool.length();
        Self {
            offset: pool.length(),
            buf: Vec::new(),
            mIndex: 0,
            mLength: 0,
        }
    }

    // Java: SwappedIntList build()
    pub fn build(&self, pool: &IntListPool) -> SwappedIntList {
        // Java: return this.pool.createSwappedIntList(this.offset, this.mLength);
        pool.createSwappedIntList(self.offset, self.mLength)
    }

    // Java: void begin()
    pub fn begin(&mut self, pool: &IntListPool) {
        // Java: this.buf = new byte[8192];
        self.buf = vec![0u8; 8192];
        // Java: this.mIndex = 0;
        self.mIndex = 0;
        // Java: this.reset();
        self.mLength = 0;
        self.offset = pool.length() + self.mIndex as i64;
    }

    // Java: void add(int value)
    pub fn add(&mut self, pool: &mut IntListPool, value: i32) {
        // Java: this.writeFullBuf();
        self.writeFullBuf(pool);
        // Java: LittleEndian.putInt(this.buf, this.mIndex, value);
        let idx = self.mIndex as usize;
        let bytes = value.to_le_bytes();
        self.buf[idx..idx + 4].copy_from_slice(&bytes);
        // Java: this.mIndex += 4;
        self.mIndex += 4;
        // Java: ++this.mLength;
        self.mLength += 1;
    }

    // Java: void end()
    pub fn end(&mut self, pool: &mut IntListPool) {
        // Java: this.writeRemaindBuf();
        self.writeRemaindBuf(pool);
        // Java: this.buf = null;
        self.buf = Vec::new();
    }

    // Java: void reset()
    pub fn reset(&mut self, pool: &IntListPool) {
        // Java: this.mLength = 0;
        self.mLength = 0;
        // Java: this.offset = this.pool.length() + (long)this.mIndex;
        self.offset = pool.length() + self.mIndex as i64;
    }

    // Java: void writeFullBuf()
    fn writeFullBuf(&mut self, pool: &mut IntListPool) {
        // Java: if (this.mIndex >= 8192)
        if self.mIndex >= 8192
        {
            // Java: this.pool.write(this.buf);
            pool.write_buf(&self.buf);
            // Java: KSLog.d(IntListPool.TAG, "**********************write page newoffset = " + this.offset + "   size = " + this.buf.length);
            // Java: this.buf = new byte[8192];
            self.buf = vec![0u8; 8192];
            // Java: this.mIndex = 0;
            self.mIndex = 0;
        }
    }

    // Java: void writeRemaindBuf()
    fn writeRemaindBuf(&mut self, pool: &mut IntListPool) {
        // Java: if (this.mIndex > 0)
        if self.mIndex > 0
        {
            // Java: _copy: &[u8] = new byte[this.mIndex];
            // Java: System.arraycopy(this.buf, 0, copy, 0, this.mIndex);
            let copy = self.buf[..self.mIndex as usize].to_vec();
            // Java: this.pool.write(copy);
            pool.write_buf(&copy);
            // Java: KSLog.d(IntListPool.TAG, "**********************write page newoffset = " + this.offset + "   size = " + copy.length);
            // Java: this.mIndex = 0;
            self.mIndex = 0;
        }
    }

    // Java: int getLength()
    pub fn getLength(&self) -> i32 {
        // Java: return this.mLength;
        self.mLength
    }
}

// Java: public class PageRecord
impl PageRecord {
    // Java: PageRecord(int nPageNo)
    pub fn new(n_page_no: i32) -> Self {
        // Java: this(nPageNo, 8192);
        Self::new_with_size(n_page_no, 8192)
    }

    // Java: PageRecord(int nPageNo, int size)
    pub fn new_with_size(n_page_no: i32, size: i32) -> Self {
        // Java: super();
        // Java: this.mPrevFree = null;
        // Java: this.mNextFree = null;
        // Java: this.buf = new byte[size];
        // Java: this.mPageNo = nPageNo;
        Self {
            mPageNo: n_page_no,
            buf: vec![0i8; size as usize],
            mRef: 0,
            mPrevFree: None,
            mNextFree: None,
        }
    }

    // Java: int get(long offset)
    pub fn get(&self, offset: i64) -> i32 {
        // Java: return LittleEndian.getInt(this.buf, (int)(offset & 65535L));
        LittleEndian::getInt(&self.buf, (offset & 65535) as i32)
    }

    // Java: void lock()
    pub fn lock(&mut self) {
        // Java: ++this.mRef;
        self.mRef += 1;
    }

    // Java: void unlock()
    pub fn unlock(&mut self) {
        // Java: --this.mRef;
        self.mRef -= 1;
        // Java: KSAssert.assertTrue(this.mRef >= 0);
        assert!(self.mRef >= 0);
    }
}
