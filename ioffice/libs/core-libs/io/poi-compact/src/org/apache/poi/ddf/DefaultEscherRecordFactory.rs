// Java: import java.lang.reflect.Constructor
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Map
use std::sync::OnceLock;
use std::sync::Mutex;
// Java: import org.apache.poi.poifs.common.IBlockDocumentInputStream
use crate::org::apache::poi::poifs::common::IBlockDocumentInputStream::IBlockDocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;

use super::EscherRecord::{EscherRecord, EscherRecordHeader};
use super::EscherRecordFactory::EscherRecordFactory;

type RecordFactory = fn() -> EscherRecord;

// Java field: Class<?>[] escherRecordClasses
// Java field: Map<Short, Constructor<? extends EscherRecord>> recordsMap
static RECORDS_MAP: OnceLock<Mutex<HashMap<i16, RecordFactory>>> = OnceLock::new();

fn default_record_factory() -> EscherRecord { EscherRecord::new() }

fn get_records_map() -> &'static Mutex<HashMap<i16, RecordFactory>> {
    RECORDS_MAP.get_or_init(|| {
        let mut map: HashMap<i16, RecordFactory> = HashMap::new();
        map.insert(-4089, default_record_factory); // EscherBSERecord
        map.insert(-4085, default_record_factory); // EscherOptRecord
        map.insert(-4080, default_record_factory); // EscherClientAnchorRecord
        map.insert(-4088, default_record_factory); // EscherDgRecord
        map.insert(-4087, default_record_factory); // EscherSpgrRecord
        map.insert(-4086, default_record_factory); // EscherSpRecord
        map.insert(-4079, default_record_factory); // EscherClientDataRecord
        map.insert(-4090, default_record_factory); // EscherDggRecord
        map.insert(-3810, default_record_factory); // EscherSplitMenuColorsRecord
        map.insert(-4081, default_record_factory); // EscherChildAnchorRecord
        map.insert(-4083, default_record_factory); // EscherTextboxRecord
        map.insert(-3806, default_record_factory); // EscherUDefPropRecord
        Mutex::new(map)
    })
}

// Java: public class DefaultEscherRecordFactory implements EscherRecordFactory
pub struct DefaultEscherRecordFactory {
}

// Java: public class DefaultEscherRecordFactory implements EscherRecordFactory
impl DefaultEscherRecordFactory {
    // Java: DefaultEscherRecordFactory()
    pub fn new() -> Self {
      // Java: super();
      DefaultEscherRecordFactory {}
    }

    // Java: EscherRecord createRecord(short id)
    pub fn createRecord_short(&self, id: i16) -> EscherRecord {
      // Java: if (id != -4095 && id != -4096 && id != -4094 && id != -4093 && id != -4092 && id != -4091)
        if id != -4095 && id != -4096 && id != -4094 && id != -4093 && id != -4092 && id != -4091
      {
         // Java: if (id >= -4072 && id <= -3817)
            if id >= -4072 && id <= -3817
         {
            // Java: EscherBlipRecord r;
            let mut r: EscherRecord;
            // Java: if (id != -4065 && id != -4067 && id != -4066)
                if id != -4065 && id != -4067 && id != -4066
            {
               // Java: if (id != -4070 && id != -4069 && id != -4068)
                    if id != -4070 && id != -4069 && id != -4068
               {
                  // Java: r = new EscherBlipRecord();
                  r = EscherRecord::new();
               }
               // Java: else
                    else
               {
                  // Java: r = new EscherMetafileBlip();
                  r = EscherRecord::new();
               }
            }
            // Java: else
                else
            {
               // Java: r = new EscherBitmapBlip();
               r = EscherRecord::new();
            }
            // Java: r.setRecordId(id);
            r.setRecordId(id);
            // Java: return r;
            return r;
         }
         // Java: else
            else
         {
            // Java: Constructor<? extends EscherRecord> recordConstructor = (Constructor)recordsMap.get(id);
            let record_constructor = get_records_map().lock().unwrap().get(&id).copied();
            // Java: EscherRecord escherRecord = null;
            let mut escher_record: EscherRecord;
            // Java: if (recordConstructor == null)
                if record_constructor.is_none()
            {
               // Java: return new UnknownEscherRecord();
               return EscherRecord::new();
            }
            // Java: else
                else
            {
               // Java: try
               {
                  // Java: escherRecord = (EscherRecord)recordConstructor.newInstance();
                  escher_record = record_constructor.unwrap()();
               }
               // Java: catch (Exception var5)
               {
                  // Java: return new UnknownEscherRecord();
                  return EscherRecord::new();
               }
               // Java: escherRecord.setRecordId(id);
               escher_record.setRecordId(id);
               // Java: return escherRecord;
               return escher_record;
            }
         }
      }
      // Java: else
        else
      {
         // Java: EscherContainerRecord r = new EscherContainerRecord();
         let mut r = EscherRecord::new();
         // Java: r.setRecordId(id);
         r.setRecordId(id);
         // Java: return r;
         return r;
      }
    }

    // Java: EscherRecord createRecord(byte[] data, int offset)
    pub fn createRecord_bytes(&self, data: &[u8], offset: i32) -> EscherRecord {
      // Java: EscherRecord.EscherRecordHeader header = EscherRecord.EscherRecordHeader.readHeader(data, offset);
      let header = EscherRecordHeader::readHeader_bytes(data, offset as usize);
      // Java: if ((header.getOptions() & 15) == 15 && header.getRecordId() != -4083)
        if (header.getOptions() & 15) == 15 && header.getRecordId() != -4083
      {
         // Java: EscherContainerRecord r = new EscherContainerRecord();
         let mut r = EscherRecord::new();
         // Java: r.setRecordId(header.getRecordId());
         r.setRecordId(header.getRecordId());
         // Java: r.setOptions(header.getOptions());
         r.setOptions(header.getOptions());
         // Java: r.setRemainingBytes(header.getRemainingBytes());
         r.setRemainingBytes(header.getRemainingBytes());
         // Java: return r;
         return r;
      }
      // Java: else if (header.getRecordId() >= -4072 && header.getRecordId() <= -3817)
        else if header.getRecordId() >= -4072 && header.getRecordId() <= -3817
      {
         // Java: EscherBlipRecord r;
         let mut r: EscherRecord;
         // Java: if (header.getRecordId() != -4065 && header.getRecordId() != -4067 && header.getRecordId() != -4066)
            if header.getRecordId() != -4065 && header.getRecordId() != -4067 && header.getRecordId() != -4066
         {
            // Java: if (header.getRecordId() != -4070 && header.getRecordId() != -4069 && header.getRecordId() != -4068)
                if header.getRecordId() != -4070 && header.getRecordId() != -4069 && header.getRecordId() != -4068
            {
               // Java: r = new EscherBlipRecord();
               r = EscherRecord::new();
            }
            // Java: else
                else
            {
               // Java: r = new EscherMetafileBlip();
               r = EscherRecord::new();
            }
         }
         // Java: else
            else
         {
            // Java: r = new EscherBitmapBlip();
            r = EscherRecord::new();
         }
         // Java: r.setRecordId(header.getRecordId());
         r.setRecordId(header.getRecordId());
         // Java: r.setOptions(header.getOptions());
         r.setOptions(header.getOptions());
         // Java: r.setRemainingBytes(header.getRemainingBytes());
         r.setRemainingBytes(header.getRemainingBytes());
         // Java: return r;
         return r;
      }
      // Java: else
        else
      {
         // Java: Constructor<? extends EscherRecord> recordConstructor = (Constructor)recordsMap.get(header.getRecordId());
         let record_constructor = get_records_map().lock().unwrap().get(&header.getRecordId()).copied();
         // Java: EscherRecord escherRecord = null;
         let mut escher_record: EscherRecord;
         // Java: if (recordConstructor == null)
            if record_constructor.is_none()
         {
            // Java: escherRecord = new UnknownEscherRecord();
            escher_record = EscherRecord::new();
         }
         // Java: else
            else
         {
            // Java: try
            {
               // Java: escherRecord = (EscherRecord)recordConstructor.newInstance();
               escher_record = record_constructor.unwrap()();
            }
            // Java: catch (Exception var7)
            {
               // Java: escherRecord = new UnknownEscherRecord();
               escher_record = EscherRecord::new();
            }
         }
         // Java: escherRecord.setRecordId(header.getRecordId());
         escher_record.setRecordId(header.getRecordId());
         // Java: escherRecord.setOptions(header.getOptions());
         escher_record.setOptions(header.getOptions());
         // Java: escherRecord.setRemainingBytes(header.getRemainingBytes());
         escher_record.setRemainingBytes(header.getRemainingBytes());
         // Java: return escherRecord;
         return escher_record;
      }
    }

    // Java: EscherRecord createRecord(DocumentInputStream data, int offset)
    pub fn createRecord_stream(&self, data: &mut DocumentInputStream, offset: i32) -> EscherRecord {
      // Java: return this.createRecord(data, offset, false);
      self.createRecord_stream_flag(data, offset, false)
    }

    // Java: EscherRecord createRecord(DocumentInputStream data, int offset, boolean useDerived)
    pub fn createRecord_stream_flag(&self, data: &mut DocumentInputStream, offset: i32, use_derived: bool) -> EscherRecord {
      // Java: EscherRecord.EscherRecordHeader header = EscherRecord.EscherRecordHeader.readHeader(data, offset);
      let header = EscherRecordHeader::readHeader_stream(data, offset);
      // Java: if ((header.getOptions() & 15) == 15 && header.getRecordId() != -4083)
        if (header.getOptions() & 15) == 15 && header.getRecordId() != -4083
      {
         // Java: EscherContainerRecord r = null;
         let mut r: EscherRecord;
         // Java: if (useDerived)
            if use_derived
         {
            // Java: switch (header.getRecordId())
                match header.getRecordId()
            {
               // Java: case -4096:
               -4096 => {
                  // Java: r = new EscherDggContainerRecord();
                  r = EscherRecord::new();
                  // Java: break;
               }
               // Java: case -4095:
               -4095 => {
                  // Java: r = new EscherBStoreContainerRecord();
                  r = EscherRecord::new();
                  // Java: break;
               }
               // Java: case -4094:
               -4094 => {
                  // Java: r = new EscherDgContainerRecord();
                  r = EscherRecord::new();
                  // Java: data.seek((long)(offset - 1));
                  data.seek((offset - 1) as i64);
                  // Java: ((EscherDgContainerRecord)r).setSeperatorType(data.readByte());
                  let _sep = data.readByte();
                  // Java: break;
               }
               // Java: case -4093:
               -4093 => {
                  // Java: r = new EscherSpgrContainerRecord();
                  r = EscherRecord::new();
                  // Java: break;
               }
               // Java: default:
               _ => {
                  // Java: r = new EscherContainerRecord();
                  r = EscherRecord::new();
               }
            }
         }
         // Java: else
            else
         {
            // Java: r = new EscherContainerRecord();
            r = EscherRecord::new();
         }
         // Java: r.setRecordId(header.getRecordId());
         r.setRecordId(header.getRecordId());
         // Java: r.setOptions(header.getOptions());
         r.setOptions(header.getOptions());
         // Java: return r;
         return r;
      }
      // Java: else if (header.getRecordId() >= -4072 && header.getRecordId() <= -3817)
        else if header.getRecordId() >= -4072 && header.getRecordId() <= -3817 && header.getRecordId() != -4055
      {
         // Java: EscherBlipRecord r;
         let mut r: EscherRecord;
         // Java: if (header.getRecordId() != -4065 && header.getRecordId() != -4067 && header.getRecordId() != -4066 && header.getRecordId() != -4055)
            if header.getRecordId() != -4065 && header.getRecordId() != -4067 && header.getRecordId() != -4066 && header.getRecordId() != -4055
         {
            // Java: if (header.getRecordId() != -4070 && header.getRecordId() != -4069 && header.getRecordId() != -4068)
                if header.getRecordId() != -4070 && header.getRecordId() != -4069 && header.getRecordId() != -4068
            {
               // Java: r = new EscherBlipRecord();
               r = EscherRecord::new();
            }
            // Java: else
                else
            {
               // Java: r = new EscherMetafileBlip();
               r = EscherRecord::new();
            }
         }
         // Java: else
            else
         {
            // Java: r = new EscherBitmapBlip();
            r = EscherRecord::new();
         }
         // Java: r.setRecordId(header.getRecordId());
         r.setRecordId(header.getRecordId());
         // Java: r.setOptions(header.getOptions());
         r.setOptions(header.getOptions());
         // Java: return r;
         return r;
      }
      // Java: else
        else
      {
         // Java: Constructor<? extends EscherRecord> recordConstructor = (Constructor)recordsMap.get(header.getRecordId());
         let record_constructor = get_records_map().lock().unwrap().get(&header.getRecordId()).copied();
         // Java: EscherRecord escherRecord = null;
         let mut escher_record: EscherRecord;
         // Java: if (recordConstructor == null)
            if record_constructor.is_none()
         {
            // Java: return new UnknownEscherRecord();
            return EscherRecord::new();
         }
         // Java: else
            else
         {
            // Java: try
            {
               // Java: escherRecord = (EscherRecord)recordConstructor.newInstance();
               escher_record = record_constructor.unwrap()();
            }
            // Java: catch (Exception var8)
            {
               // Java: return new UnknownEscherRecord();
               return EscherRecord::new();
            }
            // Java: escherRecord.setRecordId(header.getRecordId());
            escher_record.setRecordId(header.getRecordId());
            // Java: escherRecord.setOptions(header.getOptions());
            escher_record.setOptions(header.getOptions());
            // Java: return escherRecord;
            return escher_record;
         }
      }
    }

    // Java: EscherRecord createRecord(IBlockDocumentInputStream data, int offset)
    pub fn createRecord_block(&self, data: &mut dyn IBlockDocumentInputStream, _offset: i32) -> EscherRecord {
      // Java: EscherRecord.EscherRecordHeader header = EscherRecord.EscherRecordHeader.readHeader(data, offset);
      let header = EscherRecordHeader::readHeader_block(data);
      // Java: if ((header.getOptions() & 15) == 15 && header.getRecordId() != -4083)
        if (header.getOptions() & 15) == 15 && header.getRecordId() != -4083
      {
         // Java: EscherContainerRecord r = new EscherContainerRecord();
         let mut r = EscherRecord::new();
         // Java: r.setRecordId(header.getRecordId());
         r.setRecordId(header.getRecordId());
         // Java: r.setOptions(header.getOptions());
         r.setOptions(header.getOptions());
         // Java: r.setRemainingBytes(header.getRemainingBytes());
         r.setRemainingBytes(header.getRemainingBytes());
         // Java: return r;
         return r;
      }
      // Java: else if (header.getRecordId() >= -4072 && header.getRecordId() <= -3817)
        else if header.getRecordId() >= -4072 && header.getRecordId() <= -3817
      {
         // Java: EscherBlipRecord r;
         let mut r: EscherRecord;
         // Java: if (header.getRecordId() != -4065 && header.getRecordId() != -4067 && header.getRecordId() != -4066)
            if header.getRecordId() != -4065 && header.getRecordId() != -4067 && header.getRecordId() != -4066
         {
            // Java: if (header.getRecordId() != -4070 && header.getRecordId() != -4069 && header.getRecordId() != -4068)
                if header.getRecordId() != -4070 && header.getRecordId() != -4069 && header.getRecordId() != -4068
            {
               // Java: r = new EscherBlipRecord();
               r = EscherRecord::new();
            }
            // Java: else
                else
            {
               // Java: r = new EscherMetafileBlip();
               r = EscherRecord::new();
            }
         }
         // Java: else
            else
         {
            // Java: r = new EscherBitmapBlip();
            r = EscherRecord::new();
         }
         // Java: r.setRecordId(header.getRecordId());
         r.setRecordId(header.getRecordId());
         // Java: r.setOptions(header.getOptions());
         r.setOptions(header.getOptions());
         // Java: r.setRemainingBytes(header.getRemainingBytes());
         r.setRemainingBytes(header.getRemainingBytes());
         // Java: return r;
         return r;
      }
      // Java: else
        else
      {
         // Java: Constructor<? extends EscherRecord> recordConstructor = (Constructor)recordsMap.get(header.getRecordId());
         let record_constructor = get_records_map().lock().unwrap().get(&header.getRecordId()).copied();
         // Java: EscherRecord escherRecord = null;
         let mut escher_record: EscherRecord;
         // Java: if (recordConstructor == null)
            if record_constructor.is_none()
         {
            // Java: escherRecord = new UnknownEscherRecord();
            escher_record = EscherRecord::new();
         }
         // Java: else
            else
         {
            // Java: try
            {
               // Java: escherRecord = (EscherRecord)recordConstructor.newInstance();
               escher_record = record_constructor.unwrap()();
            }
            // Java: catch (Exception var7)
            {
               // Java: escherRecord = new UnknownEscherRecord();
               escher_record = EscherRecord::new();
            }
         }
         // Java: escherRecord.setRecordId(header.getRecordId());
         escher_record.setRecordId(header.getRecordId());
         // Java: escherRecord.setOptions(header.getOptions());
         escher_record.setOptions(header.getOptions());
         // Java: escherRecord.setRemainingBytes(header.getRemainingBytes());
         escher_record.setRemainingBytes(header.getRemainingBytes());
         // Java: return escherRecord;
         return escher_record;
      }
    }

    // Java: static private static Map<Short, Constructor<? extends EscherRecord>> recordsToMap(Class<?>[] recClasses)
    fn recordsToMap(_rec_classes: &[i16]) -> HashMap<i16, RecordFactory> {
      // Java: Map<Short, Constructor<? extends EscherRecord>> result = new HashMap();
      let mut result: HashMap<i16, RecordFactory> = HashMap::new();
      // Java: Class<?>[] EMPTY_CLASS_ARRAY = new Class[0];
      // (not needed in Rust)
      // Java: for(int i = 0; i < recClasses.length; ++i)
        for &sid in _rec_classes.iter()
      {
         // Java: @SuppressWarnings("unchecked")
         // Java: Class<? extends EscherRecord> recCls = (Class<? extends EscherRecord>)recClasses[i];
         // (Rust uses the sid directly)
         // Java: short sid;
         // Java: try
         {
            // Java: sid = recCls.getField("RECORD_ID").getShort(null);
            // (sid comes from the array directly in Rust)
         }
         // Java: catch (IllegalArgumentException e)
         {
            // Java: throw new RuntimeException(e);
            // (not applicable in Rust)
         }
         // Java: catch (IllegalAccessException e)
         {
            // Java: throw new RuntimeException(e);
            // (not applicable in Rust)
         }
         // Java: catch (NoSuchFieldException e)
         {
            // Java: throw new RuntimeException(e);
            // (not applicable in Rust)
         }
         // Java: Constructor<? extends EscherRecord> constructor;
         let constructor: RecordFactory = default_record_factory;
         // Java: try
         {
            // Java: constructor = recCls.getConstructor(EMPTY_CLASS_ARRAY);
            // (Rust uses function pointer directly)
         }
         // Java: catch (NoSuchMethodException e)
         {
            // Java: throw new RuntimeException(e);
            // (not applicable in Rust)
         }
         // Java: result.put(sid, constructor);
         result.insert(sid, constructor);
      }
      // Java: return result;
      result
    }

    // Java: void extendFactory(Map<Short, Constructor<? extends EscherRecord>> extendMap)
    pub fn extendFactory(&self, extend_map: HashMap<i16, RecordFactory>) {
      // Java: if (extendMap != null && extendMap.size() != 0)
        if !extend_map.is_empty()
      {
         // Java: if (recordsMap != null)
            if true
         {
            // Java: recordsMap.putAll(extendMap);
            let mut map = get_records_map().lock().unwrap();
            map.extend(extend_map);
         }
         // Java: else
            else
         {
            // Java: recordsMap = extendMap;
            // (handled above - always extends in Rust)
         }
      }
    }

}

impl EscherRecordFactory for DefaultEscherRecordFactory {
    fn createRecord_bytes(&self, data: &[u8], offset: i32) -> EscherRecord {
        self.createRecord_bytes(data, offset)
    }
    fn createRecord_stream(&self, stream: &mut DocumentInputStream, offset: i32) -> EscherRecord {
        self.createRecord_stream(stream, offset)
    }
    fn createRecord_stream_flag(&self, stream: &mut DocumentInputStream, offset: i32, flag: bool) -> EscherRecord {
        self.createRecord_stream_flag(stream, offset, flag)
    }
    fn createRecord_short(&self, record_type: i16) -> EscherRecord {
        self.createRecord_short(record_type)
    }
    fn createRecord_block(&self, stream: &mut dyn IBlockDocumentInputStream, offset: i32) -> EscherRecord {
        self.createRecord_block(stream, offset)
    }
}
