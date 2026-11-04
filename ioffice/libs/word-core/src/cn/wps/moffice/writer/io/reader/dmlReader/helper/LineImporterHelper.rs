// Java: cn.wps.moffice.writer.io.reader.dmlReader.helper.LineImporterHelper

fn java_hash(value: &str) -> i32 {
    value.encode_utf16().fold(0_i32, |hash, ch| hash.wrapping_mul(31).wrapping_add(ch as i32))
}

// Java: public class LineImporterHelper
pub struct LineImporterHelper;

impl LineImporterHelper {
    // Java: public static int toLineDashing(String src)
    pub fn toLineDashing(src: &str) -> i32 {
        match java_hash(src) {
            -1738804225 => 1,
            -1105773651 => 7,
            -887374020 => 2,
            -630159027 => 10,
            99657 => 5,
            3075986 => 6,
            109618859 => 0,
            296392284 => 9,
            883958986 => 3,
            1442454615 => 8,
            1577729055 => 4,
            _ => 0,
        }
    }

    // Java: public static int string2LineStyle(String src)
    pub fn string2LineStyle(src: &str) -> i32 {
        match java_hash(src) {
            -1008324650 => 2,
            -714048380 => 3,
            99246 => 1,
            114028 => 0,
            115115 => 4,
            _ => 0,
        }
    }

    // Java: public static int string2LineEndCapStyle(String src)
    pub fn string2LineEndCapStyle(src: &str) -> i32 {
        match java_hash(src) {
            3678 => 1,
            113064 => 0,
            3145593 => 2,
            _ => 0,
        }
    }

    // Java: public static int string2ArrowStyle(String src)
    pub fn string2ArrowStyle(src: &str) -> i32 {
        match java_hash(src) {
            -1894004733 => 2,
            3387192 => 0,
            3423314 => 4,
            93090825 => 5,
            1497762312 => 1,
            1655054676 => 3,
            _ => 0,
        }
    }

    // Java: public static int string2ArrowWidth(String src)
    pub fn string2ArrowWidth(src: &str) -> i32 {
        match java_hash(src) {
            3451 => 2,
            3674 => 0,
            107980 => 1,
            _ => 1,
        }
    }

    // Java: public static int string2ArrowLength(String src)
    pub fn string2ArrowLength(src: &str) -> i32 {
        match java_hash(src) {
            3451 => 2,
            3674 => 0,
            107980 => 1,
            _ => 1,
        }
    }
}
