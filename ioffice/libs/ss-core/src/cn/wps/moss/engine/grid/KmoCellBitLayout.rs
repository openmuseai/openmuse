// Java: import cn.wps.moss.util.KmoBitField
use crate::cn::wps::moss::util::KmoBitField::KmoBitField;

// java public static final KmoBitField fomulaType = new KmoBitField(3);
pub(crate) static fomulaType: std::sync::LazyLock<KmoBitField> =
    std::sync::LazyLock::new(|| KmoBitField::KmoBitField(3));
// java public static final KmoBitField hasComment = new KmoBitField(8);
pub(crate) static hasComment: std::sync::LazyLock<KmoBitField> =
    std::sync::LazyLock::new(|| KmoBitField::KmoBitField(8));
// java public static final KmoBitField isPicInCell = new KmoBitField(16);
pub(crate) static isPicInCell: std::sync::LazyLock<KmoBitField> =
    std::sync::LazyLock::new(|| KmoBitField::KmoBitField(16));

// java  class KmoCellBitLayout
pub struct KmoCellBitLayout {}

// java  class KmoCellBitLayout
impl KmoCellBitLayout {}
