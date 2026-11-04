#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]
// Java: import cn.wps.drawing.app.KmoProtocol
use crate::cn::wps::drawing::app::KmoProtocol::KmoProtocol;
// Java: import cn.wps.drawing.app.util.UnitsConverter
// LLM: UnitsConverter not yet available, using stub
// Java: import cn.wps.drawing.data.effect.SoftEdgeEffect
// LLM: SoftEdgeEffect is a KProto type, using stub
// Java: import cn.wps.kso.kproto.KoBin
// LLM: KoBin stub

pub struct SoftEdgeEffect;
impl SoftEdgeEffect {
    pub fn create() -> Self { SoftEdgeEffect }
    pub fn save(&self) -> KoBin { KoBin }
    pub fn load(&mut self, _bin: KoBin) {}
    pub fn hasRadius(&self) -> bool { false }
    pub fn radius(&self) -> i32 { 0 }
    pub fn putRadius(&mut self, _v: i32) {}
}
pub struct KoBin;
pub struct UnitsConverter;
impl UnitsConverter {
    pub fn emu2pt(v: f64) -> f64 { v / 12700.0 }
    pub fn pt2emu(v: f64) -> f64 { v * 12700.0 }
}

// Java: public class DgEffectSoftEdge implements KmoProtocol
pub struct DgEffectSoftEdge {
    // Java field: SoftEdgeEffect softEdgeEffect
    pub(crate) softEdgeEffect: SoftEdgeEffect,
}

impl DgEffectSoftEdge {
    pub fn DgEffectSoftEdge_new() -> Self {
        DgEffectSoftEdge {
            softEdgeEffect: SoftEdgeEffect::create(),
        }
    }
    // Java: KoBin protocol()
    pub fn protocol_local(&self) -> KoBin {
        // Java: return this.softEdgeEffect.save();
        self.softEdgeEffect.save()
    }
    // Java: void updateProtocol(KoBin data)
    pub fn updateProtocol_local(&mut self, data: KoBin) {
        // Java: this.softEdgeEffect.load(data);
        self.softEdgeEffect.load(data);
    }
    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return !this.softEdgeEffect.hasRadius();
        !self.softEdgeEffect.hasRadius()
    }
    // Java: void setSize(double size)
    pub fn setSize(&mut self, size: f64) {
        // Java: this.softEdgeEffect.putRadius((int)UnitsConverter.pt2emu(size));
        self.softEdgeEffect.putRadius(UnitsConverter::pt2emu(size) as i32);
    }
    // Java: double size()
    pub fn size(&self) -> f64 {
        // Java: return UnitsConverter.emu2pt((double)this.softEdgeEffect.radius());
        UnitsConverter::emu2pt(self.softEdgeEffect.radius() as f64)
    }
}

impl KmoProtocol for DgEffectSoftEdge {
    fn protocol(&self) -> Box<dyn crate::cn::wps::kso::kproto::KoBin::KoBin> { todo!() }
    fn updateProtocol(&mut self, _var1: &dyn crate::cn::wps::kso::kproto::KoBin::KoBin) { todo!() }
}
