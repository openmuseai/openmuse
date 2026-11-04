#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.io.ByteArrayInputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.PrintStream
// Java: import java.util.zip.InflaterInputStream
// Java: import org.apache.poi.util.HexDump
// Java: import org.apache.poi.util.LittleEndian

// Java: public class EscherDump
pub struct EscherDump;

// Java: public class EscherDump
impl EscherDump {
    // Java: EscherDump()
    pub fn new() -> Self { EscherDump }
    // Java: void dumpOld(long maxLength, InputStream in, PrintStream out)
    pub fn dumpOld(&self, _max_length: i64, _input: &mut dyn std::io::Read, _out: &mut String) {}
    // Java: String propName(short propertyId)
    pub fn propName(&self, _property_id: i16) -> String { String::from("unknown property") }
    // Java: static String getBlipType(byte b)
    pub fn getBlipType(_b: i8) -> String { String::new() }
    // Java: String dec1616(int n32)
    pub fn dec1616(&self, n32: i32) -> String { format!("{}.{}", (n32 >> 16) as i16, (n32 & -1) as i16) }
    // Java: void outHex(int bytes, InputStream in, PrintStream out)
    pub fn outHex(&self, _bytes: i32, _input: &mut dyn std::io::Read, _out: &mut String) {}
    // Java: static void main(String[] args)
    pub fn main() {}
    // Java: void dump(int recordSize, byte[] data, PrintStream out)
    pub fn dump(&self, _record_size: i32, _data: &[u8], _out: &mut String) {}
}
