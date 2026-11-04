// Java: import cn.wps.io.uof.crypt.crypto.CipherParameters
use crate::cn::wps::io::uof::crypt::crypto::CipherParameters::CipherParameters;

// Java: public class ParametersWithIV implements CipherParameters
pub struct ParametersWithIV {
    // Java field: byte[] mIv
    pub(crate) mIv: Vec<u8>,
    // Java field: CipherParameters mParameters
    pub(crate) mParameters: Box<dyn CipherParameters>,
}

// Java: public class ParametersWithIV implements CipherParameters
impl ParametersWithIV {
    // Java: ParametersWithIV(CipherParameters var1, byte[] var2)
    pub fn ParametersWithIV(var1: Box<dyn CipherParameters>, var2: &[u8]) -> Self {
      // Java: this(var1, var2, 0, var2.length);
      Self::ParametersWithIV_range(var1, var2, 0, var2.len() as i32)
    }

    // Java: ParametersWithIV(CipherParameters var1, byte[] var2, int var3, int var4)
    pub fn ParametersWithIV_range(var1: Box<dyn CipherParameters>, var2: &[u8], var3: i32, var4: i32) -> Self {
      // Java: super();
      // Java: this.mIv = new byte[var4];
      let mut mIv = vec![0u8; var4 as usize];
      // Java: this.mParameters = var1;
      // Java: System.arraycopy(var2, var3, this.mIv, 0, var4);
      mIv[..var4 as usize].copy_from_slice(&var2[var3 as usize..(var3 + var4) as usize]);
      ParametersWithIV { mIv, mParameters: var1 }
    }

    // Java: byte[] getIV()
    pub fn getIV(&self) -> &[u8] {
      // Java: return this.mIv;
      &self.mIv
    }

    // Java: CipherParameters getParameters()
    pub fn getParameters(&self) -> &dyn CipherParameters {
      // Java: return this.mParameters;
      &*self.mParameters
    }

}

impl CipherParameters for ParametersWithIV {
    fn as_any(&self) -> &dyn std::any::Any { self }
}
