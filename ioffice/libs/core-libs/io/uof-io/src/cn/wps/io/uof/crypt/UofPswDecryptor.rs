// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.io.uof.crypt.crypto.digests.SM3Digest
use crate::cn::wps::io::uof::crypt::crypto::digests::SM3Digest::SM3Digest;
// Java: import cn.wps.io.uof.reader.UofrPackage
use crate::cn::wps::io::uof::reader::UofrPackage::UofrPackage;
// Java: import cn.wps.moffice.util.FileUtil
// Java: import java.io.File
// Java: import java.io.FileOutputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.UnsupportedEncodingException

use super::UofEncryptInfo::UofEncryptInfo;
use super::UofCryptor::UofCryptor;
use std::io::{Read, Write};

// Java: public class UofPswDecryptor
pub struct UofPswDecryptor {
    // Java field: UofrPackage mPkg
    pub(crate) mPkg: UofrPackage,
    // Java field: UofEncryptInfo mUofCryptInfo
    pub(crate) mUofCryptInfo: UofEncryptInfo,
    // Java field: String mDefaultPassword
    pub(crate) mDefaultPassword: String,
    // Java field: String mPwd
    pub(crate) mPwd: String,
    // Java field: long mPos
    pub(crate) mPos: i64,
    // Java field: long mSize
    pub(crate) mSize: i64,
}

// Java: public class UofPswDecryptor
impl UofPswDecryptor {
    // Java: UofPswDecryptor(UofrPackage pkg, UofEncryptInfo uofEncryptInfo, String defaultPsw, String pwd)
    pub fn UofPswDecryptor(pkg: UofrPackage, uofEncryptInfo: UofEncryptInfo, defaultPsw: String, pwd: String) -> Self {
      // Java: super();
      // Java: this.mPkg = pkg;
      // Java: this.mUofCryptInfo = uofEncryptInfo;
      // Java: this.mDefaultPassword = defaultPsw;
      // Java: this.mPwd = pwd;
      UofPswDecryptor {
          mPkg: pkg,
          mUofCryptInfo: uofEncryptInfo,
          mDefaultPassword: defaultPsw,
          mPwd: pwd,
          mPos: 0,
          mSize: 0,
      }
    }

    // Java: String decrypt()
    pub fn decrypt(&mut self) -> Option<String> {
      // Java: if (!UofCryptor.isSupport(this.mUofCryptInfo))
      if !UofCryptor::isSupport(&self.mUofCryptInfo) {
         // Java: return null;
         return None;
      }
      // Java: else
      else {
         // Java: return !this.verifyRepeatablePassword() ? null : this.decryptPackage();
         if !self.verifyRepeatablePassword() { None } else { self.decryptPackage() }
      }
    }

    // Java: boolean verifyRepeatablePassword()
    fn verifyRepeatablePassword(&mut self) -> bool {
      // Java: if (this.mDefaultPassword != null && this.mDefaultPassword.length() > 0)
      if !self.mDefaultPassword.is_empty() {
         // Java: return this.verifyPwd(this.mDefaultPassword);
         let pwd = self.mDefaultPassword.clone();
         self.verifyPwd(&pwd)
      }
      // Java: else
      else {
         // Java: boolean hasVerified = this.verifyPwd(this.mPwd);
         let pwd = self.mPwd.clone();
         let hasVerified = self.verifyPwd(&pwd);
         // Java: return hasVerified;
         hasVerified
      }
    }

    // Java: boolean verifyPwd(String strPassword)
    fn verifyPwd(&mut self, strPassword: &str) -> bool {
      // Java: if (!strPassword.isEmpty() && UofCryptor.isSupport(this.mUofCryptInfo))
      if !strPassword.is_empty() && UofCryptor::isSupport(&self.mUofCryptInfo) {
         // Java: byte[] password;
         // Java: try
         // Java: password = strPassword.getBytes("UTF-16LE");
         let password: Vec<u8> = strPassword.encode_utf16()
             .flat_map(|c| c.to_le_bytes())
             .collect();
         // Java: catch (UnsupportedEncodingException var12)
         // Java: return false;

         // Java: if (null != this.mUofCryptInfo.getVerifiedSaltValue() && null != this.mUofCryptInfo.getVerifiedEncPassword() && null != this.mUofCryptInfo.getCryptedKey())
         if !self.mUofCryptInfo.getVerifiedSaltValue().is_empty()
             && !self.mUofCryptInfo.getVerifiedEncPassword().is_empty()
             && !self.mUofCryptInfo.getCryptedKey().is_empty()
         {
            // Java: byte[] pswHash = UofCryptor.uofHashPassword1(this.mUofCryptInfo.getSpinCount(), this.mUofCryptInfo.getVerifiedSaltValue(), password, this.mUofCryptInfo.getHashValSize());
            let pswHash = UofCryptor::uofHashPassword1(
                self.mUofCryptInfo.getSpinCount(),
                self.mUofCryptInfo.getVerifiedSaltValue(),
                &password,
                self.mUofCryptInfo.getHashValSize(),
            );
            // Java: byte[] encryptedPassword = UofCryptor.uofHashPassword2(pswHash, this.mUofCryptInfo.getHashValSize());
            let encryptedPassword = UofCryptor::uofHashPassword2(&pswHash, self.mUofCryptInfo.getHashValSize());
            // Java: byte[] decodeVerPwd = this.mUofCryptInfo.getVerifiedEncPassword();
            let decodeVerPwd = self.mUofCryptInfo.getVerifiedEncPassword();
            // Java: int nMinCount = Math.min(encryptedPassword.length, decodeVerPwd.length);
            let nMinCount = std::cmp::min(encryptedPassword.len(), decodeVerPwd.len()) as i32;
            // Java: if (!UofCryptor.uofIsByteArrayEqual(encryptedPassword, decodeVerPwd, nMinCount))
            if !UofCryptor::uofIsByteArrayEqual(&encryptedPassword, decodeVerPwd, nMinCount) {
               // Java: return false;
               return false;
            }
            // Java: else
            else {
               // Java: byte[] cryptoKeyBlock = UofCryptor.UOF_CRYPTO_KEY_BLOCK;
               let cryptoKeyBlock = &super::UofCryptor::UOF_CRYPTO_KEY_BLOCK;
               // Java: byte[] cryptoKey = UofCryptor.uofGenerateKey(pswHash, cryptoKeyBlock, this.mUofCryptInfo.getHashValSize());
               let cryptoKey = UofCryptor::uofGenerateKey(&pswHash, cryptoKeyBlock, self.mUofCryptInfo.getHashValSize());
               // Java: byte[] cryptoIVBlock = UofCryptor.UOF_VERIFIER_IV_BLOCK;
               let cryptoIVBlock = &super::UofCryptor::UOF_VERIFIER_IV_BLOCK;
               // Java: byte[] cryptoIV = UofCryptor.uofGenerateIV(pswHash, cryptoIVBlock);
               let cryptoIV = UofCryptor::uofGenerateIV(Some(&pswHash), Some(cryptoIVBlock));
               // Java: if (null == cryptoIV)
               if cryptoIV.is_none() {
                  // Java: return false;
                  return false;
               }
               // Java: else
               else {
                  // Java: byte[] cipherData = this.mUofCryptInfo.getCryptedKey();
                  let cipherData = self.mUofCryptInfo.getCryptedKey().to_vec();
                  let cd_len = cipherData.len() as i32;
                  // Java: this.mUofCryptInfo.setSessionKey(UofCryptor.sm4Decrypt(cryptoKey, cryptoIV, cipherData, cipherData.length));
                  let session_key = UofCryptor::sm4Decrypt(&cryptoKey, &cryptoIV.unwrap(), &cipherData, cd_len);
                  self.mUofCryptInfo.setSessionKey(session_key);
                  // Java: return true;
                  return true;
               }
            }
         }
         // Java: else
         else {
            // Java: return false;
            return false;
         }
      }
      // Java: else
      else {
         // Java: return false;
         false
      }
    }

    // Java: boolean checkDataIntegrity()
    fn checkDataIntegrity(&self) -> bool {
      // Java: byte[] encryptCRC = this.mUofCryptInfo.getCryptedCRC();
      let encryptCRC = self.mUofCryptInfo.getCryptedCRC();
      // Java: if (encryptCRC != null && encryptCRC.length > 0)
      if !encryptCRC.is_empty() {
         // Java: byte[] decryptData = this.getSM4DecryptData();
         let decryptData = self.getSM4DecryptData();
         // Java: byte[] fileHashValue = this.calcFileHashValue();
         let fileHashValue = self.calcFileHashValue();
         // Java: if (null != fileHashValue && null != decryptData)
         if let (Some(fhv), Some(dd)) = (&fileHashValue, &decryptData) {
            // Java: int nMinCount = Math.min(fileHashValue.length, decryptData.length);
            let nMinCount = std::cmp::min(fhv.len(), dd.len()) as i32;
            // Java: return UofCryptor.uofIsByteArrayEqual(fileHashValue, decryptData, nMinCount);
            UofCryptor::uofIsByteArrayEqual(fhv, dd, nMinCount)
         }
         // Java: else
         else {
            // Java: return false;
            false
         }
      }
      // Java: else
      else {
         // Java: return false;
         false
      }
    }

    // Java: String decryptPackage()
    fn decryptPackage(&mut self) -> Option<String> {
      // Java: File file = Platform.createTempFile("moffice", ".tmp");
      let temp_dir = std::env::temp_dir();
      let file_path = temp_dir.join(format!("moffice_{}.tmp", std::process::id()));
      // Java: FileOutputStream fos = null;
      // Java: String cipherReference = this.mUofCryptInfo.getCipherReference();
      let cipherReference = self.mUofCryptInfo.getCipherReference().to_string();
      // Java: if (null == cipherReference)
      if cipherReference.is_empty() {
         // Java: return null;
         return None;
      }
      // Java: else
      {
         // Java: cipherReference = cipherReference.replaceFirst("/", "");
         let cipherReference = if cipherReference.starts_with('/') {
             &cipherReference[1..]
         } else {
             &cipherReference
         };
         // Java: InputStream is = this.mPkg.getInputStream(cipherReference);
         let is_data = self.mPkg.getInputStream_by_name(cipherReference);
         // Java: if (null == is)
         if is_data.is_none() {
            // Java: return null;
            return None;
         }
         // Java: else
         {
            let is_data = is_data.unwrap();
            let mut is_cursor = std::io::Cursor::new(is_data);
            // Java: try
            {
               // Java: fos = new FileOutputStream(file);
               let fos_result = std::fs::File::create(&file_path);
               if fos_result.is_err() { return None; }
               let mut fos = fos_result.unwrap();
               // Java: byte[] head = new byte[8];
               let mut head = vec![0u8; 8];
               // Java: is.read(head, 0, head.length);
               let _ = is_cursor.read(&mut head);
               // Java: for(int i = 0; i < head.length; ++i)
               for i in 0..head.len() {
                  // Java: this.mSize |= (long)((head[i] & 255) << i * 8);
                  self.mSize |= ((head[i] as i64) & 255) << (i * 8);
               }
               // Java: byte[] buffer = new byte[4096];
               let mut buffer = vec![0u8; 4096];
               // Java: while(true)
               loop {
                  // Java: int cbRead = this.read(is, buffer, 4096L);
                  let cbRead = self.read_stream(&mut is_cursor, &mut buffer, 4096);
                  // Java: if (0 == cbRead)
                  if 0 == cbRead {
                     // Java: return file.getAbsolutePath();
                     return Some(file_path.to_string_lossy().to_string());
                  }
                  // Java: fos.write(buffer, 0, cbRead);
                  let _ = fos.write_all(&buffer[..cbRead as usize]);
               }
            }
            // Java: catch (IOException var11)
            // Java: // error handling
            // Java: finally
            // Java: FileUtil.closeQuietly(fos);
            // Java: return null;
         }
      }
    }

    // Java: int read(InputStream is, byte[] buffer, long cb)
    fn read_stream(&mut self, is: &mut std::io::Cursor<Vec<u8>>, buffer: &mut [u8], cb: i64) -> i32 {
      // Java: byte[] nextBuffer = null;
      let mut nextBuffer: Option<Vec<u8>> = None;
      // Java: long bufferLeft = 0L;
      let mut bufferLeft: i64 = 0;
      // Java: int nRead = 0;
      let mut nRead: i32 = 0;
      // Java: int ptr = 0;
      let mut ptr: usize = 0;
      // Java: while(cb - (long)nRead > 0L && this.mSize - this.mPos > 0L)
      while cb - (nRead as i64) > 0 && self.mSize - self.mPos > 0 {
         // Java: if (bufferLeft <= 0L)
         if bufferLeft <= 0 {
            // Java: nextBuffer = this.nextBuffer(is);
            nextBuffer = self.nextBuffer_stream(is);
            // Java: bufferLeft = (long)(nextBuffer == null ? 0 : nextBuffer.length);
            bufferLeft = match &nextBuffer {
                None => 0,
                Some(nb) => nb.len() as i64,
            };
         }
         // Java: if (bufferLeft > 0L)
         if bufferLeft > 0 {
            let nb = nextBuffer.as_ref().unwrap();
            // Java: int pos = (int)((long)nextBuffer.length - bufferLeft);
            let pos = (nb.len() as i64 - bufferLeft) as usize;
            // Java: long count = cb - (long)nRead > bufferLeft ? bufferLeft : cb - (long)nRead;
            let mut count = if cb - (nRead as i64) > bufferLeft { bufferLeft } else { cb - (nRead as i64) };
            // Java: count = this.mSize - this.mPos > count ? count : this.mSize - this.mPos;
            count = if self.mSize - self.mPos > count { count } else { self.mSize - self.mPos };
            // Java: for(int i = 0; (long)i < count; ++i)
            for i in 0..count as usize {
               // Java: buffer[ptr++] = nextBuffer[pos + i];
               buffer[ptr] = nb[pos + i];
               ptr += 1;
            }
            // Java: bufferLeft -= count;
            bufferLeft -= count;
            // Java: this.mPos += count;
            self.mPos += count;
            // Java: nRead = (int)((long)nRead + count);
            nRead = (nRead as i64 + count) as i32;
         }
      }
      // Java: return nRead;
      nRead
    }

    // Java: byte[] nextBuffer(InputStream is)
    fn nextBuffer_stream(&mut self, is: &mut std::io::Cursor<Vec<u8>>) -> Option<Vec<u8>> {
      // Java: if (null == this.mUofCryptInfo.getSaltValue())
      if self.mUofCryptInfo.getSaltValue().is_empty() {
         // Java: return null;
         return None;
      }
      // Java: else
      {
         // Java: int index = (int)(this.mPos / 4096L);
         let index = (self.mPos / 4096) as i32;
         // Java: byte[] buffer = new byte[4096];
         let mut buffer = vec![0u8; 4096];
         // Java: int readCount = this.readNextBuffer(is, buffer);
         let readCount = self.readNextBuffer_stream(is, &mut buffer);
         // Java: if (readCount == 0)
         if readCount == 0 {
            // Java: return null;
            return None;
         }
         // Java: else
         {
            // Java: byte[] block = new byte[4];
            let mut block = vec![0u8; 4];
            // Java: UofCryptor.uofMapInteger(index, block);
            UofCryptor::uofMapInteger(index, &mut block);
            // Java: byte[] iv = UofCryptor.uofGenerateIV(this.mUofCryptInfo.getSaltValue(), block);
            let salt = self.mUofCryptInfo.getSaltValue();
            let iv = UofCryptor::uofGenerateIV(Some(salt), Some(&block));
            // Java: if (null == iv)
            if iv.is_none() {
               // Java: return null;
               return None;
            }
            // Java: else
            {
               // Java: byte[] key = this.mUofCryptInfo.getSessionKey();
               let key = self.mUofCryptInfo.getSessionKey();
               // Java: return UofCryptor.sm4Decrypt(key, iv, buffer, buffer.length);
               let buf_len = buffer.len() as i32;
               Some(UofCryptor::sm4Decrypt(key, &iv.unwrap(), &buffer, buf_len))
            }
         }
      }
    }

    // Java: int readNextBuffer(InputStream is, byte[] buffer)
    fn readNextBuffer_stream(&self, is: &mut std::io::Cursor<Vec<u8>>, buffer: &mut [u8]) -> i32 {
      // Java: int count = buffer.length;
      let mut count = buffer.len() as i32;
      // Java: int offset = 0;
      let mut offset: usize = 0;
      // Java: int ret;
      // Java: int n;
      let mut ret: i32 = 0;
      // Java: for(ret = 0; count > 0; offset += n)
      while count > 0 {
         // Java: n = is.read(buffer, offset, count);
         let n = match is.read(&mut buffer[offset..offset + count as usize]) {
             Ok(0) => -1i32,
             Ok(n) => n as i32,
             Err(_) => -1i32,
         };
         // Java: if (n <= 0)
         if n <= 0 {
            // Java: break;
            break;
         }
         // Java: count -= n;
         count -= n;
         // Java: ret += n;
         ret += n;
         offset += n as usize;
      }
      // Java: return ret;
      ret
    }

    // Java: byte[] getSM4DecryptData()
    fn getSM4DecryptData(&self) -> Option<Vec<u8>> {
      // Java: if (null != this.mUofCryptInfo.getSaltValue() && null != this.mUofCryptInfo.getSessionKey() && null != this.mUofCryptInfo.getCryptedCRC())
      if !self.mUofCryptInfo.getSaltValue().is_empty()
          && !self.mUofCryptInfo.getSessionKey().is_empty()
          && !self.mUofCryptInfo.getCryptedCRC().is_empty()
      {
         // Java: byte[] diValueBlock = UofCryptor.UOF_DATAINTEGRITY_VALUE_BLOCK;
         let diValueBlock = &super::UofCryptor::UOF_DATAINTEGRITY_VALUE_BLOCK;
         // Java: byte[] diValue = UofCryptor.uofGenerateIV(this.mUofCryptInfo.getSaltValue(), diValueBlock);
         let diValue = UofCryptor::uofGenerateIV(Some(self.mUofCryptInfo.getSaltValue()), Some(diValueBlock));
         // Java: if (null == diValue)
         if diValue.is_none() {
            // Java: return null;
            return None;
         }
         // Java: else
         {
            // Java: byte[] key = this.mUofCryptInfo.getSessionKey();
            let key = self.mUofCryptInfo.getSessionKey();
            // Java: byte[] cdata = this.mUofCryptInfo.getCryptedCRC();
            let cdata = self.mUofCryptInfo.getCryptedCRC();
            let cdata_len = cdata.len() as i32;
            // Java: return UofCryptor.sm4Decrypt(key, diValue, cdata, cdata.length);
            Some(UofCryptor::sm4Decrypt(key, &diValue.unwrap(), cdata, cdata_len))
         }
      }
      // Java: else
      else {
         // Java: return null;
         None
      }
    }

    // Java: byte[] calcFileHashValue()
    fn calcFileHashValue(&self) -> Option<Vec<u8>> {
      // Java: InputStream is = this.mPkg.getInputStream(this.mUofCryptInfo.getCipherReference());
      let cipher_ref = self.mUofCryptInfo.getCipherReference();
      let is_data = self.mPkg.getInputStream_by_name(cipher_ref);
      // Java: if (null == is)
      if is_data.is_none() {
         // Java: return null;
         return None;
      }
      // Java: else
      {
         let is_data = is_data.unwrap();
         let mut is_cursor = std::io::Cursor::new(is_data);
         // Java: byte[] fileHashValue = new byte[32];
         let mut fileHashValue = vec![0u8; 32];
         // Java: byte[] buffer = new byte[4096];
         let mut buffer = vec![0u8; 4096];
         // Java: SM3Digest sm3 = new SM3Digest();
         let mut sm3 = SM3Digest::SM3Digest();
         // Java: int cbRead;
         // Java: try
         {
            // Java: do
            loop {
               // Java: cbRead = is.read(buffer, 0, 4096);
               let cbRead = match is_cursor.read(&mut buffer) {
                   Ok(0) => 0i32,
                   Ok(n) => n as i32,
                   Err(_) => return None,
               };
               // Java: if (cbRead > 0)
               if cbRead > 0 {
                  // Java: sm3.update(buffer, 0, cbRead);
                  sm3.base.update(&buffer, 0, cbRead);
               }
               if cbRead <= 0 { break; }
            }
         }
         // Java: catch (IOException var7)
         // Java: return null;
         // Java: sm3.doFinal(fileHashValue, 0);
         sm3.doFinal(&mut fileHashValue, 0);
         // Java: return fileHashValue;
         Some(fileHashValue)
      }
    }

}
