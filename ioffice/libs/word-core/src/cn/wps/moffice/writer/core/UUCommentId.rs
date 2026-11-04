// java public class UUCommentId
#[derive(Default)]
pub struct UUCommentId {
    // java private int mNextCommentId = 1;
    mNextCommentId: i32,
}

// java public class UUCommentId
impl UUCommentId {
    // java   UUCommentId()
    pub fn UUCommentId() -> Self {
      Self { mNextCommentId: Default::default() }
    }

    // java public int getUUCommentId()
    pub fn getUUCommentId(&mut self) -> i32 {
      // java return ++this.mNextCommentId;
      return { let _tmp = self.mNextCommentId; self.mNextCommentId += 1; _tmp };
    }

}
