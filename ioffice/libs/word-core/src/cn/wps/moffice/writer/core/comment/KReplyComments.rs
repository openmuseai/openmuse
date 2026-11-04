// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.core.comment.KComments
use crate::cn::wps::moffice::writer::core::comment::KComments::*;

// java public class KReplyComments
#[derive(Default)]
pub struct KReplyComments {
    // java private ArrayList<KReplyCmtChain> mReplyCmtChain = new ArrayList();
    mReplyCmtChain: Option<Vec<KReplyCmtChain>>,
}

// java public static class KReplyCmtChain extends KComments
#[derive(Default)]
pub struct KReplyCmtChain {
    // java KComments super_
    pub(crate) super_: KComments,
}

// java public class KReplyComments
impl KReplyComments {
    // java public KReplyCmtChain item(int index)
    pub fn item(&self, index: i32) -> KReplyCmtChain {
      // java return (KReplyCmtChain)this.mReplyCmtChain.get(index);
      return (self.mReplyCmtChain.get(index)).downcast_ref::<KReplyCmtChain>().unwrap();
    }

    // java public void add(KReplyCmtChain chain)
    pub fn add(&mut self, chain: &KReplyCmtChain) {
      // java this.mReplyCmtChain.add(chain);
      self.mReplyCmtChain.add(chain);
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mReplyCmtChain.clear();
      self.mReplyCmtChain.clear();
    }

}

// java public static class KReplyCmtChain extends KComments
impl KReplyCmtChain {
}

impl std::ops::Deref for KReplyCmtChain {
    type Target = KComments;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for KReplyCmtChain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
