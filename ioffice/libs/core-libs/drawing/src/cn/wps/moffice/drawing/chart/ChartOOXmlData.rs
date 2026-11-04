#![allow(non_snake_case, dead_code)]

// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.Map
use std::collections::HashMap;

// Java: public class ChartOOXmlData
pub struct ChartOOXmlData {
    // Java field: String mChartPath
    pub(crate) mChartPath: Option<String>,
    // Java field: String mXlsxPath
    pub(crate) mXlsxPath: Option<String>,
    // Java field: String mThemePath
    pub(crate) mThemePath: Option<String>,
    // Java field: String mChartStylePath
    pub(crate) mChartStylePath: Option<String>,
    // Java field: String mColorStylePath
    pub(crate) mColorStylePath: Option<String>,
    // Java field: String mExternalDataRID
    pub(crate) mExternalDataRID: Option<String>,
    // Java field: String mExternalTarget
    pub(crate) mExternalTarget: Option<String>,
    // Java field: boolean mHasTable
    pub(crate) mHasTable: bool,
    // Java field: boolean mIsGVml
    pub(crate) mIsGVml: bool,
    // Java field: Map<String, Integer> mMediaMap
    pub(crate) mMediaMap: HashMap<String, i32>,
}

// Java: public class ChartOOXmlData
impl ChartOOXmlData {
    // Java: ChartOOXmlData()
    pub fn new() -> Self {
      // Java: super();
      ChartOOXmlData {
          mChartPath: None,
          mXlsxPath: None,
          mThemePath: None,
          mChartStylePath: None,
          mColorStylePath: None,
          mExternalDataRID: None,
          mExternalTarget: None,
          mHasTable: false,
          mIsGVml: false,
          mMediaMap: HashMap::new(),
      }
    }

    // Java: void setGVml(boolean isGVml)
    pub fn setGVml(&mut self, isGVml: bool) {
      // Java: this.mIsGVml = isGVml;
      self.mIsGVml = isGVml;
    }

    // Java: boolean isGVml()
    pub fn isGVml(&self) -> bool {
      // Java: return this.mIsGVml;
      self.mIsGVml
    }

    // Java: String chartPath()
    pub fn chartPath(&self) -> String {
      // Java: return this.mChartPath;
      self.mChartPath.clone().unwrap_or_default()
    }

    // Java: void setChartPath(String chartPath)
    pub fn setChartPath(&mut self, chartPath: String) {
      // Java: this.mChartPath = chartPath;
      self.mChartPath = Some(chartPath);
    }

    // Java: String xlsxPath()
    pub fn xlsxPath(&self) -> String {
      // Java: return this.mXlsxPath;
      self.mXlsxPath.clone().unwrap_or_default()
    }

    // Java: void setXlsxPath(String xlsxPath)
    pub fn setXlsxPath(&mut self, xlsxPath: String) {
      // Java: this.mXlsxPath = xlsxPath;
      self.mXlsxPath = Some(xlsxPath);
    }

    // Java: String themePath()
    pub fn themePath(&self) -> String {
      // Java: return this.mThemePath;
      self.mThemePath.clone().unwrap_or_default()
    }

    // Java: void setThemePath(String themePath)
    pub fn setThemePath(&mut self, themePath: String) {
      // Java: this.mThemePath = themePath;
      self.mThemePath = Some(themePath);
    }

    // Java: String chartStylePath()
    pub fn chartStylePath(&self) -> String {
      // Java: return this.mChartStylePath;
      self.mChartStylePath.clone().unwrap_or_default()
    }

    // Java: void setChartStylePath(String chartStylePath)
    pub fn setChartStylePath(&mut self, chartStylePath: String) {
      // Java: this.mChartStylePath = chartStylePath;
      self.mChartStylePath = Some(chartStylePath);
    }

    // Java: String colorStylePath()
    pub fn colorStylePath(&self) -> String {
      // Java: return this.mColorStylePath;
      self.mColorStylePath.clone().unwrap_or_default()
    }

    // Java: void setColorStylePath(String colorStylePath)
    pub fn setColorStylePath(&mut self, colorStylePath: String) {
      // Java: this.mColorStylePath = colorStylePath;
      self.mColorStylePath = Some(colorStylePath);
    }

    // Java: void setExternalTarget(String target)
    pub fn setExternalTarget(&mut self, target: String) {
      // Java: this.mExternalTarget = target;
      self.mExternalTarget = Some(target);
    }

    // Java: String externalTarget()
    pub fn externalTarget(&self) -> String {
      // Java: return this.mExternalTarget;
      self.mExternalTarget.clone().unwrap_or_default()
    }

    // Java: void addMeidaItem(String rID, int mediaID)
    pub fn addMeidaItem(&mut self, rID: String, mediaID: i32) {
      // Java: this.mMediaMap.put(rID, mediaID);
      self.mMediaMap.insert(rID, mediaID);
    }

    // Java: int mediaID(String rID)
    pub fn mediaID(&self, rID: &str) -> i32 {
      // Java: Integer rid = (Integer)this.mMediaMap.get(rID);
      let rid = self.mMediaMap.get(rID);
      // Java: return null != rid ? rid : -1;
      match rid {
          Some(&v) => v,
          None => -1,
      }
    }

    // Java: ArrayList<String> getAllRids()
    pub fn getAllRids(&self) -> Vec<String> {
      // Java: ArrayList<String> rids = new ArrayList();
      let mut rids = Vec::new();
      // Java: rids.addAll(this.mMediaMap.keySet());
      for key in self.mMediaMap.keys() {
          rids.push(key.clone());
      }
      // Java: return rids;
      rids
    }

    // Java: void setExternalDataRID(String externalDataRID)
    pub fn setExternalDataRID(&mut self, externalDataRID: String) {
      // Java: this.mExternalDataRID = externalDataRID;
      self.mExternalDataRID = Some(externalDataRID);
    }

    // Java: String externalDataRID()
    pub fn externalDataRID(&self) -> String {
      // Java: return this.mExternalDataRID;
      self.mExternalDataRID.clone().unwrap_or_default()
    }

    // Java: void setHasTable(boolean has)
    pub fn setHasTable(&mut self, has: bool) {
      // Java: this.mHasTable = has;
      self.mHasTable = has;
    }

    // Java: boolean hasTable()
    pub fn hasTable(&self) -> bool {
      // Java: return this.mHasTable;
      self.mHasTable
    }

    // Java: public Map<String, Integer> getAllMedia()
    pub fn getAllMedia(&self) -> &HashMap<String, i32> {
      // Java: return this.mMediaMap;
      &self.mMediaMap
    }

}
