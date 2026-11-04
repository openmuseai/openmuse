// java public class UUFootnoteEndnoteId
pub struct UUFootnoteEndnoteId {
    // java private int mFootnoteEndnoteId = 0;
    mFootnoteEndnoteId: i32,
}

impl Default for UUFootnoteEndnoteId {
    fn default() -> Self {
        Self { mFootnoteEndnoteId: 0 }
    }
}

// java public class UUFootnoteEndnoteId
impl UUFootnoteEndnoteId {
    // java public  UUFootnoteEndnoteId()
    pub fn UUFootnoteEndnoteId() -> Self {
      // java (implicit)
      Self::default()
    }

    // java public int getUUFootnoteEndnoteId()
    pub fn getUUFootnoteEndnoteId(&mut self) -> i32 {
      // java return ++this.mFootnoteEndnoteId;
      self.mFootnoteEndnoteId += 1;
      return self.mFootnoteEndnoteId;
    }
}
