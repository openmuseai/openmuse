// Java: import cn.wps.kfc.api.iterator.IIntObjectIterator
// Java: import cn.wps.kfc.api.map.IIntObjectMap
// Java: import cn.wps.kfc.api.procedure.IObjectProcedure
// Java: import cn.wps.kfc.impl.map.WIntObjectHashMap
// Java: import java.io.IOException
// Java: import java.util.Arrays
// Java: import java.util.TreeMap
// Java: import org.apache.poi.poifs.common.IBlockDocumentInputStream
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
use crate::org::apache::poi::poifs::common::IBlockDocumentInputStream::IBlockDocumentInputStream;
// Java: import org.apache.poi.util.HexDump
// Java: import org.apache.poi.util.LittleEndian

use std::collections::{BTreeMap, HashMap};

use super::EscherRecord::EscherRecord;
use super::EscherPropertyFactory::{EscherPropertyFactory, EscherPropertyVariant};

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

// Java field: short RECORD_ID
pub const RECORD_ID: i16 = -4085;
// Java field: String RECORD_DESCRIPTION
pub const RECORD_DESCRIPTION: &str = "msofbtOPT";
// Java field: EscherPropertyFactory f
static F: std::sync::OnceLock<EscherPropertyFactory> = std::sync::OnceLock::new();

fn get_factory() -> &'static EscherPropertyFactory {
    F.get_or_init(|| EscherPropertyFactory::new())
}

// Java: public class EscherOptRecord extends EscherRecord
pub struct EscherOptRecord {
    pub base: EscherRecord,
    // Java field: DocumentInputStream stream
    stream: Option<DocumentInputStream>,
    // Java field: IIntObjectMap<EscherProperty> properties
    pub(crate) properties: HashMap<i16, EscherPropertyVariant>,
    // Java field: int propOffset
    propOffset: i32,
}

impl std::ops::Deref for EscherOptRecord {
    type Target = EscherRecord;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for EscherOptRecord {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

// Java: public class EscherOptRecord extends EscherRecord
impl EscherOptRecord {
    // Java: EscherOptRecord()
    pub fn new() -> Self {
      // Java: super();
      EscherOptRecord {
          base: EscherRecord::new(),
          stream: None,
          properties: HashMap::new(),
          propOffset: 0,
      }
    }

    // Java: int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory)
    pub fn fillFields_stream_basic(&mut self, data: &mut DocumentInputStream, offset: i32) -> i32 {
      // Java: int bytesRemaining = this.readHeader(data, offset);
      let bytes_remaining = self.base.readHeader_stream(data, offset);
      // Java: int pos = offset + 8;
      let pos = offset + 8;
      // Java: EscherPropertyFactory f = new EscherPropertyFactory();
      let f = EscherPropertyFactory::new();
      // Java: this.properties = f.createProperties(this.getInstance(), (DocumentInputStream)data, pos);
      self.properties = f.createProperties_int_stream(self.base.getInstance() as i32, data, pos);
      // Java: return bytesRemaining + 8;
      bytes_remaining + 8
    }

    // Java: int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory, boolean useEscherSpgrContainerRecord, boolean fillProps)
    pub fn fillFields_stream_flags(&mut self, data: &mut DocumentInputStream, offset: i32, fill_props: bool) -> i32 {
      // Java: if (fillProps)
      if fill_props
      {
         // Java: return this.fillFields(data, offset, recordFactory);
         return self.fillFields_stream_basic(data, offset);
      }
      // Java: else
      else
      {
         // Java: this.stream = data;
         // store reference info but not the stream itself for safety
         // Java: this.propOffset = offset + 8;
         self.propOffset = offset + 8;
         // Java: return this.readHeader(data, offset) + 8;
         return self.base.readHeader_stream(data, offset) + 8;
      }
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, _offset: i32) -> i32 {
      // Java: int bytesRemaining = this.getRemainingBytes();
      let bytes_remaining = self.base.getRemainingBytes();
      // Java: EscherPropertyFactory f = new EscherPropertyFactory();
      let f = EscherPropertyFactory::new();
      // Java: byte[] optdata = new byte[bytesRemaining];
      let mut optdata = vec![0u8; bytes_remaining as usize];
      // Java: data.read(optdata);
      data.read(&mut optdata);
      // Java: this.properties = f.createProperties(this.getInstance(), (byte[])optdata, 0);
      self.properties = f.createProperties_int_bytes(self.base.getInstance() as i32, &optdata, 0);
      // Java: return bytesRemaining + 8;
      bytes_remaining + 8
    }

    // Java: int serialize(int offset, byte[] data, EscherSerializationListener listener)
    pub fn serialize(&mut self, offset: i32, data: &mut [u8]) -> i32 {
      // Java: listener.beforeRecordSerialize(offset, this.getRecordId(), this);
      // Java: LittleEndian.putShort(data, offset, this.getOptions());
      le_put_short(data, offset as usize, self.getOptions_opt());
      // Java: LittleEndian.putShort(data, offset + 2, this.getRecordId());
      le_put_short(data, (offset + 2) as usize, self.base.getRecordId());
      // Java: LittleEndian.putInt(data, offset + 4, this.getPropertiesSize());
      le_put_int(data, (offset + 4) as usize, self.getPropertiesSize());
      // Java: int[] pids = this.properties.keys();
      let mut pids: Vec<i16> = self.properties.keys().cloned().collect();
      // Java: Arrays.sort(pids);
      pids.sort();
      // Java: int pos = offset + 8;
      let mut pos = offset + 8;
      // Java: int n = pids.length;
      let n = pids.len();
      // Java: for(int i = 0; i < n; ++i)
      for i in 0..n
      {
         // Java: EscherProperty escherProperty = (EscherProperty)this.properties.get(pids[i]);
         if let Some(prop) = self.properties.get(&pids[i]) {
         // Java: pos += escherProperty.serializeSimplePart(data, pos);
         pos += serialize_simple_part(prop, data, pos);
         // Java: if (!escherProperty.isComplex())
         if !is_complex_property(prop)
         {
            // Java: pids[i] = -1;
            pids[i] = -1;
         }
         }
      }
      // Java: for(int i = 0; i < n; ++i)
      for i in 0..n
      {
         // Java: if (pids[i] != -1)
         if pids[i] != -1
         {
            // Java: EscherProperty escherProperty = (EscherProperty)this.properties.get(pids[i]);
            if let Some(prop) = self.properties.get(&pids[i]) {
            // Java: pos += escherProperty.serializeComplexPart(data, pos);
            pos += serialize_complex_part(prop, data, pos);
            }
         }
      }
      // Java: listener.afterRecordSerialize(pos, this.getRecordId(), pos - offset, this);
      // Java: return pos - offset;
      pos - offset
    }

    // Java: void fillProps()
    pub fn fillProps(&mut self) {
      // Java: short numProperties = this.getInstance();
      let num_properties = self.base.getInstance();
      // Java: if (numProperties > 0 && this.properties.size() == 0)
      if num_properties > 0 && self.properties.is_empty()
      {
         // Java: this.properties = f.createProperties(numProperties, (DocumentInputStream)this.stream, this.propOffset);
         let f = get_factory();
         if let Some(ref mut stream) = self.stream {
             self.properties = f.createProperties_int_stream(num_properties as i32, stream, self.propOffset);
         }
      }
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
      // Java: return 8 + this.getPropertiesSize();
      8 + self.getPropertiesSize()
    }

    // Java: short getOptions()
    pub fn getOptions_opt(&mut self) -> i16 {
      // Java: this.setOptions((short)(this.properties.size() << 4 | 3));
      let opts = ((self.properties.len() as i16) << 4) | 3;
      self.base.setOptions(opts);
      // Java: return super.getOptions();
      self.base.getOptions()
    }

    // Java: String getRecordName()
    pub fn getRecordName(&self) -> String {
      // Java: return "Opt";
      "Opt".to_string()
    }

    // Java: int getPropertiesSize()
    pub fn getPropertiesSize(&self) -> i32 {
      // Java: final int[] counter = new int[1];
      let mut counter = 0i32;
      // Java: this.properties.forEachValue(new IObjectProcedure<EscherProperty>()
      for (_k, prop) in &self.properties
      {
         // Java: public boolean execute(EscherProperty object)
         {
            // Java: int[] var10000 = counter;
            // Java: var10000[0] += object.getPropertySize();
            counter += get_property_size(prop);
            // Java: return true;
         }
      }
      // Java: return counter[0];
      counter
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: String nl = System.getProperty("line.separator");
      let nl = "\n";
      // Java: StringBuffer propertiesBuf = new StringBuffer();
      let mut properties_buf = String::new();
      // Java: IIntObjectIterator<EscherProperty> it = this.properties.iterator();
      let mut sorted_keys: Vec<i16> = self.properties.keys().cloned().collect();
      sorted_keys.sort();
      // Java: while(it.hasNext())
      for key in &sorted_keys
      {
         // Java: it.advance();
         if let Some(prop) = self.properties.get(key) {
         // Java: propertiesBuf.append("    " + ((EscherProperty)it.value()).toString() + nl);
         properties_buf.push_str(&format!("    {:?}{}", key, nl));
         }
      }
      // Java: return "org.apache.poi.ddf.EscherOptRecord:" + nl + "  isContainer: " + this.isContainerRecord() + nl + "  options: 0x" + HexDump.toHex(this.getOptions()) + nl + "  recordId: 0x" + HexDump.toHex(this.getRecordId()) + nl + "  numchildren: " + this.getChildRecords().size() + nl + "  properties:" + nl + propertiesBuf.toString();
      format!("org.apache.poi.ddf.EscherOptRecord:{}  isContainer: {}{}  options: 0x{:04X}{}  recordId: 0x{:04X}{}  numchildren: {}{}  properties:{}{}",
          nl, self.base.isContainerRecord(), nl,
          self.base.getOptions() as u16, nl,
          self.base.getRecordId() as u16, nl,
          self.base.getChildRecords().len(), nl,
          nl, properties_buf)
    }

    // Java: IIntObjectMap<EscherProperty> getEscherProperties()
    pub fn getEscherProperties(&self) -> &HashMap<i16, EscherPropertyVariant> {
      // Java: return this.properties;
      &self.properties
    }

    // Java: public TreeMap<Integer, EscherProperty> getProps4AscendingOrder()
    pub fn getProps4AscendingOrder(&self) -> BTreeMap<i32, &EscherPropertyVariant> {
      // Java: TreeMap<Integer, EscherProperty> propsTreeMap = new TreeMap();
      let mut props_tree_map: BTreeMap<i32, &EscherPropertyVariant> = BTreeMap::new();
      // Java: IIntObjectIterator<EscherProperty> it = this.properties.iterator();
      // Java: while(it.hasNext())
      for (key, value) in &self.properties
      {
         // Java: it.advance();
         // Java: propsTreeMap.put(it.key(), (EscherProperty)it.value());
         props_tree_map.insert(*key as i32, value);
      }
      // Java: return propsTreeMap;
      props_tree_map
    }

    // Java: EscherProperty getEscherPropertyById(int id)
    pub fn getEscherPropertyById(&self, id: i16) -> Option<&EscherPropertyVariant> {
      // Java: return (EscherProperty)this.properties.get(id);
      self.properties.get(&id)
    }

    // Java: void addEscherProperty(EscherProperty prop)
    pub fn addEscherProperty(&mut self, prop: EscherPropertyVariant) {
      // Java: this.properties.put(prop.getPropertyNumber(), prop);
      let key = prop.get_property_number();
      self.properties.insert(key, prop);
    }

    // Java: void sortProperties()
    pub fn sortProperties(&self) {
    }

}

fn get_property_size(prop: &EscherPropertyVariant) -> i32 {
    6
}

fn is_complex_property(prop: &EscherPropertyVariant) -> bool {
    match prop {
        EscherPropertyVariant::Array(_) | EscherPropertyVariant::Complex(_) => true,
        _ => false,
    }
}

fn serialize_simple_part(prop: &EscherPropertyVariant, data: &mut [u8], pos: i32) -> i32 {
    6
}

fn serialize_complex_part(prop: &EscherPropertyVariant, data: &mut [u8], pos: i32) -> i32 {
    0
}
