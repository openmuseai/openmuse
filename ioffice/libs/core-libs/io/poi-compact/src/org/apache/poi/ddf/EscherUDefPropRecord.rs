use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use crate::org::apache::poi::ddf::EscherSimpleProperty::EscherSimpleProperty;
// Java: import org.apache.poi.poifs.common.IBlockDocumentInputStream
use crate::org::apache::poi::poifs::common::IBlockDocumentInputStream::IBlockDocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.HexDump
use util::org::apache::poi::util::HexDump::HexDump;
// Java: import org.apache.poi.util.LittleEndian
use util::org::apache::poi::util::LittleEndian::LittleEndian;

pub use super::EscherRecord::EscherRecord;
pub use super::EscherRecordFactory::EscherRecordFactory;
pub use super::EscherProperty::EscherProperty;
pub use super::EscherPropertyFactory::{EscherPropertyFactory, EscherPropertyVariant};
pub use super::EscherSerializationListener::EscherSerializationListener;

// Java field: short RECORD_ID
pub const RECORD_ID: i16 = -3806;
// Java field: String RECORD_DESCRIPTION
pub const RECORD_DESCRIPTION: &'static str = "MsofbtUDefProp";
// Java field: EscherPropertyFactory f
static F: std::sync::OnceLock<EscherPropertyFactory> = std::sync::OnceLock::new();

// Java: public class EscherUDefPropRecord extends EscherRecord
pub struct EscherUDefPropRecord {
    // Java field: DocumentInputStream stream
    stream: Option<DocumentInputStream>,
    // Java field: TreeMap<Integer, EscherProperty> properties
    pub(crate) properties: BTreeMap<i32, EscherPropertyVariant>,
    // Java field: int propOffset
    pub(crate) propOffset: i32,
    // Base class fields from EscherRecord
    base: EscherRecord,
}

// Java: public class EscherUDefPropRecord extends EscherRecord
impl EscherUDefPropRecord {
    // Java: EscherUDefPropRecord()
    pub fn new() -> Self {
      // Java: super();
      EscherUDefPropRecord {
          stream: None,
          properties: BTreeMap::new(),
          propOffset: 0,
          base: EscherRecord::new(),
      }
    }

    // Java: int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory)
    pub fn fillFields(&mut self, data: &mut DocumentInputStream, offset: i32, _recordFactory: &dyn EscherRecordFactory) -> i32 {
      // Java: int bytesRemaining = this.readHeader(data, offset);
      let bytes_remaining = self.base.readHeader_stream(data, offset);
      // Java: int pos = offset + 8;
      let pos = offset + 8;
      // Java: EscherPropertyFactory f = new EscherPropertyFactory();
      let mut f = EscherPropertyFactory::new();
      // Java: this.properties = f.createProperties(data, pos, this.getInstance());
      self.properties = f.createProperties_stream(data, pos, self.base.getInstance());
      // Java: return bytesRemaining + 8;
      bytes_remaining + 8
    }

    // Java: int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory, boolean useEscherSpgrContainerRecord, boolean fillProps)
    pub fn fillFields_with_flags(&mut self, data: &mut DocumentInputStream, offset: i32, recordFactory: &dyn EscherRecordFactory, _use_escher_spgr_container_record: bool, fill_props: bool) -> i32 {
      // Java: if (fillProps)
      if fill_props {
         // Java: return this.fillFields(data, offset, recordFactory);
         self.fillFields(data, offset, recordFactory)
      }
      // Java: else
      else {
         // Java: this.stream = data;
         self.stream = Some(data.clone());
         // Java: this.propOffset = offset + 8;
         self.propOffset = offset + 8;
         // Java: int bytesRemaining = this.readHeader(data, offset);
         let bytes_remaining = self.base.readHeader_stream(data, offset);
         // Java: return bytesRemaining + 8;
         bytes_remaining + 8
      }
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, _offset: i32, _recordFactory: &dyn EscherRecordFactory, _picture_directory: String, _extension: String) -> i32 {
      // Java: int bytesRemaining = this.getRemainingBytes();
      let bytes_remaining = self.base.getRemainingBytes();
      // Java: byte[] thedata = new byte[bytesRemaining];
      let mut thedata = vec![0u8; bytes_remaining as usize];
      // Java: data.read(thedata);
      data.read(&mut thedata);
      // Java: EscherPropertyFactory f = new EscherPropertyFactory();
      let mut f = EscherPropertyFactory::new();
      // Java: this.properties = f.createProperties((byte[])thedata, 0, this.getInstance());
      self.properties = f.createProperties_bytes(&thedata, 0, self.base.getInstance());
      // Java: return bytesRemaining + 8;
      bytes_remaining + 8
    }

    // Java: int serialize(int offset, byte[] data, EscherSerializationListener listener)
    pub fn serialize(&mut self, offset: i32, data: &mut [u8], listener: &mut dyn EscherSerializationListener) -> i32 {
      // Java: listener.beforeRecordSerialize(offset, this.getRecordId(), this);
      listener.beforeRecordSerialize(offset, self.base.getRecordId(), &self.base);
      // Java: LittleEndian.putShort(data, offset, this.getOptions());
      let data_i8 = unsafe { &mut *(data as *mut [u8] as *mut [i8]) };
      LittleEndian::putShort(data_i8, offset, self.getOptions());
      // Java: LittleEndian.putShort(data, offset + 2, this.getRecordId());
      LittleEndian::putShort(data_i8, offset + 2, self.base.getRecordId());
      // Java: LittleEndian.putInt(data, offset + 4, this.getPropertiesSize());
      LittleEndian::putInt(data_i8, offset + 4, self.getPropertiesSize());
      // Java: int pos = offset + 8;
      let mut pos = offset + 8;
      // Java: for(Map.Entry<Integer, EscherProperty> entry : this.properties.entrySet())
      for (_key, escher_property) in &self.properties {
         // Java: EscherProperty escherProperty = (EscherProperty)entry.getValue();
         // Java: pos += escherProperty.serializeSimplePart(data, pos);
         pos += match escher_property {
            EscherPropertyVariant::Simple(p) => p.serializeSimplePart(data, pos),
            _ => 0,
         };
      }
      // Java: for(Map.Entry<Integer, EscherProperty> entry : this.properties.entrySet())
      for (_key, escher_property) in &self.properties {
         // Java: EscherProperty escherProperty = (EscherProperty)entry.getValue();
         // Java: pos += escherProperty.serializeComplexPart(data, pos);
         pos += match escher_property {
            EscherPropertyVariant::Simple(p) => p.serializeComplexPart(data, pos),
            _ => 0,
         };
      }
      // Java: listener.afterRecordSerialize(pos, this.getRecordId(), pos - offset, this);
      listener.afterRecordSerialize(pos, self.base.getRecordId(), pos - offset, &self.base);
      // Java: return pos - offset;
      pos - offset
    }

    // Java: void fillProps()
    pub fn fillProps(&mut self) {
      // Java: short numProperites = this.getInstance();
      let num_properites = self.base.getInstance();
      // Java: if (numProperites > 0 && this.properties.size() == 0)
      if num_properites > 0 && self.properties.is_empty() {
         // Java: this.properties = f.createProperties(this.stream, this.propOffset, numProperites);
         if let Some(ref mut stream) = self.stream {
             let f = F.get_or_init(|| EscherPropertyFactory::new());
             self.properties = f.createProperties_stream(stream, self.propOffset, num_properites);
         }
      }
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
      // Java: return 8 + this.getPropertiesSize();
      8 + self.getPropertiesSize()
    }

    // Java: int getPropOffset()
    pub fn getPropOffset(&self) -> i32 {
      // Java: return this.propOffset;
      self.propOffset
    }

    // Java: short getOptions()
    pub fn getOptions(&mut self) -> i16 {
      // Java: this.setOptions((short)(this.properties.size() << 4 | 3));
      self.base.setOptions((self.properties.len() as i16) << 4 | 3);
      // Java: return super.getOptions();
      self.base.getOptions()
    }

    // Java: String getRecordName()
    pub fn getRecordName(&self) -> String {
      // Java: return "UDefProp";
      String::from("UDefProp")
    }

    // Java: int getPropertiesSize()
    fn getPropertiesSize(&self) -> i32 {
      // Java: int totalSize = 0;
      let mut total_size = 0;
      // Java: for(Map.Entry<Integer, EscherProperty> entry : this.properties.entrySet())
      for (_key, escher_property) in &self.properties {
         // Java: EscherProperty escherProperty = (EscherProperty)entry.getValue();
         // Java: totalSize += escherProperty.getPropertySize();
         total_size += escher_property.getPropertySize();
      }
      // Java: return totalSize;
      total_size
    }

    // Java: String toString()
    pub fn toString(&mut self) -> String {
      // Java: String nl = System.getProperty("line.separator");
      let nl = "\n";
      // Java: StringBuffer propertiesBuf = new StringBuffer();
      let mut properties_buf = String::new();
      // Java: for(Map.Entry<Integer, EscherProperty> entry : this.properties.entrySet())
      for (_key, escher_property) in &self.properties {
         // Java: EscherProperty escherProperty = (EscherProperty)entry.getValue();
         // Java: propertiesBuf.append("    " + escherProperty.toString() + nl);
         properties_buf.push_str(&format!("    {:?}{}", escher_property, nl));
      }
      // Java: return "org.apache.poi.ddf.EscherOptRecord:" + nl + "  isContainer: " + this.isContainerRecord() + nl + "  options: 0x" + HexDump.toHex(this.getOptions()) + nl + "  recordId: 0x" + HexDump.toHex(this.getRecordId()) + nl + "  numchildren: " + this.getChildRecords().size() + nl + "  properties:" + nl + propertiesBuf.toString();
      format!("org.apache.poi.ddf.EscherOptRecord:{}  isContainer: {}{}  options: 0x{}{}  recordId: 0x{}{}  numchildren: {}{}  properties:{}{}",
          nl, self.base.isContainerRecord(), nl,
          HexDump::toHex_short(self.getOptions()), nl,
          HexDump::toHex_short(self.base.getRecordId()), nl,
          self.base.getChildRecords().len(), nl, nl,
          properties_buf)
    }

    // Java: public TreeMap<Integer, EscherProperty> getEscherProperties()
    pub fn getEscherProperties(&self) -> &BTreeMap<i32, EscherPropertyVariant> {
      // Java: return this.properties;
      &self.properties
    }

    // Java: EscherProperty getEscherPropertyById(int id)
    pub fn getEscherPropertyById(&self, id: i32) -> Option<&EscherPropertyVariant> {
      // Java: return (EscherProperty)this.properties.get(id);
      self.properties.get(&id)
    }

    // Java: void addEscherProperty(EscherProperty prop)
    pub fn addEscherProperty(&mut self, prop: EscherSimpleProperty) {
      // Java: this.properties.put(Integer.valueOf(prop.getPropertyNumber()), prop);
      self.properties.insert(prop.base.getPropertyNumber() as i32, EscherPropertyVariant::Simple(prop));
    }

    // Java: void sortProperties()
    pub fn sortProperties(&mut self) {
      // BTreeMap is already sorted by key, so this is a no-op in Rust
    }

}
