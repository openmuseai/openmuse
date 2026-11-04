// Java: import cn.wps.io.uof.crypt.crypto.digests.SM3Digest
use crate::cn::wps::io::uof::crypt::crypto::digests::SM3Digest::SM3Digest;
// Java: import cn.wps.moffice.util.FileUtil
// Java: import java.io.FileInputStream
// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.io.UnsupportedEncodingException
// Java: import org.apache.poi.util.base64.Base64

use super::UofEncryptInfo::UofEncryptInfo;
use super::UofCryptor::UofCryptor;
use std::io::{Read, Write};

// Java: public class UofPswEncryptor
pub struct UofPswEncryptor {
    // Java field: String mSrcPath
    pub(crate) mSrcPath: String,
    // Java field: OutputStream mDstOs
    pub(crate) mDstOs: Box<dyn Write>,
    // Java field: UofEncryptInfo mUofEncryptInfo
    pub(crate) mUofEncryptInfo: UofEncryptInfo,
    // Java field: String mPsw
    pub(crate) mPsw: String,
    // Java field: String mPswHint
    pub(crate) mPswHint: String,
}

// Java: public class UofPswEncryptor
impl UofPswEncryptor {
    // Java: UofPswEncryptor(String srcPath, OutputStream dstOs, String psw, String pswHint)
    pub fn UofPswEncryptor(srcPath: String, dstOs: Box<dyn Write>, psw: String, pswHint: String) -> Self {
      // Java: super();
      // Java: this.mSrcPath = srcPath;
      // Java: this.mDstOs = dstOs;
      // Java: this.mPsw = psw;
      // Java: this.mPswHint = pswHint;
      UofPswEncryptor {
          mSrcPath: srcPath,
          mDstOs: dstOs,
          mUofEncryptInfo: UofEncryptInfo::new(),
          mPsw: psw,
          mPswHint: pswHint,
      }
    }

    // Java: UofEncryptInfo encrypt()
    pub fn encrypt(&mut self) -> Option<UofEncryptInfo> {
      // Java: this.createInfo();
      self.createInfo();
      // Java: this.mUofEncryptInfo.setSaltValue(Base64.encode(UofCryptor.uofGenerateRandom(16)));
      self.mUofEncryptInfo.setSaltValue(UofCryptor::uofGenerateRandom(16));
      // Java: if (!this.generateVerifyData())
      if !self.generateVerifyData() {
         // Java: return null;
         return None;
      }
      // Java: else
      {
         // Java: byte[] fileHashValue = this.encryptPackage();
         let fileHashValue = self.encryptPackage();
         // Java: return !this.encryptFileHash(fileHashValue) ? null : this.mUofEncryptInfo;
         match fileHashValue {
             Some(fhv) => {
                 if !self.encryptFileHash(&fhv) {
                     None
                 } else {
                     Some(std::mem::replace(&mut self.mUofEncryptInfo, UofEncryptInfo::new()))
                 }
             }
             None => None,
         }
      }
    }

    // Java: void createInfo()
    fn createInfo(&mut self) {
      // Java: this.mUofEncryptInfo = new UofEncryptInfo();
      self.mUofEncryptInfo = UofEncryptInfo::new();
      // Java: this.mUofEncryptInfo.setSaltSize(16);
      self.mUofEncryptInfo.setSaltSize(16);
      // Java: this.mUofEncryptInfo.setBlockSize(16);
      self.mUofEncryptInfo.setBlockSize(16);
      // Java: this.mUofEncryptInfo.setKeyBits(128);
      self.mUofEncryptInfo.setKeyBits(128);
      // Java: this.mUofEncryptInfo.setHashValSize(32);
      self.mUofEncryptInfo.setHashValSize(32);
      // Java: this.mUofEncryptInfo.setSpinCount(100000);
      self.mUofEncryptInfo.setSpinCount(100000);
      // Java: this.mUofEncryptInfo.setCipherAlgo("SM4");
      self.mUofEncryptInfo.setCipherAlgo("SM4".to_string());
      // Java: this.mUofEncryptInfo.setHashAlgo("SM3");
      self.mUofEncryptInfo.setHashAlgo("SM3".to_string());
      // Java: this.mUofEncryptInfo.setChainMode("OFB");
      self.mUofEncryptInfo.setChainMode("OFB".to_string());
    }

    // Java: boolean encryptFileHash(byte[] fileHashValue)
    fn encryptFileHash(&mut self, fileHashValue: &[u8]) -> bool {
      // Java: if (null != this.mUofEncryptInfo.getSaltValue() && null != this.mUofEncryptInfo.getSessionKey() && null != fileHashValue)
      if !self.mUofEncryptInfo.getSaltValue().is_empty()
          && !self.mUofEncryptInfo.getSessionKey().is_empty()
      {
         // Java: byte[] valBlock = UofCryptor.UOF_DATAINTEGRITY_VALUE_BLOCK;
         let valBlock = &super::UofCryptor::UOF_DATAINTEGRITY_VALUE_BLOCK;
         // Java: byte[] ivVal = UofCryptor.uofGenerateIV(this.mUofEncryptInfo.getSaltValue(), valBlock);
         let ivVal = UofCryptor::uofGenerateIV(Some(self.mUofEncryptInfo.getSaltValue()), Some(valBlock));
         // Java: if (null == ivVal)
         if ivVal.is_none() {
            // Java: return false;
            return false;
         }
         // Java: else
         {
            // Java: byte[] session = this.mUofEncryptInfo.getSessionKey();
            let session = self.mUofEncryptInfo.getSessionKey().to_vec();
            let fhv_len = fileHashValue.len() as i32;
            // Java: byte[] sm4EnByte = UofCryptor.sm4Encrypt(session, ivVal, fileHashValue, fileHashValue.length);
            let sm4EnByte = UofCryptor::sm4Encrypt(&session, &ivVal.unwrap(), fileHashValue, fhv_len);
            // Java: this.mUofEncryptInfo.setCryptedCRC(sm4EnByte);
            self.mUofEncryptInfo.setCryptedCRC(sm4EnByte);
            // Java: return true;
            true
         }
      }
      // Java: else
      else {
         // Java: return false;
         false
      }
    }

    // Java: byte[] encryptPackage()
    fn encryptPackage(&mut self) -> Option<Vec<u8>> {
      // Java: if (null != this.mUofEncryptInfo.getSaltValue() && null != this.mUofEncryptInfo.getSessionKey())
      if !self.mUofEncryptInfo.getSaltValue().is_empty()
          && !self.mUofEncryptInfo.getSessionKey().is_empty()
      {
         // Java: FileInputStream fis = null;
         // Java: try
         {
            // Java: fis = new FileInputStream(this.mSrcPath);
            let fis_result = std::fs::File::open(&self.mSrcPath);
            if fis_result.is_err() { return None; }
            let mut fis = fis_result.unwrap();
            // Java: long fileSize = fis.getChannel().size();
            let fileSize = fis.metadata().map(|m| m.len() as i64).unwrap_or(0);
            // Java: byte[] buffer = new byte[4096];
            let mut buffer = vec![0u8; 4096];
            // Java: byte[] head = new byte[8];
            let mut head = vec![0u8; 8];
            // Java: for(int i = head.length - 1; i >= 0; --i)
            for i in (0..head.len()).rev() {
               // Java: head[i] = (byte)((int)(fileSize >> i * 8 & 255L));
               head[i] = ((fileSize >> (i * 8)) & 255) as u8;
            }
            // Java: SM3Digest sm3Digest = new SM3Digest();
            let mut sm3Digest = SM3Digest::SM3Digest();
            // Java: sm3Digest.update(head, 0, head.length);
            let head_len = head.len() as i32;
            sm3Digest.base.update(&head, 0, head_len);
            // Java: for(int i = 0; i < head.length; ++i)
            for i in 0..head.len() {
               // Java: buffer[i] = head[i];
               buffer[i] = head[i];
            }
            // Java: this.mDstOs.write(buffer, 0, 8);
            let _ = self.mDstOs.write_all(&buffer[..8]);
            // Java: int index = 0;
            let mut index = 0i32;
            // Java: while(true)
            loop {
               // Java: int cbReaded = 0;
               // Java: buffer = new byte[4096];
               buffer = vec![0u8; 4096];
               // Java: cbReaded = fis.read(buffer, 0, buffer.length);
               let cbReaded = match fis.read(&mut buffer) {
                   Ok(0) => 0i32,
                   Ok(n) => n as i32,
                   Err(_) => -1i32,
               };
               // Java: if (cbReaded <= 0)
               if cbReaded <= 0 {
                  // Java: byte[] fileHashValue = new byte[32];
                  let mut fileHashValue = vec![0u8; 32];
                  // Java: sm3Digest.doFinal(fileHashValue, 0);
                  sm3Digest.doFinal(&mut fileHashValue, 0);
                  // Java: byte[] var22 = fileHashValue;
                  // Java: return var22;
                  return Some(fileHashValue);
               }
               // Java: byte[] block = new byte[4];
               let mut block = vec![0u8; 4];
               // Java: UofCryptor.uofMapInteger(index, block);
               UofCryptor::uofMapInteger(index, &mut block);
               // Java: byte[] iv = UofCryptor.uofGenerateIV(this.mUofEncryptInfo.getSaltValue(), block);
               let iv = UofCryptor::uofGenerateIV(Some(self.mUofEncryptInfo.getSaltValue()), Some(&block));
               // Java: if (null == iv)
               if iv.is_none() {
                  // Java: Object var23 = null;
                  // Java: return (byte[])var23;
                  return None;
               }
               let buf_len = buffer.len() as i32;
               // Java: byte[] encrypted = UofCryptor.sm4Encrypt(this.mUofEncryptInfo.getSessionKey(), iv, buffer, buffer.length);
               let encrypted = UofCryptor::sm4Encrypt(self.mUofEncryptInfo.getSessionKey(), &iv.unwrap(), &buffer, buf_len);
               // Java: this.mDstOs.write(encrypted);
               let _ = self.mDstOs.write_all(&encrypted);
               // Java: sm3Digest.update(encrypted, 0, encrypted.length);
               let enc_len = encrypted.len() as i32;
               sm3Digest.base.update(&encrypted, 0, enc_len);
               // Java: ++index;
               index += 1;
            }
         }
         // Java: catch (IOException var15)
         // Java: Object var3 = null;
         // Java: return (byte[])var3;
         // Java: finally
         // Java: FileUtil.closeQuietly(fis);
      }
      // Java: else
      else {
         // Java: return null;
         None
      }
    }

    // Java: boolean generateVerifyData()
    fn generateVerifyData(&mut self) -> bool {
      // Java: this.mUofEncryptInfo.setVerifiedSaltValue(UofCryptor.uofGenerateRandom(32));
      self.mUofEncryptInfo.setVerifiedSaltValue(UofCryptor::uofGenerateRandom(32));
      // Java: byte[] pswBytes;
      // Java: try
      // Java: pswBytes = this.mPsw.getBytes("UTF-16LE");
      let pswBytes: Vec<u8> = self.mPsw.encode_utf16()
          .flat_map(|c| c.to_le_bytes())
          .collect();
      // Java: catch (UnsupportedEncodingException var12)
      // Java: return false;

      // Java: byte[] pswHash = UofCryptor.uofHashPassword1(100000, this.mUofEncryptInfo.getVerifiedSaltValue(), pswBytes, 32);
      let pswHash = UofCryptor::uofHashPassword1(100000, self.mUofEncryptInfo.getVerifiedSaltValue(), &pswBytes, 32);
      // Java: byte[] fileCryptoKey = UofCryptor.uofGenerateRandom(16);
      let fileCryptoKey = UofCryptor::uofGenerateRandom(16);
      // Java: byte[] cryptoKeyBlock = UofCryptor.UOF_CRYPTO_KEY_BLOCK;
      let cryptoKeyBlock = &super::UofCryptor::UOF_CRYPTO_KEY_BLOCK;
      // Java: byte[] cryptoKey = UofCryptor.uofGenerateKey(pswHash, cryptoKeyBlock, 32);
      let cryptoKey = UofCryptor::uofGenerateKey(&pswHash, cryptoKeyBlock, 32);
      // Java: byte[] cryptoIVBlock = UofCryptor.UOF_VERIFIER_IV_BLOCK;
      let cryptoIVBlock = &super::UofCryptor::UOF_VERIFIER_IV_BLOCK;
      // Java: byte[] cryptoIv = UofCryptor.uofGenerateIV(pswHash, cryptoIVBlock);
      let cryptoIv = UofCryptor::uofGenerateIV(Some(&pswHash), Some(cryptoIVBlock));
      // Java: if (null == cryptoIv)
      if cryptoIv.is_none() {
         // Java: return false;
         return false;
      }
      // Java: else
      {
         let cryptoIv = cryptoIv.unwrap();
         let fck_len = fileCryptoKey.len() as i32;
         // Java: byte[] ekvByte = UofCryptor.sm4Encrypt(cryptoKey, cryptoIv, fileCryptoKey, fileCryptoKey.length);
         let ekvByte = UofCryptor::sm4Encrypt(&cryptoKey, &cryptoIv, &fileCryptoKey, fck_len);
         // Java: this.mUofEncryptInfo.setCryptedKey(ekvByte);
         self.mUofEncryptInfo.setCryptedKey(ekvByte);
         // Java: if (null == this.mUofEncryptInfo.getCryptedKey())
         if self.mUofEncryptInfo.getCryptedKey().is_empty() {
            // Java: return false;
            return false;
         }
         // Java: else
         {
            // Java: byte[] sessionContent = this.mUofEncryptInfo.getCryptedKey();
            let sessionContent = self.mUofEncryptInfo.getCryptedKey().to_vec();
            let sc_len = sessionContent.len() as i32;
            // Java: byte[] sessionByte = UofCryptor.sm4Decrypt(cryptoKey, cryptoIv, sessionContent, sessionContent.length);
            let sessionByte = UofCryptor::sm4Decrypt(&cryptoKey, &cryptoIv, &sessionContent, sc_len);
            // Java: this.mUofEncryptInfo.setSessionKey(sessionByte);
            self.mUofEncryptInfo.setSessionKey(sessionByte);
            // Java: byte[] vepByte = UofCryptor.uofHashPassword2(pswHash, 32);
            let vepByte = UofCryptor::uofHashPassword2(&pswHash, 32);
            // Java: this.mUofEncryptInfo.setVerifiedEncPassword(vepByte);
            self.mUofEncryptInfo.setVerifiedEncPassword(vepByte);
            // Java: return true;
            true
         }
      }
    }

}
