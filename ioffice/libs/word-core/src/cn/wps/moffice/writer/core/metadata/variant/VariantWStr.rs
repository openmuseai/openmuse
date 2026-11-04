// Rust stub for VariantWStr (auto-generated)
// Original: cn/wps/moffice/writer/core/metadata/variant/VariantWStr.java
//
// rust: ★T7-7 真实化 —— 原为纯 `todo!()` 桩，使 `KCustomIcv` 的
//   `new VariantWStr(icv)`（ICV 字符串变体，见 `KCustomIcv.java` 的
//   `updateIcv` / `addIcv`）整条不可用。
//   按 Java 忠实建模：`VariantWStr extends VariantBase`，
//   Rust 无继承 ⇒ 以 `super_: VariantBase` 字段承载基类身份
//   （与 `data::PLCField::FieldNode extends PLC.Node` 的既有写法一致）。
#![allow(non_snake_case, dead_code, unused_imports)]
use crate::*;
use crate::cn::wps::moffice::writer::core::metadata::variant::VariantBase::VariantBase;
use crate::cn::wps::moffice::writer::core::metadata::variant::VariantType::VariantType;

#[derive(Clone, Default)]
pub struct VariantWStr {
    // java VariantBase super_
    pub(crate) super_: VariantBase,
    // java String mVar = null;
    pub(crate) mVar: Option<String>,
}

impl VariantWStr {
    // java public VariantWStr(String var) {
    //    super(VariantType.var_wstr);
    //    Assert.assertNotNull("var should not be null!", var);
    //    this.mVar = var;
    // }
    pub fn VariantWStr(var: String) -> Self {
        Self {
            super_: VariantBase::VariantBase(&VariantType::var_wstr),
            mVar: Some(var),
        }
    }

    // rust-only: Rust 门面别名 —— 调用点写作 `VariantWStr::new(icv)`。
    pub fn new(var: String) -> Self {
        Self::VariantWStr(var)
    }

    // java public void setVar(String var)
    pub fn setVar(&mut self, var: String) {
        self.mVar = Some(var);
    }

    // java public String getVar()
    pub fn getVar(&self) -> String {
        self.mVar.clone().unwrap_or_default()
    }
}

impl std::ops::Deref for VariantWStr {
    type Target = VariantBase;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for VariantWStr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
