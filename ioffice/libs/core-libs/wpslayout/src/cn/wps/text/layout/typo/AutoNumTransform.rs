// Java: import cn.wps.kfc.numfmt.numbering.Numberings
use kfc::cn::wps::kfc::numfmt::numbering::Numberings::Numberings;
use crate::cn::wps::text::layout::typo::TextAutoNum::{TextAutoNum, Factory};

// Java field: int WD_WHITE_CIRCLE_NUM_START
pub(crate) const WD_WHITE_CIRCLE_NUM_START: i32 = 128;
// Java field: int WD_BLACK_CIRCLE_NUM_START
pub(crate) const WD_BLACK_CIRCLE_NUM_START: i32 = 139;

// Java: public class AutoNumTransform
pub struct AutoNumTransform {
}

// Java: public enum WingdingsType
#[derive(Clone, Copy, PartialEq)]
pub enum WingdingsType {
    BLACK_CIRCLE_NUM,
    WHITE_CIRCLE_NUM,
}

// Java: public class AutoNumTransform
impl AutoNumTransform {
    // Java: AutoNumTransform()
    pub fn new() -> Self {
        // Java: super();
        AutoNumTransform {}
    }

    // Java: String getAutoBulletStr(int numType, int bulletNumber)
    pub fn getAutoBulletStr(&self, numType: i32, bulletNumber: i32) -> String {
        // Java: String bulletStr = String.valueOf('•');
        let mut bulletStr = String::from("\u{2022}");
        // Java: Numberings numberings = new Numberings();
        let mut numberings = Numberings::new();
        // Java: bulletNumber = TextAutoNum.amendNumRange(bulletNumber);
        let bulletNumber = TextAutoNum::amendNumRange(bulletNumber);
        // Java: switch (numType)
        match numType
        {
            // Java: case 0:
            0 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 4) + ".";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 4) + ".";
                // Java: break;
            }
            // Java: case 1:
            1 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 3) + ".";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 3) + ".";
                // Java: break;
            }
            // Java: case 2:
            2 => {
                // Java: bulletStr = TextAutoNum.Factory.create(numType).getNumbering(bulletNumber);
                bulletStr = Factory::create(numType).getNumbering(bulletNumber);
                // Java: break;
            }
            // Java: case 3:
            3 => {
                // Java: bulletStr = TextAutoNum.Factory.create(numType).getNumbering(bulletNumber);
                bulletStr = Factory::create(numType).getNumbering(bulletNumber);
                // Java: break;
            }
            // Java: case 4:
            4 => {
                // Java: bulletStr = "(" + numberings.getNumbering((long)bulletNumber, 2) + ")";
                bulletStr = format!("({})", numberings.getNumbering_with_nfc(bulletNumber as i64, 2));
                // Java: break;
            }
            // Java: case 5:
            5 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 2) + ")";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 2) + ")";
                // Java: break;
            }
            // Java: case 6:
            6 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 2) + ".";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 2) + ".";
                // Java: break;
            }
            // Java: case 7:
            7 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 1) + ".";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 1) + ".";
                // Java: break;
            }
            // Java: case 8:
            8 => {
                // Java: bulletStr = "(" + numberings.getNumbering((long)bulletNumber, 4) + ")";
                bulletStr = format!("({})", numberings.getNumbering_with_nfc(bulletNumber as i64, 4));
                // Java: break;
            }
            // Java: case 9:
            9 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 4) + ")";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 4) + ")";
                // Java: break;
            }
            // Java: case 10:
            10 => {
                // Java: bulletStr = "(" + numberings.getNumbering((long)bulletNumber, 3) + ")";
                bulletStr = format!("({})", numberings.getNumbering_with_nfc(bulletNumber as i64, 3));
                // Java: break;
            }
            // Java: case 11:
            11 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 3) + ")";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 3) + ")";
                // Java: break;
            }
            // Java: case 12:
            12 => {
                // Java: bulletStr = TextAutoNum.Factory.create(numType).getNumbering(bulletNumber);
                bulletStr = Factory::create(numType).getNumbering(bulletNumber);
                // Java: break;
            }
            // Java: case 13:
            13 => {
                // Java: bulletStr = TextAutoNum.Factory.create(numType).getNumbering(bulletNumber);
                bulletStr = Factory::create(numType).getNumbering(bulletNumber);
                // Java: break;
            }
            // Java: case 14:
            14 => {
                // Java: bulletStr = "(" + numberings.getNumbering((long)bulletNumber, 1) + ")";
                bulletStr = format!("({})", numberings.getNumbering_with_nfc(bulletNumber as i64, 1));
                // Java: break;
            }
            // Java: case 15:
            15 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 1) + ")";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 1) + ")";
                // Java: break;
            }
            // Java: case 16:
            16 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 11);
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 11);
                // Java: break;
            }
            // Java: case 17:
            17 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 11) + ".";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 11) + ".";
                // Java: break;
            }
            // Java: case 18:
            18 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 28);
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 28);
                // Java: break;
            }
            // Java: case 19:
            19 => {
                // Java: numberings.getNumbering((long)bulletNumber, 28);
                let _ = numberings.getNumbering_with_nfc(bulletNumber as i64, 28);
                // Java: bulletStr = this.wdCircleNumbering(bulletNumber, AutoNumTransform.WingdingsType.WHITE_CIRCLE_NUM);
                bulletStr = self.wdCircleNumbering(bulletNumber, WingdingsType::WHITE_CIRCLE_NUM);
                // Java: break;
            }
            // Java: case 20:
            20 => {
                // Java: numberings.getNumbering((long)bulletNumber, 28);
                let _ = numberings.getNumbering_with_nfc(bulletNumber as i64, 28);
                // Java: bulletStr = this.wdCircleNumbering(bulletNumber, AutoNumTransform.WingdingsType.BLACK_CIRCLE_NUM);
                bulletStr = self.wdCircleNumbering(bulletNumber, WingdingsType::BLACK_CIRCLE_NUM);
                // Java: break;
            }
            // Java: case 21:
            21 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 11);
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 11);
                // Java: break;
            }
            // Java: case 22:
            22 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 11) + ".";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 11) + ".";
                // Java: break;
            }
            // Java: case 23:
            23 => {
                // Java: bulletStr = "-" + numberings.getNumbering((long)bulletNumber, 46);
                bulletStr = format!("-{}", numberings.getNumbering_with_nfc(bulletNumber as i64, 46));
                // Java: break;
            }
            // Java: case 24:
            24 => {
                // Java: bulletStr = "-" + numberings.getNumbering((long)bulletNumber, 48);
                bulletStr = format!("-{}", numberings.getNumbering_with_nfc(bulletNumber as i64, 48));
                // Java: break;
            }
            // Java: case 25:
            25 => {
                // Java: bulletStr = "-" + numberings.getNumbering((long)bulletNumber, 47);
                bulletStr = format!("-{}", numberings.getNumbering_with_nfc(bulletNumber as i64, 47));
                // Java: break;
            }
            // Java: case 26:
            26 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 17);
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 17);
                // Java: break;
            }
            // Java: case 27:
            27 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 17) + ".";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 17) + ".";
                // Java: break;
            }
            // Java: case 28:
            28 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 14);
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 14);
                // Java: break;
            }
            // Java: case 29:
            29 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 14) + ".";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 14) + ".";
                // Java: break;
            }
            // Java: case 30:
            30 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 53) + ".";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 53) + ".";
                // Java: break;
            }
            // Java: case 31:
            31 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 53) + ")";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 53) + ")";
                // Java: break;
            }
            // Java: case 32:
            32 => {
                // Java: bulletStr = "(" + numberings.getNumbering((long)bulletNumber, 53) + ")";
                bulletStr = format!("({})", numberings.getNumbering_with_nfc(bulletNumber as i64, 53));
                // Java: break;
            }
            // Java: case 33:
            33 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 54) + ".";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 54) + ".";
                // Java: break;
            }
            // Java: case 34:
            34 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 54) + ")";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 54) + ")";
                // Java: break;
            }
            // Java: case 35:
            35 => {
                // Java: bulletStr = "(" + numberings.getNumbering((long)bulletNumber, 54) + ")";
                bulletStr = format!("({})", numberings.getNumbering_with_nfc(bulletNumber as i64, 54));
                // Java: break;
            }
            // Java: case 36:
            36 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 50) + ".";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 50) + ".";
                // Java: break;
            }
            // Java: case 37:
            37 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 51) + ".";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 51) + ".";
                // Java: break;
            }
            // Java: case 38:
            38 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 17) + ".";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 17) + ".";
                // Java: break;
            }
            // Java: case 39:
            39 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 51) + ")";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 51) + ")";
                // Java: break;
            }
            // Java: case 40:
            40 => {
                // Java: bulletStr = numberings.getNumbering((long)bulletNumber, 49) + ".";
                bulletStr = numberings.getNumbering_with_nfc(bulletNumber as i64, 49) + ".";
            }
            _ => {}
        }
        // Java: return bulletStr;
        bulletStr
    }

    // Java: String wdCircleNumbering(int bulletNumber, WingdingsType type)
    fn wdCircleNumbering(&self, bulletNumber: i32, r#type: WingdingsType) -> String {
        // Java: String wdStr = "";
        let mut wdStr = String::new();
        // Java: if (bulletNumber >= 1 && bulletNumber <= 10)
        if bulletNumber >= 1 && bulletNumber <= 10
        {
            // Java: switch (type)
            match r#type
            {
                // Java: case BLACK_CIRCLE_NUM:
                WingdingsType::BLACK_CIRCLE_NUM => {
                    // Java: wdStr = String.valueOf((char)(139 + bulletNumber - 1));
                    wdStr = char::from_u32((139 + bulletNumber - 1) as u32).map_or(String::new(), |c| c.to_string());
                    // Java: break;
                }
                // Java: case WHITE_CIRCLE_NUM:
                WingdingsType::WHITE_CIRCLE_NUM => {
                    // Java: wdStr = String.valueOf((char)(128 + bulletNumber - 1));
                    wdStr = char::from_u32((128 + bulletNumber - 1) as u32).map_or(String::new(), |c| c.to_string());
                }
            }
        }
        // Java: else
        else
        {
            // Java: wdStr = String.valueOf(bulletNumber);
            wdStr = bulletNumber.to_string();
        }
        // Java: return wdStr;
        wdStr
    }
}

// Java: public enum WingdingsType
impl WingdingsType {
    // Java: WingdingsType()
}
