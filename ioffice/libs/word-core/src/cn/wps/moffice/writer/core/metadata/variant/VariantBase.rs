// Java: import cn.wps.moffice.writer.core.metadata.variant.VariantType
use crate::cn::wps::moffice::writer::core::metadata::variant::VariantType::*;

// java public class VariantBase
// rust: 派生 `Clone` —— 供 `VariantCustom` 的 `Clone` 传递所需。
#[derive(Default, Clone)]
pub struct VariantBase {
    // java VariantType mType;
    pub(crate) mType: Option<VariantType>,
}

// java public class VariantBase
impl VariantBase {
    // java public  VariantBase(VariantType type)
    pub fn VariantBase(r#type: &VariantType) -> Self {
      // java this.mType = VariantType.var_wstr;
      Self { mType: Some(*r#type) }
      // java this.mType = type;
    }

    // java public VariantType getType()
    pub fn getType(&self) -> VariantType {
      // java return this.mType;
      return self.mType.unwrap_or(VariantType::var_wstr);
    }

    // java public void setType(VariantType type)
    pub fn setType(&mut self, r#type: &VariantType) {
      // java this.mType = type;
      self.mType = Some(*r#type);
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
    }

}
