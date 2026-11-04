// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::BRC;

// java public class BRCBorderTool
#[derive(Default)]
pub struct BRCBorderTool {
}

// java public class BRCBorderTool
impl BRCBorderTool {
    // java public static float getActualStrokeWidth(int brcType, float strokeWidth)
    pub fn getActualStrokeWidth(brcType: i32, strokeWidth: f32) -> f32 {
      // java float exWidth = 0.0F;
      let mut exWidth: f32 = 0.0;
      // java switch (brcType)
      match brcType {
        // java case 1: case 6: case 7: case 8: case 9: case 22:
        1 | 6 | 7 | 8 | 9 | 22 => {
          // java exWidth = 0.0F;
          exWidth = 0.0;
        }
        // java case 2: case 4: case 5: case 23: default:
        2 | 4 | 5 | 23 | _ => {
          // java exWidth = 0.0F;
          exWidth = 0.0;
        }
        // java case 3:
        3 => {
          // java exWidth = strokeWidth * 2.0F;
          exWidth = strokeWidth * 2.0;
        }
        // java case 10:
        10 => {
          // java exWidth = strokeWidth * 4.0F;
          exWidth = strokeWidth * 4.0;
        }
        // java case 11: case 12:
        11 | 12 => {
          // java exWidth = 1.5F;
          exWidth = 1.5;
        }
        // java case 13:
        13 => {
          // java exWidth = 3.0F;
          exWidth = 3.0;
        }
        // java case 14: case 15:
        14 | 15 => {
          // java exWidth = strokeWidth / 2.0F + strokeWidth / 2.0F;
          exWidth = strokeWidth / 2.0 + strokeWidth / 2.0;
        }
        // java case 16:
        16 => {
          // java exWidth = strokeWidth / 2.0F + strokeWidth / 2.0F + strokeWidth / 2.0F + strokeWidth / 2.0F;
          exWidth = strokeWidth / 2.0 + strokeWidth / 2.0 + strokeWidth / 2.0 + strokeWidth / 2.0;
        }
        // java case 17: case 18:
        17 | 18 => {
          // java exWidth = 2.25F;
          exWidth = 2.25;
        }
        // java case 19:
        19 => {
          // java exWidth = 3.0F + strokeWidth;
          exWidth = 3.0 + strokeWidth;
        }
        // java case 20:
        20 => {
          // java exWidth = 2.25F;
          exWidth = 2.25;
        }
        // java case 21:
        21 => {
          // java exWidth = 4.5F;
          exWidth = 4.5;
        }
        // java case 24: case 25:
        24 | 25 => {
          // java if (strokeWidth < 3.0F)
          if strokeWidth < 3.0 {
            // java exWidth = 1.5F;
            exWidth = 1.5;
          }
          // java else
          else {
            // java exWidth = 3.0F;
            exWidth = 3.0;
          }
        }
      }
      // java return strokeWidth + exWidth;
      return strokeWidth + exWidth;
    }

    // java public static float getActualRatio(int brcType)
    pub fn getActualRatio(brcType: i32) -> f32 {
      // java switch (brcType)
      match brcType {
        // java case 1: case 6: case 7: case 8: case 9: case 22:
        1 | 6 | 7 | 8 | 9 | 22 => {
          // java return 1.0F;
          return 1.0;
        }
        // java case 2: case 4: case 5: default:
        2 | 4 | 5 | _ => {
          // java return 1.0F;
          return 1.0;
        }
        // java case 3:
        3 => {
          // java return 3.0F;
          return 3.0;
        }
        // java case 10:
        10 => {
          // java return 5.0F;
          return 5.0;
        }
        // java case 11: case 12:
        11 | 12 => {
          // java return 6.0F;
          return 6.0;
        }
        // java case 13: case 14: case 15: case 23:
        13 | 14 | 15 | 23 => {
          // java return 8.0F;
          return 8.0;
        }
        // java case 16: case 19:
        16 | 19 => {
          // java return 12.0F;
          return 12.0;
        }
        // java case 17: case 18:
        17 | 18 => {
          // java return 7.0F;
          return 7.0;
        }
        // java case 20:
        20 => {
          // java return 2.0F;
          return 2.0;
        }
        // java case 21:
        21 => {
          // java return 4.0F;
          return 4.0;
        }
        // java case 24: case 25:
        24 | 25 => {
          // java return 9.0F;
          return 9.0;
        }
      }
    }

    // java public static BRC getVaildBrc(BRC brc)
    pub fn getVaildBrc(brc: Option<&BRC>) -> Option<BRC> {
      // java return brc != null && brc.getBorderType() != 0 && brc.getBorderType() <= 27 ? brc : null;
      brc.filter(|b| b.getBorderType() != 0 && b.getBorderType() <= 27).cloned()
    }

}
