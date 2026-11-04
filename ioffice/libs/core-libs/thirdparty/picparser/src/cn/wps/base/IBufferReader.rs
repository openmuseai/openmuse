#![allow(non_snake_case, dead_code)]

// Java: public interface IBufferReader
pub trait IBufferReader {
    // Java: abstract void close()
    fn close(&mut self);

    // Java: abstract void reset()
    fn reset(&mut self);

    // Java: abstract void skip(int var1)
    fn skip(&mut self, var1: i32);

    // Java: abstract int available()
    fn available(&self) -> i32;

    // Java: abstract int getCurrentPos()
    fn getCurrentPos(&self) -> i32;

    // Java: abstract void setCurrentPos(int var1)
    fn setCurrentPos(&mut self, var1: i32);

    // Java: abstract boolean readBoolean()
    fn readBoolean(&mut self) -> bool;

    // Java: abstract byte readByte()
    fn readByte_one(&mut self) -> i8;

    // Java: abstract int readUnsignedByte()
    fn readUnsignedByte_one(&mut self) -> i32;

    // Java: abstract byte[] readByte(int var1)
    fn readByte_n(&mut self, var1: i32) -> Vec<u8>;

    // Java: abstract int[] readUnsignedByte(int var1)
    fn readUnsignedByte_n(&mut self, var1: i32) -> Vec<i32>;

    // Java: abstract short readShort()
    fn readShort_one(&mut self) -> i16;

    // Java: abstract short readShortBE()
    fn readShortBE(&mut self) -> i16;

    // Java: abstract int readUnsignedShort()
    fn readUnsignedShort(&mut self) -> i32;

    // Java: abstract int readUnsignedShortBE()
    fn readUnsignedShortBE(&mut self) -> i32;

    // Java: abstract int[] readShort(int var1)
    fn readShort_n(&mut self, var1: i32) -> Vec<i32>;

    // Java: abstract int readInt()
    fn readInt_one(&mut self) -> i32;

    // Java: abstract int readIntBE()
    fn readIntBE(&mut self) -> i32;

    // Java: abstract int[] readInt(int var1)
    fn readInt_n(&mut self, var1: i32) -> Vec<i32>;

    // Java: abstract int readUnsignedInt()
    fn readUnsignedInt_one(&mut self) -> i32;

    // Java: abstract int readUnsignedIntBE()
    fn readUnsignedIntBE(&mut self) -> i32;

    // Java: abstract int[] readUnsignedInt(int var1)
    fn readUnsignedInt_n(&mut self, var1: i32) -> Vec<i32>;

    // Java: abstract long readLong()
    fn readLong(&mut self) -> i64;

    // Java: abstract long readLongBE()
    fn readLongBE(&mut self) -> i64;

    // Java: abstract float readFloat()
    fn readFloat(&mut self) -> f32;

    // Java: abstract float readFloatBE()
    fn readFloatBE(&mut self) -> f32;

    // Java: abstract double readDouble()
    fn readDouble(&mut self) -> f64;

    // Java: abstract double readDoubleBE()
    fn readDoubleBE(&mut self) -> f64;

    // Java: abstract void read(byte[] var1)
    fn read_all(&mut self, var1: &mut [u8]);

    // Java: abstract void read(byte[] var1, int var2)
    fn read_n(&mut self, var1: &mut [u8], var2: i32);

    // Java: abstract String readString(int var1)
    fn readString(&mut self, var1: i32) -> String;

    // Java: abstract int readColor()
    fn readColor(&mut self) -> i32;

    // Java: abstract int readColorRe()
    fn readColorRe(&mut self) -> i32;

    // Java: abstract int readRGB()
    fn readRGB(&mut self) -> i32;

    // Java: abstract int readBGR()
    fn readBGR(&mut self) -> i32;

    // Java: abstract int readARGB()
    fn readARGB(&mut self) -> i32;

    // Java: abstract int readBGRA()
    fn readBGRA(&mut self) -> i32;
}
