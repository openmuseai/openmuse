// java public class ThemeColorUtil
pub struct ThemeColorUtil;
impl ThemeColorUtil {
    // java public static int colorToInt(String value, int radix)
    pub fn colorToInt(value: Option<&str>, _radix: i32) -> i32 {
        let Some(value) = value else {
            return 0;
        };
        let units: Vec<u16> = value.encode_utf16().collect();
        let rgb = match units.len() {
            8 => &units[2..],
            6 => &units[..],
            _ => return 0,
        };
        let pair =
            |s: &[u16]| i32::from_str_radix(String::from_utf16_lossy(s).trim(), 16).unwrap_or(0);
        (pair(&rgb[0..2]) << 16) + (pair(&rgb[2..4]) << 8) + pair(&rgb[4..6])
    }

    // java private static int MulDiv(int nMultiplicand,int nMultiplier,int nDivisor)
    fn MulDiv(mut a: i32, b: i32, mut c: i32) -> i32 {
        if c == 0 {
            return -1;
        }
        if c < 0 {
            a = -a;
            c = -c;
        }
        // java ret = ((long)nMultiplicand*(long)nMultiplier +/- (long)(nDivisor/2))/(long)nDivisor;
        let x = a as i64 * b as i64;
        let ret = (x + if (a >= 0 || b >= 0) && (a < 0 || b < 0) {
            -(c as i64 / 2)
        } else {
            c as i64 / 2
        }) / c as i64;
        if (-2147483647..=2147483647).contains(&ret) {
            ret as i32
        } else {
            -1
        }
    }
    // java public static int colorWithTint2RGB(int r,int g,int b,float t)
    pub fn colorWithTint2RGB(r: i32, g: i32, b: i32, t: f32) -> i32 {
        // java int truncTint=(int)(tint*32767.0F);if(truncTint==0)return (red<<16)+(green<<8)+blue;
        let truncTint = (t * 32767.0) as i32;
        if truncTint == 0 {
            return (r << 16) + (g << 8) + b;
        }
        // java rgb2hsl();
        let nMax = r.max(g).max(b);
        let nMin = r.min(g).min(b);
        let delta1 = nMax - nMin;
        let delta2 = nMax + nMin;
        let mut dl = Self::MulDiv(delta2, 240, 510);
        let (ds, dh) = if delta1 == 0 {
            (0, 160)
        } else {
            let tmpNum = if dl > 120 { 510 - delta2 } else { delta2 };
            let ds = Self::MulDiv(delta1, 240, tmpNum);
            let redHue = Self::MulDiv(nMax - r, 40, delta1);
            let greenHue = Self::MulDiv(nMax - g, 40, delta1);
            let blueHue = Self::MulDiv(nMax - b, 40, delta1);
            let tmpHue = if nMax == r {
                blueHue - greenHue
            } else if nMax == g {
                redHue - blueHue + 80
            } else {
                greenHue - redHue + 160
            };
            let mut dh = tmpHue + if tmpHue < 0 { 240 } else { 0 };
            if dh > 240 {
                dh -= 240;
            }
            (ds, dh)
        };
        // java if(truncTint<0)dl=dl*(short)(truncTint+32767)/32767;else dl=dl*(short)(32767-truncTint)/32767-240*(short)(32767-truncTint)/32767+240;
        if truncTint < 0 {
            dl = dl * ((truncTint + 32767) as i16 as i32) / 32767;
        } else {
            let factor = (32767 - truncTint) as i16 as i32;
            dl = dl * factor / 32767 - 240 * factor / 32767 + 240;
        }
        // java hsl2rgb(dh,ds,dl);
        let mut rgb = [0; 3];
        if ds == 0 {
            rgb.fill(Self::MulDiv(dl, 255, 240));
        } else {
            let mut tmp1 = Self::MulDiv(dl, ds + 240, 240);
            if dl > 120 {
                tmp1 = dl + ds - Self::MulDiv(dl, ds, 240);
            }
            let tmp2 = 2 * dl - tmp1;
            for (i, w) in [dh + 80, dh, dh - 80].into_iter().enumerate() {
                let mut weight = w + if w < 0 { 240 } else { 0 };
                if weight > 240 {
                    weight -= 240;
                }
                let clr = if weight < 40 {
                    tmp2 + Self::MulDiv(tmp1 - tmp2, weight, 40)
                } else if weight < 120 {
                    tmp1
                } else if weight < 160 {
                    tmp2 + Self::MulDiv(tmp1 - tmp2, 160 - weight, 40)
                } else {
                    tmp2
                };
                rgb[i] = Self::MulDiv(clr, 255, 240);
            }
        }
        // java if(red<0)red+=256; if(green<0)green+=256;if(blue<0)blue+=256;return(red<<16)+(green<<8)+blue;
        for x in &mut rgb {
            if *x < 0 {
                *x += 256;
            }
        }
        (rgb[0] << 16) + (rgb[1] << 8) + rgb[2]
    }
}
