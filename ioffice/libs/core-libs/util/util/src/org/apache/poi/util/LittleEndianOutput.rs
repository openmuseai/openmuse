// Java: public interface LittleEndianOutput
pub trait LittleEndianOutput {
    // Java: abstract void writeByte(int var1)
    fn writeByte(&mut self, var1: i32);

    // Java: abstract void writeShort(int var1)
    fn writeShort(&mut self, var1: i32);

    // Java: abstract void writeInt(int var1)
    fn writeInt(&mut self, var1: i32);

    // Java: abstract void writeLong(long var1)
    fn writeLong(&mut self, var1: i64);

    // Java: abstract void writeDouble(double var1)
    fn writeDouble(&mut self, var1: f64);

    // Java: abstract void write(byte[] var1)
    fn write(&mut self, var1: &[u8]);

    // Java: abstract void write(byte[] var1, int var2, int var3)
    fn write_with_offset(&mut self, var1: &[u8], var2: i32, var3: i32);
}
