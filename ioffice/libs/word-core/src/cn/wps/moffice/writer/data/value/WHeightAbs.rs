// Java: import java.lang.Object
use stubs::java::lang::Object::Object;

// java public final class WHeightAbs
#[derive(Default)]
pub struct WHeightAbs {
    // java public final int dyaHeightAbs;
    pub(crate) dyaHeightAbs: i32,
    // java public final boolean fMinHeight;
    pub(crate) fMinHeight: bool,
}

// java public final class WHeightAbs
impl WHeightAbs {
    // java public  WHeightAbs(int val)
    pub fn WHeightAbs(val: i32) -> Self {
      // java this.dyaHeightAbs = val & 32767;
      // java this.fMinHeight = (val & '耀') != 0;
      // '耀' is U+8000; Java promotes char to int as 32768.
      Self { dyaHeightAbs: val & 32767, fMinHeight: (val & ('耀' as i32)) != 0 }
    }

    // java public int getHeightAbs()
    pub fn getHeightAbs(&self) -> i32 {
      // java return this.dyaHeightAbs;
      return self.dyaHeightAbs;
    }

    // java public boolean isMinHeight()
    pub fn isMinHeight(&self) -> bool {
      // java return this.fMinHeight;
      return self.fMinHeight;
    }

    // java public int intValue()
    pub fn intValue(&self) -> i32 {
      // java return (this.fMinHeight ? '耀' : 0) | this.dyaHeightAbs;
      return (if self.fMinHeight { '耀' as i32 } else { 0 }) | self.dyaHeightAbs;
    }

    // java public boolean equals(Object o)
    pub fn equals(&self, o: &dyn Object) -> bool {
      // java if (o == this)
      if o.as_any().downcast_ref::<WHeightAbs>().is_some_and(|w| std::ptr::eq(w, self)) {
        // java return true;
        return true;
      }
      // java else if (o != null && o instanceof WHeightAbs)
      else if o.as_any().downcast_ref::<WHeightAbs>().is_some() {
        // java WHeightAbs w = (WHeightAbs)o;
        let w: &WHeightAbs = o.as_any().downcast_ref::<WHeightAbs>().unwrap();
        // java return this.dyaHeightAbs == w.dyaHeightAbs && this.fMinHeight == w.fMinHeight;
        return self.dyaHeightAbs == w.dyaHeightAbs && self.fMinHeight == w.fMinHeight;
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
      // java return this.dyaHeightAbs + (this.fMinHeight ? 1 : 0);
      return self.dyaHeightAbs + (if self.fMinHeight { 1 } else { 0 });
    }

}
