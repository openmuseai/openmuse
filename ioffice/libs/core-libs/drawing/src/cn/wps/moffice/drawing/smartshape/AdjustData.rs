#![allow(non_snake_case, non_upper_case_globals, dead_code)]

// Java: public class AdjustData
pub struct AdjustData {
    // Java field: int index
    pub(crate) index: i32,
    // Java field: float value
    pub(crate) value: f32,
    // Java field: float originValue
    pub(crate) originValue: f32,
    // Java field: int minValueIndex
    pub(crate) minValueIndex: i32,
    // Java field: float minValue
    pub(crate) minValue: f32,
    // Java field: int maxValueIndex
    pub(crate) maxValueIndex: i32,
    // Java field: float maxValue
    pub(crate) maxValue: f32,
    // Java field: float coordinateMinValue
    pub(crate) coordinateMinValue: f32,
    // Java field: float coordinateMaxValue
    pub(crate) coordinateMaxValue: f32,
    // Java field: float coordinateOriginValue
    pub(crate) coordinateOriginValue: f32,
}

// Java: public class AdjustData
impl AdjustData {
    // Java: AdjustData()
    pub fn new() -> Self {
      // Java: super();
      AdjustData {
          index: 0,
          value: 0.0_f32,
          originValue: 0.0_f32,
          minValueIndex: 0,
          minValue: 0.0_f32,
          maxValueIndex: 0,
          maxValue: 0.0_f32,
          coordinateMinValue: 0.0_f32,
          coordinateMaxValue: 0.0_f32,
          coordinateOriginValue: 0.0_f32,
      }
    }

}
