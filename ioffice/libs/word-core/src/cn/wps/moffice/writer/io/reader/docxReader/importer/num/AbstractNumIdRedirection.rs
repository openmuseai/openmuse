// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::UULsid;
use std::collections::HashMap;

// java public class AbstractNumIdRedirection
pub struct AbstractNumIdRedirection {
    // java private Map<Integer, Integer> mMapAbstractId = new HashMap();
    mMapAbstractId: HashMap<i32, i32>,
    // java private TextDocument.UULsid mUULsid = null;
    mUULsid: *mut UULsid,
}

// java public class AbstractNumIdRedirection
impl AbstractNumIdRedirection {
    // java public AbstractNumIdRedirection(TextDocument.UULsid uuLsid)
    pub fn AbstractNumIdRedirection(uuLsid: *mut UULsid) -> Self {
      // java IOAssert.assertNotNull("uuLsid should not be null", uuLsid);
      let uu_opt = if uuLsid.is_null() { None } else { Some(()) };
      IOAssert::assertNotNull_msg("uuLsid should not be null", &uu_opt);
      // java this.mUULsid = uuLsid;
      Self { mMapAbstractId: HashMap::new(), mUULsid: uuLsid }
    }

    // java public Integer getId(Integer abstractNumId)
    pub fn getId(&self, abstractNumId: Option<i32>) -> Option<i32> {
      // java IOAssert.assertNotNull("abstractNumId should not be null", abstractNumId);
      IOAssert::assertNotNull_msg("abstractNumId should not be null", &abstractNumId);
      // java return (Integer)this.mMapAbstractId.get(abstractNumId);
      abstractNumId.and_then(|id| self.mMapAbstractId.get(&id).copied())
    }

    // java public int put(Integer abstractNumId)
    pub fn put(&mut self, abstractNumId: Option<i32>) -> i32 {
      // java IOAssert.assertNotNull("abstractNumId should not be null", abstractNumId);
      IOAssert::assertNotNull_msg("abstractNumId should not be null", &abstractNumId);
      // java int id = this.mUULsid.getNextId();
      let id = unsafe { (*self.mUULsid).getNextId() };
      // java Integer previous = (Integer)this.mMapAbstractId.put(abstractNumId, id);
      if let Some(abstractNumId) = abstractNumId {
        self.mMapAbstractId.insert(abstractNumId, id);
      }
      // java return id;
      id
    }
}
