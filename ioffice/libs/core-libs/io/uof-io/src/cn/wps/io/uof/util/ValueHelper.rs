use super::UofUnit::UofUnit;

// Java: public class ValueHelper
pub struct ValueHelper {
}

// Java: public class ValueHelper
impl ValueHelper {
    // Java: ValueHelper()
    pub fn ValueHelper() -> Self {
      // Java: super();
      ValueHelper {}
    }

    // Java: static boolean parseBoolean(String value)
    pub fn parseBoolean(value: Option<&str>) -> bool {
      // Java: return parseBoolean(value, false);
      Self::parseBoolean_default(value, false)
    }

    // Java: static boolean parseBoolean(String value, boolean defaultValue)
    pub fn parseBoolean_default(value: Option<&str>, defaultValue: bool) -> bool {
      // Java: if (value != null && value.length() > 0)
      if let Some(v) = value { if !v.is_empty()
      {
         // Java: return value.equals("1") || value.equals("true");
         return v == "1" || v == "true";
      } }
      // Java: else
      {
         // Java: return defaultValue;
         defaultValue
      }
    }

    // Java: static Integer parseInteger(String value)
    pub fn parseInteger(value: Option<&str>) -> Option<i32> {
      // Java: return parseInteger(value, (Integer)null);
      Self::parseInteger_default(value, None)
    }

    // Java: static Integer parseInteger(String value, Integer defaultValue)
    pub fn parseInteger_default(value: Option<&str>, defaultValue: Option<i32>) -> Option<i32> {
      // Java: if (value != null && value.length() > 0)
      if let Some(v) = value { if !v.is_empty()
      {
         // Java: Integer result = null;
         // Java: try
         {
            // Java: result = Integer.parseInt(value);
            if let Ok(result) = v.parse::<i32>() {
            // Java: return result;
            return Some(result);
            }
         }
         // Java: catch (Throwable var4)
         {
            // Java: return defaultValue;
            return defaultValue;
         }
      } }
      // Java: else
      {
         // Java: return defaultValue;
         defaultValue
      }
    }

    // Java: static Long parseLong(String value)
    pub fn parseLong(value: Option<&str>) -> Option<i64> {
      // Java: return parseLong(value, (Long)null);
      Self::parseLong_default(value, None)
    }

    // Java: static Long parseLong(String value, Long defaultValue)
    pub fn parseLong_default(value: Option<&str>, defaultValue: Option<i64>) -> Option<i64> {
      // Java: if (value != null && value.length() > 0)
      if let Some(v) = value { if !v.is_empty()
      {
         // Java: Long result = null;
         // Java: try
         {
            // Java: result = Long.parseLong(value);
            if let Ok(result) = v.parse::<i64>() {
            // Java: return result;
            return Some(result);
            }
         }
         // Java: catch (Throwable var4)
         {
            // Java: return defaultValue;
            return defaultValue;
         }
      } }
      // Java: else
      {
         // Java: return defaultValue;
         defaultValue
      }
    }

    // Java: static UofUnit parseUof(String value)
    pub fn parseUof(value: Option<&str>) -> Option<UofUnit> {
      // Java: return value != null && value.length() > 0 ? new UofUnit(value) : null;
      match value {
          Some(v) if !v.is_empty() => Some(UofUnit::UofUnit(v.to_string())),
          _ => None,
      }
    }

    // Java: static Double parseDouble(String value)
    pub fn parseDouble(value: Option<&str>) -> Option<f64> {
      // Java: return parseDouble(value, (Double)null);
      Self::parseDouble_default(value, None)
    }

    // Java: static Double parseDouble(String value, Double defaultValue)
    pub fn parseDouble_default(value: Option<&str>, defaultValue: Option<f64>) -> Option<f64> {
      // Java: if (value != null && value.length() > 0)
      if let Some(v) = value { if !v.is_empty()
      {
         // Java: Double result = null;
         // Java: try
         {
            // Java: result = Double.parseDouble(value);
            if let Ok(result) = v.parse::<f64>() {
            // Java: return result;
            return Some(result);
            }
         }
         // Java: catch (Throwable var4)
         {
            // Java: return defaultValue;
            return defaultValue;
         }
      } }
      // Java: else
      {
         // Java: return defaultValue;
         defaultValue
      }
    }

}
