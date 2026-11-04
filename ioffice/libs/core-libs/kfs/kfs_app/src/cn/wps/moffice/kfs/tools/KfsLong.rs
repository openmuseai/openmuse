// Java: public class KfsLong 
#[derive(Default)]
pub struct KfsLong {
    // Java field:    private final long value;
    value: i64,
}

// Java: public class KfsLong 
impl KfsLong {
    // Java: public KfsLong(long val) 
    pub fn new(val: i64) -> KfsLong {
      Self { value: val }
    }

    // Java: public long getValue() 
    pub fn getValue(&self) -> i64 {
      // Java: return this.value;
      return self.value;
    }

}
