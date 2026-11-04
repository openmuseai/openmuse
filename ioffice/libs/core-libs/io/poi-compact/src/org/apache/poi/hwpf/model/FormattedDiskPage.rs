// Java: import cn.wps.util.Pair
// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;

// Java field: int LAST_BYTE_OFFSET_OF_BLOCK
pub const LAST_BYTE_OFFSET_OF_BLOCK: i32 = 511;
// Java field: int INT_SIZE
const INT_SIZE: i32 = 4;

// Java: public class FormattedDiskPage
pub struct FormattedDiskPage {
    // Java field: DocumentInputStream _fkp
    pub _fkp: Option<*mut DocumentInputStream>,
    // Java field: int _count
    pub _count: i32,
    // Java field: int _offset
    pub _offset: i32,
}

// Java: public class FormattedDiskPage
impl FormattedDiskPage {
    // Java: FormattedDiskPage()
    pub fn new() -> Self {
        // Java: super();
        FormattedDiskPage { _fkp: None, _count: 0, _offset: 0 }
    }

    // Java: FormattedDiskPage(DocumentInputStream documentStream, int offset)
    pub fn new_from_stream(document_stream: &mut DocumentInputStream, offset: i32) -> Self {
        // Java: super();
        // Java: documentStream.seek((long)(offset + 511));
        document_stream.seek((offset + 511) as i64);
        // Java: this._count = documentStream.readUByte();
        let _count = document_stream.readUByte() as i32;
        // Java: this._fkp = documentStream;
        let _fkp = Some(document_stream as *mut DocumentInputStream);
        // Java: this._offset = offset;
        FormattedDiskPage { _fkp, _count, _offset: offset }
    }

    // Java: int getStart(int index)
    pub fn getStart(&mut self, index: i32) -> i32 {
        // Java: this._fkp.seek((long)(this._offset + index * 4));
        let fkp = unsafe { &mut *self._fkp.unwrap() };
        fkp.seek((self._offset + index * 4) as i64);
        // Java: return this._fkp.readInt();
        fkp.readInt()
    }

    // Java: int getEnd(int index)
    pub fn getEnd(&mut self, index: i32) -> i32 {
        // Java: this._fkp.seek((long)(this._offset + (index + 1) * 4));
        let fkp = unsafe { &mut *self._fkp.unwrap() };
        fkp.seek((self._offset + (index + 1) * 4) as i64);
        // Java: return this._fkp.readInt();
        fkp.readInt()
    }

    // Java: void getStartEnd(int index, Pair<Integer, Integer> pair)
    pub fn getStartEnd(&mut self, index: i32) -> (i32, i32) {
        // Java: this._fkp.seek((long)(this._offset + index * 4));
        let fkp = unsafe { &mut *self._fkp.unwrap() };
        fkp.seek((self._offset + index * 4) as i64);
        // Java: int start = this._fkp.readInt();
        let start = fkp.readInt();
        // Java: int end = this._fkp.readInt();
        let end = fkp.readInt();
        // Java: pair.first = start;
        // Java: pair.second = end;
        (start, end)
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this._count;
        self._count
    }

    // Java: abstract byte[] getGrpprl(int var1)
    pub fn getGrpprl(&mut self, _index: i32) -> Vec<u8> {
        Vec::new()
    }
}
