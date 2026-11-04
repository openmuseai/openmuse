// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.List
// Java: import org.apache.poi.hwpf.model.io.HWPFFileSystem
use super::io::HWPFFileSystem::HWPFFileSystem;
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use super::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian
use super::PAPBinTable::PAPX;
use super::TextPieceTable::TextPieceTable;
use super::PageBuffer::PageBuffer;

fn le_get_int(buf: &[u8], offset: usize) -> i32 {
    (buf[offset] as i32 & 0xFF) | ((buf[offset+1] as i32 & 0xFF) << 8) |
    ((buf[offset+2] as i32 & 0xFF) << 16) | ((buf[offset+3] as i32 & 0xFF) << 24)
}

fn le_put_int(buf: &mut [u8], offset: usize, value: i32) {
    buf[offset] = value as u8;
    buf[offset + 1] = (value >> 8) as u8;
    buf[offset + 2] = (value >> 16) as u8;
    buf[offset + 3] = (value >> 24) as u8;
}

// Java field: int BX_SIZE
pub(crate) const BX_SIZE: i32 = 13;
// Java field: int FC_SIZE
pub(crate) const FC_SIZE: i32 = 4;
// Java field: byte[] EMPTY_BYTE

// Java: public class PAPFormattedDiskPage extends FormattedDiskPage
pub struct PAPFormattedDiskPage {
    // Java field: ArrayList<PAPX> _papxList
    _papxList: Vec<PAPX>,
    // Java field: ArrayList<PAPX> _overFlow
    _overFlow: Option<Vec<PAPX>>,
    // Java field: DocumentInputStream _dataStream
    _dataStream: *mut DocumentInputStream,
    // Java field: TextPieceTable _textPieceTable
    _textPieceTable: *const TextPieceTable,
    // Java field: PageBuffer _page
    _page: Option<PageBuffer>,
    // Java field: int _minPapxOffset
    _minPapxOffset: i32,
    pub(crate) _count: i32,
}

// Java: public class PAPFormattedDiskPage extends FormattedDiskPage
impl PAPFormattedDiskPage {
    // Java: PAPFormattedDiskPage(DocumentInputStream dataStream)
    pub fn new_data_only(data_stream: &mut DocumentInputStream) -> Self {
      // Java: super();
      // Java: this._dataStream = dataStream;
      // Java: this._textPieceTable = null;
      // Java: this._page = null;
      PAPFormattedDiskPage {
          _papxList: Vec::new(),
          _overFlow: None,
          _dataStream: data_stream as *mut DocumentInputStream,
          _textPieceTable: std::ptr::null(),
          _page: None,
          _minPapxOffset: 0,
          _count: 0,
      }
    }

    // Java: PAPFormattedDiskPage(DocumentInputStream documentStream, DocumentInputStream dataStream, int offset, int fcMin, TextPieceTable tpt)
    pub fn new_from_stream(document_stream: &mut DocumentInputStream, data_stream: &mut DocumentInputStream, offset: i32, _fc_min: i32, tpt: &TextPieceTable) -> Self {
      // Java: super();
      // Java: this._page = PageBuffer.Pool.InstanceBig.obtain();
      let mut page = PageBuffer::obtain(512);
      // Java: documentStream.seek((long)offset);
      document_stream.seek(offset as i64);
      // Java: byte[] buf = this._page.getByteArray();
      // Java: documentStream.read(buf);
      document_stream.read(page.getByteArrayMut());
      // Java: this._count = buf[511] & 255;
      let count = (page.getByteArray()[511] & 0xFF) as i32;
      // Java: this._textPieceTable = tpt;
      // Java: this._dataStream = dataStream;
      // Java: this._minPapxOffset = (this._count + 1) * 4 + this._count * 13;
      let min_papx_offset = (count + 1) * 4 + count * 13;
      PAPFormattedDiskPage {
          _papxList: Vec::new(),
          _overFlow: None,
          _dataStream: data_stream as *mut DocumentInputStream,
          _textPieceTable: tpt as *const TextPieceTable,
          _page: Some(page),
          _minPapxOffset: min_papx_offset,
          _count: count,
      }
    }

    // Java: void recycle()
    pub fn recycle(&mut self) {
      // Java: if (this._page != null)
      if self._page.is_some()
      {
         // Java: PageBuffer.Pool.InstanceBig.recycle(this._page);
         // Java: this._page = null;
         self._page = None;
      }
    }

    // Java: void fill(List<PAPX> filler)
    pub fn fill(&mut self, filler: Vec<PAPX>) {
      // Java: this._papxList.addAll(filler);
      self._papxList.extend(filler);
    }

    // Java: ArrayList<PAPX> getOverflow()
    pub fn getOverflow(&mut self) -> Option<Vec<PAPX>> {
      // Java: return this._overFlow;
      self._overFlow.take()
    }

    // Java: PAPX getPAPX(int index)
    pub fn getPAPX(&mut self, index: i32) -> Option<PAPX> {
      // Java: int offset = index * 4;
      let offset = (index * 4) as usize;
      // Java: int endOffset = offset + 4;
      let end_offset = offset + 4;
      // Java: byte[] buf = this._page.getByteArray();
      let buf = self._page.as_ref()?.getByteArray();
      // Java: return LittleEndian.isLegalRange_Int(buf.length, offset) && LittleEndian.isLegalRange_Int(buf.length, endOffset) ? new PAPX(LittleEndian.getInt(buf, offset), LittleEndian.getInt(buf, endOffset), this._textPieceTable, this.getGrpprl(index), this._dataStream) : null;
      if offset + 4 <= buf.len() && end_offset + 4 <= buf.len() {
          let start = le_get_int(buf, offset);
          let end = le_get_int(buf, end_offset);
          let grpprl = self.getGrpprl(index);
          Some(PAPX { start, end, start_bytes: start, end_bytes: end, grpprl })
      } else {
          None
      }
    }

    // Java: int search(int bytesOffset)
    pub fn search(&self, bytes_offset: i32) -> i32 {
      // Java: int low = 0;
      let mut low: i32 = 0;
      // Java: int mid = 0;
      let mut mid: i32 = 0;
      // Java: int high = this._count - 1;
      let mut high = self._count - 1;
      // Java: byte[] buf = this._page.getByteArray();
      let buf = match self._page.as_ref() { Some(p) => p.getByteArray(), None => return -1 };
      // Java: while(low <= high)
      while low <= high
      {
         // Java: mid = (low + high) / 2;
         mid = (low + high) / 2;
         // Java: int start = LittleEndian.getInt(buf, mid * 4);
         let start = le_get_int(buf, (mid * 4) as usize);
         // Java: int end = LittleEndian.getInt(buf, (mid + 1) * 4);
         let end = le_get_int(buf, ((mid + 1) * 4) as usize);
         // Java: if (end <= bytesOffset)
         if end <= bytes_offset
         {
            // Java: low = mid + 1;
            low = mid + 1;
         }
         // Java: else
         else
         {
            // Java: if (start <= bytesOffset)
            if start <= bytes_offset
            {
               // Java: return mid;
               return mid;
            }
            // Java: high = mid - 1;
            high = mid - 1;
         }
      }
      // Java: return -1;
      -1
    }

    // Java: byte[] getGrpprl(int index)
    pub fn getGrpprl(&self, index: i32) -> Vec<u8> {
      // Java: byte[] buf = this._page.getByteArray();
      let buf = match self._page.as_ref() { Some(p) => p.getByteArray(), None => return Vec::new() };
      // Java: int offset = (this._count + 1) * 4 + index * 13;
      let offset = ((self._count + 1) * 4 + index * 13) as usize;
      // Java: if (offset >= buf.length)
      if offset >= buf.len()
      {
         // Java: return EMPTY_BYTE;
         return Vec::new();
      }
      // Java: else
      else
      {
         // Java: int papxOffset = (buf[offset] & 255) << 1;
         let mut papx_offset = ((buf[offset] & 0xFF) as usize) << 1;
         // Java: if (papxOffset <= 0)
         if papx_offset == 0
         {
            // Java: return EMPTY_BYTE;
            return Vec::new();
         }
         // Java: else
         else
         {
            // Java: int size = (buf[papxOffset++] & 255) << 1;
            let mut size = ((buf[papx_offset] & 0xFF) as usize) << 1;
            papx_offset += 1;
            // Java: if (size == 0)
            if size == 0
            {
               // Java: size = (buf[papxOffset++] & 255) << 1;
               size = ((buf[papx_offset] & 0xFF) as usize) << 1;
               papx_offset += 1;
            }
            // Java: else
            else
            {
               // Java: --size;
               size -= 1;
            }
            // Java: if (buf.length - papxOffset < size)
            if buf.len() - papx_offset < size
            {
               // Java: return EMPTY_BYTE;
               return Vec::new();
            }
            // Java: else
            else
            {
               // Java: byte[] papx = new byte[size];
               // Java: System.arraycopy(buf, papxOffset, papx, 0, size);
               let papx = buf[papx_offset..papx_offset + size].to_vec();
               // Java: return papx;
               return papx;
            }
         }
      }
    }

    // Java: void toByteArray(byte[] buf, int fcMin, HWPFFileSystem fs)
    pub fn toByteArray(&mut self, buf: &mut [u8], fc_min: i32, _fs: &mut HWPFFileSystem) {
      // Java: Arrays.fill(buf, (byte)0);
      buf.fill(0);
      // Java: int size = this._papxList.size();
      let size = self._papxList.len();
      // Java: int grpprlOffset = 0;
      // Java: int bxOffset = 0;
      // Java: int fcOffset = 0;
      // Java: byte[] lastGrpprl = new byte[0];
      let mut last_grpprl: Vec<u8> = Vec::new();
      // Java: int totalSize = 4;
      let mut total_size: i32 = 4;
      // Java: int index;
      let mut index: usize = 0;
      // Java: for(index = 0; index < size; ++index)
      while index < size
      {
         // Java: byte[] grpprl = ((PAPX)this._papxList.get(index)).getGrpprl();
         let grpprl = self._papxList[index].grpprl.clone();
         // Java: int grpprlLength = grpprl.length;
         let mut grpprl_length = grpprl.len() as i32;
         // Java: if (grpprlLength > 488)
         if grpprl_length > 488
         {
            // Java: grpprlLength = 8;
            grpprl_length = 8;
         }
         // Java: int addition = 0;
         let addition;
         // Java: if (!Arrays.equals(grpprl, lastGrpprl))
         if grpprl != last_grpprl
         {
            // Java: addition = 17 + grpprlLength + 1;
            addition = 17 + grpprl_length + 1;
         }
         // Java: else
         else
         {
            // Java: addition = 17;
            addition = 17;
         }
         // Java: totalSize += addition;
         total_size += addition;
         // Java: if (totalSize > 511 + index % 2)
         if total_size > 511 + (index as i32 % 2)
         {
            // Java: int var10000 = totalSize - addition;
            let _var10000 = total_size - addition;
            // Java: break;
            break;
         }
         // Java: if (grpprlLength % 2 > 0)
         if grpprl_length % 2 > 0
         {
            // Java: ++totalSize;
            total_size += 1;
         }
         // Java: else
         else
         {
            // Java: totalSize += 2;
            total_size += 2;
         }
         // Java: lastGrpprl = grpprl;
         last_grpprl = grpprl;
         index += 1;
      }
      // Java: if (index != size)
      if index != size
      {
         // Java: this._overFlow = new ArrayList();
         // Java: this._overFlow.addAll(this._papxList.subList(index, size));
         self._overFlow = Some(self._papxList.split_off(index));
      }
      // Java: buf[511] = (byte)index;
      buf[511] = index as u8;
      // Java: bxOffset = 4 * index + 4;
      let mut bx_offset = 4 * index + 4;
      // Java: grpprlOffset = 511;
      let mut grpprl_offset: i32 = 511;
      // Java: PAPX papx = null;
      // Java: lastGrpprl = new byte[0];
      last_grpprl = Vec::new();
      let mut last_papx_idx: usize = 0;
      // Java: for(int x = 0; x < index; ++x)
      for x in 0..index
      {
         // Java: papx = (PAPX)this._papxList.get(x);
         last_papx_idx = x;
         // Java: byte[] grpprl = papx.getGrpprl();
         let mut grpprl = self._papxList[x].grpprl.clone();
         // Java: if (grpprl.length > 488)
         if grpprl.len() > 488
         {
            // Java: int hugeGrpprlOffset = papx.getHugeGrpprlOffset();
            let mut huge_grpprl_offset: i32 = -1;
            // Java: if (hugeGrpprlOffset == -1)
            if huge_grpprl_offset == -1
            {
               // Java: HWPFOutputStream pDataStream = fs.createStream("Data");
               // Java: int value = grpprl.length - 2;
               let _value = grpprl.len() as i32 - 2;
               // Java: hugeGrpprlOffset = pDataStream.getOffset();
               huge_grpprl_offset = 0;
               // Java: pDataStream.write((byte)(value >>> 0 & 255));
               // Java: pDataStream.write((byte)(value >>> 8 & 255));
               // Java: pDataStream.write(grpprl, 2, grpprl.length - 2);
            }
            // Java: int istd = LittleEndian.getUShort(grpprl, 0);
            let istd = (grpprl[0] as u16) | ((grpprl[1] as u16) << 8);
            // Java: grpprl = new byte[8];
            grpprl = vec![0u8; 8];
            // Java: LittleEndian.putUShort(grpprl, 0, istd);
            grpprl[0] = istd as u8;
            grpprl[1] = (istd >> 8) as u8;
            // Java: LittleEndian.putUShort(grpprl, 2, 26182);
            grpprl[2] = 0x46;
            grpprl[3] = 0x66;
            // Java: LittleEndian.putInt(grpprl, 4, hugeGrpprlOffset);
            le_put_int(&mut grpprl, 4, huge_grpprl_offset);
         }
         // Java: boolean same = Arrays.equals(lastGrpprl, grpprl);
         let same = last_grpprl == grpprl;
         // Java: if (!same)
         if !same
         {
            // Java: grpprlOffset -= grpprl.length + (2 - grpprl.length % 2);
            grpprl_offset -= grpprl.len() as i32 + (2 - grpprl.len() as i32 % 2);
            // Java: grpprlOffset -= grpprlOffset % 2;
            grpprl_offset -= grpprl_offset % 2;
         }
         // Java: LittleEndian.putInt(buf, fcOffset, papx.getStart() * 2 + fcMin);
         let fc_offset = x * 4;
         le_put_int(buf, fc_offset, self._papxList[x].getStart() * 2 + fc_min);
         // Java: buf[bxOffset] = (byte)(grpprlOffset / 2);
         buf[bx_offset] = (grpprl_offset / 2) as u8;
         // Java: if (!same)
         if !same
         {
            // Java: int var29;
            let var29;
            // Java: if (grpprl.length % 2 > 0)
            if grpprl.len() % 2 > 0
            {
               // Java: var29 = grpprlOffset + 1;
               var29 = (grpprl_offset + 1) as usize;
               // Java: buf[grpprlOffset] = (byte)((grpprl.length + 1) / 2);
               buf[grpprl_offset as usize] = ((grpprl.len() + 1) / 2) as u8;
            }
            // Java: else
            else
            {
               // Java: var29 = grpprlOffset + 1;
               var29 = (grpprl_offset + 1) as usize;
               // Java: buf[var29] = (byte)(grpprl.length / 2);
               buf[var29] = (grpprl.len() / 2) as u8;
               // Java: ++var29;
               // var29 += 1; already set
            }
            // Java: System.arraycopy(grpprl, 0, buf, var29, grpprl.length);
            let end = var29 + grpprl.len();
            if end <= buf.len() {
                buf[var29..end].copy_from_slice(&grpprl);
            }
            // Java: lastGrpprl = grpprl;
            last_grpprl = grpprl;
         }
         // Java: bxOffset += 13;
         bx_offset += 13;
         // Java: fcOffset += 4;
      }
      // Java: LittleEndian.putInt(buf, fcOffset, papx.getEnd() * 2 + fcMin);
      if !self._papxList.is_empty() && index > 0 {
          let fc_offset = index * 4;
          le_put_int(buf, fc_offset, self._papxList[last_papx_idx].getEnd() * 2 + fc_min);
      }
    }

}
