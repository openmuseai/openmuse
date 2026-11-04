// Java: import cn.wps.moffice.util.LittleEndian

// Java field: int name_offset
pub(crate) const NAME_OFFSET: i32 = 0;
// Java field: int nameSize_offset
pub(crate) const NAME_SIZE_OFFSET: i32 = 64;
// Java field: int type_offset
pub(crate) const TYPE_OFFSET: i32 = 66;
// Java field: int color_offset
pub(crate) const COLOR_OFFSET: i32 = 67;
// Java field: int left_offset
pub(crate) const LEFT_OFFSET: i32 = 68;
// Java field: int right_offset
pub(crate) const RIGHT_OFFSET: i32 = 72;
// Java field: int children_offset
pub(crate) const CHILDREN_OFFSET: i32 = 76;
// Java field: int identifier_offset
pub(crate) const IDENTIFIER_OFFSET: i32 = 80;
// Java field: int flags_offset
pub(crate) const FLAGS_OFFSET: i32 = 96;
// Java field: int ctime_offset
pub(crate) const CTIME_OFFSET: i32 = 100;
// Java field: int mtime_offset
pub(crate) const MTIME_OFFSET: i32 = 108;
// Java field: int firstSector_offset
pub(crate) const FIRST_SECTOR_OFFSET: i32 = 116;
// Java field: int size_offset
pub(crate) const SIZE_OFFSET: i32 = 120;

// Java: public class FileOLEEntry
pub struct FileOLEEntry {
}

// Java: public class FileOLEEntry
impl FileOLEEntry {
    // Java: static int getNameLength(byte[] buf, int start)
    pub fn getNameLength(_buf: &[u8], _start: i32) -> i32 {
      // Java: int nchar = LittleEndian.getUShort(buf, start + 64);

      // Java: nchar = (nchar >> 1) - 1;

      // Java: return nchar;

      todo!()
    }

    // Java: static char[] getName(byte[] buf, int start)
    pub fn getName(_buf: &[u8], _start: i32) -> Vec<char> {
      // Java: int nchar = LittleEndian.getUShort(buf, start + 64);

      // Java: nchar = (nchar >> 1) - 1;

      // Java: char[] sz = new char[nchar];

      // Java: for(int i = 0; i < nchar; ++i)

      {
         // Java: sz[i] = (char)LittleEndian.getShort(buf, start + 0 + (i << 1));

      }
      // Java: return sz;

      todo!()
    }

    // Java: static void getName(byte[] buf, int start, char[] sz, int index)
    pub fn getName_with_index(_buf: &[u8], _start: i32, _sz: &mut [char], _index: i32) {
      // Java: int nchar = LittleEndian.getUShort(buf, start + 64);

      // Java: nchar = (nchar >> 1) - 1;

      // Java: for(int i = 0; i < nchar; ++i)

      {
         // Java: sz[index + i] = (char)LittleEndian.getShort(buf, start + 0 + (i << 1));

      }
    }

    // Java: static void getName(byte[] buf, int start, char[] sz)
    pub fn getName_with_buf(_buf: &[u8], _start: i32, _sz: &mut [char]) {
      // Java: getName(buf, start, sz, 0);

    }

    // Java: static void setName(byte[] buf, int start, String name)
    pub fn setName(_buf: &mut [u8], _start: i32, _name: &str) {
      // Java: int nchar = name.length();

      // Java: int nsize = nchar + 1 << 1;

      // Java: LittleEndian.putUShort(buf, start + 64, nsize);

      // Java: for(int i = 0; i < nchar; ++i)

      {
         // Java: LittleEndian.putShort(buf, start + 0 + (i << 1), (short)name.charAt(i));

      }
      // Java: LittleEndian.putShort(buf, start + 0 + (nchar << 1), (short)0);

    }

    // Java: static int getType(byte[] buf, int start)
    pub fn getType(_buf: &[u8], _start: i32) -> i32 {
      // Java: return LittleEndian.getByte(buf, start + 66);

      todo!()
    }

    // Java: static void setType(byte[] buf, int start, int type)
    pub fn setType(_buf: &mut [u8], _start: i32, _type: i32) {
      // Java: LittleEndian.putByte(buf, start + 66, type);

    }

    // Java: static int getColor(byte[] buf, int start)
    pub fn getColor(_buf: &[u8], _start: i32) -> i32 {
      // Java: return LittleEndian.getByte(buf, start + 67);

      todo!()
    }

    // Java: static void setColor(byte[] buf, int start, int color)
    pub fn setColor(_buf: &mut [u8], _start: i32, _color: i32) {
      // Java: LittleEndian.putByte(buf, start + 67, color);

    }

    // Java: static int getLeft(byte[] buf, int start)
    pub fn getLeft(_buf: &[u8], _start: i32) -> i32 {
      // Java: return LittleEndian.getInt(buf, start + 68);

      todo!()
    }

    // Java: static void setLeft(byte[] buf, int start, int left)
    pub fn setLeft(_buf: &mut [u8], _start: i32, _left: i32) {
      // Java: LittleEndian.putInt(buf, start + 68, left);

    }

    // Java: static int getRight(byte[] buf, int start)
    pub fn getRight(_buf: &[u8], _start: i32) -> i32 {
      // Java: return LittleEndian.getInt(buf, start + 72);

      todo!()
    }

    // Java: static void setRight(byte[] buf, int start, int right)
    pub fn setRight(_buf: &mut [u8], _start: i32, _right: i32) {
      // Java: LittleEndian.putInt(buf, start + 72, right);

    }

    // Java: static int getChildren(byte[] buf, int start)
    pub fn getChildren(_buf: &[u8], _start: i32) -> i32 {
      // Java: return LittleEndian.getInt(buf, start + 76);

      todo!()
    }

    // Java: static void setChildren(byte[] buf, int start, int children)
    pub fn setChildren(_buf: &mut [u8], _start: i32, _children: i32) {
      // Java: LittleEndian.putInt(buf, start + 76, children);

    }

    // Java: static int getFirstSector(byte[] buf, int start)
    pub fn getFirstSector(_buf: &[u8], _start: i32) -> i32 {
      // Java: return LittleEndian.getInt(buf, start + 116);

      todo!()
    }

    // Java: static void setFirstSector(byte[] buf, int start, int value)
    pub fn setFirstSector(_buf: &mut [u8], _start: i32, _value: i32) {
      // Java: LittleEndian.putInt(buf, start + 116, value);

    }

    // Java: static int getSize(byte[] buf, int start)
    pub fn getSize(_buf: &[u8], _start: i32) -> i32 {
      // Java: return LittleEndian.getInt(buf, start + 120);

      todo!()
    }

    // Java: static void setSize(byte[] buf, int start, int value)
    pub fn setSize(_buf: &mut [u8], _start: i32, _value: i32) {
      // Java: LittleEndian.putInt(buf, start + 120, value);

    }

    // Java: static byte[] getClsid(byte[] buf, int start)
    pub fn getClsid(_buf: &[u8], _start: i32) -> Vec<u8> {
      // Java: byte[] clsid = new byte[16];

      // Java: System.arraycopy(buf, start + 80, clsid, 0, 16);

      // Java: return clsid;

      todo!()
    }

    // Java: static void setClsid(byte[] buf, int start, byte[] value)
    pub fn setClsid(_buf: &mut [u8], _start: i32, _value: &[u8]) {
      // Java: System.arraycopy(value, 0, buf, start + 80, value.length);

    }

}
