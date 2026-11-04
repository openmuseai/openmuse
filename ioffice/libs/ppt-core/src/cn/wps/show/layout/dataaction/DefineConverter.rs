// java public class DefineConverter
#[derive(Default)]
pub struct DefineConverter {
}

// java public class DefineConverter
impl DefineConverter {
    // java public static int convertPhHolder(int swType)
    pub fn convertPhHolder(swType: i32) -> i32 {
      // java int type = 1;
      let mut r#type: i32 = 1;
      // java switch (swType)
      match swType {
        // java case 0:
        0 => {
          // java type = 0;
          r#type = 0;
        }
        // java case 1:
        1 => {
          // java type = 1;
          r#type = 1;
        }
        // java case 2:
        2 => {
          // java type = 2;
          r#type = 2;
        }
        // java case 3:
        3 => {
          // java type = 3;
          r#type = 3;
        }
        // java case 4:
        4 => {
          // java type = 4;
          r#type = 4;
        }
        // java case 5:
        5 => {
          // java type = 5;
          r#type = 5;
        }
        // java case 6:
        6 => {
          // java type = 6;
          r#type = 6;
        }
        // java case 7:
        7 => {
          // java type = 7;
          r#type = 7;
        }
        // java case 8:
        8 => {
          // java type = 8;
          r#type = 8;
        }
        // java case 9:
        9 => {
          // java type = 9;
          r#type = 9;
        }
        // java case 10:
        10 => {
          // java type = 10;
          r#type = 10;
        }
        // java case 11:
        11 => {
          // java type = 11;
          r#type = 11;
        }
        // java case 12:
        12 => {
          // java type = 12;
          r#type = 12;
        }
        // java case 13:
        13 => {
          // java type = 13;
          r#type = 13;
        }
        // java case 14:
        14 => {
          // java type = 14;
          r#type = 14;
        }
        // java case 15:
        15 => {
          // java type = 15;
          r#type = 15;
        }
        // java default:
        _ => {
        }
      }
      // java return type;
      return r#type;
    }

    // java public static int convertTxtAnchor(int dgType)
    pub fn convertTxtAnchor(dgType: i32) -> i32 {
      // java int type = 0;
      let mut r#type: i32 = 0;
      // java switch (dgType)
      match dgType {
        // java case 0:
        0 => {
          // java type = 0;
          r#type = 0;
        }
        // java case 1:
        1 => {
          // java type = 1;
          r#type = 1;
        }
        // java case 2:
        2 => {
          // java type = 2;
          r#type = 2;
        }
        // java case 3:
        3 => {
          // java type = 3;
          r#type = 3;
        }
        // java case 4:
        4 => {
          // java type = 4;
          r#type = 4;
        }
        // java default:
        _ => {
        }
      }
      // java return type;
      return r#type;
    }

}
