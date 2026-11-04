// Java: import cn.wps.kfc.api.map.ILongObjectMap
use util::kfc::api::map::ILongObjectMap::*;
// Java: import java.io.UnsupportedEncodingException
use stubs::java::io::UnsupportedEncodingException::*;
// Java: import java.util.ArrayList
use stubs::java::util::ArrayList::*;
// Java: import java.util.Collections
use stubs::java::util::Collections::*;
// Java: import java.util.Iterator
use stubs::java::util::Iterator::*;
// Java: import java.util.List
use stubs::java::util::List::*;
// Java: import org.apache.poi.hpsf.wellknown.SectionIDMap
use crate::org::apache::poi::hpsf::wellknown::SectionIDMap::*;
// Java: import org.apache.poi.util.LittleEndian
use util::poi_util::LittleEndian::*;
// Java: import org.apache.poi.hpsf.ClassID
use crate::org::apache::poi::hpsf::ClassID::*;
// Java: import org.apache.poi.hpsf.Property
use crate::org::apache::poi::hpsf::Property::*;
// Java: import org.apache.poi.hpsf.HPSFRuntimeException
use crate::org::apache::poi::hpsf::HPSFRuntimeException::*;
use super::Util::Util;

use std::collections::HashMap;

type ILongObjectMap<T> = Option<HashMap<i64, String>>;

// Java: public class Section 
#[derive(Default)]
pub struct Section {
    // Java field:    protected ILongObjectMap<String> dictionary;
    pub dictionary: ILongObjectMap<String>,
    // Java field:    protected ClassID formatID;
    pub formatID: ClassID,
    // Java field:    protected long offset;
    pub offset: i64,
    // Java field:    protected int size;
    pub size: i32,
    // Java field:    protected Property[] properties;
    pub properties: Vec<Property>,
    // Java field:    private boolean wasNull;
    wasNull: bool,
}

// Java: class PropertyListEntry implements Comparable<PropertyListEntry> 
#[derive(Default, Clone)]
pub struct PropertyListEntry {
    // Java field:       int id;
    pub id: i32,
    // Java field:       int offset;
    pub offset: i32,
    // Java field:       int length;
    pub length: i32,
}

// Java: public class Section 
impl Section {
    // Java: public ClassID getFormatID() 
    pub fn getFormatID(&mut self) -> ClassID {
      // Java: return this.formatID;
      return self.formatID;
    }

    // Java: public long getOffset() 
    pub fn getOffset(&mut self) -> i64 {
      // Java: return this.offset;
      return self.offset;
    }

    // Java: public int getSize() 
    pub fn getSize(&mut self) -> i32 {
      // Java: return this.size;
      return self.size;
    }

    // Java: public int getPropertyCount() 
    pub fn getPropertyCount(&mut self) -> i32 {
      // Java: return this.properties.length;
      return self.properties.len() as i32;
    }

    // Java: public Property[] getProperties() 
    pub fn getProperties(&mut self) -> Vec<Property> {
      // Java: return this.properties;
      return self.properties.clone();
    }

    // Java: protected Section() 
    pub fn new() -> Section {
      Self { dictionary: Default::default(), formatID: Default::default(), offset: Default::default(), size: Default::default(), properties: Default::default(), wasNull: Default::default() }
    }

    // Java: public Section(byte[] src, int offset) throws UnsupportedEncodingException 
    pub fn new__bytearr_int(src: Vec<u8>, offset: i32) -> Result<Section, String> {
      let mut section = Section::new();
      section.formatID = ClassID::new_from_bytes(&src, offset);
      let mut o1 = offset as usize + 16;
      section.offset = util::poi_util::LittleEndian::getUInt(&src, o1 as i32) as i64;
      o1 = section.offset as usize;
      section.size = util::poi_util::LittleEndian::getUInt(&src, o1 as i32) as i32;
      o1 += 4;
      let property_count = util::poi_util::LittleEndian::getUInt(&src, o1 as i32) as i32;
      o1 += 4;
      section.properties = vec![Property::new(0, 0, PropertyValue::None); property_count as usize];
      let mut property_list: Vec<PropertyListEntry> = Vec::with_capacity(property_count as usize);
      let mut pass1_offset = o1;
      for _ in 0..property_count {
        let mut ple = PropertyListEntry::default();
        ple.id = util::poi_util::LittleEndian::getUInt(&src, pass1_offset as i32) as i32;
        pass1_offset += 4;
        ple.offset = util::poi_util::LittleEndian::getUInt(&src, pass1_offset as i32) as i32;
        pass1_offset += 4;
        property_list.push(ple);
      }
      property_list.sort_by(|a, b| {
        let mut aa = a.clone();
        aa.compareTo(b.clone())
      });
      for i in 0..(property_count - 1).max(0) {
        property_list[i as usize].length = property_list[i as usize + 1].offset - property_list[i as usize].offset;
      }
      if property_count > 0 {
        let last = property_list.len() - 1;
        property_list[last].length = section.size - property_list[last].offset;
      }
      let mut codepage = -1;
      for ple in &property_list {
        if codepage != -1 { break; }
        if ple.id == 1 {
          let o = (section.offset + ple.offset as i64) as usize;
          let prop_type = util::poi_util::LittleEndian::getUInt(&src, o as i32) as i64;
          let o = o + 4;
          if prop_type != 2 {
            return Err(format!("Value type of property ID 1 is not VT_I2 but {}.", prop_type));
          }
          codepage = util::poi_util::LittleEndian::getUShort(&src, o as i32);
        }
      }
      let mut i1 = 0usize;
      for ple in &property_list {
        let mut p = Property::new_from_bytes(ple.id as i64, &src, section.offset + ple.offset as i64, ple.length, codepage);
        if p.getID() == 1 {
          p = Property::new(p.getID(), p.getType(), PropertyValue::Int(codepage));
        }
        section.properties[i1] = p;
        i1 += 1;
      }
      if let Some(PropertyValue::Dictionary(dict)) = section.getProperty(0).cloned() {
        section.dictionary = Some(dict);
      }
      Ok(section)
    }

    // Java: public Object getProperty(long id) 
    pub fn getProperty(&mut self, id: i64) -> Option<PropertyValue> {
      // Java: this.wasNull = false;
      self.wasNull = false;
      let mut i: i32 = 0;
      // Java: for(int i = 0; i < this.properties.length; ++i)
      while (i as usize) < self.properties.len() {
         // Java: if (id == this.properties[i].getID())
         if id == self.properties[i as usize].getID() {
            // Java: return this.properties[i].getValue();
            return Some(self.properties[i as usize].getValue().clone());
         }
         i += 1;
      }
      // Java: this.wasNull = true;
      self.wasNull = true;
      // Java: return null;
      return None;
    }

    // Java: protected int getPropertyIntValue(long id) 
    pub fn getPropertyIntValue(&mut self, id: i64) -> i32 {
      // Java: Object o = this.getProperty(id);
      let o: Box<dyn std::any::Any> = self.getProperty(id);
      // Java: if (o == null)
      if o.is_none() {
         // Java: return 0;
         return 0;
      }
      // Java: else if (!(o instanceof Long) && !(o instanceof Integer))
      else if !o.downcast_ref::<i64>().is_some() && !o.downcast_ref::<i32>().is_some() {
         // Java: return 0;
         return 0;
      }
      // Java: else
      else {
         // Java: Number i = (Number)o;
         let i: Number = ((o) as Number);
         // Java: return i.intValue();
         return i.intValue();
      }
    }

    // Java: protected boolean getPropertyBooleanValue(int id) 
    pub fn getPropertyBooleanValue(&mut self, id: i32) -> bool {
      // Java: Object obj = this.getProperty((long)id);
      let obj: Box<dyn std::any::Any> = self.getProperty(((id) as i64));
      // Java: return null != obj && obj instanceof Boolean ? (Boolean)obj : false;
      return (if obj.is_some() && obj.downcast_ref::<bool>().is_some() { ((obj) as bool) } else { false });
    }

    // Java: public boolean wasNull() 
    pub fn wasNull(&mut self) -> bool {
      // Java: return this.wasNull;
      return self.wasNull;
    }

    // Java: public String getPIDString(long pid) 
    pub fn getPIDString(&mut self, pid: i64) -> String {
      // Java: String s = null;
      let s: String = None;
      // Java: if (this.dictionary != null)
      if self.dictionary.is_some() {
         // Java: s = (String)this.dictionary.get(Long.valueOf(pid));
         s = ((self.dictionary.get(Long::valueOf(pid))) as String);
      }
      // Java: if (s == null)
      if s.is_none() {
         // Java: s = SectionIDMap.getPIDString(this.getFormatID().getBytes(), pid);
         s = SectionIDMap::getPIDString(self.getFormatID().getBytes(), pid);
      }
      // Java: if (s == null)
      if s.is_none() {
         // Java: s = "[undefined]";
         s = "[undefined]";
      }
      // Java: return s;
      return s;
    }

    // Java: public boolean equals(Object o) 
    pub fn equals(&mut self, o: Box<dyn std::any::Any>) -> bool {
      // Java: if (o != null && o instanceof Section)
      if o.is_some() && o.downcast_ref::<Section>().is_some() {
         // Java: Section s = (Section)o;
         let s: Section = ((o) as Section);
         // Java: if (!s.getFormatID().equals(this.getFormatID()))
         if !s.getFormatID().equals(self.getFormatID()) {
            // Java: return false;
            return false;
         }
         // Java: else
         else {
            // Java: Property[] pa1 = new Property[this.getProperties().length];
            let pa1: Vec<Property> = vec![Default::default(); self.getProperties().length];
            // Java: Property[] pa2 = new Property[s.getProperties().length];
            let pa2: Vec<Property> = vec![Default::default(); s.getProperties().length];
            // Java: System.arraycopy(this.getProperties(), 0, pa1, 0, pa1.length);
            System::arraycopy(self.getProperties(), 0, pa1, 0, pa1.length);
            // Java: System.arraycopy(s.getProperties(), 0, pa2, 0, pa2.length);
            System::arraycopy(s.getProperties(), 0, pa2, 0, pa2.length);
            // Java: Property p10 = null;
            let p10: Property = None;
            // Java: Property p20 = null;
            let p20: Property = None;
            let i: i32 = 0;
            // Java: for(int i = 0; i < pa1.length; ++i)
            while i < pa1.length {
               // Java: long id = pa1[i].getID();
               let id: i64 = pa1[i].getID();
               // Java: if (id == 0L)
               if id == 0 {
                  // Java: p10 = pa1[i];
                  p10 = pa1[i];
                  // Java: pa1 = this.remove(pa1, i);
                  pa1 = self.remove(pa1, i);
                  // Java: --i;
                  { let _tmp = i; i -= 1; _tmp };
               }
               // Java: if (id == 1L)
               if id == 1 {
                  // Java: pa1 = this.remove(pa1, i);
                  pa1 = self.remove(pa1, i);
                  // Java: --i;
                  { let _tmp = i; i -= 1; _tmp };
               }
               i += 1;
            }
            let i: i32 = 0;
            // Java: for(int i = 0; i < pa2.length; ++i)
            while i < pa2.length {
               // Java: long id = pa2[i].getID();
               let id: i64 = pa2[i].getID();
               // Java: if (id == 0L)
               if id == 0 {
                  // Java: p20 = pa2[i];
                  p20 = pa2[i];
                  // Java: pa2 = this.remove(pa2, i);
                  pa2 = self.remove(pa2, i);
                  // Java: --i;
                  { let _tmp = i; i -= 1; _tmp };
               }
               // Java: if (id == 1L)
               if id == 1 {
                  // Java: pa2 = this.remove(pa2, i);
                  pa2 = self.remove(pa2, i);
                  // Java: --i;
                  { let _tmp = i; i -= 1; _tmp };
               }
               i += 1;
            }
            // Java: if (pa1.length != pa2.length)
            if pa1.length != pa2.length {
               // Java: return false;
               return false;
            }
            // Java: else
            else {
               // Java: boolean dictionaryEqual = true;
               let dictionaryEqual: bool = true;
               // Java: if (p10 != null && p20 != null)
               if p10.is_some() && p20.is_some() {
                  // Java: dictionaryEqual = p10.getValue().equals(p20.getValue());
                  dictionaryEqual = p10.getValue().equals(p20.getValue());
               }
               // Java: else if (p10 != null || p20 != null)
               else if p10.is_some() || p20.is_some() {
                  // Java: dictionaryEqual = false;
                  dictionaryEqual = false;
               }
               // Java: return dictionaryEqual ? Util.equals(pa1, pa2) : false;
               return (if dictionaryEqual { Util::equals_properties(&pa1, &pa2) } else { false });
            }
         }
      }
      // Java: else
      else {
         // Java: return false;
         return false;
      }
    }

    // Java: private Property[] remove(Property[] pa, int i) 
    pub fn remove(&mut self, pa: Vec<Property>, i: i32) -> Vec<Property> {
      // Java: Property[] h = new Property[pa.length - 1];
      let h: Vec<Property> = vec![Default::default(); pa.length - 1];
      // Java: if (i > 0)
      if i > 0 {
         // Java: System.arraycopy(pa, 0, h, 0, i);
         System::arraycopy(pa, 0, h, 0, i);
      }
      // Java: System.arraycopy(pa, i + 1, h, i, h.length - i);
      System::arraycopy(pa, i + 1, h, i, h.length - i);
      // Java: return h;
      return h;
    }

    // Java: public int hashCode() 
    pub fn hashCode(&mut self) -> i32 {
      // Java: long hashCode = 0L;
      let hashCode: i64 = 0;
      // Java: hashCode += (long)this.getFormatID().hashCode();
      hashCode = hashCode + ((self.getFormatID().hashCode()) as i64);
      // Java: Property[] pa = this.getProperties();
      let pa: Vec<Property> = self.getProperties();
      let i: i32 = 0;
      // Java: for(int i = 0; i < pa.length; ++i)
      while i < pa.length {
         // Java: hashCode += (long)pa[i].hashCode();
         hashCode = hashCode + ((pa[i].hashCode()) as i64);
         i += 1;
      }
      // Java: int returnHashCode = (int)(hashCode & 4294967295L);
      let returnHashCode: i32 = ((hashCode & 4294967295) as i32);
      // Java: return returnHashCode;
      return returnHashCode;
    }

    // Java: public String toString() 
    pub fn toString(&mut self) -> String {
      // Java: StringBuffer b = new StringBuffer();
      let b: StringBuffer = StringBuffer::new();
      // Java: Property[] pa = this.getProperties();
      let pa: Vec<Property> = self.getProperties();
      // Java: b.append(this.getClass().getName());
      b.append(self.getClass().getName());
      // Java: b.append('[');
      b.append('[' as u16);
      // Java: b.append("formatID: ");
      b.append("formatID: ");
      // Java: b.append(this.getFormatID());
      b.append(self.getFormatID());
      // Java: b.append(", offset: ");
      b.append(", offset: ");
      // Java: b.append(this.getOffset());
      b.append(self.getOffset());
      // Java: b.append(", propertyCount: ");
      b.append(", propertyCount: ");
      // Java: b.append(this.getPropertyCount());
      b.append(self.getPropertyCount());
      // Java: b.append(", size: ");
      b.append(", size: ");
      // Java: b.append(this.getSize());
      b.append(self.getSize());
      // Java: b.append(", properties: [\n");
      b.append(", properties: [\n");
      let i: i32 = 0;
      // Java: for(int i = 0; i < pa.length; ++i)
      while i < pa.length {
         // Java: b.append(pa[i].toString());
         b.append(pa[i].toString());
         // Java: b.append(",\n");
         b.append(",\n");
         i += 1;
      }
      // Java: b.append(']');
      b.append(']' as u16);
      // Java: b.append(']');
      b.append(']' as u16);
      // Java: return b.toString();
      return b.toString();
    }

    // Java: public ILongObjectMap<String> getDictionary() 
    pub fn getDictionary(&mut self) -> ILongObjectMap<String> {
      // Java: return this.dictionary;
      return self.dictionary;
    }

    // Java: public int getCodepage() 
    pub fn getCodepage(&mut self) -> i32 {
      // Java: Integer codepage = (Integer)this.getProperty(1L);
      let codepage: i32 = ((self.getProperty(1)) as i32);
      // Java: if (codepage == null)
      if codepage.is_none() {
         // Java: return -1;
         return -1;
      }
      // Java: else
      else {
         // Java: int cp = codepage;
         let cp: i32 = codepage;
         // Java: return cp;
         return cp;
      }
    }

}

// Java: class PropertyListEntry implements Comparable<PropertyListEntry> 
impl PropertyListEntry {
    // Java: public int compareTo(PropertyListEntry o) 
    pub fn compareTo(&mut self, o: PropertyListEntry) -> i32 {
         // Java: int otherOffset = o.offset;
         let otherOffset: i32 = o.offset;
         // Java: if (this.offset < otherOffset)
         if self.offset < otherOffset {
            // Java: return -1;
            return -1;
         }
         // Java: else
         else {
            // Java: return this.offset == otherOffset ? 0 : 1;
            return (if self.offset == otherOffset { 0 } else { 1 });
         }
    }

    // Java: public String toString() 
    pub fn toString(&mut self) -> String {
         // Java: StringBuffer b = new StringBuffer();
         let b: StringBuffer = StringBuffer::new();
         // Java: b.append(this.getClass().getName());
         b.append(self.getClass().getName());
         // Java: b.append("[id=");
         b.append("[id=");
         // Java: b.append(this.id);
         b.append(self.id);
         // Java: b.append(", offset=");
         b.append(", offset=");
         // Java: b.append(this.offset);
         b.append(self.offset);
         // Java: b.append(", length=");
         b.append(", length=");
         // Java: b.append(this.length);
         b.append(self.length);
         // Java: b.append(']');
         b.append(']' as u16);
         // Java: return b.toString();
         return b.toString();
    }

}
