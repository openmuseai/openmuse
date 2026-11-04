// Java field: byte blipERROR
pub const blipERROR: i8 = 0;
// Java field: byte blipUNKNOWN
pub const blipUNKNOWN: i8 = 1;
// Java field: byte blipEMF
pub const blipEMF: i8 = 2;
// Java field: byte blipWMF
pub const blipWMF: i8 = 3;
// Java field: byte blipPICT
pub const blipPICT: i8 = 4;
// Java field: byte blipJPEG
pub const blipJPEG: i8 = 5;
// Java field: byte blipPNG
pub const blipPNG: i8 = 6;
// Java field: byte blipDIB
pub const blipDIB: i8 = 7;
// Java field: byte blipWBIBITMAP
pub const blipWBIBITMAP: i8 = 8;
// Java field: byte blipTIFF
pub const blipTIFF: i8 = 17;
// Java field: byte blipCMYKJPEG
pub const blipCMYKJPEG: i8 = 18;
// Java field: byte hdphoto
pub const hdphoto: i8 = 19;
// Java field: String[] blipExts
static BLIP_EXTS: &[&str] = &[
    "jpg", "jpg", "emf", "wmf", "pict", "jpeg", "png", "dib",
    "", "", "", "", "", "", "", "", "", "tiff",
];

// Java: public class BlipType
pub struct BlipType {
}

// Java: public class BlipType
impl BlipType {
    // Java: BlipType()
    pub fn new() -> Self {
        // Java: super();
        BlipType {}
    }

    // Java: static byte getBlipType(String ext)
    pub fn getBlipType(ext: &str) -> i8 {
        // Java: byte blipType = 1;
        let mut blip_type: i8 = 1;
        // Java: if (ext.equalsIgnoreCase("jpg"))
        if ext.eq_ignore_ascii_case("jpg") {
            // Java: blipType = 5;
            blip_type = 5;
        }
        // Java: else if (ext.equalsIgnoreCase("jpeg"))
        else if ext.eq_ignore_ascii_case("jpeg") {
            // Java: blipType = 5;
            blip_type = 5;
        }
        // Java: else if (ext.equalsIgnoreCase("jpe"))
        else if ext.eq_ignore_ascii_case("jpe") {
            // Java: blipType = 5;
            blip_type = 5;
        }
        // Java: else if (ext.equalsIgnoreCase("jfjf"))
        else if ext.eq_ignore_ascii_case("jfjf") {
            // Java: blipType = 5;
            blip_type = 5;
        }
        // Java: else if (ext.equalsIgnoreCase("jfif"))
        else if ext.eq_ignore_ascii_case("jfif") {
            // Java: blipType = 5;
            blip_type = 5;
        }
        // Java: else if (ext.equalsIgnoreCase("png"))
        else if ext.eq_ignore_ascii_case("png") {
            // Java: blipType = 6;
            blip_type = 6;
        }
        // Java: else if (ext.equalsIgnoreCase("bmp"))
        else if ext.eq_ignore_ascii_case("bmp") {
            // Java: blipType = 7;
            blip_type = 7;
        }
        // Java: else if (ext.equalsIgnoreCase("wmf"))
        else if ext.eq_ignore_ascii_case("wmf") {
            // Java: blipType = 3;
            blip_type = 3;
        }
        // Java: else if (ext.equalsIgnoreCase("emf"))
        else if ext.eq_ignore_ascii_case("emf") {
            // Java: blipType = 2;
            blip_type = 2;
        }
        // Java: else if (ext.equalsIgnoreCase("gif"))
        else if ext.eq_ignore_ascii_case("gif") {
            // Java: blipType = 6;
            blip_type = 6;
        }
        // Java: else if (ext.equalsIgnoreCase("dib"))
        else if ext.eq_ignore_ascii_case("dib") {
            // Java: blipType = 7;
            blip_type = 7;
        }
        // Java: else if (ext.equalsIgnoreCase("pict"))
        else if ext.eq_ignore_ascii_case("pict") {
            // Java: blipType = 4;
            blip_type = 4;
        }
        // Java: else if (ext.equalsIgnoreCase("tiff"))
        else if ext.eq_ignore_ascii_case("tiff") {
            // Java: blipType = 6;
            blip_type = 6;
        }
        // Java: else if (ext.equalsIgnoreCase("tif"))
        else if ext.eq_ignore_ascii_case("tif") {
            // Java: blipType = 6;
            blip_type = 6;
        }
        // Java: return blipType;
        blip_type
    }

    // Java: static String getBlipExt(byte blipType)
    pub fn getBlipExt(blip_type: i8) -> String {
        // Java: return blipType >= 0 && blipType <= 18 ? blipExts[blipType - 0] : "";
        if blip_type >= 0 && blip_type <= 18 {
            BLIP_EXTS[blip_type as usize].to_string()
        } else {
            String::new()
        }
    }
}
