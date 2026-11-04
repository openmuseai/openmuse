#![allow(non_snake_case, dead_code)]
// Java: import cn.wps.base.io.MetricUtility
use util::base::io::MetricUtility::MetricUtility;
// Java: import cn.wps.base.io.TypeConversion
use util::base::io::TypeConversion::TypeConversion;
// Java: import cn.wps.moffice.drawing.graphics.TileFlipMode
use drawing::cn::wps::moffice::drawing::graphics::TileFlipMode::TileFlipMode;
// Java: import cn.wps.moffice.writer.io.reader.dmlReader.helper.EffectHelper
use crate::cn::wps::moffice::writer::io::reader::dmlReader::helper::EffectHelper::EffectHelper;
// Java: import org.xml.sax.Attributes
use io_wps_io::cn::wps::io::xwpf::SaxAttributes::SaxAttributes;
use io_wps_io::cn::wps::io::dom::io::WMap::WMap;

fn empty_int(attrs: &dyn SaxAttributes, name: &str) -> Option<i32> {
    TypeConversion::string2Integer(&attrs.getValue_by_uri("", name)?)
}
fn main_string(attrs: &dyn SaxAttributes, name: &str) -> Option<String> {
    if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), name) }
}
fn java_hash(value: &str) -> i32 {
    value.encode_utf16().fold(0_i32, |hash, ch| hash.wrapping_mul(31).wrapping_add(ch as i32))
}

// Java: public class Tile
pub struct Tile {
    pub mTx: Option<f32>,
    pub mTy: Option<f32>,
    pub mSx: Option<f32>,
    pub mSy: Option<f32>,
    pub mFlip: TileFlipMode,
    pub mAlgn: i32,
}
impl Tile {
    pub const X: i32 = 120;
    pub const Y: i32 = 121;
    pub const XY: i32 = 3841;
    pub const NONE: i32 = 3387192;

    // Java: public Tile(Attributes attrs)
    pub fn new(attrs: &dyn SaxAttributes) -> Self {
        // Java: Integer intVal = ConvertHelper.getInt4EmptyNamespace(attrs, "tx");
        let mTx = empty_int(attrs, "tx").map(|v| MetricUtility::EMU2Points(v as f32));
        // Java: intVal = ConvertHelper.getInt4EmptyNamespace(attrs, "ty");
        let mTy = empty_int(attrs, "ty").map(|v| MetricUtility::EMU2Points(v as f32));
        // Java: intVal = ConvertHelper.getInt4EmptyNamespace(attrs, "sx");
        let mSx = empty_int(attrs, "sx").map(|v| MetricUtility::STPercentageDecimal2Float(v as i64));
        // Java: intVal = ConvertHelper.getInt4EmptyNamespace(attrs, "sy");
        let mSy = empty_int(attrs, "sy").map(|v| MetricUtility::STPercentageDecimal2Float(v as i64));
        // Java: Integer algnInteger = EffectHelper.string2RectAlignmentType(value);
        let algn = main_string(attrs, "algn");
        let mAlgn = EffectHelper::string2RectAlignmentType(algn.as_deref()).unwrap_or(0);
        // Java: this.mFlip = null == flip ? TileFlipMode.none : this.convert2TileFlipMode(flip);
        let flip = main_string(attrs, "flip");
        let mFlip = flip.as_deref().map(Self::convert2TileFlipMode).unwrap_or(TileFlipMode::none);
        Self { mTx, mTy, mSx, mSy, mFlip, mAlgn }
    }
    // Java: private TileFlipMode convert2TileFlipMode(String flip)
    pub fn convert2TileFlipMode(flip: &str) -> TileFlipMode {
        // Java: switch (flip.hashCode())
        match java_hash(flip) {
            Self::X => TileFlipMode::x, Self::Y => TileFlipMode::y,
            Self::XY => TileFlipMode::xy, Self::NONE => TileFlipMode::none,
            _ => TileFlipMode::none,
        }
    }
}
