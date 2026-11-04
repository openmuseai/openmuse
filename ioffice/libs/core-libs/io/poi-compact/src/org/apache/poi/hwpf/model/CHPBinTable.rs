// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.util.ArrayList
// Java: import java.util.List
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
use super::CHPX::CHPX;
use super::TextPieceTable::TextPieceTable;
use super::CharIndexTranslator::CharIndexTranslator;

fn le_get_int(buf: &[u8], offset: usize) -> i32 {
    (buf[offset] as i32 & 0xFF) | ((buf[offset+1] as i32 & 0xFF) << 8) |
    ((buf[offset+2] as i32 & 0xFF) << 16) | ((buf[offset+3] as i32 & 0xFF) << 24)
}

fn le_put_int_buf(buf: &mut [u8], value: i32) {
    buf[0] = (value & 0xFF) as u8;
    buf[1] = ((value >> 8) & 0xFF) as u8;
    buf[2] = ((value >> 16) & 0xFF) as u8;
    buf[3] = ((value >> 24) & 0xFF) as u8;
}

// Java: public class CHPBinTable
pub struct CHPBinTable {
    // Java field: ArrayList<CHPX> _textRuns
    pub(crate) _textRuns: Vec<CHPX>,
    // Java field: TextPieceTable tpt
    pub(crate) tpt: Option<Box<TextPieceTable>>,
    // Java field: DocumentInputStream _documentStream
    _documentStream: Option<DocumentInputStream>,
    // Java field: PlexOfCps _binTable
    _binTable: Option<PlexOfCps>,
    // Java field: int _curPlexOfCpsIndex
    _curPlexOfCpsIndex: i32,
    // Java field: int _curChpxIndex
    _curChpxIndex: i32,
    // Java field: int _fcMin
    _fcMin: i32,
    // Java field: int _tempPlexOfCpsIndex
    _tempPlexOfCpsIndex: i32,
    // Java field: int _tempChpxIndex
    _tempChpxIndex: i32,
    // Java field: boolean fComplex
    fComplex: bool,
}

// Java: public class CHPBinTable
impl CHPBinTable {
    // Java: CHPBinTable()
    pub fn new_empty() -> Self {
      // Java: super();
      CHPBinTable {
          _textRuns: Vec::new(),
          tpt: None,
          _documentStream: None,
          _binTable: None,
          _curPlexOfCpsIndex: 0,
          _curChpxIndex: 0,
          _fcMin: 0,
          _tempPlexOfCpsIndex: 0,
          _tempChpxIndex: 0,
          fComplex: false,
      }
    }

    // Java: CHPBinTable(DocumentInputStream documentStream, DocumentInputStream tableStream, int offset, int size, int fcMin, TextPieceTable tpt, boolean fComplex)
    pub fn new(document_stream: DocumentInputStream, table_stream: DocumentInputStream, offset: i32, size: i32, fc_min: i32, tpt: TextPieceTable, f_complex: bool) -> Self {
      // Java: super();
      // Java: this._binTable = new PlexOfCps(tableStream, offset, size, 4);
      let mut bin_table = PlexOfCps::new_from_stream(table_stream, offset, size, 4);
      // Java: this.tpt = tpt;
      // Java: this._fcMin = fcMin;
      // Java: this.fComplex = fComplex;
      let mut text_runs = Vec::new();
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
            // Java: CHPFormattedDiskPage cfkp = new CHPFormattedDiskPage(documentStream, pageOffset, fcMin, tpt, false);
            // Java: int y = 0;
            // Java: for(int fkpSize = cfkp.size(); y < fkpSize; ++y)
            {
               // Java: this.insert(cfkp.getCHPX(y));
            }
         }
      }
      // Java: this._documentStream = documentStream;
      CHPBinTable {
          _textRuns: text_runs,
          tpt: Some(Box::new(tpt)),
          _documentStream: Some(document_stream),
          _binTable: Some(bin_table),
          _curPlexOfCpsIndex: cur_plex_index,
          _curChpxIndex: 0,
          _fcMin: fc_min,
          _tempPlexOfCpsIndex: 0,
          _tempChpxIndex: 0,
          fComplex: f_complex,
      }
    }

    // Java: void adjustForDelete(int listIndex, int offset, int length)
    pub fn adjustForDelete(&mut self, list_index: i32, offset: i32, length: i32) {
      // Java: int size = this._textRuns.size();
      let size = self._textRuns.len() as i32;
      // Java: int endMark = offset + length;
      let end_mark = offset + length;
      // Java: int endIndex = listIndex;
      let mut end_index = list_index;
      // Java: for(CHPX chpx = (CHPX)this._textRuns.get(listIndex); chpx.getEnd() < endMark; chpx = (CHPX)this._textRuns.get(endIndex))
        while self._textRuns[end_index as usize].getEnd() < end_mark
      {
         // Java: ++endIndex;
         end_index += 1;
      }
      // Java: if (listIndex == endIndex)
        if list_index == end_index
      {
         // Java: CHPX var9 = (CHPX)this._textRuns.get(endIndex);
         // Java: var9.setEnd(var9.getEnd() - endMark + offset);
         let end_val = self._textRuns[end_index as usize].getEnd() - end_mark + offset;
         self._textRuns[end_index as usize].setEnd(end_val);
      }
      // Java: else
        else
      {
         // Java: CHPX var10 = (CHPX)this._textRuns.get(listIndex);
         // Java: var10.setEnd(offset);
         self._textRuns[list_index as usize].setEnd(offset);
         // Java: for(int x = listIndex + 1; x < endIndex; ++x)
            for x in (list_index + 1)..end_index
         {
            // Java: var10 = (CHPX)this._textRuns.get(x);
            // Java: var10.setStart(offset);
            self._textRuns[x as usize].setStart(offset);
            // Java: var10.setEnd(offset);
            self._textRuns[x as usize].setEnd(offset);
         }
         // Java: var10 = (CHPX)this._textRuns.get(endIndex);
         // Java: var10.setEnd(var10.getEnd() - endMark + offset);
         let end_val = self._textRuns[end_index as usize].getEnd() - end_mark + offset;
         self._textRuns[end_index as usize].setEnd(end_val);
      }
      // Java: for(int x = endIndex + 1; x < size; ++x)
        for x in (end_index + 1)..size
      {
         // Java: CHPX var13 = (CHPX)this._textRuns.get(x);
         // Java: var13.setStart(var13.getStart() - length);
         let start_val = self._textRuns[x as usize].getStart() - length;
         self._textRuns[x as usize].setStart(start_val);
         // Java: var13.setEnd(var13.getEnd() - length);
         let end_val = self._textRuns[x as usize].getEnd() - length;
         self._textRuns[x as usize].setEnd(end_val);
      }
    }

    // Java: void insert(int listIndex, int cpStart, SprmBuffer buf)
    pub fn insert_at(&mut self, _list_index: i32, _cp_start: i32, _buf: SprmBuffer) {
      // Java: CHPX insertChpx = new CHPX(0, 0, this.tpt, buf);
      // Java: insertChpx.setStart(cpStart);
      // Java: insertChpx.setEnd(cpStart);
      // Java: this.insert(listIndex, insertChpx);
    }

    // Java: void add(int cpStart, int cpEnd, SprmBuffer buf)
    pub fn add(&mut self, _cp_start: i32, _cp_end: i32, _buf: SprmBuffer) {
      // Java: CHPX insertChpx = new CHPX(0, 0, this.tpt, buf);
      // Java: insertChpx.setStart(cpStart);
      // Java: insertChpx.setEnd(cpEnd);
      // Java: this._textRuns.add(insertChpx);
    }

    // Java: void insert(int listIndex, CHPX insertChpx)
    fn insert_index(&mut self, list_index: i32, insert_chpx: CHPX) {
      // Java: if (listIndex == this._textRuns.size())
        if list_index == self._textRuns.len() as i32
      {
         // Java: this._textRuns.add(insertChpx);
         self._textRuns.push(insert_chpx);
      }
      // Java: else
        else
      {
         // Java: int cpStart = insertChpx.getStart();
         // Java: CHPX chpx = (CHPX)this._textRuns.get(listIndex);
         // Java: if (chpx.getStart() < cpStart)
         {
            // Java: CHPX clone = new CHPX(0, 0, this.tpt, chpx.getSprmBuf());
            // Java: clone.setStart(cpStart);
            // Java: clone.setEnd(chpx.getEnd());
            // Java: chpx.setEnd(cpStart);
            // Java: this._textRuns.add(listIndex + 1, insertChpx);
            self._textRuns.insert(list_index as usize, insert_chpx);
            // Java: this._textRuns.add(listIndex + 2, clone);
         }
         // Java: else
         {
            // Java: this._textRuns.add(listIndex, insertChpx);
         }
      }
    }

    // Java: void insert(CHPX chpx)
    fn insert_chpx(&mut self, mut chpx: CHPX) {
      // Java: if (chpx != null && chpx.getStart() <= chpx.getEnd())
        if chpx.getStart() <= chpx.getEnd()
      {
         // Java: int index = this._textRuns.size();
         let index = self._textRuns.len() as i32;
         // Java: this.insert(index, chpx);
         self.insert_index(index, chpx);
      }
    }

    // Java: void append(CHPX chpx)
    pub fn append(&mut self, chpx: CHPX) {
      // Java: this.insert(chpx);
      self.insert_chpx(chpx);
    }

    // Java: void adjustForInsert(int listIndex, int length)
    pub fn adjustForInsert(&mut self, list_index: i32, length: i32) {
      // Java: int size = this._textRuns.size();
      let size = self._textRuns.len() as i32;
      // Java: CHPX chpx = (CHPX)this._textRuns.get(listIndex);
      // Java: chpx.setEnd(chpx.getEnd() + length);
      let end_val = self._textRuns[list_index as usize].getEnd() + length;
      self._textRuns[list_index as usize].setEnd(end_val);
      // Java: for(int x = listIndex + 1; x < size; ++x)
        for x in (list_index + 1)..size
      {
         // Java: chpx = (CHPX)this._textRuns.get(x);
         // Java: chpx.setStart(chpx.getStart() + length);
         let start_val = self._textRuns[x as usize].getStart() + length;
         self._textRuns[x as usize].setStart(start_val);
         // Java: chpx.setEnd(chpx.getEnd() + length);
         let end_val = self._textRuns[x as usize].getEnd() + length;
         self._textRuns[x as usize].setEnd(end_val);
      }
    }

    // Java: List<CHPX> getTextRuns()
    pub fn getTextRuns(&self) -> &Vec<CHPX> {
      // Java: return this._textRuns;
      &self._textRuns
    }

    // Java: CHPX getNextTextRun()
    pub fn getNextTextRun(&mut self) -> Option<&CHPX> {
      // Java: if (this._curChpxIndex < this._textRuns.size())
        if (self._curChpxIndex as usize) < self._textRuns.len()
      {
         // Java: return (CHPX)this._textRuns.get(this._curChpxIndex++);
         let idx = self._curChpxIndex; self._curChpxIndex += 1;
         return Some(&self._textRuns[idx as usize]);
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
         // Java: this._textRuns.clear();
         self._textRuns.clear();
         // Java: this._curChpxIndex = 0;
         self._curChpxIndex = 0;
         // Java: for(int length = this._binTable.length(); this._curPlexOfCpsIndex < length && this._textRuns.isEmpty(); ++this._curPlexOfCpsIndex)
         if let Some(ref mut bin_table) = self._binTable {
            let length = bin_table.length();
            while self._curPlexOfCpsIndex < length && self._textRuns.is_empty()
         {
            // Java: GenericPropertyNode node = this._binTable.getProperty(this._curPlexOfCpsIndex);
            let _node = bin_table.getProperty(self._curPlexOfCpsIndex);
            // Java: int pageOffset = pageOffset(node);
            // Java: CHPFormattedDiskPage cfkp = new CHPFormattedDiskPage(this._documentStream, pageOffset, this._fcMin, this.tpt, false);
            // Java: int fkpSize = cfkp.size();
            // Java: for(int y = 0; y < fkpSize; ++y)
            {
               // Java: this.insert(cfkp.getCHPX(y));
            }
            self._curPlexOfCpsIndex += 1;
         }
         }
         // Java: return !this._textRuns.isEmpty() ? (CHPX)this._textRuns.get(this._curChpxIndex++) : null;
         if !self._textRuns.is_empty() {
             let idx = self._curChpxIndex; self._curChpxIndex += 1;
             Some(&self._textRuns[idx as usize])
         } else {
             None
         }
      }
    }

    // Java: void mark()
    pub fn mark(&mut self) {
      // Java: this._tempPlexOfCpsIndex = this._curPlexOfCpsIndex;
      self._tempPlexOfCpsIndex = self._curPlexOfCpsIndex;
      // Java: this._tempChpxIndex = this._curChpxIndex;
      self._tempChpxIndex = self._curChpxIndex;
      // Java: this._curPlexOfCpsIndex = 0;
      self._curPlexOfCpsIndex = 0;
      // Java: this._curChpxIndex = 0;
      self._curChpxIndex = 0;
    }

    // Java: void reset()
    pub fn reset(&mut self) {
      // Java: this._curPlexOfCpsIndex = this._tempPlexOfCpsIndex;
      self._curPlexOfCpsIndex = self._tempPlexOfCpsIndex;
      // Java: this._curChpxIndex = this._tempChpxIndex;
      self._curChpxIndex = self._tempChpxIndex;
      // Java: if (!this.fComplex)
        if !self.fComplex
      {
         // Java: this._textRuns.clear();
         self._textRuns.clear();
      }
    }

    // Java: void seek(int bytesOffset)
    pub fn seek(&mut self, bytes_offset: i32) {
      // Java: if (!this.fComplex)
        if !self.fComplex
      {
         // Java: int size = this._textRuns.size();
         let size = self._textRuns.len() as i32;
         // Java: if (size > 0 && bytesOffset >= ((CHPX)this._textRuns.get(0)).getStartBytes() && bytesOffset < ((CHPX)this._textRuns.get(size - 1)).getEndBytes())
            if size > 0 && bytes_offset >= self._textRuns[0].getStartBytes() && bytes_offset < self._textRuns[(size - 1) as usize].getEndBytes()
         {
            // Java: this._curChpxIndex = binarySearch(this._textRuns, bytesOffset);
            self._curChpxIndex = Self::binary_search(&self._textRuns, bytes_offset);
            // Java: return;
            return;
         }
         // Java: this._textRuns.clear();
         self._textRuns.clear();
         // Java: int length = this._binTable.length();
         // Java: while(this._curPlexOfCpsIndex < length && this._textRuns.isEmpty())
         {
            // Java: GenericPropertyNode node = this._binTable.getProperty(this._curPlexOfCpsIndex);
            // Java: ++this._curPlexOfCpsIndex;
            // Java: if (bytesOffset >= node.getStart() && bytesOffset < node.getEnd() || this._curPlexOfCpsIndex == 1 && bytesOffset < node.getStart() && bytesOffset < node.getEnd())
            {
               // Java: int pageOffset = pageOffset(node);
               // Java: CHPFormattedDiskPage cfkp = new CHPFormattedDiskPage(this._documentStream, pageOffset, this._fcMin, this.tpt, false);
               // Java: int fkpSize = cfkp.size();
               // Java: for(int y = 0; y < fkpSize; ++y)
               {
                  // Java: CHPX chpx = cfkp.getCHPX(y);
                  // Java: if (chpx != null)
                  {
                     // Java: if (chpx.getStartBytes() <= bytesOffset && chpx.getEndBytes() > bytesOffset)
                     {
                        // Java: this._curChpxIndex = y;
                     }
                     // Java: this.insert(chpx);
                  }
               }
               // Java: break;
            }
         }
      }
      // Java: else
        else
      {
         // Java: this._curChpxIndex = binarySearch(this._textRuns, bytesOffset);
         self._curChpxIndex = Self::binary_search(&self._textRuns, bytes_offset);
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

    // Java: static int binarySearch(ArrayList<CHPX> textRuns, int bytesOffset)
    fn binary_search(text_runs: &[CHPX], bytes_offset: i32) -> i32 {
      // Java: int low = 0;
      let mut low = 0i32;
      // Java: int mid = 0;
      let mut mid = 0i32;
      // Java: CHPX midVal = null;
      // Java: int high = textRuns.size() - 1;
      let mut high = text_runs.len() as i32 - 1;
      // Java: while(low <= high)
        while low <= high
      {
         // Java: mid = (low + high) / 2;
         mid = (low + high) / 2;
         // Java: midVal = (CHPX)textRuns.get(mid);
         let mid_val = &text_runs[mid as usize];
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
      let mut bin_table = PlexOfCps::new(4);
      // Java: int docOffset = docStream.getOffset();
      // Java: int mod = docOffset % 512;
      // Java: if (mod != 0)
      {
         // Java: byte[] padding = new byte[512 - mod];
         // Java: docStream.write(padding);
      }
      // Java: docOffset = docStream.getOffset();
      // Java: int pageNum = docOffset / 512;
      // Java: int endingFc = ((PropertyNode)this._textRuns.get(this._textRuns.size() - 1)).getEnd() * 2;
      // Java: endingFc += fcMin;
      // Java: ArrayList<CHPX> overflow = this._textRuns;
      // Java: do
      {
         // Java: PropertyNode startingProp = (PropertyNode)overflow.get(0);
         // Java: int start = startingProp.getStart() * 2 + fcMin;
         // Java: CHPFormattedDiskPage cfkp = new CHPFormattedDiskPage();
         // Java: cfkp.fill(overflow);
         // Java: byte[] bufFkp = cfkp.toByteArray(fcMin);
         // Java: docStream.write(bufFkp);
         // Java: overflow = cfkp.getOverflow();
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
