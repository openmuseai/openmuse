// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::OnceLock;

// Java field: int UNKNOWN_GUID_MICROSOFT_0x01b
pub(crate) const UNKNOWN_GUID_MICROSOFT_0x01b: i32 = 27;
// Java field: int UNKNOWN_GUID_MICROSOFT_0x064
pub(crate) const UNKNOWN_GUID_MICROSOFT_0x064: i32 = 100;
// Java field: int UNKNOWN_GUID_MICROSOFT_0x065
pub(crate) const UNKNOWN_GUID_MICROSOFT_0x065: i32 = 101;
// Java field: int KNOWN_GUID_BASE_INDEX
pub(crate) const KNOWN_GUID_BASE_INDEX: i32 = 50;
// Java field: int GUID_X
pub(crate) const GUID_X: i32 = 50;
// Java field: int GUID_Y
pub(crate) const GUID_Y: i32 = 51;
// Java field: int GUID_Z
pub(crate) const GUID_Z: i32 = 52;
// Java field: int GUID_PACKET_STATUS
pub(crate) const GUID_PACKET_STATUS: i32 = 53;
// Java field: int GUID_TIMER_TICK
pub(crate) const GUID_TIMER_TICK: i32 = 54;
// Java field: int GUID_SERIAL_NUMBER
pub(crate) const GUID_SERIAL_NUMBER: i32 = 55;
// Java field: int GUID_NORMAL_PRESSURE
pub(crate) const GUID_NORMAL_PRESSURE: i32 = 56;
// Java field: int GUID_TANGENT_PRESSURE
pub(crate) const GUID_TANGENT_PRESSURE: i32 = 57;
// Java field: int GUID_BUTTON_PRESSURE
pub(crate) const GUID_BUTTON_PRESSURE: i32 = 58;
// Java field: int GUID_X_TILT_ORIENTATION
pub(crate) const GUID_X_TILT_ORIENTATION: i32 = 59;
// Java field: int GUID_Y_TILT_ORIENTATION
pub(crate) const GUID_Y_TILT_ORIENTATION: i32 = 60;
// Java field: int GUID_AZIMUTH_ORIENTATION
pub(crate) const GUID_AZIMUTH_ORIENTATION: i32 = 61;
// Java field: int GUID_ALTITUDE_ORIENTATION
pub(crate) const GUID_ALTITUDE_ORIENTATION: i32 = 62;
// Java field: int GUID_TWIST_ORIENTATION
pub(crate) const GUID_TWIST_ORIENTATION: i32 = 63;
// Java field: int GUID_PITCH_ROTATION
pub(crate) const GUID_PITCH_ROTATION: i32 = 64;
// Java field: int GUID_ROLL_ROTATION
pub(crate) const GUID_ROLL_ROTATION: i32 = 65;
// Java field: int GUID_YAW_ROTATION
pub(crate) const GUID_YAW_ROTATION: i32 = 66;
// Java field: int GUID_PEN_STYLE
pub(crate) const GUID_PEN_STYLE: i32 = 67;
// Java field: int GUID_COLORREF
pub(crate) const GUID_COLORREF: i32 = 68;
// Java field: int GUID_PEN_WIDTH
pub(crate) const GUID_PEN_WIDTH: i32 = 69;
// Java field: int GUID_PEN_HEIGHT
pub(crate) const GUID_PEN_HEIGHT: i32 = 70;
// Java field: int GUID_PEN_TIP
pub(crate) const GUID_PEN_TIP: i32 = 71;
// Java field: int GUID_DRAWING_FLAGS
pub(crate) const GUID_DRAWING_FLAGS: i32 = 72;
// Java field: int GUID_CURSORID
pub(crate) const GUID_CURSORID: i32 = 73;
// Java field: int GUID_WORD_ALTERNATES
pub(crate) const GUID_WORD_ALTERNATES: i32 = 74;
// Java field: int GUID_CHAR_ALTERNATES
pub(crate) const GUID_CHAR_ALTERNATES: i32 = 75;
// Java field: int GUID_INKMETRICS
pub(crate) const GUID_INKMETRICS: i32 = 76;
// Java field: int GUID_GUIDE_STRUCTURE
pub(crate) const GUID_GUIDE_STRUCTURE: i32 = 77;
// Java field: int GUID_TIME_STAMP
pub(crate) const GUID_TIME_STAMP: i32 = 78;
// Java field: int GUID_LANGUAGE
pub(crate) const GUID_LANGUAGE: i32 = 79;
// Java field: int GUID_TRANSPARENCY
pub(crate) const GUID_TRANSPARENCY: i32 = 80;
// Java field: int GUID_CURVE_FITTING_ERROR
pub(crate) const GUID_CURVE_FITTING_ERROR: i32 = 81;
// Java field: int GUID_RECO_LATTICE
pub(crate) const GUID_RECO_LATTICE: i32 = 82;
// Java field: int GUID_CURSORDOWN
pub(crate) const GUID_CURSORDOWN: i32 = 83;
// Java field: int GUID_SECONDARYTIPSWITCH
pub(crate) const GUID_SECONDARYTIPSWITCH: i32 = 84;
// Java field: int GUID_BARRELDOWN
pub(crate) const GUID_BARRELDOWN: i32 = 85;
// Java field: int GUID_TABLETPICK
pub(crate) const GUID_TABLETPICK: i32 = 86;
// Java field: int GUID_ROP
pub(crate) const GUID_ROP: i32 = 87;
// Java field: int KNOWN_GUID_LAST_INDEX
pub(crate) const KNOWN_GUID_LAST_INDEX: i32 = 88;
// Java field: String STR_GUID_X
pub(crate) const STR_GUID_X: &str = "GUID_X";
// Java field: String STR_GUID_Y
pub(crate) const STR_GUID_Y: &str = "GUID_Y";
// Java field: String STR_GUID_Z
pub(crate) const STR_GUID_Z: &str = "GUID_Z";
// Java field: String STR_GUID_PACKET_STATUS
pub(crate) const STR_GUID_PACKET_STATUS: &str = "GUID_PACKET_STATUS";
// Java field: String STR_GUID_TIMER_TICK
pub(crate) const STR_GUID_TIMER_TICK: &str = "GUID_TIMER_TICK";
// Java field: String STR_GUID_SERIAL_NUMBER
pub(crate) const STR_GUID_SERIAL_NUMBER: &str = "GUID_SERIAL_NUMBER";
// Java field: String STR_GUID_NORMAL_PRESSURE
pub(crate) const STR_GUID_NORMAL_PRESSURE: &str = "GUID_NORMAL_PRESSURE";
// Java field: String STR_GUID_TANGENT_PRESSURE
pub(crate) const STR_GUID_TANGENT_PRESSURE: &str = "GUID_TANGENT_PRESSURE";
// Java field: String STR_GUID_BUTTON_PRESSURE
pub(crate) const STR_GUID_BUTTON_PRESSURE: &str = "GUID_BUTTON_PRESSURE";
// Java field: String STR_GUID_X_TILT_ORIENTATION
pub(crate) const STR_GUID_X_TILT_ORIENTATION: &str = "GUID_X_TILT_ORIENTATION";
// Java field: String STR_GUID_Y_TILT_ORIENTATION
pub(crate) const STR_GUID_Y_TILT_ORIENTATION: &str = "GUID_Y_TILT_ORIENTATION";
// Java field: String STR_GUID_AZIMUTH_ORIENTATION
pub(crate) const STR_GUID_AZIMUTH_ORIENTATION: &str = "GUID_AZIMUTH_ORIENTATION";
// Java field: String STR_GUID_ALTITUDE_ORIENTATION
pub(crate) const STR_GUID_ALTITUDE_ORIENTATION: &str = "GUID_ALTITUDE_ORIENTATION";
// Java field: String STR_GUID_TWIST_ORIENTATION
pub(crate) const STR_GUID_TWIST_ORIENTATION: &str = "GUID_TWIST_ORIENTATION";
// Java field: String STR_GUID_PITCH_ROTATION
pub(crate) const STR_GUID_PITCH_ROTATION: &str = "GUID_PITCH_ROTATION";
// Java field: String STR_GUID_ROLL_ROTATION
pub(crate) const STR_GUID_ROLL_ROTATION: &str = "GUID_ROLL_ROTATION";
// Java field: String STR_GUID_YAW_ROTATION
pub(crate) const STR_GUID_YAW_ROTATION: &str = "GUID_YAW_ROTATION";
// Java field: String STR_GUID_PEN_STYLE
pub(crate) const STR_GUID_PEN_STYLE: &str = "GUID_PEN_STYLE";
// Java field: String STR_GUID_COLORREF
pub(crate) const STR_GUID_COLORREF: &str = "GUID_COLORREF";
// Java field: String STR_GUID_PEN_WIDTH
pub(crate) const STR_GUID_PEN_WIDTH: &str = "GUID_PEN_WIDTH";
// Java field: String STR_GUID_PEN_HEIGHT
pub(crate) const STR_GUID_PEN_HEIGHT: &str = "GUID_PEN_HEIGHT";
// Java field: String STR_GUID_PEN_TIP
pub(crate) const STR_GUID_PEN_TIP: &str = "GUID_PEN_TIP";
// Java field: String STR_GUID_DRAWING_FLAGS
pub(crate) const STR_GUID_DRAWING_FLAGS: &str = "GUID_DRAWING_FLAGS";
// Java field: String STR_GUID_CURSORID
pub(crate) const STR_GUID_CURSORID: &str = "GUID_CURSORID";
// Java field: String STR_GUID_WORD_ALTERNATES
pub(crate) const STR_GUID_WORD_ALTERNATES: &str = "GUID_WORD_ALTERNATES";
// Java field: String STR_GUID_CHAR_ALTERNATES
pub(crate) const STR_GUID_CHAR_ALTERNATES: &str = "GUID_CHAR_ALTERNATES";
// Java field: String STR_GUID_INKMETRICS
pub(crate) const STR_GUID_INKMETRICS: &str = "GUID_INKMETRICS";
// Java field: String STR_GUID_GUIDE_STRUCTURE
pub(crate) const STR_GUID_GUIDE_STRUCTURE: &str = "GUID_GUIDE_STRUCTURE";
// Java field: String STR_GUID_TIME_STAMP
pub(crate) const STR_GUID_TIME_STAMP: &str = "GUID_TIME_STAMP";
// Java field: String STR_GUID_LANGUAGE
pub(crate) const STR_GUID_LANGUAGE: &str = "GUID_LANGUAGE";
// Java field: String STR_GUID_TRANSPARENCY
pub(crate) const STR_GUID_TRANSPARENCY: &str = "GUID_TRANSPARENCY";
// Java field: String STR_GUID_CURVE_FITTING_ERROR
pub(crate) const STR_GUID_CURVE_FITTING_ERROR: &str = "GUID_CURVE_FITTING_ERROR";
// Java field: String STR_GUID_RECO_LATTICE
pub(crate) const STR_GUID_RECO_LATTICE: &str = "GUID_RECO_LATTICE";
// Java field: String STR_GUID_CURSORDOWN
pub(crate) const STR_GUID_CURSORDOWN: &str = "GUID_CURSORDOWN";
// Java field: String STR_GUID_SECONDARYTIPSWITCH
pub(crate) const STR_GUID_SECONDARYTIPSWITCH: &str = "GUID_SECONDARYTIPSWITCH";
// Java field: String STR_GUID_BARRELDOWN
pub(crate) const STR_GUID_BARRELDOWN: &str = "GUID_BARRELDOWN";
// Java field: String STR_GUID_TABLETPICK
pub(crate) const STR_GUID_TABLETPICK: &str = "GUID_TABLETPICK";
// Java field: String STR_GUID_ROP
pub(crate) const STR_GUID_ROP: &str = "GUID_ROP";

// Java field: HashMap<Integer, String> sInteger2StringMap
fn get_sInteger2StringMap() -> &'static HashMap<i32, &'static str> {
    static MAP: OnceLock<HashMap<i32, &str>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(50, "GUID_X");
        m.insert(51, "GUID_Y");
        m.insert(52, "GUID_Z");
        m.insert(53, "GUID_PACKET_STATUS");
        m.insert(54, "GUID_TIMER_TICK");
        m.insert(55, "GUID_SERIAL_NUMBER");
        m.insert(56, "GUID_NORMAL_PRESSURE");
        m.insert(57, "GUID_TANGENT_PRESSURE");
        m.insert(58, "GUID_BUTTON_PRESSURE");
        m.insert(59, "GUID_X_TILT_ORIENTATION");
        m.insert(60, "GUID_Y_TILT_ORIENTATION");
        m.insert(61, "GUID_AZIMUTH_ORIENTATION");
        m.insert(62, "GUID_ALTITUDE_ORIENTATION");
        m.insert(63, "GUID_TWIST_ORIENTATION");
        m.insert(64, "GUID_PITCH_ROTATION");
        m.insert(65, "GUID_ROLL_ROTATION");
        m.insert(66, "GUID_YAW_ROTATION");
        m.insert(67, "GUID_PEN_STYLE");
        m.insert(68, "GUID_COLORREF");
        m.insert(69, "GUID_PEN_WIDTH");
        m.insert(70, "GUID_PEN_HEIGHT");
        m.insert(71, "GUID_PEN_TIP");
        m.insert(72, "GUID_DRAWING_FLAGS");
        m.insert(73, "GUID_CURSORID");
        m.insert(74, "GUID_WORD_ALTERNATES");
        m.insert(75, "GUID_CHAR_ALTERNATES");
        m.insert(76, "GUID_INKMETRICS");
        m.insert(77, "GUID_GUIDE_STRUCTURE");
        m.insert(78, "GUID_TIME_STAMP");
        m.insert(79, "GUID_LANGUAGE");
        m.insert(80, "GUID_TRANSPARENCY");
        m.insert(81, "GUID_CURVE_FITTING_ERROR");
        m.insert(82, "GUID_RECO_LATTICE");
        m.insert(83, "GUID_CURSORDOWN");
        m.insert(84, "GUID_SECONDARYTIPSWITCH");
        m.insert(85, "GUID_BARRELDOWN");
        m.insert(86, "GUID_TABLETPICK");
        m.insert(87, "GUID_ROP");
        m
    })
}

// Java: public class PacketProperty
pub struct PacketProperty;

// Java: public class PacketProperty
impl PacketProperty {
    // Java: PacketProperty()
    pub fn PacketProperty() -> Self {
        // Java: super();
        PacketProperty
    }

    // Java: static String toName(int id)
    pub fn toName(id: i32) -> String {
        // Java: return (String)sInteger2StringMap.get(id);
        get_sInteger2StringMap()
            .get(&id)
            .map(|s| s.to_string())
            .unwrap_or_default()
    }
}
