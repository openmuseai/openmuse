// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.util.UnitUtil
use util::moffice::util::UnitUtil::UnitUtil;
// Java: import java.lang.Object
use stubs::java::lang::Object::Object;

// java public class CharFontTool
#[derive(Default)]
pub struct CharFontTool {
}

// java public class CharFontTool
impl CharFontTool {
    // java public static int getFontSizePropId(int hint, char c)
    pub fn getFontSizePropId(hint: i32, c: u16) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int nPropId;
      let mut nPropId: i32;
      // java if (hint == 0)
      if hint == 0 {
        // java if (!CharParse.getInstance().charIsRTL(c) && !TextTool.isIndic(c) && !TextTool.isComplexScript(c))
        if !CharParse::getInstance().charIsRTL(c) && !TextTool::isIndic(c) && !TextTool::isComplexScript(c) {
          // java nPropId = 10;
          nPropId = 10;
        }
        // java else
        else {
          // java nPropId = 37;
          nPropId = 37;
        }
      }
      // java else if (hint == 2)
      else if hint == 2 {
        // java nPropId = 37;
        nPropId = 37;
      }
      // java else
      else {
        // java nPropId = 10;
        nPropId = 10;
      }
      // java return nPropId;
      return nPropId;
      */
    }

    // java public static int getMultiPropertyId(int propertyId, int hint, char ch)
    pub fn getMultiPropertyId(propertyId: i32, hint: i32, ch: u16) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java switch (propertyId)
      match propertyId {
        // java case 3: case 4: case 35: case 36:
        3 | 4 | 35 | 36 => {
          // java return getCharFontPropId(hint, ch);
          return Self::getCharFontPropId(hint, ch);
        }
        // java case 10: case 37:
        10 | 37 => {
          // java return getFontSizePropId(hint, ch);
          return Self::getFontSizePropId(hint, ch);
        }
        // java default:
        _ => {
          // java return propertyId;
          return propertyId;
        }
      }
      */
    }

    // java public static Object getMultiDefaultValue(int propertyId)
    pub fn getMultiDefaultValue(propertyId: i32) -> Option<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java switch (propertyId)
      match propertyId {
        // java case 3:
        3 => {
          // java return "Times New Roman";
          return Some("Times New Roman");
        }
        // java case 4:
        4 => {
          // java return "宋体";
          return Some("宋体");
        }
        // java case 10:
        10 => {
          // java return 10.0F;
          return (10 as i32);
        }
        // java case 35:
        35 => {
          // java return "Times New Roman";
          return Some("Times New Roman");
        }
        // java case 36:
        36 => {
          // java return "宋体";
          return Some("宋体");
        }
        // java case 37:
        37 => {
          // java return 10.0F;
          return (10 as i32);
        }
        // java default:
        _ => {
          // java return null;
          return None;
        }
      }
      */
    }

    // java public static int getCharFontPropId(int hint, char c)
    pub fn getCharFontPropId(hint: i32, c: u16) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int nPropId;
      let mut nPropId: i32;
      // java if (hint == 0)
      if hint == 0 {
        // java if (CharParse.getInstance().charIsAscii(c))
        if CharParse::getInstance().charIsAscii(c) {
          // java nPropId = 3;
          nPropId = 3;
        }
        // java else if (CharParse.getInstance().charIsAsia(c, hint))
        else if CharParse::getInstance().charIsAsia(c, hint) {
          // java nPropId = 4;
          nPropId = 4;
        }
        // java else if (!CharParse.getInstance().charIsRTL(c) && !TextTool.isIndic(c) && !TextTool.isComplexScript(c))
        else if !CharParse::getInstance().charIsRTL(c) && !TextTool::isIndic(c) && !TextTool::isComplexScript(c) {
          // java if (CharParse.getInstance().isSymbolChar(c))
          if CharParse::getInstance().isSymbolChar(c) {
            // java nPropId = 35;
            nPropId = 35;
          }
          // java else
          else {
            // java nPropId = 3;
            nPropId = 3;
          }
        }
        // java else
        else {
          // java nPropId = 36;
          nPropId = 36;
        }
      }
      // java else if (hint == 2)
      else if hint == 2 {
        // java nPropId = 36;
        nPropId = 36;
      }
      // java else if (hint == 1)
      else if hint == 1 {
        // java if (CharParse.getInstance().charIsAscii(c))
        if CharParse::getInstance().charIsAscii(c) {
          // java nPropId = 3;
          nPropId = 3;
        }
        // java else
        else {
          // java nPropId = 4;
          nPropId = 4;
        }
      }
      // java else
      else {
        // java nPropId = 35;
        nPropId = 35;
      }
      // java return nPropId;
      return nPropId;
      */
    }

    // java public static int getFirstCharSize(PropertySet runProperty, int gridWidth)
    pub fn getFirstCharSize(runProperty: &PropertySet, gridWidth: i32) -> i32 {
      // java int fontSize = UnitUtil.PT2TWIPS(runProperty.getFloat(10, UnitUtil.EMU2PT(gridWidth)));
      let fontSize: i32 = UnitUtil::PT2TWIPS(runProperty.getFloat__int_float(10, UnitUtil::EMU2PT(gridWidth)));
      // java float charScale = runProperty.getFloat(23, 1.0F);
      let mut charScale: f32 = runProperty.getFloat__int_float(23, 1.0);
      // java if (charScale <= 0.0F)
      if charScale <= 0.0 {
        // java charScale = 1.0F;
        charScale = 1.0;
      }
      // java int fontSpace = runProperty.getInt(24, 0);
      let fontSpace: i32 = runProperty.getInt__int_int(24, 0);
      // java return (int)((float)fontSize * charScale + (float)fontSpace);
      return ((fontSize as f32) * charScale + (fontSpace as f32)) as i32;
    }

}
