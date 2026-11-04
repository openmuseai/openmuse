// Java: import cn.wps.base.log.IOLog
// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.io.PrintWriter
// Java: import java.util.Collections
// Java: import java.util.List
// Java: import org.apache.poi.poifs.common.IBlockDocumentInputStream
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// Java: import org.apache.poi.poifs.filesystem.MemoryDocumentInputStream
// Java: import org.apache.poi.util.BitField
// Java: import org.apache.poi.util.BitFieldFactory
// Java: import org.apache.poi.util.LittleEndian

use crate::org::apache::poi::poifs::filesystem::MemoryDocumentInputStream::MemoryDocumentInputStream;
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
use crate::org::apache::poi::poifs::common::IBlockDocumentInputStream::IBlockDocumentInputStream;
use crate::org::apache::poi::ddf::EscherProperty::EscherProperty;

use std::io::{self, Write};

fn le_get_short(data: &[u8], offset: usize) -> i16 {
    let lo = data[offset] as i16;
    let hi = data[offset + 1] as i16;
    (hi << 8) | (lo & 0xFF)
}

fn le_get_int(data: &[u8], offset: usize) -> i32 {
    let b0 = data[offset] as i32 & 0xFF;
    let b1 = data[offset + 1] as i32 & 0xFF;
    let b2 = data[offset + 2] as i32 & 0xFF;
    let b3 = data[offset + 3] as i32 & 0xFF;
    (b3 << 24) | (b2 << 16) | (b1 << 8) | b0
}

fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
}

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
    data[offset + 2] = (value >> 16) as u8;
    data[offset + 3] = (value >> 24) as u8;
}

// Java field: int HEADER_SIZE
pub const HEADER_SIZE: i32 = 8;
// Java field: String TAG
// Java field: byte[] buf
// Java field: BitField inst
// Java field: BitField ver

// Java: public class EscherRecord
pub struct EscherRecord {
    // Java field: EscherRecordHeader _header
    pub(crate) _header: EscherRecordHeader,
    pub(crate) child_records: Vec<EscherRecord>,
}

// Java: public class EscherRecordHeader
pub struct EscherRecordHeader {
    // Java field: short options
    pub(crate) options: i16,
    // Java field: BitField inst
    // Java field: BitField ver
    // Java field: short recordId
    pub(crate) recordId: i16,
    // Java field: int remainingBytes
    pub(crate) remainingBytes: i32,
}

// Java: public class EscherRecord
impl EscherRecord {
    // Java: EscherRecord()
    pub fn new() -> Self {
        // Java: super();
        EscherRecord {
            _header: EscherRecordHeader::new(),
            child_records: Vec::new(),
        }
    }

    // Java: int fillFields(byte[] data, int offset, EscherRecordFactory recordFactory)
    pub fn fillFields_bytes(&mut self, data: &[u8], offset: i32, _record_factory: &dyn super::EscherRecordFactory::EscherRecordFactory) -> i32 {
        // Java: try
        {
            // Java: return this.fillFields((DocumentInputStream)(new MemoryDocumentInputStream(data, 0)), offset, recordFactory);
            let mut _mis = MemoryDocumentInputStream::new(data.to_vec(), 0);
            return self.fillFields_stream(&mut DocumentInputStream::new_empty(), offset, _record_factory).unwrap_or(0);
        }
        // Java: catch (IOException e)
        // Java: e.printStackTrace();
        // Java: throw new RuntimeException("Error read EscherRecord: " + e.getMessage());
    }

    // Java: int fillFields(byte[] data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_bytes_ext(&mut self, data: &[u8], offset: i32, _record_factory: &dyn super::EscherRecordFactory::EscherRecordFactory, _picture_directory: &str, _extension: &str) -> i32 {
        // Java: try
        {
            // Java: return this.fillFields((DocumentInputStream)(new MemoryDocumentInputStream(data, 0)), offset, recordFactory);
            return self.fillFields_bytes(data, offset, _record_factory);
        }
        // Java: catch (IOException e)
        // Java: e.printStackTrace();
        // Java: throw new RuntimeException("Error read EscherRecord: " + e.getMessage());
    }

    // Java: abstract int fillFields(DocumentInputStream var1, int var2, EscherRecordFactory var3)
    pub fn fillFields_stream(&mut self, _data: &mut DocumentInputStream, _offset: i32, _record_factory: &dyn super::EscherRecordFactory::EscherRecordFactory) -> io::Result<i32> {
        Ok(0)
    }

    // Java: int fillFields(DocumentInputStream data, int pOffset, EscherRecordFactory recordFactory, boolean useEscherSpgrContainerRecord)
    pub fn fillFields_stream_flag(&mut self, data: &mut DocumentInputStream, p_offset: i32, record_factory: &dyn super::EscherRecordFactory::EscherRecordFactory, _use_spgr: bool) -> io::Result<i32> {
        // Java: return this.fillFields(data, pOffset, recordFactory);
        self.fillFields_stream(data, p_offset, record_factory)
    }

    // Java: int fillFields(DocumentInputStream data, int pOffset, EscherRecordFactory recordFactory, boolean useEscherSpgrContainerRecord, boolean fillProps)
    pub fn fillFields_stream_flags(&mut self, data: &mut DocumentInputStream, p_offset: i32, record_factory: &dyn super::EscherRecordFactory::EscherRecordFactory, _use_spgr: bool, _fill_props: bool) -> io::Result<i32> {
        // Java: return this.fillFields(data, pOffset, recordFactory);
        self.fillFields_stream(data, p_offset, record_factory)
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, _offset: i32, record_factory: &dyn super::EscherRecordFactory::EscherRecordFactory, _picture_directory: &str, _extension: &str) -> io::Result<i32> {
        // Java: byte[] header = new byte[8];
        let mut header = vec![0u8; 8];
        // Java: LittleEndian.putShort(header, this.getOptions());
        le_put_short(&mut header, 0, self.getOptions());
        // Java: LittleEndian.putShort(header, this.getRecordId());
        le_put_short(&mut header, 2, self.getRecordId());
        // Java: LittleEndian.putInt(header, this.getRemainingBytes());
        le_put_int(&mut header, 4, self.getRemainingBytes());
        // Java: byte[] theData = new byte[this.getRemainingBytes()];
        let mut the_data = vec![0u8; self.getRemainingBytes() as usize];
        // Java: data.read(theData);
        data.read(&mut the_data);
        // Java: byte[] dataIn = new byte[8 + this.getRemainingBytes()];
        let mut data_in = vec![0u8; 8 + self.getRemainingBytes() as usize];
        // Java: System.arraycopy(header, 0, dataIn, 0, 8);
        data_in[0..8].copy_from_slice(&header[0..8]);
        // Java: System.arraycopy(theData, 0, dataIn, 8, this.getRemainingBytes());
        let rem = self.getRemainingBytes() as usize;
        data_in[8..8 + rem].copy_from_slice(&the_data[0..rem]);
        // Java: return this.fillFields((byte[])dataIn, 0, recordFactory);
        Ok(self.fillFields_bytes(&data_in, 0, record_factory))
    }

    // Java: int readHeader(byte[] data, int offset)
    pub fn readHeader_bytes(&mut self, data: &[u8], offset: i32) -> i32 {
        // Java: this._header = EscherRecord.EscherRecordHeader.readHeader(data, offset);
        self._header = EscherRecordHeader::readHeader_bytes(data, offset as usize);
        // Java: return this._header.getRemainingBytes();
        self._header.getRemainingBytes()
    }

    // Java: int readHeader(DocumentInputStream data, int offset)
    pub fn readHeader_stream(&mut self, data: &mut DocumentInputStream, offset: i32) -> i32 {
        // Java: this._header = EscherRecord.EscherRecordHeader.readHeader(data, offset);
        self._header = EscherRecordHeader::readHeader_stream(data, offset);
        // Java: return this._header.getRemainingBytes();
        self._header.getRemainingBytes()
    }

    // Java: int readHeader(IBlockDocumentInputStream data, int offset)
    pub fn readHeader_block(&mut self, data: &mut dyn IBlockDocumentInputStream, _offset: i32) -> i32 {
        // Java: this._header = EscherRecord.EscherRecordHeader.readHeader(data, offset);
        self._header = EscherRecordHeader::readHeader_block(data);
        // Java: return this._header.getRemainingBytes();
        self._header.getRemainingBytes()
    }

    // Java: boolean isContainerRecord()
    pub fn isContainerRecord(&self) -> bool {
        // Java: return (this._header.getOptions() & 15) == 15;
        (self._header.getOptions() & 15) == 15
    }

    // Java: short getOptions()
    pub fn getOptions(&self) -> i16 {
        // Java: return this._header.getOptions();
        self._header.getOptions()
    }

    // Java: void setOptions(short options)
    pub fn setOptions(&mut self, options: i16) {
        // Java: this._header.setOptions(options);
        self._header.setOptions(options);
    }

    // Java: byte[] serialize()
    pub fn serialize_to_bytes(&mut self) -> Vec<u8> {
        // Java: byte[] retval = new byte[this.getRecordSize()];
        let mut retval = vec![0u8; self.getRecordSize() as usize];
        // Java: this.serialize(0, retval);
        self.serialize_offset(0, &mut retval);
        // Java: return retval;
        retval
    }

    // Java: int serialize(int offset, byte[] data)
    pub fn serialize_offset(&mut self, offset: i32, data: &mut [u8]) -> i32 {
        // Java: return this.serialize(offset, data, new NullEscherSerializationListener());
        0
    }

    // Java: abstract int serialize(int var1, byte[] var2, EscherSerializationListener var3)
    pub fn serialize_with_listener(&mut self, _offset: i32, _data: &mut [u8], _listener: &mut dyn super::EscherSerializationListener::EscherSerializationListener) -> i32 {
        0
    }

    // Java: int serialize(OutputStream output)
    pub fn serialize_to_output(&mut self, output: &mut dyn Write) -> io::Result<i32> {
        // Java: byte[] buf = new byte[this.getRecordSize()];
        let mut buf = vec![0u8; self.getRecordSize() as usize];
        // Java: int n = this.serialize(0, buf, new NullEscherSerializationListener());
        let n = self.serialize_offset(0, &mut buf);
        // Java: output.write(buf, 0, n);
        output.write_all(&buf[0..n as usize])?;
        // Java: return n;
        Ok(n)
    }

    // Java: abstract int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
        0
    }

    // Java: short getRecordId()
    pub fn getRecordId(&self) -> i16 {
        // Java: return this._header.getRecordId();
        self._header.getRecordId()
    }

    // Java: void setRecordId(short recordId)
    pub fn setRecordId(&mut self, record_id: i16) {
        // Java: this._header.setRecordId(recordId);
        self._header.setRecordId(record_id);
    }

    // Java: int getRemainingBytes()
    pub fn getRemainingBytes(&self) -> i32 {
        // Java: return this._header.getRemainingBytes();
        self._header.getRemainingBytes()
    }

    // Java: void setRemainingBytes(int remainingBytes)
    pub fn setRemainingBytes(&mut self, remaining_bytes: i32) {
        // Java: this._header.setRemainingBytes(remainingBytes);
        self._header.setRemainingBytes(remaining_bytes);
    }

    // Java: List<EscherRecord> getChildRecords()
    pub fn getChildRecords(&self) -> &Vec<EscherRecord> {
        // Java: return Collections.emptyList();
        &self.child_records
    }

    // Java: void setChildRecords(List<EscherRecord> childRecords)
    pub fn setChildRecords(&mut self, child_records: Vec<EscherRecord>) {
        // Java: throw new UnsupportedOperationException("This record does not support child records.");
        self.child_records = child_records;
    }

    // Java: Object clone()
    pub fn clone_record(&self) -> EscherRecord {
        // Java: throw new RuntimeException("The class " + this.getClass().getName() + " needs to define a clone method");
        panic!("needs to define a clone method");
    }

    // Java: EscherRecord getChild(int index)
    pub fn getChild(&self, index: i32) -> &EscherRecord {
        // Java: return (EscherRecord)this.getChildRecords().get(index);
        &self.child_records[index as usize]
    }

    // Java: void display(PrintWriter w, int indent)
    pub fn display(&self, w: &mut dyn Write, indent: i32) {
        // Java: for(int i = 0; i < indent * 4; ++i)
        for _i in 0..indent * 4 {
            // Java: w.print(' ');
            let _ = write!(w, " ");
        }
        // Java: w.println(this.getRecordName());
        let _ = writeln!(w, "{}", self.getRecordName());
    }

    // Java: abstract String getRecordName()
    pub fn getRecordName(&self) -> String {
        String::new()
    }

    // Java: short getInstance()
    pub fn getInstance(&self) -> i16 {
        // Java: return (short)(this._header.getOptions() >> 4);
        self._header.getOptions() >> 4
    }

    // Java: short getInst()
    pub fn getInst(&self) -> i16 {
        // Java: return this._header.getInst();
        self._header.getInst()
    }

    // Java: void setInst(short val)
    pub fn setInst(&mut self, val: i16) {
        // Java: this._header.setInst(val);
        self._header.setInst(val);
    }

    // Java: short getVer()
    pub fn getVer(&self) -> i16 {
        // Java: return this._header.getVer();
        self._header.getVer()
    }

    // Java: void setVer(short val)
    pub fn setVer(&mut self, val: i16) {
        // Java: this._header.setVer(val);
        self._header.setVer(val);
    }

    // Java: EscherProperty getEscherPropertyById(int id)
    pub fn getEscherPropertyById(&self, _id: i32) -> Option<&EscherProperty> {
        // Java: return null;
        None
    }
}

// Java: public class EscherRecordHeader
impl EscherRecordHeader {
    // Java: EscherRecordHeader()
    pub fn new() -> Self {
        // Java: super();
        EscherRecordHeader {
            options: 0,
            recordId: 0,
            remainingBytes: 0,
        }
    }

    // Java: static EscherRecordHeader readHeader(byte[] data, int offset)
    pub fn readHeader_bytes(data: &[u8], offset: usize) -> EscherRecordHeader {
        // Java: EscherRecordHeader header = new EscherRecordHeader();
        let mut header = EscherRecordHeader::new();
        // Java: header.options = LittleEndian.getShort(data, offset);
        header.options = le_get_short(data, offset);
        // Java: header.recordId = LittleEndian.getShort(data, offset + 2);
        header.recordId = le_get_short(data, offset + 2);
        // Java: header.remainingBytes = LittleEndian.getInt(data, offset + 4);
        header.remainingBytes = le_get_int(data, offset + 4);
        // Java: return header;
        header
    }

    // Java: static EscherRecordHeader readHeader(DocumentInputStream data, int offset)
    pub fn readHeader_stream(data: &mut DocumentInputStream, offset: i32) -> EscherRecordHeader {
        // Java: data.seek((long)offset);
        data.seek(offset as i64);
        // Java: EscherRecordHeader header = new EscherRecordHeader();
        let mut header = EscherRecordHeader::new();
        let mut buf = [0u8; 8];
        // Java: try
        {
            // Java: data.read(buf, 0, 8);
            let _ = data.read_off(&mut buf, 0, 8);
        }
        // Java: catch (IOException e)
        {
            // Java: IOLog.e(TAG, "IOException", e);
        }
        // Java: header.options = LittleEndian.getShort(buf, 0);
        header.options = le_get_short(&buf, 0);
        // Java: header.recordId = LittleEndian.getShort(buf, 2);
        header.recordId = le_get_short(&buf, 2);
        // Java: header.remainingBytes = LittleEndian.getInt(buf, 4);
        header.remainingBytes = le_get_int(&buf, 4);
        // Java: return header;
        header
    }

    // Java: static EscherRecordHeader readHeader(IBlockDocumentInputStream data, int offset)
    pub fn readHeader_block(data: &mut dyn IBlockDocumentInputStream) -> EscherRecordHeader {
        // Java: EscherRecordHeader header = new EscherRecordHeader();
        let mut header = EscherRecordHeader::new();
        // Java: header.options = data.readShort();
        header.options = data.readShort();
        // Java: header.recordId = data.readShort();
        header.recordId = data.readShort();
        // Java: header.remainingBytes = data.readInt();
        header.remainingBytes = data.readInt();
        // Java: return header;
        header
    }

    // Java: short getInst()
    pub fn getInst(&self) -> i16 {
        // Java: return inst.getShortValue(this.options);
        (self.options >> 4) & 0x0FFF
    }

    // Java: void setInst(short val)
    pub fn setInst(&mut self, val: i16) {
        // Java: this.options = inst.setShortValue(this.options, val);
        self.options = (self.options & 0x000F) | ((val & 0x0FFF) << 4);
    }

    // Java: short getVer()
    pub fn getVer(&self) -> i16 {
        // Java: return ver.getShortValue(this.options);
        self.options & 0x000F
    }

    // Java: void setVer(short val)
    pub fn setVer(&mut self, val: i16) {
        // Java: this.options = ver.setShortValue(this.options, val);
        self.options = (self.options & !0x000F) | (val & 0x000F);
    }

    // Java: short getOptions()
    pub fn getOptions(&self) -> i16 {
        // Java: return this.options;
        self.options
    }

    // Java: short getRecordId()
    pub fn getRecordId(&self) -> i16 {
        // Java: return this.recordId;
        self.recordId
    }

    // Java: int getRemainingBytes()
    pub fn getRemainingBytes(&self) -> i32 {
        // Java: return this.remainingBytes;
        self.remainingBytes
    }

    // Java: void setOptions(short options)
    pub fn setOptions(&mut self, options: i16) {
        // Java: this.options = options;
        self.options = options;
    }

    // Java: void setRecordId(short recordId)
    pub fn setRecordId(&mut self, record_id: i16) {
        // Java: this.recordId = recordId;
        self.recordId = record_id;
    }

    // Java: void setRemainingBytes(int remainingBytes)
    pub fn setRemainingBytes(&mut self, remaining_bytes: i32) {
        // Java: this.remainingBytes = remainingBytes;
        self.remainingBytes = remaining_bytes;
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, buf: &mut [u8], offset: i32) {
        // Java: LittleEndian.putShort(buf, offset, this.getOptions());
        le_put_short(buf, offset as usize, self.getOptions());
        // Java: LittleEndian.putShort(buf, offset + 2, this.getRecordId());
        le_put_short(buf, (offset + 2) as usize, self.getRecordId());
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "EscherRecordHeader{options=" + this.options + ", recordId=" + this.recordId + ", remainingBytes=" + this.remainingBytes + "}";
        format!("EscherRecordHeader{{options={}, recordId={}, remainingBytes={}}}", self.options, self.recordId, self.remainingBytes)
    }
}
