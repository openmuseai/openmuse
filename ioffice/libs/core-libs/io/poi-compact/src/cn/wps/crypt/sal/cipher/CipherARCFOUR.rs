// Java: import cn.wps.base.assertion.IOAssert

use super::Cipher::{Cipher, CipherBase};
use super::CipherDirection;
use super::CipherError;
use super::CipherMode;

const CIPHER_CBLOCK_ARCFOUR: i32 = 256;

// Java: public class CipherARCFOUR extends Cipher
pub struct CipherARCFOUR {
    // Java field: int CIPHER_CBLOCK_ARCFOUR
    base: CipherBase,
    // Java field: int[] mS
    mS: Vec<i32>,
    // Java field: int mX
    mX: i32,
    // Java field: int mY
    mY: i32,
}

// Java: public class CipherARCFOUR extends Cipher
impl CipherARCFOUR {
    // Java: CipherARCFOUR(int mode)
    pub fn new(mode: i32) -> Self {
        // Java: super();
        // Java: this.mS = new int[CIPHER_CBLOCK_ARCFOUR];
        // Java: IOAssert.logTrue("mode should equal to  CipherMode.Cipher_ModeStream!", mode == CipherMode.Cipher_ModeStream);
        assert!(mode == CipherMode::CIPHER_MODE_STREAM, "mode should equal to CipherMode.Cipher_ModeStream!");
        // Java: this.mMode = mode;
        // Java: this.mDirection = CipherDirection.Cipher_DirectionInvalid;
        // Java: this.mAlgorithm = 1;
        CipherARCFOUR {
            base: CipherBase {
                mAlgorithm: 1,
                mDirection: CipherDirection::CIPHER_DIRECTION_INVALID,
                mMode: mode,
            },
            mS: vec![0i32; CIPHER_CBLOCK_ARCFOUR as usize],
            mX: 0,
            mY: 0,
        }
    }

    // Java: int decode(byte[] data, int offset, int nLen, byte[] buffer)
    pub fn decode_impl(&mut self, data: &[u8], offset: i32, nLen: i32, buffer: &mut [u8]) -> i32 {
        // Java: if (this.mAlgorithm != 1)
        if self.base.mAlgorithm != 1 {
            // Java: return CipherError.Cipher_E_Algorithm;
            return CipherError::CIPHER_E_ALGORITHM;
        }
        // Java: else
        else {
            // Java: return this.mDirection == CipherDirection.Cipher_DirectionInvalid ? CipherError.Cipher_E_Direction : this.update(data, offset, nLen, buffer);
            if self.base.mDirection == CipherDirection::CIPHER_DIRECTION_INVALID {
                CipherError::CIPHER_E_DIRECTION
            } else {
                self.update_impl(data, offset, nLen, buffer)
            }
        }
    }

    // Java: void dispose()
    pub fn dispose_impl(&mut self) {
        // Java: for(int x = 0; x < CIPHER_CBLOCK_ARCFOUR; ++x)
        for x in 0..CIPHER_CBLOCK_ARCFOUR as usize {
            // Java: this.mS[x] = 0;
            self.mS[x] = 0;
        }
        // Java: this.mMode = CipherMode.Cipher_ModeInvalid;
        self.base.mMode = CipherMode::CIPHER_MODE_INVALID;
        // Java: this.mDirection = CipherDirection.Cipher_DirectionInvalid;
        self.base.mDirection = CipherDirection::CIPHER_DIRECTION_INVALID;
        // Java: this.mAlgorithm = 1;
        self.base.mAlgorithm = 1;
        // Java: this.mX = 0;
        self.mX = 0;
        // Java: this.mY = 0;
        self.mY = 0;
    }

    // Java: int encode(byte[] data, int offset, int nLen, byte[] buffer)
    pub fn encode_impl(&mut self, data: &[u8], offset: i32, nLen: i32, buffer: &mut [u8]) -> i32 {
        // Java: if (this.mAlgorithm != 1)
        if self.base.mAlgorithm != 1 {
            // Java: return CipherError.Cipher_E_Algorithm;
            return CipherError::CIPHER_E_ALGORITHM;
        }
        // Java: else
        else {
            // Java: return this.mDirection == CipherDirection.Cipher_DirectionInvalid ? CipherError.Cipher_E_Direction : this.update(data, offset, nLen, buffer);
            if self.base.mDirection == CipherDirection::CIPHER_DIRECTION_INVALID {
                CipherError::CIPHER_E_DIRECTION
            } else {
                self.update_impl(data, offset, nLen, buffer)
            }
        }
    }

    // Java: int init(int direction, byte[] keyData)
    pub fn init_impl(&mut self, direction: i32, keyData: &[u8]) -> i32 {
        // Java: if (keyData == null)
        // Java: return CipherError.Cipher_E_Argument;
        // Java: else if (this.mAlgorithm != 1)
        if self.base.mAlgorithm != 1 {
            // Java: return CipherError.Cipher_E_Algorithm;
            return CipherError::CIPHER_E_ALGORITHM;
        }
        // Java: else if (direction == CipherDirection.Cipher_DirectionInvalid)
        else if direction == CipherDirection::CIPHER_DIRECTION_INVALID {
            // Java: return CipherError.Cipher_E_Direction;
            return CipherError::CIPHER_E_DIRECTION;
        }
        // Java: else
        else {
            // Java: this.mDirection = direction;
            self.base.mDirection = direction;
            // Java: int[] K = new int[CIPHER_CBLOCK_ARCFOUR];
            let mut k_arr = vec![0i32; CIPHER_CBLOCK_ARCFOUR as usize];
            // Java: for(int x = 0; x < CIPHER_CBLOCK_ARCFOUR; this.mS[x] = x++)
            for x in 0..CIPHER_CBLOCK_ARCFOUR as usize {
                self.mS[x] = x as i32;
            }
            // Java: int k = 0;
            // Java: int n = 0;
            let mut n: i32;
            // Java: int base = 0;
            let mut base: usize = 0;
            // Java: for(n = CIPHER_CBLOCK_ARCFOUR; n > keyData.length; n -= keyData.length)
            n = CIPHER_CBLOCK_ARCFOUR;
            while n > keyData.len() as i32 {
                // Java: for(int var11 = 0; var11 < keyData.length; ++var11)
                for var11 in 0..keyData.len() {
                    // Java: K[var11 + base] = keyData[var11] & 255;
                    k_arr[var11 + base] = (keyData[var11] as i32) & 255;
                }
                // Java: base += keyData.length;
                base += keyData.len();
                n -= keyData.len() as i32;
            }
            // Java: for(int var12 = 0; var12 < n && var12 < n; ++var12)
            for var12 in 0..n as usize {
                // Java: K[var12 + base] = keyData[var12] & 255;
                k_arr[var12 + base] = (keyData[var12] as i32) & 255;
            }
            // Java: int x = 0;
            // Java: int y = 0;
            // Java: int t = 0;
            let mut t: i32;
            // Java: x = 0;
            // Java: for(int var15 = 0; x < CIPHER_CBLOCK_ARCFOUR; ++x)
            let mut var15: i32 = 0;
            for x in 0..CIPHER_CBLOCK_ARCFOUR as usize {
                // Java: var15 = (var15 + this.mS[x] + K[x]) % CIPHER_CBLOCK_ARCFOUR;
                var15 = (var15 + self.mS[x] + k_arr[x]) % CIPHER_CBLOCK_ARCFOUR;
                // Java: t = this.mS[x];
                t = self.mS[x];
                // Java: this.mS[x] = this.mS[var15];
                self.mS[x] = self.mS[var15 as usize];
                // Java: this.mS[var15] = t;
                self.mS[var15 as usize] = t;
            }
            // Java: this.mX = 0;
            self.mX = 0;
            // Java: this.mY = 0;
            self.mY = 0;
            // Java: return CipherError.Cipher_E_None;
            CipherError::CIPHER_E_NONE
        }
    }

    // Java: int update(byte[] data, int offset, int nLen, byte[] buffer)
    pub fn update_impl(&mut self, data: &[u8], offset: i32, nLen: i32, buffer: &mut [u8]) -> i32 {
        // Java: if (data != null && offset < data.length && nLen >= 0 && offset + nLen <= data.length && buffer != null)
        if (offset as usize) < data.len() && nLen >= 0 && (offset + nLen) as usize <= data.len() {
            // Java: if (nLen > buffer.length)
            if nLen > buffer.len() as i32 {
                // Java: return CipherError.Cipher_E_BufferSize;
                return CipherError::CIPHER_E_BUFFER_SIZE;
            }
            // Java: else
            else {
                // Java: int x = 0;
                let mut x: i32;
                // Java: int y = 0;
                let mut y: i32;
                // Java: int t = 0;
                let mut t: i32;
                // Java: for(int k = 0; k < nLen; ++k)
                for k in 0..nLen as usize {
                    // Java: x = this.mX;
                    x = self.mX;
                    // Java: y = this.mY;
                    y = self.mY;
                    // Java: x = (x + 1) % CIPHER_CBLOCK_ARCFOUR;
                    x = (x + 1) % CIPHER_CBLOCK_ARCFOUR;
                    // Java: y = (y + this.mS[x]) % CIPHER_CBLOCK_ARCFOUR;
                    y = (y + self.mS[x as usize]) % CIPHER_CBLOCK_ARCFOUR;
                    // Java: this.mX = x;
                    self.mX = x;
                    // Java: this.mY = y;
                    self.mY = y;
                    // Java: t = this.mS[x];
                    t = self.mS[x as usize];
                    // Java: this.mS[x] = this.mS[y];
                    self.mS[x as usize] = self.mS[y as usize];
                    // Java: this.mS[y] = t;
                    self.mS[y as usize] = t;
                    // Java: t = (this.mS[x] + this.mS[y]) % CIPHER_CBLOCK_ARCFOUR;
                    t = (self.mS[x as usize] + self.mS[y as usize]) % CIPHER_CBLOCK_ARCFOUR;
                    // Java: buffer[k] = (byte)(data[k + offset] & 255 ^ this.mS[t] & 255);
                    buffer[k] = ((data[k + offset as usize] as i32 & 255) ^ (self.mS[t as usize] & 255)) as u8;
                }
                // Java: return CipherError.Cipher_E_None;
                CipherError::CIPHER_E_NONE
            }
        }
        // Java: else
        else {
            // Java: return CipherError.Cipher_E_Argument;
            CipherError::CIPHER_E_ARGUMENT
        }
    }
}

impl Cipher for CipherARCFOUR {
    fn init(&mut self, var1: i32, var2: &[u8]) -> i32 {
        self.init_impl(var1, var2)
    }

    fn update(&mut self, var1: &[u8], var2: i32, var3: i32, var4: &mut [u8]) -> i32 {
        self.update_impl(var1, var2, var3, var4)
    }

    fn encode(&mut self, var1: &[u8], var2: i32, var3: i32, var4: &mut [u8]) -> i32 {
        self.encode_impl(var1, var2, var3, var4)
    }

    fn decode(&mut self, var1: &[u8], var2: i32, var3: i32, var4: &mut [u8]) -> i32 {
        self.decode_impl(var1, var2, var3, var4)
    }

    fn dispose(&mut self) {
        self.dispose_impl()
    }
}
