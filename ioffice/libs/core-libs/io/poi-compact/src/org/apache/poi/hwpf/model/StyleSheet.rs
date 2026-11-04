// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.moffice.crash.FileDamagedException
// Java: import java.io.IOException
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFFileSystem::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use super::StyleDescription::StyleDescription;
use super::HDFType::HDFType;

fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    let bytes = value.to_le_bytes();
    data[offset] = bytes[0];
    data[offset + 1] = bytes[1];
}

// Java field: String TAG
pub(crate) const TAG: &str = "";
// Java field: int NIL_STYLE
pub(crate) const NIL_STYLE: i32 = 4095;
// Java field: int PAP_TYPE
pub(crate) const PAP_TYPE: i32 = 1;
// Java field: int CHP_TYPE
pub(crate) const CHP_TYPE: i32 = 2;
// Java field: int SEP_TYPE
pub(crate) const SEP_TYPE: i32 = 4;
// Java field: int TAP_TYPE
pub(crate) const TAP_TYPE: i32 = 5;

// Java: public class StyleSheet implements HDFType
pub struct StyleSheet {
    // Java field: int _stshiLength
    _stshiLength: i32,
    // Java field: int _baseLength
    pub _baseLength: i32,
    // Java field: int _flags
    _flags: i32,
    // Java field: int _maxIndex
    _maxIndex: i32,
    // Java field: int _maxFixedIndex
    _maxFixedIndex: i32,
    // Java field: int _stylenameVersion
    _stylenameVersion: i32,
    // Java field: int[] _rgftc
    pub _rgftc: Vec<i32>,
    // Java field: int _ftcBi
    pub _ftcBi: i32,
    // Java field: StyleDescription[] _styleDescriptions
    pub _styleDescriptions: Vec<Option<StyleDescription>>,
    // Java field: boolean mException
    pub mException: bool,
}

// Java: public class StyleSheet implements HDFType
impl StyleSheet {
    // Java: StyleSheet()
    pub fn new() -> Self {
        // Java: super();
        StyleSheet {
            _stshiLength: 0, _baseLength: 0, _flags: 0, _maxIndex: 0,
            _maxFixedIndex: 0, _stylenameVersion: 0, _rgftc: vec![0; 3],
            _ftcBi: 0, _styleDescriptions: Vec::new(), mException: false,
        }
    }

    // Java: StyleSheet(DocumentInputStream tableStream, int offset)
    pub fn new_from_stream(table_stream: &mut DocumentInputStream, mut offset: i32) -> Self {
        // Java: super();
        // Java: tableStream.seek((long)offset);
        table_stream.seek(offset as i64);
        // Java: this._stshiLength = tableStream.readUShort();
        let _stshiLength = table_stream.readUShort() as i32;
        // Java: int var9 = offset + 2;
        // Java: int stdCount = tableStream.readUShort();
        let std_count = table_stream.readUShort() as i32;
        // Java: int var10 = var9 + 2;
        // Java: this._baseLength = tableStream.readUShort();
        let _baseLength = table_stream.readUShort() as i32;
        // Java: int var11 = var10 + 2;
        // Java: this._flags = tableStream.readUShort();
        let _flags = table_stream.readUShort() as i32;
        // Java: int var12 = var11 + 2;
        // Java: this._maxIndex = tableStream.readUShort();
        let _maxIndex = table_stream.readUShort() as i32;
        // Java: int var13 = var12 + 2;
        // Java: this._maxFixedIndex = tableStream.readUShort();
        let _maxFixedIndex = table_stream.readUShort() as i32;
        // Java: int var14 = var13 + 2;
        // Java: this._stylenameVersion = tableStream.readUShort();
        let _stylenameVersion = table_stream.readUShort() as i32;
        // Java: int var15 = var14 + 2;
        // Java: this._rgftc = new int[3];
        let mut _rgftc = vec![0i32; 3];
        // Java: this._rgftc[0] = tableStream.readUShort();
        _rgftc[0] = table_stream.readUShort() as i32;
        // Java: int var16 = var15 + 2;
        // Java: this._rgftc[1] = tableStream.readUShort();
        _rgftc[1] = table_stream.readUShort() as i32;
        // Java: int var17 = var16 + 2;
        // Java: this._rgftc[2] = tableStream.readUShort();
        _rgftc[2] = table_stream.readUShort() as i32;
        // Java: int var18 = var17 + 2;
        // Java: this._ftcBi = tableStream.readUShort();
        let _ftcBi = table_stream.readUShort() as i32;
        // Java: int var19 = var18 + 2;

        // Java: offset = offset + 2 + this._stshiLength;
        offset = offset + 2 + _stshiLength;
        // Java: tableStream.seek((long)offset);
        table_stream.seek(offset as i64);
        // Java: this._styleDescriptions = new StyleDescription[stdCount];
        let mut _styleDescriptions: Vec<Option<StyleDescription>> = (0..std_count).map(|_| None).collect();
        let mut mException = false;

        // Java: for(int x = 0; x < stdCount; ++x)
        for x in 0..std_count as usize {
            // Java: int stdSize = tableStream.readUShort();
            let std_size = table_stream.readUShort() as i32;
            // Java: offset += 2;
            offset += 2;
            // Java: if (stdSize > 0)
            if std_size > 0 {
                // Java: try
                // Java: StyleDescription aStyle = new StyleDescription(tableStream, this._baseLength, offset, true);
                let a_style = StyleDescription::new_from_stream(table_stream, _baseLength, offset, true);
                // Java: this._styleDescriptions[x] = aStyle;
                _styleDescriptions[x] = Some(a_style);
                // Java: catch (Throwable e)
                // Java: if (e instanceof FileDamagedException)
                // Java: this.mException = true;
                // Java: IOLog.e(TAG, "Throwable", e);
            }
            // Java: offset += stdSize;
            offset += std_size;
            // Java: tableStream.seek((long)offset);
            table_stream.seek(offset as i64);
        }

        StyleSheet {
            _stshiLength, _baseLength, _flags, _maxIndex, _maxFixedIndex,
            _stylenameVersion, _rgftc, _ftcBi, _styleDescriptions, mException,
        }
    }

    // Java: boolean isException()
    pub fn isException(&self) -> bool {
        // Java: return this.mException;
        self.mException
    }

    // Java: int getBaseLength()
    pub fn getBaseLength(&self) -> i32 {
        // Java: return this._baseLength;
        self._baseLength
    }

    // Java: int[] getRgftc()
    pub fn getRgftc(&self) -> &Vec<i32> {
        // Java: return this._rgftc;
        &self._rgftc
    }

    // Java: int getFtcBi()
    pub fn getFtcBi(&self) -> i32 {
        // Java: return this._ftcBi;
        self._ftcBi
    }

    // Java: void setRgftc(int[] rgftc)
    pub fn setRgftc(&mut self, rgftc: Vec<i32>) {
        // Java: this._rgftc = rgftc;
        self._rgftc = rgftc;
    }

    // Java: void resetStyleDescriptions(int stdCount)
    pub fn resetStyleDescriptions(&mut self, std_count: i32) {
        // Java: this._styleDescriptions = new StyleDescription[stdCount];
        self._styleDescriptions = (0..std_count).map(|_| None).collect();
    }

    // Java: void addStyleDescription(int index, StyleDescription std)
    pub fn addStyleDescription(&mut self, index: i32, sd: StyleDescription) {
        // Java: this._styleDescriptions[index] = std;
        self._styleDescriptions[index as usize] = Some(sd);
    }

    // Java: void writeTo(HWPFOutputStream out)
    pub fn writeTo(&self, out: &mut HWPFOutputStream) {
        // Java: int offset = 0;
        let mut offset: usize = 0;
        // Java: byte[] buf = new byte[this._stshiLength + 2];
        let mut buf = vec![0u8; (self._stshiLength + 2) as usize];
        // Java: LittleEndian.putShort(buf, offset, (short)this._stshiLength);
        le_put_short(&mut buf, offset, self._stshiLength as i16);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putShort(buf, offset, (short)this._styleDescriptions.length);
        le_put_short(&mut buf, offset, self._styleDescriptions.len() as i16);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putShort(buf, offset, (short)this._baseLength);
        le_put_short(&mut buf, offset, self._baseLength as i16);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putShort(buf, offset, (short)this._flags);
        le_put_short(&mut buf, offset, self._flags as i16);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putShort(buf, offset, (short)this._maxIndex);
        le_put_short(&mut buf, offset, self._maxIndex as i16);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putShort(buf, offset, (short)this._maxFixedIndex);
        le_put_short(&mut buf, offset, self._maxFixedIndex as i16);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putShort(buf, offset, (short)this._stylenameVersion);
        le_put_short(&mut buf, offset, self._stylenameVersion as i16);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putShort(buf, offset, (short)this._rgftc[0]);
        le_put_short(&mut buf, offset, self._rgftc[0] as i16);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putShort(buf, offset, (short)this._rgftc[1]);
        le_put_short(&mut buf, offset, self._rgftc[1] as i16);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putShort(buf, offset, (short)this._rgftc[2]);
        le_put_short(&mut buf, offset, self._rgftc[2] as i16);
        // Java: out.write(buf);
        out.write(&buf);
        // Java: byte[] sizeHolder = new byte[2];
        let mut size_holder = [0u8; 2];
        // Java: for(int x = 0; x < this._styleDescriptions.length; ++x)
        for x in 0..self._styleDescriptions.len() {
            // Java: if (this._styleDescriptions[x] != null)
            if let Some(ref sd) = self._styleDescriptions[x] {
                // Java: byte[] std = this._styleDescriptions[x].toByteArray();
                let std_bytes = sd.toByteArray();
                // Java: LittleEndian.putShort(sizeHolder, (short)(std.length + std.length % 2));
                le_put_short(&mut size_holder, 0, (std_bytes.len() + std_bytes.len() % 2) as i16);
                // Java: out.write(sizeHolder);
                out.write(&size_holder);
                // Java: out.write(std);
                out.write(&std_bytes);
                // Java: if (std.length % 2 == 1)
                if std_bytes.len() % 2 == 1 {
                    // Java: out.write(0);
                    out.write(&[0u8]);
                }
            }
            // Java: else
            else {
                // Java: sizeHolder[0] = 0;
                size_holder[0] = 0;
                // Java: sizeHolder[1] = 0;
                size_holder[1] = 0;
                // Java: out.write(sizeHolder);
                out.write(&size_holder);
            }
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return super.hashCode();
        let ptr = self as *const Self as usize;
        ptr as i32
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &StyleSheet) -> bool {
        // Java: if (o == null)
        // Java: return false;
        // Java: else
        // Java: StyleSheet ss = (StyleSheet)o;
        // Java: if (ss._baseLength == this._baseLength && ss._flags == this._flags && ss._maxFixedIndex == this._maxFixedIndex && ss._maxIndex == this._maxIndex && ss._rgftc[0] == this._rgftc[0] && ss._rgftc[1] == this._rgftc[1] && ss._rgftc[2] == this._rgftc[2] && ss._stshiLength == this._stshiLength && ss._stylenameVersion == this._stylenameVersion && ss._styleDescriptions.length == this._styleDescriptions.length)
        if o._baseLength == self._baseLength && o._flags == self._flags
            && o._maxFixedIndex == self._maxFixedIndex && o._maxIndex == self._maxIndex
            && o._rgftc[0] == self._rgftc[0] && o._rgftc[1] == self._rgftc[1]
            && o._rgftc[2] == self._rgftc[2] && o._stshiLength == self._stshiLength
            && o._stylenameVersion == self._stylenameVersion
            && o._styleDescriptions.len() == self._styleDescriptions.len()
        {
            // Java: for(int x = 0; x < this._styleDescriptions.length; ++x)
            for x in 0..self._styleDescriptions.len() {
                // Java: if (ss._styleDescriptions[x] != this._styleDescriptions[x] && !ss._styleDescriptions[x].equals(this._styleDescriptions[x]))
                match (&o._styleDescriptions[x], &self._styleDescriptions[x]) {
                    (Some(a), Some(b)) => {
                        if !a.equals(b) {
                            // Java: return false;
                            return false;
                        }
                    }
                    (None, None) => {}
                    _ => return false,
                }
            }
            // Java: return true;
            true
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: int numStyles()
    pub fn numStyles(&self) -> i32 {
        // Java: return this._styleDescriptions.length;
        self._styleDescriptions.len() as i32
    }

    // Java: StyleDescription getStyleDescription(int x)
    pub fn getStyleDescription(&self, x: i32) -> Option<&StyleDescription> {
        // Java: return x >= 0 && x < this._styleDescriptions.length ? this._styleDescriptions[x] : null;
        if x >= 0 && (x as usize) < self._styleDescriptions.len() {
            self._styleDescriptions[x as usize].as_ref()
        } else {
            None
        }
    }
}

impl HDFType for StyleSheet {}
