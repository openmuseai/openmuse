#![allow(non_snake_case, non_upper_case_globals, dead_code)]

// Java field: int DgVML_Max
pub(crate) const DgVML_Max: i32 = 21600;
// Java field: int DgVML_Mid
pub(crate) const DgVML_Mid: i32 = 10800;
// Java field: int DgVML_Quad
pub(crate) const DgVML_Quad: i32 = 5400;
// Java field: int DgVML_Six
pub(crate) const DgVML_Six: i32 = 3600;
// Java field: int DgVML_Eight
pub(crate) const DgVML_Eight: i32 = 2700;
// Java field: int DgXML_Max
pub(crate) const DgXML_Max: i32 = 100000;
// Java field: int DgXML_Mid
pub(crate) const DgXML_Mid: i32 = 50000;
// Java field: int DgXML_Quad
pub(crate) const DgXML_Quad: i32 = 25000;
// Java field: int GSpace_Max
pub(crate) const GSpace_Max: i32 = 100000;
// Java field: int GSpace_Mid
pub(crate) const GSpace_Mid: i32 = 50000;
// Java field: int GSpace_Quad
pub(crate) const GSpace_Quad: i32 = 25000;
// Java field: int SmileyFace_yMin_VML
pub(crate) const SmileyFace_yMin_VML: i32 = 15510;
// Java field: int SmileyFace_yMax_VML
pub(crate) const SmileyFace_yMax_VML: i32 = 17520;
// Java field: int SmileyFace_yRange_VML
pub(crate) const SmileyFace_yRange_VML: i32 = 2010;
// Java field: int SmileyFace_yMid_VML
pub(crate) const SmileyFace_yMid_VML: i32 = 16515;
// Java field: int SmileyFace_yMin
pub(crate) const SmileyFace_yMin: i32 = -4653;
// Java field: int SmileyFace_yMax
pub(crate) const SmileyFace_yMax: i32 = 4653;
// Java field: int SmileyFace_yRange
pub(crate) const SmileyFace_yRange: i32 = 9306;
// Java field: int SmileyFace_yMid
pub(crate) const SmileyFace_yMid: i32 = 0;
// Java field: int FolderdCorner_yMin
pub(crate) const FolderdCorner_yMin: i32 = 0;
// Java field: int FolderdCorner_yMax
pub(crate) const FolderdCorner_yMax: i32 = 50000;
// Java field: int UpArrowAdj2
pub(crate) const UpArrowAdj2: i32 = 4100;
// Java field: int LeftUpArrowAdj1
pub(crate) const LeftUpArrowAdj1: i32 = 18500;
// Java field: int LeftUpArrowAdj2
pub(crate) const LeftUpArrowAdj2: i32 = 9200;
// Java field: int LeftUpArrowAdj3
pub(crate) const LeftUpArrowAdj3: i32 = 6170;
// Java field: int CurvedArrowAdj1
pub(crate) const CurvedArrowAdj1: i32 = 19000;
// Java field: int CurvedArrowAdj2
pub(crate) const CurvedArrowAdj2: i32 = 12340;
// Java field: int CurvedArrowAdj3
pub(crate) const CurvedArrowAdj3: i32 = 14380;
// Java field: int BentUpArrow_Range
pub(crate) const BentUpArrow_Range: i32 = 4646;
// Java field: int BentUpArrow_xDefault
pub(crate) const BentUpArrow_xDefault: i32 = 9257;
// Java field: int BentArrow_xMin
pub(crate) const BentArrow_xMin: i32 = 12427;
// Java field: int BentArrow_yMax
pub(crate) const BentArrow_yMax: i32 = 6079;
// Java field: int Arc_Default_VML
pub(crate) const Arc_Default_VML: i32 = 11796480;
// Java field: int Arc_Default_DML
pub(crate) const Arc_Default_DML: i32 = 10800000;
// Java field: int UNITS_FOR_VML
pub(crate) const UNITS_FOR_VML: i32 = 65536;
// Java field: int UNITS_FOR_DML
pub(crate) const UNITS_FOR_DML: i32 = 60000;
// Java field: int ArrowCalloutAdj4
pub(crate) const ArrowCalloutAdj4: i32 = 14400;

// Java: public class AdjVal
pub struct AdjVal {
}

// Java: public class AdjVal
impl AdjVal {
    // Java: AdjVal()
    pub fn new() -> Self {
      // Java: super();
      AdjVal {}
    }

    // Java: static int adj2GSpace(int lAdj)
    pub fn adj2GSpace(lAdj: i32) -> i32 {
      // Java: return (int)Math.round((double)100000.0F * (double)lAdj / (double)21600.0F);
      (100000.0_f64 * lAdj as f64 / 21600.0_f64).round() as i32
    }

    // Java: static int adj2DgVml(int lAdj)
    pub fn adj2DgVml(lAdj: i32) -> i32 {
      // Java: return (int)Math.round((double)21600.0F * (double)lAdj / (double)100000.0F);
      (21600.0_f64 * lAdj as f64 / 100000.0_f64).round() as i32
    }

    // Java: static int angle2DgVml(int angle)
    pub fn angle2DgVml(angle: i32) -> i32 {
      // Java: return (int)((long)angle * 65536L / 60000L);
      (angle as i64 * 65536_i64 / 60000_i64) as i32
    }

    // Java: static int angle2GSpace(int angle)
    pub fn angle2GSpace(angle: i32) -> i32 {
      // Java: return (int)((long)angle * 60000L / 65536L);
      (angle as i64 * 60000_i64 / 65536_i64) as i32
    }

    // Java: static int ratio2DgVml(int lRatio, float dRight, int threshold)
    pub fn ratio2DgVml(lRatio: i32, dRight: f32, threshold: i32) -> i32 {
      // Java: int lCoord = (int)((float)threshold * dRight * XML2Percent(lRatio));
      let lCoord = (threshold as f32 * dRight * Self::XML2Percent(lRatio)) as i32;
      // Java: return lCoord < threshold ? lCoord : threshold;
      if lCoord < threshold { lCoord } else { threshold }
    }

    // Java: static int dgVml2Ratio(int lCoord, float dRight, int threshold)
    pub fn dgVml2Ratio(lCoord: i32, dRight: f32, threshold: i32) -> i32 {
      // Java: return percent2XML((float)lCoord / ((float)threshold * dRight));
      Self::percent2XML(lCoord as f32 / (threshold as f32 * dRight))
    }

    // Java: static int percent2XML(float per)
    pub fn percent2XML(per: f32) -> i32 {
      // Java: return (int)(per * 100000.0F);
      (per * 100000.0_f32) as i32
    }

    // Java: static float XML2Percent(int raw)
    pub fn XML2Percent(raw: i32) -> f32 {
      // Java: return (float)raw / 100000.0F;
      raw as f32 / 100000.0_f32
    }

}
