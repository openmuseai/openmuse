// Java: import cn.wps.moffice.writer.core.metadata.variant.VariantBase
use crate::cn::wps::moffice::writer::core::metadata::variant::VariantBase::*;
// Java: import cn.wps.moffice.writer.core.metadata.variant.VariantWStr
use crate::cn::wps::moffice::writer::core::metadata::variant::VariantWStr::*;

// java public class VariantCustom
// rust: 派生 `Clone` —— `CustomMetadata.mCustomData` 按下标取出需 clone。
#[derive(Default, Clone)]
pub struct VariantCustom {
    // java VariantBase mVariant;
    pub(crate) mVariant: Option<VariantBase>,
    // java String mName;
    pub(crate) mName: Option<String>,
    // java String mLinkTarget;
    pub(crate) mLinkTarget: Option<String>,
    // rust: ★T7-7 —— Java 的 `mVariant` 存的是**子类实例**（如 `VariantWStr`），
    //   取用时靠 `(VariantWStr)variant` 下转拿字符串值
    //   （`KCustomIcv.getPropertyStrVal`）。Rust 门面此前只留 `VariantBase`
    //   （仅含 `mType`），子类载荷丢失 ⇒ 追加该槽保存下转后的实例。
    pub(crate) mVariantWStr: Option<VariantWStr>,
}

// java public class VariantCustom
impl VariantCustom {
    // java public  VariantCustom(String name, VariantBase variant)
    pub fn VariantCustom(name: &str, variant: &VariantBase) -> Self {
      // java this.mName = name;
      Self { mVariant: Some(variant.clone()), mVariantWStr: None, mName: Some(name.to_string()), mLinkTarget: Default::default() }
      // java this.mVariant = variant;
    }

    // rust-only —— `KCustomIcv.getPropertyStrVal` 的 `(VariantWStr)variant` 下转，
    //   返回字符串载荷；非 `VariantWStr` 变体返回 `None`（对应 Java
    //   `!(variant instanceof VariantWStr)` 的 null 返回）。
    pub fn getVariantStringValue(&self) -> Option<String> {
        self.mVariantWStr.as_ref().map(|w| w.getVar())
    }

    // java public String getName()
    pub fn getName(&self) -> String {
      // java return this.mName;
      return self.mName.clone().unwrap_or_default();
    }

    // java public void setName(String name)
    pub fn setName(&mut self, name: &str) {
      // java this.mName = name;
      self.mName = Some(name.to_string());
    }

    // java public String getLinkTarget()
    pub fn getLinkTarget(&self) -> String {
      // java return this.mLinkTarget;
      return self.mLinkTarget.clone().unwrap_or_default();
    }

    // java public void setLinkTarget(String linkTarget)
    pub fn setLinkTarget(&mut self, linkTarget: &str) {
      // java this.mLinkTarget = linkTarget;
      self.mLinkTarget = Some(linkTarget.to_string());
    }

    // java public VariantBase getVariant()
    pub fn getVariant(&self) -> VariantBase {
      // java return this.mVariant;
      return self.mVariant.clone().unwrap_or_default();
    }

    // java public void setVariant(VariantBase variant)
    pub fn setVariant(&mut self, variant: &VariantBase) {
      // java this.mVariant = variant;
      self.mVariant = Some(variant.clone());
    }

}
