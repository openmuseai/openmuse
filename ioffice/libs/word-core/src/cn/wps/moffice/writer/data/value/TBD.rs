// Java: import java.lang.Object
use stubs::java::lang::Object::Object;

// java public class TBD
#[derive(Default, Clone)]
pub struct TBD {
    // java public int tabJustify;
    pub tabJustify: i32,
    // java public int frontCharType;
    pub frontCharType: i32,
}

// java public class TBD
impl TBD {
    // java public  TBD()
    pub fn TBD() -> Self {
      Self { tabJustify: Default::default(), frontCharType: Default::default() }
    }

    // java public  TBD(int tabJustify, int frontCharType)
    pub fn TBD__int_int(tabJustify: i32, frontCharType: i32) -> Self {
      // java this.tabJustify = tabJustify;
      Self { tabJustify: tabJustify, frontCharType: frontCharType }
      // java this.frontCharType = frontCharType;
    }

    // java public void setTabJustify(int tabJustify)
    pub fn setTabJustify(&mut self, tabJustify: i32) {
      // java this.tabJustify = tabJustify;
      self.tabJustify = tabJustify;
    }

    // java public void setFrontCharType(int frontCharType)
    pub fn setFrontCharType(&mut self, frontCharType: i32) {
      // java this.frontCharType = frontCharType;
      self.frontCharType = frontCharType;
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
      // java return (this.tabJustify << 7) + this.frontCharType;
      return (self.tabJustify << 7) + self.frontCharType;
    }

    // java public boolean equals(Object o)
    pub fn equals(&self, o: &dyn Object) -> bool {
      // java if (this == o)
      if o.as_any().downcast_ref::<TBD>().is_some_and(|t| std::ptr::eq(t, self)) {
        // java return true;
        return true;
      }
      // java else if (!(o instanceof TBD))
      else if o.as_any().downcast_ref::<TBD>().is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java TBD t = (TBD)o;
        let t: &TBD = o.as_any().downcast_ref::<TBD>().unwrap();
        // java return this.tabJustify == t.tabJustify && this.frontCharType == t.frontCharType;
        return self.tabJustify == t.tabJustify && self.frontCharType == t.frontCharType;
      }
    }

}
