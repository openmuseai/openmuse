// Java: import cn.wps.kfc.api.map.IIntObjectMap
// Java: import cn.wps.kfc.impl.map.WIntObjectHashMap
// Java: import java.io.IOException
// Java: import java.util.Map
// Java: import java.util.TreeMap
// Java: import org.apache.poi.hwpf.model.PageBuffer
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use std::collections::{BTreeMap, HashMap};

use super::EscherBoolProperty::EscherBoolProperty;
use super::EscherRGBProperty::EscherRGBProperty;
use super::EscherShapePathProperty::EscherShapePathProperty;
use super::EscherSimpleProperty::EscherSimpleProperty;
use super::EscherComplexProperty::EscherComplexProperty;
use super::EscherArrayProperty::EscherArrayProperty;
use super::EscherProperties::EscherProperties;

fn le_get_short(data: &[u8], offset: usize) -> i16 {
    (data[offset] as i16) | ((data[offset + 1] as i16) << 8)
}

fn le_get_int(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32) | ((data[offset + 1] as i32) << 8) |
    ((data[offset + 2] as i32) << 16) | ((data[offset + 3] as i32) << 24)
}

pub enum EscherPropertyVariant {
    Bool(EscherBoolProperty),
    RGB(EscherRGBProperty),
    ShapePath(EscherShapePathProperty),
    Simple(EscherSimpleProperty),
    Array(EscherArrayProperty),
    Complex(EscherComplexProperty),
}

impl EscherPropertyVariant {
    pub fn get_property_number(&self) -> i16 {
        match self {
            EscherPropertyVariant::Bool(p) => p.base.base.getPropertyNumber(),
            EscherPropertyVariant::RGB(p) => p.base.base.getPropertyNumber(),
            EscherPropertyVariant::ShapePath(p) => p.base.base.getPropertyNumber(),
            EscherPropertyVariant::Simple(p) => p.base.getPropertyNumber(),
            EscherPropertyVariant::Array(p) => p.base.base.getPropertyNumber(),
            EscherPropertyVariant::Complex(p) => p.base.getPropertyNumber(),
        }
    }

    pub fn getPropertySize(&self) -> i32 {
        match self {
            EscherPropertyVariant::Bool(p) => p.base.base.getPropertySize(),
            EscherPropertyVariant::RGB(p) => p.base.base.getPropertySize(),
            EscherPropertyVariant::ShapePath(p) => p.base.base.getPropertySize(),
            EscherPropertyVariant::Simple(p) => p.base.getPropertySize(),
            EscherPropertyVariant::Array(p) => p.base.base.getPropertySize(),
            EscherPropertyVariant::Complex(p) => p.base.getPropertySize(),
        }
    }
}

impl std::fmt::Debug for EscherPropertyVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EscherPropertyVariant::Simple(p) => write!(f, "{}", p.toString()),
            EscherPropertyVariant::Bool(p) => write!(f, "{}", p.base.toString()),
            EscherPropertyVariant::RGB(p) => write!(f, "{}", p.base.toString()),
            EscherPropertyVariant::ShapePath(p) => write!(f, "{}", p.base.toString()),
            EscherPropertyVariant::Array(_p) => write!(f, "EscherArrayProperty"),
            EscherPropertyVariant::Complex(_p) => write!(f, "EscherComplexProperty"),
        }
    }
}

// Java: public class EscherPropertyFactory
pub struct EscherPropertyFactory;

// Java: public class EscherPropertyFactory
impl EscherPropertyFactory {
    // Java: EscherPropertyFactory()
    pub fn new() -> Self {
      // Java: super();
      EscherPropertyFactory
    }

    // Java: public TreeMap<Integer, EscherProperty> createProperties(byte[] data, int offset, short numProperties)
    pub fn createProperties_bytes(&self, data: &[u8], offset: i32, num_properties: i16) -> BTreeMap<i32, EscherPropertyVariant> {
      // Java: TreeMap<Integer, EscherProperty> results = new TreeMap();
      let mut results: BTreeMap<i32, EscherPropertyVariant> = BTreeMap::new();
      // Java: int pos = offset;
      let mut pos = offset;
      // Java: for(int i = 0; i < numProperties; ++i)
      for _i in 0..(num_properties as i32)
      {
         // Java: short propId = LittleEndian.getShort(data, pos);
         let prop_id = le_get_short(data, pos as usize);
         // Java: int propData = LittleEndian.getInt(data, pos + 2);
         let prop_data = le_get_int(data, (pos + 2) as usize);
         // Java: short propNumber = (short)(propId & 16383);
         let prop_number = prop_id & 16383;
         // Java: boolean isComplex = (propId & Short.MIN_VALUE) != 0;
         let is_complex = (prop_id & i16::MIN) != 0;
         // Java: byte propertyType = EscherProperties.getPropertyType(propNumber);
         let property_type = EscherProperties::getPropertyType(prop_number);
         // Java: if (propertyType == 1)
         if property_type == 1
         {
            // Java: EscherBoolProperty boolProp = new EscherBoolProperty(propId, propData);
            let bool_prop = EscherBoolProperty::new(prop_id, prop_data);
            // Java: results.put(Integer.valueOf(boolProp.getPropertyNumber()), boolProp);
            let key = bool_prop.base.base.getPropertyNumber() as i32;
            results.insert(key, EscherPropertyVariant::Bool(bool_prop));
         }
         // Java: else if (propertyType == 2)
         else if property_type == 2
         {
            // Java: EscherRGBProperty rgbProp = new EscherRGBProperty(propId, propData);
            let rgb_prop = EscherRGBProperty::new(prop_id, prop_data);
            // Java: results.put(Integer.valueOf(rgbProp.getPropertyNumber()), rgbProp);
            let key = rgb_prop.base.base.getPropertyNumber() as i32;
            results.insert(key, EscherPropertyVariant::RGB(rgb_prop));
         }
         // Java: else if (propertyType == 3)
         else if property_type == 3
         {
            // Java: EscherShapePathProperty shapePathProp = new EscherShapePathProperty(propId, propData);
            let shape_path_prop = EscherShapePathProperty::new(prop_id, prop_data);
            // Java: results.put(Integer.valueOf(shapePathProp.getPropertyNumber()), shapePathProp);
            let key = shape_path_prop.base.base.getPropertyNumber() as i32;
            results.insert(key, EscherPropertyVariant::ShapePath(shape_path_prop));
         }
         // Java: else if (!isComplex)
         else if !is_complex
         {
            // Java: EscherSimpleProperty simpleProp = new EscherSimpleProperty(propId, propData);
            let simple_prop = EscherSimpleProperty::new(prop_id, prop_data);
            // Java: results.put(Integer.valueOf(simpleProp.getPropertyNumber()), simpleProp);
            let key = simple_prop.base.getPropertyNumber() as i32;
            results.insert(key, EscherPropertyVariant::Simple(simple_prop));
         }
         // Java: else if (propertyType == 5)
         else if property_type == 5
         {
            // Java: EscherArrayProperty arrayProp = new EscherArrayProperty(propId, new byte[propData]);
            let array_prop = EscherArrayProperty::new(prop_id, vec![0u8; prop_data as usize]);
            // Java: results.put(Integer.valueOf(arrayProp.getPropertyNumber()), arrayProp);
            let key = array_prop.base.base.getPropertyNumber() as i32;
            results.insert(key, EscherPropertyVariant::Array(array_prop));
         }
         // Java: else
         else
         {
            // Java: EscherComplexProperty complexProp = new EscherComplexProperty(propId, new byte[propData]);
            let complex_prop = EscherComplexProperty::new(prop_id, vec![0u8; prop_data as usize]);
            // Java: results.put(Integer.valueOf(complexProp.getPropertyNumber()), complexProp);
            let key = complex_prop.base.getPropertyNumber() as i32;
            results.insert(key, EscherPropertyVariant::Complex(complex_prop));
         }
         // Java: pos += 6;
         pos += 6;
      }
      // Java: for(Map.Entry<Integer, EscherProperty> entry : results.entrySet())
      for (_key, value) in results.iter_mut()
      {
         // Java: EscherProperty p = (EscherProperty)entry.getValue();
         // Java: if (p instanceof EscherComplexProperty)
         match value {
         EscherPropertyVariant::Array(ref mut arr) =>
         {
            // Java: if (p instanceof EscherArrayProperty)
            {
               // Java: pos += ((EscherArrayProperty)p).setArrayData(data, pos);
               pos += arr.setArrayData_bytes(data, pos);
            }
         }
         EscherPropertyVariant::Complex(ref mut cpx) =>
         // Java: else
         {
            {
               // Java: byte[] complexData = ((EscherComplexProperty)p).getComplexData();
               let complex_len = cpx._complexData.len();
               // Java: System.arraycopy(data, pos, complexData, 0, complexData.length);
               cpx._complexData.copy_from_slice(&data[pos as usize..(pos as usize + complex_len)]);
               // Java: pos += complexData.length;
               pos += complex_len as i32;
            }
         }
         _ => {}
         }
      }
      // Java: return results;
      results
    }

    // Java: IIntObjectMap<EscherProperty> createProperties(int numProperties, byte[] data, int offset)
    pub fn createProperties_int_bytes(&self, num_properties: i32, data: &[u8], offset: i32) -> HashMap<i16, EscherPropertyVariant> {
      // Java: IIntObjectMap<EscherProperty> result = new WIntObjectHashMap(numProperties);
      let mut result: HashMap<i16, EscherPropertyVariant> = HashMap::with_capacity(num_properties as usize);
      // Java: int off = offset;
      let mut off = offset;
      // Java: int offToComplex = offset + numProperties * 6;
      let mut off_to_complex = offset + num_properties * 6;
      // Java: for(int i = 0; i < numProperties; ++i)
      for _i in 0..num_properties
      {
         // Java: short propId = LittleEndian.getShort(data, off);
         let prop_id = le_get_short(data, off as usize);
         // Java: off += 2;
         off += 2;
         // Java: int propData = LittleEndian.getInt(data, off);
         let prop_data = le_get_int(data, off as usize);
         // Java: off += 4;
         off += 4;
         // Java: short propNumber = (short)(propId & 16383);
         let prop_number = prop_id & 16383;
         // Java: boolean isComplex = propId < 0;
         let is_complex = prop_id < 0;
         // Java: switch (EscherProperties.getPropertyType(propNumber))
         match EscherProperties::getPropertyType(prop_number)
         {
            // Java: case -1:
            -1 => {
               // Java: break;
            }
            // Java: case 0:
            // Java: case 4:
            // Java: default:
            0 | 4 | _ => {
               // Java: if (!isComplex)
               if !is_complex
               {
                  // Java: result.put(propNumber, new EscherSimpleProperty(propId, propData));
                  result.insert(prop_number, EscherPropertyVariant::Simple(EscherSimpleProperty::new(prop_id, prop_data)));
               }
               // Java: else
               else
               {
                  // Java: byte[] x = new byte[propData];
                  let mut x = vec![0u8; prop_data as usize];
                  // Java: System.arraycopy(data, offToComplex, x, 0, propData);
                  x.copy_from_slice(&data[off_to_complex as usize..(off_to_complex + prop_data) as usize]);
                  // Java: offToComplex += propData;
                  off_to_complex += prop_data;
                  // Java: result.put(propNumber, new EscherComplexProperty(propId, x));
                  result.insert(prop_number, EscherPropertyVariant::Complex(EscherComplexProperty::new(prop_id, x)));
               }
               // Java: break;
            }
            // Java: case 1:
            1 => {
               // Java: result.put(propNumber, new EscherBoolProperty(propId, propData));
               result.insert(prop_number, EscherPropertyVariant::Bool(EscherBoolProperty::new(prop_id, prop_data)));
               // Java: break;
            }
            // Java: case 2:
            2 => {
               // Java: result.put(propNumber, new EscherRGBProperty(propId, propData));
               result.insert(prop_number, EscherPropertyVariant::RGB(EscherRGBProperty::new(prop_id, prop_data)));
               // Java: break;
            }
            // Java: case 3:
            3 => {
               // Java: result.put(propNumber, new EscherShapePathProperty(propId, propData));
               result.insert(prop_number, EscherPropertyVariant::ShapePath(EscherShapePathProperty::new(prop_id, prop_data)));
               // Java: break;
            }
            // Java: case 5:
            5 => {
               // Java: if (isComplex)
               if is_complex
               {
                  // Java: EscherArrayProperty prop = new EscherArrayProperty(propId, new byte[propData]);
                  let mut prop = EscherArrayProperty::new(prop_id, vec![0u8; prop_data as usize]);
                  // Java: offToComplex += prop.setArrayData(data, offToComplex);
                  off_to_complex += prop.setArrayData_bytes(data, off_to_complex);
                  // Java: result.put(propNumber, prop);
                  result.insert(prop_number, EscherPropertyVariant::Array(prop));
               }
               // Java: else
               else
               {
                  // Java: result.put(propNumber, new EscherSimpleProperty(propId, propData));
                  result.insert(prop_number, EscherPropertyVariant::Simple(EscherSimpleProperty::new(prop_id, prop_data)));
               }
            }
         }
      }
      // Java: return result;
      result
    }

    // Java: public TreeMap<Integer, EscherProperty> createProperties(DocumentInputStream data, int offset, short numProperties)
    pub fn createProperties_stream(&self, data: &mut DocumentInputStream, offset: i32, num_properties: i16) -> BTreeMap<i32, EscherPropertyVariant> {
      // Java: TreeMap<Integer, EscherProperty> results = new TreeMap();
      let mut results: BTreeMap<i32, EscherPropertyVariant> = BTreeMap::new();
      // Java: int pos = offset;
      let mut pos = offset;
      // Java: data.seek((long)offset);
      data.seek(offset as i64);
      // Java: for(int i = 0; i < numProperties; ++i)
      for _i in 0..(num_properties as i32)
      {
         // Java: short propId = data.readShort();
         let prop_id = data.readShort();
         // Java: int propData = data.readInt();
         let prop_data = data.readInt();
         // Java: short propNumber = (short)(propId & 16383);
         let prop_number = prop_id & 16383;
         // Java: boolean isComplex = (propId & Short.MIN_VALUE) != 0;
         let is_complex = (prop_id & i16::MIN) != 0;
         // Java: byte propertyType = EscherProperties.getPropertyType(propNumber);
         let property_type = EscherProperties::getPropertyType(prop_number);
         // Java: if (propertyType == 1)
         if property_type == 1
         {
            // Java: EscherBoolProperty boolProp = new EscherBoolProperty(propId, propData);
            let bool_prop = EscherBoolProperty::new(prop_id, prop_data);
            // Java: results.put(Integer.valueOf(boolProp.getPropertyNumber()), boolProp);
            let key = bool_prop.base.base.getPropertyNumber() as i32;
            results.insert(key, EscherPropertyVariant::Bool(bool_prop));
         }
         // Java: else if (propertyType == 2)
         else if property_type == 2
         {
            // Java: EscherRGBProperty rgbProp = new EscherRGBProperty(propId, propData);
            let rgb_prop = EscherRGBProperty::new(prop_id, prop_data);
            // Java: results.put(Integer.valueOf(rgbProp.getPropertyNumber()), rgbProp);
            let key = rgb_prop.base.base.getPropertyNumber() as i32;
            results.insert(key, EscherPropertyVariant::RGB(rgb_prop));
         }
         // Java: else if (propertyType == 3)
         else if property_type == 3
         {
            // Java: EscherShapePathProperty shapePathProp = new EscherShapePathProperty(propId, propData);
            let shape_path_prop = EscherShapePathProperty::new(prop_id, prop_data);
            // Java: results.put(Integer.valueOf(shapePathProp.getPropertyNumber()), shapePathProp);
            let key = shape_path_prop.base.base.getPropertyNumber() as i32;
            results.insert(key, EscherPropertyVariant::ShapePath(shape_path_prop));
         }
         // Java: else if (!isComplex)
         else if !is_complex
         {
            // Java: EscherSimpleProperty simpleProp = new EscherSimpleProperty(propId, propData);
            let simple_prop = EscherSimpleProperty::new(prop_id, prop_data);
            // Java: results.put(Integer.valueOf(simpleProp.getPropertyNumber()), simpleProp);
            let key = simple_prop.base.getPropertyNumber() as i32;
            results.insert(key, EscherPropertyVariant::Simple(simple_prop));
         }
         // Java: else if (propertyType == 5)
         else if property_type == 5
         {
            // Java: EscherArrayProperty arrayProp = new EscherArrayProperty(propId, new byte[propData]);
            let array_prop = EscherArrayProperty::new(prop_id, vec![0u8; prop_data as usize]);
            // Java: results.put(Integer.valueOf(arrayProp.getPropertyNumber()), arrayProp);
            let key = array_prop.base.base.getPropertyNumber() as i32;
            results.insert(key, EscherPropertyVariant::Array(array_prop));
         }
         // Java: else
         else
         {
            // Java: EscherComplexProperty complexProp = new EscherComplexProperty(propId, new byte[propData]);
            let complex_prop = EscherComplexProperty::new(prop_id, vec![0u8; prop_data as usize]);
            // Java: results.put(Integer.valueOf(complexProp.getPropertyNumber()), complexProp);
            let key = complex_prop.base.getPropertyNumber() as i32;
            results.insert(key, EscherPropertyVariant::Complex(complex_prop));
         }
         // Java: pos += 6;
         pos += 6;
      }
      // Java: for(Map.Entry<Integer, EscherProperty> entry : results.entrySet())
      for (_key, value) in results.iter_mut()
      {
         // Java: EscherProperty p = (EscherProperty)entry.getValue();
         // Java: if (p instanceof EscherComplexProperty)
         match value {
         EscherPropertyVariant::Array(ref mut arr) =>
         {
            // Java: if (p instanceof EscherArrayProperty)
            {
               // Java: pos += ((EscherArrayProperty)p).setArrayData(data, pos);
               pos += arr.setArrayData_stream(data, pos).unwrap_or(0);
            }
         }
         EscherPropertyVariant::Complex(ref mut cpx) =>
         // Java: else
         {
            {
               // Java: byte[] complexData = ((EscherComplexProperty)p).getComplexData();
               let complex_len = cpx._complexData.len() as i32;
               // Java: data.read(complexData, 0, complexData.length);
               data.read_off(&mut cpx._complexData, 0, complex_len);
               // Java: pos += complexData.length;
               pos += complex_len;
            }
         }
         _ => {}
         }
      }
      // Java: return results;
      results
    }

    // Java: IIntObjectMap<EscherProperty> createProperties(int numProperties, DocumentInputStream stream, int offset)
    pub fn createProperties_int_stream(&self, num_properties: i32, stream: &mut DocumentInputStream, offset: i32) -> HashMap<i16, EscherPropertyVariant> {
      // Java: IIntObjectMap<EscherProperty> result = new WIntObjectHashMap(numProperties);
      let mut result: HashMap<i16, EscherPropertyVariant> = HashMap::with_capacity(num_properties as usize);
      // Java: if (null == stream)
      if false
      {
         // Java: return result;
         return result;
      }
      // Java: else
      {
         // Java: int fopteSize = numProperties * 6;
         let fopte_size = num_properties * 6;
         // Java: PageBuffer pb = PageBuffer.obtain(fopteSize);
         // Java: byte[] buf = pb.getByteArray();
         let mut buf = vec![0u8; fopte_size as usize];
         // Java: stream.seek((long)offset);
         stream.seek(offset as i64);
         // Java: stream.read(buf, 0, fopteSize);
         stream.read_off(&mut buf, 0, fopte_size);
         // Java: int off = 0;
         let mut off: i32 = 0;
         // Java: for(int i = 0; i < numProperties; ++i)
         for _i in 0..num_properties
         {
            // Java: short propId = LittleEndian.getShort(buf, off);
            let prop_id = le_get_short(&buf, off as usize);
            // Java: off += 2;
            off += 2;
            // Java: int propData = LittleEndian.getInt(buf, off);
            let prop_data = le_get_int(&buf, off as usize);
            // Java: off += 4;
            off += 4;
            // Java: short propNumber = (short)(propId & 16383);
            let prop_number = prop_id & 16383;
            // Java: boolean isComplex = propId < 0;
            let is_complex = prop_id < 0;
            // Java: switch (EscherProperties.getPropertyType(propNumber))
            match EscherProperties::getPropertyType(prop_number)
            {
               // Java: case -1:
               -1 => {
                  // Java: break;
               }
               // Java: case 0:
               // Java: case 4:
               // Java: default:
               0 | 4 | _ => {
                  // Java: if (!isComplex)
                  if !is_complex
                  {
                     // Java: result.put(propNumber, new EscherSimpleProperty(propId, propData));
                     result.insert(prop_number, EscherPropertyVariant::Simple(EscherSimpleProperty::new(prop_id, prop_data)));
                  }
                  // Java: else if (propData > 0)
                  else if prop_data > 0
                  {
                     // Java: byte[] data = new byte[propData];
                     let mut x_data = vec![0u8; prop_data as usize];
                     // Java: stream.read(data);
                     stream.read_off(&mut x_data, 0, prop_data);
                     // Java: result.put(propNumber, new EscherComplexProperty(propId, data));
                     result.insert(prop_number, EscherPropertyVariant::Complex(EscherComplexProperty::new(prop_id, x_data)));
                  }
                  // Java: break;
               }
               // Java: case 1:
               1 => {
                  // Java: result.put(propNumber, new EscherBoolProperty(propId, propData));
                  result.insert(prop_number, EscherPropertyVariant::Bool(EscherBoolProperty::new(prop_id, prop_data)));
                  // Java: break;
               }
               // Java: case 2:
               2 => {
                  // Java: result.put(propNumber, new EscherRGBProperty(propId, propData));
                  result.insert(prop_number, EscherPropertyVariant::RGB(EscherRGBProperty::new(prop_id, prop_data)));
                  // Java: break;
               }
               // Java: case 3:
               3 => {
                  // Java: result.put(propNumber, new EscherShapePathProperty(propId, propData));
                  result.insert(prop_number, EscherPropertyVariant::ShapePath(EscherShapePathProperty::new(prop_id, prop_data)));
                  // Java: break;
               }
               // Java: case 5:
               5 => {
                  // Java: if (isComplex)
                  if is_complex
                  {
                     // Java: EscherArrayProperty prop = new EscherArrayProperty(propId, new byte[propData]);
                     let mut prop = EscherArrayProperty::new(prop_id, vec![0u8; prop_data as usize]);
                     // Java: prop.setArrayData(stream, (int)stream.tell());
                     let tell_pos = stream.tell() as i32;
                     let _ = prop.setArrayData_stream(stream, tell_pos);
                     // Java: result.put(propNumber, prop);
                     result.insert(prop_number, EscherPropertyVariant::Array(prop));
                  }
                  // Java: else
                  else
                  {
                     // Java: result.put(propNumber, new EscherSimpleProperty(propId, propData));
                     result.insert(prop_number, EscherPropertyVariant::Simple(EscherSimpleProperty::new(prop_id, prop_data)));
                  }
               }
            }
         }
         // Java: pb.recycle();
         // Java: return result;
         return result;
      }
    }

}
