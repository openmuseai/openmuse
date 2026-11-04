// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use super::PlexOfCps::{PlexOfCps, GenericPropertyNode};
use super::TextPieceTable::TextPieceTable;
use super::CHPFormattedDiskPage::CHPFormattedDiskPage;
use super::CHPX::CHPX;

fn le_get_int(data: &[u8], offset: usize) -> i32 {
    i32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
}

// Java: public class CHPBinTableReadOnlyComplex extends CHPBinTableReadOnly
pub struct CHPBinTableReadOnlyComplex {
    // Java field: ArrayList<PageDescriptor> mPages
    mPages: Vec<PageDescriptor>,
    // Java field: int mTotalRuns
    mTotalRuns: i32,
    // Java field: int mCurrentPageIndex
    mCurrentPageIndex: i32,
    // Java field: int mChpxIndex
    pub mChpxIndex: i32,
    // Java field: CHPFormattedDiskPage mCurrentPage
    mCurrentPage: Option<CHPFormattedDiskPage>,
    // Java field: int mMarkSavePageIndex
    mMarkSavePageIndex: i32,
    // Java field: int mMarkSaveChpxIndex
    mMarkSaveChpxIndex: i32,
    pub _binTable: PlexOfCps,
    pub _fcMin: i32,
    pub _documentStream: Option<*mut DocumentInputStream>,
    pub tpt: Option<*mut TextPieceTable>,
}

// Java: public class PageDescriptor
pub struct PageDescriptor {
    // Java field: int mRunCount
    pub mRunCount: i32,
    // Java field: int mStart
    pub mStart: i32,
    // Java field: int mEnd
    pub mEnd: i32,
}

// Java: public class CHPBinTableReadOnlyComplex extends CHPBinTableReadOnly
impl CHPBinTableReadOnlyComplex {
    // Java: CHPBinTableReadOnlyComplex(DocumentInputStream documentStream, DocumentInputStream tableStream, int offset, int size, int fcMin, TextPieceTable tpt)
    pub fn new(document_stream: &mut DocumentInputStream, table_stream: DocumentInputStream, offset: i32, size: i32, fc_min: i32, tpt: *mut TextPieceTable) -> Self {
        // Java: super(documentStream, tableStream, offset, size, fcMin, tpt, true);
        let _binTable = PlexOfCps::new_from_stream(table_stream, offset, size, 4);
        let mut result = CHPBinTableReadOnlyComplex {
            mPages: Vec::new(),
            mTotalRuns: 0,
            mCurrentPageIndex: 0,
            mChpxIndex: 0,
            mCurrentPage: None,
            mMarkSavePageIndex: 0,
            mMarkSaveChpxIndex: 0,
            _binTable,
            _fcMin: fc_min,
            _documentStream: Some(document_stream as *mut DocumentInputStream),
            tpt: Some(tpt),
        };
        // Java: this.readAll();
        result.readAll();
        result
    }

    // Java: void readAll()
    pub fn readAll(&mut self) {
        // Java: int nPages = this._binTable.length();
        let n_pages = self._binTable.length();
        // Java: this.mPages.ensureCapacity(nPages);
        self.mPages.reserve(n_pages as usize);
        // Java: for(int iPage = 0; iPage < nPages; ++iPage)
        for i_page in 0..n_pages {
            // Java: GenericPropertyNode node = this._binTable.getProperty(iPage);
            let node = self._binTable.getProperty(i_page);
            // Java: int pageNum = LittleEndian.getInt(node.getBytes(), node.getOffset());
            let page_num = le_get_int(node.getBytes(), node.getOffset() as usize);
            // Java: int pageOffset = 512 * pageNum;
            let page_offset = 512 * page_num;
            // Java: PageDescriptor pPage = new PageDescriptor();
            let doc_stream = unsafe { &mut *self._documentStream.unwrap() };
            // Java: pPage.mRunCount = this.page_read_crun(this._documentStream, pageOffset);
            let run_count = Self::page_read_crun_s(doc_stream, page_offset);
            // Java: pPage.mStart = this.page_read_raw_start(this._documentStream, pageOffset, 0);
            let start = Self::page_read_raw_start_s(doc_stream, page_offset, 0);
            // Java: int nRawEnd = this.page_read_raw_end(this._documentStream, pageOffset, pPage.mRunCount - 1);
            let end = Self::page_read_raw_end_s(doc_stream, page_offset, run_count - 1);
            // Java: pPage.mEnd = nRawEnd;
            // Java: this.mPages.add(pPage);
            self.mPages.push(PageDescriptor { mRunCount: run_count, mStart: start, mEnd: end });
        }
    }

    // Java: int page_read_crun(DocumentInputStream documentStream, int pageOffset)
    fn page_read_crun_s(document_stream: &mut DocumentInputStream, page_offset: i32) -> i32 {
        // Java: documentStream.seek((long)(pageOffset + 511));
        document_stream.seek((page_offset + 511) as i64);
        // Java: return documentStream.readUByte();
        document_stream.readUByte() as i32
    }

    // Java: int page_read_raw_start(DocumentInputStream fkp, int pageOffset, int irun)
    fn page_read_raw_start_s(fkp: &mut DocumentInputStream, page_offset: i32, irun: i32) -> i32 {
        // Java: fkp.seek((long)(pageOffset + irun * 4));
        fkp.seek((page_offset + irun * 4) as i64);
        // Java: return fkp.readInt();
        fkp.readInt()
    }

    // Java: int page_read_raw_end(DocumentInputStream fkp, int pageOffset, int irun)
    fn page_read_raw_end_s(fkp: &mut DocumentInputStream, page_offset: i32, irun: i32) -> i32 {
        // Java: fkp.seek((long)(pageOffset + (irun + 1) * 4));
        fkp.seek((page_offset + (irun + 1) * 4) as i64);
        // Java: return fkp.readInt();
        fkp.readInt()
    }

    // Java: void mark()
    pub fn mark(&mut self) {
        // Java: this.mMarkSavePageIndex = this.mCurrentPageIndex;
        self.mMarkSavePageIndex = self.mCurrentPageIndex;
        // Java: this.mMarkSaveChpxIndex = this.mChpxIndex;
        self.mMarkSaveChpxIndex = self.mChpxIndex;
        // Java: this.mCurrentPageIndex = 0;
        self.mCurrentPageIndex = 0;
        // Java: this.mChpxIndex = 0;
        self.mChpxIndex = 0;
        // Java: this.mCurrentPage = null;
        self.mCurrentPage = None;
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.mCurrentPageIndex = this.mMarkSavePageIndex;
        self.mCurrentPageIndex = self.mMarkSavePageIndex;
        // Java: this.mChpxIndex = this.mMarkSaveChpxIndex;
        self.mChpxIndex = self.mMarkSaveChpxIndex;
        // Java: this.mCurrentPage = null;
        self.mCurrentPage = None;
    }

    // Java: void seek(int bytesOffset)
    pub fn seek(&mut self, bytes_offset: i32) {
        // Java: if (this.mCurrentPage == null)
        if self.mCurrentPage.is_none() {
            // Java: this.load_current_page();
            self.load_current_page();
        }
        // Java: this.mChpxIndex = binarySearch(this.mCurrentPage.getAll(), bytesOffset);
        self.mChpxIndex = Self::binary_search(self.mCurrentPage.as_ref().unwrap().getAll(), bytes_offset);
        // Java: if (this.mChpxIndex == -1)
        if self.mChpxIndex == -1 {
            // Java: this.load_page(bytesOffset);
            self.load_page(bytes_offset);
            // Java: this.mChpxIndex = binarySearch(this.mCurrentPage.getAll(), bytesOffset);
            self.mChpxIndex = Self::binary_search(self.mCurrentPage.as_ref().unwrap().getAll(), bytes_offset);
        }
    }

    // Java: void load_page(int bytesOffset)
    fn load_page(&mut self, bytes_offset: i32) {
        // Java: this.mCurrentPageIndex = binarySearch_page(this.mPages, bytesOffset);
        self.mCurrentPageIndex = Self::binary_search_page(&self.mPages, bytes_offset);
        // Java: this.load_current_page();
        self.load_current_page();
    }

    // Java: static int binarySearch(ArrayList<CHPX> runs, int bytesOffset)
    fn binary_search(runs: &Vec<CHPX>, bytes_offset: i32) -> i32 {
        // Java: int low = 0;
        let mut low: i32 = 0;
        // Java: int mid = 0;
        let mut mid: i32 = 0;
        // Java: CHPX midVal = null;
        // Java: int high = runs.size() - 1;
        let mut high: i32 = runs.len() as i32 - 1;
        // Java: while(low <= high)
        while low <= high {
            // Java: mid = (low + high) / 2;
            mid = (low + high) / 2;
            // Java: midVal = (CHPX)runs.get(mid);
            let mid_val = &runs[mid as usize];
            // Java: if (midVal.getStartBytes() > bytesOffset)
            if mid_val.getStartBytes() > bytes_offset {
                // Java: high = mid - 1;
                high = mid - 1;
            }
            // Java: else
            else {
                // Java: if (midVal.getEndBytes() > bytesOffset)
                if mid_val.getEndBytes() > bytes_offset {
                    // Java: return mid;
                    return mid;
                }
                // Java: low = mid + 1;
                low = mid + 1;
            }
        }
        // Java: return -1;
        -1
    }

    // Java: static int binarySearch_page(ArrayList<PageDescriptor> pages, int bytesOffset)
    fn binary_search_page(pages: &Vec<PageDescriptor>, bytes_offset: i32) -> i32 {
        // Java: int low = 0;
        let mut low: i32 = 0;
        // Java: int mid = 0;
        let mut mid: i32 = 0;
        // Java: PageDescriptor midVal = null;
        // Java: int high = pages.size() - 1;
        let mut high: i32 = pages.len() as i32 - 1;
        // Java: while(low <= high)
        while low <= high {
            // Java: mid = (low + high) / 2;
            mid = (low + high) / 2;
            // Java: midVal = (PageDescriptor)pages.get(mid);
            let mid_val = &pages[mid as usize];
            // Java: if (midVal.mStart > bytesOffset)
            if mid_val.mStart > bytes_offset {
                // Java: high = mid - 1;
                high = mid - 1;
            }
            // Java: else
            else {
                // Java: if (midVal.mEnd > bytesOffset)
                if mid_val.mEnd > bytes_offset {
                    // Java: return mid;
                    return mid;
                }
                // Java: low = mid + 1;
                low = mid + 1;
            }
        }
        // Java: return high;
        high
    }

    // Java: void load_current_page()
    fn load_current_page(&mut self) {
        // Java: GenericPropertyNode node = this._binTable.getProperty(this.mCurrentPageIndex);
        let node = self._binTable.getProperty(self.mCurrentPageIndex);
        // Java: int pageNum = LittleEndian.getInt(node.getBytes(), node.getOffset());
        let page_num = le_get_int(node.getBytes(), node.getOffset() as usize);
        // Java: int pageOffset = 512 * pageNum;
        let page_offset = 512 * page_num;
        // Java: this.mCurrentPage = new CHPFormattedDiskPage(this._documentStream, pageOffset, this._fcMin, this.tpt, true);
        let doc_stream = unsafe { &mut *self._documentStream.unwrap() };
        let tpt = unsafe { &*self.tpt.unwrap() };
        self.mCurrentPage = Some(CHPFormattedDiskPage::new_from_stream(doc_stream, page_offset, self._fcMin, tpt, true));
    }

    // Java: CHPX getNextTextRun()
    pub fn getNextTextRun(&self) -> Option<&CHPX> {
        // Java: return this.mChpxIndex == -1 ? null : this.mCurrentPage.getCHPX(this.mChpxIndex);
        if self.mChpxIndex == -1 { None } else { self.mCurrentPage.as_ref().and_then(|p| p.getCHPX(self.mChpxIndex)) }
    }
}

// Java: public class PageDescriptor
impl PageDescriptor {
    pub fn new() -> Self {
        PageDescriptor { mRunCount: 0, mStart: 0, mEnd: 0 }
    }
}
