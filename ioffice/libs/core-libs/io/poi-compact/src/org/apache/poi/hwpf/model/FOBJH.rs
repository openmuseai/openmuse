// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import org.apache.poi.util.BitField
use util::poi_util::BitField::*;
// Java: import org.apache.poi.util.BitFieldFactory
use util::poi_util::BitFieldFactory::*;
// Java: import org.apache.poi.util.LittleEndian
use util::poi_util::LittleEndian::*;

// java public static final int SIZE = 8;
pub(crate) const SIZE: i32 = 8;
// java private static final BitField _fCompressed = BitFieldFactory.getInstance(1);
static _fCompressed: std::sync::LazyLock<std::sync::Arc<Option<BitField>>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(BitFieldFactory::getInstance(1)));

// java public class FOBJH
#[derive(Default)]
pub struct FOBJH {
    // java private short _info;
    _info: i16,
    // java private int _cbObj;
    _cbObj: i32,
}

// java public class FOBJH
impl FOBJH {
    // java public  FOBJH(byte[] buf)
    pub fn new__bytearr(buf: &Vec<u8>) -> Self {
      // java super();
      // java IOAssert.assertTrue("SIZE == buf.length should be true!", 8 == buf.length);
      IOAssert::assertTrue__string_boolean(&"SIZE == buf.length should be true!", 8 == (buf.len() as i32));
      // java int offset = 0;
      let mut offset: i32 = 0;
      // java offset += 2;
      offset = offset + 2;
      // java this._info = LittleEndian.getShort(buf, offset);
      let mut this = Self { _info: LittleEndian::getShort__bytearr_int(&buf, offset), _cbObj: Default::default() };
      // java offset += 2;
      offset = offset + 2;
      // java this._cbObj += LittleEndian.getInt(buf, offset);
      this._cbObj = this._cbObj + LittleEndian::getInt__bytearr_int(&buf, offset);
      this
    }

    // java public  FOBJH()
    pub fn new() -> Self {
      // java super();
      Self { _info: Default::default(), _cbObj: Default::default() }
    }

    // java public byte[] serialize()
    pub fn serialize(&self) -> Vec<u8> {
      // java byte[] ret = new byte[8];
      let mut ret: Vec<u8> = vec![Default::default(); 8usize];
      // java int offset = 0;
      let mut offset: i32 = 0;
      // java LittleEndian.putShort(ret, offset, (short)8);
      LittleEndian::putShort__bytearr_int_short(&mut ret, offset, ((8) as i16));
      // java offset += 2;
      offset = offset + 2;
      // java LittleEndian.putShort(ret, offset, this._info);
      LittleEndian::putShort__bytearr_int_short(&mut ret, offset, self._info);
      // java offset += 2;
      offset = offset + 2;
      // java LittleEndian.putInt(ret, offset, this._cbObj);
      LittleEndian::putInt__bytearr_int_int(&mut ret, offset, self._cbObj);
      // java return ret;
      return ret;
    }

    // java public boolean isCompressed()
    pub fn isCompressed(&self) -> bool {
      // java return 1 == _fCompressed.getValue(this._info);
      return 1 == _fCompressed.getValue(self._info);
    }

    // java public void setCompressed(boolean value)
    pub fn setCompressed(&mut self, value: bool) {
      // java this._info = _fCompressed.setShortBoolean(this._info, value);
      self._info = _fCompressed.setShortBoolean(self._info, value);
    }

    // java public int getCbObj()
    pub fn getCbObj(&self) -> i32 {
      // java return this._cbObj;
      return self._cbObj;
    }

    // java public void setCbObj(int value)
    pub fn setCbObj(&mut self, value: i32) {
      // java this._cbObj = value;
      self._cbObj = value;
    }

}
