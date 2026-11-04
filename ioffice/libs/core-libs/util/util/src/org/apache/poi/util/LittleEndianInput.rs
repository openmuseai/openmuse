// Java: public interface LittleEndianInput
pub trait LittleEndianInput {
    // Java: abstract int available()
    fn available(&self) -> i32;
    // Java: abstract byte readByte()
    fn readByte(&mut self) -> i8;
    // Java: abstract int readUByte()
    fn readUByte(&mut self) -> i32;
    // Java: abstract short readShort()
    fn readShort(&mut self) -> i16;
    // Java: abstract int readUShort()
    fn readUShort(&mut self) -> i32;
    // Java: abstract int readInt()
    fn readInt(&mut self) -> i32;
    // Java: abstract long readLong()
    fn readLong(&mut self) -> i64;
    // Java: abstract double readDouble()
    fn readDouble(&mut self) -> f64;
    // Java: abstract void readFully(_var1: &[u8])
    fn readFully(&mut self, var1: &mut [u8]);
    // Java: abstract void readFully(_var1: &[u8], int var2, int var3)
    fn readFully_offset(&mut self, var1: &mut [u8], var2: usize, var3: usize);
    // Java: abstract long skip(long var1)
    fn skip(&mut self, var1: i64) -> i64;
}
