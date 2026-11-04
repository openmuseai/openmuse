// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.writer.core.TextDocument
// soft: TextDocument.UUNumberingId (TextDocument not in core/mod)
// Java: import java.util.HashMap
// Java: import java.util.Map
use std::collections::HashMap;

/// Soft stand-in for TextDocument.UUNumberingId (Atom id counter).
#[derive(Default, Clone)]
pub struct UUNumberingId {
    // java private int mId;
    mId: i32,
}

impl UUNumberingId {
    // java private UUNumberingId()
    pub fn new() -> Self {
        // java this.mId = 0;
        Self { mId: 0 }
    }

    // java public int getNextId()
    pub fn getNextId(&mut self) -> i32 {
        // java this.writeLockAtom(); // soft: skip Atom lock
        // java return ++this.mId;
        self.mId += 1;
        self.mId
    }

    // java public int getCurrentId()
    pub fn getCurrentId(&self) -> i32 {
        // java return this.mId;
        self.mId
    }
}

// java public final class NumIdRedirection
// rust: ★M-STAGE2 P1-D1 —— Java 侧 `DocumentImporter` 与各 SubDocumentImporter
//   **共享同一实例**；Rust 侧以 `Clone` 近似（构造时传递真实状态，而非凭空新建）。
#[derive(Default, Clone)]
pub struct NumIdRedirection {
    // java private Map<Integer, Integer> mMapNumberingId = new HashMap();
    mMapNumberingId: Option<HashMap<i32, i32>>,
    // java private TextDocument.UUNumberingId mNumberingIdMaker = null;
    mNumberingIdMaker: Option<UUNumberingId>,
}

// java public final class NumIdRedirection
impl NumIdRedirection {
    // java public  NumIdRedirection(TextDocument.UUNumberingId uuNumberingId)
    pub fn NumIdRedirection(uuNumberingId: UUNumberingId) -> Self {
      // java IOAssert.assertNotNull("uuNumberingId should not be null", uuNumberingId);
      let uu_opt = Some(uuNumberingId);
      IOAssert::assertNotNull_msg("uuNumberingId should not be null", &uu_opt);
      // java this.mNumberingIdMaker = uuNumberingId;
      let mut this = Self {
        mMapNumberingId: Some(HashMap::new()),
        mNumberingIdMaker: uu_opt,
      };
      this
    }

    // java public Integer getId(Integer numId)
    pub fn getId(&self, numId: i32) -> Option<i32> {
      // java IOAssert.assertNotNull("numId should not be null", numId);
      let num_opt = Some(numId);
      IOAssert::assertNotNull_msg("numId should not be null", &num_opt);
      // java IOAssert.assertNotNull("mMapNumberingId should not be null", this.mMapNumberingId);
      IOAssert::assertNotNull_msg("mMapNumberingId should not be null", &self.mMapNumberingId);
      // java return (Integer)this.mMapNumberingId.get(numId);
      return self.mMapNumberingId.as_ref().and_then(|m| m.get(&numId).copied());
    }

    // java public int put(Integer numId)
    pub fn put(&mut self, numId: i32) -> i32 {
      // java IOAssert.assertNotNull("numId should not be null", numId);
      let num_opt = Some(numId);
      IOAssert::assertNotNull_msg("numId should not be null", &num_opt);
      // java IOAssert.assertNotNull("mNumberingIdMaker should not be null", this.mNumberingIdMaker);
      IOAssert::assertNotNull_msg("mNumberingIdMaker should not be null", &self.mNumberingIdMaker);
      // java int id = this.mNumberingIdMaker.getNextId();
      let mut id: i32 = self.mNumberingIdMaker.as_mut().unwrap().getNextId();
      // java Integer previous = (Integer)this.mMapNumberingId.put(numId, id);
      let mut previous: Option<i32> = self
        .mMapNumberingId
        .as_mut()
        .unwrap()
        .insert(numId, id);
      let _ = previous;
      // java return id;
      return id;
    }

}
