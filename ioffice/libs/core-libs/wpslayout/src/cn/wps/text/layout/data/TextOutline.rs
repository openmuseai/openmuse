use super::GradientAttr::GradientAttr;
use super::PatternAttr::PatternAttr;

// Java: public enum TxtOutlnType
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TxtOutlnType {
    NONE,
    SOLID_LINE,
    GRAD_LINE,
    PATTERN_LINE,
}

// Java: public class TextOutline
#[derive(Clone)]
pub struct TextOutline {
    // Java field: TxtOutlnType mTxtOutlnType
    pub(crate) mTxtOutlnType: TxtOutlnType,
    // Java field: float mLineWid
    pub(crate) mLineWid: f32,
    // Java field: int mLineType
    pub(crate) mLineType: i32,
    // Java field: int mComplexType
    pub(crate) mComplexType: i32,
    // Java field: int mJoinType
    pub(crate) mJoinType: i32,
    // Java field: int mCapType
    pub(crate) mCapType: i32,
    // Java field: int mLineClr
    pub(crate) mLineClr: i32,
    // Java field: float mLineClrAlpha
    pub(crate) mLineClrAlpha: f32,
    // Java field: GradientAttr mGradAttr
    pub(crate) mGradAttr: Option<GradientAttr>,
    // Java field: PatternAttr mPatternAttr
    pub(crate) mPatternAttr: Option<PatternAttr>,
}

// Java: public class TextOutline
impl TextOutline {
    // Java: TextOutline()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.mTxtOutlnType = TextOutline.TxtOutlnType.NONE;
      // Java: this.mLineWid = 0.75F;
      // Java: this.mLineType = 0;
      // Java: this.mComplexType = 0;
      // Java: this.mJoinType = 2;
      // Java: this.mCapType = 2;
      // Java: this.mLineClr = -16777216;
      // Java: this.mLineClrAlpha = 0.0F;
      TextOutline {
          mTxtOutlnType: TxtOutlnType::NONE,
          mLineWid: 0.75,
          mLineType: 0,
          mComplexType: 0,
          mJoinType: 2,
          mCapType: 2,
          mLineClr: -16777216,
          mLineClrAlpha: 0.0,
          mGradAttr: None,
          mPatternAttr: None,
      }
    }

    // Java: void setSolidLine(int lineClr, float lineClrAlpha, float lineWid, int lineType, int complexType, int joinType, int capType)
    pub fn setSolidLine(&mut self, lineClr: i32, lineClrAlpha: f32, lineWid: f32, lineType: i32, complexType: i32, joinType: i32, capType: i32) {
      // Java: this.mTxtOutlnType = TextOutline.TxtOutlnType.SOLID_LINE;
      self.mTxtOutlnType = TxtOutlnType::SOLID_LINE;
      // Java: this.mLineClr = lineClr;
      self.mLineClr = lineClr;
      // Java: this.mLineClrAlpha = lineClrAlpha;
      self.mLineClrAlpha = lineClrAlpha;
      // Java: this.mLineWid = lineWid;
      self.mLineWid = lineWid;
      // Java: this.mLineType = lineType;
      self.mLineType = lineType;
      // Java: this.mComplexType = complexType;
      self.mComplexType = complexType;
      // Java: this.mJoinType = joinType;
      self.mJoinType = joinType;
      // Java: this.mCapType = capType;
      self.mCapType = capType;
    }

    // Java: void setGradLine(GradientAttr gradAttr, float lineWid, int lineType, int complexType, int joinType, int capType)
    pub fn setGradLine(&mut self, gradAttr: GradientAttr, lineWid: f32, lineType: i32, complexType: i32, joinType: i32, capType: i32) {
      // Java: this.mTxtOutlnType = TextOutline.TxtOutlnType.GRAD_LINE;
      self.mTxtOutlnType = TxtOutlnType::GRAD_LINE;
      // Java: this.mGradAttr = gradAttr;
      self.mGradAttr = Some(gradAttr);
      // Java: this.mLineWid = lineWid;
      self.mLineWid = lineWid;
      // Java: this.mLineType = lineType;
      self.mLineType = lineType;
      // Java: this.mComplexType = complexType;
      self.mComplexType = complexType;
      // Java: this.mJoinType = joinType;
      self.mJoinType = joinType;
      // Java: this.mCapType = capType;
      self.mCapType = capType;
    }

    // Java: void setPatternLine(PatternAttr patternAttr, float lineWid, int lineType, int complexType, int joinType, int capType)
    pub fn setPatternLine(&mut self, patternAttr: PatternAttr, lineWid: f32, lineType: i32, complexType: i32, joinType: i32, capType: i32) {
      // Java: this.mTxtOutlnType = TextOutline.TxtOutlnType.PATTERN_LINE;
      self.mTxtOutlnType = TxtOutlnType::PATTERN_LINE;
      // Java: this.mPatternAttr = patternAttr;
      self.mPatternAttr = Some(patternAttr);
      // Java: this.mLineWid = lineWid;
      self.mLineWid = lineWid;
      // Java: this.mLineType = lineType;
      self.mLineType = lineType;
      // Java: this.mComplexType = complexType;
      self.mComplexType = complexType;
      // Java: this.mJoinType = joinType;
      self.mJoinType = joinType;
      // Java: this.mCapType = capType;
      self.mCapType = capType;
    }

    // Java: TxtOutlnType getTxtOutlnType()
    pub fn getTxtOutlnType(&self) -> TxtOutlnType {
      // Java: return this.mTxtOutlnType;
      self.mTxtOutlnType
    }

    // Java: float getLineWid()
    pub fn getLineWid(&self) -> f32 {
      // Java: return this.mLineWid;
      self.mLineWid
    }

    // Java: int getLineType()
    pub fn getLineType(&self) -> i32 {
      // Java: return this.mLineType;
      self.mLineType
    }

    // Java: int getComplexType()
    pub fn getComplexType(&self) -> i32 {
      // Java: return this.mComplexType;
      self.mComplexType
    }

    // Java: int getJoinType()
    pub fn getJoinType(&self) -> i32 {
      // Java: return this.mJoinType;
      self.mJoinType
    }

    // Java: int getCapType()
    pub fn getCapType(&self) -> i32 {
      // Java: return this.mCapType;
      self.mCapType
    }

    // Java: int getLineClr()
    pub fn getLineClr(&self) -> i32 {
      // Java: return this.mLineClr;
      self.mLineClr
    }

    // Java: float getLineClrAlpha()
    pub fn getLineClrAlpha(&self) -> f32 {
      // Java: return this.mLineClrAlpha;
      self.mLineClrAlpha
    }

    // Java: GradientAttr getGradAttr()
    pub fn getGradAttr(&self) -> Option<&GradientAttr> {
      // Java: return this.mGradAttr;
      self.mGradAttr.as_ref()
    }

    // Java: PatternAttr getPatternAttr()
    pub fn getPatternAttr(&self) -> Option<&PatternAttr> {
      // Java: return this.mPatternAttr;
      self.mPatternAttr.as_ref()
    }

    // Java: boolean equals(TextOutline txtOutln)
    pub fn equals(&self, txtOutln: Option<&TextOutline>) -> bool {
      // Java: if (txtOutln == null)
      let txtOutln = match txtOutln {
          None =>
      {
         // Java: return false;
         return false;
      }
      // Java: else if (this.mTxtOutlnType != txtOutln.mTxtOutlnType)
          Some(t) => t,
      };
      if self.mTxtOutlnType != txtOutln.mTxtOutlnType
      {
         // Java: return false;
         return false;
      }
      // Java: else
      else
      {
         // Java: boolean equals = false;
         let equals: bool;
         // Java: switch (this.mTxtOutlnType)
         match self.mTxtOutlnType
         {
            // Java: case NONE:
            TxtOutlnType::NONE => {
               // Java: equals = this.mTxtOutlnType == txtOutln.getTxtOutlnType();
               equals = self.mTxtOutlnType == txtOutln.mTxtOutlnType;
               // Java: break;
            }
            // Java: case SOLID_LINE:
            TxtOutlnType::SOLID_LINE => {
               // Java: equals = this.mTxtOutlnType == txtOutln.getTxtOutlnType() && this.mLineWid == txtOutln.mLineWid && this.mLineType == txtOutln.mLineType && this.mComplexType == txtOutln.mComplexType && this.mLineClr == txtOutln.mLineClr && this.mLineClrAlpha == txtOutln.mLineClrAlpha && this.mJoinType == txtOutln.mJoinType && this.mCapType == txtOutln.mCapType;
               equals = self.mLineWid == txtOutln.mLineWid
                   && self.mLineType == txtOutln.mLineType
                   && self.mComplexType == txtOutln.mComplexType
                   && self.mLineClr == txtOutln.mLineClr
                   && self.mLineClrAlpha == txtOutln.mLineClrAlpha
                   && self.mJoinType == txtOutln.mJoinType
                   && self.mCapType == txtOutln.mCapType;
               // Java: break;
            }
            // Java: case GRAD_LINE:
            TxtOutlnType::GRAD_LINE => {
               // Java: equals = this.mTxtOutlnType == txtOutln.getTxtOutlnType() && this.mLineWid == txtOutln.mLineWid && this.mLineType == txtOutln.mLineType && this.mComplexType == txtOutln.mComplexType && this.mGradAttr.equals(txtOutln.mGradAttr) && this.mJoinType == txtOutln.mJoinType && this.mCapType == txtOutln.mCapType;
               equals = self.mLineWid == txtOutln.mLineWid
                   && self.mLineType == txtOutln.mLineType
                   && self.mComplexType == txtOutln.mComplexType
                   && self.mGradAttr.as_ref().map_or(txtOutln.mGradAttr.is_none(), |a| a.equals(txtOutln.mGradAttr.as_ref()))
                   && self.mJoinType == txtOutln.mJoinType
                   && self.mCapType == txtOutln.mCapType;
               // Java: break;
            }
            // Java: case PATTERN_LINE:
            TxtOutlnType::PATTERN_LINE => {
               // Java: equals = this.mTxtOutlnType == txtOutln.getTxtOutlnType() && this.mLineWid == txtOutln.mLineWid && this.mLineType == txtOutln.mLineType && this.mComplexType == txtOutln.mComplexType && this.mPatternAttr.equals(txtOutln.mPatternAttr) && this.mJoinType == txtOutln.mJoinType && this.mCapType == txtOutln.mCapType;
               equals = self.mLineWid == txtOutln.mLineWid
                   && self.mLineType == txtOutln.mLineType
                   && self.mComplexType == txtOutln.mComplexType
                   && self.mPatternAttr.as_ref().map_or(txtOutln.mPatternAttr.is_none(), |a| a.equals(txtOutln.mPatternAttr.as_ref()))
                   && self.mJoinType == txtOutln.mJoinType
                   && self.mCapType == txtOutln.mCapType;
            }
         }
         // Java: return equals;
         equals
      }
    }

}

// Java: public enum TxtOutlnType
impl TxtOutlnType {
    // Java: TxtOutlnType()
}
