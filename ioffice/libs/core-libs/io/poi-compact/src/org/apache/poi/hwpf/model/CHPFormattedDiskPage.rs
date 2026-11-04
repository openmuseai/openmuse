// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use super::CHPX::CHPX;
use super::TextPieceTable::TextPieceTable;

fn le_get_int(data: &[u8], offset: usize) -> i32 {
    i32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
}

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    let bytes = value.to_le_bytes();
    data[offset] = bytes[0];
    data[offset + 1] = bytes[1];
    data[offset + 2] = bytes[2];
    data[offset + 3] = bytes[3];
}

// Java field: int FC_SIZE
const FC_SIZE: i32 = 4;

// Java: public class CHPFormattedDiskPage extends FormattedDiskPage
pub struct CHPFormattedDiskPage {
    // Java field: ArrayList<CHPX> _chpxList
    pub _chpxList: Vec<CHPX>,
    // Java field: ArrayList<CHPX> _overFlow
    pub _overFlow: Option<Vec<CHPX>>,
    // Java field: int _chpxSize
    _chpxSize: i32,
    // Java field: byte[] _s_normalGrpprl
    pub _count: i32,
}

// Java: public class CHPFormattedDiskPage extends FormattedDiskPage
impl CHPFormattedDiskPage {
    // Java: CHPFormattedDiskPage()
    pub fn new() -> Self {
        // Java: super();
        // Java: this._chpxList = new ArrayList();
        CHPFormattedDiskPage { _chpxList: Vec::new(), _overFlow: None, _chpxSize: 0, _count: 0 }
    }

    // Java: CHPFormattedDiskPage(DocumentInputStream documentStream, int offset, int fcMin, TextPieceTable tpt, boolean fComplex)
    pub fn new_from_stream(document_stream: &mut DocumentInputStream, offset: i32, _fc_min: i32, tpt: &TextPieceTable, f_complex: bool) -> Self {
        // Java: super();
        // Java: PageBuffer page = PageBuffer.Pool.InstanceBig.obtain();
        // Java: try
        // Java: documentStream.seek((long)offset);
        document_stream.seek(offset as i64);
        // Java: byte[] buf = page.getByteArray();
        let mut buf = vec![0u8; 512];
        // Java: documentStream.read(buf);
        document_stream.read(&mut buf);
        // Java: this._count = buf[511] & 255;
        let _count = (buf[511] & 0xFF) as i32;
        // Java: this._chpxList = new ArrayList(this._count);
        let mut _chpxList: Vec<CHPX> = Vec::with_capacity(_count as usize);
        // Java: int szFC = (this._count + 1) * 4;
        let sz_fc = (_count + 1) * 4;
        // Java: int endAt = LittleEndian.getInt(buf, 0);
        let mut end_at = le_get_int(&buf, 0);
        // Java: int offFC = 4;
        let mut off_fc: usize = 4;
        // Java: for(int i = 0; i < this._count; ++i)
        for i in 0.._count {
            // Java: int startAt = endAt;
            let start_at = end_at;
            // Java: endAt = LittleEndian.getInt(buf, offFC);
            end_at = le_get_int(&buf, off_fc);
            // Java: offFC += 4;
            off_fc += 4;
            // Java: if (fComplex)
            if f_complex {
                // Java: int offCHPX = (buf[szFC + i] & 255) << 1;
                let off_chpx = ((buf[(sz_fc + i) as usize] as u32 & 255) << 1) as usize;
                // Java: this._chpxList.add(new CHPX(startAt, endAt, tpt, this.getGrpprl(buf, offCHPX)));
                let grpprl = Self::get_grpprl_from_buf(&mut buf, off_chpx, &_chpxList);
                _chpxList.push(CHPX::new_with_grpprl(start_at, end_at, grpprl));
            }
            // Java: else if (!tpt.isIndexInTable(startAt) && !tpt.isIndexInTable(endAt))
            else if !tpt.isIndexInTable(start_at) && !tpt.isIndexInTable(end_at) {
                // Java: if (!tpt.isIndexInTable(startAt) && !tpt.isIndexInTable(endAt))
                // Java: int startAtRight = tpt.getLastRightIndexInTable(startAt);
                let start_at_right = tpt.getLastRightIndexInTable(start_at);
                // Java: int endAtLeft = tpt.getLastLeftIndexInTable(endAt);
                let end_at_left = tpt.getLastLeftIndexInTable(end_at);
                // Java: if (-1 != startAtRight && -1 != endAtLeft && startAtRight < endAtLeft)
                if -1 != start_at_right && -1 != end_at_left && start_at_right < end_at_left {
                    // Java: int offCHPX = (buf[szFC + i] & 255) << 1;
                    let off_chpx = ((buf[(sz_fc + i) as usize] as u32 & 255) << 1) as usize;
                    // Java: this._chpxList.add(new CHPX(startAtRight, endAtLeft, tpt, this.getGrpprl(buf, offCHPX)));
                    let grpprl = Self::get_grpprl_from_buf(&mut buf, off_chpx, &_chpxList);
                    _chpxList.push(CHPX::new_with_grpprl(start_at_right, end_at_left, grpprl));
                }
            }
            // Java: else
            else {
                // Java: int offCHPX = (buf[szFC + i] & 255) << 1;
                let off_chpx = ((buf[(sz_fc + i) as usize] as u32 & 255) << 1) as usize;
                // Java: this._chpxList.add(new CHPX(startAt, endAt, tpt, this.getGrpprl(buf, offCHPX)));
                let grpprl = Self::get_grpprl_from_buf(&mut buf, off_chpx, &_chpxList);
                _chpxList.push(CHPX::new_with_grpprl(start_at, end_at, grpprl));
            }
        }
        // Java: this._chpxSize = this._chpxList.size();
        let _chpxSize = _chpxList.len() as i32;
        // Java: finally
        // Java: PageBuffer.Pool.InstanceBig.recycle(page);
        CHPFormattedDiskPage { _chpxList, _overFlow: None, _chpxSize, _count }
    }

    fn get_grpprl_from_buf(buf: &mut [u8], chpx_offset: usize, chpx_list: &Vec<CHPX>) -> Vec<u8> {
        if chpx_offset == 0 {
            return Vec::new();
        }
        let size = (buf[chpx_offset] & 0xFF) as usize;
        if size != 255 {
            let mut chpx = vec![0u8; size];
            chpx.copy_from_slice(&buf[chpx_offset + 1..chpx_offset + 1 + size]);
            buf[chpx_offset] = 0xFF;
            buf[chpx_offset + 1] = chpx_list.len() as u8;
            chpx
        } else {
            let idx = (buf[chpx_offset + 1] & 0xFF) as usize;
            if idx < chpx_list.len() {
                chpx_list[idx].getGrpprl().to_vec()
            } else {
                Vec::new()
            }
        }
    }

    // Java: CHPX getCHPX(int index)
    pub fn getCHPX(&self, index: i32) -> Option<&CHPX> {
        // Java: return index >= this._chpxSize ? null : (CHPX)this._chpxList.get(index);
        if index >= self._chpxSize { None } else { Some(&self._chpxList[index as usize]) }
    }

    // Java: ArrayList<CHPX> getAll()
    pub fn getAll(&self) -> &Vec<CHPX> {
        // Java: return this._chpxList;
        &self._chpxList
    }

    // Java: void fill(List<CHPX> filler)
    pub fn fill(&mut self, filler: Vec<CHPX>) {
        // Java: this._chpxList.addAll(filler);
        self._chpxList.extend(filler);
    }

    // Java: ArrayList<CHPX> getOverflow()
    pub fn getOverflow(&self) -> Option<&Vec<CHPX>> {
        // Java: return this._overFlow;
        self._overFlow.as_ref()
    }

    // Java: byte[] getGrpprl(byte[] buf, int chpxOffset)
    pub fn getGrpprl_buf(&mut self, buf: &mut [u8], chpx_offset: usize) -> Vec<u8> {
        // Java: if (chpxOffset == 0)
        if chpx_offset == 0 {
            // Java: return _s_normalGrpprl;
            return Vec::new();
        }
        // Java: else
        // Java: int size = buf[chpxOffset] & 255;
        let size = (buf[chpx_offset] & 0xFF) as usize;
        // Java: if (size != 255)
        if size != 255 {
            // Java: byte[] chpx = new byte[size];
            let mut chpx = vec![0u8; size];
            // Java: System.arraycopy(buf, chpxOffset + 1, chpx, 0, size);
            chpx.copy_from_slice(&buf[chpx_offset + 1..chpx_offset + 1 + size]);
            // Java: buf[chpxOffset] = -1;
            buf[chpx_offset] = 0xFF;
            // Java: buf[chpxOffset + 1] = (byte)this._chpxList.size();
            buf[chpx_offset + 1] = self._chpxList.len() as u8;
            // Java: return chpx;
            chpx
        }
        // Java: else
        else {
            // Java: return ((CHPX)this._chpxList.get(buf[chpxOffset + 1] & 255)).getGrpprl();
            let idx = (buf[chpx_offset + 1] & 0xFF) as usize;
            self._chpxList[idx].getGrpprl().to_vec()
        }
    }

    // Java: byte[] getGrpprl(int chpxOffset)
    pub fn getGrpprl_offset(&self, _chpx_offset: i32) -> Vec<u8> {
        // Java: throw new IOException("");
        panic!("not supported");
    }

    // Java: byte[] toByteArray(int fcMin)
    pub fn toByteArray(&mut self, fc_min: i32) -> Vec<u8> {
        // Java: byte[] buf = new byte[512];
        let mut buf = vec![0u8; 512];
        // Java: int size = this._chpxList.size();
        let size = self._chpxList.len();
        // Java: int grpprlOffset = 511;
        let mut grpprl_offset: i32 = 511;
        // Java: int offsetOffset = 0;
        let mut offset_offset: usize;
        // Java: int fcOffset = 0;
        let mut fc_offset: usize = 0;
        // Java: int totalSize = 6;
        let mut total_size: i32 = 6;
        // Java: int index;
        let mut index: usize = 0;
        // Java: for(index = 0; index < size; ++index)
        while index < size {
            // Java: int grpprlLength = ((CHPX)this._chpxList.get(index)).getGrpprl().length;
            let grpprl_length = self._chpxList[index].getGrpprl().len() as i32;
            // Java: totalSize += 6 + grpprlLength;
            total_size += 6 + grpprl_length;
            // Java: if (totalSize > 511 + index % 2)
            if total_size > 511 + (index as i32) % 2 {
                // Java: int var10000 = totalSize - (6 + grpprlLength);
                let _var10000 = total_size - (6 + grpprl_length);
                // Java: break;
                break;
            }
            // Java: if ((1 + grpprlLength) % 2 > 0)
            if (1 + grpprl_length) % 2 > 0 {
                // Java: ++totalSize;
                total_size += 1;
            }
            index += 1;
        }
        // Java: if (index != size)
        if index != size {
            // Java: this._overFlow = new ArrayList();
            // Java: this._overFlow.addAll(this._chpxList.subList(index, size));
            self._overFlow = Some(self._chpxList.split_off(index));
        }
        // Java: buf[511] = (byte)index;
        buf[511] = index as u8;
        // Java: offsetOffset = 4 * index + 4;
        offset_offset = 4 * index + 4;
        // Java: CHPX chpx = null;
        // Java: for(int x = 0; x < index; ++x)
        for x in 0..index {
            // Java: chpx = (CHPX)this._chpxList.get(x);
            // Java: byte[] grpprl = chpx.getGrpprl();
            let grpprl_vec = self._chpxList[x].getGrpprl().to_vec();
            // Java: LittleEndian.putInt(buf, fcOffset, chpx.getStart() * 2 + fcMin);
            let start_val = self._chpxList[x].getStart() * 2 + fc_min;
            le_put_int(&mut buf, fc_offset, start_val);
            // Java: int var12 = grpprlOffset - (1 + grpprl.length);
            let var12 = grpprl_offset - (1 + grpprl_vec.len() as i32);
            // Java: grpprlOffset = var12 - var12 % 2;
            grpprl_offset = var12 - var12 % 2;
            // Java: buf[offsetOffset] = (byte)(grpprlOffset / 2);
            buf[offset_offset] = (grpprl_offset / 2) as u8;
            // Java: buf[grpprlOffset] = (byte)grpprl.length;
            buf[grpprl_offset as usize] = grpprl_vec.len() as u8;
            // Java: System.arraycopy(grpprl, 0, buf, grpprlOffset + 1, grpprl.length);
            buf[grpprl_offset as usize + 1..grpprl_offset as usize + 1 + grpprl_vec.len()].copy_from_slice(&grpprl_vec);
            // Java: ++offsetOffset;
            offset_offset += 1;
            // Java: fcOffset += 4;
            fc_offset += 4;
        }
        // Java: LittleEndian.putInt(buf, fcOffset, chpx.getEnd() * 2 + fcMin);
        if index > 0 {
            let end_val = self._chpxList[index - 1].getEnd() * 2 + fc_min;
            le_put_int(&mut buf, fc_offset, end_val);
        }
        // Java: return buf;
        buf
    }
}
