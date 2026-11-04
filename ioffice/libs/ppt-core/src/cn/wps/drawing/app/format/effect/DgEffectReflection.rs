#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]
// Java: import cn.wps.drawing.app.KmoProtocol
use crate::cn::wps::drawing::app::KmoProtocol::KmoProtocol;
// Java: import cn.wps.drawing.app.util.UnitsConverter
// LLM: UnitsConverter not yet available, using stub
// Java: import cn.wps.drawing.data.effect.ReflectionEffect
// LLM: ReflectionEffect is a KProto type, using stub
// Java: import cn.wps.kso.kproto.KoBin
// LLM: KoBin stub

pub struct ReflectionEffect;
impl ReflectionEffect {
    pub fn create() -> Self { ReflectionEffect }
    pub fn save(&self) -> KoBin { KoBin }
    pub fn load(&mut self, _bin: KoBin) {}
    pub fn close(&mut self) {}
    pub fn blurRadius(&self) -> i32 { 0 }
    pub fn putBlurRadius(&mut self, _v: i32) {}
    pub fn stA(&self) -> f64 { 0.0 }
    pub fn putStA(&mut self, _v: f64) {}
    pub fn endA(&self) -> f64 { 0.0 }
    pub fn putEndA(&mut self, _v: f64) {}
    pub fn endPos(&self) -> f64 { 0.0 }
    pub fn putEndPos(&mut self, _v: f64) {}
    pub fn distance(&self) -> i32 { 0 }
    pub fn putDistance(&mut self, _v: i32) {}
    pub fn direction(&self) -> i32 { 0 }
    pub fn putDirection(&mut self, _v: i32) {}
    pub fn sy(&self) -> f64 { 0.0 }
    pub fn putSy(&mut self, _v: f64) {}
    pub fn algn(&self) -> i32 { 0 }
    pub fn putAlgn(&mut self, _v: i32) {}
    pub fn isRotateWithShape(&self) -> bool { false }
    pub fn putRotateWithShape(&mut self, _v: bool) {}
}
pub struct KoBin;
pub struct UnitsConverter;
impl UnitsConverter {
    pub fn emu2pt(v: f64) -> f64 { v / 12700.0 }
    pub fn pt2emu(v: f64) -> f64 { v * 12700.0 }
    pub fn degree2Angle(v: f64) -> i32 { (v * 60000.0) as i32 }
    pub fn angle2Degree(v: f64) -> f64 { v / 60000.0 }
    pub fn percent2decimals(v: f64) -> f64 { v / 100.0 }
    pub fn decimals2percent(v: f64) -> f64 { v * 100.0 }
}

// Java: public class DgEffectReflection implements KmoProtocol
pub struct DgEffectReflection {
    // Java field: ReflectionEffect reflectionEffect
    pub(crate) reflectionEffect: ReflectionEffect,
    // Java field: ReflectionEffect touch1
    pub(crate) touch1: Option<ReflectionEffect>,
    // Java field: ReflectionEffect touch2
    pub(crate) touch2: Option<ReflectionEffect>,
    // Java field: ReflectionEffect touch3
    pub(crate) touch3: Option<ReflectionEffect>,
    // Java field: ReflectionEffect fourPtOffset1
    pub(crate) fourPtOffset1: Option<ReflectionEffect>,
    // Java field: ReflectionEffect fourPtOffset2
    pub(crate) fourPtOffset2: Option<ReflectionEffect>,
    // Java field: ReflectionEffect fourPtOffset3
    pub(crate) fourPtOffset3: Option<ReflectionEffect>,
    // Java field: ReflectionEffect eightPtOffset1
    pub(crate) eightPtOffset1: Option<ReflectionEffect>,
    // Java field: ReflectionEffect eightPtOffset2
    pub(crate) eightPtOffset2: Option<ReflectionEffect>,
    // Java field: ReflectionEffect eightPtOffset3
    pub(crate) eightPtOffset3: Option<ReflectionEffect>,
}

impl DgEffectReflection {
    pub fn DgEffectReflection_new() -> Self {
        DgEffectReflection {
            reflectionEffect: ReflectionEffect::create(),
            touch1: None, touch2: None, touch3: None,
            fourPtOffset1: None, fourPtOffset2: None, fourPtOffset3: None,
            eightPtOffset1: None, eightPtOffset2: None, eightPtOffset3: None,
        }
    }
    // Java: KoBin protocol()
    pub fn protocol_local(&self) -> KoBin {
        // Java: return this.reflectionEffect.save();
        self.reflectionEffect.save()
    }
    // Java: void updateProtocol(KoBin data)
    pub fn updateProtocol_local(&mut self, data: KoBin) {
        // Java: this.reflectionEffect.load(data);
        self.reflectionEffect.load(data);
    }
    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return isEqualReflectionEffect(this.reflectionEffect, ReflectionEffect.create());
        // LLM: isEqualReflectionEffect depends on KProto comparison chain
        DgEffectReflection::isEqualReflectionEffect(&self.reflectionEffect, &ReflectionEffect::create())
    }
    // Java: void setPresetType(int type)
    pub fn setPresetType(&mut self, r#type: i32) {
        // Java: this.intPresetType();
        self.intPresetType();
        // Java: switch (type)
        match r#type {
            // Java: case 0:
            0 => {
                // Java: this.updateProtocol(this.touch1.save());
                // LLM: self.updateProtocol mapped to updateProtocol_local; touch1 is Option
                if let Some(ref t) = self.touch1 { self.reflectionEffect.load(t.save()); }
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 1:
            1 => {
                // Java: this.updateProtocol(this.touch2.save());
                // LLM: self.updateProtocol mapped to updateProtocol_local; touch2 is Option
                if let Some(ref t) = self.touch2 { self.reflectionEffect.load(t.save()); }
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 2:
            2 => {
                // Java: this.updateProtocol(this.touch3.save());
                // LLM: self.updateProtocol mapped to updateProtocol_local; touch3 is Option
                if let Some(ref t) = self.touch3 { self.reflectionEffect.load(t.save()); }
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 3:
            3 => {
                // Java: this.updateProtocol(this.fourPtOffset1.save());
                // LLM: self.updateProtocol mapped to updateProtocol_local
                if let Some(ref t) = self.fourPtOffset1 { self.reflectionEffect.load(t.save()); }
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 4:
            4 => {
                // Java: this.updateProtocol(this.fourPtOffset2.save());
                // LLM: self.updateProtocol mapped to updateProtocol_local
                if let Some(ref t) = self.fourPtOffset2 { self.reflectionEffect.load(t.save()); }
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 5:
            5 => {
                // Java: this.updateProtocol(this.fourPtOffset3.save());
                // LLM: self.updateProtocol mapped to updateProtocol_local
                if let Some(ref t) = self.fourPtOffset3 { self.reflectionEffect.load(t.save()); }
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 6:
            6 => {
                // Java: this.updateProtocol(this.eightPtOffset1.save());
                // LLM: self.updateProtocol mapped to updateProtocol_local
                if let Some(ref t) = self.eightPtOffset1 { self.reflectionEffect.load(t.save()); }
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 7:
            7 => {
                // Java: this.updateProtocol(this.eightPtOffset2.save());
                // LLM: self.updateProtocol mapped to updateProtocol_local
                if let Some(ref t) = self.eightPtOffset2 { self.reflectionEffect.load(t.save()); }
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 8:
            8 => {
                // Java: this.updateProtocol(this.eightPtOffset3.save());
                // LLM: self.updateProtocol mapped to updateProtocol_local
                if let Some(ref t) = self.eightPtOffset3 { self.reflectionEffect.load(t.save()); }
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: default:
            _ => {
                // Java: this.reflectionEffect.close();
                self.reflectionEffect.close();
                // Java: this.reflectionEffect = ReflectionEffect.create();
                self.reflectionEffect = ReflectionEffect::create();
            }
        }
    }
    // Java: int presetType()
    pub fn presetType(&mut self) -> i32 {
        // Java: this.intPresetType();
        self.intPresetType();
        // Java: if (isEqualReflectionEffect(this.reflectionEffect, this.touch1))
        // LLM: isEqualReflectionEffect depends on KProto comparison chain; entire if-else aggregated
        // Java: return 0;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualReflectionEffect(this.reflectionEffect, this.touch2))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 1;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualReflectionEffect(this.reflectionEffect, this.touch3))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 2;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualReflectionEffect(this.reflectionEffect, this.fourPtOffset1))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 3;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualReflectionEffect(this.reflectionEffect, this.fourPtOffset2))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 4;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualReflectionEffect(this.reflectionEffect, this.fourPtOffset3))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 5;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualReflectionEffect(this.reflectionEffect, this.eightPtOffset1))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 6;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualReflectionEffect(this.reflectionEffect, this.eightPtOffset2))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 7;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return isEqualReflectionEffect(this.reflectionEffect, this.eightPtOffset3) ? 8 : -1;
        todo!()
    }
    // Java: void setAlpha(int alpha)
    pub fn setAlpha(&mut self, alpha: i32) {
        // Java: this.reflectionEffect.putStA(UnitsConverter.percent2decimals((double)(100 - alpha)));
        self.reflectionEffect.putStA(UnitsConverter::percent2decimals((100 - alpha) as f64));
        // Java: this.setDefaultValue();
        self.setDefaultValue();
    }
    // Java: int alpha()
    pub fn alpha(&self) -> i32 {
        // Java: return (int)UnitsConverter.decimals2percent((double)1.0F - this.reflectionEffect.stA());
        UnitsConverter::decimals2percent(1.0 - self.reflectionEffect.stA()) as i32
    }
    // Java: void setSize(double size)
    pub fn setSize(&mut self, size: f64) {
        // Java: this.reflectionEffect.putEndPos(UnitsConverter.percent2decimals(size));
        self.reflectionEffect.putEndPos(UnitsConverter::percent2decimals(size));
        // Java: this.setDefaultValue();
        self.setDefaultValue();
    }
    // Java: double size()
    pub fn size(&mut self) -> f64 {
        // Java: int type = this.presetType();
        // LLM: type is a Rust keyword, variable renamed to t; t is declared above as let t
        let t = self.presetType();
        // Java: if (type != 7 && type != 4)
        if t != 7 && t != 4 {
            // Java: return type == 3 ? (double)38.0F : UnitsConverter.decimals2percent(this.reflectionEffect.endPos());
            // LLM: t is declared above as let t
            if t == 3 { 38.0 } else { UnitsConverter::decimals2percent(self.reflectionEffect.endPos()) }
        }
        // Java: else
        else {
            // Java: return (double)56.0F;
            56.0
        }
    }
    // Java: void setBlur(double blur)
    pub fn setBlur(&mut self, blur: f64) {
        // Java: this.reflectionEffect.putBlurRadius((int)UnitsConverter.pt2emu(blur));
        self.reflectionEffect.putBlurRadius(UnitsConverter::pt2emu(blur) as i32);
        // Java: this.setDefaultValue();
        self.setDefaultValue();
    }
    // Java: double blur()
    pub fn blur(&self) -> f64 {
        // Java: return UnitsConverter.emu2pt((double)this.reflectionEffect.blurRadius());
        UnitsConverter::emu2pt(self.reflectionEffect.blurRadius() as f64)
    }
    // Java: void setDistance(double distance)
    pub fn setDistance(&mut self, distance: f64) {
        // Java: this.reflectionEffect.putDistance((int)UnitsConverter.pt2emu(distance));
        self.reflectionEffect.putDistance(UnitsConverter::pt2emu(distance) as i32);
        // Java: this.setDefaultValue();
        self.setDefaultValue();
    }
    // Java: double distance()
    pub fn distance(&self) -> f64 {
        // Java: return UnitsConverter.emu2pt((double)this.reflectionEffect.distance());
        UnitsConverter::emu2pt(self.reflectionEffect.distance() as f64)
    }
    // Java: void setDefaultValue()
    fn setDefaultValue(&mut self) {
        // Java: this.reflectionEffect.putSy((double)-1.0F);
        self.reflectionEffect.putSy(-1.0);
        // Java: this.reflectionEffect.putAlgn(1);
        self.reflectionEffect.putAlgn(1);
        // Java: this.reflectionEffect.putRotateWithShape(false);
        self.reflectionEffect.putRotateWithShape(false);
        // Java: this.reflectionEffect.putDirection(UnitsConverter.degree2Angle((double)90.0F));
        self.reflectionEffect.putDirection(UnitsConverter::degree2Angle(90.0));
    }
    // Java: void intPresetType()
    fn intPresetType(&mut self) {
        // Java: if (this.touch1 == null)
        if self.touch1.is_none() {
            // Java: this.touch1 = ReflectionEffect.create();
            self.touch1 = Some(ReflectionEffect::create());
            // Java: this.touch2 = ReflectionEffect.create();
            self.touch2 = Some(ReflectionEffect::create());
            // Java: this.touch3 = ReflectionEffect.create();
            self.touch3 = Some(ReflectionEffect::create());
            // Java: this.fourPtOffset1 = ReflectionEffect.create();
            self.fourPtOffset1 = Some(ReflectionEffect::create());
            // Java: this.fourPtOffset2 = ReflectionEffect.create();
            self.fourPtOffset2 = Some(ReflectionEffect::create());
            // Java: this.fourPtOffset3 = ReflectionEffect.create();
            self.fourPtOffset3 = Some(ReflectionEffect::create());
            // Java: this.eightPtOffset1 = ReflectionEffect.create();
            self.eightPtOffset1 = Some(ReflectionEffect::create());
            // Java: this.eightPtOffset2 = ReflectionEffect.create();
            self.eightPtOffset2 = Some(ReflectionEffect::create());
            // Java: this.eightPtOffset3 = ReflectionEffect.create();
            self.eightPtOffset3 = Some(ReflectionEffect::create());
            // Java: this.initTouch1();
            self.initTouch1();
            // Java: this.iniTouch2();
            self.iniTouch2();
            // Java: this.iniTouch3();
            self.iniTouch3();
            // Java: this.iniFourPtOffset1();
            self.iniFourPtOffset1();
            // Java: this.iniFourPtOffset2();
            self.iniFourPtOffset2();
            // Java: this.iniFourPtOffset3();
            self.iniFourPtOffset3();
            // Java: this.iniEightOffset1();
            self.iniEightOffset1();
            // Java: this.iniEightOffset2();
            self.iniEightOffset2();
            // Java: this.iniEightOffset3();
            self.iniEightOffset3();
        }
    }
    // Java: void iniEightOffset3()
    fn iniEightOffset3(&mut self) {
        // Java: this.eightPtOffset3.load(this.eightPtOffset1.save());
        // LLM: Option fields; KProto chain; entire method aggregated
        // Java: this.eightPtOffset3.putEndA(0.00295);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.eightPtOffset3.putEndPos(0.92);
        todo!()
    }
    // Java: void iniEightOffset2()
    fn iniEightOffset2(&mut self) {
        // Java: this.eightPtOffset2.load(this.eightPtOffset1.save());
        // LLM: Option fields; KProto chain; entire method aggregated
        // Java: this.eightPtOffset2.putEndA(0.003);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.eightPtOffset2.putEndPos(0.555);
        todo!()
    }
    // Java: void iniEightOffset1()
    fn iniEightOffset1(&mut self) {
        // Java: this.eightPtOffset1.load(this.fourPtOffset1.save());
        // LLM: Option fields; KProto chain; entire method aggregated
        // Java: this.eightPtOffset1.putEndA(0.00275);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.eightPtOffset1.putEndPos(0.4);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.eightPtOffset1.putDistance((int)UnitsConverter.pt2emu((double)8.0F));
        todo!()
    }
    // Java: void iniFourPtOffset3()
    fn iniFourPtOffset3(&mut self) {
        // Java: this.fourPtOffset3.load(this.fourPtOffset1.save());
        // LLM: Option fields; KProto chain; entire method aggregated
        // Java: this.fourPtOffset3.putEndPos(0.9);
        todo!()
    }
    // Java: void iniFourPtOffset2()
    fn iniFourPtOffset2(&mut self) {
        // Java: this.fourPtOffset2.load(this.fourPtOffset1.save());
        // LLM: Option fields; KProto chain; entire method aggregated
        // Java: this.fourPtOffset2.putEndPos(0.555);
        todo!()
    }
    // Java: void iniFourPtOffset1()
    fn iniFourPtOffset1(&mut self) {
        // Java: this.fourPtOffset1.putBlurRadius((int)UnitsConverter.pt2emu((double)0.5F));
        // LLM: Option fields; KProto chain; entire method aggregated
        // Java: this.fourPtOffset1.putStA((double)0.5F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.fourPtOffset1.putEndA(0.003);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.fourPtOffset1.putEndPos(0.385);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.fourPtOffset1.putDistance((int)UnitsConverter.pt2emu((double)4.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.fourPtOffset1.putDirection(UnitsConverter.degree2Angle((double)90.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.fourPtOffset1.putSy((double)-1.0F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.fourPtOffset1.putAlgn(1);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.fourPtOffset1.putRotateWithShape(false);
        todo!()
    }
    // Java: void iniTouch3()
    fn iniTouch3(&mut self) {
        // Java: this.touch3.load(this.touch1.save());
        // LLM: Option fields; KProto chain; entire method aggregated
        // Java: this.touch3.putStA((double)0.5F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.touch3.putEndPos(0.9);
        todo!()
    }
    // Java: void iniTouch2()
    fn iniTouch2(&mut self) {
        // Java: this.touch2.load(this.touch1.save());
        // LLM: Option fields; KProto chain; entire method aggregated
        // Java: this.touch2.putStA((double)0.5F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.touch2.putEndPos(0.55);
        todo!()
    }
    // Java: void initTouch1()
    fn initTouch1(&mut self) {
        // Java: this.touch1.putBlurRadius((int)UnitsConverter.pt2emu((double)0.5F));
        // LLM: Option fields; KProto chain; entire method aggregated
        // Java: this.touch1.putStA(0.52);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.touch1.putEndA(0.003);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.touch1.putEndPos(0.35);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.touch1.putDirection(UnitsConverter.degree2Angle((double)90.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.touch1.putDistance(0);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.touch1.putSy((double)-1.0F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.touch1.putAlgn(1);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.touch1.putRotateWithShape(false);
        todo!()
    }
    // Java: boolean isEqualReflectionEffect(ReflectionEffect effect1, ReflectionEffect effect2)
    fn isEqualReflectionEffect(effect1: &ReflectionEffect, effect2: &ReflectionEffect) -> bool {
        // Java: boolean isEqual = effect1.blurRadius() == effect2.blurRadius() && effect1.stA() == effect2.stA() && effect1.endA() == effect2.endA() && effect1.endPos() == effect2.endPos() && effect1.distance() == effect2.distance() && effect1.direction() == effect2.direction() && effect1.sy() == effect2.sy() && effect1.algn() == effect2.algn() && effect1.isRotateWithShape() == effect2.isRotateWithShape();
        let isEqual = effect1.blurRadius() == effect2.blurRadius()
            && effect1.stA() == effect2.stA()
            && effect1.endA() == effect2.endA()
            && effect1.endPos() == effect2.endPos()
            && effect1.distance() == effect2.distance()
            && effect1.direction() == effect2.direction()
            && effect1.sy() == effect2.sy()
            && effect1.algn() == effect2.algn()
            && effect1.isRotateWithShape() == effect2.isRotateWithShape();
        // Java: return isEqual;
        isEqual
    }
}

impl KmoProtocol for DgEffectReflection {
    fn protocol(&self) -> Box<dyn crate::cn::wps::kso::kproto::KoBin::KoBin> { todo!() }
    fn updateProtocol(&mut self, _var1: &dyn crate::cn::wps::kso::kproto::KoBin::KoBin) { todo!() }
}
