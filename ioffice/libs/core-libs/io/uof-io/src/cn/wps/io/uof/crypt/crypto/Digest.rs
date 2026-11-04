// Java: public interface Digest
pub trait Digest {
    // Java: abstract String getAlgorithmName()
    fn getAlgorithmName(&self) -> String;

    // Java: abstract int getDigestSize()
    fn getDigestSize(&self) -> i32;

    // Java: abstract void update(byte var1)
    fn update_byte(&mut self, var1: u8);

    // Java: abstract void update(byte[] var1, int var2, int var3)
    fn update(&mut self, var1: &[u8], var2: i32, var3: i32);

    // Java: abstract int doFinal(byte[] var1, int var2)
    fn doFinal(&mut self, var1: &mut [u8], var2: i32) -> i32;

    // Java: abstract void reset()
    fn reset(&mut self);

}
