// Java: import java.util.HashMap
// Java: import java.util.Map

use std::collections::HashMap;

// Java field: int START_ID
static START_ID: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();
// Java field: int IS_HAS_TITLE
pub(crate) const IS_HAS_TITLE: i32 = 0;
// Java field: int CHART_TITLE
pub(crate) const CHART_TITLE: i32 = 1;
// Java field: int IS_INCLUDE_INLAYOUT
pub(crate) const IS_INCLUDE_INLAYOUT: i32 = 2;
// Java field: int IS_HAS_LEGEND
pub(crate) const IS_HAS_LEGEND: i32 = 3;
// Java field: int LEGEND_POS
pub(crate) const LEGEND_POS: i32 = 4;
// Java field: int IS_LEGEND_INCLUDE_INLAYOUT
pub(crate) const IS_LEGEND_INCLUDE_INLAYOUT: i32 = 5;
// Java field: int ERRORBAR_TYPE
pub(crate) const ERRORBAR_TYPE: i32 = 6;
// Java field: int TRENDLINE_TYPE
pub(crate) const TRENDLINE_TYPE: i32 = 7;
// Java field: int SERIES_INDEX
pub(crate) const SERIES_INDEX: i32 = 8;
// Java field: int TRENDLINE_OP
pub(crate) const TRENDLINE_OP: i32 = 9;
// Java field: int IS_HAS_AXIS
pub(crate) const IS_HAS_AXIS: i32 = 10;
// Java field: int IS_HAS_CATE_AXIS
pub(crate) const IS_HAS_CATE_AXIS: i32 = 11;
// Java field: int IS_HAS_VALUE_AXIS
pub(crate) const IS_HAS_VALUE_AXIS: i32 = 12;
// Java field: int AXIS_CROSSES
pub(crate) const AXIS_CROSSES: i32 = 13;
// Java field: int AXIS_CROSSES_AT
pub(crate) const AXIS_CROSSES_AT: i32 = 14;
// Java field: int AXIS_FONT_HEIGHT
pub(crate) const AXIS_FONT_HEIGHT: i32 = 15;
// Java field: int IS_HAS_DATATABLE
pub(crate) const IS_HAS_DATATABLE: i32 = 16;
// Java field: int DATALABEL_DATA
pub(crate) const DATALABEL_DATA: i32 = 17;
// Java field: int IS_SHOW_X_MAJOR_GRIDLINE
pub(crate) const IS_SHOW_X_MAJOR_GRIDLINE: i32 = 18;
// Java field: int IS_SHOW_X_MINOR_GRIDLINE
pub(crate) const IS_SHOW_X_MINOR_GRIDLINE: i32 = 19;
// Java field: int IS_SHOW_Y_MAJOR_GRIDLINE
pub(crate) const IS_SHOW_Y_MAJOR_GRIDLINE: i32 = 20;
// Java field: int IS_SHOW_Y_MINOR_GRIDLINE
pub(crate) const IS_SHOW_Y_MINOR_GRIDLINE: i32 = 21;
// Java field: int IS_SHOW_X_AXIS_TITLE
pub(crate) const IS_SHOW_X_AXIS_TITLE: i32 = 22;
// Java field: int IS_SHOW_Y_AXIS_TITLE
pub(crate) const IS_SHOW_Y_AXIS_TITLE: i32 = 23;
// Java field: int IS_SHOW_DROPLINE
pub(crate) const IS_SHOW_DROPLINE: i32 = 24;
// Java field: int IS_HAS_X_AXIS_LINE
pub(crate) const IS_HAS_X_AXIS_LINE: i32 = 25;
// Java field: int IS_HAS_Y_AXIS_LINE
pub(crate) const IS_HAS_Y_AXIS_LINE: i32 = 26;
// Java field: int IS_PLOTAREA_AUTO
pub(crate) const IS_PLOTAREA_AUTO: i32 = 27;
// Java field: int IS_HAS_ERRORBARS
pub(crate) const IS_HAS_ERRORBARS: i32 = 28;
// Java field: int IS_SHOW_AXIS_LABEL
pub(crate) const IS_SHOW_AXIS_LABEL: i32 = 29;
// Java field: int MAJOR_TICK_MARK
pub(crate) const MAJOR_TICK_MARK: i32 = 30;
// Java field: int MINOR_TICK_MARK
pub(crate) const MINOR_TICK_MARK: i32 = 31;
// Java field: int SPLIT_VALUE
pub(crate) const SPLIT_VALUE: i32 = 32;
// Java field: int OVER_LAP
pub(crate) const OVER_LAP: i32 = 33;
// Java field: int GAP_WIDTH
pub(crate) const GAP_WIDTH: i32 = 34;
// Java field: int INSERT_CHART
pub(crate) const INSERT_CHART: i32 = 35;
// Java field: int DOUGHNUT_HOLE_SIZE
pub(crate) const DOUGHNUT_HOLE_SIZE: i32 = 36;
// Java field: byte ADD
pub(crate) const ADD: i8 = 0;
// Java field: byte DEL
pub(crate) const DEL: i8 = 1;
// Java field: byte EDIT
pub(crate) const EDIT: i8 = 2;

// Java: public class ChartOptionsData
pub struct ChartOptionsData {
    // Java field: Map<Integer, Object> modifiedMap
    pub(crate) modifiedMap: Option<HashMap<i32, Box<dyn std::any::Any>>>,
}

// Java: public class TrendlineOpData
pub struct TrendlineOpData {
    // Java field: byte opType
    pub(crate) opType: i8,
    // Java field: int seriesIndex
    pub(crate) seriesIndex: i32,
    // Java field: int trendLineIndex
    pub(crate) trendLineIndex: i32,
    // Java field: int trendLineType
    pub(crate) trendLineType: i32,
    // Java field: String name
    pub(crate) name: Option<String>,
    // Java field: int period
    pub(crate) period: i32,
    // Java field: int order
    pub(crate) order: i32,
}

// Java: public class TrendlineOp
pub struct TrendlineOp {
}

// Java: public class DataLabelOptions
pub struct DataLabelOptions {
    // Java field: boolean showDataLabel
    pub(crate) showDataLabel: bool,
    // Java field: boolean showSeriesName
    pub(crate) showSeriesName: bool,
    // Java field: boolean showCategory
    pub(crate) showCategory: bool,
    // Java field: boolean showValue
    pub(crate) showValue: bool,
    // Java field: boolean showPercent
    pub(crate) showPercent: bool,
}

// Java: public class ChartOptionsData
impl ChartOptionsData {
    // Java: ChartOptionsData()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.modifiedMap = new HashMap();
      ChartOptionsData {
          modifiedMap: Some(HashMap::new()),
      }
    }

    // Java: ChartOptionsData(ChartOptionsData opts)
    pub fn new_from(_opts: &ChartOptionsData) -> Self {
      // Java: super();
      // Java: this.modifiedMap = new HashMap();
      // Java: this.modifiedMap.putAll(opts.getPropertyMap());
      ChartOptionsData {
          modifiedMap: Some(HashMap::new()),
      }
    }

    // Java: void onDestroy()
    pub fn onDestroy(&mut self) {
      // Java: if (this.modifiedMap != null)
      if let Some(ref mut map) = self.modifiedMap {
         // Java: this.modifiedMap.clear();
         map.clear();
         // Java: this.modifiedMap = null;
      }
      self.modifiedMap = None;
    }

    // Java: void addProperty(int key, Object value)
    pub fn addProperty(&mut self, _key: i32, _value: Box<dyn std::any::Any>) {
      // Java: this.modifiedMap.put(key, value);
      if let Some(ref mut map) = self.modifiedMap {
          map.insert(_key, _value);
      }
    }

    // Java: void removeProperty(int key)
    pub fn removeProperty(&mut self, _key: i32) {
      // Java: if (this.modifiedMap.containsKey(key))
      if let Some(ref mut map) = self.modifiedMap {
         // Java: this.modifiedMap.remove(key);
         map.remove(&_key);
      }
    }

    // Java: boolean containsKey(int key)
    pub fn containsKey(&self, _key: i32) -> bool {
      // Java: return this.modifiedMap.containsKey(key);
      self.modifiedMap.as_ref().map_or(false, |m| m.contains_key(&_key))
    }

    // Java: Object getProperty(int key)
    pub fn getProperty(&self, _key: i32) -> Option<&Box<dyn std::any::Any>> {
      // Java: return this.modifiedMap.get(key);
      self.modifiedMap.as_ref().and_then(|m| m.get(&_key))
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.modifiedMap.size();
      self.modifiedMap.as_ref().map_or(0, |m| m.len() as i32)
    }

    // Java: public Map<Integer, Object> getPropertyMap()
    pub fn getPropertyMap(&self) -> Option<&HashMap<i32, Box<dyn std::any::Any>>> {
      // Java: return this.modifiedMap;
      self.modifiedMap.as_ref()
    }

    // Java: void merge(ChartOptionsData opts)
    pub fn merge(&mut self, _opts: &ChartOptionsData) {
      // Java: this.modifiedMap.putAll(opts.modifiedMap);
    }

}

// Java: public class TrendlineOpData
impl TrendlineOpData {
    // Java: TrendlineOpData()
    pub fn new() -> Self {
         // Java: super();
         TrendlineOpData {
             opType: 0,
             seriesIndex: 0,
             trendLineIndex: 0,
             trendLineType: 0,
             name: None,
             period: 0,
             order: 0,
         }
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, _o: &dyn std::any::Any) -> bool {
         // Java: if (o == this)
         {
            // Java: return true;
         }
         // Java: else if (!(o instanceof TrendlineOpData))
         {
            // Java: return false;
         }
         // Java: else
         {
            // Java: TrendlineOpData obj = (TrendlineOpData)o;
            // Java: return this.opType == obj.opType && this.seriesIndex == obj.seriesIndex && this.trendLineIndex == obj.trendLineIndex && this.trendLineType == obj.trendLineType && (this.name == obj.name || this.name != null && this.name.equals(obj.name) || obj.name != null && obj.name.equals(this.name)) && this.period == obj.period && this.order == obj.order;
         }
         todo!()
    }

}

// Java: public class TrendlineOp
impl TrendlineOp {
    // Java: TrendlineOp()
    pub fn new() -> Self {
            // Java: super();
            TrendlineOp {}
    }

}

// Java: public class DataLabelOptions
impl DataLabelOptions {
    // Java: DataLabelOptions()
    pub fn new() -> Self {
         // Java: super();
         DataLabelOptions {
             showDataLabel: false,
             showSeriesName: false,
             showCategory: false,
             showValue: false,
             showPercent: false,
         }
    }

    // Java: void set(DataLabelOptions src)
    pub fn set(&mut self, _src: &DataLabelOptions) {
         // Java: this.showDataLabel = src.showDataLabel;
         self.showDataLabel = _src.showDataLabel;
         // Java: this.showCategory = src.showCategory;
         self.showCategory = _src.showCategory;
         // Java: this.showSeriesName = src.showSeriesName;
         self.showSeriesName = _src.showSeriesName;
         // Java: this.showValue = src.showValue;
         self.showValue = _src.showValue;
         // Java: this.showPercent = src.showPercent;
         self.showPercent = _src.showPercent;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: int result = 0;
         let mut result: i32 = 0;
         // Java: result += this.showCategory ? 1 : 0;
         result += if self.showCategory { 1 } else { 0 };
         // Java: result += this.showDataLabel ? 2 : 0;
         result += if self.showDataLabel { 2 } else { 0 };
         // Java: result += this.showPercent ? 4 : 0;
         result += if self.showPercent { 4 } else { 0 };
         // Java: result += this.showSeriesName ? 8 : 0;
         result += if self.showSeriesName { 8 } else { 0 };
         // Java: result += this.showValue ? 16 : 0;
         result += if self.showValue { 16 } else { 0 };
         // Java: return result;
         result
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, _o: &dyn std::any::Any) -> bool {
         // Java: if (o == this)
         {
            // Java: return true;
         }
         // Java: else if (!(o instanceof DataLabelOptions))
         {
            // Java: return false;
         }
         // Java: else
         {
            // Java: DataLabelOptions obj = (DataLabelOptions)o;
            // Java: return this.showDataLabel == obj.showDataLabel && this.showCategory == obj.showCategory && this.showSeriesName == obj.showSeriesName && this.showValue == obj.showValue && this.showPercent == obj.showPercent;
         }
         todo!()
    }

}
