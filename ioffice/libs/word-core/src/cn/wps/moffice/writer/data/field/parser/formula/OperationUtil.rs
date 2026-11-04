// Java: import cn.wps.moffice.writer.data.field.exception.GrammerErrorException
use crate::cn::wps::moffice::writer::data::field::exception::GrammerErrorException::*;
// Java: import cn.wps.moffice.writer.data.field.exception.UndefineBookmarkException
use crate::cn::wps::moffice::writer::data::field::exception::UndefineBookmarkException::*;
// Java: import java.util.Arrays
// Java: import java.util.HashMap
// Java: import cn.wps.moffice.writer.data.field.parser.formula.FormulaOprational
use crate::cn::wps::moffice::writer::data::field::parser::formula::FormulaOprational::*;
use crate::cn::wps::moffice::writer::data::field::parser::formula::Stack::Stack;
use std::collections::HashMap;
use std::sync::Mutex;

// java public static final int POINT = 0;
pub(crate) const POINT: i32 = 0;
// java public static final int RIGHT_BRACKTE = 0;
pub(crate) const RIGHT_BRACKTE: i32 = 0;
// java public static final int LETF_BRACKTE = 0;
pub(crate) const LETF_BRACKTE: i32 = 0;
// java public static final int POWER = 1;
pub(crate) const POWER: i32 = 1;
// java public static final int MULT = 2;
pub(crate) const MULT: i32 = 2;
// java public static final int QUOT = 3;
pub(crate) const QUOT: i32 = 3;
// java public static final int ADD = 4;
pub(crate) const ADD: i32 = 4;
// java public static final int SUB = 5;
pub(crate) const SUB: i32 = 5;
// java public static final int EQUAL = 6;
pub(crate) const EQUAL: i32 = 6;
// java public static final int BIGGER = 7;
pub(crate) const BIGGER: i32 = 7;
// java public static final int BIGGER_EQUAL = 8;
pub(crate) const BIGGER_EQUAL: i32 = 8;
// java public static final int SMALLER = 9;
pub(crate) const SMALLER: i32 = 9;
// java public static final int SMALLER_EQUAL = 10;
pub(crate) const SMALLER_EQUAL: i32 = 10;
// java public static final int NO_EQUAL = 11;
pub(crate) const NO_EQUAL: i32 = 11;
// java private static HashMap<String, Integer> operationMark = new HashMap();
static operationMark: std::sync::LazyLock<HashMap<String, i32>> = std::sync::LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(".".to_string(), 0);
    m.insert(")".to_string(), 0);
    m.insert("(".to_string(), 0);
    m.insert("^".to_string(), 1);
    m.insert("*".to_string(), 2);
    m.insert("/".to_string(), 3);
    m.insert("+".to_string(), 4);
    m.insert("-".to_string(), 5);
    m.insert("=".to_string(), 6);
    m.insert(">".to_string(), 7);
    m.insert(">=".to_string(), 8);
    m.insert("<".to_string(), 9);
    m.insert("<=".to_string(), 10);
    m.insert("<>".to_string(), 11);
    m
});
// java private static FormulaOprational computeUtil = null;
static computeUtil: Mutex<Option<FormulaOprational>> = Mutex::new(None);

// java public class OperationUtil
#[derive(Default)]
pub struct OperationUtil {
}

// java public class OperationUtil
impl OperationUtil {
    // java private  OperationUtil()
    pub fn OperationUtil() -> Self {
      Self {}
    }

    // java public static double operational(double a, double b, String operaCharacter)
    pub fn operational(a: f64, b: f64, operaCharacter: &str) -> f64 {
      // java int mark = (Integer)operationMark.get(operaCharacter);
      let mut mark: i32 = operationMark.get(operaCharacter).copied().unwrap_or(-1);
      // java switch (mark)
      match mark {
        // java case 1:
        1 => {
          // java double tem = (double)1.0F;
          let mut tem: f64 = 1.0;
          let mut i: i32 = 0;
          // java for(int i = 0; (double)i < b; ++i)
          while (i as f64) < b {
            // java tem *= a;
            tem = tem * a;
            i += 1;
          }
          // java return tem;
          return tem;
        }
        // java case 2:
        2 => {
          // java return a * b;
          return a * b;
        }
        // java case 3:
        3 => {
          // java return a / b;
          return a / b;
        }
        // java case 4:
        4 => {
          // java return a + b;
          return a + b;
        }
        // java case 5:
        5 => {
          // java return a - b;
          return a - b;
        }
        // java case 6:
        6 => {
          // java if (a == b)
          if a == b {
            // java return (double)1.0F;
            return 1.0;
          }
          // java return (double)0.0F;
          return 0.0;
        }
        // java case 7:
        7 => {
          // java if (a > b)
          if a > b {
            // java return (double)1.0F;
            return 1.0;
          }
          // java return (double)0.0F;
          return 0.0;
        }
        // java case 8:
        8 => {
          // java if (a >= b)
          if a >= b {
            // java return (double)1.0F;
            return 1.0;
          }
          // java return (double)0.0F;
          return 0.0;
        }
        // java case 9:
        9 => {
          // java if (a < b)
          if a < b {
            // java return (double)1.0F;
            return 1.0;
          }
          // java return (double)0.0F;
          return 0.0;
        }
        // java case 10:
        10 => {
          // java if (a <= b)
          if a <= b {
            // java return (double)1.0F;
            return 1.0;
          }
          // java return (double)0.0F;
          return 0.0;
        }
        // java case 11:
        11 => {
          // java if (a != b)
          if a != b {
            // java return (double)1.0F;
            return 1.0;
          }
          // java return (double)0.0F;
          return 0.0;
        }
        // java default:
        _ => {
          // java return (double)0.0F;
          return 0.0;
        }
      }
    }

    // java public static boolean checkStrIsOperation(String str)
    pub fn checkStrIsOperation(str: &str) -> bool {
      let mut i: i32 = 0;
      // java for(int i = 0; i < str.length(); ++i)
      while i < (str.len() as i32) {
        // java char tem = str.charAt(i);
        let mut tem: char = str.chars().nth(i as usize).unwrap_or('\0');
        // java if ((tem < '0' || tem > '9') && !operationMark.containsKey(tem + ""))
        let key = tem.to_string();
        if (tem < '0' || tem > '9') && !operationMark.contains_key(&key) {
          // java return false;
          return false;
        }
        i += 1;
      }
      // java return true;
      return true;
    }

    // java public static String getFulmulaFuncitonNameStr(String fullStr)
    pub fn getFulmulaFuncitonNameStr(fullStr: &str) -> Option<String> {
      // java char[] chTem = fullStr.toCharArray();
      let mut chTem: Vec<char> = fullStr.chars().collect();
      // java int index;
      let mut index: i32 = 0;
      // java for(index = 0; index < chTem.length && chTem[index] >= 'a' && chTem[index] <= 'z' ++index)
      while (index as usize) < chTem.len() && chTem[index as usize] >= 'a' && chTem[index as usize] <= 'z' {
        index += 1;
      }
      // java return index == 0 ? null : fullStr.substring(0, index);
      return if index == 0 { None } else { Some(fullStr.chars().take(index as usize).collect()) };
    }

    // java public static String getFunctionParams(String funStr)
    pub fn getFunctionParams(funStr: &str) -> String {
      // java Stack braketStack = Stack.getStackInstance();
      let mut braketStack: Stack = Stack::getStackInstance();
      // java braketStack.setStackLength(funStr.length());
      braketStack.setStackLength(funStr.len() as i32);
      // java int startBraketIndex = -1;
      let mut startBraketIndex: i32 = -1;
      // java int lastBraketIndex = -1;
      let mut lastBraketIndex: i32 = -1;
      // java char[] chTem = funStr.toCharArray();
      let mut chTem: Vec<char> = funStr.chars().collect();
      let mut i: i32 = 0;
      // java for(int i = 0; i < chTem.length; ++i)
      while (i as usize) < chTem.len() {
        // java if (chTem[i] == '(')
        if chTem[i as usize] == '(' {
          // java if (braketStack.isEmpty())
          if braketStack.isEmpty() {
            // java startBraketIndex = i;
            startBraketIndex = i;
          }
          // java braketStack.posh("(");
          braketStack.posh("(");
        }
        // java else if (chTem[i] == ')')
        else if chTem[i as usize] == ')' {
          // java braketStack.pop();
          braketStack.pop();
          // java if (braketStack.isEmpty())
          if braketStack.isEmpty() {
            // java lastBraketIndex = i;
            lastBraketIndex = i;
            // java break;
            break;
          }
        }
        i += 1;
      }
      // java braketStack.dispose();
      braketStack.dispose();
      // java if (lastBraketIndex != funStr.length() - 1)
      if lastBraketIndex != (funStr.len() as i32) - 1 {
        // java throw new GrammerErrorException("语法错误");
        panic!("{}", GrammerErrorException::new("语法错误"));
      }
      // java else
      else {
        // java return funStr.substring(startBraketIndex + 1, lastBraketIndex);
        return funStr.chars().skip((startBraketIndex + 1) as usize).take((lastBraketIndex - startBraketIndex - 1) as usize).collect();
      }
    }

    // java public static String[] getParamsStrArray(String paramsStr)
    pub fn getParamsStrArray(paramsStr: Option<&str>) -> Option<Vec<String>> {
      // java if (paramsStr != null && paramsStr.length() != 0)
      if let Some(paramsStr) = paramsStr {
        if !paramsStr.is_empty() {
          // java Stack braketStack = Stack.getStackInstance();
          let mut braketStack: Stack = Stack::getStackInstance();
          // java braketStack.setStackLength(paramsStr.length());
          braketStack.setStackLength(paramsStr.len() as i32);
          // java int count = 0;
          let mut count: i32 = 0;
          // java boolean flag = false;
          let mut flag: bool = false;
          // java int paramsLength = 0;
          let mut paramsLength: i32 = 0;
          // java int startIndex = 0;
          let mut startIndex: i32 = 0;
          // java int lastIndex = 0;
          let mut lastIndex: i32 = 0;
          // java char[] chTem = paramsStr.toCharArray();
          let mut chTem: Vec<char> = paramsStr.chars().collect();
          let mut i: i32 = 0;
          // java for(int i = 0; i < chTem.length; ++i)
          while (i as usize) < chTem.len() {
            // java if (',' == chTem[i])
            if ',' == chTem[i as usize] {
              // java ++count;
              count += 1;
            }
            i += 1;
          }
          // java String[] paramsArray = new String[count + 1];
          let mut paramsArray: Vec<Option<String>> = vec![None; (count + 1) as usize];
          i = 0;
          // java for(int i = 0; i < chTem.length; ++i)
          while (i as usize) < chTem.len() {
            // java if (i == chTem.length - 1)
            if i == (chTem.len() as i32) - 1 {
              // java lastIndex = chTem.length;
              lastIndex = chTem.len() as i32;
              // java paramsArray[paramsLength++] = initString(paramsStr.substring(startIndex, lastIndex));
              let sub: String = paramsStr.chars().skip(startIndex as usize).take((lastIndex - startIndex) as usize).collect();
              paramsArray[paramsLength as usize] = Some(Self::initString(&sub));
              paramsLength += 1;
            }
            // java else if (flag && chTem[i] == '(')
            else if flag && chTem[i as usize] == '(' {
              // java braketStack.posh("(");
              braketStack.posh("(");
            }
            // java else if (flag && chTem[i] == ')')
            else if flag && chTem[i as usize] == ')' {
              // java braketStack.pop();
              braketStack.pop();
              // java if (braketStack.isEmpty())
              if braketStack.isEmpty() {
                // java flag = false;
                flag = false;
              }
            }
            // java else if (!flag && ',' == chTem[i] && i != 0)
            else if !flag && ',' == chTem[i as usize] && i != 0 {
              // java paramsArray[paramsLength++] = initString(paramsStr.substring(startIndex, i));
              let sub: String = paramsStr.chars().skip(startIndex as usize).take((i - startIndex) as usize).collect();
              paramsArray[paramsLength as usize] = Some(Self::initString(&sub));
              paramsLength += 1;
              // java startIndex = i + 1;
              startIndex = i + 1;
            }
            // java else if (chTem[i] >= 'a' && chTem[i] <= 'z')
            else if chTem[i as usize] >= 'a' && chTem[i as usize] <= 'z' {
              // java flag = true;
              flag = true;
            }
            i += 1;
          }
          // java braketStack.dispose();
          braketStack.dispose();
          // java return (String[])Arrays.copyOf(paramsArray, paramsLength);
          return Some(paramsArray.into_iter().take(paramsLength as usize).map(|o| o.unwrap_or_default()).collect());
        }
      }
      // java else
      // java return null;
      return None;
    }

    // java public static int[] extractFunctionStr(String originalStr)
    pub fn extractFunctionStr(originalStr: &str) -> Vec<i32> {
      // java if (!"false".equals(originalStr) && !"true".equals(originalStr))
      if originalStr != "false" && originalStr != "true" {
        // java Stack braketStack = Stack.getStackInstance();
        let mut braketStack: Stack = Stack::getStackInstance();
        // java braketStack.setStackLength(originalStr.length());
        braketStack.setStackLength(originalStr.len() as i32);
        // java int lastIndex = -1;
        let mut lastIndex: i32 = -1;
        // java int startIndex = -1;
        let mut startIndex: i32 = -1;
        // java boolean flag = false;
        let mut flag: bool = false;
        // java char[] chTem = originalStr.toCharArray();
        let mut chTem: Vec<char> = originalStr.chars().collect();
        let mut i: i32 = 0;
        // java for(int i = 0; i < chTem.length; ++i)
        while (i as usize) < chTem.len() {
          // java if (!flag && chTem[i] >= 'a' && chTem[i] <= 'z')
          if !flag && chTem[i as usize] >= 'a' && chTem[i as usize] <= 'z' {
            // java startIndex = i;
            startIndex = i;
            // java flag = true;
            flag = true;
          }
          // java else if (flag && '(' == chTem[i])
          else if flag && '(' == chTem[i as usize] {
            // java braketStack.posh("(");
            braketStack.posh("(");
          }
          // java else if (flag && ')' == chTem[i])
          else if flag && ')' == chTem[i as usize] {
            // java if (i == chTem.length - 1)
            if i == (chTem.len() as i32) - 1 {
              // java lastIndex = i + 1;
              lastIndex = i + 1;
              // java break;
              break;
            }
            // java braketStack.pop();
            braketStack.pop();
          }
          // java else if (flag && braketStack.isEmpty() && operationMark.containsKey(chTem[i] + ""))
          else if flag && braketStack.isEmpty() && operationMark.contains_key(&chTem[i as usize].to_string()) {
            // java lastIndex = i;
            lastIndex = i;
            // java break;
            break;
          }
          i += 1;
        }
        // java braketStack.dispose();
        braketStack.dispose();
        // java if (startIndex != -1 && lastIndex == -1)
        if startIndex != -1 && lastIndex == -1 {
          // java throw new UndefineBookmarkException("");
          panic!("{}", UndefineBookmarkException::new(""));
        }
        // java else
        else {
          // java int[] index = new int[]{startIndex, lastIndex};
          let mut index: Vec<i32> = vec![startIndex, lastIndex];
          // java return index;
          return index;
        }
      }
      // java else
      else {
        // java return new int[]{0, originalStr.length()};
        return vec![0, originalStr.len() as i32];
      }
    }

    // java public static String compute(String operationStr)
    pub fn compute(operationStr: &str) -> String {
      let mut guard = computeUtil.lock().unwrap();
      // java if (computeUtil == null)
      if guard.is_none() {
        // java computeUtil = FormulaOprational.getInstance();
        *guard = Some(FormulaOprational::getInstance());
      }
      // java else
      else {
        // java computeUtil = computeUtil.recycle();
        guard.as_mut().unwrap().recycle();
      }
      let util = guard.as_mut().unwrap();
      // java return computeUtil.compute(computeUtil.middleOrder_Transto_AfterOrder(operationStr));
      let after = util.middleOrder_Transto_AfterOrder(Some(operationStr));
      return util.compute(&after);
    }

    // java public static String initString(String args)
    pub fn initString(args: &str) -> String {
      // java args = args.replaceAll("%", "/100").toLowerCase();
      let mut args: String = args.replace('%', "/100").to_lowercase();
      // java if (args.charAt(0) == '+' || args.charAt(0) == '-')
      if args.chars().next() == Some('+') || args.chars().next() == Some('-') {
        // java args = "0" + args;
        args = format!("0{}", args);
      }
      // java return args;
      return args;
    }

}
