// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.moffice.writer.data.field.exception.GrammerErrorException
use crate::cn::wps::moffice::writer::data::field::exception::GrammerErrorException::*;
// Java: import cn.wps.moffice.writer.data.field.parser.IFieldParser
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::*;
// Java: import cn.wps.moffice.writer.data.field.parser.ParserFactory
use crate::cn::wps::moffice::writer::data::field::parser::ParserFactory::*;
// Java: import java.util.Arrays

// java private static final int START = 19;
const START: i32 = 19;
// java private static final int SPERATOR = 20;
const SPERATOR: i32 = 20;
// java private static final int END = 21;
const END: i32 = 21;

// java public class ExtractFieldUtil
#[derive(Default)]
pub struct ExtractFieldUtil {
}

// java public class ExtractFieldUtil
impl ExtractFieldUtil {
    // java public static boolean checkContainField(char[] codeBuffer)
    pub fn checkContainField(codeBuffer: Option<&[char]>) -> bool {
      // java if (codeBuffer == null)
      if codeBuffer.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        let codeBuffer = codeBuffer.unwrap();
        let mut i: i32 = 0;
        // java for(int i = 0; i < codeBuffer.length; ++i)
        while i < (codeBuffer.len() as i32) {
          // java if (19 == codeBuffer[i])
          if 19 == (codeBuffer[i as usize] as i32) {
            // java return true;
            return true;
          }
          i += 1;
        }
        // java return false;
        return false;
      }
    }

    // java public static int getSperatorIndex(char[] codeBuffer)
    pub fn getSperatorIndex(codeBuffer: Option<&[char]>) -> i32 {
      // java if (codeBuffer == null)
      if codeBuffer.is_none() {
        // java return -1;
        return -1;
      }
      // java else
      else {
        let codeBuffer = codeBuffer.unwrap();
        let mut i: i32 = 0;
        // java for(int i = 0; i < codeBuffer.length; ++i)
        while i < (codeBuffer.len() as i32) {
          // java if (20 == codeBuffer[i])
          if 20 == (codeBuffer[i as usize] as i32) {
            // java return i;
            return i;
          }
          i += 1;
        }
        // java return -1;
        return -1;
      }
    }

    // java public static char[] extractField(char[] codeBuffer, Document subDoc)
    pub fn extractField(codeBuffer: Option<&[char]>, subDoc: &dyn Document) -> Option<Vec<char>> {
      // java if (codeBuffer == null)
      if codeBuffer.is_none() {
        // java return null;
        return None;
      }
      // java else if (!checkContainField(codeBuffer))
      else if !Self::checkContainField(codeBuffer) {
        let codeBuffer = codeBuffer.unwrap();
        // java int speratorIndex = getSperatorIndex(codeBuffer);
        let mut speratorIndex: i32 = Self::getSperatorIndex(Some(codeBuffer));
        // java return speratorIndex > -1 ? Arrays.copyOfRange(codeBuffer, speratorIndex + 1, codeBuffer.length) : parse(codeBuffer, subDoc);
        return if speratorIndex > -1 {
          Some(codeBuffer[(speratorIndex + 1) as usize..].to_vec())
        } else {
          Some(Self::parse(Some(codeBuffer), subDoc))
        };
      }
      // java else
      else {
        let mut codeBuffer: Vec<char> = codeBuffer.unwrap().to_vec();
        // java int stackLength = -1;
        let mut stackLength: i32 = -1;
        // java int firstIndex = -1;
        let mut firstIndex: i32 = -1;
        // java int lastIndex = -1;
        let mut lastIndex: i32 = -1;
        // java boolean flag = true;
        let mut flag: bool = true;
        // java char[] subCodeBuffer = null;
        let mut subCodeBuffer: Vec<char> = vec![];
        // java char[] subCharResult = null;
        let mut subCharResult: Vec<char> = vec![];
        // java int[] temStack = new int[codeBuffer.length];
        let mut temStack: Vec<i32> = vec![0; codeBuffer.len()];
        let mut i: i32 = 0;
        // java for(int i = 0; i < codeBuffer.length; ++i)
        while i < (codeBuffer.len() as i32) {
          // java if (flag && codeBuffer[i] == 19)
          if flag && (codeBuffer[i as usize] as i32) == 19 {
            // java firstIndex = i;
            firstIndex = i;
            // java flag = false;
            flag = false;
            // java ++stackLength;
            stackLength += 1;
            // java temStack[stackLength] = i;
            temStack[stackLength as usize] = i;
          }
          // java else if (codeBuffer[i] == 21)
          else if (codeBuffer[i as usize] as i32) == 21 {
            // java --stackLength;
            stackLength -= 1;
            // java if (stackLength == -1)
            if stackLength == -1 {
              // java lastIndex = i;
              lastIndex = i;
              // java break;
              break;
            }
          }
          i += 1;
        }
        // java subCodeBuffer = Arrays.copyOfRange(codeBuffer, firstIndex + 1, lastIndex);
        subCodeBuffer = codeBuffer[(firstIndex + 1) as usize..lastIndex as usize].to_vec();
        // java subCharResult = extractField(subCodeBuffer, subDoc);
        subCharResult = Self::extractField(Some(&subCodeBuffer), subDoc).unwrap_or_default();
        // java codeBuffer = merserCharArray(Arrays.copyOfRange(codeBuffer, 0, firstIndex), subCharResult, Arrays.copyOfRange(codeBuffer, lastIndex + 1, codeBuffer.length));
        let left = codeBuffer[0..firstIndex as usize].to_vec();
        let right = codeBuffer[(lastIndex + 1) as usize..].to_vec();
        codeBuffer = Self::merserCharArray(&[left.as_slice(), subCharResult.as_slice(), right.as_slice()]);
        // java return parse(codeBuffer, subDoc);
        return Some(Self::parse(Some(&codeBuffer), subDoc));
      }
    }

    // java private static char[] parse(char[] codeBuffer, Document subDoc)
    pub fn parse(codeBuffer: Option<&[char]>, subDoc: &dyn Document) -> Vec<char> {
      // java if (codeBuffer != null && codeBuffer.length != 0)
      if let Some(codeBuffer) = codeBuffer {
        if !codeBuffer.is_empty() {
          // java String fieldName = getFieldName(codeBuffer);
          let mut fieldName: String = Self::getFieldName(codeBuffer);
          // java KFieldType fieldType = KFieldType.getKFieldType(fieldName.toUpperCase());
          let mut fieldType: KFieldType = KFieldType::getKFieldType__String(Some(&fieldName.to_uppercase())).unwrap_or_default();
          // java if (fieldType == KFieldType.FieldAutoNum)
          if fieldType == KFieldType::FieldAutoNum {
            // java throw new GrammerErrorException(" ");
            panic!("{}", GrammerErrorException::new(" "));
          }
          // java else
          else {
            // java IFieldParser parser = ParserFactory.getFieldParser(fieldType);
            let mut parser: Box<dyn IFieldParser> = ParserFactory::getFieldParser(fieldType);
            // java parser.parseByCharBuffer(codeBuffer, subDoc, fieldType);
            parser.parseByCharBuffer(codeBuffer, subDoc, fieldType);
            // java return parser.getResult().toCharArray();
            return parser.getResult().unwrap_or_default().chars().collect();
          }
        }
      }
      // java else
      // java throw new GrammerErrorException(" ");
      panic!("{}", GrammerErrorException::new(" "));
    }

    // java private static String getFieldName(char[] codeBuffer)
    pub fn getFieldName(codeBuffer: &[char]) -> String {
      // java int lastIndex;
      let mut lastIndex: i32;
      lastIndex = 0;
      // java for(lastIndex = 0 codeBuffer[lastIndex] == ' ' ++lastIndex)
      while (lastIndex as usize) < codeBuffer.len() && codeBuffer[lastIndex as usize] == ' ' {
        lastIndex += 1;
      }
      // java if ('=' == codeBuffer[lastIndex])
      if (lastIndex as usize) < codeBuffer.len() && '=' == codeBuffer[lastIndex as usize] {
        // java return "=";
        return "=".to_string();
      }
      // java else
      else {
        // java int currIndex;
        let mut currIndex: i32;
        currIndex = lastIndex;
        // java for(currIndex = lastIndex codeBuffer[currIndex] != ' ' ++currIndex)
        while (currIndex as usize) < codeBuffer.len() && codeBuffer[currIndex as usize] != ' ' {
          currIndex += 1;
        }
        // java return new String(Arrays.copyOfRange(codeBuffer, lastIndex, currIndex));
        return codeBuffer[lastIndex as usize..currIndex as usize].iter().collect();
      }
    }

    // java public static char[] merserCharArray(char[]... args)
    pub fn merserCharArray(args: &[&[char]]) -> Vec<char> {
      // java int length = 0;
      let mut length: i32 = 0;
      let mut i: i32 = 0;
      // java for(int i = 0; i < args.length; ++i)
      while i < (args.len() as i32) {
        // java length += args[i].length;
        length = length + (args[i as usize].len() as i32);
        i += 1;
      }
      // java char[] newCharArray = new char[length];
      let mut newCharArray: Vec<char> = vec!['\0'; length as usize];
      // java int index = 0;
      let mut index: i32 = 0;
      i = 0;
      // java for(int i = 0; i < args.length; ++i)
      while i < (args.len() as i32) {
        // java char[] tem = args[i];
        let mut tem: &[char] = args[i as usize];
        let mut j: i32 = 0;
        // java for(int j = 0; j < tem.length ++j)
        while j < (tem.len() as i32) {
          // java newCharArray[index++] = tem[j];
          newCharArray[index as usize] = tem[j as usize];
          index += 1;
          j += 1;
        }
        i += 1;
      }
      // java return newCharArray;
      return newCharArray;
    }

}
