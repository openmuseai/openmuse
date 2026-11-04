// Java: import cn.wps.io.uof.crypt.crypto.CipherParameters
use crate::cn::wps::io::uof::crypt::crypto::CipherParameters::CipherParameters;

// Java: public class KeyParameter implements CipherParameters
pub struct KeyParameter {
    // Java field: byte[] mKey
    pub(crate) mKey: Vec<u8>,
}

// Java: public class KeyParameter implements CipherParameters
impl KeyParameter {
    // Java: KeyParameter(byte[] var1)
    pub fn KeyParameter(var1: &[u8]) -> Self {
      // Java: this(var1, 0, var1.length);
      Self::KeyParameter_range(var1, 0, var1.len() as i32)
    }

    // Java: KeyParameter(byte[] var1, int var2, int var3)
    pub fn KeyParameter_range(var1: &[u8], var2: i32, var3: i32) -> Self {
      // Java: super();
      // Java: this.mKey = new byte[var3];
      let mut mKey = vec![0u8; var3 as usize];
      // Java: System.arraycopy(var1, var2, this.mKey, 0, var3);
      mKey[..var3 as usize].copy_from_slice(&var1[var2 as usize..(var2 + var3) as usize]);
      KeyParameter { mKey }
    }

    // Java: byte[] getKey()
    pub fn getKey(&self) -> &[u8] {
      // Java: return this.mKey;
      &self.mKey
    }

}

impl CipherParameters for KeyParameter {
    fn as_any(&self) -> &dyn std::any::Any { self }
}
