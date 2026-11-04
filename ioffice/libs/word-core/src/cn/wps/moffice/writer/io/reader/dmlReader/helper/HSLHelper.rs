#![allow(non_snake_case)]
use crate::cn::wps::moffice::writer::io::tools::ColorTransform::ColorTransform as IOColorTransform;

// Java: public class HSLHelper
pub struct HSLHelper { m_HSL: HSL }

// Java: class HSL { float H; float S; float L; }
#[derive(Clone, Copy)]
struct HSL { H: f32, S: f32, L: f32 }

fn argb(r: i32, g: i32, b: i32, alpha: i32) -> i32 {
    IOColorTransform::argb(r, g, b, alpha)
}

impl HSLHelper {
    // Java: public HSLHelper(int _RGB)
    pub fn new(rgb: i32) -> Self {
        let var_r = IOColorTransform::getRed(rgb) as f32 / 255.0;
        let var_g = IOColorTransform::getGreen(rgb) as f32 / 255.0;
        let var_b = IOColorTransform::getBlue(rgb) as f32 / 255.0;
        let var_min = var_r.min(var_g).min(var_b);
        let var_max = var_r.max(var_g).max(var_b);
        let del_max = var_max - var_min;
        let l = (var_max + var_min) / 2.0;
        let (mut h, mut s) = (0.0, 0.0);
        if del_max != 0.0 {
            s = if l < 0.5 { del_max / (var_max + var_min) } else { del_max / (2.0 - var_max - var_min) };
            let del_r = ((var_max - var_r) / 6.0 + del_max / 2.0) / del_max;
            let del_g = ((var_max - var_g) / 6.0 + del_max / 2.0) / del_max;
            let del_b = ((var_max - var_b) / 6.0 + del_max / 2.0) / del_max;
            if var_r == var_max { h = del_b - del_g; }
            else if var_g == var_max { h = 0.33333334 + del_r - del_b; }
            else if var_b == var_max { h = 0.6666667 + del_g - del_r; }
            if h < 0.0 { h += 1.0; }
            if h > 1.0 { h -= 1.0; }
        }
        Self { m_HSL: HSL { H: h, S: s, L: l } }
    }

    pub fn H(&self) -> f32 { self.m_HSL.H }
    pub fn S(&self) -> f32 { self.m_HSL.S }
    pub fn L(&self) -> f32 { self.m_HSL.L }
    pub fn setH(&mut self, h: f32) { self.m_HSL.H = h; }
    pub fn setS(&mut self, s: f32) { self.m_HSL.S = s; }
    pub fn setL(&mut self, l: f32) { self.m_HSL.L = l; }

    // Java: public int toRGB()
    pub fn toRGB(&self) -> i32 {
        let HSL { H: h, S: s, L: l } = self.m_HSL;
        if h.total_cmp(&1.0).is_eq() { return argb(255, 255, 255, 255); }
        if s < 0.0 {
            let q = if (l as f64) < (0.5_f32 as f64) { l * (1.0 - s) } else { l - s + l * s };
            let p = 2.0 * l - q;
            let r = (2.0 * l - self._HUE2RGB(p, q, h + 0.33333334)).clamp(0.0, 1.0);
            let g = (2.0 * l - self._HUE2RGB(p, q, h)).clamp(0.0, 1.0);
            let b = (2.0 * l - self._HUE2RGB(p, q, h - 0.33333334)).clamp(0.0, 1.0);
            return argb((r as i32) * 255, (g as i32) * 255, (b as i32) * 255, 255);
        }
        if l.total_cmp(&0.0).is_eq() { return argb(0, 0, 0, 255); }
        let q = if (l as f64) < (0.5_f32 as f64) { l * (1.0 + s) } else { l + s - l * s };
        let p = 2.0 * l - q;
        let r = self._HUE2RGB(p, q, h + 0.33333334).clamp(0.0, 1.0);
        let g = self._HUE2RGB(p, q, h).clamp(0.0, 1.0);
        let b = self._HUE2RGB(p, q, h - 0.33333334).clamp(0.0, 1.0);
        argb((r * 255.0) as i32, (g * 255.0) as i32, (b * 255.0) as i32, 255)
    }

    // Java: private float _HUE2RGB(float v1, float v2, float vH)
    fn _HUE2RGB(&self, v1: f32, v2: f32, mut vH: f32) -> f32 {
        if vH < 0.0 { vH += 1.0; }
        if vH > 1.0 { vH -= 1.0; }
        if 6.0 * vH < 1.0 { v1 + (v2 - v1) * 6.0 * vH }
        else if 2.0 * vH < 1.0 { v2 }
        else if 3.0 * vH < 2.0 { v1 + (v2 - v1) * (0.6666667 - vH) * 6.0 }
        else { v1 }
    }
}
