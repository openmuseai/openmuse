// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.io.util.HexColor
use io_wps_io::io::util::HexColor::*;
// Java: import cn.wps.io.util.IntEnumMap
use io_wps_io::io::util::IntEnumMap::*;

// java public static final int MIN_LINE_WIDTH = 0;
pub(crate) const MIN_LINE_WIDTH: i32 = 0;
// java public static final int MAX_LINE_WIDTH = 96;
pub(crate) const MAX_LINE_WIDTH: i32 = 96;

// java public abstract class Border
#[derive(Default)]
pub struct Border {
}

// java public static enum BorderStyle
#[derive(Copy, Clone, Debug)]
pub enum BorderStyle {
}

// java public abstract class Border
impl Border {
    // java public abstract Long getLineWidth()
    pub fn getLineWidth(&self) -> i64 {
      panic!("Border::getLineWidth not implemented");
    }

    // java public abstract BorderStyle getBorderStyle()
    pub fn getBorderStyle(&self) -> BorderStyle {
      panic!("Border::getBorderStyle not implemented");
    }

    // java public abstract HexColor getColor()
    pub fn getColor(&self) -> HexColor {
      panic!("Border::getColor not implemented");
    }

    // java public abstract Long getSpace()
    pub fn getSpace(&self) -> i64 {
      panic!("Border::getSpace not implemented");
    }

    // java public abstract Boolean isShadow()
    pub fn isShadow(&self) -> bool {
      panic!("Border::isShadow not implemented");
    }

    // java public abstract Boolean isFrame()
    pub fn isFrame(&self) -> bool {
      panic!("Border::isFrame not implemented");
    }

}

// java public static enum BorderStyle
impl BorderStyle {
}
