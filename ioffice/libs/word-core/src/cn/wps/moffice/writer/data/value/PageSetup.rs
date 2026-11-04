// Java: import cn.wps.graphics.RectF
use util::graphics::RectF::*;
// Java: import java.io.Serializable
use stubs::java::io::Serializable::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java private static final long serialVersionUID = 1L;
const serialVersionUID: i64 = 1;
// java public static final int MIN_MARGIN = -31665;
pub(crate) const MIN_MARGIN: i32 = -31665;
// java public static final int MAX_MARGIN = 31665;
pub(crate) const MAX_MARGIN: i32 = 31665;
// java public static final int MIN_MARGIN_LEFT_RIGHT = 0;
pub(crate) const MIN_MARGIN_LEFT_RIGHT: i32 = 0;
// java public static final int MAX_MARGIN_LEFT_RIGHT = 31665;
pub(crate) const MAX_MARGIN_LEFT_RIGHT: i32 = 31665;

// java public final class PageSetup implements Serializable
#[derive(Clone, Copy, Default)]
pub struct PageSetup {
    // java int width;
    pub(crate) width: i32,
    // java int height;
    pub(crate) height: i32,
    // java int mMarginLeft;
    pub(crate) mMarginLeft: i32,
    // java int mMarginRight;
    pub(crate) mMarginRight: i32,
    // java int mMarginTop;
    pub(crate) mMarginTop: i32,
    // java int mMarginBottom;
    pub(crate) mMarginBottom: i32,
}

// java public final class PageSetup implements Serializable
impl PageSetup {
    // java public  PageSetup(int width, int height, int marginLeft, int marginRight, int marginTop, int marginBottom)
    pub fn PageSetup(width: i32, height: i32, marginLeft: i32, marginRight: i32, marginTop: i32, marginBottom: i32) -> Self {
      // java this.width = width;
      Self { width: width, height: height, mMarginLeft: marginLeft, mMarginRight: marginRight, mMarginTop: marginTop, mMarginBottom: marginBottom }
      // java this.height = height;
      // java this.mMarginLeft = marginLeft;
      // java this.mMarginRight = marginRight;
      // java this.mMarginTop = marginTop;
      // java this.mMarginBottom = marginBottom;
    }

    // java public  PageSetup(int width, int height)
    pub fn PageSetup__int_int(width: i32, height: i32) -> Self {
      // java this(width, height, 1800, 1800, 1440, 1440);
      let mut this = Self::PageSetup(width, height, 1800, 1800, 1440, 1440);
      this
    }

    // java public  PageSetup(int width, int height, int mx, int my)
    pub fn PageSetup__int_int_int_int(width: i32, height: i32, mx: i32, my: i32) -> Self {
      // java this(width, height, mx, mx, my, my);
      let mut this = Self::PageSetup(width, height, mx, mx, my, my);
      this
    }

    // java public  PageSetup(PageSetup pageSetup)
    pub fn PageSetup__pagesetup(pageSetup: &PageSetup) -> Self {
      // java this.copy(pageSetup);
      let mut this = Self::default();
      this.copy(pageSetup);
      this
    }

    // java public int getWidth()
    pub fn getWidth(&self) -> i32 {
      // java return this.width;
      return self.width;
    }

    // java public void setWidth(int newValue)
    pub fn setWidth(&mut self, newValue: i32) {
      // java this.width = newValue;
      self.width = newValue;
    }

    // java public int getHeight()
    pub fn getHeight(&self) -> i32 {
      // java return this.height;
      return self.height;
    }

    // java public void setHeight(int newValue)
    pub fn setHeight(&mut self, newValue: i32) {
      // java this.height = newValue;
      self.height = newValue;
    }

    // java public int getMarginLeft()
    pub fn getMarginLeft(&self) -> i32 {
      // java return this.mMarginLeft;
      return self.mMarginLeft;
    }

    // java public void setMarginLeft(int newValue)
    pub fn setMarginLeft(&mut self, newValue: i32) {
      // java this.mMarginLeft = newValue;
      self.mMarginLeft = newValue;
    }

    // java public int getMarginRight()
    pub fn getMarginRight(&self) -> i32 {
      // java return this.mMarginRight;
      return self.mMarginRight;
    }

    // java public void setMarginRight(int newValue)
    pub fn setMarginRight(&mut self, newValue: i32) {
      // java this.mMarginRight = newValue;
      self.mMarginRight = newValue;
    }

    // java public int getMarginTop()
    pub fn getMarginTop(&self) -> i32 {
      // java return this.mMarginTop;
      return self.mMarginTop;
    }

    // java public void setMarginTop(int newValue)
    pub fn setMarginTop(&mut self, newValue: i32) {
      // java this.mMarginTop = newValue;
      self.mMarginTop = newValue;
    }

    // java public int getMarginBottom()
    pub fn getMarginBottom(&self) -> i32 {
      // java return this.mMarginBottom;
      return self.mMarginBottom;
    }

    // java public void setMarginBottom(int newValue)
    pub fn setMarginBottom(&mut self, newValue: i32) {
      // java this.mMarginBottom = newValue;
      self.mMarginBottom = newValue;
    }

    // java public RectF getCoreRect()
    pub fn getCoreRect(&self) -> RectF {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return new RectF((float)this.mMarginLeft, (float)this.mMarginTop, (float)(this.width - this.mMarginRight), (float)(this.height - this.mMarginBottom));
      **/
    }

    pub fn contentWidth(&self) -> i32 {
      self.getWidth() - self.getMarginLeft() - self.getMarginRight()
    }

    pub fn contentHeight(&self) -> i32 {
      self.getHeight() - self.getMarginTop() - self.getMarginBottom()
    }

    // java public void copy(PageSetup pageSetup)
    pub fn copy(&mut self, pageSetup: &PageSetup) {
      // java this.width = pageSetup.getWidth();
      self.width = pageSetup.getWidth();
      // java this.height = pageSetup.getHeight();
      self.height = pageSetup.getHeight();
      // java this.mMarginLeft = pageSetup.getMarginLeft();
      self.mMarginLeft = pageSetup.getMarginLeft();
      // java this.mMarginRight = pageSetup.getMarginRight();
      self.mMarginRight = pageSetup.getMarginRight();
      // java this.mMarginTop = pageSetup.getMarginTop();
      self.mMarginTop = pageSetup.getMarginTop();
      // java this.mMarginBottom = pageSetup.getMarginBottom();
      self.mMarginBottom = pageSetup.getMarginBottom();
    }

    // java public boolean equals(Object o)
    pub fn equals(&self, o: &dyn std::any::Any) -> bool {
      // java if (null != o && o instanceof PageSetup)
      if true && o.downcast_ref::<PageSetup>().is_some() {
        // java PageSetup r = (PageSetup)o;
        let r: &PageSetup = o.downcast_ref::<PageSetup>().unwrap();
        // java return this.width == r.width && this.height == r.height && this.mMarginLeft == r.mMarginLeft && this.mMarginRight == r.mMarginRight && this.mMarginTop == r.mMarginTop && this.mMarginBottom == r.mMarginBottom;
        return self.width == r.width && self.height == r.height && self.mMarginLeft == r.mMarginLeft && self.mMarginRight == r.mMarginRight && self.mMarginTop == r.mMarginTop && self.mMarginBottom == r.mMarginBottom;
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
      // java return this.width + this.height + this.mMarginLeft + this.mMarginRight + this.mMarginTop + this.mMarginBottom;
      return self.width + self.height + self.mMarginLeft + self.mMarginRight + self.mMarginTop + self.mMarginBottom;
    }

    // java public boolean similar(Object o)
    pub fn similar(&self, o: &dyn std::any::Any) -> bool {
      // java PageSetup r = (PageSetup)o;
      let r: &PageSetup = o.downcast_ref::<PageSetup>().unwrap();
      // java return Math.abs(this.width - r.width) < 5 && Math.abs(this.height - r.height) < 5 && Math.abs(this.mMarginLeft - r.mMarginLeft) < 5 && Math.abs(this.mMarginRight - r.mMarginRight) < 5 && Math.abs(this.mMarginTop - r.mMarginTop) < 5 && Math.abs(this.mMarginBottom - r.mMarginBottom) < 5;
      return (self.width - r.width).abs() < 5 && (self.height - r.height).abs() < 5 && (self.mMarginLeft - r.mMarginLeft).abs() < 5 && (self.mMarginRight - r.mMarginRight).abs() < 5 && (self.mMarginTop - r.mMarginTop).abs() < 5 && (self.mMarginBottom - r.mMarginBottom).abs() < 5;
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java StringBuilder stringBuilder = new StringBuilder();
      let mut stringBuilder = String::new();
      // java stringBuilder.append("{");
      stringBuilder.push_str("{");
      // java stringBuilder.append("\n\t");
      stringBuilder.push_str("\n\t");
      // java stringBuilder.append("width = ");
      stringBuilder.push_str("width = ");
      // java stringBuilder.append(Integer.toString(this.width));
      stringBuilder.push_str(&self.width.to_string());
      // java stringBuilder.append("\n\t");
      stringBuilder.push_str("\n\t");
      // java stringBuilder.append("height = ");
      stringBuilder.push_str("height = ");
      // java stringBuilder.append(Integer.toString(this.height));
      stringBuilder.push_str(&self.height.to_string());
      // java stringBuilder.append("\n\t");
      stringBuilder.push_str("\n\t");
      // java stringBuilder.append("mMarginLeft = ");
      stringBuilder.push_str("mMarginLeft = ");
      // java stringBuilder.append(Integer.toString(this.mMarginLeft));
      stringBuilder.push_str(&self.mMarginLeft.to_string());
      // java stringBuilder.append("\n\t");
      stringBuilder.push_str("\n\t");
      // java stringBuilder.append("mMarginRight = ");
      stringBuilder.push_str("mMarginRight = ");
      // java stringBuilder.append(Integer.toString(this.mMarginRight));
      stringBuilder.push_str(&self.mMarginRight.to_string());
      // java stringBuilder.append("\n\t");
      stringBuilder.push_str("\n\t");
      // java stringBuilder.append("mMarginTop = ");
      stringBuilder.push_str("mMarginTop = ");
      // java stringBuilder.append(Integer.toString(this.mMarginTop));
      stringBuilder.push_str(&self.mMarginTop.to_string());
      // java stringBuilder.append("\n\t");
      stringBuilder.push_str("\n\t");
      // java stringBuilder.append("mMarginBottom = ");
      stringBuilder.push_str("mMarginBottom = ");
      // java stringBuilder.append(Integer.toString(this.mMarginBottom));
      stringBuilder.push_str(&self.mMarginBottom.to_string());
      // java stringBuilder.append("\n\t");
      stringBuilder.push_str("\n\t");
      // java stringBuilder.append("}");
      stringBuilder.push_str("}");
      // java return stringBuilder.toString();
      return stringBuilder;
    }

}
