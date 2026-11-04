use super::IWordFmtAnalyzer::IWordFmtAnalyzer;

// Java: public class WordFmtAnalyzer implements IWordFmtAnalyzer
pub struct WordFmtAnalyzer {
    // Java field: boolean mIsNumeric
    mIsNumeric: bool,
}

// Java: public class WordFmtAnalyzer implements IWordFmtAnalyzer
impl WordFmtAnalyzer {
    // Java: WordFmtAnalyzer()
    pub fn new() -> Self {
      // Java: super();
      WordFmtAnalyzer {
          mIsNumeric: false,
      }
    }

    // Java: static String split(String formatCode)
    pub fn split(format_code: &str) -> &str {
      // Java: int index = formatCode.indexOf(34);
      let index = format_code.find('"');
      // Java: return index == -1 ? formatCode : formatCode.substring(0, index);
      match index {
          None => format_code,
          Some(i) => &format_code[..i],
      }
    }

    // Java: int analyze(String formatCode, short type)
    pub fn analyze_with_type(&mut self, format_code: &str, type_: i16) -> i32 {
      // Java: String split = split(formatCode);
      let split = Self::split(format_code);
      // Java: switch (type)
      match type_
      {
         // Java: case 0:
         0 => {
            // Java: return this.analyze(split);
            return self.analyze_internal(split);
         }
         // Java: case 1:
         1 => {
            // Java: this.mIsNumeric = true;
            self.mIsNumeric = true;
            // Java: return this.analyze(split);
            return self.analyze_internal(split);
         }
         // Java: case 2:
         2 => {
            // Java: return this.analyzeGeneral(split);
            return self.analyzeGeneral(split);
         }
         // Java: default:
         _ => {
            // Java: return 1;
            return 1;
         }
      }
    }

    // Java: int analyzeGeneral(String formatCode)
    pub fn analyzeGeneral(&self, _format_code: &str) -> i32 {
      // Java: return 1;
      1
    }

    // Java: int analyze(String formatCode)
    pub fn analyze_internal(&self, format_code: &str) -> i32 {
      // Java: int len = formatCode.length();
      let _len = format_code.len();
      // Java: boolean isInRef = false;
      let mut is_in_ref = false;
      // Java: for(int i = 0; i < len; ++i)
      for c in format_code.chars()
      {
         // Java: char c = formatCode.charAt(i);
         // Java: switch (c)
         match c
         {
            // Java: case '\'':
            '\'' => {
               // Java: isInRef = !isInRef;
               is_in_ref = !is_in_ref;
               // Java: break;
            }
            // Java: case 'N':
            'N' => {
               // Java: if (!isInRef && !this.mIsNumeric)
               if !is_in_ref && !self.mIsNumeric
               {
                  // Java: return 2;
                  return 2;
               }
            }
            _ => {}
         }
      }
      // Java: if (isInRef)
      if is_in_ref
      {
         // Java: return 3;
         return 3;
      }
      // Java: else
      else
      {
         // Java: return 0;
         return 0;
      }
    }

}

impl IWordFmtAnalyzer for WordFmtAnalyzer {
    fn analyze(&self, var1: &str, var2: i16) -> i32 {
        let mut clone = WordFmtAnalyzer { mIsNumeric: self.mIsNumeric };
        clone.analyze_with_type(var1, var2)
    }
}
