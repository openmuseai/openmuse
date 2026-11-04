#![allow(non_snake_case)]
#![allow(dead_code)]

// Java: public class AnalyzeContext
pub struct AnalyzeContext {
    // Java field: short type
    pub(crate) r#type: i16,
    // Java field: int curPos
    pub(crate) curPos: i32,
    // Java field: int leftDigital
    pub(crate) leftDigital: i32,
    // Java field: int firstIntPartPos
    pub(crate) firstIntPartPos: i32,
    // Java field: int lastIntPartPos
    pub(crate) lastIntPartPos: i32,
    // Java field: int firstDeciPartPos
    pub(crate) firstDeciPartPos: i32,
    // Java field: int lastDeciPartPos
    pub(crate) lastDeciPartPos: i32,
    // Java field: int firstDeciPointPos
    pub(crate) firstDeciPointPos: i32,
    // Java field: int lastDigitalPos
    pub(crate) lastDigitalPos: i32,
    // Java field: int scienceEPos
    pub(crate) scienceEPos: i32,
    // Java field: int secondDeciPointPos
    pub(crate) secondDeciPointPos: i32,
    // Java field: int firstDigitAfterE
    pub(crate) firstDigitAfterE: i32,
    // Java field: int lastDigitAfterE
    pub(crate) lastDigitAfterE: i32,
    // Java field: int lastDigitBeforeE
    pub(crate) lastDigitBeforeE: i32,
    // Java field: int leftDigiRemind
    pub(crate) leftDigiRemind: i32,
    // Java field: boolean hasDecimal
    pub(crate) hasDecimal: bool,
    // Java field: boolean hasComma
    pub(crate) hasComma: bool,
    // Java field: boolean thouSepInIntPart
    pub(crate) thouSepInIntPart: bool,
    // Java field: StringBuffer insertContent
    pub(crate) insertContent: String,
    // Java field: int fractionStartPos
    pub(crate) fractionStartPos: i32,
    // Java field: int fractionEndPos
    pub(crate) fractionEndPos: i32,
    // Java field: int fractionPos
    pub(crate) fractionPos: i32,
    // Java field: int percentPos
    pub(crate) percentPos: i32,
}

// Java: public class AnalyzeContext
impl AnalyzeContext {
    // Java: AnalyzeContext()
    pub fn new() -> Self {
      // Java: super();
      AnalyzeContext {
          r#type: 0,
          curPos: 0,
          leftDigital: 0,
          firstIntPartPos: 0,
          lastIntPartPos: 0,
          firstDeciPartPos: 0,
          lastDeciPartPos: 0,
          firstDeciPointPos: 0,
          lastDigitalPos: 0,
          scienceEPos: 0,
          secondDeciPointPos: 0,
          firstDigitAfterE: 0,
          lastDigitAfterE: 0,
          lastDigitBeforeE: 0,
          leftDigiRemind: 0,
          hasDecimal: false,
          hasComma: false,
          thouSepInIntPart: false,
          insertContent: String::new(),
          fractionStartPos: 0,
          fractionEndPos: 0,
          fractionPos: 0,
          percentPos: 0,
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.type = 0;
      self.r#type = 0;
      // Java: this.curPos = -1;
      self.curPos = -1;
      // Java: this.leftDigital = 0;
      self.leftDigital = 0;
      // Java: this.firstIntPartPos = -1;
      self.firstIntPartPos = -1;
      // Java: this.lastIntPartPos = -1;
      self.lastIntPartPos = -1;
      // Java: this.firstDeciPartPos = -1;
      self.firstDeciPartPos = -1;
      // Java: this.lastDeciPartPos = -1;
      self.lastDeciPartPos = -1;
      // Java: this.firstDeciPointPos = -1;
      self.firstDeciPointPos = -1;
      // Java: this.lastDigitalPos = -1;
      self.lastDigitalPos = -1;
      // Java: this.scienceEPos = -1;
      self.scienceEPos = -1;
      // Java: this.secondDeciPointPos = -1;
      self.secondDeciPointPos = -1;
      // Java: this.firstDigitAfterE = -1;
      self.firstDigitAfterE = -1;
      // Java: this.lastDigitAfterE = -1;
      self.lastDigitAfterE = -1;
      // Java: this.lastDigitBeforeE = -1;
      self.lastDigitBeforeE = -1;
      // Java: this.leftDigiRemind = 0;
      self.leftDigiRemind = 0;
      // Java: this.hasDecimal = false;
      self.hasDecimal = false;
      // Java: this.hasComma = false;
      self.hasComma = false;
      // Java: this.thouSepInIntPart = false;
      self.thouSepInIntPart = false;
      // Java: this.fractionStartPos = -1;
      self.fractionStartPos = -1;
      // Java: this.fractionEndPos = -1;
      self.fractionEndPos = -1;
      // Java: this.fractionPos = -1;
      self.fractionPos = -1;
      // Java: this.percentPos = -1;
      self.percentPos = -1;
      // Java: this.insertContent.setLength(0);
      self.insertContent.clear();
    }

}
