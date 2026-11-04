use super::CipherParameters::CipherParameters;

// Java: public interface BlockCipher
pub trait BlockCipher {
    // Java: abstract void init(boolean var1, CipherParameters var2)
    fn init(&mut self, var1: bool, var2: &dyn CipherParameters);

    // Java: abstract String getAlgorithmName()
    fn getAlgorithmName(&self) -> String;

    // Java: abstract int getBlockSize()
    fn getBlockSize(&self) -> i32;

    // Java: abstract int processBlock(byte[] var1, int var2, byte[] var3, int var4)
    fn processBlock(&mut self, var1: &[u8], var2: i32, var3: &mut [u8], var4: i32) -> i32;

    // Java: abstract void reset()
    fn reset(&mut self);

}
