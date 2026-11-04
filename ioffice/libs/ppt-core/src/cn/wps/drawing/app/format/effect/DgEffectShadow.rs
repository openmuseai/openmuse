#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]
// Java: import cn.wps.drawing.app.KmoProtocol
use crate::cn::wps::drawing::app::KmoProtocol::KmoProtocol;
// Java: import cn.wps.drawing.app.format.color.DgColorFormat
use crate::cn::wps::drawing::app::format::color::DgColorFormat::DgColorFormat;
// Java: import cn.wps.drawing.app.util.PrecisionUtil
// LLM: PrecisionUtil not yet available, using stub
// Java: import cn.wps.drawing.app.util.UnitsConverter
// LLM: UnitsConverter not yet available, using stub
// Java: import cn.wps.drawing.data.color.Color
use crate::cn::wps::drawing::data::color::Color::Color;
// Java: import cn.wps.drawing.data.color.ColorTransform
use crate::cn::wps::drawing::data::color::ColorTransform::ColorTransform;
// Java: import cn.wps.drawing.data.effect.InnerShadowEffect
// LLM: InnerShadowEffect is a KProto type, using stub
// Java: import cn.wps.drawing.data.effect.OuterShadowEffect
// LLM: OuterShadowEffect is a KProto type, using stub
// Java: import cn.wps.kso.kproto.KoBin
// LLM: KoBin stub

pub struct OuterShadowEffect;
impl OuterShadowEffect {
    pub fn create() -> Self { OuterShadowEffect }
    pub fn save(&self) -> KoBin { KoBin }
    pub fn load(&mut self, _bin: KoBin) {}
    pub fn close(&mut self) {}
    pub fn clr(&mut self) -> &mut Color { todo!() }
    pub fn writeClr(&mut self, _c: Color) {}
    pub fn blurRadius(&self) -> i32 { 0 }
    pub fn putBlurRadius(&mut self, _v: i32) {}
    pub fn direction(&self) -> i32 { 0 }
    pub fn putDirection(&mut self, _v: i32) {}
    pub fn distance(&self) -> i32 { 0 }
    pub fn putDistance(&mut self, _v: i32) {}
    pub fn sx(&self) -> f64 { 1.0 }
    pub fn putSx(&mut self, _v: f64) {}
    pub fn sy(&self) -> f64 { 1.0 }
    pub fn putSy(&mut self, _v: f64) {}
    pub fn algn(&self) -> i32 { 0 }
    pub fn putAlgn(&mut self, _v: i32) {}
    pub fn isRotateWithShape(&self) -> bool { false }
    pub fn putRotateWithShape(&mut self, _v: bool) {}
    pub fn kx(&self) -> i32 { 0 }
    pub fn putKx(&mut self, _v: i32) {}
}
pub struct InnerShadowEffect;
impl InnerShadowEffect {
    pub fn create() -> Self { InnerShadowEffect }
    pub fn save(&self) -> KoBin { KoBin }
    pub fn load(&mut self, _bin: KoBin) {}
    pub fn close(&mut self) {}
    pub fn clr(&mut self) -> &mut Color { todo!() }
    pub fn writeClr(&mut self, _c: Color) {}
    pub fn radius(&self) -> i32 { 0 }
    pub fn putRadius(&mut self, _v: i32) {}
    pub fn direction(&self) -> i32 { 0 }
    pub fn putDirection(&mut self, _v: i32) {}
    pub fn distance(&self) -> i32 { 0 }
    pub fn putDistance(&mut self, _v: i32) {}
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
pub struct PrecisionUtil;
impl PrecisionUtil {
    pub fn equal(a: f64, b: f64) -> bool { (a - b).abs() < 1e-6 }
}

// Java: public class DgEffectShadow implements KmoProtocol
pub struct DgEffectShadow {
    // Java field: int type
    pub(crate) r#type: i32,
    // Java field: OuterShadowEffect outerShadowEffect
    pub(crate) outerShadowEffect: OuterShadowEffect,
    // Java field: InnerShadowEffect innerShadowEffect
    pub(crate) innerShadowEffect: InnerShadowEffect,
    // Java field: DgColorFormat colorFormat
    pub(crate) colorFormat: DgColorFormat,
    // Java field: OuterShadowEffect pTr
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) pTr: Option<OuterShadowEffect>,
    // Java field: OuterShadowEffect pTl
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) pTl: Option<OuterShadowEffect>,
    // Java field: OuterShadowEffect pB
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) pB: Option<OuterShadowEffect>,
    // Java field: OuterShadowEffect pBr
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) pBr: Option<OuterShadowEffect>,
    // Java field: OuterShadowEffect pBl
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) pBl: Option<OuterShadowEffect>,
    // Java field: InnerShadowEffect iTl
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) iTl: Option<InnerShadowEffect>,
    // Java field: InnerShadowEffect iT
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) iT: Option<InnerShadowEffect>,
    // Java field: InnerShadowEffect iTr
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) iTr: Option<InnerShadowEffect>,
    // Java field: InnerShadowEffect iL
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) iL: Option<InnerShadowEffect>,
    // Java field: InnerShadowEffect iCtr
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) iCtr: Option<InnerShadowEffect>,
    // Java field: InnerShadowEffect iR
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) iR: Option<InnerShadowEffect>,
    // Java field: InnerShadowEffect iBl
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) iBl: Option<InnerShadowEffect>,
    // Java field: InnerShadowEffect iB
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) iB: Option<InnerShadowEffect>,
    // Java field: InnerShadowEffect iBr
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) iBr: Option<InnerShadowEffect>,
    // Java field: OuterShadowEffect tl
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) tl: Option<OuterShadowEffect>,
    // Java field: OuterShadowEffect t
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) t: Option<OuterShadowEffect>,
    // Java field: OuterShadowEffect tr
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) tr: Option<OuterShadowEffect>,
    // Java field: OuterShadowEffect l
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) l: Option<OuterShadowEffect>,
    // Java field: OuterShadowEffect ctr
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) ctr: Option<OuterShadowEffect>,
    // Java field: OuterShadowEffect r
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) r: Option<OuterShadowEffect>,
    // Java field: OuterShadowEffect bl
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) bl: Option<OuterShadowEffect>,
    // Java field: OuterShadowEffect b
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) b: Option<OuterShadowEffect>,
    // Java field: OuterShadowEffect br
    // LLM: Java static field; not used at instance level, KProto preset
    pub(crate) br: Option<OuterShadowEffect>,
}

impl DgEffectShadow {
    pub fn DgEffectShadow_new() -> Self {
        DgEffectShadow {
            r#type: 1,
            outerShadowEffect: OuterShadowEffect::create(),
            innerShadowEffect: InnerShadowEffect::create(),
            colorFormat: DgColorFormat::DgColorFormat_new(),
            pTr: None, pTl: None, pB: None, pBr: None, pBl: None,
            iTl: None, iT: None, iTr: None, iL: None, iCtr: None, iR: None,
            iBl: None, iB: None, iBr: None,
            tl: None, t: None, tr: None, l: None, ctr: None, r: None,
            bl: None, b: None, br: None,
        }
    }
    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this.type != 1 ? false : isEqualPerspective(this.outerShadowEffect, OuterShadowEffect.create());
        // LLM: isEqualPerspective depends on KProto comparison chain
        todo!()
    }
    // Java: int type()
    pub fn r#type(&self) -> i32 {
        // Java: return this.type;
        self.r#type
    }
    // Java: void setType(int type)
    pub fn setType(&mut self, r#type: i32) {
        // Java: this.type = type;
        // LLM: self.type mapped to self.r#type because type is a Rust keyword
        self.r#type = r#type;
    }
    // Java: boolean isOuterShadow()
    pub fn isOuterShadow(&self) -> bool {
        // Java: return this.type == 1;
        self.r#type == 1
    }
    // Java: boolean isInnerShadow()
    pub fn isInnerShadow(&self) -> bool {
        // Java: return this.type == 0;
        self.r#type == 0
    }
    // Java: KoBin protocol()
    pub fn protocol_local(&self) -> KoBin {
        // Java: this.writeColor2Protocol();
        // LLM: writeColor2Protocol requires &mut self; KoBin chain
        // Java: return this.isInnerShadow() ? this.innerShadowEffect.save() : this.outerShadowEffect.save();
        if self.isInnerShadow() { self.innerShadowEffect.save() } else { self.outerShadowEffect.save() }
    }
    // Java: void updateProtocol(KoBin data)
    pub fn updateProtocol_local(&mut self, data: KoBin) {
        // Java: if (this.isInnerShadow())
        if self.isInnerShadow() {
            // Java: this.innerShadowEffect.load(data);
            self.innerShadowEffect.load(data);
        }
        // Java: else
        else {
            // Java: this.outerShadowEffect.load(data);
            self.outerShadowEffect.load(data);
            // Java: this.updatePerspectiveType();
            self.updatePerspectiveType();
        }
        // Java: this.iniColor();
        self.iniColor();
    }
    // Java: void updatePerspectiveType()
    fn updatePerspectiveType(&mut self) {
        // Java: if (this.isPerspectiveType())
        if self.isPerspectiveType() {
            // Java: this.setType(2);
            self.setType(2);
        }
    }
    // Java: boolean isPerspectiveType()
    fn isPerspectiveType(&mut self) -> bool {
        // Java: int type = this.perspectiveType();
        // LLM: type is a Rust keyword, variable renamed to t
        let t = self.perspectiveType();
        // Java: return type != -1;
        t != -1
    }
    // Java: void iniColor()
    fn iniColor(&mut self) {
        // Java: if (this.isInnerShadow())
        if self.isInnerShadow() {
            // Java: this.colorFormat.update(this.innerShadowEffect.clr().save());
            // LLM: clr().save() depends on KProto chain
            todo!()
        }
        // Java: else
        else {
            // Java: this.colorFormat.update(this.outerShadowEffect.clr().save());
            // LLM: clr().save() depends on KProto chain
            todo!()
        }
    }
    // Java: void writeColor2Protocol()
    fn writeColor2Protocol(&mut self) {
        // Java: if (this.isInnerShadow())
        if self.isInnerShadow() {
            // Java: this.innerShadowEffect.writeClr(Color.open(this.colorFormat.data()));
            // LLM: Color.open and colorFormat.data() depend on KProto chain
            todo!()
        }
        // Java: else
        else {
            // Java: this.outerShadowEffect.writeClr(Color.open(this.colorFormat.data()));
            // LLM: Color.open and colorFormat.data() depend on KProto chain
            todo!()
        }
    }
    // Java: double blur()
    pub fn blur(&self) -> f64 {
        // Java: int radius;
        let radius: i32;
        // Java: if (this.isInnerShadow())
        if self.isInnerShadow() {
            // Java: radius = this.innerShadowEffect.radius();
            radius = self.innerShadowEffect.radius();
        }
        // Java: else
        else {
            // Java: radius = this.outerShadowEffect.blurRadius();
            radius = self.outerShadowEffect.blurRadius();
        }
        // Java: return UnitsConverter.emu2pt((double)radius);
        UnitsConverter::emu2pt(radius as f64)
    }
    // Java: void setBlur(double blur)
    pub fn setBlur(&mut self, blur: f64) {
        // Java: if (blur > (double)100.0F)
        if blur > 100.0 {
            // Java: throw new IllegalArgumentException("模糊，不能超过100磅滴");
            panic!("IllegalArgumentException");
        }
        // Java: else
        else {
            // Java: int radius = (int)UnitsConverter.pt2emu(blur);
            let radius = UnitsConverter::pt2emu(blur) as i32;
            // Java: if (this.isInnerShadow())
            if self.isInnerShadow() {
                // Java: this.innerShadowEffect.putRadius(radius);
                self.innerShadowEffect.putRadius(radius);
            }
            // Java: else
            else {
                // Java: this.outerShadowEffect.putBlurRadius(radius);
                self.outerShadowEffect.putBlurRadius(radius);
            }
        }
    }
    // Java: void setAngle(int angle)
    pub fn setAngle(&mut self, angle: i32) {
        // Java: assert angle >= 0;
        assert!(angle >= 0);
        // Java: if (angle >= 360)
        if angle >= 360 {
            // Java: throw new IllegalArgumentException();
            panic!("IllegalArgumentException");
        }
        // Java: else
        else {
            // Java: if (this.isInnerShadow())
            if self.isInnerShadow() {
                // Java: this.innerShadowEffect.putDirection(UnitsConverter.degree2Angle((double)angle));
                self.innerShadowEffect.putDirection(UnitsConverter::degree2Angle(angle as f64));
            }
            // Java: else
            else {
                // Java: this.outerShadowEffect.putDirection(UnitsConverter.degree2Angle((double)angle));
                self.outerShadowEffect.putDirection(UnitsConverter::degree2Angle(angle as f64));
            }
        }
    }
    // Java: int angle()
    pub fn angle(&self) -> i32 {
        // Java: return this.isInnerShadow() ? (int)UnitsConverter.angle2Degree((double)this.innerShadowEffect.direction()) : (int)UnitsConverter.angle2Degree((double)this.outerShadowEffect.direction());
        if self.isInnerShadow() { UnitsConverter::angle2Degree(self.innerShadowEffect.direction() as f64) as i32 } else { UnitsConverter::angle2Degree(self.outerShadowEffect.direction() as f64) as i32 }
    }
    // Java: void setDistance(double distance)
    pub fn setDistance(&mut self, distance: f64) {
        // Java: if (distance > (double)200.0F)
        if distance > 200.0 {
            // Java: throw new IllegalArgumentException("不能超过200磅滴");
            panic!("IllegalArgumentException");
        }
        // Java: else
        else {
            // Java: if (this.isInnerShadow())
            if self.isInnerShadow() {
                // Java: this.innerShadowEffect.putDistance((int)UnitsConverter.pt2emu(distance));
                self.innerShadowEffect.putDistance(UnitsConverter::pt2emu(distance) as i32);
            }
            // Java: else
            else {
                // Java: this.outerShadowEffect.putDistance((int)UnitsConverter.pt2emu(distance));
                self.outerShadowEffect.putDistance(UnitsConverter::pt2emu(distance) as i32);
            }
        }
    }
    // Java: double distance()
    pub fn distance(&self) -> f64 {
        // Java: return this.isInnerShadow() ? UnitsConverter.emu2pt((double)this.innerShadowEffect.distance()) : UnitsConverter.emu2pt((double)this.outerShadowEffect.distance());
        if self.isInnerShadow() { UnitsConverter::emu2pt(self.innerShadowEffect.distance() as f64) } else { UnitsConverter::emu2pt(self.outerShadowEffect.distance() as f64) }
    }
    // Java: void setSize(int size)
    pub fn setSize(&mut self, size: i32) {
        // Java: if (size >= 1 && size <= 200)
        if size >= 1 && size <= 200 {
            // Java: if (this.isInnerShadow())
            if self.isInnerShadow() {
                // Java: throw new IllegalStateException("内部阴影木有大小可以设置的");
                panic!("IllegalStateException");
            }
            // Java: else
            else {
                // Java: this.outerShadowEffect.putSx(UnitsConverter.percent2decimals((double)size));
                self.outerShadowEffect.putSx(UnitsConverter::percent2decimals(size as f64));
                // Java: this.outerShadowEffect.putSy(UnitsConverter.percent2decimals((double)size));
                self.outerShadowEffect.putSy(UnitsConverter::percent2decimals(size as f64));
            }
        }
        // Java: else
        else {
            // Java: throw new IllegalArgumentException("最小是1%,最大是200%哈");
            panic!("IllegalArgumentException");
        }
    }
    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return (int)UnitsConverter.decimals2percent(this.outerShadowEffect.sx());
        UnitsConverter::decimals2percent(self.outerShadowEffect.sx()) as i32
    }
    // Java: void setAlpha(int alpha)
    pub fn setAlpha(&mut self, alpha: i32) {
        // Java: this.colorFormat.putAlpha(UnitsConverter.percent2decimals((double)alpha));
        self.colorFormat.putAlpha(UnitsConverter::percent2decimals(alpha as f64));
        // Java: this.writeColor2Protocol();
        self.writeColor2Protocol();
    }
    // Java: int alpha()
    pub fn alpha(&mut self) -> i32 {
        // Java: return (int)UnitsConverter.decimals2percent(this.colorFormat.alpha());
        UnitsConverter::decimals2percent(self.colorFormat.alpha()) as i32
    }
    // Java: void setColorFormat(DgColorFormat format)
    pub fn setColorFormat(&mut self, format: DgColorFormat) {
        // Java: this.colorFormat = format;
        self.colorFormat = format;
        // Java: this.writeColor2Protocol();
        self.writeColor2Protocol();
    }
    // Java: DgColorFormat colorFormat()
    pub fn colorFormat(&self) -> &DgColorFormat {
        // Java: return this.colorFormat;
        &self.colorFormat
    }
    // Java: void setPresetType(int type, int position)
    pub fn setPresetType(&mut self, r#type: i32, position: i32) {
        // Java: this.type = type;
        // LLM: self.type mapped to self.r#type because type is a Rust keyword
        self.r#type = r#type;
        // Java: if (type == 1)
        if r#type == 1 {
            // Java: this.setPresetOuterShadow(position);
            self.setPresetOuterShadow(position);
        }
        // Java: else if (type == 0)
        else if r#type == 0 {
            // Java: this.setPresetInnerShadow(position);
            self.setPresetInnerShadow(position);
        }
        // Java: else if (type == 2)
        else if r#type == 2 {
            // Java: this.setPerspectiveType(position);
            self.setPerspectiveType(position);
        }
        // Java: else if (type == -1)
        else if r#type == -1 {
            // Java: this.outerShadowEffect.close();
            self.outerShadowEffect.close();
            // Java: this.innerShadowEffect.close();
            self.innerShadowEffect.close();
            // Java: this.outerShadowEffect = OuterShadowEffect.create();
            self.outerShadowEffect = OuterShadowEffect::create();
            // Java: this.innerShadowEffect = InnerShadowEffect.create();
            self.innerShadowEffect = InnerShadowEffect::create();
        }
    }
    // Java: int presetType()
    pub fn presetType(&mut self) -> i32 {
        // Java: if (this.type == 1)
        if self.r#type == 1 {
            // Java: return this.presetOuterShadow();
            return self.presetOuterShadow();
        }
        // Java: else if (this.type == 0)
        else if self.r#type == 0 {
            // Java: return this.presetInnerShadow();
            return self.presetInnerShadow();
        }
        // Java: else
        else {
            // Java: return this.type == 2 ? this.perspectiveType() : -1;
            return if self.r#type == 2 { self.perspectiveType() } else { -1 };
        }
    }
    // Java: int presetOuterShadow()
    fn presetOuterShadow(&mut self) -> i32 {
        // Java: iniPresetShadow();
        // LLM: iniPresetShadow() and isEqualOuterShadow depend on KProto chain; entire method aggregated
        // Java: if (isEqualOuterShadow(this.outerShadowEffect, tl))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 2;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualOuterShadow(this.outerShadowEffect, t))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 0;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualOuterShadow(this.outerShadowEffect, tr))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 1;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualOuterShadow(this.outerShadowEffect, l))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 5;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualOuterShadow(this.outerShadowEffect, ctr))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 3;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualOuterShadow(this.outerShadowEffect, r))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 4;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualOuterShadow(this.outerShadowEffect, bl))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 8;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualOuterShadow(this.outerShadowEffect, br))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 7;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return isEqualOuterShadow(this.outerShadowEffect, b) ? 6 : -1;
        todo!()
    }
    // Java: void setPresetOuterShadow(int position)
    fn setPresetOuterShadow(&mut self, position: i32) {
        // Java: iniPresetShadow();
        // LLM: iniPresetShadow() and static preset variables depend on KProto chain; entire method aggregated
        // Java: switch (position)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 0:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(t.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 1:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(tr.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 2:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(tl.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 3:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(ctr.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 4:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(r.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 5:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(l.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 6:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: default:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(b.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 7:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(br.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 8:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(bl.save());
        // LLM: self.updateProtocol mapped to updateProtocol_local; KProto chain, entire method aggregated
        todo!()
    }
    // Java: int presetInnerShadow()
    fn presetInnerShadow(&mut self) -> i32 {
        // Java: iniPresetShadow();
        // LLM: iniPresetShadow() and isEqualInnerShadow depend on KProto chain; entire method aggregated
        // Java: if (isEqualInnerShadow(this.innerShadowEffect, iTl))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 7;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualInnerShadow(this.innerShadowEffect, iT))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 6;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualInnerShadow(this.innerShadowEffect, iTr))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 8;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualInnerShadow(this.innerShadowEffect, iL))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 4;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualInnerShadow(this.innerShadowEffect, iCtr))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 3;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualInnerShadow(this.innerShadowEffect, iR))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 5;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualInnerShadow(this.innerShadowEffect, iBl))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 1;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualInnerShadow(this.innerShadowEffect, iB))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 0;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return isEqualInnerShadow(this.innerShadowEffect, iBr) ? 2 : -1;
        todo!()
    }
    // Java: void setPresetInnerShadow(int position)
    fn setPresetInnerShadow(&mut self, position: i32) {
        // Java: iniPresetShadow();
        // LLM: all static preset initialization depends on KProto; entire method aggregated
        // Java: switch (position)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 0:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(iB.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 1:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(iBl.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 2:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(iBr.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 3:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(iCtr.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 4:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(iL.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 5:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(iR.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 6:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(iT.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 7:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(iTl.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 8:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(iTr.save());
        // LLM: self.updateProtocol mapped to updateProtocol_local; KProto chain, entire method aggregated
        todo!()
    }
    // Java: void setPerspectiveType(int type)
    fn setPerspectiveType(&mut self, r#type: i32) {
        // Java: iniPresetShadow();
        // LLM: all perspective preset depends on KProto; entire method aggregated
        // Java: switch (type)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 0:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(pB.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 1:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(pBr.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 2:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(pBl.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 3:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 4:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 5:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 6:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: default:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 7:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(pTr.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: break;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 8:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.updateProtocol(pTl.save());
        // LLM: self.updateProtocol mapped to updateProtocol_local; KProto chain, entire method aggregated
        todo!()
    }
    // Java: int perspectiveType()
    fn perspectiveType(&mut self) -> i32 {
        // Java: iniPresetShadow();
        // LLM: iniPresetShadow() and isEqualPerspective depend on KProto chain; entire method aggregated
        // Java: if (isEqualPerspective(this.outerShadowEffect, pTr))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 7;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualPerspective(this.outerShadowEffect, pTl))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 8;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualPerspective(this.outerShadowEffect, pB))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 0;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (isEqualPerspective(this.outerShadowEffect, pBr))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 1;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return isEqualPerspective(this.outerShadowEffect, pBl) ? 2 : -1;
        todo!()
    }
    // Java: void iniPresetShadow()
    fn iniPresetShadow() {
        // Java: if (tl == null)
        // LLM: all static preset initialization depends on KProto; entire method aggregated
        // Java: intOuterShadowModel();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (iTl == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniInnerShadowModel();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (pTr == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniPerspectiveModel();
        // LLM: static presets not yet initialized; todo
    }
    // Java: boolean isEqualPerspective(OuterShadowEffect effect1, OuterShadowEffect effect2)
    fn isEqualPerspective(effect1: &OuterShadowEffect, effect2: &OuterShadowEffect) -> bool {
        // Java: boolean isEqual = isEqualOuterShadow(effect1, effect2);
        let isEqual = DgEffectShadow::isEqualOuterShadow(effect1, effect2);
        // Java: boolean isEqualOther = effect1.kx() == effect2.kx();
        let isEqualOther = effect1.kx() == effect2.kx();
        // Java: return isEqual && isEqualOther;
        isEqual && isEqualOther
    }
    // Java: boolean isEqualOuterShadow(OuterShadowEffect effect1, OuterShadowEffect effect2)
    fn isEqualOuterShadow(effect1: &OuterShadowEffect, effect2: &OuterShadowEffect) -> bool {
        // Java: boolean isEqual = effect1.blurRadius() == effect2.blurRadius() && effect1.distance() == effect2.distance() && effect1.direction() == effect2.direction() && PrecisionUtil.equal(effect1.sx(), effect2.sx()) && PrecisionUtil.equal(effect1.sy(), effect2.sy()) && effect1.algn() == effect2.algn() && effect1.isRotateWithShape() == effect2.isRotateWithShape();
        let isEqual = effect1.blurRadius() == effect2.blurRadius()
            && effect1.distance() == effect2.distance()
            && effect1.direction() == effect2.direction()
            && PrecisionUtil::equal(effect1.sx(), effect2.sx())
            && PrecisionUtil::equal(effect1.sy(), effect2.sy())
            && effect1.algn() == effect2.algn()
            && effect1.isRotateWithShape() == effect2.isRotateWithShape();
        // Java: return isEqual && isEqualColor(effect1.clr(), effect2.clr());
        // LLM: clr() requires mutable access; KProto chain
        todo!()
    }
    // Java: boolean isEqualInnerShadow(InnerShadowEffect effect1, InnerShadowEffect effect2)
    fn isEqualInnerShadow(effect1: &InnerShadowEffect, effect2: &InnerShadowEffect) -> bool {
        // Java: boolean isEqual = effect1.radius() == effect2.radius() && effect1.direction() == effect2.direction() && effect1.distance() == effect2.distance();
        let isEqual = effect1.radius() == effect2.radius()
            && effect1.direction() == effect2.direction()
            && effect1.distance() == effect2.distance();
        // Java: return isEqual && isEqualColor(effect1.clr(), effect2.clr());
        // LLM: clr() requires mutable access; KProto chain
        todo!()
    }
    // Java: boolean isEqualColor(Color color1, Color color2)
    fn isEqualColor(color1: &mut Color, color2: &mut Color) -> bool {
        // Java: boolean isEqualColor = color1.mode() == color2.mode() && color1.prst() == color2.prst();
        // LLM: Color.mode() and Color.prst() depend on KProto chain; entire method aggregated
        // Java: ColorTransform transform1 = DgColorFormat.getColorTransformByType(color1, 0);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: ColorTransform transform2 = DgColorFormat.getColorTransformByType(color2, 0);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return isEqualColor && transform1.alpha() == transform2.alpha();
        todo!()
    }
    // Java: void clear()
    pub fn clear() {
        // Java: pTr = null;
        // LLM: static preset fields not yet implemented; entire method aggregated
        // Java: pTl = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pB = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBr = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBl = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iTl = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iT = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iTr = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iL = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iCtr = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iR = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iBl = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iB = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iBr = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: tl = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: t = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: tr = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: l = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: ctr = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: r = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: bl = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: b = null;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: br = null;
        // LLM: static presets not yet implemented
    }
    // Java: void iniPerspectiveModel()
    fn iniPerspectiveModel() {
        // Java: pTr = OuterShadowEffect.create();
        // LLM: all static preset initialization depends on KProto chain; entire method aggregated
        // Java: pTl = OuterShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pB = OuterShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBr = OuterShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBl = OuterShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: Color color = iniShadowColor(UnitsConverter.percent2decimals((double)20.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: color.save();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniPTr(color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniPTl(color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniPB();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniPBr(color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniPBL(color);
        todo!()
    }
    // Java: void iniPTr(Color color)
    fn iniPTr(_color: Color) {
        // Java: pTr.putRotateWithShape(false);
        // LLM: pTr is a static field; KProto chain; entire method aggregated
        // Java: pTr.putBlurRadius((int)UnitsConverter.pt2emu((double)6.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pTr.putDirection(UnitsConverter.degree2Angle((double)225.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pTr.putDistance((int)UnitsConverter.pt2emu((double)0.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pTr.putSy(0.23);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pTr.putSx((double)1.0F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pTr.putKx(UnitsConverter.degree2Angle((double)20.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pTr.putAlgn(2);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pTr.writeClr(color);
        todo!()
    }
    // Java: void iniPTl(Color color)
    fn iniPTl(_color: Color) {
        // Java: pTl.putRotateWithShape(false);
        // LLM: pTl is a static field; KProto chain; entire method aggregated
        // Java: pTl.putBlurRadius((int)UnitsConverter.pt2emu((double)6.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pTl.putDirection(UnitsConverter.degree2Angle((double)315.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pTl.putDistance((int)UnitsConverter.pt2emu((double)0.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pTl.putSy(0.23);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pTl.putSx((double)1.0F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pTl.putKx(UnitsConverter.degree2Angle((double)-20.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pTl.putAlgn(1);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pTl.writeClr(color);
        todo!()
    }
    // Java: void iniPB()
    fn iniPB() {
        // Java: pB.putRotateWithShape(false);
        // LLM: pB is a static field; KProto chain; entire method aggregated
        // Java: pB.putBlurRadius((int)UnitsConverter.pt2emu((double)12.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pB.putDirection(UnitsConverter.degree2Angle((double)90.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pB.putDistance((int)UnitsConverter.pt2emu((double)25.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pB.putSx(0.9);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pB.putSy(-0.19);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: Color color = iniShadowColor(UnitsConverter.percent2decimals((double)15.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: color.save();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pB.writeClr(color);
        todo!()
    }
    // Java: void iniPBr(Color color)
    fn iniPBr(_color: Color) {
        // Java: pBr.putRotateWithShape(false);
        // LLM: pBr is a static field; KProto chain; entire method aggregated
        // Java: pBr.putBlurRadius((int)UnitsConverter.pt2emu((double)6.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBr.putDirection(UnitsConverter.degree2Angle((double)135.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBr.putDistance((int)UnitsConverter.pt2emu((double)1.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBr.putSy(-0.23);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBr.putSx((double)1.0F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBr.putKx(UnitsConverter.degree2Angle((double)13.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBr.putAlgn(2);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBr.writeClr(color);
        todo!()
    }
    // Java: void iniPBL(Color color)
    fn iniPBL(_color: Color) {
        // Java: pBl.putRotateWithShape(false);
        // LLM: pBl is a static field; KProto chain; entire method aggregated
        // Java: pBl.putBlurRadius((int)UnitsConverter.pt2emu((double)6.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBl.putDirection(UnitsConverter.degree2Angle((double)45.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBl.putDistance((int)UnitsConverter.pt2emu((double)1.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBl.putSy(-0.23);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBl.putSx((double)1.0F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBl.putKx(UnitsConverter.degree2Angle((double)-13.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBl.putAlgn(1);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: pBl.writeClr(color);
        todo!()
    }
    // Java: void iniInnerShadowModel()
    fn iniInnerShadowModel() {
        // Java: iTl = InnerShadowEffect.create();
        // LLM: all static inner shadow preset initialization depends on KProto chain; entire method aggregated
        // Java: iT = InnerShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iTr = InnerShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iL = InnerShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iCtr = InnerShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iR = InnerShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iBl = InnerShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iB = InnerShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iBr = InnerShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: Color color = iniShadowColor(UnitsConverter.percent2decimals((double)50.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: color.save();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniInnerTl(color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniInnerT();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniInnerTr();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniInnerL();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniInnerCtr();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniInnerR();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniInnerBl();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniInnerB();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniInnerBr();
        todo!()
    }
    // Java: void iniInnerBr()
    fn iniInnerBr() {
        // Java: iBr.load(iTl.save());
        // LLM: static field; KProto chain; entire method aggregated
        // Java: iBr.putDirection(UnitsConverter.degree2Angle((double)45.0F));
        todo!()
    }
    // Java: void iniInnerB()
    fn iniInnerB() {
        // Java: iB.load(iTl.save());
        // LLM: static field; KProto chain; entire method aggregated
        // Java: iB.putDirection(UnitsConverter.degree2Angle((double)90.0F));
        todo!()
    }
    // Java: void iniInnerBl()
    fn iniInnerBl() {
        // Java: iBl.load(iTl.save());
        // LLM: static field; KProto chain; entire method aggregated
        // Java: iBl.putDirection(UnitsConverter.degree2Angle((double)135.0F));
        todo!()
    }
    // Java: void iniInnerR()
    fn iniInnerR() {
        // Java: iR.load(iTl.save());
        // LLM: static field; KProto chain; entire method aggregated
        // Java: iR.putDirection(UnitsConverter.degree2Angle((double)0.0F));
        todo!()
    }
    // Java: void iniInnerCtr()
    fn iniInnerCtr() {
        // Java: iCtr.putRadius((int)UnitsConverter.pt2emu((double)9.0F));
        // LLM: static field; KProto chain; entire method aggregated
        // Java: iCtr.putDirection(UnitsConverter.degree2Angle((double)0.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iCtr.putDistance((int)UnitsConverter.pt2emu((double)0.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iCtr.clr().putMode(5);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iCtr.clr().putPrst(7);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: ColorTransform transform = ColorTransform.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: transform.putAlpha((double)1.0F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgColorFormat.setColorTransformByType(iCtr.clr(), transform);
        // LLM: DgColorFormat::setColorTransformByType depends on KProto chain; entire method aggregated
        todo!()
    }
    // Java: void iniInnerL()
    fn iniInnerL() {
        // Java: iL.load(iTl.save());
        // LLM: static field; KProto chain; entire method aggregated
        // Java: iL.putDirection(UnitsConverter.degree2Angle((double)180.0F));
        todo!()
    }
    // Java: void iniInnerTr()
    fn iniInnerTr() {
        // Java: iTr.load(iTl.save());
        // LLM: static field; KProto chain; entire method aggregated
        // Java: iTr.putDirection(UnitsConverter.degree2Angle((double)315.0F));
        todo!()
    }
    // Java: void iniInnerT()
    fn iniInnerT() {
        // Java: iT.load(iTl.save());
        // LLM: static field; KProto chain; entire method aggregated
        // Java: iT.putDirection(UnitsConverter.degree2Angle((double)270.0F));
        todo!()
    }
    // Java: void iniInnerTl(Color color)
    fn iniInnerTl(_color: Color) {
        // Java: iTl.putRadius((int)UnitsConverter.pt2emu((double)5.0F));
        // LLM: static field; KProto chain; entire method aggregated
        // Java: iTl.putDirection(UnitsConverter.degree2Angle((double)225.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iTl.putDistance((int)UnitsConverter.pt2emu((double)4.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iTl.writeClr(color);
        todo!()
    }
    // Java: Color iniShadowColor(double alpha)
    fn iniShadowColor(_alpha: f64) -> Color {
        // Java: Color color = Color.create();
        // LLM: Color.create and KProto chain; entire method aggregated
        // Java: color.putMode(5);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: color.putPrst(7);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: ColorTransform transform = ColorTransform.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: transform.putAlpha(alpha);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: transform.putType(0);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgColorFormat.setColorTransformByType(color, transform);
        // LLM: DgColorFormat::setColorTransformByType depends on KProto chain; entire method aggregated
        // Java: return color;
        todo!()
    }
    // Java: void intOuterShadowModel()
    fn intOuterShadowModel() {
        // Java: tl = OuterShadowEffect.create();
        // LLM: all static outer shadow preset initialization depends on KProto chain; entire method aggregated
        // Java: t = OuterShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: tr = OuterShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: l = OuterShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: ctr = OuterShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: r = OuterShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: bl = OuterShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: b = OuterShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: br = OuterShadowEffect.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: Color color = iniShadowColor(UnitsConverter.percent2decimals((double)40.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: color.save();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniOuterLT(color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniOuterT();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniOuterTR();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniOuterL();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniOuterCTR(color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniOuterR();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniOuterBL();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniOuterB();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: iniOuterBR();
        todo!()
    }
    // Java: void iniOuterBR()
    fn iniOuterBR() {
        // Java: br.load(tl.save());
        // LLM: static field; KProto chain; entire method aggregated
        // Java: br.putDirection(UnitsConverter.degree2Angle((double)225.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: br.putAlgn(2);
        todo!()
    }
    // Java: void iniOuterB()
    fn iniOuterB() {
        // Java: b.load(tl.save());
        // LLM: static field; KProto chain; entire method aggregated
        // Java: b.putDirection(UnitsConverter.degree2Angle((double)270.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: b.putAlgn(0);
        todo!()
    }
    // Java: void iniOuterBL()
    fn iniOuterBL() {
        // Java: bl.load(tl.save());
        // LLM: static field; KProto chain; entire method aggregated
        // Java: bl.putDirection(UnitsConverter.degree2Angle((double)315.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: bl.putAlgn(1);
        todo!()
    }
    // Java: void iniOuterR()
    fn iniOuterR() {
        // Java: r.load(tl.save());
        // LLM: static field; KProto chain; entire method aggregated
        // Java: r.putDirection(UnitsConverter.degree2Angle((double)180.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: r.putAlgn(5);
        todo!()
    }
    // Java: void iniOuterCTR(Color color)
    fn iniOuterCTR(_color: Color) {
        // Java: ctr.putRotateWithShape(false);
        // LLM: static field; KProto chain; entire method aggregated
        // Java: ctr.putBlurRadius((int)UnitsConverter.pt2emu((double)5.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: ctr.putDirection(UnitsConverter.degree2Angle((double)0.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: ctr.putDistance((int)UnitsConverter.pt2emu((double)0.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: ctr.putAlgn(3);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: ctr.putSx(1.02);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: ctr.putSy(1.02);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: ctr.writeClr(color);
        todo!()
    }
    // Java: void iniOuterL()
    fn iniOuterL() {
        // Java: l.load(tl.save());
        // LLM: static field; KProto chain; entire method aggregated
        // Java: l.putDirection(UnitsConverter.degree2Angle((double)0.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: l.putAlgn(4);
        todo!()
    }
    // Java: void iniOuterTR()
    fn iniOuterTR() {
        // Java: tr.load(tl.save());
        // LLM: static field; KProto chain; entire method aggregated
        // Java: tr.putDirection(UnitsConverter.degree2Angle((double)135.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: tr.putAlgn(8);
        todo!()
    }
    // Java: void iniOuterT()
    fn iniOuterT() {
        // Java: t.load(tl.save());
        // LLM: static field; KProto chain; entire method aggregated
        // Java: t.putDirection(UnitsConverter.degree2Angle((double)90.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: t.putAlgn(6);
        todo!()
    }
    // Java: void iniOuterLT(Color color)
    fn iniOuterLT(_color: Color) {
        // Java: tl.putRotateWithShape(false);
        // LLM: static field; KProto chain; entire method aggregated
        // Java: tl.putBlurRadius((int)UnitsConverter.pt2emu((double)4.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: tl.putDirection(UnitsConverter.degree2Angle((double)45.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: tl.putDistance((int)UnitsConverter.pt2emu((double)3.0F));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: tl.putSx((double)1.0F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: tl.putSy((double)1.0F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: tl.putAlgn(7);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: tl.writeClr(color);
        todo!()
    }
}

impl KmoProtocol for DgEffectShadow {
    fn protocol(&self) -> Box<dyn crate::cn::wps::kso::kproto::KoBin::KoBin> { todo!() }
    fn updateProtocol(&mut self, _var1: &dyn crate::cn::wps::kso::kproto::KoBin::KoBin) { todo!() }
}
