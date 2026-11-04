// Java: public interface XmlRoAttrValue
pub trait XmlRoAttrValue {
    // Java: abstract String toString()
    fn to_string(&self) -> String;

    // Java: abstract int toInt()
    fn to_int(&self) -> i32;

    // Java: abstract byte toByte()
    fn to_byte(&self) -> i8;

    // Java: abstract short toShort()
    fn to_short(&self) -> i16;

    // Java: abstract boolean toBoolean()
    fn to_boolean(&self) -> bool;

    // Java: abstract float toFloat()
    fn to_float(&self) -> f32;

    // Java: abstract double toDouble()
    fn to_double(&self) -> f64;

    // Java: abstract long toLong()
    fn to_long(&self) -> i64;

    // Java: abstract double toPercentage()
    fn to_percentage(&self) -> f64;
}
