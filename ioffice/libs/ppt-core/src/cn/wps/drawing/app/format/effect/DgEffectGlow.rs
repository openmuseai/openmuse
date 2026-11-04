#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]
// Java: import cn.wps.drawing.app.KmoProtocol
use crate::cn::wps::drawing::app::KmoProtocol::KmoProtocol;
// Java: import cn.wps.drawing.app.format.color.DgColorFormat
use crate::cn::wps::drawing::app::format::color::DgColorFormat::DgColorFormat;
// Java: import cn.wps.drawing.app.util.UnitsConverter
// LLM: UnitsConverter not yet available, using stub
// Java: import cn.wps.drawing.data.color.Color
use crate::cn::wps::drawing::data::color::Color::Color;
// Java: import cn.wps.drawing.data.color.ColorTransform
use crate::cn::wps::drawing::data::color::ColorTransform::ColorTransform;
// Java: import cn.wps.drawing.data.effect.GlowEffect
// LLM: GlowEffect is a KProto type, using stub
// Java: import cn.wps.kso.kproto.KoBin
// LLM: KoBin stub

pub struct GlowEffect;
impl GlowEffect {
    pub fn create() -> Self { GlowEffect }
    pub fn save(&self) -> KoBin { KoBin }
    pub fn load(&mut self, _bin: KoBin) {}
    pub fn close(&mut self) {}
    pub fn hasClr(&self) -> bool { false }
    pub fn hasRadius(&self) -> bool { false }
    pub fn clr(&mut self) -> &mut Color { todo!() }
    pub fn writeClr(&mut self, _c: Color) {}
    pub fn radius(&self) -> i32 { 0 }
    pub fn putRadius(&mut self, _v: i32) {}
}
pub struct KoBin;
pub struct UnitsConverter;
impl UnitsConverter {
    pub fn emu2pt(v: f64) -> f64 { v / 12700.0 }
    pub fn pt2emu(v: f64) -> f64 { v * 12700.0 }
    pub fn percent2decimals(v: f64) -> f64 { v / 100.0 }
    pub fn decimals2percent(v: f64) -> f64 { v * 100.0 }
}

pub const PRESET_ALPHA: f64 = 40.0 / 100.0;
pub const PRESET_SAT_MOE: f64 = 175.0 / 100.0;

// Java: public class DgEffectGlow implements KmoProtocol
pub struct DgEffectGlow {
    // Java field: GlowEffect glowEffect
    pub(crate) glowEffect: GlowEffect,
    // Java field: DgColorFormat colorFormat
    pub(crate) colorFormat: DgColorFormat,
    // Java field: int[] radPreset
    // LLM: collection_type Vec<i32> for Java int[]
    pub(crate) radPreset: Vec<i32>,
    // Java field: int[] colorPreset
    // LLM: collection_type Vec<i32> for Java int[]
    pub(crate) colorPreset: Vec<i32>,
    // Java field: double PRESET_ALPHA
    // LLM: static final mapped to module-level pub const PRESET_ALPHA
    pub(crate) PRESET_ALPHA: f64,
    // Java field: double PRESET_SAT_MOE
    // LLM: static final mapped to module-level pub const PRESET_SAT_MOE
    pub(crate) PRESET_SAT_MOE: f64,
}

impl DgEffectGlow {
    // Java: DgEffectGlow()
    pub fn DgEffectGlow_new() -> Self {
        let mut _self = DgEffectGlow {
            glowEffect: GlowEffect::create(),
            colorFormat: DgColorFormat::DgColorFormat_new(),
            radPreset: vec![0; 4],
            colorPreset: vec![0; 6],
            PRESET_ALPHA: PRESET_ALPHA,
            PRESET_SAT_MOE: PRESET_SAT_MOE,
        };
        // Java: this.iniPresetData();
        _self.iniPresetData();
        _self
    }
    // Java: KoBin protocol()
    pub fn protocol_local(&mut self) -> KoBin {
        // Java: this.writeColor2Protocol();
        self.writeColor2Protocol();
        // Java: return this.glowEffect.save();
        self.glowEffect.save()
    }
    // Java: void updateProtocol(KoBin data)
    pub fn updateProtocol_local(&mut self, data: KoBin) {
        // Java: this.glowEffect.load(data);
        self.glowEffect.load(data);
        // Java: this.readColor();
        self.readColor();
    }
    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return !this.glowEffect.hasClr() && !this.glowEffect.hasRadius();
        !self.glowEffect.hasClr() && !self.glowEffect.hasRadius()
    }
    // Java: void readColor()
    fn readColor(&mut self) {
        // Java: this.colorFormat.update(this.glowEffect.clr().save());
        // LLM: clr().save() depends on KProto chain
        todo!()
    }
    // Java: void writeColor2Protocol()
    fn writeColor2Protocol(&mut self) {
        // Java: this.glowEffect.writeClr(Color.open(this.colorFormat.data()));
        // LLM: Color.open and colorFormat.data() depend on KProto chain
        todo!()
    }
    // Java: void setColorFormat(DgColorFormat colorFormat)
    pub fn setColorFormat(&mut self, colorFormat: DgColorFormat) {
        // Java: this.colorFormat = colorFormat;
        self.colorFormat = colorFormat;
        // Java: this.writeColor2Protocol();
        self.writeColor2Protocol();
    }
    // Java: DgColorFormat colorFormat()
    pub fn colorFormat(&self) -> &DgColorFormat {
        // Java: return this.colorFormat;
        &self.colorFormat
    }
    // Java: void setSize(double size)
    pub fn setSize(&mut self, size: f64) {
        // Java: this.glowEffect.putRadius((int)UnitsConverter.pt2emu(size));
        self.glowEffect.putRadius(UnitsConverter::pt2emu(size) as i32);
    }
    // Java: double size()
    pub fn size(&self) -> f64 {
        // Java: return UnitsConverter.emu2pt((double)this.glowEffect.radius());
        UnitsConverter::emu2pt(self.glowEffect.radius() as f64)
    }
    // Java: void setAlpha(int alpha)
    pub fn setAlpha(&mut self, alpha: i32) {
        // Java: this.colorFormat.putAlpha(UnitsConverter.percent2decimals((double)alpha));
        self.colorFormat.putAlpha(UnitsConverter::percent2decimals(alpha as f64));
    }
    // Java: int alpha()
    pub fn alpha(&mut self) -> i32 {
        // Java: return (int)UnitsConverter.decimals2percent(this.colorFormat.alpha());
        UnitsConverter::decimals2percent(self.colorFormat.alpha()) as i32
    }
    // Java: void setPresetType(int presetType)
    pub fn setPresetType(&mut self, presetType: i32) {
        // Java: switch (presetType)
        match presetType {
            // Java: case 0:
            0 => {
                // Java: this.fillPresetGlow(0, 0);
                self.fillPresetGlow(0, 0);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 1:
            1 => {
                // Java: this.fillPresetGlow(1, 0);
                self.fillPresetGlow(1, 0);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 2:
            2 => {
                // Java: this.fillPresetGlow(2, 0);
                self.fillPresetGlow(2, 0);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 3:
            3 => {
                // Java: this.fillPresetGlow(3, 0);
                self.fillPresetGlow(3, 0);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 4:
            4 => {
                // Java: this.fillPresetGlow(0, 1);
                self.fillPresetGlow(0, 1);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 5:
            5 => {
                // Java: this.fillPresetGlow(1, 1);
                self.fillPresetGlow(1, 1);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 6:
            6 => {
                // Java: this.fillPresetGlow(2, 1);
                self.fillPresetGlow(2, 1);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 7:
            7 => {
                // Java: this.fillPresetGlow(3, 1);
                self.fillPresetGlow(3, 1);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 8:
            8 => {
                // Java: this.fillPresetGlow(0, 2);
                self.fillPresetGlow(0, 2);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 9:
            9 => {
                // Java: this.fillPresetGlow(1, 2);
                self.fillPresetGlow(1, 2);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 10:
            10 => {
                // Java: this.fillPresetGlow(2, 2);
                self.fillPresetGlow(2, 2);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 11:
            11 => {
                // Java: this.fillPresetGlow(3, 2);
                self.fillPresetGlow(3, 2);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 12:
            12 => {
                // Java: this.fillPresetGlow(0, 3);
                self.fillPresetGlow(0, 3);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 13:
            13 => {
                // Java: this.fillPresetGlow(1, 3);
                self.fillPresetGlow(1, 3);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 14:
            14 => {
                // Java: this.fillPresetGlow(2, 3);
                self.fillPresetGlow(2, 3);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 15:
            15 => {
                // Java: this.fillPresetGlow(3, 3);
                self.fillPresetGlow(3, 3);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 16:
            16 => {
                // Java: this.fillPresetGlow(0, 4);
                self.fillPresetGlow(0, 4);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 17:
            17 => {
                // Java: this.fillPresetGlow(1, 4);
                self.fillPresetGlow(1, 4);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 18:
            18 => {
                // Java: this.fillPresetGlow(2, 4);
                self.fillPresetGlow(2, 4);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 19:
            19 => {
                // Java: this.fillPresetGlow(3, 4);
                self.fillPresetGlow(3, 4);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 20:
            20 => {
                // Java: this.fillPresetGlow(0, 5);
                self.fillPresetGlow(0, 5);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 21:
            21 => {
                // Java: this.fillPresetGlow(1, 5);
                self.fillPresetGlow(1, 5);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 22:
            22 => {
                // Java: this.fillPresetGlow(2, 5);
                self.fillPresetGlow(2, 5);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 23:
            23 => {
                // Java: this.fillPresetGlow(3, 5);
                self.fillPresetGlow(3, 5);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: default:
            _ => {
                // Java: this.glowEffect.close();
                self.glowEffect.close();
                // Java: this.glowEffect = GlowEffect.create();
                self.glowEffect = GlowEffect::create();
            }
        }
    }
    // Java: int presetType()
    pub fn presetType(&mut self) -> i32 {
        // Java: if (!this.isEqualColor())
        if !self.isEqualColor() {
            // Java: return -1;
            return -1;
        }
        // Java: else
        else {
            // Java: int rad = this.glowEffect.radius();
            let rad = self.glowEffect.radius();
            // Java: int radIndex = this.getPresetRadIndex(rad);
            let radIndex = self.getPresetRadIndex(rad);
            // Java: if (radIndex == -1)
            if radIndex == -1 {
                // Java: return -1;
                return -1;
            }
            // Java: else
            else {
                // Java: int colorIndex = this.getPresetColorIndex(this.glowEffect.clr().scheme());
                // LLM: clr().scheme() depends on KProto chain; entire block aggregated
                // Java: return colorIndex == -1 ? -1 : this.getPresetTypeBy(radIndex, colorIndex);
                // LLM: colorIndex and radIndex are declared above; depends on KProto chain
                todo!()
            }
        }
    }
    // Java: int getPresetTypeBy(int radIndex, int colorIndex)
    fn getPresetTypeBy(&self, radIndex: i32, colorIndex: i32) -> i32 {
        // LLM: radIndex and colorIndex are declared above as function parameters
        // Java: if (radIndex == 0 && colorIndex == 0)
        if radIndex == 0 && colorIndex == 0 {
            // Java: return 0;
            return 0;
        }
        // Java: else if (radIndex == 1 && colorIndex == 0)
        else if radIndex == 1 && colorIndex == 0 {
            // Java: return 1;
            return 1;
        }
        // Java: else if (radIndex == 2 && colorIndex == 0)
        else if radIndex == 2 && colorIndex == 0 {
            // Java: return 2;
            return 2;
        }
        // Java: else if (radIndex == 3 && colorIndex == 0)
        else if radIndex == 3 && colorIndex == 0 {
            // Java: return 3;
            return 3;
        }
        // Java: else if (radIndex == 0 && colorIndex == 1)
        else if radIndex == 0 && colorIndex == 1 {
            // Java: return 4;
            return 4;
        }
        // Java: else if (radIndex == 1 && colorIndex == 1)
        else if radIndex == 1 && colorIndex == 1 {
            // Java: return 5;
            return 5;
        }
        // Java: else if (radIndex == 2 && colorIndex == 1)
        else if radIndex == 2 && colorIndex == 1 {
            // Java: return 6;
            return 6;
        }
        // Java: else if (radIndex == 3 && colorIndex == 1)
        else if radIndex == 3 && colorIndex == 1 {
            // Java: return 7;
            return 7;
        }
        // Java: else if (radIndex == 0 && colorIndex == 2)
        else if radIndex == 0 && colorIndex == 2 {
            // Java: return 8;
            return 8;
        }
        // Java: else if (radIndex == 1 && colorIndex == 2)
        else if radIndex == 1 && colorIndex == 2 {
            // Java: return 9;
            return 9;
        }
        // Java: else if (radIndex == 2 && colorIndex == 2)
        else if radIndex == 2 && colorIndex == 2 {
            // Java: return 10;
            return 10;
        }
        // Java: else if (radIndex == 3 && colorIndex == 2)
        else if radIndex == 3 && colorIndex == 2 {
            // Java: return 11;
            return 11;
        }
        // Java: else if (radIndex == 0 && colorIndex == 3)
        else if radIndex == 0 && colorIndex == 3 {
            // Java: return 12;
            return 12;
        }
        // Java: else if (radIndex == 1 && colorIndex == 3)
        else if radIndex == 1 && colorIndex == 3 {
            // Java: return 13;
            return 13;
        }
        // Java: else if (radIndex == 2 && colorIndex == 3)
        else if radIndex == 2 && colorIndex == 3 {
            // Java: return 14;
            return 14;
        }
        // Java: else if (radIndex == 3 && colorIndex == 3)
        else if radIndex == 3 && colorIndex == 3 {
            // Java: return 15;
            return 15;
        }
        // Java: else if (radIndex == 0 && colorIndex == 4)
        else if radIndex == 0 && colorIndex == 4 {
            // Java: return 16;
            return 16;
        }
        // Java: else if (radIndex == 1 && colorIndex == 4)
        else if radIndex == 1 && colorIndex == 4 {
            // Java: return 17;
            return 17;
        }
        // Java: else if (radIndex == 2 && colorIndex == 4)
        else if radIndex == 2 && colorIndex == 4 {
            // Java: return 18;
            return 18;
        }
        // Java: else if (radIndex == 3 && colorIndex == 4)
        else if radIndex == 3 && colorIndex == 4 {
            // Java: return 19;
            return 19;
        }
        // Java: else if (radIndex == 0 && colorIndex == 5)
        else if radIndex == 0 && colorIndex == 5 {
            // Java: return 20;
            return 20;
        }
        // Java: else if (radIndex == 1 && colorIndex == 5)
        else if radIndex == 1 && colorIndex == 5 {
            // Java: return 21;
            return 21;
        }
        // Java: else if (radIndex == 2 && colorIndex == 5)
        else if radIndex == 2 && colorIndex == 5 {
            // Java: return 22;
            return 22;
        }
        // Java: else
        else {
            // Java: return radIndex == 3 && colorIndex == 5 ? 23 : -1;
            // LLM: radIndex and colorIndex are declared above as function parameters
            if radIndex == 3 && colorIndex == 5 { 23 } else { -1 }
        }
    }
    // Java: int getPresetRadIndex(int rad)
    fn getPresetRadIndex(&self, rad: i32) -> i32 {
        // Java: for(int i = 0; i < this.radPreset.length; ++i)
        for i in 0..self.radPreset.len() {
            // Java: if (rad == this.radPreset[i])
            if rad == self.radPreset[i] {
                // Java: return i;
                return i as i32;
            }
        }
        // Java: return -1;
        -1
    }
    // Java: int getPresetColorIndex(int type)
    fn getPresetColorIndex(&self, r#type: i32) -> i32 {
        // Java: for(int i = 0; i < this.colorPreset.length; ++i)
        for i in 0..self.colorPreset.len() {
            // Java: if (type == this.colorPreset[i])
            if r#type == self.colorPreset[i] {
                // Java: return i;
                return i as i32;
            }
        }
        // Java: return -1;
        -1
    }
    // Java: boolean isEqualColor()
    fn isEqualColor(&mut self) -> bool {
        // Java: if (this.glowEffect.clr().mode() != 4)
        // LLM: clr().mode() depends on KProto chain; entire method aggregated
        // Java: return false;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: ColorTransform colorAlpha = DgColorFormat.getColorTransformByType(this.glowEffect.clr(), 0);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: ColorTransform colorSatMod = DgColorFormat.getColorTransformByType(this.glowEffect.clr(), 24);
        // LLM: DgColorFormat::getColorTransformByType depends on KProto chain
        // Java: return colorAlpha != null && colorAlpha.alpha() == PRESET_ALPHA && colorSatMod != null && colorSatMod.satMod() == PRESET_SAT_MOE;
        todo!()
    }
    // Java: void iniPresetData()
    fn iniPresetData(&mut self) {
        // Java: this.iniPresetRad();
        self.iniPresetRad();
        // Java: this.iniPresetColor();
        self.iniPresetColor();
    }
    // Java: void fillPresetGlow(int presetRadIndex, int presetColorIndex)
    fn fillPresetGlow(&mut self, presetRadIndex: i32, presetColorIndex: i32) {
        // Java: int rad = this.radPreset[presetRadIndex];
        let rad = self.radPreset[presetRadIndex as usize];
        // Java: int sysColorType = this.colorPreset[presetColorIndex];
        let sysColorType = self.colorPreset[presetColorIndex as usize];
        // Java: this.glowEffect.putRadius(rad);
        self.glowEffect.putRadius(rad);
        // Java: Color color = Color.create();
        // LLM: Color.create and KProto chain; remaining method aggregated
        // Java: color.putMode(4);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: color.putScheme(sysColorType);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: ColorTransform transform = ColorTransform.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: transform.putAlpha(PRESET_ALPHA);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: transform.putType(0);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgColorFormat.setColorTransformByType(color, transform);
        // LLM: DgColorFormat::setColorTransformByType depends on KProto chain
        // Java: transform = ColorTransform.create();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: transform.putSatMod(PRESET_SAT_MOE);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: transform.putType(24);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgColorFormat.setColorTransformByType(color, transform);
        // LLM: DgColorFormat::setColorTransformByType depends on KProto chain
        // Java: color.save();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.glowEffect.writeClr(color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.colorFormat.update(color.save());
        todo!()
    }
    // Java: void iniPresetRad()
    fn iniPresetRad(&mut self) {
        // Java: this.radPreset[0] = (int)UnitsConverter.pt2emu((double)5.0F);
        self.radPreset[0] = UnitsConverter::pt2emu(5.0) as i32;
        // Java: this.radPreset[1] = (int)UnitsConverter.pt2emu((double)8.0F);
        self.radPreset[1] = UnitsConverter::pt2emu(8.0) as i32;
        // Java: this.radPreset[2] = (int)UnitsConverter.pt2emu((double)11.0F);
        self.radPreset[2] = UnitsConverter::pt2emu(11.0) as i32;
        // Java: this.radPreset[3] = (int)UnitsConverter.pt2emu((double)18.0F);
        self.radPreset[3] = UnitsConverter::pt2emu(18.0) as i32;
    }
    // Java: void iniPresetColor()
    fn iniPresetColor(&mut self) {
        // Java: this.colorPreset[0] = 4;
        self.colorPreset[0] = 4;
        // Java: this.colorPreset[1] = 5;
        self.colorPreset[1] = 5;
        // Java: this.colorPreset[2] = 6;
        self.colorPreset[2] = 6;
        // Java: this.colorPreset[3] = 7;
        self.colorPreset[3] = 7;
        // Java: this.colorPreset[4] = 8;
        self.colorPreset[4] = 8;
        // Java: this.colorPreset[5] = 9;
        self.colorPreset[5] = 9;
    }
}

impl KmoProtocol for DgEffectGlow {
    fn protocol(&self) -> Box<dyn crate::cn::wps::kso::kproto::KoBin::KoBin> { todo!() }
    fn updateProtocol(&mut self, _var1: &dyn crate::cn::wps::kso::kproto::KoBin::KoBin) { todo!() }
}
