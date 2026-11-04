// Java: cn.wps.moffice.writer.io.reader.dmlReader.helper.TextShapeType

fn java_hash(value: &str) -> i32 {
    value.encode_utf16().fold(0_i32, |hash, ch| hash.wrapping_mul(31).wrapping_add(ch as i32))
}

// Java: public class TextShapeType
pub struct TextShapeType;

impl TextShapeType {
    // Java: public static int toPrstTextShapeType(int value)
    pub fn toPrstTextShapeType(value: i32) -> i32 {
        match value {
            -1985643931 => 10,
            -1551638040 => 8,
            -1179570471 => 15,
            -1063987874 => 19,
            -1051666275 => 1,
            -1045509333 => 21,
            -1045509332 => 22,
            -1045509330 => 24,
            -1011874710 => 4,
            -1003658545 => 2,
            -923735709 => 6,
            -751876326 => 13,
            -621410733 => 33,
            -459119127 => 31,
            -334639903 => 14,
            -290634075 => 20,
            -97049522 => 5,
            48762713 => 7,
            256638347 => 36,
            256866544 => 34,
            287121917 => 17,
            289839163 => 16,
            542797722 => 23,
            588576258 => 32,
            669547017 => 40,
            705176126 => 26,
            890081730 => 39,
            1045751684 => 18,
            1105389594 => 25,
            1147220699 => 29,
            1217009207 => 30,
            1229534805 => 3,
            1258903667 => 0,
            1267986435 => 38,
            1281979109 => 27,
            1328692796 => 37,
            1334245662 => 9,
            1366164159 => 12,
            1383635037 => 11,
            1461718468 => 35,
            1587249673 => 28,
            _ => 0,
        }
    }

    pub fn toPrstTextShapeType_name(prst: &str) -> i32 {
        // Java: TextShapeType.toPrstTextShapeType(prst.hashCode())
        Self::toPrstTextShapeType(java_hash(prst))
    }
}
