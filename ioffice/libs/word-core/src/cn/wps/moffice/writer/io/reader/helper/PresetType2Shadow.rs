// Java: cn.wps.moffice.writer.io.reader.helper.PresetType2Shadow
use std::sync::{Arc, Mutex, OnceLock};
use std::collections::HashMap;
use drawing::cn::wps::moffice::drawing::effects::ShadowDml::ShadowDml;
use drawing::cn::wps::moffice::drawing::effects::ShadowPos::ShadowPos;
use util::base::io::MetricUtility::MetricUtility;
use util::moffice::util::UnitUtil::UnitUtil;

fn create_shadow_dml(shadow_type: i32, shadow_prst_type: i32, shadow_opacity: i32,
    shadow_offset_x: i32, shadow_offset_y: i32, shadow_offset2_x: i32, shadow_offset2_y: i32,
    shadow_origin_x: i32, shadow_origin_y: i32, shadow_scale_x_to_x: i32, shadow_scale_x_to_y: i32,
    shadow_scale_y_to_x: i32, shadow_scale_y_to_y: i32, shadow_perspective_x: i32,
    shadow_perspective_y: i32) -> ShadowDml {
    // Java: createShadowDml ignores shadowWeight and strokeOn.
    let mut shadow = ShadowDml::new();
    shadow.setShadowPrstType(shadow_prst_type);
    if shadow_prst_type == -903473718 {
        shadow.base.setShadowHighLight(275121139);
    } else {
        shadow.base.setShadowHighLight(16777215);
    }
    shadow.setShadowPos(ShadowPos::Outer);
    shadow.base.setShadowOn(true);
    shadow.base.setShadowColor(10066329);
    shadow.base.setShadowType(shadow_type);
    shadow.base.setShadowOpacity(UnitUtil::FIX2FLOAT(shadow_opacity));
    shadow.base.setShadowOffsetX(MetricUtility::EMU2Points(shadow_offset_x as f32));
    shadow.base.setShadowOffsetY(MetricUtility::EMU2Points(shadow_offset_y as f32));
    shadow.base.setShadowSecondOffsetX(MetricUtility::EMU2Points(shadow_offset2_x as f32));
    shadow.base.setShadowSecondOffsetY(MetricUtility::EMU2Points(shadow_offset2_y as f32));
    shadow.base.setShadowOriginX(UnitUtil::FIX2FLOAT(shadow_origin_x));
    shadow.base.setShadowOriginY(UnitUtil::FIX2FLOAT(shadow_origin_y));
    shadow.base.setShadowScaleXToX(UnitUtil::FIX2FLOAT(shadow_scale_x_to_x));
    shadow.base.setShadowScaleXToY(UnitUtil::FIX2FLOAT(shadow_scale_x_to_y));
    shadow.base.setShadowScaleYToX(UnitUtil::FIX2FLOAT(shadow_scale_y_to_x));
    shadow.base.setShadowScaleYToY(UnitUtil::FIX2FLOAT(shadow_scale_y_to_y));
    shadow.base.setShadowPerspectiveX(UnitUtil::FIX2FLOAT(shadow_perspective_x) / 256.0);
    shadow.base.setShadowPerspectiveY(UnitUtil::FIX2FLOAT(shadow_perspective_y) / 256.0);
    shadow
}

fn presets() -> &'static HashMap<i32, Arc<Mutex<ShadowDml>>> {
    static MAP: OnceLock<HashMap<i32, Arc<Mutex<ShadowDml>>>> = OnceLock::new();
    MAP.get_or_init(|| {
        let rows = [
            (109403017, create_shadow_dml(0, 109403017, 32768, -76200, -76200, 0, 0, 0, 0, 65536, 0, 0, 65536, 0, 0)),
            (109403018, create_shadow_dml(0, 109403018, 32768, 76200, -76200, 0, 0, 0, 0, 65536, 0, 0, 65536, 0, 0)),
            (109403019, create_shadow_dml(3, 109403019, 32768, 0, 0, 0, 0, 0, 32768, 65536, 56756, 0, 32768, 0, 0)),
            (109403020, create_shadow_dml(3, 109403020, 32768, 0, 0, 0, 0, 0, 32768, 65536, -56756, 0, 32768, 0, 0)),
            (109403021, create_shadow_dml(0, 109403021, 32768, -76200, 76200, 0, 0, 0, 0, 65536, 0, 0, 65536, 0, 0)),
            (109403022, create_shadow_dml(0, 109403022, 32768, 76200, 76200, 0, 0, 0, 0, 65536, 0, 0, 65536, 0, 0)),
            (109403023, create_shadow_dml(3, 109403023, 32768, 0, 0, 0, 0, 0, 32768, 65536, 56756, 0, -32768, 0, 0)),
            (109403024, create_shadow_dml(3, 109403024, 32768, 0, 0, 0, 0, 0, 32768, 65536, -56756, 0, -32768, 0, 0)),
            (109403025, create_shadow_dml(3, 109403025, 32768, -76200, -76200, 0, 0, -32768, -32768, 49152, 0, 0, 49152, 0, 0)),
            (-903473721, create_shadow_dml(3, -903473721, 32768, -76200, -76200, 0, 0, 32768, 32768, 81920, 0, 0, 81920, 0, 0)),
            (-903473720, create_shadow_dml(2, -903473720, 32768, 0, 0, 0, 0, -32768, 32768, 65536, 0, 92680, 65536, 0, -16)),
            (-903473719, create_shadow_dml(2, -903473719, 32768, 0, 0, 0, 0, 32768, 32768, 65536, 0, -92680, 65536, 0, -16)),
            (-903473718, create_shadow_dml(1, -903473718, 32768, -38100, -38100, 0, 0, 0, 0, 65536, 0, 0, 65536, 0, 0)),
            (-903473717, create_shadow_dml(0, -903473717, 65536, 25400, 25400, 0, 0, 0, 0, 65536, 0, 0, 65536, 0, 0)),
            (-903473716, create_shadow_dml(2, -903473716, 32768, 0, 0, 0, 0, -32768, 32768, 65536, 0, 92680, -65536, 0, -16)),
            (-903473715, create_shadow_dml(2, -903473715, 32768, 0, 0, 0, 0, 32768, 32768, 65536, 0, -92680, -65536, 0, -16)),
            (-903473714, create_shadow_dml(5, -903473714, 65536, 12700, 12700, -12700, -12700, 0, 0, 65536, 0, 0, 65536, 0, 0)),
            (-903473713, create_shadow_dml(2, -903473713, 65536, -12700, -12700, 0, 0, 0, 0, 65536, 0, 0, 65536, 0, 0)),
            (-903473712, create_shadow_dml(3, -903473712, 32768, 0, 0, 0, 0, 0, 32768, 65536, 0, 0, 32768, 0, -8)),
            (-903473690, create_shadow_dml(3, -903473690, 32768, 0, 0, 0, 0, 0, 32768, 65536, 0, 0, -65536, 0, 0)),
        ];
        rows.into_iter().map(|(id, shadow)| (id, Arc::new(Mutex::new(shadow)))).collect()
    })
}

// Java: public class PresetType2Shadow
pub struct PresetType2Shadow;

impl PresetType2Shadow {
    // Java: public static ShadowDml getShadowDml(int prstType)
    pub fn getShadowDml(prstType: i32) -> Option<Arc<Mutex<ShadowDml>>> {
        presets().get(&prstType).cloned()
    }
}
