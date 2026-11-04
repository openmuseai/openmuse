// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import java.nio.ByteBuffer
// Java: import org.apache.poi.poifs.common.POIFSBigBlockSize
// Java: import org.apache.poi.poifs.common.POIFSConstants
// Java: import org.apache.poi.util.HexDump
// Java: import org.apache.poi.util.LittleEndian

use crate::org::apache::poi::poifs::common::POIFSBigBlockSize::POIFSBigBlockSize;
use crate::org::apache::poi::poifs::common::POIFSConstants;

fn le_get_int(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32) | ((data[offset + 1] as i32) << 8) | ((data[offset + 2] as i32) << 16) | ((data[offset + 3] as i32) << 24)
}
fn le_get_long(data: &[u8], offset: usize) -> i64 {
    (data[offset] as i64) | ((data[offset + 1] as i64) << 8) | ((data[offset + 2] as i64) << 16) | ((data[offset + 3] as i64) << 24)
        | ((data[offset + 4] as i64) << 32) | ((data[offset + 5] as i64) << 40) | ((data[offset + 6] as i64) << 48) | ((data[offset + 7] as i64) << 56)
}
fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8; data[offset + 1] = (value >> 8) as u8; data[offset + 2] = (value >> 16) as u8; data[offset + 3] = (value >> 24) as u8;
}

fn long_to_hex(value: i64) -> String {
    format!("0x{:016X}", value)
}

// Java: public class HeaderBlockReader implements HeaderBlockConstants
pub struct HeaderBlockReader {
    // Java field: POIFSBigBlockSize bigBlockSize
    bigBlockSize: POIFSBigBlockSize,
    // Java field: int _bat_count
    _bat_count: i32,
    // Java field: int _property_start
    _property_start: i32,
    // Java field: int _sbat_start
    _sbat_start: i32,
    // Java field: int _sbat_count
    _sbat_count: i32,
    // Java field: int _xbat_start
    _xbat_start: i32,
    // Java field: int _xbat_count
    _xbat_count: i32,
    // Java field: byte[] _data
    _data: Vec<u8>,
}

// Java: public class HeaderBlockReader implements HeaderBlockConstants
impl HeaderBlockReader {
    // Java: HeaderBlockReader(InputStream stream)
    pub fn new_from_stream(stream: &mut dyn std::io::Read) -> std::io::Result<Self> {
        // Java: this(readFirst512(stream));
        let data = Self::readFirst512_stream(stream)?;
        let mut reader = Self::new_from_data(data)?;
        // Java: if (this.bigBlockSize.getBigBlockSize() != 512)
        if reader.bigBlockSize.getBigBlockSize() != 512 {
            // Java: int rest = this.bigBlockSize.getBigBlockSize() - 512;
            let rest = reader.bigBlockSize.getBigBlockSize() - 512;
            // Java: stream.skip((long)rest);
            let mut skip_buf = vec![0u8; rest as usize];
            let _ = stream.read(&mut skip_buf);
        }
        Ok(reader)
    }

    // Java: HeaderBlockReader(RandomAccessFile raf)
    pub fn new_from_raf(raf: &mut dyn std::io::Read) -> std::io::Result<Self> {
        // Java: this(readFirst512(raf));
        let data = Self::readFirst512_stream(raf)?;
        let mut reader = Self::new_from_data(data)?;
        // Java: if (this.bigBlockSize.getBigBlockSize() != 512)
        if reader.bigBlockSize.getBigBlockSize() != 512 {
            // Java: int rest = this.bigBlockSize.getBigBlockSize() - 512;
            let rest = reader.bigBlockSize.getBigBlockSize() - 512;
            // Java: raf.skipBytes(rest);
            let mut skip_buf = vec![0u8; rest as usize];
            let _ = raf.read(&mut skip_buf);
        }
        Ok(reader)
    }

    // Java: HeaderBlockReader(ByteBuffer byteBuffer)
    pub fn new_from_bytebuffer(byteBuffer: &[u8]) -> std::io::Result<Self> {
        // Java: this(readFirst512(byteBuffer));
        let data = Self::readFirst512_bytebuffer(byteBuffer);
        let mut reader = Self::new_from_data(data)?;
        // Java: if (this.bigBlockSize.getBigBlockSize() != 512)
        if reader.bigBlockSize.getBigBlockSize() != 512 {
            // Java: int rest = this.bigBlockSize.getBigBlockSize() - 512;
            // Java: byteBuffer.get(new byte[rest]);
        }
        Ok(reader)
    }

    // Java: HeaderBlockReader(byte[] data)
    fn new_from_data(data: Vec<u8>) -> std::io::Result<Self> {
        // Java: super();
        // Java: IOAssert.assertTrue("(data.length == 512) should be true", data.length == 512);
        assert!(data.len() == 512, "(data.length == 512) should be true");
        // Java: this._data = data;
        // Java: long signature = LittleEndian.getLong(data, 0);
        let signature = le_get_long(&data, 0);
        // Java: if (signature != -2226271756974174256L)
        if signature != -2226271756974174256i64 {
            // Java: byte[] OOXML_FILE_HEADER = POIFSConstants.OOXML_FILE_HEADER;
            let ooxml = POIFSConstants::OOXML_FILE_HEADER;
            // Java: if (data[0] == OOXML_FILE_HEADER[0] && data[1] == OOXML_FILE_HEADER[1] && data[2] == OOXML_FILE_HEADER[2] && data[3] == OOXML_FILE_HEADER[3])
            if data[0] == ooxml[0] && data[1] == ooxml[1] && data[2] == ooxml[2] && data[3] == ooxml[3] {
                // Java: throw new IllegalArgumentException("The supplied data appears to be in the Office 2007+ XML. You are calling the part of POI that deals with OLE2 Office Documents. You need to call a different part of POI to process this data (eg XSSF instead of HSSF)");
                panic!("The supplied data appears to be in the Office 2007+ XML. You are calling the part of POI that deals with OLE2 Office Documents. You need to call a different part of POI to process this data (eg XSSF instead of HSSF)");
            }
            // Java: else
            else {
                // Java: throw new IOException("Invalid header signature; read " + longToHex(signature) + ", expected " + longToHex(-2226271756974174256L));
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
                    format!("Invalid header signature; read {}, expected {}", long_to_hex(signature), long_to_hex(-2226271756974174256i64))));
            }
        }
        // Java: else
        let bigBlockSize;
        // Java: if (data[30] == 12)
        if data[30] == 12 {
            // Java: this.bigBlockSize = POIFSConstants.LARGER_BIG_BLOCK_SIZE_DETAILS;
            bigBlockSize = POIFSConstants::get_larger_big_block_size_details().clone();
        }
        // Java: else
        else {
            // Java: if (data[30] != 9)
            if data[30] != 9 {
                // Java: throw new IOException("Unsupported blocksize  (2^" + data[30] + "). Expected 2^9 or 2^12.");
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
                    format!("Unsupported blocksize  (2^{}). Expected 2^9 or 2^12.", data[30])));
            }
            // Java: this.bigBlockSize = POIFSConstants.SMALLER_BIG_BLOCK_SIZE_DETAILS;
            bigBlockSize = POIFSConstants::get_smaller_big_block_size_details().clone();
        }
        // Java: this._bat_count = LittleEndian.getInt(data, 44);
        let _bat_count = le_get_int(&data, 44);
        // Java: this._property_start = LittleEndian.getInt(data, 48);
        let _property_start = le_get_int(&data, 48);
        // Java: this._sbat_start = LittleEndian.getInt(data, 60);
        let _sbat_start = le_get_int(&data, 60);
        // Java: this._sbat_count = LittleEndian.getInt(data, 64);
        let _sbat_count = le_get_int(&data, 64);
        // Java: this._xbat_start = LittleEndian.getInt(data, 68);
        let _xbat_start = le_get_int(&data, 68);
        // Java: this._xbat_count = LittleEndian.getInt(data, 72);
        let _xbat_count = le_get_int(&data, 72);

        Ok(HeaderBlockReader {
            bigBlockSize,
            _bat_count,
            _property_start,
            _sbat_start,
            _sbat_count,
            _xbat_start,
            _xbat_count,
            _data: data,
        })
    }

    // Java: static byte[] readFirst512(InputStream stream)
    fn readFirst512_stream(stream: &mut dyn std::io::Read) -> std::io::Result<Vec<u8>> {
        // Java: byte[] data = new byte[512];
        let mut data = vec![0u8; 512];
        // Java: int l = stream.read(data);
        let l = stream.read(&mut data)?;
        // Java: if (l != 512)
        if l != 512 {
            // Java: throw new IOException();
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Could not read 512 bytes"));
        }
        // Java: else
        // Java: return data;
        Ok(data)
    }

    // Java: static byte[] readFirst512(RandomAccessFile raf)
    fn readFirst512_raf(raf: &mut dyn std::io::Read) -> std::io::Result<Vec<u8>> {
        // Java: byte[] data = new byte[512];
        let mut data = vec![0u8; 512];
        // Java: raf.readFully(data);
        raf.read_exact(&mut data)?;
        // Java: return data;
        Ok(data)
    }

    // Java: static byte[] readFirst512(ByteBuffer bb)
    fn readFirst512_bytebuffer(bb: &[u8]) -> Vec<u8> {
        // Java: byte[] data = new byte[512];
        let mut data = vec![0u8; 512];
        // Java: bb.get(data);
        let len = std::cmp::min(512, bb.len());
        data[..len].copy_from_slice(&bb[..len]);
        // Java: return data;
        data
    }

    // Java: static String longToHex(long value)
    fn longToHex(value: i64) -> String {
        // Java: return new String(HexDump.longToHex(value));
        long_to_hex(value)
    }

    // Java: int getPropertyStart()
    pub fn getPropertyStart(&self) -> i32 {
        // Java: return this._property_start;
        self._property_start
    }

    // Java: int getSBATStart()
    pub fn getSBATStart(&self) -> i32 {
        // Java: return this._sbat_start;
        self._sbat_start
    }

    // Java: int getSBATCount()
    pub fn getSBATCount(&self) -> i32 {
        // Java: return this._sbat_count;
        self._sbat_count
    }

    // Java: int getBATCount()
    pub fn getBATCount(&self) -> i32 {
        // Java: return this._bat_count;
        self._bat_count
    }

    // Java: int[] getBATArray()
    pub fn getBATArray(&self) -> Vec<i32> {
        // Java: int[] result = new int[109];
        let mut result = vec![0i32; 109];
        // Java: int offset = 76;
        let mut offset: usize = 76;
        // Java: for(int j = 0; j < 109; ++j)
        for j in 0..109 {
            // Java: result[j] = LittleEndian.getInt(this._data, offset);
            result[j] = le_get_int(&self._data, offset);
            // Java: offset += 4;
            offset += 4;
        }
        // Java: return result;
        result
    }

    // Java: int getXBATCount()
    pub fn getXBATCount(&self) -> i32 {
        // Java: return this._xbat_count;
        self._xbat_count
    }

    // Java: int getXBATIndex()
    pub fn getXBATIndex(&self) -> i32 {
        // Java: return this._xbat_start;
        self._xbat_start
    }

    // Java: POIFSBigBlockSize getBigBlockSize()
    pub fn getBigBlockSize(&self) -> &POIFSBigBlockSize {
        // Java: return this.bigBlockSize;
        &self.bigBlockSize
    }

    // Java: void writeData(OutputStream stream)
    pub fn writeData(&mut self, stream: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Java: LittleEndian.putInt(this._data, 44, this._bat_count);
        le_put_int(&mut self._data, 44, self._bat_count);
        // Java: LittleEndian.putInt(this._data, 48, this._property_start);
        le_put_int(&mut self._data, 48, self._property_start);
        // Java: LittleEndian.putInt(this._data, 60, this._sbat_start);
        le_put_int(&mut self._data, 60, self._sbat_start);
        // Java: LittleEndian.putInt(this._data, 64, this._sbat_count);
        le_put_int(&mut self._data, 64, self._sbat_count);
        // Java: LittleEndian.putInt(this._data, 68, this._xbat_start);
        le_put_int(&mut self._data, 68, self._xbat_start);
        // Java: LittleEndian.putInt(this._data, 72, this._xbat_count);
        le_put_int(&mut self._data, 72, self._xbat_count);
        // Java: stream.write(this._data, 0, 512);
        stream.write_all(&self._data[..512])?;
        // Java: int i = 512;
        // Java: for(int len = this.bigBlockSize.getBigBlockSize(); i < len; ++i)
        let len = self.bigBlockSize.getBigBlockSize();
        for _i in 512..len {
            // Java: stream.write(0);
            stream.write_all(&[0u8])?;
        }
        Ok(())
    }
}
