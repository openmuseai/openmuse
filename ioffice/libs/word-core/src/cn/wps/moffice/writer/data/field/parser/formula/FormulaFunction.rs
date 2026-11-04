// Java: import cn.wps.moffice.writer.data.field.exception.GrammerErrorException
use crate::cn::wps::moffice::writer::data::field::exception::GrammerErrorException::*;
// Java: import java.util.HashMap
use std::collections::HashMap;

// java public static final int ABS = 1;
pub(crate) const ABS: i32 = 1;
// java public static final int AND = 2;
pub(crate) const AND: i32 = 2;
// java public static final int AVERAGE = 3;
pub(crate) const AVERAGE: i32 = 3;
// java public static final int COUNT = 4;
pub(crate) const COUNT: i32 = 4;
// java public static final int DEFINE = 5;
pub(crate) const DEFINE: i32 = 5;
// java public static final int IF = 6;
pub(crate) const IF: i32 = 6;
// java public static final int INT = 7;
pub(crate) const INT: i32 = 7;
// java public static final int MIN = 8;
pub(crate) const MIN: i32 = 8;
// java public static final int MAX = 9;
pub(crate) const MAX: i32 = 9;
// java public static final int MOD = 10;
pub(crate) const MOD: i32 = 10;
// java public static final int NOT = 11;
pub(crate) const NOT: i32 = 11;
// java public static final int OR = 12;
pub(crate) const OR: i32 = 12;
// java public static final int PRODUCT = 13;
pub(crate) const PRODUCT: i32 = 13;
// java public static final int ROUND = 14;
pub(crate) const ROUND: i32 = 14;
// java public static final int SIGN = 16;
pub(crate) const SIGN: i32 = 16;
// java public static final int SUM = 17;
pub(crate) const SUM: i32 = 17;
// java public static HashMap<String, Integer> functionMap = new HashMap();
pub(crate) static functionMap: std::sync::LazyLock<HashMap<String, i32>> =
    std::sync::LazyLock::new(|| {
        let mut m = HashMap::new();
        m.insert("abs".to_string(), 1);
        m.insert("and".to_string(), 2);
        m.insert("average".to_string(), 3);
        m.insert("count".to_string(), 4);
        m.insert("defined".to_string(), 5);
        m.insert("if".to_string(), 6);
        m.insert("int".to_string(), 7);
        m.insert("min".to_string(), 8);
        m.insert("max".to_string(), 9);
        m.insert("mod".to_string(), 10);
        m.insert("not".to_string(), 11);
        m.insert("or".to_string(), 12);
        m.insert("product".to_string(), 13);
        m.insert("round".to_string(), 14);
        m.insert("sign".to_string(), 16);
        m.insert("sum".to_string(), 17);
        m
    });

fn parse_float(s: &str) -> f32 {
    s.parse::<f32>().unwrap_or(0.0)
}

// java public class FormulaFunction
#[derive(Default)]
pub struct FormulaFunction {
}

// java public class FormulaFunction
impl FormulaFunction {
    // java public static String abs(String[] args)
    pub fn abs(args: Option<&Vec<String>>) -> String {
      // java if (args != null && args.length == 1)
      if let Some(args) = args {
        if args.len() == 1 {
          // java char[] ch_tem = args[0].toCharArray();
          let mut ch_tem: Vec<char> = args[0].chars().collect();
          // java return ch_tem[0] == '-' ? args[0].substring(1) : args[0];
          return if !ch_tem.is_empty() && ch_tem[0] == '-' {
            args[0].chars().skip(1).collect()
          } else {
            args[0].clone()
          };
        }
      }
      // java else
      // java throw new GrammerErrorException("");
      panic!("{}", GrammerErrorException::new(""));
    }

    // java public static String and(String[] args)
    pub fn and(args: Option<&Vec<String>>) -> String {
      // java if (args != null && args.length == 2)
      if let Some(args) = args {
        if args.len() == 2 {
          // java float x = Float.parseFloat(args[0]);
          let mut x: f32 = parse_float(&args[0]);
          // java float y = Float.parseFloat(args[1]);
          let mut y: f32 = parse_float(&args[1]);
          // java return x != 0.0F && y != 0.0F ? "1" : "0";
          return if x != 0.0 && y != 0.0 { "1".to_string() } else { "0".to_string() };
        }
      }
      // java else
      // java throw new GrammerErrorException("语法错误");
      panic!("{}", GrammerErrorException::new("语法错误"));
    }

    // java public static String average(String[] args)
    pub fn average(args: Option<&Vec<String>>) -> String {
      // java if (args != null && args.length != 0)
      if let Some(args) = args {
        if !args.is_empty() {
          // java float result = 0.0F;
          let mut result: f32 = 0.0;
          // java int count = args.length;
          let mut count: i32 = args.len() as i32;
          let mut i: i32 = 0;
          // java for(int i = 0; i < args.length; ++i)
          while i < (args.len() as i32) {
            // java result += Float.parseFloat(args[i]);
            result = result + parse_float(&args[i as usize]);
            i += 1;
          }
          // java return String.valueOf(result / (float)count);
          return (result / (count as f32)).to_string();
        }
      }
      // java else
      // java throw new GrammerErrorException("语法错误");
      panic!("{}", GrammerErrorException::new("语法错误"));
    }

    // java public static String count(String[] args)
    pub fn count(args: Option<&Vec<String>>) -> String {
      // java if (args != null && args.length != 0)
      if let Some(args) = args {
        if !args.is_empty() {
          // java return String.valueOf(args.length);
          return args.len().to_string();
        }
      }
      // java else
      // java throw new GrammerErrorException("语法错误");
      panic!("{}", GrammerErrorException::new("语法错误"));
    }

    // java public static String define(String[] args)
    pub fn define(args: Option<&Vec<String>>) -> String {
      // java if (args != null && args.length == 1)
      if let Some(args) = args {
        if args.len() == 1 {
          // java return "1.0";
          return "1.0".to_string();
        }
      }
      // java else
      // java throw new GrammerErrorException("语法错误");
      panic!("{}", GrammerErrorException::new("语法错误"));
    }

    // java public static String if_custom(String[] args)
    pub fn if_custom(args: Option<&Vec<String>>) -> String {
      // java if (args != null && args.length == 3)
      if let Some(args) = args {
        if args.len() == 3 {
          // java return Float.parseFloat(args[0]) == 0.0F ? args[2] : args[1];
          return if parse_float(&args[0]) == 0.0 { args[2].clone() } else { args[1].clone() };
        }
      }
      // java else
      // java throw new GrammerErrorException("语法错误");
      panic!("{}", GrammerErrorException::new("语法错误"));
    }

    // java public static String int_custom(String[] args)
    pub fn int_custom(args: Option<&Vec<String>>) -> String {
      // java if (args != null && args.length == 1)
      if let Some(args) = args {
        if args.len() == 1 {
          // java float tem = Float.parseFloat(args[0]);
          let mut tem: f32 = parse_float(&args[0]);
          // java return String.valueOf((int)tem);
          return (tem as i32).to_string();
        }
      }
      // java else
      // java throw new GrammerErrorException("语法错误");
      panic!("{}", GrammerErrorException::new("语法错误"));
    }

    // java public static String min(String[] args)
    pub fn min(args: Option<&Vec<String>>) -> String {
      // java if (args != null && args.length != 0)
      if let Some(args) = args {
        if !args.is_empty() {
          // java float min = Float.parseFloat(args[0]);
          let mut min: f32 = parse_float(&args[0]);
          let mut i: i32 = 1;
          // java for(int i = 1; i < args.length; ++i)
          while i < (args.len() as i32) {
            // java float tem = Float.parseFloat(args[i]);
            let mut tem: f32 = parse_float(&args[i as usize]);
            // java if (tem < min)
            if tem < min {
              // java min = tem;
              min = tem;
            }
            i += 1;
          }
          // java return String.valueOf(min);
          return min.to_string();
        }
      }
      // java else
      // java throw new GrammerErrorException("语法错误");
      panic!("{}", GrammerErrorException::new("语法错误"));
    }

    // java public static String max(String[] args)
    pub fn max(args: Option<&Vec<String>>) -> String {
      // java if (args != null && args.length != 0)
      if let Some(args) = args {
        if !args.is_empty() {
          // java float max = Float.parseFloat(args[0]);
          let mut max: f32 = parse_float(&args[0]);
          let mut i: i32 = 1;
          // java for(int i = 1; i < args.length; ++i)
          while i < (args.len() as i32) {
            // java float tem = Float.parseFloat(args[i]);
            let mut tem: f32 = parse_float(&args[i as usize]);
            // java if (tem > max)
            if tem > max {
              // java max = tem;
              max = tem;
            }
            i += 1;
          }
          // java return String.valueOf(max);
          return max.to_string();
        }
      }
      // java else
      // java throw new GrammerErrorException("语法错误");
      panic!("{}", GrammerErrorException::new("语法错误"));
    }

    // java public static String mod(String[] args)
    pub fn r#mod(args: Option<&Vec<String>>) -> String {
      // java if (args != null && args.length == 2)
      if let Some(args) = args {
        if args.len() == 2 {
          // java float m = Float.parseFloat(args[0]);
          let mut m: f32 = parse_float(&args[0]);
          // java float n = Float.parseFloat(args[1]);
          let mut n: f32 = parse_float(&args[1]);
          // java float result = m % n;
          let mut result: f32 = m % n;
          // java return String.valueOf(result);
          return result.to_string();
        }
      }
      // java else
      // java throw new GrammerErrorException("语法错误");
      panic!("{}", GrammerErrorException::new("语法错误"));
    }

    // java public static String not(String[] args)
    pub fn not(args: Option<&Vec<String>>) -> String {
      // java if (args != null && args.length == 1)
      if let Some(args) = args {
        if args.len() == 1 {
          // java float m = Float.parseFloat(args[0]);
          let mut m: f32 = parse_float(&args[0]);
          // java return m != 0.0F ? "0.0" : "1.0";
          return if m != 0.0 { "0.0".to_string() } else { "1.0".to_string() };
        }
      }
      // java else
      // java throw new GrammerErrorException("语法错误");
      panic!("{}", GrammerErrorException::new("语法错误"));
    }

    // java public static String or(String[] args)
    pub fn or(args: Option<&Vec<String>>) -> String {
      // java if (args != null && args.length == 2)
      if let Some(args) = args {
        if args.len() == 2 {
          // java float m = Float.parseFloat(args[0]);
          let mut m: f32 = parse_float(&args[0]);
          // java float n = Float.parseFloat(args[1]);
          let mut n: f32 = parse_float(&args[1]);
          // java return m == 0.0F && n == 0.0F ? "0.0" : "1.0";
          return if m == 0.0 && n == 0.0 { "0.0".to_string() } else { "1.0".to_string() };
        }
      }
      // java else
      // java throw new GrammerErrorException("语法错误");
      panic!("{}", GrammerErrorException::new("语法错误"));
    }

    // java public static String product(String[] args)
    pub fn product(args: Option<&Vec<String>>) -> String {
      // java if (args != null && args.length != 0)
      if let Some(args) = args {
        if !args.is_empty() {
          // java float result = 1.0F;
          let mut result: f32 = 1.0;
          let mut i: i32 = 0;
          // java for(int i = 0; i < args.length; ++i)
          while i < (args.len() as i32) {
            // java float tem = Float.parseFloat(args[i]);
            let mut tem: f32 = parse_float(&args[i as usize]);
            // java result *= tem;
            result = result * tem;
            i += 1;
          }
          // java return String.valueOf(result);
          return result.to_string();
        }
      }
      // java else
      // java throw new GrammerErrorException("语法错误");
      panic!("{}", GrammerErrorException::new("语法错误"));
    }

    // java public static String round(String[] args)
    pub fn round(args: Option<&Vec<String>>) -> String {
      // java if (args != null && args.length == 2)
      if let Some(args) = args {
        if args.len() == 2 {
          // java char[] ch_tem = args[0].toCharArray();
          let mut ch_tem: Vec<char> = args[0].chars().collect();
          // java float n = Float.parseFloat(args[1]);
          let mut n: f32 = parse_float(&args[1]);
          // java int pointIndex = -1;
          let mut pointIndex: i32 = -1;
          let mut i: i32 = 0;
          // java for(int i = 0; i < ch_tem.length; ++i)
          while i < (ch_tem.len() as i32) {
            // java if ('.' == ch_tem[i])
            if '.' == ch_tem[i as usize] {
              // java pointIndex = i;
              pointIndex = i;
              // java break;
              break;
            }
            i += 1;
          }
          // java if (pointIndex != -1 && !((float)pointIndex + n >= (float)(ch_tem.length - 1)))
          if pointIndex != -1 && !(((pointIndex as f32) + n) >= (((ch_tem.len() as i32) - 1) as f32)) {
            // java if (n == 0.0F)
            if n == 0.0 {
              // java return args[0].substring(0, pointIndex);
              return args[0].chars().take(pointIndex as usize).collect();
            }
            // java else
            else {
              // java char c = ch_tem[pointIndex + (int)n + 1];
              let mut c: char = ch_tem[(pointIndex + (n as i32) + 1) as usize];
              // java if (c >= '5' && c <= '9')
              if c >= '5' && c <= '9' {
                // java ++ch_tem[pointIndex + (int)n];
                let idx = (pointIndex + (n as i32)) as usize;
                let ch = ch_tem[idx];
                ch_tem[idx] = char::from_u32(ch as u32 + 1).unwrap_or(ch);
              }
              // java pointIndex += (int)n;
              pointIndex = pointIndex + (n as i32);
              // java return new String(ch_tem, 0, pointIndex + 1);
              return ch_tem.into_iter().take((pointIndex + 1) as usize).collect();
            }
          }
          // java else
          else {
            // java return args[0];
            return args[0].clone();
          }
        }
      }
      // java else
      // java throw new GrammerErrorException("语法错误");
      panic!("{}", GrammerErrorException::new("语法错误"));
    }

    // java public static String sign(String[] args)
    pub fn sign(args: Option<&Vec<String>>) -> String {
      // java if (args != null && args.length == 1)
      if let Some(args) = args {
        if args.len() == 1 {
          // java float n = Float.parseFloat(args[0]);
          let mut n: f32 = parse_float(&args[0]);
          // java return n >= 0.0F ? "1.0" : "-1.0";
          return if n >= 0.0 { "1.0".to_string() } else { "-1.0".to_string() };
        }
      }
      // java else
      // java throw new GrammerErrorException("语法错误");
      panic!("{}", GrammerErrorException::new("语法错误"));
    }

    // java public static String sum(String[] args)
    pub fn sum(args: Option<&Vec<String>>) -> String {
      // java if (args != null && args.length != 0)
      if let Some(args) = args {
        if !args.is_empty() {
          // java float result = 0.0F;
          let mut result: f32 = 0.0;
          let mut i: i32 = 0;
          // java for(int i = 0; i < args.length; ++i)
          while i < (args.len() as i32) {
            // java result += Float.parseFloat(args[i]);
            result = result + parse_float(&args[i as usize]);
            i += 1;
          }
          // java return String.valueOf(result);
          return result.to_string();
        }
      }
      // java else
      // java throw new GrammerErrorException("语法错误");
      panic!("{}", GrammerErrorException::new("语法错误"));
    }

    // java public static boolean existFunction(String functionName)
    pub fn existFunction(functionName: &str) -> bool {
      // java return functionMap.containsKey(functionName);
      return functionMap.contains_key(functionName);
    }

}
