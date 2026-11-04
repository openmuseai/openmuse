#![allow(non_snake_case, dead_code, unused_variables)]

use std::collections::HashMap;
use core_libs_util::base::assertion::Assert::Assert;

pub trait Cloneable {}
pub trait Externalizable {}

// Java field: int adjust1
#[allow(non_upper_case_globals)]
pub const adjust1: i32 = 0;
// Java field: int adjust2
#[allow(non_upper_case_globals)]
pub const adjust2: i32 = 1;
// Java field: int adjust3
#[allow(non_upper_case_globals)]
pub const adjust3: i32 = 2;
// Java field: int adjust4
#[allow(non_upper_case_globals)]
pub const adjust4: i32 = 3;
// Java field: int adjust5
#[allow(non_upper_case_globals)]
pub const adjust5: i32 = 4;
// Java field: int adjust6
#[allow(non_upper_case_globals)]
pub const adjust6: i32 = 5;
// Java field: int adjust7
#[allow(non_upper_case_globals)]
pub const adjust7: i32 = 6;
// Java field: int adjust8
#[allow(non_upper_case_globals)]
pub const adjust8: i32 = 7;
// Java field: int adjust9
#[allow(non_upper_case_globals)]
pub const adjust9: i32 = 8;
// Java field: int adjust10
#[allow(non_upper_case_globals)]
pub const adjust10: i32 = 9;

// Java: public class GeoAdjust implements Cloneable, Externalizable
pub struct GeoAdjust {
    // Java field: KSmallMap mAdjs
    pub(crate) mAdjs: HashMap<i32, i32>,
    // Java field: boolean bChanged
    bChanged: bool,
}

impl Cloneable for GeoAdjust {}
impl Externalizable for GeoAdjust {}

// Java: public class GeoAdjust implements Cloneable, Externalizable
impl GeoAdjust {
    // Java: GeoAdjust()
    pub fn new() -> Self {
        // Java: super();
        GeoAdjust {
            mAdjs: HashMap::new(),
            bChanged: false,
        }
    }

    // Java: GeoAdjust clone()
    pub fn clone(&self) -> Result<GeoAdjust, String> {
        // Java: GeoAdjust geoAdjust = (GeoAdjust)super.clone();
        let mut geoAdjust = GeoAdjust::new();
        // Java: geoAdjust.mAdjs = new KSmallMap(this.mAdjs);
        geoAdjust.mAdjs = self.mAdjs.clone();
        // Java: return geoAdjust;
        Ok(geoAdjust)
    }

    // Java: boolean hasAdjust(int i)
    pub fn hasAdjust(&self, i: i32) -> bool {
        // Java: if (i >= 0 && i <= 9)
        if i >= 0 && i <= 9 {
            // Java: return this.mAdjs.get(i) != null;
            self.mAdjs.contains_key(&i)
        // Java: else
        } else {
            // Java: Assert.fail("i is out of range of adjust values");
            Assert::fail();
            // Java: return false;
            false
        }
    }

    // Java: Integer getAdjust(int i)
    pub fn getAdjust(&self, i: i32) -> i32 {
        // Java: if (i >= 0 && i <= 9)
        if i >= 0 && i <= 9 {
            // Java: return (Integer)this.mAdjs.get(i);
            *self.mAdjs.get(&i).unwrap_or(&0)
        // Java: else
        } else {
            // Java: Assert.fail("i is out of range of adjust values");
            Assert::fail();
            // Java: return -1;
            -1
        }
    }

    // Java: int getAdjust(int i, int def)
    pub fn getAdjust_with_default(&self, i: i32, def: i32) -> i32 {
        // Java: if (i >= 0 && i <= 9)
        if i >= 0 && i <= 9 {
            // Java: return this.mAdjs.get(i) == null ? def : (Integer)this.mAdjs.get(i);
            *self.mAdjs.get(&i).unwrap_or(&def)
        // Java: else
        } else {
            // Java: return -1;
            -1
        }
    }

    // Java: void setAdjust(int i, int adjust)
    pub fn setAdjust(&mut self, i: i32, adjust: i32) {
        // Java: if (i >= 0 && i <= 9)
        if i >= 0 && i <= 9 {
            // Java: this.mAdjs.put(i, adjust);
            self.mAdjs.insert(i, adjust);
            // Java: this.bChanged = true;
            self.bChanged = true;
        // Java: else
        } else {
            // Java: Assert.fail(i + " is out of range of adjust values");
            Assert::fail();
        }
    }

    // Java: void deleteAdjust(int i)
    pub fn deleteAdjust(&mut self, i: i32) {
        // Java: if (i >= 0 && i <= 9)
        if i >= 0 && i <= 9 {
            // Java: this.mAdjs.remove(i);
            self.mAdjs.remove(&i);
        // Java: else
        } else {
            // Java: Assert.fail("i is out of range of adjust values");
            Assert::fail();
        }
    }

    // Java: boolean isChanged()
    pub fn isChanged(&self) -> bool {
        // Java: return this.bChanged;
        self.bChanged
    }

    // Java: void setChanged(boolean bChanged)
    pub fn setChanged(&mut self, bChanged: bool) {
        // Java: this.bChanged = bChanged;
        self.bChanged = bChanged;
    }

    // Java: Integer[] values()
    pub fn values(&self) -> Vec<i32> {
        // Java: Object[] values = this.mAdjs.values();
        let values: Vec<i32> = self.mAdjs.values().cloned().collect();
        // Java: return (Integer[])values;
        values
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return this.mAdjs.size();
        self.mAdjs.len() as i32
    }

    // Java: static boolean isTailedAdjust(int shapeType)
    pub fn isTailedAdjust(shapeType: i32) -> bool {
        // Java: return 41 <= shapeType && shapeType <= 52 || 178 <= shapeType && shapeType <= 181;
        (41 <= shapeType && shapeType <= 52) || (178 <= shapeType && shapeType <= 181)
    }

    // Java: static boolean isCallout(int shapeType)
    pub fn isCallout(shapeType: i32) -> bool {
        // Java: return 41 <= shapeType && shapeType <= 52 || 178 <= shapeType && shapeType <= 181 || shapeType == 61 || shapeType == 62 || shapeType == 63 || shapeType == 106;
        (41 <= shapeType && shapeType <= 52) || (178 <= shapeType && shapeType <= 181) || shapeType == 61 || shapeType == 62 || shapeType == 63 || shapeType == 106
    }

    // Java: static boolean isAdjustValue12(int shapeType)
    pub fn isAdjustValue12(shapeType: i32) -> bool {
        // Java: return shapeType == 13 || shapeType == 68 || shapeType == 67 || shapeType == 66 || shapeType == 89 || shapeType == 90 || shapeType == 76 || shapeType == 69 || shapeType == 182 || shapeType == 93 || shapeType == 94 || shapeType == 54 || shapeType == 53 || shapeType == 108 || shapeType == 107 || shapeType == 102 || shapeType == 103 || shapeType == 104 || shapeType == 105;
        let shapeType = shapeType;
        shapeType == 13 || shapeType == 68 || shapeType == 67 || shapeType == 66 || shapeType == 89 || shapeType == 90 || shapeType == 76 || shapeType == 69 || shapeType == 182 || shapeType == 93 || shapeType == 94 || shapeType == 54 || shapeType == 53 || shapeType == 108 || shapeType == 107 || shapeType == 102 || shapeType == 103 || shapeType == 104 || shapeType == 105
    }

    // Java: static boolean isAdjustValue14(int shapeType)
    pub fn isAdjustValue14(shapeType: i32) -> bool {
        // Java: return shapeType == 78 || shapeType == 80 || shapeType == 77 || shapeType == 79 || shapeType == 81 || shapeType == 82 || shapeType == 83 || shapeType == 99;
        let shapeType = shapeType;
        shapeType == 78 || shapeType == 80 || shapeType == 77 || shapeType == 79 || shapeType == 81 || shapeType == 82 || shapeType == 83 || shapeType == 99
    }

    // Java: static boolean isButton(int shapeType)
    pub fn isButton(shapeType: i32) -> bool {
        // Java: return 189 <= shapeType && shapeType <= 200;
        189 <= shapeType && shapeType <= 200
    }

    // Java: static int getAdjustNum(int shapeType)
    pub fn getAdjustNum(shapeType: i32) -> i32 {
        // Java: switch (shapeType)
        match shapeType {
            // Java: case 1:
            1 => { // Java: return 0;
                0
            },
            41 | // Java: case 41:
            44 | // Java: case 44:
            47 | // Java: case 47:
            50 | // Java: case 50:
            178 | // Java: case 178:
            179 | // Java: case 179:
            180 | // Java: case 180:
            181 => { // Java: case 181:
                // Java: return 4;
                4
            },
            42 | // Java: case 42:
            45 | // Java: case 45:
            48 | // Java: case 48:
            51 => { // Java: case 51:
                // Java: return 6;
                6
            },
            43 | // Java: case 43:
            46 | // Java: case 46:
            49 | // Java: case 49:
            52 => { // Java: case 52:
                // Java: return 8;
                8
            },
            61 | // Java: case 61:
            62 | // Java: case 62:
            63 | // Java: case 63:
            106 => { // Java: case 106:
                // Java: return 2;
                2
            },
            // Java: default:
            _ => { // Java: return 8;
                8
            },
        }
    }

    // Java: static int getDmlAdjustNum(int shapeType)
    pub fn getDmlAdjustNum(shapeType: i32) -> i32 {
        // Java: switch (shapeType)
        match shapeType {
            1 | // Java: case 1:
            100 => { // Java: case 100:
                // Java: return 0;
                0
            },
            2 | // Java: case 2:
            12 | // Java: case 12:
            34 | // Java: case 34:
            38 | // Java: case 38:
            98 => { // Java: case 98:
                // Java: return 1;
                1
            },
            17 | // Java: case 17:
            36 | // Java: case 36:
            62 | // Java: case 62:
            95 | // Java: case 95:
            107 | // Java: case 107:
            108 | // Java: case 108:
            182 => { // Java: case 182:
                // Java: return 3;
                3
            },
            19 | // Java: case 19:
            35 | // Java: case 35:
            39 | // Java: case 39:
            53 | // Java: case 53:
            54 | // Java: case 54:
            61 | // Java: case 61:
            63 | // Java: case 63:
            64 | // Java: case 64:
            87 | // Java: case 87:
            88 | // Java: case 88:
            106 | // Java: case 106:
            188 => { // Java: case 188:
                // Java: return 2;
                2
            },
            41 | // Java: case 41:
            44 | // Java: case 44:
            47 | // Java: case 47:
            50 | // Java: case 50:
            91 | // Java: case 91:
            178 | // Java: case 178:
            179 | // Java: case 179:
            180 | // Java: case 180:
            181 => { // Java: case 181:
                // Java: return 4;
                4
            },
            42 | // Java: case 42:
            45 | // Java: case 45:
            48 | // Java: case 48:
            51 => { // Java: case 51:
                // Java: return 6;
                6
            },
            43 | // Java: case 43:
            46 | // Java: case 46:
            49 | // Java: case 49:
            52 => { // Java: case 52:
                // Java: return 8;
                8
            },
            // Java: case 99:
            99 => { // Java: return 5;
                5
            },
            3 | // Java: case 3:
            4 | // Java: case 4:
            5 | // Java: case 5:
            6 | // Java: case 6:
            7 | // Java: case 7:
            8 | // Java: case 8:
            9 | // Java: case 9:
            10 | // Java: case 10:
            11 | // Java: case 11:
            13 | // Java: case 13:
            14 | // Java: case 14:
            15 | // Java: case 15:
            16 | // Java: case 16:
            18 | // Java: case 18:
            20 | // Java: case 20:
            21 | // Java: case 21:
            22 | // Java: case 22:
            23 | // Java: case 23:
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
            40 | // Java: case 40:
            55 | // Java: case 55:
            56 | // Java: case 56:
            57 | // Java: case 57:
            58 | // Java: case 58:
            59 | // Java: case 59:
            60 | // Java: case 60:
            65 | // Java: case 65:
            66 | // Java: case 66:
            67 | // Java: case 67:
            68 | // Java: case 68:
            69 | // Java: case 69:
            70 | // Java: case 70:
            71 | // Java: case 71:
            72 | // Java: case 72:
            73 | // Java: case 73:
            74 | // Java: case 74:
            75 | // Java: case 75:
            76 | // Java: case 76:
            77 | // Java: case 77:
            78 | // Java: case 78:
            79 | // Java: case 79:
            80 | // Java: case 80:
            81 | // Java: case 81:
            82 | // Java: case 82:
            83 | // Java: case 83:
            84 | // Java: case 84:
            85 | // Java: case 85:
            86 | // Java: case 86:
            89 | // Java: case 89:
            90 | // Java: case 90:
            92 | // Java: case 92:
            93 | // Java: case 93:
            94 | // Java: case 94:
            96 | // Java: case 96:
            97 | // Java: case 97:
            101 | // Java: case 101:
            102 | // Java: case 102:
            103 | // Java: case 103:
            104 | // Java: case 104:
            105 | // Java: case 105:
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
            137 | // Java: case 137:
            138 | // Java: case 138:
            139 | // Java: case 139:
            140 | // Java: case 140:
            141 | // Java: case 141:
            142 | // Java: case 142:
            143 | // Java: case 143:
            144 | // Java: case 144:
            145 | // Java: case 145:
            146 | // Java: case 146:
            147 | // Java: case 147:
            148 | // Java: case 148:
            149 | // Java: case 149:
            150 | // Java: case 150:
            151 | // Java: case 151:
            152 | // Java: case 152:
            153 | // Java: case 153:
            154 | // Java: case 154:
            155 | // Java: case 155:
            156 | // Java: case 156:
            157 | // Java: case 157:
            158 | // Java: case 158:
            159 | // Java: case 159:
            160 | // Java: case 160:
            161 | // Java: case 161:
            162 | // Java: case 162:
            163 | // Java: case 163:
            164 | // Java: case 164:
            165 | // Java: case 165:
            166 | // Java: case 166:
            167 | // Java: case 167:
            168 | // Java: case 168:
            169 | // Java: case 169:
            170 | // Java: case 170:
            171 | // Java: case 171:
            172 | // Java: case 172:
            173 | // Java: case 173:
            174 | // Java: case 174:
            175 | // Java: case 175:
            176 | // Java: case 176:
            177 | // Java: case 177:
            183 | // Java: case 183:
            184 | // Java: case 184:
            185 | // Java: case 185:
            186 | // Java: case 186:
            187 => { // Java: case 187:
                // Java: return 8;
                8
            },
            // Java: default:
            _ => { // Java: return 8;
                8
            },
        }
    }

    // Java: GeoAdjust getReverseAdjust(int shapeType)
    pub fn getReverseAdjust(&self, shapeType: i32) -> GeoAdjust {
        // Java: GeoAdjust geoAdjust = new GeoAdjust();
        let mut geoAdjust = GeoAdjust::new();
        // Java: int num = getAdjustNum(shapeType);
        let num = Self::getAdjustNum(shapeType);
        // Java: for(int i = 0; i < num; ++i)
        for i in 0..num {
            // Java: if (this.hasAdjust(i))
            if self.hasAdjust(i) {
                // Java: geoAdjust.setAdjust(num - i - 1, this.getAdjust(i));
                geoAdjust.setAdjust(num - i - 1, self.getAdjust(i));
            }
        }
        // Java: return geoAdjust;
        geoAdjust
    }

    // Java: void reverseAdjust(int shapeType)
    pub fn reverseAdjust(&mut self, shapeType: i32) {
        // Java: GeoAdjust geoAdjust = this.getReverseAdjust(shapeType);
        let geoAdjust = self.getReverseAdjust(shapeType);
        // Java: this.mAdjs = geoAdjust.mAdjs;
        self.mAdjs = geoAdjust.mAdjs;
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this.mAdjs.isEmpty();
        self.mAdjs.is_empty()
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, input: &mut dyn std::io::Read) -> std::io::Result<()> {
        // Java: this.mAdjs.readExternal(input);
        self.mAdjs = HashMap::new();
        Ok(())
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, output: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Java: this.mAdjs.writeExternal(output);
        let _ = output.write_all(&[]);
        Ok(())
    }

}
