// Java: cn.wps.moffice.writer.data.theme.format.EffectLst
//
// ★M-STAGE1 P2-9 —— 全仓原缺失，按 Java 原文（65 行）新建。
//   6 个可空引用字段 + 6 对访问器。

// Java: import cn.wps.moffice.drawing.effects.Glow
use drawing::cn::wps::moffice::drawing::effects::Glow::Glow;
// Java: import cn.wps.moffice.drawing.effects.Perspective
use drawing::cn::wps::moffice::drawing::effects::Perspective::Perspective;
// Java: import cn.wps.moffice.drawing.effects.Reflection
use drawing::cn::wps::moffice::drawing::effects::Reflection::Reflection;
// Java: import cn.wps.moffice.drawing.effects.ShadowDml
use drawing::cn::wps::moffice::drawing::effects::ShadowDml::ShadowDml;
// Java: import cn.wps.moffice.drawing.effects.SoftEdge
use drawing::cn::wps::moffice::drawing::effects::SoftEdge::SoftEdge;
// Java: import cn.wps.moffice.drawing.graphics.FillBase
use drawing::cn::wps::moffice::drawing::graphics::FillBase::FillBase;

// java public class EffectLst
#[derive(Default)]
pub struct EffectLst {
    // java private FillBase mFillOverlay;
    mFillOverlay: Option<FillBase>,
    // java private Glow mGlow;
    mGlow: Option<Glow>,
    // java private ShadowDml mShadow;
    mShadow: Option<ShadowDml>,
    // java private Reflection mReflection;
    mReflection: Option<Reflection>,
    // java private Perspective mPerspective;
    mPerspective: Option<Perspective>,
    // java private SoftEdge mSoftEdge;
    mSoftEdge: Option<SoftEdge>,
}

// java public class EffectLst
impl EffectLst {
    // rust: Java 无显式构造器（走默认构造）⇒ `new()` 为 Rust 侧便利方法。
    pub fn new() -> Self {
        Self::default()
    }

    // java public FillBase getFillOverlay()
    pub fn getFillOverlay(&mut self) -> Option<&mut FillBase> {
        // java return this.mFillOverlay;
        return self.mFillOverlay.as_mut();
    }

    // java public void setFillOverlay(FillBase fillOverlay)
    pub fn setFillOverlay(&mut self, fillOverlay: Option<FillBase>) {
        // java this.mFillOverlay = fillOverlay;
        self.mFillOverlay = fillOverlay;
    }

    // java public Glow getGlow()
    pub fn getGlow(&mut self) -> Option<&mut Glow> {
        // java return this.mGlow;
        return self.mGlow.as_mut();
    }

    // java public void setGlow(Glow mGlow)
    pub fn setGlow(&mut self, mGlow: Option<Glow>) {
        // java this.mGlow = mGlow;
        self.mGlow = mGlow;
    }

    // java public ShadowDml getShadow()
    pub fn getShadow(&mut self) -> Option<&mut ShadowDml> {
        // java return this.mShadow;
        return self.mShadow.as_mut();
    }

    // java public void setShadow(ShadowDml mShadow)
    pub fn setShadow(&mut self, mShadow: Option<ShadowDml>) {
        // java this.mShadow = mShadow;
        self.mShadow = mShadow;
    }

    // java public Reflection getReflection()
    pub fn getReflection(&mut self) -> Option<&mut Reflection> {
        // java return this.mReflection;
        return self.mReflection.as_mut();
    }

    // java public void setReflection(Reflection mReflection)
    pub fn setReflection(&mut self, mReflection: Option<Reflection>) {
        // java this.mReflection = mReflection;
        self.mReflection = mReflection;
    }

    // java public Perspective getPerspective()
    pub fn getPerspective(&mut self) -> Option<&mut Perspective> {
        // java return this.mPerspective;
        return self.mPerspective.as_mut();
    }

    // java public void setPerspective(Perspective mPerspective)
    pub fn setPerspective(&mut self, mPerspective: Option<Perspective>) {
        // java this.mPerspective = mPerspective;
        self.mPerspective = mPerspective;
    }

    // java public SoftEdge getSoftEdge()
    pub fn getSoftEdge(&mut self) -> Option<&mut SoftEdge> {
        // java return this.mSoftEdge;
        return self.mSoftEdge.as_mut();
    }

    // java public void setSoftEdge(SoftEdge softEdge)
    pub fn setSoftEdge(&mut self, softEdge: Option<SoftEdge>) {
        // java this.mSoftEdge = softEdge;
        self.mSoftEdge = softEdge;
    }
}
