// Java: import cn.wps.moffice.writer.data.field.exception.GrammerErrorException
use crate::cn::wps::moffice::writer::data::field::exception::GrammerErrorException::*;
// Java: import cn.wps.moffice.writer.data.field.exception.UndefineBookmarkException
use crate::cn::wps::moffice::writer::data::field::exception::UndefineBookmarkException::*;
// Java: import java.util.Arrays
// Java: import java.util.HashMap
use std::collections::HashMap;
use crate::cn::wps::moffice::writer::data::field::parser::formula::Stack::Stack;
use crate::cn::wps::moffice::writer::data::field::parser::formula::OperationUtil::OperationUtil;

// java private static HashMap<String, Integer> priority = new HashMap();
static priority: std::sync::LazyLock<HashMap<String, i32>> = std::sync::LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("(".to_string(), 0);
    m.insert("=".to_string(), 1);
    m.insert(">".to_string(), 1);
    m.insert(">=".to_string(), 1);
    m.insert("<".to_string(), 1);
    m.insert("<=".to_string(), 1);
    m.insert("<>".to_string(), 1);
    m.insert("+".to_string(), 2);
    m.insert("-".to_string(), 2);
    m.insert("*".to_string(), 3);
    m.insert("/".to_string(), 3);
    m.insert("^".to_string(), 4);
    m
});

// java public class FormulaOprational
pub struct FormulaOprational {
    // java private String[] strResult;
    strResult: Option<Vec<Option<String>>>,
    // java private Stack stackOperator = Stack.getStackInstance();
    stackOperator: Stack,
    // java private Stack stackCompute = Stack.getStackInstance();
    stackCompute: Stack,
    // java private int currentIndexFormal = 0;
    currentIndexFormal: i32,
    // java private int currentIndexResult = 0;
    currentIndexResult: i32,
}

impl Default for FormulaOprational {
    fn default() -> Self {
        Self::FormulaOprational()
    }
}

// java public class FormulaOprational
impl FormulaOprational {
    // java private  FormulaOprational()
    pub fn FormulaOprational() -> Self {
      Self {
        strResult: None,
        stackOperator: Stack::getStackInstance(),
        stackCompute: Stack::getStackInstance(),
        currentIndexFormal: 0,
        currentIndexResult: 0,
      }
    }

    // java public static FormulaOprational getInstance()
    pub fn getInstance() -> FormulaOprational {
      // java return new FormulaOprational();
      return FormulaOprational::FormulaOprational();
    }

    // java public String[] middleOrder_Transto_AfterOrder(String transStr)
    pub fn middleOrder_Transto_AfterOrder(&mut self, transStr: Option<&str>) -> Vec<String> {
      // java if (transStr == null)
      if transStr.is_none() {
        // java throw new GrammerErrorException("语法错误");
        panic!("{}", GrammerErrorException::new("语法错误"));
      }
      // java else if (!OperationUtil.checkStrIsOperation(transStr))
      else if !OperationUtil::checkStrIsOperation(transStr.unwrap()) {
        // java throw new UndefineBookmarkException("语法错误");
        panic!("{}", UndefineBookmarkException::new("语法错误"));
      }
      // java else
      else {
        let transStr = transStr.unwrap();
        let len = transStr.len() as i32;
        // java this.strResult = new String[transStr.length()];
        self.strResult = Some(vec![None; len as usize]);
        // java this.stackOperator.setStackLength(transStr.length());
        self.stackOperator.setStackLength(len);
        // java this.stackCompute.setStackLength(transStr.length());
        self.stackCompute.setStackLength(len);
        let mut i: i32 = 0;
        // java for(int i = 0; i < transStr.length(); ++i)
        while i < len {
          // java char chTem = transStr.charAt(i);
          let mut chTem: char = transStr.chars().nth(i as usize).unwrap_or('\0');
          // java String strTem = Character.toString(chTem);
          let mut strTem: String = chTem.to_string();
          // java if ((chTem > '9' || chTem < '0') && chTem != '.')
          if (chTem > '9' || chTem < '0') && chTem != '.' {
            // java if (i - this.currentIndexFormal != 0)
            if i - self.currentIndexFormal != 0 {
              // java this.strResult[this.currentIndexResult++] = transStr.substring(this.currentIndexFormal, i);
              let s = transStr.chars().skip(self.currentIndexFormal as usize).take((i - self.currentIndexFormal) as usize).collect::<String>();
              if let Some(ref mut sr) = self.strResult {
                sr[self.currentIndexResult as usize] = Some(s);
                self.currentIndexResult += 1;
              }
            }
            // java if (!this.stackOperator.isEmpty() && !"(".equals(strTem))
            if !self.stackOperator.isEmpty() && strTem.as_str() != "(" {
              // java if (")".equals(strTem))
              if strTem.as_str() == ")" {
                // java String tem = this.stackOperator.pop();
                let mut tem: Option<String> = self.stackOperator.pop();
                if let Some(ref mut sr) = self.strResult {
                  sr[self.currentIndexResult as usize] = tem.clone();
                  self.currentIndexResult += 1;
                }
                // java for(... !"(".equals(tem) ...)
                while tem.as_deref() != Some("(") {
                  // java tem = this.stackOperator.pop();
                  tem = self.stackOperator.pop();
                  if let Some(ref mut sr) = self.strResult {
                    sr[self.currentIndexResult as usize] = tem.clone();
                    self.currentIndexResult += 1;
                  }
                }
              }
              // java else if (this.priorityCompare(strTem, this.stackOperator.getTop()))
              else if self.priorityCompare(&strTem, self.stackOperator.getTop().as_deref().unwrap_or("")) {
                // java this.stackOperator.posh(strTem);
                self.stackOperator.posh(&strTem);
              }
              // java else
              else {
                // java String tem = this.stackOperator.pop();
                let mut tem: Option<String> = self.stackOperator.pop();
                if let Some(ref mut sr) = self.strResult {
                  sr[self.currentIndexResult as usize] = tem.clone();
                  self.currentIndexResult += 1;
                }
                // java for(... !empty && !priorityCompare ...)
                while !self.stackOperator.isEmpty()
                  && !self.priorityCompare(&strTem, self.stackOperator.getTop().as_deref().unwrap_or(""))
                {
                  // java tem = this.stackOperator.pop();
                  tem = self.stackOperator.pop();
                  if let Some(ref mut sr) = self.strResult {
                    sr[self.currentIndexResult as usize] = tem.clone();
                    self.currentIndexResult += 1;
                  }
                }
                // java this.stackOperator.posh(strTem);
                self.stackOperator.posh(&strTem);
              }
            }
            // java else
            else {
              // java this.stackOperator.posh(strTem);
              self.stackOperator.posh(&strTem);
            }
            // java this.currentIndexFormal = i + 1;
            self.currentIndexFormal = i + 1;
          }
          i += 1;
        }
        // java if (this.currentIndexFormal < transStr.length())
        if self.currentIndexFormal < len {
          // java this.strResult[this.currentIndexResult++] = transStr.substring(this.currentIndexFormal);
          let s: String = transStr.chars().skip(self.currentIndexFormal as usize).collect();
          if let Some(ref mut sr) = self.strResult {
            sr[self.currentIndexResult as usize] = Some(s);
            self.currentIndexResult += 1;
          }
        }
        // java while (!this.stackOperator.isEmpty())
        while !self.stackOperator.isEmpty() {
          // java this.strResult[this.currentIndexResult++] = this.stackOperator.pop();
          let p = self.stackOperator.pop();
          if let Some(ref mut sr) = self.strResult {
            sr[self.currentIndexResult as usize] = p;
            self.currentIndexResult += 1;
          }
        }
        // java return (String[])Arrays.copyOf(this.strResult, this.currentIndexResult);
        let n = self.currentIndexResult as usize;
        return self.strResult.as_ref().map(|sr| {
          sr.iter().take(n).map(|o| o.clone().unwrap_or_default()).collect()
        }).unwrap_or_default();
      }
    }

    // java public String compute(String[] resultArray)
    pub fn compute(&mut self, resultArray: &Vec<String>) -> String {
      // java int length = resultArray.length;
      let mut length: i32 = resultArray.len() as i32;
      // java int i = 0;
      let mut i: i32 = 0;
      // java while (i < length)
      while i < length {
        // java String resultTem = resultArray[i];
        let mut resultTem: String = resultArray[i as usize].clone();
        // java if (!"(".equals(resultTem))
        if resultTem.as_str() != "(" {
          // java if (!priority.containsKey(resultTem))
          if !priority.contains_key(&resultTem) {
            // java this.stackCompute.posh(resultTem);
            self.stackCompute.posh(&resultTem);
            // java ++i;
            i += 1;
          }
          // java else if (">".equals(resultTem) && i < length - 1 && "<".equals(resultArray[i + 1]))
          else if resultTem.as_str() == ">" && i < length - 1 && resultArray[(i + 1) as usize].as_str() == "<" {
            // java String operationTem = resultArray[i + 1] + resultTem;
            let mut operationTem: String = format!("{}{}", resultArray[(i + 1) as usize], resultTem);
            // java this.commonOperation(operationTem);
            self.commonOperation(&operationTem);
            // java i += 2;
            i = i + 2;
          }
          // java else if ("=".equals(resultTem) && i < length - 1 && ">".equals(resultArray[i + 1]))
          else if resultTem.as_str() == "=" && i < length - 1 && resultArray[(i + 1) as usize].as_str() == ">" {
            // java String operationTem = resultArray[i + 1] + resultTem;
            let mut operationTem: String = format!("{}{}", resultArray[(i + 1) as usize], resultTem);
            // java this.commonOperation(operationTem);
            self.commonOperation(&operationTem);
            // java i += 2;
            i = i + 2;
          }
          // java else if ("=".equals(resultTem) && i < length - 1 && "<".equals(resultArray[i + 1]))
          else if resultTem.as_str() == "=" && i < length - 1 && resultArray[(i + 1) as usize].as_str() == "<" {
            // java String operationTem = resultArray[i + 1] + resultTem;
            let mut operationTem: String = format!("{}{}", resultArray[(i + 1) as usize], resultTem);
            // java this.commonOperation(operationTem);
            self.commonOperation(&operationTem);
            // java i += 2;
            i = i + 2;
          }
          // java else
          else {
            // java this.commonOperation(resultTem);
            self.commonOperation(&resultTem);
            // java ++i;
            i += 1;
          }
        }
        // java else
        else {
          // java ++i;
          i += 1;
        }
      }
      // java return this.stackCompute.getBottom();
      return self.stackCompute.getBottom().unwrap_or_default();
    }

    // java public FormulaOprational recycle()
    pub fn recycle(&mut self) -> &mut FormulaOprational {
      // java this.currentIndexFormal = 0;
      self.currentIndexFormal = 0;
      // java this.currentIndexResult = 0;
      self.currentIndexResult = 0;
      // java this.strResult = null;
      self.strResult = None;
      // java this.stackOperator.recycle();
      self.stackOperator.recycle();
      // java this.stackCompute.recycle();
      self.stackCompute.recycle();
      // java return this;
      return self;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.strResult = null;
      self.strResult = None;
      // java this.stackOperator.dispose();
      self.stackOperator.dispose();
      // java this.stackCompute.dispose();
      self.stackCompute.dispose();
    }

    // java private void commonOperation(String operaCharacter)
    pub fn commonOperation(&mut self, operaCharacter: &str) {
      // java String mStr = this.stackCompute.pop();
      let mut mStr: Option<String> = self.stackCompute.pop();
      // java String nStr = this.stackCompute.pop();
      let mut nStr: Option<String> = self.stackCompute.pop();
      // java if (mStr != null && nStr != null)
      if mStr.is_some() && nStr.is_some() {
        // java double m = Double.parseDouble(mStr);
        let mut m: f64 = mStr.as_ref().unwrap().parse().unwrap_or(0.0);
        // java double n = Double.parseDouble(nStr);
        let mut n: f64 = nStr.as_ref().unwrap().parse().unwrap_or(0.0);
        // java double temResult = OperationUtil.operational(n, m, operaCharacter);
        let mut temResult: f64 = OperationUtil::operational(n, m, operaCharacter);
        // java this.stackCompute.posh(String.valueOf(temResult));
        self.stackCompute.posh(&temResult.to_string());
      }
      // java else
      else {
        // java throw new GrammerErrorException("语法错误");
        panic!("{}", GrammerErrorException::new("语法错误"));
      }
    }

    // java private boolean priorityCompare(String operator1, String operator2)
    pub fn priorityCompare(&self, operator1: &str, operator2: &str) -> bool {
      // java if (!priority.containsKey(operator1))
      if !priority.contains_key(operator1) {
        // java throw new GrammerErrorException("语法错误");
        panic!("{}", GrammerErrorException::new("语法错误"));
      }
      // java else
      else {
        // java return (Integer)priority.get(operator1) >= (Integer)priority.get(operator2);
        let p1 = priority.get(operator1).copied().unwrap_or(0);
        let p2 = priority.get(operator2).copied().unwrap_or(0);
        return p1 >= p2;
      }
    }

}
