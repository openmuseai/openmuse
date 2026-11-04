use super::BulletsInfo::BulletsInfo;

// Java field: short ALIGN_GENERAL
pub(crate) const ALIGN_GENERAL: i16 = 0;
// Java field: short ALIGN_LEFT
pub(crate) const ALIGN_LEFT: i16 = 1;
// Java field: short ALIGN_CENTER
pub(crate) const ALIGN_CENTER: i16 = 2;
// Java field: short ALIGN_RIGHT
pub(crate) const ALIGN_RIGHT: i16 = 3;
// Java field: short ALIGN_JUSTIFY
pub(crate) const ALIGN_JUSTIFY: i16 = 5;
// Java field: short ALIGN_CENTER_SELECTION
pub(crate) const ALIGN_CENTER_SELECTION: i16 = 6;
// Java field: short ALIGN_DISTRIBUTED
pub(crate) const ALIGN_DISTRIBUTED: i16 = 7;

// Java: public class ParaInfo
pub struct ParaInfo {
    // Java field: int mDefTabSize
    pub mDefTabSize: i32,
    // Java field: boolean eaLnBrk
    pub eaLnBrk: bool,
    // Java field: boolean hangingPunct
    pub hangingPunct: bool,
    // Java field: boolean latinLnBrk
    pub latinLnBrk: bool,
    // Java field: short mHAlignment
    pub mHAlignment: i16,
    // Java field: short mVAlignment
    pub mVAlignment: i16,
    // Java field: short mMarginL
    pub mMarginL: i16,
    // Java field: short mMarginR
    pub mMarginR: i16,
    // Java field: short mIndent
    pub mIndent: i16,
    // Java field: short mAngle
    pub mAngle: i16,
    // Java field: double mSpcBef
    pub mSpcBef: f64,
    // Java field: double mSpcAft
    pub mSpcAft: f64,
    // Java field: double mSpcWithinPct
    pub mSpcWithinPct: f64,
    // Java field: double mSpcWithinPts
    pub mSpcWithinPts: f64,
    // Java field: boolean mIsWrapText
    pub mIsWrapText: bool,
    // Java field: boolean mIsVerticalText
    pub mIsVerticalText: bool,
    // Java field: boolean mIsRightToLeft
    pub mIsRightToLeft: bool,
    // Java field: BulletsInfo mbulletsInfo
    pub mbulletsInfo: BulletsInfo,
    // Java field: int start
    pub start: i32,
    // Java field: int[] invertMap
    pub invertMap: Option<Vec<i32>>,
    // Java field: int[] indexMap
    pub indexMap: Option<Vec<i32>>,
    // Java field: int mFirstLead
    pub mFirstLead: i32,
    // Java field: int mRestLead
    pub mRestLead: i32,
    // Java field: float[] mtabStops
    pub mtabStops: Option<Vec<f32>>,
    // Java field: double mSpcWithinPctOri
    pub mSpcWithinPctOri: f64,
}

// Java: public class ParaInfo
impl ParaInfo {
    // Java: ParaInfo()
    pub fn new() -> Self {
      // Java: super();
      ParaInfo {
        // Java: this.mHAlignment = 0;
        mHAlignment: 0,
        // Java: this.mIndent = 0;
        mIndent: 0,
        // Java: this.mMarginL = 0;
        mMarginL: 0,
        // Java: this.mMarginR = 0;
        mMarginR: 0,
        // Java: this.mSpcWithinPct = (double)1.0F;
        mSpcWithinPct: 1.0,
        // Java: this.mSpcWithinPts = (double)0.0F;
        mSpcWithinPts: 0.0,
        mDefTabSize: 0,
        eaLnBrk: true,
        hangingPunct: true,
        latinLnBrk: false,
        mVAlignment: 0,
        mAngle: 0,
        mSpcBef: 0.0,
        mSpcAft: 0.0,
        mIsWrapText: false,
        mIsVerticalText: false,
        mIsRightToLeft: false,
        mbulletsInfo: BulletsInfo::new(),
        start: 0,
        invertMap: None,
        indexMap: None,
        mFirstLead: 0,
        mRestLead: 0,
        mtabStops: None,
        mSpcWithinPctOri: 0.0,
      }
    }

    // Java: ParaInfo(ParaInfo para)
    pub fn new_from(para: &ParaInfo) -> Self {
      // Java: super();
      ParaInfo {
        // Java: this.mHAlignment = para.mHAlignment;
        mHAlignment: para.mHAlignment,
        // Java: this.mMarginL = para.mMarginL;
        mMarginL: para.mMarginL,
        // Java: this.mMarginR = para.mMarginR;
        mMarginR: para.mMarginR,
        // Java: this.mIndent = para.mIndent;
        mIndent: para.mIndent,
        // Java: this.mAngle = para.mAngle;
        mAngle: para.mAngle,
        // Java: this.mSpcBef = para.mSpcBef;
        mSpcBef: para.mSpcBef,
        // Java: this.mSpcAft = para.mSpcAft;
        mSpcAft: para.mSpcAft,
        // Java: this.mSpcWithinPct = para.mSpcWithinPct;
        mSpcWithinPct: para.mSpcWithinPct,
        // Java: this.mSpcWithinPts = para.mSpcWithinPts;
        mSpcWithinPts: para.mSpcWithinPts,
        // Java: this.mSpcWithinPctOri = para.mSpcWithinPct;
        mSpcWithinPctOri: para.mSpcWithinPct,
        // Java: this.mIsWrapText = para.mIsWrapText;
        mIsWrapText: para.mIsWrapText,
        // Java: this.mIsVerticalText = para.mIsVerticalText;
        mIsVerticalText: para.mIsVerticalText,
        // Java: this.mIsRightToLeft = para.mIsRightToLeft;
        mIsRightToLeft: para.mIsRightToLeft,
        // Java: this.mbulletsInfo = para.mbulletsInfo;
        mbulletsInfo: BulletsInfo::new_from(&para.mbulletsInfo),
        // Java: this.start = para.start;
        start: para.start,
        // Java: this.mFirstLead = para.mFirstLead;
        mFirstLead: para.mFirstLead,
        // Java: this.mRestLead = para.mRestLead;
        mRestLead: para.mRestLead,
        // Java: this.mDefTabSize = para.mDefTabSize;
        mDefTabSize: para.mDefTabSize,
        // Java: this.eaLnBrk = para.eaLnBrk;
        eaLnBrk: para.eaLnBrk,
        // Java: this.hangingPunct = para.hangingPunct;
        hangingPunct: para.hangingPunct,
        // Java: this.latinLnBrk = para.latinLnBrk;
        latinLnBrk: para.latinLnBrk,
        // Java: this.invertMap = para.invertMap;
        invertMap: para.invertMap.clone(),
        // Java: this.indexMap = para.indexMap;
        indexMap: para.indexMap.clone(),
        mVAlignment: para.mVAlignment,
        mtabStops: para.mtabStops.clone(),
      }
    }

    // Java: boolean reset()
    pub fn reset(&self) -> bool {
      // Java: return true;
      true
    }

    // Java: void copyFrom(ParaInfo para)
    pub fn copyFrom(&mut self, para: &ParaInfo) {
      // Java: this.mHAlignment = para.mHAlignment;
      self.mHAlignment = para.mHAlignment;
      // Java: this.mMarginL = para.mMarginL;
      self.mMarginL = para.mMarginL;
      // Java: this.mMarginR = para.mMarginR;
      self.mMarginR = para.mMarginR;
      // Java: this.mIndent = para.mIndent;
      self.mIndent = para.mIndent;
      // Java: this.mAngle = para.mAngle;
      self.mAngle = para.mAngle;
      // Java: this.mSpcBef = para.mSpcBef;
      self.mSpcBef = para.mSpcBef;
      // Java: this.mSpcAft = para.mSpcAft;
      self.mSpcAft = para.mSpcAft;
      // Java: this.mSpcWithinPct = para.mSpcWithinPct;
      self.mSpcWithinPct = para.mSpcWithinPct;
      // Java: this.mSpcWithinPts = para.mSpcWithinPts;
      self.mSpcWithinPts = para.mSpcWithinPts;
      // Java: this.mSpcWithinPctOri = para.mSpcWithinPct;
      self.mSpcWithinPctOri = para.mSpcWithinPct;
      // Java: this.mIsWrapText = para.mIsWrapText;
      self.mIsWrapText = para.mIsWrapText;
      // Java: this.mIsVerticalText = para.mIsVerticalText;
      self.mIsVerticalText = para.mIsVerticalText;
      // Java: this.mIsRightToLeft = para.mIsRightToLeft;
      self.mIsRightToLeft = para.mIsRightToLeft;
      // Java: this.mbulletsInfo = new BulletsInfo(para.mbulletsInfo);
      self.mbulletsInfo = BulletsInfo::new_from(&para.mbulletsInfo);
      // Java: this.start = para.start;
      self.start = para.start;
      // Java: this.mFirstLead = para.mFirstLead;
      self.mFirstLead = para.mFirstLead;
      // Java: this.mRestLead = para.mRestLead;
      self.mRestLead = para.mRestLead;
      // Java: this.mDefTabSize = para.mDefTabSize;
      self.mDefTabSize = para.mDefTabSize;
      // Java: this.eaLnBrk = para.eaLnBrk;
      self.eaLnBrk = para.eaLnBrk;
      // Java: this.hangingPunct = para.hangingPunct;
      self.hangingPunct = para.hangingPunct;
      // Java: this.latinLnBrk = para.latinLnBrk;
      self.latinLnBrk = para.latinLnBrk;
      // Java: this.invertMap = para.invertMap;
      self.invertMap = para.invertMap.clone();
      // Java: this.indexMap = para.indexMap;
      self.indexMap = para.indexMap.clone();
      // Java: if (para.mtabStops != null)
      if let Some(ref tabs) = para.mtabStops {
         // Java: this.mtabStops = new float[para.mtabStops.length];
         // Java: System.arraycopy(para.mtabStops, 0, this.mtabStops, 0, para.mtabStops.length);
         self.mtabStops = Some(tabs.clone());
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.mHAlignment = 0;
      self.mHAlignment = 0;
      // Java: this.mIndent = 0;
      self.mIndent = 0;
      // Java: this.mMarginL = 0;
      self.mMarginL = 0;
      // Java: this.mMarginR = 0;
      self.mMarginR = 0;
      // Java: this.mSpcWithinPct = (double)1.0F;
      self.mSpcWithinPct = 1.0;
      // Java: this.mSpcWithinPts = (double)0.0F;
      self.mSpcWithinPts = 0.0;
      // Java: this.mSpcWithinPctOri = (double)1.0F;
      self.mSpcWithinPctOri = 1.0;
      // Java: this.mAngle = 0;
      self.mAngle = 0;
      // Java: this.mSpcBef = (double)0.0F;
      self.mSpcBef = 0.0;
      // Java: this.mSpcAft = (double)0.0F;
      self.mSpcAft = 0.0;
      // Java: this.mIsWrapText = false;
      self.mIsWrapText = false;
      // Java: this.mIsVerticalText = false;
      self.mIsVerticalText = false;
      // Java: this.mIsRightToLeft = false;
      self.mIsRightToLeft = false;
      // Java: this.mbulletsInfo.clear();
      self.mbulletsInfo.clear();
      // Java: this.start = 0;
      self.start = 0;
      // Java: this.mFirstLead = 0;
      self.mFirstLead = 0;
      // Java: this.mRestLead = 0;
      self.mRestLead = 0;
      // Java: this.mDefTabSize = 0;
      self.mDefTabSize = 0;
      // Java: this.eaLnBrk = true;
      self.eaLnBrk = true;
      // Java: this.hangingPunct = true;
      self.hangingPunct = true;
      // Java: this.latinLnBrk = false;
      self.latinLnBrk = false;
      // Java: this.mtabStops = null;
      self.mtabStops = None;
      // Java: this.invertMap = null;
      self.invertMap = None;
      // Java: this.indexMap = null;
      self.indexMap = None;
    }

}
