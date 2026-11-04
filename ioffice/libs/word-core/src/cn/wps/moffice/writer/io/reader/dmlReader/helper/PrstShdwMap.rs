// Java: cn.wps.moffice.writer.io.reader.dmlReader.helper.PrstShdwMap
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use drawing::cn::wps::moffice::drawing::effects::ShadowDml::ShadowDml;
use crate::cn::wps::moffice::writer::io::reader::helper::PresetType2Shadow::PresetType2Shadow;

fn names() -> &'static HashMap<i32, &'static str> {
    static MAP: OnceLock<HashMap<i32, &'static str>> = OnceLock::new();
    MAP.get_or_init(|| HashMap::from([
        (109403017, "shdw1"), (109403018, "shdw2"), (109403019, "shdw3"),
        (109403020, "shdw4"), (109403021, "shdw5"), (109403022, "shdw6"),
        (109403023, "shdw7"), (109403024, "shdw8"), (109403025, "shdw9"),
        (-903473721, "shdw10"), (-903473720, "shdw11"), (-903473719, "shdw12"),
        (-903473718, "shdw13"), (-903473717, "shdw14"), (-903473716, "shdw15"),
        (-903473715, "shdw16"), (-903473714, "shdw17"), (-903473713, "shdw18"),
        (-903473712, "shdw19"), (-903473690, "shdw20"),
    ]))
}

// Java: public class PrstShdwMap
pub struct PrstShdwMap;

impl PrstShdwMap {
    // Java: public static String name(Integer id)
    pub fn name(id: Option<i32>) -> Option<&'static str> {
        id.and_then(|id| names().get(&id).copied())
    }

    // Java: public static ShadowDml getPresetShadow(Integer id)
    pub fn getPresetShadow(id: Option<i32>) -> Option<Arc<Mutex<ShadowDml>>> {
        id.and_then(PresetType2Shadow::getShadowDml)
    }
}
