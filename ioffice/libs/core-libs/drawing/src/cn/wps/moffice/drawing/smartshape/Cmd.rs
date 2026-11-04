#![allow(non_snake_case, non_upper_case_globals)]

// Java: public class Cmd
pub struct Cmd {
}

// Java: public class Cmd
impl Cmd {
    // Java field: short moveTo
    pub const moveTo: i16 = 64;
    // Java field: short lineTo
    pub const lineTo: i16 = 0;
    // Java field: short curveTo
    pub const curveTo: i16 = 32;
    // Java field: short close
    pub const close: i16 = 96;
    // Java field: short end
    pub const end: i16 = 128;
    // Java field: short noFill
    pub const noFill: i16 = 170;
    // Java field: short noStroke
    pub const noStroke: i16 = 171;
    // Java field: short fillNorm
    pub const fillNorm: i16 = 240;
    // Java field: short fillDarken
    pub const fillDarken: i16 = 241;
    // Java field: short fillDarkenLess
    pub const fillDarkenLess: i16 = 242;
    // Java field: short fillLighten
    pub const fillLighten: i16 = 243;
    // Java field: short fillLightenLess
    pub const fillLightenLess: i16 = 244;
    // Java field: short angleEllipseTo
    pub const angleEllipseTo: i16 = 161;
    // Java field: short angleEllipse
    pub const angleEllipse: i16 = 162;
    // Java field: short arcTo
    pub const arcTo: i16 = 163;
    // Java field: short arc
    pub const arc: i16 = 164;
    // Java field: short clockwiseArcTo
    pub const clockwiseArcTo: i16 = 165;
    // Java field: short clockwiseArc
    pub const clockwiseArc: i16 = 166;
    // Java field: short ellipticalQaudrantX
    pub const ellipticalQaudrantX: i16 = 167;
    // Java field: short ellipticalQuadrantY
    pub const ellipticalQuadrantY: i16 = 168;
    // Java field: short quadraticBezier
    pub const quadraticBezier: i16 = 169;
    // Java field: short autoLine
    pub const autoLine: i16 = 172;
    // Java field: short aUtoCurve
    pub const aUtoCurve: i16 = 173;
    // Java field: short cornerLine
    pub const cornerLine: i16 = 174;
    // Java field: short cornerCurve
    pub const cornerCurve: i16 = 175;
    // Java field: short smoothLine
    pub const smoothLine: i16 = 176;
    // Java field: short smoothCurve
    pub const smoothCurve: i16 = 177;
    // Java field: short symmetricLine
    pub const symmetricLine: i16 = 178;
    // Java field: short symmetricCurve
    pub const symmetricCurve: i16 = 179;
    // Java field: short freeform
    pub const freeform: i16 = 180;

    // Java: Cmd()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }

    // Java: static int GetCmdParameterCount(short cmd)
    pub fn GetCmdParameterCount(cmd: i16) -> i32 {
        // Java: int count = 0;
        let mut count: i32 = 0;
        // Java: switch (cmd)
        'switch_cmd: {
            match cmd {
                // Java: case 0:
                0 |
                // Java: case 64:
                64 |
                // Java: case 167:
                167 |
                // Java: case 168:
                168 => {
                    // Java: count = 2;
                    count = 2;
                    // Java: break;
                    break 'switch_cmd;
                }
                // Java: case 32:
                32 |
                // Java: case 161:
                161 |
                // Java: case 162:
                162 => {
                    // Java: count = 6;
                    count = 6;
                    // Java: break;
                    break 'switch_cmd;
                }
                // Java: case 163:
                163 => {
                    // Java: count = 8;
                    count = 8;
                    // Java: break;
                    break 'switch_cmd;
                }
                // Java: case 164:
                164 |
                // Java: case 165:
                165 |
                // Java: case 166:
                166 => {
                    // Java: count = 8;
                    count = 8;
                    // Java: break;
                    break 'switch_cmd;
                }
                // Java: case 169:
                169 => {
                    // Java: count = 4;
                    count = 4;
                }
                _ => {}
            }
        }
        // Java: return count;
        count
    }
}
