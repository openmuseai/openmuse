// Java: import cn.wps.crypt.sal.digest.DigestSHA1
use crate::cn::wps::crypt::sal::digest::DigestSHA1::DigestSHA1;
// Java: import java.io.ByteArrayInputStream
// Java: import java.io.UnsupportedEncodingException
// Java: import java.nio.charset.StandardCharsets
// Java: import java.security.MessageDigest
// Java: import java.security.SecureRandom
// Java: import javax.crypto.Cipher
// Java: import javax.xml.parsers.DocumentBuilderFactory
// Java: import org.apache.poi.EncryptedDocumentException
use crate::org::apache::poi::EncryptedDocumentException::EncryptedDocumentException;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian
// Java: import org.w3c.dom.NamedNodeMap
// Java: import org.w3c.dom.Node
// Java: import org.w3c.dom.NodeList

use crate::cn::wps::encrypt::Base64::Base64;
use crate::cn::wps::encrypt::EncryptionHeader::EncryptionInfoInitTrait;
use std::collections::HashMap;

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
    data[offset + 2] = (value >> 16) as u8;
    data[offset + 3] = (value >> 24) as u8;
}

fn secure_random_bytes(buf: &mut [u8]) {
    for b in buf.iter_mut() {
        *b = rand_byte();
    }
}

static RAND_STATE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
fn rand_byte() -> u8 {
    let mut s = RAND_STATE.load(std::sync::atomic::Ordering::Relaxed);
    if s == 0 {
        s = 0xDEADBEEFCAFE1234;
    }
    s ^= s << 13;
    s ^= s >> 7;
    s ^= s << 17;
    RAND_STATE.store(s, std::sync::atomic::Ordering::Relaxed);
    s as u8
}

// Java: public class EncryptionVerifier
pub struct EncryptionVerifier {
    // Java field: int saltSize
    pub(crate) saltSize: i32,
    // Java field: byte[] salt
    pub(crate) salt: Vec<u8>,
    // Java field: byte[] verifier
    pub(crate) verifier: Vec<u8>,
    // Java field: byte[] verifierHash
    pub(crate) verifierHash: Vec<u8>,
    // Java field: byte[] encryptedKey
    pub(crate) encryptedKey: Option<Vec<u8>>,
    // Java field: int verifierHashSize
    pub(crate) verifierHashSize: i32,
    // Java field: int spinCount
    pub(crate) spinCount: i32,
    // Java field: int algorithm
    pub(crate) algorithm: i32,
    // Java field: int cipherMode
    pub(crate) cipherMode: i32,
    // Java field: Integer iOffset
    pub(crate) iOffset: Option<i32>,
    // Java field: byte[] passwordHash
    pub(crate) passwordHash: Option<Vec<u8>>,
    // Java field: SecureRandom secureRandom
}

// Java: public class EncryptionVerifier
impl EncryptionVerifier {
    // Java: EncryptionVerifier(String descriptor)
    pub fn new_from_descriptor(descriptor: &str) -> Self {
        // Java: super();
        // Java: NamedNodeMap keyData = null;
        let keyData: HashMap<String, String>;
        // Java: try
        {
            // Java: ByteArrayInputStream is = new ByteArrayInputStream(descriptor.getBytes(StandardCharsets.UTF_8));
            // Java: NodeList keyEncryptor = DocumentBuilderFactory.newInstance().newDocumentBuilder().parse(is).getElementsByTagName("keyEncryptor").item(0).getChildNodes();
            let doc = crate::xml_util::parse_xml(descriptor);
            let children = crate::xml_util::get_tag_children(&doc, "keyEncryptor");
            let mut found_key_data: Option<HashMap<String, String>> = None;
            // Java: for(int i = 0; i < keyEncryptor.getLength(); ++i)
            for child in &children {
                // Java: Node node = keyEncryptor.item(i);
                // Java: if (node.getNodeName().equals("p:encryptedKey"))
                if child.0 == "p:encryptedKey" {
                    // Java: keyData = node.getAttributes();
                    found_key_data = Some(child.1.clone());
                    // Java: break;
                    break;
                }
            }
            // Java: if (keyData == null)
            if found_key_data.is_none() {
                // Java: throw new EncryptedDocumentException("");
                panic!("EncryptedDocumentException");
            }
            keyData = found_key_data.unwrap();
        }
        // Java: catch (Exception var7)
        {
            // Java: throw new EncryptedDocumentException("Unable to parse keyEncryptor");
        }
        // Java: this.spinCount = Integer.parseInt(keyData.getNamedItem("spinCount").getNodeValue());
        let spinCount: i32 = keyData.get("spinCount").expect("spinCount").parse().unwrap();
        // Java: this.verifier = org.apache.poi.util.base64.Base64.decode(keyData.getNamedItem("encryptedVerifierHashInput").getNodeValue().getBytes(StandardCharsets.UTF_8));
        let verifier = Base64::base64ToByteArray_simple(keyData.get("encryptedVerifierHashInput").expect("encryptedVerifierHashInput"));
        // Java: this.salt = org.apache.poi.util.base64.Base64.decode(keyData.getNamedItem("saltValue").getNodeValue().getBytes(StandardCharsets.UTF_8));
        let salt = Base64::base64ToByteArray_simple(keyData.get("saltValue").expect("saltValue"));
        // Java: this.encryptedKey = org.apache.poi.util.base64.Base64.decode(keyData.getNamedItem("encryptedKeyValue").getNodeValue().getBytes(StandardCharsets.UTF_8));
        let encryptedKey = Some(Base64::base64ToByteArray_simple(keyData.get("encryptedKeyValue").expect("encryptedKeyValue")));
        // Java: int saltSize = Integer.parseInt(keyData.getNamedItem("saltSize").getNodeValue());
        let saltSize: i32 = keyData.get("saltSize").expect("saltSize").parse().unwrap();
        // Java: if (saltSize != this.salt.length)
        if saltSize != salt.len() as i32 {
            // Java: throw new EncryptedDocumentException("Invalid salt size");
            panic!("EncryptedDocumentException: Invalid salt size");
        }
        // Java: else
        // Java: this.verifierHash = org.apache.poi.util.base64.Base64.decode(keyData.getNamedItem("encryptedVerifierHashValue").getNodeValue().getBytes(StandardCharsets.UTF_8));
        let verifierHash = Base64::base64ToByteArray_simple(keyData.get("encryptedVerifierHashValue").expect("encryptedVerifierHashValue"));
        // Java: int blockSize = Integer.parseInt(keyData.getNamedItem("blockSize").getNodeValue());
        let blockSize: i32 = keyData.get("blockSize").expect("blockSize").parse().unwrap();
        // Java: String alg = keyData.getNamedItem("cipherAlgorithm").getNodeValue();
        let alg = keyData.get("cipherAlgorithm").expect("cipherAlgorithm").clone();
        let (algorithm, cipherMode, verifierHashSize);
        // Java: if ("AES".equals(alg))
        if alg == "AES" {
            // Java: if (blockSize == 16)
            if blockSize == 16 {
                // Java: this.algorithm = 26126;
                algorithm = 26126;
            }
            // Java: else if (blockSize == 24)
            else if blockSize == 24 {
                // Java: this.algorithm = 26127;
                algorithm = 26127;
            }
            // Java: else
            else {
                // Java: if (blockSize != 32)
                if blockSize != 32 {
                    // Java: throw new EncryptedDocumentException("Unsupported block size");
                    panic!("EncryptedDocumentException: Unsupported block size");
                }
                // Java: this.algorithm = 26128;
                algorithm = 26128;
            }
            // Java: String chain = keyData.getNamedItem("cipherChaining").getNodeValue();
            let chain = keyData.get("cipherChaining").expect("cipherChaining").clone();
            // Java: if ("ChainingModeCBC".equals(chain))
            if chain == "ChainingModeCBC" {
                // Java: this.cipherMode = 2;
                cipherMode = 2;
            }
            // Java: else
            else {
                // Java: if (!"ChainingModeCFB".equals(chain))
                if chain != "ChainingModeCFB" {
                    // Java: throw new EncryptedDocumentException("Unsupported chaining mode");
                    panic!("EncryptedDocumentException: Unsupported chaining mode");
                }
                // Java: this.cipherMode = 3;
                cipherMode = 3;
            }
            // Java: this.verifierHashSize = Integer.parseInt(keyData.getNamedItem("hashSize").getNodeValue());
            verifierHashSize = keyData.get("hashSize").expect("hashSize").parse().unwrap();
        }
        // Java: else
        else {
            // Java: throw new EncryptedDocumentException("Unsupported cipher");
            panic!("EncryptedDocumentException: Unsupported cipher");
        }
        EncryptionVerifier {
            saltSize,
            salt,
            verifier,
            verifierHash,
            encryptedKey,
            verifierHashSize,
            spinCount,
            algorithm,
            cipherMode,
            iOffset: None,
            passwordHash: None,
        }
    }

    // Java: EncryptionVerifier(DocumentInputStream is, int encryptedLength)
    pub fn new_from_stream(is: &mut DocumentInputStream, encryptedLength: i32) -> Self {
        // Java: super();
        // Java: int saltSize = is.readInt();
        let saltSize = is.readInt();
        // Java: if (saltSize != 16)
        if saltSize != 16 {
            // Java: throw new RuntimeException("Salt size != 16 !?");
            panic!("Salt size != 16 !?");
        }
        // Java: else
        // Java: this.salt = new byte[16];
        let mut salt = vec![0u8; 16];
        // Java: is.readFully(this.salt);
        is.readFully(&mut salt);
        // Java: this.verifier = new byte[16];
        let mut verifier = vec![0u8; 16];
        // Java: is.readFully(this.verifier);
        is.readFully(&mut verifier);
        // Java: this.verifierHashSize = is.readInt();
        let verifierHashSize = is.readInt();
        // Java: this.verifierHash = new byte[encryptedLength];
        let mut verifierHash = vec![0u8; encryptedLength as usize];
        // Java: is.readFully(this.verifierHash);
        is.readFully(&mut verifierHash);
        // Java: this.spinCount = 50000;
        // Java: this.algorithm = 26126;
        // Java: this.cipherMode = 1;
        // Java: this.encryptedKey = null;
        EncryptionVerifier {
            saltSize,
            salt,
            verifier,
            verifierHash,
            encryptedKey: None,
            verifierHashSize,
            spinCount: 50000,
            algorithm: 26126,
            cipherMode: 1,
            iOffset: None,
            passwordHash: None,
        }
    }

    // Java: EncryptionVerifier(byte[] allInfo, Integer iOffset, String passwd, boolean bol)
    pub fn new_from_bytes(allInfo: &mut [u8], iOffset: i32, passwd: &str, _bol: bool) -> Self {
        // Java: super();
        // Java: int offset = iOffset;
        let mut offset = iOffset as usize;
        // Java: LittleEndian.putInt(allInfo, offset, 16);
        le_put_int(allInfo, offset, 16);
        // Java: offset += 4;
        offset += 4;
        // Java: byte[] a = new byte[16];
        let mut a = vec![0u8; 16];
        // Java: secureRandom.nextBytes(a);
        secure_random_bytes(&mut a);
        // Java: this.salt = a;
        let salt = a;
        // Java: System.arraycopy(this.salt, 0, allInfo, offset, 16);
        allInfo[offset..offset + 16].copy_from_slice(&salt);
        // Java: offset += 16;
        offset += 16;
        // Java: byte[] b = new byte[16];
        let mut b = vec![0u8; 16];
        // Java: secureRandom.nextBytes(b);
        secure_random_bytes(&mut b);
        // Java: this.verifier = b;
        let verifier = b;
        // Java: System.arraycopy(this.verifier, 0, allInfo, offset, 16);
        allInfo[offset..offset + 16].copy_from_slice(&verifier);
        // Java: offset += 16;
        offset += 16;
        // Java: LittleEndian.putInt(allInfo, offset, 20);
        le_put_int(allInfo, offset, 20);
        // Java: offset += 4;
        offset += 4;
        let mut result = EncryptionVerifier {
            saltSize: 16,
            salt,
            verifier,
            verifierHash: Vec::new(),
            encryptedKey: None,
            verifierHashSize: 20,
            spinCount: 50000,
            algorithm: 26126,
            cipherMode: 1,
            iOffset: None,
            passwordHash: None,
        };
        // Java: this.generateVerifierHash(passwd, 50000);
        result.generateVerifierHash(passwd, 50000);
        // Java: System.arraycopy(this.verifierHash, 0, allInfo, offset, 32);
        let vh_len = std::cmp::min(result.verifierHash.len(), 32);
        allInfo[offset..offset + vh_len].copy_from_slice(&result.verifierHash[..vh_len]);
        // Java: offset += 32;
        offset += 32;
        // Java: this.iOffset = new Integer(offset);
        result.iOffset = Some(offset as i32);
        result
    }

    // Java: EncryptionVerifier(String passwd, EncryptionInfoInit infoInit)
    pub fn new_from_info_init(passwd: &str, infoInit: &mut dyn EncryptionInfoInitTrait) -> Self {
        // Java: super();
        // Java: byte[] a = new byte[16];
        let mut a = vec![0u8; 16];
        // Java: secureRandom.nextBytes(a);
        secure_random_bytes(&mut a);
        // Java: this.salt = a;
        let salt = a;
        // Java: infoInit.putNamedItem("verifySaltValue", this.salt);
        infoInit.putNamedItem_bytes("verifySaltValue", &salt);
        // Java: byte[] b = new byte[16];
        let mut b = vec![0u8; 16];
        // Java: secureRandom.nextBytes(b);
        secure_random_bytes(&mut b);
        // Java: this.verifier = b;
        let verifier = b;
        // Java: infoInit.putNamedItem("encryptedVerifierHashInput", this.verifier);
        infoInit.putNamedItem_bytes("encryptedVerifierHashInput", &verifier);
        let mut result = EncryptionVerifier {
            saltSize: 16,
            salt,
            verifier,
            verifierHash: Vec::new(),
            encryptedKey: None,
            verifierHashSize: 0,
            spinCount: 100000,
            algorithm: 26126,
            cipherMode: 1,
            iOffset: None,
            passwordHash: None,
        };
        // Java: this.generateVerifierHash(passwd, 100000);
        result.generateVerifierHash(passwd, 100000);
        // Java: infoInit.putNamedItem("encryptedVerifierHashValue", this.verifierHash);
        infoInit.putNamedItem_bytes("encryptedVerifierHashValue", &result.verifierHash);
        result
    }

    // Java: void generateVerifierHash(String password, int count)
    pub fn generateVerifierHash(&mut self, password: &str, count: i32) {
        // Java: CryptTool cryptTool = new CryptTool();
        // Java: DigestSHA1 sha = new DigestSHA1();
        let mut sha = DigestSHA1::new();
        // Java: byte[] bytes;
        let bytes: Vec<u8>;
        // Java: try
        {
            // Java: bytes = password.getBytes("UTF-16LE");
            bytes = password.encode_utf16().flat_map(|c| c.to_le_bytes()).collect();
        }
        // Java: catch (UnsupportedEncodingException var14)
        {
            // Java: throw new EncryptedDocumentException("UTF16 not supported");
        }
        // Java: sha.update(this.salt, 0, this.salt.length);
        sha.update_data(&self.salt, 0, self.salt.len() as i32);
        // Java: sha.update(bytes, 0, bytes.length);
        sha.update_data(&bytes, 0, bytes.len() as i32);
        // Java: byte[] passwordhash = new byte[20];
        let mut passwordhash = vec![0u8; 20];
        // Java: sha.get(passwordhash);
        sha.get_digest(&mut passwordhash);
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
            // Java: sha.update(iterator, 0, 4);
            sha.update_data(&iterator, 0, 4);
            // Java: sha.update(passwordhash, 0, passwordhash.length);
            sha.update_data(&passwordhash, 0, passwordhash.len() as i32);
            // Java: sha.get(passwordhash);
            sha.get_digest(&mut passwordhash);
        }
        // Java: cryptTool.setPasswordHash(passwordhash);
        // Java: this.passwordHash = passwordhash;
        self.passwordHash = Some(passwordhash.clone());
        // Java: try
        {
            // Java: Cipher cipher = cryptTool.getCipher(16, false);
            // Java: byte[] verifier = cipher.doFinal(this.verifier);
            // Java: MessageDigest sha1 = MessageDigest.getInstance("SHA-1");
            // Java: byte[] calcVerifierHash = sha1.digest(verifier);
            // Java: byte[] verifierHashTemp = new byte[32];
            let mut verifierHashTemp = vec![0u8; 32];
            // Java: System.arraycopy(calcVerifierHash, 0, verifierHashTemp, 0, calcVerifierHash.length);
            // Java: cipher = cryptTool.getCipher(16, true);
            // Java: this.verifierHash = cipher.doFinal(verifierHashTemp);
            self.verifierHash = verifierHashTemp;
        }
        // Java: catch (Exception e)
        {
            // Java: e.printStackTrace();
        }
    }

    // Java: byte[] getSalt()
    pub fn getSalt(&self) -> &[u8] {
        // Java: return this.salt;
        &self.salt
    }

    // Java: byte[] getVerifier()
    pub fn getVerifier(&self) -> &[u8] {
        // Java: return this.verifier;
        &self.verifier
    }

    // Java: byte[] getVerifierHash()
    pub fn getVerifierHash(&self) -> &[u8] {
        // Java: return this.verifierHash;
        &self.verifierHash
    }

    // Java: int getSpinCount()
    pub fn getSpinCount(&self) -> i32 {
        // Java: return this.spinCount;
        self.spinCount
    }

    // Java: int getCipherMode()
    pub fn getCipherMode(&self) -> i32 {
        // Java: return this.cipherMode;
        self.cipherMode
    }

    // Java: int getAlgorithm()
    pub fn getAlgorithm(&self) -> i32 {
        // Java: return this.algorithm;
        self.algorithm
    }

    // Java: byte[] getEncryptedKey()
    pub fn getEncryptedKey(&self) -> Option<&[u8]> {
        // Java: return this.encryptedKey;
        self.encryptedKey.as_deref()
    }

    // Java: Integer getIOffset()
    pub fn getIOffset(&self) -> Option<i32> {
        // Java: return this.iOffset;
        self.iOffset
    }

    // Java: byte[] getPasswordHash()
    pub fn getPasswordHash(&self) -> Option<&[u8]> {
        // Java: return this.passwordHash;
        self.passwordHash.as_deref()
    }
}
