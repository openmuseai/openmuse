// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian
use super::PAPBinTableReadOnly::PAPBinTableReadOnly;
use super::PAPFormattedDiskPage::PAPFormattedDiskPage;
use super::PAPBinTable::PAPX;
use super::PlexOfCps::GenericPropertyNode;
use super::TextPieceTable::TextPieceTable;

fn le_get_int(buf: &[u8], offset: i32) -> i32 {
    let o = offset as usize;
    (buf[o] as i32 & 0xFF) | ((buf[o+1] as i32 & 0xFF) << 8) |
    ((buf[o+2] as i32 & 0xFF) << 16) | ((buf[o+3] as i32 & 0xFF) << 24)
}

// Java: public class PAPBinTableReadOnlyComplex extends PAPBinTableReadOnly
pub struct PAPBinTableReadOnlyComplex {
    pub base: PAPBinTableReadOnly,
    // Java field: ArrayList<PageDescriptor> mPages
    mPages: Vec<PageDescriptor>,
    // Java field: int mTotalRuns
    mTotalRuns: i32,
    // Java field: int mCurrentPageIndex
    mCurrentPageIndex: i32,
    // Java field: int mPapxIndex
    pub(crate) mPapxIndex: i32,
    // Java field: PAPFormattedDiskPage mCurrentPage
    mCurrentPage: Option<PAPFormattedDiskPage>,
    // Java field: int mMarkSavePageIndex
    mMarkSavePageIndex: i32,
    // Java field: int mMarkSavePapxIndex
    mMarkSavePapxIndex: i32,
}

// Java: public class PageDescriptor
pub struct PageDescriptor {
    // Java field: int mParaCount
    pub(crate) mParaCount: i32,
    // Java field: int mStart
    pub(crate) mStart: i32,
    // Java field: int mEnd
    pub(crate) mEnd: i32,
}

// Java: public class PAPBinTableReadOnlyComplex extends PAPBinTableReadOnly
impl PAPBinTableReadOnlyComplex {
    // Java: PAPBinTableReadOnlyComplex(DocumentInputStream documentStream, DocumentInputStream tableStream, DocumentInputStream dataStream, int offset, int size, int fcMin, TextPieceTable tpt, boolean fComplex)
    pub fn new(document_stream: DocumentInputStream, table_stream: DocumentInputStream, data_stream: DocumentInputStream, offset: i32, size: i32, fc_min: i32, tpt: TextPieceTable, _f_complex: bool) -> Self {
      // Java: super(documentStream, tableStream, dataStream, offset, size, fcMin, tpt, true);
      let base = PAPBinTableReadOnly::new(document_stream, table_stream, data_stream, offset, size, fc_min, tpt, true);
      let mut this = PAPBinTableReadOnlyComplex {
          base,
          mPages: Vec::new(),
          mTotalRuns: 0,
          mCurrentPageIndex: 0,
          mPapxIndex: 0,
          mCurrentPage: None,
          mMarkSavePageIndex: 0,
          mMarkSavePapxIndex: 0,
      };
      // Java: this.readAll();
      this.readAll();
      this
    }

    // Java: void readAll()
    fn readAll(&mut self) {
      // Java: int nPages = this._binTable.length();
      let n_pages = self.base.base._binTable.as_ref().map_or(0, |bt| bt.length());
      // Java: this.mPages.ensureCapacity(nPages);
      self.mPages.reserve(n_pages as usize);
      // Java: for(int iPage = 0; iPage < nPages; ++iPage)
      for i_page in 0..n_pages
      {
         // Java: GenericPropertyNode node = this._binTable.getProperty(iPage);
         let node = self.base.base._binTable.as_mut().unwrap().getProperty(i_page);
         // Java: int pageNum = LittleEndian.getInt(node.getBytes(), node.getOffset());
         let page_num = le_get_int(node.getBytes(), node.getOffset());
         // Java: int pageOffset = 512 * pageNum;
         let page_offset = 512 * page_num;
         // Java: PageDescriptor pPage = new PageDescriptor();
         let mut p_page = PageDescriptor { mParaCount: 0, mStart: 0, mEnd: 0 };
         // Java: pPage.mParaCount = this.page_read_para(this._documentStream, pageOffset);
         p_page.mParaCount = self.page_read_para(page_offset);
         // Java: pPage.mStart = this.page_read_raw_start(this._documentStream, pageOffset, 0);
         p_page.mStart = self.page_read_raw_start(page_offset, 0);
         // Java: int nRawEnd = this.page_read_raw_end(this._documentStream, pageOffset, pPage.mParaCount - 1);
         let n_raw_end = self.page_read_raw_end(page_offset, p_page.mParaCount - 1);
         // Java: pPage.mEnd = this.tpt.lookIndexBackward(nRawEnd);
         p_page.mEnd = n_raw_end;
         // Java: this.mPages.add(pPage);
         self.mPages.push(p_page);
      }
    }

    // Java: int page_read_para(DocumentInputStream documentStream, int pageOffset)
    fn page_read_para(&mut self, page_offset: i32) -> i32 {
      // Java: documentStream.seek((long)(pageOffset + 511));
      if let Some(ref mut ds) = self.base.base._documentStream {
          ds.seek((page_offset + 511) as i64);
          // Java: return documentStream.readUByte();
          return ds.readUByte() as i32;
      }
      0
    }

    // Java: int page_read_raw_start(DocumentInputStream documentStream, int pageOffset, int iPara)
    fn page_read_raw_start(&mut self, page_offset: i32, i_para: i32) -> i32 {
      // Java: documentStream.seek((long)(pageOffset + iPara * 4));
      if let Some(ref mut ds) = self.base.base._documentStream {
          ds.seek((page_offset + i_para * 4) as i64);
          // Java: return documentStream.readInt();
          return ds.readInt();
      }
      0
    }

    // Java: int page_read_raw_end(DocumentInputStream documentStream, int pageOffset, int iPara)
    fn page_read_raw_end(&mut self, page_offset: i32, i_para: i32) -> i32 {
      // Java: documentStream.seek((long)(pageOffset + (iPara + 1) * 4));
      if let Some(ref mut ds) = self.base.base._documentStream {
          ds.seek((page_offset + (i_para + 1) * 4) as i64);
          // Java: return documentStream.readInt();
          return ds.readInt();
      }
      0
    }

    // Java: void mark()
    pub fn mark(&mut self) {
      // Java: this.mMarkSavePageIndex = this.mCurrentPageIndex;
      self.mMarkSavePageIndex = self.mCurrentPageIndex;
      // Java: this.mMarkSavePapxIndex = this.mPapxIndex;
      self.mMarkSavePapxIndex = self.mPapxIndex;
      // Java: this.mCurrentPageIndex = 0;
      self.mCurrentPageIndex = 0;
      // Java: this.mPapxIndex = 0;
      self.mPapxIndex = 0;
      // Java: this.mCurrentPage = null;
      self.mCurrentPage = None;
    }

    // Java: void reset()
    pub fn reset(&mut self) {
      // Java: this.mCurrentPageIndex = this.mMarkSavePageIndex;
      self.mCurrentPageIndex = self.mMarkSavePageIndex;
      // Java: this.mPapxIndex = this.mMarkSavePapxIndex;
      self.mPapxIndex = self.mMarkSavePapxIndex;
      // Java: this.mCurrentPage = null;
      self.mCurrentPage = None;
    }

    // Java: void seek(int bytesOffset)
    pub fn seek(&mut self, bytes_offset: i32) {
      // Java: if (this.mCurrentPage == null)
      if self.mCurrentPage.is_none()
      {
         // Java: this.load_page(bytesOffset);
         self.load_page(bytes_offset);
      }
      // Java: this.mPapxIndex = this.mCurrentPage.search(bytesOffset);
      self.mPapxIndex = self.mCurrentPage.as_ref().map_or(-1, |p| p.search(bytes_offset));
      // Java: if (this.mPapxIndex == -1)
      if self.mPapxIndex == -1
      {
         // Java: this.load_page(bytesOffset);
         self.load_page(bytes_offset);
         // Java: this.mPapxIndex = this.mCurrentPage.search(bytesOffset);
         self.mPapxIndex = self.mCurrentPage.as_ref().map_or(-1, |p| p.search(bytes_offset));
      }
    }

    // Java: void load_page(int bytesOffset)
    fn load_page(&mut self, bytes_offset: i32) {
      // Java: this.mCurrentPageIndex = binarySearch_page(this.mPages, bytesOffset);
      self.mCurrentPageIndex = Self::binarySearch_page(&self.mPages, bytes_offset);
      // Java: this.load_current_page();
      self.load_current_page();
    }

    // Java: static int binarySearch_page(ArrayList<PageDescriptor> pages, int bytesOffset)
    fn binarySearch_page(pages: &Vec<PageDescriptor>, bytes_offset: i32) -> i32 {
      // Java: int low = 0;
      let mut low: i32 = 0;
      // Java: int mid = 0;
      let mut mid: i32 = 0;
      // Java: PageDescriptor midVal = null;
      // Java: int high = pages.size() - 1;
      let mut high = pages.len() as i32 - 1;
      // Java: while(low <= high)
      while low <= high
      {
         // Java: mid = (low + high) / 2;
         mid = (low + high) / 2;
         // Java: midVal = (PageDescriptor)pages.get(mid);
         let mid_val = &pages[mid as usize];
         // Java: if (midVal.mEnd <= bytesOffset)
         if mid_val.mEnd <= bytes_offset
         {
            // Java: low = mid + 1;
            low = mid + 1;
         }
         // Java: else
         else
         {
            // Java: if (midVal.mStart <= bytesOffset)
            if mid_val.mStart <= bytes_offset
            {
               // Java: return mid;
               return mid;
            }
            // Java: high = mid - 1;
            high = mid - 1;
         }
      }
      // Java: return high;
      high
    }

    // Java: void load_current_page()
    fn load_current_page(&mut self) {
      // Java: GenericPropertyNode node = this._binTable.getProperty(this.mCurrentPageIndex);
      let node = self.base.base._binTable.as_mut().unwrap().getProperty(self.mCurrentPageIndex);
      // Java: int pageNum = LittleEndian.getInt(node.getBytes(), node.getOffset());
      let page_num = le_get_int(node.getBytes(), node.getOffset());
      // Java: int pageOffset = 512 * pageNum;
      let page_offset = 512 * page_num;
      // Java: this.mCurrentPage = new PAPFormattedDiskPage(this._documentStream, this._dataStream, pageOffset, this._fcMin, this.tpt);
      let ds = unsafe { &mut *(self.base.base._documentStream.as_mut().unwrap() as *mut DocumentInputStream) };
      let data_s = unsafe { &mut *(self.base.base._dataStream.as_mut().unwrap() as *mut DocumentInputStream) };
      let tpt = self.base.base.tpt.as_ref().unwrap();
      self.mCurrentPage = Some(PAPFormattedDiskPage::new_from_stream(ds, data_s, page_offset, self.base.base._fcMin, tpt.as_ref()));
    }

    // Java: PAPX getNextParagraph()
    pub fn getNextParagraph(&mut self) -> Option<PAPX> {
      // Java: return this.mPapxIndex == -1 ? null : this.mCurrentPage.getPAPX(this.mPapxIndex);
      if self.mPapxIndex == -1 {
          None
      } else {
          self.mCurrentPage.as_mut().and_then(|p| p.getPAPX(self.mPapxIndex))
      }
    }

}

// Java: public class PageDescriptor
impl PageDescriptor {
    pub fn new() -> Self {
        PageDescriptor { mParaCount: 0, mStart: 0, mEnd: 0 }
    }
}
