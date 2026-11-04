// Java: import cn.wps.base.assertion.IOAssert
// Java: import java.io.ByteArrayInputStream
// Java: import java.io.IOException
// Java: import java.io.UnsupportedEncodingException
// Java: import java.nio.charset.StandardCharsets
// Java: import javax.xml.parsers.DocumentBuilderFactory
// Java: import org.apache.poi.EncryptedDocumentException
use crate::org::apache::poi::EncryptedDocumentException::EncryptedDocumentException;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian
// Java: import org.w3c.dom.NamedNodeMap

use crate::cn::wps::encrypt::Base64::Base64;

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
    data[offset + 2] = (value >> 16) as u8;
    data[offset + 3] = (value >> 24) as u8;
}
fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
}

// Java field: int ALGORITHM_RC4
pub(crate) const ALGORITHM_RC4: i32 = 26625;
// Java field: int ALGORITHM_AES_128
pub(crate) const ALGORITHM_AES_128: i32 = 26126;
// Java field: int ALGORITHM_AES_192
pub(crate) const ALGORITHM_AES_192: i32 = 26127;
// Java field: int ALGORITHM_AES_256
pub(crate) const ALGORITHM_AES_256: i32 = 26128;
// Java field: int HASH_SHA1
pub(crate) const HASH_SHA1: i32 = 32772;
// Java field: int HASH_SHA512
pub(crate) const HASH_SHA512: i32 = 32782;
// Java field: int PROVIDER_RC4
pub(crate) const PROVIDER_RC4: i32 = 1;
// Java field: int PROVIDER_AES
pub(crate) const PROVIDER_AES: i32 = 24;
// Java field: int MODE_ECB
pub(crate) const MODE_ECB: i32 = 1;
// Java field: int MODE_CBC
pub(crate) const MODE_CBC: i32 = 2;
// Java field: int MODE_CFB
pub(crate) const MODE_CFB: i32 = 3;

// Java: public class EncryptionHeader
pub struct EncryptionHeader {
    // Java field: int flags
    pub(crate) flags: i32,
    // Java field: int sizeExtra
    pub(crate) sizeExtra: i32,
    // Java field: int algorithm
    pub(crate) algorithm: i32,
    // Java field: int hashAlgorithm
    pub(crate) hashAlgorithm: i32,
    // Java field: int keySize
    pub(crate) keySize: i32,
    // Java field: int providerType
    pub(crate) providerType: i32,
    // Java field: int cipherMode
    pub(crate) cipherMode: i32,
    // Java field: byte[] keySalt
    pub(crate) keySalt: Option<Vec<u8>>,
    // Java field: String cspName
    pub(crate) cspName: Option<String>,
    // Java field: Integer iOffset
    pub(crate) iOffset: Option<i32>,
}

// Java: public class EncryptionHeader
impl EncryptionHeader {
    // Java: EncryptionHeader(byte[] allInfo, Integer iOffset)
    pub fn new_from_bytes(allInfo: &mut [u8], iOffset: i32) -> Self {
        // Java: super();
        // Java: int offset = iOffset;
        let mut offset = iOffset as usize;
        // Java: this.flags = 36;
        let flags = 36;
        // Java: LittleEndian.putInt(allInfo, offset, this.flags);
        le_put_int(allInfo, offset, flags);
        // Java: offset += 4;
        offset += 4;
        // Java: this.sizeExtra = 0;
        let sizeExtra = 0;
        // Java: LittleEndian.putInt(allInfo, offset, this.sizeExtra);
        le_put_int(allInfo, offset, sizeExtra);
        // Java: offset += 4;
        offset += 4;
        // Java: this.algorithm = 26126;
        let algorithm = 26126;
        // Java: LittleEndian.putInt(allInfo, offset, this.algorithm);
        le_put_int(allInfo, offset, algorithm);
        // Java: offset += 4;
        offset += 4;
        // Java: this.hashAlgorithm = 32772;
        let hashAlgorithm = 32772;
        // Java: LittleEndian.putInt(allInfo, offset, this.hashAlgorithm);
        le_put_int(allInfo, offset, hashAlgorithm);
        // Java: offset += 4;
        offset += 4;
        // Java: this.keySize = 128;
        let keySize = 128;
        // Java: LittleEndian.putInt(allInfo, offset, this.keySize);
        le_put_int(allInfo, offset, keySize);
        // Java: offset += 4;
        offset += 4;
        // Java: this.providerType = 24;
        let providerType = 24;
        // Java: LittleEndian.putInt(allInfo, offset, this.providerType);
        le_put_int(allInfo, offset, providerType);
        // Java: offset += 4;
        offset += 4;
        // Java: LittleEndian.putInt(allInfo, offset, 0);
        le_put_int(allInfo, offset, 0);
        // Java: offset += 4;
        offset += 4;
        // Java: LittleEndian.putInt(allInfo, offset, 0);
        le_put_int(allInfo, offset, 0);
        // Java: offset += 4;
        offset += 4;
        // Java: this.cspName = "Microsoft Enhanced RSA and AES Cryptographic Provider";
        let cspName = "Microsoft Enhanced RSA and AES Cryptographic Provider".to_string();
        // Java: byte[] bytes = null;
        let bytes: Vec<u8>;
        // Java: try
        {
            // Java: bytes = this.cspName.getBytes("UTF-16LE");
            bytes = cspName.encode_utf16().flat_map(|c| c.to_le_bytes()).collect();
        }
        // Java: catch (UnsupportedEncodingException var6)
        {
            // Java: throw new EncryptedDocumentException("UTF16 not supported");
        }
        // Java: System.arraycopy(bytes, 0, allInfo, offset, bytes.length);
        allInfo[offset..offset + bytes.len()].copy_from_slice(&bytes);
        // Java: offset += bytes.length;
        offset += bytes.len();
        // Java: LittleEndian.putShort(allInfo, offset, (short)0);
        le_put_short(allInfo, offset, 0);
        // Java: offset += 2;
        offset += 2;
        // Java: this.iOffset = new Integer(offset);
        let iOffset_val = offset as i32;
        // Java: this.cipherMode = 1;
        let cipherMode = 1;
        // Java: this.keySalt = null;
        EncryptionHeader {
            flags,
            sizeExtra,
            algorithm,
            hashAlgorithm,
            keySize,
            providerType,
            cipherMode,
            keySalt: None,
            cspName: Some(cspName),
            iOffset: Some(iOffset_val),
        }
    }

    // Java: EncryptionHeader(DocumentInputStream is)
    pub fn new_from_stream(is: &mut DocumentInputStream) -> Self {
        // Java: super();
        // Java: this.flags = is.readInt();
        let flags = is.readInt();
        // Java: this.sizeExtra = is.readInt();
        let sizeExtra = is.readInt();
        // Java: this.algorithm = is.readInt();
        let algorithm = is.readInt();
        // Java: this.hashAlgorithm = is.readInt();
        let hashAlgorithm = is.readInt();
        // Java: this.keySize = is.readInt();
        let keySize = is.readInt();
        // Java: this.providerType = is.readInt();
        let providerType = is.readInt();
        // Java: is.readLong();
        let _ = is.readLong();
        // Java: StringBuilder builder = new StringBuilder();
        let mut builder = String::new();
        // Java: while(true)
        let cspName;
        loop {
            // Java: char c = (char)is.readShort();
            let c = is.readShort() as u16;
            // Java: if (c == 0)
            if c == 0 {
                // Java: this.cspName = builder.toString();
                cspName = builder;
                // Java: this.cipherMode = 1;
                // Java: this.keySalt = null;
                // Java: return;
                break;
            }
            // Java: builder.append(c);
            if let Some(ch) = char::from_u32(c as u32) {
                builder.push(ch);
            }
        }
        EncryptionHeader {
            flags,
            sizeExtra,
            algorithm,
            hashAlgorithm,
            keySize,
            providerType,
            cipherMode: 1,
            keySalt: None,
            cspName: Some(cspName),
            iOffset: None,
        }
    }

    // Java: EncryptionHeader(String descriptor)
    pub fn new_from_descriptor(descriptor: &str) -> Self {
        // Java: super();
        // Java: NamedNodeMap keyData;
        // Java: try
        // Java: ByteArrayInputStream is = new ByteArrayInputStream(descriptor.getBytes(StandardCharsets.UTF_8));
        // Java: keyData = DocumentBuilderFactory.newInstance().newDocumentBuilder().parse(is).getElementsByTagName("keyData").item(0).getAttributes();
        let doc = crate::xml_util::parse_xml(descriptor);
        let keyData = crate::xml_util::get_tag_attributes(&doc, "keyData");
        // Java: catch (Exception var10)
        // Java: throw new EncryptedDocumentException("Unable to parse keyData");

        // Java: this.keySize = Integer.parseInt(keyData.getNamedItem("keyBits").getNodeValue());
        let keySize: i32 = keyData.get("keyBits").expect("keyBits").parse().unwrap();
        // Java: this.flags = 0;
        let flags = 0;
        // Java: this.sizeExtra = 0;
        let sizeExtra = 0;
        // Java: this.cspName = null;
        let cspName: Option<String> = None;
        // Java: int blockSize = Integer.parseInt(keyData.getNamedItem("blockSize").getNodeValue());
        let blockSize: i32 = keyData.get("blockSize").expect("blockSize").parse().unwrap();
        // Java: String cipher = keyData.getNamedItem("cipherAlgorithm").getNodeValue();
        let cipher = keyData.get("cipherAlgorithm").expect("cipherAlgorithm").clone();
        // Java: if ("AES".equals(cipher))
        let (algorithm, providerType, cipherMode, hashAlgorithm, keySalt);
        if cipher == "AES" {
            // Java: this.providerType = 24;
            providerType = 24;
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
                    // Java: throw new EncryptedDocumentException("Unsupported key length");
                    panic!("EncryptedDocumentException: Unsupported key length");
                }
                // Java: this.algorithm = 26128;
                algorithm = 26128;
            }
            // Java: String chaining = keyData.getNamedItem("cipherChaining").getNodeValue();
            let chaining = keyData.get("cipherChaining").expect("cipherChaining").clone();
            // Java: if ("ChainingModeCBC".equals(chaining))
            if chaining == "ChainingModeCBC" {
                // Java: this.cipherMode = 2;
                cipherMode = 2;
            }
            // Java: else
            else {
                // Java: if (!"ChainingModeCFB".equals(chaining))
                if chaining != "ChainingModeCFB" {
                    // Java: throw new EncryptedDocumentException("Unsupported chaining mode");
                    panic!("EncryptedDocumentException: Unsupported chaining mode");
                }
                // Java: this.cipherMode = 3;
                cipherMode = 3;
            }
            // Java: String hashAlg = keyData.getNamedItem("hashAlgorithm").getNodeValue();
            let hashAlg = keyData.get("hashAlgorithm").expect("hashAlgorithm").clone();
            // Java: int hashSize = Integer.parseInt(keyData.getNamedItem("hashSize").getNodeValue());
            let hashSize: i32 = keyData.get("hashSize").expect("hashSize").parse().unwrap();
            // Java: if ("SHA1".equals(hashAlg) && hashSize == 20)
            if hashAlg == "SHA1" && hashSize == 20 {
                // Java: this.hashAlgorithm = 32772;
                hashAlgorithm = 32772;
            }
            // Java: else
            else {
                // Java: if (!"SHA512".equals(hashAlg) || hashSize != 64)
                if hashAlg != "SHA512" || hashSize != 64 {
                    // Java: throw new EncryptedDocumentException("Unsupported hash algorithm");
                    panic!("EncryptedDocumentException: Unsupported hash algorithm");
                }
                // Java: this.hashAlgorithm = 32782;
                hashAlgorithm = 32782;
            }
            // Java: String salt = keyData.getNamedItem("saltValue").getNodeValue();
            let salt = keyData.get("saltValue").expect("saltValue").clone();
            // Java: int saltLength = Integer.parseInt(keyData.getNamedItem("saltSize").getNodeValue());
            let saltLength: i32 = keyData.get("saltSize").expect("saltSize").parse().unwrap();
            // Java: this.keySalt = org.apache.poi.util.base64.Base64.decode(salt.getBytes(StandardCharsets.UTF_8));
            keySalt = Some(Base64::base64ToByteArray_simple(&salt));
            // Java: if (this.keySalt.length != saltLength)
            if keySalt.as_ref().unwrap().len() as i32 != saltLength {
                // Java: throw new EncryptedDocumentException("Invalid salt length");
                panic!("EncryptedDocumentException: Invalid salt length");
            }
        }
        // Java: else
        else {
            // Java: throw new EncryptedDocumentException("Unsupported cipher");
            panic!("EncryptedDocumentException: Unsupported cipher");
        }
        EncryptionHeader {
            flags,
            sizeExtra,
            algorithm,
            hashAlgorithm,
            keySize,
            providerType,
            cipherMode,
            keySalt,
            cspName,
            iOffset: None,
        }
    }

    // Java: EncryptionHeader(EncryptionInfoInit infoInit)
    pub fn new_from_info_init(infoInit: &mut dyn EncryptionInfoInitTrait) -> Self {
        // Java: super();
        // Java: this.flags = 36;
        let flags = 36;
        // Java: this.sizeExtra = 0;
        let sizeExtra = 0;
        // Java: this.algorithm = 26126;
        let algorithm = 26126;
        // Java: infoInit.putNamedItem("cipherAlgorithm", this.algorithm);
        infoInit.putNamedItem_int("cipherAlgorithm", algorithm);
        // Java: this.hashAlgorithm = 32772;
        let hashAlgorithm = 32772;
        // Java: infoInit.putNamedItem("hashAlgorithm", this.hashAlgorithm);
        infoInit.putNamedItem_int("hashAlgorithm", hashAlgorithm);
        // Java: this.keySize = 128;
        let keySize = 128;
        // Java: infoInit.putNamedItem("keyBits", this.keySize);
        infoInit.putNamedItem_int("keyBits", keySize);
        // Java: this.providerType = 24;
        let providerType = 24;
        // Java: this.cspName = "Microsoft Enhanced RSA and AES Cryptographic Provider";
        let cspName = Some("Microsoft Enhanced RSA and AES Cryptographic Provider".to_string());
        // Java: this.cipherMode = 1;
        let cipherMode = 1;
        // Java: infoInit.putNamedItem("cipherChaining", this.cipherMode);
        infoInit.putNamedItem_int("cipherChaining", cipherMode);
        // Java: this.keySalt = null;
        EncryptionHeader {
            flags,
            sizeExtra,
            algorithm,
            hashAlgorithm,
            keySize,
            providerType,
            cipherMode,
            keySalt: None,
            cspName,
            iOffset: None,
        }
    }

    // Java: int getCipherMode()
    pub fn getCipherMode(&self) -> i32 {
        // Java: return this.cipherMode;
        self.cipherMode
    }

    // Java: int getFlags()
    pub fn getFlags(&self) -> i32 {
        // Java: return this.flags;
        self.flags
    }

    // Java: int getSizeExtra()
    pub fn getSizeExtra(&self) -> i32 {
        // Java: return this.sizeExtra;
        self.sizeExtra
    }

    // Java: int getAlgorithm()
    pub fn getAlgorithm(&self) -> i32 {
        // Java: return this.algorithm;
        self.algorithm
    }

    // Java: int getHashAlgorithm()
    pub fn getHashAlgorithm(&self) -> i32 {
        // Java: return this.hashAlgorithm;
        self.hashAlgorithm
    }

    // Java: String getHashAlgorithmName()
    pub fn getHashAlgorithmName(&self) -> Option<String> {
        // Java: if (this.hashAlgorithm == 32772)
        if self.hashAlgorithm == 32772 {
            // Java: return "SHA-1";
            return Some("SHA-1".to_string());
        }
        // Java: else if (this.hashAlgorithm == 32782)
        else if self.hashAlgorithm == 32782 {
            // Java: return "SHA-512";
            return Some("SHA-512".to_string());
        }
        // Java: else
        else {
            // Java: IOAssert.fail("NoSupportedHashAlgorithm!");
            // Java: return null;
            return None;
        }
    }

    // Java: int getKeySize()
    pub fn getKeySize(&self) -> i32 {
        // Java: return this.keySize;
        self.keySize
    }

    // Java: byte[] getKeySalt()
    pub fn getKeySalt(&self) -> Option<&[u8]> {
        // Java: return this.keySalt;
        self.keySalt.as_deref()
    }

    // Java: int getProviderType()
    pub fn getProviderType(&self) -> i32 {
        // Java: return this.providerType;
        self.providerType
    }

    // Java: String getCspName()
    pub fn getCspName(&self) -> Option<&str> {
        // Java: return this.cspName;
        self.cspName.as_deref()
    }

    // Java: Integer getIOffset()
    pub fn getIOffset(&self) -> Option<i32> {
        // Java: return this.iOffset;
        self.iOffset
    }
}

pub trait EncryptionInfoInitTrait {
    fn putNamedItem_int(&mut self, key: &str, value: i32);
    fn putNamedItem_bytes(&mut self, key: &str, value: &[u8]);
}
