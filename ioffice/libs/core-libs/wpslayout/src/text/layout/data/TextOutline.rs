// Java: public class TextOutline
pub struct TextOutline {
    // Java field: TxtOutlnType mTxtOutlnType
    pub(crate) mTxtOutlnType: Arc<Mutex<TxtOutlnType>>,
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
    pub(crate) mGradAttr: Arc<Mutex<GradientAttr>>,
    // Java field: PatternAttr mPatternAttr
    pub(crate) mPatternAttr: Arc<Mutex<PatternAttr>>,
}

// Java: public enum TxtOutlnType
pub struct TxtOutlnType {
}

// Java: public class TextOutline
impl TextOutline {
    // Java: TextOutline()
    pub fn TextOutline(&self) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.mTxtOutlnType = TextOutline.TxtOutlnType.NONE;
      // TODO_RUST_CODE
      // Java: this.mLineWid = 0.75F;
      // TODO_RUST_CODE
      // Java: this.mLineType = 0;
      // TODO_RUST_CODE
      // Java: this.mComplexType = 0;
      // TODO_RUST_CODE
      // Java: this.mJoinType = 2;
      // TODO_RUST_CODE
      // Java: this.mCapType = 2;
      // TODO_RUST_CODE
      // Java: this.mLineClr = -16777216;
      // TODO_RUST_CODE
      // Java: this.mLineClrAlpha = 0.0F;
      // TODO_RUST_CODE
    }

    // Java: void setSolidLine(int lineClr, float lineClrAlpha, float lineWid, int lineType, int complexType, int joinType, int capType)
    pub fn setSolidLine(&self, int lineClr, float lineClrAlpha, float lineWid, int lineType, int complexType, int joinType, int capType) {
      // Java: this.mTxtOutlnType = TextOutline.TxtOutlnType.SOLID_LINE;
      // TODO_RUST_CODE
      // Java: this.mLineClr = lineClr;
      // TODO_RUST_CODE
      // Java: this.mLineClrAlpha = lineClrAlpha;
      // TODO_RUST_CODE
      // Java: this.mLineWid = lineWid;
      // TODO_RUST_CODE
      // Java: this.mLineType = lineType;
      // TODO_RUST_CODE
      // Java: this.mComplexType = complexType;
      // TODO_RUST_CODE
      // Java: this.mJoinType = joinType;
      // TODO_RUST_CODE
      // Java: this.mCapType = capType;
      // TODO_RUST_CODE
    }

    // Java: void setGradLine(GradientAttr gradAttr, float lineWid, int lineType, int complexType, int joinType, int capType)
    pub fn setGradLine(&self, GradientAttr gradAttr, float lineWid, int lineType, int complexType, int joinType, int capType) {
      // Java: this.mTxtOutlnType = TextOutline.TxtOutlnType.GRAD_LINE;
      // TODO_RUST_CODE
      // Java: this.mGradAttr = gradAttr;
      // TODO_RUST_CODE
      // Java: this.mLineWid = lineWid;
      // TODO_RUST_CODE
      // Java: this.mLineType = lineType;
      // TODO_RUST_CODE
      // Java: this.mComplexType = complexType;
      // TODO_RUST_CODE
      // Java: this.mJoinType = joinType;
      // TODO_RUST_CODE
      // Java: this.mCapType = capType;
      // TODO_RUST_CODE
    }

    // Java: void setPatternLine(PatternAttr patternAttr, float lineWid, int lineType, int complexType, int joinType, int capType)
    pub fn setPatternLine(&self, PatternAttr patternAttr, float lineWid, int lineType, int complexType, int joinType, int capType) {
      // Java: this.mTxtOutlnType = TextOutline.TxtOutlnType.PATTERN_LINE;
      // TODO_RUST_CODE
      // Java: this.mPatternAttr = patternAttr;
      // TODO_RUST_CODE
      // Java: this.mLineWid = lineWid;
      // TODO_RUST_CODE
      // Java: this.mLineType = lineType;
      // TODO_RUST_CODE
      // Java: this.mComplexType = complexType;
      // TODO_RUST_CODE
      // Java: this.mJoinType = joinType;
      // TODO_RUST_CODE
      // Java: this.mCapType = capType;
      // TODO_RUST_CODE
    }

    // Java: TxtOutlnType getTxtOutlnType()
    pub fn getTxtOutlnType(&self) -> TxtOutlnType {
      // Java: return this.mTxtOutlnType;
      // TODO_RUST_CODE
    }

    // Java: float getLineWid()
    pub fn getLineWid(&self) -> f32 {
      // Java: return this.mLineWid;
      // TODO_RUST_CODE
    }

    // Java: int getLineType()
    pub fn getLineType(&self) -> i32 {
      // Java: return this.mLineType;
      // TODO_RUST_CODE
    }

    // Java: int getComplexType()
    pub fn getComplexType(&self) -> i32 {
      // Java: return this.mComplexType;
      // TODO_RUST_CODE
    }

    // Java: int getJoinType()
    pub fn getJoinType(&self) -> i32 {
      // Java: return this.mJoinType;
      // TODO_RUST_CODE
    }

    // Java: int getCapType()
    pub fn getCapType(&self) -> i32 {
      // Java: return this.mCapType;
      // TODO_RUST_CODE
    }

    // Java: int getLineClr()
    pub fn getLineClr(&self) -> i32 {
      // Java: return this.mLineClr;
      // TODO_RUST_CODE
    }

    // Java: float getLineClrAlpha()
    pub fn getLineClrAlpha(&self) -> f32 {
      // Java: return this.mLineClrAlpha;
      // TODO_RUST_CODE
    }

    // Java: GradientAttr getGradAttr()
    pub fn getGradAttr(&self) -> GradientAttr {
      // Java: return this.mGradAttr;
      // TODO_RUST_CODE
    }

    // Java: PatternAttr getPatternAttr()
    pub fn getPatternAttr(&self) -> PatternAttr {
      // Java: return this.mPatternAttr;
      // TODO_RUST_CODE
    }

    // Java: boolean equals(TextOutline txtOutln)
    pub fn equals(&self, TextOutline txtOutln) -> bool {
      // Java: if (txtOutln == null)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else if (this.mTxtOutlnType != txtOutln.mTxtOutlnType)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: boolean equals = false;
         // TODO_RUST_CODE
         // Java: switch (this.mTxtOutlnType)
            // TODO_RUST_CODE
         {
            // Java: case NONE:
            // TODO_RUST_CODE
               // Java: equals = this.mTxtOutlnType == txtOutln.getTxtOutlnType();
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case SOLID_LINE:
            // TODO_RUST_CODE
               // Java: equals = this.mTxtOutlnType == txtOutln.getTxtOutlnType() && this.mLineWid == txtOutln.mLineWid && this.mLineType == txtOutln.mLineType && this.mComplexType == txtOutln.mComplexType && this.mLineClr == txtOutln.mLineClr && this.mLineClrAlpha == txtOutln.mLineClrAlpha && this.mJoinType == txtOutln.mJoinType && this.mCapType == txtOutln.mCapType;
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case GRAD_LINE:
            // TODO_RUST_CODE
               // Java: equals = this.mTxtOutlnType == txtOutln.getTxtOutlnType() && this.mLineWid == txtOutln.mLineWid && this.mLineType == txtOutln.mLineType && this.mComplexType == txtOutln.mComplexType && this.mGradAttr.equals(txtOutln.mGradAttr) && this.mJoinType == txtOutln.mJoinType && this.mCapType == txtOutln.mCapType;
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case PATTERN_LINE:
            // TODO_RUST_CODE
               // Java: equals = this.mTxtOutlnType == txtOutln.getTxtOutlnType() && this.mLineWid == txtOutln.mLineWid && this.mLineType == txtOutln.mLineType && this.mComplexType == txtOutln.mComplexType && this.mPatternAttr.equals(txtOutln.mPatternAttr) && this.mJoinType == txtOutln.mJoinType && this.mCapType == txtOutln.mCapType;
               // TODO_RUST_CODE
         }
         // Java: return equals;
         // TODO_RUST_CODE
      }
    }

}

// Java: public enum TxtOutlnType
impl TxtOutlnType {
    // Java: TxtOutlnType()
    pub fn TxtOutlnType(&self) {
    // TODO_RUST_CODE
    }

}
