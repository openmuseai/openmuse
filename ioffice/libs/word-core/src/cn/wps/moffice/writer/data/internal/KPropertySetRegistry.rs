// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::{empty as PROPERTYSET_EMPTY, PropertySet};
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.List
// Java: import java.util.Map
use std::collections::HashMap;

// java public class KPropertySetRegistry
#[derive(Default)]
pub struct KPropertySetRegistry {
    // java private int mRegistryId;
    mRegistryId: i32,
    // java private List<PropertySet> mList = new ArrayList();
    mList: Option<Vec<PropertySet>>,
    // java private Map<PropertySet, Integer> mMap = new HashMap();
    mMap: Option<HashMap<i32, i32>>,
}

// java public class KPropertySetRegistry
impl KPropertySetRegistry {
    // java public  KPropertySetRegistry(int registerId)
    pub fn KPropertySetRegistry(registerId: i32) -> Self {
      // java this.mRegistryId = registerId;
      let mut this = Self {
        mRegistryId: registerId,
        mList: Some(Vec::new()),
        mMap: Some(HashMap::new()),
      };
      // java this.register(PropertySet.empty);
      this.register(Some(&*PROPERTYSET_EMPTY));
      this
    }

    // java public int register(PropertySet p)
    pub fn register(&mut self, p: Option<&PropertySet>) -> i32 {
      // java if (p == null)
      if p.is_none() {
        // java return 0;
        return 0;
      }
      // java else
      else {
        // Pointer intern is wrong: SAX snapshots live at the same stack address
        // across runs, so later rPr would reuse the first CHPX. Always clone.
        if let Some(ref mut list) = self.mList {
          list.push(p.cloned().unwrap_or_else(|| (*PROPERTYSET_EMPTY).clone()));
          return (list.len() as i32) - 1;
        }
        0
      }
    }

    // java public PropertySet get(int id)
    pub fn get(&self, id: i32) -> PropertySet {
      // java return id >= this.mList.size() ? PropertySet.empty : (PropertySet)this.mList.get(id);
      match self.mList.as_ref() {
        Some(list) if id >= 0 && (id as usize) < list.len() => list[id as usize].clone(),
        _ => (*PROPERTYSET_EMPTY).clone(),
      }
    }

    // java public int getRegistryId()
    pub fn getRegistryId(&self) -> i32 {
      // java return this.mRegistryId;
      return self.mRegistryId;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java if (null != this.mList)
      if self.mList.is_some() {
        // java this.mList.clear();
        if let Some(ref mut mList) = self.mList {
          mList.clear();
        }
        // java this.mList = null;
        self.mList = None;
      }
    }

}
