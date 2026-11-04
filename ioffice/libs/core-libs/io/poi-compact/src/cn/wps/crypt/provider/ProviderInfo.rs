// Java: import cn.wps.crypt.sal.digest.DigestSHA1
use crate::cn::wps::crypt::sal::digest::DigestSHA1::DigestSHA1;
// Java: import java.io.UnsupportedEncodingException
// Java: import java.security.SecureRandom
// Java: import java.util.Arrays
// Java: import javax.crypto.SecretKey
// Java: import javax.crypto.spec.SecretKeySpec
// Java: import org.apache.poi.EncryptedDocumentException
// Java: import org.apache.poi.poifs.crypt.RC4Cipher
use crate::org::apache::poi::poifs::crypt::RC4Cipher::RC4Cipher;
// Java: import org.apache.poi.util.LittleEndian

use super::EnumalhsEx::EnumalhsEx;
use super::MSCodec_Std97::MSCodec_Std97;
use super::MSCodec_Xor95::MSCodec_Xor95;

fn le_get_byte_array(data: &[u8], offset: usize, len: usize) -> Vec<u8> {
    let end = std::cmp::min(offset + len, data.len());
    data[offset..end].to_vec()
}
fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
    data[offset + 2] = (value >> 16) as u8;
    data[offset + 3] = (value >> 24) as u8;
}
fn le_get_int(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32 & 0xFF)
        | ((data[offset + 1] as i32 & 0xFF) << 8)
        | ((data[offset + 2] as i32 & 0xFF) << 16)
        | ((data[offset + 3] as i32 & 0xFF) << 24)
}

// Java field: int ENCRYPT_XOR
pub const ENCRYPT_XOR: i32 = 0;
// Java field: int ENCRYPT_STD
pub const ENCRYPT_STD: i32 = 1;
// Java field: int ENCRYPT_OTHER
pub const ENCRYPT_OTHER: i32 = 2;
// Java field: int WPS_ANDROID_VERSION
pub const WPS_ANDROID_VERSION: i32 = 0;
// Java field: int NO_ENCRYPTDOCATTR
pub const NO_ENCRYPTDOCATTR: i32 = 12;
// Java field: int CRYPTDOCUMENT_WPS
pub const CRYPTDOCUMENT_WPS: i32 = 0;
// Java field: int CRYPTDOCUMENT_ET
pub const CRYPTDOCUMENT_ET: i32 = 1;
// Java field: int CRYPTDOCUMENT_WPP
pub const CRYPTDOCUMENT_WPP: i32 = 2;
// Java field: int CRYPTDOCUMENT_MSO2007
pub const CRYPTDOCUMENT_MSO2007: i32 = 3;
// Java field: String ENCRYPT_XOR_NAME
pub const ENCRYPT_XOR_NAME: &str = "OfficeXor";
// Java field: String ENCRYPT_STD_NAME
pub const ENCRYPT_STD_NAME: &str = "OfficeStandard";
// Java field: String EXCEL_DEFAULT_PASSWORD
pub const EXCEL_DEFAULT_PASSWORD: &str = "VelvetSweatshop";
// Java field: String PPT_DEFAULT_PASSWORD
pub const PPT_DEFAULT_PASSWORD: &str = "/01Hannes Ruescher/01";
// Java field: String SUMMARYINFORMATION_NAME
pub const SUMMARYINFORMATION_NAME: &str = "\x00x05SummaryInformation";
// Java field: String DOC_SUMMARYINFORMATION_NAME
pub const DOC_SUMMARYINFORMATION_NAME: &str = "\x00x05DocumentSummaryInformation";
// Java field: int DEFAULT_INITLEN
pub const DEFAULT_INITLEN: i32 = 16;

// Java field: SecureRandom random

// Java: public class ProviderInfo extends EnumalhsEx
pub struct ProviderInfo {
    pub(crate) base: EnumalhsEx,
    // Java field: int nInfoVersion
    pub nInfoVersion: i32,
    // Java field: int nAddRef
    pub nAddRef: i32,
    // Java field: int nEncryptType
    pub nEncryptType: i32,
    // Java field: int nEncryptDocumentType
    pub nEncryptDocumentType: i32,
    // Java field: String strPassword
    pub strPassword: String,
    // Java field: String strProvName
    pub strProvName: Option<String>,
    // Java field: String strAlgorithmName
    pub strAlgorithmName: String,
    // Java field: MSCodec_Xor95 m_pXOR
    pub m_pXOR: Option<MSCodec_Xor95>,
    // Java field: MSCodec_Std97 m_pStd
    pub m_pStd: Option<MSCodec_Std97>,
    // Java field: RC4Cipher cipher
    pub cipher: Option<RC4Cipher>,
    // Java field: byte[] m_pnDocId
    pub m_pnDocId: Vec<u8>,
    // Java field: int nProvType
    pub nProvType: i32,
    // Java field: int nEncryptDocAttr
    pub nEncryptDocAttr: i32,
    // Java field: int nKeyLen
    pub nKeyLen: i32,
    // Java field: int nHashALG
    pub nHashALG: i32,
    // Java field: int nDeriveKeyALG
    pub nDeriveKeyALG: i32,
    // Java field: byte[] pInitData
    pub pInitData: Vec<u8>,
    // Java field: int nInitDataLen
    pub nInitDataLen: i32,
    // Java field: int hCryptProv
    pub(crate) hCryptProv: i32,
    // Java field: int nGlobleHashDataLen
    pub nGlobleHashDataLen: i32,
    // Java field: int hKey
    pub(crate) hKey: i32,
}

// Java: public class ProviderInfo extends EnumalhsEx
impl ProviderInfo {
    // Java: ProviderInfo()
    pub fn new() -> Self {
        // Java: super();
        ProviderInfo {
            base: EnumalhsEx::new(),
            nInfoVersion: 0,
            nAddRef: 0,
            nEncryptType: 0,
            nEncryptDocumentType: 0,
            strPassword: String::new(),
            strProvName: None,
            strAlgorithmName: String::new(),
            m_pXOR: None,
            m_pStd: None,
            cipher: None,
            m_pnDocId: vec![0u8; 16],
            nProvType: 0,
            nEncryptDocAttr: 0,
            nKeyLen: 0,
            nHashALG: 0,
            nDeriveKeyALG: 0,
            pInitData: Vec::new(),
            nInitDataLen: 0,
            hCryptProv: 0,
            nGlobleHashDataLen: 0,
            hKey: 0,
        }
    }

    // Java: void init_STD(int docType, String strPassword, byte[] data)
    pub fn init_STD(&mut self, docType: i32, strPassword: &str, data: Option<&[u8]>) {
        // Java: this.nInfoVersion = WPS_ANDROID_VERSION;
        self.nInfoVersion = WPS_ANDROID_VERSION;
        // Java: this.nAddRef = 1;
        self.nAddRef = 1;
        // Java: this.nEncryptType = ENCRYPT_STD;
        self.nEncryptType = ENCRYPT_STD;
        // Java: this.nEncryptDocAttr = NO_ENCRYPTDOCATTR;
        self.nEncryptDocAttr = NO_ENCRYPTDOCATTR;
        // Java: this.strProvName = null;
        self.strProvName = None;
        // Java: this.strPassword = strPassword;
        self.strPassword = strPassword.to_string();
        // Java: this.nEncryptDocumentType = docType;
        self.nEncryptDocumentType = docType;
        // Java: this.strAlgorithmName = ENCRYPT_STD_NAME;
        self.strAlgorithmName = ENCRYPT_STD_NAME.to_string();
        // Java: if (data != null)
        if let Some(d) = data {
            // Java: System.arraycopy(data, 0, this.m_pnDocId, 0, data.length);
            let len = std::cmp::min(d.len(), self.m_pnDocId.len());
            self.m_pnDocId[..len].copy_from_slice(&d[..len]);
        }
        // Java: else
        else {
            // Java: for(int i = 0; i < this.m_pnDocId.length; i += 4)
            for i in (0..self.m_pnDocId.len()).step_by(4) {
                // Java: LittleEndian.putInt(this.m_pnDocId, i, random.nextInt());
                le_put_int(&mut self.m_pnDocId, i, 0);
            }
        }
        // Java: this.m_pStd = new MSCodec_Std97();
        let mut m_pStd = MSCodec_Std97::new();
        // Java: short[] password = new short[16];
        let mut password = [0i16; 16];
        // Java: for(int i = 0; i < 16 && i < strPassword.length(); ++i)
        for (i, ch) in strPassword.chars().take(16).enumerate() {
            // Java: password[i] = (short)strPassword.charAt(i);
            password[i] = ch as i16;
        }
        // Java: this.m_pStd.InitKey(password, this.m_pnDocId);
        let doc_id = self.m_pnDocId.clone();
        m_pStd.InitKey(&password, &doc_id);
        self.m_pStd = Some(m_pStd);
    }

    // Java: void init_XOR(int docType, String strPassword)
    pub fn init_XOR(&mut self, docType: i32, strPassword: &str) {
        // Java: this.nInfoVersion = WPS_ANDROID_VERSION;
        self.nInfoVersion = WPS_ANDROID_VERSION;
        // Java: this.nAddRef = 1;
        self.nAddRef = 1;
        // Java: this.nEncryptType = ENCRYPT_XOR;
        self.nEncryptType = ENCRYPT_XOR;
        // Java: this.nEncryptDocAttr = NO_ENCRYPTDOCATTR;
        self.nEncryptDocAttr = NO_ENCRYPTDOCATTR;
        // Java: this.strProvName = null;
        self.strProvName = None;
        // Java: this.strAlgorithmName = ENCRYPT_XOR_NAME;
        self.strAlgorithmName = ENCRYPT_XOR_NAME.to_string();
        // Java: this.strPassword = strPassword;
        self.strPassword = strPassword.to_string();
        // Java: this.nEncryptDocumentType = docType;
        self.nEncryptDocumentType = docType;
        // Java: if (docType == CRYPTDOCUMENT_WPS || docType == CRYPTDOCUMENT_ET)
        if docType == CRYPTDOCUMENT_WPS || docType == CRYPTDOCUMENT_ET {
            // Java: if (docType == CRYPTDOCUMENT_WPS)
            if docType == CRYPTDOCUMENT_WPS {
                // Java: this.m_pXOR = new MSCodec_XorWord95();
                self.m_pXOR = Some(MSCodec_Xor95::new(7));
            }
            // Java: else if (docType == CRYPTDOCUMENT_ET)
            else if docType == CRYPTDOCUMENT_ET {
                // Java: this.m_pXOR = new MSCodec_XorXLS95();
                self.m_pXOR = Some(MSCodec_Xor95::new(2));
            }
            // Java: byte[] passwd = new byte[16];
            let mut passwd = [0u8; 16];
            // Java: int nLen = 0;
            let mut nLen: usize = 0;
            // Java: try
            // Java: byte[] p = strPassword.getBytes("US-ASCII");
            let p: Vec<u8> = strPassword.bytes().collect();
            // Java: if (p != null)
            // Java: nLen = p.length;
            nLen = p.len();
            // Java: System.arraycopy(p, 0, passwd, 0, nLen);
            let copy_len = std::cmp::min(nLen, 16);
            passwd[..copy_len].copy_from_slice(&p[..copy_len]);
            // Java: catch (UnsupportedEncodingException var7)
            // Java: if (passwd != null)
            // Java: short eKey = MSCodec_Xor95.GetKey(passwd, nLen);
            let eKey = MSCodec_Xor95::GetKey(&passwd, nLen as i32);
            // Java: short hashValue = MSCodec_Xor95.getHash(passwd, nLen);
            let hashValue = MSCodec_Xor95::getHash_static(&passwd, nLen);
            // Java: this.m_pXOR.initKey(passwd, eKey, hashValue);
            if let Some(ref mut xor) = self.m_pXOR {
                xor.initKey(&passwd, eKey, hashValue);
            }
        }
    }

    // Java: void init_Other(String passwd)
    pub fn init_Other_simple(&mut self, passwd: &str) {
        // Java: byte[] data = new byte[]{-73, 120, -64, 46, 99, 20, 52, 14, -121, 125, -68, 54, -58, -10, 122, -11};
        let data: Vec<u8> = vec![0xB7, 0x78, 0xC0, 0x2E, 0x63, 0x14, 0x34, 0x0E, 0x87, 0x7D, 0xBC, 0x36, 0xC6, 0xF6, 0x7A, 0xF5];
        // Java: this.init_Other(CRYPTDOCUMENT_WPS, "Microsoft Enhanced Cryptographic Provider v1.0", 1, passwd, 32772, 26625, 128, data, 12);
        self.init_Other(CRYPTDOCUMENT_WPS, "Microsoft Enhanced Cryptographic Provider v1.0", 1, passwd, 32772, 26625, 128, Some(&data), 12);
    }

    // Java: void init_Other(int docType, String strName, int nType, String strPassword, int nHashALG, int nDeriveKeyALG, int nKeyLen, byte[] data, int docAttr)
    pub fn init_Other(&mut self, docType: i32, strName: &str, nType: i32, strPassword: &str, nHashALG: i32, nDeriveKeyALG: i32, nKeyLen: i32, data: Option<&[u8]>, docAttr: i32) {
        // Java: this.nInfoVersion = WPS_ANDROID_VERSION;
        self.nInfoVersion = WPS_ANDROID_VERSION;
        // Java: this.nAddRef = 1;
        self.nAddRef = 1;
        // Java: this.nEncryptType = ENCRYPT_OTHER;
        self.nEncryptType = ENCRYPT_OTHER;
        // Java: this.nEncryptDocumentType = docType;
        self.nEncryptDocumentType = docType;
        // Java: this.strProvName = strName;
        self.strProvName = Some(strName.to_string());
        // Java: this.nProvType = nType;
        self.nProvType = nType;
        // Java: this.nKeyLen = nKeyLen;
        self.nKeyLen = nKeyLen;
        // Java: this.nEncryptDocAttr = docAttr;
        self.nEncryptDocAttr = docAttr;
        // Java: this.strPassword = strPassword;
        self.strPassword = strPassword.to_string();
        // Java: this.nHashALG = nHashALG;
        self.nHashALG = nHashALG;
        // Java: this.nDeriveKeyALG = nDeriveKeyALG;
        self.nDeriveKeyALG = nDeriveKeyALG;
        // Java: if (data != null && data.length != 0)
        if let Some(d) = data {
            if !d.is_empty() {
                // Java: this.nInitDataLen = data.length;
                self.nInitDataLen = d.len() as i32;
                // Java: this.pInitData = LittleEndian.getByteArray(data, 0, data.length);
                self.pInitData = le_get_byte_array(d, 0, d.len());
            } else {
                // Java: else
                // Java: this.nInitDataLen = DEFAULT_INITLEN;
                self.nInitDataLen = DEFAULT_INITLEN;
                // Java: this.pInitData = new byte[this.nInitDataLen];
                self.pInitData = vec![0u8; self.nInitDataLen as usize];
            }
        } else {
            self.nInitDataLen = DEFAULT_INITLEN;
            self.pInitData = vec![0u8; self.nInitDataLen as usize];
        }
        // Java: this.hCryptProv = 0;
        self.hCryptProv = 0;
        // Java: this.nGlobleHashDataLen = 0;
        self.nGlobleHashDataLen = 0;
        // Java: this.hKey = 0;
        self.hKey = 0;
        // Java: this.initCipher(this.pInitData, this.nInitDataLen, strPassword, nKeyLen / 8, new byte[4]);
        let init_data = self.pInitData.clone();
        let init_data_len = self.nInitDataLen;
        let password = self.strPassword.clone();
        let block = [0u8; 4];
        self.initCipher(&init_data, init_data_len, &password, nKeyLen / 8, &block);
    }

    // Java: boolean initCipher(byte[] initData, int initDataLen, String passwd, int nKey, byte[] block)
    pub fn initCipher(&mut self, initData: &[u8], initDataLen: i32, passwd: &str, nKey: i32, block: &[u8]) -> bool {
        // Java: boolean isOK = false;
        let mut isOK = false;
        // Java: if (this.strProvName != null && this.nDeriveKeyALG == 26625 && this.nHashALG == 32772)
        if self.strProvName.is_some() && self.nDeriveKeyALG == 26625 && self.nHashALG == 32772 {
            // Java: try
            // Java: DigestSHA1 sha = new DigestSHA1();
            let mut sha = DigestSHA1::new();
            // Java: sha.update(initData, 0, initDataLen);
            sha.update_data(initData, 0, initDataLen);
            // Java: byte[] bytes;
            // Java: try
            // Java: bytes = passwd.getBytes("UTF-16LE");
            let bytes: Vec<u8> = passwd.encode_utf16().flat_map(|c| c.to_le_bytes()).collect();
            // Java: catch (UnsupportedEncodingException var12)
            // Java: throw new EncryptedDocumentException("UTF16 not supported");
            // Java: sha.update(bytes, 0, bytes.length);
            sha.update_data(&bytes, 0, bytes.len() as i32);
            // Java: byte[] hash = new byte[20];
            let mut hash = vec![0u8; 20];
            // Java: sha.get(hash);
            sha.get_digest(&mut hash);
            // Java: sha.update(hash, 0, hash.length);
            sha.update_data(&hash, 0, hash.len() as i32);
            // Java: sha.update(block, 0, block.length);
            sha.update_data(block, 0, block.len() as i32);
            // Java: sha.get(hash);
            sha.get_digest(&mut hash);
            // Java: byte[] hashData = new byte[nKey == 5 ? 16 : nKey];
            let hash_data_len = if nKey == 5 { 16 } else { nKey as usize };
            let mut hashData = vec![0u8; hash_data_len];
            // Java: Arrays.fill(hashData, (byte)0);
            // Java: System.arraycopy(hash, 0, hashData, 0, nKey);
            let copy_len = std::cmp::min(nKey as usize, hash.len());
            hashData[..copy_len].copy_from_slice(&hash[..copy_len]);
            // Java: SecretKey skey = new SecretKeySpec(hashData, "RC4");
            // Java: if (this.cipher == null)
            if self.cipher.is_none() {
                // Java: this.cipher = new RC4Cipher();
                self.cipher = Some(RC4Cipher::new());
            }
            // Java: this.cipher.init(2, (SecretKey)skey);
            if let Some(ref mut cipher) = self.cipher {
                cipher.init_with_secret_key(2, &hashData);
            }
            // Java: isOK = true;
            isOK = true;
            // Java: catch (Exception e)
            // Java: String error = e.getMessage();
            // Java: if (error != null)
            // Java: System.out.println(error);
        }
        // Java: return isOK;
        isOK
    }
}
