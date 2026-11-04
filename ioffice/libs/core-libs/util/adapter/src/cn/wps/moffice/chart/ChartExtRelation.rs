// Java: import java.util.ArrayList

// Java: public class ChartExtRelation
pub struct ChartExtRelation {
    // Java field: ArrayList<ChartRelation> mRels
    pub(crate) mRels: Option<Vec<ChartRelation>>,
    // Java field: String mChartId
    pub(crate) mChartId: String,
}

// Java: public class ChartRelation
pub struct ChartRelation {
    // Java field: String mID
    pub(crate) mID: String,
    // Java field: String mPartName
    pub(crate) mPartName: String,
    // Java field: String mType
    pub(crate) mType: String,
    // Java field: String mCachePath
    pub(crate) mCachePath: String,
}

// Java: public class ChartExtRelation
impl ChartExtRelation {
    // Java: ChartExtRelation()
    pub fn new() -> Self {
      // Java: super();
      ChartExtRelation {
          mRels: None,
          mChartId: String::new(),
      }
    }

    // Java: void setChartId(String id)
    pub fn setChartId(&mut self, _id: &str) {
      // Java: this.mChartId = id;
      self.mChartId = _id.to_string();
    }

    // Java: String getChartId()
    pub fn getChartId(&self) -> &str {
      // Java: return this.mChartId;
      &self.mChartId
    }

    // Java: void addRelation(String id, String partName, String type, String cachePath)
    pub fn addRelation(&mut self, _id: &str, _partName: &str, _type: &str, _cachePath: &str) {
      // Java: if (this.mRels == null)
      if self.mRels.is_none() {
         // Java: this.mRels = new ArrayList();
         self.mRels = Some(Vec::new());
      }
      // Java: this.mRels.add(new ChartRelation(id, partName, type, cachePath));
      if let Some(ref mut rels) = self.mRels {
          rels.push(ChartRelation::new(_id, _partName, _type, _cachePath));
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: if (this.mRels != null)
      if let Some(ref mut rels) = self.mRels {
         // Java: this.mRels.clear();
         rels.clear();
      }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.mRels != null ? this.mRels.size() : 0;
      self.mRels.as_ref().map_or(0, |r| r.len() as i32)
    }

    // Java: ChartRelation get(int i)
    pub fn get(&self, _i: i32) -> Option<&ChartRelation> {
      // Java: return this.mRels != null && i >= 0 && i < this.mRels.size() ? (ChartRelation)this.mRels.get(i) : null;
      self.mRels.as_ref().and_then(|r| r.get(_i as usize))
    }

}

// Java: public class ChartRelation
impl ChartRelation {
    // Java: ChartRelation(String id, String partName, String type, String cachePath)
    pub fn new(_id: &str, _partName: &str, _type: &str, _cachePath: &str) -> Self {
         // Java: super();
         // Java: this.mID = id;
         // Java: this.mPartName = partName;
         // Java: this.mType = type;
         // Java: this.mCachePath = cachePath;
         ChartRelation {
             mID: _id.to_string(),
             mPartName: _partName.to_string(),
             mType: _type.to_string(),
             mCachePath: _cachePath.to_string(),
         }
    }

    // Java: String getID()
    pub fn getID(&self) -> &str {
         // Java: return this.mID;
         &self.mID
    }

    // Java: String getPartName()
    pub fn getPartName(&self) -> &str {
         // Java: return this.mPartName;
         &self.mPartName
    }

    // Java: String getType()
    pub fn getType(&self) -> &str {
         // Java: return this.mType;
         &self.mType
    }

    // Java: String getCachePath()
    pub fn getCachePath(&self) -> &str {
         // Java: return this.mCachePath;
         &self.mCachePath
    }

}
