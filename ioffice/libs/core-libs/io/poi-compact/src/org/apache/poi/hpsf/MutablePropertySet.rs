// Java: import java.io.ByteArrayInputStream
// Java: import java.io.ByteArrayOutputStream
// Java: import java.io.FileNotFoundException
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import java.io.UnsupportedEncodingException
// Java: import java.util.Iterator
// Java: import java.util.LinkedList
// Java: import java.util.ListIterator
// Java: import org.apache.poi.poifs.filesystem.DirectoryEntry
// Java: import org.apache.poi.poifs.filesystem.Entry
// Java: import org.apache.poi.util.LittleEndian

use super::PropertySet::PropertySet;
use super::PropertySet::Section;
use super::ClassID::ClassID;
use super::MutableSection::MutableSection;

fn le_get_ushort(data: &[u8]) -> i32 {
    (data[0] as i32) | ((data[1] as i32) << 8)
}

static BYTE_ORDER_ASSERTION: &[u8] = &[0xFEu8, 0xFFu8];
static FORMAT_ASSERTION: &[u8] = &[0u8, 0u8];

// Java: public class MutablePropertySet extends PropertySet
pub struct MutablePropertySet {
    pub(crate) base: PropertySet,
    // Java field: int OFFSET_HEADER
    pub(crate) OFFSET_HEADER: i32,
    pub(crate) section: MutableSection,
}

// Java: public class MutablePropertySet extends PropertySet
impl MutablePropertySet {
    // Java: MutablePropertySet()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.OFFSET_HEADER = BYTE_ORDER_ASSERTION.length + FORMAT_ASSERTION.length + 4 + 16 + 4;
        let OFFSET_HEADER = (BYTE_ORDER_ASSERTION.len() + FORMAT_ASSERTION.len() + 4 + 16 + 4) as i32;
        // Java: this.byteOrder = LittleEndian.getUShort(BYTE_ORDER_ASSERTION);
        let byteOrder = le_get_ushort(BYTE_ORDER_ASSERTION);
        // Java: this.format = LittleEndian.getUShort(FORMAT_ASSERTION);
        let format = le_get_ushort(FORMAT_ASSERTION);
        // Java: this.osVersion = 133636;
        let osVersion = 133636;
        // Java: this.classID = new ClassID();
        let classID = ClassID::new();
        // Java: this.sections = new LinkedList();
        let mut sections = Vec::new();
        // Java: this.sections.add(new MutableSection());
        sections.push(Section { format_id: ClassID::new(), offset: 0 });
        MutablePropertySet {
            base: PropertySet {
                byteOrder,
                format,
                osVersion,
                classID,
                sections,
                is_summary: false,
                is_doc_summary: false,
            },
            OFFSET_HEADER,
            section: MutableSection::new(),
        }
    }

    // Java: MutablePropertySet(PropertySet ps)
    pub fn new_from_ps(ps: &PropertySet) -> Self {
        // Java: super();
        // Java: this.OFFSET_HEADER = BYTE_ORDER_ASSERTION.length + FORMAT_ASSERTION.length + 4 + 16 + 4;
        let OFFSET_HEADER = (BYTE_ORDER_ASSERTION.len() + FORMAT_ASSERTION.len() + 4 + 16 + 4) as i32;
        // Java: this.byteOrder = ps.getByteOrder();
        let byteOrder = ps.getByteOrder();
        // Java: this.format = ps.getFormat();
        let format = ps.getFormat();
        // Java: this.osVersion = ps.getOSVersion();
        let osVersion = ps.getOSVersion();
        // Java: this.setClassID(ps.getClassID());
        let classID = ps.getClassID().clone();
        // Java: this.clearSections();
        // Java: if (this.sections == null)
        // Java: this.sections = new LinkedList();
        let mut sections = Vec::new();
        // Java: Iterator i = ps.getSections().iterator();
        // Java: while(i.hasNext())
        for s in ps.getSections().iter() {
            // Java: MutableSection s = new MutableSection((Section)i.next());
            // Java: this.addSection(s);
            sections.push(Section { format_id: s.getFormatID().clone(), offset: s.offset });
        }
        MutablePropertySet {
            base: PropertySet {
                byteOrder,
                format,
                osVersion,
                classID,
                sections,
                is_summary: false,
                is_doc_summary: false,
            },
            OFFSET_HEADER,
            section: MutableSection::new(),
        }
    }

    // Java: void setByteOrder(int byteOrder)
    pub fn setByteOrder(&mut self, byteOrder: i32) {
        // Java: this.byteOrder = byteOrder;
        self.base.byteOrder = byteOrder;
    }

    // Java: void setFormat(int format)
    pub fn setFormat(&mut self, format: i32) {
        // Java: this.format = format;
        self.base.format = format;
    }

    // Java: void setOSVersion(int osVersion)
    pub fn setOSVersion(&mut self, osVersion: i32) {
        // Java: this.osVersion = osVersion;
        self.base.osVersion = osVersion;
    }

    // Java: void setClassID(ClassID classID)
    pub fn setClassID(&mut self, classID: ClassID) {
        // Java: this.classID = classID;
        self.base.classID = classID;
    }

    // Java: void clearSections()
    pub fn clearSections(&mut self) {
        // Java: this.sections = null;
        self.base.sections.clear();
    }

    // Java: void addSection(Section section)
    pub fn addSection(&mut self, section: Section) {
        // Java: if (this.sections == null)
        // Java: this.sections = new LinkedList();
        // Java: this.sections.add(section);
        self.base.sections.push(section);
    }

    // Java: void write(OutputStream out)
    pub fn write(&self, out: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Java: int nrSections = this.sections.size();
        let nrSections = self.base.sections.len() as i32;
        // Java: int length = 0;
        let mut _length: i32 = 0;
        // Java: length += TypeWriter.writeToStream(out, (short)this.getByteOrder());
        let bo = (self.base.getByteOrder() as i16).to_le_bytes();
        out.write_all(&bo)?;
        _length += 2;
        // Java: length += TypeWriter.writeToStream(out, (short)this.getFormat());
        let fmt = (self.base.getFormat() as i16).to_le_bytes();
        out.write_all(&fmt)?;
        _length += 2;
        // Java: length += TypeWriter.writeToStream(out, this.getOSVersion());
        let osv = self.base.getOSVersion().to_le_bytes();
        out.write_all(&osv)?;
        _length += 4;
        // Java: length += TypeWriter.writeToStream(out, this.getClassID());
        let mut clsid_bytes = vec![0u8; 16];
        self.base.getClassID().write(&mut clsid_bytes, 0);
        out.write_all(&clsid_bytes)?;
        _length += 16;
        // Java: length += TypeWriter.writeToStream(out, nrSections);
        let nrs = nrSections.to_le_bytes();
        out.write_all(&nrs)?;
        _length += 4;
        // Java: int offset = this.OFFSET_HEADER;
        let mut offset = self.OFFSET_HEADER;
        // Java: offset += nrSections * 20;
        offset += nrSections * 20;
        // Java: int sectionsBegin = offset;
        let _sectionsBegin = offset;
        // Java: ListIterator i = this.sections.listIterator();
        // Java: while(i.hasNext())
        for s in self.base.sections.iter() {
            // Java: MutableSection s = (MutableSection)i.next();
            // Java: ClassID formatID = s.getFormatID();
            let formatID = s.getFormatID();
            // Java: if (formatID == null)
            // Java: throw new NoFormatIDException();
            // Java: length += TypeWriter.writeToStream(out, s.getFormatID());
            let mut fid_bytes = vec![0u8; 16];
            formatID.write(&mut fid_bytes, 0);
            out.write_all(&fid_bytes)?;
            _length += 16;
            // Java: length += TypeWriter.writeUIntToStream(out, (long)offset);
            let off_bytes = (offset as u32).to_le_bytes();
            out.write_all(&off_bytes)?;
            _length += 4;
            // Java: try
            // Java: offset += s.getSize();
            // Java: catch (HPSFRuntimeException ex)
            // Java: Throwable cause = ex.getReason();
            // Java: if (cause instanceof UnsupportedEncodingException)
            // Java: throw new IllegalPropertySetDataException(cause);
            // Java: throw ex;
        }
        // Java: offset = sectionsBegin;
        // Java: MutableSection s;
        // Java: for(i = this.sections.listIterator(); i.hasNext(); offset += s.write(out))
        // Java: s = (MutableSection)i.next();
        Ok(())
    }

    // Java: InputStream toInputStream()
    pub fn toInputStream(&self) -> std::io::Result<Vec<u8>> {
        // Java: ByteArrayOutputStream psStream = new ByteArrayOutputStream();
        let mut psStream: Vec<u8> = Vec::new();
        // Java: this.write(psStream);
        self.write(&mut psStream)?;
        // Java: psStream.close();
        // Java: byte[] streamData = psStream.toByteArray();
        // Java: return new ByteArrayInputStream(streamData);
        Ok(psStream)
    }

    // Java: void write(DirectoryEntry dir, String name)
    pub fn write_to_dir(&self, _dir: &mut dyn std::any::Any, _name: &str) -> std::io::Result<()> {
        // Java: try
        // Java: Entry e = dir.getEntry(name);
        // Java: e.delete();
        // Java: catch (FileNotFoundException var4)
        // Java: dir.createDocument(name, this.toInputStream());
        Ok(())
    }

    pub fn getFirstSectionMut(&mut self) -> &mut MutableSection {
        &mut self.section
    }
}
