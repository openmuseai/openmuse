#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.collections.KList

pub trait IBitmapRef {
    fn dispose(&mut self);
}

// Java: public class BitmapRefNode extends KList
pub struct BitmapRefNode {
    // Java field: IBitmapRef bmpRef
    pub(crate) bmpRef: Option<Box<dyn IBitmapRef>>,
}

// Java: public class BitmapRefNode extends KList
impl BitmapRefNode {
    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: super.dispose();
      // Java: if (null != this.bmpRef)
      if self.bmpRef.is_some()
      {
         // Java: this.bmpRef.dispose();
         if let Some(ref mut r) = self.bmpRef {
             r.dispose();
         }
         // Java: this.bmpRef = null;
         self.bmpRef = None;
      }
    }

}
