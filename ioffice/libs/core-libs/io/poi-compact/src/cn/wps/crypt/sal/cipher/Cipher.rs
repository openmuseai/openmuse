// Java: public class Cipher
pub struct CipherBase {
    // Java field: int mAlgorithm
    pub(crate) mAlgorithm: i32,
    // Java field: int mDirection
    pub(crate) mDirection: i32,
    // Java field: int mMode
    pub(crate) mMode: i32,
}

// Java: public class Cipher
impl CipherBase {
    // Java: Cipher()
    pub fn new() -> Self {
        // Java: super();
        CipherBase {
            mAlgorithm: 0,
            mDirection: 0,
            mMode: 0,
        }
    }
}

// rust: owned cipher buffers travel with the locked ProviderInfo graph.
pub trait Cipher:Send+Sync {
    // Java: abstract int init(int var1, byte[] var2)
    fn init(&mut self, var1: i32, var2: &[u8]) -> i32;

    // Java: abstract int update(byte[] var1, int var2, int var3, byte[] var4)
    fn update(&mut self, var1: &[u8], var2: i32, var3: i32, var4: &mut [u8]) -> i32;

    // Java: abstract int encode(byte[] var1, int var2, int var3, byte[] var4)
    fn encode(&mut self, var1: &[u8], var2: i32, var3: i32, var4: &mut [u8]) -> i32;

    // Java: abstract int decode(byte[] var1, int var2, int var3, byte[] var4)
    fn decode(&mut self, var1: &[u8], var2: i32, var3: i32, var4: &mut [u8]) -> i32;

    // Java: abstract void dispose()
    fn dispose(&mut self);
}
