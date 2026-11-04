// Java: import cn.wps.moffice.writer.data.field.exception.GrammerErrorException
use crate::cn::wps::moffice::writer::data::field::exception::GrammerErrorException::*;
// Java: import cn.wps.moffice.writer.data.field.exception.UndefineBookmarkException
use crate::cn::wps::moffice::writer::data::field::exception::UndefineBookmarkException::*;
use crate::cn::wps::moffice::writer::data::field::parser::formula::OperationUtil::OperationUtil;
use crate::cn::wps::moffice::writer::data::field::parser::formula::FormulaFunction::FormulaFunction;
use crate::cn::wps::moffice::writer::data::field::parser::formula::FormulaFunction::functionMap;

// java public class FormulaCompre
#[derive(Default)]
pub struct FormulaCompre {
}

// java public class FormulaCompre
impl FormulaCompre {
    // java public static String parseFunction(String args)
    pub fn parseFunction(args: Option<&str>) -> Option<String> {
      // java if (args != null && args.length() != 0)
      if let Some(args) = args {
        if !args.is_empty() {
          // java args = OperationUtil.initString(args);
          let mut args: String = OperationUtil::initString(args);
          // java if (OperationUtil.checkStrIsOperation(args))
          if OperationUtil::checkStrIsOperation(&args) {
            // java return OperationUtil.compute(args);
            return Some(OperationUtil::compute(&args));
          }
          // java else
          else {
            // java int[] index = OperationUtil.extractFunctionStr(args);
            let mut index: Vec<i32> = OperationUtil::extractFunctionStr(&args);
            // java String functionStr = args.substring(index[0], index[1]);
            let mut functionStr: String = args.chars().skip(index[0] as usize).take((index[1] - index[0]) as usize).collect();
            // java String functionName = OperationUtil.getFulmulaFuncitonNameStr(functionStr);
            let mut functionName: Option<String> = OperationUtil::getFulmulaFuncitonNameStr(&functionStr);
            // java if ("false".equals(functionName))
            if functionName.as_deref() == Some("false") {
              // java return "0";
              return Some("0".to_string());
            }
            // java else if ("true".equals(functionName))
            else if functionName.as_deref() == Some("true") {
              // java return "1";
              return Some("1".to_string());
            }
            // java else
            else {
              // java String params = OperationUtil.getFunctionParams(functionStr);
              let mut params: String = OperationUtil::getFunctionParams(&functionStr);
              // java String[] paramsArray = OperationUtil.getParamsStrArray(params);
              let mut paramsArray: Option<Vec<String>> = OperationUtil::getParamsStrArray(Some(&params));
              // java String functionResult = operationalFunction(functionName, paramsArray);
              let mut functionResult: Option<String> = Self::operationalFunction(functionName.as_deref(), paramsArray.as_mut());
              // java args = args.substring(0, index[0]) + functionResult + args.substring(index[1]);
              let prefix: String = args.chars().take(index[0] as usize).collect();
              let suffix: String = args.chars().skip(index[1] as usize).collect();
              args = format!("{}{}{}", prefix, functionResult.unwrap_or_default(), suffix);
              // java return parseFunction(args);
              return Self::parseFunction(Some(&args));
            }
          }
        }
      }
      // java else
      // java return null;
      return None;
    }

    // java public static String operationalFunction(String functionName, String[] params)
    pub fn operationalFunction(functionName: Option<&str>, params: Option<&mut Vec<String>>) -> Option<String> {
      // java if (functionName != null && params != null)
      if let (Some(functionName), Some(params)) = (functionName, params) {
        // java if (!FormulaFunction.existFunction(functionName))
        if !FormulaFunction::existFunction(functionName) {
          // java throw new GrammerErrorException(functionName);
          panic!("{}", GrammerErrorException::new(functionName));
        }
        // java else
        else {
          let mut i: i32 = 0;
          // java for(int i = 0; i < params.length; ++i)
          while i < (params.len() as i32) {
            // java if (OperationUtil.checkStrIsOperation(params[i]))
            if OperationUtil::checkStrIsOperation(&params[i as usize]) {
              // java params[i] = OperationUtil.compute(params[i]);
              params[i as usize] = OperationUtil::compute(&params[i as usize]);
            }
            // java else
            else {
              // java params[i] = parseFunction(params[i]);
              params[i as usize] = Self::parseFunction(Some(&params[i as usize])).unwrap_or_default();
            }
            i += 1;
          }
          // java int functionMark = (Integer)FormulaFunction.functionMap.get(functionName);
          let mut functionMark: i32 = functionMap.get(functionName).copied().unwrap_or(-1);
          // java switch (functionMark)
          match functionMark {
            // java case 1:
            1 => {
              // java return FormulaFunction.abs(params);
              return Some(FormulaFunction::abs(Some(params)));
            }
            // java case 2:
            2 => {
              // java return FormulaFunction.and(params);
              return Some(FormulaFunction::and(Some(params)));
            }
            // java case 3:
            3 => {
              // java return FormulaFunction.average(params);
              return Some(FormulaFunction::average(Some(params)));
            }
            // java case 4:
            4 => {
              // java return FormulaFunction.count(params);
              return Some(FormulaFunction::count(Some(params)));
            }
            // java case 5:
            5 => {
              // java return FormulaFunction.define(params);
              return Some(FormulaFunction::define(Some(params)));
            }
            // java case 6:
            6 => {
              // java return FormulaFunction.if_custom(params);
              return Some(FormulaFunction::if_custom(Some(params)));
            }
            // java case 7:
            7 => {
              // java return FormulaFunction.int_custom(params);
              return Some(FormulaFunction::int_custom(Some(params)));
            }
            // java case 8:
            8 => {
              // java return FormulaFunction.min(params);
              return Some(FormulaFunction::min(Some(params)));
            }
            // java case 9:
            9 => {
              // java return FormulaFunction.max(params);
              return Some(FormulaFunction::max(Some(params)));
            }
            // java case 10:
            10 => {
              // java return FormulaFunction.mod(params);
              return Some(FormulaFunction::r#mod(Some(params)));
            }
            // java case 11:
            11 => {
              // java return FormulaFunction.not(params);
              return Some(FormulaFunction::not(Some(params)));
            }
            // java case 12:
            12 => {
              // java return FormulaFunction.or(params);
              return Some(FormulaFunction::or(Some(params)));
            }
            // java case 13:
            13 => {
              // java return FormulaFunction.product(params);
              return Some(FormulaFunction::product(Some(params)));
            }
            // java case 14:
            14 => {
              // java return FormulaFunction.round(params);
              return Some(FormulaFunction::round(Some(params)));
            }
            // java case 16:
            16 => {
              // java return FormulaFunction.sign(params);
              return Some(FormulaFunction::sign(Some(params)));
            }
            // java case 17:
            17 => {
              // java return FormulaFunction.sum(params);
              return Some(FormulaFunction::sum(Some(params)));
            }
            // java case 15: default:
            15 | _ => {
              // java throw new UndefineBookmarkException("");
              panic!("{}", UndefineBookmarkException::new(""));
            }
          }
        }
      }
      // java else
      // java return null;
      return None;
    }

}
