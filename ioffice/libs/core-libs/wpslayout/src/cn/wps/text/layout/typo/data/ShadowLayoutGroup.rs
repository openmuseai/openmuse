use crate::cn::wps::text::layout::typo::layout::Spannable::Spannable;
use super::ShadowLayoutType::ShadowLayoutType;

pub struct Layout;
pub struct SpannableStringBuilder;

// Java: public class ShadowLayoutGroup
pub struct ShadowLayoutGroup {
    // Java field: Layout mFillShadowLayout
    pub(crate) mFillShadowLayout: Option<Layout>,
    // Java field: Layout mOutlineShadowLayout
    pub(crate) mOutlineShadowLayout: Option<Layout>,
    // Java field: Layout mBlurShadowLayout
    pub(crate) mBlurShadowLayout: Option<Layout>,
}

pub struct ShadowLayoutData;

// Java: public class ShadowLayoutGroup
impl ShadowLayoutGroup {
    // Java: ShadowLayoutGroup()
    pub fn new() -> Self {
      // Java: super();
      ShadowLayoutGroup {
          mFillShadowLayout: None,
          mOutlineShadowLayout: None,
          mBlurShadowLayout: None,
      }
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.mFillShadowLayout == null || this.mBlurShadowLayout == null || this.mOutlineShadowLayout == null;
      self.mFillShadowLayout.is_none() || self.mBlurShadowLayout.is_none() || self.mOutlineShadowLayout.is_none()
    }

    // Java: List<Layout> getLayoutList()
    pub fn getLayoutList(&self) -> Vec<&Layout> {
      // Java: List<Layout> layouts = new ArrayList();
      let mut layouts = Vec::new();
      // Java: layouts.add(this.mFillShadowLayout);
      if let Some(ref l) = self.mFillShadowLayout { layouts.push(l); }
      // Java: layouts.add(this.mOutlineShadowLayout);
      if let Some(ref l) = self.mOutlineShadowLayout { layouts.push(l); }
      // Java: layouts.add(this.mBlurShadowLayout);
      if let Some(ref l) = self.mBlurShadowLayout { layouts.push(l); }
      // Java: return layouts;
      layouts
    }

    // Java: List<Spannable> getSpanList(boolean isRef)
    pub fn getSpanList(&self, isRef: bool) -> Vec<Box<dyn Spannable>> {
      // Java: List<Spannable> spanList = new ArrayList();
      let spanList: Vec<Box<dyn Spannable>> = Vec::new();
      // Java: if (isRef)
      if isRef
      {
         // Java: if (this.mFillShadowLayout != null)
         {
            // Java: spanList.add((Spannable)this.mFillShadowLayout.getText());
         }
         // Java: if (this.mOutlineShadowLayout != null)
         {
            // Java: spanList.add((Spannable)this.mOutlineShadowLayout.getText());
         }
         // Java: if (this.mBlurShadowLayout != null)
         {
            // Java: spanList.add((Spannable)this.mBlurShadowLayout.getText());
         }
      }
      // Java: else
      else
      {
         // Java: if (this.mFillShadowLayout != null)
         {
            // Java: spanList.add(new SpannableStringBuilder(this.mFillShadowLayout.getText()));
         }
         // Java: if (this.mOutlineShadowLayout != null)
         {
            // Java: spanList.add(new SpannableStringBuilder(this.mOutlineShadowLayout.getText()));
         }
         // Java: if (this.mBlurShadowLayout != null)
         {
            // Java: spanList.add(new SpannableStringBuilder(this.mBlurShadowLayout.getText()));
         }
      }
      // Java: return spanList;
      spanList
    }

    // Java: List<ShadowLayoutData> getTypoData()
    pub fn getTypoData(&self) -> Vec<ShadowLayoutData> {
      // Java: List<ShadowLayoutData> typoDataList = new ArrayList();
      let typoDataList: Vec<ShadowLayoutData> = Vec::new();
      // Java: if (this.mFillShadowLayout != null)
      // Java: Spannable fillSpan = new SpannableStringBuilder(this.mFillShadowLayout.getText());
      // Java: ShadowLayoutData fillTypoData = new ShadowLayoutData(this.mFillShadowLayout, fillSpan, ShadowLayoutType.fill);
      // Java: typoDataList.add(fillTypoData);
      // Java: if (this.mOutlineShadowLayout != null)
      // Java: Spannable outlineSpan = new SpannableStringBuilder(this.mOutlineShadowLayout.getText());
      // Java: ShadowLayoutData outlineTypoData = new ShadowLayoutData(this.mOutlineShadowLayout, outlineSpan, ShadowLayoutType.outline);
      // Java: typoDataList.add(outlineTypoData);
      // Java: if (this.mBlurShadowLayout != null)
      // Java: Spannable blurSpan = new SpannableStringBuilder(this.mBlurShadowLayout.getText());
      // Java: ShadowLayoutData blurData = new ShadowLayoutData(this.mBlurShadowLayout, blurSpan, ShadowLayoutType.blur);
      // Java: typoDataList.add(blurData);
      // Java: return typoDataList;
      typoDataList
    }

}
