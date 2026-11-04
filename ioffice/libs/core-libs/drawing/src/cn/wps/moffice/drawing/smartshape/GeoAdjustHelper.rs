#![allow(non_snake_case)]

use super::GeoAdjust::GeoAdjust;

// Java: public class GeoAdjustHelper
pub struct GeoAdjustHelper {
}

// Java: public class GeoAdjustHelper
impl GeoAdjustHelper {
    // Java: GeoAdjustHelper()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }

    // Java: static void addDefaultGeoAdjust(GeoAdjust geoAdjust, int shapeType)
    pub fn addDefaultGeoAdjust(geoAdjust: &mut GeoAdjust, shapeType: i32) {
        // Java: switch (shapeType)
        match shapeType {
            // Java: case 2:
            2 => {
                // Java: geoAdjust.setAdjust(0, 3600);
                geoAdjust.setAdjust(0, 3600);
            }
            3 | // Java: case 3:
            4 | // Java: case 4:
            6 | // Java: case 6:
            12 | // Java: case 12:
            14 | // Java: case 14:
            15 | // Java: case 15:
            18 | // Java: case 18:
            20 | // Java: case 20:
            24 | // Java: case 24:
            25 | // Java: case 25:
            26 | // Java: case 26:
            27 | // Java: case 27:
            28 | // Java: case 28:
            29 | // Java: case 29:
            30 | // Java: case 30:
            31 | // Java: case 31:
            32 | // Java: case 32:
            33 | // Java: case 33:
            37 | // Java: case 37:
            55 | // Java: case 55:
            56 | // Java: case 56:
            61 | // Java: case 61:
            63 | // Java: case 63:
            71 | // Java: case 71:
            72 | // Java: case 72:
            73 | // Java: case 73:
            74 | // Java: case 74:
            75 | // Java: case 75:
            100 | // Java: case 100:
            101 | // Java: case 101:
            109 | // Java: case 109:
            110 | // Java: case 110:
            111 | // Java: case 111:
            112 | // Java: case 112:
            113 | // Java: case 113:
            114 | // Java: case 114:
            115 | // Java: case 115:
            116 | // Java: case 116:
            117 | // Java: case 117:
            118 | // Java: case 118:
            119 | // Java: case 119:
            120 | // Java: case 120:
            121 | // Java: case 121:
            122 | // Java: case 122:
            123 | // Java: case 123:
            124 | // Java: case 124:
            125 | // Java: case 125:
            126 | // Java: case 126:
            127 | // Java: case 127:
            128 | // Java: case 128:
            129 | // Java: case 129:
            130 | // Java: case 130:
            131 | // Java: case 131:
            132 | // Java: case 132:
            133 | // Java: case 133:
            134 | // Java: case 134:
            135 | // Java: case 135:
            136 | // Java: case 136:
            177 => { // Java: case 177:
                // Java: break;
            }
            // Java: case 5:
            5 => {
                // Java: geoAdjust.setAdjust(0, 10800);
                geoAdjust.setAdjust(0, 10800);
                // Java: break;
                return;
            }
            // Java: case 7:
            7 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: break;
                return;
            }
            // Java: case 8:
            8 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: break;
                return;
            }
            // Java: case 9:
            9 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: break;
                return;
            }
            // Java: case 10:
            10 => {
                // Java: geoAdjust.setAdjust(0, 6326);
                geoAdjust.setAdjust(0, 6326);
                // Java: break;
                return;
            }
            // Java: case 11:
            11 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: break;
                return;
            }
            // Java: case 13:
            13 => {
                // Java: geoAdjust.setAdjust(0, 16200);
                geoAdjust.setAdjust(0, 16200);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: break;
                return;
            }
            // Java: case 16:
            16 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: break;
                return;
            }
            // Java: case 17:
            17 => {
                // Java: geoAdjust.setAdjust(0, 1800);
                geoAdjust.setAdjust(0, 1800);
                // Java: break;
                return;
            }
            // Java: case 19:
            19 => {
                // Java: geoAdjust.setAdjust(0, -5898240);
                geoAdjust.setAdjust(0, -5898240);
                // Java: geoAdjust.setAdjust(1, 0);
                geoAdjust.setAdjust(1, 0);
                // Java: geoAdjust.setAdjust(2, 0);
                geoAdjust.setAdjust(2, 0);
                // Java: geoAdjust.setAdjust(3, 21600);
                geoAdjust.setAdjust(3, 21600);
                // Java: geoAdjust.setAdjust(4, 21600);
                geoAdjust.setAdjust(4, 21600);
                // Java: break;
                return;
            }
            // Java: case 21:
            21 => {
                // Java: geoAdjust.setAdjust(0, 3600);
                geoAdjust.setAdjust(0, 3600);
                // Java: break;
                return;
            }
            // Java: case 22:
            22 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: break;
                return;
            }
            // Java: case 23:
            23 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: break;
                return;
            }
            // Java: case 34:
            34 => {
                // Java: geoAdjust.setAdjust(0, 10800);
                geoAdjust.setAdjust(0, 10800);
                // Java: break;
                return;
            }
            // Java: case 35:
            35 => {
                // Java: geoAdjust.setAdjust(0, 10800);
                geoAdjust.setAdjust(0, 10800);
                // Java: geoAdjust.setAdjust(1, 10800);
                geoAdjust.setAdjust(1, 10800);
                // Java: break;
                return;
            }
            // Java: case 36:
            36 => {
                // Java: geoAdjust.setAdjust(0, 10800);
                geoAdjust.setAdjust(0, 10800);
                // Java: geoAdjust.setAdjust(1, 10800);
                geoAdjust.setAdjust(1, 10800);
                // Java: geoAdjust.setAdjust(2, 10800);
                geoAdjust.setAdjust(2, 10800);
                // Java: break;
                return;
            }
            // Java: case 38:
            38 => {
                // Java: geoAdjust.setAdjust(0, 10800);
                geoAdjust.setAdjust(0, 10800);
                // Java: break;
                return;
            }
            // Java: case 39:
            39 => {
                // Java: geoAdjust.setAdjust(0, 6520);
                geoAdjust.setAdjust(0, 6520);
                // Java: geoAdjust.setAdjust(1, 25412);
                geoAdjust.setAdjust(1, 25412);
                // Java: geoAdjust.setAdjust(2, -78949);
                geoAdjust.setAdjust(2, -78949);
                // Java: break;
                return;
            }
            // Java: case 40:
            40 => {
                // Java: geoAdjust.setAdjust(0, -4024);
                geoAdjust.setAdjust(0, -4024);
                // Java: geoAdjust.setAdjust(1, 12564);
                geoAdjust.setAdjust(1, 12564);
                // Java: geoAdjust.setAdjust(2, 26406);
                geoAdjust.setAdjust(2, 26406);
                // Java: break;
                return;
            }
            // Java: case 41:
            41 => {
                // Java: geoAdjust.setAdjust(0, -8280);
                geoAdjust.setAdjust(0, -8280);
                // Java: geoAdjust.setAdjust(1, 24300);
                geoAdjust.setAdjust(1, 24300);
                // Java: geoAdjust.setAdjust(2, -1800);
                geoAdjust.setAdjust(2, -1800);
                // Java: geoAdjust.setAdjust(3, 4050);
                geoAdjust.setAdjust(3, 4050);
                // Java: break;
                return;
            }
            // Java: case 42:
            42 => {
                // Java: geoAdjust.setAdjust(0, -10080);
                geoAdjust.setAdjust(0, -10080);
                // Java: geoAdjust.setAdjust(1, 24300);
                geoAdjust.setAdjust(1, 24300);
                // Java: geoAdjust.setAdjust(2, -3600);
                geoAdjust.setAdjust(2, -3600);
                // Java: geoAdjust.setAdjust(3, 4050);
                geoAdjust.setAdjust(3, 4050);
                // Java: geoAdjust.setAdjust(4, -1800);
                geoAdjust.setAdjust(4, -1800);
                // Java: geoAdjust.setAdjust(5, 4050);
                geoAdjust.setAdjust(5, 4050);
                // Java: break;
                return;
            }
            // Java: case 43:
            43 => {
                // Java: geoAdjust.setAdjust(0, 23400);
                geoAdjust.setAdjust(0, 23400);
                // Java: geoAdjust.setAdjust(1, 24400);
                geoAdjust.setAdjust(1, 24400);
                // Java: geoAdjust.setAdjust(2, 25200);
                geoAdjust.setAdjust(2, 25200);
                // Java: geoAdjust.setAdjust(3, 21600);
                geoAdjust.setAdjust(3, 21600);
                // Java: geoAdjust.setAdjust(4, 25200);
                geoAdjust.setAdjust(4, 25200);
                // Java: geoAdjust.setAdjust(5, 4050);
                geoAdjust.setAdjust(5, 4050);
                // Java: geoAdjust.setAdjust(6, 23400);
                geoAdjust.setAdjust(6, 23400);
                // Java: geoAdjust.setAdjust(7, 4050);
                geoAdjust.setAdjust(7, 4050);
                // Java: break;
                return;
            }
            // Java: case 44:
            44 => {
                // Java: geoAdjust.setAdjust(0, -8280);
                geoAdjust.setAdjust(0, -8280);
                // Java: geoAdjust.setAdjust(1, 24300);
                geoAdjust.setAdjust(1, 24300);
                // Java: geoAdjust.setAdjust(2, -1800);
                geoAdjust.setAdjust(2, -1800);
                // Java: geoAdjust.setAdjust(3, 4050);
                geoAdjust.setAdjust(3, 4050);
                // Java: break;
                return;
            }
            // Java: case 45:
            45 => {
                // Java: geoAdjust.setAdjust(0, -10080);
                geoAdjust.setAdjust(0, -10080);
                // Java: geoAdjust.setAdjust(1, 24300);
                geoAdjust.setAdjust(1, 24300);
                // Java: geoAdjust.setAdjust(2, -3600);
                geoAdjust.setAdjust(2, -3600);
                // Java: geoAdjust.setAdjust(3, 4050);
                geoAdjust.setAdjust(3, 4050);
                // Java: geoAdjust.setAdjust(4, -1800);
                geoAdjust.setAdjust(4, -1800);
                // Java: geoAdjust.setAdjust(5, 4050);
                geoAdjust.setAdjust(5, 4050);
                // Java: break;
                return;
            }
            // Java: case 46:
            46 => {
                // Java: geoAdjust.setAdjust(0, 23400);
                geoAdjust.setAdjust(0, 23400);
                // Java: geoAdjust.setAdjust(1, 24400);
                geoAdjust.setAdjust(1, 24400);
                // Java: geoAdjust.setAdjust(2, 25200);
                geoAdjust.setAdjust(2, 25200);
                // Java: geoAdjust.setAdjust(3, 21600);
                geoAdjust.setAdjust(3, 21600);
                // Java: geoAdjust.setAdjust(4, 25200);
                geoAdjust.setAdjust(4, 25200);
                // Java: geoAdjust.setAdjust(5, 4050);
                geoAdjust.setAdjust(5, 4050);
                // Java: geoAdjust.setAdjust(6, 23400);
                geoAdjust.setAdjust(6, 23400);
                // Java: geoAdjust.setAdjust(7, 4050);
                geoAdjust.setAdjust(7, 4050);
                // Java: break;
                return;
            }
            // Java: case 47:
            47 => {
                // Java: geoAdjust.setAdjust(0, -8280);
                geoAdjust.setAdjust(0, -8280);
                // Java: geoAdjust.setAdjust(1, 24300);
                geoAdjust.setAdjust(1, 24300);
                // Java: geoAdjust.setAdjust(2, -1800);
                geoAdjust.setAdjust(2, -1800);
                // Java: geoAdjust.setAdjust(3, 4050);
                geoAdjust.setAdjust(3, 4050);
                // Java: break;
                return;
            }
            // Java: case 48:
            48 => {
                // Java: geoAdjust.setAdjust(0, -10080);
                geoAdjust.setAdjust(0, -10080);
                // Java: geoAdjust.setAdjust(1, 24300);
                geoAdjust.setAdjust(1, 24300);
                // Java: geoAdjust.setAdjust(2, -3600);
                geoAdjust.setAdjust(2, -3600);
                // Java: geoAdjust.setAdjust(3, 4050);
                geoAdjust.setAdjust(3, 4050);
                // Java: geoAdjust.setAdjust(4, -1800);
                geoAdjust.setAdjust(4, -1800);
                // Java: geoAdjust.setAdjust(5, 4050);
                geoAdjust.setAdjust(5, 4050);
                // Java: break;
                return;
            }
            // Java: case 49:
            49 => {
                // Java: geoAdjust.setAdjust(0, 23400);
                geoAdjust.setAdjust(0, 23400);
                // Java: geoAdjust.setAdjust(1, 24400);
                geoAdjust.setAdjust(1, 24400);
                // Java: geoAdjust.setAdjust(2, 25200);
                geoAdjust.setAdjust(2, 25200);
                // Java: geoAdjust.setAdjust(3, 21600);
                geoAdjust.setAdjust(3, 21600);
                // Java: geoAdjust.setAdjust(4, 25200);
                geoAdjust.setAdjust(4, 25200);
                // Java: geoAdjust.setAdjust(5, 4050);
                geoAdjust.setAdjust(5, 4050);
                // Java: geoAdjust.setAdjust(6, 23400);
                geoAdjust.setAdjust(6, 23400);
                // Java: geoAdjust.setAdjust(7, 4050);
                geoAdjust.setAdjust(7, 4050);
                // Java: break;
                return;
            }
            // Java: case 50:
            50 => {
                // Java: geoAdjust.setAdjust(0, -8280);
                geoAdjust.setAdjust(0, -8280);
                // Java: geoAdjust.setAdjust(1, 24300);
                geoAdjust.setAdjust(1, 24300);
                // Java: geoAdjust.setAdjust(2, -1800);
                geoAdjust.setAdjust(2, -1800);
                // Java: geoAdjust.setAdjust(3, 4050);
                geoAdjust.setAdjust(3, 4050);
                // Java: break;
                return;
            }
            // Java: case 51:
            51 => {
                // Java: geoAdjust.setAdjust(0, -10080);
                geoAdjust.setAdjust(0, -10080);
                // Java: geoAdjust.setAdjust(1, 24300);
                geoAdjust.setAdjust(1, 24300);
                // Java: geoAdjust.setAdjust(2, -3600);
                geoAdjust.setAdjust(2, -3600);
                // Java: geoAdjust.setAdjust(3, 4050);
                geoAdjust.setAdjust(3, 4050);
                // Java: geoAdjust.setAdjust(4, -1800);
                geoAdjust.setAdjust(4, -1800);
                // Java: geoAdjust.setAdjust(5, 4050);
                geoAdjust.setAdjust(5, 4050);
                // Java: break;
                return;
            }
            // Java: case 52:
            52 => {
                // Java: geoAdjust.setAdjust(0, 23400);
                geoAdjust.setAdjust(0, 23400);
                // Java: geoAdjust.setAdjust(1, 24400);
                geoAdjust.setAdjust(1, 24400);
                // Java: geoAdjust.setAdjust(2, 25200);
                geoAdjust.setAdjust(2, 25200);
                // Java: geoAdjust.setAdjust(3, 21600);
                geoAdjust.setAdjust(3, 21600);
                // Java: geoAdjust.setAdjust(4, 25200);
                geoAdjust.setAdjust(4, 25200);
                // Java: geoAdjust.setAdjust(5, 4050);
                geoAdjust.setAdjust(5, 4050);
                // Java: geoAdjust.setAdjust(6, 23400);
                geoAdjust.setAdjust(6, 23400);
                // Java: geoAdjust.setAdjust(7, 4050);
                geoAdjust.setAdjust(7, 4050);
                // Java: break;
                return;
            }
            // Java: case 53:
            53 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: geoAdjust.setAdjust(1, 2700);
                geoAdjust.setAdjust(1, 2700);
                // Java: break;
                return;
            }
            // Java: case 54:
            54 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: geoAdjust.setAdjust(1, 18900);
                geoAdjust.setAdjust(1, 18900);
                // Java: break;
                return;
            }
            // Java: case 57:
            57 => {
                // Java: geoAdjust.setAdjust(0, 2700);
                geoAdjust.setAdjust(0, 2700);
                // Java: break;
                return;
            }
            // Java: case 58:
            58 => {
                // Java: geoAdjust.setAdjust(0, 2538);
                geoAdjust.setAdjust(0, 2538);
                // Java: break;
                return;
            }
            // Java: case 59:
            59 => {
                // Java: geoAdjust.setAdjust(0, 2700);
                geoAdjust.setAdjust(0, 2700);
                // Java: break;
                return;
            }
            // Java: case 60:
            60 => {
                // Java: geoAdjust.setAdjust(0, 2700);
                geoAdjust.setAdjust(0, 2700);
                // Java: break;
                return;
            }
            // Java: case 62:
            62 => {
                // Java: geoAdjust.setAdjust(0, -43750);
                geoAdjust.setAdjust(0, -43750);
                // Java: geoAdjust.setAdjust(1, 70000);
                geoAdjust.setAdjust(1, 70000);
                // Java: geoAdjust.setAdjust(2, 16667);
                geoAdjust.setAdjust(2, 16667);
                // Java: break;
                return;
            }
            // Java: case 64:
            64 => {
                // Java: geoAdjust.setAdjust(0, 2809);
                geoAdjust.setAdjust(0, 2809);
                // Java: geoAdjust.setAdjust(1, 10800);
                geoAdjust.setAdjust(1, 10800);
                // Java: break;
                return;
            }
            // Java: case 65:
            65 => {
                // Java: geoAdjust.setAdjust(0, 18900);
                geoAdjust.setAdjust(0, 18900);
                // Java: break;
                return;
            }
            // Java: case 66:
            66 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: break;
                return;
            }
            // Java: case 67:
            67 => {
                // Java: geoAdjust.setAdjust(0, 16200);
                geoAdjust.setAdjust(0, 16200);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: break;
                return;
            }
            // Java: case 68:
            68 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: break;
                return;
            }
            // Java: case 69:
            69 => {
                // Java: geoAdjust.setAdjust(0, 4320);
                geoAdjust.setAdjust(0, 4320);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: break;
                return;
            }
            // Java: case 70:
            70 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: geoAdjust.setAdjust(1, 4320);
                geoAdjust.setAdjust(1, 4320);
                // Java: break;
                return;
            }
            // Java: case 76:
            76 => {
                // Java: geoAdjust.setAdjust(0, 6480);
                geoAdjust.setAdjust(0, 6480);
                // Java: geoAdjust.setAdjust(1, 8640);
                geoAdjust.setAdjust(1, 8640);
                // Java: geoAdjust.setAdjust(2, 4320);
                geoAdjust.setAdjust(2, 4320);
                // Java: break;
                return;
            }
            // Java: case 77:
            77 => {
                // Java: geoAdjust.setAdjust(0, 7200);
                geoAdjust.setAdjust(0, 7200);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: geoAdjust.setAdjust(2, 3600);
                geoAdjust.setAdjust(2, 3600);
                // Java: geoAdjust.setAdjust(3, 8100);
                geoAdjust.setAdjust(3, 8100);
                // Java: break;
                return;
            }
            // Java: case 78:
            78 => {
                // Java: geoAdjust.setAdjust(0, 14400);
                geoAdjust.setAdjust(0, 14400);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: geoAdjust.setAdjust(2, 18000);
                geoAdjust.setAdjust(2, 18000);
                // Java: geoAdjust.setAdjust(3, 8100);
                geoAdjust.setAdjust(3, 8100);
                // Java: break;
                return;
            }
            // Java: case 79:
            79 => {
                // Java: geoAdjust.setAdjust(0, 7200);
                geoAdjust.setAdjust(0, 7200);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: geoAdjust.setAdjust(2, 3600);
                geoAdjust.setAdjust(2, 3600);
                // Java: geoAdjust.setAdjust(3, 8100);
                geoAdjust.setAdjust(3, 8100);
                // Java: break;
                return;
            }
            // Java: case 80:
            80 => {
                // Java: geoAdjust.setAdjust(0, 14400);
                geoAdjust.setAdjust(0, 14400);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: geoAdjust.setAdjust(2, 18000);
                geoAdjust.setAdjust(2, 18000);
                // Java: geoAdjust.setAdjust(3, 8100);
                geoAdjust.setAdjust(3, 8100);
                // Java: break;
                return;
            }
            // Java: case 81:
            81 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: geoAdjust.setAdjust(2, 2700);
                geoAdjust.setAdjust(2, 2700);
                // Java: geoAdjust.setAdjust(3, 8100);
                geoAdjust.setAdjust(3, 8100);
                // Java: break;
                return;
            }
            // Java: case 82:
            82 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: geoAdjust.setAdjust(2, 2700);
                geoAdjust.setAdjust(2, 2700);
                // Java: geoAdjust.setAdjust(3, 8100);
                geoAdjust.setAdjust(3, 8100);
                // Java: break;
                return;
            }
            // Java: case 83:
            83 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: geoAdjust.setAdjust(1, 8100);
                geoAdjust.setAdjust(1, 8100);
                // Java: geoAdjust.setAdjust(2, 2700);
                geoAdjust.setAdjust(2, 2700);
                // Java: geoAdjust.setAdjust(3, 9450);
                geoAdjust.setAdjust(3, 9450);
                // Java: break;
                return;
            }
            // Java: case 84:
            84 => {
                // Java: geoAdjust.setAdjust(0, 2700);
                geoAdjust.setAdjust(0, 2700);
                // Java: break;
                return;
            }
            // Java: case 85:
            85 => {
                // Java: geoAdjust.setAdjust(0, 1800);
                geoAdjust.setAdjust(0, 1800);
                // Java: break;
                return;
            }
            // Java: case 86:
            86 => {
                // Java: geoAdjust.setAdjust(0, 1800);
                geoAdjust.setAdjust(0, 1800);
                // Java: break;
                return;
            }
            // Java: case 87:
            87 => {
                // Java: geoAdjust.setAdjust(0, 1800);
                geoAdjust.setAdjust(0, 1800);
                // Java: geoAdjust.setAdjust(1, 10800);
                geoAdjust.setAdjust(1, 10800);
                // Java: break;
                return;
            }
            // Java: case 88:
            88 => {
                // Java: geoAdjust.setAdjust(0, 1800);
                geoAdjust.setAdjust(0, 1800);
                // Java: geoAdjust.setAdjust(1, 10800);
                geoAdjust.setAdjust(1, 10800);
                // Java: break;
                return;
            }
            // Java: case 89:
            89 => {
                // Java: geoAdjust.setAdjust(0, 9257);
                geoAdjust.setAdjust(0, 9257);
                // Java: geoAdjust.setAdjust(1, 18514);
                geoAdjust.setAdjust(1, 18514);
                // Java: geoAdjust.setAdjust(2, 6171);
                geoAdjust.setAdjust(2, 6171);
                // Java: break;
                return;
            }
            // Java: case 90:
            90 => {
                // Java: geoAdjust.setAdjust(0, 9257);
                geoAdjust.setAdjust(0, 9257);
                // Java: geoAdjust.setAdjust(1, 18514);
                geoAdjust.setAdjust(1, 18514);
                // Java: geoAdjust.setAdjust(2, 7200);
                geoAdjust.setAdjust(2, 7200);
                // Java: break;
                return;
            }
            // Java: case 91:
            91 => {
                // Java: geoAdjust.setAdjust(0, 15126);
                geoAdjust.setAdjust(0, 15126);
                // Java: geoAdjust.setAdjust(1, 2912);
                geoAdjust.setAdjust(1, 2912);
                // Java: break;
                return;
            }
            // Java: case 92:
            92 => {
                // Java: geoAdjust.setAdjust(0, 2700);
                geoAdjust.setAdjust(0, 2700);
                // Java: break;
                return;
            }
            // Java: case 93:
            93 => {
                // Java: geoAdjust.setAdjust(0, 16200);
                geoAdjust.setAdjust(0, 16200);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: break;
                return;
            }
            // Java: case 94:
            94 => {
                // Java: geoAdjust.setAdjust(0, 16200);
                geoAdjust.setAdjust(0, 16200);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: break;
                return;
            }
            // Java: case 95:
            95 => {
                // Java: geoAdjust.setAdjust(0, 11796480);
                geoAdjust.setAdjust(0, 11796480);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: break;
                return;
            }
            // Java: case 96:
            96 => {
                // Java: geoAdjust.setAdjust(0, 17520);
                geoAdjust.setAdjust(0, 17520);
                // Java: break;
                return;
            }
            // Java: case 97:
            97 => {
                // Java: geoAdjust.setAdjust(0, 2700);
                geoAdjust.setAdjust(0, 2700);
                // Java: break;
                return;
            }
            // Java: case 98:
            98 => {
                // Java: geoAdjust.setAdjust(0, 2700);
                geoAdjust.setAdjust(0, 2700);
                // Java: break;
                return;
            }
            // Java: case 99:
            99 => {
                // Java: geoAdjust.setAdjust(0, -11796480);
                geoAdjust.setAdjust(0, -11796480);
                // Java: geoAdjust.setAdjust(2, 5400);
                geoAdjust.setAdjust(2, 5400);
                // Java: break;
                return;
            }
            // Java: case 102:
            102 => {
                // Java: geoAdjust.setAdjust(0, 12960);
                geoAdjust.setAdjust(0, 12960);
                // Java: geoAdjust.setAdjust(1, 19440);
                geoAdjust.setAdjust(1, 19440);
                // Java: geoAdjust.setAdjust(2, 14400);
                geoAdjust.setAdjust(2, 14400);
                // Java: break;
                return;
            }
            // Java: case 103:
            103 => {
                // Java: geoAdjust.setAdjust(0, 12960);
                geoAdjust.setAdjust(0, 12960);
                // Java: geoAdjust.setAdjust(1, 19440);
                geoAdjust.setAdjust(1, 19440);
                // Java: geoAdjust.setAdjust(2, 7200);
                geoAdjust.setAdjust(2, 7200);
                // Java: break;
                return;
            }
            // Java: case 104:
            104 => {
                // Java: geoAdjust.setAdjust(0, 12960);
                geoAdjust.setAdjust(0, 12960);
                // Java: geoAdjust.setAdjust(1, 19440);
                geoAdjust.setAdjust(1, 19440);
                // Java: geoAdjust.setAdjust(2, 7200);
                geoAdjust.setAdjust(2, 7200);
                // Java: break;
                return;
            }
            // Java: case 105:
            105 => {
                // Java: geoAdjust.setAdjust(0, 12960);
                geoAdjust.setAdjust(0, 12960);
                // Java: geoAdjust.setAdjust(1, 19440);
                geoAdjust.setAdjust(1, 19440);
                // Java: geoAdjust.setAdjust(2, 14400);
                geoAdjust.setAdjust(2, 14400);
                // Java: break;
                return;
            }
            // Java: case 106:
            106 => {
                // Java: geoAdjust.setAdjust(0, 1350);
                geoAdjust.setAdjust(0, 1350);
                // Java: geoAdjust.setAdjust(1, 25920);
                geoAdjust.setAdjust(1, 25920);
                // Java: break;
                return;
            }
            // Java: case 107:
            107 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: geoAdjust.setAdjust(2, 18900);
                geoAdjust.setAdjust(2, 18900);
                // Java: break;
                return;
            }
            // Java: case 108:
            108 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: geoAdjust.setAdjust(1, 16200);
                geoAdjust.setAdjust(1, 16200);
                // Java: geoAdjust.setAdjust(2, 2700);
                geoAdjust.setAdjust(2, 2700);
                // Java: break;
                return;
            }
            // Java: case 137:
            137 => {
                // Java: geoAdjust.setAdjust(0, 4800);
                geoAdjust.setAdjust(0, 4800);
                // Java: break;
                return;
            }
            // Java: case 138:
            138 => {
                // Java: geoAdjust.setAdjust(0, 10800);
                geoAdjust.setAdjust(0, 10800);
                // Java: break;
                return;
            }
            // Java: case 139:
            139 => {
                // Java: geoAdjust.setAdjust(0, 10800);
                geoAdjust.setAdjust(0, 10800);
                // Java: break;
                return;
            }
            // Java: case 140:
            140 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: break;
                return;
            }
            // Java: case 141:
            141 => {
                // Java: geoAdjust.setAdjust(0, 16200);
                geoAdjust.setAdjust(0, 16200);
                // Java: break;
                return;
            }
            // Java: case 142:
            142 => {
                // Java: geoAdjust.setAdjust(0, 13500);
                geoAdjust.setAdjust(0, 13500);
                // Java: break;
                return;
            }
            // Java: case 143:
            143 => {
                // Java: geoAdjust.setAdjust(0, 13500);
                geoAdjust.setAdjust(0, 13500);
                // Java: break;
                return;
            }
            // Java: case 144:
            144 => {
                // Java: geoAdjust.setAdjust(0, 11796480);
                geoAdjust.setAdjust(0, 11796480);
                // Java: break;
                return;
            }
            // Java: case 145:
            145 => {
                // Java: geoAdjust.setAdjust(0, 0);
                geoAdjust.setAdjust(0, 0);
                // Java: break;
                return;
            }
            // Java: case 146:
            146 => {
                // Java: geoAdjust.setAdjust(0, -11730944);
                geoAdjust.setAdjust(0, -11730944);
                // Java: break;
                return;
            }
            // Java: case 147:
            147 => {
                // Java: geoAdjust.setAdjust(0, 11796480);
                geoAdjust.setAdjust(0, 11796480);
                // Java: break;
                return;
            }
            // Java: case 148:
            148 => {
                // Java: geoAdjust.setAdjust(0, 11796480);
                geoAdjust.setAdjust(0, 11796480);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: break;
                return;
            }
            // Java: case 149:
            149 => {
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: break;
                return;
            }
            // Java: case 150:
            150 => {
                // Java: geoAdjust.setAdjust(0, -11730944);
                geoAdjust.setAdjust(0, -11730944);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: break;
                return;
            }
            // Java: case 151:
            151 => {
                // Java: geoAdjust.setAdjust(0, 11796480);
                geoAdjust.setAdjust(0, 11796480);
                // Java: geoAdjust.setAdjust(1, 5400);
                geoAdjust.setAdjust(1, 5400);
                // Java: break;
                return;
            }
            // Java: case 152:
            152 => {
                // Java: geoAdjust.setAdjust(0, 9931);
                geoAdjust.setAdjust(0, 9931);
                // Java: break;
                return;
            }
            // Java: case 153:
            153 => {
                // Java: geoAdjust.setAdjust(0, 9391);
                geoAdjust.setAdjust(0, 9391);
                // Java: break;
                return;
            }
            // Java: case 154:
            154 => {
                // Java: geoAdjust.setAdjust(0, 9600);
                geoAdjust.setAdjust(0, 9600);
                // Java: break;
                return;
            }
            // Java: case 155:
            155 => {
                // Java: geoAdjust.setAdjust(0, 9600);
                geoAdjust.setAdjust(0, 9600);
                // Java: break;
                return;
            }
            // Java: case 156:
            156 => {
                // Java: geoAdjust.setAdjust(0, 2809);
                geoAdjust.setAdjust(0, 2809);
                // Java: geoAdjust.setAdjust(1, 10800);
                geoAdjust.setAdjust(1, 10800);
                // Java: break;
                return;
            }
            // Java: case 157:
            157 => {
                // Java: geoAdjust.setAdjust(0, 2809);
                geoAdjust.setAdjust(0, 2809);
                // Java: geoAdjust.setAdjust(1, 10800);
                geoAdjust.setAdjust(1, 10800);
                // Java: break;
                return;
            }
            // Java: case 158:
            158 => {
                // Java: geoAdjust.setAdjust(0, 1404);
                geoAdjust.setAdjust(0, 1404);
                // Java: geoAdjust.setAdjust(1, 10800);
                geoAdjust.setAdjust(1, 10800);
                // Java: break;
                return;
            }
            // Java: case 159:
            159 => {
                // Java: geoAdjust.setAdjust(0, 1404);
                geoAdjust.setAdjust(0, 1404);
                // Java: geoAdjust.setAdjust(1, 10800);
                geoAdjust.setAdjust(1, 10800);
                // Java: break;
                return;
            }
            // Java: case 160:
            160 => {
                // Java: geoAdjust.setAdjust(0, 2945);
                geoAdjust.setAdjust(0, 2945);
                // Java: break;
                return;
            }
            // Java: case 161:
            161 => {
                // Java: geoAdjust.setAdjust(0, 4050);
                geoAdjust.setAdjust(0, 4050);
                // Java: break;
                return;
            }
            // Java: case 162:
            162 => {
                // Java: geoAdjust.setAdjust(0, 14706);
                geoAdjust.setAdjust(0, 14706);
                // Java: break;
                return;
            }
            // Java: case 163:
            163 => {
                // Java: geoAdjust.setAdjust(0, 11475);
                geoAdjust.setAdjust(0, 11475);
                // Java: break;
                return;
            }
            // Java: case 164:
            164 => {
                // Java: geoAdjust.setAdjust(0, 6894);
                geoAdjust.setAdjust(0, 6894);
                // Java: break;
                return;
            }
            // Java: case 165:
            165 => {
                // Java: geoAdjust.setAdjust(0, 10125);
                geoAdjust.setAdjust(0, 10125);
                // Java: break;
                return;
            }
            // Java: case 166:
            166 => {
                // Java: geoAdjust.setAdjust(0, 6054);
                geoAdjust.setAdjust(0, 6054);
                // Java: break;
                return;
            }
            // Java: case 167:
            167 => {
                // Java: geoAdjust.setAdjust(0, 6054);
                geoAdjust.setAdjust(0, 6054);
                // Java: break;
                return;
            }
            // Java: case 168:
            168 => {
                // Java: geoAdjust.setAdjust(0, 7200);
                geoAdjust.setAdjust(0, 7200);
                // Java: break;
                return;
            }
            // Java: case 169:
            169 => {
                // Java: geoAdjust.setAdjust(0, 7200);
                geoAdjust.setAdjust(0, 7200);
                // Java: break;
                return;
            }
            // Java: case 170:
            170 => {
                // Java: geoAdjust.setAdjust(0, 7200);
                geoAdjust.setAdjust(0, 7200);
                // Java: break;
                return;
            }
            // Java: case 171:
            171 => {
                // Java: geoAdjust.setAdjust(0, 7200);
                geoAdjust.setAdjust(0, 7200);
                // Java: break;
                return;
            }
            // Java: case 172:
            172 => {
                // Java: geoAdjust.setAdjust(0, 12000);
                geoAdjust.setAdjust(0, 12000);
                // Java: break;
                return;
            }
            // Java: case 173:
            173 => {
                // Java: geoAdjust.setAdjust(0, 9600);
                geoAdjust.setAdjust(0, 9600);
                // Java: break;
                return;
            }
            // Java: case 174:
            174 => {
                // Java: geoAdjust.setAdjust(0, 18514);
                geoAdjust.setAdjust(0, 18514);
                // Java: break;
                return;
            }
            // Java: case 175:
            175 => {
                // Java: geoAdjust.setAdjust(0, 3086);
                geoAdjust.setAdjust(0, 3086);
                // Java: break;
                return;
            }
            // Java: case 176:
            176 => {
                // Java: geoAdjust.setAdjust(0, 2700);
                geoAdjust.setAdjust(0, 2700);
                // Java: break;
                return;
            }
            // Java: case 178:
            178 => {
                // Java: geoAdjust.setAdjust(0, -1800);
                geoAdjust.setAdjust(0, -1800);
                // Java: geoAdjust.setAdjust(1, 24300);
                geoAdjust.setAdjust(1, 24300);
                // Java: geoAdjust.setAdjust(2, -1800);
                geoAdjust.setAdjust(2, -1800);
                // Java: geoAdjust.setAdjust(3, 4050);
                geoAdjust.setAdjust(3, 4050);
                // Java: break;
                return;
            }
            // Java: case 179:
            179 => {
                // Java: geoAdjust.setAdjust(0, -1800);
                geoAdjust.setAdjust(0, -1800);
                // Java: geoAdjust.setAdjust(1, 24300);
                geoAdjust.setAdjust(1, 24300);
                // Java: geoAdjust.setAdjust(2, -1800);
                geoAdjust.setAdjust(2, -1800);
                // Java: geoAdjust.setAdjust(3, 4050);
                geoAdjust.setAdjust(3, 4050);
                // Java: break;
                return;
            }
            // Java: case 180:
            180 => {
                // Java: geoAdjust.setAdjust(0, -1800);
                geoAdjust.setAdjust(0, -1800);
                // Java: geoAdjust.setAdjust(1, 24300);
                geoAdjust.setAdjust(1, 24300);
                // Java: geoAdjust.setAdjust(2, -1800);
                geoAdjust.setAdjust(2, -1800);
                // Java: geoAdjust.setAdjust(3, 4050);
                geoAdjust.setAdjust(3, 4050);
                // Java: break;
                return;
            }
            // Java: case 181:
            181 => {
                // Java: geoAdjust.setAdjust(0, -1800);
                geoAdjust.setAdjust(0, -1800);
                // Java: geoAdjust.setAdjust(1, 24300);
                geoAdjust.setAdjust(1, 24300);
                // Java: geoAdjust.setAdjust(2, -1800);
                geoAdjust.setAdjust(2, -1800);
                // Java: geoAdjust.setAdjust(3, 4050);
                geoAdjust.setAdjust(3, 4050);
                // Java: break;
                return;
            }
            // Java: case 182:
            182 => {
                // Java: geoAdjust.setAdjust(0, 6480);
                geoAdjust.setAdjust(0, 6480);
                // Java: geoAdjust.setAdjust(1, 8640);
                geoAdjust.setAdjust(1, 8640);
                // Java: geoAdjust.setAdjust(2, 6171);
                geoAdjust.setAdjust(2, 6171);
                // Java: break;
                return;
            }
            // Java: case 183:
            183 => {
                // Java: geoAdjust.setAdjust(0, 5400);
                geoAdjust.setAdjust(0, 5400);
                // Java: break;
                return;
            }
            // Java: case 184:
            184 => {
                // Java: geoAdjust.setAdjust(0, 10800);
                geoAdjust.setAdjust(0, 10800);
                // Java: break;
                return;
            }
            // Java: case 185:
            185 => {
                // Java: geoAdjust.setAdjust(0, 3600);
                geoAdjust.setAdjust(0, 3600);
                // Java: break;
                return;
            }
            // Java: case 186:
            186 => {
                // Java: geoAdjust.setAdjust(0, 1800);
                geoAdjust.setAdjust(0, 1800);
                // Java: break;
                return;
            }
            // Java: case 187:
            187 => {
                // Java: geoAdjust.setAdjust(0, 8100);
                geoAdjust.setAdjust(0, 8100);
                // Java: break;
                return;
            }
            // Java: case 188:
            188 => {
                // Java: geoAdjust.setAdjust(0, 1404);
                geoAdjust.setAdjust(0, 1404);
                // Java: geoAdjust.setAdjust(1, 10800);
                geoAdjust.setAdjust(1, 10800);
                // Java: break;
                return;
            }
            // Java: case 189:
            189 => {
                // Java: geoAdjust.setAdjust(0, 1350);
                geoAdjust.setAdjust(0, 1350);
                // Java: break;
                return;
            }
            // Java: case 190:
            190 => {
                // Java: geoAdjust.setAdjust(0, 1350);
                geoAdjust.setAdjust(0, 1350);
                // Java: break;
                return;
            }
            // Java: case 191:
            191 => {
                // Java: geoAdjust.setAdjust(0, 1350);
                geoAdjust.setAdjust(0, 1350);
                // Java: break;
                return;
            }
            // Java: case 192:
            192 => {
                // Java: geoAdjust.setAdjust(0, 1350);
                geoAdjust.setAdjust(0, 1350);
                // Java: break;
                return;
            }
            // Java: case 193:
            193 => {
                // Java: geoAdjust.setAdjust(0, 1350);
                geoAdjust.setAdjust(0, 1350);
                // Java: break;
                return;
            }
            // Java: case 194:
            194 => {
                // Java: geoAdjust.setAdjust(0, 1350);
                geoAdjust.setAdjust(0, 1350);
                // Java: break;
                return;
            }
            // Java: case 195:
            195 => {
                // Java: geoAdjust.setAdjust(0, 1350);
                geoAdjust.setAdjust(0, 1350);
                // Java: break;
                return;
            }
            // Java: case 196:
            196 => {
                // Java: geoAdjust.setAdjust(0, 1350);
                geoAdjust.setAdjust(0, 1350);
                // Java: break;
                return;
            }
            // Java: case 197:
            197 => {
                // Java: geoAdjust.setAdjust(0, 1350);
                geoAdjust.setAdjust(0, 1350);
                // Java: break;
                return;
            }
            // Java: case 198:
            198 => {
                // Java: geoAdjust.setAdjust(0, 1350);
                geoAdjust.setAdjust(0, 1350);
                // Java: break;
                return;
            }
            // Java: case 199:
            199 => {
                // Java: geoAdjust.setAdjust(0, 1350);
                geoAdjust.setAdjust(0, 1350);
                // Java: break;
                return;
            }
            // Java: case 200:
            200 => {
                // Java: geoAdjust.setAdjust(0, 1350);
                geoAdjust.setAdjust(0, 1350);
            }
            // Java: default:
            _ => {}
        }
    }

    // Java: static void addDefaultDmlGeoAdjust(GeoAdjust geoAdjust, int shapeType)
    pub fn addDefaultDmlGeoAdjust(geoAdjust: &mut GeoAdjust, shapeType: i32) {
        // Java: switch (shapeType)
        match shapeType {
            // Java: case 2:
            2 => {
                // Java: geoAdjust.setAdjust(0, 18000);
                geoAdjust.setAdjust(0, 18000);
                // Java: break;
                return;
            }
            // Java: case 12:
            12 => {
                // Java: geoAdjust.setAdjust(0, 19098);
                geoAdjust.setAdjust(0, 19098);
                // Java: break;
                return;
            }
            // Java: case 17:
            17 |
            // Java: case 62:
            62 => {
                // Java: geoAdjust.setAdjust(0, -20833);
                geoAdjust.setAdjust(0, -20833);
                // Java: geoAdjust.setAdjust(1, 62500);
                geoAdjust.setAdjust(1, 62500);
                // Java: geoAdjust.setAdjust(2, 16667);
                geoAdjust.setAdjust(2, 16667);
                // Java: break;
                return;
            }
            // Java: case 19:
            19 => {
                // Java: geoAdjust.setAdjust(0, 16200000);
                geoAdjust.setAdjust(0, 16200000);
                // Java: geoAdjust.setAdjust(1, 0);
                geoAdjust.setAdjust(1, 0);
                // Java: break;
                return;
            }
            // Java: case 35:
            35 => {
                // Java: geoAdjust.setAdjust(0, 50000);
                geoAdjust.setAdjust(0, 50000);
                // Java: geoAdjust.setAdjust(1, 50000);
                geoAdjust.setAdjust(1, 50000);
                // Java: break;
                return;
            }
            // Java: case 36:
            36 => {
                // Java: geoAdjust.setAdjust(0, 50000);
                geoAdjust.setAdjust(0, 50000);
                // Java: geoAdjust.setAdjust(1, 50000);
                geoAdjust.setAdjust(1, 50000);
                // Java: geoAdjust.setAdjust(2, 50000);
                geoAdjust.setAdjust(2, 50000);
                // Java: break;
                return;
            }
            // Java: case 39:
            39 => {
                // Java: geoAdjust.setAdjust(0, 50000);
                geoAdjust.setAdjust(0, 50000);
                // Java: geoAdjust.setAdjust(1, 50000);
                geoAdjust.setAdjust(1, 50000);
                // Java: break;
                return;
            }
            // Java: case 53:
            53 => {
                // Java: geoAdjust.setAdjust(0, 16667);
                geoAdjust.setAdjust(0, 16667);
                // Java: geoAdjust.setAdjust(1, 50000);
                geoAdjust.setAdjust(1, 50000);
                // Java: break;
                return;
            }
            // Java: case 54:
            54 => {
                // Java: geoAdjust.setAdjust(0, 16667);
                geoAdjust.setAdjust(0, 16667);
                // Java: geoAdjust.setAdjust(1, 50000);
                geoAdjust.setAdjust(1, 50000);
                // Java: break;
                return;
            }
            // Java: case 64:
            64 => {
                // Java: geoAdjust.setAdjust(0, 20000);
                geoAdjust.setAdjust(0, 20000);
                // Java: geoAdjust.setAdjust(1, 0);
                geoAdjust.setAdjust(1, 0);
                // Java: break;
                return;
            }
            // Java: case 87:
            87 => {
                // Java: geoAdjust.setAdjust(0, 8333);
                geoAdjust.setAdjust(0, 8333);
                // Java: geoAdjust.setAdjust(1, 50000);
                geoAdjust.setAdjust(1, 50000);
                // Java: break;
                return;
            }
            // Java: case 88:
            88 => {
                // Java: geoAdjust.setAdjust(0, 8333);
                geoAdjust.setAdjust(0, 8333);
                // Java: geoAdjust.setAdjust(1, 50000);
                geoAdjust.setAdjust(1, 50000);
                // Java: break;
                return;
            }
            // Java: case 91:
            91 => {
                // Java: geoAdjust.setAdjust(0, 25000);
                geoAdjust.setAdjust(0, 25000);
                // Java: geoAdjust.setAdjust(1, 25000);
                geoAdjust.setAdjust(1, 25000);
                // Java: geoAdjust.setAdjust(2, 25000);
                geoAdjust.setAdjust(2, 25000);
                // Java: geoAdjust.setAdjust(3, 43750);
                geoAdjust.setAdjust(3, 43750);
                // Java: break;
                return;
            }
            // Java: case 95:
            95 => {
                // Java: geoAdjust.setAdjust(0, 10800000);
                geoAdjust.setAdjust(0, 10800000);
                // Java: geoAdjust.setAdjust(1, 0);
                geoAdjust.setAdjust(1, 0);
                // Java: geoAdjust.setAdjust(2, 25000);
                geoAdjust.setAdjust(2, 25000);
                // Java: break;
                return;
            }
            // Java: case 99:
            99 => {
                // Java: geoAdjust.setAdjust(0, 12500);
                geoAdjust.setAdjust(0, 12500);
                // Java: geoAdjust.setAdjust(1, 1142319);
                geoAdjust.setAdjust(1, 1142319);
                // Java: geoAdjust.setAdjust(2, 20457681);
                geoAdjust.setAdjust(2, 20457681);
                // Java: geoAdjust.setAdjust(3, 10800000);
                geoAdjust.setAdjust(3, 10800000);
                // Java: geoAdjust.setAdjust(4, 12500);
                geoAdjust.setAdjust(4, 12500);
                // Java: break;
                return;
            }
            // Java: case 107:
            107 => {
                // Java: geoAdjust.setAdjust(0, 25000);
                geoAdjust.setAdjust(0, 25000);
                // Java: geoAdjust.setAdjust(1, 50000);
                geoAdjust.setAdjust(1, 50000);
                // Java: geoAdjust.setAdjust(2, 12500);
                geoAdjust.setAdjust(2, 12500);
                // Java: break;
                return;
            }
            // Java: case 108:
            108 => {
                // Java: geoAdjust.setAdjust(0, 25000);
                geoAdjust.setAdjust(0, 25000);
                // Java: geoAdjust.setAdjust(1, 50000);
                geoAdjust.setAdjust(1, 50000);
                // Java: geoAdjust.setAdjust(2, 12500);
                geoAdjust.setAdjust(2, 12500);
                // Java: break;
                return;
            }
            // Java: case 182:
            182 => {
                // Java: geoAdjust.setAdjust(0, 25000);
                geoAdjust.setAdjust(0, 25000);
                // Java: geoAdjust.setAdjust(1, 25000);
                geoAdjust.setAdjust(1, 25000);
                // Java: geoAdjust.setAdjust(2, 25000);
                geoAdjust.setAdjust(2, 25000);
                // Java: break;
                return;
            }
            // Java: case 188:
            188 => {
                // Java: geoAdjust.setAdjust(0, 12500);
                geoAdjust.setAdjust(0, 12500);
                // Java: geoAdjust.setAdjust(1, 0);
                geoAdjust.setAdjust(1, 0);
            }
            _ => {}
        }
    }
}
