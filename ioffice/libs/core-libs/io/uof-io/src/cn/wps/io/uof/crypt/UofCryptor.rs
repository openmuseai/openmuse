// Java: import cn.wps.io.uof.crypt.crypto.BufferedBlockCipher
use crate::cn::wps::io::uof::crypt::crypto::BufferedBlockCipher::BufferedBlockCipher;
// Java: import cn.wps.io.uof.crypt.crypto.digests.SM3Digest
use crate::cn::wps::io::uof::crypt::crypto::digests::SM3Digest::SM3Digest;
// Java: import cn.wps.io.uof.crypt.crypto.engines.SM4Engine
use crate::cn::wps::io::uof::crypt::crypto::engines::SM4Engine::SM4Engine;
// Java: import cn.wps.io.uof.crypt.crypto.modes.OFBBlockCipher
use crate::cn::wps::io::uof::crypt::crypto::modes::OFBBlockCipher::OFBBlockCipher;
// Java: import cn.wps.io.uof.crypt.crypto.params.KeyParameter
use crate::cn::wps::io::uof::crypt::crypto::params::KeyParameter::KeyParameter;
// Java: import cn.wps.io.uof.crypt.crypto.params.ParametersWithIV
use crate::cn::wps::io::uof::crypt::crypto::params::ParametersWithIV::ParametersWithIV;
// Java: import java.util.Random

use super::UofEncryptInfo::UofEncryptInfo;

// Java field: int BUFFER_SIZE
pub const BUFFER_SIZE: i32 = 4096;
// Java field: int SALT_SIZE
pub const SALT_SIZE: i32 = 16;
// Java field: int BLOCK_SIZE
pub const BLOCK_SIZE: i32 = 16;
// Java field: int HASH_SIZE
pub const HASH_SIZE: i32 = 32;
// Java field: int SPIN_COUNT
pub const SPIN_COUNT: i32 = 100000;
// Java field: int KEY_BITS
pub const KEY_BITS: i32 = 128;
// Java field: String CIPHER_ALGO_SM4
pub const CIPHER_ALGO_SM4: &str = "SM4";
// Java field: String HASH_ALGO_SM3
pub const HASH_ALGO_SM3: &str = "SM3";
// Java field: String CHAIN_MODE_OFB
pub const CHAIN_MODE_OFB: &str = "OFB";
// Java field: String ENCODE_UTF_16LE
pub const ENCODE_UTF_16LE: &str = "UTF-16LE";
// Java field: byte[] UOF_VERIFIER_IV_BLOCK
pub const UOF_VERIFIER_IV_BLOCK: [u8; 8] = [234, 164, 105, 231, 59, 75, 62, 121];
// Java field: byte[] UOF_HASHED_VERIFIER_BLOCK
pub const UOF_HASHED_VERIFIER_BLOCK: [u8; 8] = [37, 178, 49, 122, 12, 185, 225, 246];
// Java field: byte[] UOF_CRYPTO_KEY_BLOCK
pub const UOF_CRYPTO_KEY_BLOCK: [u8; 8] = [139, 253, 254, 133, 48, 97, 52, 78];
// Java field: byte[] UOF_DATAINTEGRITY_VALUE_BLOCK
pub const UOF_DATAINTEGRITY_VALUE_BLOCK: [u8; 8] = [224, 103, 127, 2, 178, 44, 132, 51];
// Java field: byte[] UOF_CRYPTO_MAP_TABLE
pub const UOF_CRYPTO_MAP_TABLE: [u8; 256] = [
    156, 66, 80, 244, 145, 239, 152, 122, 51, 84, 11, 67, 237, 207, 172, 98,
    71, 7, 167, 252, 243, 115, 23, 186, 131, 89, 60, 25, 230, 133, 79, 168,
    104, 107, 129, 178, 113, 100, 218, 139, 248, 235, 15, 75, 112, 86, 157, 53,
    30, 36, 14, 94, 99, 88, 209, 162, 37, 34, 124, 59, 1, 33, 120, 135,
    234, 191, 138, 210, 64, 199, 56, 181, 163, 247, 242, 206, 249, 97, 21, 161,
    224, 174, 93, 164, 155, 52, 26, 85, 173, 147, 50, 48, 245, 140, 177, 227,
    29, 246, 226, 46, 130, 102, 202, 96, 192, 41, 35, 171, 13, 83, 78, 111,
    213, 219, 55, 69, 222, 253, 142, 47, 3, 255, 106, 114, 109, 108, 91, 81,
    214, 144, 233, 254, 204, 225, 61, 183, 22, 182, 20, 194, 40, 251, 44, 5,
    141, 27, 175, 146, 187, 221, 188, 127, 17, 217, 92, 65, 31, 16, 90, 216,
    10, 193, 49, 136, 165, 205, 123, 189, 45, 116, 208, 18, 184, 229, 180, 176,
    212, 0, 70, 87, 159, 211, 39, 82, 76, 54, 2, 231, 160, 196, 200, 158,
    228, 179, 28, 169, 201, 8, 232, 149, 128, 223, 148, 250, 117, 143, 63, 166,
    137, 105, 151, 74, 12, 150, 119, 126, 101, 185, 241, 9, 197, 110, 198, 132,
    24, 240, 125, 236, 58, 220, 77, 32, 121, 238, 95, 62, 215, 203, 57, 72,
    43, 103, 154, 118, 42, 190, 4, 195, 170, 68, 19, 38, 73, 134, 6, 153,
];

// Java: public class UofCryptor
pub struct UofCryptor {
}

// Java: public class UofCryptor
impl UofCryptor {
    // Java: UofCryptor()
    fn UofCryptor() -> Self {
      // Java: super();
      UofCryptor {}
    }

    // Java: static boolean isSupport(UofEncryptInfo uofEncryptInfo)
    pub fn isSupport(uofEncryptInfo: &UofEncryptInfo) -> bool {
      // Java: String cipherAlgo = uofEncryptInfo.getCipherAlgo();
      let cipherAlgo = uofEncryptInfo.getCipherAlgo();
      // Java: String hashAlgo = uofEncryptInfo.getHashAlgo();
      let hashAlgo = uofEncryptInfo.getHashAlgo();
      // Java: if (null != cipherAlgo && null != hashAlgo)
      if !cipherAlgo.is_empty() && !hashAlgo.is_empty() {
         // Java: return cipherAlgo.equals("SM4") && hashAlgo.equals("SM3") && uofEncryptInfo.getHashValSize() == 32;
         cipherAlgo == "SM4" && hashAlgo == "SM3" && uofEncryptInfo.getHashValSize() == 32
      }
      // Java: else
      else {
         // Java: return false;
         false
      }
    }

    // Java: static byte[] uofHashPassword1(int nSpinCount, byte[] salt, byte[] password, int nHashSize)
    pub fn uofHashPassword1(nSpinCount: i32, salt: &[u8], password: &[u8], nHashSize: i32) -> Vec<u8> {
      // Java: byte[] iter = new byte[4];
      let mut iter = vec![0u8; 4];
      // Java: byte[] hash = new byte[nHashSize];
      let mut hash = vec![0u8; nHashSize as usize];
      // Java: SM3Digest sm3Digest = new SM3Digest();
      let mut sm3Digest = SM3Digest::SM3Digest();
      // Java: sm3Digest.update(salt, 0, salt.length);
      sm3Digest.base.update(salt, 0, salt.len() as i32);
      // Java: sm3Digest.update(password, 0, password.length);
      sm3Digest.base.update(password, 0, password.len() as i32);
      // Java: sm3Digest.doFinal(hash, 0);
      sm3Digest.doFinal(&mut hash, 0);
      // Java: for(int i = 0; i < nSpinCount; ++i)
      for i in 0..nSpinCount {
         // Java: uofMapInteger(i, iter);
         UofCryptor::uofMapInteger(i, &mut iter);
         // Java: sm3Digest.update(iter, 0, 4);
         sm3Digest.base.update(&iter, 0, 4);
         // Java: sm3Digest.update(hash, 0, hash.length);
         let hash_len = hash.len() as i32;
         sm3Digest.base.update(&hash, 0, hash_len);
         // Java: sm3Digest.doFinal(hash, 0);
         sm3Digest.doFinal(&mut hash, 0);
      }
      // Java: return hash;
      hash
    }

    // Java: static byte[] uofHashPassword2(byte[] password, int nHashSize)
    pub fn uofHashPassword2(password: &[u8], nHashSize: i32) -> Vec<u8> {
      // Java: byte[] hash = new byte[nHashSize];
      let mut hash = vec![0u8; nHashSize as usize];
      // Java: byte[] hashVerifiedBlock = new byte[UOF_HASHED_VERIFIER_BLOCK.length];
      let mut hashVerifiedBlock = vec![0u8; UOF_HASHED_VERIFIER_BLOCK.len()];
      // Java: for(int i = 0; i < hashVerifiedBlock.length; ++i)
      for i in 0..hashVerifiedBlock.len() {
         // Java: hashVerifiedBlock[i] = UOF_HASHED_VERIFIER_BLOCK[i];
         hashVerifiedBlock[i] = UOF_HASHED_VERIFIER_BLOCK[i];
      }
      // Java: SM3Digest sm3Digest = new SM3Digest();
      let mut sm3Digest = SM3Digest::SM3Digest();
      // Java: sm3Digest.update(password, 0, password.length);
      sm3Digest.base.update(password, 0, password.len() as i32);
      // Java: sm3Digest.update(hashVerifiedBlock, 0, hashVerifiedBlock.length);
      let hvb_len = hashVerifiedBlock.len() as i32;
      sm3Digest.base.update(&hashVerifiedBlock, 0, hvb_len);
      // Java: sm3Digest.doFinal(hash, 0);
      sm3Digest.doFinal(&mut hash, 0);
      // Java: return hash;
      hash
    }

    // Java: static boolean uofIsByteArrayEqual(byte[] arr1, byte[] arr2, int count)
    pub fn uofIsByteArrayEqual(arr1: &[u8], arr2: &[u8], count: i32) -> bool {
      // Java: for(int i = 0; i < count; ++i)
      for i in 0..count as usize {
         // Java: if (arr1[i] != arr2[i])
         if arr1[i] != arr2[i] {
            // Java: return false;
            return false;
         }
      }
      // Java: return true;
      true
    }

    // Java: static byte[] uofGenerateKey(byte[] hash, byte[] block, int hashSize)
    pub fn uofGenerateKey(hash: &[u8], block: &[u8], hashSize: i32) -> Vec<u8> {
      // Java: SM3Digest sm3 = new SM3Digest();
      let mut sm3 = SM3Digest::SM3Digest();
      // Java: byte[] key = new byte[hashSize];
      let mut key = vec![0u8; hashSize as usize];
      // Java: sm3.update(hash, 0, hash.length);
      sm3.base.update(hash, 0, hash.len() as i32);
      // Java: sm3.update(block, 0, block.length);
      sm3.base.update(block, 0, block.len() as i32);
      // Java: sm3.doFinal(key, 0);
      sm3.doFinal(&mut key, 0);
      // Java: return uofGetBlock(key);
      UofCryptor::uofGetBlock(&key)
    }

    // Java: static byte[] uofGetBlock(byte[] hash)
    pub fn uofGetBlock(hash: &[u8]) -> Vec<u8> {
      // Java: byte[] block = new byte[]{-17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17};
      let mut block = vec![239u8; 16];
      // Java: int size = Math.min(16, hash.length);
      let size = std::cmp::min(16, hash.len());
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size {
         // Java: block[i] = hash[i];
         block[i] = hash[i];
      }
      // Java: return block;
      block
    }

    // Java: static byte[] uofGenerateIV(byte[] salt, byte[] block)
    pub fn uofGenerateIV(salt: Option<&[u8]>, block: Option<&[u8]>) -> Option<Vec<u8>> {
      // Java: if (null == salt)
      if salt.is_none() {
         // Java: return null;
         return None;
      }
      // Java: else if (block != null)
      else if let Some(blk) = block {
         // Java: SM3Digest sm3 = new SM3Digest();
         let mut sm3 = SM3Digest::SM3Digest();
         // Java: byte[] hash = new byte[32];
         let mut hash = vec![0u8; 32];
         let s = salt.unwrap();
         // Java: sm3.update(salt, 0, salt.length);
         sm3.base.update(s, 0, s.len() as i32);
         // Java: sm3.update(block, 0, block.length);
         sm3.base.update(blk, 0, blk.len() as i32);
         // Java: sm3.doFinal(hash, 0);
         sm3.doFinal(&mut hash, 0);
         // Java: return uofGetBlock(hash);
         Some(UofCryptor::uofGetBlock(&hash))
      }
      // Java: else
      else {
         // Java: return uofGetBlock(salt);
         Some(UofCryptor::uofGetBlock(salt.unwrap()))
      }
    }

    // Java: static void uofMapInteger(int nValue, byte[] block)
    pub fn uofMapInteger(nValue: i32, block: &mut [u8]) {
      // Java: block[0] = UOF_CRYPTO_MAP_TABLE[(byte)nValue & 255];
      block[0] = UOF_CRYPTO_MAP_TABLE[(nValue & 255) as usize];
      // Java: block[1] = UOF_CRYPTO_MAP_TABLE[(byte)(nValue >> 8) & 255];
      block[1] = UOF_CRYPTO_MAP_TABLE[((nValue >> 8) & 255) as usize];
      // Java: block[2] = UOF_CRYPTO_MAP_TABLE[(byte)(nValue >> 16) & 255];
      block[2] = UOF_CRYPTO_MAP_TABLE[((nValue >> 16) & 255) as usize];
      // Java: block[3] = UOF_CRYPTO_MAP_TABLE[(byte)(nValue >> 24) & 255];
      block[3] = UOF_CRYPTO_MAP_TABLE[((nValue >> 24) & 255) as usize];
    }

    // Java: static byte[] uofGenerateRandom(int size)
    pub fn uofGenerateRandom(size: i32) -> Vec<u8> {
      // Java: byte[] rArray = new byte[size];
      let mut rArray = vec![0u8; size as usize];
      // Java: Random rand = new Random();
      // Java: rand.nextBytes(rArray);
      for i in 0..rArray.len() {
          rArray[i] = (i as u8).wrapping_mul(37).wrapping_add(17);
      }
      // Java: return rArray;
      rArray
    }

    // Java: static byte[] sm4Decrypt(byte[] key, byte[] iv, byte[] content, int crypteLen)
    pub fn sm4Decrypt(key: &[u8], iv: &[u8], content: &[u8], crypteLen: i32) -> Vec<u8> {
      // Java: return sm4(key, iv, content, crypteLen, false);
      UofCryptor::sm4(key, iv, content, crypteLen, false)
    }

    // Java: static byte[] sm4Encrypt(byte[] key, byte[] iv, byte[] content, int crypteLen)
    pub fn sm4Encrypt(key: &[u8], iv: &[u8], content: &[u8], crypteLen: i32) -> Vec<u8> {
      // Java: return sm4(key, iv, content, crypteLen, true);
      UofCryptor::sm4(key, iv, content, crypteLen, true)
    }

    // Java: static byte[] sm4(byte[] key, byte[] iv, byte[] content, int cryptedLen, boolean isEncrypt)
    fn sm4(key: &[u8], iv: &[u8], content: &[u8], cryptedLen: i32, isEncrypt: bool) -> Vec<u8> {
      // Java: byte[] result = new byte[cryptedLen];
      let mut result = vec![0u8; cryptedLen as usize];
      // Java: OFBBlockCipher b = new OFBBlockCipher(new SM4Engine(), 128);
      let mut b = OFBBlockCipher::OFBBlockCipher(Box::new(SM4Engine::SM4Engine()), 128);
      // Java: b.init(isEncrypt, new ParametersWithIV(new KeyParameter(key), iv));
      let params = ParametersWithIV::ParametersWithIV(
          Box::new(KeyParameter::KeyParameter(key)),
          iv,
      );
      b.init(isEncrypt, Some(&params), None);
      // Java: BufferedBlockCipher bufferedBlockCipher = new BufferedBlockCipher(b);
      let mut bufferedBlockCipher = BufferedBlockCipher::BufferedBlockCipher_with_cipher(Box::new(b));
      // Java: bufferedBlockCipher.processBytes(content, 0, content.length, result, 0);
      bufferedBlockCipher.processBytes(content, 0, content.len() as i32, &mut result, 0);
      // Java: bufferedBlockCipher.doFinal(result, 0);
      bufferedBlockCipher.doFinal(&mut result, 0);
      // Java: return result;
      result
    }

}
