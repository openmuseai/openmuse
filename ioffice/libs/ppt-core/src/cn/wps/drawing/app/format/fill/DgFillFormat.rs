#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]
// Java: import cn.wps.drawing.app.KmoProtocol
use crate::cn::wps::drawing::app::KmoProtocol::KmoProtocol;
// Java: import cn.wps.drawing.app.format.color.DgColorFormat
use crate::cn::wps::drawing::app::format::color::DgColorFormat::DgColorFormat;
// Java: import cn.wps.drawing.app.util.UnitsConverter
// LLM: UnitsConverter module not yet available, using stub
// Java: import cn.wps.drawing.data.basic.RelativeRect
// LLM: RelativeRect is a KProto type, method chains use todo!()
// Java: import cn.wps.drawing.data.blip.Blip
// LLM: Blip is a KProto type, method chains use todo!()
// Java: import cn.wps.drawing.data.color.Color
use crate::cn::wps::drawing::data::color::Color::Color;
// Java: import cn.wps.drawing.data.color.ColorTransform
use crate::cn::wps::drawing::data::color::ColorTransform::ColorTransform;
// Java: import cn.wps.drawing.data.effect.Effect
// LLM: Effect is a KProto type, method chains use todo!()
// Java: import cn.wps.drawing.data.fill.Fill
// LLM: Fill is a KProto type, using stub
// Java: import cn.wps.drawing.data.fill.GradientFill
// LLM: GradientFill is a KProto type, method chains use todo!()
// Java: import cn.wps.drawing.data.fill.Shade
// LLM: Shade is a KProto type, method chains use todo!()
// Java: import cn.wps.kso.kproto.KoBin
// LLM: KoBin stub
// Java: import java.util.ArrayList
// LLM: using Vec
// Java: import java.util.List
// LLM: using Vec

pub struct Fill;
impl Fill {
    pub fn new() -> Self { Fill }
    pub fn fill_type(&self) -> i32 { 5 }
    pub fn hasType(&self) -> bool { false }
    pub fn putType(&mut self, _t: i32) {}
    pub fn hasSolidFill(&self) -> bool { false }
    pub fn save(&self) -> KoBin { KoBin }
    pub fn load(&mut self, _bin: KoBin) {}
    pub fn r#type(&self) -> i32 { 5 }
    pub fn writeGradFill(&mut self, _gf: ()) {}
    pub fn gradFill(&mut self) -> GradFillStub { GradFillStub }
    pub fn blipFill(&mut self) -> BlipFillStub { BlipFillStub }
    pub fn pattFill(&mut self) -> PattFillStub { PattFillStub }
    pub fn writeSolidFill(&mut self, _c: Color) {}
    pub fn solidFill(&self) -> Color { Color::Color() }
}
pub struct GradFillStub;
impl GradFillStub {
    pub fn shade(&mut self) -> ShadeStub { ShadeStub }
    pub fn hasShade(&self) -> bool { false }
    pub fn deleteShade(&mut self) {}
    pub fn isRotWithShape(&self) -> bool { false }
    pub fn putRotWithShape(&mut self, _v: bool) {}
    pub fn r#type(&self) -> i32 { 0 }
    pub fn putType(&mut self, _t: i32) {}
    pub fn lin(&mut self) -> LinStub { LinStub }
    pub fn hasLin(&self) -> bool { false }
    pub fn deleteLin(&mut self) {}
    pub fn path(&mut self) -> PathStub { PathStub }
    pub fn hasPath(&self) -> bool { false }
    pub fn deletePath(&mut self) {}
    pub fn save(&self) -> KoBin { KoBin }
    pub fn load(&mut self, _bin: KoBin) {}
    pub fn tileRect(&mut self) -> RectStub { RectStub }
    pub fn forceWriteTileRect(&mut self, _r: RelativeRectStub) {}
}
pub struct RelativeRectStub;
impl RelativeRectStub {
    pub fn create() -> Self { RelativeRectStub }
}
pub struct ShadeStub;
impl ShadeStub {
    pub fn gsLst(&mut self) -> GsLstStub { GsLstStub }
    pub fn lin(&mut self) -> LinStub { LinStub }
    pub fn path(&mut self) -> PathShadeStub { PathShadeStub }
    pub fn deleteLin(&mut self) {}
    pub fn deletePath(&mut self) {}
    pub fn hasLin(&self) -> bool { false }
}
pub struct PathShadeStub;
impl PathShadeStub {
    pub fn putPath(&mut self, _v: i32) {}
    pub fn fillToRect(&mut self) -> RectStub { RectStub }
    pub fn path(&self) -> i32 { 0 }
}
pub struct GsLstStub;
impl GsLstStub {
    pub fn read(&self, _lst: &mut Vec<KoBin>) {}
    pub fn write(&mut self, _lst: Vec<KoBin>) {}
}
pub struct LinStub;
impl LinStub {
    pub fn ang(&self) -> i32 { 0 }
    pub fn putAng(&mut self, _a: i32) {}
    pub fn putScaled(&mut self, _v: bool) {}
    pub fn isScaled(&self) -> bool { false }
}
pub struct PathStub;
impl PathStub {
    pub fn r#type(&self) -> i32 { 0 }
    pub fn putType(&mut self, _t: i32) {}
}
pub struct BlipFillStub;
impl BlipFillStub {
    pub fn blip(&mut self) -> BlipStub { BlipStub }
    pub fn fillMode(&mut self) -> FillModeStub { FillModeStub }
    pub fn isRotWithShape(&self) -> bool { false }
    pub fn putRotWithShape(&mut self, _v: bool) {}
}
pub struct BlipStub;
impl BlipStub {
    pub fn hasEffects(&self) -> bool { false }
    pub fn effects(&mut self) -> EffectsStub { EffectsStub }
    pub fn hasEmbedMediaID(&self) -> bool { false }
    pub fn deleteEmbedMediaID(&mut self) {}
    pub fn putLinkMediaID(&mut self, _id: i32) {}
    pub fn hasLinkMediaID(&self) -> bool { false }
    pub fn deleteLinkMediaID(&mut self) {}
    pub fn putEmbedMediaID(&mut self, _id: i32) {}
    pub fn embedMediaID(&self) -> i32 { -1 }
    pub fn linkMediaID(&self) -> i32 { -1 }
}
pub struct EffectsStub;
impl EffectsStub {
    pub fn read(&self, _lst: &mut Vec<()>) {}
    pub fn write(&mut self, _lst: Vec<()>) {}
}
pub struct FillModeStub;
impl FillModeStub {
    pub fn hasTile(&self) -> bool { false }
    pub fn stretch(&mut self) -> StretchStub { StretchStub }
    pub fn tile(&mut self) -> TileStub { TileStub }
    pub fn deleteStretch(&mut self) {}
    pub fn deleteTile(&mut self) {}
}
pub struct StretchStub;
impl StretchStub {
    pub fn fillRect(&mut self) -> RectStub { RectStub }
}
pub struct RectStub;
impl RectStub {
    pub fn left(&self) -> f64 { 0.0 }
    pub fn putLeft(&mut self, _v: f64) {}
    pub fn right(&self) -> f64 { 0.0 }
    pub fn putRight(&mut self, _v: f64) {}
    pub fn top(&self) -> f64 { 0.0 }
    pub fn putTop(&mut self, _v: f64) {}
    pub fn bottom(&self) -> f64 { 0.0 }
    pub fn putBottom(&mut self, _v: f64) {}
    pub fn hasLeft(&self) -> bool { false }
    pub fn hasRight(&self) -> bool { false }
    pub fn hasTop(&self) -> bool { false }
    pub fn hasBottom(&self) -> bool { false }
    pub fn deleteLeft(&mut self) {}
    pub fn deleteRight(&mut self) {}
    pub fn deleteTop(&mut self) {}
    pub fn deleteBottom(&mut self) {}
}
pub struct TileStub;
impl TileStub {
    pub fn tx(&self) -> i32 { 0 }
    pub fn putTx(&mut self, _v: i32) {}
    pub fn ty(&self) -> i32 { 0 }
    pub fn putTy(&mut self, _v: i32) {}
    pub fn sx(&self) -> f64 { 0.0 }
    pub fn putSx(&mut self, _v: f64) {}
    pub fn sy(&self) -> f64 { 0.0 }
    pub fn putSy(&mut self, _v: f64) {}
    pub fn flip(&self) -> i32 { 0 }
    pub fn putFlip(&mut self, _v: i32) {}
    pub fn algn(&self) -> i32 { 0 }
    pub fn putAlgn(&mut self, _v: i32) {}
}
pub struct PattFillStub;
impl PattFillStub {
    pub fn prst(&self) -> i32 { 0 }
    pub fn putPrst(&mut self, _v: i32) {}
    pub fn fgClr(&self) -> Color { Color::Color() }
    pub fn bgClr(&self) -> Color { Color::Color() }
    pub fn fgClrField(&mut self) -> &mut Color { todo!() }
    pub fn bgClrField(&mut self) -> &mut Color { todo!() }
}
pub struct UnitsConverter;
impl UnitsConverter {
    pub fn decimals2percent(v: f64) -> f64 { v * 100.0 }
    pub fn percent2decimals(v: f64) -> f64 { v / 100.0 }
    pub fn pt2emu(v: f64) -> f64 { v * 12700.0 }
    pub fn emu2pt(v: f64) -> f64 { v / 12700.0 }
    pub fn degree2Angle(v: f64) -> i32 { (v * 60000.0) as i32 }
    pub fn angle2Degree(v: f64) -> f64 { v / 60000.0 }
}
pub struct KoBin;
pub trait Cloneable {}
pub struct GradientStop;
impl GradientStop {
    pub fn new() -> Self { GradientStop }
    pub fn load(&mut self, _gs: KoBin) {}
    pub fn save(&self) -> KoBin { KoBin }
    pub fn putPos(&mut self, _v: f64) {}
    pub fn pos(&self) -> f64 { 0.0 }
    pub fn color(&mut self) -> &mut Color { todo!() }
    pub fn writeColor(&mut self, _c: Color) {}
}

// Java: public class DgFillFormat implements KmoProtocol, Cloneable
pub struct DgFillFormat {
    // Java field: Fill fill
    pub(crate) fill: Fill,
    // Java field: List<Halo> halos
    pub(crate) halos: Vec<Halo>, // LLM: halos is java.util.List<Halo>, using Vec
    // Java field: boolean isNeedTilePicAsTexture
    pub(crate) isNeedTilePicAsTexture: bool,
}
// Java: public class Halo
pub struct Halo {
    // Java field: GradientFill.GradientStop gradientStop
    pub(crate) gradientStop: GradientStop,
    // Java field: DgColorFormat color
    pub(crate) color: DgColorFormat,
}

impl DgFillFormat {
    pub fn DgFillFormat_new() -> Self {
        DgFillFormat { fill: Fill::new(), halos: Vec::new(), isNeedTilePicAsTexture: true }
    }
    // Java: Fill getFill()
    pub fn getFill(&self) -> &Fill {
        // Java: return this.fill;
        &self.fill
    }
    // Java: int type()
    pub fn r#type(&self) -> i32 {
        // Java: return this.fill.type();
        self.fill.r#type()
    }
    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return !this.fill.hasType();
        !self.fill.hasType()
    }
    // Java: void setType(int type)
    pub fn setType(&mut self, r#type: i32) {
        // Java: this.fill.putType(type);
        self.fill.putType(r#type);
    }
    // Java: KoBin protocol()
    pub fn protocol(&self) -> KoBin {
        // Java: this.writeData2Protocol();
        // LLM: writeData2Protocol requires &mut self; calling save directly
        // Java: return this.fill.save();
        self.fill.save()
    }
    // Java: void writeData2Protocol()
    pub fn writeData2Protocol(&mut self) {
        // Java: this.writeGradFillData2Protocol();
        self.writeGradFillData2Protocol();
    }
    // Java: void writeGradFillData2Protocol()
    pub fn writeGradFillData2Protocol(&mut self) {
        // Java: if (this.fill.type() == 4)
        if self.fill.r#type() == 4 {
            // Java: this.fill.writeGradFill(GradientFill.open(this.gradientData()));
            // LLM: GradientFill.open depends on KProto
            todo!()
        }
    }
    // Java: KoBin gradientData()
    pub fn gradientData(&mut self) -> KoBin {
        // Java: List<GradientFill.GradientStop> gsLst = new ArrayList();
        // LLM: GradientFill.GradientStop is KProto type, using KoBin as placeholder
        let mut gsLst: Vec<KoBin> = Vec::new();
        // Java: int i = 0;
        let mut i = 0;
        // Java: for(int size = this.halos.size(); i < size; ++i)
        let size = self.halos.len();
        while i < size {
            // Java: GradientFill.GradientStop gs = GradientFill.GradientStop.open(((Halo)this.halos.get(i)).data());
            // LLM: GradientFill.GradientStop.open depends on KProto
            let gs = self.halos[i].data();
            // Java: gsLst.add(gs);
            gsLst.push(gs);
            i += 1;
        }
        // Java: this.fill.gradFill().gsLst().write(gsLst);
        self.fill.gradFill().shade().gsLst().write(gsLst);
        // Java: return this.fill.gradFill().save();
        self.fill.gradFill().save()
    }
    // Java: void updateProtocol(KoBin fill)
    pub fn updateProtocol(&mut self, fill: KoBin) {
        // Java: this.halos.clear();
        self.halos.clear();
        // Java: this.fill.load(fill);
        self.fill.load(fill);
        // Java: if (this.fill.type() == 4)
        if self.fill.r#type() == 4 {
            // Java: this.updateGradientData();
            self.updateGradientData();
        }
        // Java: if (this.fill.type() == 2)
        if self.fill.r#type() == 2 {
            // Java: this.isNeedTilePicAsTexture = this.fill.blipFill().fillMode().hasTile();
            self.isNeedTilePicAsTexture = self.fill.blipFill().fillMode().hasTile();
        }
    }
    // Java: void updateGradientData()
    pub fn updateGradientData(&mut self) {
        // Java: this.iniHaloData();
        self.iniHaloData();
    }
    // Java: void iniHaloData()
    pub fn iniHaloData(&mut self) {
        // Java: List<GradientFill.GradientStop> gs = new ArrayList();
        // LLM: GradientFill.GradientStop is KProto type, using KoBin as placeholder
        let mut gs: Vec<KoBin> = Vec::new();
        // Java: this.fill.gradFill().gsLst().read(gs);
        self.fill.gradFill().shade().gsLst().read(&mut gs);
        // Java: int i = 0;
        let mut i = 0;
        // Java: for(int size = gs.size(); i < size; ++i)
        let size = gs.len();
        while i < size {
            // Java: Halo halo = new Halo();
            let mut halo = Halo::Halo_new();
            // Java: halo.update(((GradientFill.GradientStop)gs.get(i)).save());
            // LLM: GradientFill.GradientStop.save() depends on KProto
            let gs_item = std::mem::replace(&mut gs[i], KoBin);
            halo.update(gs_item);
            // Java: this.halos.add(halo);
            self.halos.push(halo);
            i += 1;
        }
    }
    // Java: int patternType()
    pub fn patternType(&mut self) -> i32 {
        // Java: return this.fill.pattFill().prst();
        self.fill.pattFill().prst()
    }
    // Java: void setPatternType(int type)
    pub fn setPatternType(&mut self, r#type: i32) {
        // Java: this.fill.pattFill().putPrst(type);
        self.fill.pattFill().putPrst(r#type);
    }
    // Java: Color patternForegroundColor()
    pub fn patternForegroundColor(&mut self) -> Color {
        // Java: return this.fill.pattFill().fgClr();
        self.fill.pattFill().fgClr()
    }
    // Java: void setPatternForegroundColor(int fgColor)
    pub fn setPatternForegroundColor(&mut self, fgColor: i32) {
        // Java: DgColorFormat colorFormat = new DgColorFormat();
        let mut colorFormat = DgColorFormat::DgColorFormat_new();
        // Java: colorFormat.putColor(fgColor);
        colorFormat.putColor(fgColor);
        // Java: this.setType(1);
        self.setType(1);
        // Java: this.fill.pattFill().writeFgClr(Color.open(colorFormat.data()));
        // LLM: pattFill().writeFgClr and Color.open depend on KProto chain
        todo!()
    }
    // Java: Color patternBackgroundColor()
    pub fn patternBackgroundColor(&mut self) -> Color {
        // Java: return this.fill.pattFill().bgClr();
        self.fill.pattFill().bgClr()
    }
    // Java: void setPatternBackgroundColor(int bgColor)
    pub fn setPatternBackgroundColor(&mut self, bgColor: i32) {
        // Java: DgColorFormat colorFormat = new DgColorFormat();
        let mut colorFormat = DgColorFormat::DgColorFormat_new();
        // Java: colorFormat.putColor(bgColor);
        colorFormat.putColor(bgColor);
        // Java: this.setType(1);
        self.setType(1);
        // Java: this.fill.pattFill().writeBgClr(Color.open(colorFormat.data()));
        // LLM: pattFill().writeBgClr and Color.open depend on KProto chain
        todo!()
    }
    // Java: void setGradientType(int gradientType)
    pub fn setGradientType(&mut self, gradientType: i32) {
        // Java: this.clearShade();
        self.clearShade();
        // Java: if (gradientType == 3)
        if gradientType == 3 {
            // Java: this.fill.gradFill().shade().lin().putAng(UnitsConverter.degree2Angle((double)45.0F));
            self.fill.gradFill().shade().lin().putAng(UnitsConverter::degree2Angle(45.0));
            // Java: this.fill.gradFill().shade().lin().putScaled(false);
            self.fill.gradFill().shade().lin().putScaled(false);
        }
        // Java: else
        else {
            // Java: Shade.PathShade pathShade = this.fill.gradFill().shade().path();
            let mut pathShade = self.fill.gradFill().shade().path();
            // Java: pathShade.putPath(gradientType);
            pathShade.putPath(gradientType);
            // Java: RelativeRect fillToRect = pathShade.fillToRect();
            let mut fillToRect = pathShade.fillToRect();
            // Java: if (gradientType == 0)
            if gradientType == 0 {
                // Java: int defaultValue = 50;
                let defaultValue = 50;
                // Java: fillToRect.putTop((double)defaultValue);
                fillToRect.putTop(defaultValue as f64);
                // Java: fillToRect.putBottom((double)defaultValue);
                fillToRect.putBottom(defaultValue as f64);
                // Java: fillToRect.putRight((double)defaultValue);
                fillToRect.putRight(defaultValue as f64);
                // Java: fillToRect.putLeft((double)defaultValue);
                fillToRect.putLeft(defaultValue as f64);
            }
            // Java: else
            else {
                // Java: int defaultValue = 100;
                let defaultValue = 100;
                // Java: fillToRect.putLeft((double)defaultValue);
                fillToRect.putLeft(defaultValue as f64);
                // Java: fillToRect.putTop((double)defaultValue);
                fillToRect.putTop(defaultValue as f64);
            }
        }
    }
    // Java: void clearShade()
    pub fn clearShade(&mut self) {
        // Java: this.fill.gradFill().shade().deleteLin();
        self.fill.gradFill().shade().deleteLin();
        // Java: this.fill.gradFill().shade().deletePath();
        self.fill.gradFill().shade().deletePath();
    }
    // Java: int gradientType()
    pub fn gradientType(&mut self) -> i32 {
        // Java: return this.fill.gradFill().shade().hasLin() ? 3 : this.fill.gradFill().shade().path().path();
        if self.fill.gradFill().shade().hasLin() { 3 } else { self.fill.gradFill().shade().path().path() }
    }
    // Java: void setGradientLinearTypeAngle(int angle)
    pub fn setGradientLinearTypeAngle(&mut self, angle: i32) {
        // Java: if (this.gradientType() != 3)
        if self.gradientType() != 3 {
            // Java: throw new IllegalStateException("只有线性类型，才可以设置角度");
            panic!("IllegalStateException");
        }
        // Java: else
        else {
            // Java: this.fill.gradFill().shade().lin().putAng(UnitsConverter.degree2Angle((double)angle));
            self.fill.gradFill().shade().lin().putAng(UnitsConverter::degree2Angle(angle as f64));
        }
    }
    // Java: int gradientLinearTypeAngle()
    pub fn gradientLinearTypeAngle(&mut self) -> i32 {
        // Java: return (int)UnitsConverter.angle2Degree((double)this.fill.gradFill().shade().lin().ang());
        UnitsConverter::angle2Degree(self.fill.gradFill().shade().lin().ang() as f64) as i32
    }
    // Java: int gradientTypeDirection()
    pub fn gradientTypeDirection(&mut self) -> i32 {
        // Java: if (this.gradientType() == 3)
        if self.gradientType() == 3 {
            // Java: return this.getLinearDirection();
            return self.getLinearDirection();
        }
        // Java: else
        else {
            // Java: RelativeRect relativeRect = this.fill.gradFill().tileRect();
            let relativeRect = self.fill.gradFill().tileRect();
            // Java: boolean hasBottom = relativeRect.hasBottom();
            let hasBottom = relativeRect.hasBottom();
            // Java: boolean hasTop = relativeRect.hasTop();
            let hasTop = relativeRect.hasTop();
            // Java: boolean hasRight = relativeRect.hasRight();
            let hasRight = relativeRect.hasRight();
            // Java: boolean hasLeft = relativeRect.hasLeft();
            let hasLeft = relativeRect.hasLeft();
            // Java: if (hasBottom && hasLeft)
            if hasBottom && hasLeft {
                // Java: return 1;
                return 1;
            }
            // Java: else if (hasBottom && hasRight)
            else if hasBottom && hasRight {
                // Java: return 0;
                return 0;
            }
            // Java: else if (hasTop && hasLeft)
            else if hasTop && hasLeft {
                // Java: return 3;
                return 3;
            }
            // Java: else
            else {
                // Java: return hasTop && hasRight ? 2 : 4;
                return if hasTop && hasRight { 2 } else { 4 };
            }
        }
    }
    // Java: int getLinearDirection()
    pub fn getLinearDirection(&mut self) -> i32 {
        // Java: Shade.LinearShade linearShade = this.fill.gradFill().shade().lin();
        let linearShade = self.fill.gradFill().shade().lin();
        // Java: int angle = (int)UnitsConverter.angle2Degree((double)linearShade.ang());
        let angle = UnitsConverter::angle2Degree(linearShade.ang() as f64) as i32;
        // Java: if (angle == 45)
        if angle == 45 {
            // Java: return 5;
            return 5;
        }
        // Java: else if (angle == 90)
        else if angle == 90 {
            // Java: return 6;
            return 6;
        }
        // Java: else if (angle == 135)
        else if angle == 135 {
            // Java: return 7;
            return 7;
        }
        // Java: else if (angle == 0 && linearShade.isScaled())
        else if angle == 0 && linearShade.isScaled() {
            // Java: return 8;
            return 8;
        }
        // Java: else if (angle == 180)
        else if angle == 180 {
            // Java: return 9;
            return 9;
        }
        // Java: else if (angle == 315)
        else if angle == 315 {
            // Java: return 10;
            return 10;
        }
        // Java: else if (angle == 270)
        else if angle == 270 {
            // Java: return 11;
            return 11;
        }
        // Java: else
        else {
            // Java: return angle == 225 ? 12 : -1;
            // LLM: angle is declared above as let angle
            return if angle == 225 { 12 } else { -1 };
        }
    }
    // Java: void setAngleByDirectionInGradientLinear(int direction)
    pub fn setAngleByDirectionInGradientLinear(&mut self, direction: i32) {
        // Java: Shade.LinearShade linearShade = this.fill.gradFill().shade().lin();
        let mut linearShade = self.fill.gradFill().shade().lin();
        // Java: linearShade.putScaled(true);
        linearShade.putScaled(true);
        // Java: switch (direction)
        match direction {
            // Java: case 5:
            5 => {
                // Java: linearShade.putAng(UnitsConverter.degree2Angle((double)45.0F));
                linearShade.putAng(UnitsConverter::degree2Angle(45.0));
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 6:
            6 => {
                // Java: linearShade.putAng(UnitsConverter.degree2Angle((double)90.0F));
                linearShade.putAng(UnitsConverter::degree2Angle(90.0));
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 7:
            7 => {
                // Java: linearShade.putAng(UnitsConverter.degree2Angle((double)135.0F));
                linearShade.putAng(UnitsConverter::degree2Angle(135.0));
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 8:
            8 => {
                // Java: linearShade.putAng(UnitsConverter.degree2Angle((double)0.0F));
                linearShade.putAng(UnitsConverter::degree2Angle(0.0));
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 9:
            9 => {
                // Java: linearShade.putAng(UnitsConverter.degree2Angle((double)180.0F));
                linearShade.putAng(UnitsConverter::degree2Angle(180.0));
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 10:
            10 => {
                // Java: linearShade.putAng(UnitsConverter.degree2Angle((double)315.0F));
                linearShade.putAng(UnitsConverter::degree2Angle(315.0));
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 11:
            11 => {
                // Java: linearShade.putAng(UnitsConverter.degree2Angle((double)270.0F));
                linearShade.putAng(UnitsConverter::degree2Angle(270.0));
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 12:
            12 => {
                // Java: linearShade.putAng(UnitsConverter.degree2Angle((double)225.0F));
                linearShade.putAng(UnitsConverter::degree2Angle(225.0));
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: default:
            _ => {
                // Java: linearShade.putAng(UnitsConverter.degree2Angle((double)0.0F));
                linearShade.putAng(UnitsConverter::degree2Angle(0.0));
                // Java: linearShade.putScaled(false);
                linearShade.putScaled(false);
            }
        }
    }
    // Java: void setGradientTypeDirection(int direction)
    pub fn setGradientTypeDirection(&mut self, direction: i32) {
        // Java: RelativeRect rect = this.fill.gradFill().tileRect();
        let mut rect = self.fill.gradFill().tileRect();
        // Java: if (this.gradientType() == 3)
        if self.gradientType() == 3 {
            // Java: this.setAngleByDirectionInGradientLinear(direction);
            self.setAngleByDirectionInGradientLinear(direction);
            // Java: this.fill.gradFill().forceWriteTileRect(RelativeRect.create());
            self.fill.gradFill().forceWriteTileRect(RelativeRectStub::create());
        }
        // Java: else
        else {
            // Java: int defaultValue = -100;
            let defaultValue: f64 = -100.0;
            // Java: switch (direction)
            match direction {
                // Java: case 0:
                0 => {
                    // Java: rect.deleteTop();
                    rect.deleteTop();
                    // Java: rect.deleteLeft();
                    rect.deleteLeft();
                    // Java: rect.putRight((double)defaultValue);
                    rect.putRight(defaultValue);
                    // Java: rect.putBottom((double)defaultValue);
                    rect.putBottom(defaultValue);
                    // Java: break;
                    // LLM No Rust equivalent: Rust match arms don't need break
                }
                // Java: case 1:
                1 => {
                    // Java: rect.deleteTop();
                    rect.deleteTop();
                    // Java: rect.deleteRight();
                    rect.deleteRight();
                    // Java: rect.putLeft((double)defaultValue);
                    rect.putLeft(defaultValue);
                    // Java: rect.putBottom((double)defaultValue);
                    rect.putBottom(defaultValue);
                    // Java: break;
                    // LLM No Rust equivalent: Rust match arms don't need break
                }
                // Java: case 2:
                2 => {
                    // Java: rect.deleteBottom();
                    rect.deleteBottom();
                    // Java: rect.deleteLeft();
                    rect.deleteLeft();
                    // Java: rect.putRight((double)defaultValue);
                    rect.putRight(defaultValue);
                    // Java: rect.putTop((double)defaultValue);
                    rect.putTop(defaultValue);
                    // Java: break;
                    // LLM No Rust equivalent: Rust match arms don't need break
                }
                // Java: case 3:
                3 => {
                    // Java: rect.deleteBottom();
                    rect.deleteBottom();
                    // Java: rect.deleteRight();
                    rect.deleteRight();
                    // Java: rect.putLeft((double)defaultValue);
                    rect.putLeft(defaultValue);
                    // Java: rect.putTop((double)defaultValue);
                    rect.putTop(defaultValue);
                    // Java: break;
                    // LLM No Rust equivalent: Rust match arms don't need break
                }
                // Java: default:
                _ => {
                    // Java: this.fill.gradFill().forceWriteTileRect(RelativeRect.create());
                    // LLM: cannot access self.fill while rect borrows it; using drop
                    drop(rect);
                    self.fill.gradFill().forceWriteTileRect(RelativeRectStub::create());
                }
            }
        }
    }
    // Java: void addHalo(Halo halo)
    pub fn addHalo(&mut self, halo: Halo) {
        // Java: this.setType(4);
        self.setType(4);
        // Java: if (this.halos.size() >= 6)
        if self.halos.len() >= 6 {
            // Java: throw new IllegalStateException("达到上限6个光圈");
            panic!("IllegalStateException");
        }
        // Java: else
        else {
            // Java: this.halos.add(halo);
            self.halos.push(halo);
        }
    }
    // Java: Halo haloAt(int index)
    pub fn haloAt(&self, index: i32) -> &Halo {
        // Java: assert index >= 0 && index < this.halos.size();
        assert!(index >= 0 && index < self.halos.len() as i32);
        // Java: return (Halo)this.halos.get(index);
        &self.halos[index as usize]
    }
    // Java: int haloCount()
    pub fn haloCount(&self) -> i32 {
        // Java: return this.halos.size();
        self.halos.len() as i32
    }
    // Java: void removeHalo(int index)
    pub fn removeHalo(&mut self, index: i32) {
        // Java: if (this.halos.size() > index && index >= 0)
        if self.halos.len() as i32 > index && index >= 0 {
            // Java: if (this.halos.size() == 2)
            if self.halos.len() == 2 {
                // Java: throw new IllegalStateException("无法继续删除了，达到下限2个光圈了哇");
                panic!("IllegalStateException");
            }
            // Java: else
            else {
                // Java: this.halos.remove(index);
                self.halos.remove(index as usize);
            }
        }
    }
    // Java: void setAlpha(int alpha)
    pub fn setAlpha(&mut self, alpha: i32) {
        // Java: DgColorFormat colorFormat = new DgColorFormat();
        // LLM: solidFill chain depends on KProto
        // Java: colorFormat.update(this.fill.solidFill().save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: colorFormat.putAlpha(UnitsConverter.percent2decimals((double)alpha));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.setType(0);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.fill.writeSolidFill(Color.open(colorFormat.data()));
        todo!()
    }
    // Java: int alpha()
    pub fn alpha(&self) -> i32 {
        // Java: if (this.type() == 0)
        if self.r#type() == 0 {
            // Java: DgColorFormat colorFormat = new DgColorFormat();
            // LLM: solidFill chain depends on KProto
            // Java: colorFormat.update(this.fill.solidFill().save());
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: return (int)UnitsConverter.decimals2percent(colorFormat.alpha());
            todo!()
        }
        // Java: else
        else {
            // Java: return 0;
            0
        }
    }
    // Java: Color solidColor()
    pub fn solidColor(&self) -> Color {
        // Java: return this.fill.solidFill();
        self.fill.solidFill()
    }
    // Java: boolean hasSolidColor()
    pub fn hasSolidColor(&self) -> bool {
        // Java: return this.fill.hasSolidFill();
        self.fill.hasSolidFill()
    }
    // Java: void setBlipAlpha(int alpha)
    pub fn setBlipAlpha(&mut self, alpha: i32) {
        // Java: if (alpha != 0)
        if alpha != 0 {
            // Java: List<Effect> effects = new ArrayList(1);
            // LLM: Effect/blipFill chain depends on KProto
            // Java: Effect effect = Effect.create();
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: effect.alphaModFix().putAmount(UnitsConverter.percent2decimals((double)(100 - alpha)));
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: effect.save();
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: effects.add(effect);
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: this.fill.blipFill().blip().effects().write(effects);
            todo!()
        }
    }
    // Java: int blipAlpha()
    pub fn blipAlpha(&mut self) -> i32 {
        // Java: Blip blip = this.fill.blipFill().blip();
        // LLM: blipFill chain depends on KProto; entire method aggregated
        // Java: if (!blip.hasEffects())
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 0;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: Blip.Effects effects = blip.effects();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: List<Effect> effectLst = new ArrayList(1);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: effects.read(effectLst);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return effectLst.isEmpty() ? 0 : (int)((double)100.0F - UnitsConverter.decimals2percent(((Effect)effectLst.get(0)).alphaModFix().amount()));
        todo!()
    }
    // Java: void setTexturePicId(int mediaId)
    pub fn setTexturePicId(&mut self, mediaId: i32) {
        // Java: Blip blip = this.fill.blipFill().blip();
        let mut blip = self.fill.blipFill().blip();
        // Java: if (blip.hasEmbedMediaID())
        if blip.hasEmbedMediaID() {
            // Java: blip.deleteEmbedMediaID();
            blip.deleteEmbedMediaID();
        }
        // Java: blip.putLinkMediaID(mediaId);
        blip.putLinkMediaID(mediaId);
        // Java: this.setIsNeedTilePicAsTexture(true);
        self.isNeedTilePicAsTexture = true;
        // LLM: inlining setIsNeedTilePicAsTexture to avoid borrow conflict; blipFill chain for fillMode is todo
    }
    // Java: void setPicId(int mediaId)
    pub fn setPicId(&mut self, mediaId: i32) {
        // Java: Blip blip = this.fill.blipFill().blip();
        let mut blip = self.fill.blipFill().blip();
        // Java: if (blip.hasLinkMediaID())
        if blip.hasLinkMediaID() {
            // Java: blip.deleteLinkMediaID();
            blip.deleteLinkMediaID();
        }
        // Java: blip.putEmbedMediaID(mediaId);
        blip.putEmbedMediaID(mediaId);
        // Java: this.setIsNeedTilePicAsTexture(false);
        self.isNeedTilePicAsTexture = false;
        // LLM: inlining setIsNeedTilePicAsTexture to avoid borrow conflict
    }
    // Java: int picId()
    pub fn picId(&mut self) -> i32 {
        // Java: Blip blip = this.fill.blipFill().blip();
        let blip = self.fill.blipFill().blip();
        // Java: if (blip.hasEmbedMediaID())
        if blip.hasEmbedMediaID() {
            // Java: return blip.embedMediaID();
            return blip.embedMediaID();
        }
        // Java: else
        else {
            // Java: return blip.hasLinkMediaID() ? blip.linkMediaID() : -1;
            return if blip.hasLinkMediaID() { blip.linkMediaID() } else { -1 };
        }
    }
    // Java: void setIsNeedTilePicAsTexture(boolean isNeedTilePicAsTexture)
    pub fn setIsNeedTilePicAsTexture(&mut self, val: bool) {
        // Java: this.isNeedTilePicAsTexture = isNeedTilePicAsTexture;
        self.isNeedTilePicAsTexture = val;
        // Java: if (isNeedTilePicAsTexture)
        if val {
            // Java: this.fill.blipFill().fillMode().deleteStretch();
            self.fill.blipFill().fillMode().deleteStretch();
        }
        // Java: else
        else {
            // Java: this.fill.blipFill().fillMode().deleteTile();
            self.fill.blipFill().fillMode().deleteTile();
        }
    }
    // Java: boolean isNeedTilePicAsTexture()
    pub fn isNeedTilePicAsTexture_getter(&self) -> bool {
        // Java: return this.isNeedTilePicAsTexture;
        self.isNeedTilePicAsTexture
    }
    // Java: boolean isBlipStretch()
    pub fn isBlipStretch(&self) -> bool {
        // Java: return this.type() == 2 && !this.isNeedTilePicAsTexture;
        let t = self.r#type();
        // LLM: self.type() is mapped to self.r#type() because type is a Rust keyword
        t == 2 && !self.isNeedTilePicAsTexture
    }
    // Java: boolean isBlipTile()
    pub fn isBlipTile(&self) -> bool {
        // Java: return this.type() == 2 && this.isNeedTilePicAsTexture;
        let t = self.r#type();
        // LLM: self.type() is mapped to self.r#type() because type is a Rust keyword
        t == 2 && self.isNeedTilePicAsTexture
    }
    // Java: double blipStretchLeft()
    pub fn blipStretchLeft(&mut self) -> f64 {
        // Java: return UnitsConverter.decimals2percent(this.fill.blipFill().fillMode().stretch().fillRect().left());
        UnitsConverter::decimals2percent(self.fill.blipFill().fillMode().stretch().fillRect().left())
    }
    // Java: void setBlipStretchLeft(double left)
    pub fn setBlipStretchLeft(&mut self, left: f64) {
        // Java: if (this.isBlipStretch())
        if self.isBlipStretch() {
            // Java: this.fill.blipFill().fillMode().stretch().fillRect().putLeft(UnitsConverter.percent2decimals(left));
            self.fill.blipFill().fillMode().stretch().fillRect().putLeft(UnitsConverter::percent2decimals(left));
        }
    }
    // Java: double blipStretchRight()
    pub fn blipStretchRight(&mut self) -> f64 {
        // Java: return UnitsConverter.decimals2percent(this.fill.blipFill().fillMode().stretch().fillRect().right());
        UnitsConverter::decimals2percent(self.fill.blipFill().fillMode().stretch().fillRect().right())
    }
    // Java: void setBlipStretchRight(double right)
    pub fn setBlipStretchRight(&mut self, right: f64) {
        // Java: if (this.isBlipStretch())
        if self.isBlipStretch() {
            // Java: this.fill.blipFill().fillMode().stretch().fillRect().putRight(UnitsConverter.percent2decimals(right));
            self.fill.blipFill().fillMode().stretch().fillRect().putRight(UnitsConverter::percent2decimals(right));
        }
    }
    // Java: double blipStretchTop()
    pub fn blipStretchTop(&mut self) -> f64 {
        // Java: return UnitsConverter.decimals2percent(this.fill.blipFill().fillMode().stretch().fillRect().top());
        UnitsConverter::decimals2percent(self.fill.blipFill().fillMode().stretch().fillRect().top())
    }
    // Java: void setBlipStretchTop(double top)
    pub fn setBlipStretchTop(&mut self, top: f64) {
        // Java: if (this.isBlipStretch())
        if self.isBlipStretch() {
            // Java: this.fill.blipFill().fillMode().stretch().fillRect().putTop(UnitsConverter.percent2decimals(top));
            self.fill.blipFill().fillMode().stretch().fillRect().putTop(UnitsConverter::percent2decimals(top));
        }
    }
    // Java: double blipStretchBottom()
    pub fn blipStretchBottom(&mut self) -> f64 {
        // Java: return UnitsConverter.decimals2percent(this.fill.blipFill().fillMode().stretch().fillRect().bottom());
        UnitsConverter::decimals2percent(self.fill.blipFill().fillMode().stretch().fillRect().bottom())
    }
    // Java: void setBlipStretchBottom(double bottom)
    pub fn setBlipStretchBottom(&mut self, bottom: f64) {
        // Java: if (this.isBlipStretch())
        if self.isBlipStretch() {
            // Java: this.fill.blipFill().fillMode().stretch().fillRect().putBottom(UnitsConverter.percent2decimals(bottom));
            self.fill.blipFill().fillMode().stretch().fillRect().putBottom(UnitsConverter::percent2decimals(bottom));
        }
    }
    // Java: void setBlipOffsetX(double x)
    pub fn setBlipOffsetX(&mut self, x: f64) {
        // Java: if (this.isBlipTile())
        if self.isBlipTile() {
            // Java: this.fill.blipFill().fillMode().tile().putTx((int)UnitsConverter.pt2emu(x));
            self.fill.blipFill().fillMode().tile().putTx(UnitsConverter::pt2emu(x) as i32);
        }
    }
    // Java: double blipOffsetX()
    pub fn blipOffsetX(&mut self) -> f64 {
        // Java: int emuX = this.fill.blipFill().fillMode().tile().tx();
        let emuX = self.fill.blipFill().fillMode().tile().tx();
        // Java: return UnitsConverter.emu2pt((double)emuX);
        UnitsConverter::emu2pt(emuX as f64)
    }
    // Java: void setBlipOffsetY(double y)
    pub fn setBlipOffsetY(&mut self, y: f64) {
        // Java: if (this.isBlipTile())
        if self.isBlipTile() {
            // Java: this.fill.blipFill().fillMode().tile().putTy((int)UnitsConverter.pt2emu(y));
            self.fill.blipFill().fillMode().tile().putTy(UnitsConverter::pt2emu(y) as i32);
        }
    }
    // Java: double blipOffsetY()
    pub fn blipOffsetY(&mut self) -> f64 {
        // Java: int emuY = this.fill.blipFill().fillMode().tile().ty();
        let emuY = self.fill.blipFill().fillMode().tile().ty();
        // Java: return UnitsConverter.emu2pt((double)emuY);
        UnitsConverter::emu2pt(emuY as f64)
    }
    // Java: double blipZoomRatioX()
    pub fn blipZoomRatioX(&mut self) -> f64 {
        // Java: return UnitsConverter.decimals2percent(this.fill.blipFill().fillMode().tile().sx());
        UnitsConverter::decimals2percent(self.fill.blipFill().fillMode().tile().sx())
    }
    // Java: void setBlipZoomRatioX(double ratioX)
    pub fn setBlipZoomRatioX(&mut self, ratioX: f64) {
        // Java: if (this.isBlipTile())
        if self.isBlipTile() {
            // Java: this.fill.blipFill().fillMode().tile().putSx(UnitsConverter.percent2decimals(ratioX));
            self.fill.blipFill().fillMode().tile().putSx(UnitsConverter::percent2decimals(ratioX));
        }
    }
    // Java: double blipZoomRatioY()
    pub fn blipZoomRatioY(&mut self) -> f64 {
        // Java: return UnitsConverter.decimals2percent(this.fill.blipFill().fillMode().tile().sy());
        UnitsConverter::decimals2percent(self.fill.blipFill().fillMode().tile().sy())
    }
    // Java: void setBlipZoomRatioY(double ratioY)
    pub fn setBlipZoomRatioY(&mut self, ratioY: f64) {
        // Java: if (this.isBlipTile())
        if self.isBlipTile() {
            // Java: this.fill.blipFill().fillMode().tile().putSy(UnitsConverter.percent2decimals(ratioY));
            self.fill.blipFill().fillMode().tile().putSy(UnitsConverter::percent2decimals(ratioY));
        }
    }
    // Java: int blipFlipType()
    pub fn blipFlipType(&mut self) -> i32 {
        // Java: return this.fill.blipFill().fillMode().tile().flip();
        self.fill.blipFill().fillMode().tile().flip()
    }
    // Java: void setBlipFlipType(int flipType)
    pub fn setBlipFlipType(&mut self, flipType: i32) {
        // Java: if (this.isBlipTile())
        if self.isBlipTile() {
            // Java: this.fill.blipFill().fillMode().tile().putFlip(flipType);
            self.fill.blipFill().fillMode().tile().putFlip(flipType);
        }
    }
    // Java: int blipAlignmentType()
    pub fn blipAlignmentType(&mut self) -> i32 {
        // Java: return this.fill.blipFill().fillMode().tile().algn();
        self.fill.blipFill().fillMode().tile().algn()
    }
    // Java: void setBlipAlignmentType(int alignmentType)
    pub fn setBlipAlignmentType(&mut self, alignmentType: i32) {
        // Java: if (this.isBlipTile())
        if self.isBlipTile() {
            // Java: this.fill.blipFill().fillMode().tile().putAlgn(alignmentType);
            self.fill.blipFill().fillMode().tile().putAlgn(alignmentType);
        }
    }
    // Java: boolean isRotateWithShape()
    pub fn isRotateWithShape(&mut self) -> bool {
        // Java: if (this.type() == 4)
        if self.r#type() == 4 {
            // Java: return this.fill.gradFill().isRotWithShape();
            return self.fill.gradFill().isRotWithShape();
        }
        // Java: else
        else {
            // Java: return this.type() == 2 ? this.fill.blipFill().isRotWithShape() : false;
            let t = self.r#type();
            // LLM: self.type() is mapped to self.r#type() because type is a Rust keyword
            return if t == 2 { self.fill.blipFill().isRotWithShape() } else { false };
        }
    }
    // Java: void setRotateWithShape(boolean isRotWithShape)
    pub fn setRotateWithShape(&mut self, isRotWithShape: bool) {
        // Java: if (this.type() == 4)
        if self.r#type() == 4 {
            // Java: this.fill.gradFill().putRotWithShape(isRotWithShape);
            self.fill.gradFill().putRotWithShape(isRotWithShape);
        }
        // Java: else if (this.type() == 2)
        else if self.r#type() == 2 {
            // Java: this.fill.blipFill().putRotWithShape(isRotWithShape);
            self.fill.blipFill().putRotWithShape(isRotWithShape);
        }
    }
    // Java: DgFillFormat clone()
    pub fn clone(&self) -> DgFillFormat {
        // Java: DgFillFormat fillFormat = new DgFillFormat();
        let fillFormat = DgFillFormat::DgFillFormat_new();
        // Java: fillFormat.updateProtocol(this.protocol());
        // LLM: KoBin serialization not yet available
        // Java: return fillFormat;
        fillFormat
    }
    // Java: static boolean isEqual(DgFillFormat fillFormat1, DgFillFormat fillFormat2)
    pub fn isEqual(fillFormat1: &mut DgFillFormat, fillFormat2: &mut DgFillFormat) -> bool {
        // Java: if (fillFormat1.type() != fillFormat2.type())
        if fillFormat1.r#type() != fillFormat2.r#type() {
            // Java: return false;
            return false;
        }
        // Java: else if (fillFormat1.type() != 0)
        else if fillFormat1.r#type() != 0 {
            // Java: if (fillFormat1.type() == 4)
            if fillFormat1.r#type() == 4 {
                // Java: boolean isEqualGradientType = fillFormat1.gradientType() == fillFormat2.gradientType();
                let isEqualGradientType = fillFormat1.gradientType() == fillFormat2.gradientType();
                // Java: boolean isEqualAngel = fillFormat1.gradientLinearTypeAngle() == fillFormat2.gradientLinearTypeAngle();
                let isEqualAngel = fillFormat1.gradientLinearTypeAngle() == fillFormat2.gradientLinearTypeAngle();
                // Java: if (isEqualGradientType && isEqualAngel)
                if isEqualGradientType && isEqualAngel {
                    // Java: return isEqualHalos(fillFormat1, fillFormat2);
                    return DgFillFormat::isEqualHalos(fillFormat1, fillFormat2);
                }
                // Java: else
                else {
                    // Java: return false;
                    return false;
                }
            }
            // Java: else
            else {
                // Java: return true;
                return true;
            }
        }
        // Java: else
        else {
            // Java: DgColorFormat colorFormat = new DgColorFormat();
            // LLM: solidColor/save chain depends on KProto
            // Java: colorFormat.update(fillFormat1.solidColor().save());
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: DgColorFormat colorFormat2 = new DgColorFormat();
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: colorFormat2.update(fillFormat2.solidColor().save());
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: boolean isEqualColor = DgColorFormat.isEqual(colorFormat, colorFormat2);
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: return isEqualColor && fillFormat1.alpha() == fillFormat2.alpha();
            todo!()
        }
    }
    // Java: static boolean isEqualHalos(DgFillFormat fillFormat1, DgFillFormat fillFormat2)
    pub fn isEqualHalos(f1: &DgFillFormat, f2: &DgFillFormat) -> bool {
        // Java: int haloCount1 = fillFormat1.haloCount();
        let haloCount1 = f1.haloCount();
        // Java: int haloCount2 = fillFormat2.haloCount();
        let haloCount2 = f2.haloCount();
        // Java: if (haloCount1 != haloCount2)
        if haloCount1 != haloCount2 {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: for(int i = 0; i < haloCount1; ++i)
            for i in 0..haloCount1 {
                // Java: if (!isEqualHalo(fillFormat1.haloAt(i), fillFormat2.haloAt(i)))
                if !DgFillFormat::isEqualHalo(f1.haloAt(i), f2.haloAt(i)) {
                    // Java: return false;
                    return false;
                }
            }
            // Java: return true;
            true
        }
    }
    // Java: static boolean isEqualHalo(Halo halo1, Halo halo2)
    pub fn isEqualHalo(halo1: &Halo, halo2: &Halo) -> bool {
        // Java: boolean isEqualColor = DgColorFormat.isEqual(halo1.colorFormat(), halo2.colorFormat());
        // LLM: colorFormat comparison depends on mutable access for DgColorFormat.isEqual
        // Java: boolean isEqualOther = halo1.alpha() == halo2.alpha() && halo1.position() == halo2.position() && halo1.luminance() == halo2.luminance();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return isEqualColor && isEqualOther;
        todo!()
    }
}

impl Halo {
    pub fn Halo_new() -> Self {
        Halo { gradientStop: GradientStop::new(), color: DgColorFormat::DgColorFormat_new() }
    }
    // Java: void update(KoBin gs)
    pub fn update(&mut self, gs: KoBin) {
        // Java: this.gradientStop.load(gs);
        self.gradientStop.load(gs);
        // Java: this.color.update(this.gradientStop.color().save());
        // LLM: gradientStop.color().save() depends on KProto chain
        todo!()
    }
    // Java: KoBin data()
    pub fn data(&mut self) -> KoBin {
        // Java: this.gradientStop.writeColor(Color.open(this.color.data()));
        // LLM: KoBin serialization chain
        // Java: return this.gradientStop.save();
        todo!()
    }
    // Java: void setPosition(int position)
    pub fn setPosition(&mut self, position: i32) {
        // Java: this.gradientStop.putPos(UnitsConverter.percent2decimals((double)position));
        self.gradientStop.putPos(UnitsConverter::percent2decimals(position as f64));
    }
    // Java: int position()
    pub fn position(&self) -> i32 {
        // Java: return (int)UnitsConverter.decimals2percent(this.gradientStop.pos());
        UnitsConverter::decimals2percent(self.gradientStop.pos()) as i32
    }
    // Java: void setAlpha(int alpha)
    pub fn setAlpha(&mut self, alpha: i32) {
        // Java: this.writeColor();
        self.writeColor();
        // Java: ColorTransform transform = ColorTransform.create();
        let mut transform = ColorTransform::create();
        // Java: transform.putAlpha(UnitsConverter.percent2decimals((double)(100 - alpha)));
        transform.putAlpha(UnitsConverter::percent2decimals((100 - alpha) as f64));
        // Java: transform.putType(0);
        transform.putType(0);
        // Java: DgColorFormat.setColorTransformByType(this.gradientStop.color(), transform);
        DgColorFormat::setColorTransformByType(self.gradientStop.color(), transform);
        // Java: this.readColor();
        self.readColor();
    }
    // Java: int alpha()
    pub fn alpha(&mut self) -> i32 {
        // Java: return (int)UnitsConverter.decimals2percent(this.color.alpha());
        UnitsConverter::decimals2percent(self.color.alpha()) as i32
    }
    // Java: void setLuminance(int luminance)
    pub fn setLuminance(&mut self, luminance: i32) {
        // Java: this.writeColor();
        self.writeColor();
        // Java: double lumMod = (double)1.0F;
        let mut lumMod: f64 = 1.0;
        // Java: double lum = UnitsConverter.percent2decimals((double)luminance);
        let lum = UnitsConverter::percent2decimals(luminance as f64);
        // Java: if (lum > (double)0.0F)
        if lum > 0.0 {
            // Java: lumMod -= lum;
            lumMod -= lum;
            // Java: ColorTransform transform = ColorTransform.create();
            let mut transform = ColorTransform::create();
            // Java: transform.putLumOff(lum);
            transform.putLumOff(lum);
            // Java: transform.putType(19);
            transform.putType(19);
            // Java: DgColorFormat.setColorTransformByType(this.gradientStop.color(), transform);
            DgColorFormat::setColorTransformByType(self.gradientStop.color(), transform);
        }
        // Java: else
        else {
            // Java: lumMod += lum;
            lumMod += lum;
            // Java: DgColorFormat.delColorTransformByType(this.gradientStop.color(), 19);
            DgColorFormat::delColorTransformByType(self.gradientStop.color(), 19);
        }
        // Java: ColorTransform transform = ColorTransform.create();
        let mut transform = ColorTransform::create();
        // Java: transform.putLumMod(lumMod);
        transform.putLumMod(lumMod);
        // Java: transform.putType(18);
        transform.putType(18);
        // Java: DgColorFormat.setColorTransformByType(this.gradientStop.color(), transform);
        DgColorFormat::setColorTransformByType(self.gradientStop.color(), transform);
        // Java: this.readColor();
        self.readColor();
    }
    // Java: void writeColor()
    pub fn writeColor(&mut self) {
        // Java: this.gradientStop.writeColor(Color.open(this.color.data()));
        // LLM: KoBin serialization chain
        todo!()
    }
    // Java: void readColor()
    pub fn readColor(&mut self) {
        // Java: this.color.update(this.gradientStop.color().save());
        // LLM: KProto chain
        todo!()
    }
    // Java: int luminance()
    pub fn luminance(&mut self) -> i32 {
        // Java: Color color1 = Color.open(this.color.data());
        // LLM: KoBin serialization chain; entire method aggregated
        // Java: ColorTransform lumMod = DgColorFormat.getColorTransformByType(color1, 18);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: ColorTransform lumOff = DgColorFormat.getColorTransformByType(color1, 19);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (lumMod == null && lumOff == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 0;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return lumOff == null ? (int)(UnitsConverter.decimals2percent(lumMod.lumMod()) - (double)100.0F) : (int)UnitsConverter.decimals2percent(lumOff.lumOff());
        // LLM: lumOff and lumMod are declared above; entire method depends on KProto Color.open chain
        todo!()
    }
    // Java: DgColorFormat colorFormat()
    pub fn colorFormat(&self) -> &DgColorFormat {
        // Java: return this.color;
        &self.color
    }
}

impl KmoProtocol for DgFillFormat {
    fn protocol(&self) -> Box<dyn crate::cn::wps::kso::kproto::KoBin::KoBin> {
        todo!()
    }
    fn updateProtocol(&mut self, _var1: &dyn crate::cn::wps::kso::kproto::KoBin::KoBin) {
        todo!()
    }
}
impl Cloneable for DgFillFormat {}
