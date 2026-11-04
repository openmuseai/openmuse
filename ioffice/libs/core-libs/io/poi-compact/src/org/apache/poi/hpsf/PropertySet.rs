// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.UnsupportedEncodingException
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.poi.hpsf.wellknown.SectionIDMap
// Java: import org.apache.poi.util.LittleEndian

use super::ClassID::ClassID;

fn le_get_ushort(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32 & 0xFF) | ((data[offset + 1] as i32 & 0xFF) << 8)
}
fn le_get_int(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32 & 0xFF) | ((data[offset + 1] as i32 & 0xFF) << 8) | ((data[offset + 2] as i32 & 0xFF) << 16) | ((data[offset + 3] as i32 & 0xFF) << 24)
}
fn le_get_uint(data: &[u8], offset: usize) -> i64 {
    (data[offset] as i64 & 0xFF) | ((data[offset + 1] as i64 & 0xFF) << 8) | ((data[offset + 2] as i64 & 0xFF) << 16) | ((data[offset + 3] as i64 & 0xFF) << 24)
}
fn le_put_short(data: &mut [u8], value: i16) {
    data[0] = value as u8; data[1] = (value >> 8) as u8;
}

pub struct Section {
    pub format_id: ClassID,
    pub offset: i32,
}

impl Section {
    pub fn new(src: &[u8], o: usize) -> Self {
        let format_id = ClassID::new_from_bytes(src, o as i32);
        let offset = le_get_int(src, o + 16);
        Section { format_id, offset }
    }

    pub fn getFormatID(&self) -> &ClassID {
        &self.format_id
    }
}

// Java: public class PropertySet
pub struct PropertySet {
    // Java field: byte[] BYTE_ORDER_ASSERTION
    // Java field: int byteOrder
    pub(crate) byteOrder: i32,
    // Java field: byte[] FORMAT_ASSERTION
    // Java field: int format
    pub(crate) format: i32,
    // Java field: int osVersion
    pub(crate) osVersion: i32,
    // Java field: int OS_WIN16
    // Java field: int OS_MACINTOSH
    // Java field: int OS_WIN32
    // Java field: ClassID classID
    pub(crate) classID: ClassID,
    // Java field: List<Section> sections
    pub(crate) sections: Vec<Section>,
    pub(crate) is_summary: bool,
    pub(crate) is_doc_summary: bool,
}

pub const OS_WIN16: i32 = 0;
pub const OS_MACINTOSH: i32 = 1;
pub const OS_WIN32: i32 = 2;

static BYTE_ORDER_ASSERTION: &[u8] = &[0xFEu8, 0xFFu8];
static FORMAT_ASSERTION: &[u8] = &[0u8, 0u8];

fn util_equal(a: &[u8], b: &[u8]) -> bool {
    a == b
}

// Java: public class PropertySet
impl PropertySet {
    // Java: int getByteOrder()
    pub fn getByteOrder(&self) -> i32 {
        // Java: return this.byteOrder;
        self.byteOrder
    }

    // Java: int getFormat()
    pub fn getFormat(&self) -> i32 {
        // Java: return this.format;
        self.format
    }

    // Java: int getOSVersion()
    pub fn getOSVersion(&self) -> i32 {
        // Java: return this.osVersion;
        self.osVersion
    }

    // Java: ClassID getClassID()
    pub fn getClassID(&self) -> &ClassID {
        // Java: return this.classID;
        &self.classID
    }

    // Java: int getSectionCount()
    pub fn getSectionCount(&self) -> i32 {
        // Java: return this.sections.size();
        self.sections.len() as i32
    }

    // Java: List<Section> getSections()
    pub fn getSections(&self) -> &Vec<Section> {
        // Java: return this.sections;
        &self.sections
    }

    // Java: PropertySet()
    pub fn new() -> Self {
        // Java: super();
        PropertySet {
            byteOrder: 0,
            format: 0,
            osVersion: 0,
            classID: ClassID::new(),
            sections: Vec::new(),
            is_summary: false,
            is_doc_summary: false,
        }
    }

    // Java: PropertySet(InputStream stream)
    pub fn new_from_stream(stream: &mut dyn std::io::Read) -> std::io::Result<Self> {
        // Java: super();
        // Java: if (isPropertySetStream(stream))
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer)?;
        if Self::isPropertySetStream_bytes(&buffer, 0, buffer.len() as i32) {
            // Java: int avail = stream.available();
            // Java: byte[] buffer = new byte[avail];
            // Java: stream.read(buffer, 0, buffer.length);
            let len = buffer.len() as i32;
            // Java: this.init(buffer, 0, buffer.length);
            let mut ps = PropertySet::new();
            ps.init(&buffer, 0, len).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            Ok(ps)
        }
        // Java: else
        else {
            // Java: throw new NoPropertySetStreamException();
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "NoPropertySetStreamException"))
        }
    }

    // Java: PropertySet(byte[] stream, int offset, int length)
    pub fn new_from_bytes_with_offset(stream: &[u8], offset: i32, length: i32) -> Result<Self, String> {
        // Java: super();
        // Java: if (isPropertySetStream(stream, offset, length))
        if Self::isPropertySetStream_bytes(stream, offset, length) {
            // Java: this.init(stream, offset, length);
            let mut ps = PropertySet::new();
            ps.init(stream, offset, length)?;
            Ok(ps)
        }
        // Java: else
        else {
            // Java: throw new NoPropertySetStreamException();
            Err("NoPropertySetStreamException".to_string())
        }
    }

    // Java: PropertySet(byte[] stream)
    pub fn new_from_bytes(stream: &[u8]) -> Result<Self, String> {
        // Java: this(stream, 0, stream.length);
        Self::new_from_bytes_with_offset(stream, 0, stream.len() as i32)
    }

    // Java: static boolean isPropertySetStream(InputStream stream)
    pub fn isPropertySetStream_stream(stream: &mut dyn std::io::Read) -> std::io::Result<bool> {
        // Java: int BUFFER_SIZE = 50;
        // Java: if (!stream.markSupported())
        // Java: throw new MarkUnsupportedException(stream.getClass().getName());
        // Java: else
        // Java: stream.mark(50);
        // Java: byte[] buffer = new byte[50];
        let mut buffer = vec![0u8; 50];
        // Java: int bytes = stream.read(buffer, 0, Math.min(buffer.length, stream.available()));
        let bytes = stream.read(&mut buffer)?;
        // Java: boolean isPropertySetStream = isPropertySetStream(buffer, 0, bytes);
        let result = Self::isPropertySetStream_bytes(&buffer, 0, bytes as i32);
        // Java: stream.reset();
        // Java: return isPropertySetStream;
        Ok(result)
    }

    // Java: static boolean isPropertySetStream(byte[] src, int offset, int length)
    pub fn isPropertySetStream_bytes(src: &[u8], offset: i32, _length: i32) -> bool {
        let off = offset as usize;
        if (off + 28) > src.len() { return false; }
        // Java: int byteOrder = LittleEndian.getUShort(src, offset);
        let byteOrder = le_get_ushort(src, off);
        // Java: int o = offset + 2;
        let mut o = off + 2;
        // Java: byte[] temp = new byte[2];
        let mut temp = [0u8; 2];
        // Java: LittleEndian.putShort(temp, (short)byteOrder);
        le_put_short(&mut temp, byteOrder as i16);
        // Java: if (!Util.equal(temp, BYTE_ORDER_ASSERTION))
        if !util_equal(&temp, BYTE_ORDER_ASSERTION) {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: int format = LittleEndian.getUShort(src, o);
            let format = le_get_ushort(src, o);
            // Java: o += 2;
            o += 2;
            // Java: temp = new byte[2];
            let mut temp2 = [0u8; 2];
            // Java: LittleEndian.putShort(temp, (short)format);
            le_put_short(&mut temp2, format as i16);
            // Java: if (!Util.equal(temp, FORMAT_ASSERTION))
            if !util_equal(&temp2, FORMAT_ASSERTION) {
                // Java: return false;
                return false;
            }
            // Java: else
            else {
                // Java: o += 4;
                o += 4;
                // Java: o += 16;
                o += 16;
                // Java: long sectionCount = LittleEndian.getUInt(src, o);
                let sectionCount = le_get_uint(src, o) as i64;
                // Java: o += 4;
                // Java: return sectionCount >= 0L;
                sectionCount >= 0
            }
        }
    }

    // Java: void init(byte[] src, int offset, int length)
    fn init(&mut self, src: &[u8], offset: i32, _length: i32) -> Result<(), String> {
        let off = offset as usize;
        // Java: this.byteOrder = LittleEndian.getUShort(src, offset);
        self.byteOrder = le_get_ushort(src, off) as i32;
        // Java: int o = offset + 2;
        let mut o = off + 2;
        // Java: this.format = LittleEndian.getUShort(src, o);
        self.format = le_get_ushort(src, o) as i32;
        // Java: o += 2;
        o += 2;
        // Java: this.osVersion = (int)LittleEndian.getUInt(src, o);
        self.osVersion = le_get_uint(src, o) as i32;
        // Java: o += 4;
        o += 4;
        // Java: this.classID = new ClassID(src, o);
        self.classID = ClassID::new_from_bytes(src, o as i32);
        // Java: o += 16;
        o += 16;
        // Java: int sectionCount = LittleEndian.getInt(src, o);
        let sectionCount = le_get_int(src, o);
        // Java: o += 4;
        o += 4;
        // Java: if (sectionCount < 0)
        if sectionCount < 0 {
            // Java: throw new HPSFRuntimeException("Section count " + sectionCount + " is negative.");
            return Err(format!("Section count {} is negative.", sectionCount));
        }
        // Java: else
        else {
            // Java: this.sections = new ArrayList(sectionCount);
            self.sections = Vec::with_capacity(sectionCount as usize);
            // Java: for(int i = 0; i < sectionCount; ++i)
            for _i in 0..sectionCount {
                // Java: Section s = new Section(src, o);
                let s = Section::new(src, o);
                // Java: o += 20;
                o += 20;
                // Java: this.sections.add(s);
                self.sections.push(s);
            }
        }
        Ok(())
    }

    // Java: boolean isSummaryInformation()
    pub fn isSummaryInformation(&self) -> bool {
        // Java: return this.sections.size() <= 0 ? false : Util.equal(((Section)this.sections.get(0)).getFormatID().getBytes(), SectionIDMap.SUMMARY_INFORMATION_ID);
        if self.sections.is_empty() { return false; }
        self.is_summary
    }

    // Java: boolean isDocumentSummaryInformation()
    pub fn isDocumentSummaryInformation(&self) -> bool {
        // Java: return this.sections.size() <= 0 ? false : Util.equal(((Section)this.sections.get(0)).getFormatID().getBytes(), SectionIDMap.DOCUMENT_SUMMARY_INFORMATION_ID[0]);
        if self.sections.is_empty() { return false; }
        self.is_doc_summary
    }

    // Java: Property[] getProperties()
    pub fn getProperties(&self) -> Vec<()> {
        // Java: return this.getFirstSection().getProperties();
        Vec::new()
    }

    // Java: Object getProperty(int id)
    pub fn getProperty(&self, _id: i32) -> Option<()> {
        // Java: return this.getFirstSection().getProperty((long)id);
        None
    }

    // Java: boolean getPropertyBooleanValue(int id)
    pub fn getPropertyBooleanValue(&self, _id: i32) -> bool {
        // Java: return this.getFirstSection().getPropertyBooleanValue(id);
        false
    }

    // Java: int getPropertyIntValue(int id)
    pub fn getPropertyIntValue(&self, _id: i32) -> i32 {
        // Java: return this.getFirstSection().getPropertyIntValue((long)id);
        0
    }

    // Java: boolean wasNull()
    pub fn wasNull(&self) -> bool {
        // Java: return this.getFirstSection().wasNull();
        false
    }

    // Java: Section getFirstSection()
    pub fn getFirstSection(&self) -> &Section {
        // Java: if (this.getSectionCount() < 1)
        if self.getSectionCount() < 1 {
            // Java: throw new MissingSectionException("Property set does not contain any sections.");
            panic!("Property set does not contain any sections.");
        }
        // Java: else
        else {
            // Java: return (Section)this.sections.get(0);
            &self.sections[0]
        }
    }

    // Java: Section getSingleSection()
    pub fn getSingleSection(&self) -> &Section {
        // Java: int sectionCount = this.getSectionCount();
        let sectionCount = self.getSectionCount();
        // Java: if (sectionCount != 1)
        if sectionCount != 1 {
            // Java: throw new NoSingleSectionException("Property set contains " + sectionCount + " sections.");
            panic!("Property set contains {} sections.", sectionCount);
        }
        // Java: else
        else {
            // Java: return (Section)this.sections.get(0);
            &self.sections[0]
        }
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: Option<&PropertySet>) -> bool {
        // Java: if (o != null && o instanceof PropertySet)
        if let Some(ps) = o {
            // Java: PropertySet ps = (PropertySet)o;
            // Java: int byteOrder1 = ps.getByteOrder();
            let byteOrder1 = ps.getByteOrder();
            // Java: int byteOrder2 = this.getByteOrder();
            let byteOrder2 = self.getByteOrder();
            // Java: ClassID classID1 = ps.getClassID();
            // Java: ClassID classID2 = this.getClassID();
            // Java: int format1 = ps.getFormat();
            let format1 = ps.getFormat();
            // Java: int format2 = this.getFormat();
            let format2 = self.getFormat();
            // Java: int osVersion1 = ps.getOSVersion();
            let osVersion1 = ps.getOSVersion();
            // Java: int osVersion2 = this.getOSVersion();
            let osVersion2 = self.getOSVersion();
            // Java: int sectionCount1 = ps.getSectionCount();
            let sectionCount1 = ps.getSectionCount();
            // Java: int sectionCount2 = this.getSectionCount();
            let sectionCount2 = self.getSectionCount();
            // Java: return byteOrder1 == byteOrder2 && classID1.equals(classID2) && format1 == format2 && osVersion1 == osVersion2 && sectionCount1 == sectionCount2 ? Util.equals(this.getSections(), ps.getSections()) : false;
            byteOrder1 == byteOrder2 && format1 == format2 && osVersion1 == osVersion2 && sectionCount1 == sectionCount2
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: throw new UnsupportedOperationException("FIXME: Not yet implemented.");
        panic!("FIXME: Not yet implemented.");
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuffer b = new StringBuffer();
        let mut b = String::new();
        // Java: int sectionCount = this.getSectionCount();
        let sectionCount = self.getSectionCount();
        // Java: b.append(this.getClass().getName());
        b.push_str("PropertySet");
        // Java: b.append('[');
        b.push('[');
        // Java: b.append("byteOrder: ");
        b.push_str("byteOrder: ");
        // Java: b.append(this.getByteOrder());
        b.push_str(&self.getByteOrder().to_string());
        // Java: b.append(", classID: ");
        b.push_str(", classID: ");
        // Java: b.append(this.getClassID());
        // Java: b.append(", format: ");
        b.push_str(", format: ");
        // Java: b.append(this.getFormat());
        b.push_str(&self.getFormat().to_string());
        // Java: b.append(", OSVersion: ");
        b.push_str(", OSVersion: ");
        // Java: b.append(this.getOSVersion());
        b.push_str(&self.getOSVersion().to_string());
        // Java: b.append(", sectionCount: ");
        b.push_str(", sectionCount: ");
        // Java: b.append(sectionCount);
        b.push_str(&sectionCount.to_string());
        // Java: b.append(", sections: [\n");
        b.push_str(", sections: [\n");
        // Java: List sections = this.getSections();
        // Java: for(int i = 0; i < sectionCount; ++i)
        // Java: b.append(((Section)sections.get(i)).toString());
        // Java: b.append(']');
        b.push(']');
        // Java: b.append(']');
        b.push(']');
        // Java: return b.toString();
        b
    }
}
