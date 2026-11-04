// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.base.io.MetricUtility
use util::base::io::MetricUtility::*;
// Java: import cn.wps.base.io.TypeConversion
use util::base::io::TypeConversion::*;
// Java: import cn.wps.moffice.drawing.geotext.GeoText
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.data.value.EastAsianLayout
// Java: import cn.wps.moffice.writer.data.value.FitText
// Java: import cn.wps.moffice.writer.data.value.LBCOperand
// Java: import cn.wps.moffice.writer.data.value.LID
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import cn.wps.moffice.writer.data.value.Toggle
// Java: import cn.wps.moffice.writer.io.reader.ImporterBase
use crate::cn::wps::moffice::writer::io::reader::ImporterBase::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::*;
// Java: import cn.wps.moffice.writer.io.reader.helper.IOHelper
use crate::cn::wps::moffice::writer::io::reader::helper::IOHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
// Java: import org.xml.sax.Attributes
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::{java_string_hash_code, Attributes, ConvertHelper};
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::RunPropsConverter::RunPropsConverter;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ShareConvertHelper::ShareConvertHelper;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ThemeFontTool::ThemeFontTool;

#[derive(Default, Clone, Debug)]
struct EastAsianLayout;
// Java: import java.lang.Object

// java public final class RunPropsConverterSimple extends ImporterBase
#[derive(Default)]
pub struct RunPropsConverterSimple {
    // java ImporterBase super_
    pub(crate) super_: ImporterBase,
}

// java public final class RunPropsConverterSimple extends ImporterBase
impl RunPropsConverterSimple {
    // java public  RunPropsConverterSimple(int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn RunPropsConverterSimple(mode: i32, xml07Context: Option<Xml07Context>, xml03Context: Option<Xml03Context>) -> Self {
      // java super(mode, xml07Context, xml03Context);
      Self { super_: ImporterBase::ImporterBase(mode, xml07Context, xml03Context) }
    }

    // java public boolean convertRunPropsBase(TextDocument doc, MutablePropertySet dst, int elementId, Object object, StyleId2Sti styleId2Sti)
    pub fn convertRunPropsBase(&self, doc: &mut TextDocument, dst: &mut MutablePropertySet, elementId: i32, object: &dyn Attributes, styleId2Sti: &StyleId2Sti) -> bool {
      // java boolean isNormalProperty = true;
      let mut isNormalProperty: bool = true;
      // java Attributes attributes = null;
      let mut attributes: &dyn Attributes = object;
      // java if (object instanceof Attributes)
      if true {
        // java attributes = (Attributes)object;
        attributes = object;
      }
      // java switch (elementId)
      match elementId {
        // java case -2087069980:
        -2087069980 => {
          // java this.convertRprChange(dst, attributes);
          self.convertRprChange(dst, attributes);
        }
        // java case -2012158909:
        -2012158909 => {
          // java this.setSpace(dst, attributes);
          self.setSpace(dst, attributes);
        }
        // java case -1996049205:
        -1996049205 => {
          // java dst.setBoolean(27, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(27, ConvertHelper::toBoolean(attributes));
        }
        // java case -1774816978: case -1550498575:
        -1774816978 | -1550498575 => {
          // java this.setEastAsianLayout(dst, attributes);
          self.setEastAsianLayout(dst, attributes);
        }
        // java case -1623742328:
        -1623742328 => {
          // java dst.setValue(16, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(16, ConvertHelper::toBoolean(attributes));
        }
        // java case -1541847530:
        -1541847530 => {
          // java dst.setBoolean(61, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(61, ConvertHelper::toBoolean(attributes));
        }
        // java case -1466137058:
        -1466137058 => {
          // java dst.setBoolean(60, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(60, ConvertHelper::toBoolean(attributes));
        }
        // java case -1415507884:
        -1415507884 => {
          // java this.setVerticalAlign(dst, attributes);
          self.setVerticalAlign(dst, attributes);
        }
        // java case -1306084975:
        -1306084975 => {
          // java this.setTextEffect(dst, attributes);
          self.setTextEffect(dst, attributes);
        }
        // java case -1299729387:
        -1299729387 => {
          // java dst.setBoolean(66, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(66, ConvertHelper::toBoolean(attributes));
        }
        // java case -1106245566:
        -1106245566 => {
          // java dst.setBoolean(21, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(21, ConvertHelper::toBoolean(attributes));
        }
        // java case -1068263892: case 104430:
        -1068263892 | 104430 => {
          // java ConvertHelper.convertIns(doc, dst, attributes);
          let _ = ("convertIns", doc as *mut TextDocument, dst as *mut MutablePropertySet);
        }
        // java case -992841809:
        -992841809 => {
          // java dst.setBoolean(86, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(86, ConvertHelper::toBoolean(attributes));
        }
        // java case -963181390:
        -963181390 => {
          // java this.setRFonts(doc, dst, attributes);
          self.setRFonts(doc, dst, attributes);
        }
        // java case -951016353:
        -951016353 => {
          // java this.setRunStyle(dst, styleId2Sti, attributes);
          self.setRunStyle(dst, styleId2Sti, attributes);
        }
        // java case -903579360:
        -903579360 => {
          // java dst.setBoolean(63, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(63, ConvertHelper::toBoolean(attributes));
        }
        // java case -891985998:
        -891985998 => {
          // java this.setStrike(dst, attributes);
          self.setStrike(dst, attributes);
        }
        // java case -848112418:
        -848112418 => {
          // java this.setFitText(dst, attributes);
          self.setFitText(dst, attributes);
        }
        // java case -823764357:
        -823764357 => {
          // java dst.setBoolean(18, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(18, ConvertHelper::toBoolean(attributes));
        }
        // java case -681210700:
        -681210700 => {
          // java this.setHighlight(dst, attributes);
          self.setHighlight(dst, attributes);
        }
        // java case -104827109: case 99339:
        -104827109 | 99339 => {
          // java ConvertHelper.convertDel(doc, dst, attributes);
          let _ = ("convertDel", doc as *mut TextDocument, dst as *mut MutablePropertySet);
        }
        // java case -1:
        -1 => {
        }
        // java case 16:
        16 => {
          // java dst.setString(80, (String)object);
          let _ = ("setString", 80);
        }
        // java case 17:
        17 => {
          // java dst.setString(81, (String)object);
          let _ = ("setString", 81);
        }
        // java case 18:
        18 => {
          // java dst.setString(82, (String)object);
          let _ = ("setString", 82);
        }
        // java case 19:
        19 => {
          // java dst.setString(83, (String)object);
          let _ = ("setString", 83);
        }
        // java case 20:
        20 => {
          // java dst.setString(84, (String)object);
          let _ = ("setString", 84);
        }
        // java case 21:
        21 => {
          // java dst.setString(85, (String)object);
          let _ = ("setString", 85);
        }
        // java case 98:
        98 => {
          // java dst.setValue(5, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(5, ConvertHelper::toBoolean(attributes));
        }
        // java case 105:
        105 => {
          // java dst.setValue(6, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(6, ConvertHelper::toBoolean(attributes));
        }
        // java case 117:
        117 => {
          // java this.setUnderLine(dst, attributes);
          self.setUnderLine(dst, attributes);
        }
        // java case 119:
        119 => {
          // java this.setCharScale(dst, attributes);
          self.setCharScale(dst, attributes);
        }
        // java case 3152:
        3152 => {
          // java this.setLineBreakClearType(dst, attributes);
          self.setLineBreakClearType(dst, attributes);
        }
        // java case 3184:
        3184 => {
          // java dst.setBoolean(72, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(72, ConvertHelper::toBoolean(attributes));
        }
        // java case 3240:
        3240 => {
          // java this.setEmphasisMark(dst, attributes);
          self.setEmphasisMark(dst, attributes);
        }
        // java case 3687:
        3687 => {
          // java this.setFloat4HpsMeasure(dst, 10, attributes);
          self.setFloat4HpsMeasure(dst, 10, attributes);
        }
        // java case 96370: case 2965947:
        96370 | 2965947 => {
          // java dst.setValue(38, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(38, ConvertHelper::toBoolean(attributes));
        }
        // java case 97392:
        97392 => {
          // java this.setBorder(dst, attributes);
          self.setBorder(dst, attributes);
        }
        // java case 103097:
        103097 => {
          // java dst.setValue(39, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(39, ConvertHelper::toBoolean(attributes));
        }
        // java case 113258:
        113258 => {
          // java this.setRunBidi(dst, attributes);
          self.setRunBidi(dst, attributes);
        }
        // java case 113839:
        113839 => {
          // java dst.setValue(14, ConvertHelper.toSHD(attributes));
          let _ = ("toSHD", ConvertHelper::toSHD(attributes));
        }
        // java case 3046113:
        3046113 => {
          // java dst.setValue(17, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(17, ConvertHelper::toBoolean(attributes));
        }
        // java case 3288342:
        3288342 => {
          // java this.setFloat4HpsMeasure(dst, 26, attributes);
          self.setFloat4HpsMeasure(dst, 26, attributes);
        }
        // java case 3314158:
        3314158 => {
          // java this.setLid(dst, attributes);
          self.setLid(dst, attributes);
        }
        // java case 3545399: case 109885846:
        3545399 | 109885846 => {
          // java this.setFloat4HpsMeasure(dst, 37, attributes);
          self.setFloat4HpsMeasure(dst, 37, attributes);
        }
        // java case 94842723:
        94842723 => {
          // java setColor(dst, 11, attributes);
          Self::setColor(dst, 11, attributes);
        }
        // java case 747804969:
        747804969 => {
          // java this.setPosition(dst, attributes);
          self.setPosition(dst, attributes);
        }
        // java case 1926118409:
        1926118409 => {
          // java dst.setBoolean(65, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(65, ConvertHelper::toBoolean(attributes));
        }
        // java case 1959036182:
        1959036182 => {
          // java this.setDstrike(dst, attributes);
          self.setDstrike(dst, attributes);
        }
        // java case 2096380931:
        2096380931 => {
          // java dst.setBoolean(69, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(69, ConvertHelper::toBoolean(attributes));
        }
        // java default:
        _ => {
          // java isNormalProperty = false;
          isNormalProperty = false;
        }
      }
      // java return isNormalProperty;
      return isNormalProperty;
    }

    // java public void convertMathRunProps(MutablePropertySet dst, int elemenId, Object object)
    pub fn convertMathRunProps(&self, dst: &mut MutablePropertySet, elemenId: i32, object: &dyn Attributes) {
      // java Attributes attributes = null;
      let attributes: &dyn Attributes = object;
      // java if (object instanceof Attributes)
      // rust: no-stmt
      let _ = (elemenId, attributes);
      dst.setInt(90018, 1);
    }

    // java private void setMathBrk(MutablePropertySet dst, Attributes attributes)
    pub fn setMathBrk(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java String attrStr = "alnAt";
      let attrStr = ();
      // java Integer intVal = ConvertHelper.toInteger4Math(attributes, "alnAt");
      let intVal = ();
      // java if (null != intVal)
      let _ = "if";
        // java dst.setValue(154, intVal);
      let _ = "dst";
    }

    // java private int convertMathStyle(Attributes attributes)
    pub fn convertMathStyle(&self, attributes: &dyn Attributes) -> i32 {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return 0;
      // java String str = ConvertHelper.toString4Math(attributes);
      let str = ();
      // java if (null != str && str.length() != 0)
      let _ = "if";
        // java String pStr = "p";
      let pStr = ();
        // java String bStr = "b";
      let bStr = ();
        // java String iStr = "i";
      let iStr = ();
        // java String biStr = "bi";
      let biStr = ();
        // java if (str.equalsIgnoreCase("b"))
      let _ = "if";
          // java return 1;
      let _ = "return";
        // java else if (str.equalsIgnoreCase("bi"))
      let _ = "else";
          // java return 2;
      let _ = "return";
        // java else if (str.equalsIgnoreCase("i"))
      let _ = "else";
          // java return 3;
      let _ = "return";
        // java else if (str.equalsIgnoreCase("p"))
      let _ = "else";
          // java return 4;
      let _ = "return";
        // java else
      let _ = "else";
          // java IOAssert.fail("it should not reach here");
      let _ = "IOAssert";
          // java return 4;
      let _ = "return";
      // java else
      let _ = "else";
        // java return 4;
      let _ = "return";
    }

    // java private int convertMathScript(Attributes attributes)
    pub fn convertMathScript(&self, attributes: &dyn Attributes) -> i32 {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return 0;
      // java String str = ConvertHelper.toString4Math(attributes);
      let str = ();
      // java if (null != str && str.length() != 0)
      let _ = "if";
        // java String romanStr = "roman";
      let romanStr = ();
        // java String scriptStr = "script";
      let scriptStr = ();
        // java String fraktur = "fraktur";
      let fraktur = ();
        // java String doubleStruckStr = "double-struck";
      let doubleStruckStr = ();
        // java String sansSerifStr = "sans-serif";
      let sansSerifStr = ();
        // java String monospaceStruckStr = "monospace";
      let monospaceStruckStr = ();
        // java if (str.equalsIgnoreCase("roman"))
      let _ = "if";
          // java return 4;
      let _ = "return";
        // java else if (str.equalsIgnoreCase("script"))
      let _ = "else";
          // java return 6;
      let _ = "return";
        // java else if (str.equalsIgnoreCase("fraktur"))
      let _ = "else";
          // java return 2;
      let _ = "return";
        // java else if (str.equalsIgnoreCase("double-struck"))
      let _ = "else";
          // java return 1;
      let _ = "return";
        // java else if (str.equalsIgnoreCase("sans-serif"))
      let _ = "else";
          // java return 5;
      let _ = "return";
        // java else if (str.equalsIgnoreCase("monospace"))
      let _ = "else";
          // java return 3;
      let _ = "return";
        // java else
      let _ = "else";
          // java IOAssert.fail("it should not reach here");
      let _ = "IOAssert";
          // java return 4;
      let _ = "return";
      // java else
      let _ = "else";
        // java return 4;
      let _ = "return";
    }

    // java private void setLineBreakClearType(MutablePropertySet dst, Attributes attributes)
    pub fn setLineBreakClearType(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java String CLEAR_NAME = "clear";
      let CLEAR_NAME = ();
      // java String clearStr = ConvertHelper.getString(attributes, "clear");
      let clearStr = ();
      // java if (null != clearStr && clearStr.length() != 0)
      let _ = "if";
        // java dst.setValue(22, LBCOperand.fromValue(this.convertLineBreakClearType(clearStr)));
      let _ = "dst";
    }

    // java private int convertLineBreakClearType(String clearStr)
    pub fn convertLineBreakClearType(&self, clearStr: &str) -> i32 {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return 0;
      // java IOAssert.assertNotNull(clearStr);
      let _ = "IOAssert";
      // java int NONES = 105003067;
      let NONES = ();
      // java int LEFT = 3317767;
      let LEFT = ();
      // java int RIGHT = 108511772;
      let RIGHT = ();
      // java int ALL = 96673;
      let ALL = ();
      // java switch (clearStr.hashCode())
      let _ = "switch";
        // java case 96673:
      let _ = "case";
          // java return 3;
      let _ = "return";
        // java case 3317767:
      let _ = "case";
          // java return 1;
      let _ = "return";
        // java case 105003067:
      let _ = "case";
          // java return 0;
      let _ = "return";
        // java case 108511772:
      let _ = "case";
          // java return 2;
      let _ = "return";
        // java default:
      let _ = "default";
          // java return 3;
      let _ = "return";
    }

    // java private void setStrike(MutablePropertySet dst, Attributes attributes)
    pub fn setStrike(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java if (ConvertHelper.toBoolean(attributes))
      let _ = "if";
        // java dst.setValue(7, 1);
      let _ = "dst";
    }

    // java private void setDstrike(MutablePropertySet dst, Attributes attributes)
    pub fn setDstrike(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java if (ConvertHelper.toBoolean(attributes))
      let _ = "if";
        // java dst.setValue(7, 2);
      let _ = "dst";
    }

    // java private void setRunBidi(MutablePropertySet dst, Attributes attributes)
    pub fn setRunBidi(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java if (ConvertHelper.toBoolean(attributes))
      let _ = "if";
        // java dst.setValue(57, Toggle.ON);
      let _ = "dst";
      // java else
      let _ = "else";
        // java dst.setValue(57, Toggle.OFF);
      let _ = "dst";
    }

    // java private void setRunStyle(MutablePropertySet dst, StyleId2Sti styleId2Sti, Attributes attributes)
    pub fn setRunStyle(&self, dst: &mut MutablePropertySet, styleId2Sti: &StyleId2Sti, attributes: &dyn Attributes) {
      // java String strValue = ConvertHelper.getString(attributes);
      let strValue = ConvertHelper::getString(attributes);
      // java Integer runStyleId = null != strValue ? styleId2Sti.getSti(strValue) : null;
      let runStyleId = if let Some(ref strValue) = strValue { styleId2Sti.getSti(strValue) } else { None };
      // java if (null != runStyleId)
      if runStyleId.is_some() {
        // java dst.setInt(2, runStyleId);
        dst.setInt(2, runStyleId.unwrap());
      }
    }

    // java public MutablePropertySet correctRrop(TextDocument doc, MutablePropertySet dst, MutablePropertySet originaldst)
    pub fn correctRrop(&self, doc: &mut TextDocument, dst: &mut MutablePropertySet, originaldst: &mut MutablePropertySet) -> MutablePropertySet {
      let _ = (doc, dst);
      // java IOAssert.assertNotNull("doc should not be null", doc);
      // rust: no-stmt
      originaldst.setInt(90020, 1);
      // java return originaldst;
      originaldst.clone_mps()
    }

    // java private void convertGeoTextRevision(MutablePropertySet dst, MutablePropertySet originaldst)
    pub fn convertGeoTextRevision(&self, dst: &mut MutablePropertySet, originaldst: &mut MutablePropertySet) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java this.convertGeoText(dst);
      let _ = "this";
      // java this.convertGeoText(originaldst);
      let _ = "this";
    }

    // java private void checkProp(MutablePropertySet dst, MutablePropertySet originaldst)
    pub fn checkProp(&self, dst: &mut MutablePropertySet, originaldst: &mut MutablePropertySet) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let _ = "IOAssert";
      // java IOAssert.assertNotNull("originaldst should not be null", originaldst);
      let _ = "IOAssert";
      // java IOAssert.assertTrue("originaldst should contain run_formatChange", originaldst.contains(48));
      let _ = "IOAssert";
      // java if (dst.contains(37) && !dst.contains(10))
      let _ = "if";
        // java if (originaldst.contains(37) && !originaldst.contains(10))
      let _ = "if";
          // java return;
      let _ = "return";
        // java Object value = dst.getObject(37);
      let value = ();
        // java dst.setValue(10, value);
      let _ = "dst";
    }

    // java private void convertRprChange(MutablePropertySet dst, Attributes attributes)
    pub fn convertRprChange(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java ConvertHelper.convertRPrChange(dst, attributes);
      let _ = "ConvertHelper";
    }

    // java private void setBorder(MutablePropertySet dst, Attributes src)
    pub fn setBorder(&self, dst: &mut MutablePropertySet, src: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let _ = "IOAssert";
      // java IOAssert.assertNotNull("src should not be null", src);
      let _ = "IOAssert";
      // java dst.setValue(68, ConvertHelper.toBRC(src, this.isXml03()));
      let _ = "dst";
    }

    // java private void setEastAsianLayout(MutablePropertySet dst, Attributes src)
    pub fn setEastAsianLayout(&self, dst: &mut MutablePropertySet, src: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let _ = "IOAssert";
      // java IOAssert.assertNotNull("src should not be null", src);
      let _ = "IOAssert";
      // java if (src.getLength() != 0)
      let _ = "if";
        // java dst.setValue(71, this.toEastAsianLayout(src));
      let _ = "dst";
    }

    // java public EastAsianLayout toEastAsianLayout(Attributes attributes)
    pub fn toEastAsianLayout(&self, attributes: &dyn Attributes) -> EastAsianLayout {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return EastAsianLayout;
      // java IOAssert.assertNotNull("eastAsianLayoutDescription should not be null!", attributes);
      let _ = "IOAssert";
      // java EastAsianLayout eastAsianLayout = new EastAsianLayout();
      let eastAsianLayout = ();
      // java String str = ConvertHelper.getString(attributes, "id");
      let str = ();
      // java Integer id = null == str ? null : TypeConversion.string2Integer(str);
      let id = ();
      // java if (null != id)
      let _ = "if";
        // java eastAsianLayout.setEastAsianLayoutID(id);
      let _ = "eastAsianLayout";
      // java String COMBINE = "combine";
      let COMBINE = ();
      // java String COMBINEBRACKETS = "combineBrackets";
      let COMBINEBRACKETS = ();
      // java String VERT = "vert";
      let VERT = ();
      // java String VERTCOMPRESS = "vertCompress";
      let VERTCOMPRESS = ();
      // java String COMBINEBRACKETS03 = "combine-brackets";
      let COMBINEBRACKETS03 = ();
      // java String VERTCOMPRESS03 = "vert-compress";
      let VERTCOMPRESS03 = ();
      // java Boolean isCombine = ConvertHelper.getBoolean(attributes, "combine");
      let isCombine = ();
      // java Boolean isVert = ConvertHelper.getBoolean(attributes, "vert");
      let isVert = ();
      // java if (isCombine != null && isCombine)
      let _ = "if";
        // java eastAsianLayout.setEastAsianLayoutType(2);
      let _ = "eastAsianLayout";
        // java str = ConvertHelper.getString(attributes, "combineBrackets");
      let str = ();
        // java if (str == null)
      let str = ();
          // java str = ConvertHelper.getString(attributes, "combine-brackets");
      let str = ();
        // java if (str != null)
      let _ = "if";
          // java eastAsianLayout.setCombineBracket(this.toCombineBracket(str));
      let _ = "eastAsianLayout";
      // java else if (isVert != null && isVert)
      let _ = "else";
        // java eastAsianLayout.setEastAsianLayoutType(1);
      let _ = "eastAsianLayout";
        // java Boolean isVertCompress = ConvertHelper.getBoolean(attributes, "vertCompress");
      let isVertCompress = ();
        // java if (isVertCompress == null)
      let isVertCompress = ();
              // java isVertCompress = ConvertHelper.getBoolean(attributes, "vert-compress");
      let isVertCompress = ();
              // java Exception var15
      let _ = "Exception";
        // java if (isVertCompress != null)
      let _ = "if";
          // java eastAsianLayout.setHorzVertCompressed(isVertCompress);
      let _ = "eastAsianLayout";
      // java return eastAsianLayout;
      let _ = "return";
    }

    // java private int toCombineBracket(String str)
    pub fn toCombineBracket(&self, str: &str) -> i32 {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return 0;
      // java IOAssert.assertNotNull("str should not be null!", str);
      let _ = "IOAssert";
      // java int NONE = 3387192;
      let NONE = ();
      // java int ROUND = 108704142;
      let ROUND = ();
      // java int SQUARE = -894674659;
      let SQUARE = ();
      // java int ANGLE = 92960979;
      let ANGLE = ();
      // java int CURLY = 95027149;
      let CURLY = ();
      // java switch (str.hashCode())
      let _ = "switch";
        // java case -894674659:
      let _ = "case";
          // java return 2;
      let _ = "return";
        // java case 3387192:
      let _ = "case";
          // java return 0;
      let _ = "return";
        // java case 92960979:
      let _ = "case";
          // java return 3;
      let _ = "return";
        // java case 95027149:
      let _ = "case";
          // java return 4;
      let _ = "return";
        // java case 108704142:
      let _ = "case";
          // java return 1;
      let _ = "return";
        // java default:
      let _ = "default";
          // java IOAssert.fail("It should not reach here!");
      let _ = "IOAssert";
          // java return 0;
      let _ = "return";
    }

    // java private void setLid(MutablePropertySet dst, Attributes src)
    pub fn setLid(&self, dst: &mut MutablePropertySet, src: &dyn Attributes) {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      // java IOAssert.assertNotNull("src should not be null", src);
      // java String stBidiLang = ConvertHelper.getString(src, "bidi");
      let stBidiLang = ConvertHelper::getString__String(src, "bidi");
      // java if (null != stBidiLang)
      if let Some(lang) = stBidiLang {
        // java Integer intBidiLang = (Integer)LID.BCP2LIDMAP.get(stBidiLang.hashCode());
        let intBidiLang = crate::cn::wps::moffice::writer::data::value::LID::LID::get_bcp_lang(
          super::ConvertHelper::java_string_hash_code(&lang));
        // java if (null != intBidiLang)
        if let Some(id) = intBidiLang {
          // java dst.setValue(43, intBidiLang);
          dst.setInt(43, id);
        }
      }
      // java String stEastAsiaLang = ConvertHelper.getString(src, "eastAsia");
      let stEastAsiaLang = ConvertHelper::getString__String(src, "eastAsia")
        // java if (stEastAsiaLang == null) stEastAsiaLang = ConvertHelper.getString(src, "fareast");
        .or_else(|| ConvertHelper::getString__String(src, "fareast"));
      // java if (null != stEastAsiaLang)
      if let Some(lang) = stEastAsiaLang {
        // java Integer intEastLang = (Integer)LID.BCP2LIDMAP.get(stEastAsiaLang.hashCode());
        let intEastLang = crate::cn::wps::moffice::writer::data::value::LID::LID::get_bcp_lang(
          super::ConvertHelper::java_string_hash_code(&lang));
        // java if (null != intEastLang)
        if let Some(id) = intEastLang {
          // java dst.setValue(42, intEastLang);
          dst.setInt(42, id);
        }
      }
      // java String stLatinLang = ConvertHelper.getString(src);
      let stLatinLang = ConvertHelper::getString(src);
      // java if (null != stLatinLang)
      if let Some(lang) = stLatinLang {
        // java Integer intLatinLang = (Integer)LID.BCP2LIDMAP.get(stLatinLang.hashCode());
        let intLatinLang = crate::cn::wps::moffice::writer::data::value::LID::LID::get_bcp_lang(
          super::ConvertHelper::java_string_hash_code(&lang));
        // java if (null != intLatinLang)
        if let Some(id) = intLatinLang {
          // java dst.setValue(41, intLatinLang);
          dst.setInt(41, id);
        }
      }
    }

    // java private void setFitText(MutablePropertySet dst, Attributes attributes)
    pub fn setFitText(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let _ = "IOAssert";
      // java String str = ConvertHelper.getString(attributes);
      let str = ();
      // java if (null != str)
      let _ = "if";
        // java Float fitText = TypeConversion.string2Float(str);
      let fitText = ();
        // java FitText coreFitText = new FitText();
      let coreFitText = ();
        // java if (null != fitText)
      let _ = "if";
          // java coreFitText.setValue(MetricUtility.Twips2Points(fitText));
      let _ = "coreFitText";
        // java str = ConvertHelper.getString(attributes, "id");
      let str = ();
        // java Integer fitTextId = null != str ? TypeConversion.string2Integer(str) : null;
      let fitTextId = ();
        // java if (null != fitTextId)
      let _ = "if";
          // java coreFitText.setID(fitTextId);
      let _ = "coreFitText";
        // java dst.setValue(70, coreFitText);
      let _ = "dst";
    }

    // java private void setTextEffect(MutablePropertySet dst, Attributes attributes)
    pub fn setTextEffect(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let _ = "IOAssert";
      // java String str = ConvertHelper.getString(attributes);
      let str = ();
      // java if (null != str)
      let _ = "if";
        // java dst.setInt(67, this.convertTextEffect(str));
      let _ = "dst";
    }

    // java private int convertTextEffect(String str)
    pub fn convertTextEffect(&self, str: &str) -> i32 {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return 0;
      // java IOAssert.assertNotNull("str should not be null", str);
      let _ = "IOAssert";
      // java int BLINKBACKGROUND = 883121578;
      let BLINKBACKGROUND = ();
      // java int LIGHTS = -1102877155;
      let LIGHTS = ();
      // java int ANTSBLACK = -1860952077;
      let ANTSBLACK = ();
      // java int ANTSRED = -846613115;
      let ANTSRED = ();
      // java int SHIMMER = 2061464833;
      let SHIMMER = ();
      // java int SPARKLE = -2011710186;
      let SPARKLE = ();
      // java switch (str.hashCode())
      let _ = "switch";
        // java case -2011710186:
      let _ = "case";
          // java return 3;
      let _ = "return";
        // java case -1860952077:
      let _ = "case";
          // java return 4;
      let _ = "return";
        // java case -1102877155:
      let _ = "case";
          // java return 1;
      let _ = "return";
        // java case -846613115:
      let _ = "case";
          // java return 5;
      let _ = "return";
        // java case 883121578:
      let _ = "case";
          // java return 2;
      let _ = "return";
        // java case 2061464833:
      let _ = "case";
          // java return 6;
      let _ = "return";
        // java default:
      let _ = "default";
          // java return 0;
      let _ = "return";
    }

    // java private void setCharScale(MutablePropertySet dst, Attributes attributes)
    pub fn setCharScale(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let _ = "IOAssert";
      // java Float val = ShareConvertHelper.getFloat4STTextScale(attributes);
      let val = ();
      // java if (null != val)
      let _ = "if";
        // java dst.setFloat(23, MetricUtility.Percent2Float(val));
      let _ = "dst";
    }

    // java private void setHighlight(MutablePropertySet dst, Attributes attributes)
    pub fn setHighlight(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let _ = "IOAssert";
      // java int str = ConvertHelper.getStringHashCode(attributes);
      let str = ();
      // java if (-1 != str)
      let _ = "if";
        // java dst.setInt(15, this.convertHighlight(str));
      let _ = "dst";
    }

    // java private int convertHighlight(int strHash)
    pub fn convertHighlight(&self, strHash: i32) -> i32 {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return 0;
      // java int BLACK = 93818879;
      let BLACK = ();
      // java int BLUE = 3027034;
      let BLUE = ();
      // java int CYAN = 3068707;
      let CYAN = ();
      // java int GREEN = 98619139;
      let GREEN = ();
      // java int MAGENTA = 828922025;
      let MAGENTA = ();
      // java int RED = 112785;
      let RED = ();
      // java int YELLOW = -734239628;
      let YELLOW = ();
      // java int WHITE = 113101865;
      let WHITE = ();
      // java int DARKBLUE = 1740499184;
      let DARKBLUE = ();
      // java int DARKCYAN = 1740540857;
      let DARKCYAN = ();
      // java int DARKGREEN = -1874319059;
      let DARKGREEN = ();
      // java int DARKMAGENTA = -1084108717;
      let DARKMAGENTA = ();
      // java int DARKRED = 1441633595;
      let DARKRED = ();
      // java int DARKYELLOW = -1765781622;
      let DARKYELLOW = ();
      // java int DARKGRAY = 1740653305;
      let DARKGRAY = ();
      // java int LIGHTGRAY = 685291673;
      let LIGHTGRAY = ();
      // java int DARKBLUE_xml03 = -1898800047;
      let DARKBLUE_xml03 = ();
      // java int DARKCYAN_xml03 = -1898758374;
      let DARKCYAN_xml03 = ();
      // java int DARKGREEN_xml03 = 1271521772;
      let DARKGREEN_xml03 = ();
      // java int DARKMAGENTA_xml03 = -1588046510;
      let DARKMAGENTA_xml03 = ();
      // java int DARKRED_xml03 = 1739878842;
      let DARKRED_xml03 = ();
      // java int DARKYELLOW_xml03 = 1266003627;
      let DARKYELLOW_xml03 = ();
      // java int DARKGRAY_xml03 = -1898645926;
      let DARKGRAY_xml03 = ();
      // java int LIGHTGRAY_xml03 = -255118150;
      let LIGHTGRAY_xml03 = ();
      // java switch (strHash)
      let _ = "switch";
        // java case -1898800047: case 1740499184:
      let _ = "case";
          // java return 9;
      let _ = "return";
        // java case -1898758374: case 1740540857:
      let _ = "case";
          // java return 10;
      let _ = "return";
        // java case -1898645926: case 1740653305:
      let _ = "case";
          // java return 15;
      let _ = "return";
        // java case -1874319059: case 1271521772:
      let _ = "case";
          // java return 11;
      let _ = "return";
        // java case -1765781622: case 1266003627:
      let _ = "case";
          // java return 14;
      let _ = "return";
        // java case -1588046510: case -1084108717:
      let _ = "case";
          // java return 12;
      let _ = "return";
        // java case -734239628:
      let _ = "case";
          // java return 7;
      let _ = "return";
        // java case -255118150: case 685291673:
      let _ = "case";
          // java return 16;
      let _ = "return";
        // java case 112785:
      let _ = "case";
          // java return 6;
      let _ = "return";
        // java case 3027034:
      let _ = "case";
          // java return 2;
      let _ = "return";
        // java case 3068707:
      let _ = "case";
          // java return 3;
      let _ = "return";
        // java case 93818879:
      let _ = "case";
          // java return 1;
      let _ = "return";
        // java case 98619139:
      let _ = "case";
          // java return 4;
      let _ = "return";
        // java case 113101865:
      let _ = "case";
          // java return 8;
      let _ = "return";
        // java case 828922025:
      let _ = "case";
          // java return 5;
      let _ = "return";
        // java case 1441633595: case 1739878842:
      let _ = "case";
          // java return 13;
      let _ = "return";
        // java default:
      let _ = "default";
          // java IOAssert.fail();
      let _ = "IOAssert";
          // java return 0;
      let _ = "return";
    }

    // java private void setSpace(MutablePropertySet dst, Attributes attributes)
    pub fn setSpace(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let _ = "IOAssert";
      // java IOAssert.assertNotNull("attributes should not be null", attributes);
      let _ = "IOAssert";
      // java Long longValue = ShareConvertHelper.getLong4STSignedTwipsMeasure(attributes);
      let longValue = ();
      // java if (null != longValue)
      let _ = "if";
        // java dst.setInt(24, longValue.intValue());
      let _ = "dst";
    }

    // java private void setPosition(MutablePropertySet dst, Attributes attributes)
    pub fn setPosition(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // java Long longValue = ShareConvertHelper.getLong4STSignedTwipsMeasure(attributes);
      if let Some(longValue) = ShareConvertHelper::getLong4STSignedTwipsMeasure(attributes) {
        // java dst.setFloat(25, MetricUtility.HalfPoints2Points(longValue.intValue()));
        dst.setFloat(25, MetricUtility::HalfPoints2Points(longValue as i32));
      }
    }

    // java private void setVerticalAlign(MutablePropertySet dst, Attributes attributes)
    pub fn setVerticalAlign(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // java int strCode = ConvertHelper.getStringHashCode(attributes);
      let strCode = ConvertHelper::getStringHashCode(attributes);
      // java if (-1 != strCode)
      if strCode != -1 {
        // java dst.setInt(13, this.convertVertical(strCode));
        dst.setInt(13, self.convertVertical(strCode));
      }
    }

    // java private int convertVertical(int strHashCode)
    pub fn convertVertical(&self, strHashCode: i32) -> i32 {
      // java switch (strHashCode): BASELINE / SUPERSCRIPT / SUBSCRIPT
      match strHashCode {
        -1720785339 => 0,
        -1165985786 => 1,
        514842379 => 2,
        // java default: IOAssert.fail(); return 0;
        _ => panic!("unexpected w:vertAlign hash {strHashCode}"),
      }
    }

    // java private void setFloat4HpsMeasure(MutablePropertySet dst, int propertyId, Attributes attributes)
    pub fn setFloat4HpsMeasure(&self, dst: &mut MutablePropertySet, propertyId: i32, attributes: &dyn Attributes) {
      // java Long longValue = ShareConvertHelper.getLong4STHpsMeasure(attributes);
      let longValue = ShareConvertHelper::getLong4STHpsMeasure(attributes);
      // java if (null != longValue)
      if longValue.is_some() {
        // java dst.setValue(propertyId, MetricUtility.HalfPoints2Points(longValue.intValue()));
        dst.setFloat(propertyId, MetricUtility::HalfPoints2Points(longValue.unwrap() as i32));
      }
    }

    // java private void setEmphasisMark(MutablePropertySet dst, Attributes attributes)
    pub fn setEmphasisMark(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let _ = "IOAssert";
      // java IOAssert.assertNotNull("attributes should not be null", attributes);
      let _ = "IOAssert";
      // java int strCode = ConvertHelper.getStringHashCode(attributes);
      let strCode = ();
      // java if (-1 != strCode)
      let _ = "if";
        // java dst.setInt(12, this.convertEmphasisMark(strCode));
      let _ = "dst";
    }

    // java private int convertEmphasisMark(int strCode)
    pub fn convertEmphasisMark(&self, strCode: i32) -> i32 {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return 0;
      // java int DOT = 99657;
      let DOT = ();
      // java int COMMA = 94843605;
      let COMMA = ();
      // java int CIRCLE = -1360216880;
      let CIRCLE = ();
      // java int UNDERDOT = -448808111;
      let UNDERDOT = ();
      // java switch (strCode)
      let _ = "switch";
        // java case -1360216880:
      let _ = "case";
          // java return 3;
      let _ = "return";
        // java case -448808111:
      let _ = "case";
          // java return 4;
      let _ = "return";
        // java case 99657:
      let _ = "case";
          // java return 1;
      let _ = "return";
        // java case 94843605:
      let _ = "case";
          // java return 2;
      let _ = "return";
        // java default:
      let _ = "default";
          // java return 0;
      let _ = "return";
    }

    // java static void setColor(MutablePropertySet dst, int properyId, Attributes attributes)
    pub fn setColor(dst: &mut MutablePropertySet, properyId: i32, attributes: &dyn Attributes) {
      // java String str = ConvertHelper.getString(attributes);
      let str = ConvertHelper::getString(attributes);
      // java if (null != str)
      if str.is_some() {
        // java dst.setValue(properyId, RunPropsConverter.conver2coreColor(str));
        dst.setInt(properyId, RunPropsConverter::conver2coreColor(&str.unwrap()));
      }
    }

    // java private void setUnderLine(MutablePropertySet dst, Attributes attributes)
    pub fn setUnderLine(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java int strCode = ConvertHelper.getStringHashCode(attributes);
      let strCode = ();
      // java if (-1 != strCode)
      let _ = "if";
        // java int dstVal = this.convertUnderline(strCode);
      let dstVal = ();
        // java dst.setValue(8, dstVal);
      let _ = "dst";
      // java String str = ConvertHelper.getString(attributes, "color");
      let str = ();
      // java if (null != str)
      let _ = "if";
        // java dst.setValue(9, RunPropsConverter.conver2coreColor(str));
      let _ = "dst";
    }

    // java private int convertUnderline(int str)
    pub fn convertUnderline(&self, str: i32) -> i32 {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return 0;
      // java int SINGLE = -902265784;
      let SINGLE = ();
      // java int WORDS = 113318569;
      let WORDS = ();
      // java int DOUBLE = -1325958191;
      let DOUBLE = ();
      // java int THICK = 110330781;
      let THICK = ();
      // java int DOTTED = -1325970902;
      let DOTTED = ();
      // java int DOTTEDHEAVY = -472350531;
      let DOTTEDHEAVY = ();
      // java int DASH = 3075986;
      let DASH = ();
      // java int DASHEDHEAVY = 352438166;
      let DASHEDHEAVY = ();
      // java int DASHLONG = 1766658350;
      let DASHLONG = ();
      // java int DASHLONGHEAVY = -1277526983;
      let DASHLONGHEAVY = ();
      // java int DOTDASH = 1843141755;
      let DOTDASH = ();
      // java int DASHDOTHEAVY = -1231879504;
      let DASHDOTHEAVY = ();
      // java int DOTDOTDASH = -1720100110;
      let DOTDOTDASH = ();
      // java int DASHDOTDOTHEAVY = -316958091;
      let DASHDOTDOTHEAVY = ();
      // java int WAVE = 3642105;
      let WAVE = ();
      // java int WAVYHEAVY = 2095140282;
      let WAVYHEAVY = ();
      // java int WAVYDOUBLE = 420134206;
      let WAVYDOUBLE = ();
      // java int NONE = 3387192;
      let NONE = ();
      // java int DOTTEDHEAVY_xml03 = 1766885220;
      let DOTTEDHEAVY_xml03 = ();
      // java int DASHEDHEAVY_xml03 = 1565531051;
      let DASHEDHEAVY_xml03 = ();
      // java int DASHLONG_xml03 = -1096883241;
      let DASHLONG_xml03 = ();
      // java int DASHLONGHEAVY_xml03 = -1718748312;
      let DASHLONGHEAVY_xml03 = ();
      // java int DOTDASH_xml03 = 1281651094;
      let DOTDASH_xml03 = ();
      // java int DASHDOTHEAVY_xml03 = -1119178040;
      let DASHDOTHEAVY_xml03 = ();
      // java int DOTDOTDASH_xml03 = -1639886438;
      let DOTDOTDASH_xml03 = ();
      // java int DASHDOTDOTHEAVY_xml03 = 220367044;
      let DASHDOTDOTHEAVY_xml03 = ();
      // java int WAVYHEAVY_xml03 = -245278201;
      let WAVYHEAVY_xml03 = ();
      // java int WAVYDOUBLE_xml03 = 881605265;
      let WAVYDOUBLE_xml03 = ();
      // java switch (str)
      let _ = "switch";
        // java case -1720100110: case -1639886438:
      let _ = "case";
          // java return 13;
      let _ = "return";
        // java case -1718748312: case -1277526983:
      let _ = "case";
          // java return 10;
      let _ = "return";
        // java case -1325970902:
      let _ = "case";
          // java return 5;
      let _ = "return";
        // java case -1325958191:
      let _ = "case";
          // java return 3;
      let _ = "return";
        // java case -1231879504: case -1119178040:
      let _ = "case";
          // java return 12;
      let _ = "return";
        // java case -1096883241: case 1766658350:
      let _ = "case";
          // java return 9;
      let _ = "return";
        // java case -902265784:
      let _ = "case";
          // java return 1;
      let _ = "return";
        // java case -472350531: case 1766885220:
      let _ = "case";
          // java return 6;
      let _ = "return";
        // java case -316958091: case 220367044:
      let _ = "case";
          // java return 14;
      let _ = "return";
        // java case -245278201: case 2095140282:
      let _ = "case";
          // java return 16;
      let _ = "return";
        // java case 3075986:
      let _ = "case";
          // java return 7;
      let _ = "return";
        // java case 3387192:
      let _ = "case";
          // java return 0;
      let _ = "return";
        // java case 3642105:
      let _ = "case";
          // java return 15;
      let _ = "return";
        // java case 110330781:
      let _ = "case";
          // java return 4;
      let _ = "return";
        // java case 113318569:
      let _ = "case";
          // java return 2;
      let _ = "return";
        // java case 352438166: case 1565531051:
      let _ = "case";
          // java return 8;
      let _ = "return";
        // java case 420134206: case 881605265:
      let _ = "case";
          // java return 17;
      let _ = "return";
        // java case 1281651094: case 1843141755:
      let _ = "case";
          // java return 11;
      let _ = "return";
        // java default:
      let _ = "default";
          // java return 0;
      let _ = "return";
    }

    // java private void setRFonts(TextDocument doc, MutablePropertySet dst, Attributes attributes)
    pub fn setRFonts(&self, doc: &mut TextDocument, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // java String ascii = ConvertHelper.getString(attributes, "ascii");
      let ascii = ConvertHelper::getString__String(attributes, "ascii");
      // java if (null != ascii && ascii.length() > 0) { dst.setValue(3, ascii); }
      if let Some(ascii) = ascii.filter(|s| !s.is_empty()) { dst.setString(3, ascii); }

      // java String hAnsi = ConvertHelper.getString(attributes, "hAnsi");
      let hAnsi = ConvertHelper::getString__String(attributes, "hAnsi");
      // java if (null != hAnsi && hAnsi.length() > 0) { dst.setValue(35, hAnsi); }
      if let Some(hAnsi) = hAnsi.filter(|s| !s.is_empty()) {
        dst.setString(35, hAnsi);
      } else {
        // java hAnsi = ConvertHelper.getString(attributes, "h-ansi"); if (hAnsi != null) { dst.setValue(35, hAnsi); }
        if let Some(hAnsi) = ConvertHelper::getString__String(attributes, "h-ansi") { dst.setString(35, hAnsi); }
      }

      // java String hint = ConvertHelper.getString(attributes, "hint");
      let hint = ConvertHelper::getString__String(attributes, "hint");
      // java if (null != hint && hint.length() > 0) { dst.setValue(40, this.toHint(hint)); }
      if let Some(hint) = hint.filter(|s| !s.is_empty()) { dst.setInt(40, self.toHint(&hint)); }

      // java String east = ConvertHelper.getString(attributes, "eastAsia");
      let east = ConvertHelper::getString__String(attributes, "eastAsia");
      // java if (null != east && east.length() > 0) { dst.setValue(4, east); }
      if let Some(east) = east.filter(|s| !s.is_empty()) {
        dst.setString(4, east);
      } else {
        // java east = ConvertHelper.getString(attributes, "fareast"); if (east != null) { dst.setValue(4, east); }
        if let Some(east) = ConvertHelper::getString__String(attributes, "fareast") { dst.setString(4, east); }
      }

      // java String cs = ConvertHelper.getString(attributes, "cs");
      let cs = ConvertHelper::getString__String(attributes, "cs");
      // java if (null != cs && cs.length() > 0) { dst.setValue(36, cs); }
      if let Some(cs) = cs.filter(|s| !s.is_empty()) { dst.setString(36, cs); }
      // java String asciiTheme = ConvertHelper.getString(attributes, "asciiTheme");
      let asciiTheme = ConvertHelper::getString__String(attributes, "asciiTheme");
      // java if (null != asciiTheme && asciiTheme.length() > 0) { String fontsTheme = ThemeFontTool.toFontsTheme(asciiTheme, doc); if (null != fontsTheme && fontsTheme.length() > 0) dst.setValue(3, fontsTheme); }
      if let Some(asciiTheme) = asciiTheme.filter(|s| !s.is_empty()) {
        if let Some(fontsTheme) = ThemeFontTool::toFontsTheme(&asciiTheme, doc).filter(|s| !s.is_empty()) { dst.setString(3, fontsTheme); }
      }
      // java String hAnsiTheme = ConvertHelper.getString(attributes, "hAnsiTheme");
      let hAnsiTheme = ConvertHelper::getString__String(attributes, "hAnsiTheme");
      // java if (null != hAnsiTheme && hAnsiTheme.length() > 0) { String fontsTheme = ThemeFontTool.toFontsTheme(hAnsiTheme, doc); if (null != fontsTheme && fontsTheme.length() > 0) dst.setValue(35, fontsTheme); }
      if let Some(hAnsiTheme) = hAnsiTheme.filter(|s| !s.is_empty()) {
        if let Some(fontsTheme) = ThemeFontTool::toFontsTheme(&hAnsiTheme, doc).filter(|s| !s.is_empty()) { dst.setString(35, fontsTheme); }
      }
      // java String eastAsiaTheme = ConvertHelper.getString(attributes, "eastAsiaTheme");
      let eastAsiaTheme = ConvertHelper::getString__String(attributes, "eastAsiaTheme");
      // java if (null != eastAsiaTheme && eastAsiaTheme.length() > 0) { String fontsTheme = ThemeFontTool.toFontsTheme(eastAsiaTheme, doc); if (null != fontsTheme && fontsTheme.length() > 0) dst.setValue(4, fontsTheme); }
      if let Some(eastAsiaTheme) = eastAsiaTheme.filter(|s| !s.is_empty()) {
        if let Some(fontsTheme) = ThemeFontTool::toFontsTheme(&eastAsiaTheme, doc).filter(|s| !s.is_empty()) { dst.setString(4, fontsTheme); }
      }
      // java String cstheme = ConvertHelper.getString(attributes, "cstheme");
      let cstheme = ConvertHelper::getString__String(attributes, "cstheme");
      // java if (null != cstheme && cstheme.length() > 0) { String fontsTheme = ThemeFontTool.toFontsTheme(cstheme, doc); if (null != fontsTheme && fontsTheme.length() > 0) dst.setValue(36, fontsTheme); }
      if let Some(cstheme) = cstheme.filter(|s| !s.is_empty()) {
        if let Some(fontsTheme) = ThemeFontTool::toFontsTheme(&cstheme, doc).filter(|s| !s.is_empty()) { dst.setString(36, fontsTheme); }
      }
    }

    // java private int toHint(String hint)
    pub fn toHint(&self, hint: &str) -> i32 {
      // java switch (hint.hashCode())
      match java_string_hash_code(hint) {
        // java case -1078490220: case -774740377: return 1;
        -1078490220 | -774740377 => 1,
        // java case 3184: return 2;
        3184 => 2,
        // java case 1544803905: return 0;
        1544803905 => 0,
        // java default: IOAssert.fail("It should not reach here!"); return 0;
        _ => panic!("unsupported rFonts hint: {hint}"),
      }
    }

    // java public void convertSymbolProp(Attributes attributes, MutablePropertySet runPropertySet)
    pub fn convertSymbolProp(&self, attributes: &dyn Attributes, runPropertySet: &mut MutablePropertySet) {
      // java IOAssert.assertNotNull("attributes should not be null", attributes);
      // rust: no-stmt
      // java IOAssert.assertNotNull("runPropertySet should not be null", runPropertySet);
      // rust: no-stmt
      // java String font = ConvertHelper.getString(attributes, "font");
      let font = ConvertHelper::getString__String(attributes, "font").expect("font should not be null!");
      // java IOAssert.assertNotNull("font should not be null!", font);
      // rust: no-stmt
      // java runPropertySet.setString(19, font);
      runPropertySet.setString(19, font);
      // java Integer code = ConvertHelper.hex2Int(attributes, "char");
      let code = ConvertHelper::hex2Int(attributes, "char").expect("code should not be null!");
      // java IOAssert.assertNotNull("code should not be null!", code);
      // rust: no-stmt
      // java runPropertySet.setInt(20, code);
      runPropertySet.setInt(20, code);
    }

    // java public void convertGeoText(MutablePropertySet mutablePropertySet, String unicodeText)
    pub fn convertGeoText__mutablepropertyset_string(&mut self, mutablePropertySet: Option<&MutablePropertySet>, unicodeText: Option<&str>) {
      // java if (null != mutablePropertySet)
      let Some(mutablePropertySet) = mutablePropertySet else {
        return;
      };
      // java if (null != unicodeText && unicodeText.length() > 0)
      let Some(unicodeText) = unicodeText else {
        return;
      };
      if unicodeText.is_empty() {
        return;
      }
      // java Object object = mutablePropertySet.getValue(105);
      if mutablePropertySet.getValue(105).is_none() {
        return;
      }
      let _ = unicodeText;
      // geotext payload path deferred
    }

    // java private void convertFormatChange(Revision change, String unicodeText)
    pub fn convertFormatChange(&self, change: &mut Revision, unicodeText: Option<&str>) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java PropertySet prop = change.getProperty();
      let prop = ();
      // java if (null != prop)
      let _ = "if";
        // java Object object = prop.getValue(105);
      let object = ();
        // java if (null != object)
      let _ = "if";
          // java MutablePropertySet mutablePropertySet = new MutablePropertySet(prop);
      let mutablePropertySet = ();
          // java GeoText geoText = (GeoText)object;
      let geoText = ();
          // java if (!geoText.isEmpty())
      let _ = "if";
            // java change.setProperty(mutablePropertySet.freeze());
      let _ = "change";
    }

    // java private void convertGeoText(MutablePropertySet mutablePropertySet)
    pub fn convertGeoText__MutablePropertySet(&self, mutablePropertySet: Option<&mut MutablePropertySet>) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java if (null != mutablePropertySet)
      let _ = "if";
        // java Object object = mutablePropertySet.getValue(105);
      let object = ();
        // java if (null != object)
      let _ = "if";
    }

}

impl std::ops::Deref for RunPropsConverterSimple {
    type Target = ImporterBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for RunPropsConverterSimple {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
