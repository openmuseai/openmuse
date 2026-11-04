// Java: import cn.wps.base.assertion.Assert
// Java: import cn.wps.base.log.Log
// Java: import java.math.BigDecimal
// Java: import java.math.BigInteger

// Java: public class TypeConversion
pub struct TypeConversion;

// Java: public class TypeConversion
impl TypeConversion {
    // Java field: String TAG
    pub const TAG: &str = "TypeConversion";
    // Java field: String BIGDECIMAL_LOWERCASE_CHAR
    const BIGDECIMAL_LOWERCASE_CHAR: &str = "e";
    // Java field: String BIGDECIMAL_UPPRTCASE_CHAR
    const BIGDECIMAL_UPPRTCASE_CHAR: &str = "E";

    // Java: TypeConversion()
    pub fn new() -> Self {
        // Java: super();
        TypeConversion
    }

    // Java: static BigInteger string2BigInteger(String str)
    pub fn string2BigInteger(str: &str) -> Option<i128> {
        // Java: Assert.assertNotNull("str should not be null!", str);
        // Java: str = str.trim();
        let str = str.trim();
        // Java: BigInteger bigInt = null;
        // Java: try
        {
            // Java: bigInt = new BigInteger(str);
        }
        // Java: catch (NumberFormatException e)
        {
            // Java: Log.e(TAG, "NumberFormatException for String: " + str, e);
        }
        // Java: catch (Exception e)
        {
            // Java: Log.e(TAG, "Exception for String: " + str, e);
        }
        // Java: return bigInt;
        str.parse::<i128>().ok()
    }

    // Java: static Integer string2Integer(String str)
    pub fn string2Integer(str: &str) -> Option<i32> {
        // Java: Assert.assertNotNull("str should not be null!", str);
        // java Long l = null; try {
        let l=(||{
            // java if (str.contains("e") || str.contains("E")) {
            let converted;
            let str=if str.contains('e')||str.contains('E'){
                // java BigDecimal bd = new BigDecimal(str); str = bd.toPlainString();
                converted=crate::java_runtime::DecimalText::toPlainString(str)?;converted.as_str()
            }else{str};
            // java str = str.trim();
            let str=str.trim_matches(|c:char|c<='\u{20}');
            // java l = Long.parseLong(str);
            kfc_kfc_alpha::native_runtime::DecimalInteger::parseLong(&str.encode_utf16().collect::<Vec<_>>())
        })();
        // java } catch (NumberFormatException e) { Log.e(TAG, "NumberFormatException for String: " + str, e); }
        // java catch (Exception e) { Log.e(TAG, "Exception for String: " + str, e); }
        if l.is_none(){eprintln!("{}: NumberFormatException for String: {}",Self::TAG,str);}
        // java Integer i = null != l ? l.intValue() : null; return i;
        l.map(|n|n as i32)
    }

    // Java: static Character string2Character(String str)
    pub fn string2Character(str: &str) -> char {
        // Java: Assert.assertNotNull("str should not be null!", str);
        // Java: return 0 == str.length() ? '\u0000' : str.charAt(0);
        if str.is_empty() { '\0' } else { str.chars().next().unwrap() }
    }

    // Java: static Integer hexString2Integer(String str)
    pub fn hexString2Integer(str: &str) -> Option<i32> {
        // Java: Assert.assertNotNull("str should not be null!", str);
        // Java: str = str.trim();
        let str = str.trim_matches(|c:char|c as u32<=32);
        // Java: int RADIX = 16;
        // Java: Long l = null;
        // Java: try
        {
            // Java: l = Long.parseLong(str, 16);
        }
        // Java: catch (NumberFormatException e)
        {
            // Java: Log.e(TAG, "NumberFormatException for String: " + str, e);
        }
        // Java: catch (Exception e)
        {
            // Java: Log.e(TAG, "Exception for String: " + str, e);
        }
        // Java: Integer i = null != l ? l.intValue() : null;
        // Java: return i;
        kfc_kfc_alpha::native_runtime::DecimalInteger::parseLongRadix(&str.encode_utf16().collect::<Vec<_>>(),16).map(|v|v as i32)
    }

    // Java: static Long string2Long(String str)
    pub fn string2Long(str: &str) -> Option<i64> {
        // Java: Assert.assertNotNull("str should not be null!", str);
        // Java: str = str.trim();
        let str = str.trim();
        // Java: Long l = null;
        // Java: try
        {
            // Java: l = Long.parseLong(str);
        }
        // Java: catch (NumberFormatException e)
        {
            // Java: Log.e(TAG, "NumberFormatException for String: " + str, e);
        }
        // Java: catch (Exception e)
        {
            // Java: Log.e(TAG, "Exception for String: " + str, e);
        }
        // Java: return l;
        str.parse::<i64>().ok()
    }

    // Java: static Long hexString2Long(String str)
    pub fn hexString2Long(str: &str) -> Option<i64> {
        // Java: Assert.assertNotNull("str should not be null!", str);
        // Java: str = str.trim();
        let str = str.trim();
        // Java: int RADIX = 16;
        // Java: Long l = null;
        // Java: try
        {
            // Java: l = Long.parseLong(str, 16);
        }
        // Java: catch (NumberFormatException e)
        {
            // Java: Log.e(TAG, "NumberFormatException for String: " + str, e);
        }
        // Java: catch (Exception e)
        {
            // Java: Log.e(TAG, "Exception for String: " + str, e);
        }
        // Java: return l;
        i64::from_str_radix(str, 16).ok()
    }

    // Java: static Float string2Float(String str)
    pub fn string2Float(str: &str) -> Option<f32> {
        // Java: Assert.assertNotNull("str should not be null!", str);
        // Java: str = str.trim();
        let str = str.trim();
        // Java: if (null != str && str.length() > 0)
        if !str.is_empty() {
            // Java: Float f = null;
            // Java: try
            {
                // Java: f = Float.parseFloat(str);
            }
            // Java: catch (NumberFormatException e)
            {
                // Java: Log.e(TAG, "NumberFormatException for String: " + str, e);
            }
            // Java: catch (Exception e)
            {
                // Java: Log.e(TAG, "Exception for String: " + str, e);
            }
            // Java: return f;
            str.parse::<f32>().ok()
        // Java: else
        } else {
            // Java: return null;
            None
        }
    }

    // Java: static Double string2Double(String str)
    pub fn string2Double(str: &str) -> Option<f64> {
        // Java: Assert.assertNotNull("str should not be null!", str);
        // Java: str = str.trim();
        let str = str.trim();
        // Java: if (null != str && str.length() > 0)
        if !str.is_empty() {
            // Java: Double d = null;
            // Java: try
            {
                // Java: d = Double.parseDouble(str);
            }
            // Java: catch (NumberFormatException e)
            {
                // Java: Log.e(TAG, "NumberFormatException for String: " + str, e);
            }
            // Java: catch (Exception e)
            {
                // Java: Log.e(TAG, "Exception for String: " + str, e);
            }
            // Java: return d;
            str.parse::<f64>().ok()
        // Java: else
        } else {
            // Java: return null;
            None
        }
    }

    // Java: static Integer[] string2IntegerArray(String str)
    pub fn string2IntegerArray(str: &str) -> Vec<Option<i32>> {
        // Java: Assert.assertNotNull("str should not be null", str);
        // Java: str = str.trim();
        let str = str.trim();
        // Java: Assert.assertNotNull("str should not be null", str);
        // Java: _strArr: &[String] = str.split(",");
        let str_arr: Vec<&str> = str.split(',').collect();
        // Java: if (null != strArr && strArr.length > 0)
        if !str_arr.is_empty() {
            // Java: Assert.logTrue("(strPair.length > 0) should be true.", strArr.length > 0);
            // Java: Integer[] array = new Integer[strArr.length];
            let mut array: Vec<Option<i32>> = vec![None; str_arr.len()];
            // Java: int i = 0;
            // Java: for(int len = array.length; i < len; ++i)
            for i in 0..str_arr.len() {
                // Java: String elementStr = strArr[i];
                let element_str = str_arr[i];
                // Java: if (null != elementStr)
                {
                    // Java: String item = strArr[i].trim();
                    let item = element_str.trim();
                    // Java: if (0 < item.length())
                    if !item.is_empty() {
                        // Java: try
                        {
                            // Java: array[i] = Integer.valueOf(item);
                        }
                        // Java: catch (NumberFormatException e)
                        {
                            // Java: Log.e(TAG, "NumberFormatException for String: " + str, e);
                        }
                        // Java: catch (Exception e)
                        {
                            // Java: Log.e(TAG, "Exception for String: " + str, e);
                        }
                        array[i] = item.parse::<i32>().ok();
                    }
                }
            }
            // Java: return array;
            array
        // Java: else
        } else {
            // Java: return new Integer[0];
            Vec::new()
        }
    }

    // Java: static double bigInteger2Double(BigInteger value)
    pub fn bigInteger2Double(value: i128) -> f64 {
        // Java: Assert.assertNotNull("value should not be null!", value);
        // Java: double d = value.doubleValue();
        let d = value as f64;
        // Java: if (Double.POSITIVE_INFINITY == d)
        if d == f64::INFINITY {
            // Java: d = Double.MAX_VALUE;
            f64::MAX
        // Java: else if (Double.NEGATIVE_INFINITY == d)
        } else if d == f64::NEG_INFINITY {
            // Java: d = Double.MIN_VALUE;
            f64::MIN
        } else {
            // Java: return d;
            d
        }
    }

    // Java: static float bigInteger2Float(BigInteger value)
    pub fn bigInteger2Float(value: i128) -> f32 {
        // Java: Assert.assertNotNull("value should not be null!", value);
        // Java: float d = value.floatValue();
        let d = value as f32;
        // Java: if (Float.POSITIVE_INFINITY == d)
        if d == f32::INFINITY {
            // Java: d = Float.MAX_VALUE;
            f32::MAX
        // Java: else if (Float.NEGATIVE_INFINITY == d)
        } else if d == f32::NEG_INFINITY {
            // Java: d = Float.MIN_VALUE;
            f32::MIN
        } else {
            // Java: return d;
            d
        }
    }

    // Java: static String integer2String(int value)
    pub fn integer2String(value: i32) -> Option<String> {
        // Java: String result = null;
        // Java: try
        {
            // Java: result = Integer.toString(value);
        }
        // Java: catch (NumberFormatException e)
        {
            // Java: Log.e(TAG, "NumberFormatException for int: " + value, e);
        }
        // Java: catch (Exception e)
        {
            // Java: Log.e(TAG, "Exception for int: " + value, e);
        }
        // Java: return result;
        Some(value.to_string())
    }

    // Java: static String float2String(float value)
    pub fn float2String(value: f32) -> Option<String> {
        // Java: String result = null;
        // Java: try
        {
            // Java: result = Float.toString(value);
        }
        // Java: catch (NumberFormatException e)
        {
            // Java: Log.e(TAG, "NumberFormatException for float: " + value, e);
        }
        // Java: catch (Exception e)
        {
            // Java: Log.e(TAG, "Exception for float: " + value, e);
        }
        // Java: return result;
        Some(value.to_string())
    }
}
