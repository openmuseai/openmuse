// Java: import cn.wps.text.layout.typo.layout.Layout
// Java: import cn.wps.text.layout.typo.layout.Spannable

use super::ShadowLayoutType::ShadowLayoutType;

pub struct Layout;
pub trait Spannable {}

// Java: public class ShadowLayoutData
pub struct ShadowLayoutData {
    // Java field: Layout mOrigLayout
    pub(crate) mOrigLayout: Option<Layout>,
    // Java field: Spannable mSpannable
    pub(crate) mSpannable: Option<Box<dyn Spannable>>,
    // Java field: ShadowLayoutType mShadowType
    pub(crate) mShadowType: ShadowLayoutType,
}

// Java: public class ShadowLayoutData
impl ShadowLayoutData {
    // Java: ShadowLayoutData()
    pub fn new() -> Self {
      // Java: super();
      ShadowLayoutData {
          mOrigLayout: None,
          mSpannable: None,
          mShadowType: ShadowLayoutType::fill,
      }
    }

    // Java: ShadowLayoutData(Layout layout, Spannable span, ShadowLayoutType type)
    pub fn new_with(layout: Option<Layout>, span: Option<Box<dyn Spannable>>, r#type: ShadowLayoutType) -> Self {
      // Java: super();
      // Java: this.mOrigLayout = layout;
      // Java: this.mSpannable = span;
      // Java: this.mShadowType = type;
      ShadowLayoutData {
          mOrigLayout: layout,
          mSpannable: span,
          mShadowType: r#type,
      }
    }

    // Java: Layout getOrigLayout()
    pub fn getOrigLayout(&self) -> &Option<Layout> {
      // Java: return this.mOrigLayout;
      &self.mOrigLayout
    }

    // Java: void setOrigLayout(Layout origLayout)
    pub fn setOrigLayout(&mut self, origLayout: Option<Layout>) {
      // Java: this.mOrigLayout = origLayout;
      self.mOrigLayout = origLayout;
    }

    // Java: Spannable getSpannable()
    pub fn getSpannable(&self) -> &Option<Box<dyn Spannable>> {
      // Java: return this.mSpannable;
      &self.mSpannable
    }

    // Java: void setSpannable(Spannable spannable)
    pub fn setSpannable(&mut self, spannable: Option<Box<dyn Spannable>>) {
      // Java: this.mSpannable = spannable;
      self.mSpannable = spannable;
    }

    // Java: ShadowLayoutType getShadowType()
    pub fn getShadowType(&self) -> &ShadowLayoutType {
      // Java: return this.mShadowType;
      &self.mShadowType
    }

    // Java: void setShadowType(ShadowLayoutType shadowType)
    pub fn setShadowType(&mut self, shadowType: ShadowLayoutType) {
      // Java: this.mShadowType = shadowType;
      self.mShadowType = shadowType;
    }

}
