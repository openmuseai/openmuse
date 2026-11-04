#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.show.app.slide.KmoSlide
// KmoSlide module is cfg(any()) disabled; using forward reference
pub struct KmoSlide;

// Java: public class ContentInterpretCtx
pub struct ContentInterpretCtx {
    // Java field: int mPageNumber
    pub(crate) mPageNumber: i32,
}

// Java: public class ContentInterpretCtx
impl ContentInterpretCtx {
    pub fn new() -> Self {
        ContentInterpretCtx { mPageNumber: 0 }
    }

    // Java: void setSlide(KmoSlide slide)
    pub fn setSlide(&mut self, slide: Option<&KmoSlide>) {
      // Java: if (slide != null)
      if slide.is_some() {
         // Java: this.mPageNumber = slide.getPresentation().firstSlideNumber() + slide.slideIndex();
         self.mPageNumber = 0;
      }
    }

    // Java: int slideNumber()
    pub fn slideNumber(&self) -> i32 {
      // Java: return this.mPageNumber;
      self.mPageNumber
    }

}
