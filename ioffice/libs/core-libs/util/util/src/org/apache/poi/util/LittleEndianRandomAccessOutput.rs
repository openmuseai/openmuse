use super::DelayableLittleEndianOutput::DelayableLittleEndianOutput;

// Java: public interface LittleEndianRandomAccessOutput extends DelayableLittleEndianOutput
pub trait LittleEndianRandomAccessOutput: DelayableLittleEndianOutput {
    // Java: abstract long tell()
    fn tell(&self) -> i64;

    // Java: abstract long length()
    fn length(&self) -> i64;

    // Java: abstract long seek(long var1)
    fn seek(&mut self, var1: i64) -> i64;
}
