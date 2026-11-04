use super::LittleEndianOutput::LittleEndianOutput;

// Java: public interface DelayableLittleEndianOutput extends LittleEndianOutput
pub trait DelayableLittleEndianOutput: LittleEndianOutput {
    // Java: abstract LittleEndianOutput createDelayedOutput(int var1)
    fn createDelayedOutput(&mut self, var1: i32) -> Box<dyn LittleEndianOutput>;
}
