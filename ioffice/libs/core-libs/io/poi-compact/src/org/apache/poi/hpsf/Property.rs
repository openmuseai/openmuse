#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.kfc.api.map.ILongObjectMap
// Java: import cn.wps.kfc.impl.map.WLongObjectHashMap
// Java: import java.io.UnsupportedEncodingException
// Java: import java.nio.charset.StandardCharsets
// Java: import org.apache.poi.util.HexDump
// Java: import org.apache.poi.util.LittleEndian
use std::collections::HashMap;

fn le_get_uint(data: &[u8], offset: usize) -> i64 {
    (data[offset] as i64 & 0xFF)
        | ((data[offset + 1] as i64 & 0xFF) << 8)
        | ((data[offset + 2] as i64 & 0xFF) << 16)
        | ((data[offset + 3] as i64 & 0xFF) << 24)
}

// Java field: String TAG
const TAG: Option<&str> = None;

#[derive(Clone, Debug)]
pub enum PropertyValue {
    Str(String),
    Long(i64),
    Int(i32),
    Double(f64),
    Bool(bool),
    // Native object transport for java.util.Date; retain mutable reference identity.
    Date(std::sync::Arc<util::java_runtime::Date>),
    Bytes(Vec<u8>),
    Dictionary(HashMap<i64, String>),
    None,
}

// Java: public class Property
pub struct Property {
    // Java field: long id
    pub(crate) id: i64,
    // Java field: long type
    pub(crate) type_: i64,
    // Java field: Object value
    pub(crate) value: PropertyValue,
}

// Java: public class Property
impl Property {
    // Java: long getID()
    pub fn getID(&self) -> i64 {
        // Java: return this.id;
        self.id
    }

    // Java: long getType()
    pub fn getType(&self) -> i64 {
        // Java: return this.type;
        self.type_
    }

    // Java: Object getValue()
    pub fn getValue(&self) -> &PropertyValue {
        // Java: return this.value;
        &self.value
    }

    // Java: Property(long id, long type, Object value)
    pub fn new(id: i64, type_: i64, value: PropertyValue) -> Self {
        // Java: super();
        // Java: this.id = id;
        // Java: this.type = type;
        // Java: this.value = value;
        Self { id, type_, value }
    }

    // Java: Property(long id, byte[] src, long offset, int length, int codepage)
    pub fn new_from_bytes(id: i64, src: &[u8], offset: i64, length: i32, codepage: i32) -> Self {
        // Java: super();
        // Java: this.id = id;
        let mut prop = Self { id, type_: 0, value: PropertyValue::None };
        // Java: if (id == 0L)
        if id == 0
        {
            // Java: this.value = this.readDictionary(src, offset, length, codepage);
            prop.value = PropertyValue::Dictionary(Self::read_dictionary(src, offset, length, codepage));
        }
        // Java: else
        else
        {
            // Java: int o = (int)offset;
            let mut o = offset as usize;
            // Java: this.type = LittleEndian.getUInt(src, o);
            prop.type_ = le_get_uint(src, o);
            // Java: o += 4;
            o += 4;
            // Java: try
            // Java: this.value = VariantSupport.read(src, o, length, (long)((int)this.type), codepage);
            prop.value = PropertyValue::None;
            // Java: catch (UnsupportedVariantTypeException ex)
            // Java: this.value = ex.getValue();
        }
        prop
    }

    // Java: Property()
    pub fn new_default() -> Self {
        // Java: super();
        Self { id: 0, type_: 0, value: PropertyValue::None }
    }

    // Java: ILongObjectMap<String> readDictionary(byte[] src, long offset, int length, int codepage)
    fn read_dictionary(src: &[u8], offset: i64, length: i32, codepage: i32) -> HashMap<i64, String> {
        // Java: if (offset >= 0L && offset <= (long)src.length)
        if offset >= 0 && offset <= src.len() as i64
        {
            // Java: int o = (int)offset;
            let mut o = offset as usize;
            // Java: long nrEntries = LittleEndian.getUInt(src, o);
            let nr_entries = le_get_uint(src, o);
            // Java: o += 4;
            o += 4;
            // Java: ILongObjectMap<String> m = new WLongObjectHashMap((int)nrEntries, 1.0F);
            let mut m: HashMap<i64, String> = HashMap::with_capacity(nr_entries as usize);
            // Java: try
            {
                // Java: for(int i = 0; (long)i < nrEntries; ++i)
                for _i in 0..nr_entries
                {
                    // Java: long id = LittleEndian.getUInt(src, o);
                    let id = le_get_uint(src, o);
                    // Java: o += 4;
                    o += 4;
                    // Java: long sLength = LittleEndian.getUInt(src, o);
                    let mut s_length = le_get_uint(src, o);
                    // Java: o += 4;
                    o += 4;
                    // Java: StringBuffer b = new StringBuffer();
                    let mut b = String::new();
                    // Java: switch (codepage)
                    match codepage
                    {
                        // Java: case -1:
                        -1 => {
                            // Java: b.append(new String(src, o, (int)sLength, StandardCharsets.UTF_8));
                            if let Ok(s) = std::str::from_utf8(&src[o..o + s_length as usize]) {
                                b.push_str(s);
                            }
                            // Java: break;
                        }
                        // Java: case 1200:
                        1200 => {
                            // Java: int nrBytes = (int)(sLength * 2L);
                            let nr_bytes = (s_length * 2) as usize;
                            // Java: byte[] h = new byte[nrBytes];
                            let mut h = vec![0u8; nr_bytes];
                            // Java: for(int i2 = 0; i2 < nrBytes; i2 += 2)
                            let mut i2 = 0;
                            while i2 < nr_bytes
                            {
                                // Java: h[i2] = src[o + i2 + 1];
                                h[i2] = src[o + i2 + 1];
                                // Java: h[i2 + 1] = src[o + i2];
                                h[i2 + 1] = src[o + i2];
                                i2 += 2;
                            }
                            // Java: b.append(new String(h, 0, nrBytes, VariantSupport.codepageToEncoding(codepage)));
                            let decoded: Vec<u16> = h.chunks(2)
                                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                                .collect();
                            if let Ok(s) = String::from_utf16(&decoded) {
                                b.push_str(&s);
                            }
                            // Java: break;
                        }
                        // Java: default:
                        _ => {
                            // Java: b.append(new String(src, o, (int)sLength, VariantSupport.codepageToEncoding(codepage)));
                            if let Ok(s) = std::str::from_utf8(&src[o..o + s_length as usize]) {
                                b.push_str(s);
                            }
                        }
                    }
                    // Java: while(b.length() > 0 && b.charAt(b.length() - 1) == 0)
                    while !b.is_empty() && b.ends_with('\0')
                    {
                        // Java: b.setLength(b.length() - 1);
                        b.pop();
                    }
                    // Java: if (codepage == 1200)
                    if codepage == 1200
                    {
                        // Java: if (sLength % 2L == 1L)
                        if s_length % 2 == 1
                        {
                            // Java: ++sLength;
                            s_length += 1;
                        }
                        // Java: o = (int)((long)o + sLength + sLength);
                        o = (o as i64 + s_length + s_length) as usize;
                    }
                    // Java: else
                    else
                    {
                        // Java: o = (int)((long)o + sLength);
                        o = (o as i64 + s_length) as usize;
                    }
                    // Java: m.put(id, b.toString());
                    m.insert(id, b);
                }
            }
            // Java: catch (RuntimeException ex)
            // Java: IOLog.w(TAG, "The property set's dictionary contains bogus data. All dictionary entries starting with the one with ID " + this.id + " will be ignored.", ex);
            // Java: return m;
            m
        }
        // Java: else
        else
        {
            // Java: throw new HPSFRuntimeException("Illegal offset " + offset + " while HPSF stream contains " + length + " bytes.");
            panic!("Illegal offset {} while HPSF stream contains {} bytes.", offset, length);
        }
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: int length = VariantSupport.getVariantLength(this.type);
        let mut length = get_variant_length(self.type_);
        // Java: if (length >= 0)
        if length >= 0
        {
            // Java: return length;
            return length;
        }
        // Java: else if (length == -2)
        else if length == -2
        {
            // Java: throw new WritingNotSupportedException(this.type, null);
            panic!("WritingNotSupportedException: type {}", self.type_);
        }
        // Java: else
        else
        {
            // Java: int PADDING = 4;
            let _padding = 4;
            // Java: switch ((int)this.type)
            match self.type_ as i32
            {
                // Java: case 30:
                30 => {
                    // Java: int l = ((String)this.value).length() + 1;
                    let mut l = match &self.value {
                        PropertyValue::Str(s) => s.len() as i32 + 1,
                        _ => 1,
                    };
                    // Java: int r = l % 4;
                    let r = l % 4;
                    // Java: if (r > 0)
                    if r > 0
                    {
                        // Java: l += 4 - r;
                        l += 4 - r;
                    }
                    // Java: length += l;
                    length += l;
                    length
                }
                // Java: case 0:
                0 => {
                    // Java: return length;
                    length
                }
                // Java: default:
                _ => {
                    // Java: throw new WritingNotSupportedException(this.type, this.value);
                    panic!("WritingNotSupportedException: type {}", self.type_);
                }
            }
        }
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Property) -> bool {
        // Java: if (!(o instanceof Property))
        // Java: return false;
        // Java: else
        {
            // Java: Property p = (Property)o;
            // Java: Object pValue = p.getValue();
            // Java: long pId = p.getID();
            let p_id = o.getID();
            // Java: if (this.id == pId && (this.id == 0L || this.typesAreEqual(this.type, p.getType())))
            if self.id == p_id && (self.id == 0 || self.typesAreEqual(self.type_, o.getType()))
            {
                // Java: if (this.value == null && pValue == null)
                if matches!(self.value, PropertyValue::None) && matches!(o.value, PropertyValue::None)
                {
                    // Java: return true;
                    return true;
                }
                // Java: else if (this.value != null && pValue != null)
                else if !matches!(self.value, PropertyValue::None) && !matches!(o.value, PropertyValue::None)
                {
                    // Java: Class<?> valueClass = this.value.getClass();
                    // Java: Class<?> pValueClass = pValue.getClass();
                    // Java: if (!valueClass.isAssignableFrom(pValueClass) && !pValueClass.isAssignableFrom(valueClass))
                    // Java: return false;
                    // Java: else
                    // Java: return this.value instanceof byte[] ? Util.equal((byte[])this.value, (byte[])pValue) : this.value.equals(pValue);
                    match (&self.value, &o.value) {
                        (PropertyValue::Bytes(a), PropertyValue::Bytes(b)) => a == b,
                        (PropertyValue::Str(a), PropertyValue::Str(b)) => a == b,
                        (PropertyValue::Long(a), PropertyValue::Long(b)) => a == b,
                        (PropertyValue::Int(a), PropertyValue::Int(b)) => a == b,
                        (PropertyValue::Double(a), PropertyValue::Double(b)) => a == b,
                        (PropertyValue::Bool(a), PropertyValue::Bool(b)) => a == b,
                        (PropertyValue::Date(a), PropertyValue::Date(b)) => a.getTime() == b.getTime(),
                        _ => false,
                    }
                }
                // Java: else
                else
                {
                    // Java: return false;
                    false
                }
            }
            // Java: else
            else
            {
                // Java: return false;
                false
            }
        }
    }

    // Java: boolean typesAreEqual(long t1, long t2)
    pub fn typesAreEqual(&self, t1: i64, t2: i64) -> bool {
        // Java: return t1 == t2 || t1 == 30L && t2 == 31L || t2 == 30L && t1 == 31L;
        t1 == t2 || (t1 == 30 && t2 == 31) || (t2 == 30 && t1 == 31)
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: long hashCode = 0L;
        let mut hash_code: i64 = 0;
        // Java: hashCode += this.id;
        hash_code += self.id;
        // Java: hashCode += this.type;
        hash_code += self.type_;
        // Java: if (this.value != null)
        if !matches!(self.value, PropertyValue::None)
        {
            // Java: hashCode += (long)this.value.hashCode();
            hash_code += match &self.value {
                PropertyValue::Str(s) => {
                    let mut h: i64 = 0;
                    for b in s.bytes() { h = h.wrapping_mul(31).wrapping_add(b as i64); }
                    h
                },
                PropertyValue::Long(v) => *v,
                PropertyValue::Int(v) => *v as i64,
                PropertyValue::Double(v) => v.to_bits() as i64,
                PropertyValue::Bool(v) => if *v { 1 } else { 0 },
                PropertyValue::Date(v) => {let t=v.getTime();(t^(t as u64>>32)as i64)as i32 as i64},
                _ => 0,
            };
        }
        // Java: int returnHashCode = (int)(hashCode & 4294967295L);
        let return_hash_code = (hash_code & 0xFFFFFFFF) as i32;
        // Java: return returnHashCode;
        return_hash_code
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuffer b = new StringBuffer();
        let mut b = String::new();
        // Java: b.append(this.getClass().getName());
        b.push_str("Property");
        // Java: b.append('[');
        b.push('[');
        // Java: b.append("id: ");
        b.push_str("id: ");
        // Java: b.append(this.getID());
        b.push_str(&self.getID().to_string());
        // Java: b.append(", type: ");
        b.push_str(", type: ");
        // Java: b.append(this.getType());
        b.push_str(&self.getType().to_string());
        // Java: Object value = this.getValue();
        let value = self.getValue();
        // Java: b.append(", value: ");
        b.push_str(", value: ");
        // Java: b.append(value.toString());
        b.push_str(&format!("{:?}", value));
        // Java: if (value instanceof String)
        if let PropertyValue::Str(s) = value
        {
            // Java: String s = (String)value;
            // Java: int l = s.length();
            let l = s.len();
            // Java: byte[] bytes = new byte[l * 2];
            let mut bytes = vec![0u8; l * 2];
            // Java: for(int i = 0; i < l; ++i)
            for (i, ch) in s.chars().enumerate()
            {
                // Java: char c = s.charAt(i);
                // Java: byte high = (byte)((c & '\uff00') >> 8);
                let high = ((ch as u16 & 0xFF00) >> 8) as u8;
                // Java: byte low = (byte)((c & 255) >> 0);
                let low = (ch as u16 & 0xFF) as u8;
                // Java: bytes[i * 2] = high;
                if i * 2 + 1 < bytes.len() {
                    bytes[i * 2] = high;
                    // Java: bytes[i * 2 + 1] = low;
                    bytes[i * 2 + 1] = low;
                }
            }
            // Java: String hex = HexDump.dump(bytes, 0L, 0);
            let hex = format!("{:?}", bytes);
            // Java: b.append(" [");
            b.push_str(" [");
            // Java: b.append(hex);
            b.push_str(&hex);
            // Java: b.append("]");
            b.push(']');
        }
        // Java: b.append(']');
        b.push(']');
        // Java: return b.toString();
        b
    }
}

fn get_variant_length(type_: i64) -> i32 {
    match type_ as i32 {
        0 => 0,     // VT_EMPTY
        2 => 2,     // VT_I2
        3 => 4,     // VT_I4
        4 => 4,     // VT_R4
        5 => 8,     // VT_R8
        11 => 4,    // VT_BOOL
        20 => 8,    // VT_I8
        21 => 8,    // VT_UI8
        30 => -1,   // VT_LPSTR (variable)
        31 => -1,   // VT_LPWSTR (variable)
        64 => 8,    // VT_FILETIME
        65 => -1,   // VT_BLOB (variable)
        _ => -2,    // unsupported
    }
}
