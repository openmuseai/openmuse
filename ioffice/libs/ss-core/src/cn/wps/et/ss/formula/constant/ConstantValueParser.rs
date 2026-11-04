// java package cn.wps.et.ss.formula.constant;

#[derive(Clone, Debug, PartialEq)]
pub enum ConstantValue {
    Empty,
    Number(f64),
    String(String),
    // java String can retain unmatched UTF-16 surrogate code units.
    StringUtf16(Vec<u16>),
    Boolean(bool),
    Error(u16),
}

// java public final class ConstantValueParser
pub struct ConstantValueParser;

impl ConstantValueParser {
    // java public static Object[] parse(LittleEndianInput in, int nValues)
    pub fn parse(data: &[u8], offset: &mut usize, n_values: usize) -> Result<Vec<ConstantValue>, ()> {
        // java Object[] result = new Object[nValues]; for each readAConstantValue(in).
        let mut result = Vec::with_capacity(n_values);
        for _ in 0..n_values {
            result.push(Self::readAConstantValue(data, offset)?);
        }
        Ok(result)
    }

    // java private static Object readAConstantValue(LittleEndianInput in)
    fn readAConstantValue(data: &[u8], offset: &mut usize) -> Result<ConstantValue, ()> {
        // java byte grbit = in.readByte(); switch (grbit)
        let grbit = read::<1>(data, offset)?[0];
        match grbit {
            // java case 0: in.readLong(); return EMPTY_REPRESENTATION;
            0 => { read::<8>(data, offset)?; Ok(ConstantValue::Empty) }
            // java case 1: return new Double(in.readDouble());
            1 => Ok(ConstantValue::Number(f64::from_le_bytes(read::<8>(data, offset)?))),
            // java case 2: return StringUtil.readUnicodeString(in);
            2 => {
                let len = u16::from_le_bytes(read::<2>(data, offset)?) as usize;
                // java StringUtil.readUnicodeString: zero chars and no remaining bytes does not read a flag.
                if len == 0 && *offset == data.len() { return Ok(ConstantValue::String(String::new())); }
                let wide = read::<1>(data, offset)?[0] & 1 != 0;
                let mut units = Vec::with_capacity(len);
                for _ in 0..len {
                    units.push(if wide { u16::from_le_bytes(read::<2>(data, offset)?) } else { read::<1>(data, offset)?[0] as u16 });
                }
                // java StringUtil.readUnicodeString constructs a Java String from code units,
                // including unpaired surrogates; preserve those units for exact re-encoding.
                Ok(match String::from_utf16(&units) {
                    Ok(text) => ConstantValue::String(text),
                    Err(_) => ConstantValue::StringUtf16(units),
                })
            }
            // java case 4: return readBoolean(in); any nonzero low byte becomes true.
            4 => Ok(ConstantValue::Boolean(read::<8>(data, offset)?[0] != 0)),
            // java case 16: readUShort error; readUShort; readInt; ErrorConstant.valueOf(errCode).
            16 => {
                let code = u16::from_le_bytes(read::<2>(data, offset)?);
                read::<2>(data, offset)?;
                read::<4>(data, offset)?;
                Ok(ConstantValue::Error(code))
            }
            // java default: throw new RuntimeException("Unknown grbit value").
            _ => Err(()),
        }
    }

    // java public static int getEncodedSize(Object[] values)
    pub fn getEncodedSize(values: &[ConstantValue]) -> usize {
        values.iter().map(|value| match value {
            ConstantValue::String(text) => 1 + 3 + text.encode_utf16().count() * if multibyte(text) { 2 } else { 1 },
            ConstantValue::StringUtf16(units) => 1 + 3 + units.len() * if units.iter().any(|unit| *unit > 0xff) { 2 } else { 1 },
            _ => 9,
        }).sum()
    }

    // java public static void encode(LittleEndianOutput out, Object[] values)
    pub fn encode(out: &mut Vec<u8>, values: &[ConstantValue]) {
        for value in values { Self::encodeSingleValue(out, value); }
    }

    // java private static void encodeSingleValue(LittleEndianOutput out, Object value)
    fn encodeSingleValue(out: &mut Vec<u8>, value: &ConstantValue) {
        match value {
            // java EMPTY_REPRESENTATION: writeByte(0); writeLong(0L).
            ConstantValue::Empty => { out.push(0); out.extend_from_slice(&[0; 8]); }
            // java Boolean: writeByte(4); writeLong(b ? 1L : 0L).
            ConstantValue::Boolean(b) => { out.push(4); out.extend_from_slice(&(if *b { 1u64 } else { 0 }).to_le_bytes()); }
            // java Double: writeByte(1); writeDouble(dVal).
            ConstantValue::Number(n) => { out.push(1); let bits=if n.is_nan(){0x7ff8000000000000}else{n.to_bits()};out.extend_from_slice(&bits.to_le_bytes()); }
            // java String: writeByte(2); StringUtil.writeUnicodeString(out, val).
            ConstantValue::String(text) => {
                out.push(2);
                let units: Vec<u16> = text.encode_utf16().collect();
                out.extend_from_slice(&(units.len() as u16).to_le_bytes());
                let wide = multibyte(text);
                out.push(if wide { 1 } else { 0 });
                for unit in units { if wide { out.extend_from_slice(&unit.to_le_bytes()); } else { out.push(unit as u8); } }
            }
            // java StringUtil.writeUnicodeString writes the original UTF-16 code units.
            ConstantValue::StringUtf16(units) => {
                out.push(2);
                out.extend_from_slice(&(units.len() as u16).to_le_bytes());
                let wide = units.iter().any(|unit| *unit > 0xff);
                out.push(if wide { 1 } else { 0 });
                for unit in units {
                    if wide { out.extend_from_slice(&unit.to_le_bytes()); }
                    else { out.push(*unit as u8); }
                }
            }
            // java ErrorConstant: writeByte(16); writeLong(errorCode).
            ConstantValue::Error(code) => { out.push(16); out.extend_from_slice(&(*code as u64).to_le_bytes()); }
        }
    }
}

fn read<const N: usize>(data: &[u8], offset: &mut usize) -> Result<[u8; N], ()> {
    let end = offset.checked_add(N).ok_or(())?;
    let bytes = data.get(*offset..end).ok_or(())?.try_into().map_err(|_| ())?;
    *offset = end;
    Ok(bytes)
}

fn multibyte(text: &str) -> bool { text.encode_utf16().any(|unit| unit > 0xff) }
// Original Object[] facade for CRN records; existing token enums are not used
// to collapse Java Integer into ErrorConstant or to copy the caller's array.
impl ConstantValueParser{
    // java public static int getEncodedSize(Object[] values){int result=values.length*1;for(int i=0;i<values.length;++i){result+=getEncodedSize(values[i]);}return result;}
    pub fn getEncodedSize__objects(values:&[crate::java_runtime::JavaObject::NullableObject])->i32{let mut result=values.len()as i32;for i in 0..values.len(){result=result.wrapping_add(Self::getEncodedSize__object(values[i].as_ref()));}result}
    // java private static int getEncodedSize(Object object){if(object==EMPTY_REPRESENTATION){return 8;}else{Class cls=object.getClass();if(cls!=Boolean.class&&cls!=Double.class&&cls!=ErrorConstant.class){String strVal=(String)object;return StringUtil.getEncodedSize(strVal);}else{return 8;}}}
    fn getEncodedSize__object(object:Option<&crate::java_runtime::JavaObject::ObjectRef>)->i32{let Some(object)=object else{return 8;};let cls=object.as_any();if !cls.is::<bool>()&&!cls.is::<f64>()&&!cls.is::<super::ErrorConstant::ErrorConstant>(){let strVal=cls.downcast_ref::<String>().expect("ClassCastException");util::org::apache::poi::util::StringUtil::StringUtil::getEncodedSize(strVal)}else{8}}
    // java public static void encode(LittleEndianOutput out,Object[] values){for(int i=0;i<values.length;++i){encodeSingleValue(out,values[i]);}}
    pub fn encode__objects(out:&mut dyn util::org::apache::poi::util::LittleEndianOutput::LittleEndianOutput,values:&[crate::java_runtime::JavaObject::NullableObject]){for i in 0..values.len(){Self::encodeSingleValue__object(out,values[i].as_ref());}}
    // java private static void encodeSingleValue(LittleEndianOutput out,Object value) {
    fn encodeSingleValue__object(out:&mut dyn util::org::apache::poi::util::LittleEndianOutput::LittleEndianOutput,value:Option<&crate::java_runtime::JavaObject::ObjectRef>){
        // java if(value==EMPTY_REPRESENTATION){out.writeByte(0);out.writeLong(0L);}
        let Some(value)=value else{out.writeByte(0);out.writeLong(0);return;};let value=value.as_any();
        // java else if(value instanceof Boolean){Boolean bVal=(Boolean)value;out.writeByte(4);long longVal=bVal?1L:0L;out.writeLong(longVal);}
        if let Some(bVal)=value.downcast_ref::<bool>(){out.writeByte(4);let longVal=if *bVal{1}else{0};out.writeLong(longVal);}
        // java else if(value instanceof Double){Double dVal=(Double)value;out.writeByte(1);out.writeDouble(dVal);}
        else if let Some(dVal)=value.downcast_ref::<f64>(){out.writeByte(1);out.writeDouble(*dVal);}
        // java else if(value instanceof String){String val=(String)value;out.writeByte(2);StringUtil.writeUnicodeString(out,val);}
        else if let Some(val)=value.downcast_ref::<String>(){out.writeByte(2);util::org::apache::poi::util::StringUtil::StringUtil::writeUnicodeString(out,val);}
        // java else if(value instanceof ErrorConstant){ErrorConstant ecVal=(ErrorConstant)value;out.writeByte(16);long longVal=(long)ecVal.getErrorCode();out.writeLong(longVal);}
        else if let Some(ecVal)=value.downcast_ref::<super::ErrorConstant::ErrorConstant>(){out.writeByte(16);let longVal=ecVal.getErrorCode()as i64;out.writeLong(longVal);}
        // java else{throw new IllegalStateException("Unexpected value type ("+value.getClass().getName()+"'");}
        else{panic!("IllegalStateException Unexpected value type");}
    }
}
