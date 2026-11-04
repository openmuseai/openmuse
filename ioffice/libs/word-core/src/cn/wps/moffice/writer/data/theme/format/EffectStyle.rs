// Java: cn.wps.moffice.writer.data.theme.format.EffectStyle
//
// ★M-STAGE1 P2-9 —— 全仓原缺失，按 Java 原文（24 行）新建。
//   2 个可空引用字段 + 2 对访问器。

// Java: import cn.wps.moffice.drawing.effects.Object3DDml
use drawing::cn::wps::moffice::drawing::effects::Object3DDml::Object3DDml;
// Java: （同包）EffectLst
use crate::cn::wps::moffice::writer::data::theme::format::EffectLst::EffectLst;

// java public class EffectStyle
#[derive(Default)]
pub struct EffectStyle {
    // java private EffectLst mEffectLst;
    mEffectLst: Option<EffectLst>,
    // java private Object3DDml mObject3DDml;
    mObject3DDml: Option<Object3DDml>,
}

// java public class EffectStyle
impl EffectStyle {
    // rust: Java 无显式构造器（走默认构造）⇒ `new()` 为 Rust 侧便利方法。
    pub fn new() -> Self {
        Self::default()
    }

    // java public EffectLst getEffectLst()
    pub fn getEffectLst(&mut self) -> Option<&mut EffectLst> {
        // java return this.mEffectLst;
        return self.mEffectLst.as_mut();
    }

    // java public void setEffectLst(EffectLst effectLst)
    pub fn setEffectLst(&mut self, effectLst: Option<EffectLst>) {
        // java this.mEffectLst = effectLst;
        self.mEffectLst = effectLst;
    }

    // java public Object3DDml getObject3DDml()
    pub fn getObject3DDml(&mut self) -> Option<&mut Object3DDml> {
        // java return this.mObject3DDml;
        return self.mObject3DDml.as_mut();
    }

    // java public void setObject3DDml(Object3DDml object3DDml)
    pub fn setObject3DDml(&mut self, object3DDml: Option<Object3DDml>) {
        // java this.mObject3DDml = object3DDml;
        self.mObject3DDml = object3DDml;
    }
}
