// java public class FilterArgs
#[derive(Default, Clone, Debug)]
pub struct FilterArgs {
    // java public int start;
    pub start: i32,
    // java public int end;
    pub end: i32,
    // java public int next;
    pub next: i32,
}

// java public class FilterArgs
impl FilterArgs {
    // java public void init(int start, int end)
    pub fn init(&mut self, start: i32, end: i32) {
      // java this.start = start;
      self.start = start;
      // java this.end = end;
      self.end = end;
      // java this.next = end;
      self.next = end;
    }

}