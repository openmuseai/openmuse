// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import cn.wps.moffice.writer.core.comment.KComment
use crate::cn::wps::moffice::writer::core::comment::KComment::*;

// java public class KComments
#[derive(Default)]
pub struct KComments {
    // java List<KComment> comments = new ArrayList();
    pub(crate) comments: Option<Vec<KComment>>,
}

// java public class KComments
impl KComments {
    // java public KComment add(Document document, int ref)
    pub fn add(&mut self, document: &dyn Document, r#ref: i32) -> KComment {
      // java KComment comment = new KComment(document, ref);
      let mut comment: KComment = KComment::new(document, r#ref);
      // java int index = this.lowerBoundsIndex(ref);
      let mut index: i32 = self.lowerBoundsIndex(r#ref);
      // java this.comments.add(index, comment);
      self.comments.add(index, comment);
      // java return comment;
      return comment;
    }

    // java private int lowerBoundsIndex(int ref)
    pub fn lowerBoundsIndex(&mut self, r#ref: i32) -> i32 {
      // java int count = this.count();
      let mut count: i32 = self.count();
      let mut i: i32 = 0;
      // java for(int i = 0; i < count; ++i)
      while i < count {
        // java KComment com = this.item(i);
        let mut com: KComment = self.item(i);
        // java if (com.getCommentRefCp() >= ref)
        if com.getCommentRefCp() >= r#ref {
          // java return i;
          return i;
        }
      }
      // java return count;
      return count;
    }

    // java public int count()
    pub fn count(&self) -> i32 {
      // java return this.comments.size();
      return (self.comments.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
    }

    // java public KComment item(int index)
    pub fn item(&self, index: i32) -> KComment {
      // java return (KComment)this.comments.get(index);
      return (self.comments.get(index)).downcast_ref::<KComment>().unwrap();
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.comments.clear();
      self.comments.clear();
    }

    // java public KComment findComment(int ref)
    pub fn findComment(&mut self, r#ref: i32) -> Option<KComment> {
      // java int count = this.count();
      let mut count: i32 = self.count();
      let mut i: i32 = 0;
      // java for(int i = 0; i < count; ++i)
      while i < count {
        // java KComment com = this.item(i);
        let mut com: KComment = self.item(i);
        // java if (ref == com.commentRef.getCP())
        if r#ref == com.commentRef.getCP() {
          // java return com;
          return com;
        }
      }
      // java return null;
      return None;
    }

}
