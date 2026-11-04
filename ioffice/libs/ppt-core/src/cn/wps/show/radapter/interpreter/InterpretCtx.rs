// Java: import cn.wps.drawing.data.style.ColorMapping
use crate::cn::wps::drawing::data::style::ColorMapping::ColorMapping;
// Java: import cn.wps.drawing.data.style.ColorScheme
use crate::cn::wps::drawing::data::style::ColorScheme::ColorScheme;
// Java: import cn.wps.show.app.slide.ISlide
use crate::cn::wps::show::app::slide::ISlide::ISlide;
// Java: import cn.wps.show.app.util.ColorInterpretCtx
use crate::cn::wps::show::app::util::ColorInterpretCtx::ColorInterpretCtx;
// Java: import cn.wps.show.radapter.interpreter.TableInterpreCtx
use crate::cn::wps::show::radapter::interpreter::TableInterpreCtx::TableInterpreCtx;
// Java: import cn.wps.show.radapter.interpreter.ThemeInterpretCtx
use crate::cn::wps::show::radapter::interpreter::ThemeInterpretCtx::ThemeInterpretCtx;
// Java: import cn.wps.show.radapter.interpreter.FilterInterpretCtx
use crate::cn::wps::show::radapter::interpreter::FilterInterpretCtx::FilterInterpretCtx;
// Java: import cn.wps.show.radapter.interpreter.ContentInterpretCtx
use crate::cn::wps::show::radapter::interpreter::ContentInterpretCtx::ContentInterpretCtx;

// java public class InterpretCtx
#[derive(Default)]
pub struct InterpretCtx {
    // java private ColorInterpretCtx mColorInterpretCtx = null;
    mColorInterpretCtx: ColorInterpretCtx,
    // java private TableInterpreCtx mTableInterpretCtx = null;
    mTableInterpretCtx: TableInterpreCtx,
    // java private ThemeInterpretCtx mThemeInterpretCtx = null;
    mThemeInterpretCtx: ThemeInterpretCtx,
    // java private FilterInterpretCtx mFilterInterpretCtx = null;
    mFilterInterpretCtx: FilterInterpretCtx,
    // java private ContentInterpretCtx mContentInterpretCtr = null;
    mContentInterpretCtr: ContentInterpretCtx,
}

// java public class InterpretCtx
impl InterpretCtx {
    // java public  InterpretCtx()
    pub fn InterpretCtx() -> Self {
      // java this.mColorInterpretCtx = new ColorInterpretCtx((ColorScheme)null, (ColorMapping)null);
      Self { mColorInterpretCtx: ColorInterpretCtx::new(None, None), mTableInterpretCtx: TableInterpreCtx::new(), mThemeInterpretCtx: ThemeInterpretCtx::new(), mFilterInterpretCtx: FilterInterpretCtx::new(), mContentInterpretCtr: ContentInterpretCtx::new() }
      // java this.mTableInterpretCtx = new TableInterpreCtx();
      // java this.mThemeInterpretCtx = new ThemeInterpretCtx();
      // java this.mFilterInterpretCtx = new FilterInterpretCtx();
      // java this.mContentInterpretCtr = new ContentInterpretCtx();
    }

    // java public ColorInterpretCtx getColorInterpretCtx()
    pub fn getColorInterpretCtx(&self) -> &ColorInterpretCtx {
      // java return this.mColorInterpretCtx;
      return &self.mColorInterpretCtx;
    }

    // java public TableInterpreCtx getTableInterpretCtx()
    pub fn getTableInterpretCtx(&self) -> &TableInterpreCtx {
      // java return this.mTableInterpretCtx;
      return &self.mTableInterpretCtx;
    }

    // java public ThemeInterpretCtx getThemeInterpretCtx()
    pub fn getThemeInterpretCtx(&self) -> &ThemeInterpretCtx {
      // java return this.mThemeInterpretCtx;
      return &self.mThemeInterpretCtx;
    }

    // java public FilterInterpretCtx getFilterInterpretCtx()
    pub fn getFilterInterpretCtx(&self) -> &FilterInterpretCtx {
      // java return this.mFilterInterpretCtx;
      return &self.mFilterInterpretCtx;
    }

    // java public ContentInterpretCtx getContentInterpretCtx()
    pub fn getContentInterpretCtx(&self) -> &ContentInterpretCtx {
      // java return this.mContentInterpretCtr;
      return &self.mContentInterpretCtr;
    }

    // java public void setSlide(ISlide slide)
    pub fn setSlide(&mut self, slide: Option<&ISlide>) {
      // java this.mColorInterpretCtx.setSlide(slide);
      self.mColorInterpretCtx.setSlide(slide);
      // java this.mTableInterpretCtx.setSlide(slide);
      self.mTableInterpretCtx.setSlide(slide);
      // java this.mThemeInterpretCtx.setSlide(slide);
      self.mThemeInterpretCtx.setSlide(slide);
    }

    // java public void reset()
    pub fn reset(&mut self) {
      // java this.mColorInterpretCtx.reset();
      self.mColorInterpretCtx.reset();
      // java this.mFilterInterpretCtx.reset();
      self.mFilterInterpretCtx.reset();
    }

}
