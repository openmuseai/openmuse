// java import cn.wps.moffice.util.LittleEndian
use util::cn::wps::moffice::util::LittleEndian::LittleEndian;

// java public final class KmoFormulaData
pub struct KmoFormulaData {
    // java private int _tokensId = 0;
    _tokensId: i32,
    // java private int _tokensPlainSize = 0;
    _tokensPlainSize: i32,
    // java private byte _calcFlags = 0;
    _calcFlags: i8,
    // java private int _cacheValueId = 0;
    _cacheValueId: i32,
}

// java public static final int BYTES_COUNT = 11;
pub const BYTES_COUNT: i32 = 11;

// java public final class KmoFormulaData
impl KmoFormulaData {
    pub fn KmoFormulaData() -> Self {
        KmoFormulaData {
            _tokensId: 0,
            _tokensPlainSize: 0,
            _calcFlags: 0,
            _cacheValueId: 0,
        }
    }

    // java public void clear()
    pub fn clear(&mut self) {
        // java this._tokensId = 0;
        self._tokensId = 0;
        // java this._tokensPlainSize = 0;
        self._tokensPlainSize = 0;
        // java this._calcFlags = 0;
        self._calcFlags = 0;
        // java this._cacheValueId = 0;
        self._cacheValueId = 0;
    }

    // java public int getTokensId()
    pub fn getTokensId(&self) -> i32 {
        // java return this._tokensId;
        self._tokensId
    }

    // java public int getTokensPlainSize()
    pub fn getTokensPlainSize(&self) -> i32 {
        // java return this._tokensPlainSize;
        self._tokensPlainSize
    }

    // java public void setTokens(int tokensId, int tokensPlainSize)
    pub fn setTokens(&mut self, tokensId: i32, tokensPlainSize: i32) {
        // java this._tokensId = tokensId;
        self._tokensId = tokensId;
        // java this._tokensPlainSize = tokensPlainSize;
        self._tokensPlainSize = tokensPlainSize;
    }

    // java public byte getCalcFlags()
    pub fn getCalcFlags(&self) -> i8 {
        // java return this._calcFlags;
        self._calcFlags
    }

    // java public void setCalcFlags(byte flags)
    pub fn setCalcFlags(&mut self, flags: i8) {
        // java this._calcFlags = flags;
        self._calcFlags = flags;
    }

    // java public int getCacheValueId()
    pub fn getCacheValueId(&self) -> i32 {
        // java return this._cacheValueId;
        self._cacheValueId
    }

    // java public void setCacheValueId(int id)
    pub fn setCacheValueId(&mut self, id: i32) {
        // java this._cacheValueId = id;
        self._cacheValueId = id;
    }

    // java public void write(byte[] buf, int offset)
    pub fn write(&self, buf: &mut [u8], mut offset: i32) {
        // java LittleEndian.putInt(buf, offset, this._tokensId);
        LittleEndian::putInt(buf, offset as usize, self._tokensId);
        // java offset += 4;
        offset += 4;
        // java LittleEndian.putUShort(buf, offset, this._tokensPlainSize);
        LittleEndian::putUShort(buf, offset as usize, self._tokensPlainSize);
        // java offset += 2;
        offset += 2;
        // java LittleEndian.putInt(buf, offset, this._cacheValueId);
        LittleEndian::putInt(buf, offset as usize, self._cacheValueId);
        // java offset += 4;
        offset += 4;
        // java buf[offset] = this._calcFlags;
        buf[offset as usize] = self._calcFlags as u8;
    }

    // java public byte[] write()
    pub fn write__1(&self) -> Vec<u8> {
        // java byte[] buf = new byte[11];
        let mut buf = vec![0u8; 11];
        // java this.write(buf, 0);
        self.write(&mut buf, 0);
        // java return buf;
        buf
    }

    // java public int read(byte[] buf, int offset)
    pub fn read(&mut self, buf: &[u8], mut offset: i32) -> i32 {
        // java this._tokensId = LittleEndian.getInt(buf, offset);
        self._tokensId = LittleEndian::getInt(buf, offset as usize);
        // java offset += 4;
        offset += 4;
        // java this._tokensPlainSize = LittleEndian.getUShort(buf, offset);
        self._tokensPlainSize = LittleEndian::getUShort(buf, offset as usize);
        // java offset += 2;
        offset += 2;
        // java this._cacheValueId = LittleEndian.getInt(buf, offset);
        self._cacheValueId = LittleEndian::getInt(buf, offset as usize);
        // java offset += 4;
        offset += 4;
        // java this._calcFlags = buf[offset];
        self._calcFlags = buf[offset as usize] as i8;
        // java return 11;
        11
    }

    // java public int read(byte[] buf)
    pub fn read__1(&mut self, buf: &[u8]) -> i32 {
        // java this.read(buf, 0);
        self.read(buf, 0);
        // java return 11;
        11
    }
}
