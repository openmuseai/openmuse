#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.app.eval.DgFormatLocator
// LLM: DgFormatLocator is not yet available, not used in this file
// Java: import cn.wps.drawing.app.eval.DgFormatMixer
// LLM: DgFormatMixer is not yet available, not used in this file
// Java: import cn.wps.drawing.app.eval.color.DgColorEval
use crate::cn::wps::drawing::app::eval::color::DgColorEval::DgColorEval;
// Java: import cn.wps.drawing.app.util.PrecisionUtil
use crate::cn::wps::drawing::app::util::PrecisionUtil::PrecisionUtil;
// Java: import cn.wps.drawing.data.color.Color
use crate::cn::wps::drawing::data::color::Color::Color;
// Java: import cn.wps.drawing.data.color.ColorTransform
use crate::cn::wps::drawing::data::color::ColorTransform::ColorTransform;
// Java: import cn.wps.kso.kproto.KoBin
// LLM: KoBin is from kso kproto, using stub
// Java: import java.util.ArrayList
// LLM: using Vec
// Java: import java.util.List
// LLM: using Vec

pub struct DgColor;
impl DgColor {
    pub fn r#type(value: i32) -> i32 { (value >> 24) & 0xFF }
    pub fn r(value: i32) -> i32 { (value >> 16) & 0xFF }
    pub fn g(value: i32) -> i32 { (value >> 8) & 0xFF }
    pub fn b(value: i32) -> i32 { value & 0xFF }
    pub fn indexValue(value: i32) -> i32 { value & 0xFFFFFF }
    pub fn rgb(r: i32, g: i32, b: i32) -> i32 { (1 << 24) | (r << 16) | (g << 8) | b }
    pub fn scheme(idx: i32) -> i32 { (4 << 24) | idx }
    pub const WHITE: i32 = (1 << 24) | 0xFFFFFF;
}

pub struct KoBin;

pub trait Cloneable {}

// Java: public class DgColorFormat implements Cloneable
pub struct DgColorFormat {
    // Java field: Color color
    pub(crate) color: Color,
}

// Java: public class DgColorFormat implements Cloneable
impl DgColorFormat {
    // Java: DgColorFormat()
    pub fn DgColorFormat_new() -> Self {
        DgColorFormat { color: Color::Color() }
    }

    // Java: DgColorFormat(int value)
    pub fn DgColorFormat_from_int(value: i32) -> Self {
        let mut _self = DgColorFormat::DgColorFormat_new();
        // Java: this.putColor(value);
        _self.putColor(value);
        _self
    }

    // Java: DgColorFormat(Color color)
    pub fn DgColorFormat_from_color(color: Option<&Color>) -> Self {
        let mut _self = DgColorFormat::DgColorFormat_new();
        // Java: if (color != null)
        if let Some(c) = color {
            // Java: this.update(color.save());
            // LLM: color.save() returns KoBin; KoBin serialization not yet available
            todo!()
        }
        _self
    }

    // Java: void update(KoBin data)
    pub fn update(&mut self, _data: KoBin) {
        // Java: this.color.load(data);
        // LLM: KoBin serialization not yet available
        todo!()
    }

    // Java: KoBin data()
    pub fn data(&self) -> KoBin {
        // Java: return this.color.save();
        // LLM: KoBin serialization not yet available
        todo!()
    }

    // Java: Color getColor()
    pub fn getColor(&self) -> &Color {
        // Java: return this.color;
        &self.color
    }

    // Java: void putColor(int value)
    pub fn putColor(&mut self, value: i32) {
        // Java: switch (DgColor.type(value))
        match DgColor::r#type(value) {
            // Java: case 1:
            1 => {
                // Java: this.color.putMode(1);
                self.color.putMode(1);
                // Java: this.color.srgb().putR(DgColor.r(value));
                self.color.srgb().putR(DgColor::r(value));
                // Java: this.color.srgb().putG(DgColor.g(value));
                self.color.srgb().putG(DgColor::g(value));
                // Java: this.color.srgb().putB(DgColor.b(value));
                self.color.srgb().putB(DgColor::b(value));
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 2:
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: default:
            // LLM JavaLine Aggregation: merged into the next Rust statement
            2 => {
                // Java: throw new IllegalArgumentException();
                panic!("IllegalArgumentException");
            }
            // Java: case 3:
            3 => {
                // Java: this.color.putMode(3);
                self.color.putMode(3);
                // Java: this.color.putSys(DgColor.indexValue(value));
                self.color.putSys(DgColor::indexValue(value));
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 4:
            4 => {
                // Java: this.color.putMode(4);
                self.color.putMode(4);
                // Java: this.color.putScheme(DgColor.indexValue(value));
                self.color.putScheme(DgColor::indexValue(value));
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 5:
            5 => {
                // Java: this.color.putMode(5);
                self.color.putMode(5);
                // Java: this.color.putPrst(DgColor.indexValue(value));
                self.color.putPrst(DgColor::indexValue(value));
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: default:
            _ => {
                // Java: throw new IllegalArgumentException();
                panic!("IllegalArgumentException");
            }
        }
    }

    // Java: int type()
    pub fn r#type(&self) -> i32 {
        // Java: return this.color.mode();
        self.color.mode()
    }

    // Java: int schemeColor()
    pub fn schemeColor(&mut self) -> i32 {
        // Java: return DgColorEval.toScheme(this.color);
        DgColorEval::toScheme(&mut self.color)
    }

    // Java: int rgb()
    pub fn rgb(&mut self) -> i32 {
        // Java: return DgColorEval.toRGB(this.color);
        DgColorEval::toRGB(&mut self.color)
    }

    // Java: double tint()
    pub fn tint(&mut self) -> f64 {
        // Java: ColorTransform transform = getColorTransformByType(this.color, 27);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return transform == null ? (double)0.0F : transform.tint();
        let transform = DgColorFormat::getColorTransformByType(&mut self.color, 27);
        transform.map_or(0.0, |t| t.tint())
    }

    // Java: void putTint(double value)
    pub fn putTint(&mut self, value: f64) {
        // Java: ColorTransform transform = ColorTransform.create();
        let mut transform = ColorTransform::create();
        // Java: transform.putTint(value);
        transform.putTint(value);
        // Java: transform.putType(27);
        transform.putType(27);
        // Java: setColorTransformByType(this.color, transform);
        DgColorFormat::setColorTransformByType(&mut self.color, transform);
    }

    // Java: double brightness()
    pub fn brightness(&mut self) -> f64 {
        // Java: ColorTransform colorTransform = getColorTransformByType(this.color, 17);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return colorTransform == null ? (double)0.0F : colorTransform.lum();
        let colorTransform = DgColorFormat::getColorTransformByType(&mut self.color, 17);
        colorTransform.map_or(0.0, |t| t.lum())
    }

    // Java: void putBrightness(double value)
    pub fn putBrightness(&mut self, value: f64) {
        // Java: ColorTransform transform = ColorTransform.create();
        let mut transform = ColorTransform::create();
        // Java: transform.putLum(value);
        transform.putLum(value);
        // Java: transform.putType(17);
        transform.putType(17);
        // Java: setColorTransformByType(this.color, transform);
        DgColorFormat::setColorTransformByType(&mut self.color, transform);
    }

    // Java: double shade()
    pub fn shade(&mut self) -> f64 {
        // Java: ColorTransform colorTransform = getColorTransformByType(this.color, 26);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return colorTransform == null ? (double)0.0F : colorTransform.shade();
        let colorTransform = DgColorFormat::getColorTransformByType(&mut self.color, 26);
        colorTransform.map_or(0.0, |t| t.shade())
    }

    // Java: void putShade(double shade)
    pub fn putShade(&mut self, shade: f64) {
        // Java: ColorTransform transform = ColorTransform.create();
        let mut transform = ColorTransform::create();
        // Java: transform.putShade(shade);
        transform.putShade(shade);
        // Java: transform.putType(26);
        transform.putType(26);
        // Java: setColorTransformByType(this.color, transform);
        DgColorFormat::setColorTransformByType(&mut self.color, transform);
    }

    // Java: double alpha()
    pub fn alpha(&mut self) -> f64 {
        // Java: ColorTransform colorTransform = getColorTransformByType(this.color, 0);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return colorTransform == null ? (double)0.0F : PrecisionUtil.sub((double)1.0F, colorTransform.alpha());
        let colorTransform = DgColorFormat::getColorTransformByType(&mut self.color, 0);
        colorTransform.map_or(0.0, |t| PrecisionUtil::sub(1.0, t.alpha()))
    }

    // Java: void putAlpha(double alpha)
    pub fn putAlpha(&mut self, alpha: f64) {
        // Java: ColorTransform transform = ColorTransform.create();
        let mut transform = ColorTransform::create();
        // Java: transform.putAlpha(PrecisionUtil.sub((double)1.0F, alpha));
        transform.putAlpha(PrecisionUtil::sub(1.0, alpha));
        // Java: transform.putType(0);
        transform.putType(0);
        // Java: setColorTransformByType(this.color, transform);
        DgColorFormat::setColorTransformByType(&mut self.color, transform);
    }

    // Java: static void setColorTransformByType(Color color, ColorTransform colorTransform)
    pub fn setColorTransformByType(color: &mut Color, colorTransform: ColorTransform) {
        // Java: List<ColorTransform> lst = new ArrayList();
        let mut lst: Vec<ColorTransform> = Vec::new();
        // Java: color.transformLst().read(lst);
        color.transformLst().read(&mut lst);
        // Java: boolean isExit = false;
        let mut isExit = false;
        let target_type = colorTransform.get_type();
        let mut found_index = 0usize;
        // Java: for(int i = 0; i < lst.size(); ++i)
        for i in 0..lst.len() {
            // Java: ColorTransform clrTran = (ColorTransform)lst.get(i);
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: if (clrTran.type() == colorTransform.type())
            if lst[i].get_type() == target_type {
                // Java: clrTran.load(colorTransform.save());
                found_index = i;
                // Java: isExit = true;
                isExit = true;
                // Java: break;
                break;
            }
        }
        // Java: if (!isExit)
        if !isExit {
            // Java: colorTransform.save();
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: lst.add(colorTransform);
            lst.push(colorTransform);
        } else {
            // LLM: deferred assignment from loop body for Rust ownership
            lst[found_index] = colorTransform;
        }
        // Java: color.transformLst().write(lst);
        color.transformLst().write(&lst);
    }

    // Java: static ColorTransform getColorTransformByType(Color color, int type)
    pub fn getColorTransformByType(color: &mut Color, r#type: i32) -> Option<ColorTransform> {
        // Java: List<ColorTransform> lst = new ArrayList();
        let mut lst: Vec<ColorTransform> = Vec::new();
        // Java: color.transformLst().read(lst);
        color.transformLst().read(&mut lst);
        // Java: for(int i = 0; i < lst.size(); ++i)
        for i in 0..lst.len() {
            // Java: ColorTransform colorTransform = (ColorTransform)lst.get(i);
            let colorTransform = &lst[i];
            // Java: if (colorTransform.type() == type)
            if colorTransform.get_type() == r#type {
                // Java: return colorTransform;
                return Some(lst.remove(i));
            }
        }
        // Java: return null;
        None
    }

    // Java: static void delColorTransformByType(Color color, int type)
    pub fn delColorTransformByType(color: &mut Color, r#type: i32) {
        // Java: List<ColorTransform> lst = new ArrayList();
        let mut lst: Vec<ColorTransform> = Vec::new();
        // Java: color.transformLst().read(lst);
        color.transformLst().read(&mut lst);
        // Java: for(int i = 0; i < lst.size(); ++i)
        lst.retain(|ct| {
            // Java: ColorTransform colorTransform = (ColorTransform)lst.get(i);
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: if (colorTransform.type() == type)
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: lst.remove(colorTransform);
            ct.get_type() != r#type
        });
        // Java: color.transformLst().write(lst);
        color.transformLst().write(&lst);
    }

    // Java: DgColorFormat clone()
    pub fn clone(&self) -> DgColorFormat {
        // Java: DgColorFormat format = new DgColorFormat();
        let mut format = DgColorFormat::DgColorFormat_new();
        // Java: format.update(this.data());
        // LLM: KoBin serialization not yet available
        // Java: return format;
        format
    }

    // Java: static boolean isEqual(DgColorFormat format1, DgColorFormat format2)
    pub fn isEqual(format1: Option<&mut DgColorFormat>, format2: Option<&mut DgColorFormat>) -> bool {
        // Java: if (format1 != null && format2 != null)
        if format1.is_some() && format2.is_some() {
            let f1 = format1.unwrap();
            let f2 = format2.unwrap();
            // Java: if (format1.type() != format2.type())
            if f1.r#type() != f2.r#type() {
                // Java: return false;
                return false;
            }
            // Java: else if (format1.rgb() != format2.rgb())
            else if f1.rgb() != f2.rgb() {
                // Java: return false;
                return false;
            }
            // Java: else if (format1.alpha() != format2.alpha())
            else if f1.alpha() != f2.alpha() {
                // Java: return false;
                return false;
            }
            // Java: else if (format1.brightness() != format2.brightness())
            else if f1.brightness() != f2.brightness() {
                // Java: return false;
                return false;
            }
            // Java: else if (format1.schemeColor() != format2.schemeColor())
            else if f1.schemeColor() != f2.schemeColor() {
                // Java: return false;
                return false;
            }
            // Java: else if (format1.tint() != format2.tint())
            else if f1.tint() != f2.tint() {
                // Java: return false;
                return false;
            }
            // Java: else
            else {
                // Java: return PrecisionUtil.equal(format1.shade(), format2.shade());
                PrecisionUtil::equal(f1.shade(), f2.shade())
            }
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }
}

impl Cloneable for DgColorFormat {}
