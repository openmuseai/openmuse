#![allow(non_snake_case)]

use super::PreDefData::PreDefData;
use std::sync::{OnceLock, Mutex};

// Java: import java.util.ArrayList

// Java field: int _IS
pub(crate) const _IS: i32 = 48;
// Java field: int MAP_SIZE
pub(crate) const MAP_SIZE: i32 = 264;
// Java field: PreDefData[] map
static map: OnceLock<Mutex<Vec<Option<PreDefData>>>> = OnceLock::new();
// Java field: PreDefData rect
static rect: OnceLock<Mutex<PreDefData>> = OnceLock::new();

// Java: public class CDL
pub struct CDL {
}

// Java: public class CDL
impl CDL {
    // Java: CDL()
    pub fn new() -> Self {
        // Java: super();
        CDL {}
    }

    // Java: static void dispose()
    pub fn dispose() {
        // Java: if (map != null)
        if let Some(map_mutex) = map.get() {
            // Java: int size = map.length;
            let mut map_guard = map_mutex.lock().unwrap();
            // Java: for(int i = 0; i < size; ++i)
            // Java: map[i] = null;
            map_guard.clear();
        }
    }

    // Java: static PreDefData getDefData(int shapeType)
    pub fn getDefData(shapeType: i32) -> PreDefData {
        // Java: if (shapeType >= 0 && shapeType <= 264)
        if shapeType >= 0 && shapeType <= 264 {
            let map_mutex = map.get_or_init(|| {
                Mutex::new(vec![None; (MAP_SIZE + 1) as usize])
            });
            let mut map_guard = map_mutex.lock().unwrap();
            // Java: PreDefData defData = map[shapeType];
            let idx = shapeType as usize;
            // Java: if (defData == null)
            if map_guard[idx].is_none() {
                // Java: defData = initDefData(shapeType);
                let def_data = Self::initDefData(shapeType);
                // Java: map[shapeType] = defData;
                map_guard[idx] = Some(def_data);
            }
            // Java: return defData;
            map_guard[idx].as_ref().unwrap().clone()
        }
        // Java: else
        else {
            // Java: return rect;
            let r = rect.get_or_init(|| Mutex::new(PreDefData::new()));
            r.lock().unwrap().clone()
        }
    }

    // Java: static PreDefData initDefData(int shapeType)
    pub fn initDefData(shapeType: i32) -> PreDefData {
        // Java: switch (shapeType)
        match shapeType {
            // Java: case 1:
            // Java: return get_rect();
            1 => Self::get_rect(),
            // Java: case 2:
            // Java: return get_roundRect();
            2 => Self::get_roundRect(),
            // Java: case 3:
            // Java: return get_ellipse();
            3 => Self::get_ellipse(),
            // Java: case 4:
            // Java: return get_diamond();
            4 => Self::get_diamond(),
            // Java: case 5:
            // Java: return get_triangle();
            5 => Self::get_triangle(),
            // Java: case 6:
            // Java: return get_rtTriangle();
            6 => Self::get_rtTriangle(),
            // Java: case 7:
            // Java: return get_parallelogram();
            7 => Self::get_parallelogram(),
            // Java: case 8:
            // Java: return get_trapezoid();
            8 => Self::get_trapezoid(),
            // Java: case 9:
            // Java: return get_hexagon();
            9 => Self::get_hexagon(),
            // Java: case 10:
            // Java: return get_octagon();
            10 => Self::get_octagon(),
            // Java: case 11:
            // Java: return get_plus();
            11 => Self::get_plus(),
            // Java: case 12:
            // Java: return get_star5();
            12 => Self::get_star5(),
            // Java: case 13:
            // Java: return get_rightArrow();
            13 => Self::get_rightArrow(),
            // Java: case 14:
            // Java: case 24:
            // Java: case 25:
            // Java: case 26:
            // Java: case 27:
            // Java: case 28:
            // Java: case 29:
            // Java: case 30:
            // Java: case 31:
            // Java: case 100:
            // Java: case 136:
            // Java: case 137:
            // Java: case 138:
            // Java: case 139:
            // Java: case 140:
            // Java: case 141:
            // Java: case 142:
            // Java: case 143:
            // Java: case 144:
            // Java: case 145:
            // Java: case 146:
            // Java: case 147:
            // Java: case 148:
            // Java: case 149:
            // Java: case 150:
            // Java: case 151:
            // Java: case 152:
            // Java: case 153:
            // Java: case 154:
            // Java: case 155:
            // Java: case 156:
            // Java: case 157:
            // Java: case 158:
            // Java: case 159:
            // Java: case 160:
            // Java: case 161:
            // Java: case 162:
            // Java: case 163:
            // Java: case 164:
            // Java: case 165:
            // Java: case 166:
            // Java: case 167:
            // Java: case 168:
            // Java: case 169:
            // Java: case 170:
            // Java: case 171:
            // Java: case 172:
            // Java: case 173:
            // Java: case 174:
            // Java: case 175:
            // Java: case 201:
            // Java: case 202:
            // Java: case 203:
            // Java: case 204:
            // Java: case 205:
            // Java: case 206:
            // Java: case 207:
            // Java: case 208:
            // Java: case 209:
            // Java: case 210:
            // Java: case 211:
            // Java: case 212:
            // Java: case 213:
            // Java: case 214:
            // Java: case 215:
            // Java: case 216:
            // Java: case 217:
            // Java: case 218:
            // Java: case 219:
            // Java: case 220:
            // Java: default:
            // Java: return get_rect();
            14 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 100 | 136 | 137 | 138 | 139 | 140 | 141 | 142 | 143 | 144 | 145 | 146 | 147 | 148 | 149 | 150 | 151 | 152 | 153 | 154 | 155 | 156 | 157 | 158 | 159 | 160 | 161 | 162 | 163 | 164 | 165 | 166 | 167 | 168 | 169 | 170 | 171 | 172 | 173 | 174 | 175 | 201 | 202 | 203 | 204 | 205 | 206 | 207 | 208 | 209 | 210 | 211 | 212 | 213 | 214 | 215 | 216 | 217 | 218 | 219 | 220 => Self::get_rect(),
            // Java: case 15:
            // Java: return get_homePlate();
            15 => Self::get_homePlate(),
            // Java: case 16:
            // Java: return get_cube();
            16 => Self::get_cube(),
            // Java: case 17:
            // Java: case 62:
            // Java: return get_wedgeRoundRectCallout();
            17 | 62 => Self::get_wedgeRoundRectCallout(),
            // Java: case 18:
            // Java: case 59:
            // Java: return get_star16();
            18 | 59 => Self::get_star16(),
            // Java: case 19:
            // Java: return get_arc();
            19 => Self::get_arc(),
            // Java: case 20:
            // Java: return get_line();
            20 => Self::get_line(),
            // Java: case 21:
            // Java: return get_plaque();
            21 => Self::get_plaque(),
            // Java: case 22:
            // Java: return get_can();
            22 => Self::get_can(),
            // Java: case 23:
            // Java: return get_donut();
            23 => Self::get_donut(),
            // Java: case 32:
            // Java: return get_straightConnector1();
            32 => Self::get_straightConnector1(),
            // Java: case 33:
            // Java: return get_bentConnector2();
            33 => Self::get_bentConnector2(),
            // Java: case 34:
            // Java: return get_bentConnector3();
            34 => Self::get_bentConnector3(),
            // Java: case 35:
            // Java: return get_bentConnector4();
            35 => Self::get_bentConnector4(),
            // Java: case 36:
            // Java: return get_bentConnector5();
            36 => Self::get_bentConnector5(),
            // Java: case 37:
            // Java: return get_curvedConnector2();
            37 => Self::get_curvedConnector2(),
            // Java: case 38:
            // Java: return get_curvedConnector3();
            38 => Self::get_curvedConnector3(),
            // Java: case 39:
            // Java: return get_curvedConnector4();
            39 => Self::get_curvedConnector4(),
            // Java: case 40:
            // Java: return get_curvedConnector5();
            40 => Self::get_curvedConnector5(),
            // Java: case 41:
            // Java: case 178:
            // Java: return get_callout1();
            41 | 178 => Self::get_callout1(),
            // Java: case 42:
            // Java: return get_callout2();
            42 => Self::get_callout2(),
            // Java: case 43:
            // Java: return get_callout3();
            43 => Self::get_callout3(),
            // Java: case 44:
            // Java: return get_accentCallout1();
            44 => Self::get_accentCallout1(),
            // Java: case 45:
            // Java: return get_accentCallout2();
            45 => Self::get_accentCallout2(),
            // Java: case 46:
            // Java: return get_accentCallout3();
            46 => Self::get_accentCallout3(),
            // Java: case 47:
            // Java: case 180:
            // Java: return get_borderCallout1();
            47 | 180 => Self::get_borderCallout1(),
            // Java: case 48:
            // Java: return get_borderCallout2();
            48 => Self::get_borderCallout2(),
            // Java: case 49:
            // Java: return get_borderCallout3();
            49 => Self::get_borderCallout3(),
            // Java: case 50:
            // Java: return get_accentBorderCallout1();
            50 => Self::get_accentBorderCallout1(),
            // Java: case 51:
            // Java: return get_accentBorderCallout2();
            51 => Self::get_accentBorderCallout2(),
            // Java: case 52:
            // Java: return get_accentBorderCallout3();
            52 => Self::get_accentBorderCallout3(),
            // Java: case 53:
            // Java: return get_ribbon();
            53 => Self::get_ribbon(),
            // Java: case 54:
            // Java: return get_ribbon2();
            54 => Self::get_ribbon2(),
            // Java: case 55:
            // Java: return get_chevron();
            55 => Self::get_chevron(),
            // Java: case 56:
            // Java: return get_pentagon();
            56 => Self::get_pentagon(),
            // Java: case 57:
            // Java: return get_noSmoking();
            57 => Self::get_noSmoking(),
            // Java: case 58:
            // Java: return get_star8();
            58 => Self::get_star8(),
            // Java: case 60:
            // Java: return get_star32();
            60 => Self::get_star32(),
            // Java: case 61:
            // Java: return get_wedgeRectCallout();
            61 => Self::get_wedgeRectCallout(),
            // Java: case 63:
            // Java: return get_wedgeEllipseCallout();
            63 => Self::get_wedgeEllipseCallout(),
            // Java: case 64:
            // Java: return get_wave();
            64 => Self::get_wave(),
            // Java: case 65:
            // Java: return get_foldedCorner();
            65 => Self::get_foldedCorner(),
            // Java: case 66:
            // Java: return get_leftArrow();
            66 => Self::get_leftArrow(),
            // Java: case 67:
            // Java: return get_downArrow();
            67 => Self::get_downArrow(),
            // Java: case 68:
            // Java: return get_upArrow();
            68 => Self::get_upArrow(),
            // Java: case 69:
            // Java: return get_leftRightArrow();
            69 => Self::get_leftRightArrow(),
            // Java: case 70:
            // Java: return get_upDownArrow();
            70 => Self::get_upDownArrow(),
            // Java: case 71:
            // Java: return get_irregularSeal1();
            71 => Self::get_irregularSeal1(),
            // Java: case 72:
            // Java: return get_irregularSeal2();
            72 => Self::get_irregularSeal2(),
            // Java: case 73:
            // Java: return get_lightningBolt();
            73 => Self::get_lightningBolt(),
            // Java: case 74:
            // Java: return get_heart();
            74 => Self::get_heart(),
            // Java: case 75:
            // Java: return get_frame();
            75 => Self::get_frame(),
            // Java: case 76:
            // Java: return get_quadArrow();
            76 => Self::get_quadArrow(),
            // Java: case 77:
            // Java: return get_leftArrowCallout();
            77 => Self::get_leftArrowCallout(),
            // Java: case 78:
            // Java: return get_rightArrowCallout();
            78 => Self::get_rightArrowCallout(),
            // Java: case 79:
            // Java: return get_upArrowCallout();
            79 => Self::get_upArrowCallout(),
            // Java: case 80:
            // Java: return get_downArrowCallout();
            80 => Self::get_downArrowCallout(),
            // Java: case 81:
            // Java: return get_leftRightArrowCallout();
            81 => Self::get_leftRightArrowCallout(),
            // Java: case 82:
            // Java: return get_upDownArrowCallout();
            82 => Self::get_upDownArrowCallout(),
            // Java: case 83:
            // Java: return get_quadArrowCallout();
            83 => Self::get_quadArrowCallout(),
            // Java: case 84:
            // Java: return get_bevel();
            84 => Self::get_bevel(),
            // Java: case 85:
            // Java: return get_leftBracket();
            85 => Self::get_leftBracket(),
            // Java: case 86:
            // Java: return get_rightBracket();
            86 => Self::get_rightBracket(),
            // Java: case 87:
            // Java: return get_leftBrace();
            87 => Self::get_leftBrace(),
            // Java: case 88:
            // Java: return get_rightBrace();
            88 => Self::get_rightBrace(),
            // Java: case 89:
            // Java: return get_leftUpArrow();
            89 => Self::get_leftUpArrow(),
            // Java: case 90:
            // Java: return get_bentUpArrow();
            90 => Self::get_bentUpArrow(),
            // Java: case 91:
            // Java: return get_bentArrow();
            91 => Self::get_bentArrow(),
            // Java: case 92:
            // Java: return get_star24();
            92 => Self::get_star24(),
            // Java: case 93:
            // Java: return get_stripedRightArrow();
            93 => Self::get_stripedRightArrow(),
            // Java: case 94:
            // Java: return get_notchedRightArrow();
            94 => Self::get_notchedRightArrow(),
            // Java: case 95:
            // Java: return get_blockArc();
            95 => Self::get_blockArc(),
            // Java: case 96:
            // Java: return get_smileyFace();
            96 => Self::get_smileyFace(),
            // Java: case 97:
            // Java: return get_verticalScroll();
            97 => Self::get_verticalScroll(),
            // Java: case 98:
            // Java: return get_horizontalScroll();
            98 => Self::get_horizontalScroll(),
            // Java: case 99:
            // Java: return get_circularArrow();
            99 => Self::get_circularArrow(),
            // Java: case 101:
            // Java: return get_uturnArrow();
            101 => Self::get_uturnArrow(),
            // Java: case 102:
            // Java: return get_curvedRightArrow();
            102 => Self::get_curvedRightArrow(),
            // Java: case 103:
            // Java: return get_curvedLeftArrow();
            103 => Self::get_curvedLeftArrow(),
            // Java: case 104:
            // Java: return get_curvedUpArrow();
            104 => Self::get_curvedUpArrow(),
            // Java: case 105:
            // Java: return get_curvedDownArrow();
            105 => Self::get_curvedDownArrow(),
            // Java: case 106:
            // Java: return get_cloudCallout();
            106 => Self::get_cloudCallout(),
            // Java: case 107:
            // Java: return get_ellipseRibbon();
            107 => Self::get_ellipseRibbon(),
            // Java: case 108:
            // Java: return get_ellipseRibbon2();
            108 => Self::get_ellipseRibbon2(),
            // Java: case 109:
            // Java: return get_flowChartProcess();
            109 => Self::get_flowChartProcess(),
            // Java: case 110:
            // Java: return get_flowChartDecision();
            110 => Self::get_flowChartDecision(),
            // Java: case 111:
            // Java: return get_flowChartInputOutput();
            111 => Self::get_flowChartInputOutput(),
            // Java: case 112:
            // Java: return get_flowChartPredefinedProcess();
            112 => Self::get_flowChartPredefinedProcess(),
            // Java: case 113:
            // Java: return get_flowChartInternalStorage();
            113 => Self::get_flowChartInternalStorage(),
            // Java: case 114:
            // Java: return get_flowChartDocument();
            114 => Self::get_flowChartDocument(),
            // Java: case 115:
            // Java: return get_flowChartMultidocument();
            115 => Self::get_flowChartMultidocument(),
            // Java: case 116:
            // Java: return get_flowChartTerminator();
            116 => Self::get_flowChartTerminator(),
            // Java: case 117:
            // Java: return get_flowChartPreparation();
            117 => Self::get_flowChartPreparation(),
            // Java: case 118:
            // Java: return get_flowChartManualInput();
            118 => Self::get_flowChartManualInput(),
            // Java: case 119:
            // Java: return get_flowChartManualOperation();
            119 => Self::get_flowChartManualOperation(),
            // Java: case 120:
            // Java: return get_flowChartConnector();
            120 => Self::get_flowChartConnector(),
            // Java: case 121:
            // Java: return get_flowChartPunchedCard();
            121 => Self::get_flowChartPunchedCard(),
            // Java: case 122:
            // Java: return get_flowChartPunchedTape();
            122 => Self::get_flowChartPunchedTape(),
            // Java: case 123:
            // Java: return get_flowChartSummingJunction();
            123 => Self::get_flowChartSummingJunction(),
            // Java: case 124:
            // Java: return get_flowChartOr();
            124 => Self::get_flowChartOr(),
            // Java: case 125:
            // Java: return get_flowChartCollate();
            125 => Self::get_flowChartCollate(),
            // Java: case 126:
            // Java: return get_flowChartSort();
            126 => Self::get_flowChartSort(),
            // Java: case 127:
            // Java: return get_flowChartExtract();
            127 => Self::get_flowChartExtract(),
            // Java: case 128:
            // Java: return get_flowChartMerge();
            128 => Self::get_flowChartMerge(),
            // Java: case 129:
            // Java: return get_flowChartOfflineStorage();
            129 => Self::get_flowChartOfflineStorage(),
            // Java: case 130:
            // Java: return get_flowChartOnlineStorage();
            130 => Self::get_flowChartOnlineStorage(),
            // Java: case 131:
            // Java: return get_flowChartMagneticTape();
            131 => Self::get_flowChartMagneticTape(),
            // Java: case 132:
            // Java: return get_flowChartMagneticDisk();
            132 => Self::get_flowChartMagneticDisk(),
            // Java: case 133:
            // Java: return get_flowChartMagneticDrum();
            133 => Self::get_flowChartMagneticDrum(),
            // Java: case 134:
            // Java: return get_flowChartDisplay();
            134 => Self::get_flowChartDisplay(),
            // Java: case 135:
            // Java: return get_flowChartDelay();
            135 => Self::get_flowChartDelay(),
            // Java: case 176:
            // Java: return get_flowChartAlternateProcess();
            176 => Self::get_flowChartAlternateProcess(),
            // Java: case 177:
            // Java: return get_flowChartOffpageConnector();
            177 => Self::get_flowChartOffpageConnector(),
            // Java: case 179:
            // Java: return get_accentCallout90();
            179 => Self::get_accentCallout90(),
            // Java: case 181:
            // Java: return get_accentBorderCallout90();
            181 => Self::get_accentBorderCallout90(),
            // Java: case 182:
            // Java: return get_leftRightUpArrow();
            182 => Self::get_leftRightUpArrow(),
            // Java: case 183:
            // Java: return get_sun();
            183 => Self::get_sun(),
            // Java: case 184:
            // Java: return get_moon();
            184 => Self::get_moon(),
            // Java: case 185:
            // Java: return get_bracketPair();
            185 => Self::get_bracketPair(),
            // Java: case 186:
            // Java: return get_bracePair();
            186 => Self::get_bracePair(),
            // Java: case 187:
            // Java: return get_star4();
            187 => Self::get_star4(),
            // Java: case 188:
            // Java: return get_doubleWave();
            188 => Self::get_doubleWave(),
            // Java: case 189:
            // Java: return get_actionButtonBlank();
            189 => Self::get_actionButtonBlank(),
            // Java: case 190:
            // Java: return get_actionButtonHome();
            190 => Self::get_actionButtonHome(),
            // Java: case 191:
            // Java: return get_actionButtonHelp();
            191 => Self::get_actionButtonHelp(),
            // Java: case 192:
            // Java: return get_actionButtonInformation();
            192 => Self::get_actionButtonInformation(),
            // Java: case 193:
            // Java: return get_actionButtonForwardNext();
            193 => Self::get_actionButtonForwardNext(),
            // Java: case 194:
            // Java: return get_actionButtonBackPrevious();
            194 => Self::get_actionButtonBackPrevious(),
            // Java: case 195:
            // Java: return get_actionButtonEnd();
            195 => Self::get_actionButtonEnd(),
            // Java: case 196:
            // Java: return get_actionButtonBeginning();
            196 => Self::get_actionButtonBeginning(),
            // Java: case 197:
            // Java: return get_actionButtonReturn();
            197 => Self::get_actionButtonReturn(),
            // Java: case 198:
            // Java: return get_actionButtonDocument();
            198 => Self::get_actionButtonDocument(),
            // Java: case 199:
            // Java: return get_actionButtonSound();
            199 => Self::get_actionButtonSound(),
            // Java: case 200:
            // Java: return get_actionButtonMovie();
            200 => Self::get_actionButtonMovie(),
            // Java: case 221:
            // Java: return get_chartPlus();
            221 => Self::get_chartPlus(),
            // Java: case 222:
            // Java: return get_chartStar();
            222 => Self::get_chartStar(),
            // Java: case 223:
            // Java: return get_chartX();
            223 => Self::get_chartX(),
            // Java: case 224:
            // Java: return get_chord();
            224 => Self::get_chord(),
            // Java: case 225:
            // Java: return get_cloud();
            225 => Self::get_cloud(),
            // Java: case 226:
            // Java: return get_corner();
            226 => Self::get_corner(),
            // Java: case 227:
            // Java: return get_cornerTabs();
            227 => Self::get_cornerTabs(),
            // Java: case 228:
            // Java: return get_decagon();
            228 => Self::get_decagon(),
            // Java: case 229:
            // Java: return get_diagStripe();
            229 => Self::get_diagStripe(),
            // Java: case 230:
            // Java: return get_dodecagon();
            230 => Self::get_dodecagon(),
            // Java: case 231:
            // Java: return get_funnel();
            231 => Self::get_funnel(),
            // Java: case 232:
            // Java: return get_gear6();
            232 => Self::get_gear6(),
            // Java: case 233:
            // Java: return get_gear9();
            233 => Self::get_gear9(),
            // Java: case 234:
            // Java: return get_halfFrame();
            234 => Self::get_halfFrame(),
            // Java: case 235:
            // Java: return get_heptagon();
            235 => Self::get_heptagon(),
            // Java: case 236:
            // Java: return get_leftCircularArrow();
            236 => Self::get_leftCircularArrow(),
            // Java: case 237:
            // Java: return get_leftRightCircularArrow();
            237 => Self::get_leftRightCircularArrow(),
            // Java: case 238:
            // Java: return get_leftRightRibbon();
            238 => Self::get_leftRightRibbon(),
            // Java: case 239:
            // Java: return get_lineInv();
            239 => Self::get_lineInv(),
            // Java: case 240:
            // Java: return get_mathDivide();
            240 => Self::get_mathDivide(),
            // Java: case 241:
            // Java: return get_mathEqual();
            241 => Self::get_mathEqual(),
            // Java: case 242:
            // Java: return get_mathMinus();
            242 => Self::get_mathMinus(),
            // Java: case 243:
            // Java: return get_mathMultiply();
            243 => Self::get_mathMultiply(),
            // Java: case 244:
            // Java: return get_mathNotEqual();
            244 => Self::get_mathNotEqual(),
            // Java: case 245:
            // Java: return get_mathPlus();
            245 => Self::get_mathPlus(),
            // Java: case 246:
            // Java: return get_nonIsoscelesTrapezoid();
            246 => Self::get_nonIsoscelesTrapezoid(),
            // Java: case 247:
            // Java: return get_pie();
            247 => Self::get_pie(),
            // Java: case 248:
            // Java: return get_pieWedge();
            248 => Self::get_pieWedge(),
            // Java: case 249:
            // Java: return get_plaqueTabs();
            249 => Self::get_plaqueTabs(),
            // Java: case 250:
            // Java: return get_round1Rect();
            250 => Self::get_round1Rect(),
            // Java: case 251:
            // Java: return get_round2DiagRect();
            251 => Self::get_round2DiagRect(),
            // Java: case 252:
            // Java: return get_round2SameRect();
            252 => Self::get_round2SameRect(),
            // Java: case 253:
            // Java: return get_snip1Rect();
            253 => Self::get_snip1Rect(),
            // Java: case 254:
            // Java: return get_snip2DiagRect();
            254 => Self::get_snip2DiagRect(),
            // Java: case 255:
            // Java: return get_snip2SameRect();
            255 => Self::get_snip2SameRect(),
            // Java: case 256:
            // Java: return get_snipRoundRect();
            256 => Self::get_snipRoundRect(),
            // Java: case 257:
            // Java: return get_squareTabs();
            257 => Self::get_squareTabs(),
            // Java: case 258:
            // Java: return get_star10();
            258 => Self::get_star10(),
            // Java: case 259:
            // Java: return get_star12();
            259 => Self::get_star12(),
            // Java: case 260:
            // Java: return get_star6();
            260 => Self::get_star6(),
            // Java: case 261:
            // Java: return get_star7();
            261 => Self::get_star7(),
            // Java: case 262:
            // Java: return get_swooshArrow();
            262 => Self::get_swooshArrow(),
            // Java: case 263:
            // Java: return get_teardrop();
            263 => Self::get_teardrop(),
            _ => Self::get_rect(),
        }
    }

    // Java: static ArrayList<PreDefData> initTestArray()
    pub fn initTestArray() -> Vec<PreDefData> {
        // Java: ArrayList<PreDefData> arr = new ArrayList();
        let mut arr = Vec::new();
        // Java: arr.add(get_chartPlus());
        arr.push(Self::get_chartPlus());
        // Java: arr.add(get_chartStar());
        arr.push(Self::get_chartStar());
        // Java: arr.add(get_chartX());
        arr.push(Self::get_chartX());
        // Java: arr.add(get_chord());
        arr.push(Self::get_chord());
        // Java: arr.add(get_cloud());
        arr.push(Self::get_cloud());
        // Java: arr.add(get_corner());
        arr.push(Self::get_corner());
        // Java: arr.add(get_cornerTabs());
        arr.push(Self::get_cornerTabs());
        // Java: arr.add(get_decagon());
        arr.push(Self::get_decagon());
        // Java: arr.add(get_diagStripe());
        arr.push(Self::get_diagStripe());
        // Java: arr.add(get_dodecagon());
        arr.push(Self::get_dodecagon());
        // Java: arr.add(get_funnel());
        arr.push(Self::get_funnel());
        // Java: arr.add(get_gear6());
        arr.push(Self::get_gear6());
        // Java: arr.add(get_gear9());
        arr.push(Self::get_gear9());
        // Java: arr.add(get_halfFrame());
        arr.push(Self::get_halfFrame());
        // Java: arr.add(get_heptagon());
        arr.push(Self::get_heptagon());
        // Java: arr.add(get_leftCircularArrow());
        arr.push(Self::get_leftCircularArrow());
        // Java: arr.add(get_leftRightCircularArrow());
        arr.push(Self::get_leftRightCircularArrow());
        // Java: arr.add(get_leftRightRibbon());
        arr.push(Self::get_leftRightRibbon());
        // Java: arr.add(get_lineInv());
        arr.push(Self::get_lineInv());
        // Java: arr.add(get_mathDivide());
        arr.push(Self::get_mathDivide());
        // Java: arr.add(get_mathEqual());
        arr.push(Self::get_mathEqual());
        // Java: arr.add(get_mathMinus());
        arr.push(Self::get_mathMinus());
        // Java: arr.add(get_mathMultiply());
        arr.push(Self::get_mathMultiply());
        // Java: arr.add(get_mathNotEqual());
        arr.push(Self::get_mathNotEqual());
        // Java: arr.add(get_mathPlus());
        arr.push(Self::get_mathPlus());
        // Java: arr.add(get_nonIsoscelesTrapezoid());
        arr.push(Self::get_nonIsoscelesTrapezoid());
        // Java: arr.add(get_pie());
        arr.push(Self::get_pie());
        // Java: arr.add(get_pieWedge());
        arr.push(Self::get_pieWedge());
        // Java: arr.add(get_plaqueTabs());
        arr.push(Self::get_plaqueTabs());
        // Java: arr.add(get_round1Rect());
        arr.push(Self::get_round1Rect());
        // Java: arr.add(get_round2DiagRect());
        arr.push(Self::get_round2DiagRect());
        // Java: arr.add(get_round2SameRect());
        arr.push(Self::get_round2SameRect());
        // Java: arr.add(get_snip1Rect());
        arr.push(Self::get_snip1Rect());
        // Java: arr.add(get_snip2DiagRect());
        arr.push(Self::get_snip2DiagRect());
        // Java: arr.add(get_snip2SameRect());
        arr.push(Self::get_snip2SameRect());
        // Java: arr.add(get_snipRoundRect());
        arr.push(Self::get_snipRoundRect());
        // Java: arr.add(get_squareTabs());
        arr.push(Self::get_squareTabs());
        // Java: arr.add(get_star10());
        arr.push(Self::get_star10());
        // Java: arr.add(get_star12());
        arr.push(Self::get_star12());
        // Java: arr.add(get_star6());
        arr.push(Self::get_star6());
        // Java: arr.add(get_star7());
        arr.push(Self::get_star7());
        // Java: arr.add(get_swooshArrow());
        arr.push(Self::get_swooshArrow());
        // Java: arr.add(get_teardrop());
        arr.push(Self::get_teardrop());
        // Java: return arr;
        arr
    }

    // Java: static PreDefData get_accentBorderCallout90()
    fn get_accentBorderCallout90() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{18750, -8333, 112500, -38333, 100000, 0, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![18750, -8333, 112500, -38333, 100000, 0, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 1, 48, 52, 196609, 0, 49, 52, 196609, 1, 50, 52, 196609, 0, 51, 52};
        data.gdLst = vec![196609, 1, 48, 52, 196609, 0, 49, 52, 196609, 1, 50, 52, 196609, 0, 51, 52];
        // Java: data.ahLst = new int[]{3, 49, 54, 55, 48, 54, 55, 60, 56, 3, 51, 54, 55, 50, 54, 55, 68, 64};
        data.ahLst = vec![3, 49, 54, 55, 48, 54, 55, 60, 56, 3, 51, 54, 55, 50, 54, 55, 68, 64];
        // Java: data.cxnLst = new int[]{53, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![53, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}, {10, 13, 0, 68, 64, 1, 60, 56}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2], vec![10, 13, 0, 68, 64, 1, 60, 56]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_accentBorderCallout1()
    fn get_accentBorderCallout1() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{18750, -8333, 112500, -38333, 100000, 0, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![18750, -8333, 112500, -38333, 100000, 0, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 1, 48, 52, 196609, 0, 49, 52, 196609, 1, 50, 52, 196609, 0, 51, 52};
        data.gdLst = vec![196609, 1, 48, 52, 196609, 0, 49, 52, 196609, 1, 50, 52, 196609, 0, 51, 52];
        // Java: data.ahLst = new int[]{3, 49, 54, 55, 48, 54, 55, 60, 56, 3, 51, 54, 55, 50, 54, 55, 68, 64};
        data.ahLst = vec![3, 49, 54, 55, 48, 54, 55, 60, 56, 3, 51, 54, 55, 50, 54, 55, 68, 64];
        // Java: data.cxnLst = new int[]{53, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![53, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}, {10, 13, 2, 0, 60, 26, 1, 60, 1}, {10, 13, 0, 68, 64, 1, 60, 56}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2], vec![10, 13, 2, 0, 60, 26, 1, 60, 1], vec![10, 13, 0, 68, 64, 1, 60, 56]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_accentBorderCallout2()
    fn get_accentBorderCallout2() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{18750, -8333, 18750, -16667, 112500, -46667, 100000, 0, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![18750, -8333, 18750, -16667, 112500, -46667, 100000, 0, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 1, 48, 54, 196609, 0, 49, 54, 196609, 1, 50, 54, 196609, 0, 51, 54, 196609, 1, 52, 54, 196609, 0, 53, 54};
        data.gdLst = vec![196609, 1, 48, 54, 196609, 0, 49, 54, 196609, 1, 50, 54, 196609, 0, 51, 54, 196609, 1, 52, 54, 196609, 0, 53, 54];
        // Java: data.ahLst = new int[]{3, 49, 56, 57, 48, 56, 57, 62, 58, 3, 51, 56, 57, 50, 56, 57, 70, 66, 3, 53, 56, 57, 52, 56, 57, 78, 74};
        data.ahLst = vec![3, 49, 56, 57, 48, 56, 57, 62, 58, 3, 51, 56, 57, 50, 56, 57, 70, 66, 3, 53, 56, 57, 52, 56, 57, 78, 74];
        // Java: data.cxnLst = new int[]{55, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![55, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}, {10, 13, 2, 0, 62, 26, 1, 62, 1}, {10, 13, 0, 78, 74, 1, 70, 66, 1, 62, 58}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2], vec![10, 13, 2, 0, 62, 26, 1, 62, 1], vec![10, 13, 0, 78, 74, 1, 70, 66, 1, 62, 58]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_accentBorderCallout3()
    fn get_accentBorderCallout3() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{18750, -8333, 18750, -16667, 100000, -16667, 112963, -8333, 100000, 0, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![18750, -8333, 18750, -16667, 100000, -16667, 112963, -8333, 100000, 0, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 1, 48, 56, 196609, 0, 49, 56, 196609, 1, 50, 56, 196609, 0, 51, 56, 196609, 1, 52, 56, 196609, 0, 53, 56, 196609, 1, 54, 56, 196609, 0, 55, 56};
        data.gdLst = vec![196609, 1, 48, 56, 196609, 0, 49, 56, 196609, 1, 50, 56, 196609, 0, 51, 56, 196609, 1, 52, 56, 196609, 0, 53, 56, 196609, 1, 54, 56, 196609, 0, 55, 56];
        // Java: data.ahLst = new int[]{3, 49, 58, 59, 48, 58, 59, 64, 60, 3, 51, 58, 59, 50, 58, 59, 72, 68, 3, 53, 58, 59, 52, 58, 59, 80, 76, 3, 55, 58, 59, 54, 58, 59, 88, 84};
        data.ahLst = vec![3, 49, 58, 59, 48, 58, 59, 64, 60, 3, 51, 58, 59, 50, 58, 59, 72, 68, 3, 53, 58, 59, 52, 58, 59, 80, 76, 3, 55, 58, 59, 54, 58, 59, 88, 84];
        // Java: data.cxnLst = new int[]{57, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![57, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}, {10, 13, 2, 0, 64, 26, 1, 64, 1}, {10, 13, 0, 88, 84, 1, 80, 76, 1, 72, 68, 1, 64, 60}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2], vec![10, 13, 2, 0, 64, 26, 1, 64, 1], vec![10, 13, 0, 88, 84, 1, 80, 76, 1, 72, 68, 1, 64, 60]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_accentCallout90()
    fn get_accentCallout90() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{18750, -8333, 112500, -38333, 100000, 0, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![18750, -8333, 112500, -38333, 100000, 0, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 1, 48, 52, 196609, 0, 49, 52, 196609, 1, 50, 52, 196609, 0, 51, 52};
        data.gdLst = vec![196609, 1, 48, 52, 196609, 0, 49, 52, 196609, 1, 50, 52, 196609, 0, 51, 52];
        // Java: data.ahLst = new int[]{3, 49, 54, 55, 48, 54, 55, 60, 56, 3, 51, 54, 55, 50, 54, 55, 68, 64};
        data.ahLst = vec![3, 49, 54, 55, 48, 54, 55, 60, 56, 3, 51, 54, 55, 50, 54, 55, 68, 64];
        // Java: data.cxnLst = new int[]{53, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![53, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}, {10, 13, 0, 68, 64, 1, 60, 56}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2], vec![10, 13, 0, 68, 64, 1, 60, 56]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_accentCallout1()
    fn get_accentCallout1() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{18750, -8333, 112500, -38333, 100000, 0, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![18750, -8333, 112500, -38333, 100000, 0, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 1, 48, 52, 196609, 0, 49, 52, 196609, 1, 50, 52, 196609, 0, 51, 52};
        data.gdLst = vec![196609, 1, 48, 52, 196609, 0, 49, 52, 196609, 1, 50, 52, 196609, 0, 51, 52];
        // Java: data.ahLst = new int[]{3, 49, 54, 55, 48, 54, 55, 60, 56, 3, 51, 54, 55, 50, 54, 55, 68, 64};
        data.ahLst = vec![3, 49, 54, 55, 48, 54, 55, 60, 56, 3, 51, 54, 55, 50, 54, 55, 68, 64];
        // Java: data.cxnLst = new int[]{53, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![53, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}, {10, 13, 2, 0, 60, 26, 1, 60, 1}, {10, 13, 0, 68, 64, 1, 60, 56}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2], vec![10, 13, 2, 0, 60, 26, 1, 60, 1], vec![10, 13, 0, 68, 64, 1, 60, 56]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_accentCallout2()
    fn get_accentCallout2() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{18750, -8333, 18750, -16667, 112500, -46667, 100000, 0, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![18750, -8333, 18750, -16667, 112500, -46667, 100000, 0, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 1, 48, 54, 196609, 0, 49, 54, 196609, 1, 50, 54, 196609, 0, 51, 54, 196609, 1, 52, 54, 196609, 0, 53, 54};
        data.gdLst = vec![196609, 1, 48, 54, 196609, 0, 49, 54, 196609, 1, 50, 54, 196609, 0, 51, 54, 196609, 1, 52, 54, 196609, 0, 53, 54];
        // Java: data.ahLst = new int[]{3, 49, 56, 57, 48, 56, 57, 62, 58, 3, 51, 56, 57, 50, 56, 57, 70, 66, 3, 53, 56, 57, 52, 56, 57, 78, 74};
        data.ahLst = vec![3, 49, 56, 57, 48, 56, 57, 62, 58, 3, 51, 56, 57, 50, 56, 57, 70, 66, 3, 53, 56, 57, 52, 56, 57, 78, 74];
        // Java: data.cxnLst = new int[]{55, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![55, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}, {10, 13, 2, 0, 62, 26, 1, 62, 1}, {10, 13, 0, 78, 74, 1, 70, 66, 1, 62, 58}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2], vec![10, 13, 2, 0, 62, 26, 1, 62, 1], vec![10, 13, 0, 78, 74, 1, 70, 66, 1, 62, 58]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_accentCallout3()
    fn get_accentCallout3() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{18750, -8333, 18750, -16667, 100000, -16667, 112963, -8333, 100000, 0, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![18750, -8333, 18750, -16667, 100000, -16667, 112963, -8333, 100000, 0, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 1, 48, 56, 196609, 0, 49, 56, 196609, 1, 50, 56, 196609, 0, 51, 56, 196609, 1, 52, 56, 196609, 0, 53, 56, 196609, 1, 54, 56, 196609, 0, 55, 56};
        data.gdLst = vec![196609, 1, 48, 56, 196609, 0, 49, 56, 196609, 1, 50, 56, 196609, 0, 51, 56, 196609, 1, 52, 56, 196609, 0, 53, 56, 196609, 1, 54, 56, 196609, 0, 55, 56];
        // Java: data.ahLst = new int[]{3, 49, 58, 59, 48, 58, 59, 64, 60, 3, 51, 58, 59, 50, 58, 59, 72, 68, 3, 53, 58, 59, 52, 58, 59, 80, 76, 3, 55, 58, 59, 54, 58, 59, 88, 84};
        data.ahLst = vec![3, 49, 58, 59, 48, 58, 59, 64, 60, 3, 51, 58, 59, 50, 58, 59, 72, 68, 3, 53, 58, 59, 52, 58, 59, 80, 76, 3, 55, 58, 59, 54, 58, 59, 88, 84];
        // Java: data.cxnLst = new int[]{57, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![57, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}, {10, 13, 2, 0, 64, 26, 1, 64, 1}, {10, 13, 0, 88, 84, 1, 80, 76, 1, 72, 68, 1, 64, 60}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2], vec![10, 13, 2, 0, 64, 26, 1, 64, 1], vec![10, 13, 0, 88, 84, 1, 80, 76, 1, 72, 68, 1, 64, 60]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_actionButtonBackPrevious()
    fn get_actionButtonBackPrevious() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 8, 0};
        data.avcLst = vec![3, 8, 0];
        // Java: data.gdLst = new int[]{196609, 28, 48, 49, 196608, 3, 50, 51, 196608, 3, 51, 50, 196608, 2, 50, 51, 196608, 2, 51, 50};
        data.gdLst = vec![196609, 28, 48, 49, 196608, 3, 50, 51, 196608, 3, 51, 50, 196608, 2, 50, 51, 196608, 2, 51, 50];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 63, 3, 1, 67, 55, 1, 67, 59, 2}, {6, 12, 0, 63, 3, 1, 67, 55, 1, 67, 59, 2}, {10, 13, 0, 63, 3, 1, 67, 55, 1, 67, 59, 2}, {10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 63, 3, 1, 67, 55, 1, 67, 59, 2], vec![6, 12, 0, 63, 3, 1, 67, 55, 1, 67, 59, 2], vec![10, 13, 0, 63, 3, 1, 67, 55, 1, 67, 59, 2], vec![10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_actionButtonBeginning()
    fn get_actionButtonBeginning() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 8, 0, 4, 1};
        data.avcLst = vec![3, 8, 0, 4, 1];
        // Java: data.gdLst = new int[]{196609, 28, 48, 49, 196608, 3, 50, 53, 196608, 3, 53, 50, 196608, 2, 50, 53, 196608, 2, 53, 50, 196609, 28, 48, 51, 196609, 73, 52, 49, 196609, 73, 52, 51, 196608, 65, 77, 50, 196608, 65, 81, 50};
        data.gdLst = vec![196609, 28, 48, 49, 196608, 3, 50, 53, 196608, 3, 53, 50, 196608, 2, 50, 53, 196608, 2, 53, 50, 196609, 28, 48, 51, 196609, 73, 52, 49, 196609, 73, 52, 51, 196608, 65, 77, 50, 196608, 65, 81, 50];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 89, 3, 1, 69, 57, 1, 69, 61, 2, 0, 85, 57, 1, 65, 57, 1, 65, 61, 1, 85, 61, 2}, {6, 12, 0, 89, 3, 1, 69, 57, 1, 69, 61, 2, 0, 85, 57, 1, 65, 57, 1, 65, 61, 1, 85, 61, 2}, {10, 13, 0, 89, 3, 1, 69, 57, 1, 69, 61, 2, 0, 85, 57, 1, 85, 61, 1, 65, 61, 1, 65, 57, 2}, {10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 89, 3, 1, 69, 57, 1, 69, 61, 2, 0, 85, 57, 1, 65, 57, 1, 65, 61, 1, 85, 61, 2], vec![6, 12, 0, 89, 3, 1, 69, 57, 1, 69, 61, 2, 0, 85, 57, 1, 65, 57, 1, 65, 61, 1, 85, 61, 2], vec![10, 13, 0, 89, 3, 1, 69, 57, 1, 69, 61, 2, 0, 85, 57, 1, 85, 61, 1, 65, 61, 1, 65, 57, 2], vec![10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_actionButtonBlank()
    fn get_actionButtonBlank() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{0};
        data.avcLst = vec![0];
        // Java: data.gdLst = new int[0];
        data.gdLst = vec![];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{48, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![48, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_actionButtonDocument()
    fn get_actionButtonDocument() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 8, 0, 9, 32, 16};
        data.avcLst = vec![3, 8, 0, 9, 32, 16];
        // Java: data.gdLst = new int[]{196609, 28, 48, 49, 196608, 3, 50, 54, 196608, 3, 54, 50, 196609, 28, 51, 52, 196608, 2, 50, 66, 196608, 2, 66, 50, 196609, 28, 48, 53, 196608, 74, 50, 78, 196608, 58, 78, 50};
        data.gdLst = vec![196609, 28, 48, 49, 196608, 3, 50, 54, 196608, 3, 54, 50, 196609, 28, 51, 52, 196608, 2, 50, 66, 196608, 2, 66, 50, 196609, 28, 48, 53, 196608, 74, 50, 78, 196608, 58, 78, 50];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 70, 58, 1, 82, 58, 1, 74, 86, 1, 74, 62, 1, 70, 62, 2}, {7, 12, 0, 70, 58, 1, 82, 58, 1, 82, 86, 1, 74, 86, 1, 74, 62, 1, 70, 62, 2}, {6, 12, 0, 82, 58, 1, 82, 86, 1, 74, 86, 2}, {10, 13, 0, 70, 58, 1, 82, 58, 1, 74, 86, 1, 74, 62, 1, 70, 62, 2, 0, 74, 86, 1, 82, 86, 1, 82, 58}, {10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 70, 58, 1, 82, 58, 1, 74, 86, 1, 74, 62, 1, 70, 62, 2], vec![7, 12, 0, 70, 58, 1, 82, 58, 1, 82, 86, 1, 74, 86, 1, 74, 62, 1, 70, 62, 2], vec![6, 12, 0, 82, 58, 1, 82, 86, 1, 74, 86, 2], vec![10, 13, 0, 70, 58, 1, 82, 58, 1, 74, 86, 1, 74, 62, 1, 70, 62, 2, 0, 74, 86, 1, 82, 86, 1, 82, 58], vec![10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_actionButtonEnd()
    fn get_actionButtonEnd() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 8, 0, 4, 7};
        data.avcLst = vec![3, 8, 0, 4, 7];
        // Java: data.gdLst = new int[]{196609, 28, 48, 49, 196608, 3, 50, 53, 196608, 3, 53, 50, 196608, 2, 50, 53, 196608, 2, 53, 50, 196609, 28, 48, 51, 196609, 73, 48, 51, 196609, 73, 52, 49, 196608, 65, 77, 50, 196608, 65, 81, 50};
        data.gdLst = vec![196609, 28, 48, 49, 196608, 3, 50, 53, 196608, 3, 53, 50, 196608, 2, 50, 53, 196608, 2, 53, 50, 196609, 28, 48, 51, 196609, 73, 48, 51, 196609, 73, 52, 49, 196608, 65, 77, 50, 196608, 65, 81, 50];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 85, 3, 1, 65, 57, 1, 65, 61, 2, 0, 89, 57, 1, 69, 57, 1, 69, 61, 1, 89, 61, 2}, {6, 12, 0, 85, 3, 1, 65, 57, 1, 65, 61, 2, 0, 89, 57, 1, 69, 57, 1, 69, 61, 1, 89, 61, 2}, {10, 13, 0, 85, 3, 1, 65, 61, 1, 65, 57, 2, 0, 89, 57, 1, 69, 57, 1, 69, 61, 1, 89, 61, 2}, {10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 85, 3, 1, 65, 57, 1, 65, 61, 2, 0, 89, 57, 1, 69, 57, 1, 69, 61, 1, 89, 61, 2], vec![6, 12, 0, 85, 3, 1, 65, 57, 1, 65, 61, 2, 0, 89, 57, 1, 69, 57, 1, 69, 61, 1, 89, 61, 2], vec![10, 13, 0, 85, 3, 1, 65, 61, 1, 65, 57, 2, 0, 89, 57, 1, 69, 57, 1, 69, 61, 1, 89, 61, 2], vec![10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_actionButtonForwardNext()
    fn get_actionButtonForwardNext() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 8, 0};
        data.avcLst = vec![3, 8, 0];
        // Java: data.gdLst = new int[]{196609, 28, 48, 49, 196608, 3, 50, 51, 196608, 3, 51, 50, 196608, 2, 50, 51, 196608, 2, 51, 50};
        data.gdLst = vec![196609, 28, 48, 49, 196608, 3, 50, 51, 196608, 3, 51, 50, 196608, 2, 50, 51, 196608, 2, 51, 50];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 67, 3, 1, 63, 55, 1, 63, 59, 2}, {6, 12, 0, 67, 3, 1, 63, 55, 1, 63, 59, 2}, {10, 13, 0, 67, 3, 1, 63, 59, 1, 63, 55, 2}, {10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 67, 3, 1, 63, 55, 1, 63, 59, 2], vec![6, 12, 0, 67, 3, 1, 63, 55, 1, 63, 59, 2], vec![10, 13, 0, 67, 3, 1, 63, 59, 1, 63, 55, 2], vec![10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_actionButtonHelp()
    fn get_actionButtonHelp() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 8, 0, 4, 1, 7, 14, 2, 17, 28, 21, 11, -5400000, -10800000, 21600000};
        data.avcLst = vec![3, 8, 0, 4, 1, 7, 14, 2, 17, 28, 21, 11, -5400000, -10800000, 21600000];
        // Java: data.gdLst = new int[]{196609, 28, 48, 49, 196608, 3, 50, 63, 196608, 2, 50, 63, 196609, 28, 48, 51, 196609, 75, 52, 53, 196609, 75, 48, 54, 196609, 75, 55, 53, 196609, 75, 48, 53, 196609, 75, 51, 53, 196609, 75, 56, 57, 196609, 75, 58, 57, 196609, 75, 59, 54, 196608, 67, 87, 50, 196608, 67, 99, 50, 196608, 67, 103, 50, 196608, 67, 107, 50, 196608, 71, 83, 50, 196608, 71, 91, 50, 196608, 71, 95, 50, 196609, 75, 52, 54, 196609, 75, 48, 57};
        data.gdLst = vec![196609, 28, 48, 49, 196608, 3, 50, 63, 196608, 2, 50, 63, 196609, 28, 48, 51, 196609, 75, 52, 53, 196609, 75, 48, 54, 196609, 75, 55, 53, 196609, 75, 48, 53, 196609, 75, 51, 53, 196609, 75, 56, 57, 196609, 75, 58, 57, 196609, 75, 59, 54, 196608, 67, 87, 50, 196608, 67, 99, 50, 196608, 67, 103, 50, 196608, 67, 107, 50, 196608, 71, 83, 50, 196608, 71, 91, 50, 196608, 71, 95, 50, 196609, 75, 52, 54, 196609, 75, 48, 57];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 127, 111, 3, 87, 87, 19, 19, 3, 79, 83, 50, 20, 3, 139, 143, 15, 60, 1, 135, 119, 1, 131, 119, 1, 131, 115, 3, 79, 83, 19, 20, 3, 139, 143, 20, 60, 3, 79, 79, 50, 61, 2, 0, 2, 123, 3, 143, 143, 15, 62, 2}, {6, 12, 0, 127, 111, 3, 87, 87, 19, 19, 3, 79, 83, 50, 20, 3, 139, 143, 15, 60, 1, 135, 119, 1, 131, 119, 1, 131, 115, 3, 79, 83, 19, 20, 3, 139, 143, 20, 60, 3, 79, 79, 50, 61, 2, 0, 2, 123, 3, 143, 143, 15, 62, 2}, {10, 13, 0, 127, 111, 3, 87, 87, 19, 19, 3, 79, 83, 50, 20, 3, 139, 143, 15, 60, 1, 135, 119, 1, 131, 119, 1, 131, 115, 3, 79, 83, 19, 20, 3, 139, 143, 20, 60, 3, 79, 79, 50, 61, 2, 0, 2, 123, 3, 143, 143, 15, 62, 2}, {10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 127, 111, 3, 87, 87, 19, 19, 3, 79, 83, 50, 20, 3, 139, 143, 15, 60, 1, 135, 119, 1, 131, 119, 1, 131, 115, 3, 79, 83, 19, 20, 3, 139, 143, 20, 60, 3, 79, 79, 50, 61, 2, 0, 2, 123, 3, 143, 143, 15, 62, 2], vec![6, 12, 0, 127, 111, 3, 87, 87, 19, 19, 3, 79, 83, 50, 20, 3, 139, 143, 15, 60, 1, 135, 119, 1, 131, 119, 1, 131, 115, 3, 79, 83, 19, 20, 3, 139, 143, 20, 60, 3, 79, 79, 50, 61, 2, 0, 2, 123, 3, 143, 143, 15, 62, 2], vec![10, 13, 0, 127, 111, 3, 87, 87, 19, 19, 3, 79, 83, 50, 20, 3, 139, 143, 15, 60, 1, 135, 119, 1, 131, 119, 1, 131, 115, 3, 79, 83, 19, 20, 3, 139, 143, 20, 60, 3, 79, 79, 50, 61, 2, 0, 2, 123, 3, 143, 143, 15, 62, 2], vec![10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_actionButtonHome()
    fn get_actionButtonHome() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 8, 0, 4, 1, 16, 5, 7, 9, 11, 13};
        data.avcLst = vec![3, 8, 0, 4, 1, 16, 5, 7, 9, 11, 13];
        // Java: data.gdLst = new int[]{196609, 28, 48, 49, 196608, 3, 50, 59, 196608, 3, 59, 50, 196608, 2, 50, 59, 196608, 2, 59, 50, 196609, 28, 48, 51, 196609, 79, 52, 53, 196609, 79, 52, 49, 196609, 79, 48, 53, 196609, 79, 54, 53, 196609, 79, 55, 53, 196609, 79, 56, 53, 196609, 79, 57, 53, 196609, 79, 48, 51, 196609, 79, 58, 53, 196609, 79, 55, 49, 196608, 63, 83, 50, 196608, 63, 91, 50, 196608, 63, 95, 50, 196608, 63, 111, 50, 196608, 71, 87, 50, 196608, 71, 99, 50, 196608, 71, 103, 50, 196608, 71, 107, 50, 196608, 71, 115, 50, 196608, 71, 119, 50};
        data.gdLst = vec![196609, 28, 48, 49, 196608, 3, 50, 59, 196608, 3, 59, 50, 196608, 2, 50, 59, 196608, 2, 59, 50, 196609, 28, 48, 51, 196609, 79, 52, 53, 196609, 79, 52, 49, 196609, 79, 48, 53, 196609, 79, 54, 53, 196609, 79, 55, 53, 196609, 79, 56, 53, 196609, 79, 57, 53, 196609, 79, 48, 51, 196609, 79, 58, 53, 196609, 79, 55, 49, 196608, 63, 83, 50, 196608, 63, 91, 50, 196608, 63, 95, 50, 196608, 63, 111, 50, 196608, 71, 87, 50, 196608, 71, 99, 50, 196608, 71, 103, 50, 196608, 71, 107, 50, 196608, 71, 115, 50, 196608, 71, 119, 50];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 2, 63, 1, 71, 3, 1, 139, 3, 1, 139, 67, 1, 159, 67, 1, 159, 3, 1, 75, 3, 1, 155, 131, 1, 155, 123, 1, 151, 123, 1, 151, 127, 2}, {7, 12, 0, 155, 131, 1, 155, 123, 1, 151, 123, 1, 151, 127, 2, 0, 139, 3, 1, 139, 67, 1, 143, 67, 1, 143, 135, 1, 147, 135, 1, 147, 67, 1, 159, 67, 1, 159, 3, 2}, {6, 12, 0, 2, 63, 1, 71, 3, 1, 75, 3, 2, 0, 143, 135, 1, 147, 135, 1, 147, 67, 1, 143, 67, 2}, {10, 13, 0, 2, 63, 1, 151, 127, 1, 151, 123, 1, 155, 123, 1, 155, 131, 1, 75, 3, 1, 159, 3, 1, 159, 67, 1, 139, 67, 1, 139, 3, 1, 71, 3, 2, 0, 151, 127, 1, 155, 131, 0, 159, 3, 1, 139, 3, 0, 143, 67, 1, 143, 135, 1, 147, 135, 1, 147, 67}, {10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 2, 63, 1, 71, 3, 1, 139, 3, 1, 139, 67, 1, 159, 67, 1, 159, 3, 1, 75, 3, 1, 155, 131, 1, 155, 123, 1, 151, 123, 1, 151, 127, 2], vec![7, 12, 0, 155, 131, 1, 155, 123, 1, 151, 123, 1, 151, 127, 2, 0, 139, 3, 1, 139, 67, 1, 143, 67, 1, 143, 135, 1, 147, 135, 1, 147, 67, 1, 159, 67, 1, 159, 3, 2], vec![6, 12, 0, 2, 63, 1, 71, 3, 1, 75, 3, 2, 0, 143, 135, 1, 147, 135, 1, 147, 67, 1, 143, 67, 2], vec![10, 13, 0, 2, 63, 1, 151, 127, 1, 151, 123, 1, 155, 123, 1, 155, 131, 1, 75, 3, 1, 159, 3, 1, 159, 67, 1, 139, 67, 1, 139, 3, 1, 71, 3, 2, 0, 151, 127, 1, 155, 131, 0, 159, 3, 1, 139, 3, 0, 143, 67, 1, 143, 135, 1, 147, 135, 1, 147, 67], vec![10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_actionButtonInformation()
    fn get_actionButtonInformation() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 8, 0, 4, 1, 32, 5, 16, 13, 19, 11, 7, 21600000};
        data.avcLst = vec![3, 8, 0, 4, 1, 32, 5, 16, 13, 19, 11, 7, 21600000];
        // Java: data.gdLst = new int[]{196609, 28, 48, 49, 196608, 3, 50, 61, 196608, 2, 50, 61, 196609, 28, 48, 51, 196609, 73, 52, 53, 196609, 73, 54, 55, 196609, 73, 48, 49, 196609, 73, 56, 53, 196609, 73, 57, 53, 196609, 73, 58, 55, 196609, 73, 56, 55, 196609, 73, 59, 49, 196608, 65, 77, 50, 196608, 65, 81, 50, 196608, 65, 85, 50, 196608, 65, 101, 50, 196608, 65, 105, 50, 196608, 69, 81, 50, 196608, 69, 89, 50, 196608, 69, 93, 50, 196608, 69, 97, 50, 196609, 73, 48, 53};
        data.gdLst = vec![196609, 28, 48, 49, 196608, 3, 50, 61, 196608, 2, 50, 61, 196609, 28, 48, 51, 196609, 73, 52, 53, 196609, 73, 54, 55, 196609, 73, 48, 49, 196609, 73, 56, 53, 196609, 73, 57, 53, 196609, 73, 58, 55, 196609, 73, 56, 55, 196609, 73, 59, 49, 196608, 65, 77, 50, 196608, 65, 81, 50, 196608, 65, 85, 50, 196608, 65, 101, 50, 196608, 65, 105, 50, 196608, 69, 81, 50, 196608, 69, 89, 50, 196608, 69, 93, 50, 196608, 69, 97, 50, 196609, 73, 48, 53];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 2, 65, 3, 61, 61, 15, 60, 2}, {6, 12, 0, 2, 65, 3, 61, 61, 15, 60, 2, 0, 2, 109, 3, 145, 145, 15, 60, 0, 129, 113, 1, 129, 117, 1, 133, 117, 1, 133, 121, 1, 129, 121, 1, 129, 125, 1, 141, 125, 1, 141, 121, 1, 137, 121, 1, 137, 113, 2}, {8, 12, 0, 2, 109, 3, 145, 145, 15, 60, 0, 129, 113, 1, 137, 113, 1, 137, 121, 1, 141, 121, 1, 141, 125, 1, 129, 125, 1, 129, 121, 1, 133, 121, 1, 133, 117, 1, 129, 117, 2}, {10, 13, 0, 2, 65, 3, 61, 61, 15, 60, 2, 0, 2, 109, 3, 145, 145, 15, 60, 0, 129, 113, 1, 137, 113, 1, 137, 121, 1, 141, 121, 1, 141, 125, 1, 129, 125, 1, 129, 121, 1, 133, 121, 1, 133, 117, 1, 129, 117, 2}, {10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 2, 65, 3, 61, 61, 15, 60, 2], vec![6, 12, 0, 2, 65, 3, 61, 61, 15, 60, 2, 0, 2, 109, 3, 145, 145, 15, 60, 0, 129, 113, 1, 129, 117, 1, 133, 117, 1, 133, 121, 1, 129, 121, 1, 129, 125, 1, 141, 125, 1, 141, 121, 1, 137, 121, 1, 137, 113, 2], vec![8, 12, 0, 2, 109, 3, 145, 145, 15, 60, 0, 129, 113, 1, 137, 113, 1, 137, 121, 1, 141, 121, 1, 141, 125, 1, 129, 125, 1, 129, 121, 1, 133, 121, 1, 133, 117, 1, 129, 117, 2], vec![10, 13, 0, 2, 65, 3, 61, 61, 15, 60, 2, 0, 2, 109, 3, 145, 145, 15, 60, 0, 129, 113, 1, 137, 113, 1, 137, 121, 1, 141, 121, 1, 141, 125, 1, 129, 125, 1, 129, 121, 1, 133, 121, 1, 133, 117, 1, 129, 117, 2], vec![10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_actionButtonMovie()
    fn get_actionButtonMovie() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 8, 0, 4, 1455, 21600, 1905, 2325, 16155, 17010, 19335, 19725, 20595, 5280, 5730, 6630, 7492, 9067, 9555, 13342, 14580, 15592};
        data.avcLst = vec![3, 8, 0, 4, 1455, 21600, 1905, 2325, 16155, 17010, 19335, 19725, 20595, 5280, 5730, 6630, 7492, 9067, 9555, 13342, 14580, 15592];
        // Java: data.gdLst = new int[]{196609, 28, 48, 49, 196608, 3, 50, 70, 196608, 3, 70, 50, 196608, 2, 50, 70, 196608, 2, 70, 50, 196609, 28, 48, 51, 196609, 90, 52, 53, 196609, 90, 54, 53, 196609, 90, 55, 53, 196609, 90, 56, 53, 196609, 90, 57, 53, 196609, 90, 58, 53, 196609, 90, 59, 53, 196609, 90, 60, 53, 196609, 90, 61, 53, 196609, 90, 62, 53, 196609, 90, 63, 53, 196609, 90, 64, 53, 196609, 90, 65, 53, 196609, 90, 66, 53, 196609, 90, 67, 53, 196609, 90, 68, 53, 196609, 90, 69, 53, 196608, 82, 94, 50, 196608, 82, 98, 50, 196608, 82, 102, 50, 196608, 82, 106, 50, 196608, 82, 110, 50, 196608, 82, 114, 50, 196608, 82, 118, 50, 196608, 82, 122, 50, 196608, 74, 126, 50, 196608, 74, 130, 50, 196608, 74, 134, 50, 196608, 74, 138, 50, 196608, 74, 142, 50, 196608, 74, 146, 50, 196608, 74, 150, 50, 196608, 74, 154, 50, 196608, 74, 158, 50, 196608, 74, 162, 50};
        data.gdLst = vec![196609, 28, 48, 49, 196608, 3, 50, 70, 196608, 3, 70, 50, 196608, 2, 50, 70, 196608, 2, 70, 50, 196609, 28, 48, 51, 196609, 90, 52, 53, 196609, 90, 54, 53, 196609, 90, 55, 53, 196609, 90, 56, 53, 196609, 90, 57, 53, 196609, 90, 58, 53, 196609, 90, 59, 53, 196609, 90, 60, 53, 196609, 90, 61, 53, 196609, 90, 62, 53, 196609, 90, 63, 53, 196609, 90, 64, 53, 196609, 90, 65, 53, 196609, 90, 66, 53, 196609, 90, 67, 53, 196609, 90, 68, 53, 196609, 90, 69, 53, 196608, 82, 94, 50, 196608, 82, 98, 50, 196608, 82, 102, 50, 196608, 82, 106, 50, 196608, 82, 110, 50, 196608, 82, 114, 50, 196608, 82, 118, 50, 196608, 82, 122, 50, 196608, 74, 126, 50, 196608, 74, 130, 50, 196608, 74, 134, 50, 196608, 74, 138, 50, 196608, 74, 142, 50, 196608, 74, 146, 50, 196608, 74, 150, 50, 196608, 74, 154, 50, 196608, 74, 158, 50, 196608, 74, 162, 50];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 82, 194, 1, 82, 214, 1, 162, 214, 1, 166, 210, 1, 170, 210, 1, 170, 226, 1, 178, 226, 1, 178, 218, 1, 182, 218, 1, 190, 222, 1, 86, 222, 1, 86, 202, 1, 190, 202, 1, 186, 206, 1, 178, 206, 1, 178, 202, 1, 174, 198, 1, 166, 198, 1, 162, 194, 2}, {6, 12, 0, 82, 194, 1, 82, 214, 1, 162, 214, 1, 166, 210, 1, 170, 210, 1, 170, 226, 1, 178, 226, 1, 178, 218, 1, 182, 218, 1, 190, 222, 1, 86, 222, 1, 86, 202, 1, 190, 202, 1, 186, 206, 1, 178, 206, 1, 178, 202, 1, 174, 198, 1, 166, 198, 1, 162, 194, 2}, {10, 13, 0, 82, 194, 1, 162, 194, 1, 166, 198, 1, 174, 198, 1, 178, 202, 1, 178, 206, 1, 186, 206, 1, 190, 202, 1, 86, 202, 1, 86, 222, 1, 190, 222, 1, 182, 218, 1, 178, 218, 1, 178, 226, 1, 170, 226, 1, 170, 210, 1, 166, 210, 1, 162, 214, 1, 82, 214, 2}, {10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 82, 194, 1, 82, 214, 1, 162, 214, 1, 166, 210, 1, 170, 210, 1, 170, 226, 1, 178, 226, 1, 178, 218, 1, 182, 218, 1, 190, 222, 1, 86, 222, 1, 86, 202, 1, 190, 202, 1, 186, 206, 1, 178, 206, 1, 178, 202, 1, 174, 198, 1, 166, 198, 1, 162, 194, 2], vec![6, 12, 0, 82, 194, 1, 82, 214, 1, 162, 214, 1, 166, 210, 1, 170, 210, 1, 170, 226, 1, 178, 226, 1, 178, 218, 1, 182, 218, 1, 190, 222, 1, 86, 222, 1, 86, 202, 1, 190, 202, 1, 186, 206, 1, 178, 206, 1, 178, 202, 1, 174, 198, 1, 166, 198, 1, 162, 194, 2], vec![10, 13, 0, 82, 194, 1, 162, 194, 1, 166, 198, 1, 174, 198, 1, 178, 202, 1, 178, 206, 1, 186, 206, 1, 190, 202, 1, 86, 202, 1, 86, 222, 1, 190, 222, 1, 182, 218, 1, 178, 218, 1, 178, 226, 1, 170, 226, 1, 170, 210, 1, 166, 210, 1, 162, 214, 1, 82, 214, 2], vec![10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_actionButtonReturn()
    fn get_actionButtonReturn() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 8, 0, 4, 7, 5, 1, -5400000};
        data.avcLst = vec![3, 8, 0, 4, 7, 5, 1, -5400000];
        // Java: data.gdLst = new int[]{196609, 28, 48, 49, 196608, 3, 50, 56, 196608, 3, 56, 50, 196608, 2, 50, 56, 196608, 2, 56, 50, 196609, 28, 48, 51, 196609, 76, 52, 49, 196609, 76, 48, 51, 196609, 76, 53, 49, 196609, 76, 48, 49, 196609, 76, 54, 51, 196608, 60, 84, 50, 196608, 60, 88, 50, 196608, 60, 96, 50, 196608, 68, 80, 50, 196608, 68, 84, 50, 196608, 68, 88, 50, 196608, 68, 92, 50, 196608, 68, 96, 50, 196609, 76, 54, 49};
        data.gdLst = vec![196609, 28, 48, 49, 196608, 3, 50, 56, 196608, 3, 56, 50, 196608, 2, 50, 56, 196608, 2, 56, 50, 196609, 28, 48, 51, 196609, 76, 52, 49, 196609, 76, 48, 51, 196609, 76, 53, 49, 196609, 76, 48, 49, 196609, 76, 54, 51, 196608, 60, 84, 50, 196608, 60, 88, 50, 196608, 60, 96, 50, 196608, 68, 80, 50, 196608, 68, 84, 50, 196608, 68, 88, 50, 196608, 68, 92, 50, 196608, 68, 96, 50, 196609, 76, 54, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 72, 108, 1, 116, 60, 1, 2, 108, 1, 120, 108, 1, 120, 104, 3, 132, 132, 50, 20, 1, 124, 100, 3, 132, 132, 20, 20, 1, 128, 108, 1, 68, 108, 1, 68, 104, 3, 92, 92, 19, 55, 1, 2, 64, 3, 92, 92, 20, 55, 1, 112, 108, 2}, {6, 12, 0, 72, 108, 1, 116, 60, 1, 2, 108, 1, 120, 108, 1, 120, 104, 3, 132, 132, 50, 20, 1, 124, 100, 3, 132, 132, 20, 20, 1, 128, 108, 1, 68, 108, 1, 68, 104, 3, 92, 92, 19, 55, 1, 2, 64, 3, 92, 92, 20, 55, 1, 112, 108, 2}, {10, 13, 0, 72, 108, 1, 112, 108, 1, 112, 104, 3, 92, 92, 50, 20, 1, 124, 64, 3, 92, 92, 20, 20, 1, 68, 108, 1, 128, 108, 1, 128, 104, 3, 132, 132, 19, 55, 1, 2, 100, 3, 132, 132, 20, 55, 1, 120, 108, 1, 2, 108, 1, 116, 60, 2}, {10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 72, 108, 1, 116, 60, 1, 2, 108, 1, 120, 108, 1, 120, 104, 3, 132, 132, 50, 20, 1, 124, 100, 3, 132, 132, 20, 20, 1, 128, 108, 1, 68, 108, 1, 68, 104, 3, 92, 92, 19, 55, 1, 2, 64, 3, 92, 92, 20, 55, 1, 112, 108, 2], vec![6, 12, 0, 72, 108, 1, 116, 60, 1, 2, 108, 1, 120, 108, 1, 120, 104, 3, 132, 132, 50, 20, 1, 124, 100, 3, 132, 132, 20, 20, 1, 128, 108, 1, 68, 108, 1, 68, 104, 3, 92, 92, 19, 55, 1, 2, 64, 3, 92, 92, 20, 55, 1, 112, 108, 2], vec![10, 13, 0, 72, 108, 1, 112, 108, 1, 112, 104, 3, 92, 92, 50, 20, 1, 124, 64, 3, 92, 92, 20, 20, 1, 68, 108, 1, 128, 108, 1, 128, 104, 3, 132, 132, 19, 55, 1, 2, 100, 3, 132, 132, 20, 55, 1, 120, 108, 1, 2, 108, 1, 116, 60, 2], vec![10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_actionButtonSound()
    fn get_actionButtonSound() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 8, 0, 4, 1, 5, 16, 11, 7};
        data.avcLst = vec![3, 8, 0, 4, 1, 5, 16, 11, 7];
        // Java: data.gdLst = new int[]{196609, 28, 48, 49, 196608, 3, 50, 57, 196608, 3, 57, 50, 196608, 2, 50, 57, 196608, 2, 57, 50, 196609, 28, 48, 51, 196609, 77, 52, 49, 196609, 77, 53, 54, 196609, 77, 53, 49, 196609, 77, 55, 54, 196609, 77, 48, 51, 196609, 77, 56, 49, 196608, 61, 81, 50, 196608, 61, 85, 50, 196608, 61, 93, 50, 196608, 61, 101, 50, 196608, 69, 85, 50, 196608, 69, 89, 50, 196608, 69, 97, 50};
        data.gdLst = vec![196609, 28, 48, 49, 196608, 3, 50, 57, 196608, 3, 57, 50, 196608, 2, 50, 57, 196608, 2, 57, 50, 196609, 28, 48, 51, 196609, 77, 52, 49, 196609, 77, 53, 54, 196609, 77, 53, 49, 196609, 77, 55, 54, 196609, 77, 48, 51, 196609, 77, 56, 49, 196608, 61, 81, 50, 196608, 61, 85, 50, 196608, 61, 93, 50, 196608, 61, 101, 50, 196608, 69, 85, 50, 196608, 69, 89, 50, 196608, 69, 97, 50];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 69, 109, 1, 69, 113, 1, 121, 113, 1, 125, 65, 1, 125, 61, 1, 121, 109, 2}, {6, 12, 0, 69, 109, 1, 69, 113, 1, 121, 113, 1, 125, 65, 1, 125, 61, 1, 121, 109, 2}, {10, 13, 0, 69, 109, 1, 121, 109, 1, 125, 61, 1, 125, 65, 1, 121, 113, 1, 69, 113, 2, 0, 129, 109, 1, 73, 105, 0, 129, 3, 1, 73, 3, 0, 129, 113, 1, 73, 117}, {10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 69, 109, 1, 69, 113, 1, 121, 113, 1, 125, 65, 1, 125, 61, 1, 121, 109, 2], vec![6, 12, 0, 69, 109, 1, 69, 113, 1, 121, 113, 1, 125, 65, 1, 125, 61, 1, 121, 109, 2], vec![10, 13, 0, 69, 109, 1, 121, 109, 1, 125, 61, 1, 125, 65, 1, 121, 113, 1, 69, 113, 2, 0, 129, 109, 1, 73, 105, 0, 129, 3, 1, 73, 3, 0, 129, 113, 1, 73, 117], vec![10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_arc()
    fn get_arc() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{16200000, 0, 0, 21599999, 21600000, 27000000, 32400000, 37800000, 2};
        data.avcLst = vec![16200000, 0, 0, 21599999, 21600000, 27000000, 32400000, 37800000, 2];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196625, 50, 49, 51, 196608, 61, 50, 57, 196608, 65, 52, 50, 196614, 65, 65, 69, 131092, 2, 57, 131093, 3, 57, 196619, 2, 80, 77, 196620, 3, 80, 77, 131092, 2, 61, 131093, 3, 61, 196619, 2, 94, 91, 196620, 3, 94, 91, 196608, 2, 83, 50, 196608, 3, 87, 50, 196608, 2, 97, 50, 196608, 3, 101, 50, 196608, 52, 50, 57, 196608, 73, 50, 121, 131077, 105, 113, 196614, 125, 0, 129, 196608, 20, 50, 57, 196608, 53, 50, 57, 196614, 136, 136, 140, 196608, 73, 50, 144, 131077, 109, 117, 196614, 148, 1, 152, 196608, 19, 50, 57, 196608, 54, 50, 57, 196614, 159, 159, 163, 196608, 73, 50, 167, 131076, 105, 113, 196614, 171, 26, 175, 196608, 15, 50, 57, 196608, 55, 50, 57, 196614, 182, 182, 186, 196608, 73, 50, 190, 131076, 109, 117, 196614, 194, 26, 198, 196608, 57, 50, 20, 196608, 61, 20, 50, 196610, 205, 209, 56};
        data.gdLst = vec![196625, 50, 48, 51, 196625, 50, 49, 51, 196608, 61, 50, 57, 196608, 65, 52, 50, 196614, 65, 65, 69, 131092, 2, 57, 131093, 3, 57, 196619, 2, 80, 77, 196620, 3, 80, 77, 131092, 2, 61, 131093, 3, 61, 196619, 2, 94, 91, 196620, 3, 94, 91, 196608, 2, 83, 50, 196608, 3, 87, 50, 196608, 2, 97, 50, 196608, 3, 101, 50, 196608, 52, 50, 57, 196608, 73, 50, 121, 131077, 105, 113, 196614, 125, 0, 129, 196608, 20, 50, 57, 196608, 53, 50, 57, 196614, 136, 136, 140, 196608, 73, 50, 144, 131077, 109, 117, 196614, 148, 1, 152, 196608, 19, 50, 57, 196608, 54, 50, 57, 196614, 159, 159, 163, 196608, 73, 50, 167, 131076, 105, 113, 196614, 171, 26, 175, 196608, 15, 50, 57, 196608, 55, 50, 57, 196614, 182, 182, 186, 196608, 73, 50, 190, 131076, 109, 117, 196614, 194, 26, 198, 196608, 57, 50, 20, 196608, 61, 20, 50, 196610, 205, 209, 56];
        // Java: data.ahLst = new int[]{4, 48, 50, 51, 105, 109, 4, 49, 50, 51, 113, 117};
        data.ahLst = vec![4, 48, 50, 51, 105, 109, 4, 49, 50, 51, 113, 117];
        // Java: data.cxnLst = new int[]{205, 105, 109, 213, 2, 3, 209, 113, 117};
        data.cxnLst = vec![205, 105, 109, 213, 2, 3, 209, 113, 117];
        // Java: data.rect = new int[]{178, 201, 132, 155};
        data.rect = vec![178, 201, 132, 155];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 105, 109, 3, 2, 3, 57, 73, 1, 2, 3, 2}, {10, 13, 0, 105, 109, 3, 2, 3, 57, 73}};
        data.pathLst = vec![vec![11, 12, 0, 105, 109, 3, 2, 3, 57, 73, 1, 2, 3, 2], vec![10, 13, 0, 105, 109, 3, 2, 3, 57, 73]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_bentArrow()
    fn get_bentArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 25000, 25000, 43750, 0, 50000, 2, 1, 100000, -5400000};
        data.avcLst = vec![25000, 25000, 25000, 43750, 0, 50000, 2, 1, 100000, -5400000];
        // Java: data.gdLst = new int[]{196625, 52, 49, 53, 196609, 58, 54, 55, 196625, 52, 48, 62, 196625, 52, 50, 53, 196609, 28, 66, 56, 196609, 28, 58, 56, 196609, 74, 55, 54, 196608, 78, 52, 82, 196609, 28, 70, 56, 196608, 0, 52, 90, 196608, 1, 52, 86, 131076, 94, 98, 196609, 56, 102, 28, 196625, 52, 51, 105, 196609, 28, 109, 56, 196608, 113, 52, 74, 131077, 117, 52, 196608, 74, 121, 52, 196608, 0, 52, 90, 196608, 86, 74, 52, 196608, 132, 86, 52, 196608, 86, 113, 52, 196608, 132, 121, 52};
        data.gdLst = vec![196625, 52, 49, 53, 196609, 58, 54, 55, 196625, 52, 48, 62, 196625, 52, 50, 53, 196609, 28, 66, 56, 196609, 28, 58, 56, 196609, 74, 55, 54, 196608, 78, 52, 82, 196609, 28, 70, 56, 196608, 0, 52, 90, 196608, 1, 52, 86, 131076, 94, 98, 196609, 56, 102, 28, 196625, 52, 51, 105, 196609, 28, 109, 56, 196608, 113, 52, 74, 131077, 117, 52, 196608, 74, 121, 52, 196608, 0, 52, 90, 196608, 86, 74, 52, 196608, 132, 86, 52, 196608, 86, 113, 52, 196608, 132, 121, 52];
        // Java: data.ahLst = new int[]{1, 48, 52, 62, 74, 1, 2, 49, 52, 53, 0, 136, 1, 50, 52, 53, 128, 26, 1, 51, 52, 105, 113, 26};
        data.ahLst = vec![1, 48, 52, 62, 74, 1, 2, 49, 52, 53, 0, 136, 1, 50, 52, 53, 128, 26, 1, 51, 52, 105, 113, 26];
        // Java: data.cxnLst = new int[]{15, 128, 26, 20, 128, 136, 20, 82, 1, 52, 0, 78};
        data.cxnLst = vec![15, 128, 26, 20, 128, 136, 20, 82, 1, 52, 0, 78];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 1, 1, 26, 140, 3, 113, 113, 19, 20, 1, 128, 86, 1, 128, 26, 1, 0, 78, 1, 128, 136, 1, 128, 132, 1, 124, 132, 3, 121, 121, 15, 57, 1, 74, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 1, 1, 26, 140, 3, 113, 113, 19, 20, 1, 128, 86, 1, 128, 26, 1, 0, 78, 1, 128, 136, 1, 128, 132, 1, 124, 132, 3, 121, 121, 15, 57, 1, 74, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_bentConnector2()
    fn get_bentConnector2() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[0];
        data.avcLst = vec![];
        // Java: data.gdLst = new int[0];
        data.gdLst = vec![];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[0];
        data.cxnLst = vec![];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1}};
        data.pathLst = vec![vec![10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_bentConnector3()
    fn get_bentConnector3() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 100000, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![50000, 100000, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49};
        data.gdLst = vec![196609, 0, 48, 49];
        // Java: data.ahLst = new int[]{1, 48, 50, 51, 52, 3};
        data.ahLst = vec![1, 48, 50, 51, 52, 3];
        // Java: data.cxnLst = new int[0];
        data.cxnLst = vec![];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{10, 13, 0, 26, 26, 1, 52, 26, 1, 52, 1, 1, 0, 1}};
        data.pathLst = vec![vec![10, 13, 0, 26, 26, 1, 52, 26, 1, 52, 1, 1, 0, 1]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_bentConnector4()
    fn get_bentConnector4() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 50000, 100000, 2, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![50000, 50000, 100000, 2, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 0, 48, 50, 196610, 54, 0, 51, 196609, 1, 49, 50, 196610, 26, 62, 51};
        data.gdLst = vec![196609, 0, 48, 50, 196610, 54, 0, 51, 196609, 1, 49, 50, 196610, 26, 62, 51];
        // Java: data.ahLst = new int[]{1, 48, 52, 53, 54, 66, 2, 49, 52, 53, 58, 62};
        data.ahLst = vec![1, 48, 52, 53, 54, 66, 2, 49, 52, 53, 58, 62];
        // Java: data.cxnLst = new int[0];
        data.cxnLst = vec![];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{10, 13, 0, 26, 26, 1, 54, 26, 1, 54, 62, 1, 0, 62, 1, 0, 1}};
        data.pathLst = vec![vec![10, 13, 0, 26, 26, 1, 54, 26, 1, 54, 62, 1, 0, 62, 1, 0, 1]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_bentConnector5()
    fn get_bentConnector5() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 50000, 50000, 100000, 2, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![50000, 50000, 50000, 100000, 2, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 0, 48, 51, 196609, 0, 50, 51, 196610, 55, 59, 52, 196609, 1, 49, 51, 196610, 26, 67, 52, 196610, 1, 67, 52};
        data.gdLst = vec![196609, 0, 48, 51, 196609, 0, 50, 51, 196610, 55, 59, 52, 196609, 1, 49, 51, 196610, 26, 67, 52, 196610, 1, 67, 52];
        // Java: data.ahLst = new int[]{1, 48, 53, 54, 55, 71, 2, 49, 53, 54, 63, 67, 1, 50, 53, 54, 59, 75};
        data.ahLst = vec![1, 48, 53, 54, 55, 71, 2, 49, 53, 54, 63, 67, 1, 50, 53, 54, 59, 75];
        // Java: data.cxnLst = new int[0];
        data.cxnLst = vec![];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{10, 13, 0, 26, 26, 1, 55, 26, 1, 55, 67, 1, 59, 67, 1, 59, 1, 1, 0, 1}};
        data.pathLst = vec![vec![10, 13, 0, 26, 26, 1, 55, 26, 1, 55, 67, 1, 59, 67, 1, 59, 1, 1, 0, 1]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_bentUpArrow()
    fn get_bentUpArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 25000, 25000, 0, 50000, 100000, 200000, 1, 2};
        data.avcLst = vec![25000, 25000, 25000, 0, 50000, 100000, 200000, 1, 2];
        // Java: data.gdLst = new int[]{196625, 51, 48, 52, 196625, 51, 49, 52, 196625, 51, 50, 52, 196609, 28, 65, 53, 196609, 28, 61, 52, 196608, 0, 51, 73, 196609, 28, 61, 53, 196608, 0, 51, 81, 196609, 28, 57, 54, 196608, 85, 51, 89, 196608, 85, 89, 51, 196609, 28, 57, 53, 196608, 1, 51, 101, 196609, 97, 55, 56, 196610, 105, 1, 56, 196610, 69, 1, 56};
        data.gdLst = vec![196625, 51, 48, 52, 196625, 51, 49, 52, 196625, 51, 50, 52, 196609, 28, 65, 53, 196609, 28, 61, 52, 196608, 0, 51, 73, 196609, 28, 61, 53, 196608, 0, 51, 81, 196609, 28, 57, 54, 196608, 85, 51, 89, 196608, 85, 89, 51, 196609, 28, 57, 53, 196608, 1, 51, 101, 196609, 97, 55, 56, 196610, 105, 1, 56, 196610, 69, 1, 56];
        // Java: data.ahLst = new int[]{2, 48, 51, 52, 26, 105, 1, 49, 51, 52, 77, 26, 2, 50, 51, 52, 93, 69};
        data.ahLst = vec![2, 48, 51, 52, 26, 105, 1, 49, 51, 52, 77, 26, 2, 50, 51, 52, 93, 69];
        // Java: data.cxnLst = new int[]{15, 85, 26, 19, 77, 69, 19, 26, 113, 20, 109, 1, 51, 97, 117, 51, 0, 69};
        data.cxnLst = vec![15, 85, 26, 19, 77, 69, 19, 26, 113, 20, 109, 1, 51, 97, 117, 51, 0, 69];
        // Java: data.rect = new int[]{26, 105, 97, 1};
        data.rect = vec![26, 105, 97, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 105, 1, 93, 105, 1, 93, 69, 1, 77, 69, 1, 85, 26, 1, 0, 69, 1, 97, 69, 1, 97, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 105, 1, 93, 105, 1, 93, 69, 1, 77, 69, 1, 85, 26, 1, 0, 69, 1, 97, 69, 1, 97, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_bevel()
    fn get_bevel() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{12500, 0, 50000, 100000};
        data.avcLst = vec![12500, 0, 50000, 100000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 52, 51, 196608, 0, 49, 56, 196608, 1, 49, 56};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 52, 51, 196608, 0, 49, 56, 196608, 1, 49, 56];
        // Java: data.ahLst = new int[]{1, 48, 49, 50, 56, 26};
        data.ahLst = vec![1, 48, 49, 50, 56, 26];
        // Java: data.cxnLst = new int[]{49, 0, 3, 49, 60, 3, 20, 2, 1, 20, 2, 64, 19, 26, 3, 19, 56, 3, 15, 2, 26, 15, 2, 56};
        data.cxnLst = vec![49, 0, 3, 49, 60, 3, 20, 2, 1, 20, 2, 64, 19, 26, 3, 19, 56, 3, 15, 2, 26, 15, 2, 56];
        // Java: data.rect = new int[]{56, 56, 60, 64};
        data.rect = vec![56, 56, 60, 64];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 56, 56, 1, 60, 56, 1, 60, 64, 1, 56, 64, 2}, {9, 12, 0, 26, 26, 1, 0, 26, 1, 60, 56, 1, 56, 56, 2}, {7, 12, 0, 26, 1, 1, 56, 64, 1, 60, 64, 1, 0, 1, 2}, {8, 12, 0, 26, 26, 1, 56, 56, 1, 56, 64, 1, 26, 1, 2}, {6, 12, 0, 0, 26, 1, 0, 1, 1, 60, 64, 1, 60, 56, 2}, {10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 56, 56, 1, 60, 56, 1, 60, 64, 1, 56, 64, 2, 0, 26, 26, 1, 56, 56, 0, 26, 1, 1, 56, 64, 0, 0, 26, 1, 60, 56, 0, 0, 1, 1, 60, 64}};
        data.pathLst = vec![vec![11, 12, 0, 56, 56, 1, 60, 56, 1, 60, 64, 1, 56, 64, 2], vec![9, 12, 0, 26, 26, 1, 0, 26, 1, 60, 56, 1, 56, 56, 2], vec![7, 12, 0, 26, 1, 1, 56, 64, 1, 60, 64, 1, 0, 1, 2], vec![8, 12, 0, 26, 26, 1, 56, 56, 1, 56, 64, 1, 26, 1, 2], vec![6, 12, 0, 0, 26, 1, 0, 1, 1, 60, 64, 1, 60, 56, 2], vec![10, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 56, 56, 1, 60, 56, 1, 60, 64, 1, 56, 64, 2, 0, 26, 26, 1, 56, 56, 0, 26, 1, 1, 56, 64, 0, 0, 26, 1, 60, 56, 0, 0, 1, 1, 60, 64]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_blockArc()
    fn get_blockArc() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{10800000, 0, 25000, 0, 21599999, 50000, 21600000, 100000, 27000000, 32400000, 37800000, 2};
        data.avcLst = vec![10800000, 0, 25000, 0, 21599999, 50000, 21600000, 100000, 27000000, 32400000, 37800000, 2];
        // Java: data.gdLst = new int[]{196625, 51, 48, 52, 196625, 51, 49, 52, 196625, 51, 50, 53, 196608, 64, 51, 60, 196608, 72, 54, 51, 196614, 72, 72, 76, 196608, 51, 51, 80, 131092, 2, 60, 131093, 3, 60, 131092, 2, 64, 131093, 3, 64, 196619, 2, 91, 88, 196620, 3, 91, 88, 196619, 2, 97, 94, 196620, 3, 97, 94, 196608, 2, 100, 51, 196608, 3, 104, 51, 196608, 2, 108, 51, 196608, 3, 112, 51, 196609, 28, 68, 55, 196608, 2, 51, 132, 196608, 3, 51, 132, 131092, 136, 64, 131093, 140, 64, 131092, 136, 60, 131093, 140, 60, 196619, 136, 147, 144, 196620, 140, 147, 144, 196619, 136, 153, 150, 196620, 140, 153, 150, 196608, 2, 156, 51, 196608, 3, 160, 51, 196608, 2, 164, 51, 196608, 3, 168, 51, 196608, 54, 51, 60, 196608, 80, 51, 188, 131077, 116, 172, 131077, 124, 180, 131077, 196, 199, 196614, 192, 0, 202, 196608, 20, 51, 60, 196608, 56, 51, 60, 196614, 209, 209, 213, 196608, 80, 51, 217, 131077, 120, 176, 131077, 128, 184, 131077, 225, 228, 196614, 221, 1, 231, 196608, 19, 51, 60, 196608, 57, 51, 60, 196614, 238, 238, 242, 196608, 80, 51, 246, 131076, 116, 172, 131076, 124, 180, 131076, 254, 257, 196614, 250, 26, 260, 196608, 15, 51, 60, 196608, 58, 51, 60, 196614, 267, 267, 271, 196608, 80, 51, 275, 131076, 120, 176, 131076, 128, 184, 131076, 283, 286, 196614, 279, 26, 289, 196610, 116, 180, 59, 196610, 120, 184, 59, 196610, 124, 172, 59, 196610, 128, 176, 59, 196608, 60, 51, 20, 196608, 64, 20, 51, 196610, 312, 316, 59};
        data.gdLst = vec![196625, 51, 48, 52, 196625, 51, 49, 52, 196625, 51, 50, 53, 196608, 64, 51, 60, 196608, 72, 54, 51, 196614, 72, 72, 76, 196608, 51, 51, 80, 131092, 2, 60, 131093, 3, 60, 131092, 2, 64, 131093, 3, 64, 196619, 2, 91, 88, 196620, 3, 91, 88, 196619, 2, 97, 94, 196620, 3, 97, 94, 196608, 2, 100, 51, 196608, 3, 104, 51, 196608, 2, 108, 51, 196608, 3, 112, 51, 196609, 28, 68, 55, 196608, 2, 51, 132, 196608, 3, 51, 132, 131092, 136, 64, 131093, 140, 64, 131092, 136, 60, 131093, 140, 60, 196619, 136, 147, 144, 196620, 140, 147, 144, 196619, 136, 153, 150, 196620, 140, 153, 150, 196608, 2, 156, 51, 196608, 3, 160, 51, 196608, 2, 164, 51, 196608, 3, 168, 51, 196608, 54, 51, 60, 196608, 80, 51, 188, 131077, 116, 172, 131077, 124, 180, 131077, 196, 199, 196614, 192, 0, 202, 196608, 20, 51, 60, 196608, 56, 51, 60, 196614, 209, 209, 213, 196608, 80, 51, 217, 131077, 120, 176, 131077, 128, 184, 131077, 225, 228, 196614, 221, 1, 231, 196608, 19, 51, 60, 196608, 57, 51, 60, 196614, 238, 238, 242, 196608, 80, 51, 246, 131076, 116, 172, 131076, 124, 180, 131076, 254, 257, 196614, 250, 26, 260, 196608, 15, 51, 60, 196608, 58, 51, 60, 196614, 267, 267, 271, 196608, 80, 51, 275, 131076, 120, 176, 131076, 128, 184, 131076, 283, 286, 196614, 279, 26, 289, 196610, 116, 180, 59, 196610, 120, 184, 59, 196610, 124, 172, 59, 196610, 128, 176, 59, 196608, 60, 51, 20, 196608, 64, 20, 51, 196610, 312, 316, 59];
        // Java: data.ahLst = new int[]{4, 48, 51, 52, 116, 120, 6, 49, 51, 52, 50, 51, 53, 172, 176};
        data.ahLst = vec![4, 48, 51, 52, 116, 120, 6, 49, 51, 52, 50, 51, 53, 172, 176];
        // Java: data.cxnLst = new int[]{312, 296, 300, 316, 304, 308, 320, 2, 3};
        data.cxnLst = vec![312, 296, 300, 316, 304, 308, 320, 2, 3];
        // Java: data.rect = new int[]{263, 292, 205, 234};
        data.rect = vec![263, 292, 205, 234];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 116, 120, 3, 2, 3, 60, 80, 1, 172, 176, 3, 136, 140, 64, 84, 2}};
        data.pathLst = vec![vec![11, 13, 0, 116, 120, 3, 2, 3, 60, 80, 1, 172, 176, 3, 136, 140, 64, 84, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_borderCallout1()
    fn get_borderCallout1() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{18750, -8333, 112500, -38333, 100000, 0, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![18750, -8333, 112500, -38333, 100000, 0, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 1, 48, 52, 196609, 0, 49, 52, 196609, 1, 50, 52, 196609, 0, 51, 52};
        data.gdLst = vec![196609, 1, 48, 52, 196609, 0, 49, 52, 196609, 1, 50, 52, 196609, 0, 51, 52];
        // Java: data.ahLst = new int[]{3, 49, 54, 55, 48, 54, 55, 60, 56, 3, 51, 54, 55, 50, 54, 55, 68, 64};
        data.ahLst = vec![3, 49, 54, 55, 48, 54, 55, 60, 56, 3, 51, 54, 55, 50, 54, 55, 68, 64];
        // Java: data.cxnLst = new int[]{53, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![53, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}, {10, 13, 0, 68, 64, 1, 60, 56}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2], vec![10, 13, 0, 68, 64, 1, 60, 56]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_borderCallout2()
    fn get_borderCallout2() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{18750, -8333, 18750, -16667, 112500, -46667, 100000, 0, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![18750, -8333, 18750, -16667, 112500, -46667, 100000, 0, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 1, 48, 54, 196609, 0, 49, 54, 196609, 1, 50, 54, 196609, 0, 51, 54, 196609, 1, 52, 54, 196609, 0, 53, 54};
        data.gdLst = vec![196609, 1, 48, 54, 196609, 0, 49, 54, 196609, 1, 50, 54, 196609, 0, 51, 54, 196609, 1, 52, 54, 196609, 0, 53, 54];
        // Java: data.ahLst = new int[]{3, 49, 56, 57, 48, 56, 57, 62, 58, 3, 51, 56, 57, 50, 56, 57, 70, 66, 3, 53, 56, 57, 52, 56, 57, 78, 74};
        data.ahLst = vec![3, 49, 56, 57, 48, 56, 57, 62, 58, 3, 51, 56, 57, 50, 56, 57, 70, 66, 3, 53, 56, 57, 52, 56, 57, 78, 74];
        // Java: data.cxnLst = new int[]{55, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![55, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}, {10, 13, 0, 78, 74, 1, 70, 66, 1, 62, 58}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2], vec![10, 13, 0, 78, 74, 1, 70, 66, 1, 62, 58]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_borderCallout3()
    fn get_borderCallout3() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{18750, -8333, 18750, -16667, 100000, -16667, 112963, -8333, 100000, 0, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![18750, -8333, 18750, -16667, 100000, -16667, 112963, -8333, 100000, 0, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 1, 48, 56, 196609, 0, 49, 56, 196609, 1, 50, 56, 196609, 0, 51, 56, 196609, 1, 52, 56, 196609, 0, 53, 56, 196609, 1, 54, 56, 196609, 0, 55, 56};
        data.gdLst = vec![196609, 1, 48, 56, 196609, 0, 49, 56, 196609, 1, 50, 56, 196609, 0, 51, 56, 196609, 1, 52, 56, 196609, 0, 53, 56, 196609, 1, 54, 56, 196609, 0, 55, 56];
        // Java: data.ahLst = new int[]{3, 49, 58, 59, 48, 58, 59, 64, 60, 3, 51, 58, 59, 50, 58, 59, 72, 68, 3, 53, 58, 59, 52, 58, 59, 80, 76, 3, 55, 58, 59, 54, 58, 59, 88, 84};
        data.ahLst = vec![3, 49, 58, 59, 48, 58, 59, 64, 60, 3, 51, 58, 59, 50, 58, 59, 72, 68, 3, 53, 58, 59, 52, 58, 59, 80, 76, 3, 55, 58, 59, 54, 58, 59, 88, 84];
        // Java: data.cxnLst = new int[]{57, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![57, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}, {10, 13, 0, 88, 84, 1, 80, 76, 1, 72, 68, 1, 64, 60}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2], vec![10, 13, 0, 88, 84, 1, 80, 76, 1, 72, 68, 1, 64, 60]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_bracePair()
    fn get_bracePair() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{8333, 0, 25000, 100000, 50000, 29289, -5400000};
        data.avcLst = vec![8333, 0, 25000, 100000, 50000, 29289, -5400000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 55, 51, 196609, 28, 55, 52, 196608, 0, 49, 63, 196608, 0, 49, 59, 196608, 3, 49, 59, 196608, 3, 59, 49, 196608, 1, 49, 59, 196609, 59, 53, 51, 196608, 59, 87, 49, 196608, 0, 49, 91, 196608, 1, 49, 87};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 55, 51, 196609, 28, 55, 52, 196608, 0, 49, 63, 196608, 0, 49, 59, 196608, 3, 49, 59, 196608, 3, 59, 49, 196608, 1, 49, 59, 196609, 59, 53, 51, 196608, 59, 87, 49, 196608, 0, 49, 91, 196608, 1, 49, 87];
        // Java: data.ahLst = new int[]{2, 48, 49, 50, 26, 59};
        data.ahLst = vec![2, 48, 49, 50, 26, 59];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3];
        // Java: data.rect = new int[]{91, 91, 95, 99};
        data.rect = vec![91, 91, 95, 99];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 63, 1, 3, 59, 59, 20, 20, 1, 59, 79, 3, 59, 59, 49, 54, 3, 59, 59, 20, 54, 1, 59, 59, 3, 59, 59, 19, 20, 1, 67, 26, 3, 59, 59, 15, 20, 1, 71, 75, 3, 59, 59, 19, 54, 3, 59, 59, 15, 54, 1, 71, 83, 3, 59, 59, 49, 20, 2}, {10, 13, 0, 63, 1, 3, 59, 59, 20, 20, 1, 59, 79, 3, 59, 59, 49, 54, 3, 59, 59, 20, 54, 1, 59, 59, 3, 59, 59, 19, 20, 0, 67, 26, 3, 59, 59, 15, 20, 1, 71, 75, 3, 59, 59, 19, 54, 3, 59, 59, 15, 54, 1, 71, 83, 3, 59, 59, 49, 20}};
        data.pathLst = vec![vec![11, 12, 0, 63, 1, 3, 59, 59, 20, 20, 1, 59, 79, 3, 59, 59, 49, 54, 3, 59, 59, 20, 54, 1, 59, 59, 3, 59, 59, 19, 20, 1, 67, 26, 3, 59, 59, 15, 20, 1, 71, 75, 3, 59, 59, 19, 54, 3, 59, 59, 15, 54, 1, 71, 83, 3, 59, 59, 49, 20, 2], vec![10, 13, 0, 63, 1, 3, 59, 59, 20, 20, 1, 59, 79, 3, 59, 59, 49, 54, 3, 59, 59, 20, 54, 1, 59, 59, 3, 59, 59, 19, 20, 0, 67, 26, 3, 59, 59, 15, 20, 1, 71, 75, 3, 59, 59, 19, 54, 3, 59, 59, 15, 54, 1, 71, 83, 3, 59, 59, 49, 20]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_bracketPair()
    fn get_bracketPair() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{16667, 0, 50000, 100000, 29289};
        data.avcLst = vec![16667, 0, 50000, 100000, 29289];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 53, 51, 196608, 0, 49, 57, 196608, 1, 49, 57, 196609, 57, 52, 51, 196608, 0, 49, 69, 196608, 1, 49, 69};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 53, 51, 196608, 0, 49, 57, 196608, 1, 49, 57, 196609, 57, 52, 51, 196608, 0, 49, 69, 196608, 1, 49, 69];
        // Java: data.ahLst = new int[]{2, 48, 49, 50, 26, 57};
        data.ahLst = vec![2, 48, 49, 50, 26, 57];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3];
        // Java: data.rect = new int[]{69, 69, 73, 77};
        data.rect = vec![69, 69, 73, 77];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 57, 3, 57, 57, 19, 20, 1, 61, 26, 3, 57, 57, 15, 20, 1, 0, 65, 3, 57, 57, 49, 20, 1, 57, 1, 3, 57, 57, 20, 20, 2}, {10, 13, 0, 57, 1, 3, 57, 57, 20, 20, 1, 26, 57, 3, 57, 57, 19, 20, 0, 61, 26, 3, 57, 57, 15, 20, 1, 0, 65, 3, 57, 57, 49, 20}};
        data.pathLst = vec![vec![11, 12, 0, 26, 57, 3, 57, 57, 19, 20, 1, 61, 26, 3, 57, 57, 15, 20, 1, 0, 65, 3, 57, 57, 49, 20, 1, 57, 1, 3, 57, 57, 20, 20, 2], vec![10, 13, 0, 57, 1, 3, 57, 57, 20, 20, 1, 26, 57, 3, 57, 57, 19, 20, 0, 61, 26, 3, 57, 57, 15, 20, 1, 0, 65, 3, 57, 57, 49, 20]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_callout1()
    fn get_callout1() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{18750, -8333, 112500, -38333, 100000, 0, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![18750, -8333, 112500, -38333, 100000, 0, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 1, 48, 52, 196609, 0, 49, 52, 196609, 1, 50, 52, 196609, 0, 51, 52};
        data.gdLst = vec![196609, 1, 48, 52, 196609, 0, 49, 52, 196609, 1, 50, 52, 196609, 0, 51, 52];
        // Java: data.ahLst = new int[]{3, 49, 54, 55, 48, 54, 55, 60, 56, 3, 51, 54, 55, 50, 54, 55, 68, 64};
        data.ahLst = vec![3, 49, 54, 55, 48, 54, 55, 60, 56, 3, 51, 54, 55, 50, 54, 55, 68, 64];
        // Java: data.cxnLst = new int[]{53, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![53, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}, {10, 13, 0, 68, 64, 1, 60, 56}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2], vec![10, 13, 0, 68, 64, 1, 60, 56]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_callout2()
    fn get_callout2() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{18750, -8333, 18750, -16667, 112500, -46667, 100000, 0, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![18750, -8333, 18750, -16667, 112500, -46667, 100000, 0, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 1, 48, 54, 196609, 0, 49, 54, 196609, 1, 50, 54, 196609, 0, 51, 54, 196609, 1, 52, 54, 196609, 0, 53, 54};
        data.gdLst = vec![196609, 1, 48, 54, 196609, 0, 49, 54, 196609, 1, 50, 54, 196609, 0, 51, 54, 196609, 1, 52, 54, 196609, 0, 53, 54];
        // Java: data.ahLst = new int[]{3, 49, 56, 57, 48, 56, 57, 62, 58, 3, 51, 56, 57, 50, 56, 57, 70, 66, 3, 53, 56, 57, 52, 56, 57, 78, 74};
        data.ahLst = vec![3, 49, 56, 57, 48, 56, 57, 62, 58, 3, 51, 56, 57, 50, 56, 57, 70, 66, 3, 53, 56, 57, 52, 56, 57, 78, 74];
        // Java: data.cxnLst = new int[]{55, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![55, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}, {10, 13, 0, 78, 74, 1, 70, 66, 1, 62, 58}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2], vec![10, 13, 0, 78, 74, 1, 70, 66, 1, 62, 58]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_callout3()
    fn get_callout3() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{18750, -8333, 18750, -16667, 100000, -16667, 112963, -8333, 100000, 0, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![18750, -8333, 18750, -16667, 100000, -16667, 112963, -8333, 100000, 0, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 1, 48, 56, 196609, 0, 49, 56, 196609, 1, 50, 56, 196609, 0, 51, 56, 196609, 1, 52, 56, 196609, 0, 53, 56, 196609, 1, 54, 56, 196609, 0, 55, 56};
        data.gdLst = vec![196609, 1, 48, 56, 196609, 0, 49, 56, 196609, 1, 50, 56, 196609, 0, 51, 56, 196609, 1, 52, 56, 196609, 0, 53, 56, 196609, 1, 54, 56, 196609, 0, 55, 56];
        // Java: data.ahLst = new int[]{3, 49, 58, 59, 48, 58, 59, 64, 60, 3, 51, 58, 59, 50, 58, 59, 72, 68, 3, 53, 58, 59, 52, 58, 59, 80, 76, 3, 55, 58, 59, 54, 58, 59, 88, 84};
        data.ahLst = vec![3, 49, 58, 59, 48, 58, 59, 64, 60, 3, 51, 58, 59, 50, 58, 59, 72, 68, 3, 53, 58, 59, 52, 58, 59, 80, 76, 3, 55, 58, 59, 54, 58, 59, 88, 84];
        // Java: data.cxnLst = new int[]{57, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![57, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}, {10, 13, 0, 88, 84, 1, 80, 76, 1, 72, 68, 1, 64, 60}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2], vec![10, 13, 0, 88, 84, 1, 80, 76, 1, 72, 68, 1, 64, 60]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_can()
    fn get_can() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 50000, 0, 200000, -10800000};
        data.avcLst = vec![25000, 50000, 0, 200000, -10800000];
        // Java: data.gdLst = new int[]{196609, 49, 1, 28, 196625, 50, 48, 53, 196609, 28, 57, 51, 196608, 61, 61, 50, 196608, 1, 50, 61};
        data.gdLst = vec![196609, 49, 1, 28, 196625, 50, 48, 53, 196609, 28, 57, 51, 196608, 61, 61, 50, 196608, 1, 50, 61];
        // Java: data.ahLst = new int[]{2, 48, 50, 53, 2, 65};
        data.ahLst = vec![2, 48, 50, 53, 2, 65];
        // Java: data.cxnLst = new int[]{15, 2, 65, 15, 2, 26, 19, 26, 3, 20, 2, 1, 50, 0, 3};
        data.cxnLst = vec![15, 2, 65, 15, 2, 26, 19, 26, 3, 20, 2, 1, 50, 0, 3];
        // Java: data.rect = new int[]{26, 65, 0, 69};
        data.rect = vec![26, 65, 0, 69];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 61, 3, 2, 61, 19, 52, 1, 0, 69, 3, 2, 61, 50, 19, 2}, {8, 12, 0, 26, 61, 3, 2, 61, 19, 19, 3, 2, 61, 50, 19, 2}, {10, 13, 0, 0, 61, 3, 2, 61, 50, 19, 3, 2, 61, 19, 19, 1, 0, 69, 3, 2, 61, 50, 19, 1, 26, 61}};
        data.pathLst = vec![vec![11, 12, 0, 26, 61, 3, 2, 61, 19, 52, 1, 0, 69, 3, 2, 61, 50, 19, 2], vec![8, 12, 0, 26, 61, 3, 2, 61, 19, 19, 3, 2, 61, 50, 19, 2], vec![10, 13, 0, 0, 61, 3, 2, 61, 50, 19, 3, 2, 61, 19, 19, 1, 0, 69, 3, 2, 61, 50, 19, 1, 26, 61]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_chartPlus()
    fn get_chartPlus() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{5, 0, 10};
        data.avcLst = vec![5, 0, 10];
        // Java: data.gdLst = new int[0];
        data.gdLst = vec![];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[0];
        data.cxnLst = vec![];
        // Java: data.rect = new int[0];
        data.rect = vec![];
        // Java: data.pathLst = new int[][]{{10, 13, 0, 48, 49, 1, 48, 50, 0, 49, 48, 1, 50, 48}, {11, 12, 0, 49, 49, 1, 49, 50, 1, 50, 50, 1, 50, 49, 2}};
        data.pathLst = vec![vec![10, 13, 0, 48, 49, 1, 48, 50, 0, 49, 48, 1, 50, 48], vec![11, 12, 0, 49, 49, 1, 49, 50, 1, 50, 50, 1, 50, 49, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_chartStar()
    fn get_chartStar() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{0, 10, 5};
        data.avcLst = vec![0, 10, 5];
        // Java: data.gdLst = new int[0];
        data.gdLst = vec![];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[0];
        data.cxnLst = vec![];
        // Java: data.rect = new int[0];
        data.rect = vec![];
        // Java: data.pathLst = new int[][]{{10, 13, 0, 48, 48, 1, 49, 49, 0, 48, 49, 1, 49, 48, 0, 50, 48, 1, 50, 49}, {11, 12, 0, 48, 48, 1, 48, 49, 1, 49, 49, 1, 49, 48, 2}};
        data.pathLst = vec![vec![10, 13, 0, 48, 48, 1, 49, 49, 0, 48, 49, 1, 49, 48, 0, 50, 48, 1, 50, 49], vec![11, 12, 0, 48, 48, 1, 48, 49, 1, 49, 49, 1, 49, 48, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_chartX()
    fn get_chartX() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{0, 10};
        data.avcLst = vec![0, 10];
        // Java: data.gdLst = new int[0];
        data.gdLst = vec![];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[0];
        data.cxnLst = vec![];
        // Java: data.rect = new int[0];
        data.rect = vec![];
        // Java: data.pathLst = new int[][]{{10, 13, 0, 48, 48, 1, 49, 49, 0, 48, 49, 1, 49, 48}, {11, 12, 0, 48, 48, 1, 48, 49, 1, 49, 49, 1, 49, 48, 2}};
        data.pathLst = vec![vec![10, 13, 0, 48, 48, 1, 49, 49, 0, 48, 49, 1, 49, 48], vec![11, 12, 0, 48, 48, 1, 48, 49, 1, 49, 49, 1, 49, 48, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_chevron()
    fn get_chevron() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 100000, 0, 1, 2};
        data.avcLst = vec![50000, 100000, 0, 1, 2];
        // Java: data.gdLst = new int[]{196609, 49, 0, 28, 196625, 50, 48, 53, 196609, 28, 57, 49, 196608, 0, 50, 61, 196609, 65, 51, 52, 196608, 65, 50, 61, 196614, 73, 61, 26, 196614, 73, 65, 0};
        data.gdLst = vec![196609, 49, 0, 28, 196625, 50, 48, 53, 196609, 28, 57, 49, 196608, 0, 50, 61, 196609, 65, 51, 52, 196608, 65, 50, 61, 196614, 73, 61, 26, 196614, 73, 65, 0];
        // Java: data.ahLst = new int[]{1, 48, 50, 53, 65, 26};
        data.ahLst = vec![1, 48, 50, 53, 65, 26];
        // Java: data.cxnLst = new int[]{15, 69, 26, 19, 61, 3, 20, 69, 1, 50, 0, 3};
        data.cxnLst = vec![15, 69, 26, 19, 61, 3, 20, 69, 1, 50, 0, 3];
        // Java: data.rect = new int[]{77, 26, 81, 1};
        data.rect = vec![77, 26, 81, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 65, 26, 1, 0, 3, 1, 65, 1, 1, 26, 1, 1, 61, 3, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 65, 26, 1, 0, 3, 1, 65, 1, 1, 26, 1, 1, 61, 3, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_chord()
    fn get_chord() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{2700000, 16200000, 0, 21599999, 21600000, 2, 1, 2700000};
        data.avcLst = vec![2700000, 16200000, 0, 21599999, 21600000, 2, 1, 2700000];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196625, 50, 49, 51, 196608, 60, 50, 56, 196608, 64, 52, 50, 196614, 64, 64, 68, 131092, 2, 56, 131093, 3, 56, 196619, 2, 79, 76, 196620, 3, 79, 76, 131092, 2, 60, 131093, 3, 60, 196619, 2, 93, 90, 196620, 3, 93, 90, 196608, 2, 82, 50, 196608, 3, 86, 50, 196608, 2, 96, 50, 196608, 3, 100, 50, 196610, 104, 112, 53, 196610, 108, 116, 53, 196609, 72, 54, 53, 196608, 56, 128, 19, 131093, 2, 55, 131092, 3, 55, 196608, 2, 50, 136, 196608, 2, 136, 50, 196608, 3, 50, 139, 196608, 3, 139, 50};
        data.gdLst = vec![196625, 50, 48, 51, 196625, 50, 49, 51, 196608, 60, 50, 56, 196608, 64, 52, 50, 196614, 64, 64, 68, 131092, 2, 56, 131093, 3, 56, 196619, 2, 79, 76, 196620, 3, 79, 76, 131092, 2, 60, 131093, 3, 60, 196619, 2, 93, 90, 196620, 3, 93, 90, 196608, 2, 82, 50, 196608, 3, 86, 50, 196608, 2, 96, 50, 196608, 3, 100, 50, 196610, 104, 112, 53, 196610, 108, 116, 53, 196609, 72, 54, 53, 196608, 56, 128, 19, 131093, 2, 55, 131092, 3, 55, 196608, 2, 50, 136, 196608, 2, 136, 50, 196608, 3, 50, 139, 196608, 3, 139, 50];
        // Java: data.ahLst = new int[]{4, 48, 50, 51, 104, 108, 4, 49, 50, 51, 112, 116};
        data.ahLst = vec![4, 48, 50, 51, 104, 108, 4, 49, 50, 51, 112, 116];
        // Java: data.cxnLst = new int[]{56, 104, 108, 60, 112, 116, 132, 120, 124};
        data.cxnLst = vec![56, 104, 108, 60, 112, 116, 132, 120, 124];
        // Java: data.rect = new int[]{142, 150, 146, 154};
        data.rect = vec![142, 150, 146, 154];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 104, 108, 3, 2, 3, 56, 72, 2}};
        data.pathLst = vec![vec![11, 13, 0, 104, 108, 3, 2, 3, 56, 72, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_circularArrow()
    fn get_circularArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{12500, 1142319, 20457681, 10800000, 12500, 0, 25000, 2, 1, 21599999, 100000, 21600000, -1, 2700000};
        data.avcLst = vec![12500, 1142319, 20457681, 10800000, 12500, 0, 25000, 2, 1, 21599999, 100000, 21600000, -1, 2700000];
        // Java: data.gdLst = new int[]{196625, 56, 52, 54, 196609, 62, 55, 56, 196625, 53, 48, 66, 196625, 56, 50, 57, 196625, 53, 51, 57, 196609, 28, 70, 58, 196609, 28, 62, 58, 196609, 82, 56, 55, 196608, 2, 90, 86, 196608, 3, 90, 86, 196608, 94, 53, 82, 196608, 98, 53, 82, 196608, 102, 90, 53, 196608, 106, 90, 53, 131092, 110, 74, 131093, 114, 74, 196619, 110, 121, 118, 196620, 114, 121, 118, 196608, 2, 124, 53, 196608, 3, 128, 53, 131076, 102, 106, 196609, 124, 124, 56, 196609, 128, 128, 56, 196609, 140, 140, 56, 196608, 143, 53, 151, 196608, 147, 53, 151, 196609, 155, 159, 143, 196609, 163, 56, 147, 196608, 56, 53, 167, 65549, 171, 196609, 155, 56, 124, 196609, 177, 56, 128, 196610, 56, 175, 181, 131091, 56, 185, 196608, 189, 59, 53, 196614, 189, 189, 192, 196608, 196, 53, 74, 196608, 200, 59, 53, 196614, 200, 200, 204, 196608, 208, 53, 19, 196608, 208, 53, 59, 196614, 212, 216, 208, 65539, 220, 196625, 53, 49, 224, 196608, 74, 226, 53, 131092, 110, 230, 131093, 114, 230, 196619, 110, 237, 234, 196620, 114, 237, 234, 196608, 2, 240, 53, 196608, 3, 244, 53, 131092, 94, 78, 131093, 98, 78, 196619, 94, 259, 256, 196620, 98, 259, 256, 196608, 2, 262, 53, 196608, 3, 266, 53, 131093, 86, 230, 131092, 86, 230, 196608, 132, 278, 53, 196608, 136, 281, 53, 131093, 86, 230, 131092, 86, 230, 196608, 132, 53, 292, 196608, 136, 53, 295, 196608, 298, 53, 2, 196608, 302, 53, 3, 196608, 284, 53, 2, 196608, 288, 53, 3, 131076, 94, 98, 196609, 306, 322, 94, 196609, 310, 322, 98, 196609, 314, 322, 94, 196609, 318, 322, 98, 196608, 333, 53, 325, 196608, 337, 53, 329, 196615, 341, 345, 53, 196609, 325, 337, 56, 196609, 333, 329, 56, 196608, 353, 53, 357, 196609, 322, 322, 56, 196609, 349, 349, 56, 196609, 365, 369, 56, 196609, 361, 361, 56, 196608, 373, 53, 377, 131077, 381, 53, 65549, 385, 196609, 345, 60, 56, 196614, 390, 60, 56, 196609, 394, 341, 56, 196609, 398, 388, 56, 196609, 361, 345, 56, 196610, 406, 402, 369, 196608, 406, 53, 402, 196609, 414, 56, 369, 65539, 345, 196609, 422, 388, 56, 196609, 361, 341, 60, 196610, 428, 424, 369, 196608, 428, 53, 424, 196609, 436, 56, 369, 196608, 333, 53, 410, 196608, 333, 53, 418, 196608, 337, 53, 432, 196608, 337, 53, 440, 196615, 444, 452, 53, 196615, 448, 456, 53, 196608, 464, 53, 460, 196614, 468, 410, 418, 196614, 468, 432, 440, 196609, 472, 94, 322, 196609, 476, 98, 322, 196608, 2, 480, 53, 196608, 3, 484, 53, 196609, 306, 140, 102, 196609, 310, 140, 106, 196609, 314, 140, 102, 196609, 318, 140, 106, 196608, 504, 53, 496, 196608, 508, 53, 500, 196615, 512, 516, 53, 196609, 496, 508, 56, 196609, 504, 500, 56, 196608, 524, 53, 528, 196609, 140, 140, 56, 196609, 520, 520, 56, 196609, 536, 540, 56, 196609, 532, 532, 56, 196608, 544, 53, 548, 131077, 552, 53, 65549, 556, 196609, 394, 512, 56, 196609, 561, 559, 56, 196609, 532, 516, 56, 196610, 569, 565, 540, 196608, 569, 53, 565, 196609, 577, 56, 540, 65539, 516, 196609, 585, 559, 56, 196609, 532, 512, 60, 196610, 591, 587, 540, 196608, 591, 53, 587, 196609, 599, 56, 540, 196608, 496, 53, 573, 196608, 496, 53, 581, 196608, 500, 53, 595, 196608, 500, 53, 603, 196615, 607, 615, 53, 196615, 611, 619, 53, 196608, 627, 53, 623, 196614, 631, 573, 581, 196614, 631, 595, 603, 196609, 635, 102, 140, 196609, 639, 106, 140, 196608, 2, 643, 53, 196608, 3, 647, 53, 131091, 643, 647, 196608, 659, 59, 53, 196614, 659, 659, 662, 196608, 78, 53, 666, 196608, 670, 53, 59, 196614, 670, 674, 670, 196608, 488, 53, 651, 196608, 492, 53, 655, 196615, 682, 686, 53, 196609, 690, 56, 55, 196608, 694, 53, 86, 196614, 698, 488, 284, 196614, 698, 492, 288, 196614, 698, 651, 298, 196614, 698, 655, 302, 131091, 480, 484, 196608, 718, 59, 53, 196614, 718, 718, 721, 196608, 725, 53, 78, 196608, 729, 59, 53, 196614, 729, 729, 733, 131092, 110, 78, 131093, 114, 78, 196619, 110, 744, 741, 196620, 114, 744, 741, 196608, 2, 747, 53, 196608, 3, 751, 53, 196608, 78, 53, 20, 196608, 230, 20, 53, 196608, 230, 19, 53, 131093, 94, 61, 131092, 98, 61, 196608, 2, 53, 775, 196608, 2, 775, 53, 196608, 3, 53, 778, 196608, 3, 778, 53};
        data.gdLst = vec![196625, 56, 52, 54, 196609, 62, 55, 56, 196625, 53, 48, 66, 196625, 56, 50, 57, 196625, 53, 51, 57, 196609, 28, 70, 58, 196609, 28, 62, 58, 196609, 82, 56, 55, 196608, 2, 90, 86, 196608, 3, 90, 86, 196608, 94, 53, 82, 196608, 98, 53, 82, 196608, 102, 90, 53, 196608, 106, 90, 53, 131092, 110, 74, 131093, 114, 74, 196619, 110, 121, 118, 196620, 114, 121, 118, 196608, 2, 124, 53, 196608, 3, 128, 53, 131076, 102, 106, 196609, 124, 124, 56, 196609, 128, 128, 56, 196609, 140, 140, 56, 196608, 143, 53, 151, 196608, 147, 53, 151, 196609, 155, 159, 143, 196609, 163, 56, 147, 196608, 56, 53, 167, 65549, 171, 196609, 155, 56, 124, 196609, 177, 56, 128, 196610, 56, 175, 181, 131091, 56, 185, 196608, 189, 59, 53, 196614, 189, 189, 192, 196608, 196, 53, 74, 196608, 200, 59, 53, 196614, 200, 200, 204, 196608, 208, 53, 19, 196608, 208, 53, 59, 196614, 212, 216, 208, 65539, 220, 196625, 53, 49, 224, 196608, 74, 226, 53, 131092, 110, 230, 131093, 114, 230, 196619, 110, 237, 234, 196620, 114, 237, 234, 196608, 2, 240, 53, 196608, 3, 244, 53, 131092, 94, 78, 131093, 98, 78, 196619, 94, 259, 256, 196620, 98, 259, 256, 196608, 2, 262, 53, 196608, 3, 266, 53, 131093, 86, 230, 131092, 86, 230, 196608, 132, 278, 53, 196608, 136, 281, 53, 131093, 86, 230, 131092, 86, 230, 196608, 132, 53, 292, 196608, 136, 53, 295, 196608, 298, 53, 2, 196608, 302, 53, 3, 196608, 284, 53, 2, 196608, 288, 53, 3, 131076, 94, 98, 196609, 306, 322, 94, 196609, 310, 322, 98, 196609, 314, 322, 94, 196609, 318, 322, 98, 196608, 333, 53, 325, 196608, 337, 53, 329, 196615, 341, 345, 53, 196609, 325, 337, 56, 196609, 333, 329, 56, 196608, 353, 53, 357, 196609, 322, 322, 56, 196609, 349, 349, 56, 196609, 365, 369, 56, 196609, 361, 361, 56, 196608, 373, 53, 377, 131077, 381, 53, 65549, 385, 196609, 345, 60, 56, 196614, 390, 60, 56, 196609, 394, 341, 56, 196609, 398, 388, 56, 196609, 361, 345, 56, 196610, 406, 402, 369, 196608, 406, 53, 402, 196609, 414, 56, 369, 65539, 345, 196609, 422, 388, 56, 196609, 361, 341, 60, 196610, 428, 424, 369, 196608, 428, 53, 424, 196609, 436, 56, 369, 196608, 333, 53, 410, 196608, 333, 53, 418, 196608, 337, 53, 432, 196608, 337, 53, 440, 196615, 444, 452, 53, 196615, 448, 456, 53, 196608, 464, 53, 460, 196614, 468, 410, 418, 196614, 468, 432, 440, 196609, 472, 94, 322, 196609, 476, 98, 322, 196608, 2, 480, 53, 196608, 3, 484, 53, 196609, 306, 140, 102, 196609, 310, 140, 106, 196609, 314, 140, 102, 196609, 318, 140, 106, 196608, 504, 53, 496, 196608, 508, 53, 500, 196615, 512, 516, 53, 196609, 496, 508, 56, 196609, 504, 500, 56, 196608, 524, 53, 528, 196609, 140, 140, 56, 196609, 520, 520, 56, 196609, 536, 540, 56, 196609, 532, 532, 56, 196608, 544, 53, 548, 131077, 552, 53, 65549, 556, 196609, 394, 512, 56, 196609, 561, 559, 56, 196609, 532, 516, 56, 196610, 569, 565, 540, 196608, 569, 53, 565, 196609, 577, 56, 540, 65539, 516, 196609, 585, 559, 56, 196609, 532, 512, 60, 196610, 591, 587, 540, 196608, 591, 53, 587, 196609, 599, 56, 540, 196608, 496, 53, 573, 196608, 496, 53, 581, 196608, 500, 53, 595, 196608, 500, 53, 603, 196615, 607, 615, 53, 196615, 611, 619, 53, 196608, 627, 53, 623, 196614, 631, 573, 581, 196614, 631, 595, 603, 196609, 635, 102, 140, 196609, 639, 106, 140, 196608, 2, 643, 53, 196608, 3, 647, 53, 131091, 643, 647, 196608, 659, 59, 53, 196614, 659, 659, 662, 196608, 78, 53, 666, 196608, 670, 53, 59, 196614, 670, 674, 670, 196608, 488, 53, 651, 196608, 492, 53, 655, 196615, 682, 686, 53, 196609, 690, 56, 55, 196608, 694, 53, 86, 196614, 698, 488, 284, 196614, 698, 492, 288, 196614, 698, 651, 298, 196614, 698, 655, 302, 131091, 480, 484, 196608, 718, 59, 53, 196614, 718, 718, 721, 196608, 725, 53, 78, 196608, 729, 59, 53, 196614, 729, 729, 733, 131092, 110, 78, 131093, 114, 78, 196619, 110, 744, 741, 196620, 114, 744, 741, 196608, 2, 747, 53, 196608, 3, 751, 53, 196608, 78, 53, 20, 196608, 230, 20, 53, 196608, 230, 19, 53, 131093, 94, 61, 131092, 98, 61, 196608, 2, 53, 775, 196608, 2, 775, 53, 196608, 3, 53, 778, 196608, 3, 778, 53];
        // Java: data.ahLst = new int[]{4, 49, 53, 224, 248, 252, 4, 51, 53, 57, 270, 274, 6, 50, 53, 57, 48, 53, 66, 488, 492, 5, 52, 56, 54, 298, 302};
        data.ahLst = vec![4, 49, 53, 224, 248, 252, 4, 51, 53, 57, 270, 274, 6, 50, 53, 57, 48, 53, 66, 488, 492, 5, 52, 56, 54, 298, 302];
        // Java: data.cxnLst = new int[]{763, 755, 759, 230, 702, 706, 767, 248, 252, 771, 710, 714};
        data.cxnLst = vec![763, 755, 759, 230, 702, 706, 767, 248, 252, 771, 710, 714];
        // Java: data.rect = new int[]{781, 789, 785, 793};
        data.rect = vec![781, 789, 785, 793];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 270, 274, 3, 94, 98, 78, 737, 1, 702, 706, 1, 248, 252, 1, 710, 714, 1, 651, 655, 3, 102, 106, 666, 678, 2}};
        data.pathLst = vec![vec![11, 13, 0, 270, 274, 3, 94, 98, 78, 737, 1, 702, 706, 1, 248, 252, 1, 710, 714, 1, 651, 655, 3, 102, 106, 666, 678, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_cloud()
    fn get_cloud() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{2977, 21600, 3262, 17087, 17337, 67, 21577, 21582, 1235, 3900, 43200, 14370, 6753, 9190, 5333, 7267, 4365, 5945, 4857, 6595, 7273, 6775, 9220, 5785, 7867, 6752, 9215, 7720, 10543, 4360, 5918, 4345, 4693, 26177, 6928, 34899, 16478, 39090, 28827, 34751, 34129, 22954, 41798, 15354, 38324, 5426, 29078, 3952, 22141, 4720, 14000, 5192, 4127, 15789, -11429249, 7426832, -8646143, 5396714, -8748475, 5983381, -7859164, 7034504, -4722533, 6541615, -2776035, 7816140, 37501, 6842000, 1347096, 6910353, 3974558, 4542661, -16496525, 8804134, -14809710, 9151131, 5204520, 1585770, 4416628, 686848, 8257449, 844866, 387196, 959901, -4217541, 4255042, 1819082, 1665090, -824660, 891534, -8950887, 1091722, -9809656, 1061181, -4002417, 739161, 9459261, 711490, 0};
        data.avcLst = vec![2977, 21600, 3262, 17087, 17337, 67, 21577, 21582, 1235, 3900, 43200, 14370, 6753, 9190, 5333, 7267, 4365, 5945, 4857, 6595, 7273, 6775, 9220, 5785, 7867, 6752, 9215, 7720, 10543, 4360, 5918, 4345, 4693, 26177, 6928, 34899, 16478, 39090, 28827, 34751, 34129, 22954, 41798, 15354, 38324, 5426, 29078, 3952, 22141, 4720, 14000, 5192, 4127, 15789, -11429249, 7426832, -8646143, 5396714, -8748475, 5983381, -7859164, 7034504, -4722533, 6541615, -2776035, 7816140, 37501, 6842000, 1347096, 6910353, 3974558, 4542661, -16496525, 8804134, -14809710, 9151131, 5204520, 1585770, 4416628, 686848, 8257449, 844866, 387196, 959901, -4217541, 4255042, 1819082, 1665090, -824660, 891534, -8950887, 1091722, -9809656, 1061181, -4002417, 739161, 9459261, 711490, 0];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 1, 50, 49, 196609, 0, 51, 49, 196609, 1, 52, 49, 196609, 0, 53, 49, 196609, 1, 54, 49, 196609, 0, 55, 49, 196609, 1, 56, 49, 196609, 0, 57, 49, 196609, 0, 57, 58, 196609, 1, 59, 58, 196609, 0, 60, 58, 196609, 1, 61, 58, 196609, 0, 62, 58, 196609, 1, 63, 58, 196609, 0, 64, 58, 196609, 1, 65, 58, 196609, 0, 66, 58, 196609, 1, 67, 58, 196609, 1, 68, 58, 196609, 0, 69, 58, 196609, 1, 70, 58, 196609, 0, 71, 58, 196609, 1, 72, 58, 196609, 0, 73, 58, 196609, 1, 74, 58, 196609, 0, 75, 58, 196609, 1, 76, 58, 196609, 0, 77, 58, 196609, 1, 78, 58, 196609, 0, 79, 58, 196609, 0, 80, 58, 196609, 1, 81, 58, 196609, 0, 82, 58, 196609, 1, 83, 58, 196609, 0, 84, 58, 196609, 1, 85, 58, 196609, 0, 86, 58, 196609, 1, 87, 58, 196609, 0, 88, 58, 196609, 1, 89, 58, 196609, 0, 90, 58, 196609, 1, 91, 58, 196609, 0, 92, 58, 196609, 1, 93, 58, 196609, 0, 94, 58, 196609, 1, 95, 58, 196609, 0, 96, 58, 196609, 1, 97, 58, 196609, 0, 98, 58, 196609, 1, 99, 58, 196609, 0, 100, 58, 196609, 1, 101, 58};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 1, 50, 49, 196609, 0, 51, 49, 196609, 1, 52, 49, 196609, 0, 53, 49, 196609, 1, 54, 49, 196609, 0, 55, 49, 196609, 1, 56, 49, 196609, 0, 57, 49, 196609, 0, 57, 58, 196609, 1, 59, 58, 196609, 0, 60, 58, 196609, 1, 61, 58, 196609, 0, 62, 58, 196609, 1, 63, 58, 196609, 0, 64, 58, 196609, 1, 65, 58, 196609, 0, 66, 58, 196609, 1, 67, 58, 196609, 1, 68, 58, 196609, 0, 69, 58, 196609, 1, 70, 58, 196609, 0, 71, 58, 196609, 1, 72, 58, 196609, 0, 73, 58, 196609, 1, 74, 58, 196609, 0, 75, 58, 196609, 1, 76, 58, 196609, 0, 77, 58, 196609, 1, 78, 58, 196609, 0, 79, 58, 196609, 0, 80, 58, 196609, 1, 81, 58, 196609, 0, 82, 58, 196609, 1, 83, 58, 196609, 0, 84, 58, 196609, 1, 85, 58, 196609, 0, 86, 58, 196609, 1, 87, 58, 196609, 0, 88, 58, 196609, 1, 89, 58, 196609, 0, 90, 58, 196609, 1, 91, 58, 196609, 0, 92, 58, 196609, 1, 93, 58, 196609, 0, 94, 58, 196609, 1, 95, 58, 196609, 0, 96, 58, 196609, 1, 97, 58, 196609, 0, 98, 58, 196609, 1, 99, 58, 196609, 0, 100, 58, 196609, 1, 101, 58];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{146, 171, 3, 20, 2, 167, 19, 163, 3, 15, 2, 175};
        data.cxnLst = vec![146, 171, 3, 20, 2, 167, 19, 163, 3, 15, 2, 175];
        // Java: data.rect = new int[]{147, 151, 155, 159};
        data.rect = vec![147, 151, 155, 159];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 183, 187, 3, 191, 195, 102, 103, 3, 199, 203, 104, 105, 3, 207, 211, 106, 107, 3, 215, 219, 108, 109, 3, 199, 223, 110, 111, 3, 227, 231, 112, 113, 3, 235, 239, 114, 115, 3, 243, 247, 116, 117, 3, 251, 255, 118, 119, 3, 259, 263, 120, 121, 3, 267, 211, 122, 123, 2}, {10, 13, 0, 271, 275, 3, 267, 211, 124, 125, 0, 279, 283, 3, 259, 263, 126, 127, 0, 287, 291, 3, 243, 247, 128, 129, 0, 295, 299, 3, 243, 247, 130, 131, 0, 303, 307, 3, 235, 239, 132, 133, 0, 311, 315, 3, 199, 223, 134, 135, 0, 319, 323, 3, 215, 219, 136, 137, 0, 327, 331, 3, 215, 219, 138, 139, 0, 335, 339, 3, 207, 211, 140, 141, 0, 343, 347, 3, 191, 195, 142, 143, 0, 351, 355, 3, 191, 195, 144, 145}};
        data.pathLst = vec![vec![11, 13, 0, 183, 187, 3, 191, 195, 102, 103, 3, 199, 203, 104, 105, 3, 207, 211, 106, 107, 3, 215, 219, 108, 109, 3, 199, 223, 110, 111, 3, 227, 231, 112, 113, 3, 235, 239, 114, 115, 3, 243, 247, 116, 117, 3, 251, 255, 118, 119, 3, 259, 263, 120, 121, 3, 267, 211, 122, 123, 2], vec![10, 13, 0, 271, 275, 3, 267, 211, 124, 125, 0, 279, 283, 3, 259, 263, 126, 127, 0, 287, 291, 3, 243, 247, 128, 129, 0, 295, 299, 3, 243, 247, 130, 131, 0, 303, 307, 3, 235, 239, 132, 133, 0, 311, 315, 3, 199, 223, 134, 135, 0, 319, 323, 3, 215, 219, 136, 137, 0, 327, 331, 3, 215, 219, 138, 139, 0, 335, 339, 3, 207, 211, 140, 141, 0, 343, 347, 3, 191, 195, 142, 143, 0, 351, 355, 3, 191, 195, 144, 145]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_cloudCallout()
    fn get_cloudCallout() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{-20833, 62500, 100000, 0, 6600, 21600, 1, 3, 1800, 4800, 2, 1200, 600, 2977, 3262, 17087, 17337, 67, 21577, 21582, 1235, 3900, 43200, 14370, 6753, 9190, 5333, 7267, 4365, 5945, 4857, 6595, 7273, 6775, 9220, 5785, 7867, 6752, 9215, 7720, 10543, 4360, 5918, 4345, 4693, 26177, 6928, 34899, 16478, 39090, 28827, 34751, 34129, 22954, 41798, 15354, 38324, 5426, 29078, 3952, 22141, 4720, 14000, 5192, 4127, 15789, -11429249, 7426832, -8646143, 5396714, -8748475, 5983381, -7859164, 7034504, -4722533, 6541615, -2776035, 7816140, 37501, 6842000, 1347096, 6910353, 3974558, 4542661, -16496525, 8804134, -14809710, 9151131, 21600000, 5204520, 1585770, 4416628, 686848, 8257449, 844866, 387196, 959901, -4217541, 4255042, 1819082, 1665090, -824660, 891534, -8950887, 1091722, -9809656, 1061181, -4002417, 739161, 9459261, 711490, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![-20833, 62500, 100000, 0, 6600, 21600, 1, 3, 1800, 4800, 2, 1200, 600, 2977, 3262, 17087, 17337, 67, 21577, 21582, 1235, 3900, 43200, 14370, 6753, 9190, 5333, 7267, 4365, 5945, 4857, 6595, 7273, 6775, 9220, 5785, 7867, 6752, 9215, 7720, 10543, 4360, 5918, 4345, 4693, 26177, 6928, 34899, 16478, 39090, 28827, 34751, 34129, 22954, 41798, 15354, 38324, 5426, 29078, 3952, 22141, 4720, 14000, 5192, 4127, 15789, -11429249, 7426832, -8646143, 5396714, -8748475, 5983381, -7859164, 7034504, -4722533, 6541615, -2776035, 7816140, 37501, 6842000, 1347096, 6910353, 3974558, 4542661, -16496525, 8804134, -14809710, 9151131, 21600000, 5204520, 1585770, 4416628, 686848, 8257449, 844866, 387196, 959901, -4217541, 4255042, 1819082, 1665090, -824660, 891534, -8950887, 1091722, -9809656, 1061181, -4002417, 739161, 9459261, 711490, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 0, 48, 50, 196609, 1, 49, 50, 196608, 2, 161, 51, 196608, 3, 165, 51, 196619, 3, 161, 165, 196620, 2, 161, 165, 196619, 2, 177, 181, 196620, 3, 177, 181, 196608, 2, 185, 51, 196608, 3, 189, 51, 196608, 193, 51, 169, 196608, 197, 51, 173, 196615, 201, 205, 51, 196609, 28, 52, 53, 196608, 209, 51, 213, 196609, 217, 54, 55, 196609, 28, 56, 53, 196608, 221, 225, 51, 196609, 229, 201, 209, 196609, 229, 205, 209, 196608, 233, 169, 51, 196608, 237, 173, 51, 196609, 28, 57, 53, 196609, 221, 58, 54, 196608, 249, 253, 51, 196609, 257, 201, 209, 196609, 257, 205, 209, 196608, 261, 169, 51, 196608, 265, 173, 51, 196609, 28, 59, 53, 196609, 28, 60, 53, 196608, 169, 281, 51, 196608, 241, 277, 51, 196608, 269, 225, 51, 196609, 0, 61, 53, 196609, 1, 62, 53, 196609, 0, 63, 53, 196609, 1, 64, 53, 196609, 0, 65, 53, 196609, 1, 66, 53, 196609, 0, 67, 53, 196609, 1, 68, 53, 131091, 161, 165, 196609, 0, 69, 70, 196609, 1, 71, 70, 196609, 0, 72, 70, 196609, 1, 73, 70, 196609, 0, 74, 70, 196609, 1, 75, 70, 196609, 0, 76, 70, 196609, 1, 77, 70, 196609, 0, 78, 70, 196609, 1, 79, 70, 196609, 0, 74, 70, 196609, 1, 80, 70, 196609, 0, 81, 70, 196609, 1, 82, 70, 196609, 0, 83, 70, 196609, 1, 84, 70, 196609, 0, 85, 70, 196609, 1, 86, 70, 196609, 0, 87, 70, 196609, 1, 88, 70, 196609, 0, 89, 70, 196609, 1, 90, 70, 196609, 0, 91, 70, 196609, 1, 77, 70, 196609, 0, 92, 70, 196609, 1, 93, 70, 196609, 0, 91, 70, 196609, 1, 77, 70, 196609, 0, 94, 70, 196609, 1, 95, 70, 196609, 0, 89, 70, 196609, 1, 90, 70, 196609, 0, 96, 70, 196609, 1, 97, 70, 196609, 0, 85, 70, 196609, 1, 86, 70, 196609, 0, 98, 70, 196609, 1, 99, 70, 196609, 0, 85, 70, 196609, 1, 86, 70, 196609, 0, 100, 70, 196609, 1, 101, 70, 196609, 0, 83, 70, 196609, 1, 84, 70, 196609, 0, 102, 70, 196609, 1, 103, 70, 196609, 0, 74, 70, 196609, 1, 80, 70, 196609, 0, 104, 70, 196609, 1, 105, 70, 196609, 0, 78, 70, 196609, 1, 79, 70, 196609, 0, 106, 70, 196609, 1, 107, 70, 196609, 0, 78, 70, 196609, 1, 79, 70, 196609, 0, 108, 70, 196609, 1, 109, 70, 196609, 0, 76, 70, 196609, 1, 77, 70, 196609, 0, 110, 70, 196609, 1, 111, 70, 196609, 0, 72, 70, 196609, 1, 73, 70, 196609, 0, 112, 70, 196609, 1, 113, 70, 196609, 0, 72, 70, 196609, 1, 73, 70};
        data.gdLst = vec![196609, 0, 48, 50, 196609, 1, 49, 50, 196608, 2, 161, 51, 196608, 3, 165, 51, 196619, 3, 161, 165, 196620, 2, 161, 165, 196619, 2, 177, 181, 196620, 3, 177, 181, 196608, 2, 185, 51, 196608, 3, 189, 51, 196608, 193, 51, 169, 196608, 197, 51, 173, 196615, 201, 205, 51, 196609, 28, 52, 53, 196608, 209, 51, 213, 196609, 217, 54, 55, 196609, 28, 56, 53, 196608, 221, 225, 51, 196609, 229, 201, 209, 196609, 229, 205, 209, 196608, 233, 169, 51, 196608, 237, 173, 51, 196609, 28, 57, 53, 196609, 221, 58, 54, 196608, 249, 253, 51, 196609, 257, 201, 209, 196609, 257, 205, 209, 196608, 261, 169, 51, 196608, 265, 173, 51, 196609, 28, 59, 53, 196609, 28, 60, 53, 196608, 169, 281, 51, 196608, 241, 277, 51, 196608, 269, 225, 51, 196609, 0, 61, 53, 196609, 1, 62, 53, 196609, 0, 63, 53, 196609, 1, 64, 53, 196609, 0, 65, 53, 196609, 1, 66, 53, 196609, 0, 67, 53, 196609, 1, 68, 53, 131091, 161, 165, 196609, 0, 69, 70, 196609, 1, 71, 70, 196609, 0, 72, 70, 196609, 1, 73, 70, 196609, 0, 74, 70, 196609, 1, 75, 70, 196609, 0, 76, 70, 196609, 1, 77, 70, 196609, 0, 78, 70, 196609, 1, 79, 70, 196609, 0, 74, 70, 196609, 1, 80, 70, 196609, 0, 81, 70, 196609, 1, 82, 70, 196609, 0, 83, 70, 196609, 1, 84, 70, 196609, 0, 85, 70, 196609, 1, 86, 70, 196609, 0, 87, 70, 196609, 1, 88, 70, 196609, 0, 89, 70, 196609, 1, 90, 70, 196609, 0, 91, 70, 196609, 1, 77, 70, 196609, 0, 92, 70, 196609, 1, 93, 70, 196609, 0, 91, 70, 196609, 1, 77, 70, 196609, 0, 94, 70, 196609, 1, 95, 70, 196609, 0, 89, 70, 196609, 1, 90, 70, 196609, 0, 96, 70, 196609, 1, 97, 70, 196609, 0, 85, 70, 196609, 1, 86, 70, 196609, 0, 98, 70, 196609, 1, 99, 70, 196609, 0, 85, 70, 196609, 1, 86, 70, 196609, 0, 100, 70, 196609, 1, 101, 70, 196609, 0, 83, 70, 196609, 1, 84, 70, 196609, 0, 102, 70, 196609, 1, 103, 70, 196609, 0, 74, 70, 196609, 1, 80, 70, 196609, 0, 104, 70, 196609, 1, 105, 70, 196609, 0, 78, 70, 196609, 1, 79, 70, 196609, 0, 106, 70, 196609, 1, 107, 70, 196609, 0, 78, 70, 196609, 1, 79, 70, 196609, 0, 108, 70, 196609, 1, 109, 70, 196609, 0, 76, 70, 196609, 1, 77, 70, 196609, 0, 110, 70, 196609, 1, 111, 70, 196609, 0, 72, 70, 196609, 1, 73, 70, 196609, 0, 112, 70, 196609, 1, 113, 70, 196609, 0, 72, 70, 196609, 1, 73, 70];
        // Java: data.ahLst = new int[]{3, 48, 159, 160, 49, 159, 160, 169, 173};
        data.ahLst = vec![3, 48, 159, 160, 49, 159, 160, 169, 173];
        // Java: data.cxnLst = new int[]{19, 313, 3, 20, 2, 317, 51, 321, 3, 15, 2, 325, 329, 169, 173};
        data.cxnLst = vec![19, 313, 3, 20, 2, 317, 51, 321, 3, 15, 2, 325, 329, 169, 173];
        // Java: data.rect = new int[]{297, 301, 305, 309};
        data.rect = vec![297, 301, 305, 309];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 332, 336, 3, 340, 344, 114, 115, 3, 348, 352, 116, 117, 3, 356, 360, 118, 119, 3, 364, 368, 120, 121, 3, 372, 376, 122, 123, 3, 380, 384, 124, 125, 3, 388, 392, 126, 127, 3, 396, 400, 128, 129, 3, 404, 408, 130, 131, 3, 412, 416, 132, 133, 3, 420, 424, 134, 135, 2}, {11, 13, 0, 285, 173, 3, 281, 281, 51, 136, 2}, {11, 13, 0, 289, 245, 3, 277, 277, 51, 136, 2}, {11, 13, 0, 293, 273, 3, 225, 225, 51, 136, 2}, {10, 13, 0, 428, 432, 3, 436, 440, 137, 138, 0, 444, 448, 3, 452, 456, 139, 140, 0, 460, 464, 3, 468, 472, 141, 142, 0, 476, 480, 3, 484, 488, 143, 144, 0, 492, 496, 3, 500, 504, 145, 146, 0, 508, 512, 3, 516, 520, 147, 148, 0, 524, 528, 3, 532, 536, 149, 150, 0, 540, 544, 3, 548, 552, 151, 152, 0, 556, 560, 3, 564, 568, 153, 154, 0, 572, 576, 3, 580, 584, 155, 156, 0, 588, 592, 3, 596, 600, 157, 158}};
        data.pathLst = vec![vec![11, 13, 0, 332, 336, 3, 340, 344, 114, 115, 3, 348, 352, 116, 117, 3, 356, 360, 118, 119, 3, 364, 368, 120, 121, 3, 372, 376, 122, 123, 3, 380, 384, 124, 125, 3, 388, 392, 126, 127, 3, 396, 400, 128, 129, 3, 404, 408, 130, 131, 3, 412, 416, 132, 133, 3, 420, 424, 134, 135, 2], vec![11, 13, 0, 285, 173, 3, 281, 281, 51, 136, 2], vec![11, 13, 0, 289, 245, 3, 277, 277, 51, 136, 2], vec![11, 13, 0, 293, 273, 3, 225, 225, 51, 136, 2], vec![10, 13, 0, 428, 432, 3, 436, 440, 137, 138, 0, 444, 448, 3, 452, 456, 139, 140, 0, 460, 464, 3, 468, 472, 141, 142, 0, 476, 480, 3, 484, 488, 143, 144, 0, 492, 496, 3, 500, 504, 145, 146, 0, 508, 512, 3, 516, 520, 147, 148, 0, 524, 528, 3, 532, 536, 149, 150, 0, 540, 544, 3, 548, 552, 151, 152, 0, 556, 560, 3, 564, 568, 153, 154, 0, 572, 576, 3, 580, 584, 155, 156, 0, 588, 592, 3, 596, 600, 157, 158]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_corner()
    fn get_corner() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 50000, 100000, 0, 1, 2};
        data.avcLst = vec![50000, 50000, 100000, 0, 1, 2];
        // Java: data.gdLst = new int[]{196609, 50, 1, 28, 196609, 50, 0, 28, 196625, 51, 48, 54, 196625, 51, 49, 58, 196609, 28, 66, 50, 196609, 28, 62, 50, 196608, 1, 51, 74, 196609, 70, 52, 53, 196610, 78, 1, 53, 196608, 0, 51, 1, 196614, 90, 78, 26, 196614, 90, 0, 70};
        data.gdLst = vec![196609, 50, 1, 28, 196609, 50, 0, 28, 196625, 51, 48, 54, 196625, 51, 49, 58, 196609, 28, 66, 50, 196609, 28, 62, 50, 196608, 1, 51, 74, 196609, 70, 52, 53, 196610, 78, 1, 53, 196608, 0, 51, 1, 196614, 90, 78, 26, 196614, 90, 0, 70];
        // Java: data.ahLst = new int[]{2, 48, 51, 54, 26, 78, 1, 49, 51, 58, 70, 26};
        data.ahLst = vec![2, 48, 51, 54, 26, 78, 1, 49, 51, 58, 70, 26];
        // Java: data.cxnLst = new int[]{51, 0, 86, 20, 2, 1, 19, 26, 3, 15, 82, 26};
        data.cxnLst = vec![51, 0, 86, 20, 2, 1, 19, 26, 3, 15, 82, 26];
        // Java: data.rect = new int[]{26, 94, 98, 1};
        data.rect = vec![26, 94, 98, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 70, 26, 1, 70, 78, 1, 0, 78, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 70, 26, 1, 70, 78, 1, 0, 78, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_cornerTabs()
    fn get_cornerTabs() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{0, 1, 20};
        data.avcLst = vec![0, 1, 20];
        // Java: data.gdLst = new int[]{196615, 0, 1, 48, 196609, 49, 51, 50, 196608, 48, 1, 55, 196608, 48, 0, 55};
        data.gdLst = vec![196615, 0, 1, 48, 196609, 49, 51, 50, 196608, 48, 1, 55, 196608, 48, 0, 55];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{19, 26, 26, 19, 26, 55, 19, 26, 59, 19, 26, 1, 15, 55, 26, 15, 63, 26, 20, 55, 1, 20, 63, 1, 48, 0, 26, 48, 0, 55, 48, 0, 59, 48, 0, 1};
        data.cxnLst = vec![19, 26, 26, 19, 26, 55, 19, 26, 59, 19, 26, 1, 15, 55, 26, 15, 63, 26, 20, 55, 1, 20, 63, 1, 48, 0, 26, 48, 0, 55, 48, 0, 59, 48, 0, 1];
        // Java: data.rect = new int[]{55, 55, 63, 59};
        data.rect = vec![55, 55, 63, 59];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 55, 26, 1, 26, 55, 2}, {11, 13, 0, 26, 59, 1, 55, 1, 1, 26, 1, 2}, {11, 13, 0, 63, 26, 1, 0, 26, 1, 0, 55, 2}, {11, 13, 0, 0, 59, 1, 0, 1, 1, 63, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 55, 26, 1, 26, 55, 2], vec![11, 13, 0, 26, 59, 1, 55, 1, 1, 26, 1, 2], vec![11, 13, 0, 63, 26, 1, 0, 26, 1, 0, 55, 2], vec![11, 13, 0, 0, 59, 1, 0, 1, 1, 63, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_cube()
    fn get_cube() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 0, 100000, 1, 2};
        data.avcLst = vec![25000, 0, 100000, 1, 2];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 53, 50, 196608, 1, 49, 57, 196609, 61, 51, 52, 196610, 57, 1, 52, 196608, 0, 49, 57, 196609, 73, 51, 52, 196610, 57, 0, 52};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 53, 50, 196608, 1, 49, 57, 196609, 61, 51, 52, 196610, 57, 1, 52, 196608, 0, 49, 57, 196609, 73, 51, 52, 196610, 57, 0, 52];
        // Java: data.ahLst = new int[]{2, 48, 49, 50, 26, 57};
        data.ahLst = vec![2, 48, 49, 50, 26, 57];
        // Java: data.cxnLst = new int[]{15, 81, 26, 15, 77, 57, 19, 26, 69, 20, 77, 1, 49, 73, 69, 49, 0, 65};
        data.cxnLst = vec![15, 81, 26, 15, 77, 57, 19, 26, 69, 20, 77, 1, 49, 73, 69, 49, 0, 65];
        // Java: data.rect = new int[]{26, 57, 73, 1};
        data.rect = vec![26, 57, 73, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 57, 1, 73, 57, 1, 73, 1, 1, 26, 1, 2}, {7, 12, 0, 73, 57, 1, 0, 26, 1, 0, 61, 1, 73, 1, 2}, {9, 12, 0, 26, 57, 1, 57, 26, 1, 0, 26, 1, 73, 57, 2}, {10, 13, 0, 26, 57, 1, 57, 26, 1, 0, 26, 1, 0, 61, 1, 73, 1, 1, 26, 1, 2, 0, 26, 57, 1, 73, 57, 1, 0, 26, 0, 73, 57, 1, 73, 1}};
        data.pathLst = vec![vec![11, 12, 0, 26, 57, 1, 73, 57, 1, 73, 1, 1, 26, 1, 2], vec![7, 12, 0, 73, 57, 1, 0, 26, 1, 0, 61, 1, 73, 1, 2], vec![9, 12, 0, 26, 57, 1, 57, 26, 1, 0, 26, 1, 73, 57, 2], vec![10, 13, 0, 26, 57, 1, 57, 26, 1, 0, 26, 1, 0, 61, 1, 73, 1, 1, 26, 1, 2, 0, 26, 57, 1, 73, 57, 1, 0, 26, 0, 73, 57, 1, 73, 1]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_curvedConnector2()
    fn get_curvedConnector2() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[0];
        data.avcLst = vec![];
        // Java: data.gdLst = new int[0];
        data.gdLst = vec![];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[0];
        data.cxnLst = vec![];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{10, 13, 0, 26, 26, 5, 2, 26, 0, 3, 0, 1}};
        data.pathLst = vec![vec![10, 13, 0, 26, 26, 5, 2, 26, 0, 3, 0, 1]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_curvedConnector3()
    fn get_curvedConnector3() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 100000, 2, 3, 4, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![50000, 100000, 2, 3, 4, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196610, 26, 55, 50, 196610, 0, 55, 50, 196609, 1, 51, 52};
        data.gdLst = vec![196609, 0, 48, 49, 196610, 26, 55, 50, 196610, 0, 55, 50, 196609, 1, 51, 52];
        // Java: data.ahLst = new int[]{1, 48, 53, 54, 55, 3};
        data.ahLst = vec![1, 48, 53, 54, 55, 3];
        // Java: data.cxnLst = new int[0];
        data.cxnLst = vec![];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{10, 13, 0, 26, 26, 5, 59, 26, 55, 22, 55, 3, 5, 55, 67, 63, 1, 0, 1}};
        data.pathLst = vec![vec![10, 13, 0, 26, 26, 5, 59, 26, 55, 22, 55, 3, 5, 55, 67, 63, 1, 0, 1]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_curvedConnector4()
    fn get_curvedConnector4() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 50000, 100000, 2, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![50000, 50000, 100000, 2, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 0, 48, 50, 196610, 26, 54, 51, 196610, 0, 54, 51, 196610, 54, 62, 51, 196610, 62, 0, 51, 196609, 1, 49, 50, 196610, 26, 74, 51, 196610, 26, 78, 51, 196610, 78, 74, 51, 196610, 1, 74, 51};
        data.gdLst = vec![196609, 0, 48, 50, 196610, 26, 54, 51, 196610, 0, 54, 51, 196610, 54, 62, 51, 196610, 62, 0, 51, 196609, 1, 49, 50, 196610, 26, 74, 51, 196610, 26, 78, 51, 196610, 78, 74, 51, 196610, 1, 74, 51];
        // Java: data.ahLst = new int[]{1, 48, 52, 53, 54, 78, 2, 49, 52, 53, 62, 74};
        data.ahLst = vec![1, 48, 52, 53, 54, 78, 2, 49, 52, 53, 62, 74];
        // Java: data.cxnLst = new int[0];
        data.cxnLst = vec![];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{10, 13, 0, 26, 26, 5, 58, 26, 54, 82, 54, 78, 5, 54, 86, 66, 74, 62, 74, 5, 70, 74, 0, 90, 0, 1}};
        data.pathLst = vec![vec![10, 13, 0, 26, 26, 5, 58, 26, 54, 82, 54, 78, 5, 54, 86, 66, 74, 62, 74, 5, 70, 74, 0, 90, 0, 1]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_curvedConnector5()
    fn get_curvedConnector5() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 50000, 50000, 100000, 2, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![50000, 50000, 50000, 100000, 2, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 0, 48, 51, 196609, 0, 50, 51, 196610, 55, 59, 52, 196610, 26, 55, 52, 196610, 55, 63, 52, 196610, 59, 63, 52, 196610, 59, 0, 52, 196609, 1, 49, 51, 196610, 26, 83, 52, 196610, 26, 87, 52, 196610, 87, 83, 52, 196610, 1, 83, 52, 196610, 99, 83, 52, 196610, 99, 1, 52};
        data.gdLst = vec![196609, 0, 48, 51, 196609, 0, 50, 51, 196610, 55, 59, 52, 196610, 26, 55, 52, 196610, 55, 63, 52, 196610, 59, 63, 52, 196610, 59, 0, 52, 196609, 1, 49, 51, 196610, 26, 83, 52, 196610, 26, 87, 52, 196610, 87, 83, 52, 196610, 1, 83, 52, 196610, 99, 83, 52, 196610, 99, 1, 52];
        // Java: data.ahLst = new int[]{1, 48, 53, 54, 55, 87, 2, 49, 53, 54, 63, 83, 1, 50, 53, 54, 59, 99};
        data.ahLst = vec![1, 48, 53, 54, 55, 87, 2, 49, 53, 54, 63, 83, 1, 50, 53, 54, 59, 99];
        // Java: data.cxnLst = new int[0];
        data.cxnLst = vec![];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{10, 13, 0, 26, 26, 5, 67, 26, 55, 91, 55, 87, 5, 55, 95, 71, 83, 63, 83, 5, 75, 83, 59, 103, 59, 99, 5, 59, 107, 79, 1, 0, 1}};
        data.pathLst = vec![vec![10, 13, 0, 26, 26, 5, 67, 26, 55, 91, 55, 87, 5, 55, 95, 71, 83, 63, 83, 5, 75, 83, 59, 103, 59, 99, 5, 59, 107, 79, 1, 0, 1]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_curvedDownArrow()
    fn get_curvedDownArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 50000, 25000, 50000, 0, 100000, 4, 2, 1};
        data.avcLst = vec![25000, 50000, 25000, 50000, 0, 100000, 4, 2, 1];
        // Java: data.gdLst = new int[]{196609, 51, 0, 28, 196625, 52, 49, 57, 196625, 52, 48, 53, 196609, 28, 65, 53, 196609, 28, 61, 53, 196610, 69, 73, 54, 196608, 2, 52, 77, 196609, 81, 55, 56, 196609, 85, 85, 56, 196609, 69, 69, 56, 196608, 89, 52, 93, 65549, 97, 196609, 101, 1, 85, 196609, 53, 103, 28, 196625, 52, 50, 107, 196609, 28, 50, 53, 196608, 81, 69, 52, 196609, 1, 1, 56, 196609, 115, 115, 56, 196608, 123, 52, 127, 65549, 131, 196609, 135, 81, 1, 196608, 81, 137, 52, 196608, 119, 137, 52, 196608, 73, 52, 69, 196609, 149, 56, 55, 196608, 141, 52, 153, 196608, 145, 153, 52, 196609, 73, 56, 55, 196608, 0, 52, 165, 196608, 1, 52, 115, 131091, 115, 137, 196608, 52, 52, 177, 196608, 1, 52, 103, 196610, 81, 119, 55, 196609, 69, 56, 55, 131091, 103, 192, 196608, 15, 177, 52, 196608, 15, 52, 196, 196608, 196, 52, 20, 196608, 20, 196, 52};
        data.gdLst = vec![196609, 51, 0, 28, 196625, 52, 49, 57, 196625, 52, 48, 53, 196609, 28, 65, 53, 196609, 28, 61, 53, 196610, 69, 73, 54, 196608, 2, 52, 77, 196609, 81, 55, 56, 196609, 85, 85, 56, 196609, 69, 69, 56, 196608, 89, 52, 93, 65549, 97, 196609, 101, 1, 85, 196609, 53, 103, 28, 196625, 52, 50, 107, 196609, 28, 50, 53, 196608, 81, 69, 52, 196609, 1, 1, 56, 196609, 115, 115, 56, 196608, 123, 52, 127, 65549, 131, 196609, 135, 81, 1, 196608, 81, 137, 52, 196608, 119, 137, 52, 196608, 73, 52, 69, 196609, 149, 56, 55, 196608, 141, 52, 153, 196608, 145, 153, 52, 196609, 73, 56, 55, 196608, 0, 52, 165, 196608, 1, 52, 115, 131091, 115, 137, 196608, 52, 52, 177, 196608, 1, 52, 103, 196610, 81, 119, 55, 196609, 69, 56, 55, 131091, 103, 192, 196608, 15, 177, 52, 196608, 15, 52, 196, 196608, 196, 52, 20, 196608, 20, 196, 52];
        // Java: data.ahLst = new int[]{1, 48, 52, 49, 145, 173, 1, 49, 52, 57, 157, 1, 2, 50, 52, 107, 0, 173};
        data.ahLst = vec![1, 48, 52, 49, 145, 173, 1, 49, 52, 57, 157, 1, 2, 50, 52, 107, 0, 173];
        // Java: data.cxnLst = new int[]{15, 188, 26, 20, 192, 1, 20, 157, 173, 20, 169, 1, 52, 161, 173};
        data.cxnLst = vec![15, 188, 26, 20, 192, 1, 20, 157, 173, 20, 169, 1, 52, 161, 173];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 169, 1, 1, 157, 173, 1, 141, 173, 3, 81, 1, 199, 180, 1, 119, 26, 3, 81, 1, 15, 177, 1, 161, 173, 2}, {7, 12, 0, 188, 184, 3, 81, 1, 203, 207, 1, 26, 1, 3, 81, 1, 19, 211, 2}, {10, 13, 0, 188, 184, 3, 81, 1, 203, 207, 1, 26, 1, 3, 81, 1, 19, 20, 1, 119, 26, 3, 81, 1, 15, 177, 1, 161, 173, 1, 169, 1, 1, 157, 173, 1, 141, 173, 3, 81, 1, 199, 180}};
        data.pathLst = vec![vec![11, 12, 0, 169, 1, 1, 157, 173, 1, 141, 173, 3, 81, 1, 199, 180, 1, 119, 26, 3, 81, 1, 15, 177, 1, 161, 173, 2], vec![7, 12, 0, 188, 184, 3, 81, 1, 203, 207, 1, 26, 1, 3, 81, 1, 19, 211, 2], vec![10, 13, 0, 188, 184, 3, 81, 1, 203, 207, 1, 26, 1, 3, 81, 1, 19, 20, 1, 119, 26, 3, 81, 1, 15, 177, 1, 161, 173, 1, 169, 1, 1, 157, 173, 1, 141, 173, 3, 81, 1, 199, 180]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_curvedLeftArrow()
    fn get_curvedLeftArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 50000, 25000, 50000, 0, 100000, 4, 2, 1, -5400000};
        data.avcLst = vec![25000, 50000, 25000, 50000, 0, 100000, 4, 2, 1, -5400000];
        // Java: data.gdLst = new int[]{196609, 51, 1, 28, 196625, 52, 49, 58, 196625, 52, 48, 62, 196609, 28, 66, 53, 196609, 28, 62, 53, 196610, 70, 74, 54, 196608, 3, 52, 78, 196609, 82, 55, 56, 196609, 86, 86, 56, 196609, 70, 70, 56, 196608, 90, 52, 94, 65549, 98, 196609, 102, 0, 86, 196609, 53, 104, 28, 196625, 52, 50, 108, 196609, 28, 112, 53, 196608, 82, 70, 52, 196609, 0, 0, 56, 196609, 116, 116, 56, 196608, 124, 52, 128, 65549, 132, 196609, 136, 82, 0, 196608, 82, 138, 52, 196608, 120, 138, 52, 196608, 74, 52, 70, 196609, 150, 56, 55, 196608, 142, 52, 154, 196608, 146, 154, 52, 196609, 74, 56, 55, 196608, 1, 52, 166, 196608, 26, 116, 52, 131091, 116, 138, 196608, 52, 52, 178, 196608, 26, 104, 52, 196610, 82, 120, 55, 196609, 70, 56, 55, 131091, 104, 193, 196608, 197, 52, 178, 196608, 178, 197, 52, 196608, 52, 52, 197};
        data.gdLst = vec![196609, 51, 1, 28, 196625, 52, 49, 58, 196625, 52, 48, 62, 196609, 28, 66, 53, 196609, 28, 62, 53, 196610, 70, 74, 54, 196608, 3, 52, 78, 196609, 82, 55, 56, 196609, 86, 86, 56, 196609, 70, 70, 56, 196608, 90, 52, 94, 65549, 98, 196609, 102, 0, 86, 196609, 53, 104, 28, 196625, 52, 50, 108, 196609, 28, 112, 53, 196608, 82, 70, 52, 196609, 0, 0, 56, 196609, 116, 116, 56, 196608, 124, 52, 128, 65549, 132, 196609, 136, 82, 0, 196608, 82, 138, 52, 196608, 120, 138, 52, 196608, 74, 52, 70, 196609, 150, 56, 55, 196608, 142, 52, 154, 196608, 146, 154, 52, 196609, 74, 56, 55, 196608, 1, 52, 166, 196608, 26, 116, 52, 131091, 116, 138, 196608, 52, 52, 178, 196608, 26, 104, 52, 196610, 82, 120, 55, 196609, 70, 56, 55, 131091, 104, 193, 196608, 197, 52, 178, 196608, 178, 197, 52, 196608, 52, 52, 197];
        // Java: data.ahLst = new int[]{2, 48, 52, 62, 174, 142, 2, 49, 52, 58, 0, 158, 1, 50, 52, 108, 174, 1};
        data.ahLst = vec![2, 48, 52, 62, 174, 142, 2, 49, 52, 58, 0, 158, 1, 50, 52, 108, 174, 1];
        // Java: data.cxnLst = new int[]{19, 26, 193, 19, 174, 158, 44, 26, 170, 20, 174, 162, 52, 0, 189};
        data.cxnLst = vec![19, 26, 193, 19, 174, 158, 44, 26, 170, 20, 174, 162, 52, 0, 189];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 170, 1, 174, 158, 1, 174, 142, 3, 0, 82, 178, 200, 3, 0, 82, 208, 204, 1, 174, 162, 2}, {7, 12, 0, 0, 120, 3, 0, 82, 52, 57, 1, 26, 26, 3, 0, 82, 15, 20, 2}, {10, 13, 0, 0, 120, 3, 0, 82, 52, 57, 1, 26, 26, 3, 0, 82, 15, 20, 1, 0, 120, 3, 0, 82, 52, 178, 1, 174, 162, 1, 26, 170, 1, 174, 158, 1, 174, 142, 3, 0, 82, 178, 200}};
        data.pathLst = vec![vec![11, 12, 0, 26, 170, 1, 174, 158, 1, 174, 142, 3, 0, 82, 178, 200, 3, 0, 82, 208, 204, 1, 174, 162, 2], vec![7, 12, 0, 0, 120, 3, 0, 82, 52, 57, 1, 26, 26, 3, 0, 82, 15, 20, 2], vec![10, 13, 0, 0, 120, 3, 0, 82, 52, 57, 1, 26, 26, 3, 0, 82, 15, 20, 1, 0, 120, 3, 0, 82, 52, 178, 1, 174, 162, 1, 26, 170, 1, 174, 158, 1, 174, 142, 3, 0, 82, 178, 200]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_curvedRightArrow()
    fn get_curvedRightArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 50000, 25000, 50000, 0, 100000, 4, 2, 1};
        data.avcLst = vec![25000, 50000, 25000, 50000, 0, 100000, 4, 2, 1];
        // Java: data.gdLst = new int[]{196609, 51, 1, 28, 196625, 52, 49, 57, 196625, 52, 48, 61, 196609, 28, 65, 53, 196609, 28, 61, 53, 196610, 69, 73, 54, 196608, 3, 52, 77, 196609, 81, 55, 56, 196609, 85, 85, 56, 196609, 69, 69, 56, 196608, 89, 52, 93, 65549, 97, 196609, 101, 0, 85, 196609, 53, 103, 28, 196625, 52, 50, 107, 196609, 28, 111, 53, 196608, 81, 69, 52, 196609, 0, 0, 56, 196609, 115, 115, 56, 196608, 123, 52, 127, 65549, 131, 196609, 135, 81, 0, 196608, 81, 137, 52, 196608, 119, 137, 52, 196608, 73, 52, 69, 196609, 149, 56, 55, 196608, 141, 52, 153, 196608, 145, 153, 52, 196609, 73, 56, 55, 196608, 1, 52, 165, 196608, 0, 52, 115, 131091, 115, 137, 196608, 19, 52, 177, 196608, 52, 52, 177, 196608, 0, 52, 103, 196610, 81, 119, 55, 196609, 69, 56, 55, 131091, 103, 196, 196608, 200, 52, 20, 196608, 20, 200, 52, 196608, 19, 52, 200};
        data.gdLst = vec![196609, 51, 1, 28, 196625, 52, 49, 57, 196625, 52, 48, 61, 196609, 28, 65, 53, 196609, 28, 61, 53, 196610, 69, 73, 54, 196608, 3, 52, 77, 196609, 81, 55, 56, 196609, 85, 85, 56, 196609, 69, 69, 56, 196608, 89, 52, 93, 65549, 97, 196609, 101, 0, 85, 196609, 53, 103, 28, 196625, 52, 50, 107, 196609, 28, 111, 53, 196608, 81, 69, 52, 196609, 0, 0, 56, 196609, 115, 115, 56, 196608, 123, 52, 127, 65549, 131, 196609, 135, 81, 0, 196608, 81, 137, 52, 196608, 119, 137, 52, 196608, 73, 52, 69, 196609, 149, 56, 55, 196608, 141, 52, 153, 196608, 145, 153, 52, 196609, 73, 56, 55, 196608, 1, 52, 165, 196608, 0, 52, 115, 131091, 115, 137, 196608, 19, 52, 177, 196608, 52, 52, 177, 196608, 0, 52, 103, 196610, 81, 119, 55, 196609, 69, 56, 55, 131091, 103, 196, 196608, 200, 52, 20, 196608, 20, 200, 52, 196608, 19, 52, 200];
        // Java: data.ahLst = new int[]{2, 48, 52, 61, 173, 141, 2, 49, 52, 57, 0, 157, 1, 50, 52, 107, 173, 1};
        data.ahLst = vec![2, 48, 52, 61, 173, 141, 2, 49, 52, 57, 0, 157, 1, 50, 52, 107, 173, 1];
        // Java: data.cxnLst = new int[]{19, 26, 192, 20, 173, 161, 52, 0, 169, 52, 173, 157, 52, 0, 196};
        data.cxnLst = vec![19, 26, 192, 20, 173, 161, 52, 0, 169, 52, 173, 157, 52, 0, 196];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 81, 3, 0, 81, 19, 184, 1, 173, 157, 1, 0, 169, 1, 173, 161, 1, 173, 145, 3, 0, 81, 180, 177, 2}, {7, 12, 0, 0, 69, 3, 0, 81, 15, 203, 3, 0, 81, 211, 207, 2}, {10, 13, 0, 26, 81, 3, 0, 81, 19, 184, 1, 173, 157, 1, 0, 169, 1, 173, 161, 1, 173, 145, 3, 0, 81, 180, 177, 1, 26, 81, 3, 0, 81, 19, 20, 1, 0, 69, 3, 0, 81, 15, 203}};
        data.pathLst = vec![vec![11, 12, 0, 26, 81, 3, 0, 81, 19, 184, 1, 173, 157, 1, 0, 169, 1, 173, 161, 1, 173, 145, 3, 0, 81, 180, 177, 2], vec![7, 12, 0, 0, 69, 3, 0, 81, 15, 203, 3, 0, 81, 211, 207, 2], vec![10, 13, 0, 26, 81, 3, 0, 81, 19, 184, 1, 173, 157, 1, 0, 169, 1, 173, 161, 1, 173, 145, 3, 0, 81, 180, 177, 1, 26, 81, 3, 0, 81, 19, 20, 1, 0, 69, 3, 0, 81, 15, 203]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_curvedUpArrow()
    fn get_curvedUpArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 50000, 25000, 50000, 0, 100000, 4, 2, 1, -5400000};
        data.avcLst = vec![25000, 50000, 25000, 50000, 0, 100000, 4, 2, 1, -5400000];
        // Java: data.gdLst = new int[]{196609, 51, 0, 28, 196625, 52, 49, 58, 196625, 52, 48, 53, 196609, 28, 66, 53, 196609, 28, 62, 53, 196610, 70, 74, 54, 196608, 2, 52, 78, 196609, 82, 55, 56, 196609, 86, 86, 56, 196609, 70, 70, 56, 196608, 90, 52, 94, 65549, 98, 196609, 102, 1, 86, 196609, 53, 104, 28, 196625, 52, 50, 108, 196609, 28, 50, 53, 196608, 82, 70, 52, 196609, 1, 1, 56, 196609, 116, 116, 56, 196608, 124, 52, 128, 65549, 132, 196609, 136, 82, 1, 196608, 82, 138, 52, 196608, 120, 138, 52, 196608, 74, 52, 70, 196609, 150, 56, 55, 196608, 142, 52, 154, 196608, 146, 154, 52, 196609, 74, 56, 55, 196608, 0, 52, 166, 196608, 26, 116, 52, 131091, 116, 138, 196608, 52, 52, 178, 196608, 26, 104, 52, 196610, 82, 120, 55, 196609, 70, 56, 55, 131091, 104, 193, 196608, 197, 52, 178, 196608, 52, 52, 200, 196608, 20, 52, 178, 196608, 178, 197, 52, 196608, 20, 52, 197};
        data.gdLst = vec![196609, 51, 0, 28, 196625, 52, 49, 58, 196625, 52, 48, 53, 196609, 28, 66, 53, 196609, 28, 62, 53, 196610, 70, 74, 54, 196608, 2, 52, 78, 196609, 82, 55, 56, 196609, 86, 86, 56, 196609, 70, 70, 56, 196608, 90, 52, 94, 65549, 98, 196609, 102, 1, 86, 196609, 53, 104, 28, 196625, 52, 50, 108, 196609, 28, 50, 53, 196608, 82, 70, 52, 196609, 1, 1, 56, 196609, 116, 116, 56, 196608, 124, 52, 128, 65549, 132, 196609, 136, 82, 1, 196608, 82, 138, 52, 196608, 120, 138, 52, 196608, 74, 52, 70, 196609, 150, 56, 55, 196608, 142, 52, 154, 196608, 146, 154, 52, 196609, 74, 56, 55, 196608, 0, 52, 166, 196608, 26, 116, 52, 131091, 116, 138, 196608, 52, 52, 178, 196608, 26, 104, 52, 196610, 82, 120, 55, 196609, 70, 56, 55, 131091, 104, 193, 196608, 197, 52, 178, 196608, 52, 52, 200, 196608, 20, 52, 178, 196608, 178, 197, 52, 196608, 20, 52, 197];
        // Java: data.ahLst = new int[]{1, 48, 52, 62, 146, 174, 1, 49, 52, 58, 158, 26, 2, 50, 52, 108, 0, 174};
        data.ahLst = vec![1, 48, 52, 62, 146, 174, 1, 49, 52, 58, 158, 26, 2, 50, 52, 108, 0, 174];
        // Java: data.cxnLst = new int[]{15, 170, 26, 15, 158, 174, 15, 193, 26, 20, 189, 1, 52, 162, 174};
        data.cxnLst = vec![15, 170, 26, 15, 158, 174, 15, 193, 26, 20, 189, 1, 52, 162, 174];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 170, 26, 1, 162, 174, 1, 146, 174, 3, 82, 1, 208, 212, 3, 82, 1, 216, 200, 1, 158, 174, 2}, {7, 12, 0, 82, 1, 3, 82, 1, 20, 20, 1, 70, 26, 3, 82, 1, 19, 57, 2}, {10, 13, 0, 189, 185, 3, 82, 1, 216, 200, 1, 158, 174, 1, 170, 26, 1, 162, 174, 1, 146, 174, 3, 82, 1, 208, 178, 1, 82, 1, 3, 82, 1, 20, 20, 1, 70, 26, 3, 82, 1, 19, 57}};
        data.pathLst = vec![vec![11, 12, 0, 170, 26, 1, 162, 174, 1, 146, 174, 3, 82, 1, 208, 212, 3, 82, 1, 216, 200, 1, 158, 174, 2], vec![7, 12, 0, 82, 1, 3, 82, 1, 20, 20, 1, 70, 26, 3, 82, 1, 19, 57, 2], vec![10, 13, 0, 189, 185, 3, 82, 1, 216, 200, 1, 158, 174, 1, 170, 26, 1, 162, 174, 1, 146, 174, 3, 82, 1, 208, 178, 1, 82, 1, 3, 82, 1, 20, 20, 1, 70, 26, 3, 82, 1, 19, 57]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_decagon()
    fn get_decagon() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{105146, 100000, 2160000, 4320000, 0};
        data.avcLst = vec![105146, 100000, 2160000, 4320000, 0];
        // Java: data.gdLst = new int[]{196609, 3, 48, 49, 131093, 2, 50, 131093, 2, 51, 196608, 2, 52, 57, 196608, 2, 52, 60, 196608, 2, 60, 52, 196608, 2, 57, 52, 131092, 53, 51, 131092, 53, 50, 196608, 3, 52, 79, 196608, 3, 52, 82, 196608, 3, 82, 52, 196608, 3, 79, 52};
        data.gdLst = vec![196609, 3, 48, 49, 131093, 2, 50, 131093, 2, 51, 196608, 2, 52, 57, 196608, 2, 52, 60, 196608, 2, 60, 52, 196608, 2, 57, 52, 131092, 53, 51, 131092, 53, 50, 196608, 3, 52, 79, 196608, 3, 52, 82, 196608, 3, 82, 52, 196608, 3, 79, 52];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{52, 75, 89, 52, 0, 3, 52, 75, 93, 20, 71, 97, 20, 67, 97, 19, 63, 93, 19, 26, 3, 19, 63, 89, 15, 67, 85, 15, 71, 85};
        data.cxnLst = vec![52, 75, 89, 52, 0, 3, 52, 75, 93, 20, 71, 97, 20, 67, 97, 19, 63, 93, 19, 26, 3, 19, 63, 89, 15, 67, 85, 15, 71, 85];
        // Java: data.rect = new int[]{63, 89, 75, 93};
        data.rect = vec![63, 89, 75, 93];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 1, 63, 89, 1, 67, 85, 1, 71, 85, 1, 75, 89, 1, 0, 3, 1, 75, 93, 1, 71, 97, 1, 67, 97, 1, 63, 93, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 1, 63, 89, 1, 67, 85, 1, 71, 85, 1, 75, 89, 1, 0, 3, 1, 75, 93, 1, 71, 97, 1, 67, 97, 1, 63, 93, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_diagStripe()
    fn get_diagStripe() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 0, 100000, 1, 2};
        data.avcLst = vec![50000, 0, 100000, 1, 2];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 0, 53, 50, 196609, 57, 51, 52, 196610, 57, 0, 52, 196609, 1, 53, 50, 196609, 69, 51, 52, 196610, 69, 1, 52};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 0, 53, 50, 196609, 57, 51, 52, 196610, 57, 0, 52, 196609, 1, 53, 50, 196609, 69, 51, 52, 196610, 69, 1, 52];
        // Java: data.ahLst = new int[]{2, 48, 49, 50, 26, 69};
        data.ahLst = vec![2, 48, 49, 50, 26, 69];
        // Java: data.cxnLst = new int[]{49, 2, 3, 19, 26, 77, 19, 61, 73, 15, 65, 26};
        data.cxnLst = vec![49, 2, 3, 19, 26, 77, 19, 61, 73, 15, 65, 26];
        // Java: data.rect = new int[]{26, 26, 65, 77};
        data.rect = vec![26, 26, 65, 77];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 69, 1, 57, 26, 1, 0, 26, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 69, 1, 57, 26, 1, 0, 26, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_diamond()
    fn get_diamond() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 4, 0};
        data.avcLst = vec![3, 4, 0];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 1, 48, 49};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 1, 48, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 50, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 50, 0, 3];
        // Java: data.rect = new int[]{33, 22, 51, 55};
        data.rect = vec![33, 22, 51, 55];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 1, 2, 26, 1, 0, 3, 1, 2, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 1, 2, 26, 1, 0, 3, 1, 2, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_dodecagon()
    fn get_dodecagon() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{2894, 21600, 7906, 13694, 18706, 0};
        data.avcLst = vec![2894, 21600, 7906, 13694, 18706, 0];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 0, 50, 49, 196609, 0, 51, 49, 196609, 0, 52, 49, 196609, 1, 48, 49, 196609, 1, 50, 49, 196609, 1, 51, 49, 196609, 1, 52, 49};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 0, 50, 49, 196609, 0, 51, 49, 196609, 0, 52, 49, 196609, 1, 48, 49, 196609, 1, 50, 49, 196609, 1, 51, 49, 196609, 1, 52, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{53, 66, 70, 53, 0, 74, 53, 0, 78, 53, 66, 82, 20, 62, 1, 20, 58, 1, 19, 54, 82, 19, 26, 78, 19, 26, 74, 19, 54, 70, 15, 58, 26, 15, 62, 26};
        data.cxnLst = vec![53, 66, 70, 53, 0, 74, 53, 0, 78, 53, 66, 82, 20, 62, 1, 20, 58, 1, 19, 54, 82, 19, 26, 78, 19, 26, 74, 19, 54, 70, 15, 58, 26, 15, 62, 26];
        // Java: data.rect = new int[]{54, 70, 66, 82};
        data.rect = vec![54, 70, 66, 82];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 74, 1, 54, 70, 1, 58, 26, 1, 62, 26, 1, 66, 70, 1, 0, 74, 1, 0, 78, 1, 66, 82, 1, 62, 1, 1, 58, 1, 1, 54, 82, 1, 26, 78, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 74, 1, 54, 70, 1, 58, 26, 1, 62, 26, 1, 66, 70, 1, 0, 74, 1, 0, 78, 1, 66, 82, 1, 62, 1, 1, 58, 1, 1, 54, 82, 1, 26, 78, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_donut()
    fn get_donut() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 0, 50000, 100000, 2700000, -5400000};
        data.avcLst = vec![25000, 0, 50000, 100000, 2700000, -5400000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 54, 51, 196608, 2, 49, 58, 196608, 3, 49, 58, 131093, 2, 52, 131092, 3, 52, 196608, 2, 49, 70, 196608, 2, 70, 49, 196608, 3, 49, 73, 196608, 3, 73, 49};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 54, 51, 196608, 2, 49, 58, 196608, 3, 49, 58, 131093, 2, 52, 131092, 3, 52, 196608, 2, 49, 70, 196608, 2, 70, 49, 196608, 3, 49, 73, 196608, 3, 73, 49];
        // Java: data.ahLst = new int[]{5, 48, 49, 50, 58, 3};
        data.ahLst = vec![5, 48, 49, 50, 58, 3];
        // Java: data.cxnLst = new int[]{15, 2, 26, 15, 76, 84, 19, 26, 3, 20, 76, 88, 20, 2, 1, 20, 80, 88, 49, 0, 3, 15, 80, 84};
        data.cxnLst = vec![15, 2, 26, 15, 76, 84, 19, 26, 3, 20, 76, 88, 20, 2, 1, 20, 80, 88, 49, 0, 3, 15, 80, 84];
        // Java: data.rect = new int[]{76, 84, 80, 88};
        data.rect = vec![76, 84, 80, 88];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2, 0, 58, 3, 3, 62, 66, 19, 53, 3, 62, 66, 20, 53, 3, 62, 66, 49, 53, 3, 62, 66, 15, 53, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2, 0, 58, 3, 3, 62, 66, 19, 53, 3, 62, 66, 20, 53, 3, 62, 66, 49, 53, 3, 62, 66, 15, 53, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_doubleWave()
    fn get_doubleWave() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{6250, 0, 0, 12500, -10000, 10000, 100000, 10, 3, 50000, 6, 2};
        data.avcLst = vec![6250, 0, 0, 12500, -10000, 10000, 100000, 10, 3, 50000, 6, 2];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196625, 52, 49, 53, 196609, 1, 60, 54, 196609, 68, 55, 56, 196608, 68, 50, 72, 196608, 68, 72, 50, 196608, 1, 50, 68, 196608, 84, 50, 72, 196608, 84, 72, 50, 196609, 0, 64, 54, 196609, 0, 64, 57, 65539, 96, 196614, 100, 50, 100, 196608, 26, 50, 106, 196614, 100, 100, 50, 196608, 0, 50, 114, 196610, 106, 118, 58, 196608, 110, 122, 50, 196610, 106, 118, 56, 196608, 110, 130, 50, 196610, 110, 118, 59, 196608, 138, 122, 50, 196610, 142, 118, 59, 196608, 26, 114, 50, 196608, 0, 106, 50, 196608, 150, 122, 50, 196608, 150, 130, 50, 196610, 150, 154, 59, 196608, 166, 122, 50, 196610, 170, 154, 59, 196608, 0, 50, 104, 196608, 2, 96, 50, 131077, 110, 150, 131076, 118, 154, 196609, 1, 60, 57, 196608, 1, 50, 192};
        data.gdLst = vec![196625, 50, 48, 51, 196625, 52, 49, 53, 196609, 1, 60, 54, 196609, 68, 55, 56, 196608, 68, 50, 72, 196608, 68, 72, 50, 196608, 1, 50, 68, 196608, 84, 50, 72, 196608, 84, 72, 50, 196609, 0, 64, 54, 196609, 0, 64, 57, 65539, 96, 196614, 100, 50, 100, 196608, 26, 50, 106, 196614, 100, 100, 50, 196608, 0, 50, 114, 196610, 106, 118, 58, 196608, 110, 122, 50, 196610, 106, 118, 56, 196608, 110, 130, 50, 196610, 110, 118, 59, 196608, 138, 122, 50, 196610, 142, 118, 59, 196608, 26, 114, 50, 196608, 0, 106, 50, 196608, 150, 122, 50, 196608, 150, 130, 50, 196610, 150, 154, 59, 196608, 166, 122, 50, 196610, 170, 154, 59, 196608, 0, 50, 104, 196608, 2, 96, 50, 131077, 110, 150, 131076, 118, 154, 196609, 1, 60, 57, 196608, 1, 50, 192];
        // Java: data.ahLst = new int[]{2, 48, 50, 51, 26, 68, 1, 49, 52, 53, 182, 1};
        data.ahLst = vec![2, 48, 50, 51, 26, 68, 1, 49, 52, 53, 182, 1];
        // Java: data.cxnLst = new int[]{20, 166, 68, 19, 104, 3, 15, 138, 84, 50, 178, 3};
        data.cxnLst = vec![20, 166, 68, 19, 104, 3, 15, 138, 84, 50, 178, 3];
        // Java: data.rect = new int[]{186, 192, 189, 196};
        data.rect = vec![186, 192, 189, 196];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 110, 68, 5, 126, 76, 134, 80, 138, 68, 5, 142, 76, 146, 80, 118, 68, 1, 154, 84, 5, 174, 92, 170, 88, 166, 84, 5, 162, 92, 158, 88, 150, 84, 2}};
        data.pathLst = vec![vec![11, 13, 0, 110, 68, 5, 126, 76, 134, 80, 138, 68, 5, 142, 76, 146, 80, 118, 68, 1, 154, 84, 5, 174, 92, 170, 88, 166, 84, 5, 162, 92, 158, 88, 150, 84, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_downArrow()
    fn get_downArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 50000, 100000, 0, 200000};
        data.avcLst = vec![50000, 50000, 100000, 0, 200000];
        // Java: data.gdLst = new int[]{196609, 50, 1, 28, 196625, 51, 48, 50, 196625, 51, 49, 53, 196609, 28, 61, 50, 196608, 1, 51, 65, 196609, 0, 57, 52, 196608, 2, 51, 73, 196608, 2, 73, 51, 196609, 77, 65, 2, 196608, 69, 85, 51};
        data.gdLst = vec![196609, 50, 1, 28, 196625, 51, 48, 50, 196625, 51, 49, 53, 196609, 28, 61, 50, 196608, 1, 51, 65, 196609, 0, 57, 52, 196608, 2, 51, 73, 196608, 2, 73, 51, 196609, 77, 65, 2, 196608, 69, 85, 51];
        // Java: data.ahLst = new int[]{1, 48, 51, 50, 77, 26, 2, 49, 51, 53, 26, 69};
        data.ahLst = vec![1, 48, 51, 50, 77, 26, 2, 49, 51, 53, 26, 69];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 69, 20, 2, 1, 51, 0, 69};
        data.cxnLst = vec![15, 2, 26, 19, 26, 69, 20, 2, 1, 51, 0, 69];
        // Java: data.rect = new int[]{77, 26, 81, 89};
        data.rect = vec![77, 26, 81, 89];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 69, 1, 77, 69, 1, 77, 26, 1, 81, 26, 1, 81, 69, 1, 0, 69, 1, 2, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 69, 1, 77, 69, 1, 77, 26, 1, 81, 26, 1, 81, 69, 1, 0, 69, 1, 2, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_upArrow()
    fn get_upArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 50000, 100000, 0, 200000};
        data.avcLst = vec![50000, 50000, 100000, 0, 200000];
        // Java: data.gdLst = new int[]{196609, 50, 1, 28, 196625, 51, 48, 50, 196625, 51, 49, 53, 196609, 28, 61, 50, 196608, 1, 51, 65, 196609, 0, 57, 52, 196608, 2, 51, 73, 196608, 2, 73, 51, 196609, 77, 65, 2, 196608, 65, 51, 85};
        data.gdLst = vec![196609, 50, 1, 28, 196625, 51, 48, 50, 196625, 51, 49, 53, 196609, 28, 61, 50, 196608, 1, 51, 65, 196609, 0, 57, 52, 196608, 2, 51, 73, 196608, 2, 73, 51, 196609, 77, 65, 2, 196608, 65, 51, 85];
        // Java: data.ahLst = new int[]{1, 48, 51, 50, 77, 26, 2, 49, 51, 53, 26, 69};
        data.ahLst = vec![1, 48, 51, 50, 77, 26, 2, 49, 51, 53, 26, 69];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 69, 20, 2, 1, 51, 0, 69};
        data.cxnLst = vec![15, 2, 26, 19, 26, 69, 20, 2, 1, 51, 0, 69];
        // Java: data.rect = new int[]{77, 89, 81, 1};
        data.rect = vec![77, 89, 81, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 65, 1, 2, 26, 1, 0, 65, 1, 81, 65, 1, 81, 1, 1, 77, 1, 1, 77, 65, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 65, 1, 2, 26, 1, 0, 65, 1, 81, 65, 1, 81, 1, 1, 77, 1, 1, 77, 65, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_downArrowCallout()
    fn get_downArrowCallout() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 25000, 25000, 64977, 50000, 0, 2, 1, 100000, 200000};
        data.avcLst = vec![25000, 25000, 25000, 64977, 50000, 0, 2, 1, 100000, 200000];
        // Java: data.gdLst = new int[]{196609, 52, 0, 28, 196625, 53, 49, 58, 196609, 62, 54, 55, 196625, 53, 48, 66, 196609, 56, 1, 28, 196625, 53, 50, 74, 196609, 78, 28, 1, 196608, 56, 53, 82, 196625, 53, 51, 86, 196609, 28, 62, 56, 196609, 28, 70, 57, 196608, 2, 53, 94, 196608, 2, 53, 98, 196608, 2, 98, 53, 196608, 2, 94, 53, 196609, 28, 78, 56, 196608, 1, 53, 118, 196609, 1, 90, 56, 196609, 126, 55, 54};
        data.gdLst = vec![196609, 52, 0, 28, 196625, 53, 49, 58, 196609, 62, 54, 55, 196625, 53, 48, 66, 196609, 56, 1, 28, 196625, 53, 50, 74, 196609, 78, 28, 1, 196608, 56, 53, 82, 196625, 53, 51, 86, 196609, 28, 62, 56, 196609, 28, 70, 57, 196608, 2, 53, 94, 196608, 2, 53, 98, 196608, 2, 98, 53, 196608, 2, 94, 53, 196609, 28, 78, 56, 196608, 1, 53, 118, 196609, 1, 90, 56, 196609, 126, 55, 54];
        // Java: data.ahLst = new int[]{1, 48, 53, 66, 106, 122, 1, 49, 53, 58, 102, 1, 2, 50, 53, 74, 0, 122, 2, 51, 53, 86, 26, 126};
        data.ahLst = vec![1, 48, 53, 66, 106, 122, 1, 49, 53, 58, 102, 1, 2, 50, 53, 74, 0, 122, 2, 51, 53, 86, 26, 126];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 130, 20, 2, 1, 53, 0, 130};
        data.cxnLst = vec![15, 2, 26, 19, 26, 130, 20, 2, 1, 53, 0, 130];
        // Java: data.rect = new int[]{26, 26, 0, 126};
        data.rect = vec![26, 26, 0, 126];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 126, 1, 110, 126, 1, 110, 122, 1, 114, 122, 1, 2, 1, 1, 102, 122, 1, 106, 122, 1, 106, 126, 1, 26, 126, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 126, 1, 110, 126, 1, 110, 122, 1, 114, 122, 1, 2, 1, 1, 102, 122, 1, 106, 122, 1, 106, 126, 1, 26, 126, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_ellipse()
    fn get_ellipse() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{2700000, 0};
        data.avcLst = vec![2700000, 0];
        // Java: data.gdLst = new int[]{131093, 2, 48, 131092, 3, 48, 196608, 2, 49, 50, 196608, 2, 50, 49, 196608, 3, 49, 53, 196608, 3, 53, 49};
        data.gdLst = vec![131093, 2, 48, 131092, 3, 48, 196608, 2, 49, 50, 196608, 2, 50, 49, 196608, 3, 49, 53, 196608, 3, 53, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 15, 56, 64, 19, 26, 3, 20, 56, 68, 20, 2, 1, 20, 60, 68, 49, 0, 3, 15, 60, 64};
        data.cxnLst = vec![15, 2, 26, 15, 56, 64, 19, 26, 3, 20, 56, 68, 20, 2, 1, 20, 60, 68, 49, 0, 3, 15, 60, 64];
        // Java: data.rect = new int[]{56, 64, 60, 68};
        data.rect = vec![56, 64, 60, 68];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_ellipseRibbon()
    fn get_ellipseRibbon() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 50000, 12500, 0, 100000, 25000, 75000, 1, 2, 200000, 4, 14, 16};
        data.avcLst = vec![25000, 50000, 12500, 0, 100000, 25000, 75000, 1, 2, 200000, 4, 14, 16];
        // Java: data.gdLst = new int[]{196625, 51, 48, 52, 196625, 53, 49, 54, 196608, 52, 51, 61, 196609, 69, 55, 56, 196608, 61, 51, 73, 131077, 51, 77, 196625, 81, 50, 61, 196609, 0, 65, 57, 196608, 2, 51, 88, 196608, 92, 36, 51, 196608, 0, 51, 96, 196608, 0, 51, 92, 196608, 0, 51, 36, 196609, 1, 84, 52, 196609, 58, 112, 0, 196609, 96, 96, 0, 196608, 96, 51, 120, 196609, 116, 124, 55, 196609, 96, 55, 56, 196609, 116, 132, 55, 196608, 0, 51, 132, 196609, 1, 61, 52, 196608, 144, 51, 112, 196609, 92, 92, 0, 196608, 92, 51, 152, 196609, 116, 156, 55, 196608, 160, 148, 51, 196608, 112, 148, 164, 196608, 168, 112, 51, 196608, 172, 148, 51, 196608, 1, 51, 144, 196609, 112, 59, 60, 196610, 184, 180, 56, 196608, 160, 180, 51, 196608, 164, 180, 51, 196609, 92, 55, 56, 196609, 116, 200, 55, 196608, 204, 180, 51, 196608, 0, 51, 200, 196608, 176, 180, 51, 196608, 128, 148, 51, 196608, 144, 144, 220, 196608, 1, 51, 112};
        data.gdLst = vec![196625, 51, 48, 52, 196625, 53, 49, 54, 196608, 52, 51, 61, 196609, 69, 55, 56, 196608, 61, 51, 73, 131077, 51, 77, 196625, 81, 50, 61, 196609, 0, 65, 57, 196608, 2, 51, 88, 196608, 92, 36, 51, 196608, 0, 51, 96, 196608, 0, 51, 92, 196608, 0, 51, 36, 196609, 1, 84, 52, 196609, 58, 112, 0, 196609, 96, 96, 0, 196608, 96, 51, 120, 196609, 116, 124, 55, 196609, 96, 55, 56, 196609, 116, 132, 55, 196608, 0, 51, 132, 196609, 1, 61, 52, 196608, 144, 51, 112, 196609, 92, 92, 0, 196608, 92, 51, 152, 196609, 116, 156, 55, 196608, 160, 148, 51, 196608, 112, 148, 164, 196608, 168, 112, 51, 196608, 172, 148, 51, 196608, 1, 51, 144, 196609, 112, 59, 60, 196610, 184, 180, 56, 196608, 160, 180, 51, 196608, 164, 180, 51, 196609, 92, 55, 56, 196609, 116, 200, 55, 196608, 204, 180, 51, 196608, 0, 51, 200, 196608, 176, 180, 51, 196608, 128, 148, 51, 196608, 144, 144, 220, 196608, 1, 51, 112];
        // Java: data.ahLst = new int[]{2, 48, 51, 52, 2, 144, 1, 49, 53, 54, 92, 1, 2, 50, 81, 61, 26, 228};
        data.ahLst = vec![2, 48, 51, 52, 2, 144, 1, 49, 53, 54, 92, 1, 2, 50, 81, 61, 26, 228];
        // Java: data.cxnLst = new int[]{15, 2, 144, 19, 36, 188, 20, 2, 1, 51, 108, 188};
        data.cxnLst = vec![15, 2, 144, 19, 36, 188, 20, 2, 1, 51, 108, 188];
        // Java: data.rect = new int[]{92, 144, 104, 196};
        data.rect = vec![92, 144, 104, 196];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 4, 132, 136, 96, 128, 1, 92, 164, 4, 2, 176, 104, 164, 1, 100, 128, 4, 140, 136, 0, 26, 1, 108, 188, 1, 0, 180, 4, 212, 208, 104, 192, 1, 104, 196, 4, 2, 216, 92, 196, 1, 92, 192, 4, 200, 208, 26, 180, 1, 36, 188, 2}, {7, 12, 0, 96, 220, 1, 96, 128, 1, 92, 164, 4, 2, 176, 104, 164, 1, 100, 128, 1, 100, 220, 4, 2, 224, 96, 220, 2}, {10, 13, 0, 26, 26, 4, 132, 136, 96, 128, 1, 92, 164, 4, 2, 176, 104, 164, 1, 100, 128, 4, 140, 136, 0, 26, 1, 108, 188, 1, 0, 180, 4, 212, 208, 104, 192, 1, 104, 196, 4, 2, 216, 92, 196, 1, 92, 192, 4, 200, 208, 26, 180, 1, 36, 188, 2, 0, 92, 192, 1, 92, 164, 0, 104, 164, 1, 104, 192, 0, 96, 128, 1, 96, 220, 0, 100, 220, 1, 100, 128}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 4, 132, 136, 96, 128, 1, 92, 164, 4, 2, 176, 104, 164, 1, 100, 128, 4, 140, 136, 0, 26, 1, 108, 188, 1, 0, 180, 4, 212, 208, 104, 192, 1, 104, 196, 4, 2, 216, 92, 196, 1, 92, 192, 4, 200, 208, 26, 180, 1, 36, 188, 2], vec![7, 12, 0, 96, 220, 1, 96, 128, 1, 92, 164, 4, 2, 176, 104, 164, 1, 100, 128, 1, 100, 220, 4, 2, 224, 96, 220, 2], vec![10, 13, 0, 26, 26, 4, 132, 136, 96, 128, 1, 92, 164, 4, 2, 176, 104, 164, 1, 100, 128, 4, 140, 136, 0, 26, 1, 108, 188, 1, 0, 180, 4, 212, 208, 104, 192, 1, 104, 196, 4, 2, 216, 92, 196, 1, 92, 192, 4, 200, 208, 26, 180, 1, 36, 188, 2, 0, 92, 192, 1, 92, 164, 0, 104, 164, 1, 104, 192, 0, 96, 128, 1, 96, 220, 0, 100, 220, 1, 100, 128]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_ellipseRibbon2()
    fn get_ellipseRibbon2() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 50000, 12500, 0, 100000, 25000, 75000, 1, 2, 200000, 4, 14, 16};
        data.avcLst = vec![25000, 50000, 12500, 0, 100000, 25000, 75000, 1, 2, 200000, 4, 14, 16];
        // Java: data.gdLst = new int[]{196625, 51, 48, 52, 196625, 53, 49, 54, 196608, 52, 51, 61, 196609, 69, 55, 56, 196608, 61, 51, 73, 131077, 51, 77, 196625, 81, 50, 61, 196609, 0, 65, 57, 196608, 2, 51, 88, 196608, 92, 36, 51, 196608, 0, 51, 96, 196608, 0, 51, 92, 196608, 0, 51, 36, 196609, 1, 84, 52, 196609, 58, 112, 0, 196609, 96, 96, 0, 196608, 96, 51, 120, 196609, 116, 124, 55, 196608, 1, 51, 128, 196609, 96, 55, 56, 196609, 116, 136, 55, 196608, 1, 51, 140, 196608, 0, 51, 136, 196609, 1, 61, 52, 196608, 152, 51, 112, 196609, 92, 92, 0, 196608, 92, 51, 160, 196609, 116, 164, 55, 196608, 168, 156, 51, 196608, 1, 51, 172, 196608, 112, 156, 172, 196608, 180, 112, 51, 196608, 184, 156, 51, 196608, 1, 51, 188, 196608, 1, 51, 152, 196609, 112, 59, 60, 196610, 200, 196, 56, 196608, 1, 51, 204, 196608, 168, 196, 51, 196608, 1, 51, 212, 196608, 172, 196, 51, 196608, 1, 51, 220, 196609, 92, 55, 56, 196609, 116, 228, 55, 196608, 232, 196, 51, 196608, 1, 51, 236, 196608, 0, 51, 228, 196608, 188, 196, 51, 196608, 1, 51, 248, 196608, 128, 156, 51, 196608, 1, 51, 256, 196608, 152, 152, 256, 196608, 1, 51, 264};
        data.gdLst = vec![196625, 51, 48, 52, 196625, 53, 49, 54, 196608, 52, 51, 61, 196609, 69, 55, 56, 196608, 61, 51, 73, 131077, 51, 77, 196625, 81, 50, 61, 196609, 0, 65, 57, 196608, 2, 51, 88, 196608, 92, 36, 51, 196608, 0, 51, 96, 196608, 0, 51, 92, 196608, 0, 51, 36, 196609, 1, 84, 52, 196609, 58, 112, 0, 196609, 96, 96, 0, 196608, 96, 51, 120, 196609, 116, 124, 55, 196608, 1, 51, 128, 196609, 96, 55, 56, 196609, 116, 136, 55, 196608, 1, 51, 140, 196608, 0, 51, 136, 196609, 1, 61, 52, 196608, 152, 51, 112, 196609, 92, 92, 0, 196608, 92, 51, 160, 196609, 116, 164, 55, 196608, 168, 156, 51, 196608, 1, 51, 172, 196608, 112, 156, 172, 196608, 180, 112, 51, 196608, 184, 156, 51, 196608, 1, 51, 188, 196608, 1, 51, 152, 196609, 112, 59, 60, 196610, 200, 196, 56, 196608, 1, 51, 204, 196608, 168, 196, 51, 196608, 1, 51, 212, 196608, 172, 196, 51, 196608, 1, 51, 220, 196609, 92, 55, 56, 196609, 116, 228, 55, 196608, 232, 196, 51, 196608, 1, 51, 236, 196608, 0, 51, 228, 196608, 188, 196, 51, 196608, 1, 51, 248, 196608, 128, 156, 51, 196608, 1, 51, 256, 196608, 152, 152, 256, 196608, 1, 51, 264];
        // Java: data.ahLst = new int[]{2, 48, 51, 52, 2, 196, 1, 49, 53, 52, 92, 26, 2, 50, 81, 61, 26, 112};
        data.ahLst = vec![2, 48, 51, 52, 2, 196, 1, 49, 53, 52, 92, 26, 2, 50, 81, 61, 26, 112];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 36, 208, 20, 2, 196, 51, 108, 208};
        data.cxnLst = vec![15, 2, 26, 19, 36, 208, 20, 2, 196, 51, 108, 208];
        // Java: data.rect = new int[]{92, 224, 104, 196};
        data.rect = vec![92, 224, 104, 196];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 1, 4, 136, 144, 96, 132, 1, 92, 176, 4, 2, 192, 104, 176, 1, 100, 132, 4, 148, 144, 0, 1, 1, 108, 208, 1, 0, 152, 4, 244, 240, 104, 216, 1, 104, 224, 4, 2, 252, 92, 224, 1, 92, 216, 4, 228, 240, 26, 152, 1, 36, 208, 2}, {7, 12, 0, 96, 260, 1, 96, 132, 1, 92, 176, 4, 2, 192, 104, 176, 1, 100, 132, 1, 100, 260, 4, 2, 268, 96, 260, 2}, {10, 13, 0, 26, 1, 1, 36, 208, 1, 26, 152, 4, 228, 240, 92, 216, 1, 92, 224, 4, 2, 252, 104, 224, 1, 104, 216, 4, 244, 240, 0, 152, 1, 108, 208, 1, 0, 1, 4, 148, 144, 100, 132, 1, 104, 176, 4, 2, 192, 92, 176, 1, 96, 132, 4, 136, 144, 26, 1, 2, 0, 92, 176, 1, 92, 216, 0, 104, 216, 1, 104, 176, 0, 96, 260, 1, 96, 132, 0, 100, 132, 1, 100, 260}};
        data.pathLst = vec![vec![11, 12, 0, 26, 1, 4, 136, 144, 96, 132, 1, 92, 176, 4, 2, 192, 104, 176, 1, 100, 132, 4, 148, 144, 0, 1, 1, 108, 208, 1, 0, 152, 4, 244, 240, 104, 216, 1, 104, 224, 4, 2, 252, 92, 224, 1, 92, 216, 4, 228, 240, 26, 152, 1, 36, 208, 2], vec![7, 12, 0, 96, 260, 1, 96, 132, 1, 92, 176, 4, 2, 192, 104, 176, 1, 100, 132, 1, 100, 260, 4, 2, 268, 96, 260, 2], vec![10, 13, 0, 26, 1, 1, 36, 208, 1, 26, 152, 4, 228, 240, 92, 216, 1, 92, 224, 4, 2, 252, 104, 224, 1, 104, 216, 4, 244, 240, 0, 152, 1, 108, 208, 1, 0, 1, 4, 148, 144, 100, 132, 1, 104, 176, 4, 2, 192, 92, 176, 1, 96, 132, 4, 136, 144, 26, 1, 2, 0, 92, 176, 1, 92, 216, 0, 104, 216, 1, 104, 176, 0, 96, 260, 1, 96, 132, 0, 100, 132, 1, 100, 260]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartAlternateProcess()
    fn get_flowChartAlternateProcess() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{0, 29289, 100000};
        data.avcLst = vec![0, 29289, 100000];
        // Java: data.gdLst = new int[]{196608, 0, 48, 31, 196608, 1, 48, 31, 196609, 31, 49, 50, 196608, 0, 48, 59, 196608, 1, 48, 59};
        data.gdLst = vec![196608, 0, 48, 31, 196608, 1, 48, 31, 196609, 31, 49, 50, 196608, 0, 48, 59, 196608, 1, 48, 59];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 48, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 48, 0, 3];
        // Java: data.rect = new int[]{59, 59, 63, 67};
        data.rect = vec![59, 59, 63, 67];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 31, 3, 31, 31, 19, 20, 1, 51, 26, 3, 31, 31, 15, 20, 1, 0, 55, 3, 31, 31, 48, 20, 1, 31, 1, 3, 31, 31, 20, 20, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 31, 3, 31, 31, 19, 20, 1, 51, 26, 3, 31, 31, 15, 20, 1, 0, 55, 3, 31, 31, 48, 20, 1, 31, 1, 3, 31, 31, 20, 20, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartCollate()
    fn get_flowChartCollate() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 4};
        data.avcLst = vec![3, 4];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 1, 48, 49};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 1, 48, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 15, 2, 3, 20, 2, 1};
        data.cxnLst = vec![15, 2, 26, 15, 2, 3, 20, 2, 1];
        // Java: data.rect = new int[]{33, 22, 50, 54};
        data.rect = vec![33, 22, 50, 54];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 2, 3, 1, 0, 1, 1, 26, 1, 1, 2, 3, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 2, 3, 1, 0, 1, 1, 26, 1, 1, 2, 3, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartConnector()
    fn get_flowChartConnector() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{2700000, 0};
        data.avcLst = vec![2700000, 0];
        // Java: data.gdLst = new int[]{131093, 2, 48, 131092, 3, 48, 196608, 2, 49, 50, 196608, 2, 50, 49, 196608, 3, 49, 53, 196608, 3, 53, 49};
        data.gdLst = vec![131093, 2, 48, 131092, 3, 48, 196608, 2, 49, 50, 196608, 2, 50, 49, 196608, 3, 49, 53, 196608, 3, 53, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 15, 56, 64, 19, 26, 3, 20, 56, 68, 20, 2, 1, 20, 60, 68, 49, 0, 3, 15, 60, 64};
        data.cxnLst = vec![15, 2, 26, 15, 56, 64, 19, 26, 3, 20, 56, 68, 20, 2, 1, 20, 60, 68, 49, 0, 3, 15, 60, 64];
        // Java: data.rect = new int[]{56, 64, 60, 68};
        data.rect = vec![56, 64, 60, 68];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartDecision()
    fn get_flowChartDecision() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 4, 0};
        data.avcLst = vec![3, 4, 0];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 1, 48, 49};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 1, 48, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 50, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 50, 0, 3];
        // Java: data.rect = new int[]{33, 22, 51, 55};
        data.rect = vec![33, 22, 51, 55];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 1, 2, 26, 1, 0, 3, 1, 2, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 1, 2, 26, 1, 0, 3, 1, 2, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartDelay()
    fn get_flowChartDelay() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{2700000, 0};
        data.avcLst = vec![2700000, 0];
        // Java: data.gdLst = new int[]{131093, 2, 48, 131092, 3, 48, 196608, 2, 50, 49, 196608, 3, 49, 53, 196608, 3, 53, 49};
        data.gdLst = vec![131093, 2, 48, 131092, 3, 48, 196608, 2, 50, 49, 196608, 3, 49, 53, 196608, 3, 53, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3];
        // Java: data.rect = new int[]{26, 60, 56, 64};
        data.rect = vec![26, 60, 56, 64];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 2, 26, 3, 2, 3, 15, 19, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 2, 26, 3, 2, 3, 15, 19, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartDisplay()
    fn get_flowChartDisplay() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{5, 6, 1, 0};
        data.avcLst = vec![5, 6, 1, 0];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 0, 50, 49, 196609, 0, 48, 49};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 0, 50, 49, 196609, 0, 48, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 51, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 51, 0, 3];
        // Java: data.rect = new int[]{35, 26, 52, 1};
        data.rect = vec![35, 26, 52, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 1, 56, 26, 1, 60, 26, 3, 56, 3, 15, 19, 1, 56, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 1, 56, 26, 1, 60, 26, 3, 56, 3, 15, 19, 1, 56, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartDocument()
    fn get_flowChartDocument() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{17322, 21600, 20172, 10800, 23922, 0};
        data.avcLst = vec![17322, 21600, 20172, 10800, 23922, 0];
        // Java: data.gdLst = new int[]{196609, 1, 48, 49, 196609, 1, 50, 49, 196609, 0, 51, 49, 196609, 1, 52, 49};
        data.gdLst = vec![196609, 1, 48, 49, 196609, 1, 50, 49, 196609, 0, 51, 49, 196609, 1, 52, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 58, 53, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 58, 53, 0, 3];
        // Java: data.rect = new int[]{26, 26, 0, 54};
        data.rect = vec![26, 26, 0, 54];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 54, 5, 62, 54, 62, 66, 53, 58, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 54, 5, 62, 54, 62, 66, 53, 58, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartExtract()
    fn get_flowChartExtract() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 4, 0};
        data.avcLst = vec![3, 4, 0];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49};
        data.gdLst = vec![196609, 0, 48, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 33, 3, 20, 2, 1, 50, 51, 3};
        data.cxnLst = vec![15, 2, 26, 19, 33, 3, 20, 2, 1, 50, 51, 3];
        // Java: data.rect = new int[]{33, 3, 51, 1};
        data.rect = vec![33, 3, 51, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 1, 1, 2, 26, 1, 0, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 1, 1, 2, 26, 1, 0, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartInputOutput()
    fn get_flowChartInputOutput() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{2, 5, 3, 4, 9, 10, 1, 0};
        data.avcLst = vec![2, 5, 3, 4, 9, 10, 1, 0];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 0, 50, 49, 196609, 0, 51, 49, 196609, 0, 52, 53, 196609, 0, 54, 49, 196609, 0, 51, 49};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 0, 50, 49, 196609, 0, 51, 49, 196609, 0, 52, 53, 196609, 0, 54, 49, 196609, 0, 51, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 60, 26, 15, 2, 26, 19, 37, 3, 20, 56, 1, 20, 2, 1, 55, 68, 3};
        data.cxnLst = vec![15, 60, 26, 15, 2, 26, 19, 37, 3, 20, 56, 1, 20, 2, 1, 55, 68, 3];
        // Java: data.rect = new int[]{34, 26, 64, 1};
        data.rect = vec![34, 26, 64, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 1, 1, 72, 26, 1, 0, 26, 1, 76, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 1, 1, 72, 26, 1, 0, 26, 1, 76, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartInternalStorage()
    fn get_flowChartInternalStorage() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{1, 5, 0};
        data.avcLst = vec![1, 5, 0];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 1, 48, 49};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 1, 48, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 50, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 50, 0, 3];
        // Java: data.rect = new int[]{36, 25, 0, 1};
        data.rect = vec![36, 25, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}, {10, 13, 0, 36, 26, 1, 36, 1, 0, 26, 25, 1, 0, 25}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2], vec![10, 13, 0, 36, 26, 1, 36, 1, 0, 26, 25, 1, 0, 25]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartMagneticDisk()
    fn get_flowChartMagneticDisk() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{5, 6, 1, 0};
        data.avcLst = vec![5, 6, 1, 0];
        // Java: data.gdLst = new int[]{196609, 1, 48, 49, 196609, 1, 50, 49};
        data.gdLst = vec![196609, 1, 48, 49, 196609, 1, 50, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 38, 15, 2, 26, 19, 26, 3, 20, 2, 1, 51, 0, 3};
        data.cxnLst = vec![15, 2, 38, 15, 2, 26, 19, 26, 3, 20, 2, 1, 51, 0, 3];
        // Java: data.rect = new int[]{26, 38, 0, 52};
        data.rect = vec![26, 38, 0, 52];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 56, 3, 2, 56, 19, 19, 1, 0, 52, 3, 2, 56, 51, 19, 2}, {10, 13, 0, 0, 56, 3, 2, 56, 51, 19}, {10, 13, 0, 26, 56, 3, 2, 56, 19, 19, 1, 0, 52, 3, 2, 56, 51, 19, 2}};
        data.pathLst = vec![vec![11, 12, 0, 26, 56, 3, 2, 56, 19, 19, 1, 0, 52, 3, 2, 56, 51, 19, 2], vec![10, 13, 0, 0, 56, 3, 2, 56, 51, 19], vec![10, 13, 0, 26, 56, 3, 2, 56, 19, 19, 1, 0, 52, 3, 2, 56, 51, 19, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartMagneticDrum()
    fn get_flowChartMagneticDrum() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{2, 3, 1, 6, 5, 0};
        data.avcLst = vec![2, 3, 1, 6, 5, 0];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 0, 50, 51, 196609, 0, 52, 51};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 0, 50, 51, 196609, 0, 52, 51];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 53, 54, 3, 53, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 53, 54, 3, 53, 0, 3];
        // Java: data.rect = new int[]{35, 26, 54, 1};
        data.rect = vec![35, 26, 54, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 58, 26, 1, 62, 26, 3, 58, 3, 15, 19, 1, 58, 1, 3, 58, 3, 20, 19, 2}, {10, 13, 0, 62, 1, 3, 58, 3, 20, 19}, {10, 13, 0, 58, 26, 1, 62, 26, 3, 58, 3, 15, 19, 1, 58, 1, 3, 58, 3, 20, 19, 2}};
        data.pathLst = vec![vec![11, 12, 0, 58, 26, 1, 62, 26, 3, 58, 3, 15, 19, 1, 58, 1, 3, 58, 3, 20, 19, 2], vec![10, 13, 0, 62, 1, 3, 58, 3, 20, 19], vec![10, 13, 0, 58, 26, 1, 62, 26, 3, 58, 3, 15, 19, 1, 58, 1, 3, 58, 3, 20, 19, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartMagneticTape()
    fn get_flowChartMagneticTape() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{2700000, 0};
        data.avcLst = vec![2700000, 0];
        // Java: data.gdLst = new int[]{131093, 2, 48, 131092, 3, 48, 196608, 2, 49, 50, 196608, 2, 50, 49, 196608, 3, 49, 53, 196608, 3, 53, 49, 131091, 0, 1};
        data.gdLst = vec![131093, 2, 48, 131092, 3, 48, 196608, 2, 49, 50, 196608, 2, 50, 49, 196608, 3, 49, 53, 196608, 3, 53, 49, 131091, 0, 1];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3];
        // Java: data.rect = new int[]{56, 64, 60, 68};
        data.rect = vec![56, 64, 60, 68];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 2, 1, 3, 2, 3, 20, 20, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 72, 1, 0, 68, 1, 0, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 2, 1, 3, 2, 3, 20, 20, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 72, 1, 0, 68, 1, 0, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartManualInput()
    fn get_flowChartManualInput() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{1, 5, 0};
        data.avcLst = vec![1, 5, 0];
        // Java: data.gdLst = new int[]{196609, 1, 48, 49};
        data.gdLst = vec![196609, 1, 48, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 45, 19, 26, 3, 20, 2, 1, 50, 0, 3};
        data.cxnLst = vec![15, 2, 45, 19, 26, 3, 20, 2, 1, 50, 0, 3];
        // Java: data.rect = new int[]{26, 23, 0, 1};
        data.rect = vec![26, 23, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 51, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 51, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartManualOperation()
    fn get_flowChartManualOperation() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{4, 5, 9, 10, 1, 0};
        data.avcLst = vec![4, 5, 9, 10, 1, 0];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 0, 50, 51, 196609, 0, 52, 49};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 0, 50, 51, 196609, 0, 52, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 37, 3, 20, 2, 1, 53, 58, 3};
        data.cxnLst = vec![15, 2, 26, 19, 37, 3, 20, 2, 1, 53, 58, 3];
        // Java: data.rect = new int[]{34, 26, 54, 1};
        data.rect = vec![34, 26, 54, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 54, 1, 1, 62, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 54, 1, 1, 62, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartMerge()
    fn get_flowChartMerge() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 4, 0};
        data.avcLst = vec![3, 4, 0];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49};
        data.gdLst = vec![196609, 0, 48, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 33, 3, 20, 2, 1, 50, 51, 3};
        data.cxnLst = vec![15, 2, 26, 19, 33, 3, 20, 2, 1, 50, 51, 3];
        // Java: data.rect = new int[]{33, 26, 51, 3};
        data.rect = vec![33, 26, 51, 3];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 2, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 2, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartMultidocument()
    fn get_flowChartMultidocument() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3675, 21600, 20782, 9298, 12286, 18595, 0, 23542, 18022, 1532, 1815, 20000, 16252, 19298, 16352, 2972, 14392, 20800, 14467};
        data.avcLst = vec![3675, 21600, 20782, 9298, 12286, 18595, 0, 23542, 18022, 1532, 1815, 20000, 16252, 19298, 16352, 2972, 14392, 20800, 14467];
        // Java: data.gdLst = new int[]{196609, 1, 48, 49, 196609, 1, 50, 49, 196609, 0, 51, 49, 196609, 0, 52, 49, 196609, 0, 53, 49, 196609, 0, 54, 49, 196609, 1, 50, 49, 196609, 0, 51, 49, 196609, 1, 55, 49, 196609, 1, 56, 49, 196609, 0, 53, 49, 196609, 1, 48, 49, 196609, 0, 57, 49, 196609, 1, 58, 49, 196609, 0, 59, 49, 196609, 1, 60, 49, 196609, 0, 61, 49, 196609, 1, 62, 49, 196609, 0, 63, 49, 196609, 1, 54, 49, 196609, 0, 49, 49, 196609, 1, 64, 49, 196609, 0, 65, 49, 196609, 1, 66, 49};
        data.gdLst = vec![196609, 1, 48, 49, 196609, 1, 50, 49, 196609, 0, 51, 49, 196609, 0, 52, 49, 196609, 0, 53, 49, 196609, 0, 54, 49, 196609, 1, 50, 49, 196609, 0, 51, 49, 196609, 1, 55, 49, 196609, 1, 56, 49, 196609, 0, 53, 49, 196609, 1, 48, 49, 196609, 0, 57, 49, 196609, 1, 58, 49, 196609, 0, 59, 49, 196609, 1, 60, 49, 196609, 0, 61, 49, 196609, 1, 62, 49, 196609, 0, 63, 49, 196609, 1, 54, 49, 196609, 0, 49, 49, 196609, 1, 64, 49, 196609, 0, 65, 49, 196609, 1, 66, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 79, 26, 19, 26, 3, 20, 75, 71, 54, 0, 3};
        data.cxnLst = vec![15, 79, 26, 19, 26, 3, 20, 75, 71, 54, 0, 3];
        // Java: data.rect = new int[]{26, 67, 83, 71};
        data.rect = vec![26, 67, 83, 71];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 87, 91, 5, 95, 99, 95, 103, 107, 103, 1, 107, 111, 1, 87, 111, 2, 0, 115, 111, 1, 115, 119, 1, 123, 119, 1, 123, 127, 5, 131, 127, 107, 135, 107, 135, 1, 107, 111, 2, 0, 139, 119, 1, 139, 143, 1, 147, 143, 1, 147, 151, 5, 155, 151, 123, 159, 123, 159, 1, 123, 119, 2}, {10, 13, 0, 87, 111, 1, 107, 111, 1, 107, 103, 5, 95, 103, 95, 99, 87, 91, 2, 0, 115, 111, 1, 115, 119, 1, 123, 119, 1, 123, 127, 5, 131, 127, 107, 135, 107, 135, 0, 139, 119, 1, 139, 143, 1, 147, 143, 1, 147, 151, 5, 155, 151, 123, 159, 123, 159}, {10, 12, 0, 87, 91, 5, 95, 99, 95, 103, 107, 103, 1, 107, 135, 5, 107, 135, 131, 127, 123, 127, 1, 123, 159, 5, 123, 159, 155, 151, 147, 151, 1, 147, 143, 1, 139, 143, 1, 139, 119, 1, 115, 119, 1, 115, 111, 1, 87, 111, 2}};
        data.pathLst = vec![vec![11, 12, 0, 87, 91, 5, 95, 99, 95, 103, 107, 103, 1, 107, 111, 1, 87, 111, 2, 0, 115, 111, 1, 115, 119, 1, 123, 119, 1, 123, 127, 5, 131, 127, 107, 135, 107, 135, 1, 107, 111, 2, 0, 139, 119, 1, 139, 143, 1, 147, 143, 1, 147, 151, 5, 155, 151, 123, 159, 123, 159, 1, 123, 119, 2], vec![10, 13, 0, 87, 111, 1, 107, 111, 1, 107, 103, 5, 95, 103, 95, 99, 87, 91, 2, 0, 115, 111, 1, 115, 119, 1, 123, 119, 1, 123, 127, 5, 131, 127, 107, 135, 107, 135, 0, 139, 119, 1, 139, 143, 1, 147, 143, 1, 147, 151, 5, 155, 151, 123, 159, 123, 159], vec![10, 12, 0, 87, 91, 5, 95, 99, 95, 103, 107, 103, 1, 107, 135, 5, 107, 135, 131, 127, 123, 127, 1, 123, 159, 5, 123, 159, 155, 151, 147, 151, 1, 147, 143, 1, 139, 143, 1, 139, 119, 1, 115, 119, 1, 115, 111, 1, 87, 111, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartOfflineStorage()
    fn get_flowChartOfflineStorage() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 4, 2, 5, 0};
        data.avcLst = vec![3, 4, 2, 5, 0];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 0, 50, 51, 196609, 1, 49, 51, 196609, 0, 48, 51};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 0, 50, 51, 196609, 1, 49, 51, 196609, 0, 48, 51];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{52, 53, 3, 20, 2, 1, 19, 33, 3, 15, 2, 26};
        data.cxnLst = vec![52, 53, 3, 20, 2, 1, 19, 33, 3, 15, 2, 26];
        // Java: data.rect = new int[]{33, 26, 53, 3};
        data.rect = vec![33, 26, 53, 3];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 2, 1, 2}, {10, 13, 0, 57, 61, 1, 65, 61}, {10, 13, 0, 26, 26, 1, 0, 26, 1, 2, 1, 2}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 2, 1, 2], vec![10, 13, 0, 57, 61, 1, 65, 61], vec![10, 13, 0, 26, 26, 1, 0, 26, 1, 2, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartOffpageConnector()
    fn get_flowChartOffpageConnector() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{4, 5, 0, 10, 8};
        data.avcLst = vec![4, 5, 0, 10, 8];
        // Java: data.gdLst = new int[]{196609, 1, 48, 49, 196609, 0, 50, 51, 196609, 1, 50, 51, 196609, 0, 51, 51, 196609, 1, 52, 51, 196609, 0, 49, 51, 196609, 1, 51, 51};
        data.gdLst = vec![196609, 1, 48, 49, 196609, 0, 50, 51, 196609, 1, 50, 51, 196609, 0, 51, 51, 196609, 1, 52, 51, 196609, 0, 49, 51, 196609, 1, 51, 51];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 50, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 50, 0, 3];
        // Java: data.rect = new int[]{26, 26, 0, 53};
        data.rect = vec![26, 26, 0, 53];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 57, 61, 1, 65, 61, 1, 65, 69, 1, 73, 77, 1, 57, 69, 2}};
        data.pathLst = vec![vec![11, 13, 0, 57, 61, 1, 65, 61, 1, 65, 69, 1, 73, 77, 1, 57, 69, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartOnlineStorage()
    fn get_flowChartOnlineStorage() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{5, 6, 1, -10800000, 0};
        data.avcLst = vec![5, 6, 1, -10800000, 0];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 0, 50, 49};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 0, 50, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 52, 53, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 52, 53, 3];
        // Java: data.rect = new int[]{35, 26, 53, 1};
        data.rect = vec![35, 26, 53, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 57, 26, 1, 0, 26, 3, 57, 3, 15, 51, 1, 57, 1, 3, 57, 3, 20, 19, 2}};
        data.pathLst = vec![vec![11, 13, 0, 57, 26, 1, 0, 26, 3, 57, 3, 15, 51, 1, 57, 1, 3, 57, 3, 20, 19, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartOr()
    fn get_flowChartOr() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{2700000, 0};
        data.avcLst = vec![2700000, 0];
        // Java: data.gdLst = new int[]{131093, 2, 48, 131092, 3, 48, 196608, 2, 49, 50, 196608, 2, 50, 49, 196608, 3, 49, 53, 196608, 3, 53, 49};
        data.gdLst = vec![131093, 2, 48, 131092, 3, 48, 196608, 2, 49, 50, 196608, 2, 50, 49, 196608, 3, 49, 53, 196608, 3, 53, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 15, 56, 64, 19, 26, 3, 20, 56, 68, 20, 2, 1, 20, 60, 68, 49, 0, 3, 15, 60, 64};
        data.cxnLst = vec![15, 2, 26, 15, 56, 64, 19, 26, 3, 20, 56, 68, 20, 2, 1, 20, 60, 68, 49, 0, 3, 15, 60, 64];
        // Java: data.rect = new int[]{56, 64, 60, 68};
        data.rect = vec![56, 64, 60, 68];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 3, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2}, {10, 13, 0, 2, 26, 1, 2, 1, 0, 26, 3, 1, 0, 3}, {10, 13, 0, 26, 3, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2}};
        data.pathLst = vec![vec![11, 12, 0, 26, 3, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2], vec![10, 13, 0, 2, 26, 1, 2, 1, 0, 26, 3, 1, 0, 3], vec![10, 13, 0, 26, 3, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartPredefinedProcess()
    fn get_flowChartPredefinedProcess() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{7, 8, 1, 0};
        data.avcLst = vec![7, 8, 1, 0];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 0, 50, 49};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 0, 50, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 51, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 51, 0, 3];
        // Java: data.rect = new int[]{36, 26, 52, 1};
        data.rect = vec![36, 26, 52, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 56, 26, 1, 56, 1, 1, 26, 1, 2}, {11, 13, 0, 56, 26, 1, 52, 26, 1, 52, 1, 1, 56, 1, 2}, {11, 13, 0, 52, 26, 1, 0, 26, 1, 0, 1, 1, 52, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 56, 26, 1, 56, 1, 1, 26, 1, 2], vec![11, 13, 0, 56, 26, 1, 52, 26, 1, 52, 1, 1, 56, 1, 2], vec![11, 13, 0, 52, 26, 1, 0, 26, 1, 0, 1, 1, 52, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartPreparation()
    fn get_flowChartPreparation() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{4, 5, 0, 10, 2, 8};
        data.avcLst = vec![4, 5, 0, 10, 2, 8];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 0, 50, 51, 196609, 1, 49, 51, 196609, 0, 52, 51, 196609, 1, 50, 51, 196609, 0, 53, 51, 196609, 0, 51, 51, 196609, 1, 51, 51};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 0, 50, 51, 196609, 1, 49, 51, 196609, 0, 52, 51, 196609, 1, 50, 51, 196609, 0, 53, 51, 196609, 0, 51, 51, 196609, 1, 51, 51];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 50, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 50, 0, 3];
        // Java: data.rect = new int[]{34, 26, 54, 1};
        data.rect = vec![34, 26, 54, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 58, 62, 1, 66, 70, 1, 74, 70, 1, 78, 62, 1, 74, 82, 1, 66, 82, 2}};
        data.pathLst = vec![vec![11, 13, 0, 58, 62, 1, 66, 70, 1, 74, 70, 1, 78, 62, 1, 74, 82, 1, 66, 82, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartProcess()
    fn get_flowChartProcess() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{0};
        data.avcLst = vec![0];
        // Java: data.gdLst = new int[0];
        data.gdLst = vec![];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 48, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 48, 0, 3];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartPunchedCard()
    fn get_flowChartPunchedCard() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{1, 5, 0};
        data.avcLst = vec![1, 5, 0];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 1, 48, 49};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 1, 48, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 50, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 50, 0, 3];
        // Java: data.rect = new int[]{26, 23, 0, 1};
        data.rect = vec![26, 23, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 55, 1, 51, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 55, 1, 51, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartPunchedTape()
    fn get_flowChartPunchedTape() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{9, 10, 4, 5, 2, 20, 18, -10800000, 0};
        data.avcLst = vec![9, 10, 4, 5, 2, 20, 18, -10800000, 0];
        // Java: data.gdLst = new int[]{196609, 1, 48, 49, 196609, 1, 50, 51, 196609, 1, 52, 53, 196609, 0, 51, 53, 196609, 1, 54, 53};
        data.gdLst = vec![196609, 1, 48, 49, 196609, 1, 50, 51, 196609, 1, 52, 53, 196609, 0, 51, 53, 196609, 1, 54, 53];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 45, 19, 26, 3, 20, 2, 57, 56, 0, 3};
        data.cxnLst = vec![15, 2, 45, 19, 26, 3, 20, 2, 57, 56, 0, 3];
        // Java: data.rect = new int[]{26, 23, 0, 61};
        data.rect = vec![26, 23, 0, 61];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 65, 3, 69, 65, 19, 55, 3, 69, 65, 19, 19, 1, 0, 73, 3, 69, 65, 56, 55, 3, 69, 65, 56, 19, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 65, 3, 69, 65, 19, 55, 3, 69, 65, 19, 19, 1, 0, 73, 3, 69, 65, 56, 55, 3, 69, 65, 56, 19, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartSort()
    fn get_flowChartSort() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{3, 4, 0, 2, 1};
        data.avcLst = vec![3, 4, 0, 2, 1];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 1, 48, 49, 196609, 0, 50, 51, 196609, 1, 52, 51, 196609, 0, 52, 51, 196609, 1, 50, 51, 196609, 0, 51, 51, 196609, 1, 51, 51};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 1, 48, 49, 196609, 0, 50, 51, 196609, 1, 52, 51, 196609, 0, 52, 51, 196609, 1, 50, 51, 196609, 0, 51, 51, 196609, 1, 51, 51];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 50, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 50, 0, 3];
        // Java: data.rect = new int[]{33, 22, 53, 57};
        data.rect = vec![33, 22, 53, 57];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 61, 65, 1, 69, 73, 1, 77, 65, 1, 69, 81, 2}, {10, 13, 0, 61, 65, 1, 77, 65}, {10, 13, 0, 61, 65, 1, 69, 73, 1, 77, 65, 1, 69, 81, 2}};
        data.pathLst = vec![vec![11, 12, 0, 61, 65, 1, 69, 73, 1, 77, 65, 1, 69, 81, 2], vec![10, 13, 0, 61, 65, 1, 77, 65], vec![10, 13, 0, 61, 65, 1, 69, 73, 1, 77, 65, 1, 69, 81, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartSummingJunction()
    fn get_flowChartSummingJunction() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{2700000, 0};
        data.avcLst = vec![2700000, 0];
        // Java: data.gdLst = new int[]{131093, 2, 48, 131092, 3, 48, 196608, 2, 49, 50, 196608, 2, 50, 49, 196608, 3, 49, 53, 196608, 3, 53, 49};
        data.gdLst = vec![131093, 2, 48, 131092, 3, 48, 196608, 2, 49, 50, 196608, 2, 50, 49, 196608, 3, 49, 53, 196608, 3, 53, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 15, 56, 64, 19, 26, 3, 20, 56, 68, 20, 2, 1, 20, 60, 68, 49, 0, 3, 15, 60, 64};
        data.cxnLst = vec![15, 2, 26, 15, 56, 64, 19, 26, 3, 20, 56, 68, 20, 2, 1, 20, 60, 68, 49, 0, 3, 15, 60, 64];
        // Java: data.rect = new int[]{56, 64, 60, 68};
        data.rect = vec![56, 64, 60, 68];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 3, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2}, {10, 13, 0, 56, 64, 1, 60, 68, 0, 60, 64, 1, 56, 68}, {10, 13, 0, 26, 3, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2}};
        data.pathLst = vec![vec![11, 12, 0, 26, 3, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2], vec![10, 13, 0, 56, 64, 1, 60, 68, 0, 60, 64, 1, 56, 68], vec![10, 13, 0, 26, 3, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_flowChartTerminator()
    fn get_flowChartTerminator() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{1018, 21600, 20582, 3163, 18437, 3475, 18125, 10800, 21600, 0};
        data.avcLst = vec![1018, 21600, 20582, 3163, 18437, 3475, 18125, 10800, 21600, 0];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 0, 50, 49, 196609, 1, 51, 49, 196609, 1, 52, 49, 196609, 0, 53, 49, 196609, 0, 54, 49, 196609, 1, 55, 56};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 0, 50, 49, 196609, 1, 51, 49, 196609, 1, 52, 49, 196609, 0, 53, 49, 196609, 0, 54, 49, 196609, 1, 55, 56];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 57, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 57, 0, 3];
        // Java: data.rect = new int[]{58, 66, 62, 70};
        data.rect = vec![58, 66, 62, 70];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 74, 26, 1, 78, 26, 3, 74, 82, 15, 19, 1, 74, 1, 3, 74, 82, 20, 19, 2}};
        data.pathLst = vec![vec![11, 13, 0, 74, 26, 1, 78, 26, 3, 74, 82, 15, 19, 1, 74, 1, 3, 74, 82, 20, 19, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_foldedCorner()
    fn get_foldedCorner() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{16667, 0, 50000, 100000, 1, 5};
        data.avcLst = vec![16667, 0, 50000, 100000, 1, 5];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 54, 51, 196609, 58, 52, 53, 196608, 0, 49, 58, 196608, 66, 62, 49, 196608, 1, 49, 58, 196608, 74, 62, 49};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 54, 51, 196609, 58, 52, 53, 196608, 0, 49, 58, 196608, 66, 62, 49, 196608, 1, 49, 58, 196608, 74, 62, 49];
        // Java: data.ahLst = new int[]{1, 48, 49, 50, 66, 1};
        data.ahLst = vec![1, 48, 49, 50, 66, 1];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3];
        // Java: data.rect = new int[]{26, 26, 0, 74};
        data.rect = vec![26, 26, 0, 74];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 74, 1, 66, 1, 1, 26, 1, 2}, {7, 12, 0, 66, 1, 1, 70, 78, 1, 0, 74, 2}, {10, 13, 0, 66, 1, 1, 70, 78, 1, 0, 74, 1, 66, 1, 1, 26, 1, 1, 26, 26, 1, 0, 26, 1, 0, 74}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 0, 26, 1, 0, 74, 1, 66, 1, 1, 26, 1, 2], vec![7, 12, 0, 66, 1, 1, 70, 78, 1, 0, 74, 2], vec![10, 13, 0, 66, 1, 1, 70, 78, 1, 0, 74, 1, 66, 1, 1, 26, 1, 1, 26, 26, 1, 0, 26, 1, 0, 74]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_frame()
    fn get_frame() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{12500, 0, 50000, 100000};
        data.avcLst = vec![12500, 0, 50000, 100000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 52, 51, 196608, 0, 49, 56, 196608, 1, 49, 56};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 52, 51, 196608, 0, 49, 56, 196608, 1, 49, 56];
        // Java: data.ahLst = new int[]{1, 48, 49, 50, 56, 26};
        data.ahLst = vec![1, 48, 49, 50, 56, 26];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3];
        // Java: data.rect = new int[]{56, 56, 60, 64};
        data.rect = vec![56, 56, 60, 64];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 56, 56, 1, 56, 64, 1, 60, 64, 1, 60, 56, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2, 0, 56, 56, 1, 56, 64, 1, 60, 64, 1, 60, 56, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_funnel()
    fn get_funnel() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{1, 20, 0, 480000, 2, 4, -21600000};
        data.avcLst = vec![1, 20, 0, 480000, 2, 4, -21600000];
        // Java: data.gdLst = new int[]{196609, 28, 48, 49, 196608, 2, 50, 55, 196608, 22, 50, 55, 131093, 2, 51, 131092, 22, 51, 131091, 67, 70, 196609, 73, 52, 48, 196608, 19, 50, 73, 196608, 19, 76, 50, 196608, 19, 50, 76, 196609, 2, 48, 53, 196609, 22, 48, 53, 131093, 22, 80, 131092, 2, 80, 196615, 100, 103, 50, 196609, 2, 22, 106, 131093, 110, 80, 131092, 110, 80, 196608, 2, 114, 50, 196608, 22, 117, 50, 131093, 96, 73, 131092, 92, 73, 196615, 128, 131, 50, 196609, 92, 96, 134, 131093, 138, 73, 131092, 138, 73, 196608, 2, 142, 50, 196608, 1, 50, 96, 196608, 152, 145, 50, 196608, 2, 50, 59, 196609, 19, 52, 48};
        data.gdLst = vec![196609, 28, 48, 49, 196608, 2, 50, 55, 196608, 22, 50, 55, 131093, 2, 51, 131092, 22, 51, 131091, 67, 70, 196609, 73, 52, 48, 196608, 19, 50, 73, 196608, 19, 76, 50, 196608, 19, 50, 76, 196609, 2, 48, 53, 196609, 22, 48, 53, 131093, 22, 80, 131092, 2, 80, 196615, 100, 103, 50, 196609, 2, 22, 106, 131093, 110, 80, 131092, 110, 80, 196608, 2, 114, 50, 196608, 22, 117, 50, 131093, 96, 73, 131092, 92, 73, 196615, 128, 131, 50, 196609, 92, 96, 134, 131093, 138, 73, 131092, 138, 73, 196608, 2, 142, 50, 196608, 1, 50, 96, 196608, 152, 145, 50, 196608, 2, 50, 59, 196609, 19, 52, 48];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[0];
        data.cxnLst = vec![];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 120, 124, 3, 2, 22, 80, 84, 1, 148, 156, 3, 92, 96, 73, 88, 2, 0, 160, 22, 3, 59, 63, 19, 54, 2}};
        data.pathLst = vec![vec![11, 13, 0, 120, 124, 3, 2, 22, 80, 84, 1, 148, 156, 3, 92, 96, 73, 88, 2, 0, 160, 22, 3, 59, 63, 19, 54, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_gear6()
    fn get_gear6() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{15000, 3526, 0, 20000, 5358, 100000, 1, 2, 19800000, 1800000, 9000000, 12600000};
        data.avcLst = vec![15000, 3526, 0, 20000, 5358, 100000, 1, 2, 19800000, 1800000, 9000000, 12600000];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196625, 50, 49, 52, 196609, 28, 60, 53, 196609, 28, 64, 53, 196609, 68, 54, 55, 196609, 72, 54, 55, 196608, 76, 80, 50, 196608, 3, 50, 68, 196608, 2, 50, 68, 196608, 92, 50, 88, 196614, 96, 88, 92, 131091, 100, 84, 196608, 56, 50, 104, 196608, 56, 104, 50, 131093, 92, 107, 131092, 88, 107, 131091, 115, 118, 131093, 88, 121, 131092, 92, 121, 196615, 124, 127, 50, 196609, 92, 88, 130, 131093, 134, 121, 131092, 134, 121, 196608, 2, 138, 50, 196608, 3, 141, 50, 131093, 92, 111, 131092, 88, 111, 131091, 152, 155, 131093, 88, 158, 131092, 92, 158, 196615, 161, 164, 50, 196609, 92, 88, 167, 131093, 171, 158, 131092, 171, 158, 196608, 2, 175, 50, 196608, 3, 178, 50, 196608, 144, 50, 181, 196608, 148, 50, 185, 196615, 189, 193, 50, 131091, 193, 189, 131092, 72, 201, 131093, 72, 201, 196608, 181, 204, 50, 196608, 185, 207, 50, 196608, 144, 50, 204, 196608, 148, 50, 207, 131092, 68, 201, 131093, 68, 201, 196608, 214, 226, 50, 196608, 210, 50, 229, 196608, 222, 226, 50, 196608, 218, 50, 229, 196608, 15, 104, 50, 131093, 92, 248, 131092, 88, 248, 131091, 252, 255, 131093, 88, 258, 131092, 92, 258, 196615, 261, 264, 50, 196609, 92, 88, 267, 131093, 271, 258, 131092, 271, 258, 196608, 2, 275, 50, 196608, 3, 278, 50, 196608, 2, 50, 275, 196608, 281, 50, 72, 196608, 289, 72, 50, 196608, 285, 50, 68, 196608, 121, 50, 258, 196608, 57, 50, 104, 196608, 57, 104, 50, 131093, 92, 309, 131092, 88, 309, 131091, 317, 320, 196608, 1, 50, 185, 131093, 92, 313, 131092, 88, 313, 131091, 330, 333, 196608, 1, 50, 148, 196608, 1, 50, 240, 196608, 1, 50, 232, 65554, 236, 196608, 323, 50, 158, 196608, 20, 104, 50, 131093, 92, 357, 131092, 88, 357, 131091, 361, 364, 196608, 1, 50, 285, 196608, 1, 50, 301, 196608, 58, 104, 50, 131093, 92, 378, 131092, 88, 378, 131091, 382, 385, 196608, 0, 50, 181, 196608, 0, 50, 236, 196608, 0, 50, 244, 196608, 59, 104, 50, 131093, 92, 403, 131092, 88, 403, 131091, 407, 410, 196608, 0, 50, 144, 196608, 0, 50, 244, 196608, 0, 50, 236, 196610, 244, 236, 55, 196610, 240, 232, 55, 196608, 1, 50, 432, 196610, 0, 50, 428};
        data.gdLst = vec![196625, 50, 48, 51, 196625, 50, 49, 52, 196609, 28, 60, 53, 196609, 28, 64, 53, 196609, 68, 54, 55, 196609, 72, 54, 55, 196608, 76, 80, 50, 196608, 3, 50, 68, 196608, 2, 50, 68, 196608, 92, 50, 88, 196614, 96, 88, 92, 131091, 100, 84, 196608, 56, 50, 104, 196608, 56, 104, 50, 131093, 92, 107, 131092, 88, 107, 131091, 115, 118, 131093, 88, 121, 131092, 92, 121, 196615, 124, 127, 50, 196609, 92, 88, 130, 131093, 134, 121, 131092, 134, 121, 196608, 2, 138, 50, 196608, 3, 141, 50, 131093, 92, 111, 131092, 88, 111, 131091, 152, 155, 131093, 88, 158, 131092, 92, 158, 196615, 161, 164, 50, 196609, 92, 88, 167, 131093, 171, 158, 131092, 171, 158, 196608, 2, 175, 50, 196608, 3, 178, 50, 196608, 144, 50, 181, 196608, 148, 50, 185, 196615, 189, 193, 50, 131091, 193, 189, 131092, 72, 201, 131093, 72, 201, 196608, 181, 204, 50, 196608, 185, 207, 50, 196608, 144, 50, 204, 196608, 148, 50, 207, 131092, 68, 201, 131093, 68, 201, 196608, 214, 226, 50, 196608, 210, 50, 229, 196608, 222, 226, 50, 196608, 218, 50, 229, 196608, 15, 104, 50, 131093, 92, 248, 131092, 88, 248, 131091, 252, 255, 131093, 88, 258, 131092, 92, 258, 196615, 261, 264, 50, 196609, 92, 88, 267, 131093, 271, 258, 131092, 271, 258, 196608, 2, 275, 50, 196608, 3, 278, 50, 196608, 2, 50, 275, 196608, 281, 50, 72, 196608, 289, 72, 50, 196608, 285, 50, 68, 196608, 121, 50, 258, 196608, 57, 50, 104, 196608, 57, 104, 50, 131093, 92, 309, 131092, 88, 309, 131091, 317, 320, 196608, 1, 50, 185, 131093, 92, 313, 131092, 88, 313, 131091, 330, 333, 196608, 1, 50, 148, 196608, 1, 50, 240, 196608, 1, 50, 232, 65554, 236, 196608, 323, 50, 158, 196608, 20, 104, 50, 131093, 92, 357, 131092, 88, 357, 131091, 361, 364, 196608, 1, 50, 285, 196608, 1, 50, 301, 196608, 58, 104, 50, 131093, 92, 378, 131092, 88, 378, 131091, 382, 385, 196608, 0, 50, 181, 196608, 0, 50, 236, 196608, 0, 50, 244, 196608, 59, 104, 50, 131093, 92, 403, 131092, 88, 403, 131091, 407, 410, 196608, 0, 50, 144, 196608, 0, 50, 244, 196608, 0, 50, 236, 196610, 244, 236, 55, 196610, 240, 232, 55, 196608, 1, 50, 432, 196610, 0, 50, 428];
        // Java: data.ahLst = new int[]{2, 48, 50, 51, 281, 285, 1, 49, 50, 52, 289, 285};
        data.ahLst = vec![2, 48, 50, 51, 281, 285, 1, 49, 50, 52, 289, 285];
        // Java: data.cxnLst = new int[]{56, 428, 432, 57, 428, 436, 20, 2, 374, 58, 440, 436, 59, 440, 432, 15, 2, 301};
        data.cxnLst = vec![56, 428, 432, 57, 428, 436, 20, 2, 374, 58, 440, 436, 59, 440, 432, 15, 2, 301];
        // Java: data.rect = new int[]{416, 148, 144, 339};
        data.rect = vec![416, 148, 144, 339];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 144, 148, 1, 244, 240, 1, 236, 232, 1, 181, 185, 3, 92, 88, 158, 353, 1, 236, 347, 1, 244, 343, 1, 144, 339, 3, 92, 88, 336, 305, 1, 293, 374, 1, 297, 374, 1, 289, 370, 3, 92, 88, 367, 305, 1, 399, 343, 1, 395, 347, 1, 391, 326, 3, 92, 88, 388, 353, 1, 424, 232, 1, 420, 240, 1, 416, 148, 3, 92, 88, 413, 305, 1, 297, 301, 1, 293, 301, 1, 281, 285, 3, 92, 88, 258, 305, 2}};
        data.pathLst = vec![vec![11, 13, 0, 144, 148, 1, 244, 240, 1, 236, 232, 1, 181, 185, 3, 92, 88, 158, 353, 1, 236, 347, 1, 244, 343, 1, 144, 339, 3, 92, 88, 336, 305, 1, 293, 374, 1, 297, 374, 1, 289, 370, 3, 92, 88, 367, 305, 1, 399, 343, 1, 395, 347, 1, 391, 326, 3, 92, 88, 388, 353, 1, 424, 232, 1, 420, 240, 1, 416, 148, 3, 92, 88, 413, 305, 1, 297, 301, 1, 293, 301, 1, 281, 285, 3, 92, 88, 258, 305, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_gear9()
    fn get_gear9() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{10000, 1763, 0, 20000, 2679, 100000, 1, 2, 18600000, 21000000, 1800000, 4200000, 6600000, 9000000, 11400000, 13800000};
        data.avcLst = vec![10000, 1763, 0, 20000, 2679, 100000, 1, 2, 18600000, 21000000, 1800000, 4200000, 6600000, 9000000, 11400000, 13800000];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196625, 50, 49, 52, 196609, 28, 64, 53, 196609, 28, 68, 53, 196609, 72, 54, 55, 196609, 76, 54, 55, 196608, 80, 84, 50, 196608, 3, 50, 72, 196608, 2, 50, 72, 196608, 96, 50, 92, 196614, 100, 92, 96, 131091, 104, 88, 196608, 56, 50, 108, 196608, 56, 108, 50, 131093, 96, 111, 131092, 92, 111, 131091, 119, 122, 131093, 92, 125, 131092, 96, 125, 196615, 128, 131, 50, 196609, 96, 92, 134, 131093, 138, 125, 131092, 138, 125, 196608, 2, 142, 50, 196608, 3, 145, 50, 131093, 96, 115, 131092, 92, 115, 131091, 156, 159, 131093, 92, 162, 131092, 96, 162, 196615, 165, 168, 50, 196609, 96, 92, 171, 131093, 175, 162, 131092, 175, 162, 196608, 2, 179, 50, 196608, 3, 182, 50, 196608, 148, 50, 185, 196608, 152, 50, 189, 196615, 193, 197, 50, 131091, 197, 193, 131092, 76, 205, 131093, 76, 205, 196608, 185, 208, 50, 196608, 189, 211, 50, 196608, 148, 50, 208, 196608, 152, 50, 211, 131092, 72, 205, 131093, 72, 205, 196608, 218, 230, 50, 196608, 214, 50, 233, 196608, 226, 230, 50, 196608, 222, 50, 233, 196608, 57, 50, 108, 196608, 57, 108, 50, 131093, 96, 252, 131092, 92, 252, 131091, 260, 263, 131093, 92, 266, 131092, 96, 266, 196615, 269, 272, 50, 196609, 96, 92, 275, 131093, 279, 266, 131092, 279, 266, 196608, 2, 283, 50, 196608, 3, 286, 50, 131093, 96, 256, 131092, 92, 256, 131091, 297, 300, 131093, 92, 303, 131092, 96, 303, 196615, 306, 309, 50, 196609, 96, 92, 312, 131093, 316, 303, 131092, 316, 303, 196608, 2, 320, 50, 196608, 3, 323, 50, 196608, 289, 50, 326, 196608, 293, 50, 330, 196615, 334, 338, 50, 131091, 338, 334, 131092, 76, 346, 131093, 76, 346, 196608, 326, 349, 50, 196608, 330, 352, 50, 196608, 289, 50, 349, 196608, 293, 50, 352, 131092, 72, 346, 131093, 72, 346, 196608, 359, 371, 50, 196608, 355, 50, 374, 196608, 367, 371, 50, 196608, 363, 50, 374, 196608, 266, 50, 162, 196608, 58, 50, 108, 196608, 58, 108, 50, 131093, 96, 397, 131092, 92, 397, 131091, 405, 408, 131093, 92, 411, 131092, 96, 411, 196615, 414, 417, 50, 196609, 96, 92, 420, 131093, 424, 411, 131092, 424, 411, 196608, 2, 428, 50, 196608, 3, 431, 50, 131093, 96, 401, 131092, 92, 401, 131091, 442, 445, 131093, 92, 448, 131092, 96, 448, 196615, 451, 454, 50, 196609, 96, 92, 457, 131093, 461, 448, 131092, 461, 448, 196608, 2, 465, 50, 196608, 3, 468, 50, 196608, 434, 50, 471, 196608, 438, 50, 475, 196615, 479, 483, 50, 131091, 483, 479, 131092, 76, 491, 131093, 76, 491, 196608, 471, 494, 50, 196608, 475, 497, 50, 196608, 434, 50, 494, 196608, 438, 50, 497, 131092, 72, 491, 131093, 72, 491, 196608, 504, 516, 50, 196608, 500, 50, 519, 196608, 512, 516, 50, 196608, 508, 50, 519, 196608, 411, 50, 303, 196608, 59, 50, 108, 196608, 59, 108, 50, 131093, 96, 542, 131092, 92, 542, 131091, 550, 553, 131093, 92, 556, 131092, 96, 556, 196615, 559, 562, 50, 196609, 96, 92, 565, 131093, 569, 556, 131092, 569, 556, 196608, 2, 573, 50, 196608, 3, 576, 50, 131093, 96, 546, 131092, 92, 546, 131091, 587, 590, 131093, 92, 593, 131092, 96, 593, 196615, 596, 599, 50, 196609, 96, 92, 602, 131093, 606, 593, 131092, 606, 593, 196608, 2, 610, 50, 196608, 3, 613, 50, 196608, 579, 50, 616, 196608, 583, 50, 620, 196615, 624, 628, 50, 131091, 628, 624, 131092, 76, 636, 131093, 76, 636, 196608, 616, 639, 50, 196608, 620, 642, 50, 196608, 579, 50, 639, 196608, 583, 50, 642, 131092, 72, 636, 131093, 72, 636, 196608, 649, 661, 50, 196608, 645, 50, 664, 196608, 657, 661, 50, 196608, 653, 50, 664, 196608, 556, 50, 448, 196608, 60, 50, 108, 196608, 60, 108, 50, 131093, 96, 687, 131092, 92, 687, 131091, 695, 698, 131093, 96, 691, 131092, 92, 691, 131091, 704, 707, 196608, 0, 50, 579, 196608, 0, 50, 679, 196608, 0, 50, 671, 196608, 701, 50, 593, 196608, 61, 108, 50, 131093, 96, 729, 131092, 92, 729, 131091, 733, 736, 196608, 0, 50, 434, 196608, 0, 50, 534, 196608, 0, 50, 526, 196608, 62, 108, 50, 131093, 96, 754, 131092, 92, 754, 131091, 758, 761, 196608, 0, 50, 289, 196608, 0, 50, 389, 196608, 0, 50, 381, 196608, 63, 108, 50, 131093, 96, 779, 131092, 92, 779, 131091, 783, 786, 196608, 0, 50, 185, 196608, 0, 50, 148, 196608, 0, 50, 248, 196608, 0, 50, 240, 196608, 15, 50, 108, 196608, 15, 108, 50, 131093, 96, 812, 131092, 92, 812, 131091, 816, 819, 131093, 92, 822, 131092, 96, 822, 196615, 825, 828, 50, 196609, 96, 92, 831, 131093, 835, 822, 131092, 835, 822, 196608, 2, 839, 50, 196608, 3, 842, 50, 131093, 96, 808, 131092, 92, 808, 131091, 853, 856, 196608, 2, 50, 839, 196608, 845, 50, 76, 196608, 862, 76, 50, 196608, 849, 50, 72, 196608, 859, 50, 789, 196610, 248, 240, 55, 196610, 244, 236, 55, 196610, 389, 381, 55, 196610, 385, 377, 55, 196610, 534, 526, 55, 196610, 530, 522, 55, 196610, 679, 671, 55, 196610, 675, 667, 55, 196610, 0, 50, 906, 196610, 0, 50, 898, 196610, 0, 50, 890, 196610, 0, 50, 882};
        data.gdLst = vec![196625, 50, 48, 51, 196625, 50, 49, 52, 196609, 28, 64, 53, 196609, 28, 68, 53, 196609, 72, 54, 55, 196609, 76, 54, 55, 196608, 80, 84, 50, 196608, 3, 50, 72, 196608, 2, 50, 72, 196608, 96, 50, 92, 196614, 100, 92, 96, 131091, 104, 88, 196608, 56, 50, 108, 196608, 56, 108, 50, 131093, 96, 111, 131092, 92, 111, 131091, 119, 122, 131093, 92, 125, 131092, 96, 125, 196615, 128, 131, 50, 196609, 96, 92, 134, 131093, 138, 125, 131092, 138, 125, 196608, 2, 142, 50, 196608, 3, 145, 50, 131093, 96, 115, 131092, 92, 115, 131091, 156, 159, 131093, 92, 162, 131092, 96, 162, 196615, 165, 168, 50, 196609, 96, 92, 171, 131093, 175, 162, 131092, 175, 162, 196608, 2, 179, 50, 196608, 3, 182, 50, 196608, 148, 50, 185, 196608, 152, 50, 189, 196615, 193, 197, 50, 131091, 197, 193, 131092, 76, 205, 131093, 76, 205, 196608, 185, 208, 50, 196608, 189, 211, 50, 196608, 148, 50, 208, 196608, 152, 50, 211, 131092, 72, 205, 131093, 72, 205, 196608, 218, 230, 50, 196608, 214, 50, 233, 196608, 226, 230, 50, 196608, 222, 50, 233, 196608, 57, 50, 108, 196608, 57, 108, 50, 131093, 96, 252, 131092, 92, 252, 131091, 260, 263, 131093, 92, 266, 131092, 96, 266, 196615, 269, 272, 50, 196609, 96, 92, 275, 131093, 279, 266, 131092, 279, 266, 196608, 2, 283, 50, 196608, 3, 286, 50, 131093, 96, 256, 131092, 92, 256, 131091, 297, 300, 131093, 92, 303, 131092, 96, 303, 196615, 306, 309, 50, 196609, 96, 92, 312, 131093, 316, 303, 131092, 316, 303, 196608, 2, 320, 50, 196608, 3, 323, 50, 196608, 289, 50, 326, 196608, 293, 50, 330, 196615, 334, 338, 50, 131091, 338, 334, 131092, 76, 346, 131093, 76, 346, 196608, 326, 349, 50, 196608, 330, 352, 50, 196608, 289, 50, 349, 196608, 293, 50, 352, 131092, 72, 346, 131093, 72, 346, 196608, 359, 371, 50, 196608, 355, 50, 374, 196608, 367, 371, 50, 196608, 363, 50, 374, 196608, 266, 50, 162, 196608, 58, 50, 108, 196608, 58, 108, 50, 131093, 96, 397, 131092, 92, 397, 131091, 405, 408, 131093, 92, 411, 131092, 96, 411, 196615, 414, 417, 50, 196609, 96, 92, 420, 131093, 424, 411, 131092, 424, 411, 196608, 2, 428, 50, 196608, 3, 431, 50, 131093, 96, 401, 131092, 92, 401, 131091, 442, 445, 131093, 92, 448, 131092, 96, 448, 196615, 451, 454, 50, 196609, 96, 92, 457, 131093, 461, 448, 131092, 461, 448, 196608, 2, 465, 50, 196608, 3, 468, 50, 196608, 434, 50, 471, 196608, 438, 50, 475, 196615, 479, 483, 50, 131091, 483, 479, 131092, 76, 491, 131093, 76, 491, 196608, 471, 494, 50, 196608, 475, 497, 50, 196608, 434, 50, 494, 196608, 438, 50, 497, 131092, 72, 491, 131093, 72, 491, 196608, 504, 516, 50, 196608, 500, 50, 519, 196608, 512, 516, 50, 196608, 508, 50, 519, 196608, 411, 50, 303, 196608, 59, 50, 108, 196608, 59, 108, 50, 131093, 96, 542, 131092, 92, 542, 131091, 550, 553, 131093, 92, 556, 131092, 96, 556, 196615, 559, 562, 50, 196609, 96, 92, 565, 131093, 569, 556, 131092, 569, 556, 196608, 2, 573, 50, 196608, 3, 576, 50, 131093, 96, 546, 131092, 92, 546, 131091, 587, 590, 131093, 92, 593, 131092, 96, 593, 196615, 596, 599, 50, 196609, 96, 92, 602, 131093, 606, 593, 131092, 606, 593, 196608, 2, 610, 50, 196608, 3, 613, 50, 196608, 579, 50, 616, 196608, 583, 50, 620, 196615, 624, 628, 50, 131091, 628, 624, 131092, 76, 636, 131093, 76, 636, 196608, 616, 639, 50, 196608, 620, 642, 50, 196608, 579, 50, 639, 196608, 583, 50, 642, 131092, 72, 636, 131093, 72, 636, 196608, 649, 661, 50, 196608, 645, 50, 664, 196608, 657, 661, 50, 196608, 653, 50, 664, 196608, 556, 50, 448, 196608, 60, 50, 108, 196608, 60, 108, 50, 131093, 96, 687, 131092, 92, 687, 131091, 695, 698, 131093, 96, 691, 131092, 92, 691, 131091, 704, 707, 196608, 0, 50, 579, 196608, 0, 50, 679, 196608, 0, 50, 671, 196608, 701, 50, 593, 196608, 61, 108, 50, 131093, 96, 729, 131092, 92, 729, 131091, 733, 736, 196608, 0, 50, 434, 196608, 0, 50, 534, 196608, 0, 50, 526, 196608, 62, 108, 50, 131093, 96, 754, 131092, 92, 754, 131091, 758, 761, 196608, 0, 50, 289, 196608, 0, 50, 389, 196608, 0, 50, 381, 196608, 63, 108, 50, 131093, 96, 779, 131092, 92, 779, 131091, 783, 786, 196608, 0, 50, 185, 196608, 0, 50, 148, 196608, 0, 50, 248, 196608, 0, 50, 240, 196608, 15, 50, 108, 196608, 15, 108, 50, 131093, 96, 812, 131092, 92, 812, 131091, 816, 819, 131093, 92, 822, 131092, 96, 822, 196615, 825, 828, 50, 196609, 96, 92, 831, 131093, 835, 822, 131092, 835, 822, 196608, 2, 839, 50, 196608, 3, 842, 50, 131093, 96, 808, 131092, 92, 808, 131091, 853, 856, 196608, 2, 50, 839, 196608, 845, 50, 76, 196608, 862, 76, 50, 196608, 849, 50, 72, 196608, 859, 50, 789, 196610, 248, 240, 55, 196610, 244, 236, 55, 196610, 389, 381, 55, 196610, 385, 377, 55, 196610, 534, 526, 55, 196610, 530, 522, 55, 196610, 679, 671, 55, 196610, 675, 667, 55, 196610, 0, 50, 906, 196610, 0, 50, 898, 196610, 0, 50, 890, 196610, 0, 50, 882];
        // Java: data.ahLst = new int[]{2, 48, 50, 51, 845, 849, 1, 49, 50, 52, 862, 849};
        data.ahLst = vec![2, 48, 50, 51, 845, 849, 1, 49, 50, 52, 862, 849];
        // Java: data.cxnLst = new int[]{56, 882, 886, 57, 890, 894, 58, 898, 902, 59, 906, 910, 60, 914, 910, 61, 918, 902, 62, 922, 894, 63, 926, 886, 15, 2, 874};
        data.cxnLst = vec![56, 882, 886, 57, 890, 894, 58, 898, 902, 59, 906, 910, 60, 914, 910, 61, 918, 902, 62, 922, 894, 63, 926, 886, 15, 2, 874];
        // Java: data.rect = new int[]{792, 189, 185, 475};
        data.rect = vec![792, 189, 185, 475];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 148, 152, 1, 248, 244, 1, 240, 236, 1, 185, 189, 3, 96, 92, 162, 393, 1, 389, 385, 1, 381, 377, 1, 326, 330, 3, 96, 92, 303, 538, 1, 534, 530, 1, 526, 522, 1, 471, 475, 3, 96, 92, 448, 683, 1, 679, 675, 1, 671, 667, 1, 616, 620, 3, 96, 92, 593, 725, 1, 721, 667, 1, 717, 675, 1, 713, 583, 3, 96, 92, 710, 683, 1, 750, 522, 1, 746, 530, 1, 742, 438, 3, 96, 92, 739, 538, 1, 775, 377, 1, 771, 385, 1, 767, 293, 3, 96, 92, 764, 393, 1, 804, 236, 1, 800, 244, 1, 796, 152, 3, 96, 92, 789, 878, 1, 870, 874, 1, 866, 874, 1, 845, 849, 3, 96, 92, 822, 878, 2}};
        data.pathLst = vec![vec![11, 13, 0, 148, 152, 1, 248, 244, 1, 240, 236, 1, 185, 189, 3, 96, 92, 162, 393, 1, 389, 385, 1, 381, 377, 1, 326, 330, 3, 96, 92, 303, 538, 1, 534, 530, 1, 526, 522, 1, 471, 475, 3, 96, 92, 448, 683, 1, 679, 675, 1, 671, 667, 1, 616, 620, 3, 96, 92, 593, 725, 1, 721, 667, 1, 717, 675, 1, 713, 583, 3, 96, 92, 710, 683, 1, 750, 522, 1, 746, 530, 1, 742, 438, 3, 96, 92, 739, 538, 1, 775, 377, 1, 771, 385, 1, 767, 293, 3, 96, 92, 764, 393, 1, 804, 236, 1, 800, 244, 1, 796, 152, 3, 96, 92, 789, 878, 1, 870, 874, 1, 866, 874, 1, 845, 849, 3, 96, 92, 822, 878, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_halfFrame()
    fn get_halfFrame() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{33333, 33333, 100000, 0, 1, 2};
        data.avcLst = vec![33333, 33333, 100000, 0, 1, 2];
        // Java: data.gdLst = new int[]{196609, 50, 0, 28, 196625, 51, 49, 54, 196609, 28, 58, 50, 196609, 1, 62, 0, 196608, 1, 51, 66, 196609, 50, 70, 28, 196625, 51, 48, 74, 196609, 28, 78, 50, 196609, 82, 0, 1, 196608, 0, 51, 86, 196609, 62, 1, 0, 196608, 1, 51, 94, 196609, 62, 52, 53, 196610, 98, 1, 53, 196610, 90, 0, 53, 196609, 82, 52, 53};
        data.gdLst = vec![196609, 50, 0, 28, 196625, 51, 49, 54, 196609, 28, 58, 50, 196609, 1, 62, 0, 196608, 1, 51, 66, 196609, 50, 70, 28, 196625, 51, 48, 74, 196609, 28, 78, 50, 196609, 82, 0, 1, 196608, 0, 51, 86, 196609, 62, 1, 0, 196608, 1, 51, 94, 196609, 62, 52, 53, 196610, 98, 1, 53, 196610, 90, 0, 53, 196609, 82, 52, 53];
        // Java: data.ahLst = new int[]{2, 48, 51, 74, 26, 82, 1, 49, 51, 54, 62, 26};
        data.ahLst = vec![2, 48, 51, 74, 26, 82, 1, 49, 51, 54, 62, 26];
        // Java: data.cxnLst = new int[]{51, 110, 114, 20, 102, 106, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![51, 110, 114, 20, 102, 106, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 90, 82, 1, 62, 82, 1, 62, 98, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 90, 82, 1, 62, 82, 1, 62, 98, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_heart()
    fn get_heart() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{49, 48, 10, 0, 1, 6, 5, 2, 3};
        data.avcLst = vec![49, 48, 10, 0, 1, 6, 5, 2, 3];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 0, 50, 49, 196608, 2, 51, 57, 196608, 2, 51, 61, 196608, 2, 61, 51, 196608, 2, 57, 51, 196608, 26, 51, 38, 196609, 0, 52, 53, 196609, 0, 54, 53, 196609, 1, 55, 56};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 0, 50, 49, 196608, 2, 51, 57, 196608, 2, 51, 61, 196608, 2, 61, 51, 196608, 2, 57, 51, 196608, 26, 51, 38, 196609, 0, 52, 53, 196609, 0, 54, 53, 196609, 1, 55, 56];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 22, 20, 2, 1};
        data.cxnLst = vec![15, 2, 22, 20, 2, 1];
        // Java: data.rect = new int[]{85, 22, 89, 93};
        data.rect = vec![85, 22, 89, 93];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 2, 22, 5, 73, 81, 77, 22, 2, 1, 5, 65, 22, 69, 81, 2, 22, 2}};
        data.pathLst = vec![vec![11, 13, 0, 2, 22, 5, 73, 81, 77, 22, 2, 1, 5, 65, 22, 69, 81, 2, 22, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_heptagon()
    fn get_heptagon() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{102572, 105210, 100000, 97493, 78183, 43388, 62349, 22252, 90097, 0};
        data.avcLst = vec![102572, 105210, 100000, 97493, 78183, 43388, 62349, 22252, 90097, 0];
        // Java: data.gdLst = new int[]{196609, 2, 48, 50, 196609, 3, 49, 50, 196609, 3, 49, 50, 196609, 58, 51, 50, 196609, 58, 52, 50, 196609, 58, 53, 50, 196609, 62, 54, 50, 196609, 62, 55, 50, 196609, 62, 56, 50, 196608, 2, 57, 70, 196608, 2, 57, 74, 196608, 2, 57, 78, 196608, 2, 78, 57, 196608, 2, 74, 57, 196608, 2, 70, 57, 196608, 66, 57, 82, 196608, 66, 86, 57, 196608, 66, 90, 57, 196608, 1, 57, 118};
        data.gdLst = vec![196609, 2, 48, 50, 196609, 3, 49, 50, 196609, 3, 49, 50, 196609, 58, 51, 50, 196609, 58, 52, 50, 196609, 58, 53, 50, 196609, 62, 54, 50, 196609, 62, 55, 50, 196609, 62, 56, 50, 196608, 2, 57, 70, 196608, 2, 57, 74, 196608, 2, 57, 78, 196608, 2, 78, 57, 196608, 2, 74, 57, 196608, 2, 70, 57, 196608, 66, 57, 82, 196608, 66, 86, 57, 196608, 66, 90, 57, 196608, 1, 57, 118];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{57, 110, 118, 57, 114, 122, 20, 106, 126, 20, 102, 126, 19, 94, 122, 19, 98, 118, 15, 2, 26};
        data.cxnLst = vec![57, 110, 118, 57, 114, 122, 20, 106, 126, 20, 102, 126, 19, 94, 122, 19, 98, 118, 15, 2, 26];
        // Java: data.rect = new int[]{98, 118, 110, 130};
        data.rect = vec![98, 118, 110, 130];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 94, 122, 1, 98, 118, 1, 2, 26, 1, 110, 118, 1, 114, 122, 1, 106, 126, 1, 102, 126, 2}};
        data.pathLst = vec![vec![11, 13, 0, 94, 122, 1, 98, 118, 1, 2, 26, 1, 110, 118, 1, 114, 122, 1, 106, 126, 1, 102, 126, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_hexagon()
    fn get_hexagon() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 115470, 50000, 0, 100000, 3600000, -1, 2, 4, 3, 24};
        data.avcLst = vec![25000, 115470, 50000, 0, 100000, 3600000, -1, 2, 4, 3, 24];
        // Java: data.gdLst = new int[]{196609, 50, 0, 28, 196625, 51, 48, 59, 196609, 3, 49, 52, 196609, 28, 63, 52, 196608, 0, 51, 71, 131092, 67, 53, 196608, 3, 51, 79, 196608, 3, 79, 51, 196609, 59, 54, 55, 196608, 63, 90, 51, 196614, 94, 56, 55, 196614, 94, 57, 55, 196614, 94, 90, 51, 196610, 63, 106, 90, 196609, 110, 102, 54, 196608, 98, 114, 51, 196609, 0, 118, 58, 196609, 1, 118, 58, 196608, 0, 51, 122, 196608, 1, 51, 126};
        data.gdLst = vec![196609, 50, 0, 28, 196625, 51, 48, 59, 196609, 3, 49, 52, 196609, 28, 63, 52, 196608, 0, 51, 71, 131092, 67, 53, 196608, 3, 51, 79, 196608, 3, 79, 51, 196609, 59, 54, 55, 196608, 63, 90, 51, 196614, 94, 56, 55, 196614, 94, 57, 55, 196614, 94, 90, 51, 196610, 63, 106, 90, 196609, 110, 102, 54, 196608, 98, 114, 51, 196609, 0, 118, 58, 196609, 1, 118, 58, 196608, 0, 51, 122, 196608, 1, 51, 126];
        // Java: data.ahLst = new int[]{1, 48, 51, 59, 71, 26};
        data.ahLst = vec![1, 48, 51, 59, 71, 26];
        // Java: data.cxnLst = new int[]{51, 0, 3, 20, 75, 86, 20, 71, 86, 19, 26, 3, 15, 71, 82, 15, 75, 82};
        data.cxnLst = vec![51, 0, 3, 20, 75, 86, 20, 71, 86, 19, 26, 3, 15, 71, 82, 15, 75, 82];
        // Java: data.rect = new int[]{122, 126, 130, 134};
        data.rect = vec![122, 126, 130, 134];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 1, 71, 82, 1, 75, 82, 1, 0, 3, 1, 75, 86, 1, 71, 86, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 1, 71, 82, 1, 75, 82, 1, 0, 3, 1, 75, 86, 1, 71, 86, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_homePlate()
    fn get_homePlate() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 100000, 0, 2, 1};
        data.avcLst = vec![50000, 100000, 0, 2, 1];
        // Java: data.gdLst = new int[]{196609, 49, 0, 28, 196625, 50, 48, 53, 196609, 28, 57, 49, 196608, 0, 50, 61, 196610, 65, 0, 51, 196609, 65, 52, 51};
        data.gdLst = vec![196609, 49, 0, 28, 196625, 50, 48, 53, 196609, 28, 57, 49, 196608, 0, 50, 61, 196610, 65, 0, 51, 196609, 65, 52, 51];
        // Java: data.ahLst = new int[]{1, 48, 50, 53, 65, 26};
        data.ahLst = vec![1, 48, 50, 53, 65, 26];
        // Java: data.cxnLst = new int[]{15, 73, 26, 19, 26, 3, 20, 65, 1, 50, 0, 3};
        data.cxnLst = vec![15, 73, 26, 19, 26, 3, 20, 65, 1, 50, 0, 3];
        // Java: data.rect = new int[]{26, 26, 69, 1};
        data.rect = vec![26, 26, 69, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 65, 26, 1, 0, 3, 1, 65, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 65, 26, 1, 0, 3, 1, 65, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_horizontalScroll()
    fn get_horizontalScroll() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{12500, 0, 25000, 100000, 1, 2, 4, -5400000, -10800000, -16200000};
        data.avcLst = vec![12500, 0, 25000, 100000, 1, 2, 4, -5400000, -10800000, -16200000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 58, 51, 196609, 62, 52, 53, 196609, 62, 52, 54, 196608, 62, 66, 49, 196608, 62, 62, 49, 196608, 1, 49, 62, 196608, 1, 49, 66, 196608, 82, 49, 66, 196608, 0, 49, 62, 196608, 0, 49, 66};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 58, 51, 196609, 62, 52, 53, 196609, 62, 52, 54, 196608, 62, 66, 49, 196608, 62, 62, 49, 196608, 1, 49, 62, 196608, 1, 49, 66, 196608, 82, 49, 66, 196608, 0, 49, 62, 196608, 0, 49, 66];
        // Java: data.ahLst = new int[]{1, 48, 49, 50, 62, 26};
        data.ahLst = vec![1, 48, 49, 50, 62, 26];
        // Java: data.cxnLst = new int[]{20, 2, 62, 19, 26, 3, 15, 2, 82, 49, 0, 3};
        data.cxnLst = vec![20, 2, 62, 19, 26, 3, 15, 2, 82, 49, 0, 3];
        // Java: data.rect = new int[]{62, 62, 98, 82};
        data.rect = vec![62, 62, 98, 82];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 0, 66, 3, 66, 66, 49, 20, 1, 98, 66, 3, 70, 70, 49, 19, 1, 94, 62, 1, 66, 62, 3, 66, 66, 15, 55, 1, 26, 86, 3, 66, 66, 19, 56, 1, 62, 82, 1, 98, 82, 3, 66, 66, 20, 55, 2, 0, 66, 78, 3, 66, 66, 20, 55, 3, 70, 70, 49, 56, 2}, {7, 12, 0, 66, 78, 3, 66, 66, 20, 55, 3, 70, 70, 49, 56, 2, 0, 98, 62, 3, 66, 66, 20, 57, 3, 70, 70, 19, 56, 2}, {10, 13, 0, 26, 74, 3, 66, 66, 19, 20, 1, 94, 62, 1, 94, 66, 3, 66, 66, 19, 19, 1, 0, 90, 3, 66, 66, 49, 20, 1, 62, 82, 1, 62, 86, 3, 66, 66, 49, 19, 2, 0, 94, 62, 1, 98, 62, 3, 66, 66, 20, 55, 0, 98, 62, 1, 98, 66, 3, 70, 70, 49, 19, 0, 66, 78, 1, 66, 74, 3, 70, 70, 19, 19, 3, 66, 66, 49, 19, 0, 62, 74, 1, 62, 82}};
        data.pathLst = vec![vec![11, 12, 0, 0, 66, 3, 66, 66, 49, 20, 1, 98, 66, 3, 70, 70, 49, 19, 1, 94, 62, 1, 66, 62, 3, 66, 66, 15, 55, 1, 26, 86, 3, 66, 66, 19, 56, 1, 62, 82, 1, 98, 82, 3, 66, 66, 20, 55, 2, 0, 66, 78, 3, 66, 66, 20, 55, 3, 70, 70, 49, 56, 2], vec![7, 12, 0, 66, 78, 3, 66, 66, 20, 55, 3, 70, 70, 49, 56, 2, 0, 98, 62, 3, 66, 66, 20, 57, 3, 70, 70, 19, 56, 2], vec![10, 13, 0, 26, 74, 3, 66, 66, 19, 20, 1, 94, 62, 1, 94, 66, 3, 66, 66, 19, 19, 1, 0, 90, 3, 66, 66, 49, 20, 1, 62, 82, 1, 62, 86, 3, 66, 66, 49, 19, 2, 0, 94, 62, 1, 98, 62, 3, 66, 66, 20, 55, 0, 98, 62, 1, 98, 66, 3, 70, 70, 49, 19, 0, 66, 78, 1, 66, 74, 3, 70, 70, 19, 19, 3, 66, 66, 49, 19, 0, 62, 74, 1, 62, 82]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_irregularSeal1()
    fn get_irregularSeal1() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{4627, 21600, 8485, 16702, 14522, 6320, 8615, 13937, 13290, 10800, 5800, 0, 14155, 5325, 18380, 4457, 7315, 21097, 8137, 17607, 10475, 16837, 12942, 18145, 18095, 14020, 14457, 13247, 19737, 10532, 14935, 7715, 15627, 4762, 17617, 5667, 135, 14587, 3722, 11775, 7617, 370, 2295, 7312, 8352};
        data.avcLst = vec![4627, 21600, 8485, 16702, 14522, 6320, 8615, 13937, 13290, 10800, 5800, 0, 14155, 5325, 18380, 4457, 7315, 21097, 8137, 17607, 10475, 16837, 12942, 18145, 18095, 14020, 14457, 13247, 19737, 10532, 14935, 7715, 15627, 4762, 17617, 5667, 135, 14587, 3722, 11775, 7617, 370, 2295, 7312, 8352];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 0, 50, 49, 196609, 0, 51, 49, 196609, 0, 52, 49, 196609, 1, 53, 49, 196609, 1, 54, 49, 196609, 1, 55, 49, 196609, 1, 56, 49, 196609, 0, 57, 49, 196609, 1, 58, 49, 196609, 0, 52, 49, 196609, 1, 59, 49, 196609, 0, 60, 49, 196609, 1, 61, 49, 196609, 0, 62, 49, 196609, 1, 63, 49, 196609, 0, 51, 49, 196609, 1, 64, 49, 196609, 0, 65, 49, 196609, 1, 66, 49, 196609, 0, 67, 49, 196609, 1, 68, 49, 196609, 0, 49, 49, 196609, 1, 56, 49, 196609, 0, 69, 49, 196609, 1, 70, 49, 196609, 0, 71, 49, 196609, 1, 72, 49, 196609, 0, 73, 49, 196609, 1, 74, 49, 196609, 0, 75, 49, 196609, 1, 76, 49, 196609, 0, 77, 49, 196609, 1, 78, 49, 196609, 0, 50, 49, 196609, 1, 49, 49, 196609, 0, 79, 49, 196609, 1, 80, 49, 196609, 0, 81, 49, 196609, 1, 82, 49, 196609, 0, 83, 49, 196609, 1, 55, 49, 196609, 0, 84, 49, 196609, 1, 85, 49, 196609, 0, 86, 49, 196609, 1, 87, 49, 196609, 0, 59, 49, 196609, 1, 54, 49, 196609, 0, 48, 49, 196609, 1, 88, 49, 196609, 0, 89, 49, 196609, 1, 90, 49, 196609, 0, 91, 49, 196609, 1, 53, 49, 196609, 0, 92, 49};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 0, 50, 49, 196609, 0, 51, 49, 196609, 0, 52, 49, 196609, 1, 53, 49, 196609, 1, 54, 49, 196609, 1, 55, 49, 196609, 1, 56, 49, 196609, 0, 57, 49, 196609, 1, 58, 49, 196609, 0, 52, 49, 196609, 1, 59, 49, 196609, 0, 60, 49, 196609, 1, 61, 49, 196609, 0, 62, 49, 196609, 1, 63, 49, 196609, 0, 51, 49, 196609, 1, 64, 49, 196609, 0, 65, 49, 196609, 1, 66, 49, 196609, 0, 67, 49, 196609, 1, 68, 49, 196609, 0, 49, 49, 196609, 1, 56, 49, 196609, 0, 69, 49, 196609, 1, 70, 49, 196609, 0, 71, 49, 196609, 1, 72, 49, 196609, 0, 73, 49, 196609, 1, 74, 49, 196609, 0, 75, 49, 196609, 1, 76, 49, 196609, 0, 77, 49, 196609, 1, 78, 49, 196609, 0, 50, 49, 196609, 1, 49, 49, 196609, 0, 79, 49, 196609, 1, 80, 49, 196609, 0, 81, 49, 196609, 1, 82, 49, 196609, 0, 83, 49, 196609, 1, 55, 49, 196609, 0, 84, 49, 196609, 1, 85, 49, 196609, 0, 86, 49, 196609, 1, 87, 49, 196609, 0, 59, 49, 196609, 1, 54, 49, 196609, 0, 48, 49, 196609, 1, 88, 49, 196609, 0, 89, 49, 196609, 1, 90, 49, 196609, 0, 91, 49, 196609, 1, 53, 49, 196609, 0, 92, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 105, 26, 19, 26, 113, 20, 97, 1, 59, 0, 121};
        data.cxnLst = vec![15, 105, 26, 19, 26, 113, 20, 97, 1, 59, 0, 121];
        // Java: data.rect = new int[]{93, 109, 101, 117};
        data.rect = vec![93, 109, 101, 117];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 125, 129, 1, 133, 137, 1, 141, 145, 1, 149, 153, 1, 157, 161, 1, 165, 169, 1, 173, 177, 1, 181, 185, 1, 189, 193, 1, 197, 201, 1, 205, 209, 1, 213, 217, 1, 221, 225, 1, 229, 233, 1, 237, 241, 1, 245, 249, 1, 253, 257, 1, 261, 265, 1, 269, 273, 1, 277, 281, 1, 285, 289, 1, 293, 297, 1, 301, 305, 1, 309, 297, 2}};
        data.pathLst = vec![vec![11, 13, 0, 125, 129, 1, 133, 137, 1, 141, 145, 1, 149, 153, 1, 157, 161, 1, 165, 169, 1, 173, 177, 1, 181, 185, 1, 189, 193, 1, 197, 201, 1, 205, 209, 1, 213, 217, 1, 221, 225, 1, 229, 233, 1, 237, 241, 1, 245, 249, 1, 253, 257, 1, 261, 265, 1, 269, 273, 1, 277, 281, 1, 285, 289, 1, 293, 297, 1, 301, 305, 1, 309, 297, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_irregularSeal2()
    fn get_irregularSeal2() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{9722, 21600, 5372, 11612, 14640, 1887, 6382, 12877, 19712, 18842, 15935, 6645, 11462, 4342, 14790, 0, 14525, 5777, 18007, 3172, 16380, 6532, 16985, 9402, 18270, 11290, 12310, 18877, 15632, 14350, 14942, 17370, 12180, 9872, 8700, 7527, 18125, 4917, 4805, 18240, 1285, 17825, 3330, 15370, 3935, 11592, 1172, 8270, 7817, 4502, 3625, 8550};
        data.avcLst = vec![9722, 21600, 5372, 11612, 14640, 1887, 6382, 12877, 19712, 18842, 15935, 6645, 11462, 4342, 14790, 0, 14525, 5777, 18007, 3172, 16380, 6532, 16985, 9402, 18270, 11290, 12310, 18877, 15632, 14350, 14942, 17370, 12180, 9872, 8700, 7527, 18125, 4917, 4805, 18240, 1285, 17825, 3330, 15370, 3935, 11592, 1172, 8270, 7817, 4502, 3625, 8550];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 0, 50, 49, 196609, 0, 51, 49, 196609, 0, 52, 49, 196609, 1, 53, 49, 196609, 1, 54, 49, 196609, 1, 55, 49, 196609, 1, 56, 49, 196609, 1, 57, 49, 196609, 1, 58, 49, 196609, 1, 59, 49, 196609, 0, 60, 49, 196609, 1, 61, 49, 196609, 0, 62, 49, 196609, 1, 63, 49, 196609, 0, 64, 49, 196609, 1, 65, 49, 196609, 0, 66, 49, 196609, 1, 67, 49, 196609, 0, 68, 49, 196609, 1, 69, 49, 196609, 0, 49, 49, 196609, 1, 59, 49, 196609, 0, 70, 49, 196609, 1, 71, 49, 196609, 0, 72, 49, 196609, 1, 73, 49, 196609, 1, 74, 49, 196609, 0, 75, 49, 196609, 1, 76, 49, 196609, 0, 52, 49, 196609, 1, 77, 49, 196609, 0, 78, 49, 196609, 1, 79, 49, 196609, 0, 80, 49, 196609, 1, 58, 49, 196609, 0, 51, 49, 196609, 1, 57, 49, 196609, 0, 81, 49, 196609, 0, 82, 49, 196609, 1, 56, 49, 196609, 0, 83, 49, 196609, 1, 84, 49, 196609, 0, 85, 49, 196609, 1, 49, 49, 196609, 0, 86, 49, 196609, 1, 87, 49, 196609, 0, 88, 49, 196609, 1, 89, 49, 196609, 0, 90, 49, 196609, 1, 91, 49, 196609, 0, 63, 49, 196609, 1, 55, 49, 196609, 0, 92, 49, 196609, 1, 93, 49, 196609, 0, 94, 49, 196609, 1, 95, 49, 196609, 0, 50, 49, 196609, 1, 96, 49, 196609, 0, 97, 49, 196609, 1, 98, 49, 196609, 0, 99, 49, 196609, 1, 54, 49, 196609, 0, 48, 49, 196609, 1, 53, 49};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 0, 50, 49, 196609, 0, 51, 49, 196609, 0, 52, 49, 196609, 1, 53, 49, 196609, 1, 54, 49, 196609, 1, 55, 49, 196609, 1, 56, 49, 196609, 1, 57, 49, 196609, 1, 58, 49, 196609, 1, 59, 49, 196609, 0, 60, 49, 196609, 1, 61, 49, 196609, 0, 62, 49, 196609, 1, 63, 49, 196609, 0, 64, 49, 196609, 1, 65, 49, 196609, 0, 66, 49, 196609, 1, 67, 49, 196609, 0, 68, 49, 196609, 1, 69, 49, 196609, 0, 49, 49, 196609, 1, 59, 49, 196609, 0, 70, 49, 196609, 1, 71, 49, 196609, 0, 72, 49, 196609, 1, 73, 49, 196609, 1, 74, 49, 196609, 0, 75, 49, 196609, 1, 76, 49, 196609, 0, 52, 49, 196609, 1, 77, 49, 196609, 0, 78, 49, 196609, 1, 79, 49, 196609, 0, 80, 49, 196609, 1, 58, 49, 196609, 0, 51, 49, 196609, 1, 57, 49, 196609, 0, 81, 49, 196609, 0, 82, 49, 196609, 1, 56, 49, 196609, 0, 83, 49, 196609, 1, 84, 49, 196609, 0, 85, 49, 196609, 1, 49, 49, 196609, 0, 86, 49, 196609, 1, 87, 49, 196609, 0, 88, 49, 196609, 1, 89, 49, 196609, 0, 90, 49, 196609, 1, 91, 49, 196609, 0, 63, 49, 196609, 1, 55, 49, 196609, 0, 92, 49, 196609, 1, 93, 49, 196609, 0, 94, 49, 196609, 1, 95, 49, 196609, 0, 50, 49, 196609, 1, 96, 49, 196609, 0, 97, 49, 196609, 1, 98, 49, 196609, 0, 99, 49, 196609, 1, 54, 49, 196609, 0, 48, 49, 196609, 1, 53, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 100, 116, 19, 26, 124, 20, 108, 132, 63, 0, 140};
        data.cxnLst = vec![15, 100, 116, 19, 26, 124, 20, 108, 132, 63, 0, 140];
        // Java: data.rect = new int[]{104, 120, 112, 136};
        data.rect = vec![104, 120, 112, 136];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 144, 148, 1, 152, 156, 1, 160, 164, 1, 168, 172, 1, 176, 180, 1, 184, 188, 1, 192, 196, 1, 200, 204, 1, 176, 208, 1, 212, 216, 1, 220, 224, 1, 228, 232, 1, 236, 240, 1, 244, 248, 1, 252, 232, 1, 256, 260, 1, 264, 268, 1, 272, 276, 1, 280, 284, 1, 288, 292, 1, 296, 300, 1, 304, 308, 1, 312, 316, 1, 320, 324, 1, 328, 332, 1, 336, 340, 1, 344, 348, 1, 352, 356, 2}};
        data.pathLst = vec![vec![11, 13, 0, 144, 148, 1, 152, 156, 1, 160, 164, 1, 168, 172, 1, 176, 180, 1, 184, 188, 1, 192, 196, 1, 200, 204, 1, 176, 208, 1, 212, 216, 1, 220, 224, 1, 228, 232, 1, 236, 240, 1, 244, 248, 1, 252, 232, 1, 256, 260, 1, 264, 268, 1, 272, 276, 1, 280, 284, 1, 288, 292, 1, 296, 300, 1, 304, 308, 1, 312, 316, 1, 320, 324, 1, 328, 332, 1, 336, 340, 1, 344, 348, 1, 352, 356, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_leftArrow()
    fn get_leftArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 50000, 100000, 0, 200000};
        data.avcLst = vec![50000, 50000, 100000, 0, 200000];
        // Java: data.gdLst = new int[]{196609, 50, 0, 28, 196625, 51, 48, 50, 196625, 51, 49, 53, 196609, 28, 61, 50, 196608, 26, 65, 51, 196609, 1, 57, 52, 196608, 3, 51, 73, 196608, 3, 73, 51, 196609, 77, 65, 3, 196608, 69, 51, 85};
        data.gdLst = vec![196609, 50, 0, 28, 196625, 51, 48, 50, 196625, 51, 49, 53, 196609, 28, 61, 50, 196608, 26, 65, 51, 196609, 1, 57, 52, 196608, 3, 51, 73, 196608, 3, 73, 51, 196609, 77, 65, 3, 196608, 69, 51, 85];
        // Java: data.ahLst = new int[]{2, 48, 51, 50, 0, 77, 1, 49, 51, 53, 69, 26};
        data.ahLst = vec![2, 48, 51, 50, 0, 77, 1, 49, 51, 53, 69, 26];
        // Java: data.cxnLst = new int[]{15, 69, 26, 19, 26, 3, 20, 69, 1, 51, 0, 3};
        data.cxnLst = vec![15, 69, 26, 19, 26, 3, 20, 69, 1, 51, 0, 3];
        // Java: data.rect = new int[]{89, 77, 0, 81};
        data.rect = vec![89, 77, 0, 81];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 1, 69, 26, 1, 69, 77, 1, 0, 77, 1, 0, 81, 1, 69, 81, 1, 69, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 1, 69, 26, 1, 69, 77, 1, 0, 77, 1, 0, 81, 1, 69, 81, 1, 69, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_leftArrowCallout()
    fn get_leftArrowCallout() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 25000, 25000, 64977, 50000, 0, 2, 1, 100000, 200000};
        data.avcLst = vec![25000, 25000, 25000, 64977, 50000, 0, 2, 1, 100000, 200000];
        // Java: data.gdLst = new int[]{196609, 52, 1, 28, 196625, 53, 49, 58, 196609, 62, 54, 55, 196625, 53, 48, 66, 196609, 56, 0, 28, 196625, 53, 50, 74, 196609, 78, 28, 0, 196608, 56, 53, 82, 196625, 53, 51, 86, 196609, 28, 62, 56, 196609, 28, 70, 57, 196608, 3, 53, 94, 196608, 3, 53, 98, 196608, 3, 98, 53, 196608, 3, 94, 53, 196609, 28, 78, 56, 196609, 0, 90, 56, 196608, 0, 53, 122, 196610, 126, 0, 54};
        data.gdLst = vec![196609, 52, 1, 28, 196625, 53, 49, 58, 196609, 62, 54, 55, 196625, 53, 48, 66, 196609, 56, 0, 28, 196625, 53, 50, 74, 196609, 78, 28, 0, 196608, 56, 53, 82, 196625, 53, 51, 86, 196609, 28, 62, 56, 196609, 28, 70, 57, 196608, 3, 53, 94, 196608, 3, 53, 98, 196608, 3, 98, 53, 196608, 3, 94, 53, 196609, 28, 78, 56, 196609, 0, 90, 56, 196608, 0, 53, 122, 196610, 126, 0, 54];
        // Java: data.ahLst = new int[]{2, 48, 53, 66, 118, 106, 2, 49, 53, 58, 26, 102, 1, 50, 53, 74, 118, 26, 1, 51, 53, 86, 126, 1};
        data.ahLst = vec![2, 48, 53, 66, 118, 106, 2, 49, 53, 58, 26, 102, 1, 50, 53, 74, 118, 26, 1, 51, 53, 86, 126, 1];
        // Java: data.cxnLst = new int[]{15, 130, 26, 19, 26, 3, 20, 130, 1, 53, 0, 3};
        data.cxnLst = vec![15, 130, 26, 19, 26, 3, 20, 130, 1, 53, 0, 3];
        // Java: data.rect = new int[]{126, 26, 0, 1};
        data.rect = vec![126, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 1, 118, 102, 1, 118, 106, 1, 126, 106, 1, 126, 26, 1, 0, 26, 1, 0, 1, 1, 126, 1, 1, 126, 110, 1, 118, 110, 1, 118, 114, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 1, 118, 102, 1, 118, 106, 1, 126, 106, 1, 126, 26, 1, 0, 26, 1, 0, 1, 1, 126, 1, 1, 126, 110, 1, 118, 110, 1, 118, 114, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_leftBrace()
    fn get_leftBrace() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{8333, 50000, 0, 100000, 1, 2, 2700000, -5400000};
        data.avcLst = vec![8333, 50000, 0, 100000, 1, 2, 2700000, -5400000];
        // Java: data.gdLst = new int[]{196625, 50, 49, 51, 196608, 51, 50, 56, 131076, 60, 56, 196609, 64, 52, 53, 196609, 67, 1, 28, 196625, 50, 48, 71, 196609, 28, 75, 51, 196609, 1, 56, 51, 196608, 83, 79, 50, 131093, 2, 54, 131092, 79, 54, 196608, 0, 50, 91, 196608, 79, 50, 94, 196608, 1, 94, 79};
        data.gdLst = vec![196625, 50, 49, 51, 196608, 51, 50, 56, 131076, 60, 56, 196609, 64, 52, 53, 196609, 67, 1, 28, 196625, 50, 48, 71, 196609, 28, 75, 51, 196609, 1, 56, 51, 196608, 83, 79, 50, 131093, 2, 54, 131092, 79, 54, 196608, 0, 50, 91, 196608, 79, 50, 94, 196608, 1, 94, 79];
        // Java: data.ahLst = new int[]{2, 48, 50, 71, 2, 79, 2, 49, 50, 51, 26, 83};
        data.ahLst = vec![2, 48, 50, 71, 2, 79, 2, 49, 50, 51, 26, 83];
        // Java: data.cxnLst = new int[]{20, 0, 26, 19, 26, 83, 15, 0, 1};
        data.cxnLst = vec![20, 0, 26, 19, 26, 83, 15, 0, 1];
        // Java: data.rect = new int[]{97, 101, 0, 105};
        data.rect = vec![97, 101, 0, 105];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 0, 1, 3, 2, 79, 20, 20, 1, 2, 87, 3, 2, 79, 50, 55, 3, 2, 79, 20, 55, 1, 2, 79, 3, 2, 79, 19, 20, 2}, {10, 13, 0, 0, 1, 3, 2, 79, 20, 20, 1, 2, 87, 3, 2, 79, 50, 55, 3, 2, 79, 20, 55, 1, 2, 79, 3, 2, 79, 19, 20}};
        data.pathLst = vec![vec![11, 12, 0, 0, 1, 3, 2, 79, 20, 20, 1, 2, 87, 3, 2, 79, 50, 55, 3, 2, 79, 20, 55, 1, 2, 79, 3, 2, 79, 19, 20, 2], vec![10, 13, 0, 0, 1, 3, 2, 79, 20, 20, 1, 2, 87, 3, 2, 79, 50, 55, 3, 2, 79, 20, 55, 1, 2, 79, 3, 2, 79, 19, 20]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_leftBracket()
    fn get_leftBracket() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{8333, 50000, 0, 100000, 2700000};
        data.avcLst = vec![8333, 50000, 0, 100000, 2700000];
        // Java: data.gdLst = new int[]{196609, 49, 1, 28, 196625, 50, 48, 53, 196609, 28, 57, 51, 196608, 1, 50, 61, 131093, 0, 52, 131092, 61, 52, 196608, 0, 50, 69, 196608, 61, 50, 72, 196608, 1, 72, 61};
        data.gdLst = vec![196609, 49, 1, 28, 196625, 50, 48, 53, 196609, 28, 57, 51, 196608, 1, 50, 61, 131093, 0, 52, 131092, 61, 52, 196608, 0, 50, 69, 196608, 61, 50, 72, 196608, 1, 72, 61];
        // Java: data.ahLst = new int[]{2, 48, 50, 53, 26, 61};
        data.ahLst = vec![2, 48, 50, 53, 26, 61];
        // Java: data.cxnLst = new int[]{20, 0, 26, 19, 26, 3, 15, 0, 1};
        data.cxnLst = vec![20, 0, 26, 19, 26, 3, 15, 0, 1];
        // Java: data.rect = new int[]{75, 79, 0, 83};
        data.rect = vec![75, 79, 0, 83];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 0, 1, 3, 0, 61, 20, 20, 1, 26, 61, 3, 0, 61, 19, 20, 2}, {10, 13, 0, 0, 1, 3, 0, 61, 20, 20, 1, 26, 61, 3, 0, 61, 19, 20}};
        data.pathLst = vec![vec![11, 12, 0, 0, 1, 3, 0, 61, 20, 20, 1, 26, 61, 3, 0, 61, 19, 20, 2], vec![10, 13, 0, 0, 1, 3, 0, 61, 20, 20, 1, 26, 61, 3, 0, 61, 19, 20]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_leftCircularArrow()
    fn get_leftCircularArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{12500, -1142319, 1142319, 10800000, 12500, 0, 25000, 2, 1, 21599999, 100000, 21600000, -1, 2700000};
        data.avcLst = vec![12500, -1142319, 1142319, 10800000, 12500, 0, 25000, 2, 1, 21599999, 100000, 21600000, -1, 2700000];
        // Java: data.gdLst = new int[]{196625, 53, 52, 54, 196609, 62, 55, 56, 196625, 53, 48, 66, 196625, 56, 50, 57, 196625, 53, 51, 57, 196609, 28, 70, 58, 196609, 28, 62, 58, 196609, 82, 56, 55, 196608, 2, 90, 86, 196608, 3, 90, 86, 196608, 94, 53, 82, 196608, 98, 53, 82, 196608, 102, 90, 53, 196608, 106, 90, 53, 131092, 110, 74, 131093, 114, 74, 196619, 110, 121, 118, 196620, 114, 121, 118, 196608, 2, 124, 53, 196608, 3, 128, 53, 131076, 102, 106, 196609, 124, 124, 56, 196609, 128, 128, 56, 196609, 140, 140, 56, 196608, 143, 53, 151, 196608, 147, 53, 151, 196609, 155, 159, 143, 196609, 163, 56, 147, 196608, 56, 53, 167, 65549, 171, 196609, 155, 56, 124, 196609, 177, 56, 128, 196610, 56, 175, 181, 131091, 56, 185, 196608, 189, 59, 53, 196614, 189, 189, 192, 196608, 196, 53, 74, 196608, 200, 59, 53, 196614, 200, 200, 204, 196608, 208, 53, 19, 196608, 208, 53, 59, 196614, 212, 216, 208, 65539, 220, 196609, 224, 60, 56, 65539, 49, 196609, 230, 60, 56, 196625, 226, 232, 53, 196608, 74, 236, 53, 131092, 110, 240, 131093, 114, 240, 196619, 110, 247, 244, 196620, 114, 247, 244, 196608, 2, 250, 53, 196608, 3, 254, 53, 131092, 94, 78, 131093, 98, 78, 196619, 94, 269, 266, 196620, 98, 269, 266, 196608, 2, 272, 53, 196608, 3, 276, 53, 131092, 102, 78, 131093, 106, 78, 196619, 102, 291, 288, 196620, 106, 291, 288, 196608, 2, 294, 53, 196608, 3, 298, 53, 131093, 86, 240, 131092, 86, 240, 196608, 132, 310, 53, 196608, 136, 313, 53, 131093, 86, 240, 131092, 86, 240, 262144, 132, 53, 324, 53, 262144, 136, 53, 327, 53, 196608, 330, 53, 2, 196608, 335, 53, 3, 196608, 316, 53, 2, 196608, 320, 53, 3, 131076, 94, 98, 196609, 340, 356, 94, 196609, 344, 356, 98, 196609, 348, 356, 94, 196609, 352, 356, 98, 196608, 367, 53, 359, 196608, 371, 53, 363, 196615, 375, 379, 53, 196609, 359, 371, 56, 196609, 367, 363, 56, 196608, 387, 53, 391, 196609, 356, 356, 56, 196609, 383, 383, 56, 196609, 399, 403, 56, 196609, 395, 395, 56, 196608, 407, 53, 411, 131077, 415, 53, 65549, 419, 196609, 379, 60, 56, 196614, 424, 60, 56, 196609, 428, 375, 56, 196609, 432, 422, 56, 196609, 395, 379, 56, 196610, 440, 436, 403, 196608, 440, 53, 436, 196609, 448, 56, 403, 65539, 379, 196609, 456, 422, 56, 196609, 395, 375, 60, 196610, 462, 458, 403, 196608, 462, 53, 458, 196609, 470, 56, 403, 196608, 367, 53, 444, 196608, 367, 53, 452, 196608, 371, 53, 466, 196608, 371, 53, 474, 196615, 478, 486, 53, 196615, 482, 490, 53, 196608, 498, 53, 494, 196614, 502, 444, 452, 196614, 502, 466, 474, 196609, 506, 94, 356, 196609, 510, 98, 356, 196608, 2, 514, 53, 196608, 3, 518, 53, 196609, 340, 140, 102, 196609, 344, 140, 106, 196609, 348, 140, 102, 196609, 352, 140, 106, 196608, 538, 53, 530, 196608, 542, 53, 534, 196615, 546, 550, 53, 196609, 530, 542, 56, 196609, 538, 534, 56, 196608, 558, 53, 562, 196609, 140, 140, 56, 196609, 554, 554, 56, 196609, 570, 574, 56, 196609, 566, 566, 56, 196608, 578, 53, 582, 131077, 586, 53, 65549, 590, 196609, 428, 546, 56, 196609, 595, 593, 56, 196609, 566, 550, 56, 196610, 603, 599, 574, 196608, 603, 53, 599, 196609, 611, 56, 574, 65539, 550, 196609, 619, 593, 56, 196609, 566, 546, 60, 196610, 625, 621, 574, 196608, 625, 53, 621, 196609, 633, 56, 574, 196608, 530, 53, 607, 196608, 530, 53, 615, 196608, 534, 53, 629, 196608, 534, 53, 637, 196615, 641, 649, 53, 196615, 645, 653, 53, 196608, 661, 53, 657, 196614, 665, 607, 615, 196614, 665, 629, 637, 196609, 669, 102, 140, 196609, 673, 106, 140, 196608, 2, 677, 53, 196608, 3, 681, 53, 131091, 677, 681, 196608, 693, 59, 53, 196614, 693, 693, 696, 196608, 78, 53, 700, 196608, 704, 59, 53, 196614, 704, 704, 708, 196608, 700, 712, 53, 196608, 53, 53, 712, 196608, 522, 53, 685, 196608, 526, 53, 689, 196615, 724, 728, 53, 196609, 732, 56, 55, 196608, 736, 53, 86, 196614, 740, 522, 316, 196614, 740, 526, 320, 196614, 740, 685, 330, 196614, 740, 689, 335, 131091, 514, 518, 196608, 760, 59, 53, 196614, 760, 760, 763, 196608, 767, 53, 78, 196608, 771, 53, 59, 196614, 771, 775, 771, 196608, 78, 779, 53, 196608, 53, 53, 779, 131092, 110, 78, 131093, 114, 78, 196619, 110, 794, 791, 196620, 114, 794, 791, 196608, 2, 797, 53, 196608, 3, 801, 53, 196608, 78, 20, 53, 196608, 240, 53, 20, 196608, 240, 19, 53, 131093, 94, 61, 131092, 98, 61, 196608, 2, 53, 825, 196608, 2, 825, 53, 196608, 3, 53, 828, 196608, 3, 828, 53};
        data.gdLst = vec![196625, 53, 52, 54, 196609, 62, 55, 56, 196625, 53, 48, 66, 196625, 56, 50, 57, 196625, 53, 51, 57, 196609, 28, 70, 58, 196609, 28, 62, 58, 196609, 82, 56, 55, 196608, 2, 90, 86, 196608, 3, 90, 86, 196608, 94, 53, 82, 196608, 98, 53, 82, 196608, 102, 90, 53, 196608, 106, 90, 53, 131092, 110, 74, 131093, 114, 74, 196619, 110, 121, 118, 196620, 114, 121, 118, 196608, 2, 124, 53, 196608, 3, 128, 53, 131076, 102, 106, 196609, 124, 124, 56, 196609, 128, 128, 56, 196609, 140, 140, 56, 196608, 143, 53, 151, 196608, 147, 53, 151, 196609, 155, 159, 143, 196609, 163, 56, 147, 196608, 56, 53, 167, 65549, 171, 196609, 155, 56, 124, 196609, 177, 56, 128, 196610, 56, 175, 181, 131091, 56, 185, 196608, 189, 59, 53, 196614, 189, 189, 192, 196608, 196, 53, 74, 196608, 200, 59, 53, 196614, 200, 200, 204, 196608, 208, 53, 19, 196608, 208, 53, 59, 196614, 212, 216, 208, 65539, 220, 196609, 224, 60, 56, 65539, 49, 196609, 230, 60, 56, 196625, 226, 232, 53, 196608, 74, 236, 53, 131092, 110, 240, 131093, 114, 240, 196619, 110, 247, 244, 196620, 114, 247, 244, 196608, 2, 250, 53, 196608, 3, 254, 53, 131092, 94, 78, 131093, 98, 78, 196619, 94, 269, 266, 196620, 98, 269, 266, 196608, 2, 272, 53, 196608, 3, 276, 53, 131092, 102, 78, 131093, 106, 78, 196619, 102, 291, 288, 196620, 106, 291, 288, 196608, 2, 294, 53, 196608, 3, 298, 53, 131093, 86, 240, 131092, 86, 240, 196608, 132, 310, 53, 196608, 136, 313, 53, 131093, 86, 240, 131092, 86, 240, 262144, 132, 53, 324, 53, 262144, 136, 53, 327, 53, 196608, 330, 53, 2, 196608, 335, 53, 3, 196608, 316, 53, 2, 196608, 320, 53, 3, 131076, 94, 98, 196609, 340, 356, 94, 196609, 344, 356, 98, 196609, 348, 356, 94, 196609, 352, 356, 98, 196608, 367, 53, 359, 196608, 371, 53, 363, 196615, 375, 379, 53, 196609, 359, 371, 56, 196609, 367, 363, 56, 196608, 387, 53, 391, 196609, 356, 356, 56, 196609, 383, 383, 56, 196609, 399, 403, 56, 196609, 395, 395, 56, 196608, 407, 53, 411, 131077, 415, 53, 65549, 419, 196609, 379, 60, 56, 196614, 424, 60, 56, 196609, 428, 375, 56, 196609, 432, 422, 56, 196609, 395, 379, 56, 196610, 440, 436, 403, 196608, 440, 53, 436, 196609, 448, 56, 403, 65539, 379, 196609, 456, 422, 56, 196609, 395, 375, 60, 196610, 462, 458, 403, 196608, 462, 53, 458, 196609, 470, 56, 403, 196608, 367, 53, 444, 196608, 367, 53, 452, 196608, 371, 53, 466, 196608, 371, 53, 474, 196615, 478, 486, 53, 196615, 482, 490, 53, 196608, 498, 53, 494, 196614, 502, 444, 452, 196614, 502, 466, 474, 196609, 506, 94, 356, 196609, 510, 98, 356, 196608, 2, 514, 53, 196608, 3, 518, 53, 196609, 340, 140, 102, 196609, 344, 140, 106, 196609, 348, 140, 102, 196609, 352, 140, 106, 196608, 538, 53, 530, 196608, 542, 53, 534, 196615, 546, 550, 53, 196609, 530, 542, 56, 196609, 538, 534, 56, 196608, 558, 53, 562, 196609, 140, 140, 56, 196609, 554, 554, 56, 196609, 570, 574, 56, 196609, 566, 566, 56, 196608, 578, 53, 582, 131077, 586, 53, 65549, 590, 196609, 428, 546, 56, 196609, 595, 593, 56, 196609, 566, 550, 56, 196610, 603, 599, 574, 196608, 603, 53, 599, 196609, 611, 56, 574, 65539, 550, 196609, 619, 593, 56, 196609, 566, 546, 60, 196610, 625, 621, 574, 196608, 625, 53, 621, 196609, 633, 56, 574, 196608, 530, 53, 607, 196608, 530, 53, 615, 196608, 534, 53, 629, 196608, 534, 53, 637, 196615, 641, 649, 53, 196615, 645, 653, 53, 196608, 661, 53, 657, 196614, 665, 607, 615, 196614, 665, 629, 637, 196609, 669, 102, 140, 196609, 673, 106, 140, 196608, 2, 677, 53, 196608, 3, 681, 53, 131091, 677, 681, 196608, 693, 59, 53, 196614, 693, 693, 696, 196608, 78, 53, 700, 196608, 704, 59, 53, 196614, 704, 704, 708, 196608, 700, 712, 53, 196608, 53, 53, 712, 196608, 522, 53, 685, 196608, 526, 53, 689, 196615, 724, 728, 53, 196609, 732, 56, 55, 196608, 736, 53, 86, 196614, 740, 522, 316, 196614, 740, 526, 320, 196614, 740, 685, 330, 196614, 740, 689, 335, 131091, 514, 518, 196608, 760, 59, 53, 196614, 760, 760, 763, 196608, 767, 53, 78, 196608, 771, 53, 59, 196614, 771, 775, 771, 196608, 78, 779, 53, 196608, 53, 53, 779, 131092, 110, 78, 131093, 114, 78, 196619, 110, 794, 791, 196620, 114, 794, 791, 196608, 2, 797, 53, 196608, 3, 801, 53, 196608, 78, 20, 53, 196608, 240, 53, 20, 196608, 240, 19, 53, 131093, 94, 61, 131092, 98, 61, 196608, 2, 53, 825, 196608, 2, 825, 53, 196608, 3, 53, 828, 196608, 3, 828, 53];
        // Java: data.ahLst = new int[]{4, 49, 226, 53, 258, 262, 4, 51, 53, 57, 280, 284, 6, 50, 53, 57, 48, 53, 66, 522, 526, 5, 52, 53, 54, 330, 335};
        data.ahLst = vec![4, 49, 226, 53, 258, 262, 4, 51, 53, 57, 280, 284, 6, 50, 53, 57, 48, 53, 66, 522, 526, 5, 52, 53, 54, 330, 335];
        // Java: data.cxnLst = new int[]{813, 805, 809, 240, 744, 748, 817, 258, 262, 821, 752, 756};
        data.cxnLst = vec![813, 805, 809, 240, 744, 748, 817, 258, 262, 821, 752, 756];
        // Java: data.rect = new int[]{831, 839, 835, 843};
        data.rect = vec![831, 839, 835, 843];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 280, 284, 1, 302, 306, 3, 102, 106, 716, 720, 1, 752, 756, 1, 258, 262, 1, 744, 748, 1, 522, 526, 3, 94, 98, 783, 787, 2}};
        data.pathLst = vec![vec![11, 13, 0, 280, 284, 1, 302, 306, 3, 102, 106, 716, 720, 1, 752, 756, 1, 258, 262, 1, 744, 748, 1, 522, 526, 3, 94, 98, 783, 787, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_leftRightArrow()
    fn get_leftRightArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 50000, 50000, 0, 100000, 200000};
        data.avcLst = vec![50000, 50000, 50000, 0, 100000, 200000];
        // Java: data.gdLst = new int[]{196609, 50, 0, 28, 196625, 51, 48, 52, 196625, 51, 49, 54, 196609, 28, 62, 52, 196608, 0, 51, 66, 196609, 1, 58, 53, 196608, 3, 51, 74, 196608, 3, 74, 51, 196609, 78, 66, 3, 196608, 66, 51, 86, 196608, 70, 86, 51};
        data.gdLst = vec![196609, 50, 0, 28, 196625, 51, 48, 52, 196625, 51, 49, 54, 196609, 28, 62, 52, 196608, 0, 51, 66, 196609, 1, 58, 53, 196608, 3, 51, 74, 196608, 3, 74, 51, 196609, 78, 66, 3, 196608, 66, 51, 86, 196608, 70, 86, 51];
        // Java: data.ahLst = new int[]{2, 48, 51, 52, 70, 78, 1, 49, 51, 54, 66, 26};
        data.ahLst = vec![2, 48, 51, 52, 70, 78, 1, 49, 51, 54, 66, 26];
        // Java: data.cxnLst = new int[]{51, 0, 3, 20, 70, 1, 20, 66, 1, 19, 26, 3, 15, 66, 26, 15, 70, 26};
        data.cxnLst = vec![51, 0, 3, 20, 70, 1, 20, 66, 1, 19, 26, 3, 15, 66, 26, 15, 70, 26];
        // Java: data.rect = new int[]{90, 78, 94, 82};
        data.rect = vec![90, 78, 94, 82];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 1, 66, 26, 1, 66, 78, 1, 70, 78, 1, 70, 26, 1, 0, 3, 1, 70, 1, 1, 70, 82, 1, 66, 82, 1, 66, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 1, 66, 26, 1, 66, 78, 1, 70, 78, 1, 70, 26, 1, 0, 3, 1, 70, 1, 1, 70, 82, 1, 66, 82, 1, 66, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_leftRightArrowCallout()
    fn get_leftRightArrowCallout() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 25000, 25000, 48123, 50000, 0, 2, 1, 100000, 200000};
        data.avcLst = vec![25000, 25000, 25000, 48123, 50000, 0, 2, 1, 100000, 200000];
        // Java: data.gdLst = new int[]{196609, 52, 1, 28, 196625, 53, 49, 58, 196609, 62, 54, 55, 196625, 53, 48, 66, 196609, 52, 0, 28, 196625, 53, 50, 74, 196609, 78, 28, 2, 196608, 56, 53, 82, 196625, 53, 51, 86, 196609, 28, 62, 56, 196609, 28, 70, 57, 196608, 3, 53, 94, 196608, 3, 53, 98, 196608, 3, 98, 53, 196608, 3, 94, 53, 196609, 28, 78, 56, 196608, 0, 53, 118, 196609, 0, 90, 57, 196608, 2, 53, 126, 196608, 2, 126, 53};
        data.gdLst = vec![196609, 52, 1, 28, 196625, 53, 49, 58, 196609, 62, 54, 55, 196625, 53, 48, 66, 196609, 52, 0, 28, 196625, 53, 50, 74, 196609, 78, 28, 2, 196608, 56, 53, 82, 196625, 53, 51, 86, 196609, 28, 62, 56, 196609, 28, 70, 57, 196608, 3, 53, 94, 196608, 3, 53, 98, 196608, 3, 98, 53, 196608, 3, 94, 53, 196609, 28, 78, 56, 196608, 0, 53, 118, 196609, 0, 90, 57, 196608, 2, 53, 126, 196608, 2, 126, 53];
        // Java: data.ahLst = new int[]{2, 48, 53, 66, 118, 106, 2, 49, 53, 58, 26, 102, 1, 50, 53, 74, 118, 26, 1, 51, 53, 86, 130, 1};
        data.ahLst = vec![2, 48, 53, 66, 118, 106, 2, 49, 53, 58, 26, 102, 1, 50, 53, 74, 118, 26, 1, 51, 53, 86, 130, 1];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 53, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 53, 0, 3];
        // Java: data.rect = new int[]{130, 26, 134, 1};
        data.rect = vec![130, 26, 134, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 1, 118, 102, 1, 118, 106, 1, 130, 106, 1, 130, 26, 1, 134, 26, 1, 134, 106, 1, 122, 106, 1, 122, 102, 1, 0, 3, 1, 122, 114, 1, 122, 110, 1, 134, 110, 1, 134, 1, 1, 130, 1, 1, 130, 110, 1, 118, 110, 1, 118, 114, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 1, 118, 102, 1, 118, 106, 1, 130, 106, 1, 130, 26, 1, 134, 26, 1, 134, 106, 1, 122, 106, 1, 122, 102, 1, 0, 3, 1, 122, 114, 1, 122, 110, 1, 134, 110, 1, 134, 1, 1, 130, 1, 1, 130, 110, 1, 118, 110, 1, 118, 114, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_leftRightCircularArrow()
    fn get_leftRightCircularArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{12500, 1142319, 20457681, 11942319, 12500, 0, 25000, 2, 1, 21599999, 100000, 21600000, -1, 2700000};
        data.avcLst = vec![12500, 1142319, 20457681, 11942319, 12500, 0, 25000, 2, 1, 21599999, 100000, 21600000, -1, 2700000];
        // Java: data.gdLst = new int[]{196625, 53, 52, 54, 196609, 62, 55, 56, 196625, 53, 48, 66, 196625, 56, 50, 57, 196625, 53, 51, 57, 196609, 28, 70, 58, 196609, 28, 62, 58, 196609, 82, 56, 55, 196608, 2, 90, 86, 196608, 3, 90, 86, 196608, 94, 53, 82, 196608, 98, 53, 82, 196608, 102, 90, 53, 196608, 106, 90, 53, 131092, 110, 74, 131093, 114, 74, 196619, 110, 121, 118, 196620, 114, 121, 118, 196608, 2, 124, 53, 196608, 3, 128, 53, 131076, 102, 106, 196609, 124, 124, 56, 196609, 128, 128, 56, 196609, 140, 140, 56, 196608, 143, 53, 151, 196608, 147, 53, 151, 196609, 155, 159, 143, 196609, 163, 56, 147, 196608, 56, 53, 167, 65549, 171, 196609, 155, 56, 124, 196609, 177, 56, 128, 196610, 56, 175, 181, 131091, 56, 185, 196608, 189, 59, 53, 196614, 189, 189, 192, 196608, 196, 53, 74, 196608, 200, 59, 53, 196614, 200, 200, 204, 196608, 208, 53, 19, 196608, 208, 53, 59, 196614, 212, 216, 208, 65539, 220, 196625, 53, 49, 224, 196608, 74, 226, 53, 131092, 110, 230, 131093, 114, 230, 196619, 110, 237, 234, 196620, 114, 237, 234, 196608, 2, 240, 53, 196608, 3, 244, 53, 131093, 86, 230, 131092, 86, 230, 196608, 132, 256, 53, 196608, 136, 259, 53, 131093, 86, 230, 131092, 86, 230, 262144, 132, 53, 270, 53, 262144, 136, 53, 273, 53, 196608, 276, 53, 2, 196608, 281, 53, 3, 196608, 262, 53, 2, 196608, 266, 53, 3, 131076, 94, 98, 196609, 286, 302, 94, 196609, 290, 302, 98, 196609, 294, 302, 94, 196609, 298, 302, 98, 196608, 313, 53, 305, 196608, 317, 53, 309, 196615, 321, 325, 53, 196609, 305, 317, 56, 196609, 313, 309, 56, 196608, 333, 53, 337, 196609, 302, 302, 56, 196609, 329, 329, 56, 196609, 345, 349, 56, 196609, 341, 341, 56, 196608, 353, 53, 357, 131077, 361, 53, 65549, 365, 196609, 325, 60, 56, 196614, 370, 60, 56, 196609, 374, 321, 56, 196609, 378, 368, 56, 196609, 341, 325, 56, 196610, 386, 382, 349, 196608, 386, 53, 382, 196609, 394, 56, 349, 65539, 325, 196609, 402, 368, 56, 196609, 341, 321, 60, 196610, 408, 404, 349, 196608, 408, 53, 404, 196609, 416, 56, 349, 196608, 313, 53, 390, 196608, 313, 53, 398, 196608, 317, 53, 412, 196608, 317, 53, 420, 196615, 424, 432, 53, 196615, 428, 436, 53, 196608, 444, 53, 440, 196614, 448, 390, 398, 196614, 448, 412, 420, 196609, 452, 94, 302, 196609, 456, 98, 302, 196608, 2, 460, 53, 196608, 3, 464, 53, 196609, 286, 140, 102, 196609, 290, 140, 106, 196609, 294, 140, 102, 196609, 298, 140, 106, 196608, 484, 53, 476, 196608, 488, 53, 480, 196615, 492, 496, 53, 196609, 476, 488, 56, 196609, 484, 480, 56, 196608, 504, 53, 508, 196609, 140, 140, 56, 196609, 500, 500, 56, 196609, 516, 520, 56, 196609, 512, 512, 56, 196608, 524, 53, 528, 131077, 532, 53, 65549, 536, 196609, 374, 492, 56, 196609, 541, 539, 56, 196609, 512, 496, 56, 196610, 549, 545, 520, 196608, 549, 53, 545, 196609, 557, 56, 520, 65539, 496, 196609, 565, 539, 56, 196609, 512, 492, 60, 196610, 571, 567, 520, 196608, 571, 53, 567, 196609, 579, 56, 520, 196608, 476, 53, 553, 196608, 476, 53, 561, 196608, 480, 53, 575, 196608, 480, 53, 583, 196615, 587, 595, 53, 196615, 591, 599, 53, 196608, 607, 53, 603, 196614, 611, 553, 561, 196614, 611, 575, 583, 196609, 615, 102, 140, 196609, 619, 106, 140, 196608, 2, 623, 53, 196608, 3, 627, 53, 131092, 110, 78, 131093, 114, 78, 196619, 110, 642, 639, 196620, 114, 642, 639, 196608, 2, 645, 53, 196608, 3, 649, 53, 196608, 78, 53, 226, 131092, 110, 661, 131093, 114, 661, 196619, 110, 668, 665, 196620, 114, 668, 665, 196608, 2, 671, 53, 196608, 3, 675, 53, 131093, 86, 661, 131092, 86, 661, 196608, 653, 687, 53, 196608, 657, 690, 53, 131093, 86, 661, 131092, 86, 661, 262144, 653, 53, 701, 53, 262144, 657, 53, 704, 53, 196608, 468, 53, 631, 196608, 472, 53, 635, 196615, 717, 721, 53, 196609, 725, 56, 55, 196608, 729, 53, 86, 196614, 733, 468, 262, 196614, 733, 472, 266, 196614, 733, 631, 276, 196614, 733, 635, 281, 131091, 460, 464, 196608, 753, 59, 53, 196614, 753, 753, 756, 196608, 760, 53, 74, 196608, 764, 59, 53, 196614, 764, 764, 768, 196608, 78, 53, 772, 196608, 776, 59, 53, 196614, 776, 776, 780, 196608, 760, 53, 784, 196608, 788, 59, 53, 196614, 788, 788, 792, 131091, 623, 627, 196608, 800, 59, 53, 196614, 800, 800, 803, 196608, 807, 53, 74, 196608, 811, 53, 59, 196614, 811, 815, 811, 196608, 78, 53, 819, 196608, 823, 53, 59, 196614, 827, 827, 823, 196608, 831, 53, 807, 196608, 835, 53, 59, 196614, 835, 839, 835, 131092, 94, 784, 131093, 98, 784, 196619, 94, 850, 847, 196620, 98, 850, 847, 196608, 2, 853, 53, 196608, 3, 857, 53, 131092, 102, 831, 131093, 106, 831, 196619, 102, 872, 869, 196620, 106, 872, 869, 196608, 2, 875, 53, 196608, 3, 879, 53, 196614, 733, 861, 693, 196614, 733, 865, 697, 196614, 733, 883, 707, 196614, 733, 887, 712, 196608, 661, 53, 20, 196608, 230, 20, 53, 196608, 230, 19, 53, 196608, 661, 19, 53, 131093, 94, 61, 131092, 98, 61, 196608, 2, 53, 923, 196608, 2, 923, 53, 196608, 3, 53, 926, 196608, 3, 926, 53};
        data.gdLst = vec![196625, 53, 52, 54, 196609, 62, 55, 56, 196625, 53, 48, 66, 196625, 56, 50, 57, 196625, 53, 51, 57, 196609, 28, 70, 58, 196609, 28, 62, 58, 196609, 82, 56, 55, 196608, 2, 90, 86, 196608, 3, 90, 86, 196608, 94, 53, 82, 196608, 98, 53, 82, 196608, 102, 90, 53, 196608, 106, 90, 53, 131092, 110, 74, 131093, 114, 74, 196619, 110, 121, 118, 196620, 114, 121, 118, 196608, 2, 124, 53, 196608, 3, 128, 53, 131076, 102, 106, 196609, 124, 124, 56, 196609, 128, 128, 56, 196609, 140, 140, 56, 196608, 143, 53, 151, 196608, 147, 53, 151, 196609, 155, 159, 143, 196609, 163, 56, 147, 196608, 56, 53, 167, 65549, 171, 196609, 155, 56, 124, 196609, 177, 56, 128, 196610, 56, 175, 181, 131091, 56, 185, 196608, 189, 59, 53, 196614, 189, 189, 192, 196608, 196, 53, 74, 196608, 200, 59, 53, 196614, 200, 200, 204, 196608, 208, 53, 19, 196608, 208, 53, 59, 196614, 212, 216, 208, 65539, 220, 196625, 53, 49, 224, 196608, 74, 226, 53, 131092, 110, 230, 131093, 114, 230, 196619, 110, 237, 234, 196620, 114, 237, 234, 196608, 2, 240, 53, 196608, 3, 244, 53, 131093, 86, 230, 131092, 86, 230, 196608, 132, 256, 53, 196608, 136, 259, 53, 131093, 86, 230, 131092, 86, 230, 262144, 132, 53, 270, 53, 262144, 136, 53, 273, 53, 196608, 276, 53, 2, 196608, 281, 53, 3, 196608, 262, 53, 2, 196608, 266, 53, 3, 131076, 94, 98, 196609, 286, 302, 94, 196609, 290, 302, 98, 196609, 294, 302, 94, 196609, 298, 302, 98, 196608, 313, 53, 305, 196608, 317, 53, 309, 196615, 321, 325, 53, 196609, 305, 317, 56, 196609, 313, 309, 56, 196608, 333, 53, 337, 196609, 302, 302, 56, 196609, 329, 329, 56, 196609, 345, 349, 56, 196609, 341, 341, 56, 196608, 353, 53, 357, 131077, 361, 53, 65549, 365, 196609, 325, 60, 56, 196614, 370, 60, 56, 196609, 374, 321, 56, 196609, 378, 368, 56, 196609, 341, 325, 56, 196610, 386, 382, 349, 196608, 386, 53, 382, 196609, 394, 56, 349, 65539, 325, 196609, 402, 368, 56, 196609, 341, 321, 60, 196610, 408, 404, 349, 196608, 408, 53, 404, 196609, 416, 56, 349, 196608, 313, 53, 390, 196608, 313, 53, 398, 196608, 317, 53, 412, 196608, 317, 53, 420, 196615, 424, 432, 53, 196615, 428, 436, 53, 196608, 444, 53, 440, 196614, 448, 390, 398, 196614, 448, 412, 420, 196609, 452, 94, 302, 196609, 456, 98, 302, 196608, 2, 460, 53, 196608, 3, 464, 53, 196609, 286, 140, 102, 196609, 290, 140, 106, 196609, 294, 140, 102, 196609, 298, 140, 106, 196608, 484, 53, 476, 196608, 488, 53, 480, 196615, 492, 496, 53, 196609, 476, 488, 56, 196609, 484, 480, 56, 196608, 504, 53, 508, 196609, 140, 140, 56, 196609, 500, 500, 56, 196609, 516, 520, 56, 196609, 512, 512, 56, 196608, 524, 53, 528, 131077, 532, 53, 65549, 536, 196609, 374, 492, 56, 196609, 541, 539, 56, 196609, 512, 496, 56, 196610, 549, 545, 520, 196608, 549, 53, 545, 196609, 557, 56, 520, 65539, 496, 196609, 565, 539, 56, 196609, 512, 492, 60, 196610, 571, 567, 520, 196608, 571, 53, 567, 196609, 579, 56, 520, 196608, 476, 53, 553, 196608, 476, 53, 561, 196608, 480, 53, 575, 196608, 480, 53, 583, 196615, 587, 595, 53, 196615, 591, 599, 53, 196608, 607, 53, 603, 196614, 611, 553, 561, 196614, 611, 575, 583, 196609, 615, 102, 140, 196609, 619, 106, 140, 196608, 2, 623, 53, 196608, 3, 627, 53, 131092, 110, 78, 131093, 114, 78, 196619, 110, 642, 639, 196620, 114, 642, 639, 196608, 2, 645, 53, 196608, 3, 649, 53, 196608, 78, 53, 226, 131092, 110, 661, 131093, 114, 661, 196619, 110, 668, 665, 196620, 114, 668, 665, 196608, 2, 671, 53, 196608, 3, 675, 53, 131093, 86, 661, 131092, 86, 661, 196608, 653, 687, 53, 196608, 657, 690, 53, 131093, 86, 661, 131092, 86, 661, 262144, 653, 53, 701, 53, 262144, 657, 53, 704, 53, 196608, 468, 53, 631, 196608, 472, 53, 635, 196615, 717, 721, 53, 196609, 725, 56, 55, 196608, 729, 53, 86, 196614, 733, 468, 262, 196614, 733, 472, 266, 196614, 733, 631, 276, 196614, 733, 635, 281, 131091, 460, 464, 196608, 753, 59, 53, 196614, 753, 753, 756, 196608, 760, 53, 74, 196608, 764, 59, 53, 196614, 764, 764, 768, 196608, 78, 53, 772, 196608, 776, 59, 53, 196614, 776, 776, 780, 196608, 760, 53, 784, 196608, 788, 59, 53, 196614, 788, 788, 792, 131091, 623, 627, 196608, 800, 59, 53, 196614, 800, 800, 803, 196608, 807, 53, 74, 196608, 811, 53, 59, 196614, 811, 815, 811, 196608, 78, 53, 819, 196608, 823, 53, 59, 196614, 827, 827, 823, 196608, 831, 53, 807, 196608, 835, 53, 59, 196614, 835, 839, 835, 131092, 94, 784, 131093, 98, 784, 196619, 94, 850, 847, 196620, 98, 850, 847, 196608, 2, 853, 53, 196608, 3, 857, 53, 131092, 102, 831, 131093, 106, 831, 196619, 102, 872, 869, 196620, 106, 872, 869, 196608, 2, 875, 53, 196608, 3, 879, 53, 196614, 733, 861, 693, 196614, 733, 865, 697, 196614, 733, 883, 707, 196614, 733, 887, 712, 196608, 661, 53, 20, 196608, 230, 20, 53, 196608, 230, 19, 53, 196608, 661, 19, 53, 131093, 94, 61, 131092, 98, 61, 196608, 2, 53, 923, 196608, 2, 923, 53, 196608, 3, 53, 926, 196608, 3, 926, 53];
        // Java: data.ahLst = new int[]{4, 49, 53, 224, 248, 252, 4, 51, 53, 57, 861, 865, 6, 50, 53, 57, 48, 53, 66, 468, 472, 5, 52, 53, 54, 276, 281};
        data.ahLst = vec![4, 49, 53, 224, 248, 252, 4, 51, 53, 57, 861, 865, 6, 50, 53, 57, 48, 53, 66, 468, 472, 5, 52, 53, 54, 276, 281];
        // Java: data.cxnLst = new int[]{907, 679, 683, 661, 891, 895, 230, 737, 741, 911, 248, 252, 915, 745, 749, 919, 899, 903};
        data.cxnLst = vec![907, 679, 683, 661, 891, 895, 230, 737, 741, 911, 248, 252, 915, 745, 749, 919, 899, 903];
        // Java: data.rect = new int[]{929, 937, 933, 941};
        data.rect = vec![929, 937, 933, 941];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 679, 683, 1, 891, 895, 1, 861, 865, 3, 94, 98, 784, 796, 1, 737, 741, 1, 248, 252, 1, 745, 749, 1, 631, 635, 3, 102, 106, 807, 843, 1, 899, 903, 2}};
        data.pathLst = vec![vec![11, 13, 0, 679, 683, 1, 891, 895, 1, 861, 865, 3, 94, 98, 784, 796, 1, 737, 741, 1, 248, 252, 1, 745, 749, 1, 631, 635, 3, 102, 106, 807, 843, 1, 899, 903, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_leftRightRibbon()
    fn get_leftRightRibbon() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 50000, 16667, 0, 33333, 100000, 200000, -200000, 2, 1, 400000, -10800000};
        data.avcLst = vec![50000, 50000, 16667, 0, 33333, 100000, 200000, -200000, 2, 1, 400000, -10800000];
        // Java: data.gdLst = new int[]{196625, 51, 50, 52, 196608, 53, 51, 60, 196625, 51, 48, 64, 196608, 2, 51, 43, 196609, 53, 72, 28, 196625, 51, 49, 76, 196609, 28, 80, 53, 196608, 0, 51, 84, 196609, 1, 68, 54, 196609, 1, 60, 55, 196608, 3, 96, 92, 196608, 3, 92, 96, 196608, 100, 92, 51, 196608, 1, 51, 108, 196609, 108, 56, 57, 196608, 1, 51, 116, 196608, 116, 51, 100, 196608, 1, 51, 124, 196609, 60, 28, 58, 196608, 2, 51, 43, 196608, 2, 43, 51, 196608, 100, 132, 51, 196608, 128, 51, 132};
        data.gdLst = vec![196625, 51, 50, 52, 196608, 53, 51, 60, 196625, 51, 48, 64, 196608, 2, 51, 43, 196609, 53, 72, 28, 196625, 51, 49, 76, 196609, 28, 80, 53, 196608, 0, 51, 84, 196609, 1, 68, 54, 196609, 1, 60, 55, 196608, 3, 96, 92, 196608, 3, 92, 96, 196608, 100, 92, 51, 196608, 1, 51, 108, 196609, 108, 56, 57, 196608, 1, 51, 116, 196608, 116, 51, 100, 196608, 1, 51, 124, 196609, 60, 28, 58, 196608, 2, 51, 43, 196608, 2, 43, 51, 196608, 100, 132, 51, 196608, 128, 51, 132];
        // Java: data.ahLst = new int[]{2, 48, 51, 64, 88, 128, 1, 49, 51, 76, 84, 26, 2, 50, 51, 52, 140, 128};
        data.ahLst = vec![2, 48, 51, 64, 88, 128, 1, 49, 51, 76, 84, 26, 2, 50, 51, 52, 140, 128];
        // Java: data.cxnLst = new int[]{51, 0, 112, 20, 88, 1, 20, 84, 116, 19, 26, 108, 15, 84, 26, 15, 88, 120};
        data.cxnLst = vec![51, 0, 112, 20, 88, 1, 20, 84, 116, 19, 26, 108, 15, 84, 26, 15, 88, 120];
        // Java: data.rect = new int[]{84, 100, 88, 104};
        data.rect = vec![84, 100, 88, 104];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 108, 1, 84, 26, 1, 84, 100, 1, 2, 100, 3, 43, 132, 15, 19, 3, 43, 132, 15, 59, 1, 88, 128, 1, 88, 120, 1, 0, 112, 1, 88, 1, 1, 88, 104, 1, 2, 104, 3, 43, 132, 20, 20, 1, 136, 124, 1, 84, 124, 1, 84, 116, 2}, {7, 12, 0, 140, 144, 3, 43, 132, 51, 20, 3, 43, 132, 15, 59, 1, 140, 128, 2}, {10, 13, 0, 26, 108, 1, 84, 26, 1, 84, 100, 1, 2, 100, 3, 43, 132, 15, 19, 3, 43, 132, 15, 59, 1, 88, 128, 1, 88, 120, 1, 0, 112, 1, 88, 1, 1, 88, 104, 1, 2, 104, 3, 43, 132, 20, 20, 1, 136, 124, 1, 84, 124, 1, 84, 116, 2, 0, 140, 144, 1, 140, 128, 0, 136, 148, 1, 136, 124}};
        data.pathLst = vec![vec![11, 12, 0, 26, 108, 1, 84, 26, 1, 84, 100, 1, 2, 100, 3, 43, 132, 15, 19, 3, 43, 132, 15, 59, 1, 88, 128, 1, 88, 120, 1, 0, 112, 1, 88, 1, 1, 88, 104, 1, 2, 104, 3, 43, 132, 20, 20, 1, 136, 124, 1, 84, 124, 1, 84, 116, 2], vec![7, 12, 0, 140, 144, 3, 43, 132, 51, 20, 3, 43, 132, 15, 59, 1, 140, 128, 2], vec![10, 13, 0, 26, 108, 1, 84, 26, 1, 84, 100, 1, 2, 100, 3, 43, 132, 15, 19, 3, 43, 132, 15, 59, 1, 88, 128, 1, 88, 120, 1, 0, 112, 1, 88, 1, 1, 88, 104, 1, 2, 104, 3, 43, 132, 20, 20, 1, 136, 124, 1, 84, 124, 1, 84, 116, 2, 0, 140, 144, 1, 140, 128, 0, 136, 148, 1, 136, 124]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_leftRightUpArrow()
    fn get_leftRightUpArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 25000, 25000, 0, 50000, 2, 1, 100000, 200000};
        data.avcLst = vec![25000, 25000, 25000, 0, 50000, 2, 1, 100000, 200000];
        // Java: data.gdLst = new int[]{196625, 51, 49, 52, 196609, 57, 53, 54, 196625, 51, 48, 61, 196608, 55, 51, 61, 196609, 69, 54, 53, 196625, 51, 50, 73, 196609, 28, 77, 55, 196609, 28, 57, 55, 196608, 2, 51, 85, 196608, 2, 85, 51, 196609, 28, 65, 56, 196608, 2, 51, 97, 196608, 2, 97, 51, 196608, 0, 51, 81, 196609, 28, 57, 52, 196608, 1, 51, 113, 196608, 1, 51, 85, 196608, 121, 51, 97, 196608, 121, 97, 51, 196609, 97, 81, 85, 196608, 0, 51, 133};
        data.gdLst = vec![196625, 51, 49, 52, 196609, 57, 53, 54, 196625, 51, 48, 61, 196608, 55, 51, 61, 196609, 69, 54, 53, 196625, 51, 50, 73, 196609, 28, 77, 55, 196609, 28, 57, 55, 196608, 2, 51, 85, 196608, 2, 85, 51, 196609, 28, 65, 56, 196608, 2, 51, 97, 196608, 2, 97, 51, 196608, 0, 51, 81, 196609, 28, 57, 52, 196608, 1, 51, 113, 196608, 1, 51, 85, 196608, 121, 51, 97, 196608, 121, 97, 51, 196609, 97, 81, 85, 196608, 0, 51, 133];
        // Java: data.ahLst = new int[]{1, 48, 51, 61, 101, 81, 1, 49, 51, 52, 89, 26, 2, 50, 51, 73, 0, 81};
        data.ahLst = vec![1, 48, 51, 61, 101, 81, 1, 49, 51, 52, 89, 26, 2, 50, 51, 73, 0, 81];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 121, 20, 2, 129, 51, 0, 121};
        data.cxnLst = vec![15, 2, 26, 19, 26, 121, 20, 2, 129, 51, 0, 121];
        // Java: data.rect = new int[]{133, 125, 137, 129};
        data.rect = vec![133, 125, 137, 129];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 121, 1, 81, 117, 1, 81, 125, 1, 101, 125, 1, 101, 81, 1, 89, 81, 1, 2, 26, 1, 93, 81, 1, 105, 81, 1, 105, 125, 1, 109, 125, 1, 109, 117, 1, 0, 121, 1, 109, 1, 1, 109, 129, 1, 81, 129, 1, 81, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 121, 1, 81, 117, 1, 81, 125, 1, 101, 125, 1, 101, 81, 1, 89, 81, 1, 2, 26, 1, 93, 81, 1, 105, 81, 1, 105, 125, 1, 109, 125, 1, 109, 117, 1, 0, 121, 1, 109, 1, 1, 109, 129, 1, 81, 129, 1, 81, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_leftUpArrow()
    fn get_leftUpArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 25000, 25000, 0, 50000, 2, 1, 100000, 200000};
        data.avcLst = vec![25000, 25000, 25000, 0, 50000, 2, 1, 100000, 200000];
        // Java: data.gdLst = new int[]{196625, 51, 49, 52, 196609, 57, 53, 54, 196625, 51, 48, 61, 196608, 55, 51, 61, 196625, 51, 50, 69, 196609, 28, 73, 55, 196609, 28, 57, 52, 196608, 0, 51, 81, 196608, 1, 51, 81, 196609, 28, 57, 55, 196608, 0, 51, 93, 196608, 1, 51, 93, 196609, 28, 65, 56, 196608, 97, 51, 105, 196608, 97, 105, 51, 196608, 101, 51, 105, 196608, 101, 105, 51, 196609, 105, 77, 93, 196610, 77, 113, 53, 196610, 77, 121, 53};
        data.gdLst = vec![196625, 51, 49, 52, 196609, 57, 53, 54, 196625, 51, 48, 61, 196608, 55, 51, 61, 196625, 51, 50, 69, 196609, 28, 73, 55, 196609, 28, 57, 52, 196608, 0, 51, 81, 196608, 1, 51, 81, 196609, 28, 57, 55, 196608, 0, 51, 93, 196608, 1, 51, 93, 196609, 28, 65, 56, 196608, 97, 51, 105, 196608, 97, 105, 51, 196608, 101, 51, 105, 196608, 101, 105, 51, 196609, 105, 77, 93, 196610, 77, 113, 53, 196610, 77, 121, 53];
        // Java: data.ahLst = new int[]{2, 48, 51, 61, 109, 117, 1, 49, 51, 52, 85, 26, 2, 50, 51, 69, 109, 77};
        data.ahLst = vec![2, 48, 51, 61, 109, 117, 1, 49, 51, 52, 85, 26, 2, 50, 51, 69, 109, 77];
        // Java: data.cxnLst = new int[]{15, 97, 26, 19, 85, 77, 15, 77, 89, 19, 26, 101, 20, 77, 1, 20, 129, 121, 51, 113, 133, 51, 0, 77};
        data.cxnLst = vec![15, 97, 26, 19, 85, 77, 15, 77, 89, 19, 26, 101, 20, 77, 1, 20, 129, 121, 51, 113, 133, 51, 0, 77];
        // Java: data.rect = new int[]{125, 117, 97, 121};
        data.rect = vec![125, 117, 97, 121];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 101, 1, 77, 89, 1, 77, 117, 1, 109, 117, 1, 109, 77, 1, 85, 77, 1, 97, 26, 1, 0, 77, 1, 113, 77, 1, 113, 121, 1, 77, 121, 1, 77, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 101, 1, 77, 89, 1, 77, 117, 1, 109, 117, 1, 109, 77, 1, 85, 77, 1, 97, 26, 1, 0, 77, 1, 113, 77, 1, 113, 121, 1, 77, 121, 1, 77, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_lightningBolt()
    fn get_lightningBolt() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{5022, 21600, 8472, 8757, 10012, 12860, 13917, 16577, 3890, 6080, 7437, 9705, 12007, 14277, 14915, 0, 11050, 6797, 14767, 12877, 12222, 13987, 7602, 8382};
        data.avcLst = vec![5022, 21600, 8472, 8757, 10012, 12860, 13917, 16577, 3890, 6080, 7437, 9705, 12007, 14277, 14915, 0, 11050, 6797, 14767, 12877, 12222, 13987, 7602, 8382];
        // Java: data.gdLst = new int[]{196609, 0, 48, 49, 196609, 0, 50, 49, 196609, 0, 51, 49, 196609, 0, 52, 49, 196609, 0, 53, 49, 196609, 0, 54, 49, 196609, 0, 55, 49, 196609, 1, 56, 49, 196609, 1, 57, 49, 196609, 1, 58, 49, 196609, 1, 59, 49, 196609, 1, 60, 49, 196609, 1, 61, 49, 196609, 1, 62, 49, 196609, 0, 50, 49, 196609, 1, 63, 49, 196609, 0, 53, 49, 196609, 1, 57, 49, 196609, 0, 64, 49, 196609, 1, 65, 49, 196609, 0, 55, 49, 196609, 1, 60, 49, 196609, 0, 66, 49, 196609, 1, 67, 49, 196609, 0, 49, 49, 196609, 1, 49, 49, 196609, 0, 52, 49, 196609, 1, 62, 49, 196609, 0, 68, 49, 196609, 1, 69, 49, 196609, 0, 48, 49, 196609, 1, 59, 49, 196609, 0, 70, 49, 196609, 1, 71, 49, 196609, 0, 63, 49, 196609, 1, 56, 49};
        data.gdLst = vec![196609, 0, 48, 49, 196609, 0, 50, 49, 196609, 0, 51, 49, 196609, 0, 52, 49, 196609, 0, 53, 49, 196609, 0, 54, 49, 196609, 0, 55, 49, 196609, 1, 56, 49, 196609, 1, 57, 49, 196609, 1, 58, 49, 196609, 1, 59, 49, 196609, 1, 60, 49, 196609, 1, 61, 49, 196609, 1, 62, 49, 196609, 0, 50, 49, 196609, 1, 63, 49, 196609, 0, 53, 49, 196609, 1, 57, 49, 196609, 0, 64, 49, 196609, 1, 65, 49, 196609, 0, 55, 49, 196609, 1, 60, 49, 196609, 0, 66, 49, 196609, 1, 67, 49, 196609, 0, 49, 49, 196609, 1, 49, 49, 196609, 0, 52, 49, 196609, 1, 62, 49, 196609, 0, 68, 49, 196609, 1, 69, 49, 196609, 0, 48, 49, 196609, 1, 59, 49, 196609, 0, 70, 49, 196609, 1, 71, 49, 196609, 0, 63, 49, 196609, 1, 56, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 76, 26, 15, 26, 100, 19, 72, 112, 19, 84, 124, 20, 0, 1, 63, 96, 116, 63, 88, 104};
        data.cxnLst = vec![15, 76, 26, 15, 26, 100, 19, 72, 112, 19, 84, 124, 20, 0, 1, 63, 96, 116, 63, 88, 104];
        // Java: data.rect = new int[]{80, 108, 92, 120};
        data.rect = vec![80, 108, 92, 120];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 128, 132, 1, 136, 140, 1, 144, 148, 1, 152, 156, 1, 160, 164, 1, 168, 172, 1, 176, 180, 1, 184, 188, 1, 192, 196, 1, 200, 204, 1, 208, 212, 2}};
        data.pathLst = vec![vec![11, 13, 0, 128, 132, 1, 136, 140, 1, 144, 148, 1, 152, 156, 1, 160, 164, 1, 168, 172, 1, 176, 180, 1, 184, 188, 1, 192, 196, 1, 200, 204, 1, 208, 212, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_line()
    fn get_line() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[0];
        data.avcLst = vec![];
        // Java: data.gdLst = new int[0];
        data.gdLst = vec![];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{20, 26, 26, 15, 0, 1};
        data.cxnLst = vec![20, 26, 26, 15, 0, 1];
        // Java: data.rect = new int[0];
        data.rect = vec![];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 1}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 1]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_lineInv()
    fn get_lineInv() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[0];
        data.avcLst = vec![];
        // Java: data.gdLst = new int[0];
        data.gdLst = vec![];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{20, 26, 1, 15, 0, 26};
        data.cxnLst = vec![20, 26, 1, 15, 0, 26];
        // Java: data.rect = new int[0];
        data.rect = vec![];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 1, 1, 0, 26}};
        data.pathLst = vec![vec![11, 13, 0, 26, 1, 1, 0, 26]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_mathDivide()
    fn get_mathDivide() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{23520, 5880, 11760, 1000, 36745, 0, 73490, 4, -4, 1, 200000, 100000, 21600000};
        data.avcLst = vec![23520, 5880, 11760, 1000, 36745, 0, 73490, 4, -4, 1, 200000, 100000, 21600000];
        // Java: data.gdLst = new int[]{196625, 51, 48, 52, 196608, 53, 53, 61, 196610, 54, 65, 55, 196609, 52, 0, 1, 131076, 69, 73, 196625, 51, 50, 77, 196609, 56, 80, 57, 196608, 54, 84, 61, 196625, 53, 49, 88, 196609, 1, 61, 58, 196609, 1, 92, 59, 196609, 1, 80, 59, 196609, 0, 54, 58, 196608, 3, 53, 96, 196608, 3, 96, 53, 196608, 100, 104, 53, 196608, 112, 53, 120, 196608, 124, 53, 104, 196608, 1, 53, 128, 196608, 2, 53, 108, 196608, 2, 108, 53, 196608, 2, 53, 104};
        data.gdLst = vec![196625, 51, 48, 52, 196608, 53, 53, 61, 196610, 54, 65, 55, 196609, 52, 0, 1, 131076, 69, 73, 196625, 51, 50, 77, 196609, 56, 80, 57, 196608, 54, 84, 61, 196625, 53, 49, 88, 196609, 1, 61, 58, 196609, 1, 92, 59, 196609, 1, 80, 59, 196609, 0, 54, 58, 196608, 3, 53, 96, 196608, 3, 96, 53, 196608, 100, 104, 53, 196608, 112, 53, 120, 196608, 124, 53, 104, 196608, 1, 53, 128, 196608, 2, 53, 108, 196608, 2, 108, 53, 196608, 2, 53, 104];
        // Java: data.ahLst = new int[]{2, 48, 51, 52, 26, 112, 2, 49, 53, 88, 0, 124, 1, 50, 51, 77, 144, 26};
        data.ahLst = vec![2, 48, 51, 52, 26, 112, 2, 49, 53, 88, 0, 124, 1, 50, 51, 77, 144, 26];
        // Java: data.cxnLst = new int[]{53, 140, 3, 20, 2, 132, 19, 136, 3, 15, 2, 128};
        data.cxnLst = vec![53, 140, 3, 20, 2, 132, 19, 136, 3, 15, 2, 128];
        // Java: data.rect = new int[]{136, 112, 140, 116};
        data.rect = vec![136, 112, 140, 116];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 2, 128, 3, 104, 104, 15, 60, 2, 0, 2, 132, 3, 104, 104, 20, 60, 2, 0, 136, 112, 1, 140, 112, 1, 140, 116, 1, 136, 116, 2}};
        data.pathLst = vec![vec![11, 13, 0, 2, 128, 3, 104, 104, 15, 60, 2, 0, 2, 132, 3, 104, 104, 20, 60, 2, 0, 136, 112, 1, 140, 112, 1, 140, 116, 1, 136, 116, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_mathEqual()
    fn get_mathEqual() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{23520, 11760, 0, 36745, 2, 1, 100000, 200000, 73490};
        data.avcLst = vec![23520, 11760, 0, 36745, 2, 1, 100000, 200000, 73490];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196609, 57, 52, 53, 196608, 54, 50, 61, 196625, 50, 49, 65, 196609, 1, 57, 54, 196609, 1, 69, 55, 196609, 0, 56, 55, 196608, 3, 50, 77, 196608, 3, 77, 50, 196608, 85, 50, 73, 196608, 89, 73, 50, 196608, 2, 50, 81, 196608, 2, 81, 50, 196610, 93, 85, 52, 196610, 89, 97, 52};
        data.gdLst = vec![196625, 50, 48, 51, 196609, 57, 52, 53, 196608, 54, 50, 61, 196625, 50, 49, 65, 196609, 1, 57, 54, 196609, 1, 69, 55, 196609, 0, 56, 55, 196608, 3, 50, 77, 196608, 3, 77, 50, 196608, 85, 50, 73, 196608, 89, 73, 50, 196608, 2, 50, 81, 196608, 2, 81, 50, 196610, 93, 85, 52, 196610, 89, 97, 52];
        // Java: data.ahLst = new int[]{2, 48, 50, 51, 26, 93, 2, 49, 50, 65, 0, 85};
        data.ahLst = vec![2, 48, 50, 51, 26, 93, 2, 49, 50, 65, 0, 85];
        // Java: data.cxnLst = new int[]{50, 105, 109, 50, 105, 113, 20, 2, 97, 19, 101, 109, 19, 101, 113, 15, 2, 93};
        data.cxnLst = vec![50, 105, 109, 50, 105, 113, 20, 2, 97, 19, 101, 109, 19, 101, 113, 15, 2, 93];
        // Java: data.rect = new int[]{101, 93, 105, 97};
        data.rect = vec![101, 93, 105, 97];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 101, 93, 1, 105, 93, 1, 105, 85, 1, 101, 85, 2, 0, 101, 89, 1, 105, 89, 1, 105, 97, 1, 101, 97, 2}};
        data.pathLst = vec![vec![11, 13, 0, 101, 93, 1, 105, 93, 1, 105, 85, 1, 101, 85, 2, 0, 101, 89, 1, 105, 89, 1, 105, 97, 1, 101, 97, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_mathMinus()
    fn get_mathMinus() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{23520, 0, 100000, 200000, 73490};
        data.avcLst = vec![23520, 0, 100000, 200000, 73490];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 1, 53, 51, 196609, 0, 52, 51, 196608, 3, 49, 57, 196608, 3, 57, 49, 196608, 2, 49, 61, 196608, 2, 61, 49};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 1, 53, 51, 196609, 0, 52, 51, 196608, 3, 49, 57, 196608, 3, 57, 49, 196608, 2, 49, 61, 196608, 2, 61, 49];
        // Java: data.ahLst = new int[]{2, 48, 49, 50, 26, 65};
        data.ahLst = vec![2, 48, 49, 50, 26, 65];
        // Java: data.cxnLst = new int[]{49, 77, 3, 20, 2, 69, 19, 73, 3, 15, 2, 65};
        data.cxnLst = vec![49, 77, 3, 20, 2, 69, 19, 73, 3, 15, 2, 65];
        // Java: data.rect = new int[]{73, 65, 77, 69};
        data.rect = vec![73, 65, 77, 69];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 73, 65, 1, 77, 65, 1, 77, 69, 1, 73, 69, 2}};
        data.pathLst = vec![vec![11, 13, 0, 73, 65, 1, 77, 65, 1, 77, 69, 1, 73, 69, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_mathMultiply()
    fn get_mathMultiply() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{23520, 0, 51965, 100000, 1, 2};
        data.avcLst = vec![23520, 0, 51965, 100000, 1, 2];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 54, 51, 131091, 0, 1, 131092, 52, 62, 131093, 52, 62, 131094, 52, 62, 196615, 0, 1, 49, 196609, 74, 50, 51, 196608, 74, 49, 78, 196609, 68, 82, 53, 196609, 65, 82, 53, 196609, 65, 58, 53, 196609, 68, 58, 53, 196608, 86, 49, 94, 196608, 90, 98, 49, 196608, 86, 94, 49, 196608, 90, 49, 98, 196608, 2, 49, 110, 196609, 118, 71, 52, 196608, 122, 114, 49, 196608, 0, 49, 110, 196608, 0, 49, 102, 196608, 3, 49, 106, 196609, 138, 52, 71, 196608, 134, 49, 142, 196608, 102, 142, 49, 196608, 1, 49, 106, 196608, 1, 49, 114, 196608, 1, 49, 126, 196608, 0, 49, 86, 196608, 1, 49, 90};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 54, 51, 131091, 0, 1, 131092, 52, 62, 131093, 52, 62, 131094, 52, 62, 196615, 0, 1, 49, 196609, 74, 50, 51, 196608, 74, 49, 78, 196609, 68, 82, 53, 196609, 65, 82, 53, 196609, 65, 58, 53, 196609, 68, 58, 53, 196608, 86, 49, 94, 196608, 90, 98, 49, 196608, 86, 94, 49, 196608, 90, 49, 98, 196608, 2, 49, 110, 196609, 118, 71, 52, 196608, 122, 114, 49, 196608, 0, 49, 110, 196608, 0, 49, 102, 196608, 3, 49, 106, 196609, 138, 52, 71, 196608, 134, 49, 142, 196608, 102, 142, 49, 196608, 1, 49, 106, 196608, 1, 49, 114, 196608, 1, 49, 126, 196608, 0, 49, 86, 196608, 1, 49, 90];
        // Java: data.ahLst = new int[]{2, 48, 49, 50, 26, 58};
        data.ahLst = vec![2, 48, 49, 50, 26, 58];
        // Java: data.cxnLst = new int[]{19, 86, 90, 15, 166, 90, 49, 166, 170, 20, 86, 170};
        data.cxnLst = vec![19, 86, 90, 15, 166, 90, 49, 166, 170, 20, 86, 170];
        // Java: data.rect = new int[]{102, 114, 134, 158};
        data.rect = vec![102, 114, 134, 158];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 102, 106, 1, 110, 114, 1, 2, 126, 1, 130, 114, 1, 134, 106, 1, 146, 3, 1, 134, 154, 1, 130, 158, 1, 2, 162, 1, 110, 158, 1, 102, 154, 1, 150, 3, 2}};
        data.pathLst = vec![vec![11, 13, 0, 102, 106, 1, 110, 114, 1, 2, 126, 1, 130, 114, 1, 134, 106, 1, 146, 3, 1, 134, 154, 1, 130, 158, 1, 2, 162, 1, 110, 158, 1, 102, 154, 1, 150, 3, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_mathNotEqual()
    fn get_mathNotEqual() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{23520, 6600000, 11760, 0, 50000, 4200000, 6600000, 2, 1, 100000, 200000, 73490};
        data.avcLst = vec![23520, 6600000, 11760, 0, 50000, 4200000, 6600000, 2, 1, 100000, 200000, 73490];
        // Java: data.gdLst = new int[]{196625, 51, 48, 52, 196625, 53, 49, 54, 196609, 60, 55, 56, 196608, 57, 51, 68, 196625, 51, 50, 72, 196609, 1, 60, 57, 196609, 1, 76, 58, 196609, 0, 59, 58, 196608, 2, 51, 88, 196608, 2, 88, 51, 196608, 3, 51, 84, 196608, 3, 84, 51, 196608, 100, 51, 80, 196608, 104, 80, 51, 196608, 64, 51, 20, 131094, 3, 116, 196615, 120, 3, 51, 196609, 123, 80, 3, 196609, 127, 56, 55, 196608, 2, 120, 131, 196609, 120, 108, 3, 196608, 135, 51, 139, 196609, 120, 100, 3, 196608, 135, 51, 147, 196609, 120, 104, 3, 196608, 135, 51, 155, 196609, 120, 112, 3, 196608, 135, 51, 163, 196609, 120, 55, 56, 196608, 135, 51, 171, 196608, 135, 127, 51, 196608, 143, 127, 51, 196608, 151, 127, 51, 196608, 159, 127, 51, 196608, 167, 127, 51, 196608, 175, 127, 51, 196609, 80, 3, 123, 196608, 135, 203, 51, 196608, 179, 51, 203, 196614, 116, 207, 179, 196614, 116, 135, 211, 196609, 80, 120, 123, 196608, 51, 51, 223, 196614, 116, 223, 26, 196614, 116, 26, 227, 196608, 0, 51, 215, 196608, 0, 51, 219, 196608, 1, 51, 231, 196608, 1, 51, 235, 196610, 215, 219, 55, 196610, 243, 239, 55, 196610, 231, 235, 55, 196610, 108, 100, 55, 196610, 104, 112, 55, 196610, 251, 247, 55};
        data.gdLst = vec![196625, 51, 48, 52, 196625, 53, 49, 54, 196609, 60, 55, 56, 196608, 57, 51, 68, 196625, 51, 50, 72, 196609, 1, 60, 57, 196609, 1, 76, 58, 196609, 0, 59, 58, 196608, 2, 51, 88, 196608, 2, 88, 51, 196608, 3, 51, 84, 196608, 3, 84, 51, 196608, 100, 51, 80, 196608, 104, 80, 51, 196608, 64, 51, 20, 131094, 3, 116, 196615, 120, 3, 51, 196609, 123, 80, 3, 196609, 127, 56, 55, 196608, 2, 120, 131, 196609, 120, 108, 3, 196608, 135, 51, 139, 196609, 120, 100, 3, 196608, 135, 51, 147, 196609, 120, 104, 3, 196608, 135, 51, 155, 196609, 120, 112, 3, 196608, 135, 51, 163, 196609, 120, 55, 56, 196608, 135, 51, 171, 196608, 135, 127, 51, 196608, 143, 127, 51, 196608, 151, 127, 51, 196608, 159, 127, 51, 196608, 167, 127, 51, 196608, 175, 127, 51, 196609, 80, 3, 123, 196608, 135, 203, 51, 196608, 179, 51, 203, 196614, 116, 207, 179, 196614, 116, 135, 211, 196609, 80, 120, 123, 196608, 51, 51, 223, 196614, 116, 223, 26, 196614, 116, 26, 227, 196608, 0, 51, 215, 196608, 0, 51, 219, 196608, 1, 51, 231, 196608, 1, 51, 235, 196610, 215, 219, 55, 196610, 243, 239, 55, 196610, 231, 235, 55, 196610, 108, 100, 55, 196610, 104, 112, 55, 196610, 251, 247, 55];
        // Java: data.ahLst = new int[]{2, 48, 51, 52, 26, 108, 4, 49, 53, 54, 219, 26, 2, 50, 51, 72, 0, 100};
        data.ahLst = vec![2, 48, 51, 52, 26, 108, 4, 49, 53, 54, 219, 26, 2, 50, 51, 72, 0, 100];
        // Java: data.cxnLst = new int[]{51, 96, 267, 51, 96, 271, 20, 259, 275, 19, 92, 267, 19, 92, 271, 15, 255, 263};
        data.cxnLst = vec![51, 96, 267, 51, 96, 271, 20, 259, 275, 19, 92, 267, 19, 92, 271, 15, 255, 263];
        // Java: data.rect = new int[]{92, 108, 96, 112};
        data.rect = vec![92, 108, 96, 112];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 92, 108, 1, 143, 108, 1, 219, 235, 1, 215, 231, 1, 183, 108, 1, 96, 108, 1, 96, 100, 1, 187, 100, 1, 191, 104, 1, 96, 104, 1, 96, 112, 1, 195, 112, 1, 243, 251, 1, 239, 247, 1, 167, 112, 1, 92, 112, 1, 92, 104, 1, 159, 104, 1, 151, 100, 1, 92, 100, 2}};
        data.pathLst = vec![vec![11, 13, 0, 92, 108, 1, 143, 108, 1, 219, 235, 1, 215, 231, 1, 183, 108, 1, 96, 108, 1, 96, 100, 1, 187, 100, 1, 191, 104, 1, 96, 104, 1, 96, 112, 1, 195, 112, 1, 243, 251, 1, 239, 247, 1, 167, 112, 1, 92, 112, 1, 92, 104, 1, 159, 104, 1, 151, 100, 1, 92, 100, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_mathPlus()
    fn get_mathPlus() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{23520, 0, 73490, 200000};
        data.avcLst = vec![23520, 0, 73490, 200000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 0, 50, 51, 196609, 1, 50, 51, 196609, 28, 52, 51, 196608, 2, 49, 56, 196608, 2, 49, 64, 196608, 2, 64, 49, 196608, 2, 56, 49, 196608, 3, 49, 60, 196608, 3, 49, 64, 196608, 3, 64, 49, 196608, 3, 60, 49};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 0, 50, 51, 196609, 1, 50, 51, 196609, 28, 52, 51, 196608, 2, 49, 56, 196608, 2, 49, 64, 196608, 2, 64, 49, 196608, 2, 56, 49, 196608, 3, 49, 60, 196608, 3, 49, 64, 196608, 3, 64, 49, 196608, 3, 60, 49];
        // Java: data.ahLst = new int[]{2, 48, 49, 50, 26, 88};
        data.ahLst = vec![2, 48, 49, 50, 26, 88];
        // Java: data.cxnLst = new int[]{49, 80, 3, 20, 2, 96, 19, 68, 3, 15, 2, 84};
        data.cxnLst = vec![49, 80, 3, 20, 2, 96, 19, 68, 3, 15, 2, 84];
        // Java: data.rect = new int[]{68, 88, 80, 92};
        data.rect = vec![68, 88, 80, 92];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 68, 88, 1, 72, 88, 1, 72, 84, 1, 76, 84, 1, 76, 88, 1, 80, 88, 1, 80, 92, 1, 76, 92, 1, 76, 96, 1, 72, 96, 1, 72, 92, 1, 68, 92, 2}};
        data.pathLst = vec![vec![11, 13, 0, 68, 88, 1, 72, 88, 1, 72, 84, 1, 76, 84, 1, 76, 88, 1, 80, 88, 1, 80, 92, 1, 76, 92, 1, 76, 96, 1, 72, 96, 1, 72, 92, 1, 68, 92, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_moon()
    fn get_moon() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 0, 87500, 100000, 2, 1, 9598, 32768, -1, 21600000};
        data.avcLst = vec![50000, 0, 87500, 100000, 2, 1, 9598, 32768, -1, 21600000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 58, 51, 196609, 62, 0, 28, 196608, 28, 49, 62, 196609, 62, 62, 70, 196609, 28, 28, 70, 196609, 78, 52, 53, 196608, 82, 49, 74, 196608, 86, 49, 62, 196609, 90, 0, 28, 196609, 86, 53, 52, 196608, 98, 49, 62, 196609, 102, 3, 28, 196608, 3, 49, 106, 196608, 3, 106, 49, 196609, 62, 54, 55, 196609, 118, 0, 28, 196608, 28, 49, 118, 196609, 28, 28, 53, 196609, 126, 126, 53, 196608, 130, 49, 134, 65549, 138, 196609, 142, 3, 28, 196608, 3, 49, 144, 196608, 3, 144, 49, 196608, 94, 49, 66, 196609, 156, 53, 52, 196608, 66, 160, 0, 196609, 164, 56, 53, 196609, 3, 56, 53, 131091, 168, 172, 131091, 168, 3, 196608, 179, 49, 57, 196608, 182, 49, 176};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 58, 51, 196609, 62, 0, 28, 196608, 28, 49, 62, 196609, 62, 62, 70, 196609, 28, 28, 70, 196609, 78, 52, 53, 196608, 82, 49, 74, 196608, 86, 49, 62, 196609, 90, 0, 28, 196609, 86, 53, 52, 196608, 98, 49, 62, 196609, 102, 3, 28, 196608, 3, 49, 106, 196608, 3, 106, 49, 196609, 62, 54, 55, 196609, 118, 0, 28, 196608, 28, 49, 118, 196609, 28, 28, 53, 196609, 126, 126, 53, 196608, 130, 49, 134, 65549, 138, 196609, 142, 3, 28, 196608, 3, 49, 144, 196608, 3, 144, 49, 196608, 94, 49, 66, 196609, 156, 53, 52, 196608, 66, 160, 0, 196609, 164, 56, 53, 196609, 3, 56, 53, 131091, 168, 172, 131091, 168, 3, 196608, 179, 49, 57, 196608, 182, 49, 176];
        // Java: data.ahLst = new int[]{1, 48, 49, 50, 66, 3};
        data.ahLst = vec![1, 48, 49, 50, 66, 3];
        // Java: data.cxnLst = new int[]{15, 0, 26, 19, 26, 3, 20, 0, 1, 49, 66, 3};
        data.cxnLst = vec![15, 0, 26, 19, 26, 3, 20, 0, 1, 49, 66, 3];
        // Java: data.rect = new int[]{122, 148, 66, 152};
        data.rect = vec![122, 148, 66, 152];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 0, 1, 3, 0, 3, 20, 19, 3, 160, 106, 176, 186, 2}};
        data.pathLst = vec![vec![11, 13, 0, 0, 1, 3, 0, 3, 20, 19, 3, 160, 106, 176, 186, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_nonIsoscelesTrapezoid()
    fn get_nonIsoscelesTrapezoid() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 25000, 50000, 0, 200000, 100000, 2};
        data.avcLst = vec![25000, 25000, 50000, 0, 200000, 100000, 2];
        // Java: data.gdLst = new int[]{196609, 50, 0, 28, 196625, 51, 48, 55, 196625, 51, 49, 55, 196609, 28, 59, 52, 196609, 28, 59, 53, 196609, 28, 63, 53, 196608, 0, 51, 75, 196610, 0, 79, 54, 196609, 39, 59, 55, 131077, 59, 63, 196609, 38, 91, 55, 196609, 39, 63, 55, 196608, 0, 51, 98};
        data.gdLst = vec![196609, 50, 0, 28, 196625, 51, 48, 55, 196625, 51, 49, 55, 196609, 28, 59, 52, 196609, 28, 59, 53, 196609, 28, 63, 53, 196608, 0, 51, 75, 196610, 0, 79, 54, 196609, 39, 59, 55, 131077, 59, 63, 196609, 38, 91, 55, 196609, 39, 63, 55, 196608, 0, 51, 98];
        // Java: data.ahLst = new int[]{1, 48, 51, 55, 71, 26, 1, 49, 51, 55, 79, 26};
        data.ahLst = vec![1, 48, 51, 55, 71, 26, 1, 49, 51, 55, 79, 26];
        // Java: data.cxnLst = new int[]{51, 83, 3, 20, 2, 1, 19, 67, 3, 15, 2, 26};
        data.cxnLst = vec![51, 83, 3, 20, 2, 1, 19, 67, 3, 15, 2, 26];
        // Java: data.rect = new int[]{87, 94, 102, 1};
        data.rect = vec![87, 94, 102, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 1, 1, 71, 26, 1, 79, 26, 1, 0, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 1, 1, 71, 26, 1, 79, 26, 1, 0, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_noSmoking()
    fn get_noSmoking() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{18750, 0, 50000, 100000, 1, 2, -10800000, 2700000};
        data.avcLst = vec![18750, 0, 50000, 100000, 1, 2, -10800000, 2700000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 56, 51, 196608, 2, 49, 60, 196608, 3, 49, 60, 131091, 0, 1, 131093, 68, 72, 131092, 64, 72, 196615, 75, 78, 49, 196609, 64, 68, 81, 196609, 60, 52, 53, 131091, 85, 89, 196609, 93, 53, 52, 196608, 54, 96, 49, 131091, 0, 1, 196608, 104, 49, 93, 196608, 107, 49, 19, 131093, 68, 107, 131092, 64, 107, 196615, 115, 118, 49, 196609, 64, 68, 121, 131093, 125, 107, 131092, 125, 107, 196608, 2, 129, 49, 196608, 3, 132, 49, 196608, 2, 49, 129, 196608, 3, 49, 132, 131093, 2, 55, 131092, 3, 55, 196608, 2, 49, 151, 196608, 2, 151, 49, 196608, 3, 49, 154, 196608, 3, 154, 49};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 56, 51, 196608, 2, 49, 60, 196608, 3, 49, 60, 131091, 0, 1, 131093, 68, 72, 131092, 64, 72, 196615, 75, 78, 49, 196609, 64, 68, 81, 196609, 60, 52, 53, 131091, 85, 89, 196609, 93, 53, 52, 196608, 54, 96, 49, 131091, 0, 1, 196608, 104, 49, 93, 196608, 107, 49, 19, 131093, 68, 107, 131092, 64, 107, 196615, 115, 118, 49, 196609, 64, 68, 121, 131093, 125, 107, 131092, 125, 107, 196608, 2, 129, 49, 196608, 3, 132, 49, 196608, 2, 49, 129, 196608, 3, 49, 132, 131093, 2, 55, 131092, 3, 55, 196608, 2, 49, 151, 196608, 2, 151, 49, 196608, 3, 49, 154, 196608, 3, 154, 49];
        // Java: data.ahLst = new int[]{5, 48, 49, 50, 60, 3};
        data.ahLst = vec![5, 48, 49, 50, 60, 3];
        // Java: data.cxnLst = new int[]{15, 2, 26, 15, 157, 165, 19, 26, 3, 20, 157, 169, 20, 2, 1, 20, 161, 169, 49, 0, 3, 15, 161, 165};
        data.cxnLst = vec![15, 2, 26, 15, 157, 165, 19, 26, 3, 20, 157, 169, 20, 2, 1, 20, 161, 169, 49, 0, 3, 15, 161, 165];
        // Java: data.rect = new int[]{157, 165, 161, 169};
        data.rect = vec![157, 165, 161, 169];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2, 0, 135, 139, 3, 64, 68, 107, 100, 2, 0, 143, 147, 3, 64, 68, 111, 100, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 3, 2, 3, 19, 20, 3, 2, 3, 15, 20, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2, 0, 135, 139, 3, 64, 68, 107, 100, 2, 0, 143, 147, 3, 64, 68, 111, 100, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_notchedRightArrow()
    fn get_notchedRightArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 50000, 100000, 0, 200000};
        data.avcLst = vec![50000, 50000, 100000, 0, 200000];
        // Java: data.gdLst = new int[]{196609, 50, 0, 28, 196625, 51, 48, 50, 196625, 51, 49, 53, 196609, 28, 61, 50, 196608, 0, 51, 65, 196609, 1, 57, 52, 196608, 3, 51, 73, 196608, 3, 73, 51, 196609, 73, 65, 3, 196608, 0, 51, 85};
        data.gdLst = vec![196609, 50, 0, 28, 196625, 51, 48, 50, 196625, 51, 49, 53, 196609, 28, 61, 50, 196608, 0, 51, 65, 196609, 1, 57, 52, 196608, 3, 51, 73, 196608, 3, 73, 51, 196609, 73, 65, 3, 196608, 0, 51, 85];
        // Java: data.ahLst = new int[]{2, 48, 51, 50, 0, 77, 1, 49, 51, 53, 69, 26};
        data.ahLst = vec![2, 48, 51, 50, 0, 77, 1, 49, 51, 53, 69, 26];
        // Java: data.cxnLst = new int[]{15, 69, 26, 19, 85, 3, 20, 69, 1, 51, 0, 3};
        data.cxnLst = vec![15, 69, 26, 19, 85, 3, 20, 69, 1, 51, 0, 3];
        // Java: data.rect = new int[]{85, 77, 89, 81};
        data.rect = vec![85, 77, 89, 81];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 77, 1, 69, 77, 1, 69, 26, 1, 0, 3, 1, 69, 1, 1, 69, 81, 1, 26, 81, 1, 85, 3, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 77, 1, 69, 77, 1, 69, 26, 1, 0, 3, 1, 69, 1, 1, 69, 81, 1, 26, 81, 1, 85, 3, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_octagon()
    fn get_octagon() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{29289, 0, 50000, 100000, 1, 2};
        data.avcLst = vec![29289, 0, 50000, 100000, 1, 2];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 54, 51, 196608, 0, 49, 58, 196608, 1, 49, 58, 196609, 58, 52, 53, 196608, 0, 49, 70, 196608, 1, 49, 70};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 54, 51, 196608, 0, 49, 58, 196608, 1, 49, 58, 196609, 58, 52, 53, 196608, 0, 49, 70, 196608, 1, 49, 70];
        // Java: data.ahLst = new int[]{1, 48, 49, 50, 58, 26};
        data.ahLst = vec![1, 48, 49, 50, 58, 26];
        // Java: data.cxnLst = new int[]{49, 0, 58, 49, 0, 66, 20, 62, 1, 20, 58, 1, 19, 26, 66, 19, 26, 58, 15, 58, 26, 15, 62, 26};
        data.cxnLst = vec![49, 0, 58, 49, 0, 66, 20, 62, 1, 20, 58, 1, 19, 26, 66, 19, 26, 58, 15, 58, 26, 15, 62, 26];
        // Java: data.rect = new int[]{70, 70, 74, 78};
        data.rect = vec![70, 70, 74, 78];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 58, 1, 58, 26, 1, 62, 26, 1, 0, 58, 1, 0, 66, 1, 62, 1, 1, 58, 1, 1, 26, 66, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 58, 1, 58, 26, 1, 62, 26, 1, 0, 58, 1, 0, 66, 1, 62, 1, 1, 58, 1, 1, 26, 66, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_parallelogram()
    fn get_parallelogram() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 100000, 0, 200000, 1, 2, 5, 12};
        data.avcLst = vec![25000, 100000, 0, 200000, 1, 2, 5, 12];
        // Java: data.gdLst = new int[]{196609, 49, 0, 28, 196625, 50, 48, 56, 196609, 28, 60, 51, 196609, 28, 60, 49, 196608, 0, 50, 64, 196608, 0, 50, 68, 196609, 76, 52, 53, 196608, 0, 50, 80, 196609, 2, 60, 56, 196609, 54, 60, 56, 196610, 52, 92, 55, 196609, 96, 0, 52, 196609, 96, 1, 52, 196608, 0, 50, 100, 196608, 1, 50, 104, 196609, 1, 2, 68, 196625, 50, 116, 1, 196608, 1, 50, 120};
        data.gdLst = vec![196609, 49, 0, 28, 196625, 50, 48, 56, 196609, 28, 60, 51, 196609, 28, 60, 49, 196608, 0, 50, 64, 196608, 0, 50, 68, 196609, 76, 52, 53, 196608, 0, 50, 80, 196609, 2, 60, 56, 196609, 54, 60, 56, 196610, 52, 92, 55, 196609, 96, 0, 52, 196609, 96, 1, 52, 196608, 0, 50, 100, 196608, 1, 50, 104, 196609, 1, 2, 68, 196625, 50, 116, 1, 196608, 1, 50, 120];
        // Java: data.ahLst = new int[]{1, 48, 50, 56, 68, 26};
        data.ahLst = vec![1, 48, 50, 56, 68, 26];
        // Java: data.cxnLst = new int[]{15, 2, 124, 15, 84, 26, 50, 72, 3, 20, 80, 1, 20, 2, 120, 19, 64, 3};
        data.cxnLst = vec![15, 2, 124, 15, 84, 26, 50, 72, 3, 20, 80, 1, 20, 2, 120, 19, 64, 3];
        // Java: data.rect = new int[]{100, 104, 108, 112};
        data.rect = vec![100, 104, 108, 112];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 1, 1, 68, 26, 1, 0, 26, 1, 76, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 1, 1, 68, 26, 1, 0, 26, 1, 76, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_pentagon()
    fn get_pentagon() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{105146, 110557, 100000, 1080000, 18360000, 0};
        data.avcLst = vec![105146, 110557, 100000, 1080000, 18360000, 0];
        // Java: data.gdLst = new int[]{196609, 2, 48, 50, 196609, 3, 49, 50, 196609, 3, 49, 50, 131093, 54, 51, 131093, 54, 52, 131092, 58, 51, 131092, 58, 52, 196608, 2, 53, 66, 196608, 2, 53, 69, 196608, 2, 69, 53, 196608, 2, 66, 53, 196608, 62, 53, 72, 196608, 62, 53, 75, 196609, 94, 69, 66};
        data.gdLst = vec![196609, 2, 48, 50, 196609, 3, 49, 50, 196609, 3, 49, 50, 131093, 54, 51, 131093, 54, 52, 131092, 58, 51, 131092, 58, 52, 196608, 2, 53, 66, 196608, 2, 53, 69, 196608, 2, 69, 53, 196608, 2, 66, 53, 196608, 62, 53, 72, 196608, 62, 53, 75, 196609, 94, 69, 66];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 78, 94, 20, 82, 98, 20, 2, 1, 20, 86, 98, 53, 90, 94};
        data.cxnLst = vec![15, 2, 26, 19, 78, 94, 20, 82, 98, 20, 2, 1, 20, 86, 98, 53, 90, 94];
        // Java: data.rect = new int[]{82, 102, 86, 98};
        data.rect = vec![82, 102, 86, 98];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 78, 94, 1, 2, 26, 1, 90, 94, 1, 86, 98, 1, 82, 98, 2}};
        data.pathLst = vec![vec![11, 13, 0, 78, 94, 1, 2, 26, 1, 90, 94, 1, 86, 98, 1, 82, 98, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_pie()
    fn get_pie() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{0, 16200000, 0, 21599999, 21600000, 2700000};
        data.avcLst = vec![0, 16200000, 0, 21599999, 21600000, 2700000];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196625, 50, 49, 51, 196608, 58, 50, 54, 196608, 62, 52, 50, 196614, 62, 62, 66, 131092, 2, 54, 131093, 3, 54, 196619, 2, 77, 74, 196620, 3, 77, 74, 196608, 2, 80, 50, 196608, 3, 84, 50, 131092, 2, 58, 131093, 3, 58, 196619, 2, 99, 96, 196620, 3, 99, 96, 196608, 2, 102, 50, 196608, 3, 106, 50, 131093, 2, 53, 131092, 3, 53, 196608, 2, 50, 118, 196608, 2, 118, 50, 196608, 3, 50, 121, 196608, 3, 121, 50};
        data.gdLst = vec![196625, 50, 48, 51, 196625, 50, 49, 51, 196608, 58, 50, 54, 196608, 62, 52, 50, 196614, 62, 62, 66, 131092, 2, 54, 131093, 3, 54, 196619, 2, 77, 74, 196620, 3, 77, 74, 196608, 2, 80, 50, 196608, 3, 84, 50, 131092, 2, 58, 131093, 3, 58, 196619, 2, 99, 96, 196620, 3, 99, 96, 196608, 2, 102, 50, 196608, 3, 106, 50, 131093, 2, 53, 131092, 3, 53, 196608, 2, 50, 118, 196608, 2, 118, 50, 196608, 3, 50, 121, 196608, 3, 121, 50];
        // Java: data.ahLst = new int[]{4, 48, 50, 51, 88, 92, 4, 49, 50, 51, 110, 114};
        data.ahLst = vec![4, 48, 50, 51, 88, 92, 4, 49, 50, 51, 110, 114];
        // Java: data.cxnLst = new int[]{50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{124, 132, 128, 136};
        data.rect = vec![124, 132, 128, 136];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 88, 92, 3, 2, 3, 54, 70, 1, 2, 3, 2}};
        data.pathLst = vec![vec![11, 13, 0, 88, 92, 3, 2, 3, 54, 70, 1, 2, 3, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_pieWedge()
    fn get_pieWedge() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{13500000, 0};
        data.avcLst = vec![13500000, 0];
        // Java: data.gdLst = new int[]{131093, 0, 48, 131092, 1, 48, 196608, 0, 50, 49, 196608, 1, 53, 49};
        data.gdLst = vec![131093, 0, 48, 131092, 1, 48, 196608, 0, 50, 49, 196608, 1, 53, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{49, 0, 3, 20, 2, 1};
        data.cxnLst = vec![49, 0, 3, 20, 2, 1];
        // Java: data.rect = new int[]{56, 60, 0, 1};
        data.rect = vec![56, 60, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 1, 3, 0, 1, 19, 20, 1, 0, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 1, 3, 0, 1, 19, 20, 1, 0, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_plaque()
    fn get_plaque() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{16667, 0, 50000, 100000, 70711, -5400000};
        data.avcLst = vec![16667, 0, 50000, 100000, 70711, -5400000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 54, 51, 196608, 0, 49, 58, 196608, 1, 49, 58, 196609, 58, 52, 51, 196608, 0, 49, 70, 196608, 1, 49, 70};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 54, 51, 196608, 0, 49, 58, 196608, 1, 49, 58, 196609, 58, 52, 51, 196608, 0, 49, 70, 196608, 1, 49, 70];
        // Java: data.ahLst = new int[]{1, 48, 49, 50, 58, 26};
        data.ahLst = vec![1, 48, 49, 50, 58, 26];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3];
        // Java: data.rect = new int[]{70, 70, 74, 78};
        data.rect = vec![70, 70, 74, 78];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 58, 3, 58, 58, 20, 53, 1, 62, 26, 3, 58, 58, 19, 53, 1, 0, 66, 3, 58, 58, 15, 53, 1, 58, 1, 3, 58, 58, 49, 53, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 58, 3, 58, 58, 20, 53, 1, 62, 26, 3, 58, 58, 19, 53, 1, 0, 66, 3, 58, 58, 15, 53, 1, 58, 1, 3, 58, 58, 49, 53, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_plaqueTabs()
    fn get_plaqueTabs() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{0, 1, 20};
        data.avcLst = vec![0, 1, 20];
        // Java: data.gdLst = new int[]{196615, 0, 1, 48, 196609, 49, 51, 50, 196608, 48, 1, 55, 196608, 48, 0, 55};
        data.gdLst = vec![196615, 0, 1, 48, 196609, 49, 51, 50, 196608, 48, 1, 55, 196608, 48, 0, 55];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{19, 26, 26, 19, 26, 55, 19, 26, 59, 19, 26, 1, 15, 55, 26, 15, 63, 26, 20, 55, 1, 20, 63, 1, 48, 0, 26, 48, 0, 55, 48, 0, 59, 48, 0, 1};
        data.cxnLst = vec![19, 26, 26, 19, 26, 55, 19, 26, 59, 19, 26, 1, 15, 55, 26, 15, 63, 26, 20, 55, 1, 20, 63, 1, 48, 0, 26, 48, 0, 55, 48, 0, 59, 48, 0, 1];
        // Java: data.rect = new int[]{55, 55, 63, 59};
        data.rect = vec![55, 55, 63, 59];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 55, 26, 3, 55, 55, 48, 20, 2}, {11, 13, 0, 26, 59, 3, 55, 55, 15, 20, 1, 26, 1, 2}, {11, 13, 0, 0, 26, 1, 0, 55, 3, 55, 55, 20, 20, 2}, {11, 13, 0, 63, 1, 3, 55, 55, 19, 20, 1, 0, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 55, 26, 3, 55, 55, 48, 20, 2], vec![11, 13, 0, 26, 59, 3, 55, 55, 15, 20, 1, 26, 1, 2], vec![11, 13, 0, 0, 26, 1, 0, 55, 3, 55, 55, 20, 20, 2], vec![11, 13, 0, 63, 1, 3, 55, 55, 19, 20, 1, 0, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_plus()
    fn get_plus() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 0, 50000, 100000};
        data.avcLst = vec![25000, 0, 50000, 100000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 52, 51, 196608, 0, 49, 56, 196608, 1, 49, 56, 196608, 0, 49, 1, 196614, 68, 26, 56, 196614, 68, 0, 60, 196614, 68, 56, 26, 196614, 68, 64, 1};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 52, 51, 196608, 0, 49, 56, 196608, 1, 49, 56, 196608, 0, 49, 1, 196614, 68, 26, 56, 196614, 68, 0, 60, 196614, 68, 56, 26, 196614, 68, 64, 1];
        // Java: data.ahLst = new int[]{1, 48, 49, 50, 56, 26};
        data.ahLst = vec![1, 48, 49, 50, 56, 26];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3];
        // Java: data.rect = new int[]{72, 80, 76, 84};
        data.rect = vec![72, 80, 76, 84];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 56, 1, 56, 56, 1, 56, 26, 1, 60, 26, 1, 60, 56, 1, 0, 56, 1, 0, 64, 1, 60, 64, 1, 60, 1, 1, 56, 1, 1, 56, 64, 1, 26, 64, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 56, 1, 56, 56, 1, 56, 26, 1, 60, 26, 1, 60, 56, 1, 0, 56, 1, 0, 64, 1, 60, 64, 1, 60, 1, 1, 56, 1, 1, 56, 64, 1, 26, 64, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_quadArrow()
    fn get_quadArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{22500, 22500, 22500, 0, 50000, 2, 1, 100000, 200000};
        data.avcLst = vec![22500, 22500, 22500, 0, 50000, 2, 1, 100000, 200000];
        // Java: data.gdLst = new int[]{196625, 51, 49, 52, 196609, 57, 53, 54, 196625, 51, 48, 61, 196608, 55, 51, 61, 196609, 69, 54, 53, 196625, 51, 50, 73, 196609, 28, 77, 55, 196609, 28, 57, 55, 196608, 2, 51, 85, 196608, 2, 85, 51, 196609, 28, 65, 56, 196608, 2, 51, 97, 196608, 2, 97, 51, 196608, 0, 51, 81, 196608, 3, 51, 85, 196608, 3, 85, 51, 196608, 3, 51, 97, 196608, 3, 97, 51, 196608, 1, 51, 81, 196609, 97, 81, 85, 196608, 0, 51, 133};
        data.gdLst = vec![196625, 51, 49, 52, 196609, 57, 53, 54, 196625, 51, 48, 61, 196608, 55, 51, 61, 196609, 69, 54, 53, 196625, 51, 50, 73, 196609, 28, 77, 55, 196609, 28, 57, 55, 196608, 2, 51, 85, 196608, 2, 85, 51, 196609, 28, 65, 56, 196608, 2, 51, 97, 196608, 2, 97, 51, 196608, 0, 51, 81, 196608, 3, 51, 85, 196608, 3, 85, 51, 196608, 3, 51, 97, 196608, 3, 97, 51, 196608, 1, 51, 81, 196609, 97, 81, 85, 196608, 0, 51, 133];
        // Java: data.ahLst = new int[]{1, 48, 51, 61, 101, 81, 1, 49, 51, 52, 89, 26, 2, 50, 51, 73, 0, 81};
        data.ahLst = vec![1, 48, 51, 61, 101, 81, 1, 49, 51, 52, 89, 26, 2, 50, 51, 73, 0, 81];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 51, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 51, 0, 3];
        // Java: data.rect = new int[]{133, 121, 137, 125};
        data.rect = vec![133, 121, 137, 125];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 1, 81, 113, 1, 81, 121, 1, 101, 121, 1, 101, 81, 1, 89, 81, 1, 2, 26, 1, 93, 81, 1, 105, 81, 1, 105, 121, 1, 109, 121, 1, 109, 113, 1, 0, 3, 1, 109, 117, 1, 109, 125, 1, 105, 125, 1, 105, 129, 1, 93, 129, 1, 2, 1, 1, 89, 129, 1, 101, 129, 1, 101, 125, 1, 81, 125, 1, 81, 117, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 1, 81, 113, 1, 81, 121, 1, 101, 121, 1, 101, 81, 1, 89, 81, 1, 2, 26, 1, 93, 81, 1, 105, 81, 1, 105, 121, 1, 109, 121, 1, 109, 113, 1, 0, 3, 1, 109, 117, 1, 109, 125, 1, 105, 125, 1, 105, 129, 1, 93, 129, 1, 2, 1, 1, 89, 129, 1, 101, 129, 1, 101, 125, 1, 81, 125, 1, 81, 117, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_quadArrowCallout()
    fn get_quadArrowCallout() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{18515, 18515, 18515, 48123, 0, 50000, 2, 1, 100000, 200000};
        data.avcLst = vec![18515, 18515, 18515, 48123, 0, 50000, 2, 1, 100000, 200000];
        // Java: data.gdLst = new int[]{196625, 52, 49, 53, 196609, 58, 54, 55, 196625, 52, 48, 62, 196608, 53, 52, 58, 196625, 52, 50, 70, 196609, 74, 54, 55, 196608, 56, 52, 78, 196625, 66, 51, 82, 196609, 28, 58, 56, 196609, 28, 66, 57, 196609, 28, 74, 56, 196609, 0, 86, 57, 196609, 1, 86, 57, 196608, 0, 52, 98, 196608, 2, 52, 102, 196608, 2, 102, 52, 196608, 2, 52, 90, 196608, 2, 90, 52, 196608, 2, 52, 94, 196608, 2, 94, 52, 196608, 1, 52, 98, 196608, 3, 52, 106, 196608, 3, 106, 52, 196608, 3, 52, 90, 196608, 3, 90, 52, 196608, 3, 52, 94, 196608, 3, 94, 52};
        data.gdLst = vec![196625, 52, 49, 53, 196609, 58, 54, 55, 196625, 52, 48, 62, 196608, 53, 52, 58, 196625, 52, 50, 70, 196609, 74, 54, 55, 196608, 56, 52, 78, 196625, 66, 51, 82, 196609, 28, 58, 56, 196609, 28, 66, 57, 196609, 28, 74, 56, 196609, 0, 86, 57, 196609, 1, 86, 57, 196608, 0, 52, 98, 196608, 2, 52, 102, 196608, 2, 102, 52, 196608, 2, 52, 90, 196608, 2, 90, 52, 196608, 2, 52, 94, 196608, 2, 94, 52, 196608, 1, 52, 98, 196608, 3, 52, 106, 196608, 3, 106, 52, 196608, 3, 52, 90, 196608, 3, 90, 52, 196608, 3, 52, 94, 196608, 3, 94, 52];
        // Java: data.ahLst = new int[]{1, 48, 52, 62, 130, 98, 1, 49, 52, 53, 122, 26, 2, 50, 52, 70, 0, 98, 2, 51, 66, 82, 26, 142};
        data.ahLst = vec![1, 48, 52, 62, 130, 98, 1, 49, 52, 53, 122, 26, 2, 50, 52, 70, 0, 98, 2, 51, 66, 82, 26, 142];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 52, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 52, 0, 3];
        // Java: data.rect = new int[]{114, 142, 118, 146};
        data.rect = vec![114, 142, 118, 146];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 1, 98, 150, 1, 98, 158, 1, 114, 158, 1, 114, 142, 1, 130, 142, 1, 130, 98, 1, 122, 98, 1, 2, 26, 1, 126, 98, 1, 134, 98, 1, 134, 142, 1, 118, 142, 1, 118, 158, 1, 110, 158, 1, 110, 150, 1, 0, 3, 1, 110, 154, 1, 110, 162, 1, 118, 162, 1, 118, 146, 1, 134, 146, 1, 134, 138, 1, 126, 138, 1, 2, 1, 1, 122, 138, 1, 130, 138, 1, 130, 146, 1, 114, 146, 1, 114, 162, 1, 98, 162, 1, 98, 154, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 1, 98, 150, 1, 98, 158, 1, 114, 158, 1, 114, 142, 1, 130, 142, 1, 130, 98, 1, 122, 98, 1, 2, 26, 1, 126, 98, 1, 134, 98, 1, 134, 142, 1, 118, 142, 1, 118, 158, 1, 110, 158, 1, 110, 150, 1, 0, 3, 1, 110, 154, 1, 110, 162, 1, 118, 162, 1, 118, 146, 1, 134, 146, 1, 134, 138, 1, 126, 138, 1, 2, 1, 1, 122, 138, 1, 130, 138, 1, 130, 146, 1, 114, 146, 1, 114, 162, 1, 98, 162, 1, 98, 154, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_rect()
    fn get_rect() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{0};
        data.avcLst = vec![0];
        // Java: data.gdLst = new int[0];
        data.gdLst = vec![];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 48, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 48, 0, 3];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 0, 26, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_ribbon()
    fn get_ribbon() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{16667, 50000, 0, 33333, 25000, 75000, 200000, 100000, 1, 2, 400000, -10800000, -5400000};
        data.avcLst = vec![16667, 50000, 0, 33333, 25000, 75000, 200000, 100000, 1, 2, 400000, -10800000, -5400000];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196625, 52, 49, 53, 196608, 0, 50, 36, 196609, 0, 65, 54, 196608, 2, 50, 73, 196608, 2, 73, 50, 196608, 77, 43, 50, 196608, 81, 50, 43, 196608, 77, 36, 50, 196608, 81, 50, 36, 196608, 93, 50, 43, 196608, 97, 43, 50, 196609, 1, 61, 54, 196609, 1, 61, 55, 196608, 1, 50, 113, 196609, 117, 56, 57, 196609, 1, 61, 58, 196608, 1, 50, 125, 196608, 113, 50, 125};
        data.gdLst = vec![196625, 50, 48, 51, 196625, 52, 49, 53, 196608, 0, 50, 36, 196609, 0, 65, 54, 196608, 2, 50, 73, 196608, 2, 73, 50, 196608, 77, 43, 50, 196608, 81, 50, 43, 196608, 77, 36, 50, 196608, 81, 50, 36, 196608, 93, 50, 43, 196608, 97, 43, 50, 196609, 1, 61, 54, 196609, 1, 61, 55, 196608, 1, 50, 113, 196609, 117, 56, 57, 196609, 1, 61, 58, 196608, 1, 50, 125, 196608, 113, 50, 125];
        // Java: data.ahLst = new int[]{2, 48, 50, 51, 2, 113, 1, 49, 52, 53, 77, 26};
        data.ahLst = vec![2, 48, 50, 51, 2, 113, 1, 49, 52, 53, 77, 26];
        // Java: data.cxnLst = new int[]{15, 2, 113, 19, 36, 121, 20, 2, 1, 50, 69, 121};
        data.cxnLst = vec![15, 2, 113, 19, 36, 121, 20, 2, 1, 50, 69, 121];
        // Java: data.rect = new int[]{77, 113, 81, 1};
        data.rect = vec![77, 113, 81, 1];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 1, 101, 26, 3, 43, 125, 15, 19, 1, 85, 109, 3, 43, 125, 15, 59, 1, 89, 113, 3, 43, 125, 20, 59, 1, 105, 109, 3, 43, 125, 20, 19, 1, 0, 26, 1, 69, 121, 1, 0, 117, 1, 81, 117, 1, 81, 129, 3, 43, 125, 50, 20, 1, 85, 1, 3, 43, 125, 20, 20, 1, 77, 117, 1, 26, 117, 1, 36, 121, 2}, {7, 12, 0, 93, 125, 3, 43, 125, 50, 20, 1, 85, 109, 3, 43, 125, 15, 59, 1, 93, 113, 2, 0, 97, 125, 3, 43, 125, 19, 60, 1, 89, 109, 3, 43, 125, 15, 19, 1, 97, 113, 2}, {10, 13, 0, 26, 26, 1, 101, 26, 3, 43, 125, 15, 19, 1, 85, 109, 3, 43, 125, 15, 59, 1, 89, 113, 3, 43, 125, 20, 59, 1, 105, 109, 3, 43, 125, 20, 19, 1, 0, 26, 1, 69, 121, 1, 0, 117, 1, 81, 117, 1, 81, 129, 3, 43, 125, 50, 20, 1, 85, 1, 3, 43, 125, 20, 20, 1, 77, 117, 1, 26, 117, 1, 36, 121, 2, 0, 93, 125, 1, 93, 113, 0, 97, 113, 1, 97, 125, 0, 77, 117, 1, 77, 133, 0, 81, 133, 1, 81, 117}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 1, 101, 26, 3, 43, 125, 15, 19, 1, 85, 109, 3, 43, 125, 15, 59, 1, 89, 113, 3, 43, 125, 20, 59, 1, 105, 109, 3, 43, 125, 20, 19, 1, 0, 26, 1, 69, 121, 1, 0, 117, 1, 81, 117, 1, 81, 129, 3, 43, 125, 50, 20, 1, 85, 1, 3, 43, 125, 20, 20, 1, 77, 117, 1, 26, 117, 1, 36, 121, 2], vec![7, 12, 0, 93, 125, 3, 43, 125, 50, 20, 1, 85, 109, 3, 43, 125, 15, 59, 1, 93, 113, 2, 0, 97, 125, 3, 43, 125, 19, 60, 1, 89, 109, 3, 43, 125, 15, 19, 1, 97, 113, 2], vec![10, 13, 0, 26, 26, 1, 101, 26, 3, 43, 125, 15, 19, 1, 85, 109, 3, 43, 125, 15, 59, 1, 89, 113, 3, 43, 125, 20, 59, 1, 105, 109, 3, 43, 125, 20, 19, 1, 0, 26, 1, 69, 121, 1, 0, 117, 1, 81, 117, 1, 81, 129, 3, 43, 125, 50, 20, 1, 85, 1, 3, 43, 125, 20, 20, 1, 77, 117, 1, 26, 117, 1, 36, 121, 2, 0, 93, 125, 1, 93, 113, 0, 97, 113, 1, 97, 125, 0, 77, 117, 1, 77, 133, 0, 81, 133, 1, 81, 117]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_ribbon2()
    fn get_ribbon2() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{16667, 50000, 0, 33333, 25000, 75000, 200000, 100000, 2, 400000, -10800000, -5400000};
        data.avcLst = vec![16667, 50000, 0, 33333, 25000, 75000, 200000, 100000, 2, 400000, -10800000, -5400000];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196625, 52, 49, 53, 196608, 0, 50, 36, 196609, 0, 64, 54, 196608, 2, 50, 72, 196608, 2, 72, 50, 196608, 76, 43, 50, 196608, 80, 50, 43, 196608, 76, 36, 50, 196608, 80, 50, 36, 196608, 92, 50, 43, 196608, 96, 43, 50, 196609, 1, 60, 54, 196608, 1, 50, 108, 196609, 1, 60, 55, 196608, 1, 50, 116, 196608, 26, 116, 50, 196610, 124, 1, 56, 196609, 1, 60, 57, 196608, 1, 50, 132, 196608, 112, 50, 132};
        data.gdLst = vec![196625, 50, 48, 51, 196625, 52, 49, 53, 196608, 0, 50, 36, 196609, 0, 64, 54, 196608, 2, 50, 72, 196608, 2, 72, 50, 196608, 76, 43, 50, 196608, 80, 50, 43, 196608, 76, 36, 50, 196608, 80, 50, 36, 196608, 92, 50, 43, 196608, 96, 43, 50, 196609, 1, 60, 54, 196608, 1, 50, 108, 196609, 1, 60, 55, 196608, 1, 50, 116, 196608, 26, 116, 50, 196610, 124, 1, 56, 196609, 1, 60, 57, 196608, 1, 50, 132, 196608, 112, 50, 132];
        // Java: data.ahLst = new int[]{2, 48, 50, 51, 2, 120, 1, 49, 52, 53, 76, 1};
        data.ahLst = vec![2, 48, 50, 51, 2, 120, 1, 49, 52, 53, 76, 1];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 36, 128, 20, 2, 120, 50, 68, 128};
        data.cxnLst = vec![15, 2, 26, 19, 36, 128, 20, 2, 120, 50, 68, 128];
        // Java: data.rect = new int[]{76, 26, 80, 120};
        data.rect = vec![76, 26, 80, 120];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 1, 1, 100, 1, 3, 43, 132, 20, 58, 1, 84, 112, 3, 43, 132, 20, 19, 1, 88, 120, 3, 43, 132, 15, 19, 1, 104, 112, 3, 43, 132, 15, 58, 1, 0, 1, 1, 68, 128, 1, 0, 124, 1, 80, 124, 1, 80, 132, 3, 43, 132, 50, 59, 1, 84, 26, 3, 43, 132, 15, 59, 1, 76, 124, 1, 26, 124, 1, 36, 128, 2}, {7, 12, 0, 92, 136, 3, 43, 132, 50, 59, 1, 84, 112, 3, 43, 132, 20, 19, 1, 92, 120, 2, 0, 96, 136, 3, 43, 132, 19, 20, 1, 88, 112, 3, 43, 132, 20, 58, 1, 96, 120, 2}, {10, 13, 0, 26, 1, 1, 36, 128, 1, 26, 124, 1, 76, 124, 1, 76, 132, 3, 43, 132, 19, 20, 1, 88, 26, 3, 43, 132, 15, 20, 1, 80, 124, 1, 80, 124, 1, 0, 124, 1, 68, 128, 1, 0, 1, 1, 104, 1, 3, 43, 132, 20, 19, 1, 88, 112, 3, 43, 132, 20, 58, 1, 84, 120, 3, 43, 132, 15, 58, 1, 100, 112, 3, 43, 132, 15, 19, 2, 0, 92, 120, 1, 92, 136, 0, 96, 136, 1, 96, 120, 0, 76, 140, 1, 76, 124, 0, 80, 124, 1, 80, 140}};
        data.pathLst = vec![vec![11, 12, 0, 26, 1, 1, 100, 1, 3, 43, 132, 20, 58, 1, 84, 112, 3, 43, 132, 20, 19, 1, 88, 120, 3, 43, 132, 15, 19, 1, 104, 112, 3, 43, 132, 15, 58, 1, 0, 1, 1, 68, 128, 1, 0, 124, 1, 80, 124, 1, 80, 132, 3, 43, 132, 50, 59, 1, 84, 26, 3, 43, 132, 15, 59, 1, 76, 124, 1, 26, 124, 1, 36, 128, 2], vec![7, 12, 0, 92, 136, 3, 43, 132, 50, 59, 1, 84, 112, 3, 43, 132, 20, 19, 1, 92, 120, 2, 0, 96, 136, 3, 43, 132, 19, 20, 1, 88, 112, 3, 43, 132, 20, 58, 1, 96, 120, 2], vec![10, 13, 0, 26, 1, 1, 36, 128, 1, 26, 124, 1, 76, 124, 1, 76, 132, 3, 43, 132, 19, 20, 1, 88, 26, 3, 43, 132, 15, 20, 1, 80, 124, 1, 80, 124, 1, 0, 124, 1, 68, 128, 1, 0, 1, 1, 104, 1, 3, 43, 132, 20, 19, 1, 88, 112, 3, 43, 132, 20, 58, 1, 84, 120, 3, 43, 132, 15, 58, 1, 100, 112, 3, 43, 132, 15, 19, 2, 0, 92, 120, 1, 92, 136, 0, 96, 136, 1, 96, 120, 0, 76, 140, 1, 76, 124, 0, 80, 124, 1, 80, 140]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_rightArrow()
    fn get_rightArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 50000, 100000, 0, 200000};
        data.avcLst = vec![50000, 50000, 100000, 0, 200000];
        // Java: data.gdLst = new int[]{196609, 50, 0, 28, 196625, 51, 48, 50, 196625, 51, 49, 53, 196609, 28, 61, 50, 196608, 0, 51, 65, 196609, 1, 57, 52, 196608, 3, 51, 73, 196608, 3, 73, 51, 196609, 77, 65, 3, 196608, 69, 85, 51};
        data.gdLst = vec![196609, 50, 0, 28, 196625, 51, 48, 50, 196625, 51, 49, 53, 196609, 28, 61, 50, 196608, 0, 51, 65, 196609, 1, 57, 52, 196608, 3, 51, 73, 196608, 3, 73, 51, 196609, 77, 65, 3, 196608, 69, 85, 51];
        // Java: data.ahLst = new int[]{2, 48, 51, 50, 26, 77, 1, 49, 51, 53, 69, 26};
        data.ahLst = vec![2, 48, 51, 50, 26, 77, 1, 49, 51, 53, 69, 26];
        // Java: data.cxnLst = new int[]{15, 69, 26, 19, 26, 3, 20, 69, 1, 51, 0, 3};
        data.cxnLst = vec![15, 69, 26, 19, 26, 3, 20, 69, 1, 51, 0, 3];
        // Java: data.rect = new int[]{26, 77, 89, 81};
        data.rect = vec![26, 77, 89, 81];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 77, 1, 69, 77, 1, 69, 26, 1, 0, 3, 1, 69, 1, 1, 69, 81, 1, 26, 81, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 77, 1, 69, 77, 1, 69, 26, 1, 0, 3, 1, 69, 1, 1, 69, 81, 1, 26, 81, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_rightArrowCallout()
    fn get_rightArrowCallout() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 25000, 25000, 64977, 50000, 0, 2, 1, 100000, 200000};
        data.avcLst = vec![25000, 25000, 25000, 64977, 50000, 0, 2, 1, 100000, 200000];
        // Java: data.gdLst = new int[]{196609, 52, 1, 28, 196625, 53, 49, 58, 196609, 62, 54, 55, 196625, 53, 48, 66, 196609, 56, 0, 28, 196625, 53, 50, 74, 196609, 78, 28, 0, 196608, 56, 53, 82, 196625, 53, 51, 86, 196609, 28, 62, 56, 196609, 28, 70, 57, 196608, 3, 53, 94, 196608, 3, 53, 98, 196608, 3, 98, 53, 196608, 3, 94, 53, 196609, 28, 78, 56, 196608, 0, 53, 118, 196609, 0, 90, 56, 196609, 126, 55, 54};
        data.gdLst = vec![196609, 52, 1, 28, 196625, 53, 49, 58, 196609, 62, 54, 55, 196625, 53, 48, 66, 196609, 56, 0, 28, 196625, 53, 50, 74, 196609, 78, 28, 0, 196608, 56, 53, 82, 196625, 53, 51, 86, 196609, 28, 62, 56, 196609, 28, 70, 57, 196608, 3, 53, 94, 196608, 3, 53, 98, 196608, 3, 98, 53, 196608, 3, 94, 53, 196609, 28, 78, 56, 196608, 0, 53, 118, 196609, 0, 90, 56, 196609, 126, 55, 54];
        // Java: data.ahLst = new int[]{2, 48, 53, 66, 122, 106, 2, 49, 53, 58, 0, 102, 1, 50, 53, 74, 122, 26, 1, 51, 53, 86, 126, 1};
        data.ahLst = vec![2, 48, 53, 66, 122, 106, 2, 49, 53, 58, 0, 102, 1, 50, 53, 74, 122, 26, 1, 51, 53, 86, 126, 1];
        // Java: data.cxnLst = new int[]{15, 130, 26, 19, 26, 3, 20, 130, 1, 53, 0, 3};
        data.cxnLst = vec![15, 130, 26, 19, 26, 3, 20, 130, 1, 53, 0, 3];
        // Java: data.rect = new int[]{26, 26, 126, 1};
        data.rect = vec![26, 26, 126, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 126, 26, 1, 126, 106, 1, 122, 106, 1, 122, 102, 1, 0, 3, 1, 122, 114, 1, 122, 110, 1, 126, 110, 1, 126, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 126, 26, 1, 126, 106, 1, 122, 106, 1, 122, 102, 1, 0, 3, 1, 122, 114, 1, 122, 110, 1, 126, 110, 1, 126, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_rightBrace()
    fn get_rightBrace() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{8333, 50000, 0, 100000, 1, 2, 2700000, -5400000};
        data.avcLst = vec![8333, 50000, 0, 100000, 1, 2, 2700000, -5400000];
        // Java: data.gdLst = new int[]{196625, 50, 49, 51, 196608, 51, 50, 56, 131076, 60, 56, 196609, 64, 52, 53, 196609, 67, 1, 28, 196625, 50, 48, 71, 196609, 28, 75, 51, 196609, 1, 56, 51, 196608, 83, 50, 79, 196608, 1, 50, 79, 131093, 2, 54, 131092, 79, 54, 196608, 26, 95, 50, 196608, 79, 50, 98, 196608, 1, 98, 79};
        data.gdLst = vec![196625, 50, 49, 51, 196608, 51, 50, 56, 131076, 60, 56, 196609, 64, 52, 53, 196609, 67, 1, 28, 196625, 50, 48, 71, 196609, 28, 75, 51, 196609, 1, 56, 51, 196608, 83, 50, 79, 196608, 1, 50, 79, 131093, 2, 54, 131092, 79, 54, 196608, 26, 95, 50, 196608, 79, 50, 98, 196608, 1, 98, 79];
        // Java: data.ahLst = new int[]{2, 48, 50, 71, 2, 79, 2, 49, 50, 51, 0, 83};
        data.ahLst = vec![2, 48, 50, 71, 2, 79, 2, 49, 50, 51, 0, 83];
        // Java: data.cxnLst = new int[]{20, 26, 26, 19, 0, 83, 15, 26, 1};
        data.cxnLst = vec![20, 26, 26, 19, 0, 83, 15, 26, 1];
        // Java: data.rect = new int[]{26, 105, 101, 109};
        data.rect = vec![26, 105, 101, 109];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 3, 2, 79, 15, 20, 1, 2, 87, 3, 2, 79, 19, 55, 3, 2, 79, 15, 55, 1, 2, 91, 3, 2, 79, 50, 20, 2}, {10, 13, 0, 26, 26, 3, 2, 79, 15, 20, 1, 2, 87, 3, 2, 79, 19, 55, 3, 2, 79, 15, 55, 1, 2, 91, 3, 2, 79, 50, 20}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 3, 2, 79, 15, 20, 1, 2, 87, 3, 2, 79, 19, 55, 3, 2, 79, 15, 55, 1, 2, 91, 3, 2, 79, 50, 20, 2], vec![10, 13, 0, 26, 26, 3, 2, 79, 15, 20, 1, 2, 87, 3, 2, 79, 19, 55, 3, 2, 79, 15, 55, 1, 2, 91, 3, 2, 79, 50, 20]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_rightBracket()
    fn get_rightBracket() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{8333, 50000, 0, 100000, 2700000};
        data.avcLst = vec![8333, 50000, 0, 100000, 2700000];
        // Java: data.gdLst = new int[]{196609, 49, 1, 28, 196625, 50, 48, 53, 196609, 28, 57, 51, 196608, 1, 50, 61, 131093, 0, 52, 131092, 61, 52, 196608, 26, 69, 50, 196608, 61, 50, 72, 196608, 1, 72, 61};
        data.gdLst = vec![196609, 49, 1, 28, 196625, 50, 48, 53, 196609, 28, 57, 51, 196608, 1, 50, 61, 131093, 0, 52, 131092, 61, 52, 196608, 26, 69, 50, 196608, 61, 50, 72, 196608, 1, 72, 61];
        // Java: data.ahLst = new int[]{2, 48, 50, 53, 0, 61};
        data.ahLst = vec![2, 48, 50, 53, 0, 61];
        // Java: data.cxnLst = new int[]{20, 26, 26, 15, 26, 1, 19, 0, 3};
        data.cxnLst = vec![20, 26, 26, 15, 26, 1, 19, 0, 3];
        // Java: data.rect = new int[]{26, 79, 75, 83};
        data.rect = vec![26, 79, 75, 83];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 26, 3, 0, 61, 15, 20, 1, 0, 65, 3, 0, 61, 50, 20, 2}, {10, 13, 0, 26, 26, 3, 0, 61, 15, 20, 1, 0, 65, 3, 0, 61, 50, 20}};
        data.pathLst = vec![vec![11, 12, 0, 26, 26, 3, 0, 61, 15, 20, 1, 0, 65, 3, 0, 61, 50, 20, 2], vec![10, 13, 0, 26, 26, 3, 0, 61, 15, 20, 1, 0, 65, 3, 0, 61, 50, 20]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_round1Rect()
    fn get_round1Rect() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{16667, 0, 50000, 100000, 29289};
        data.avcLst = vec![16667, 0, 50000, 100000, 29289];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 53, 51, 196608, 0, 49, 57, 196609, 57, 52, 51, 196608, 0, 49, 65};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 53, 51, 196608, 0, 49, 57, 196609, 57, 52, 51, 196608, 0, 49, 65];
        // Java: data.ahLst = new int[]{1, 48, 49, 50, 61, 26};
        data.ahLst = vec![1, 48, 49, 50, 61, 26];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3];
        // Java: data.rect = new int[]{26, 26, 69, 1};
        data.rect = vec![26, 26, 69, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 61, 26, 3, 57, 57, 15, 20, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 61, 26, 3, 57, 57, 15, 20, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_round2DiagRect()
    fn get_round2DiagRect() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{16667, 0, 0, 50000, 100000, 29289};
        data.avcLst = vec![16667, 0, 0, 50000, 100000, 29289];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196625, 50, 49, 51, 196609, 28, 54, 52, 196608, 1, 50, 62, 196609, 28, 58, 52, 196608, 0, 50, 70, 196608, 1, 50, 70, 196609, 62, 53, 52, 196609, 70, 53, 52, 196608, 82, 50, 86, 196614, 90, 82, 86, 196608, 0, 50, 94, 196608, 1, 50, 94};
        data.gdLst = vec![196625, 50, 48, 51, 196625, 50, 49, 51, 196609, 28, 54, 52, 196608, 1, 50, 62, 196609, 28, 58, 52, 196608, 0, 50, 70, 196608, 1, 50, 70, 196609, 62, 53, 52, 196609, 70, 53, 52, 196608, 82, 50, 86, 196614, 90, 82, 86, 196608, 0, 50, 94, 196608, 1, 50, 94];
        // Java: data.ahLst = new int[]{1, 48, 50, 51, 62, 26, 1, 49, 50, 51, 74, 26};
        data.ahLst = vec![1, 48, 50, 51, 62, 26, 1, 49, 50, 51, 74, 26];
        // Java: data.cxnLst = new int[]{50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{94, 94, 98, 102};
        data.rect = vec![94, 94, 98, 102];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 62, 26, 1, 74, 26, 3, 70, 70, 15, 20, 1, 0, 66, 3, 62, 62, 50, 20, 1, 70, 1, 3, 70, 70, 20, 20, 1, 26, 62, 3, 62, 62, 19, 20, 2}};
        data.pathLst = vec![vec![11, 13, 0, 62, 26, 1, 74, 26, 3, 70, 70, 15, 20, 1, 0, 66, 3, 62, 62, 50, 20, 1, 70, 1, 3, 70, 70, 20, 20, 1, 26, 62, 3, 62, 62, 19, 20, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_round2SameRect()
    fn get_round2SameRect() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{16667, 0, 0, 50000, 100000, 29289};
        data.avcLst = vec![16667, 0, 0, 50000, 100000, 29289];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196625, 50, 49, 51, 196609, 28, 54, 52, 196608, 0, 50, 62, 196609, 28, 58, 52, 196608, 0, 50, 70, 196608, 1, 50, 70, 196608, 62, 50, 70, 196609, 62, 53, 52, 196609, 70, 53, 52, 196614, 82, 86, 90, 196608, 0, 50, 94, 196608, 1, 50, 90};
        data.gdLst = vec![196625, 50, 48, 51, 196625, 50, 49, 51, 196609, 28, 54, 52, 196608, 0, 50, 62, 196609, 28, 58, 52, 196608, 0, 50, 70, 196608, 1, 50, 70, 196608, 62, 50, 70, 196609, 62, 53, 52, 196609, 70, 53, 52, 196614, 82, 86, 90, 196608, 0, 50, 94, 196608, 1, 50, 90];
        // Java: data.ahLst = new int[]{1, 48, 50, 51, 66, 26, 1, 49, 50, 51, 70, 1};
        data.ahLst = vec![1, 48, 50, 51, 66, 26, 1, 49, 50, 51, 70, 1];
        // Java: data.cxnLst = new int[]{50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{94, 86, 98, 102};
        data.rect = vec![94, 86, 98, 102];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 62, 26, 1, 66, 26, 3, 62, 62, 15, 20, 1, 0, 78, 3, 70, 70, 50, 20, 1, 70, 1, 3, 70, 70, 20, 20, 1, 26, 62, 3, 62, 62, 19, 20, 2}};
        data.pathLst = vec![vec![11, 13, 0, 62, 26, 1, 66, 26, 3, 62, 62, 15, 20, 1, 0, 78, 3, 70, 70, 50, 20, 1, 70, 1, 3, 70, 70, 20, 20, 1, 26, 62, 3, 62, 62, 19, 20, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_roundRect()
    fn get_roundRect() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{16667, 0, 50000, 100000, 29289};
        data.avcLst = vec![16667, 0, 50000, 100000, 29289];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 53, 51, 196608, 0, 49, 57, 196608, 1, 49, 57, 196609, 57, 52, 51, 196608, 0, 49, 69, 196608, 1, 49, 69};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 53, 51, 196608, 0, 49, 57, 196608, 1, 49, 57, 196609, 57, 52, 51, 196608, 0, 49, 69, 196608, 1, 49, 69];
        // Java: data.ahLst = new int[]{1, 48, 49, 50, 57, 26};
        data.ahLst = vec![1, 48, 49, 50, 57, 26];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3];
        // Java: data.rect = new int[]{69, 69, 73, 77};
        data.rect = vec![69, 69, 73, 77];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 57, 3, 57, 57, 19, 20, 1, 61, 26, 3, 57, 57, 15, 20, 1, 0, 65, 3, 57, 57, 49, 20, 1, 57, 1, 3, 57, 57, 20, 20, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 57, 3, 57, 57, 19, 20, 1, 61, 26, 3, 57, 57, 15, 20, 1, 0, 65, 3, 57, 57, 49, 20, 1, 57, 1, 3, 57, 57, 20, 20, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_rtTriangle()
    fn get_rtTriangle() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{7, 12, 11, 0};
        data.avcLst = vec![7, 12, 11, 0];
        // Java: data.gdLst = new int[]{196609, 1, 48, 49, 196609, 0, 48, 49, 196609, 1, 50, 49};
        data.gdLst = vec![196609, 1, 48, 49, 196609, 0, 48, 49, 196609, 1, 50, 49];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{15, 26, 26, 19, 26, 3, 20, 26, 1, 20, 2, 1, 20, 0, 1, 51, 2, 3};
        data.cxnLst = vec![15, 26, 26, 19, 26, 3, 20, 26, 1, 20, 2, 1, 20, 0, 1, 51, 2, 3];
        // Java: data.rect = new int[]{46, 52, 56, 60};
        data.rect = vec![46, 52, 56, 60];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 1, 1, 26, 26, 1, 0, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 1, 1, 26, 26, 1, 0, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_smileyFace()
    fn get_smileyFace() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{4653, -4653, 4653, 4969, 21699, 6215, 21600, 13135, 16640, 7570, 16515, 100000, 0, 50000, 2700000, 1125, 21600000};
        data.avcLst = vec![4653, -4653, 4653, 4969, 21699, 6215, 21600, 13135, 16640, 7570, 16515, 100000, 0, 50000, 2700000, 1125, 21600000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 0, 51, 52, 196609, 0, 53, 54, 196609, 0, 55, 54, 196609, 0, 56, 54, 196609, 1, 57, 54, 196609, 1, 58, 54, 196609, 1, 65, 59, 196608, 89, 60, 93, 196608, 89, 93, 60, 196609, 1, 65, 61, 196608, 101, 105, 60, 131093, 2, 62, 131092, 3, 62, 196608, 2, 60, 113, 196608, 2, 113, 60, 196608, 3, 60, 116, 196608, 3, 116, 60, 196609, 0, 63, 54, 196609, 1, 63, 54};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 0, 51, 52, 196609, 0, 53, 54, 196609, 0, 55, 54, 196609, 0, 56, 54, 196609, 1, 57, 54, 196609, 1, 58, 54, 196609, 1, 65, 59, 196608, 89, 60, 93, 196608, 89, 93, 60, 196609, 1, 65, 61, 196608, 101, 105, 60, 131093, 2, 62, 131092, 3, 62, 196608, 2, 60, 113, 196608, 2, 113, 60, 196608, 3, 60, 116, 196608, 3, 116, 60, 196609, 0, 63, 54, 196609, 1, 63, 54];
        // Java: data.ahLst = new int[]{2, 48, 49, 50, 2, 101};
        data.ahLst = vec![2, 48, 49, 50, 2, 101];
        // Java: data.cxnLst = new int[]{15, 2, 26, 15, 119, 127, 19, 26, 3, 20, 119, 131, 20, 2, 1, 20, 123, 131, 60, 0, 3, 15, 123, 127};
        data.cxnLst = vec![15, 2, 26, 15, 119, 127, 19, 26, 3, 20, 119, 131, 20, 2, 1, 20, 123, 131, 60, 0, 3, 15, 123, 127];
        // Java: data.rect = new int[]{119, 127, 123, 131};
        data.rect = vec![119, 127, 123, 131];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 26, 3, 3, 2, 3, 19, 64, 2}, {7, 13, 0, 73, 85, 3, 135, 139, 19, 64, 0, 77, 85, 3, 135, 139, 19, 64}, {10, 13, 0, 69, 97, 4, 2, 109, 81, 97}, {10, 13, 0, 26, 3, 3, 2, 3, 19, 64, 2}};
        data.pathLst = vec![vec![11, 12, 0, 26, 3, 3, 2, 3, 19, 64, 2], vec![7, 13, 0, 73, 85, 3, 135, 139, 19, 64, 0, 77, 85, 3, 135, 139, 19, 64], vec![10, 13, 0, 69, 97, 4, 2, 109, 81, 97], vec![10, 13, 0, 26, 3, 3, 2, 3, 19, 64, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_snip1Rect()
    fn get_snip1Rect() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{16667, 0, 50000, 100000, 1, 2};
        data.avcLst = vec![16667, 0, 50000, 100000, 1, 2];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 54, 51, 196608, 0, 49, 58, 196609, 58, 52, 53, 196610, 62, 0, 53};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 54, 51, 196608, 0, 49, 58, 196609, 58, 52, 53, 196610, 62, 0, 53];
        // Java: data.ahLst = new int[]{1, 48, 49, 50, 62, 26};
        data.ahLst = vec![1, 48, 49, 50, 62, 26];
        // Java: data.cxnLst = new int[]{49, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![49, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{26, 66, 70, 1};
        data.rect = vec![26, 66, 70, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 62, 26, 1, 0, 58, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 62, 26, 1, 0, 58, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_snip2DiagRect()
    fn get_snip2DiagRect() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{0, 16667, 0, 50000, 100000, 1, 2};
        data.avcLst = vec![0, 16667, 0, 50000, 100000, 1, 2];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196625, 50, 49, 51, 196609, 28, 55, 52, 196608, 0, 50, 63, 196608, 1, 50, 63, 196609, 28, 59, 52, 196608, 0, 50, 75, 196608, 1, 50, 75, 196608, 63, 50, 75, 196614, 87, 63, 75, 196609, 91, 53, 54, 196608, 0, 50, 95, 196608, 1, 50, 95};
        data.gdLst = vec![196625, 50, 48, 51, 196625, 50, 49, 51, 196609, 28, 55, 52, 196608, 0, 50, 63, 196608, 1, 50, 63, 196609, 28, 59, 52, 196608, 0, 50, 75, 196608, 1, 50, 75, 196608, 63, 50, 75, 196614, 87, 63, 75, 196609, 91, 53, 54, 196608, 0, 50, 95, 196608, 1, 50, 95];
        // Java: data.ahLst = new int[]{1, 48, 50, 51, 63, 26, 1, 49, 50, 51, 79, 26};
        data.ahLst = vec![1, 48, 50, 51, 63, 26, 1, 49, 50, 51, 79, 26];
        // Java: data.cxnLst = new int[]{50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{95, 95, 99, 103};
        data.rect = vec![95, 95, 99, 103];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 63, 26, 1, 79, 26, 1, 0, 75, 1, 0, 71, 1, 67, 1, 1, 75, 1, 1, 26, 83, 1, 26, 63, 2}};
        data.pathLst = vec![vec![11, 13, 0, 63, 26, 1, 79, 26, 1, 0, 75, 1, 0, 71, 1, 67, 1, 1, 75, 1, 1, 26, 83, 1, 26, 63, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_snip2SameRect()
    fn get_snip2SameRect() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{16667, 0, 0, 50000, 100000, 1, 2};
        data.avcLst = vec![16667, 0, 0, 50000, 100000, 1, 2];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196625, 50, 49, 51, 196609, 28, 55, 52, 196608, 0, 50, 63, 196609, 28, 59, 52, 196608, 0, 50, 71, 196608, 1, 50, 71, 196608, 63, 50, 71, 196614, 83, 63, 71, 196609, 87, 53, 54, 196608, 0, 50, 91, 196609, 63, 53, 54, 196610, 79, 1, 54};
        data.gdLst = vec![196625, 50, 48, 51, 196625, 50, 49, 51, 196609, 28, 55, 52, 196608, 0, 50, 63, 196609, 28, 59, 52, 196608, 0, 50, 71, 196608, 1, 50, 71, 196608, 63, 50, 71, 196614, 83, 63, 71, 196609, 87, 53, 54, 196608, 0, 50, 91, 196609, 63, 53, 54, 196610, 79, 1, 54];
        // Java: data.ahLst = new int[]{1, 48, 50, 51, 67, 26, 1, 49, 50, 51, 71, 1};
        data.ahLst = vec![1, 48, 50, 51, 67, 26, 1, 49, 50, 51, 71, 1];
        // Java: data.cxnLst = new int[]{50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{91, 99, 95, 103};
        data.rect = vec![91, 99, 95, 103];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 63, 26, 1, 67, 26, 1, 0, 63, 1, 0, 79, 1, 75, 1, 1, 71, 1, 1, 26, 79, 1, 26, 63, 2}};
        data.pathLst = vec![vec![11, 13, 0, 63, 26, 1, 67, 26, 1, 0, 63, 1, 0, 79, 1, 75, 1, 1, 71, 1, 1, 26, 79, 1, 26, 63, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_snipRoundRect()
    fn get_snipRoundRect() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{16667, 16667, 0, 50000, 100000, 29289, 2};
        data.avcLst = vec![16667, 16667, 0, 50000, 100000, 29289, 2];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196625, 50, 49, 51, 196609, 28, 55, 52, 196609, 28, 59, 52, 196608, 0, 50, 67, 196609, 63, 53, 52, 196610, 71, 0, 54};
        data.gdLst = vec![196625, 50, 48, 51, 196625, 50, 49, 51, 196609, 28, 55, 52, 196609, 28, 59, 52, 196608, 0, 50, 67, 196609, 63, 53, 52, 196610, 71, 0, 54];
        // Java: data.ahLst = new int[]{1, 48, 50, 51, 63, 26, 1, 49, 50, 51, 71, 26};
        data.ahLst = vec![1, 48, 50, 51, 63, 26, 1, 49, 50, 51, 71, 26];
        // Java: data.cxnLst = new int[]{50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26};
        data.cxnLst = vec![50, 0, 3, 20, 2, 1, 19, 26, 3, 15, 2, 26];
        // Java: data.rect = new int[]{75, 75, 79, 1};
        data.rect = vec![75, 75, 79, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 63, 26, 1, 71, 26, 1, 0, 67, 1, 0, 1, 1, 26, 1, 1, 26, 63, 3, 63, 63, 19, 20, 2}};
        data.pathLst = vec![vec![11, 13, 0, 63, 26, 1, 71, 26, 1, 0, 67, 1, 0, 1, 1, 26, 1, 1, 26, 63, 3, 63, 63, 19, 20, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_squareTabs()
    fn get_squareTabs() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{0, 1, 20};
        data.avcLst = vec![0, 1, 20];
        // Java: data.gdLst = new int[]{196615, 0, 1, 48, 196609, 49, 51, 50, 196608, 48, 1, 55, 196608, 48, 0, 55};
        data.gdLst = vec![196615, 0, 1, 48, 196609, 49, 51, 50, 196608, 48, 1, 55, 196608, 48, 0, 55];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[]{19, 26, 26, 19, 26, 55, 19, 26, 59, 19, 26, 1, 19, 55, 55, 19, 55, 63, 15, 55, 26, 15, 63, 26, 20, 55, 1, 20, 63, 1, 48, 0, 26, 48, 0, 55, 48, 0, 59, 48, 0, 1, 48, 63, 55, 48, 63, 59};
        data.cxnLst = vec![19, 26, 26, 19, 26, 55, 19, 26, 59, 19, 26, 1, 19, 55, 55, 19, 55, 63, 15, 55, 26, 15, 63, 26, 20, 55, 1, 20, 63, 1, 48, 0, 26, 48, 0, 55, 48, 0, 59, 48, 0, 1, 48, 63, 55, 48, 63, 59];
        // Java: data.rect = new int[]{55, 55, 63, 59};
        data.rect = vec![55, 55, 63, 59];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 55, 26, 1, 55, 55, 1, 26, 55, 2}, {11, 13, 0, 26, 59, 1, 55, 59, 1, 55, 1, 1, 26, 1, 2}, {11, 13, 0, 63, 26, 1, 0, 26, 1, 0, 55, 1, 63, 55, 2}, {11, 13, 0, 63, 59, 1, 0, 59, 1, 0, 1, 1, 63, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 55, 26, 1, 55, 55, 1, 26, 55, 2], vec![11, 13, 0, 26, 59, 1, 55, 59, 1, 55, 1, 1, 26, 1, 2], vec![11, 13, 0, 63, 26, 1, 0, 26, 1, 0, 55, 1, 63, 55, 2], vec![11, 13, 0, 63, 59, 1, 0, 59, 1, 0, 1, 1, 63, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_star10()
    fn get_star10() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{42533, 105146, 0, 50000, 100000, 95106, 58779, 80902, 30902};
        data.avcLst = vec![42533, 105146, 0, 50000, 100000, 95106, 58779, 80902, 30902];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196609, 2, 49, 52, 196609, 61, 53, 52, 196609, 61, 54, 52, 196608, 2, 50, 65, 196608, 2, 50, 69, 196608, 2, 69, 50, 196608, 2, 65, 50, 196609, 3, 55, 52, 196609, 3, 56, 52, 196608, 3, 50, 89, 196608, 3, 50, 93, 196608, 3, 93, 50, 196608, 3, 89, 50, 196609, 61, 57, 51, 196609, 3, 57, 51, 196609, 113, 55, 52, 196609, 113, 56, 52, 196609, 117, 53, 52, 196609, 117, 54, 52, 196608, 2, 50, 113, 196608, 2, 50, 121, 196608, 2, 50, 125, 196608, 2, 125, 50, 196608, 2, 121, 50, 196608, 2, 113, 50, 196608, 3, 50, 129, 196608, 3, 50, 133, 196608, 3, 133, 50, 196608, 3, 129, 50, 196608, 3, 50, 117};
        data.gdLst = vec![196625, 50, 48, 51, 196609, 2, 49, 52, 196609, 61, 53, 52, 196609, 61, 54, 52, 196608, 2, 50, 65, 196608, 2, 50, 69, 196608, 2, 69, 50, 196608, 2, 65, 50, 196609, 3, 55, 52, 196609, 3, 56, 52, 196608, 3, 50, 89, 196608, 3, 50, 93, 196608, 3, 93, 50, 196608, 3, 89, 50, 196609, 61, 57, 51, 196609, 3, 57, 51, 196609, 113, 55, 52, 196609, 113, 56, 52, 196609, 117, 53, 52, 196609, 117, 54, 52, 196608, 2, 50, 113, 196608, 2, 50, 121, 196608, 2, 50, 125, 196608, 2, 125, 50, 196608, 2, 121, 50, 196608, 2, 113, 50, 196608, 3, 50, 129, 196608, 3, 50, 133, 196608, 3, 133, 50, 196608, 3, 129, 50, 196608, 3, 50, 117];
        // Java: data.ahLst = new int[]{2, 48, 50, 51, 2, 177};
        data.ahLst = vec![2, 48, 50, 51, 2, 177];
        // Java: data.cxnLst = new int[]{50, 85, 101, 50, 85, 105, 20, 81, 109, 20, 2, 1, 20, 77, 109, 19, 73, 105, 19, 73, 101, 15, 77, 97, 15, 2, 26, 15, 81, 97};
        data.cxnLst = vec![50, 85, 101, 50, 85, 105, 20, 81, 109, 20, 2, 1, 20, 77, 109, 19, 73, 105, 19, 73, 101, 15, 77, 97, 15, 2, 26, 15, 81, 97];
        // Java: data.rect = new int[]{141, 165, 153, 169};
        data.rect = vec![141, 165, 153, 169];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 73, 101, 1, 141, 165, 1, 77, 97, 1, 145, 161, 1, 2, 26, 1, 149, 161, 1, 81, 97, 1, 153, 165, 1, 85, 101, 1, 157, 3, 1, 85, 105, 1, 153, 169, 1, 81, 109, 1, 149, 173, 1, 2, 1, 1, 145, 173, 1, 77, 109, 1, 141, 169, 1, 73, 105, 1, 137, 3, 2}};
        data.pathLst = vec![vec![11, 13, 0, 73, 101, 1, 141, 165, 1, 77, 97, 1, 145, 161, 1, 2, 26, 1, 149, 161, 1, 81, 97, 1, 153, 165, 1, 85, 101, 1, 157, 3, 1, 85, 105, 1, 153, 169, 1, 81, 109, 1, 149, 173, 1, 2, 1, 1, 145, 173, 1, 77, 109, 1, 141, 169, 1, 73, 105, 1, 137, 3, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_star12()
    fn get_star12() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{37500, 0, 50000, 1800000, 3600000, 3, 4, 900000, 2700000, 4500000};
        data.avcLst = vec![37500, 0, 50000, 1800000, 3600000, 3, 4, 900000, 2700000, 4500000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 131093, 2, 51, 131092, 3, 52, 196608, 2, 49, 62, 196609, 0, 53, 54, 196608, 2, 62, 49, 196608, 3, 49, 65, 196609, 1, 53, 54, 196608, 3, 65, 49, 196609, 2, 58, 50, 196609, 3, 58, 50, 131093, 92, 55, 131093, 92, 56, 131093, 92, 57, 131092, 96, 57, 131092, 96, 56, 131092, 96, 55, 196608, 2, 49, 100, 196608, 2, 49, 103, 196608, 2, 49, 106, 196608, 2, 106, 49, 196608, 2, 103, 49, 196608, 2, 100, 49, 196608, 3, 49, 109, 196608, 3, 49, 112, 196608, 3, 49, 115, 196608, 3, 115, 49, 196608, 3, 112, 49, 196608, 3, 109, 49, 196608, 3, 49, 96};
        data.gdLst = vec![196625, 49, 48, 50, 131093, 2, 51, 131092, 3, 52, 196608, 2, 49, 62, 196609, 0, 53, 54, 196608, 2, 62, 49, 196608, 3, 49, 65, 196609, 1, 53, 54, 196608, 3, 65, 49, 196609, 2, 58, 50, 196609, 3, 58, 50, 131093, 92, 55, 131093, 92, 56, 131093, 92, 57, 131092, 96, 57, 131092, 96, 56, 131092, 96, 55, 196608, 2, 49, 100, 196608, 2, 49, 103, 196608, 2, 49, 106, 196608, 2, 106, 49, 196608, 2, 103, 49, 196608, 2, 100, 49, 196608, 3, 49, 109, 196608, 3, 49, 112, 196608, 3, 49, 115, 196608, 3, 115, 49, 196608, 3, 112, 49, 196608, 3, 109, 49, 196608, 3, 49, 96];
        // Java: data.ahLst = new int[]{2, 48, 49, 50, 2, 166};
        data.ahLst = vec![2, 48, 49, 50, 2, 166];
        // Java: data.cxnLst = new int[]{49, 76, 22, 49, 0, 3, 49, 76, 84, 20, 72, 88, 20, 2, 1, 20, 33, 88, 19, 68, 84, 19, 26, 3, 19, 68, 22, 15, 33, 80, 15, 2, 26, 15, 72, 80};
        data.cxnLst = vec![49, 76, 22, 49, 0, 3, 49, 76, 84, 20, 72, 88, 20, 2, 1, 20, 33, 88, 19, 68, 84, 19, 26, 3, 19, 68, 22, 15, 33, 80, 15, 2, 26, 15, 72, 80];
        // Java: data.rect = new int[]{122, 146, 134, 158};
        data.rect = vec![122, 146, 134, 158];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 1, 118, 150, 1, 68, 22, 1, 122, 146, 1, 33, 80, 1, 126, 142, 1, 2, 26, 1, 130, 142, 1, 72, 80, 1, 134, 146, 1, 76, 22, 1, 138, 150, 1, 0, 3, 1, 138, 154, 1, 76, 84, 1, 134, 158, 1, 72, 88, 1, 130, 162, 1, 2, 1, 1, 126, 162, 1, 33, 88, 1, 122, 158, 1, 68, 84, 1, 118, 154, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 1, 118, 150, 1, 68, 22, 1, 122, 146, 1, 33, 80, 1, 126, 142, 1, 2, 26, 1, 130, 142, 1, 72, 80, 1, 134, 146, 1, 76, 22, 1, 138, 150, 1, 0, 3, 1, 138, 154, 1, 76, 84, 1, 134, 158, 1, 72, 88, 1, 130, 162, 1, 2, 1, 1, 126, 162, 1, 33, 88, 1, 122, 158, 1, 68, 84, 1, 118, 154, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_star16()
    fn get_star16() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{37500, 0, 50000, 92388, 100000, 70711, 38268, 98079, 83147, 55557, 19509, 2700000};
        data.avcLst = vec![37500, 0, 50000, 92388, 100000, 70711, 38268, 98079, 83147, 55557, 19509, 2700000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 2, 51, 52, 196609, 2, 53, 52, 196609, 2, 54, 52, 196609, 3, 51, 52, 196609, 3, 53, 52, 196609, 3, 54, 52, 196608, 2, 49, 64, 196608, 2, 49, 68, 196608, 2, 49, 72, 196608, 2, 72, 49, 196608, 2, 68, 49, 196608, 2, 64, 49, 196608, 3, 49, 76, 196608, 3, 49, 80, 196608, 3, 49, 84, 196608, 3, 84, 49, 196608, 3, 80, 49, 196608, 3, 76, 49, 196609, 2, 60, 50, 196609, 3, 60, 50, 196609, 136, 55, 52, 196609, 136, 56, 52, 196609, 136, 57, 52, 196609, 136, 58, 52, 196609, 140, 55, 52, 196609, 140, 56, 52, 196609, 140, 57, 52, 196609, 140, 58, 52, 196608, 2, 49, 144, 196608, 2, 49, 148, 196608, 2, 49, 152, 196608, 2, 49, 156, 196608, 2, 156, 49, 196608, 2, 152, 49, 196608, 2, 148, 49, 196608, 2, 144, 49, 196608, 3, 49, 160, 196608, 3, 49, 164, 196608, 3, 49, 168, 196608, 3, 49, 172, 196608, 3, 172, 49, 196608, 3, 168, 49, 196608, 3, 164, 49, 196608, 3, 160, 49, 131093, 136, 59, 131092, 140, 59, 196608, 2, 49, 240, 196608, 3, 49, 243, 196608, 2, 240, 49, 196608, 3, 243, 49, 196608, 3, 49, 140};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 2, 51, 52, 196609, 2, 53, 52, 196609, 2, 54, 52, 196609, 3, 51, 52, 196609, 3, 53, 52, 196609, 3, 54, 52, 196608, 2, 49, 64, 196608, 2, 49, 68, 196608, 2, 49, 72, 196608, 2, 72, 49, 196608, 2, 68, 49, 196608, 2, 64, 49, 196608, 3, 49, 76, 196608, 3, 49, 80, 196608, 3, 49, 84, 196608, 3, 84, 49, 196608, 3, 80, 49, 196608, 3, 76, 49, 196609, 2, 60, 50, 196609, 3, 60, 50, 196609, 136, 55, 52, 196609, 136, 56, 52, 196609, 136, 57, 52, 196609, 136, 58, 52, 196609, 140, 55, 52, 196609, 140, 56, 52, 196609, 140, 57, 52, 196609, 140, 58, 52, 196608, 2, 49, 144, 196608, 2, 49, 148, 196608, 2, 49, 152, 196608, 2, 49, 156, 196608, 2, 156, 49, 196608, 2, 152, 49, 196608, 2, 148, 49, 196608, 2, 144, 49, 196608, 3, 49, 160, 196608, 3, 49, 164, 196608, 3, 49, 168, 196608, 3, 49, 172, 196608, 3, 172, 49, 196608, 3, 168, 49, 196608, 3, 164, 49, 196608, 3, 160, 49, 131093, 136, 59, 131092, 140, 59, 196608, 2, 49, 240, 196608, 3, 49, 243, 196608, 2, 240, 49, 196608, 3, 243, 49, 196608, 3, 49, 140];
        // Java: data.ahLst = new int[]{2, 48, 49, 50, 2, 262};
        data.ahLst = vec![2, 48, 49, 50, 2, 262];
        // Java: data.cxnLst = new int[]{49, 104, 116, 49, 108, 120, 49, 0, 3, 49, 108, 124, 49, 104, 128, 20, 100, 132, 20, 2, 1, 20, 96, 132, 19, 92, 128, 19, 88, 124, 19, 26, 3, 19, 88, 120, 19, 92, 116, 15, 96, 112, 15, 2, 26, 15, 100, 112};
        data.cxnLst = vec![49, 104, 116, 49, 108, 120, 49, 0, 3, 49, 108, 124, 49, 104, 128, 20, 100, 132, 20, 2, 1, 20, 96, 132, 19, 92, 128, 19, 88, 124, 19, 26, 3, 19, 88, 120, 19, 92, 116, 15, 96, 112, 15, 2, 26, 15, 100, 112];
        // Java: data.rect = new int[]{246, 250, 254, 258};
        data.rect = vec![246, 250, 254, 258];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 1, 176, 220, 1, 88, 120, 1, 180, 216, 1, 92, 116, 1, 184, 212, 1, 96, 112, 1, 188, 208, 1, 2, 26, 1, 192, 208, 1, 100, 112, 1, 196, 212, 1, 104, 116, 1, 200, 216, 1, 108, 120, 1, 204, 220, 1, 0, 3, 1, 204, 224, 1, 108, 124, 1, 200, 228, 1, 104, 128, 1, 196, 232, 1, 100, 132, 1, 192, 236, 1, 2, 1, 1, 188, 236, 1, 96, 132, 1, 184, 232, 1, 92, 128, 1, 180, 228, 1, 88, 124, 1, 176, 224, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 1, 176, 220, 1, 88, 120, 1, 180, 216, 1, 92, 116, 1, 184, 212, 1, 96, 112, 1, 188, 208, 1, 2, 26, 1, 192, 208, 1, 100, 112, 1, 196, 212, 1, 104, 116, 1, 200, 216, 1, 108, 120, 1, 204, 220, 1, 0, 3, 1, 204, 224, 1, 108, 124, 1, 200, 228, 1, 104, 128, 1, 196, 232, 1, 100, 132, 1, 192, 236, 1, 2, 1, 1, 188, 236, 1, 96, 132, 1, 184, 232, 1, 92, 128, 1, 180, 228, 1, 88, 124, 1, 176, 224, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_star24()
    fn get_star24() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{37500, 0, 50000, 900000, 1800000, 2700000, 4500000, 3600000, 99144, 100000, 92388, 79335, 60876, 38268, 13053};
        data.avcLst = vec![37500, 0, 50000, 900000, 1800000, 2700000, 4500000, 3600000, 99144, 100000, 92388, 79335, 60876, 38268, 13053];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 131093, 2, 51, 131093, 2, 52, 131093, 2, 53, 65554, 33, 131093, 2, 54, 131092, 3, 54, 131092, 3, 55, 131092, 3, 53, 65554, 22, 131092, 3, 51, 196608, 2, 49, 67, 196608, 2, 49, 70, 196608, 2, 49, 73, 196608, 2, 49, 76, 196608, 2, 49, 78, 196608, 2, 78, 49, 196608, 2, 76, 49, 196608, 2, 73, 49, 196608, 2, 70, 49, 196608, 2, 67, 49, 196608, 3, 49, 81, 196608, 3, 49, 84, 196608, 3, 49, 87, 196608, 3, 49, 90, 196608, 3, 49, 92, 196608, 3, 92, 49, 196608, 3, 90, 49, 196608, 3, 87, 49, 196608, 3, 84, 49, 196608, 3, 81, 49, 196609, 2, 63, 50, 196609, 3, 63, 50, 196609, 175, 56, 57, 196609, 175, 58, 57, 196609, 175, 59, 57, 196609, 175, 60, 57, 196609, 175, 61, 57, 196609, 175, 62, 57, 196609, 179, 56, 57, 196609, 179, 58, 57, 196609, 179, 59, 57, 196609, 179, 60, 57, 196609, 179, 61, 57, 196609, 179, 62, 57, 196608, 2, 49, 183, 196608, 2, 49, 187, 196608, 2, 49, 191, 196608, 2, 49, 195, 196608, 2, 49, 199, 196608, 2, 49, 203, 196608, 2, 203, 49, 196608, 2, 199, 49, 196608, 2, 195, 49, 196608, 2, 191, 49, 196608, 2, 187, 49, 196608, 2, 183, 49, 196608, 3, 49, 207, 196608, 3, 49, 211, 196608, 3, 49, 215, 196608, 3, 49, 219, 196608, 3, 49, 223, 196608, 3, 49, 227, 196608, 3, 227, 49, 196608, 3, 223, 49, 196608, 3, 219, 49, 196608, 3, 215, 49, 196608, 3, 211, 49, 196608, 3, 207, 49, 131093, 175, 53, 131092, 179, 53, 196608, 2, 49, 327, 196608, 3, 49, 330, 196608, 2, 327, 49, 196608, 3, 330, 49, 196608, 3, 49, 179};
        data.gdLst = vec![196625, 49, 48, 50, 131093, 2, 51, 131093, 2, 52, 131093, 2, 53, 65554, 33, 131093, 2, 54, 131092, 3, 54, 131092, 3, 55, 131092, 3, 53, 65554, 22, 131092, 3, 51, 196608, 2, 49, 67, 196608, 2, 49, 70, 196608, 2, 49, 73, 196608, 2, 49, 76, 196608, 2, 49, 78, 196608, 2, 78, 49, 196608, 2, 76, 49, 196608, 2, 73, 49, 196608, 2, 70, 49, 196608, 2, 67, 49, 196608, 3, 49, 81, 196608, 3, 49, 84, 196608, 3, 49, 87, 196608, 3, 49, 90, 196608, 3, 49, 92, 196608, 3, 92, 49, 196608, 3, 90, 49, 196608, 3, 87, 49, 196608, 3, 84, 49, 196608, 3, 81, 49, 196609, 2, 63, 50, 196609, 3, 63, 50, 196609, 175, 56, 57, 196609, 175, 58, 57, 196609, 175, 59, 57, 196609, 175, 60, 57, 196609, 175, 61, 57, 196609, 175, 62, 57, 196609, 179, 56, 57, 196609, 179, 58, 57, 196609, 179, 59, 57, 196609, 179, 60, 57, 196609, 179, 61, 57, 196609, 179, 62, 57, 196608, 2, 49, 183, 196608, 2, 49, 187, 196608, 2, 49, 191, 196608, 2, 49, 195, 196608, 2, 49, 199, 196608, 2, 49, 203, 196608, 2, 203, 49, 196608, 2, 199, 49, 196608, 2, 195, 49, 196608, 2, 191, 49, 196608, 2, 187, 49, 196608, 2, 183, 49, 196608, 3, 49, 207, 196608, 3, 49, 211, 196608, 3, 49, 215, 196608, 3, 49, 219, 196608, 3, 49, 223, 196608, 3, 49, 227, 196608, 3, 227, 49, 196608, 3, 223, 49, 196608, 3, 219, 49, 196608, 3, 215, 49, 196608, 3, 211, 49, 196608, 3, 207, 49, 131093, 175, 53, 131092, 179, 53, 196608, 2, 49, 327, 196608, 3, 49, 330, 196608, 2, 327, 49, 196608, 3, 330, 49, 196608, 3, 49, 179];
        // Java: data.ahLst = new int[]{2, 48, 49, 50, 2, 349};
        data.ahLst = vec![2, 48, 49, 50, 2, 349];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3];
        // Java: data.rect = new int[]{333, 337, 341, 345};
        data.rect = vec![333, 337, 341, 345];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 1, 231, 299, 1, 95, 151, 1, 235, 295, 1, 99, 147, 1, 239, 291, 1, 103, 143, 1, 243, 287, 1, 107, 139, 1, 247, 283, 1, 111, 135, 1, 251, 279, 1, 2, 26, 1, 255, 279, 1, 115, 135, 1, 259, 283, 1, 119, 139, 1, 263, 287, 1, 123, 143, 1, 267, 291, 1, 127, 147, 1, 271, 295, 1, 131, 151, 1, 275, 299, 1, 0, 3, 1, 275, 303, 1, 131, 155, 1, 271, 307, 1, 127, 159, 1, 267, 311, 1, 123, 163, 1, 263, 315, 1, 119, 167, 1, 259, 319, 1, 115, 171, 1, 255, 323, 1, 2, 1, 1, 251, 323, 1, 111, 171, 1, 247, 319, 1, 107, 167, 1, 243, 315, 1, 103, 163, 1, 239, 311, 1, 99, 159, 1, 235, 307, 1, 95, 155, 1, 231, 303, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 1, 231, 299, 1, 95, 151, 1, 235, 295, 1, 99, 147, 1, 239, 291, 1, 103, 143, 1, 243, 287, 1, 107, 139, 1, 247, 283, 1, 111, 135, 1, 251, 279, 1, 2, 26, 1, 255, 279, 1, 115, 135, 1, 259, 283, 1, 119, 139, 1, 263, 287, 1, 123, 143, 1, 267, 291, 1, 127, 147, 1, 271, 295, 1, 131, 151, 1, 275, 299, 1, 0, 3, 1, 275, 303, 1, 131, 155, 1, 271, 307, 1, 127, 159, 1, 267, 311, 1, 123, 163, 1, 263, 315, 1, 119, 167, 1, 259, 319, 1, 115, 171, 1, 255, 323, 1, 2, 1, 1, 251, 323, 1, 111, 171, 1, 247, 319, 1, 107, 167, 1, 243, 315, 1, 103, 163, 1, 239, 311, 1, 99, 159, 1, 235, 307, 1, 95, 155, 1, 231, 303, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_star32()
    fn get_star32() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{37500, 0, 50000, 98079, 100000, 92388, 83147, 2700000, 55557, 38268, 19509, 99518, 95694, 88192, 77301, 63439, 47140, 29028, 9802};
        data.avcLst = vec![37500, 0, 50000, 98079, 100000, 92388, 83147, 2700000, 55557, 38268, 19509, 99518, 95694, 88192, 77301, 63439, 47140, 29028, 9802];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 2, 51, 52, 196609, 2, 53, 52, 196609, 2, 54, 52, 131093, 2, 55, 196609, 2, 56, 52, 196609, 2, 57, 52, 196609, 2, 58, 52, 196609, 3, 51, 52, 196609, 3, 53, 52, 196609, 3, 54, 52, 131092, 3, 55, 196609, 3, 56, 52, 196609, 3, 57, 52, 196609, 3, 58, 52, 196608, 2, 49, 71, 196608, 2, 49, 75, 196608, 2, 49, 79, 196608, 2, 49, 83, 196608, 2, 49, 86, 196608, 2, 49, 90, 196608, 2, 49, 94, 196608, 2, 94, 49, 196608, 2, 90, 49, 196608, 2, 86, 49, 196608, 2, 83, 49, 196608, 2, 79, 49, 196608, 2, 75, 49, 196608, 2, 71, 49, 196608, 3, 49, 98, 196608, 3, 49, 102, 196608, 3, 49, 106, 196608, 3, 49, 110, 196608, 3, 49, 113, 196608, 3, 49, 117, 196608, 3, 49, 121, 196608, 3, 121, 49, 196608, 3, 117, 49, 196608, 3, 113, 49, 196608, 3, 110, 49, 196608, 3, 106, 49, 196608, 3, 102, 49, 196608, 3, 98, 49, 196609, 2, 67, 50, 196609, 3, 67, 50, 196609, 237, 59, 52, 196609, 237, 60, 52, 196609, 237, 61, 52, 196609, 237, 62, 52, 196609, 237, 63, 52, 196609, 237, 64, 52, 196609, 237, 65, 52, 196609, 237, 66, 52, 196609, 241, 59, 52, 196609, 241, 60, 52, 196609, 241, 61, 52, 196609, 241, 62, 52, 196609, 241, 63, 52, 196609, 241, 64, 52, 196609, 241, 65, 52, 196609, 241, 66, 52, 196608, 2, 49, 245, 196608, 2, 49, 249, 196608, 2, 49, 253, 196608, 2, 49, 257, 196608, 2, 49, 261, 196608, 2, 49, 265, 196608, 2, 49, 269, 196608, 2, 49, 273, 196608, 2, 273, 49, 196608, 2, 269, 49, 196608, 2, 265, 49, 196608, 2, 261, 49, 196608, 2, 257, 49, 196608, 2, 253, 49, 196608, 2, 249, 49, 196608, 2, 245, 49, 196608, 3, 49, 277, 196608, 3, 49, 281, 196608, 3, 49, 285, 196608, 3, 49, 289, 196608, 3, 49, 293, 196608, 3, 49, 297, 196608, 3, 49, 301, 196608, 3, 49, 305, 196608, 3, 305, 49, 196608, 3, 301, 49, 196608, 3, 297, 49, 196608, 3, 293, 49, 196608, 3, 289, 49, 196608, 3, 285, 49, 196608, 3, 281, 49, 196608, 3, 277, 49, 131093, 237, 55, 131092, 241, 55, 196608, 2, 49, 437, 196608, 3, 49, 440, 196608, 2, 437, 49, 196608, 3, 440, 49, 196608, 3, 49, 241};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 2, 51, 52, 196609, 2, 53, 52, 196609, 2, 54, 52, 131093, 2, 55, 196609, 2, 56, 52, 196609, 2, 57, 52, 196609, 2, 58, 52, 196609, 3, 51, 52, 196609, 3, 53, 52, 196609, 3, 54, 52, 131092, 3, 55, 196609, 3, 56, 52, 196609, 3, 57, 52, 196609, 3, 58, 52, 196608, 2, 49, 71, 196608, 2, 49, 75, 196608, 2, 49, 79, 196608, 2, 49, 83, 196608, 2, 49, 86, 196608, 2, 49, 90, 196608, 2, 49, 94, 196608, 2, 94, 49, 196608, 2, 90, 49, 196608, 2, 86, 49, 196608, 2, 83, 49, 196608, 2, 79, 49, 196608, 2, 75, 49, 196608, 2, 71, 49, 196608, 3, 49, 98, 196608, 3, 49, 102, 196608, 3, 49, 106, 196608, 3, 49, 110, 196608, 3, 49, 113, 196608, 3, 49, 117, 196608, 3, 49, 121, 196608, 3, 121, 49, 196608, 3, 117, 49, 196608, 3, 113, 49, 196608, 3, 110, 49, 196608, 3, 106, 49, 196608, 3, 102, 49, 196608, 3, 98, 49, 196609, 2, 67, 50, 196609, 3, 67, 50, 196609, 237, 59, 52, 196609, 237, 60, 52, 196609, 237, 61, 52, 196609, 237, 62, 52, 196609, 237, 63, 52, 196609, 237, 64, 52, 196609, 237, 65, 52, 196609, 237, 66, 52, 196609, 241, 59, 52, 196609, 241, 60, 52, 196609, 241, 61, 52, 196609, 241, 62, 52, 196609, 241, 63, 52, 196609, 241, 64, 52, 196609, 241, 65, 52, 196609, 241, 66, 52, 196608, 2, 49, 245, 196608, 2, 49, 249, 196608, 2, 49, 253, 196608, 2, 49, 257, 196608, 2, 49, 261, 196608, 2, 49, 265, 196608, 2, 49, 269, 196608, 2, 49, 273, 196608, 2, 273, 49, 196608, 2, 269, 49, 196608, 2, 265, 49, 196608, 2, 261, 49, 196608, 2, 257, 49, 196608, 2, 253, 49, 196608, 2, 249, 49, 196608, 2, 245, 49, 196608, 3, 49, 277, 196608, 3, 49, 281, 196608, 3, 49, 285, 196608, 3, 49, 289, 196608, 3, 49, 293, 196608, 3, 49, 297, 196608, 3, 49, 301, 196608, 3, 49, 305, 196608, 3, 305, 49, 196608, 3, 301, 49, 196608, 3, 297, 49, 196608, 3, 293, 49, 196608, 3, 289, 49, 196608, 3, 285, 49, 196608, 3, 281, 49, 196608, 3, 277, 49, 131093, 237, 55, 131092, 241, 55, 196608, 2, 49, 437, 196608, 3, 49, 440, 196608, 2, 437, 49, 196608, 3, 440, 49, 196608, 3, 49, 241];
        // Java: data.ahLst = new int[]{2, 48, 49, 50, 2, 459};
        data.ahLst = vec![2, 48, 49, 50, 2, 459];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3];
        // Java: data.rect = new int[]{443, 447, 451, 455};
        data.rect = vec![443, 447, 451, 455];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 1, 309, 401, 1, 125, 205, 1, 313, 397, 1, 129, 201, 1, 317, 393, 1, 133, 197, 1, 321, 389, 1, 137, 193, 1, 325, 385, 1, 141, 189, 1, 329, 381, 1, 145, 185, 1, 333, 377, 1, 149, 181, 1, 337, 373, 1, 2, 26, 1, 341, 373, 1, 153, 181, 1, 345, 377, 1, 157, 185, 1, 349, 381, 1, 161, 189, 1, 353, 385, 1, 165, 193, 1, 357, 389, 1, 169, 197, 1, 361, 393, 1, 173, 201, 1, 365, 397, 1, 177, 205, 1, 369, 401, 1, 0, 3, 1, 369, 405, 1, 177, 209, 1, 365, 409, 1, 173, 213, 1, 361, 413, 1, 169, 217, 1, 357, 417, 1, 165, 221, 1, 353, 421, 1, 161, 225, 1, 349, 425, 1, 157, 229, 1, 345, 429, 1, 153, 233, 1, 341, 433, 1, 2, 1, 1, 337, 433, 1, 149, 233, 1, 333, 429, 1, 145, 229, 1, 329, 425, 1, 141, 225, 1, 325, 421, 1, 137, 221, 1, 321, 417, 1, 133, 217, 1, 317, 413, 1, 129, 213, 1, 313, 409, 1, 125, 209, 1, 309, 405, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 1, 309, 401, 1, 125, 205, 1, 313, 397, 1, 129, 201, 1, 317, 393, 1, 133, 197, 1, 321, 389, 1, 137, 193, 1, 325, 385, 1, 141, 189, 1, 329, 381, 1, 145, 185, 1, 333, 377, 1, 149, 181, 1, 337, 373, 1, 2, 26, 1, 341, 373, 1, 153, 181, 1, 345, 377, 1, 157, 185, 1, 349, 381, 1, 161, 189, 1, 353, 385, 1, 165, 193, 1, 357, 389, 1, 169, 197, 1, 361, 393, 1, 173, 201, 1, 365, 397, 1, 177, 205, 1, 369, 401, 1, 0, 3, 1, 369, 405, 1, 177, 209, 1, 365, 409, 1, 173, 213, 1, 361, 413, 1, 169, 217, 1, 357, 417, 1, 165, 221, 1, 353, 421, 1, 161, 225, 1, 349, 425, 1, 157, 229, 1, 345, 429, 1, 153, 233, 1, 341, 433, 1, 2, 1, 1, 337, 433, 1, 149, 233, 1, 333, 429, 1, 145, 229, 1, 329, 425, 1, 141, 225, 1, 325, 421, 1, 137, 221, 1, 321, 417, 1, 133, 217, 1, 317, 413, 1, 129, 213, 1, 313, 409, 1, 125, 209, 1, 309, 405, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_star4()
    fn get_star4() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{12500, 0, 50000, 2700000};
        data.avcLst = vec![12500, 0, 50000, 2700000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 2, 52, 50, 196609, 3, 52, 50, 131093, 56, 51, 131092, 60, 51, 196608, 2, 49, 64, 196608, 2, 64, 49, 196608, 3, 49, 67, 196608, 3, 67, 49, 196608, 3, 49, 60};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 2, 52, 50, 196609, 3, 52, 50, 131093, 56, 51, 131092, 60, 51, 196608, 2, 49, 64, 196608, 2, 64, 49, 196608, 3, 49, 67, 196608, 3, 67, 49, 196608, 3, 49, 60];
        // Java: data.ahLst = new int[]{2, 48, 49, 50, 2, 86};
        data.ahLst = vec![2, 48, 49, 50, 2, 86];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 49, 0, 3];
        // Java: data.rect = new int[]{70, 78, 74, 82};
        data.rect = vec![70, 78, 74, 82];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 1, 70, 78, 1, 2, 26, 1, 74, 78, 1, 0, 3, 1, 74, 82, 1, 2, 1, 1, 70, 82, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 1, 70, 78, 1, 2, 26, 1, 74, 78, 1, 0, 3, 1, 74, 82, 1, 2, 1, 1, 70, 82, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_star5()
    fn get_star5() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{19098, 105146, 110557, 0, 50000, 100000, 1080000, 18360000, 20520000, 3240000};
        data.avcLst = vec![19098, 105146, 110557, 0, 50000, 100000, 1080000, 18360000, 20520000, 3240000];
        // Java: data.gdLst = new int[]{196625, 51, 48, 52, 196609, 2, 49, 53, 196609, 3, 50, 53, 196609, 3, 50, 53, 131093, 62, 54, 131093, 62, 55, 131092, 66, 54, 131092, 66, 55, 196608, 2, 51, 74, 196608, 2, 51, 77, 196608, 2, 77, 51, 196608, 2, 74, 51, 196608, 70, 51, 80, 196608, 70, 51, 83, 196609, 62, 58, 52, 196609, 66, 58, 52, 131093, 110, 56, 131093, 110, 57, 131092, 114, 57, 131092, 114, 56, 196608, 2, 51, 118, 196608, 2, 51, 121, 196608, 2, 121, 51, 196608, 2, 118, 51, 196608, 70, 51, 124, 196608, 70, 51, 127, 196608, 70, 114, 51, 196608, 70, 51, 114};
        data.gdLst = vec![196625, 51, 48, 52, 196609, 2, 49, 53, 196609, 3, 50, 53, 196609, 3, 50, 53, 131093, 62, 54, 131093, 62, 55, 131092, 66, 54, 131092, 66, 55, 196608, 2, 51, 74, 196608, 2, 51, 77, 196608, 2, 77, 51, 196608, 2, 74, 51, 196608, 70, 51, 80, 196608, 70, 51, 83, 196609, 62, 58, 52, 196609, 66, 58, 52, 131093, 110, 56, 131093, 110, 57, 131092, 114, 57, 131092, 114, 56, 196608, 2, 51, 118, 196608, 2, 51, 121, 196608, 2, 121, 51, 196608, 2, 118, 51, 196608, 70, 51, 124, 196608, 70, 51, 127, 196608, 70, 114, 51, 196608, 70, 51, 114];
        // Java: data.ahLst = new int[]{2, 48, 51, 52, 2, 158};
        data.ahLst = vec![2, 48, 51, 52, 2, 158];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 86, 102, 20, 90, 106, 20, 94, 106, 51, 98, 102};
        data.cxnLst = vec![15, 2, 26, 19, 86, 102, 20, 90, 106, 20, 94, 106, 51, 98, 102];
        // Java: data.rect = new int[]{130, 146, 142, 154};
        data.rect = vec![130, 146, 142, 154];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 86, 102, 1, 134, 146, 1, 2, 26, 1, 138, 146, 1, 98, 102, 1, 142, 150, 1, 94, 106, 1, 2, 154, 1, 90, 106, 1, 130, 150, 2}};
        data.pathLst = vec![vec![11, 13, 0, 86, 102, 1, 134, 146, 1, 2, 26, 1, 138, 146, 1, 98, 102, 1, 142, 150, 1, 94, 106, 1, 2, 154, 1, 90, 106, 1, 130, 150, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_star6()
    fn get_star6() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{28868, 115470, 0, 50000, 100000, 1800000, 1, 2, 3600000};
        data.avcLst = vec![28868, 115470, 0, 50000, 100000, 1800000, 1, 2, 3600000];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196609, 2, 49, 52, 131093, 61, 53, 196608, 2, 50, 65, 196608, 2, 65, 50, 196608, 3, 22, 50, 196609, 61, 57, 51, 196609, 3, 57, 51, 196609, 80, 54, 55, 196608, 2, 50, 80, 196608, 2, 50, 88, 196608, 2, 88, 50, 196608, 2, 80, 50, 131092, 84, 56, 196608, 3, 50, 108, 196608, 3, 108, 50, 196608, 3, 50, 84};
        data.gdLst = vec![196625, 50, 48, 51, 196609, 2, 49, 52, 131093, 61, 53, 196608, 2, 50, 65, 196608, 2, 65, 50, 196608, 3, 22, 50, 196609, 61, 57, 51, 196609, 3, 57, 51, 196609, 80, 54, 55, 196608, 2, 50, 80, 196608, 2, 50, 88, 196608, 2, 88, 50, 196608, 2, 80, 50, 131092, 84, 56, 196608, 3, 50, 108, 196608, 3, 108, 50, 196608, 3, 50, 84];
        // Java: data.ahLst = new int[]{2, 48, 50, 51, 2, 119};
        data.ahLst = vec![2, 48, 50, 51, 2, 119];
        // Java: data.cxnLst = new int[]{50, 72, 22, 50, 72, 76, 20, 2, 1, 19, 68, 76, 19, 68, 22, 15, 2, 26};
        data.cxnLst = vec![50, 72, 22, 50, 72, 76, 20, 2, 1, 19, 68, 76, 19, 68, 22, 15, 2, 26];
        // Java: data.rect = new int[]{92, 111, 104, 115};
        data.rect = vec![92, 111, 104, 115];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 68, 22, 1, 96, 111, 1, 2, 26, 1, 100, 111, 1, 72, 22, 1, 104, 3, 1, 72, 76, 1, 100, 115, 1, 2, 1, 1, 96, 115, 1, 68, 76, 1, 92, 3, 2}};
        data.pathLst = vec![vec![11, 13, 0, 68, 22, 1, 96, 111, 1, 2, 26, 1, 100, 111, 1, 72, 22, 1, 104, 3, 1, 72, 76, 1, 100, 115, 1, 2, 1, 1, 96, 115, 1, 68, 76, 1, 92, 3, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_star7()
    fn get_star7() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{34601, 102572, 105210, 0, 50000, 100000, 97493, 78183, 43388, 62349, 22252, 90097};
        data.avcLst = vec![34601, 102572, 105210, 0, 50000, 100000, 97493, 78183, 43388, 62349, 22252, 90097];
        // Java: data.gdLst = new int[]{196625, 51, 48, 52, 196609, 2, 49, 53, 196609, 3, 50, 53, 196609, 3, 50, 53, 196609, 64, 54, 53, 196609, 64, 55, 53, 196609, 64, 56, 53, 196609, 68, 57, 53, 196609, 68, 58, 53, 196609, 68, 59, 53, 196608, 2, 51, 76, 196608, 2, 51, 80, 196608, 2, 51, 84, 196608, 2, 84, 51, 196608, 2, 80, 51, 196608, 2, 76, 51, 196608, 72, 51, 88, 196608, 72, 92, 51, 196608, 72, 96, 51, 196609, 64, 60, 52, 196609, 68, 60, 52, 196609, 136, 54, 53, 196609, 136, 55, 53, 196609, 136, 56, 53, 196608, 2, 51, 144, 196608, 2, 51, 148, 196608, 2, 51, 152, 196608, 2, 152, 51, 196608, 2, 148, 51, 196608, 2, 144, 51, 196609, 140, 59, 53, 196609, 140, 58, 53, 196609, 140, 57, 53, 196608, 72, 51, 180, 196608, 72, 51, 184, 196608, 72, 188, 51, 196608, 72, 140, 51, 196608, 72, 51, 140};
        data.gdLst = vec![196625, 51, 48, 52, 196609, 2, 49, 53, 196609, 3, 50, 53, 196609, 3, 50, 53, 196609, 64, 54, 53, 196609, 64, 55, 53, 196609, 64, 56, 53, 196609, 68, 57, 53, 196609, 68, 58, 53, 196609, 68, 59, 53, 196608, 2, 51, 76, 196608, 2, 51, 80, 196608, 2, 51, 84, 196608, 2, 84, 51, 196608, 2, 80, 51, 196608, 2, 76, 51, 196608, 72, 51, 88, 196608, 72, 92, 51, 196608, 72, 96, 51, 196609, 64, 60, 52, 196609, 68, 60, 52, 196609, 136, 54, 53, 196609, 136, 55, 53, 196609, 136, 56, 53, 196608, 2, 51, 144, 196608, 2, 51, 148, 196608, 2, 51, 152, 196608, 2, 152, 51, 196608, 2, 148, 51, 196608, 2, 144, 51, 196609, 140, 59, 53, 196609, 140, 58, 53, 196609, 140, 57, 53, 196608, 72, 51, 180, 196608, 72, 51, 184, 196608, 72, 188, 51, 196608, 72, 140, 51, 196608, 72, 51, 140];
        // Java: data.ahLst = new int[]{2, 48, 51, 52, 2, 208};
        data.ahLst = vec![2, 48, 51, 52, 2, 208];
        // Java: data.cxnLst = new int[]{51, 116, 124, 51, 120, 128, 20, 112, 132, 20, 108, 132, 19, 100, 128, 19, 104, 124, 15, 2, 26};
        data.cxnLst = vec![51, 116, 124, 51, 120, 128, 20, 112, 132, 20, 108, 132, 19, 100, 128, 19, 104, 124, 15, 2, 26];
        // Java: data.rect = new int[]{160, 192, 172, 200};
        data.rect = vec![160, 192, 172, 200];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 100, 128, 1, 156, 196, 1, 104, 124, 1, 164, 192, 1, 2, 26, 1, 168, 192, 1, 116, 124, 1, 176, 196, 1, 120, 128, 1, 172, 200, 1, 112, 132, 1, 2, 204, 1, 108, 132, 1, 160, 200, 2}};
        data.pathLst = vec![vec![11, 13, 0, 100, 128, 1, 156, 196, 1, 104, 124, 1, 164, 192, 1, 2, 26, 1, 168, 192, 1, 116, 124, 1, 176, 196, 1, 120, 128, 1, 172, 200, 1, 112, 132, 1, 2, 204, 1, 108, 132, 1, 160, 200, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_star8()
    fn get_star8() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{37500, 0, 50000, 2700000, 92388, 100000, 38268};
        data.avcLst = vec![37500, 0, 50000, 2700000, 92388, 100000, 38268];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 131093, 2, 51, 196608, 2, 49, 59, 196608, 2, 59, 49, 131092, 3, 51, 196608, 3, 49, 70, 196608, 3, 70, 49, 196609, 2, 55, 50, 196609, 3, 55, 50, 196609, 81, 52, 53, 196609, 81, 54, 53, 196609, 85, 52, 53, 196609, 85, 54, 53, 196608, 2, 49, 89, 196608, 2, 49, 93, 196608, 2, 93, 49, 196608, 2, 89, 49, 196608, 3, 49, 97, 196608, 3, 49, 101, 196608, 3, 101, 49, 196608, 3, 97, 49, 196608, 3, 49, 85};
        data.gdLst = vec![196625, 49, 48, 50, 131093, 2, 51, 196608, 2, 49, 59, 196608, 2, 59, 49, 131092, 3, 51, 196608, 3, 49, 70, 196608, 3, 70, 49, 196609, 2, 55, 50, 196609, 3, 55, 50, 196609, 81, 52, 53, 196609, 81, 54, 53, 196609, 85, 52, 53, 196609, 85, 54, 53, 196608, 2, 49, 89, 196608, 2, 49, 93, 196608, 2, 93, 49, 196608, 2, 89, 49, 196608, 3, 49, 97, 196608, 3, 49, 101, 196608, 3, 101, 49, 196608, 3, 97, 49, 196608, 3, 49, 85];
        // Java: data.ahLst = new int[]{2, 48, 49, 50, 2, 137};
        data.ahLst = vec![2, 48, 49, 50, 2, 137];
        // Java: data.cxnLst = new int[]{49, 0, 3, 20, 66, 77, 20, 2, 1, 20, 62, 77, 19, 26, 3, 15, 62, 73, 15, 2, 26, 15, 66, 73};
        data.cxnLst = vec![49, 0, 3, 20, 66, 77, 20, 2, 1, 20, 62, 77, 19, 26, 3, 15, 62, 73, 15, 2, 26, 15, 66, 73];
        // Java: data.rect = new int[]{105, 121, 117, 133};
        data.rect = vec![105, 121, 117, 133];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 1, 105, 125, 1, 62, 73, 1, 109, 121, 1, 2, 26, 1, 113, 121, 1, 66, 73, 1, 117, 125, 1, 0, 3, 1, 117, 129, 1, 66, 77, 1, 113, 133, 1, 2, 1, 1, 109, 133, 1, 62, 77, 1, 105, 129, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 1, 105, 125, 1, 62, 73, 1, 109, 121, 1, 2, 26, 1, 113, 121, 1, 66, 73, 1, 117, 125, 1, 0, 3, 1, 117, 129, 1, 66, 77, 1, 113, 133, 1, 2, 1, 1, 109, 133, 1, 62, 77, 1, 105, 129, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_straightConnector1()
    fn get_straightConnector1() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[0];
        data.avcLst = vec![];
        // Java: data.gdLst = new int[0];
        data.gdLst = vec![];
        // Java: data.ahLst = new int[0];
        data.ahLst = vec![];
        // Java: data.cxnLst = new int[0];
        data.cxnLst = vec![];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{10, 13, 0, 26, 26, 1, 0, 1}};
        data.pathLst = vec![vec![10, 13, 0, 26, 26, 1, 0, 1]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_stripedRightArrow()
    fn get_stripedRightArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 50000, 84375, 0, 100000, 5, 32, 200000};
        data.avcLst = vec![50000, 50000, 84375, 0, 100000, 5, 32, 200000];
        // Java: data.gdLst = new int[]{196609, 50, 0, 28, 196625, 51, 48, 52, 196625, 51, 49, 56, 196609, 28, 53, 54, 196609, 28, 64, 52, 196608, 0, 51, 72, 196609, 1, 60, 55, 196608, 3, 51, 80, 196608, 3, 80, 51, 196609, 80, 72, 3, 196608, 0, 51, 92};
        data.gdLst = vec![196609, 50, 0, 28, 196625, 51, 48, 52, 196625, 51, 49, 56, 196609, 28, 53, 54, 196609, 28, 64, 52, 196608, 0, 51, 72, 196609, 1, 60, 55, 196608, 3, 51, 80, 196608, 3, 80, 51, 196609, 80, 72, 3, 196608, 0, 51, 92];
        // Java: data.ahLst = new int[]{2, 48, 51, 52, 26, 84, 1, 49, 51, 56, 76, 26};
        data.ahLst = vec![2, 48, 51, 52, 26, 84, 1, 49, 51, 56, 76, 26];
        // Java: data.cxnLst = new int[]{15, 76, 26, 19, 26, 3, 20, 76, 1, 51, 0, 3};
        data.cxnLst = vec![15, 76, 26, 19, 26, 3, 20, 76, 1, 51, 0, 3];
        // Java: data.rect = new int[]{68, 84, 96, 88};
        data.rect = vec![68, 84, 96, 88];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 84, 1, 41, 84, 1, 41, 88, 1, 26, 88, 2, 0, 40, 84, 1, 32, 84, 1, 32, 88, 1, 40, 88, 2, 0, 68, 84, 1, 76, 84, 1, 76, 26, 1, 0, 3, 1, 76, 1, 1, 76, 88, 1, 68, 88, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 84, 1, 41, 84, 1, 41, 88, 1, 26, 88, 2, 0, 40, 84, 1, 32, 84, 1, 32, 88, 1, 40, 88, 2, 0, 68, 84, 1, 76, 84, 1, 76, 26, 1, 0, 3, 1, 76, 1, 1, 76, 88, 1, 68, 88, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_sun()
    fn get_sun() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 12500, 46875, 50000, 0, 30274, 32768, 12540, 23170, 3, 4, 3662, 100000, 18436, 21600, 3163, 21600000};
        data.avcLst = vec![25000, 12500, 46875, 50000, 0, 30274, 32768, 12540, 23170, 3, 4, 3662, 100000, 18436, 21600, 3163, 21600000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196608, 51, 52, 65, 196609, 69, 53, 54, 196609, 69, 55, 54, 196608, 73, 51, 52, 196608, 77, 51, 52, 196608, 51, 52, 73, 196608, 51, 52, 77, 196609, 69, 56, 54, 196608, 51, 97, 52, 196608, 51, 52, 97, 196609, 89, 57, 58, 196609, 93, 57, 58, 196608, 109, 59, 52, 196608, 113, 59, 52, 196608, 113, 49, 52, 196608, 60, 52, 109, 196608, 60, 52, 117, 196608, 60, 52, 121, 196608, 60, 52, 125, 196609, 0, 61, 62, 196609, 1, 63, 62, 196609, 0, 63, 62, 196609, 1, 61, 62, 196609, 0, 101, 60, 196609, 0, 105, 60, 196609, 0, 109, 60, 196609, 0, 117, 60, 196609, 0, 121, 60, 196609, 0, 125, 60, 196609, 0, 129, 60, 196609, 0, 133, 60, 196609, 0, 137, 60, 196609, 0, 141, 60, 196609, 0, 65, 60, 196609, 0, 69, 60, 196609, 1, 69, 60, 196609, 1, 101, 60, 196609, 1, 105, 60, 196609, 1, 109, 60, 196609, 1, 117, 60, 196609, 1, 121, 60, 196609, 1, 125, 60, 196609, 1, 129, 60, 196609, 1, 133, 60, 196609, 1, 137, 60, 196609, 1, 141, 60};
        data.gdLst = vec![196625, 49, 48, 50, 196608, 51, 52, 65, 196609, 69, 53, 54, 196609, 69, 55, 54, 196608, 73, 51, 52, 196608, 77, 51, 52, 196608, 51, 52, 73, 196608, 51, 52, 77, 196609, 69, 56, 54, 196608, 51, 97, 52, 196608, 51, 52, 97, 196609, 89, 57, 58, 196609, 93, 57, 58, 196608, 109, 59, 52, 196608, 113, 59, 52, 196608, 113, 49, 52, 196608, 60, 52, 109, 196608, 60, 52, 117, 196608, 60, 52, 121, 196608, 60, 52, 125, 196609, 0, 61, 62, 196609, 1, 63, 62, 196609, 0, 63, 62, 196609, 1, 61, 62, 196609, 0, 101, 60, 196609, 0, 105, 60, 196609, 0, 109, 60, 196609, 0, 117, 60, 196609, 0, 121, 60, 196609, 0, 125, 60, 196609, 0, 129, 60, 196609, 0, 133, 60, 196609, 0, 137, 60, 196609, 0, 141, 60, 196609, 0, 65, 60, 196609, 0, 69, 60, 196609, 1, 69, 60, 196609, 1, 101, 60, 196609, 1, 105, 60, 196609, 1, 109, 60, 196609, 1, 117, 60, 196609, 1, 121, 60, 196609, 1, 125, 60, 196609, 1, 129, 60, 196609, 1, 133, 60, 196609, 1, 137, 60, 196609, 1, 141, 60];
        // Java: data.ahLst = new int[]{1, 48, 49, 50, 201, 3};
        data.ahLst = vec![1, 48, 49, 50, 201, 3];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 52, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 52, 0, 3];
        // Java: data.rect = new int[]{165, 217, 161, 213};
        data.rect = vec![165, 217, 161, 213];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 0, 3, 1, 185, 249, 1, 185, 233, 2, 0, 145, 149, 1, 189, 229, 1, 193, 225, 2, 0, 2, 26, 1, 197, 221, 1, 181, 221, 2, 0, 153, 149, 1, 177, 225, 1, 173, 229, 2, 0, 26, 3, 1, 169, 233, 1, 169, 249, 2, 0, 153, 157, 1, 173, 245, 1, 177, 241, 2, 0, 2, 1, 1, 181, 237, 1, 197, 237, 2, 0, 145, 157, 1, 193, 241, 1, 189, 245, 2, 0, 201, 3, 3, 205, 209, 19, 64, 2}};
        data.pathLst = vec![vec![11, 13, 0, 0, 3, 1, 185, 249, 1, 185, 233, 2, 0, 145, 149, 1, 189, 229, 1, 193, 225, 2, 0, 2, 26, 1, 197, 221, 1, 181, 221, 2, 0, 153, 149, 1, 177, 225, 1, 173, 229, 2, 0, 26, 3, 1, 169, 233, 1, 169, 249, 2, 0, 153, 157, 1, 173, 245, 1, 177, 241, 2, 0, 2, 1, 1, 181, 237, 1, 197, 237, 2, 0, 145, 157, 1, 193, 241, 1, 189, 245, 2, 0, 201, 3, 3, 205, 209, 19, 64, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_swooshArrow()
    fn get_swooshArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 16667, 1, 75000, 70000, 0, 100000, 14, 2, 20};
        data.avcLst = vec![25000, 16667, 1, 75000, 70000, 0, 100000, 14, 2, 20];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196609, 52, 0, 28, 196625, 53, 49, 62, 196609, 1, 58, 54, 196609, 28, 66, 54, 196608, 0, 53, 74, 196608, 26, 32, 53, 196609, 20, 50, 55, 131094, 32, 86, 196608, 78, 53, 90, 131094, 70, 86, 196608, 82, 70, 53, 196608, 78, 97, 53, 196608, 104, 90, 53, 196608, 100, 32, 53, 196608, 112, 53, 26, 196609, 116, 50, 56, 196609, 1, 50, 57, 196608, 26, 120, 124, 196609, 42, 50, 50, 196608, 42, 132, 53, 65554, 35, 196609, 42, 50, 56, 196608, 100, 142, 53, 65554, 33};
        data.gdLst = vec![196625, 50, 48, 51, 196609, 52, 0, 28, 196625, 53, 49, 62, 196609, 1, 58, 54, 196609, 28, 66, 54, 196608, 0, 53, 74, 196608, 26, 32, 53, 196609, 20, 50, 55, 131094, 32, 86, 196608, 78, 53, 90, 131094, 70, 86, 196608, 82, 70, 53, 196608, 78, 97, 53, 196608, 104, 90, 53, 196608, 100, 32, 53, 196608, 112, 53, 26, 196609, 116, 50, 56, 196609, 1, 50, 57, 196608, 26, 120, 124, 196609, 42, 50, 50, 196608, 42, 132, 53, 65554, 35, 196609, 42, 50, 56, 196608, 100, 142, 53, 65554, 33];
        // Java: data.ahLst = new int[]{2, 48, 50, 51, 104, 100, 1, 49, 53, 62, 78, 82};
        data.ahLst = vec![2, 48, 50, 51, 104, 100, 1, 49, 53, 62, 78, 82];
        // Java: data.cxnLst = new int[]{20, 26, 1, 15, 93, 26, 53, 0, 128, 20, 108, 112};
        data.cxnLst = vec![20, 26, 1, 15, 93, 26, 53, 0, 128, 20, 108, 112];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 1, 4, 140, 136, 78, 82, 1, 93, 26, 1, 0, 128, 1, 108, 112, 1, 104, 100, 4, 150, 146, 26, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 1, 4, 140, 136, 78, 82, 1, 93, 26, 1, 0, 128, 1, 108, 112, 1, 104, 100, 4, 150, 146, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_teardrop()
    fn get_teardrop() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{100000, 0, 200000, 2, 1, 100000, 2700000};
        data.avcLst = vec![100000, 0, 200000, 2, 1, 100000, 2700000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 65549, 51, 196609, 2, 59, 52, 196609, 3, 59, 52, 196609, 61, 55, 53, 196609, 65, 55, 53, 131093, 69, 54, 131092, 73, 54, 196608, 2, 77, 49, 196608, 3, 49, 80, 196610, 2, 83, 51, 196610, 3, 87, 51, 131093, 2, 54, 131092, 3, 54, 196608, 2, 49, 99, 196608, 2, 99, 49, 196608, 3, 49, 102, 196608, 3, 102, 49};
        data.gdLst = vec![196625, 49, 48, 50, 65549, 51, 196609, 2, 59, 52, 196609, 3, 59, 52, 196609, 61, 55, 53, 196609, 65, 55, 53, 131093, 69, 54, 131092, 73, 54, 196608, 2, 77, 49, 196608, 3, 49, 80, 196610, 2, 83, 51, 196610, 3, 87, 51, 131093, 2, 54, 131092, 3, 54, 196608, 2, 49, 99, 196608, 2, 99, 49, 196608, 3, 49, 102, 196608, 3, 102, 49];
        // Java: data.ahLst = new int[]{1, 48, 49, 50, 83, 26};
        data.ahLst = vec![1, 48, 49, 50, 83, 26];
        // Java: data.cxnLst = new int[]{49, 0, 3, 20, 109, 117, 20, 2, 1, 20, 105, 117, 19, 26, 3, 15, 105, 113, 15, 2, 26, 15, 83, 87};
        data.cxnLst = vec![49, 0, 3, 20, 109, 117, 20, 2, 1, 20, 105, 117, 19, 26, 3, 15, 105, 113, 15, 2, 26, 15, 83, 87];
        // Java: data.rect = new int[]{105, 113, 109, 117};
        data.rect = vec![105, 113, 109, 117];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 3, 3, 2, 3, 19, 20, 4, 91, 26, 83, 87, 4, 0, 95, 0, 3, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 3, 3, 2, 3, 19, 20, 4, 91, 26, 83, 87, 4, 0, 95, 0, 3, 3, 2, 3, 49, 20, 3, 2, 3, 20, 20, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_trapezoid()
    fn get_trapezoid() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 50000, 0, 200000, 100000};
        data.avcLst = vec![25000, 50000, 0, 200000, 100000];
        // Java: data.gdLst = new int[]{196609, 49, 0, 28, 196625, 50, 48, 53, 196609, 28, 57, 51, 196609, 28, 57, 52, 196608, 0, 50, 65, 196608, 0, 50, 61, 196609, 39, 57, 53, 196609, 38, 57, 53, 196608, 0, 50, 77};
        data.gdLst = vec![196609, 49, 0, 28, 196625, 50, 48, 53, 196609, 28, 57, 51, 196609, 28, 57, 52, 196608, 0, 50, 65, 196608, 0, 50, 61, 196609, 39, 57, 53, 196609, 38, 57, 53, 196608, 0, 50, 77];
        // Java: data.ahLst = new int[]{1, 48, 50, 53, 65, 26};
        data.ahLst = vec![1, 48, 50, 53, 65, 26];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 61, 3, 20, 2, 1, 50, 73, 3};
        data.cxnLst = vec![15, 2, 26, 19, 61, 3, 20, 2, 1, 50, 73, 3];
        // Java: data.rect = new int[]{77, 81, 85, 1};
        data.rect = vec![77, 81, 85, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 1, 1, 65, 26, 1, 69, 26, 1, 0, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 1, 1, 65, 26, 1, 69, 26, 1, 0, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_triangle()
    fn get_triangle() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 0, 100000, 200000};
        data.avcLst = vec![50000, 0, 100000, 200000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 0, 52, 51, 196609, 0, 52, 50, 196608, 56, 2, 49};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 0, 52, 51, 196609, 0, 52, 50, 196608, 56, 2, 49];
        // Java: data.ahLst = new int[]{1, 48, 49, 50, 60, 26};
        data.ahLst = vec![1, 48, 49, 50, 60, 26];
        // Java: data.cxnLst = new int[]{15, 60, 26, 19, 56, 3, 20, 26, 1, 20, 60, 1, 20, 0, 1, 49, 64, 3};
        data.cxnLst = vec![15, 60, 26, 19, 56, 3, 20, 26, 1, 20, 60, 1, 20, 0, 1, 49, 64, 3];
        // Java: data.rect = new int[]{56, 3, 64, 1};
        data.rect = vec![56, 3, 64, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 1, 1, 60, 26, 1, 0, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 1, 1, 60, 26, 1, 0, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_upArrowCallout()
    fn get_upArrowCallout() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 25000, 25000, 64977, 50000, 0, 2, 1, 100000, 200000};
        data.avcLst = vec![25000, 25000, 25000, 64977, 50000, 0, 2, 1, 100000, 200000];
        // Java: data.gdLst = new int[]{196609, 52, 0, 28, 196625, 53, 49, 58, 196609, 62, 54, 55, 196625, 53, 48, 66, 196609, 56, 1, 28, 196625, 53, 50, 74, 196609, 78, 28, 1, 196608, 56, 53, 82, 196625, 53, 51, 86, 196609, 28, 62, 56, 196609, 28, 70, 57, 196608, 2, 53, 94, 196608, 2, 53, 98, 196608, 2, 98, 53, 196608, 2, 94, 53, 196609, 28, 78, 56, 196609, 1, 90, 56, 196608, 1, 53, 122, 196610, 126, 1, 54};
        data.gdLst = vec![196609, 52, 0, 28, 196625, 53, 49, 58, 196609, 62, 54, 55, 196625, 53, 48, 66, 196609, 56, 1, 28, 196625, 53, 50, 74, 196609, 78, 28, 1, 196608, 56, 53, 82, 196625, 53, 51, 86, 196609, 28, 62, 56, 196609, 28, 70, 57, 196608, 2, 53, 94, 196608, 2, 53, 98, 196608, 2, 98, 53, 196608, 2, 94, 53, 196609, 28, 78, 56, 196609, 1, 90, 56, 196608, 1, 53, 122, 196610, 126, 1, 54];
        // Java: data.ahLst = new int[]{1, 48, 53, 66, 106, 118, 1, 49, 53, 58, 102, 26, 2, 50, 53, 74, 0, 118, 2, 51, 53, 86, 26, 126};
        data.ahLst = vec![1, 48, 53, 66, 106, 118, 1, 49, 53, 58, 102, 26, 2, 50, 53, 74, 0, 118, 2, 51, 53, 86, 26, 126];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 126, 20, 2, 1, 53, 0, 126};
        data.cxnLst = vec![15, 2, 26, 19, 26, 126, 20, 2, 1, 53, 0, 126];
        // Java: data.rect = new int[]{26, 126, 0, 1};
        data.rect = vec![26, 126, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 126, 1, 106, 126, 1, 106, 118, 1, 102, 118, 1, 2, 26, 1, 114, 118, 1, 110, 118, 1, 110, 126, 1, 0, 126, 1, 0, 1, 1, 26, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 126, 1, 106, 126, 1, 106, 118, 1, 102, 118, 1, 2, 26, 1, 114, 118, 1, 110, 118, 1, 110, 126, 1, 0, 126, 1, 0, 1, 1, 26, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_upDownArrow()
    fn get_upDownArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{50000, 50000, 50000, 0, 100000, 200000};
        data.avcLst = vec![50000, 50000, 50000, 0, 100000, 200000];
        // Java: data.gdLst = new int[]{196609, 50, 1, 28, 196625, 51, 48, 52, 196625, 51, 49, 54, 196609, 28, 62, 52, 196608, 1, 51, 66, 196609, 0, 58, 53, 196608, 2, 51, 74, 196608, 2, 74, 51, 196609, 78, 66, 2, 196608, 66, 51, 86, 196608, 70, 86, 51};
        data.gdLst = vec![196609, 50, 1, 28, 196625, 51, 48, 52, 196625, 51, 49, 54, 196609, 28, 62, 52, 196608, 1, 51, 66, 196609, 0, 58, 53, 196608, 2, 51, 74, 196608, 2, 74, 51, 196609, 78, 66, 2, 196608, 66, 51, 86, 196608, 70, 86, 51];
        // Java: data.ahLst = new int[]{1, 48, 51, 52, 78, 70, 2, 49, 51, 54, 26, 66};
        data.ahLst = vec![1, 48, 51, 52, 78, 70, 2, 49, 51, 54, 26, 66];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 66, 19, 78, 3, 19, 26, 70, 20, 2, 1, 51, 0, 70, 51, 82, 3, 51, 0, 66};
        data.cxnLst = vec![15, 2, 26, 19, 26, 66, 19, 78, 3, 19, 26, 70, 20, 2, 1, 51, 0, 70, 51, 82, 3, 51, 0, 66];
        // Java: data.rect = new int[]{78, 90, 82, 94};
        data.rect = vec![78, 90, 82, 94];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 66, 1, 2, 26, 1, 0, 66, 1, 82, 66, 1, 82, 70, 1, 0, 70, 1, 2, 1, 1, 26, 70, 1, 78, 70, 1, 78, 66, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 66, 1, 2, 26, 1, 0, 66, 1, 82, 66, 1, 82, 70, 1, 0, 70, 1, 2, 1, 1, 26, 70, 1, 78, 70, 1, 78, 66, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_upDownArrowCallout()
    fn get_upDownArrowCallout() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 25000, 25000, 48123, 50000, 0, 2, 1, 100000, 200000};
        data.avcLst = vec![25000, 25000, 25000, 48123, 50000, 0, 2, 1, 100000, 200000];
        // Java: data.gdLst = new int[]{196609, 52, 0, 28, 196625, 53, 49, 58, 196609, 62, 54, 55, 196625, 53, 48, 66, 196609, 52, 1, 28, 196625, 53, 50, 74, 196609, 78, 28, 3, 196608, 56, 53, 82, 196625, 53, 51, 86, 196609, 28, 62, 56, 196609, 28, 70, 57, 196608, 2, 53, 94, 196608, 2, 53, 98, 196608, 2, 98, 53, 196608, 2, 94, 53, 196609, 28, 78, 56, 196608, 1, 53, 118, 196609, 1, 90, 57, 196608, 3, 53, 126, 196608, 3, 126, 53};
        data.gdLst = vec![196609, 52, 0, 28, 196625, 53, 49, 58, 196609, 62, 54, 55, 196625, 53, 48, 66, 196609, 52, 1, 28, 196625, 53, 50, 74, 196609, 78, 28, 3, 196608, 56, 53, 82, 196625, 53, 51, 86, 196609, 28, 62, 56, 196609, 28, 70, 57, 196608, 2, 53, 94, 196608, 2, 53, 98, 196608, 2, 98, 53, 196608, 2, 94, 53, 196609, 28, 78, 56, 196608, 1, 53, 118, 196609, 1, 90, 57, 196608, 3, 53, 126, 196608, 3, 126, 53];
        // Java: data.ahLst = new int[]{1, 48, 53, 66, 106, 118, 1, 49, 53, 58, 102, 26, 2, 50, 53, 74, 0, 118, 2, 51, 53, 86, 26, 130};
        data.ahLst = vec![1, 48, 53, 66, 106, 118, 1, 49, 53, 58, 102, 26, 2, 50, 53, 74, 0, 118, 2, 51, 53, 86, 26, 130];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 53, 0, 3};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 53, 0, 3];
        // Java: data.rect = new int[]{26, 130, 0, 134};
        data.rect = vec![26, 130, 0, 134];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 130, 1, 106, 130, 1, 106, 118, 1, 102, 118, 1, 2, 26, 1, 114, 118, 1, 110, 118, 1, 110, 130, 1, 0, 130, 1, 0, 134, 1, 110, 134, 1, 110, 122, 1, 114, 122, 1, 2, 1, 1, 102, 122, 1, 106, 122, 1, 106, 134, 1, 26, 134, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 130, 1, 106, 130, 1, 106, 118, 1, 102, 118, 1, 2, 26, 1, 114, 118, 1, 110, 118, 1, 110, 130, 1, 0, 130, 1, 0, 134, 1, 110, 134, 1, 110, 122, 1, 114, 122, 1, 2, 1, 1, 102, 122, 1, 106, 122, 1, 106, 134, 1, 26, 134, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_uturnArrow()
    fn get_uturnArrow() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{25000, 25000, 25000, 43750, 75000, 0, 25000, 2, 1, 100000, -5400000};
        data.avcLst = vec![25000, 25000, 25000, 43750, 75000, 0, 25000, 2, 1, 100000, -5400000];
        // Java: data.gdLst = new int[]{196625, 53, 49, 54, 196609, 59, 55, 56, 196625, 53, 48, 63, 196609, 67, 28, 1, 196608, 57, 53, 71, 196609, 75, 1, 28, 196625, 53, 50, 79, 196608, 83, 67, 53, 196609, 87, 28, 1, 196625, 91, 52, 57, 196609, 28, 67, 57, 196609, 28, 59, 57, 196609, 99, 56, 55, 196608, 103, 53, 107, 196609, 1, 95, 57, 196609, 28, 83, 57, 196608, 115, 53, 119, 196608, 0, 53, 111, 196609, 127, 56, 55, 131076, 131, 123, 196609, 135, 57, 28, 196625, 53, 51, 138, 196609, 28, 142, 57, 196608, 146, 53, 99, 131077, 150, 53, 196608, 99, 154, 53, 196608, 0, 53, 103, 196608, 161, 53, 103, 196608, 165, 111, 53, 196608, 127, 53, 146, 196608, 169, 53, 154, 196610, 99, 169, 55};
        data.gdLst = vec![196625, 53, 49, 54, 196609, 59, 55, 56, 196625, 53, 48, 63, 196609, 67, 28, 1, 196608, 57, 53, 71, 196609, 75, 1, 28, 196625, 53, 50, 79, 196608, 83, 67, 53, 196609, 87, 28, 1, 196625, 91, 52, 57, 196609, 28, 67, 57, 196609, 28, 59, 57, 196609, 99, 56, 55, 196608, 103, 53, 107, 196609, 1, 95, 57, 196609, 28, 83, 57, 196608, 115, 53, 119, 196608, 0, 53, 111, 196609, 127, 56, 55, 131076, 131, 123, 196609, 135, 57, 28, 196625, 53, 51, 138, 196609, 28, 142, 57, 196608, 146, 53, 99, 131077, 150, 53, 196608, 99, 154, 53, 196608, 0, 53, 103, 196608, 161, 53, 103, 196608, 165, 111, 53, 196608, 127, 53, 146, 196608, 169, 53, 154, 196610, 99, 169, 55];
        // Java: data.ahLst = new int[]{1, 48, 53, 63, 99, 1, 1, 49, 53, 54, 165, 1, 2, 50, 53, 79, 165, 123, 1, 51, 53, 138, 146, 26, 2, 52, 91, 57, 0, 115};
        data.ahLst = vec![1, 48, 53, 63, 99, 1, 1, 49, 53, 54, 165, 1, 2, 50, 53, 79, 165, 123, 1, 51, 53, 138, 146, 26, 2, 52, 91, 57, 0, 115];
        // Java: data.cxnLst = new int[]{20, 165, 123, 20, 161, 115, 53, 0, 123, 15, 181, 26, 20, 107, 1};
        data.cxnLst = vec![20, 165, 123, 20, 161, 115, 53, 0, 123, 15, 181, 26, 20, 107, 1];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 1, 1, 26, 146, 3, 146, 146, 19, 20, 1, 173, 26, 3, 146, 146, 15, 20, 1, 127, 123, 1, 0, 123, 1, 161, 115, 1, 165, 123, 1, 169, 123, 1, 169, 157, 3, 154, 154, 53, 58, 1, 157, 99, 3, 154, 154, 15, 58, 1, 99, 1, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 1, 1, 26, 146, 3, 146, 146, 19, 20, 1, 173, 26, 3, 146, 146, 15, 20, 1, 127, 123, 1, 0, 123, 1, 161, 115, 1, 165, 123, 1, 169, 123, 1, 169, 157, 3, 154, 154, 53, 58, 1, 157, 99, 3, 154, 154, 15, 58, 1, 99, 1, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_verticalScroll()
    fn get_verticalScroll() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{12500, 0, 25000, 100000, 1, 2, 4, -5400000, -10800000};
        data.avcLst = vec![12500, 0, 25000, 100000, 1, 2, 4, -5400000, -10800000];
        // Java: data.gdLst = new int[]{196625, 49, 48, 50, 196609, 28, 57, 51, 196609, 61, 52, 53, 196609, 61, 52, 54, 196608, 61, 65, 49, 196608, 61, 61, 49, 196608, 0, 49, 61, 196608, 0, 49, 65, 196608, 81, 49, 65, 196608, 1, 49, 61, 196608, 1, 49, 65};
        data.gdLst = vec![196625, 49, 48, 50, 196609, 28, 57, 51, 196609, 61, 52, 53, 196609, 61, 52, 54, 196608, 61, 65, 49, 196608, 61, 61, 49, 196608, 0, 49, 61, 196608, 0, 49, 65, 196608, 81, 49, 65, 196608, 1, 49, 61, 196608, 1, 49, 65];
        // Java: data.ahLst = new int[]{2, 48, 49, 50, 26, 61};
        data.ahLst = vec![2, 48, 49, 50, 26, 61];
        // Java: data.cxnLst = new int[]{15, 2, 26, 49, 61, 3, 20, 2, 1, 19, 81, 3};
        data.cxnLst = vec![15, 2, 26, 49, 61, 3, 20, 2, 1, 19, 81, 3];
        // Java: data.rect = new int[]{61, 61, 81, 97};
        data.rect = vec![61, 61, 81, 97];
        // Java: data.pathLst = new int[][]{{11, 12, 0, 65, 1, 3, 65, 65, 20, 55, 1, 65, 97, 3, 69, 69, 20, 56, 1, 61, 93, 1, 61, 65, 3, 65, 65, 19, 20, 1, 85, 26, 3, 65, 65, 15, 19, 1, 81, 61, 1, 81, 97, 3, 65, 65, 49, 20, 2, 0, 77, 65, 3, 65, 65, 49, 20, 3, 69, 69, 20, 19, 2}, {7, 12, 0, 77, 65, 3, 65, 65, 49, 20, 3, 69, 69, 20, 19, 2, 0, 61, 97, 3, 65, 65, 49, 15, 3, 69, 69, 15, 19, 2}, {10, 13, 0, 61, 93, 1, 61, 65, 3, 65, 65, 19, 20, 1, 85, 26, 3, 65, 65, 15, 19, 1, 81, 61, 1, 81, 97, 3, 65, 65, 49, 20, 1, 65, 1, 3, 65, 65, 20, 19, 2, 0, 73, 26, 3, 65, 65, 15, 19, 3, 69, 69, 20, 19, 1, 77, 65, 0, 81, 61, 1, 73, 61, 0, 65, 93, 3, 69, 69, 15, 19, 1, 61, 97, 0, 65, 1, 3, 65, 65, 20, 55, 1, 61, 93}};
        data.pathLst = vec![vec![11, 12, 0, 65, 1, 3, 65, 65, 20, 55, 1, 65, 97, 3, 69, 69, 20, 56, 1, 61, 93, 1, 61, 65, 3, 65, 65, 19, 20, 1, 85, 26, 3, 65, 65, 15, 19, 1, 81, 61, 1, 81, 97, 3, 65, 65, 49, 20, 2, 0, 77, 65, 3, 65, 65, 49, 20, 3, 69, 69, 20, 19, 2], vec![7, 12, 0, 77, 65, 3, 65, 65, 49, 20, 3, 69, 69, 20, 19, 2, 0, 61, 97, 3, 65, 65, 49, 15, 3, 69, 69, 15, 19, 2], vec![10, 13, 0, 61, 93, 1, 61, 65, 3, 65, 65, 19, 20, 1, 85, 26, 3, 65, 65, 15, 19, 1, 81, 61, 1, 81, 97, 3, 65, 65, 49, 20, 1, 65, 1, 3, 65, 65, 20, 19, 2, 0, 73, 26, 3, 65, 65, 15, 19, 3, 69, 69, 20, 19, 1, 77, 65, 0, 81, 61, 1, 73, 61, 0, 65, 93, 3, 69, 69, 15, 19, 1, 61, 97, 0, 65, 1, 3, 65, 65, 20, 55, 1, 61, 93]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_wave()
    fn get_wave() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{12500, 0, 0, 20000, -10000, 10000, 100000, 10, 3, 50000, 2};
        data.avcLst = vec![12500, 0, 0, 20000, -10000, 10000, 100000, 10, 3, 50000, 2];
        // Java: data.gdLst = new int[]{196625, 50, 48, 51, 196625, 52, 49, 53, 196609, 1, 59, 54, 196609, 67, 55, 56, 196608, 67, 50, 71, 196608, 67, 71, 50, 196608, 1, 50, 67, 196608, 83, 50, 71, 196608, 83, 71, 50, 196609, 0, 63, 54, 196609, 0, 63, 57, 65539, 95, 196614, 99, 50, 99, 196608, 26, 50, 105, 196614, 99, 99, 50, 196608, 0, 50, 113, 196610, 105, 117, 56, 196608, 109, 121, 50, 196610, 125, 117, 58, 196608, 26, 113, 50, 196608, 0, 105, 50, 196608, 133, 121, 50, 196610, 141, 137, 58, 196608, 0, 50, 103, 196608, 2, 95, 50, 196608, 2, 50, 95, 131077, 109, 133, 131076, 117, 137, 196609, 1, 59, 57, 196608, 1, 50, 167};
        data.gdLst = vec![196625, 50, 48, 51, 196625, 52, 49, 53, 196609, 1, 59, 54, 196609, 67, 55, 56, 196608, 67, 50, 71, 196608, 67, 71, 50, 196608, 1, 50, 67, 196608, 83, 50, 71, 196608, 83, 71, 50, 196609, 0, 63, 54, 196609, 0, 63, 57, 65539, 95, 196614, 99, 50, 99, 196608, 26, 50, 105, 196614, 99, 99, 50, 196608, 0, 50, 113, 196610, 105, 117, 56, 196608, 109, 121, 50, 196610, 125, 117, 58, 196608, 26, 113, 50, 196608, 0, 105, 50, 196608, 133, 121, 50, 196610, 141, 137, 58, 196608, 0, 50, 103, 196608, 2, 95, 50, 196608, 2, 50, 95, 131077, 109, 133, 131076, 117, 137, 196609, 1, 59, 57, 196608, 1, 50, 167];
        // Java: data.ahLst = new int[]{2, 48, 50, 51, 26, 67, 1, 49, 52, 53, 153, 1};
        data.ahLst = vec![2, 48, 50, 51, 26, 67, 1, 49, 52, 53, 153, 1];
        // Java: data.cxnLst = new int[]{20, 157, 67, 19, 103, 3, 15, 153, 83, 50, 149, 3};
        data.cxnLst = vec![20, 157, 67, 19, 103, 3, 15, 153, 83, 50, 149, 3];
        // Java: data.rect = new int[]{161, 167, 164, 171};
        data.rect = vec![161, 167, 164, 171];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 109, 67, 5, 125, 75, 129, 79, 117, 67, 1, 137, 83, 5, 145, 91, 141, 87, 133, 83, 2}};
        data.pathLst = vec![vec![11, 13, 0, 109, 67, 5, 125, 75, 129, 79, 117, 67, 1, 137, 83, 5, 145, 91, 141, 87, 133, 83, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_wedgeEllipseCallout()
    fn get_wedgeEllipseCallout() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{-20833, 62500, 100000, 0, 1, 660000, 21600000, 2700000, -2147483647, Integer.MAX_VALUE};
        data.avcLst = vec![-20833, 62500, 100000, 0, 1, 660000, 21600000, 2700000, -2147483647, i32::MAX];
        // Java: data.gdLst = new int[]{196609, 0, 48, 50, 196609, 1, 49, 50, 196609, 58, 52, 2, 196609, 62, 52, 3, 196615, 66, 70, 51, 196608, 74, 51, 52, 196608, 2, 58, 51, 196608, 3, 62, 51, 196609, 58, 1, 52, 196609, 62, 0, 52, 131091, 90, 94, 196608, 98, 53, 51, 196608, 98, 51, 53, 131093, 2, 101, 131092, 3, 101, 196608, 2, 109, 51, 196608, 3, 112, 51, 131093, 2, 105, 131092, 3, 105, 196608, 2, 123, 51, 196608, 3, 126, 51, 131093, 2, 98, 131092, 3, 98, 196608, 2, 137, 51, 196608, 3, 140, 51, 196614, 78, 82, 143, 196614, 78, 86, 147, 131091, 109, 112, 131091, 123, 126, 196608, 162, 51, 159, 196608, 165, 54, 51, 196614, 165, 165, 169, 131093, 2, 55, 131092, 3, 55, 196608, 2, 51, 177, 196608, 2, 177, 51, 196608, 3, 51, 180, 196608, 3, 180, 51};
        data.gdLst = vec![196609, 0, 48, 50, 196609, 1, 49, 50, 196609, 58, 52, 2, 196609, 62, 52, 3, 196615, 66, 70, 51, 196608, 74, 51, 52, 196608, 2, 58, 51, 196608, 3, 62, 51, 196609, 58, 1, 52, 196609, 62, 0, 52, 131091, 90, 94, 196608, 98, 53, 51, 196608, 98, 51, 53, 131093, 2, 101, 131092, 3, 101, 196608, 2, 109, 51, 196608, 3, 112, 51, 131093, 2, 105, 131092, 3, 105, 196608, 2, 123, 51, 196608, 3, 126, 51, 131093, 2, 98, 131092, 3, 98, 196608, 2, 137, 51, 196608, 3, 140, 51, 196614, 78, 82, 143, 196614, 78, 86, 147, 131091, 109, 112, 131091, 123, 126, 196608, 162, 51, 159, 196608, 165, 54, 51, 196614, 165, 165, 169, 131093, 2, 55, 131092, 3, 55, 196608, 2, 51, 177, 196608, 2, 177, 51, 196608, 3, 51, 180, 196608, 3, 180, 51];
        // Java: data.ahLst = new int[]{3, 48, 56, 57, 49, 56, 57, 82, 86};
        data.ahLst = vec![3, 48, 56, 57, 49, 56, 57, 82, 86];
        // Java: data.cxnLst = new int[]{15, 2, 26, 15, 183, 191, 20, 187, 195, 20, 2, 1, 20, 187, 195, 51, 0, 3, 15, 187, 191, 98, 82, 86};
        data.cxnLst = vec![15, 2, 26, 15, 183, 191, 20, 187, 195, 20, 2, 1, 20, 187, 195, 51, 0, 3, 15, 187, 191, 98, 82, 86];
        // Java: data.rect = new int[]{183, 191, 187, 195};
        data.rect = vec![183, 191, 187, 195];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 151, 155, 1, 115, 119, 3, 2, 3, 159, 173, 2}};
        data.pathLst = vec![vec![11, 13, 0, 151, 155, 1, 115, 119, 3, 2, 3, 159, 173, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_wedgeRectCallout()
    fn get_wedgeRectCallout() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{-20833, 62500, 100000, 0, 7, 2, 10, 5, 12, -2147483647, Integer.MAX_VALUE, 50000};
        data.avcLst = vec![-20833, 62500, 100000, 0, 7, 2, 10, 5, 12, -2147483647, i32::MAX, 50000];
        // Java: data.gdLst = new int[]{196609, 0, 48, 50, 196609, 1, 49, 50, 65539, 48, 65539, 49, 196608, 59, 51, 68, 196608, 59, 51, 70, 196608, 2, 60, 51, 196608, 3, 64, 51, 196608, 80, 51, 2, 196608, 84, 51, 3, 196609, 60, 1, 0, 65539, 64, 65539, 96, 196608, 100, 51, 102, 196614, 60, 52, 53, 196614, 60, 54, 55, 196609, 0, 108, 56, 196609, 0, 112, 56, 196614, 64, 52, 53, 196614, 64, 54, 55, 196609, 1, 124, 56, 196609, 1, 128, 56, 196614, 60, 26, 80, 196614, 104, 26, 140, 196614, 72, 26, 144, 196614, 64, 116, 80, 196614, 104, 152, 116, 196614, 60, 80, 0, 196614, 104, 0, 160, 196614, 72, 0, 164, 196614, 64, 80, 116, 196614, 104, 172, 116, 196614, 60, 132, 84, 196614, 104, 132, 180, 196614, 64, 26, 84, 196614, 104, 188, 26, 196614, 76, 26, 192, 196614, 60, 84, 132, 196614, 104, 132, 200, 196614, 64, 84, 1, 196614, 104, 208, 1, 196614, 76, 1, 212};
        data.gdLst = vec![196609, 0, 48, 50, 196609, 1, 49, 50, 65539, 48, 65539, 49, 196608, 59, 51, 68, 196608, 59, 51, 70, 196608, 2, 60, 51, 196608, 3, 64, 51, 196608, 80, 51, 2, 196608, 84, 51, 3, 196609, 60, 1, 0, 65539, 64, 65539, 96, 196608, 100, 51, 102, 196614, 60, 52, 53, 196614, 60, 54, 55, 196609, 0, 108, 56, 196609, 0, 112, 56, 196614, 64, 52, 53, 196614, 64, 54, 55, 196609, 1, 124, 56, 196609, 1, 128, 56, 196614, 60, 26, 80, 196614, 104, 26, 140, 196614, 72, 26, 144, 196614, 64, 116, 80, 196614, 104, 152, 116, 196614, 60, 80, 0, 196614, 104, 0, 160, 196614, 72, 0, 164, 196614, 64, 80, 116, 196614, 104, 172, 116, 196614, 60, 132, 84, 196614, 104, 132, 180, 196614, 64, 26, 84, 196614, 104, 188, 26, 196614, 76, 26, 192, 196614, 60, 84, 132, 196614, 104, 132, 200, 196614, 64, 84, 1, 196614, 104, 208, 1, 196614, 76, 1, 212];
        // Java: data.ahLst = new int[]{3, 48, 57, 58, 49, 57, 58, 80, 84};
        data.ahLst = vec![3, 48, 57, 58, 49, 57, 58, 80, 84];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 51, 0, 3, 20, 80, 84};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 51, 0, 3, 20, 80, 84];
        // Java: data.rect = new int[]{26, 26, 0, 1};
        data.rect = vec![26, 26, 0, 1];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 26, 1, 116, 26, 1, 156, 196, 1, 120, 26, 1, 0, 26, 1, 0, 132, 1, 168, 204, 1, 0, 136, 1, 0, 1, 1, 120, 1, 1, 176, 216, 1, 116, 1, 1, 26, 1, 1, 26, 136, 1, 148, 184, 1, 26, 132, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 26, 1, 116, 26, 1, 156, 196, 1, 120, 26, 1, 0, 26, 1, 0, 132, 1, 168, 204, 1, 0, 136, 1, 0, 1, 1, 120, 1, 1, 176, 216, 1, 116, 1, 1, 26, 1, 1, 26, 136, 1, 148, 184, 1, 26, 132, 2]];
        // Java: return data;
        data
    }

    // Java: static PreDefData get_wedgeRoundRectCallout()
    fn get_wedgeRoundRectCallout() -> PreDefData {
        // Java: PreDefData data = new PreDefData();
        let mut data = PreDefData::new();
        // Java: data.avcLst = new int[]{-20833, 62500, 16667, 100000, 0, 7, 2, 10, 5, 12, 29289, -2147483647, Integer.MAX_VALUE, 50000, 1};
        data.avcLst = vec![-20833, 62500, 16667, 100000, 0, 7, 2, 10, 5, 12, 29289, -2147483647, i32::MAX, 50000, 1];
        // Java: data.gdLst = new int[]{196609, 0, 48, 51, 196609, 1, 49, 51, 65539, 48, 65539, 49, 196608, 61, 52, 71, 196608, 61, 52, 73, 196608, 2, 63, 52, 196608, 3, 67, 52, 196609, 63, 1, 0, 65539, 67, 65539, 91, 196608, 95, 52, 97, 196614, 63, 53, 54, 196614, 63, 55, 56, 196609, 0, 103, 57, 196609, 0, 107, 57, 196614, 67, 53, 54, 196614, 67, 55, 56, 196609, 1, 119, 57, 196609, 1, 123, 57, 196614, 63, 26, 83, 196614, 99, 26, 135, 196614, 75, 26, 139, 196614, 67, 111, 83, 196614, 99, 147, 111, 196614, 63, 83, 0, 196614, 99, 0, 155, 196614, 75, 0, 159, 196614, 67, 83, 111, 196614, 99, 167, 111, 196614, 63, 127, 87, 196614, 99, 127, 175, 196614, 67, 26, 87, 196614, 99, 183, 26, 196614, 79, 26, 187, 196614, 63, 87, 127, 196614, 99, 127, 195, 196614, 67, 87, 1, 196614, 99, 203, 1, 196614, 79, 1, 207, 196608, 50, 52, 62, 196609, 28, 215, 51, 196608, 0, 52, 219, 196608, 1, 52, 219, 196609, 219, 58, 51, 196608, 0, 52, 231, 196608, 1, 52, 231};
        data.gdLst = vec![196609, 0, 48, 51, 196609, 1, 49, 51, 65539, 48, 65539, 49, 196608, 61, 52, 71, 196608, 61, 52, 73, 196608, 2, 63, 52, 196608, 3, 67, 52, 196609, 63, 1, 0, 65539, 67, 65539, 91, 196608, 95, 52, 97, 196614, 63, 53, 54, 196614, 63, 55, 56, 196609, 0, 103, 57, 196609, 0, 107, 57, 196614, 67, 53, 54, 196614, 67, 55, 56, 196609, 1, 119, 57, 196609, 1, 123, 57, 196614, 63, 26, 83, 196614, 99, 26, 135, 196614, 75, 26, 139, 196614, 67, 111, 83, 196614, 99, 147, 111, 196614, 63, 83, 0, 196614, 99, 0, 155, 196614, 75, 0, 159, 196614, 67, 83, 111, 196614, 99, 167, 111, 196614, 63, 127, 87, 196614, 99, 127, 175, 196614, 67, 26, 87, 196614, 99, 183, 26, 196614, 79, 26, 187, 196614, 63, 87, 127, 196614, 99, 127, 195, 196614, 67, 87, 1, 196614, 99, 203, 1, 196614, 79, 1, 207, 196608, 50, 52, 62, 196609, 28, 215, 51, 196608, 0, 52, 219, 196608, 1, 52, 219, 196609, 219, 58, 51, 196608, 0, 52, 231, 196608, 1, 52, 231];
        // Java: data.ahLst = new int[]{3, 48, 59, 60, 49, 59, 60, 83, 87};
        data.ahLst = vec![3, 48, 59, 60, 49, 59, 60, 83, 87];
        // Java: data.cxnLst = new int[]{15, 2, 26, 19, 26, 3, 20, 2, 1, 52, 0, 3, 20, 83, 87};
        data.cxnLst = vec![15, 2, 26, 19, 26, 3, 20, 2, 1, 52, 0, 3, 20, 83, 87];
        // Java: data.rect = new int[]{231, 231, 235, 239};
        data.rect = vec![231, 231, 235, 239];
        // Java: data.pathLst = new int[][]{{11, 13, 0, 26, 219, 3, 219, 219, 19, 20, 1, 111, 26, 1, 151, 191, 1, 115, 26, 1, 223, 26, 3, 219, 219, 15, 20, 1, 0, 127, 1, 163, 199, 1, 0, 131, 1, 0, 227, 3, 219, 219, 52, 20, 1, 115, 1, 1, 171, 211, 1, 111, 1, 1, 219, 1, 3, 219, 219, 20, 20, 1, 26, 131, 1, 143, 179, 1, 26, 127, 2}};
        data.pathLst = vec![vec![11, 13, 0, 26, 219, 3, 219, 219, 19, 20, 1, 111, 26, 1, 151, 191, 1, 115, 26, 1, 223, 26, 3, 219, 219, 15, 20, 1, 0, 127, 1, 163, 199, 1, 0, 131, 1, 0, 227, 3, 219, 219, 52, 20, 1, 115, 1, 1, 171, 211, 1, 111, 1, 1, 219, 1, 3, 219, 219, 20, 20, 1, 26, 131, 1, 143, 179, 1, 26, 127, 2]];
        // Java: return data;
        data
    }

}
