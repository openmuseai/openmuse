// Java: import cn.wps.kfc.api.map.IIntObjectMap
// Java: import cn.wps.kfc.impl.map.WIntObjectHashMap
// Java: import java.io.IOException
// Java: import java.util.Arrays
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use std::collections::HashMap;

fn le_get_short(buf: &[u8], offset: usize) -> i16 {
    ((buf[offset] as i16) & 0xFF) | (((buf[offset+1] as i16) & 0xFF) << 8)
}
fn le_get_int(buf: &[u8], offset: usize) -> i32 {
    (buf[offset] as i32 & 0xFF) | ((buf[offset+1] as i32 & 0xFF) << 8) |
    ((buf[offset+2] as i32 & 0xFF) << 16) | ((buf[offset+3] as i32 & 0xFF) << 24)
}
fn le_put_short(buf: &mut [u8], offset: usize, value: i16) {
    buf[offset] = (value & 0xFF) as u8;
    buf[offset+1] = ((value >> 8) & 0xFF) as u8;
}
fn le_put_int(buf: &mut [u8], offset: usize, value: i32) {
    buf[offset] = (value & 0xFF) as u8;
    buf[offset+1] = ((value >> 8) & 0xFF) as u8;
    buf[offset+2] = ((value >> 16) & 0xFF) as u8;
    buf[offset+3] = ((value >> 24) & 0xFF) as u8;
}

pub struct UnhandledDataStructure {
    pub(crate) _buf: Option<Vec<u8>>,
    pub(crate) stream: Option<*mut DocumentInputStream>,
    pub(crate) offset: i32,
    pub(crate) length: i32,
}

impl UnhandledDataStructure {
    pub fn new_from_stream(stream: *mut DocumentInputStream, offset: i32, length: i32) -> Self {
        UnhandledDataStructure { _buf: None, stream: Some(stream), offset, length }
    }
    pub fn new_from_bytes(buf: &[u8], offset: i32, length: i32) -> Self {
        let end = (offset + length) as usize;
        let start = offset as usize;
        let data = buf[start..end].to_vec();
        UnhandledDataStructure { _buf: Some(data), stream: None, offset, length }
    }
    pub fn getBuf(&mut self) -> &[u8] {
        if self._buf.is_none() {
            let mut data = vec![0u8; self.length as usize];
            if let Some(stream_ptr) = self.stream {
                unsafe {
                    let stream = &mut *stream_ptr;
                    stream.seek(self.offset as i64);
                    stream.read(&mut data);
                }
            }
            self._buf = Some(data);
        }
        self._buf.as_ref().unwrap()
    }
}

// Java field: String TAG
#[allow(dead_code)]
pub(crate) const TAG: &str = "";
// Java field: int STSHFORIG
pub(crate) const STSHFORIG: i32 = 0;
// Java field: int STSHF
pub(crate) const STSHF: i32 = 1;
// Java field: int PLCFFNDREF
pub(crate) const PLCFFNDREF: i32 = 2;
// Java field: int PLCFFNDTXT
pub(crate) const PLCFFNDTXT: i32 = 3;
// Java field: int PLCFANDREF
pub(crate) const PLCFANDREF: i32 = 4;
// Java field: int PLCFANDTXT
pub(crate) const PLCFANDTXT: i32 = 5;
// Java field: int PLCFSED
pub(crate) const PLCFSED: i32 = 6;
// Java field: int PLCFPAD
pub(crate) const PLCFPAD: i32 = 7;
// Java field: int PLCFPHE
pub(crate) const PLCFPHE: i32 = 8;
// Java field: int STTBGLSY
pub(crate) const STTBGLSY: i32 = 9;
// Java field: int PLCFGLSY
pub(crate) const PLCFGLSY: i32 = 10;
// Java field: int PLCFHDD
pub(crate) const PLCFHDD: i32 = 11;
// Java field: int PLCFBTECHPX
pub(crate) const PLCFBTECHPX: i32 = 12;
// Java field: int PLCFBTEPAPX
pub(crate) const PLCFBTEPAPX: i32 = 13;
// Java field: int PLCFSEA
pub(crate) const PLCFSEA: i32 = 14;
// Java field: int STTBFFFN
pub(crate) const STTBFFFN: i32 = 15;
// Java field: int PLCFFLDMOM
pub(crate) const PLCFFLDMOM: i32 = 16;
// Java field: int PLCFFLDHDR
pub(crate) const PLCFFLDHDR: i32 = 17;
// Java field: int PLCFFLDFTN
pub(crate) const PLCFFLDFTN: i32 = 18;
// Java field: int PLCFFLDATN
pub(crate) const PLCFFLDATN: i32 = 19;
// Java field: int PLCFFLDMCR
pub(crate) const PLCFFLDMCR: i32 = 20;
// Java field: int STTBFBKMK
pub(crate) const STTBFBKMK: i32 = 21;
// Java field: int PLCFBKF
pub(crate) const PLCFBKF: i32 = 22;
// Java field: int PLCFBKL
pub(crate) const PLCFBKL: i32 = 23;
// Java field: int CMDS
pub(crate) const CMDS: i32 = 24;
// Java field: int PLCMCR
pub(crate) const PLCMCR: i32 = 25;
// Java field: int STTBFMCR
pub(crate) const STTBFMCR: i32 = 26;
// Java field: int PRDRVR
pub(crate) const PRDRVR: i32 = 27;
// Java field: int PRENVPORT
pub(crate) const PRENVPORT: i32 = 28;
// Java field: int PRENVLAND
pub(crate) const PRENVLAND: i32 = 29;
// Java field: int WSS
pub(crate) const WSS: i32 = 30;
// Java field: int DOP
pub(crate) const DOP: i32 = 31;
// Java field: int STTBFASSOC
pub(crate) const STTBFASSOC: i32 = 32;
// Java field: int CLX
pub(crate) const CLX: i32 = 33;
// Java field: int PLCFPGDFTN
pub(crate) const PLCFPGDFTN: i32 = 34;
// Java field: int AUTOSAVESOURCE
pub(crate) const AUTOSAVESOURCE: i32 = 35;
// Java field: int GRPXSTATNOWNERS
pub(crate) const GRPXSTATNOWNERS: i32 = 36;
// Java field: int STTBFATNBKMK
pub(crate) const STTBFATNBKMK: i32 = 37;
// Java field: int PLCFDOAMOM
pub(crate) const PLCFDOAMOM: i32 = 38;
// Java field: int PLCDOAHDR
pub(crate) const PLCDOAHDR: i32 = 39;
// Java field: int PLCSPAMOM
pub(crate) const PLCSPAMOM: i32 = 40;
// Java field: int PLCSPAHDR
pub(crate) const PLCSPAHDR: i32 = 41;
// Java field: int PLCFATNBKF
pub(crate) const PLCFATNBKF: i32 = 42;
// Java field: int PLCFATNBKL
pub(crate) const PLCFATNBKL: i32 = 43;
// Java field: int PMS
pub(crate) const PMS: i32 = 44;
// Java field: int FORMFLDSTTBS
pub(crate) const FORMFLDSTTBS: i32 = 45;
// Java field: int PLCFENDREF
pub(crate) const PLCFENDREF: i32 = 46;
// Java field: int PLCFENDTXT
pub(crate) const PLCFENDTXT: i32 = 47;
// Java field: int PLCFFLDEDN
pub(crate) const PLCFFLDEDN: i32 = 48;
// Java field: int PLCFPGDEDN
pub(crate) const PLCFPGDEDN: i32 = 49;
// Java field: int DGGINFO
pub(crate) const DGGINFO: i32 = 50;
// Java field: int STTBFRMARK
pub(crate) const STTBFRMARK: i32 = 51;
// Java field: int STTBCAPTION
pub(crate) const STTBCAPTION: i32 = 52;
// Java field: int STTBAUTOCAPTION
pub(crate) const STTBAUTOCAPTION: i32 = 53;
// Java field: int PLCFWKB
pub(crate) const PLCFWKB: i32 = 54;
// Java field: int PLCFSPL
pub(crate) const PLCFSPL: i32 = 55;
// Java field: int PLCFTXBXTXT
pub(crate) const PLCFTXBXTXT: i32 = 56;
// Java field: int PLCFFLDTXBX
pub(crate) const PLCFFLDTXBX: i32 = 57;
// Java field: int PLCFHDRTXBXTXT
pub(crate) const PLCFHDRTXBXTXT: i32 = 58;
// Java field: int PLCFFLDHDRTXBX
pub(crate) const PLCFFLDHDRTXBX: i32 = 59;
// Java field: int STWUSER
pub(crate) const STWUSER: i32 = 60;
// Java field: int STTBTTMBD
pub(crate) const STTBTTMBD: i32 = 61;
// Java field: int UNUSED
pub(crate) const UNUSED: i32 = 62;
// Java field: int PGDMOTHER
pub(crate) const PGDMOTHER: i32 = 63;
// Java field: int BKDMOTHER
pub(crate) const BKDMOTHER: i32 = 64;
// Java field: int PGDFTN
pub(crate) const PGDFTN: i32 = 65;
// Java field: int BKDFTN
pub(crate) const BKDFTN: i32 = 66;
// Java field: int PGDEDN
pub(crate) const PGDEDN: i32 = 67;
// Java field: int BKDEDN
pub(crate) const BKDEDN: i32 = 68;
// Java field: int STTBFINTFLD
pub(crate) const STTBFINTFLD: i32 = 69;
// Java field: int ROUTESLIP
pub(crate) const ROUTESLIP: i32 = 70;
// Java field: int STTBSAVEDBY
pub(crate) const STTBSAVEDBY: i32 = 71;
// Java field: int STTBFNM
pub(crate) const STTBFNM: i32 = 72;
// Java field: int PLCFLST
pub(crate) const PLCFLST: i32 = 73;
// Java field: int PLFLFO
pub(crate) const PLFLFO: i32 = 74;
// Java field: int PLCFTXBXBKD
pub(crate) const PLCFTXBXBKD: i32 = 75;
// Java field: int PLCFTXBXHDRBKD
pub(crate) const PLCFTXBXHDRBKD: i32 = 76;
// Java field: int DOCUNDO
pub(crate) const DOCUNDO: i32 = 77;
// Java field: int RGBUSE
pub(crate) const RGBUSE: i32 = 78;
// Java field: int USP
pub(crate) const USP: i32 = 79;
// Java field: int USKF
pub(crate) const USKF: i32 = 80;
// Java field: int PLCUPCRGBUSE
pub(crate) const PLCUPCRGBUSE: i32 = 81;
// Java field: int PLCUPCUSP
pub(crate) const PLCUPCUSP: i32 = 82;
// Java field: int STTBGLSYSTYLE
pub(crate) const STTBGLSYSTYLE: i32 = 83;
// Java field: int PLGOSL
pub(crate) const PLGOSL: i32 = 84;
// Java field: int PLCOCX
pub(crate) const PLCOCX: i32 = 85;
// Java field: int PLCFBTELVC
pub(crate) const PLCFBTELVC: i32 = 86;
// Java field: int MODIFIED
pub(crate) const MODIFIED: i32 = 87;
// Java field: int PLCFLVC
pub(crate) const PLCFLVC: i32 = 88;
// Java field: int PLCASUMY
pub(crate) const PLCASUMY: i32 = 89;
// Java field: int PLCFGRAM
pub(crate) const PLCFGRAM: i32 = 90;
// Java field: int STTBLISTNAMES
pub(crate) const STTBLISTNAMES: i32 = 91;
// Java field: int STTBFUSSR
pub(crate) const STTBFUSSR: i32 = 92;
// Java field: int STTBRGTPLC
pub(crate) const STTBRGTPLC: i32 = 96;
// Java field: int ATRDEXTRA
pub(crate) const ATRDEXTRA: i32 = 112;
// Java field: int PLRSID
pub(crate) const PLRSID: i32 = 113;
// Java field: int FIELD_SIZE
pub(crate) const FIELD_SIZE: i32 = 8;
// Java field: int FIELD_MAXCOUNT
pub(crate) const FIELD_MAXCOUNT: i32 = 164;

// Java: public class FIBFieldHandler
pub struct FIBFieldHandler {
    // Java field: IIntObjectMap<UnhandledDataStructure> _unknownMap
    pub(crate) _unknownMap: HashMap<i32, UnhandledDataStructure>,
    // Java field: int[] _fields
    pub(crate) _fields: Vec<i32>,
    // Java field: int _numFields
    pub(crate) _numFields: i32,
}

// Java: public class FIBFieldHandler
impl FIBFieldHandler {
    // Java: FIBFieldHandler(byte[] mainStream, int offset, byte[] tableStream, boolean areKnown)
    pub fn new_from_bytes(main_stream: &[u8], offset: usize, table_stream: &[u8], are_known: bool) -> Self {
      // Java: super();
      // Java: this._numFields = LittleEndian.getShort(mainStream, offset);
      let num_fields = le_get_short(main_stream, offset) as i32;
      // Java: if (this._numFields < 164)
      let fields = if num_fields < 164 {
         // Java: this._fields = new int[328];
         vec![0i32; 328]
      }
      // Java: else
      else {
         // Java: this._fields = new int[this._numFields * 2];
         vec![0i32; (num_fields * 2) as usize]
      };
      let mut fields = fields;
      // Java: offset += 2;
      let offset = offset + 2;
      // Java: int fieldOffset = 0;
      let mut unknown_map: HashMap<i32, UnhandledDataStructure> = HashMap::new();
      // Java: for(int x = 0; x < this._numFields; ++x)
      for x in 0..num_fields {
         // Java: fieldOffset = x * 8 + offset;
         let mut field_offset = (x as usize) * 8 + offset;
         // Java: int dsOffset = LittleEndian.getInt(mainStream, fieldOffset);
         let ds_offset = le_get_int(main_stream, field_offset);
         // Java: fieldOffset += 4;
         field_offset += 4;
         // Java: int dsSize = LittleEndian.getInt(mainStream, fieldOffset);
         let ds_size = le_get_int(main_stream, field_offset);
         // Java: if (this.isKnownField(x) ^ areKnown && dsSize > 0 && dsOffset + dsSize <= tableStream.length)
         if (is_known_field(x) ^ are_known) && ds_size > 0 && (ds_offset + ds_size) <= table_stream.len() as i32 {
            // Java: UnhandledDataStructure unhandled = new UnhandledDataStructure(tableStream, dsOffset, dsSize);
            let unhandled = UnhandledDataStructure::new_from_bytes(table_stream, ds_offset, ds_size);
            // Java: this._unknownMap.put(x, unhandled);
            unknown_map.insert(x, unhandled);
         }
         // Java: this._fields[x * 2] = dsOffset;
         fields[(x * 2) as usize] = ds_offset;
         // Java: this._fields[x * 2 + 1] = dsSize;
         fields[(x * 2 + 1) as usize] = ds_size;
      }
      FIBFieldHandler { _unknownMap: unknown_map, _fields: fields, _numFields: num_fields }
    }

    // Java: FIBFieldHandler(DocumentInputStream mainStream, int offset, DocumentInputStream tableStream, boolean areKnown)
    pub fn new_from_stream(main_stream: &mut DocumentInputStream, offset: i32, table_stream: &mut DocumentInputStream, are_known: bool) -> Self {
      // Java: super();
      // Java: mainStream.seek((long)offset);
      main_stream.seek(offset as i64);
      // Java: this._numFields = mainStream.readUShort();
      let num_fields = main_stream.readUShort() as i32;
      // Java: if (this._numFields < 164)
      let fields = if num_fields < 164 {
         // Java: this._fields = new int[328];
         vec![0i32; 328]
      }
      // Java: else
      else {
         // Java: this._fields = new int[this._numFields * 2];
         vec![0i32; (num_fields * 2) as usize]
      };
      let mut fields = fields;
      let mut unknown_map: HashMap<i32, UnhandledDataStructure> = HashMap::new();
      // Java: for(int x = 0; x < this._numFields; ++x)
      for x in 0..num_fields {
         // Java: int dsOffset = mainStream.readInt();
         let ds_offset = main_stream.readInt();
         // Java: int dsSize = mainStream.readInt();
         let ds_size = main_stream.readInt();
         // Java: if (dsOffset >= 0 && dsSize <= tableStream.available())
         if ds_offset >= 0 && ds_size <= table_stream.available() {
            // Java: if (this.isKnownField(x) ^ areKnown && dsSize > 0)
            if (is_known_field(x) ^ are_known) && ds_size > 0 {
               // Java: UnhandledDataStructure unhandled = new UnhandledDataStructure(tableStream, dsOffset, dsSize);
               let unhandled = UnhandledDataStructure::new_from_stream(table_stream as *mut DocumentInputStream, ds_offset, ds_size);
               // Java: this._unknownMap.put(x, unhandled);
               unknown_map.insert(x, unhandled);
            }
            // Java: int toIndex = x * 2;
            let to_index = (x * 2) as usize;
            // Java: this._fields[toIndex] = dsOffset;
            fields[to_index] = ds_offset;
            // Java: this._fields[toIndex + 1] = dsSize;
            fields[to_index + 1] = ds_size;
         }
      }
      FIBFieldHandler { _unknownMap: unknown_map, _fields: fields, _numFields: num_fields }
    }

    // Java: void clearFields()
    pub fn clearFields(&mut self) {
      // Java: Arrays.fill(this._fields, 0);
      self._fields.fill(0);
    }

    // Java: int getFieldOffset(int field)
    pub fn getFieldOffset(&self, field: i32) -> i32 {
      // Java: return this._fields[field * 2];
      self._fields[(field * 2) as usize]
    }

    // Java: int getFieldSize(int field)
    pub fn getFieldSize(&self, field: i32) -> i32 {
      // Java: return this._fields[field * 2 + 1];
      self._fields[(field * 2 + 1) as usize]
    }

    // Java: void setFieldOffset(int field, int offset)
    pub fn setFieldOffset(&mut self, field: i32, offset: i32) {
      // Java: this._fields[field * 2] = offset;
      self._fields[(field * 2) as usize] = offset;
    }

    // Java: void setFieldSize(int field, int size)
    pub fn setFieldSize(&mut self, field: i32, size: i32) {
      // Java: this._fields[field * 2 + 1] = size;
      self._fields[(field * 2 + 1) as usize] = size;
    }

    // Java: int sizeInBytes()
    pub fn sizeInBytes(&self) -> i32 {
      // Java: return this._numFields * 2 * 4 + 2;
      self._numFields * 2 * 4 + 2
    }

    // Java: void writeTo(byte[] mainStream, int offset, HWPFOutputStream tableStream)
    pub fn writeTo(&mut self, main_stream: &mut [u8], offset: usize, table_stream: &mut HWPFOutputStream) {
      // Java: int length = this._fields.length / 2;
      let length = self._fields.len() / 2;
      // Java: LittleEndian.putShort(mainStream, offset, (short)length);
      le_put_short(main_stream, offset, length as i16);
      // Java: offset += 2;
      let mut offset = offset + 2;
      // Java: for(int x = 0; x < length; ++x)
      for x in 0..length {
         // Java: UnhandledDataStructure ds = (UnhandledDataStructure)this._unknownMap.get(x);
         let has_ds = self._unknownMap.contains_key(&(x as i32));
         // Java: if (ds != null)
         if has_ds {
            // Java: LittleEndian.putInt(mainStream, offset, tableStream.getOffset());
            le_put_int(main_stream, offset, table_stream.getOffset());
            // Java: offset += 4;
            offset += 4;
            // Java: byte[] buf = ds.getBuf();
            let ds = self._unknownMap.get_mut(&(x as i32)).unwrap();
            let buf = ds.getBuf().to_vec();
            // Java: tableStream.write(buf);
            use std::io::Write;
            let _ = table_stream.write(&buf);
            // Java: LittleEndian.putInt(mainStream, offset, buf.length);
            le_put_int(main_stream, offset, buf.len() as i32);
            // Java: offset += 4;
            offset += 4;
         }
         // Java: else
         else {
            // Java: LittleEndian.putInt(mainStream, offset, this._fields[x * 2]);
            le_put_int(main_stream, offset, self._fields[x * 2]);
            // Java: offset += 4;
            offset += 4;
            // Java: LittleEndian.putInt(mainStream, offset, this._fields[x * 2 + 1]);
            le_put_int(main_stream, offset, self._fields[x * 2 + 1]);
            // Java: offset += 4;
            offset += 4;
         }
      }
      // Java: mainStream[offset++] = 2;
      main_stream[offset] = 2; offset += 1;
      // Java: mainStream[offset++] = 0;
      main_stream[offset] = 0; offset += 1;
      // Java: mainStream[offset++] = 12;
      main_stream[offset] = 12; offset += 1;
      // Java: mainStream[offset++] = 1;
      main_stream[offset] = 1; offset += 1;
      // Java: mainStream[offset++] = 0;
      main_stream[offset] = 0; offset += 1;
      // Java: mainStream[offset++] = 0;
      main_stream[offset] = 0; offset += 1;
      // Java: mainStream[offset++] = 0;
      main_stream[offset] = 0; offset += 1;
      // Java: mainStream[offset++] = 0;
      main_stream[offset] = 0;
    }

    // Java: int getFieldsNum()
    pub fn getFieldsNum(&self) -> i32 {
      // Java: return this._numFields;
      self._numFields
    }

    // Java: IIntObjectMap<UnhandledDataStructure> getUnhandleDataMap()
    pub fn getUnhandleDataMap(&mut self) -> &mut HashMap<i32, UnhandledDataStructure> {
      // Java: return this._unknownMap;
      &mut self._unknownMap
    }

    // Java: boolean isKnownField(int fieldId)
    pub fn isKnownField(&self, field_id: i32) -> bool {
      is_known_field(field_id)
    }

}

fn is_known_field(field_id: i32) -> bool {
    // Java: switch (fieldId)
    match field_id {
       // Java: case 0:
       // Java: case 1:
       // Java: case 2:
       // Java: case 3:
       // Java: case 4:
       // Java: case 5:
       // Java: case 6:
       // Java: case 11:
       // Java: case 12:
       // Java: case 13:
       // Java: case 15:
       // Java: case 16:
       // Java: case 17:
       // Java: case 18:
       // Java: case 19:
       // Java: case 21:
       // Java: case 22:
       // Java: case 23:
       // Java: case 31:
       // Java: case 32:
       // Java: case 33:
       // Java: case 36:
       // Java: case 37:
       // Java: case 40:
       // Java: case 41:
       // Java: case 42:
       // Java: case 43:
       // Java: case 46:
       // Java: case 48:
       // Java: case 50:
       // Java: case 51:
       // Java: case 56:
       // Java: case 57:
       // Java: case 58:
       // Java: case 59:
       0 | 1 | 2 | 3 | 4 | 5 | 6 | 11 | 12 | 13 | 15 | 16 | 17 | 18 | 19 |
       21 | 22 | 23 | 31 | 32 | 33 | 36 | 37 | 40 | 41 | 42 | 43 | 46 | 48 |
       50 | 51 | 56 | 57 | 58 | 59 => {
          // Java: return true;
          true
       }
       // Java: case 7:
       // Java: case 8:
       // Java: case 9:
       // Java: case 10:
       // Java: case 14:
       // Java: case 20:
       // Java: case 24:
       // Java: case 25:
       // Java: case 26:
       // Java: case 27:
       // Java: case 28:
       // Java: case 29:
       // Java: case 30:
       // Java: case 34:
       // Java: case 35:
       // Java: case 38:
       // Java: case 39:
       // Java: case 44:
       // Java: case 45:
       // Java: case 47:
       // Java: case 49:
       // Java: case 52:
       // Java: case 53:
       // Java: case 54:
       // Java: case 55:
       // Java: default:
       _ => {
          // Java: switch (fieldId)
          match field_id {
             // Java: case 71:
             // Java: case 73:
             // Java: case 74:
             // Java: case 75:
             // Java: case 76:
             71 | 73 | 74 | 75 | 76 => {
                // Java: return true;
                true
             }
             // Java: case 72:
             // Java: default:
             _ => {
                // Java: switch (fieldId)
                match field_id {
                   // Java: case 87:
                   // Java: case 96:
                   // Java: case 112:
                   // Java: case 113:
                   87 | 96 | 112 | 113 => {
                      // Java: return true;
                      true
                   }
                   // Java: default:
                   _ => {
                      // Java: return false;
                      false
                   }
                }
             }
          }
       }
    }
}
