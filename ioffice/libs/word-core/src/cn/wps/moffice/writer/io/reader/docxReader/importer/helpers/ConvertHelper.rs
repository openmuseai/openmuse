// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.base.io.MetricUtility
use util::base::io::MetricUtility::*;
// Java: import cn.wps.base.io.TypeConversion
use util::base::io::TypeConversion::*;
// Java: import cn.wps.io.dom.io.WMap
use io_wps_io::cn::wps::io::dom::io::WMap::*;
// Java: import cn.wps.io.opt.Attributes1
use io_wps_io::cn::wps::io::opt::Attributes1::*;
// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::KStyle::TextDocument;
// Java: import cn.wps.moffice.writer.data.theme.font.SupplementalFont
use crate::cn::wps::moffice::writer::data::theme::font::SupplementalFont::*;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::*;
// Java: import cn.wps.moffice.writer.data.value.CellSpacing
use crate::cn::wps::moffice::writer::data::value::CellSpacing::*;
// Java: import cn.wps.moffice.writer.data.value.DateTime
use crate::cn::wps::moffice::writer::data::value::DateTime::*;
// Java: import cn.wps.moffice.writer.data.value.DropCap
use crate::cn::wps::moffice::writer::data::value::DropCap::*;
// Java: import cn.wps.moffice.writer.data.value.FontInfo
use crate::cn::wps::moffice::writer::data::value::FontInfo::*;
// Java: import cn.wps.moffice.writer.data.value.PositionCode
use crate::cn::wps::moffice::writer::data::value::PositionCode::*;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import cn.wps.moffice.writer.data.value.WHeightAbs
use crate::cn::wps::moffice::writer::data::value::WHeightAbs::*;
// Java: import cn.wps.moffice.writer.io.reader.helper.IOHelper
use crate::cn::wps::moffice::writer::io::reader::helper::IOHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Utils
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Utils::*;
// Java: import java.util.ArrayList
// Java: import java.util.Date
use stubs::java::util::Date::*;
// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import java.util.Random
use stubs::java::util::Random::*;
// Java: import org.xml.sax.Attributes
use std::any::Any;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ShareConvertHelper::ShareConvertHelper;
/// SAX `Attributes` surface used by ConvertHelper: `getLength` + `getValue(uri, localName)`.
/// Tests adapt `WordAttrs`; production later can impl the same trait on the real SAX type.
pub trait Attributes {
    fn getLength(&self) -> i32;
    fn getValue(&self, uri: &str, localName: &str) -> Option<String>;
}
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.WmlUnit
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::WmlUnit::*;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::BorderConvertSimple::BorderConvertSimple;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ImporterHelper::ImporterHelper;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use std::collections::{HashMap};

pub(crate) fn java_string_hash_code(s: &str) -> i32 {
    s.encode_utf16()
        .fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
}

// java public static final int INVALID = -1;
pub(crate) const INVALID: i32 = -1;
// java private static final int PANOSE_ITEM_SIZE = 2;
const PANOSE_ITEM_SIZE: i32 = 2;
// java public static final String FONT = "font";
pub(crate) const FONT: &str = "font";
// java public static final String CHAR = "char";
pub(crate) const CHAR: &str = "char";
// java public static final String INSTR = "instr";
pub(crate) const INSTR: &str = "instr";
// java public static final String FLDLOCK = "fldLock";
pub(crate) const FLDLOCK: &str = "fldLock";
// java public static final String ID = "id";
pub(crate) const ID: &str = "id";
// java public static final String CUSTOMMARKFOLLOWS = "customMarkFollows";
pub(crate) const CUSTOMMARKFOLLOWS: &str = "customMarkFollows";
// java public static final String H = "h";
pub(crate) const H: &str = "h";
// java public static final String W = "w";
pub(crate) const W: &str = "w";
// java public static final String VAL = "val";
pub(crate) const VAL: &str = "val";
// java public static final String SPACE = "space";
pub(crate) const SPACE: &str = "space";
// java public static final String TYPE = "type";
pub(crate) const TYPE: &str = "type";
// java public static final String COLOR = "color";
pub(crate) const COLOR: &str = "color";
// java private static final String M_NAMESPACING = "http://schemas.openxmlformats.org/officeDocument/2006/math";
const M_NAMESPACING: &str = "http://schemas.openxmlformats.org/officeDocument/2006/math";
// java private static final String HRULE = "hRule";
const HRULE: &str = "hRule";
// java private static final String ATLEAST = "atLeast";
const ATLEAST: &str = "atLeast";
// java private static final String EXACT = "exact";
const EXACT: &str = "exact";
// java private static final String SZ = "sz";
const SZ: &str = "sz";
// java private static final String SHADOW = "shadow";
const SHADOW: &str = "shadow";
// java private static final String FRAME = "frame";
const FRAME: &str = "frame";
// java public static final String AUTHOR = "author";
pub(crate) const AUTHOR: &str = "author";
// java public static final String DATE = "date";
pub(crate) const DATE: &str = "date";
// java private static final String FILL = "fill";
const FILL: &str = "fill";
// java private static final String DATE_XML03 = "createdate";
const DATE_XML03: &str = "createdate";
// java private static String CHARSET = "charset";
static CHARSET: std::sync::LazyLock<Option<String>> =
                    std::sync::LazyLock::new(|| Some(String::from("charset")));
// java private static String PANOSE = "panose";
static PANOSE: std::sync::LazyLock<Option<String>> =
                    std::sync::LazyLock::new(|| Some(String::from("panose")));
// java private static String PITCHFAMILY = "pitchFamily";
static PITCHFAMILY: std::sync::LazyLock<Option<String>> =
                    std::sync::LazyLock::new(|| Some(String::from("pitchFamily")));
// java private static String TYPEFACE = "typeface";
static TYPEFACE: std::sync::LazyLock<Option<String>> =
                    std::sync::LazyLock::new(|| Some(String::from("typeface")));
// java private static String SCRIPT = "script";
static SCRIPT: std::sync::LazyLock<Option<String>> =
                    std::sync::LazyLock::new(|| Some(String::from("script")));
// java private static Random sRandom = null;
static sRandom: std::sync::LazyLock<std::sync::Arc<Option<Random>>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(None));
// java private static Map<String, Integer> sNfcMap = null;
static sNfcMap: std::sync::LazyLock<Option<HashMap<String, i32>>> =
                    std::sync::LazyLock::new(|| None);

// java public class ConvertHelper
#[derive(Default)]
pub struct ConvertHelper {
}

// java public class ConvertHelper
impl ConvertHelper {
    // java public static WHeightAbs toWHeightAbs(Attributes framePr)
    pub fn toWHeightAbs(framePr: &dyn Attributes) -> Option<WHeightAbs> {
      // java IOAssert.assertNotNull("framePr should not be null!", framePr);
      IOAssert::assertNotNull_msg("framePr should not be null!", &Some(framePr));
      // java Long height = getLong(framePr, "h");
      let height = Self::getLong(framePr, "h");
      // java if (null == height)
      if height.is_none() {
        // java return null;
        return None;
      // java else
      } else {
        // java WHeightAbs wHeightAbs = null;
        let mut wHeightAbs: Option<WHeightAbs> = None;
        // java String heightRule = getString(framePr, "hRule");
        let heightRule = Self::getString__String(framePr, "hRule");
        let height_bits = height.unwrap() as i32;
        // java if (heightRule == null)
        if heightRule.is_none() {
          // java wHeightAbs = new WHeightAbs(height.intValue() | '耀');
          wHeightAbs = Some(WHeightAbs::WHeightAbs(height_bits | ('耀' as i32)));
        // java else if (heightRule.equalsIgnoreCase("atLeast"))
        } else if heightRule.as_ref().unwrap().eq_ignore_ascii_case("atLeast") {
          // java wHeightAbs = new WHeightAbs(height.intValue() | '耀');
          wHeightAbs = Some(WHeightAbs::WHeightAbs(height_bits | ('耀' as i32)));
        // java else if (heightRule.equalsIgnoreCase("exact"))
        } else if heightRule.as_ref().unwrap().eq_ignore_ascii_case("exact") {
          // java wHeightAbs = new WHeightAbs(height.intValue());
          wHeightAbs = Some(WHeightAbs::WHeightAbs(height_bits));
        }
        // java return wHeightAbs;
        return wHeightAbs;
      }
    }

    // java public static DropCap toDropCap(Attributes framePr)
    pub fn toDropCap(framePr: &dyn Attributes) -> Option<DropCap> {
      // java IOAssert.assertNotNull("framePr should not be null!", framePr);
      // java DropCap dropCap = null;
      let mut dropCap: Option<DropCap> = None;
      // java String DROPCAP = "dropCap";
      let DROPCAP = "dropCap";
      let _ = DROPCAP;
      // java String dropCapType = getString(framePr, "dropCap");
      let mut dropCapType: Option<String> = Self::getString__String(framePr, "dropCap");
      // java if (dropCapType == null)
      if dropCapType.is_none() {
        // java dropCapType = getString(framePr, "drop-cap");
        dropCapType = Self::getString__String(framePr, "drop-cap");
      }
      // java if (dropCapType != null)
      if dropCapType.is_some() {
        // java dropCap = new DropCap();
        dropCap = Some(DropCap::DropCap());
        // java dropCap.setType(toDropCapType(dropCapType));
        dropCap.as_mut().unwrap().setType(Self::toDropCapType(&dropCapType.clone().unwrap()));
        // java String LINES = "lines";
        let LINES = "lines";
        let _ = LINES;
        // java Long lines = getLong(framePr, "lines");
        let lines: Option<i64> = Self::getLong(framePr, "lines");
        // java if (lines != null)
        if lines.is_some() {
            // java dropCap.setLines(lines.intValue());
            dropCap.as_mut().unwrap().setLines(lines.unwrap() as i32);
        }
      }
      // java return dropCap;
      return dropCap;
    }

    // java private static int toDropCapType(String dropCapType)
    pub fn toDropCapType(dropCapType: &str) -> i32 {
      // java IOAssert.assertNotNull("dropCapType should not be null!", dropCapType);
      // java if (dropCapType.equalsIgnoreCase("none"))
      if dropCapType.eq_ignore_ascii_case("none") {
        // java return 0;
        return 0;
      // java } else if (dropCapType.equalsIgnoreCase("drop"))
      } else if dropCapType.eq_ignore_ascii_case("drop") {
        // java return 1;
        return 1;
      // java } else if (dropCapType.equalsIgnoreCase("margin"))
      } else if dropCapType.eq_ignore_ascii_case("margin") {
        // java return 2;
        return 2;
      } else {
        // java IOAssert.fail("It should not reach here!");
        IOAssert::fail_msg("It should not reach here!");
        // java return 0;
        return 0;
      }
    }

    // java public static SHD toSHD(Attributes attributes)
    pub fn toSHD(attributes: &dyn Attributes) -> SHD {
      // java return SHD.obtain(toColor(attributes, "color"), toColor(attributes, "fill"), toPat(attributes));
      return SHD::obtain(Self::toColor(attributes, "color"), Self::toColor(attributes, "fill"), Self::toPat(attributes));
    }

    // java public static int toColor(Attributes attributes, String name)
    pub fn toColor(attributes: &dyn Attributes, name: &str) -> i32 {
      // java String str = getString(attributes, name);
      let str = Self::getString__String(attributes, name);
      // java return null != str ? convert2coreColor(str) : -16777216;
      return match str {
        Some(s) => Self::convert2coreColor(&s),
        None => -16777216,
      };
    }

    // java public static int getStringHashCode(Attributes attributes)
    pub fn getStringHashCode(attributes: &dyn Attributes) -> i32 {
      // java if (attributes instanceof Attributes1)
      if false {
        // java return getStringHashCode((Attributes1)attributes);
        return Self::getStringHashCode__Attributes1(attributes);
      }
      // java else
      else {
        // java String str = getString(attributes);
        let str = Self::getString(attributes);
        // java return str == null ? -1 : str.hashCode();
        return match str {
            None => -1,
            Some(s) => java_string_hash_code(&s),
        };
      }
    }

    // java public static String getString(Attributes attributes)
    pub fn getString(attributes: &dyn Attributes) -> Option<String> {
      // java return attributes.getLength() == 0 ? null : attributes.getValue(WMap.MAIN_NAMESPACING, "val");
      return if attributes.getLength() == 0 {
        None
      } else {
        attributes.getValue(WMap::MAIN_NAMESPACING(), "val")
      };
    }

    // java private static int getStringHashCode(Attributes1 attributes)
    pub fn getStringHashCode__Attributes1(attributes: &dyn Attributes) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java return attributes.getLength() == 0 ? -1 : attributes.getStringHashCode(WMap.MAIN_NAMESPACING, "val");
    }

    // java public static String getString(Attributes attributes, String name)
    pub fn getString__String(attributes: &dyn Attributes, name: &str) -> Option<String> {
      // java return attributes.getLength() == 0 ? null : attributes.getValue(WMap.MAIN_NAMESPACING, name);
      return if attributes.getLength() == 0 {
        None
      } else {
        attributes.getValue(WMap::MAIN_NAMESPACING(), name)
      };
    }

    // java public static int getStringHashCode(Attributes attributes, String name)
    pub fn getStringHashCode__String(attributes: &dyn Attributes, name: &str) -> i32 {
      // java if (attributes.getLength() == 0)
      if attributes.getLength() == 0 {
        // java return -1;
        return -1;
      }
      // java else if (attributes instanceof Attributes1)
      else if false {
        // java Attributes1 attributes1 = (Attributes1)attributes;
        let _attributes1 = attributes;
        // java return attributes1.getStringHashCode(WMap.MAIN_NAMESPACING, name);
        return Self::getStringHashCode__Attributes1(attributes);
      }
      // java else
      else {
        // java String str = attributes.getValue(WMap.MAIN_NAMESPACING, name);
        let str = attributes.getValue(WMap::MAIN_NAMESPACING(), name);
        // java return null == str ? -1 : str.hashCode();
        return match str {
            None => -1,
            Some(s) => java_string_hash_code(&s),
        };
      }
    }

    // java public static String getString(Attributes attributes, String name, String namespace)
    pub fn getString__String_String(attributes: &dyn Attributes, name: &str, namespace: &str) -> Option<String> {
      // java IOAssert.assertNotNull("attributes should not be null", attributes);
      let attributes_opt: Option<&dyn Attributes> = Some(attributes);
      IOAssert::assertNotNull_msg("attributes should not be null", &attributes_opt);
      // java IOAssert.assertNotNull("name should not be null", name);
      let name_opt: Option<&str> = Some(name);
      IOAssert::assertNotNull_msg("name should not be null", &name_opt);
      // java IOAssert.assertNotNull("namespace should not be null", namespace);
      let namespace_opt: Option<&str> = Some(namespace);
      IOAssert::assertNotNull_msg("namespace should not be null", &namespace_opt);
      // java return attributes.getLength() == 0 ? null : attributes.getValue(namespace, name);
      return if attributes.getLength() == 0 {
        None
      } else {
        attributes.getValue(namespace, name)
      };
    }

    // java private static int convert2coreColor(String srcVal)
    pub fn convert2coreColor(srcVal: &str) -> i32 {
      // java int dstVal = -2;
      let mut dstVal: i32 = -2;
      // java if (srcVal.equalsIgnoreCase("auto"))
      if srcVal.eq_ignore_ascii_case("auto") {
        // java dstVal = -16777216;
        dstVal = -16777216;
      }
      // java else
      else {
        // java Integer tmpVal = TypeConversion.hexString2Integer(srcVal);
        let tmpVal = TypeConversion::hexString2Integer(srcVal);
        // java if (null != tmpVal)
        if tmpVal.is_some() {
          // java dstVal = TypeConversion.hexString2Integer(srcVal);
          dstVal = TypeConversion::hexString2Integer(srcVal).unwrap();
        }
      }
      // java return dstVal;
      return dstVal;
    }

    // java private static int toPat(Attributes attributes)
    pub fn toPat(attributes: &dyn Attributes) -> i32 {
      // java int str = getStringHashCode(attributes);
      let str = Self::getStringHashCode(attributes);
      // java if (-1 == str)
      if str == -1 {
        // java return -2;
        return -2;
      }
      // java else
      else {
        // java int NIL = 109073;
        let NIL: i32 = 109073;
        // java int CLEAR = 94746189;
        let CLEAR: i32 = 94746189;
        // java int SOLID = 109618859;
        let SOLID: i32 = 109618859;
        // java int HORZSTRIPE = -1816487236;
        let HORZSTRIPE: i32 = -1816487236;
        // java int VERTSTRIPE = -408088354;
        let VERTSTRIPE: i32 = -408088354;
        // java int REVERSEDIAGSTRIPE = -940091366;
        let REVERSEDIAGSTRIPE: i32 = -940091366;
        // java int DIAGSTRIPE = 1978653592;
        let DIAGSTRIPE: i32 = 1978653592;
        // java int HORZCROSS = -1458908175;
        let HORZCROSS: i32 = -1458908175;
        // java int DIAGCROSS = 1157367701;
        let DIAGCROSS: i32 = 1157367701;
        // java int THINHORZSTRIPE = 1785207349;
        let THINHORZSTRIPE: i32 = 1785207349;
        // java int THINVERTSTRIPE = -1101361065;
        let THINVERTSTRIPE: i32 = -1101361065;
        // java int THINREVERSEDIAGSTRIPE = 1398484609;
        let THINREVERSEDIAGSTRIPE: i32 = 1398484609;
        // java int THINDIAGSTRIPE = 1285380881;
        let THINDIAGSTRIPE: i32 = 1285380881;
        // java int THINHORZCROSS = 1566769496;
        let THINHORZCROSS: i32 = 1566769496;
        // java int THINDIAGCROSS = -111921924;
        let THINDIAGCROSS: i32 = -111921924;
        // java int HORZSTRIPE_xml03 = 131548747;
        let HORZSTRIPE_xml03: i32 = 131548747;
        // java int VERTSTRIPE_xml03 = 842241129;
        let VERTSTRIPE_xml03: i32 = 842241129;
        // java int REVERSEDIAGSTRIPE_xml03 = -1428995452;
        let REVERSEDIAGSTRIPE_xml03: i32 = -1428995452;
        // java int DIAGSTRIPE_xml03 = 1816797423;
        let DIAGSTRIPE_xml03: i32 = 1816797423;
        // java int HORZCROSS_xml03 = 1374878338;
        let HORZCROSS_xml03: i32 = 1374878338;
        // java int DIAGCROSS_xml03 = 875051870;
        let DIAGCROSS_xml03: i32 = 875051870;
        // java int THINHORZSTRIPE_xml03 = 965839127;
        let THINHORZSTRIPE_xml03: i32 = 965839127;
        // java int THINVERTSTRIPE_xml03 = 1676531509;
        let THINVERTSTRIPE_xml03: i32 = 1676531509;
        // java int THINREVERSEDIAGSTRIPE_xml03 = 1303640400;
        let THINREVERSEDIAGSTRIPE_xml03: i32 = 1303640400;
        // java int THINDIAGSTRIPE_xml03 = -1643879493;
        let THINDIAGSTRIPE_xml03: i32 = -1643879493;
        // java int THINHORZCROSS_xml03 = 431959606;
        let THINHORZCROSS_xml03: i32 = 431959606;
        // java int THINDIAGCROSS_xml03 = -67866862;
        let THINDIAGCROSS_xml03: i32 = -67866862;
        // java int PCT5 = 3435380;
        let PCT5: i32 = 3435380;
        // java int PCT10 = 106496704;
        let PCT10: i32 = 106496704;
        // java int PCT12 = 106496706;
        let PCT12: i32 = 106496706;
        // java int PCT15 = 106496709;
        let PCT15: i32 = 106496709;
        // java int PCT20 = 106496735;
        let PCT20: i32 = 106496735;
        // java int PCT25 = 106496740;
        let PCT25: i32 = 106496740;
        // java int PCT30 = 106496766;
        let PCT30: i32 = 106496766;
        // java int PCT35 = 106496771;
        let PCT35: i32 = 106496771;
        // java int PCT37 = 106496773;
        let PCT37: i32 = 106496773;
        // java int PCT40 = 106496797;
        let PCT40: i32 = 106496797;
        // java int PCT45 = 106496802;
        let PCT45: i32 = 106496802;
        // java int PCT50 = 106496828;
        let PCT50: i32 = 106496828;
        // java int PCT55 = 106496833;
        let PCT55: i32 = 106496833;
        // java int PCT60 = 106496859;
        let PCT60: i32 = 106496859;
        // java int PCT62 = 106496861;
        let PCT62: i32 = 106496861;
        // java int PCT65 = 106496864;
        let PCT65: i32 = 106496864;
        // java int PCT70 = 106496890;
        let PCT70: i32 = 106496890;
        // java int PCT75 = 106496895;
        let PCT75: i32 = 106496895;
        // java int PCT80 = 106496921;
        let PCT80: i32 = 106496921;
        // java int PCT85 = 106496926;
        let PCT85: i32 = 106496926;
        // java int PCT87 = 106496928;
        let PCT87: i32 = 106496928;
        // java int PCT90 = 106496952;
        let PCT90: i32 = 106496952;
        // java int PCT95 = 106496957;
        let PCT95: i32 = 106496957;
        // java int PCT_5 = 106496585;
        let PCT_5: i32 = 106496585;
        // java int PCT_10 = -993573237;
        let PCT_10: i32 = -993573237;
        // java int PCT_12 = -993573235;
        let PCT_12: i32 = -993573235;
        // java int PCT_15 = -993573232;
        let PCT_15: i32 = -993573232;
        // java int PCT_20 = -993573206;
        let PCT_20: i32 = -993573206;
        // java int PCT_25 = -993573201;
        let PCT_25: i32 = -993573201;
        // java int PCT_30 = -993573175;
        let PCT_30: i32 = -993573175;
        // java int PCT_35 = -993573170;
        let PCT_35: i32 = -993573170;
        // java int PCT_37 = -993573168;
        let PCT_37: i32 = -993573168;
        // java int PCT_40 = -993573144;
        let PCT_40: i32 = -993573144;
        // java int PCT_45 = -993573139;
        let PCT_45: i32 = -993573139;
        // java int PCT_50 = -993573113;
        let PCT_50: i32 = -993573113;
        // java int PCT_55 = -993573108;
        let PCT_55: i32 = -993573108;
        // java int PCT_60 = -993573082;
        let PCT_60: i32 = -993573082;
        // java int PCT_62 = -993573080;
        let PCT_62: i32 = -993573080;
        // java int PCT_65 = -993573077;
        let PCT_65: i32 = -993573077;
        // java int PCT_70 = -993573051;
        let PCT_70: i32 = -993573051;
        // java int PCT_75 = -993573046;
        let PCT_75: i32 = -993573046;
        // java int PCT_80 = -993573020;
        let PCT_80: i32 = -993573020;
        // java int PCT_85 = -993573015;
        let PCT_85: i32 = -993573015;
        // java int PCT_87 = -993573013;
        let PCT_87: i32 = -993573013;
        // java int PCT_90 = -993572989;
        let PCT_90: i32 = -993572989;
        // java int PCT_95 = -993572984;
        let PCT_95: i32 = -993572984;
        // java switch (str)
          // java case -1816487236: case 131548747:
            // java return 14;
          // java case -1643879493: case 1285380881:
            // java return 23;
          // java case -1458908175: case 1374878338:
            // java return 18;
          // java case -1428995452: case -940091366:
            // java return 16;
          // java case -1101361065: case 1676531509:
            // java return 21;
          // java case -993573237: case 106496704:
            // java return 3;
          // java case -993573235: case 106496706:
            // java return 37;
          // java case -993573232: case 106496709:
            // java return 38;
          // java case -993573206: case 106496735:
            // java return 4;
          // java case -993573201: case 106496740:
            // java return 5;
          // java case -993573175: case 106496766:
            // java return 6;
          // java case -993573170: case 106496771:
            // java return 43;
          // java case -993573168: case 106496773:
            // java return 44;
          // java case -993573144: case 106496797:
            // java return 7;
          // java case -993573139: case 106496802:
            // java return 46;
          // java case -993573113: case 106496828:
            // java return 8;
          // java case -993573108: case 106496833:
            // java return 49;
          // java case -993573082: case 106496859:
            // java return 9;
          // java case -993573080: case 106496861:
            // java return 51;
          // java case -993573077: case 106496864:
            // java return 52;
          // java case -993573051: case 106496890:
            // java return 10;
          // java case -993573046: case 106496895:
            // java return 11;
          // java case -993573020: case 106496921:
            // java return 12;
          // java case -993573015: case 106496926:
            // java return 57;
          // java case -993573013: case 106496928:
            // java return 58;
          // java case -993572989: case 106496952:
            // java return 13;
          // java case -993572984: case 106496957:
            // java return 60;
          // java case -408088354: case 842241129:
            // java return 15;
          // java case -111921924: case -67866862:
            // java return 25;
          // java case 109073:
            // java return 65535;
          // java case 3435380: case 106496585:
            // java return 2;
          // java case 94746189:
            // java return 0;
          // java case 109618859:
            // java return 1;
          // java case 431959606: case 1566769496:
            // java return 24;
          // java case 875051870: case 1157367701:
            // java return 19;
          // java case 965839127: case 1785207349:
            // java return 20;
          // java case 1303640400: case 1398484609:
            // java return 22;
          // java case 1816797423: case 1978653592:
            // java return 17;
          // java default:
            // java return -2;

        let _ = (NIL, CLEAR, SOLID, HORZSTRIPE, VERTSTRIPE, REVERSEDIAGSTRIPE, DIAGSTRIPE, HORZCROSS, DIAGCROSS, THINHORZSTRIPE, THINVERTSTRIPE, THINREVERSEDIAGSTRIPE, THINDIAGSTRIPE, THINHORZCROSS, THINDIAGCROSS, HORZSTRIPE_xml03, VERTSTRIPE_xml03, REVERSEDIAGSTRIPE_xml03, DIAGSTRIPE_xml03, HORZCROSS_xml03, DIAGCROSS_xml03, THINHORZSTRIPE_xml03, THINVERTSTRIPE_xml03, THINREVERSEDIAGSTRIPE_xml03, THINDIAGSTRIPE_xml03, THINHORZCROSS_xml03, THINDIAGCROSS_xml03, PCT5, PCT10, PCT12, PCT15, PCT20, PCT25, PCT30, PCT35, PCT37, PCT40, PCT45, PCT50, PCT55, PCT60, PCT62, PCT65, PCT70, PCT75, PCT80, PCT85, PCT87, PCT90, PCT95, PCT_5, PCT_10, PCT_12, PCT_15, PCT_20, PCT_25, PCT_30, PCT_35, PCT_37, PCT_40, PCT_45, PCT_50, PCT_55, PCT_60, PCT_62, PCT_65, PCT_70, PCT_75, PCT_80, PCT_85, PCT_87, PCT_90, PCT_95);
        return match str {
          -1816487236 | 131548747 => 14,
          -1643879493 | 1285380881 => 23,
          -1458908175 | 1374878338 => 18,
          -1428995452 | -940091366 => 16,
          -1101361065 | 1676531509 => 21,
          -993573237 | 106496704 => 3,
          -993573235 | 106496706 => 37,
          -993573232 | 106496709 => 38,
          -993573206 | 106496735 => 4,
          -993573201 | 106496740 => 5,
          -993573175 | 106496766 => 6,
          -993573170 | 106496771 => 43,
          -993573168 | 106496773 => 44,
          -993573144 | 106496797 => 7,
          -993573139 | 106496802 => 46,
          -993573113 | 106496828 => 8,
          -993573108 | 106496833 => 49,
          -993573082 | 106496859 => 9,
          -993573080 | 106496861 => 51,
          -993573077 | 106496864 => 52,
          -993573051 | 106496890 => 10,
          -993573046 | 106496895 => 11,
          -993573020 | 106496921 => 12,
          -993573015 | 106496926 => 57,
          -993573013 | 106496928 => 58,
          -993572989 | 106496952 => 13,
          -993572984 | 106496957 => 60,
          -408088354 | 842241129 => 15,
          -111921924 | -67866862 => 25,
          109073 => 65535,
          3435380 | 106496585 => 2,
          94746189 => 0,
          109618859 => 1,
          431959606 | 1566769496 => 24,
          875051870 | 1157367701 => 19,
          965839127 | 1785207349 => 20,
          1303640400 | 1398484609 => 22,
          1816797423 | 1978653592 => 17,
          _ => -2,
        };
      }
    }

    // java public static Long getLong(Attributes attributes, String name)
    pub fn getLong(attributes: &dyn Attributes, name: &str) -> Option<i64> {
      // java IOAssert.assertTrue(!name.contains(":"));
      IOAssert::assertTrue(!name.contains(':'));
      // java String str = attributes.getValue(WMap.MAIN_NAMESPACING, name);
      let str = attributes.getValue(WMap::MAIN_NAMESPACING(), name);
      // java if (null == str)
      if str.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java WmlUnit unit = new WmlUnit(str);
        let unit = WmlUnit::WmlUnit(str.as_ref().unwrap());
        // java if (null == unit.mMagnitude)
        if unit.mMagnitude.is_none() {
          // java return null;
          return None;
        }
        // java else if (null == unit.mUnit)
        else if unit.mUnit.is_none() {
          // java return unit.mMagnitude.longValue();
          return Some(unit.mMagnitude.unwrap() as i64);
        }
        // java else
        else {
          // java switch (unit.mUnit)
          match unit.mUnit {
            // java case PT:
            Some(Unit::PT) => {
              // java return (long)MetricUtility.Points2Twips(unit.mMagnitude);
              return Some(MetricUtility::Points2Twips_f32(unit.mMagnitude.unwrap()) as i64);
            }
            // java default:
            _ => {
              // java return unit.mMagnitude.longValue();
              return Some(unit.mMagnitude.unwrap() as i64);
            }
          }
        }
      }
    }

    // java public static Boolean getBoolean(Attributes attributes, String name)
    pub fn getBoolean(attributes: &dyn Attributes, name: &str) -> Option<bool> {
      // java String str = attributes.getValue(WMap.MAIN_NAMESPACING, name);
      let str = attributes.getValue(WMap::MAIN_NAMESPACING(), name);
      // java return null == str ? null : toBoolean(str);
      return match str {
        None => None,
        Some(s) => Self::toBoolean__String(&s),
      };
    }

    // java private static Boolean toBoolean(String str)
    pub fn toBoolean__String(str: &str) -> Option<bool> {
      // java if (null != str && str.length() > 0)
      if !str.is_empty() {
        // java return isFalse(str) ? Boolean.FALSE : Boolean.TRUE;
        return Some(!Self::isFalse(str));
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java private static Boolean toBoolean(String str, Boolean defaultBoolean)
    pub fn toBoolean__String_Boolean(str: &str, defaultBoolean: Option<bool>) -> Option<bool> {
      // java if (null != str && str.length() > 0)
      if !str.is_empty() {
        // java return isFalse(str) ? Boolean.FALSE : Boolean.TRUE;
        return Some(!Self::isFalse(str));
      }
      // java else
      else {
        // java return defaultBoolean;
        return defaultBoolean;
      }
    }

    // java public static boolean toBoolean(Attributes attributes)
    pub fn toBoolean(attributes: &dyn Attributes) -> bool {
      // java if (attributes instanceof Attributes1)
      if false {
        // java Attributes1 attributes1 = (Attributes1)attributes;
        let _attributes1 = attributes;
        // java return attributes1.getBooleanValue(WMap.MAIN_NAMESPACING, "val", true);
        return true;
      }
      // java else
      else {
        // java String boldStr = attributes.getLength() == 0 ? null : attributes.getValue(WMap.MAIN_NAMESPACING, "val");
        let boldStr = if attributes.getLength() == 0 {
          None
        } else {
          attributes.getValue(WMap::MAIN_NAMESPACING(), "val")
        };
        // java if (null == boldStr)
        if boldStr.is_none() {
          // java return true;
          return true;
        }
        // java else
        else {
          // java Boolean result = toBoolean(boldStr);
          let result = Self::toBoolean__String(boldStr.as_ref().unwrap());
          // java return result == null ? true : result;
          return result.unwrap_or(true);
        }
      }
    }

    // java public static boolean toBoolean4Math(Attributes attributes)
    pub fn toBoolean4Math(attributes: &dyn Attributes) -> bool {
      // java String boldStr = attributes.getLength() == 0 ? null : attributes.getValue("http://schemas.openxmlformats.org/officeDocument/2006/math", "val");
      let boldStr: Option<String> = if attributes.getLength() == 0 {
          None
      } else {
          attributes.getValue("http://schemas.openxmlformats.org/officeDocument/2006/math", "val")
      };
      // java return boldStr == null ? true : toBoolean(boldStr, Boolean.TRUE);
      return match boldStr {
          None => true,
          Some(s) => Self::toBoolean__String_Boolean(&s, Some(true)).unwrap_or(true),
      };
    }

    // java public static Boolean toBoolean4NotNull(Attributes attributes)
    pub fn toBoolean4NotNull(attributes: &dyn Attributes) -> Option<bool> {
      // java return toBoolean4NotNull(attributes, Boolean.TRUE);
      return Self::toBoolean4NotNull__Boolean(attributes, Some(true));
    }

    // java public static Boolean toBoolean4NotNull(Attributes attributes, Boolean defaultBoolean)
    pub fn toBoolean4NotNull__Boolean(attributes: &dyn Attributes, defaultBoolean: Option<bool>) -> Option<bool> {
      // java IOAssert.assertNotNull("attributes should not be null", attributes);
      let attributes_opt: Option<&dyn Attributes> = Some(attributes);
      IOAssert::assertNotNull_msg("attributes should not be null", &attributes_opt);
      // java if (attributes.getLength() == 0)
      if attributes.getLength() == 0 {
        // java return defaultBoolean;
        return defaultBoolean;
      }
      // java else
      else {
        // java String boldStr = attributes.getValue(WMap.MAIN_NAMESPACING, "val");
        let boldStr = attributes.getValue(WMap::MAIN_NAMESPACING(), "val");
        // java if (null == boldStr)
        if boldStr.is_none() {
          // java return defaultBoolean;
          return defaultBoolean;
        }
        // java else
        else {
          // java return isFalse(boldStr) ? Boolean.FALSE : Boolean.TRUE;
          return Some(!Self::isFalse(boldStr.as_deref().unwrap()));
        }
      }
    }

    // java private static boolean isFalse(String str)
    pub fn isFalse(str: &str) -> bool {
      // java IOAssert.assertNotNull("str should not be null", str);
      let str_opt: Option<&str> = Some(str);
      IOAssert::assertNotNull_msg("str should not be null", &str_opt);
      // java return "off".equalsIgnoreCase(str) || "0".equals(str) || "false".equalsIgnoreCase(str) || "f".equalsIgnoreCase(str);
      return str.eq_ignore_ascii_case("off")
        || str == "0"
        || str.eq_ignore_ascii_case("false")
        || str.eq_ignore_ascii_case("f");
    }

    // java public static Integer toInteger(Attributes attributes)
    pub fn toInteger(attributes: &dyn Attributes) -> Option<i32> {
      // java if (attributes instanceof Attributes1)
      if false {
        // java Attributes1 attributes1 = (Attributes1)attributes;
        let _attributes1 = attributes;
        // java return attributes1.getIntValue(WMap.MAIN_NAMESPACING, "val");
        return None;
      }
      // java else
      else {
        // java String str = attributes.getValue(WMap.MAIN_NAMESPACING, "val");
        let str = attributes.getValue(WMap::MAIN_NAMESPACING(), "val");
        // java return str == null ? null : TypeConversion.string2Integer(str);
        return match str {
          None => None,
          Some(s) => TypeConversion::string2Integer(&s),
        };
      }
    }

    // java public static Integer toInteger4Math(Attributes attributes)
    pub fn toInteger4Math(attributes: &dyn Attributes) -> Option<i32> {
      // java return toInteger4Math(attributes, "val");
      return Self::toInteger4Math__String(attributes, "val");
    }

    // java public static Integer toInteger4Math(Attributes attributes, String name)
    pub fn toInteger4Math__String(attributes: &dyn Attributes, name: &str) -> Option<i32> {
      // java String str = attributes.getValue("http://schemas.openxmlformats.org/officeDocument/2006/math", name);
      let str: Option<String> = attributes.getValue("http://schemas.openxmlformats.org/officeDocument/2006/math", name);
      // java return str == null ? null : TypeConversion.string2Integer(str);
      return match str {
          None => None,
          Some(v) => TypeConversion::string2Integer(&v),
      };
    }

    // java public static String toString4Math(Attributes attributes)
    pub fn toString4Math(attributes: &dyn Attributes) -> Option<String> {
      // java return attributes.getLength() == 0 ? null : attributes.getValue("http://schemas.openxmlformats.org/officeDocument/2006/math", "val");
      return if attributes.getLength() == 0 {
        None
      } else {
        attributes.getValue("http://schemas.openxmlformats.org/officeDocument/2006/math", "val")
      };
    }

    // java public static Character toCharacter4Math(Attributes attributes)
    pub fn toCharacter4Math(attributes: &dyn Attributes) -> Option<char> {
      // java String str = attributes.getValue("http://schemas.openxmlformats.org/officeDocument/2006/math", "val");
      let str: Option<String> = attributes.getValue("http://schemas.openxmlformats.org/officeDocument/2006/math", "val");
      // java return str == null ? null : TypeConversion.string2Character(str);
      return str.map(|v| TypeConversion::string2Character(&v));
    }

    // java public static Float getFloat(Attributes attributes, String name)
    pub fn getFloat(attributes: &dyn Attributes, name: &str) -> Option<f32> {
      // java return getFloat(attributes.getValue(WMap.MAIN_NAMESPACING, name));
      return match attributes.getValue(WMap::MAIN_NAMESPACING(), name) {
        None => None,
        Some(s) => Self::getFloat__String(&s),
      };
    }

    // java public static Float getFloat(String str)
    pub fn getFloat__String(str: &str) -> Option<f32> {
      // java if (null == str)
      if false {
        // java return null;
        return None;
      }
      // java else
      else {
        // java WmlUnit unit = new WmlUnit(str);
        let unit = WmlUnit::WmlUnit(str);
        // java if (null == unit.mMagnitude)
        if unit.mMagnitude.is_none() {
          // java return null;
          return None;
        }
        // java else if (null == unit.mUnit)
        else if unit.mUnit.is_none() {
          // java return unit.mMagnitude;
          return unit.mMagnitude;
        }
        // java else
        else {
          // java switch (unit.mUnit)
          match unit.mUnit {
            // java case PT:
            Some(Unit::PT) => {
              // java return MetricUtility.Points2Twips(unit.mMagnitude);
              return Some(MetricUtility::Points2Twips_f32(unit.mMagnitude.unwrap()));
            }
            // java default:
            _ => {
              // java return unit.mMagnitude;
              return unit.mMagnitude;
            }
          }
        }
      }
    }

    // java public static Float toFloat(Attributes attributes)
    pub fn toFloat(attributes: &dyn Attributes) -> Option<f32> {
      // java return getFloat(attributes, "val");
      return Self::getFloat(attributes, "val");
    }

    // java public static Integer getInt(Attributes attributes, String name)
    pub fn getInt(attributes: &dyn Attributes, name: &str) -> Option<i32> {
      // java String str = attributes.getValue(WMap.MAIN_NAMESPACING, name);
      let str = attributes.getValue(WMap::MAIN_NAMESPACING(), name);
      // java if (null == str)
      if str.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java WmlUnit unit = new WmlUnit(str);
        let unit = WmlUnit::WmlUnit(str.as_ref().unwrap());
        // java if (null == unit.mMagnitude)
        if unit.mMagnitude.is_none() {
          // java return null;
          return None;
        }
        // java else if (null == unit.mUnit)
        else if unit.mUnit.is_none() {
          // java return unit.mMagnitude.intValue();
          return Some(unit.mMagnitude.unwrap() as i32);
        }
        // java else
        else {
          // java switch (unit.mUnit)
          match unit.mUnit {
            // java case PT:
            Some(Unit::PT) => {
              // java return (int)MetricUtility.Points2Twips(unit.mMagnitude);
              return Some(MetricUtility::Points2Twips_f32(unit.mMagnitude.unwrap()) as i32);
            }
            // java default:
            _ => {
              // java return unit.mMagnitude.intValue();
              return Some(unit.mMagnitude.unwrap() as i32);
            }
          }
        }
      }
    }

    // java public static Integer hex2Int(Attributes attributes, String name)
    pub fn hex2Int(attributes: &dyn Attributes, name: &str) -> Option<i32> {
      // java String str = attributes.getValue(WMap.MAIN_NAMESPACING, name);
      let str = attributes.getValue(WMap::MAIN_NAMESPACING(), name);
      // java return str == null ? null : TypeConversion.hexString2Integer(str);
      return match str {
        None => None,
        Some(s) => TypeConversion::hexString2Integer(&s),
      };
    }

    // java public static BRC toBRC(Attributes attributes, boolean isXml03)
    pub fn toBRC(attributes: &dyn Attributes, isXml03: bool) -> BRC {
      // java float lineWidth = ImporterHelper.getLineWidthBRC(getLong(attributes, "sz"));
      let lineWidth = ImporterHelper::getLineWidthBRC(Self::getLong(attributes, "sz"));
      // java int type = getBorderType(getString(attributes, "val"), isXml03);
      let r#type = Self::getBorderType(Self::getString__String(attributes, "val").as_deref(), isXml03);
      // java int color = toColor(attributes, "color");
      let color = Self::toColor(attributes, "color");
      // java float space = ImporterHelper.getSpace(getLong(attributes, "space"));
      let space = ImporterHelper::getSpace(Self::getLong(attributes, "space"));
      // java boolean isShadow = ImporterHelper.getBorderShadow(getBoolean(attributes, "shadow"));
      let isShadow = ImporterHelper::getBorderShadow(Self::getBoolean(attributes, "shadow"));
      // java boolean isFrame = ImporterHelper.getBorderFrame(getBoolean(attributes, "frame"));
      let isFrame = ImporterHelper::getBorderFrame(Self::getBoolean(attributes, "frame"));
      // java return BRC.obtain(lineWidth, type, color, space, isShadow, isFrame);
      return BRC::obtain(lineWidth, r#type, color, space, isShadow, isFrame);
    }

    // java private static int getBorderType(String borderStyle, boolean isXml03)
    pub fn getBorderType(borderStyle: Option<&str>, isXml03: bool) -> i32 {
      // java if (null != borderStyle)
      if let Some(borderStyle) = borderStyle {
        // java if (isXml03)
        if isXml03 {
          // java borderStyle = Xml03Utils.replaceHyphenMinusToUpperCase(borderStyle);
          let _ = borderStyle;
        }
        // java return BorderConvertSimple.convertCoreBorder(borderStyle.hashCode());
        return BorderConvertSimple::convertCoreBorder(java_string_hash_code(borderStyle));
      }
      // java else
      else {
        // java int DEFAULT_BORDER_TYPE = 0;
        let DEFAULT_BORDER_TYPE: i32 = 0;
        let _ = DEFAULT_BORDER_TYPE;
        // java return 0;
        return 0;
      }
    }

    // java public static int convertPositionY(Long absolutePosY, String relativePosY)
    pub fn convertPositionY(absolutePosY: Option<i64>, relativePosY: &str) -> i32 {
      // java if (null != absolutePosY)
      if absolutePosY.is_some() {
        // java return absolutePosY.intValue();
        return absolutePosY.unwrap() as i32;
      // java else
      } else {
        // java if (null != relativePosY)
        if !relativePosY.is_empty() {
            // java if (relativePosY.equalsIgnoreCase("inline"))
            if relativePosY.eq_ignore_ascii_case("inline") {
                // java return 0;
                return 0;
            }
            // java if (relativePosY.equalsIgnoreCase("top"))
            if relativePosY.eq_ignore_ascii_case("top") {
                // java return -4;
                return -4;
            }
            // java if (relativePosY.equalsIgnoreCase("center"))
            if relativePosY.eq_ignore_ascii_case("center") {
                // java return -8;
                return -8;
            }
            // java if (relativePosY.equalsIgnoreCase("bottom"))
            if relativePosY.eq_ignore_ascii_case("bottom") {
                // java return -12;
                return -12;
            }
            // java if (relativePosY.equalsIgnoreCase("inside"))
            if relativePosY.eq_ignore_ascii_case("inside") {
                // java return -16;
                return -16;
            }
            // java if (relativePosY.equalsIgnoreCase("outside"))
            if relativePosY.eq_ignore_ascii_case("outside") {
                // java return -20;
                return -20;
            }
            // java IOAssert.fail("It should not reach here!");
            IOAssert::fail_msg("It should not reach here!");
        }
        // java return 0;
        return 0;
      }
    }

    // java public static int convertPositionX(Long absolutePosX, String relativePosX)
    pub fn convertPositionX(absolutePosX: Option<i64>, relativePosX: &str) -> i32 {
      // java if (null != absolutePosX)
      if absolutePosX.is_some() {
        // java return absolutePosX.intValue();
        return absolutePosX.unwrap() as i32;
      // java else
      } else {
        // java if (null != relativePosX)
        if !relativePosX.is_empty() {
            // java if (relativePosX.equalsIgnoreCase("left"))
            if relativePosX.eq_ignore_ascii_case("left") {
                // java return 0;
                return 0;
            }
            // java if (relativePosX.equalsIgnoreCase("center"))
            if relativePosX.eq_ignore_ascii_case("center") {
                // java return -4;
                return -4;
            }
            // java if (relativePosX.equalsIgnoreCase("right"))
            if relativePosX.eq_ignore_ascii_case("right") {
                // java return -8;
                return -8;
            }
            // java if (relativePosX.equalsIgnoreCase("inside"))
            if relativePosX.eq_ignore_ascii_case("inside") {
                // java return -12;
                return -12;
            }
            // java if (relativePosX.equalsIgnoreCase("outside"))
            if relativePosX.eq_ignore_ascii_case("outside") {
                // java return -16;
                return -16;
            }
            // java IOAssert.fail("It should not reach here!");
            IOAssert::fail_msg("It should not reach here!");
        }
        // java return 0;
        return 0;
      }
    }

    // java public static CellSpacing toCellSpacing(Attributes attributes)
    pub fn toCellSpacing(attributes: &dyn Attributes) -> CellSpacing {
      // java IOAssert.assertNotNull("attributes is not null!", attributes);
      IOAssert::assertNotNull_msg("attributes is not null!", &Some(attributes));
      // java int unit = getStringHashCode(attributes, "type");
      let unit = Self::getStringHashCode__String(attributes, "type");
      // java int type = toTableLengthType(unit);
      let r#type = Self::toTableLengthType(unit);
      // java IOAssert.assertTrue("(TableLengthType.vetNil <= type && TableLengthType.vetDxa <= type) should not be false!", 0 <= type && type <= 3);
      IOAssert::assertTrue_msg(
        "(TableLengthType.vetNil <= type && TableLengthType.vetDxa <= type) should not be false!",
        0 <= r#type && r#type <= 3,
      );
      // java float DEFAULT_W = 0.0F;
      let DEFAULT_W: f32 = 0.0;
      let _ = DEFAULT_W;
      // java float value = 0.0F;
      let mut value: f32 = 0.0;
      // java if (null != attributes && 1 != type && 2 != type)
      if 1 != r#type && 2 != r#type {
        // java Float getValue = ShareConvertHelper.getFloatSTMeasurementOrPercent(attributes, "w");
        let getValue = ShareConvertHelper::getFloatSTMeasurementOrPercent(attributes, "w");
        // java value = null == getValue ? 0.0F : getValue;
        value = getValue.unwrap_or(0.0);
      }
      // java return CellSpacing.obtain(0, type, (int)value);
      return CellSpacing::obtain(0, r#type, value as i32);
    }

    // java static int toTableLengthType(int unit)
    pub fn toTableLengthType(unit: i32) -> i32 {
      // java if (-1 == unit)
      if -1 == unit {
        // java return 3;
        return 3;
      // java else
      } else {
        // java int NIL = 109073;
        let NIL: i32 = 109073; let _ = NIL;
        // java int AUTO = 3005871;
        let AUTO: i32 = 3005871; let _ = AUTO;
        // java int PCT = 110817;
        let PCT: i32 = 110817; let _ = PCT;
        // java int DXA = 99917;
        let DXA: i32 = 99917; let _ = DXA;
        // java switch (unit)
        return match unit {
            // java case 99917:
            // java return 3;
            99917 => 3,
            // java case 109073:
            // java return 0;
            109073 => 0,
            // java case 110817:
            // java return 2;
            110817 => 2,
            // java case 3005871:
            // java return 1;
            3005871 => 1,
            // java default:
            _ => {
                // java IOAssert.fail("It should not reach here!");
                IOAssert::fail_msg("It should not reach here!");
                // java return 3;
                3
            }
        };
      }
    }

    // java public static PositionCode convertHorzAnchorAndVertAnchor(String hAnchor, String vAnchor)
    pub fn convertHorzAnchorAndVertAnchor(hAnchor: Option<&str>, vAnchor: Option<&str>) -> PositionCode {
      // java IOAssert.assertTrue("(hAnchor != null || vAnchor != null) must be true!", hAnchor != null || vAnchor != null);
      IOAssert::assertTrue_msg("(hAnchor != null || vAnchor != null) must be true!", hAnchor.is_some() || vAnchor.is_some());
      // java int pcVert = 3;
      let mut pcVert = 3;
      // java int pcHorz = 3;
      let mut pcHorz = 3;
      // java if (hAnchor != null)
      if let Some(hAnchor) = hAnchor {
        // java if (hAnchor.equalsIgnoreCase("text"))
        if hAnchor.eq_ignore_ascii_case("text") {
          // java pcHorz = 0;
          pcHorz = 0;
        // java else if (hAnchor.equalsIgnoreCase("margin"))
        } else if hAnchor.eq_ignore_ascii_case("margin") {
          // java pcHorz = 1;
          pcHorz = 1;
        // java else if (hAnchor.equalsIgnoreCase("page"))
        } else if hAnchor.eq_ignore_ascii_case("page") {
          // java pcHorz = 2;
          pcHorz = 2;
        // java else
        } else {
          // java IOAssert.fail("It should not reach here!");
          IOAssert::fail_msg("It should not reach here!");
          // java pcHorz = 2;
          pcHorz = 2;
        }
      }
      // java if (vAnchor != null)
      if let Some(vAnchor) = vAnchor {
        // java if (vAnchor.equalsIgnoreCase("text"))
        if vAnchor.eq_ignore_ascii_case("text") {
          // java pcVert = 2;
          pcVert = 2;
        // java else if (vAnchor.equalsIgnoreCase("margin"))
        } else if vAnchor.eq_ignore_ascii_case("margin") {
          // java pcVert = 0;
          pcVert = 0;
        // java else if (vAnchor.equalsIgnoreCase("page"))
        } else if vAnchor.eq_ignore_ascii_case("page") {
          // java pcVert = 1;
          pcVert = 1;
        // java else
        } else {
          // java IOAssert.fail("It should not reach here!");
          IOAssert::fail_msg("It should not reach here!");
          // java pcVert = 0;
          pcVert = 0;
        }
      }
      // java return PositionCode.obtain(pcVert, pcHorz);
      PositionCode::obtain(pcVert, pcHorz)
    }

    // java public static int toTextFlow(String textDirection, boolean isXml03)
    pub fn toTextFlow(textDirection: &str, isXml03: bool) -> i32 {
      let replaced;
      // java if (isXml03)
      let textDirection = if isXml03 {
        // java textDirection = Xml03Utils.replaceHyphenMinusToUpperCase(textDirection);
        replaced = Xml03Utils::replaceHyphenMinusToUpperCase(Some(textDirection)).unwrap_or_default();
        replaced.as_str()
      } else {
        textDirection
      };
      // java if (null != textDirection && textDirection.length() != 0)
      if !textDirection.is_empty() {
        // java if (textDirection.equalsIgnoreCase("tb"))
        if textDirection.eq_ignore_ascii_case("tb") {
          // java return 0;
          return 0;
        }
        // java else if (textDirection.equalsIgnoreCase("rl"))
        else if textDirection.eq_ignore_ascii_case("rl") {
          // java return 1;
          return 1;
        }
        // java else if (textDirection.equalsIgnoreCase("lr"))
        else if textDirection.eq_ignore_ascii_case("lr") {
          // java return 3;
          return 3;
        }
        // java else if (textDirection.equalsIgnoreCase("tbV"))
        else if textDirection.eq_ignore_ascii_case("tbV") {
          // java return 4;
          return 4;
        }
        // java else if (textDirection.equalsIgnoreCase("rlV"))
        else if textDirection.eq_ignore_ascii_case("rlV") {
          // java return 5;
          return 5;
        }
        // java else if (textDirection.equalsIgnoreCase("lrV"))
        else if textDirection.eq_ignore_ascii_case("lrV") {
          // java return 0;
          return 0;
        }
        // java else if (textDirection.equalsIgnoreCase("btLr"))
        else if textDirection.eq_ignore_ascii_case("btLr") {
          // java return 3;
          return 3;
        }
        // java else if (textDirection.equalsIgnoreCase("lrTb"))
        else if textDirection.eq_ignore_ascii_case("lrTb") {
          // java return 0;
          return 0;
        }
        // java else if (textDirection.equalsIgnoreCase("lrTbV"))
        else if textDirection.eq_ignore_ascii_case("lrTbV") {
          // java return 4;
          return 4;
        }
        // java else if (textDirection.equalsIgnoreCase("tbLrV"))
        else if textDirection.eq_ignore_ascii_case("tbLrV") {
          // java return 0;
          return 0;
        }
        // java else if (textDirection.equalsIgnoreCase("tbRl"))
        else if textDirection.eq_ignore_ascii_case("tbRl") {
          // java return 1;
          return 1;
        }
        // java else if (textDirection.equalsIgnoreCase("tbRlV"))
        else if textDirection.eq_ignore_ascii_case("tbRlV") {
          // java return 5;
          return 5;
        }
        // java else
        else {
          // java IOAssert.fail("It should not reach here!");
          IOAssert::fail__string("It should not reach here!");
          // java return 0;
          return 0;
        }
      }
      // java else
      // java return 0;
      return 0;
    }

    // java public static Revision convertRPrChange(MutablePropertySet dst, Attributes trackChange)
    pub fn convertRPrChange(dst: &mut MutablePropertySet, trackChange: &dyn Attributes) {
      todo!("DEFERRED_NON_FUNCTION");
      // java IOAssert.assertNotNull("dst should not be null", dst);
      // java IOAssert.assertNotNull("trackChange should not be null", trackChange);
      // java Revision revision = new Revision(3);
      let revision = ();
      // java setRevision(revision, trackChange);
      // java dst.setBoolean(47, true);
      // java dst.setValue(48, revision);
      // java return revision;
    }

    // java public static Revision convertIns(TextDocument doc, MutablePropertySet dst, Attributes trackChange)
    pub fn convertIns(doc: &mut TextDocument, dst: &mut MutablePropertySet, trackChange: &dyn Attributes) {
      todo!("DEFERRED_NON_FUNCTION");
      // java IOAssert.assertNotNull("dst should not be null", dst);
      // java IOAssert.assertNotNull("trackChange should not be null", trackChange);
      // java Revision revision = convertInsRevision(trackChange);
      let revision = ();
      // java addInsRevision(doc, dst, revision);
      // java return revision;
    }

    // java public static Revision convertInsRevision(Attributes attributes)
    pub fn convertInsRevision(attributes: &dyn Attributes) -> Revision {
      // java IOAssert.assertNotNull("trackChange should not be null", attributes);
      IOAssert::assertNotNull_msg("trackChange should not be null", &Some(attributes));
      // java Revision revision = new Revision(2);
      let mut revision = Revision::Revision__int(Revision::INSERT);
      // java setRevision(revision, attributes);
      Self::setRevision(&mut revision, attributes);
      // java return revision;
      return revision;
    }

    // java public static void addInsRevision(TextDocument doc, MutablePropertySet dst, Revision revision)
    pub fn addInsRevision(doc: &mut TextDocument, dst: &mut MutablePropertySet, revision: Revision) {
      todo!("DEFERRED_NON_FUNCTION");
      // java boolean isAllowAddInsert = isAllowAddInsRevision(dst, revision);
      let isAllowAddInsert = ();
      // java if (isAllowAddInsert)
        // java dst.setValue(46, revision);
        // java dst.setInt(50, addRsidToTextDocument(doc));
        // java dst.setBoolean(55, true);
    }

    // java private static boolean isAllowAddInsRevision(MutablePropertySet dst, Revision insRevision)
    pub fn isAllowAddInsRevision(dst: &MutablePropertySet, insRevision: Revision) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java Object deleteObj = dst.getValue(45);
      // java if (null != deleteObj)
        // java Revision deleteRevision = (Revision)deleteObj;
        let deleteRevision = ();
        // java DateTime delTime = deleteRevision.getDateTime();
        // java DateTime insTime = insRevision.getDateTime();
        // java if (insTime.isBefore(delTime))
          // java return false;
        // java dst.setValue(45, (Object)null);
        // java dst.setInt(49, 0);
        // java dst.setBoolean(54, false);
      // java return true;
    }

    // java public static Revision convertDel(TextDocument doc, MutablePropertySet dst, Attributes trackChange)
    pub fn convertDel(doc: &mut TextDocument, dst: &mut MutablePropertySet, trackChange: &dyn Attributes) {
      todo!("DEFERRED_NON_FUNCTION");
      // java IOAssert.assertNotNull("dst should not be null", dst);
      // java IOAssert.assertNotNull("trackChange should not be null", trackChange);
      // java Revision revision = convertDelRevision(trackChange);
      let revision = ();
      // java addDelRevision(doc, dst, revision);
      // java return revision;
    }

    // java public static Revision convertDelRevision(Attributes attributes)
    pub fn convertDelRevision(attributes: &dyn Attributes) -> Revision {
      // java IOAssert.assertNotNull("trackChange should not be null", attributes);
      IOAssert::assertNotNull_msg("trackChange should not be null", &Some(attributes));
      // java Revision revision = new Revision(1);
      let mut revision = Revision::Revision__int(Revision::DELETE);
      // java setRevision(revision, attributes);
      Self::setRevision(&mut revision, attributes);
      // java return revision;
      return revision;
    }

    // rust: ★原 `convertDelRevision` 的挂起体（保留原文注释，见上）
    #[allow(dead_code)]
    fn convertDelRevision_deferred(attributes: &dyn Attributes) {
      todo!("DEFERRED_NON_FUNCTION");
      // java IOAssert.assertNotNull("trackChange should not be null", attributes);
      // java Revision revision = new Revision(1);
      let revision = ();
      // java setRevision(revision, attributes);
      // java return revision;
    }

    // java public static void addDelRevision(TextDocument doc, MutablePropertySet dst, Revision revision)
    pub fn addDelRevision(doc: &mut TextDocument, dst: &mut MutablePropertySet, revision: Revision) {
      todo!("DEFERRED_NON_FUNCTION");
      // java boolean isAllowAddDel = isAllowAddDelRevision(dst, revision);
      let isAllowAddDel = ();
      // java if (isAllowAddDel)
        // java dst.setValue(45, revision);
        // java dst.setInt(49, addRsidToTextDocument(doc));
        // java dst.setBoolean(54, true);
    }

    // java private static boolean isAllowAddDelRevision(MutablePropertySet dst, Revision delRevision)
    pub fn isAllowAddDelRevision(dst: &MutablePropertySet, delRevision: Revision) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java Object insertObj = dst.getValue(46);
      // java if (null != insertObj)
        // java Revision insertRevision = (Revision)insertObj;
        let insertRevision = ();
        // java DateTime insTime = insertRevision.getDateTime();
        // java DateTime delTime = delRevision.getDateTime();
        // java if (delTime.isBefore(insTime))
          // java return false;
        // java dst.setValue(46, (Object)null);
        // java dst.setInt(50, 0);
        // java dst.setBoolean(55, false);
      // java return true;
    }

    // java public static Revision convertTableRevision(MutablePropertySet dst, Attributes trackChange)
    pub fn convertTableRevision(dst: &mut MutablePropertySet, trackChange: &dyn Attributes) -> Revision {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      IOAssert::assertNotNull_msg("dst should not be null", &Some(&*dst));
      // java IOAssert.assertNotNull("trackChange should not be null", trackChange);
      IOAssert::assertNotNull_msg("trackChange should not be null", &Some(trackChange));
      // java Revision revision = new Revision(3);
      let mut revision = Revision::Revision__int(3);
      // java setRevision(revision, trackChange);
      Self::setRevision(&mut revision, trackChange);
      // java dst.setValue(363, revision);
      dst.setValue(363, Some(std::sync::Arc::new(revision.clone())));
      // java dst.setBoolean(364, true);
      dst.setBoolean(364, true);
      // java return revision;
      return revision;
    }

    // java public static void setRevision(Revision revision, Attributes trackChange)
    pub fn setRevision(revision: &mut Revision, trackChange: &dyn Attributes) {
      // java IOAssert.assertNotNull("revision should not be null", revision);
      IOAssert::assertNotNull_msg("revision should not be null", &Some(&*revision));
      // java IOAssert.assertNotNull("trackChange should not be null", trackChange);
      IOAssert::assertNotNull_msg("trackChange should not be null", &Some(trackChange));
      // java String author = getString(trackChange, "author");
      let author = Self::getString__String(trackChange, "author");
      // java IOAssert.logNotNull("author should not be null", author);
      IOAssert::logNotNull_msg("author should not be null", &author);
      // java if (null != author && 0 != author.length())
      if author.as_ref().map(|a| !a.is_empty()).unwrap_or(false) {
        // java revision.setAuthor(author);
        revision.setAuthor(author);
      }

      // java Date date = getDate(trackChange);
      // java IOAssert.logNotNull("date should not be null", date);
      // java if (null != date)
        // java dateTime = ImporterHelper.date2DateTime(date);
      // rust: ★date 支路诚实挂起（**非管线核心**）：需要 `java.util.Date` +
      //   `IOHelper.convertDate` + `ImporterHelper.date2DateTime` 三个未装配依赖；
      //   语料 C1/C2a/C2b/C6′ 均无 `w:date` 修订属性 ⇒ 不影响渲染。
      //   ★此处**不可**放 `todo!()`：`convertIns/DelRevision` 会走到这里（见 §65.2 教训）。
      //   ★author 支路已真实生效，`revision.setAuthor(author)` 不再丢失作者信息。
      let _date: Option<()> = Self::getDate_deferred(trackChange);
    }

    // rust: ★`getDate` 的挂起实现（Date 闭包未装配）。
    #[allow(dead_code)]
    fn getDate_deferred(_attributes: &dyn Attributes) -> Option<()> {
      // Date closure stays off the render path: java.util.Date + IOHelper.convertDate
      // are not assembled. Returning None keeps author revision wiring live.
      None
    }

    // java public static Date getDate(Attributes attributes)
    pub fn getDate(attributes: &dyn Attributes) {
      todo!("DEFERRED_NON_FUNCTION");
      // java String str = getString(attributes, "date");
      let str = ();
      // java if (str == null || str.length() == 0)
        // java str = getString(attributes, "createdate");
        let str = ();
      // java return null != str && str.length() != 0 ? IOHelper.convertDate(str) : null;
    }

    // java public static int addRsidToTextDocument(TextDocument doc)
    pub fn addRsidToTextDocument(doc: &mut TextDocument) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java IOAssert.assertNotNull("doc should not be null", doc);
      // java IOAssert.assertNotNull("doc should not be null", doc);
      // java ArrayList<Integer> rsids = doc.getRsids();
      // java IOAssert.assertNotNull("rsids should not be null", rsids);
      // java int rid = getRandom().nextInt();
      // java rsids.add(rid);
      // java return rid;
    }

    // java private static Random getRandom()
    pub fn getRandom() {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (null == sRandom)
        // java sRandom = new Random();
          let _sRandom = ();
      // java return sRandom;
    }

    // java public static synchronized Integer xmlNfc2MsoNfc(String xmlNfc, boolean isXml03)
    pub fn xmlNfc2MsoNfc(xmlNfc: &str, isXml03: bool) -> Option<i32> {
      // java if (isXml03) xmlNfc = Xml03Utils.replaceHyphenMinusToUpperCase(xmlNfc);
      let normalized = if isXml03 {
        Xml03Utils::replaceHyphenMinusToUpperCase(Some(xmlNfc)).unwrap_or_default()
      } else { xmlNfc.to_string() };
      // java return (Integer)sNfcMap.get(xmlNfc); unknown names return null.
      let value = match normalized.as_str() {
        "decimal" => 0, "upperRoman" => 1, "lowerRoman" => 2,
        "upperLetter" => 3, "lowerLetter" => 4, "ordinal" => 5,
        "cardinalText" => 6, "ordinalText" => 7, "hex" => 8,
        "chicago" => 9, "ideographDigital" => 10, "japaneseCounting" => 11,
        "aiueo" => 12, "iroha" => 13, "decimalFullWidth" => 14,
        "decimalHalfWidth" => 15, "japaneseLegal" => 16,
        "japaneseDigitalTenThousand" => 17, "decimalEnclosedCircle" => 18,
        "decimalFullWidth2" => 19, "aiueoFullWidth" => 20, "irohaFullWidth" => 21,
        "decimalZero" => 22, "bullet" => 23, "ganada" => 24,
        "chosung" => 25, "decimalEnclosedFullstop" => 26,
        "decimalEnclosedParen" => 27, "decimalEnclosedCircleChinese" => 28,
        "ideographEnclosedCircle" => 29, "ideographTraditional" => 30,
        "ideographZodiac" => 31, "ideographZodiacTraditional" => 32,
        "taiwaneseCounting" => 33, "ideographLegalTraditional" => 34,
        "taiwaneseCountingThousand" => 35, "taiwaneseDigital" => 36,
        "chineseCounting" => 37, "chineseLegalSimplified" => 38,
        "chineseCountingThousand" => 39, "koreanDigital" => 41,
        "koreanCounting" => 42, "koreanLegal" => 43, "koreanDigital2" => 44,
        "hebrew1" => 45, "arabicAlpha" => 46, "hebrew2" => 47,
        "arabicAbjad" => 48, "hindiVowels" => 49, "hindiConsonants" => 50,
        "hindiNumbers" => 51, "hindiCounting" => 52, "thaiLetters" => 53,
        "thaiNumbers" => 54, "thaiCounting" => 55, "vietnameseCounting" => 56,
        "numberInDash" => 57, "russianLower" => 58, "russianUpper" => 59,
        "none" | "bahtText" | "dollarText" | "custom" => 255,
        _ => return None,
      };
      return Some(value);
      // java if (null != xmlNfc && xmlNfc.length() > 0)
        // java if (isXml03)
          // java xmlNfc = Xml03Utils.replaceHyphenMinusToUpperCase(xmlNfc);
        // java String DECIMAL = "decimal";
        let DECIMAL = ();
        // java String UPPERROMAN = "upperRoman";
        let UPPERROMAN = ();
        // java String LOWERROMAN = "lowerRoman";
        let LOWERROMAN = ();
        // java String UPPERLETTER = "upperLetter";
        let UPPERLETTER = ();
        // java String LOWERLETTER = "lowerLetter";
        let LOWERLETTER = ();
        // java String ORDINAL = "ordinal";
        let ORDINAL = ();
        // java String CARDINALTEXT = "cardinalText";
        let CARDINALTEXT = ();
        // java String ORDINALTEXT = "ordinalText";
        let ORDINALTEXT = ();
        // java String HEX = "hex";
        let HEX = ();
        // java String CHICAGO = "chicago";
        let CHICAGO = ();
        // java String IDEOGRAPHDIGITAL = "ideographDigital";
        let IDEOGRAPHDIGITAL = ();
        // java String JAPANESECOUNTING = "japaneseCounting";
        let JAPANESECOUNTING = ();
        // java String AIUEO = "aiueo";
        let AIUEO = ();
        // java String IROHA = "iroha";
        let IROHA = ();
        // java String DECIMALFULLWIDTH = "decimalFullWidth";
        let DECIMALFULLWIDTH = ();
        // java String DECIMALHALFWIDTH = "decimalHalfWidth";
        let DECIMALHALFWIDTH = ();
        // java String JAPANESELEGAL = "japaneseLegal";
        let JAPANESELEGAL = ();
        // java String JAPANESEDIGITALTENTHOUSAND = "japaneseDigitalTenThousand";
        let JAPANESEDIGITALTENTHOUSAND = ();
        // java String DECIMALENCLOSEDCIRCLE = "decimalEnclosedCircle";
        let DECIMALENCLOSEDCIRCLE = ();
        // java String DECIMALFULLWIDTH2 = "decimalFullWidth2";
        let DECIMALFULLWIDTH2 = ();
        // java String AIUEOFULLWIDTH = "aiueoFullWidth";
        let AIUEOFULLWIDTH = ();
        // java String IROHAFULLWIDTH = "irohaFullWidth";
        let IROHAFULLWIDTH = ();
        // java String DECIMALZERO = "decimalZero";
        let DECIMALZERO = ();
        // java String BULLET = "bullet";
        let BULLET = ();
        // java String GANADA = "ganada";
        let GANADA = ();
        // java String CHOSUNG = "chosung";
        let CHOSUNG = ();
        // java String DECIMALENCLOSEDFULLSTOP = "decimalEnclosedFullstop";
        let DECIMALENCLOSEDFULLSTOP = ();
        // java String DECIMALENCLOSEDPAREN = "decimalEnclosedParen";
        let DECIMALENCLOSEDPAREN = ();
        // java String DECIMALENCLOSEDCIRCLECHINESE = "decimalEnclosedCircleChinese";
        let DECIMALENCLOSEDCIRCLECHINESE = ();
        // java String IDEOGRAPHENCLOSEDCIRCLE = "ideographEnclosedCircle";
        let IDEOGRAPHENCLOSEDCIRCLE = ();
        // java String IDEOGRAPHTRADITIONAL = "ideographTraditional";
        let IDEOGRAPHTRADITIONAL = ();
        // java String IDEOGRAPHZODIAC = "ideographZodiac";
        let IDEOGRAPHZODIAC = ();
        // java String IDEOGRAPHZODIACTRADITIONAL = "ideographZodiacTraditional";
        let IDEOGRAPHZODIACTRADITIONAL = ();
        // java String TAIWANESECOUNTING = "taiwaneseCounting";
        let TAIWANESECOUNTING = ();
        // java String IDEOGRAPHLEGALTRADITIONAL = "ideographLegalTraditional";
        let IDEOGRAPHLEGALTRADITIONAL = ();
        // java String TAIWANESECOUNTINGTHOUSAND = "taiwaneseCountingThousand";
        let TAIWANESECOUNTINGTHOUSAND = ();
        // java String TAIWANESEDIGITAL = "taiwaneseDigital";
        let TAIWANESEDIGITAL = ();
        // java String CHINESECOUNTING = "chineseCounting";
        let CHINESECOUNTING = ();
        // java String CHINESELEGALSIMPLIFIED = "chineseLegalSimplified";
        let CHINESELEGALSIMPLIFIED = ();
        // java String CHINESECOUNTINGTHOUSAND = "chineseCountingThousand";
        let CHINESECOUNTINGTHOUSAND = ();
        // java String KOREANDIGITAL = "koreanDigital";
        let KOREANDIGITAL = ();
        // java String KOREANCOUNTING = "koreanCounting";
        let KOREANCOUNTING = ();
        // java String KOREANLEGAL = "koreanLegal";
        let KOREANLEGAL = ();
        // java String KOREANDIGITAL2 = "koreanDigital2";
        let KOREANDIGITAL2 = ();
        // java String VIETNAMESECOUNTING = "vietnameseCounting";
        let VIETNAMESECOUNTING = ();
        // java String RUSSIANLOWER = "russianLower";
        let RUSSIANLOWER = ();
        // java String RUSSIANUPPER = "russianUpper";
        let RUSSIANUPPER = ();
        // java String NONE = "none";
        let NONE = ();
        // java String NUMBERINDASH = "numberInDash";
        let NUMBERINDASH = ();
        // java String HEBREW1 = "hebrew1";
        let HEBREW1 = ();
        // java String HEBREW2 = "hebrew2";
        let HEBREW2 = ();
        // java String ARABICALPHA = "arabicAlpha";
        let ARABICALPHA = ();
        // java String ARABICABJAD = "arabicAbjad";
        let ARABICABJAD = ();
        // java String HINDIVOWELS = "hindiVowels";
        let HINDIVOWELS = ();
        // java String HINDICONSONANTS = "hindiConsonants";
        let HINDICONSONANTS = ();
        // java String HINDINUMBERS = "hindiNumbers";
        let HINDINUMBERS = ();
        // java String HINDICOUNTING = "hindiCounting";
        let HINDICOUNTING = ();
        // java String THAILETTERS = "thaiLetters";
        let THAILETTERS = ();
        // java String THAINUMBERS = "thaiNumbers";
        let THAINUMBERS = ();
        // java String THAICOUNTING = "thaiCounting";
        let THAICOUNTING = ();
        // java String BAHTTEXT = "bahtText";
        let BAHTTEXT = ();
        // java String DOLLARTEXT = "dollarText";
        let DOLLARTEXT = ();
        // java String CUSTOM = "custom";
        let CUSTOM = ();
        // java if (null == sNfcMap)
          // java sNfcMap = new HashMap();
            let _sNfcMap = ();
          // java sNfcMap.put("decimal", 0);
          // java sNfcMap.put("upperRoman", 1);
          // java sNfcMap.put("lowerRoman", 2);
          // java sNfcMap.put("upperLetter", 3);
          // java sNfcMap.put("lowerLetter", 4);
          // java sNfcMap.put("ordinal", 5);
          // java sNfcMap.put("cardinalText", 6);
          // java sNfcMap.put("ordinalText", 7);
          // java sNfcMap.put("hex", 8);
          // java sNfcMap.put("chicago", 9);
          // java sNfcMap.put("ideographDigital", 10);
          // java sNfcMap.put("japaneseCounting", 11);
          // java sNfcMap.put("aiueo", 12);
          // java sNfcMap.put("iroha", 13);
          // java sNfcMap.put("decimalFullWidth", 14);
          // java sNfcMap.put("decimalHalfWidth", 15);
          // java sNfcMap.put("japaneseLegal", 16);
          // java sNfcMap.put("japaneseDigitalTenThousand", 17);
          // java sNfcMap.put("decimalEnclosedCircle", 18);
          // java sNfcMap.put("decimalFullWidth2", 19);
          // java sNfcMap.put("aiueoFullWidth", 20);
          // java sNfcMap.put("irohaFullWidth", 21);
          // java sNfcMap.put("decimalZero", 22);
          // java sNfcMap.put("bullet", 23);
          // java sNfcMap.put("ganada", 24);
          // java sNfcMap.put("chosung", 25);
          // java sNfcMap.put("decimalEnclosedFullstop", 26);
          // java sNfcMap.put("decimalEnclosedParen", 27);
          // java sNfcMap.put("decimalEnclosedCircleChinese", 28);
          // java sNfcMap.put("ideographEnclosedCircle", 29);
          // java sNfcMap.put("ideographTraditional", 30);
          // java sNfcMap.put("ideographZodiac", 31);
          // java sNfcMap.put("ideographZodiacTraditional", 32);
          // java sNfcMap.put("taiwaneseCounting", 33);
          // java sNfcMap.put("ideographLegalTraditional", 34);
          // java sNfcMap.put("taiwaneseCountingThousand", 35);
          // java sNfcMap.put("taiwaneseDigital", 36);
          // java sNfcMap.put("chineseCounting", 37);
          // java sNfcMap.put("chineseLegalSimplified", 38);
          // java sNfcMap.put("chineseCountingThousand", 39);
          // java sNfcMap.put("koreanDigital", 41);
          // java sNfcMap.put("koreanCounting", 42);
          // java sNfcMap.put("koreanLegal", 43);
          // java sNfcMap.put("koreanDigital2", 44);
          // java sNfcMap.put("vietnameseCounting", 56);
          // java sNfcMap.put("russianLower", 58);
          // java sNfcMap.put("russianUpper", 59);
          // java sNfcMap.put("none", 255);
          // java sNfcMap.put("numberInDash", 57);
          // java sNfcMap.put("hebrew1", 45);
          // java sNfcMap.put("hebrew2", 47);
          // java sNfcMap.put("arabicAlpha", 46);
          // java sNfcMap.put("arabicAbjad", 48);
          // java sNfcMap.put("hindiVowels", 49);
          // java sNfcMap.put("hindiConsonants", 50);
          // java sNfcMap.put("hindiNumbers", 51);
          // java sNfcMap.put("hindiCounting", 52);
          // java sNfcMap.put("thaiLetters", 53);
          // java sNfcMap.put("thaiNumbers", 54);
          // java sNfcMap.put("thaiCounting", 55);
          // java sNfcMap.put("bahtText", 255);
          // java sNfcMap.put("dollarText", 255);
          // java sNfcMap.put("custom", 255);
        // java return (Integer)sNfcMap.get(xmlNfc);
      // java else
        // java return null;
    }

    // java public static FontInfo getTextFont(Attributes attributes)
    pub fn getTextFont(attributes: &dyn Attributes) -> FontInfo {
      // java FontInfo fontInfo = new FontInfo();
      let mut fontInfo: FontInfo = FontInfo::default();
      // java Integer charset = getInt(attributes, CHARSET);
      let charset: Option<i32> = Self::getInt(attributes, CHARSET.as_deref().unwrap_or("charset"));
      // java if (null != charset)
      if let Some(charset) = charset {
        // java fontInfo.setCharacterSet(charset.shortValue());
        fontInfo.setCharacterSet(charset as i16);
      }
      // java Integer pitchFamily = getInt(attributes, PITCHFAMILY);
      let pitchFamily: Option<i32> = Self::getInt(attributes, PITCHFAMILY.as_deref().unwrap_or("pitchFamily"));
      // java if (null != pitchFamily)
      if let Some(pitchFamily) = pitchFamily {
        // java fontInfo.setFontFamily(pitchFamily.byteValue());
        fontInfo.setFontFamily(pitchFamily as i8);
      }
      // java String panose = getString(attributes, PANOSE);
      let panose: Option<String> = Self::getString__String(attributes, PANOSE.as_deref().unwrap_or("panose"));
      // java if (null != panose)
      if let Some(panose) = panose.as_ref() {
        // java setPanose(panose, fontInfo);
        Self::setPanose(panose, &mut fontInfo);
      }
      // java String typeface = getString(attributes, TYPEFACE);
      let typeface: Option<String> = Self::getString__String(attributes, TYPEFACE.as_deref().unwrap_or("typeface"));
      // java if (null != typeface)
      if let Some(typeface) = typeface {
        // java fontInfo.setFontName(typeface);
        fontInfo.setFontName(typeface);
      }
      // java return fontInfo;
      return fontInfo;
    }

    // java public static void setPanose(String panose, FontInfo fontInfo)
    pub fn setPanose(panose: &str, fontInfo: &mut FontInfo) {
      // java int length = panose.length();
      let length: usize = panose.len();
      // java int PANOSE_SIZE = 20;
      let PANOSE_SIZE: i32 = 20;
      // java if (20 == length)
      if PANOSE_SIZE as usize == length {
        // java convertPanose2Core(panose, fontInfo);
        Self::convertPanose2Core(panose, fontInfo);
      }
    }

    // java private static void convertPanose2Core(String panose1, FontInfo fontInfo)
    pub fn convertPanose2Core(panose1: &str, fontInfo: &mut FontInfo) {
      // java int offset = 0;
      let mut offset: i32 = 0;
      // java Byte familyType = panoseItem(panose1, offset);
      let familyType: Option<i8> = Self::panoseItem(panose1, offset);
      // java if (null != familyType)
      if let Some(familyType) = familyType {
        // java offset += 2;
        offset += PANOSE_ITEM_SIZE;
        // java Byte serifStyle = panoseItem(panose1, offset);
        let serifStyle: Option<i8> = Self::panoseItem(panose1, offset);
        // java if (null != serifStyle)
        if let Some(serifStyle) = serifStyle {
          // java offset += 2;
          offset += PANOSE_ITEM_SIZE;
          // java Byte weight = panoseItem(panose1, offset);
          let weight: Option<i8> = Self::panoseItem(panose1, offset);
          // java if (null != weight)
          if let Some(weight) = weight {
            // java offset += 2;
            offset += PANOSE_ITEM_SIZE;
            // java Byte proportion = panoseItem(panose1, offset);
            let proportion: Option<i8> = Self::panoseItem(panose1, offset);
            // java if (null != proportion)
            if let Some(proportion) = proportion {
              // java offset += 2;
              offset += PANOSE_ITEM_SIZE;
              // java Byte constrast = panoseItem(panose1, offset);
              let constrast: Option<i8> = Self::panoseItem(panose1, offset);
              // java if (null != constrast)
              if let Some(constrast) = constrast {
                // java offset += 2;
                offset += PANOSE_ITEM_SIZE;
                // java Byte strokeVariation = panoseItem(panose1, offset);
                let strokeVariation: Option<i8> = Self::panoseItem(panose1, offset);
                // java if (null != strokeVariation)
                if let Some(strokeVariation) = strokeVariation {
                  // java offset += 2;
                  offset += PANOSE_ITEM_SIZE;
                  // java Byte armStyle = panoseItem(panose1, offset);
                  let armStyle: Option<i8> = Self::panoseItem(panose1, offset);
                  // java if (null != armStyle)
                  if let Some(armStyle) = armStyle {
                    // java offset += 2;
                    offset += PANOSE_ITEM_SIZE;
                    // java Byte letterForm = panoseItem(panose1, offset);
                    let letterForm: Option<i8> = Self::panoseItem(panose1, offset);
                    // java if (null != letterForm)
                    if let Some(letterForm) = letterForm {
                      // java offset += 2;
                      offset += PANOSE_ITEM_SIZE;
                      // java Byte midline = panoseItem(panose1, offset);
                      let midline: Option<i8> = Self::panoseItem(panose1, offset);
                      // java if (null != midline)
                      if let Some(midline) = midline {
                        // java offset += 2;
                        offset += PANOSE_ITEM_SIZE;
                        // java Byte height = panoseItem(panose1, offset);
                        let height: Option<i8> = Self::panoseItem(panose1, offset);
                        // java if (null != height)
                        if let Some(height) = height {
                          // java fontInfo.setPanoseArmStyle(armStyle);
                          fontInfo.setPanoseArmStyle(armStyle);
                          // java fontInfo.setPanoseConstrast(constrast);
                          fontInfo.setPanoseConstrast(constrast);
                          // java fontInfo.setPanoseFamilyType(familyType);
                          fontInfo.setPanoseFamilyType(familyType);
                          // java fontInfo.setPanoseHeight(height);
                          fontInfo.setPanoseHeight(height);
                          // java fontInfo.setPanoseLetterform(letterForm);
                          fontInfo.setPanoseLetterform(letterForm);
                          // java fontInfo.setPanoseMidline(midline);
                          fontInfo.setPanoseMidline(midline);
                          // java fontInfo.setPanoseProportion(proportion);
                          fontInfo.setPanoseProportion(proportion);
                          // java fontInfo.setPanoseSerifStyle(serifStyle);
                          fontInfo.setPanoseSerifStyle(serifStyle);
                          // java fontInfo.setPanoseStrokeVariation(strokeVariation);
                          fontInfo.setPanoseStrokeVariation(strokeVariation);
                          // java fontInfo.setPanoseWeight(weight);
                          fontInfo.setPanoseWeight(weight);
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }

    // java private static Byte panoseItem(String value, int offset)
    pub fn panoseItem(value: &str, offset: i32) -> Option<i8> {
      // java String strItem = value.substring(offset, offset + 2);
      // rust: Java `substring` 越界会抛 `StringIndexOutOfBoundsException`；
      //   调用方 `setPanose` 已保证 `length == 20`，此处 `expect` 忠实保留该前置条件。
      let strItem: &str = value
          .get(offset as usize..(offset + PANOSE_ITEM_SIZE) as usize)
          .expect("panoseItem: value.substring out of range");
      // java Integer intItem = TypeConversion.hexString2Integer(strItem);
      let intItem: Option<i32> = TypeConversion::hexString2Integer(strItem);
      // java return null == intItem ? null : intItem.byteValue();
      return intItem.map(|v| v as i8);
    }

    // java public static SupplementalFont getSupplementalFont(Attributes attributes)
    pub fn getSupplementalFont(attributes: &dyn Attributes) -> SupplementalFont {
      // java SupplementalFont supplementalFont = new SupplementalFont();
      let mut supplementalFont: SupplementalFont = SupplementalFont::new();
      // java String script = getString(attributes, SCRIPT);
      let script: Option<String> = Self::getString__String(attributes, SCRIPT.as_deref().unwrap_or("script"));
      // java if (null != script)
      if let Some(script) = script.as_ref() {
        // java supplementalFont.setScript(script.hashCode());
        supplementalFont.setScript(java_string_hash_code(script));
      }
      // java String typeface = getString(attributes, TYPEFACE);
      let typeface: Option<String> = Self::getString__String(attributes, TYPEFACE.as_deref().unwrap_or("typeface"));
      // java if (null != typeface)
      if let Some(typeface) = typeface {
        // java supplementalFont.setTypeface(typeface);
        supplementalFont.setTypeface(Some(typeface));
      }
      // java return supplementalFont;
      return supplementalFont;
    }

}
