use super::CipherParameters::CipherParameters;

// Java: public interface StreamCipher
pub trait StreamCipher {
    // Java: abstract void init(boolean var1, CipherParameters var2)
    fn init(&mut self, var1: bool, var2: &dyn CipherParameters);

    // Java: abstract String getAlgorithmName()
    fn get_algorithm_name(&self) -> String;

    // Java: abstract byte returnByte(byte var1)
    fn return_byte(&mut self, var1: u8) -> u8;

    // Java: abstract void processBytes(byte[] var1, int var2, int var3, byte[] var4, int var5)
    fn process_bytes(&mut self, var1: &[u8], var2: i32, var3: i32, var4: &mut [u8], var5: i32);

    // Java: abstract void reset()
    fn reset(&mut self);
}
