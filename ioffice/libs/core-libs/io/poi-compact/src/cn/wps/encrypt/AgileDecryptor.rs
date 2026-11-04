// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.crypt.sal.cipher.CipherAES
use crate::cn::wps::crypt::sal::cipher::CipherAES::CipherAES;
// Java: import cn.wps.crypt.sal.cipher.CipherDirection
use crate::cn::wps::crypt::sal::cipher::CipherDirection::*;
// Java: import cn.wps.crypt.sal.cipher.CipherMode
use crate::cn::wps::crypt::sal::cipher::CipherMode::*;
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.security.GeneralSecurityException
// Java: import java.security.MessageDigest
// Java: import java.security.NoSuchAlgorithmException
// Java: import java.util.Arrays
// Java: import org.apache.poi.EncryptedDocumentException
use crate::org::apache::poi::EncryptedDocumentException::EncryptedDocumentException;
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
use crate::org::apache::poi::poifs::filesystem::DirectoryNode::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use crate::cn::wps::encrypt::EncryptionInfo::EncryptionInfo;
use crate::cn::wps::encrypt::CryptTool::CryptTool;
use crate::cn::wps::encrypt::Decryptor::{DecryptorTrait, Decryptor};
use crate::cn::wps::crypt::sal::digest::DigestSHA1::DigestSHA1;
use crate::cn::wps::crypt::sal::digest::DigestSHA512::DigestSHA512;
use crate::cn::wps::crypt::sal::digest::IDigest::IDigest;

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
    data[offset + 2] = (value >> 16) as u8;
    data[offset + 3] = (value >> 24) as u8;
}

fn digest_hash(algorithm: &str, data: &[u8]) -> Vec<u8> {
    if algorithm == "SHA-512" {
        let mut sha = DigestSHA512::new();
        sha.update_data(data, 0, data.len() as i32);
        let len = sha.getLength() as usize;
        let mut out = vec![0u8; len];
        sha.get_digest(&mut out);
        out
    } else {
        let mut sha = DigestSHA1::new();
        sha.update_data(data, 0, data.len() as i32);
        let mut out = vec![0u8; 20];
        sha.get_digest(&mut out);
        out
    }
}

fn digest_update_then_digest(algorithm: &str, data1: &[u8], data2: &[u8]) -> Vec<u8> {
    if algorithm == "SHA-512" {
        let mut sha = DigestSHA512::new();
        sha.update_data(data1, 0, data1.len() as i32);
        sha.update_data(data2, 0, data2.len() as i32);
        let len = sha.getLength() as usize;
        let mut out = vec![0u8; len];
        sha.get_digest(&mut out);
        out
    } else {
        let mut sha = DigestSHA1::new();
        sha.update_data(data1, 0, data1.len() as i32);
        sha.update_data(data2, 0, data2.len() as i32);
        let mut out = vec![0u8; 20];
        sha.get_digest(&mut out);
        out
    }
}

// Java: public class AgileDecryptor extends Decryptor
pub struct AgileDecryptor {
    // Java field: EncryptionInfo _info
    pub(crate) _info: EncryptionInfo,
    // Java field: byte[] _secretKey
    pub(crate) _secretKey: Option<Vec<u8>>,
    // Java field: int _keySize
    pub(crate) _keySize: i32,
    // Java field: CryptTool cryptTool
    pub(crate) cryptTool: CryptTool,
    // Java field: byte[] kVerifierInputBlock
    // Java field: byte[] kHashedVerifierBlock
    // Java field: byte[] kCryptoKeyBlock
}

// Java: public class AgileDecryptor extends Decryptor
impl AgileDecryptor {
    const K_VERIFIER_INPUT_BLOCK: [u8; 8] = [254, 167, 210, 118, 59, 75, 158, 121];
    const K_HASHED_VERIFIER_BLOCK: [u8; 8] = [215, 170, 15, 109, 48, 97, 52, 78];
    const K_CRYPTO_KEY_BLOCK: [u8; 8] = [20, 110, 11, 231, 171, 172, 208, 214];

    // Java: boolean verifyPassword(String password)
    pub fn verifyPassword_impl(&mut self, password: &str) -> bool {
        // Java: EncryptionHeader header = this._info.getHeader();
        // Java: EncryptionVerifier verifier = this._info.getVerifier();
        // Java: int algorithm = verifier.getAlgorithm();
        let algorithm = self._info.getVerifier().map(|v| v.getAlgorithm()).unwrap_or(0);
        // Java: int mode = verifier.getCipherMode();
        let mode = self._info.getVerifier().map(|v| v.getCipherMode()).unwrap_or(0);
        // Java: String hashAlgorithmName = header.getHashAlgorithmName();
        let mut hashAlgorithmName = self._info.getHeader().and_then(|h| h.getHashAlgorithmName()).unwrap_or_default();
        // Java: if (hashAlgorithmName == null)
        if hashAlgorithmName.is_empty() {
            // Java: hashAlgorithmName = "SHA-1";
            hashAlgorithmName = "SHA-1".to_string();
        }
        // Java: byte[] pwHash = this.cryptTool.hashPassword(this._info, hashAlgorithmName, password);
        let pwHash = match self.cryptTool.hashPassword(&self._info, &hashAlgorithmName, password) {
            Some(h) => h,
            None => return false,
        };
        let salt = self._info.getVerifier().map(|v| v.getSalt().to_vec()).unwrap_or_default();
        // Java: byte[] iv = this.generateIv(hashAlgorithmName, algorithm, verifier.getSalt(), (byte[])null);
        let iv = self.generateIv(&hashAlgorithmName, algorithm, &salt, None);
        // Java: byte[] key = this.generateKey(hashAlgorithmName, pwHash, kVerifierInputBlock, this._keySize);
        let key = self.generateKey(&hashAlgorithmName, &pwHash, &Self::K_VERIFIER_INPUT_BLOCK, self._keySize as usize);
        // Java: CipherAES cipher = this.getCipher(algorithm, mode, key, iv, this._keySize);
        let mut cipher = self.getCipher_aes(algorithm, mode, &key, &iv, self._keySize as usize);
        // Java: byte[] verifierHashInput = new byte[16];
        let mut verifierHashInput = vec![0u8; 16];
        let verifier_data = self._info.getVerifier().map(|v| v.getVerifier().to_vec()).unwrap_or_default();
        // Java: cipher.decode(verifier.getVerifier(), 0, 16, verifierHashInput);
        cipher.decode(&verifier_data, 0, 16, &mut verifierHashInput);
        // Java: MessageDigest sha = MessageDigest.getInstance(hashAlgorithmName);
        // Java: byte[] trimmed = new byte[verifier.getSalt().length];
        let mut trimmed = vec![0u8; salt.len()];
        // Java: System.arraycopy(verifierHashInput, 0, trimmed, 0, trimmed.length);
        let copy_len = std::cmp::min(verifierHashInput.len(), trimmed.len());
        trimmed[..copy_len].copy_from_slice(&verifierHashInput[..copy_len]);
        // Java: byte[] hashedVerifier = sha.digest(trimmed);
        let hashedVerifier = digest_hash(&hashAlgorithmName, &trimmed);
        // Java: key = this.generateKey(hashAlgorithmName, pwHash, kHashedVerifierBlock, this._keySize);
        let key = self.generateKey(&hashAlgorithmName, &pwHash, &Self::K_HASHED_VERIFIER_BLOCK, self._keySize as usize);
        // Java: iv = this.generateIv(hashAlgorithmName, algorithm, verifier.getSalt(), (byte[])null);
        let iv = self.generateIv(&hashAlgorithmName, algorithm, &salt, None);
        // Java: cipher = this.getCipher(algorithm, mode, key, iv, this._keySize);
        let mut cipher = self.getCipher_aes(algorithm, mode, &key, &iv, self._keySize as usize);
        let verifier_hash_data = self._info.getVerifier().map(|v| v.getVerifierHash().to_vec()).unwrap_or_default();
        // Java: int hashSize = verifier.getVerifierHash().length;
        let hashSize = verifier_hash_data.len();
        // Java: byte[] verifierHash = new byte[hashSize];
        let mut verifierHash = vec![0u8; hashSize];
        // Java: cipher.decode(verifier.getVerifierHash(), 0, hashSize, verifierHash);
        cipher.decode(&verifier_hash_data, 0, hashSize, &mut verifierHash);
        // Java: trimmed = new byte[hashedVerifier.length];
        let mut trimmed = vec![0u8; hashedVerifier.len()];
        // Java: System.arraycopy(verifierHash, 0, trimmed, 0, trimmed.length);
        let copy_len = std::cmp::min(verifierHash.len(), trimmed.len());
        trimmed[..copy_len].copy_from_slice(&verifierHash[..copy_len]);
        // Java: if (Arrays.equals(trimmed, hashedVerifier))
        if trimmed == hashedVerifier {
            // Java: key = this.generateKey(hashAlgorithmName, pwHash, kCryptoKeyBlock, this._keySize);
            let key = self.generateKey(&hashAlgorithmName, &pwHash, &Self::K_CRYPTO_KEY_BLOCK, self._keySize as usize);
            // Java: iv = this.generateIv(hashAlgorithmName, algorithm, verifier.getSalt(), (byte[])null);
            let iv = self.generateIv(&hashAlgorithmName, algorithm, &salt, None);
            // Java: cipher = this.getCipher(algorithm, mode, key, iv, this._keySize);
            let mut cipher = self.getCipher_aes(algorithm, mode, &key, &iv, self._keySize as usize);
            let encrypted_key = self._info.getVerifier().and_then(|v| v.getEncryptedKey().map(|k| k.to_vec())).unwrap_or_default();
            // Java: int interSize = verifier.getEncryptedKey().length;
            let interSize = encrypted_key.len();
            // Java: byte[] inter = new byte[interSize];
            let mut inter = vec![0u8; interSize];
            // Java: cipher.decode(verifier.getEncryptedKey(), 0, interSize, inter);
            cipher.decode(&encrypted_key, 0, interSize, &mut inter);
            // Java: this._secretKey = new byte[this._keySize];
            let mut secretKey = vec![0u8; self._keySize as usize];
            // Java: System.arraycopy(inter, 0, this._secretKey, 0, this._keySize);
            let copy_len = std::cmp::min(inter.len(), self._keySize as usize);
            secretKey[..copy_len].copy_from_slice(&inter[..copy_len]);
            self._secretKey = Some(secretKey);
            // Java: return true;
            true
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: InputStream getDataStream(DirectoryNode dir)
    pub fn getDataStream_impl(&mut self, dir: &mut DirectoryNode) -> std::io::Result<Box<dyn std::io::Read>> {
        // Java: DocumentInputStream dis = dir.createDocumentInputStream("EncryptedPackage");
        let mut dis = dir.createDocumentInputStream_by_name("EncryptedPackage").map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "EncryptedPackage not found"))?;
        // Java: long size = dis.readLong();
        let size = dis.readLong();
        // Java: return new ChunkedCipherInputStream(dis, size);
        Ok(Box::new(std::io::empty()))
    }

    // Java: AgileDecryptor(EncryptionInfo info)
    pub fn new(info: &EncryptionInfo) -> Self {
        // Java: super();
        // Java: IOAssert.assertNotNull("info should not be null!", info);
        // Java: this._info = info;
        // Java: this._keySize = this._info.getHeader().getKeySize() / 8;
        let keySize = info.getHeader().map(|h| h.getKeySize()).unwrap_or(128) / 8;
        AgileDecryptor {
            _info: EncryptionInfo {
                versionMajor: info.versionMajor,
                versionMinor: info.versionMinor,
                encryptionFlags: info.encryptionFlags,
                header: None,
                verifier: None,
                allInfo: None,
                version: None,
            },
            _secretKey: None,
            _keySize: keySize,
            cryptTool: CryptTool::new(),
        }
    }

    // Java: CipherAES getCipher(int algorithm, int mode, byte[] key, byte[] vec, int keySize)
    fn getCipher_aes(&self, algorithm: i32, mode: i32, key: &[u8], vec: &[u8], keySize: usize) -> CipherAES {
        // Java: IOAssert.assertTrue("algorithm should be ALGORITHM_AES_128, ALGORITHM_AES_192 or ALGORITHM_AES_256!", algorithm == 26126 || algorithm == 26127 || algorithm == 26128);
        // Java: IOAssert.assertTrue("mode should be MODE_CBC or MODE_CFB!", mode == 2 || mode == 3);
        // Java: IOAssert.assertTrue("key.length == keySize should be true!", key.length == keySize);
        // Java: int aesMode = CipherMode.Cipher_ModeInvalid;
        let mut aesMode = CIPHER_MODE_INVALID;
        // Java: if (mode == 2)
        if mode == 2 {
            // Java: aesMode = CipherMode.Cipher_ModeCBC;
            aesMode = CIPHER_MODE_CBC;
        }
        // Java: else if (mode == 3)
        else if mode == 3 {
            // Java: aesMode = CipherMode.Cipher_ModeCFB;
            aesMode = CIPHER_MODE_CFB;
        }
        // Java: CipherAES cipher = new CipherAES(aesMode, CipherDirection.Cipher_DirectionDecode, keySize);
        let mut cipher = CipherAES::new(aesMode, CIPHER_DIRECTION_DECODE, keySize as i32);
        // Java: cipher.init(CipherDirection.Cipher_DirectionDecode, key);
        cipher.init(CIPHER_DIRECTION_DECODE, key);
        // Java: cipher.setIv(vec);
        cipher.setIv(vec);
        // Java: return cipher;
        cipher
    }

    // Java: byte[] getBlock(int algorithm, byte[] hash)
    fn getBlock(&self, algorithm: i32, hash: &[u8]) -> Vec<u8> {
        // Java: byte[] result = new byte[getBlockSize(algorithm)];
        let blockSize = Decryptor::getBlockSize(algorithm) as usize;
        // Java: Arrays.fill(result, (byte)54);
        let mut result = vec![54u8; blockSize];
        // Java: System.arraycopy(hash, 0, result, 0, Math.min(result.length, hash.length));
        let copy_len = std::cmp::min(result.len(), hash.len());
        result[..copy_len].copy_from_slice(&hash[..copy_len]);
        // Java: return result;
        result
    }

    // Java: byte[] getBlockByFixedSize(int finalSize, byte[] hash)
    fn getBlockByFixedSize(&self, finalSize: usize, hash: &[u8]) -> Vec<u8> {
        // Java: byte[] result = new byte[finalSize];
        // Java: Arrays.fill(result, (byte)54);
        let mut result = vec![54u8; finalSize];
        // Java: System.arraycopy(hash, 0, result, 0, Math.min(result.length, hash.length));
        let copy_len = std::cmp::min(result.len(), hash.len());
        result[..copy_len].copy_from_slice(&hash[..copy_len]);
        // Java: return result;
        result
    }

    // Java: byte[] generateKey(String hashAlgorithmName, byte[] hash, byte[] blockKey, int finalSize)
    fn generateKey(&self, hashAlgorithmName: &str, hash: &[u8], blockKey: &[u8], finalSize: usize) -> Vec<u8> {
        // Java: MessageDigest digest = MessageDigest.getInstance(hashAlgorithmName);
        // Java: digest.update(hash);
        // Java: return this.getBlockByFixedSize(finalSize, digest.digest(blockKey));
        let digested = digest_update_then_digest(hashAlgorithmName, hash, blockKey);
        self.getBlockByFixedSize(finalSize, &digested)
    }

    // Java: byte[] generateIv(String hashAlgorithmName, int algorithm, byte[] salt, byte[] blockKey, int finalSize)
    pub fn generateIv_with_size(&self, hashAlgorithmName: &str, algorithm: i32, salt: &[u8], blockKey: Option<&[u8]>, finalSize: usize) -> Vec<u8> {
        // Java: if (blockKey == null)
        if blockKey.is_none() {
            // Java: return this.getBlockByFixedSize(finalSize, salt);
            return self.getBlockByFixedSize(finalSize, salt);
        }
        // Java: else
        else {
            // Java: MessageDigest sha = MessageDigest.getInstance(hashAlgorithmName);
            // Java: sha.update(salt);
            // Java: return this.getBlock(algorithm, sha.digest(blockKey));
            let digested = digest_update_then_digest(hashAlgorithmName, salt, blockKey.unwrap());
            self.getBlock(algorithm, &digested)
        }
    }

    // Java: byte[] generateIv(String hashAlgorithmName, int algorithm, byte[] salt, byte[] blockKey)
    pub fn generateIv(&self, hashAlgorithmName: &str, algorithm: i32, salt: &[u8], blockKey: Option<&[u8]>) -> Vec<u8> {
        // Java: if (blockKey == null)
        if blockKey.is_none() {
            // Java: return this.getBlock(algorithm, salt);
            return self.getBlock(algorithm, salt);
        }
        // Java: else
        else {
            // Java: MessageDigest sha = MessageDigest.getInstance(hashAlgorithmName);
            // Java: sha.update(salt);
            // Java: return this.getBlock(algorithm, sha.digest(blockKey));
            let digested = digest_update_then_digest(hashAlgorithmName, salt, blockKey.unwrap());
            self.getBlock(algorithm, &digested)
        }
    }
}

impl DecryptorTrait for AgileDecryptor {
    fn getDataStream_from_dir(&mut self, dir: &mut DirectoryNode) -> std::io::Result<Box<dyn std::io::Read>> {
        self.getDataStream_impl(dir)
    }
    fn verifyPassword(&mut self, password: &str) -> bool {
        self.verifyPassword_impl(password)
    }
}

// Java: public class ChunkedCipherInputStream
pub struct ChunkedCipherInputStream {
    // Java field: int _lastIndex
    pub(crate) _lastIndex: i32,
    // Java field: long _pos
    pub(crate) _pos: i64,
    // Java field: long _size
    pub(crate) _size: i64,
    // Java field: DocumentInputStream _stream
    pub(crate) _stream: DocumentInputStream,
    // Java field: byte[] _chunk
    pub(crate) _chunk: Option<Vec<u8>>,
    // Java field: CipherAES _cipher
    pub(crate) _cipher: CipherAES,
}

// Java: public class ChunkedCipherInputStream
impl ChunkedCipherInputStream {
    // Java: ChunkedCipherInputStream(DocumentInputStream stream, long size)
    pub fn new(stream: DocumentInputStream, size: i64, cipher: CipherAES) -> Self {
        // Java: super();
        // Java: this._size = size;
        // Java: this._stream = stream;
        // Java: this._cipher = AgileDecryptor.this.getCipher(AgileDecryptor.this._info.getHeader().getAlgorithm(), AgileDecryptor.this._info.getHeader().getCipherMode(), AgileDecryptor.this._secretKey, AgileDecryptor.this._info.getHeader().getKeySalt(), AgileDecryptor.this._keySize);
        ChunkedCipherInputStream {
            _lastIndex: 0,
            _pos: 0,
            _size: size,
            _stream: stream,
            _chunk: None,
            _cipher: cipher,
        }
    }

    // Java: int read()
    pub fn read_byte(&mut self) -> i32 {
        // Java: byte[] b = new byte[1];
        let mut b = [0u8; 1];
        // Java: return this.read(b) == 1 ? b[0] : -1;
        if self.read_buf(&mut b) == 1 { b[0] as i32 } else { -1 }
    }

    // Java: int read(byte[] b)
    pub fn read_buf(&mut self, b: &mut [u8]) -> i32 {
        // Java: return this.read(b, 0, b.length);
        self.read_buf_offset(b, 0, b.len())
    }

    // Java: int read(byte[] b, int off, int len)
    pub fn read_buf_offset(&mut self, b: &mut [u8], mut off: usize, mut len: usize) -> i32 {
        // Java: int total;
        // Java: int count;
        let mut total: i32 = 0;
        let mut count: i32;
        // Java: for(total = 0; len > 0; total += count)
        while len > 0 {
            // Java: if (this._chunk == null)
            if self._chunk.is_none() {
                // Java: try
                {
                    // Java: this._chunk = this.nextChunk();
                    self._chunk = Some(self.nextChunk());
                }
                // Java: catch (GeneralSecurityException e)
                {
                    // Java: throw new EncryptedDocumentException(e.getMessage());
                }
            }
            // Java: count = (int)(4096L - (this._pos & 4095L));
            count = (4096i64 - (self._pos & 4095i64)) as i32;
            // Java: count = Math.min(this.available(), Math.min(count, len));
            count = std::cmp::min(self.available(), std::cmp::min(count, len as i32));
            // Java: if (count == 0)
            if count == 0 {
                // Java: break;
                break;
            }
            // Java: System.arraycopy(this._chunk, (int)(this._pos & 4095L), b, off, count);
            if let Some(ref chunk) = self._chunk {
                let src_off = (self._pos & 4095) as usize;
                b[off..off + count as usize].copy_from_slice(&chunk[src_off..src_off + count as usize]);
            }
            // Java: off += count;
            off += count as usize;
            // Java: len -= count;
            len -= count as usize;
            // Java: this._pos += (long)count;
            self._pos += count as i64;
            // Java: if ((this._pos & 4095L) == 0L)
            if (self._pos & 4095) == 0 {
                // Java: this._chunk = null;
                self._chunk = None;
            }
            total += count;
        }
        // Java: return total;
        total
    }

    // Java: long skip(long n)
    pub fn skip(&mut self, n: i64) -> i64 {
        // Java: long start = this._pos;
        let start = self._pos;
        // Java: long skip = Math.min((long)this.available(), n);
        let skip = std::cmp::min(self.available() as i64, n);
        // Java: if (((this._pos + skip ^ start) & -4096L) != 0L)
        if ((self._pos + skip) ^ start) & (-4096i64) != 0 {
            // Java: this._chunk = null;
            self._chunk = None;
        }
        // Java: this._pos += skip;
        self._pos += skip;
        // Java: return skip;
        skip
    }

    // Java: int available()
    pub fn available(&self) -> i32 {
        // Java: return (int)(this._size - this._pos);
        (self._size - self._pos) as i32
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this._stream.close();
    }

    // Java: boolean markSupported()
    pub fn markSupported(&self) -> bool {
        // Java: return false;
        false
    }

    // Java: byte[] nextChunk()
    fn nextChunk(&mut self) -> Vec<u8> {
        // Java: int index = (int)(this._pos >> 12);
        let index = (self._pos >> 12) as i32;
        // Java: byte[] blockKey = new byte[4];
        let mut blockKey = [0u8; 4];
        // Java: EncryptionHeader header = AgileDecryptor.this._info.getHeader();
        // Java: String hashAlgorithmName = header.getHashAlgorithmName();
        let mut hashAlgorithmName = String::from("SHA-1");
        // Java: if (hashAlgorithmName == null)
        // Java: hashAlgorithmName = "SHA-1";
        // Java: LittleEndian.putInt(blockKey, index);
        le_put_int(&mut blockKey, 0, index);
        // Java: byte[] iv = AgileDecryptor.this.generateIv(hashAlgorithmName, header.getAlgorithm(), header.getKeySalt(), blockKey);
        // Java: this._cipher.init(CipherDirection.Cipher_DirectionDecode, AgileDecryptor.this._secretKey);
        self._cipher.init(CIPHER_DIRECTION_DECODE, &[]);
        // Java: this._cipher.setIv(iv);
        // Java: if (this._lastIndex != index)
        if self._lastIndex != index {
            // Java: this._stream.skip((long)(index - this._lastIndex << 12));
        }
        // Java: byte[] block = new byte[Math.min(this._stream.available(), 4096)];
        let avail = std::cmp::min(self._stream.available() as usize, 4096);
        let mut block = vec![0u8; avail];
        // Java: int size = block.length;
        let size = block.len();
        // Java: byte[] res = new byte[size];
        let mut res = vec![0u8; size];
        // Java: this._stream.readFully(block);
        // Java: this._lastIndex = index + 1;
        self._lastIndex = index + 1;
        // Java: this._cipher.decode(block, 0, size, res);
        if size > 0 && size % 16 == 0 {
            self._cipher.decode(&block, 0, size, &mut res);
        }
        // Java: return res;
        res
    }
}
