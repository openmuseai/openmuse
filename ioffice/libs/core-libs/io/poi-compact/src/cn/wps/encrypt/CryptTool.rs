// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.crypt.sal.digest.DigestSHA1
use crate::cn::wps::crypt::sal::digest::DigestSHA1::DigestSHA1;
// Java: import cn.wps.crypt.sal.digest.DigestSHA512
use crate::cn::wps::crypt::sal::digest::DigestSHA512::DigestSHA512;
// Java: import cn.wps.crypt.sal.digest.IDigest
use crate::cn::wps::crypt::sal::digest::IDigest::IDigest;
// Java: import java.io.UnsupportedEncodingException
// Java: import java.security.GeneralSecurityException
// Java: import java.security.MessageDigest
// Java: import java.security.NoSuchAlgorithmException
// Java: import java.util.Arrays
// Java: import javax.crypto.Cipher
// Java: import javax.crypto.SecretKey
// Java: import javax.crypto.spec.SecretKeySpec
// Java: import org.apache.poi.EncryptedDocumentException
use crate::org::apache::poi::EncryptedDocumentException::EncryptedDocumentException;
// Java: import org.apache.poi.util.LittleEndian

use crate::cn::wps::encrypt::Base64::Base64;
use crate::cn::wps::encrypt::ModifyVerifier::ModifyVerifier;
use crate::cn::wps::encrypt::EncryptionInfo::EncryptionInfo;

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
    data[offset + 2] = (value >> 16) as u8;
    data[offset + 3] = (value >> 24) as u8;
}

fn sha1_digest(data: &[u8]) -> Vec<u8> {
    let mut sha = DigestSHA1::new();
    sha.update_data(data, 0, data.len() as i32);
    let mut result = vec![0u8; 20];
    sha.get_digest(&mut result);
    result
}

fn sha1_update_digest(sha: &mut DigestSHA1, data: &[u8]) -> Vec<u8> {
    sha.update_data(data, 0, data.len() as i32);
    let mut result = vec![0u8; 20];
    sha.get_digest(&mut result);
    result
}

// Java: public class CryptTool
pub struct CryptTool {
    // Java field: byte[] passwordHash
    pub(crate) passwordHash: Option<Vec<u8>>,
    // Java field: SecretKey key
    pub(crate) key: Option<Vec<u8>>,
}

// Java: public class CryptTool
impl CryptTool {
    // Java: CryptTool()
    pub fn new() -> Self {
        // Java: super();
        CryptTool {
            passwordHash: None,
            key: None,
        }
    }

    // Java: Cipher getCipher(int keySize, boolean bool)
    pub fn getCipher(&mut self, keySize: i32, encrypt: bool) -> AesCipher {
        // Java: byte[] key = this.generateKey(0, keySize);
        let key = self.generateKey(0, keySize);
        // Java: Cipher cipher = Cipher.getInstance("AES/ECB/NoPadding");
        // Java: SecretKey skey = new SecretKeySpec(key, "AES");
        // Java: this.key = skey;
        self.key = Some(key.clone());
        // Java: if (bool)
        let mode = if encrypt {
            // Java: cipher.init(1, skey);
            1
        }
        // Java: else
        else {
            // Java: cipher.init(2, skey);
            2
        };
        // Java: return cipher;
        AesCipher { key, mode }
    }

    // Java: byte[] generateKey(int block, int keySize)
    pub fn generateKey(&self, block: i32, keySize: i32) -> Vec<u8> {
        // Java: MessageDigest sha1 = MessageDigest.getInstance("SHA-1");
        let mut sha = DigestSHA1::new();
        // Java: sha1.update(this.passwordHash);
        if let Some(ref ph) = self.passwordHash {
            sha.update_data(ph, 0, ph.len() as i32);
        }
        // Java: byte[] blockValue = new byte[4];
        let mut blockValue = vec![0u8; 4];
        // Java: LittleEndian.putInt(blockValue, block);
        le_put_int(&mut blockValue, 0, block);
        // Java: byte[] finalHash = sha1.digest(blockValue);
        let finalHash = sha1_update_digest(&mut sha, &blockValue);
        // Java: byte[] buff = new byte[64];
        let mut buff = vec![0u8; 64];
        // Java: Arrays.fill(buff, (byte)54);
        buff.fill(54u8);
        // Java: for(int i = 0; i < finalHash.length; ++i)
        for i in 0..finalHash.len() {
            // Java: buff[i] ^= finalHash[i];
            buff[i] ^= finalHash[i];
        }
        // Java: sha1.reset();
        let mut sha = DigestSHA1::new();
        // Java: byte[] x1 = sha1.digest(buff);
        let x1 = sha1_update_digest(&mut sha, &buff);
        // Java: Arrays.fill(buff, (byte)92);
        buff.fill(92u8);
        // Java: for(int i = 0; i < finalHash.length; ++i)
        for i in 0..finalHash.len() {
            // Java: buff[i] ^= finalHash[i];
            buff[i] ^= finalHash[i];
        }
        // Java: sha1.reset();
        let mut sha = DigestSHA1::new();
        // Java: byte[] x2 = sha1.digest(buff);
        let x2 = sha1_update_digest(&mut sha, &buff);
        // Java: byte[] x3 = new byte[x1.length + x2.length];
        let mut x3 = vec![0u8; x1.len() + x2.len()];
        // Java: System.arraycopy(x1, 0, x3, 0, x1.length);
        x3[..x1.len()].copy_from_slice(&x1);
        // Java: System.arraycopy(x2, 0, x3, x1.length, x2.length);
        x3[x1.len()..].copy_from_slice(&x2);
        // Java: return this.truncateOrPad(x3, keySize);
        Self::truncateOrPad(&x3, keySize as usize)
    }

    // Java: byte[] hashPassword(EncryptionInfo info, String password)
    pub fn hashPassword_simple(&self, info: &EncryptionInfo, password: &str) -> Vec<u8> {
        // Java: DigestSHA1 sha = new DigestSHA1();
        let mut sha = DigestSHA1::new();
        // Java: byte[] bytes;
        let bytes: Vec<u8>;
        // Java: try
        {
            // Java: bytes = password.getBytes("UTF-16LE");
            bytes = password.encode_utf16().flat_map(|c| c.to_le_bytes()).collect();
        }
        // Java: catch (UnsupportedEncodingException var9)
        {
            // Java: throw new EncryptedDocumentException("UTF16 not supported");
        }
        // Java: sha.update(info.getVerifier().getSalt(), 0, info.getVerifier().getSalt().length);
        let verifier = info.getVerifier().unwrap();
        let salt = verifier.getSalt();
        sha.update_data(salt, 0, salt.len() as i32);
        // Java: sha.update(bytes, 0, bytes.length);
        sha.update_data(&bytes, 0, bytes.len() as i32);
        // Java: byte[] hash = new byte[20];
        let mut hash = vec![0u8; 20];
        // Java: sha.get(hash);
        sha.get_digest(&mut hash);
        // Java: byte[] iterator = new byte[4];
        let mut iterator = vec![0u8; 4];
        // Java: int count = info.getVerifier().getSpinCount();
        let count = verifier.getSpinCount();
        // Java: for(int i = 0; i < count; ++i)
        for i in 0..count {
            // Java: iterator[0] = (byte)(i >>> 0 & 255);
            iterator[0] = ((i as u32 >> 0) & 255) as u8;
            // Java: iterator[1] = (byte)(i >>> 8 & 255);
            iterator[1] = ((i as u32 >> 8) & 255) as u8;
            // Java: iterator[2] = (byte)(i >>> 16 & 255);
            iterator[2] = ((i as u32 >> 16) & 255) as u8;
            // Java: iterator[3] = (byte)(i >>> 24 & 255);
            iterator[3] = ((i as u32 >> 24) & 255) as u8;
            // Java: sha.update(iterator, 0, 4);
            sha.update_data(&iterator, 0, 4);
            // Java: sha.update(hash, 0, hash.length);
            sha.update_data(&hash, 0, hash.len() as i32);
            // Java: sha.get(hash);
            sha.get_digest(&mut hash);
        }
        // Java: return hash;
        hash
    }

    // Java: byte[] hashPassword(EncryptionInfo info, String hashAlgorithmName, String password)
    pub fn hashPassword(&self, info: &EncryptionInfo, hashAlgorithmName: &str, password: &str) -> Option<Vec<u8>> {
        // Java: IDigest digest;
        let mut digest: Box<dyn IDigest>;
        // Java: if (hashAlgorithmName.equals("SHA-1"))
        if hashAlgorithmName == "SHA-1" {
            // Java: digest = new DigestSHA1();
            digest = Box::new(DigestSHA1::new());
        }
        // Java: else
        else {
            // Java: if (!hashAlgorithmName.equals("SHA-512"))
            if hashAlgorithmName != "SHA-512" {
                // Java: IOAssert.fail("UnsupportedAlgorithm!");
                // Java: return null;
                return None;
            }
            // Java: digest = new DigestSHA512();
            digest = Box::new(DigestSHA512::new());
        }
        // Java: byte[] bytes;
        let bytes: Vec<u8>;
        // Java: try
        {
            // Java: bytes = password.getBytes("UTF-16LE");
            bytes = password.encode_utf16().flat_map(|c| c.to_le_bytes()).collect();
        }
        // Java: catch (UnsupportedEncodingException var11)
        {
            // Java: throw new EncryptedDocumentException("UTF16 not supported");
        }
        // Java: int hashLen = digest.getLength();
        let hashLen = digest.get_length();
        // Java: byte[] hash = new byte[hashLen];
        let mut hash = vec![0u8; hashLen as usize];
        // Java: digest.update(info.getVerifier().getSalt(), 0, info.getVerifier().getSalt().length);
        let verifier = info.getVerifier().unwrap();
        let salt = verifier.getSalt();
        digest.update(salt, 0, salt.len() as i32);
        // Java: digest.update(bytes, 0, bytes.length);
        digest.update(&bytes, 0, bytes.len() as i32);
        // Java: digest.get(hash);
        digest.get(&mut hash);
        // Java: byte[] iterator = new byte[4];
        let mut iterator = vec![0u8; 4];
        // Java: int count = info.getVerifier().getSpinCount();
        let count = verifier.getSpinCount();
        // Java: for(int i = 0; i < count; ++i)
        for i in 0..count {
            // Java: iterator[0] = (byte)(i >>> 0 & 255);
            iterator[0] = ((i as u32 >> 0) & 255) as u8;
            // Java: iterator[1] = (byte)(i >>> 8 & 255);
            iterator[1] = ((i as u32 >> 8) & 255) as u8;
            // Java: iterator[2] = (byte)(i >>> 16 & 255);
            iterator[2] = ((i as u32 >> 16) & 255) as u8;
            // Java: iterator[3] = (byte)(i >>> 24 & 255);
            iterator[3] = ((i as u32 >> 24) & 255) as u8;
            // Java: digest.update(iterator, 0, 4);
            digest.update(&iterator, 0, 4);
            // Java: digest.update(hash, 0, hashLen);
            digest.update(&hash, 0, hashLen);
            // Java: digest.get(hash);
            digest.get(&mut hash);
        }
        // Java: return hash;
        Some(hash)
    }

    // Java: static boolean checkModifyPassword(ModifyVerifier srcModifyVer, String pwd)
    pub fn checkModifyPassword(srcModifyVer: &ModifyVerifier, pwd: &str) -> bool {
        // rust: existing non-null transport.
        Self::checkModifyPassword__nullable(Some(srcModifyVer),Some(pwd))
    }
    // java public static boolean checkModifyPassword(ModifyVerifier srcModifyVer, String pwd) {
    pub fn checkModifyPassword__nullable(srcModifyVer:Option<&ModifyVerifier>,pwd:Option<&str>)->bool{
        // Java: String saltData = srcModifyVer.getSaltData();
        let srcModifyVer=srcModifyVer.expect("NullPointerException");
        let saltData = srcModifyVer.getSaltData().map(str::to_owned);
        // Java: String hashData = srcModifyVer.getHashData();
        let hashData = srcModifyVer.getHashData().map(str::to_owned);
        // Java: int spinCount = srcModifyVer.getSpinCount();
        let spinCount = srcModifyVer.getSpinCount();
        // Java: int algothmId = srcModifyVer.getAlogrithmSid();
        let algothmId = srcModifyVer.getAlogrithmSid();
        // Java: boolean isPasswordOk = false;
        let mut isPasswordOk = false;
        // Java: StringBuilder saltBuilder = new StringBuilder(saltData);
        let mut saltBuilder = saltData.clone().expect("NullPointerException");
        // Java: int addcharCount = saltData.length() % 3;
        let mut addcharCount = saltData.as_ref().unwrap().encode_utf16().count() % 3;
        // Java: if (addcharCount != 0)
        if addcharCount != 0 {
            // Java: addcharCount = 3 - addcharCount;
            addcharCount = 3 - addcharCount;
            // Java: for(int i = 0; i < addcharCount; ++i)
            for _i in 0..addcharCount {
                // Java: saltBuilder.append('=');
                saltBuilder.push('=');
            }
        }
        // Java: byte[] salt = Base64.base64ToByteArray(saltBuilder.toString());
        let salt = Base64::base64ToByteArray_simple(&saltBuilder);
        // Java: IDigest sha = null;
        let mut sha: Box<dyn IDigest>;
        // Java: if (algothmId == 4)
        if algothmId == 4 {
            // Java: sha = new DigestSHA1();
            sha = Box::new(DigestSHA1::new());
        }
        // Java: else
        else {
            // Java: if (algothmId != 14)
            if algothmId != 14 {
                // Java: IOAssert.fail("UnsupportedAlgorithm!");
                // Java: return false;
                return false;
            }
            // Java: sha = new DigestSHA512();
            sha = Box::new(DigestSHA512::new());
        }
        // Java: byte[] bytes;
        let bytes: Vec<u8>;
        // Java: try
        {
            // Java: bytes = pwd.getBytes("UTF-16LE");
            bytes = pwd.expect("NullPointerException").encode_utf16().flat_map(|c| c.to_le_bytes()).collect();
        }
        // Java: catch (UnsupportedEncodingException var16)
        {
            // Java: throw new EncryptedDocumentException("UTF16 not supported");
        }
        // Java: int hashLen = sha.getLength();
        let hashLen = sha.get_length();
        // Java: byte[] hash = new byte[hashLen];
        let mut hash = vec![0u8; hashLen as usize];
        // Java: sha.update(salt, 0, salt.length);
        sha.update(&salt, 0, salt.len() as i32);
        // Java: sha.update(bytes, 0, bytes.length);
        sha.update(&bytes, 0, bytes.len() as i32);
        // Java: sha.get(hash);
        sha.get(&mut hash);
        // Java: byte[] iterator = new byte[4];
        let mut iterator = vec![0u8; 4];
        // Java: for(int i = 0; i < spinCount; ++i)
        for i in 0..spinCount {
            // Java: iterator[0] = (byte)(i >>> 0 & 255);
            iterator[0] = ((i as u32 >> 0) & 255) as u8;
            // Java: iterator[1] = (byte)(i >>> 8 & 255);
            iterator[1] = ((i as u32 >> 8) & 255) as u8;
            // Java: iterator[2] = (byte)(i >>> 16 & 255);
            iterator[2] = ((i as u32 >> 16) & 255) as u8;
            // Java: iterator[3] = (byte)(i >>> 24 & 255);
            iterator[3] = ((i as u32 >> 24) & 255) as u8;
            // Java: sha.update(hash, 0, hash.length);
            sha.update(&hash, 0, hash.len() as i32);
            // Java: sha.update(iterator, 0, 4);
            sha.update(&iterator, 0, 4);
            // Java: sha.get(hash);
            sha.get(&mut hash);
        }
        // Java: String hashStr = Base64.byteArrayToBase64(hash);
        let hashStr = Base64::byteArrayToBase64_simple(&hash);
        // Java: if (hashStr.startsWith(hashData))
        if hashStr.starts_with(hashData.as_deref().expect("NullPointerException")) {
            // Java: isPasswordOk = true;
            isPasswordOk = true;
        }
        // Java: return isPasswordOk;
        isPasswordOk
    }

    // Java: static ModifyVerifier createModifyVerifier(String password)
    pub fn createModifyVerifier(password: &str) -> ModifyVerifier {
        // Java: String saltData = "YkYhGRU3qoMzbKivmTBKIg==";
        let saltData = "YkYhGRU3qoMzbKivmTBKIg==";
        // Java: StringBuilder saltBuilder = new StringBuilder(saltData);
        let saltBuilder = saltData.to_string();
        // Java: int spinCount = 100000;
        let spinCount = 100000;
        // Java: byte[] salt = Base64.base64ToByteArray(saltBuilder.toString());
        let salt = Base64::base64ToByteArray_simple(&saltBuilder);
        // Java: DigestSHA1 sha = new DigestSHA1();
        let mut sha = DigestSHA1::new();
        // Java: byte[] bytes;
        let bytes: Vec<u8>;
        // Java: try
        {
            // Java: bytes = password.getBytes("UTF-16LE");
            bytes = password.encode_utf16().flat_map(|c| c.to_le_bytes()).collect();
        }
        // Java: catch (UnsupportedEncodingException var11)
        {
            // Java: throw new EncryptedDocumentException("UTF16 not supported");
        }
        // Java: sha.update(salt, 0, salt.length);
        sha.update_data(&salt, 0, salt.len() as i32);
        // Java: sha.update(bytes, 0, bytes.length);
        sha.update_data(&bytes, 0, bytes.len() as i32);
        // Java: byte[] hash = new byte[20];
        let mut hash = vec![0u8; 20];
        // Java: sha.get(hash);
        sha.get_digest(&mut hash);
        // Java: byte[] iterator = new byte[4];
        let mut iterator = vec![0u8; 4];
        // Java: for(int i = 0; i < spinCount; ++i)
        for i in 0..spinCount {
            // Java: iterator[0] = (byte)(i >>> 0 & 255);
            iterator[0] = ((i as u32 >> 0) & 255) as u8;
            // Java: iterator[1] = (byte)(i >>> 8 & 255);
            iterator[1] = ((i as u32 >> 8) & 255) as u8;
            // Java: iterator[2] = (byte)(i >>> 16 & 255);
            iterator[2] = ((i as u32 >> 16) & 255) as u8;
            // Java: iterator[3] = (byte)(i >>> 24 & 255);
            iterator[3] = ((i as u32 >> 24) & 255) as u8;
            // Java: sha.update(hash, 0, hash.length);
            sha.update_data(&hash, 0, hash.len() as i32);
            // Java: sha.update(iterator, 0, 4);
            sha.update_data(&iterator, 0, 4);
            // Java: sha.get(hash);
            sha.get_digest(&mut hash);
        }
        // Java: String hashData = Base64.byteArrayToBase64(hash);
        let hashData = Base64::byteArrayToBase64_simple(&hash);
        // Java: ModifyVerifier mVerifier = new ModifyVerifier(hashData, saltData, spinCount, 4);
        let mVerifier = ModifyVerifier::new_with_algorithm(&hashData, saltData, spinCount, 4);
        // Java: return mVerifier;
        mVerifier
    }

    // Java: static ModifyVerifier createModifyVerifierSHA512(String password)
    pub fn createModifyVerifierSHA512(password: &str) -> ModifyVerifier {
        // rust: existing non-null transport.
        Self::createModifyVerifierSHA512__nullable(Some(password))
    }
    // java public static ModifyVerifier createModifyVerifierSHA512(String password) {
    pub fn createModifyVerifierSHA512__nullable(password:Option<&str>)->ModifyVerifier{
        // Java: String saltData = "YkYhGRU3qoMzbKivmTBKIg==";
        let saltData = "YkYhGRU3qoMzbKivmTBKIg==";
        // Java: StringBuilder saltBuilder = new StringBuilder(saltData);
        let saltBuilder = saltData.to_string();
        // Java: int spinCount = 100000;
        let spinCount = 100000;
        // Java: byte[] salt = Base64.base64ToByteArray(saltBuilder.toString());
        let salt = Base64::base64ToByteArray_simple(&saltBuilder);
        // Java: DigestSHA512 sha = new DigestSHA512();
        let mut sha = DigestSHA512::new();
        // Java: byte[] bytes;
        let bytes: Vec<u8>;
        // Java: try
        {
            // Java: bytes = password.getBytes("UTF-16LE");
            bytes = password.expect("NullPointerException").encode_utf16().flat_map(|c| c.to_le_bytes()).collect();
        }
        // Java: catch (UnsupportedEncodingException var11)
        {
            // Java: throw new EncryptedDocumentException("UTF16 not supported");
        }
        // Java: sha.update(salt, 0, salt.length);
        sha.update_data(&salt, 0, salt.len() as i32);
        // Java: sha.update(bytes, 0, bytes.length);
        sha.update_data(&bytes, 0, bytes.len() as i32);
        // Java: byte[] hash = new byte[sha.getLength()];
        let hash_len = sha.getLength() as usize;
        let mut hash = vec![0u8; hash_len];
        // Java: sha.get(hash);
        sha.get_digest(&mut hash);
        // Java: byte[] iterator = new byte[4];
        let mut iterator = vec![0u8; 4];
        // Java: for(int i = 0; i < spinCount; ++i)
        for i in 0..spinCount {
            // Java: iterator[0] = (byte)(i >>> 0 & 255);
            iterator[0] = ((i as u32 >> 0) & 255) as u8;
            // Java: iterator[1] = (byte)(i >>> 8 & 255);
            iterator[1] = ((i as u32 >> 8) & 255) as u8;
            // Java: iterator[2] = (byte)(i >>> 16 & 255);
            iterator[2] = ((i as u32 >> 16) & 255) as u8;
            // Java: iterator[3] = (byte)(i >>> 24 & 255);
            iterator[3] = ((i as u32 >> 24) & 255) as u8;
            // Java: sha.update(hash, 0, hash.length);
            sha.update_data(&hash, 0, hash.len() as i32);
            // Java: sha.update(iterator, 0, 4);
            sha.update_data(&iterator, 0, 4);
            // Java: sha.get(hash);
            sha.get_digest(&mut hash);
        }
        // Java: String hashData = Base64.byteArrayToBase64(hash);
        let hashData = Base64::byteArrayToBase64_simple(&hash);
        // Java: ModifyVerifier mVerifier = new ModifyVerifier(hashData, saltData, spinCount, 14);
        let mVerifier = ModifyVerifier::new_with_algorithm(&hashData, saltData, spinCount, 14);
        // Java: return mVerifier;
        mVerifier
    }

    // Java: String hashModifyPassword(String saltData, String password)
    pub fn hashModifyPassword(&self, saltData: &str, password: &str) -> String {
        // Java: MessageDigest sha1 = MessageDigest.getInstance("SHA-1");
        // Java: int saltLen = saltData.length();
        let _saltLen = saltData.len();
        // Java: StringBuilder saltBuilder = new StringBuilder(saltData);
        let mut saltBuilder = saltData.to_string();
        // Java: int count = 100000;
        let mut count = 100000;
        // Java: if (saltData.length() != 24)
        if saltData.len() != 24 {
            // Java: count = 50000;
            count = 50000;
            // Java: int strCount = 24 - saltData.length();
            let strCount = 24i32 - saltData.len() as i32;
            // Java: for(int i = 0; i < strCount; ++i)
            for _i in 0..strCount {
                // Java: saltBuilder.append('=');
                saltBuilder.push('=');
            }
        }
        // Java: byte[] salt = Base64.base64ToByteArray(saltBuilder.toString());
        let salt = Base64::base64ToByteArray_simple(&saltBuilder);
        // Java: DigestSHA1 sha = new DigestSHA1();
        let mut sha = DigestSHA1::new();
        // Java: byte[] bytes;
        let bytes: Vec<u8>;
        // Java: try
        {
            // Java: bytes = password.getBytes("UTF-16LE");
            bytes = password.encode_utf16().flat_map(|c| c.to_le_bytes()).collect();
        }
        // Java: catch (UnsupportedEncodingException var13)
        {
            // Java: throw new EncryptedDocumentException("UTF16 not supported");
        }
        // Java: sha.update(salt, 0, salt.length);
        sha.update_data(&salt, 0, salt.len() as i32);
        // Java: sha.update(bytes, 0, bytes.length);
        sha.update_data(&bytes, 0, bytes.len() as i32);
        // Java: byte[] hash = new byte[20];
        let mut hash = vec![0u8; 20];
        // Java: sha.get(hash);
        sha.get_digest(&mut hash);
        // Java: byte[] iterator = new byte[4];
        let mut iterator = vec![0u8; 4];
        // Java: for(int i = 0; i < count; ++i)
        for i in 0..count {
            // Java: iterator[0] = (byte)(i >>> 0 & 255);
            iterator[0] = ((i as u32 >> 0) & 255) as u8;
            // Java: iterator[1] = (byte)(i >>> 8 & 255);
            iterator[1] = ((i as u32 >> 8) & 255) as u8;
            // Java: iterator[2] = (byte)(i >>> 16 & 255);
            iterator[2] = ((i as u32 >> 16) & 255) as u8;
            // Java: iterator[3] = (byte)(i >>> 24 & 255);
            iterator[3] = ((i as u32 >> 24) & 255) as u8;
            // Java: sha.update(hash, 0, hash.length);
            sha.update_data(&hash, 0, hash.len() as i32);
            // Java: sha.update(iterator, 0, 4);
            sha.update_data(&iterator, 0, 4);
            // Java: sha.get(hash);
            sha.get_digest(&mut hash);
        }
        // Java: return Base64.byteArrayToBase64(hash);
        Base64::byteArrayToBase64_simple(&hash)
    }

    // Java: byte[] truncateOrPad(byte[] source, int length)
    pub fn truncateOrPad(source: &[u8], length: usize) -> Vec<u8> {
        // Java: byte[] result = new byte[length];
        let mut result = vec![0u8; length];
        // Java: System.arraycopy(source, 0, result, 0, Math.min(length, source.length));
        let copy_len = std::cmp::min(length, source.len());
        result[..copy_len].copy_from_slice(&source[..copy_len]);
        // Java: if (length > source.length)
        if length > source.len() {
            // Java: for(int i = source.length; i < length; ++i)
            for i in source.len()..length {
                // Java: result[i] = 0;
                result[i] = 0;
            }
        }
        // Java: return result;
        result
    }

    // Java: SecretKey getKey()
    pub fn getKey(&self) -> Option<&[u8]> {
        // Java: return this.key;
        self.key.as_deref()
    }

    // Java: void setPasswordHash(byte[] passwordHash)
    pub fn setPasswordHash(&mut self, passwordHash: Vec<u8>) {
        // Java: this.passwordHash = passwordHash;
        self.passwordHash = Some(passwordHash);
    }

    // Java: byte[] generateVerifier()
    pub fn generateVerifier(&self) -> Option<Vec<u8>> {
        // Java: return null;
        None
    }
}

pub struct AesCipher {
    pub key: Vec<u8>,
    pub mode: i32,
}

impl AesCipher {
    pub fn doFinal(&self, _input: &[u8]) -> Vec<u8> {
        _input.to_vec()
    }
}
