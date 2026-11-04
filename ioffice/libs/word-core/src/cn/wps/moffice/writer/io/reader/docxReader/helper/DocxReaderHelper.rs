// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.IOStyle
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::IOStyle::IOStyle;

// java public class DocxReaderHelper
pub struct DocxReaderHelper {}

// java public class DocxReaderHelper
impl DocxReaderHelper {
    // java public static void setTblStylePrType(String type, IOStyle ioStyle)
    pub fn setTblStylePrType(r#type: Option<&str>, ioStyle: Option<&mut IOStyle>) {
      // java IOAssert.assertNotNull("type should not be null", type);
      IOAssert::assertNotNull_msg("type should not be null", &r#type);
      let Some(r#type) = r#type else {
        panic!("type should not be null");
      };
      // java IOAssert.assertNotNull("ioStyle should not be null", ioStyle);
      IOAssert::assertNotNull_msg("ioStyle should not be null", &ioStyle);
      let Some(ioStyle) = ioStyle else {
        panic!("ioStyle should not be null");
      };
      // java if ("wholeTable".equals(type))
      if "wholeTable" == r#type {
         // java ioStyle.mTblStyleRPrId = 87;
         ioStyle.mTblStyleRPrId = 87;
         // java ioStyle.mTblStylePPrId = 264;
         ioStyle.mTblStylePPrId = 264;
         // java ioStyle.mTblStyleTblPrId = 329;
         ioStyle.mTblStyleTblPrId = 329;
      } else if "firstRow" == r#type {
         // java ioStyle.mTblStyleRPrId = 88;
         ioStyle.mTblStyleRPrId = 88;
         // java ioStyle.mTblStylePPrId = 265;
         ioStyle.mTblStylePPrId = 265;
         // java ioStyle.mTblStyleTblPrId = 330;
         ioStyle.mTblStyleTblPrId = 330;
      } else if "lastRow" == r#type {
         // java ioStyle.mTblStyleRPrId = 89;
         ioStyle.mTblStyleRPrId = 89;
         // java ioStyle.mTblStylePPrId = 266;
         ioStyle.mTblStylePPrId = 266;
         // java ioStyle.mTblStyleTblPrId = 331;
         ioStyle.mTblStyleTblPrId = 331;
      } else if "firstCol" == r#type {
         // java ioStyle.mTblStyleRPrId = 92;
         ioStyle.mTblStyleRPrId = 92;
         // java ioStyle.mTblStylePPrId = 269;
         ioStyle.mTblStylePPrId = 269;
         // java ioStyle.mTblStyleTblPrId = 334;
         ioStyle.mTblStyleTblPrId = 334;
      } else if "lastCol" == r#type {
         // java ioStyle.mTblStyleRPrId = 93;
         ioStyle.mTblStyleRPrId = 93;
         // java ioStyle.mTblStylePPrId = 270;
         ioStyle.mTblStylePPrId = 270;
         // java ioStyle.mTblStyleTblPrId = 335;
         ioStyle.mTblStyleTblPrId = 335;
      } else if "band1Vert" == r#type {
         // java ioStyle.mTblStyleRPrId = 94;
         ioStyle.mTblStyleRPrId = 94;
         // java ioStyle.mTblStylePPrId = 271;
         ioStyle.mTblStylePPrId = 271;
         // java ioStyle.mTblStyleTblPrId = 336;
         ioStyle.mTblStyleTblPrId = 336;
      } else if "band2Vert" == r#type {
         // java ioStyle.mTblStyleRPrId = 95;
         ioStyle.mTblStyleRPrId = 95;
         // java ioStyle.mTblStylePPrId = 272;
         ioStyle.mTblStylePPrId = 272;
         // java ioStyle.mTblStyleTblPrId = 337;
         ioStyle.mTblStyleTblPrId = 337;
      } else if "band1Horz" == r#type {
         // java ioStyle.mTblStyleRPrId = 90;
         ioStyle.mTblStyleRPrId = 90;
         // java ioStyle.mTblStylePPrId = 267;
         ioStyle.mTblStylePPrId = 267;
         // java ioStyle.mTblStyleTblPrId = 332;
         ioStyle.mTblStyleTblPrId = 332;
      } else if "band2Horz" == r#type {
         // java ioStyle.mTblStyleRPrId = 91;
         ioStyle.mTblStyleRPrId = 91;
         // java ioStyle.mTblStylePPrId = 268;
         ioStyle.mTblStylePPrId = 268;
         // java ioStyle.mTblStyleTblPrId = 333;
         ioStyle.mTblStyleTblPrId = 333;
      } else if "neCell" == r#type {
         // java ioStyle.mTblStyleRPrId = 96;
         ioStyle.mTblStyleRPrId = 96;
         // java ioStyle.mTblStylePPrId = 273;
         ioStyle.mTblStylePPrId = 273;
         // java ioStyle.mTblStyleTblPrId = 338;
         ioStyle.mTblStyleTblPrId = 338;
      } else if "nwCell" == r#type {
         // java ioStyle.mTblStyleRPrId = 97;
         ioStyle.mTblStyleRPrId = 97;
         // java ioStyle.mTblStylePPrId = 274;
         ioStyle.mTblStylePPrId = 274;
         // java ioStyle.mTblStyleTblPrId = 339;
         ioStyle.mTblStyleTblPrId = 339;
      } else if "seCell" == r#type {
         // java ioStyle.mTblStyleRPrId = 98;
         ioStyle.mTblStyleRPrId = 98;
         // java ioStyle.mTblStylePPrId = 275;
         ioStyle.mTblStylePPrId = 275;
         // java ioStyle.mTblStyleTblPrId = 340;
         ioStyle.mTblStyleTblPrId = 340;
      } else if "swCell" == r#type {
         // java ioStyle.mTblStyleRPrId = 99;
         ioStyle.mTblStyleRPrId = 99;
         // java ioStyle.mTblStylePPrId = 276;
         ioStyle.mTblStylePPrId = 276;
         // java ioStyle.mTblStyleTblPrId = 341;
         ioStyle.mTblStyleTblPrId = 341;
      } else {
         // java IOAssert.fail("it should not reach here");
         IOAssert::fail_msg("it should not reach here");
         // java ioStyle.mTblStyleRPrId = 87;
         ioStyle.mTblStyleRPrId = 87;
         // java ioStyle.mTblStylePPrId = 264;
         ioStyle.mTblStylePPrId = 264;
         // java ioStyle.mTblStyleTblPrId = 329;
         ioStyle.mTblStyleTblPrId = 329;
      }
    }
}
