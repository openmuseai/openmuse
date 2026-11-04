// Java: import cn.wps.moffice.util.UnitUtil
// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import org.apache.poi.util.LittleEndian
use std::io::Write;

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    let bytes = value.to_le_bytes();
    data[offset..offset + 4].copy_from_slice(&bytes);
}

fn le_put_ushort(data: &mut [u8], offset: usize, value: i32) {
    let v = value as u16;
    let bytes = v.to_le_bytes();
    data[offset..offset + 2].copy_from_slice(&bytes);
}

fn emu2pt(emu: i32) -> f32 {
    emu as f32 / 12700.0_f32
}

// Java: public class AldusHeader
pub struct AldusHeader {
    // Java field: int APMHEADER_KEY
    APMHEADER_KEY: i32,
    // Java field: int handle
    handle: i32,
    // Java field: int left
    left: i32,
    // Java field: int top
    top: i32,
    // Java field: int right
    right: i32,
    // Java field: int bottom
    bottom: i32,
    // Java field: int inch
    inch: i32,
    // Java field: int reserved
    reserved: i32,
    // Java field: int checksum
    pub(crate) checksum: i32,
}

// Java: public class AldusHeader
impl AldusHeader {
    // Java: AldusHeader()
    pub fn new() -> Self {
      // Java: super();
      AldusHeader {
          APMHEADER_KEY: -1698247209i32,
          handle: 0,
          // Java: this.left = 0;
          left: 0,
          // Java: this.top = 8;
          top: 8,
          // Java: this.right = 1789;
          right: 1789,
          // Java: this.bottom = 1539;
          bottom: 1539,
          // Java: this.inch = 606;
          inch: 606,
          reserved: 0,
          checksum: 0,
      }
    }

    // Java: AldusHeader(int width, int height)
    pub fn new_with_size(width: i32, height: i32) -> Self {
      // Java: super();
      let mut h = AldusHeader {
          APMHEADER_KEY: -1698247209i32,
          handle: 0,
          left: 0,
          top: 8,
          right: 0,
          bottom: 0,
          // Java: this.inch = 606;
          inch: 606,
          reserved: 0,
          checksum: 0,
      };
      // Java: this.left = 0;
      h.left = 0;
      // Java: this.top = 8;
      h.top = 8;
      // Java: this.right = this.left + this.emu2Logical(width);
      h.right = h.left + h.emu2Logical(width);
      // Java: this.bottom = this.top + this.emu2Logical(height);
      h.bottom = h.top + h.emu2Logical(height);
      h
    }

    // Java: int emu2Logical(int emu)
    pub fn emu2Logical(&self, emu: i32) -> i32 {
      // Java: return (int)(UnitUtil.EMU2PT(emu) * (float)this.inch / 72.0F);
      (emu2pt(emu) * self.inch as f32 / 72.0_f32) as i32
    }

    // Java: int getChecksum()
    pub fn getChecksum(&self) -> i32 {
      // Java: int checksum = 0;
      let mut checksum: i32 = 0;
      // Java: checksum ^= this.APMHEADER_KEY & '\uffff';
      checksum ^= self.APMHEADER_KEY & 0xFFFF;
      // Java: checksum ^= (this.APMHEADER_KEY & -65536) >> 16;
      checksum ^= (self.APMHEADER_KEY & -65536i32) >> 16;
      // Java: checksum ^= this.left;
      checksum ^= self.left;
      // Java: checksum ^= this.top;
      checksum ^= self.top;
      // Java: checksum ^= this.right;
      checksum ^= self.right;
      // Java: checksum ^= this.bottom;
      checksum ^= self.bottom;
      // Java: checksum ^= this.inch;
      checksum ^= self.inch;
      // Java: return checksum;
      checksum
    }

    // Java: byte[] serialize()
    pub fn serialize(&mut self) -> Vec<u8> {
      // Java: byte[] header = new byte[22];
      let mut header = vec![0u8; 22];
      // Java: int pos = 0;
      let mut pos: usize = 0;
      // Java: LittleEndian.putInt(header, pos, this.APMHEADER_KEY);
      le_put_int(&mut header, pos, self.APMHEADER_KEY);
      // Java: pos += 4;
      pos += 4;
      // Java: LittleEndian.putUShort(header, pos, this.handle);
      le_put_ushort(&mut header, pos, self.handle);
      // Java: pos += 2;
      pos += 2;
      // Java: LittleEndian.putUShort(header, pos, this.left);
      le_put_ushort(&mut header, pos, self.left);
      // Java: pos += 2;
      pos += 2;
      // Java: LittleEndian.putUShort(header, pos, this.top);
      le_put_ushort(&mut header, pos, self.top);
      // Java: pos += 2;
      pos += 2;
      // Java: LittleEndian.putUShort(header, pos, this.right);
      le_put_ushort(&mut header, pos, self.right);
      // Java: pos += 2;
      pos += 2;
      // Java: LittleEndian.putUShort(header, pos, this.bottom);
      le_put_ushort(&mut header, pos, self.bottom);
      // Java: pos += 2;
      pos += 2;
      // Java: LittleEndian.putUShort(header, pos, this.inch);
      le_put_ushort(&mut header, pos, self.inch);
      // Java: pos += 2;
      pos += 2;
      // Java: LittleEndian.putInt(header, pos, this.reserved);
      le_put_int(&mut header, pos, self.reserved);
      // Java: pos += 4;
      pos += 4;
      // Java: this.checksum = this.getChecksum();
      self.checksum = self.getChecksum();
      // Java: LittleEndian.putUShort(header, pos, this.checksum);
      le_put_ushort(&mut header, pos, self.checksum);
      // Java: return header;
      header
    }

    // Java: void write(OutputStream out)
    pub fn write(&mut self, out: &mut dyn Write) -> std::io::Result<()> {
      // Java: byte[] header = this.serialize();
      let header = self.serialize();
      // Java: out.write(header);
      out.write_all(&header)?;
      Ok(())
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return 22;
      22
    }

}
