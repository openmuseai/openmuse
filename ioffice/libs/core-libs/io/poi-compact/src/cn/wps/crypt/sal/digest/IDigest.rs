// Java: public interface IDigest
// rust: digest instances contain owned buffers; Java permits transferring the
// provider graph between threads. Mutation still requires the owning graph lock.
pub trait IDigest:Send+Sync {
    // Java: abstract int init(byte[] var1, int var2, int var3)
    fn init(&mut self, var1: &[u8], var2: i32, var3: i32) -> i32;

    // Java: abstract int update(byte[] var1, int var2, int var3)
    fn update(&mut self, var1: &[u8], var2: i32, var3: i32) -> i32;

    // Java: abstract int get(byte[] var1)
    fn get(&mut self, var1: &mut [u8]) -> i32;

    // Java: abstract int get(byte[] var1, int var2, int var3, byte[] var4)
    fn get_with_params(&mut self, var1: &[u8], var2: i32, var3: i32, var4: &mut [u8]) -> i32;

    // Java: abstract int raw(byte[] var1)
    fn raw(&mut self, var1: &mut [u8]) -> i32;

    // Java: abstract int getLength()
    fn get_length(&self) -> i32;

    // Java: abstract void dispose()
    fn dispose(&mut self);
}
