// java public class HtmlPasteBuriedPointModel
#[derive(Default)]
pub struct HtmlPasteBuriedPointModel {
    // java int mW = 0;
    pub(crate) mW: i32,
    // java int mH = 0;
    pub(crate) mH: i32,
    // java int mTableNumber = 0;
    pub(crate) mTableNumber: i32,
    // java public int mTempW;
    pub(crate) mTempW: i32,
    // java public int mMaxTableW;
    pub(crate) mMaxTableW: i32,
    // java public int mMaxTableH;
    pub(crate) mMaxTableH: i32,
}

// java public class HtmlPasteBuriedPointModel
impl HtmlPasteBuriedPointModel {
    // java public void clear()
    pub fn clear(&mut self) {
      // java this.mTempW = 0;
      self.mTempW = 0;
      // java this.mMaxTableW = 0;
      self.mMaxTableW = 0;
      // java this.mMaxTableH = 0;
      self.mMaxTableH = 0;
    }

    // java public void trEnd()
    pub fn trEnd(&mut self) {
      // java this.mMaxTableW = Math.max(this.mTempW, this.mMaxTableW);
      self.mMaxTableW = self.mTempW.max(self.mMaxTableW);
      // java this.mTempW = 0;
      self.mTempW = 0;
    }

    // java public void tableEnd()
    pub fn tableEnd(&mut self) {
      // java if (this.mMaxTableW * this.mMaxTableH > this.mW * this.mH)
      if self.mMaxTableW * self.mMaxTableH > self.mW * self.mH {
        // java this.mW = this.mMaxTableW;
        self.mW = self.mMaxTableW;
        // java this.mH = this.mMaxTableH;
        self.mH = self.mMaxTableH;
      }
      // java this.clear();
      self.clear();
    }

    // java public void tdBegin()
    pub fn tdBegin(&mut self) {
      // java ++this.mTempW;
      { let _tmp = self.mTempW; self.mTempW += 1; _tmp };
    }

    // java public void trBegin()
    pub fn trBegin(&mut self) {
      // java ++this.mMaxTableH;
      { let _tmp = self.mMaxTableH; self.mMaxTableH += 1; _tmp };
    }

    // java public int getTableNumber()
    pub fn getTableNumber(&self) -> i32 {
      // java return this.mTableNumber;
      return self.mTableNumber;
    }

    // java public String getWHString()
    pub fn getWHString(&self) -> String {
      // java return this.mH + "_" + this.mW;
      return format!("{}_{}", self.mH, self.mW);
    }

    // java public void addTableNumber()
    pub fn addTableNumber(&mut self) {
      // java ++this.mTableNumber;
      { let _tmp = self.mTableNumber; self.mTableNumber += 1; _tmp };
    }

}
