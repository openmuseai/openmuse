#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]
// Java: import cn.wps.drawing.app.KmoProtocol
use crate::cn::wps::drawing::app::KmoProtocol::KmoProtocol;
// Java: import cn.wps.drawing.data.effect.EffectList
// LLM: EffectList is a KProto type, using stub
// Java: import cn.wps.drawing.data.effect.EffectProperties
// LLM: EffectProperties is a KProto type, using stub
// Java: import cn.wps.drawing.data.effect.GlowEffect
// LLM: GlowEffect is a KProto type, using stub
// Java: import cn.wps.drawing.data.effect.InnerShadowEffect
// LLM: InnerShadowEffect is a KProto type, using stub
// Java: import cn.wps.drawing.data.effect.OuterShadowEffect
// LLM: OuterShadowEffect is a KProto type, using stub
// Java: import cn.wps.drawing.data.effect.ReflectionEffect
// LLM: ReflectionEffect is a KProto type, using stub
// Java: import cn.wps.drawing.data.effect.SoftEdgeEffect
// LLM: SoftEdgeEffect is a KProto type, using stub
// Java: import cn.wps.kso.kproto.KoBin
// LLM: KoBin stub

use super::DgEffectShadow::DgEffectShadow;
use super::DgEffectReflection::DgEffectReflection;
use super::DgEffectGlow::DgEffectGlow;
use super::DgEffectSoftEdge::DgEffectSoftEdge;

pub struct KoBin;
pub struct EffectList;
impl EffectList {
    pub fn hasOuterShadow(&self) -> bool { false }
    pub fn outerShadow(&self) -> OuterShadowEffect { OuterShadowEffect }
    pub fn hasInnerShadow(&self) -> bool { false }
    pub fn innerShadow(&self) -> InnerShadowEffect { InnerShadowEffect }
    pub fn hasReflection(&self) -> bool { false }
    pub fn reflection(&self) -> ReflectionEffect { ReflectionEffect }
    pub fn hasGlow(&self) -> bool { false }
    pub fn glow(&self) -> GlowEffect { GlowEffect }
    pub fn hasSoftEdge(&self) -> bool { false }
    pub fn softEdge(&self) -> SoftEdgeEffect { SoftEdgeEffect }
    pub fn writeOuterShadow(&mut self, _e: OuterShadowEffect) {}
    pub fn writeInnerShadow(&mut self, _e: InnerShadowEffect) {}
    pub fn writeReflection(&mut self, _e: ReflectionEffect) {}
    pub fn writeGlow(&mut self, _e: GlowEffect) {}
    pub fn writeSoftEdge(&mut self, _e: SoftEdgeEffect) {}
}
pub struct EffectProperties;
impl EffectProperties {
    pub fn create() -> Self { EffectProperties }
    pub fn save(&self) -> KoBin { KoBin }
    pub fn load(&mut self, _bin: KoBin) {}
    pub fn r#type(&self) -> i32 { 0 }
    pub fn putType(&mut self, _t: i32) {}
    pub fn effectLst(&mut self) -> EffectList { EffectList }
}
pub struct OuterShadowEffect;
impl OuterShadowEffect {
    pub fn open(_bin: KoBin) -> Self { OuterShadowEffect }
    pub fn save(&self) -> KoBin { KoBin }
}
pub struct InnerShadowEffect;
impl InnerShadowEffect {
    pub fn open(_bin: KoBin) -> Self { InnerShadowEffect }
    pub fn save(&self) -> KoBin { KoBin }
}
pub struct ReflectionEffect;
impl ReflectionEffect {
    pub fn open(_bin: KoBin) -> Self { ReflectionEffect }
}
pub struct GlowEffect;
impl GlowEffect {
    pub fn open(_bin: KoBin) -> Self { GlowEffect }
}
pub struct SoftEdgeEffect;
impl SoftEdgeEffect {
    pub fn open(_bin: KoBin) -> Self { SoftEdgeEffect }
}

// Java: public class DgEffectFormat implements KmoProtocol, Cloneable
pub struct DgEffectFormat {
    // Java field: EffectProperties effectProperties
    pub(crate) effectProperties: EffectProperties,
    // Java field: DgEffectShadow shadow
    pub(crate) shadow: DgEffectShadow,
    // Java field: DgEffectReflection reflection
    pub(crate) reflection: DgEffectReflection,
    // Java field: DgEffectGlow glow
    pub(crate) glow: DgEffectGlow,
    // Java field: DgEffectSoftEdge softEdge
    pub(crate) softEdge: DgEffectSoftEdge,
}

impl DgEffectFormat {
    // Java: DgEffectFormat()
    pub fn DgEffectFormat_new() -> Self {
        DgEffectFormat {
            // Java: private EffectProperties effectProperties = EffectProperties.create();
            effectProperties: EffectProperties::create(),
            // Java: private DgEffectShadow shadow = new DgEffectShadow();
            shadow: DgEffectShadow::DgEffectShadow_new(),
            // Java: private DgEffectReflection reflection = new DgEffectReflection();
            reflection: DgEffectReflection::DgEffectReflection_new(),
            // Java: private DgEffectGlow glow = new DgEffectGlow();
            glow: DgEffectGlow::DgEffectGlow_new(),
            // Java: private DgEffectSoftEdge softEdge = new DgEffectSoftEdge();
            softEdge: DgEffectSoftEdge::DgEffectSoftEdge_new(),
        }
    }

    // Java: void updateProtocol(KoBin effectPrData)
    pub fn updateProtocol_local(&mut self, effectPrData: KoBin) {
        // Java: this.effectProperties.load(effectPrData);
        self.effectProperties.load(effectPrData);
        // Java: this.init();
        self.init();
    }

    // Java: KoBin protocol()
    pub fn protocol_local(&self) -> KoBin {
        // Java: return this.effectProperties.save();
        self.effectProperties.save()
    }

    // Java: void init()
    pub fn init(&mut self) {
        // Java: if (this.effectProperties.type() == 0)
        if self.effectProperties.r#type() == 0 {
            // Java: EffectList effectList = this.effectProperties.effectLst();
            let mut effectList = self.effectProperties.effectLst();
            // Java: if (effectList.hasOuterShadow())
            if effectList.hasOuterShadow() {
                // Java: this.shadow = new DgEffectShadow();
                self.shadow = DgEffectShadow::DgEffectShadow_new();
                // Java: this.shadow.setType(1);
                self.shadow.setType(1);
                // Java: this.shadow.updateProtocol(effectList.outerShadow().save());
                // LLM: self.shadow.updateProtocol mapped to updateProtocol_local; cross-module KoBin types differ
                todo!()
            }
            // Java: if (effectList.hasInnerShadow())
            if effectList.hasInnerShadow() {
                // Java: this.shadow = new DgEffectShadow();
                self.shadow = DgEffectShadow::DgEffectShadow_new();
                // Java: this.shadow.setType(0);
                self.shadow.setType(0);
                // Java: this.shadow.updateProtocol(effectList.innerShadow().save());
                // LLM: self.shadow.updateProtocol mapped to updateProtocol_local; cross-module KoBin types differ
                todo!()
            }
            // Java: if (effectList.hasReflection())
            if effectList.hasReflection() {
                // Java: this.reflection = new DgEffectReflection();
                self.reflection = DgEffectReflection::DgEffectReflection_new();
                // Java: this.reflection.updateProtocol(effectList.reflection().save());
                // LLM: self.reflection.updateProtocol mapped to updateProtocol_local; reflection().save() depends on KProto chain
                todo!()
            }
            // Java: if (effectList.hasGlow())
            if effectList.hasGlow() {
                // Java: this.glow = new DgEffectGlow();
                self.glow = DgEffectGlow::DgEffectGlow_new();
                // Java: this.glow.updateProtocol(effectList.glow().save());
                // LLM: self.glow.updateProtocol mapped to updateProtocol_local; glow().save() depends on KProto chain
                todo!()
            }
            // Java: if (effectList.hasSoftEdge())
            if effectList.hasSoftEdge() {
                // Java: this.softEdge = new DgEffectSoftEdge();
                self.softEdge = DgEffectSoftEdge::DgEffectSoftEdge_new();
                // Java: this.softEdge.updateProtocol(effectList.softEdge().save());
                // LLM: self.softEdge.updateProtocol mapped to updateProtocol_local; softEdge().save() depends on KProto chain
                todo!()
            }
        }
    }

    // Java: DgEffectShadow shadow()
    pub fn shadow(&mut self) -> &mut DgEffectShadow {
        // Java: return this.shadow;
        &mut self.shadow
    }

    // Java: void updateShadow()
    pub fn updateShadow(&mut self) {
        // Java: this.effectProperties.putType(0);
        self.effectProperties.putType(0);
        // Java: if (this.shadow.isInnerShadow())
        if self.shadow.isInnerShadow() {
            // Java: this.effectProperties.effectLst().writeInnerShadow(InnerShadowEffect.open(this.shadow.protocol()));
            // LLM: self.shadow.protocol mapped to protocol_local; InnerShadowEffect.open depends on KProto chain
            todo!()
        // Java: else
        } else {
            // Java: this.effectProperties.effectLst().writeOuterShadow(OuterShadowEffect.open(this.shadow.protocol()));
            // LLM: self.shadow.protocol mapped to protocol_local; OuterShadowEffect.open depends on KProto chain
            todo!()
        }
    }

    // Java: DgEffectReflection reflection()
    pub fn reflection(&mut self) -> &mut DgEffectReflection {
        // Java: return this.reflection;
        &mut self.reflection
    }

    // Java: void updateReflection()
    pub fn updateReflection(&mut self) {
        // Java: this.effectProperties.putType(0);
        self.effectProperties.putType(0);
        // Java: this.effectProperties.effectLst().writeReflection(ReflectionEffect.open(this.reflection.protocol()));
        // LLM: self.reflection.protocol mapped to protocol_local; ReflectionEffect.open depends on KProto chain
        todo!()
    }

    // Java: DgEffectGlow glow()
    pub fn glow(&mut self) -> &mut DgEffectGlow {
        // Java: return this.glow;
        &mut self.glow
    }

    // Java: void updateGlow()
    pub fn updateGlow(&mut self) {
        // Java: this.effectProperties.putType(0);
        self.effectProperties.putType(0);
        // Java: this.effectProperties.effectLst().writeGlow(GlowEffect.open(this.glow.protocol()));
        // LLM: self.glow.protocol mapped to protocol_local; GlowEffect.open depends on KProto chain
        todo!()
    }

    // Java: DgEffectSoftEdge softEdge()
    pub fn softEdge(&mut self) -> &mut DgEffectSoftEdge {
        // Java: return this.softEdge;
        &mut self.softEdge
    }

    // Java: void updateSoftEdge()
    pub fn updateSoftEdge(&mut self) {
        // Java: this.effectProperties.putType(0);
        self.effectProperties.putType(0);
        // Java: this.effectProperties.effectLst().writeSoftEdge(SoftEdgeEffect.open(this.softEdge.protocol()));
        // LLM: self.softEdge.protocol mapped to protocol_local; SoftEdgeEffect.open depends on KProto chain
        todo!()
    }

    // Java: DgEffectFormat clone()
    pub fn clone_effect(&self) -> DgEffectFormat {
        // Java: DgEffectFormat effectFormat = new DgEffectFormat();
        let mut effectFormat = DgEffectFormat::DgEffectFormat_new();
        // Java: effectFormat.updateProtocol(this.protocol());
        // LLM: effectFormat.updateProtocol mapped to updateProtocol_local; this.protocol mapped to protocol_local
        effectFormat.updateProtocol_local(self.protocol_local());
        // Java: return effectFormat;
        effectFormat
    }
}

pub trait Cloneable {}

impl KmoProtocol for DgEffectFormat {
    fn protocol(&self) -> Box<dyn crate::cn::wps::kso::kproto::KoBin::KoBin> { todo!() }
    fn updateProtocol(&mut self, _var1: &dyn crate::cn::wps::kso::kproto::KoBin::KoBin) { todo!() }
}

impl Cloneable for DgEffectFormat {}
