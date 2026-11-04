// Java: import cn.wps.io.uof.crypt.crypto.BlockCipher
use crate::cn::wps::io::uof::crypt::crypto::BlockCipher::BlockCipher;
// Java: import cn.wps.io.uof.crypt.crypto.CipherParameters
use crate::cn::wps::io::uof::crypt::crypto::CipherParameters::CipherParameters;
// Java: import cn.wps.io.uof.crypt.crypto.DataLengthException
use crate::cn::wps::io::uof::crypt::crypto::DataLengthException::DataLengthException;
// Java: import cn.wps.io.uof.crypt.crypto.OutputLengthException
use crate::cn::wps::io::uof::crypt::crypto::OutputLengthException::OutputLengthException;
// Java: import cn.wps.io.uof.crypt.crypto.params.KeyParameter
use crate::cn::wps::io::uof::crypt::crypto::params::KeyParameter::KeyParameter;
// Java: import cn.wps.io.uof.crypt.util.Pack
use crate::cn::wps::io::uof::crypt::util::Pack::Pack;

// Java field: int BLOCK_SIZE
pub(crate) const BLOCK_SIZE: i32 = 16;
// Java field: byte[] SBOX
const SBOX: [u8; 256] = [
    214, 144, 233, 254, 204, 225, 61, 183, 22, 182, 20, 194, 40, 251, 44, 5,
    43, 103, 154, 118, 42, 190, 4, 195, 170, 68, 19, 38, 73, 134, 6, 153,
    156, 66, 80, 244, 145, 239, 152, 122, 51, 84, 11, 67, 237, 207, 172, 98,
    228, 179, 28, 169, 201, 8, 232, 149, 128, 223, 148, 250, 117, 143, 63, 166,
    71, 7, 167, 252, 243, 115, 23, 186, 131, 89, 60, 25, 230, 133, 79, 168,
    104, 107, 129, 178, 113, 100, 218, 139, 248, 235, 15, 75, 112, 86, 157, 53,
    30, 36, 14, 94, 99, 88, 209, 162, 37, 34, 124, 59, 1, 33, 120, 135,
    212, 0, 70, 87, 159, 211, 39, 82, 76, 54, 2, 231, 160, 196, 200, 158,
    234, 191, 138, 210, 64, 199, 56, 181, 163, 247, 242, 206, 249, 97, 21, 161,
    224, 174, 93, 164, 155, 52, 26, 85, 173, 147, 50, 48, 245, 140, 177, 227,
    29, 246, 226, 46, 130, 102, 202, 96, 192, 41, 35, 171, 13, 83, 78, 111,
    213, 219, 55, 69, 222, 253, 142, 47, 3, 255, 106, 114, 109, 108, 91, 81,
    141, 27, 175, 146, 187, 221, 188, 127, 17, 217, 92, 65, 31, 16, 90, 216,
    10, 193, 49, 136, 165, 205, 123, 189, 45, 116, 208, 18, 184, 229, 180, 176,
    137, 105, 151, 74, 12, 150, 119, 126, 101, 185, 241, 9, 197, 110, 198, 132,
    24, 240, 125, 236, 58, 220, 77, 32, 121, 238, 95, 62, 215, 203, 57, 72,
];
// Java field: int[] CK
const CK: [i32; 32] = [
    462357, 472066609, 943670861, 1415275113, 1886879365,
    -1936483679i32, -1464879427i32, -993275175i32, -521670923i32, -66909679i32,
    404694573, 876298825, 1347903077, 1819507329,
    -2003855715i32, -1532251463i32, -1060647211i32, -589042959i32, -117504499i32,
    337322537, 808926789, 1280531041, 1752135293,
    -2071227751i32, -1599623499i32, -1128019247i32, -656414995i32, -184876535i32,
    269950501, 741554753, 1213159005, 1684763257,
];
// Java field: int[] FK
const FK: [i32; 4] = [-1548633402i32, 1453994832, 1736282519, -1301273892i32];

// Java: public class SM4Engine implements BlockCipher
pub struct SM4Engine {
    // Java field: int[] mXArray
    pub(crate) mXArray: Vec<i32>,
    // Java field: int[] mRk
    pub(crate) mRk: Option<Vec<i32>>,
}

// Java: public class SM4Engine implements BlockCipher
impl SM4Engine {
    // Java: SM4Engine()
    pub fn SM4Engine() -> Self {
      // Java: super();
      SM4Engine {
          mXArray: vec![0i32; 4],
          mRk: None,
      }
    }

    // Java: int rotateLeft(int var1, int var2)
    fn rotateLeft(var1: i32, var2: i32) -> i32 {
      // Java: return var1 << var2 | var1 >>> -var2;
      (var1 as u32).rotate_left(var2 as u32) as i32
    }

    // Java: int tau(int var1)
    fn tau(var1: i32) -> i32 {
      // Java: int var2 = SBOX[var1 >> 24 & 255] & 255;
      let var2 = SBOX[((var1 >> 24) & 255) as usize] as i32;
      // Java: int var3 = SBOX[var1 >> 16 & 255] & 255;
      let var3 = SBOX[((var1 >> 16) & 255) as usize] as i32;
      // Java: int var4 = SBOX[var1 >> 8 & 255] & 255;
      let var4 = SBOX[((var1 >> 8) & 255) as usize] as i32;
      // Java: int var5 = SBOX[var1 & 255] & 255;
      let var5 = SBOX[(var1 & 255) as usize] as i32;
      // Java: return var2 << 24 | var3 << 16 | var4 << 8 | var5;
      (var2 << 24) | (var3 << 16) | (var4 << 8) | var5
    }

    // Java: int lap(int var1)
    fn lap(var1: i32) -> i32 {
      // Java: return var1 ^ this.rotateLeft(var1, 13) ^ this.rotateLeft(var1, 23);
      var1 ^ SM4Engine::rotateLeft(var1, 13) ^ SM4Engine::rotateLeft(var1, 23)
    }

    // Java: int tap(int var1)
    fn tap(var1: i32) -> i32 {
      // Java: return this.lap(this.tau(var1));
      SM4Engine::lap(SM4Engine::tau(var1))
    }

    // Java: int[] expandKey(boolean var1, byte[] var2)
    fn expandKey(var1: bool, var2: &[u8]) -> Vec<i32> {
      // Java: int[] var3 = new int[32];
      let mut var3 = vec![0i32; 32];
      // Java: int[] var4 = new int[]{Pack.bigEndianToInt(var2, 0), Pack.bigEndianToInt(var2, 4), Pack.bigEndianToInt(var2, 8), Pack.bigEndianToInt(var2, 12)};
      let var4 = [
          Pack::bigEndianToInt(var2, 0),
          Pack::bigEndianToInt(var2, 4),
          Pack::bigEndianToInt(var2, 8),
          Pack::bigEndianToInt(var2, 12),
      ];
      // Java: int[] var6 = new int[]{var4[0] ^ FK[0], var4[1] ^ FK[1], var4[2] ^ FK[2], var4[3] ^ FK[3]};
      let var6 = [
          var4[0] ^ FK[0],
          var4[1] ^ FK[1],
          var4[2] ^ FK[2],
          var4[3] ^ FK[3],
      ];
      // Java: if (var1)
      if var1 {
         // Java: var3[0] = var6[0] ^ this.tap(var6[1] ^ var6[2] ^ var6[3] ^ CK[0]);
         var3[0] = var6[0] ^ SM4Engine::tap(var6[1] ^ var6[2] ^ var6[3] ^ CK[0]);
         // Java: var3[1] = var6[1] ^ this.tap(var6[2] ^ var6[3] ^ var3[0] ^ CK[1]);
         var3[1] = var6[1] ^ SM4Engine::tap(var6[2] ^ var6[3] ^ var3[0] ^ CK[1]);
         // Java: var3[2] = var6[2] ^ this.tap(var6[3] ^ var3[0] ^ var3[1] ^ CK[2]);
         var3[2] = var6[2] ^ SM4Engine::tap(var6[3] ^ var3[0] ^ var3[1] ^ CK[2]);
         // Java: var3[3] = var6[3] ^ this.tap(var3[0] ^ var3[1] ^ var3[2] ^ CK[3]);
         var3[3] = var6[3] ^ SM4Engine::tap(var3[0] ^ var3[1] ^ var3[2] ^ CK[3]);
         // Java: for(int var5 = 4; var5 < 32; ++var5)
         for var5 in 4..32usize {
            // Java: var3[var5] = var3[var5 - 4] ^ this.tap(var3[var5 - 3] ^ var3[var5 - 2] ^ var3[var5 - 1] ^ CK[var5]);
            var3[var5] = var3[var5 - 4] ^ SM4Engine::tap(var3[var5 - 3] ^ var3[var5 - 2] ^ var3[var5 - 1] ^ CK[var5]);
         }
      }
      // Java: else
      else {
         // Java: var3[31] = var6[0] ^ this.tap(var6[1] ^ var6[2] ^ var6[3] ^ CK[0]);
         var3[31] = var6[0] ^ SM4Engine::tap(var6[1] ^ var6[2] ^ var6[3] ^ CK[0]);
         // Java: var3[30] = var6[1] ^ this.tap(var6[2] ^ var6[3] ^ var3[31] ^ CK[1]);
         var3[30] = var6[1] ^ SM4Engine::tap(var6[2] ^ var6[3] ^ var3[31] ^ CK[1]);
         // Java: var3[29] = var6[2] ^ this.tap(var6[3] ^ var3[31] ^ var3[30] ^ CK[2]);
         var3[29] = var6[2] ^ SM4Engine::tap(var6[3] ^ var3[31] ^ var3[30] ^ CK[2]);
         // Java: var3[28] = var6[3] ^ this.tap(var3[31] ^ var3[30] ^ var3[29] ^ CK[3]);
         var3[28] = var6[3] ^ SM4Engine::tap(var3[31] ^ var3[30] ^ var3[29] ^ CK[3]);
         // Java: for(int var5 = 27; var5 >= 0; --var5)
         for var5 in (0..=27usize).rev() {
            // Java: var3[var5] = var3[var5 + 4] ^ this.tap(var3[var5 + 3] ^ var3[var5 + 2] ^ var3[var5 + 1] ^ CK[31 - var5]);
            var3[var5] = var3[var5 + 4] ^ SM4Engine::tap(var3[var5 + 3] ^ var3[var5 + 2] ^ var3[var5 + 1] ^ CK[31 - var5]);
         }
      }
      // Java: return var3;
      var3
    }

    // Java: int l(int var1)
    fn l(var1: i32) -> i32 {
      // Java: int var2 = var1 ^ this.rotateLeft(var1, 2) ^ this.rotateLeft(var1, 10) ^ this.rotateLeft(var1, 18) ^ this.rotateLeft(var1, 24);
      let var2 = var1 ^ SM4Engine::rotateLeft(var1, 2) ^ SM4Engine::rotateLeft(var1, 10) ^ SM4Engine::rotateLeft(var1, 18) ^ SM4Engine::rotateLeft(var1, 24);
      // Java: return var2;
      var2
    }

    // Java: int t(int var1)
    fn t(var1: i32) -> i32 {
      // Java: return this.l(this.tau(var1));
      SM4Engine::l(SM4Engine::tau(var1))
    }

    // Java: int f0(int[] var1, int var2)
    fn f0(var1: &[i32], var2: i32) -> i32 {
      // Java: return var1[0] ^ this.t(var1[1] ^ var1[2] ^ var1[3] ^ var2);
      var1[0] ^ SM4Engine::t(var1[1] ^ var1[2] ^ var1[3] ^ var2)
    }

    // Java: int f1(int[] var1, int var2)
    fn f1(var1: &[i32], var2: i32) -> i32 {
      // Java: return var1[1] ^ this.t(var1[2] ^ var1[3] ^ var1[0] ^ var2);
      var1[1] ^ SM4Engine::t(var1[2] ^ var1[3] ^ var1[0] ^ var2)
    }

    // Java: int f2(int[] var1, int var2)
    fn f2(var1: &[i32], var2: i32) -> i32 {
      // Java: return var1[2] ^ this.t(var1[3] ^ var1[0] ^ var1[1] ^ var2);
      var1[2] ^ SM4Engine::t(var1[3] ^ var1[0] ^ var1[1] ^ var2)
    }

    // Java: int f3(int[] var1, int var2)
    fn f3(var1: &[i32], var2: i32) -> i32 {
      // Java: return var1[3] ^ this.t(var1[0] ^ var1[1] ^ var1[2] ^ var2);
      var1[3] ^ SM4Engine::t(var1[0] ^ var1[1] ^ var1[2] ^ var2)
    }

    // Java: void init(boolean var1, CipherParameters var2)
    pub fn init(&mut self, var1: bool, var2: &KeyParameter) {
      // Java: if (var2 instanceof KeyParameter)
      {
         // Java: byte[] var3 = ((KeyParameter)var2).getKey();
         let var3 = var2.getKey();
         // Java: if (var3.length != 16)
         if var3.len() != 16 {
            // Java: throw new IllegalArgumentException("SM4 requires a 128 bit key");
            panic!("SM4 requires a 128 bit key");
         }
         // Java: else
         else {
            // Java: this.mRk = this.expandKey(var1, var3);
            self.mRk = Some(SM4Engine::expandKey(var1, var3));
         }
      }
      // Java: else
      // Java: throw new IllegalArgumentException("invalid parameter passed to SM4 init - " + var2.getClass().getName());
    }

    // Java: String getAlgorithmName()
    pub fn getAlgorithmName(&self) -> String {
      // Java: return "SM4";
      "SM4".to_string()
    }

    // Java: int getBlockSize()
    pub fn getBlockSize(&self) -> i32 {
      // Java: return 16;
      16
    }

    // Java: int processBlock(byte[] var1, int var2, byte[] var3, int var4)
    pub fn processBlock(&mut self, var1: &[u8], var2: i32, var3: &mut [u8], var4: i32) -> i32 {
      // Java: if (this.mRk == null)
      if self.mRk.is_none() {
         // Java: throw new IllegalStateException("SM4 not initialised");
         panic!("SM4 not initialised");
      }
      // Java: else if (var2 + 16 > var1.length)
      else if (var2 + 16) as usize > var1.len() {
         // Java: throw new DataLengthException("input buffer too short");
         panic!("input buffer too short");
      }
      // Java: else if (var4 + 16 > var3.length)
      else if (var4 + 16) as usize > var3.len() {
         // Java: throw new OutputLengthException("output buffer too short");
         panic!("output buffer too short");
      }
      // Java: else
      else {
         // Java: this.mXArray[0] = Pack.bigEndianToInt(var1, var2);
         self.mXArray[0] = Pack::bigEndianToInt(var1, var2);
         // Java: this.mXArray[1] = Pack.bigEndianToInt(var1, var2 + 4);
         self.mXArray[1] = Pack::bigEndianToInt(var1, var2 + 4);
         // Java: this.mXArray[2] = Pack.bigEndianToInt(var1, var2 + 8);
         self.mXArray[2] = Pack::bigEndianToInt(var1, var2 + 8);
         // Java: this.mXArray[3] = Pack.bigEndianToInt(var1, var2 + 12);
         self.mXArray[3] = Pack::bigEndianToInt(var1, var2 + 12);
         let rk = self.mRk.as_ref().unwrap().clone();
         // Java: for(int var5 = 0; var5 < 32; var5 += 4)
         for var5 in (0..32usize).step_by(4) {
            // Java: this.mXArray[0] = this.f0(this.mXArray, this.mRk[var5]);
            self.mXArray[0] = SM4Engine::f0(&self.mXArray, rk[var5]);
            // Java: this.mXArray[1] = this.f1(this.mXArray, this.mRk[var5 + 1]);
            self.mXArray[1] = SM4Engine::f1(&self.mXArray, rk[var5 + 1]);
            // Java: this.mXArray[2] = this.f2(this.mXArray, this.mRk[var5 + 2]);
            self.mXArray[2] = SM4Engine::f2(&self.mXArray, rk[var5 + 2]);
            // Java: this.mXArray[3] = this.f3(this.mXArray, this.mRk[var5 + 3]);
            self.mXArray[3] = SM4Engine::f3(&self.mXArray, rk[var5 + 3]);
         }
         // Java: Pack.intToBigEndian(this.mXArray[3], var3, var4);
         Pack::intToBigEndian_into(self.mXArray[3], var3, var4);
         // Java: Pack.intToBigEndian(this.mXArray[2], var3, var4 + 4);
         Pack::intToBigEndian_into(self.mXArray[2], var3, var4 + 4);
         // Java: Pack.intToBigEndian(this.mXArray[1], var3, var4 + 8);
         Pack::intToBigEndian_into(self.mXArray[1], var3, var4 + 8);
         // Java: Pack.intToBigEndian(this.mXArray[0], var3, var4 + 12);
         Pack::intToBigEndian_into(self.mXArray[0], var3, var4 + 12);
         // Java: return 16;
         16
      }
    }

    // Java: void reset()
    pub fn reset(&mut self) {
    }

}

impl BlockCipher for SM4Engine {
    fn init(&mut self, var1: bool, var2: &dyn CipherParameters) {
        let kp = var2.as_any().downcast_ref::<KeyParameter>()
            .expect("SM4Engine requires KeyParameter");
        self.init(var1, kp);
    }
    fn getAlgorithmName(&self) -> String { self.getAlgorithmName() }
    fn getBlockSize(&self) -> i32 { self.getBlockSize() }
    fn processBlock(&mut self, var1: &[u8], var2: i32, var3: &mut [u8], var4: i32) -> i32 {
        self.processBlock(var1, var2, var3, var4)
    }
    fn reset(&mut self) { self.reset() }
}
