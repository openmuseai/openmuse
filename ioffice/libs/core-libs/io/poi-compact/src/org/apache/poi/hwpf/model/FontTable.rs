// Java: import java.io.IOException
// Java: import org.apache.poi.hwpf.model.io.HWPFFileSystem
use crate::org::apache::poi::hwpf::model::io::HWPFFileSystem::HWPFFileSystem;
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian
use super::Ffn::Ffn;

fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    let bytes = value.to_le_bytes();
    data[offset] = bytes[0];
    data[offset + 1] = bytes[1];
}

// Java: public class FontTable
pub struct FontTable {
    // Java field: short _stringCount
    pub _stringCount: i16,
    // Java field: short _extraDataSz
    pub _extraDataSz: i16,
    // Java field: int lcbSttbfffn
    pub lcbSttbfffn: i32,
    // Java field: int fcSttbfffn
    fcSttbfffn: i32,
    // Java field: Ffn[] _fontNames
    pub _fontNames: Vec<Ffn>,
}

// Java: public class FontTable
impl FontTable {
    // Java: FontTable()
    pub fn new() -> Self {
        // Java: super();
        FontTable {
            _stringCount: 0,
            _extraDataSz: 0,
            lcbSttbfffn: 0,
            fcSttbfffn: 0,
            _fontNames: Vec::new(),
        }
    }

    // Java: FontTable(DocumentInputStream stream, int offset, int lcbSttbfffn)
    pub fn new_from_stream(stream: &mut DocumentInputStream, offset: i32, lcb_sttbfffn: i32) -> Self {
        // Java: super();
        // Java: this.lcbSttbfffn = lcbSttbfffn;
        let lcbSttbfffn = lcb_sttbfffn;
        // Java: this.fcSttbfffn = offset;
        let fcSttbfffn = offset;
        // Java: stream.seek((long)offset);
        stream.seek(offset as i64);
        // Java: this._stringCount = stream.readShort();
        let _stringCount = stream.readShort();
        // Java: offset += 2;
        let mut offset = offset + 2;
        // Java: this._extraDataSz = stream.readShort();
        let _extraDataSz = stream.readShort();
        // Java: offset += 2;
        offset += 2;
        // Java: this._fontNames = new Ffn[this._stringCount];
        let mut _fontNames: Vec<Ffn> = Vec::with_capacity(_stringCount as usize);
        // Java: for(int i = 0; i < this._stringCount; ++i)
        for _i in 0.._stringCount {
            // Java: this._fontNames[i] = new Ffn(stream, offset);
            let ffn = Ffn::new_from_stream(stream, offset);
            // Java: offset += this._fontNames[i].getSize();
            offset += ffn.getSize();
            _fontNames.push(ffn);
        }

        FontTable {
            _stringCount,
            _extraDataSz,
            lcbSttbfffn,
            fcSttbfffn,
            _fontNames,
        }
    }

    // Java: short getStringCount()
    pub fn getStringCount(&self) -> i16 {
        // Java: return this._stringCount;
        self._stringCount
    }

    // Java: void setStringCount(short stringCount)
    pub fn setStringCount(&mut self, string_count: i16) {
        // Java: this._stringCount = stringCount;
        self._stringCount = string_count;
    }

    // Java: short getExtraDataSz()
    pub fn getExtraDataSz(&self) -> i16 {
        // Java: return this._extraDataSz;
        self._extraDataSz
    }

    // Java: Ffn[] getFontNames()
    pub fn getFontNames(&self) -> &Vec<Ffn> {
        // Java: return this._fontNames;
        &self._fontNames
    }

    // Java: void setFontNames(Ffn[] fontNames)
    pub fn setFontNames(&mut self, font_names: Vec<Ffn>) {
        // Java: this._fontNames = fontNames;
        self._fontNames = font_names;
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return this.lcbSttbfffn;
        self.lcbSttbfffn
    }

    // Java: void setSize(int lcbSttbfffn)
    pub fn setSize(&mut self, lcb_sttbfffn: i32) {
        // Java: this.lcbSttbfffn = lcbSttbfffn;
        self.lcbSttbfffn = lcb_sttbfffn;
    }

    // Java: String getMainFont(int chpFtc)
    pub fn getMainFont(&self, chp_ftc: i32) -> Option<String> {
        // Java: if (chpFtc < this._stringCount && chpFtc >= 0)
        if chp_ftc < self._stringCount as i32 && chp_ftc >= 0 {
            // Java: return this._fontNames[chpFtc].getMainFontName();
            Some(self._fontNames[chp_ftc as usize].getMainFontName())
        }
        // Java: else
        else {
            // Java: System.out.println("Mismatch in chpFtc with stringCount");
            // Java: return null;
            None
        }
    }

    // Java: String getAltFont(int chpFtc)
    pub fn getAltFont(&self, chp_ftc: i32) -> Option<String> {
        // Java: if (chpFtc >= this._stringCount)
        if chp_ftc >= self._stringCount as i32 {
            // Java: System.out.println("Mismatch in chpFtc with stringCount");
            // Java: return null;
            None
        }
        // Java: else
        else {
            // Java: return this._fontNames[chpFtc].getAltFontName();
            Some(self._fontNames[chp_ftc as usize].getAltFontName())
        }
    }

    // Java: void fcSttbfffn(int fcSttbfffn)
    pub fn fcSttbfffn(&mut self, fc_sttbfffn: i32) {
        // Java: this.fcSttbfffn = fcSttbfffn;
        self.fcSttbfffn = fc_sttbfffn;
    }

    // Java: void writeTo(HWPFFileSystem sys)
    pub fn writeTo(&self, sys: &mut HWPFFileSystem) {
        // Java: HWPFOutputStream tableStream = sys.getStream("0Table");
        let table_stream = sys.getStream("0Table").unwrap();
        // Java: byte[] buf = new byte[2];
        let mut buf = [0u8; 2];
        // Java: LittleEndian.putShort(buf, this._stringCount);
        le_put_short(&mut buf, 0, self._stringCount);
        // Java: tableStream.write(buf);
        table_stream.write(&buf);
        // Java: LittleEndian.putShort(buf, this._extraDataSz);
        le_put_short(&mut buf, 0, self._extraDataSz);
        // Java: tableStream.write(buf);
        table_stream.write(&buf);
        // Java: for(int i = 0; i < this._fontNames.length; ++i)
        for i in 0..self._fontNames.len() {
            // Java: tableStream.write(this._fontNames[i].toByteArray());
            let bytes = self._fontNames[i].toByteArray();
            table_stream.write(&bytes);
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return super.hashCode();
        let ptr = self as *const Self as usize;
        ptr as i32
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &FontTable) -> bool {
        // Java: if (o == null)
        // Java: return false;
        // Java: else
        {
            // Java: boolean retVal = true;
            let mut ret_val = true;
            // Java: if (((FontTable)o).getStringCount() == this._stringCount)
            if o.getStringCount() == self._stringCount {
                // Java: if (((FontTable)o).getExtraDataSz() == this._extraDataSz)
                if o.getExtraDataSz() == self._extraDataSz {
                    // Java: Ffn[] fontNamesNew = ((FontTable)o).getFontNames();
                    let font_names_new = o.getFontNames();
                    // Java: for(int i = 0; i < this._stringCount; ++i)
                    for i in 0..self._stringCount as usize {
                        // Java: if (!this._fontNames[i].equals(fontNamesNew[i]))
                        if !self._fontNames[i].equals(&font_names_new[i]) {
                            // Java: retVal = false;
                            ret_val = false;
                        }
                    }
                }
                // Java: else
                else {
                    // Java: retVal = false;
                    ret_val = false;
                }
            }
            // Java: else
            else {
                // Java: retVal = false;
                ret_val = false;
            }
            // Java: return retVal;
            ret_val
        }
    }
}
