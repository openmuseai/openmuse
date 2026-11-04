// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.util.ArrayList
// Java: import org.apache.poi.hwpf.model.io.HWPFFileSystem
use super::io::HWPFFileSystem::HWPFFileSystem;
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use super::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.hwpf.sprm.SprmBuffer
use crate::org::apache::poi::hwpf::sprm::SprmBuffer::SprmBuffer;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian
use super::PlexOfCps::{PlexOfCps, GenericPropertyNode};
use super::TextPieceTable::TextPieceTable;

fn le_get_int(buf: &[u8], offset: usize) -> i32 {
    (buf[offset] as i32 & 0xFF) | ((buf[offset+1] as i32 & 0xFF) << 8) |
    ((buf[offset+2] as i32 & 0xFF) << 16) | ((buf[offset+3] as i32 & 0xFF) << 24)
}

pub struct PAPX {
    pub(crate) start: i32,
    pub(crate) end: i32,
    pub(crate) start_bytes: i32,
    pub(crate) end_bytes: i32,
    pub(crate) grpprl: Vec<u8>,
}

impl PAPX {
    pub fn getStart(&self) -> i32 { self.start }
    pub fn getEnd(&self) -> i32 { self.end }
    pub fn setStart(&mut self, v: i32) { self.start = v; self.start_bytes = v; }
    pub fn setEnd(&mut self, v: i32) { self.end = v; self.end_bytes = v; }
    pub fn getStartBytes(&self) -> i32 { self.start_bytes }
    pub fn getEndBytes(&self) -> i32 { self.end_bytes }
    pub fn getSprmBuf(&self) -> SprmBuffer { SprmBuffer { _buf: self.grpprl.clone() } }
}

// Java: public class PAPBinTable
pub struct PAPBinTable {
    // Java field: ArrayList<PAPX> _paragraphs
    pub(crate) _paragraphs: Vec<PAPX>,
    // Java field: DocumentInputStream _dataStream
    pub(crate) _dataStream: Option<DocumentInputStream>,
    // Java field: DocumentInputStream _documentStream
    pub(crate) _documentStream: Option<DocumentInputStream>,
    // Java field: TextPieceTable tpt
    pub(crate) tpt: Option<Box<TextPieceTable>>,
    // Java field: PlexOfCps _binTable
    pub(crate) _binTable: Option<PlexOfCps>,
    // Java field: int _curPlexOfCpsIndex
    _curPlexOfCpsIndex: i32,
    // Java field: int _curPapxIndex
    _curPapxIndex: i32,
    // Java field: int _fcMin
    pub(crate) _fcMin: i32,
    // Java field: int _tempPlexOfCpsIndex
    _tempPlexOfCpsIndex: i32,
    // Java field: int _tempPapxIndex
    _tempPapxIndex: i32,
    // Java field: boolean fComplex
    pub(crate) fComplex: bool,
}

// Java: public class PAPBinTable
impl PAPBinTable {
    // Java: PAPBinTable()
    pub fn new_empty() -> Self {
      // Java: super();
      PAPBinTable {
          _paragraphs: Vec::new(),
          _dataStream: None,
          _documentStream: None,
          tpt: None,
          _binTable: None,
          _curPlexOfCpsIndex: 0,
          _curPapxIndex: 0,
          _fcMin: 0,
          _tempPlexOfCpsIndex: 0,
          _tempPapxIndex: 0,
          fComplex: false,
      }
    }

    // Java: PAPBinTable(DocumentInputStream documentStream, DocumentInputStream tableStream, DocumentInputStream dataStream, int offset, int size, int fcMin, TextPieceTable tpt, boolean fComplex)
    pub fn new(document_stream: DocumentInputStream, table_stream: DocumentInputStream, data_stream: DocumentInputStream, offset: i32, size: i32, fc_min: i32, tpt: TextPieceTable, f_complex: bool) -> Self {
      // Java: super();
      // Java: this._binTable = new PlexOfCps(tableStream, offset, size, 4);
      let mut bin_table = PlexOfCps::new_from_stream(table_stream, offset, size, 4);
      // Java: this.tpt = tpt;
      // Java: this._fcMin = fcMin;
      // Java: this.fComplex = fComplex;
      let mut paragraphs = Vec::new();
      let mut cur_plex_index = 0i32;
      // Java: if (fComplex)
        if f_complex
      {
         // Java: int length = this._binTable.length();
         let length = bin_table.length();
         // Java: while(this._curPlexOfCpsIndex < length)
            while cur_plex_index < length
         {
            // Java: GenericPropertyNode node = this._binTable.getProperty(this._curPlexOfCpsIndex);
            let node = bin_table.getProperty(cur_plex_index);
            // Java: ++this._curPlexOfCpsIndex;
            cur_plex_index += 1;
            // Java: int pageOffset = pageOffset(node);
            let _page_offset = Self::page_offset_node(&node);
            // Java: PAPFormattedDiskPage pfkp = new PAPFormattedDiskPage(documentStream, dataStream, pageOffset, fcMin, tpt);
            // Java: int y = 0;
            // Java: for(int fkpSize = pfkp.size(); y < fkpSize; ++y)
            {
               // Java: this.insert(pfkp.getPAPX(y));
            }
            // Java: pfkp.recycle();
         }
      }
      // Java: this._dataStream = dataStream;
      // Java: this._documentStream = documentStream;
      PAPBinTable {
          _paragraphs: paragraphs,
          _dataStream: Some(data_stream),
          _documentStream: Some(document_stream),
          tpt: Some(Box::new(tpt)),
          _binTable: Some(bin_table),
          _curPlexOfCpsIndex: cur_plex_index,
          _curPapxIndex: 0,
          _fcMin: fc_min,
          _tempPlexOfCpsIndex: 0,
          _tempPapxIndex: 0,
          fComplex: f_complex,
      }
    }

    // Java: void insert(PAPX papx)
    fn insert_papx(&mut self, mut papx: PAPX) {
      // Java: if (papx != null && papx.getStart() <= papx.getEnd())
        if papx.getStart() <= papx.getEnd()
      {
         // Java: int index = this._paragraphs.size();
         let index = self._paragraphs.len() as i32;
         // Java: this.insert(index, papx);
         self.insert_index(index, papx);
      }
    }

    // Java: void insert(int listIndex, PAPX forInsert)
    fn insert_index(&mut self, list_index: i32, for_insert: PAPX) {
      // Java: if (listIndex == this._paragraphs.size())
        if list_index == self._paragraphs.len() as i32
      {
         // Java: this._paragraphs.add(forInsert);
         self._paragraphs.push(for_insert);
      }
      // Java: else
        else
      {
         // Java: int cpStart = forInsert.getStart();
         // Java: PAPX currentPap = (PAPX)this._paragraphs.get(listIndex);
         // Java: if (currentPap != null && currentPap.getStart() < cpStart)
         {
            // Java: SprmBuffer clonedBuf = null;
            // Java: try
            {
               // Java: clonedBuf = (SprmBuffer)currentPap.getSprmBuf().clone();
            }
            // Java: catch (CloneNotSupportedException exc)
            {
               // Java: exc.printStackTrace();
            }
            // Java: PAPX clone = new PAPX(0, 0, this.tpt, clonedBuf, this._dataStream);
            // Java: clone.setStart(cpStart);
            // Java: clone.setEnd(currentPap.getEnd());
            // Java: currentPap.setEnd(cpStart);
            // Java: this._paragraphs.add(listIndex + 1, forInsert);
            self._paragraphs.insert(list_index as usize, for_insert);
            // Java: this._paragraphs.add(listIndex + 2, clone);
         }
         // Java: else
         {
            // Java: this._paragraphs.add(listIndex, forInsert);
         }
      }
    }

    // Java: void insert(int listIndex, int cpStart, SprmBuffer buf)
    pub fn insert_at(&mut self, _list_index: i32, _cp_start: i32, _buf: SprmBuffer) {
      // Java: PAPX forInsert = new PAPX(0, 0, this.tpt, buf, this._dataStream);
      // Java: forInsert.setStart(cpStart);
      // Java: forInsert.setEnd(cpStart);
      // Java: this.insert(listIndex, forInsert);
    }

    // Java: void add(int cpStart, int cpEnd, SprmBuffer buf)
    pub fn add(&mut self, _cp_start: i32, _cp_end: i32, _buf: SprmBuffer) {
      // Java: PAPX forInsert = new PAPX(0, 0, this.tpt, buf, this._dataStream);
      // Java: forInsert.setStart(cpStart);
      // Java: forInsert.setEnd(cpEnd);
      // Java: this._paragraphs.add(forInsert);
    }

    // Java: void adjustForDelete(int listIndex, int offset, int length)
    pub fn adjustForDelete(&mut self, list_index: i32, offset: i32, length: i32) {
      // Java: int size = this._paragraphs.size();
      let size = self._paragraphs.len() as i32;
      // Java: int endMark = offset + length;
      let end_mark = offset + length;
      // Java: int endIndex = listIndex;
      let mut end_index = list_index;
      // Java: for(PAPX papx = (PAPX)this._paragraphs.get(listIndex); papx.getEnd() < endMark; papx = (PAPX)this._paragraphs.get(endIndex))
        while self._paragraphs[end_index as usize].getEnd() < end_mark
      {
         // Java: ++endIndex;
         end_index += 1;
      }
      // Java: if (listIndex == endIndex)
        if list_index == end_index
      {
         // Java: PAPX var9 = (PAPX)this._paragraphs.get(endIndex);
         // Java: var9.setEnd(var9.getEnd() - endMark + offset);
         let end_val = self._paragraphs[end_index as usize].getEnd() - end_mark + offset;
         self._paragraphs[end_index as usize].setEnd(end_val);
      }
      // Java: else
        else
      {
         // Java: PAPX var10 = (PAPX)this._paragraphs.get(listIndex);
         // Java: var10.setEnd(offset);
         self._paragraphs[list_index as usize].setEnd(offset);
         // Java: for(int x = listIndex + 1; x < endIndex; ++x)
            for x in (list_index + 1)..end_index
         {
            // Java: var10 = (PAPX)this._paragraphs.get(x);
            // Java: var10.setStart(offset);
            self._paragraphs[x as usize].setStart(offset);
            // Java: var10.setEnd(offset);
            self._paragraphs[x as usize].setEnd(offset);
         }
         // Java: var10 = (PAPX)this._paragraphs.get(endIndex);
         // Java: var10.setEnd(var10.getEnd() - endMark + offset);
         let end_val = self._paragraphs[end_index as usize].getEnd() - end_mark + offset;
         self._paragraphs[end_index as usize].setEnd(end_val);
      }
      // Java: for(int x = endIndex + 1; x < size; ++x)
        for x in (end_index + 1)..size
      {
         // Java: PAPX var13 = (PAPX)this._paragraphs.get(x);
         // Java: var13.setStart(var13.getStart() - length);
         let start_val = self._paragraphs[x as usize].getStart() - length;
         self._paragraphs[x as usize].setStart(start_val);
         // Java: var13.setEnd(var13.getEnd() - length);
         let end_val = self._paragraphs[x as usize].getEnd() - length;
         self._paragraphs[x as usize].setEnd(end_val);
      }
    }

    // Java: void adjustForInsert(int listIndex, int length)
    pub fn adjustForInsert(&mut self, list_index: i32, length: i32) {
      // Java: int size = this._paragraphs.size();
      let size = self._paragraphs.len() as i32;
      // Java: PAPX papx = (PAPX)this._paragraphs.get(listIndex);
      // Java: papx.setEnd(papx.getEnd() + length);
      let end_val = self._paragraphs[list_index as usize].getEnd() + length;
      self._paragraphs[list_index as usize].setEnd(end_val);
      // Java: for(int x = listIndex + 1; x < size; ++x)
        for x in (list_index + 1)..size
      {
         // Java: papx = (PAPX)this._paragraphs.get(x);
         // Java: papx.setStart(papx.getStart() + length);
         let start_val = self._paragraphs[x as usize].getStart() + length;
         self._paragraphs[x as usize].setStart(start_val);
         // Java: papx.setEnd(papx.getEnd() + length);
         let end_val = self._paragraphs[x as usize].getEnd() + length;
         self._paragraphs[x as usize].setEnd(end_val);
      }
    }

    // Java: ArrayList<PAPX> getParagraphs()
    pub fn getParagraphs(&self) -> &Vec<PAPX> {
      // Java: return this._paragraphs;
      &self._paragraphs
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this._paragraphs.isEmpty() && this._binTable.length() == this._curPlexOfCpsIndex;
      self._paragraphs.is_empty() && self._binTable.as_ref().map(|b| b.length()).unwrap_or(0) == self._curPlexOfCpsIndex
    }

    // Java: PAPX getNextParagraph()
    pub fn getNextParagraph(&mut self) -> Option<&PAPX> {
      // Java: if (this._curPapxIndex < this._paragraphs.size())
        if (self._curPapxIndex as usize) < self._paragraphs.len()
      {
         // Java: return (PAPX)this._paragraphs.get(this._curPapxIndex++);
         let idx = self._curPapxIndex; self._curPapxIndex += 1;
         return Some(&self._paragraphs[idx as usize]);
      }
      // Java: else if (this.fComplex)
        else if self.fComplex
      {
         // Java: return null;
         return None;
      }
      // Java: else
        else
      {
         // Java: this._paragraphs.clear();
         self._paragraphs.clear();
         // Java: this._curPapxIndex = 0;
         self._curPapxIndex = 0;
         // Java: while(this._curPlexOfCpsIndex < this._binTable.length() && this._paragraphs.isEmpty())
         if let Some(ref mut bin_table) = self._binTable {
            while self._curPlexOfCpsIndex < bin_table.length() && self._paragraphs.is_empty()
         {
            // Java: GenericPropertyNode node = this._binTable.getProperty(this._curPlexOfCpsIndex);
            let _node = bin_table.getProperty(self._curPlexOfCpsIndex);
            // Java: int pageOffset = pageOffset(node);
            // Java: PAPFormattedDiskPage pfkp = new PAPFormattedDiskPage(this._documentStream, this._dataStream, pageOffset, this._fcMin, this.tpt);
            // Java: int fkpSize = pfkp.size();
            // Java: for(int y = 0; y < fkpSize; ++y)
            {
               // Java: this.insert(pfkp.getPAPX(y));
            }
            // Java: ++this._curPlexOfCpsIndex;
            self._curPlexOfCpsIndex += 1;
            // Java: pfkp.recycle();
         }
         }
         // Java: return !this._paragraphs.isEmpty() ? (PAPX)this._paragraphs.get(this._curPapxIndex++) : null;
         if !self._paragraphs.is_empty() {
             let idx = self._curPapxIndex; self._curPapxIndex += 1;
             Some(&self._paragraphs[idx as usize])
         } else {
             None
         }
      }
    }

    // Java: void mark()
    pub fn mark(&mut self) {
      // Java: this._tempPlexOfCpsIndex = this._curPlexOfCpsIndex;
      self._tempPlexOfCpsIndex = self._curPlexOfCpsIndex;
      // Java: this._tempPapxIndex = this._curPapxIndex;
      self._tempPapxIndex = self._curPapxIndex;
      // Java: this._curPlexOfCpsIndex = 0;
      self._curPlexOfCpsIndex = 0;
      // Java: this._curPapxIndex = 0;
      self._curPapxIndex = 0;
    }

    // Java: void reset()
    pub fn reset(&mut self) {
      // Java: this._curPlexOfCpsIndex = this._tempPlexOfCpsIndex;
      self._curPlexOfCpsIndex = self._tempPlexOfCpsIndex;
      // Java: this._curPapxIndex = this._tempPapxIndex;
      self._curPapxIndex = self._tempPapxIndex;
      // Java: if (!this.fComplex)
        if !self.fComplex
      {
         // Java: this._paragraphs.clear();
         self._paragraphs.clear();
      }
    }

    // Java: void seek(int bytesOffset)
    pub fn seek(&mut self, bytes_offset: i32) {
      // Java: if (!this.fComplex)
        if !self.fComplex
      {
         // Java: int size = this._paragraphs.size();
         let size = self._paragraphs.len() as i32;
         // Java: if (size > 0 && bytesOffset >= ((PAPX)this._paragraphs.get(0)).getStartBytes() && bytesOffset < ((PAPX)this._paragraphs.get(size - 1)).getEndBytes())
            if size > 0 && bytes_offset >= self._paragraphs[0].getStartBytes() && bytes_offset < self._paragraphs[(size - 1) as usize].getEndBytes()
         {
            // Java: this._curPapxIndex = binarySearch(this._paragraphs, bytesOffset);
            self._curPapxIndex = Self::binary_search(&self._paragraphs, bytes_offset);
            // Java: return;
            return;
         }
         // Java: this._paragraphs.clear();
         self._paragraphs.clear();
         // Java: int length = this._binTable.length();
         // Java: while(this._curPlexOfCpsIndex < length && this._paragraphs.isEmpty())
         {
            // Java: GenericPropertyNode node = this._binTable.getProperty(this._curPlexOfCpsIndex);
            // Java: ++this._curPlexOfCpsIndex;
            // Java: int startFc = node.getStart();
            // Java: int endFc = node.getEnd();
            // Java: if (bytesOffset >= startFc && bytesOffset < endFc || this._curPlexOfCpsIndex == 1 && bytesOffset < startFc && bytesOffset < endFc)
            {
               // Java: int pageOffset = pageOffset(node);
               // Java: PAPFormattedDiskPage pfkp = new PAPFormattedDiskPage(this._documentStream, this._dataStream, pageOffset, this._fcMin, this.tpt);
               // Java: int y = 0;
               // Java: for(int fkpSize = pfkp.size(); y < fkpSize; ++y)
               {
                  // Java: PAPX papx = pfkp.getPAPX(y);
                  // Java: if (papx != null)
                  {
                     // Java: if (papx.getStartBytes() <= bytesOffset && papx.getEndBytes() > bytesOffset)
                     {
                        // Java: this._curPapxIndex = y;
                     }
                     // Java: this.insert(papx);
                  }
               }
               // Java: pfkp.recycle();
            }
         }
      }
      // Java: else
        else
      {
         // Java: this._curPapxIndex = binarySearch(this._paragraphs, bytesOffset);
         self._curPapxIndex = Self::binary_search(&self._paragraphs, bytes_offset);
      }
    }

    // Java: static int pageOffset(GenericPropertyNode node)
    fn page_offset_node(node: &GenericPropertyNode) -> i32 {
      // Java: return 512 * pageNum(node);
      512 * Self::page_num_node(node)
    }

    // Java: static int pageNum(GenericPropertyNode node)
    fn page_num_node(node: &GenericPropertyNode) -> i32 {
      // Java: return LittleEndian.getInt(node.getBytes(), node.getOffset());
      le_get_int(node.getBytes(), node.getOffset() as usize)
    }

    // Java: static int binarySearch(ArrayList<PAPX> paragraphs, int bytesOffset)
    fn binary_search(paragraphs: &[PAPX], bytes_offset: i32) -> i32 {
      // Java: int low = 0;
      let mut low = 0i32;
      // Java: int mid = 0;
      let mut mid = 0i32;
      // Java: PAPX midVal = null;
      // Java: int high = paragraphs.size() - 1;
      let mut high = paragraphs.len() as i32 - 1;
      // Java: while(low <= high)
        while low <= high
      {
         // Java: mid = (low + high) / 2;
         mid = (low + high) / 2;
         // Java: midVal = (PAPX)paragraphs.get(mid);
         let mid_val = &paragraphs[mid as usize];
         // Java: if (midVal.getEndBytes() < bytesOffset)
            if mid_val.getEndBytes() < bytes_offset
         {
            // Java: low = mid + 1;
            low = mid + 1;
         }
         // Java: else
            else
         {
            // Java: if (midVal.getStartBytes() <= bytesOffset)
                if mid_val.getStartBytes() <= bytes_offset
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

    // Java: void writeTo(HWPFFileSystem sys, int fcMin)
    pub fn writeTo(&mut self, _sys: &mut HWPFFileSystem, _fc_min: i32) {
      // Java: HWPFOutputStream docStream = sys.getStream("WordDocument");
      // Java: OutputStream tableStream = sys.getStream("0Table");
      // Java: PlexOfCps binTable = new PlexOfCps(4);
      let mut _bin_table = PlexOfCps::new(4);
      // Java: int docOffset = docStream.getOffset();
      // Java: int mod = docOffset % 512;
      // Java: if (mod != 0)
      {
         // Java: byte[] padding = new byte[512 - mod];
         // Java: docStream.write(padding);
      }
      // Java: docOffset = docStream.getOffset();
      // Java: int pageNum = docOffset / 512;
      // Java: int endingFc = ((PropertyNode)this._paragraphs.get(this._paragraphs.size() - 1)).getEnd() * 2;
      // Java: endingFc += fcMin;
      // Java: ArrayList<PAPX> overflow = this._paragraphs;
      // Java: byte[] bufFkp = new byte[512];
      // Java: do
      {
         // Java: PropertyNode startingProp = (PropertyNode)overflow.get(0);
         // Java: int start = startingProp.getStart() * 2 + fcMin;
         // Java: PAPFormattedDiskPage pfkp = new PAPFormattedDiskPage(this._dataStream);
         // Java: pfkp.fill(overflow);
         // Java: pfkp.toByteArray(bufFkp, fcMin, sys);
         // Java: docStream.write(bufFkp);
         // Java: overflow = pfkp.getOverflow();
         // Java: int end = endingFc;
         // Java: if (overflow != null)
         {
            // Java: end = ((PropertyNode)overflow.get(0)).getStart() * 2 + fcMin;
         }
         // Java: byte[] intHolder = new byte[4];
         // Java: LittleEndian.putInt(intHolder, pageNum++);
         // Java: binTable.addProperty(new GenericPropertyNode(start, end, intHolder, 0));
      }
      // Java: tableStream.write(binTable.toByteArray());
    }

}
