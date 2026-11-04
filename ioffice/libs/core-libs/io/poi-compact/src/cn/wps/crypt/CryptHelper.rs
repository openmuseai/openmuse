// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.crypt.provider.MSCodec_Xor95
use crate::cn::wps::crypt::provider::MSCodec_Xor95::MSCodec_Xor95;
// Java: import cn.wps.crypt.provider.ProviderInfo
use crate::cn::wps::crypt::provider::ProviderInfo::{self, ProviderInfo as ProviderInfoStruct};
// Java: import cn.wps.crypt.sal.digest.DigestSHA1
use crate::cn::wps::crypt::sal::digest::DigestSHA1::DigestSHA1;
// Java: import cn.wps.ole.IOLEStream
// Java: import cn.wps.ole.OLEEntry
// Java: import java.io.ByteArrayOutputStream
// Java: import java.io.IOException
// Java: import java.io.UnsupportedEncodingException
// Java: import java.util.Arrays
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.IntegerField
// Java: import org.apache.poi.util.LittleEndian

fn le_get_byte_array(data: &[u8], offset: usize, len: usize) -> Vec<u8> {
    let end = std::cmp::min(offset + len, data.len());
    data[offset..end].to_vec()
}
fn le_get_int(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32 & 0xFF)
        | ((data[offset + 1] as i32 & 0xFF) << 8)
        | ((data[offset + 2] as i32 & 0xFF) << 16)
        | ((data[offset + 3] as i32 & 0xFF) << 24)
}
fn le_get_ushort(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32 & 0xFF) | ((data[offset + 1] as i32 & 0xFF) << 8)
}
fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
    data[offset + 2] = (value >> 16) as u8;
    data[offset + 3] = (value >> 24) as u8;
}

// Java field: int ENCRYPT_BLOCK_SIZE
pub const ENCRYPT_BLOCK_SIZE: i32 = 512;
// Java field: int ENCRYPTDOCATTR
pub(crate) const ENCRYPTDOCATTR: i32 = 4;
// Java field: int NO_ENCRYPTDOCATTR
pub(crate) const NO_ENCRYPTDOCATTR: i32 = 12;
// Java field: int MSO2007ENCRYPTDOCATTR
pub(crate) const MSO2007ENCRYPTDOCATTR: i32 = 36;

pub struct IntegerFieldLocal {
    value: i32,
}
impl IntegerFieldLocal {
    pub fn new(_offset: i32, value: i32) -> Self {
        IntegerFieldLocal { value }
    }
    pub fn get(&self) -> i32 { self.value }
    pub fn set(&mut self, v: i32) { self.value = v; }
}

// Java: public class CryptHelper
pub struct CryptHelper;

// Java: public class CryptHelper
impl CryptHelper {
    // Java: CryptHelper()
    pub fn new() -> Self {
        // Java: super();
        CryptHelper
    }

    // Java: byte lclRotateLeft(byte value, byte nBits)
    pub fn lclRotateLeft(value: u8, nBits: u8) -> u8 {
        // Java: IOAssert.assertTrue("nBits should be smaller than 8!", nBits < 8);
        // Java: value = (byte)((short)(value & 255) << nBits | (short)(value & 255) >> 8 - nBits);
        let v = value as u16;
        let result = ((v << nBits) | (v >> (8 - nBits))) as u8;
        // Java: return value;
        result
    }

    // Java: short lclRotateLeft(short value, byte nBits)
    fn lclRotateLeft_short(value: i16, nBits: u8) -> i16 {
        // Java: IOAssert.assertTrue("nBits should be smaller than 2*8!", nBits < 16);
        // Java: value = (short)((value & '\uffff') << nBits | (value & '\uffff') >> 16 - nBits);
        let v = value as u16;
        let result = ((v << nBits) | (v >> (16 - nBits))) as i16;
        // Java: return value;
        result
    }

    // Java: short lclRotateLeft(short value, byte nBits, byte nWidth)
    fn lclRotateLeft_short_width(value: i16, nBits: u8, nWidth: u8) -> i16 {
        // Java: IOAssert.assertTrue("nBits should be smaller than nWidth!", nBits < nWidth && nWidth < 16);
        // Java: short nMask = (short)((1 << (short)(nWidth & 255)) - 1);
        let nMask = ((1u16 << (nWidth as u16)) - 1) as u16;
        // Java: value = (short)(((value & '\uffff') << nBits | (value & '\uffff' & nMask) >> nWidth - nBits) & nMask);
        let v = value as u16;
        let result = (((v << nBits) | ((v & nMask) >> (nWidth - nBits))) & nMask) as i16;
        // Java: return value;
        result
    }

    // Java: byte lclRotateRight(byte value, byte nBits)
    pub fn lclRotateRight(value: u8, nBits: u8) -> u8 {
        // Java: IOAssert.assertTrue("nBits should be smaller than 8!", nBits < 8);
        // Java: value = (byte)((short)(value & 255) >> nBits | (short)(value & 255) << 8 - nBits);
        let v = value as u16;
        let result = ((v >> nBits) | (v << (8 - nBits))) as u8;
        // Java: return value;
        result
    }

    // Java: int lclGetLen(byte[] pnPassData, int nBufferSize)
    pub fn lclGetLen(pnPassData: &[u8], nBufferSize: i32) -> i32 {
        // Java: int nLen;
        let mut nLen: i32 = 0;
        // Java: for(nLen = 0; nLen < nBufferSize && pnPassData[nLen] != 0; ++nLen)
        while nLen < nBufferSize && (nLen as usize) < pnPassData.len() && pnPassData[nLen as usize] != 0 {
            nLen += 1;
        }
        // Java: return nLen;
        nLen
    }

    // Java: void shortToBytes(short value, byte[] ret)
    fn shortToBytes(value: i16, ret: &mut [u8]) {
        // Java: IOAssert.assertTrue("ret should not be null!", ret != null);
        // Java: IOAssert.assertTrue("ret.length  should be 2!", ret.length == 2);
        // Java: ret[1] = (byte)(value >> 8 & 255);
        ret[1] = (value >> 8) as u8;
        // Java: ret[0] = (byte)(value & 255);
        ret[0] = value as u8;
    }

    // Java: short lclGetHash(byte[] pnPassData, int nBufferSize)
    pub fn lclGetHash(pnPassData: &[u8], nBufferSize: i32) -> i16 {
        // Java: int nLen = lclGetLen(pnPassData, nBufferSize);
        let nLen = Self::lclGetLen(pnPassData, nBufferSize);
        // Java: short nHash = (short)nLen;
        let mut nHash: i16 = nLen as i16;
        // Java: if (nLen != 0)
        if nLen != 0 {
            // Java: nHash = (short)(nHash ^ '칋');
            nHash = nHash ^ 0xCE4Bu16 as i16;
        }
        // Java: int nOffset = 0;
        let mut nOffset: i32 = 0;
        // Java: for(int nIndex = 0; nIndex < nLen; ++nOffset)
        let mut nIndex = 0;
        while nIndex < nLen {
            // Java: short cChar = (short)pnPassData[nOffset];
            let cChar = pnPassData[nOffset as usize] as i16;
            // Java: byte nRot = (byte)((nIndex + 1) % 15);
            let nRot = ((nIndex + 1) % 15) as u8;
            // Java: cChar = lclRotateLeft(cChar, nRot, (byte)15);
            let cChar = Self::lclRotateLeft_short_width(cChar, nRot, 15);
            // Java: nHash ^= cChar;
            nHash ^= cChar;
            // Java: ++nIndex;
            nIndex += 1;
            nOffset += 1;
        }
        // Java: return nHash;
        nHash
    }

    // Java: short lclGetKey(byte[] pnPassData, int nBufferSize)
    pub fn lclGetKey(pnPassData: &[u8], nBufferSize: i32) -> i16 {
        // Java: int nLen = lclGetLen(pnPassData, nBufferSize);
        let nLen = Self::lclGetLen(pnPassData, nBufferSize);
        // Java: if (nLen == 0)
        if nLen == 0 {
            // Java: return 0;
            return 0;
        }
        // Java: else
        // Java: short nKey = 0;
        let mut nKey: i16 = 0;
        // Java: short nKeyBase = Short.MIN_VALUE;
        let mut nKeyBase: i16 = i16::MIN;
        // Java: short nKeyEnd = -1;
        let mut nKeyEnd: i16 = -1;
        // Java: int nOffset = nLen - 1;
        let mut nOffset = nLen - 1;
        // Java: for(int nIndex = 0; nIndex < nLen; --nOffset)
        let mut nIndex = 0;
        while nIndex < nLen {
            // Java: byte cChar = (byte)(pnPassData[nOffset] & 127);
            let mut cChar = (pnPassData[nOffset as usize] & 0x7F) as u8;
            // Java: for(byte nBit = 0; nBit < 8; ++nBit)
            for _nBit in 0..8u8 {
                // Java: nKeyBase = lclRotateLeft((short)nKeyBase, (byte)1);
                nKeyBase = Self::lclRotateLeft_short(nKeyBase, 1);
                // Java: if ((nKeyBase & 1) != 0)
                if (nKeyBase & 1) != 0 {
                    // Java: nKeyBase = (short)(nKeyBase ^ 4128);
                    nKeyBase = nKeyBase ^ 4128;
                }
                // Java: if ((cChar & 1) != 0)
                if (cChar & 1) != 0 {
                    // Java: nKey ^= nKeyBase;
                    nKey ^= nKeyBase;
                }
                // Java: cChar = (byte)(cChar >> 1);
                cChar = cChar >> 1;
                // Java: nKeyEnd = lclRotateLeft((short)nKeyEnd, (byte)1);
                nKeyEnd = Self::lclRotateLeft_short(nKeyEnd, 1);
                // Java: if ((nKeyEnd & 1) != 0)
                if (nKeyEnd & 1) != 0 {
                    // Java: nKeyEnd = (short)(nKeyEnd ^ 4128);
                    nKeyEnd = nKeyEnd ^ 4128;
                }
            }
            // Java: ++nIndex;
            nIndex += 1;
            nOffset -= 1;
        }
        // Java: return (short)(nKey ^ nKeyEnd);
        nKey ^ nKeyEnd
    }

    // Java: boolean verifyXOREncryptInfo(int docType, short EKey, short hashValue, String strPassword, ProviderInfo provider)
    pub fn verifyXOREncryptInfo(docType: i32, eKey: i16, hashValue: i16, strPassword: &str, provider: &mut ProviderInfoStruct) -> bool {
        // Java: provider.init_XOR(docType, strPassword);
        provider.init_XOR(docType, strPassword);
        // Java: return provider.m_pXOR.verifyKey(EKey, hashValue);
        if let Some(ref xor) = provider.m_pXOR {
            xor.verifyKey(eKey, hashValue)
        } else {
            false
        }
    }

    // Java: boolean verifySTDEncryptInfo(int docType, DocumentInputStream data, IntegerField offset, String strPassword, ProviderInfo provider)
    fn verifySTDEncryptInfo_stream(docType: i32, data: &mut DocumentInputStream, offset: &mut IntegerFieldLocal, strPassword: &str, provider: &mut ProviderInfoStruct) -> bool {
        // Java: if (data != null && provider != null && offset != null && strPassword != null && strPassword.length() <= 15)
        if strPassword.len() > 15 {
            return false;
        }
        // Java: int nEncryptHeadInfoLen = offset.get();
        let mut nEncryptHeadInfoLen = offset.get();
        // Java: data.seek((long)nEncryptHeadInfoLen);
        data.seek(nEncryptHeadInfoLen as i64);
        // Java: byte[] docId = new byte[16];
        let mut docId = vec![0u8; 16];
        // Java: data.read(docId);
        data.read(&mut docId);
        // Java: if (docId.length != 16)
        // Java: return false;
        // Java: else
        // Java: nEncryptHeadInfoLen += 16;
        nEncryptHeadInfoLen += 16;
        // Java: offset.set(nEncryptHeadInfoLen);
        offset.set(nEncryptHeadInfoLen);
        // Java: byte[] saltData = new byte[16];
        let mut saltData = vec![0u8; 16];
        // Java: data.read(saltData);
        data.read(&mut saltData);
        // Java: if (saltData.length != 16)
        // Java: return false;
        // Java: else
        // Java: nEncryptHeadInfoLen += 16;
        nEncryptHeadInfoLen += 16;
        // Java: offset.set(nEncryptHeadInfoLen);
        offset.set(nEncryptHeadInfoLen);
        // Java: byte[] saltHash = new byte[16];
        let mut saltHash = vec![0u8; 16];
        // Java: data.read(saltHash);
        data.read(&mut saltHash);
        // Java: if (saltHash.length != 16)
        // Java: return false;
        // Java: else
        // Java: nEncryptHeadInfoLen += 16;
        nEncryptHeadInfoLen += 16;
        // Java: offset.set(nEncryptHeadInfoLen);
        offset.set(nEncryptHeadInfoLen);
        // Java: provider.init_STD(docType, strPassword, docId);
        provider.init_STD(docType, strPassword, Some(&docId));
        // Java: return provider.m_pStd.verifyKey(saltData, saltHash);
        if let Some(ref mut std97) = provider.m_pStd {
            std97.verifyKey(&saltData, &saltHash)
        } else {
            // Java: else
            // Java: return false;
            false
        }
    }

    // Java: boolean verifySTDEncryptInfo(int docType, byte[] data, IntegerField offset, String strPassword, ProviderInfo provider)
    fn verifySTDEncryptInfo_bytes(docType: i32, data: &[u8], offset: &mut IntegerFieldLocal, strPassword: &str, provider: &mut ProviderInfoStruct) -> bool {
        // Java: if (data != null && provider != null && offset != null && strPassword != null && strPassword.length() <= 15)
        if strPassword.len() > 15 {
            return false;
        }
        // Java: int nEncryptHeadInfoLen = offset.get();
        let mut nEncryptHeadInfoLen = offset.get();
        // Java: byte[] docId = LittleEndian.getByteArray(data, nEncryptHeadInfoLen, 16);
        let docId = le_get_byte_array(data, nEncryptHeadInfoLen as usize, 16);
        // Java: if (docId.length != 16)
        if docId.len() != 16 {
            // Java: return false;
            return false;
        }
        // Java: else
        // Java: nEncryptHeadInfoLen += 16;
        nEncryptHeadInfoLen += 16;
        // Java: offset.set(nEncryptHeadInfoLen);
        offset.set(nEncryptHeadInfoLen);
        // Java: byte[] saltData = LittleEndian.getByteArray(data, nEncryptHeadInfoLen, 16);
        let saltData = le_get_byte_array(data, nEncryptHeadInfoLen as usize, 16);
        // Java: if (saltData.length != 16)
        if saltData.len() != 16 {
            // Java: return false;
            return false;
        }
        // Java: else
        // Java: nEncryptHeadInfoLen += 16;
        nEncryptHeadInfoLen += 16;
        // Java: offset.set(nEncryptHeadInfoLen);
        offset.set(nEncryptHeadInfoLen);
        // Java: byte[] saltHash = LittleEndian.getByteArray(data, nEncryptHeadInfoLen, 16);
        let saltHash = le_get_byte_array(data, nEncryptHeadInfoLen as usize, 16);
        // Java: if (saltHash.length != 16)
        if saltHash.len() != 16 {
            // Java: return false;
            return false;
        }
        // Java: else
        // Java: nEncryptHeadInfoLen += 16;
        nEncryptHeadInfoLen += 16;
        // Java: offset.set(nEncryptHeadInfoLen);
        offset.set(nEncryptHeadInfoLen);
        // Java: provider.init_STD(docType, strPassword, docId);
        provider.init_STD(docType, strPassword, Some(&docId));
        // Java: return provider.m_pStd.verifyKey(saltData, saltHash);
        if let Some(ref mut std97) = provider.m_pStd {
            std97.verifyKey(&saltData, &saltHash)
        } else {
            // Java: else
            // Java: return false;
            false
        }
    }

    // Java: boolean readEncryptHeadInfo(int docType, DocumentInputStream data, IntegerField offset, String strPassword, ProviderInfo provider)
    pub fn readEncryptHeadInfo_stream(docType: i32, data: &mut DocumentInputStream, offset: &mut IntegerFieldLocal, strPassword: &str, provider: &mut ProviderInfoStruct) -> bool {
        // Java: if (data != null && provider != null && offset != null && strPassword != null)
        // Java: int nEncryptHeadInfoLen = offset.get();
        let mut nEncryptHeadInfoLen = offset.get();
        // Java: data.seek((long)nEncryptHeadInfoLen);
        data.seek(nEncryptHeadInfoLen as i64);
        // Java: int nMajor = data.readUShort();
        let nMajor = data.readUShort();
        // Java: int nMinor = data.readUShort();
        let nMinor = data.readUShort();
        // Java: nEncryptHeadInfoLen += 4;
        nEncryptHeadInfoLen += 4;
        // Java: offset.set(nEncryptHeadInfoLen);
        offset.set(nEncryptHeadInfoLen);
        // Java: if (nMajor == 1 && nMinor == 1)
        if nMajor == 1 && nMinor == 1 {
            // Java: return verifySTDEncryptInfo(docType, data, offset, strPassword, provider);
            return Self::verifySTDEncryptInfo_stream(docType, data, offset, strPassword, provider);
        }
        // Java: else if ((nMajor == 2 || nMinor == 2) && (nMajor == 3 || nMinor == 2))
        else if (nMajor == 2 || nMinor == 2) && (nMajor == 3 || nMinor == 2) {
            // Java: int nEncryptContext = data.readInt();
            let _nEncryptContext = data.readInt();
            // Java: IOAssert.logTrue(nEncryptContext == ENCRYPTDOCATTR || nEncryptContext == NO_ENCRYPTDOCATTR || nEncryptContext == MSO2007ENCRYPTDOCATTR);
            // Java: nEncryptHeadInfoLen += 4;
            nEncryptHeadInfoLen += 4;
            // Java: offset.set(nEncryptHeadInfoLen);
            offset.set(nEncryptHeadInfoLen);
            // Java: byte[] encryptHeadInfo = new byte[36];
            let mut encryptHeadInfo = vec![0u8; 36];
            // Java: data.read(encryptHeadInfo);
            data.read(&mut encryptHeadInfo);
            // Java: nEncryptHeadInfoLen += 36;
            nEncryptHeadInfoLen += 36;
            // Java: offset.set(nEncryptHeadInfoLen);
            offset.set(nEncryptHeadInfoLen);
            // Java: byte[] nameData = null;
            // Java: int nameLen = LittleEndian.getInt(encryptHeadInfo) - 32;
            let mut nameLen = le_get_int(&encryptHeadInfo, 0) - 32;
            // Java: if (nameLen > 0)
            if nameLen > 0 {
                // Java: nameData = new byte[nameLen];
                let mut nameData = vec![0u8; nameLen as usize];
                // Java: data.read(nameData);
                data.read(&mut nameData);
                // Java: nEncryptHeadInfoLen += nameLen;
                nEncryptHeadInfoLen += nameLen;
                // Java: offset.set(nEncryptHeadInfoLen);
                offset.set(nEncryptHeadInfoLen);
                // Java: Object initData = null;
                // Java: int nLen = data.readInt();
                let mut nLen = data.readInt();
                // Java: nEncryptHeadInfoLen += 4;
                nEncryptHeadInfoLen += 4;
                // Java: offset.set(nEncryptHeadInfoLen);
                offset.set(nEncryptHeadInfoLen);
                // Java: if (nLen > 0)
                if nLen > 0 {
                    // Java: byte[] var32 = new byte[nLen];
                    let mut var32 = vec![0u8; nLen as usize];
                    // Java: data.read(var32);
                    data.read(&mut var32);
                    // Java: nEncryptHeadInfoLen += nLen;
                    nEncryptHeadInfoLen += nLen;
                    // Java: offset.set(nEncryptHeadInfoLen);
                    offset.set(nEncryptHeadInfoLen);
                    // Java: if (nameLen > 2 && nameData[nameLen - 2] == 0)
                    if nameLen > 2 && nameData[(nameLen - 2) as usize] == 0 {
                        // Java: nameLen -= 2;
                        nameLen -= 2;
                    }
                    // Java: String strProvName = null;
                    // Java: try
                    // Java: strProvName = new String(nameData, 0, nameLen, "UTF-16LE");
                    let strProvName = String::from_utf16_lossy(
                        &nameData[..nameLen as usize].chunks(2).map(|c| u16::from_le_bytes([c[0], *c.get(1).unwrap_or(&0)])).collect::<Vec<u16>>()
                    );
                    // Java: catch (UnsupportedEncodingException var21)
                    // Java: provider.init_Other(docType, strProvName, LittleEndian.getInt(encryptHeadInfo, 24), strPassword, LittleEndian.getInt(encryptHeadInfo, 16), LittleEndian.getInt(encryptHeadInfo, 12), LittleEndian.getInt(encryptHeadInfo, 20), var32, LittleEndian.getInt(encryptHeadInfo, 4));
                    provider.init_Other(
                        docType, &strProvName,
                        le_get_int(&encryptHeadInfo, 24),
                        strPassword,
                        le_get_int(&encryptHeadInfo, 16),
                        le_get_int(&encryptHeadInfo, 12),
                        le_get_int(&encryptHeadInfo, 20),
                        Some(&var32),
                        le_get_int(&encryptHeadInfo, 4),
                    );
                    // Java: boolean bPasswdOK = false;
                    let mut bPasswdOK = false;
                    // Java: byte[] nextData = new byte[nLen];
                    let mut nextData = vec![0u8; nLen as usize];
                    // Java: data.read(nextData);
                    data.read(&mut nextData);
                    // Java: nEncryptHeadInfoLen += nLen;
                    nEncryptHeadInfoLen += nLen;
                    // Java: offset.set(nEncryptHeadInfoLen);
                    offset.set(nEncryptHeadInfoLen);
                    // Java: IntegerField nKeyIndex = new IntegerField(0, 0);
                    let mut nKeyIndex = IntegerFieldLocal::new(0, 0);
                    // Java: nextData = decryptBytes(provider, nKeyIndex, nextData, 0, nLen);
                    if let Some(d) = Self::decryptBytes(provider, &mut nKeyIndex, &nextData, 0, nLen) {
                        nextData = d;
                    }
                    // Java: nLen = data.readInt();
                    nLen = data.readInt();
                    // Java: nEncryptHeadInfoLen += 4;
                    nEncryptHeadInfoLen += 4;
                    // Java: offset.set(nEncryptHeadInfoLen);
                    offset.set(nEncryptHeadInfoLen);
                    // Java: if (provider.nEncryptDocumentType == ProviderInfo.CRYPTDOCUMENT_MSO2007)
                    if provider.nEncryptDocumentType == ProviderInfo::CRYPTDOCUMENT_MSO2007 {
                        // Java: nLen += 12;
                        nLen += 12;
                    }
                    // Java: byte[] nextNextData = null;
                    let mut nextNextData: Vec<u8>;
                    // Java: if (nLen > 0)
                    if nLen > 0 {
                        // Java: nextNextData = new byte[nLen];
                        nextNextData = vec![0u8; nLen as usize];
                        // Java: data.read(nextNextData);
                        data.read(&mut nextNextData);
                        // Java: nEncryptHeadInfoLen += nLen;
                        nEncryptHeadInfoLen += nLen;
                        // Java: offset.set(nEncryptHeadInfoLen);
                        offset.set(nEncryptHeadInfoLen);
                    }
                    // Java: else
                    else {
                        // Java: nextNextData = new byte[0];
                        nextNextData = Vec::new();
                    }
                    // Java: DigestSHA1 sha = new DigestSHA1();
                    let mut sha = DigestSHA1::new();
                    // Java: byte[] nextDataHash = new byte[20];
                    let mut nextDataHash = vec![0u8; 20];
                    // Java: sha.get(nextData, 0, nextData.length, nextDataHash);
                    sha.get_with_params(&nextData, 0, nextData.len() as i32, &mut nextDataHash);
                    // Java: nextNextData = provider.cipher.doFinal(nextNextData);
                    if let Some(ref mut cipher) = provider.cipher {
                        nextNextData = cipher.doFinal_bytes(&nextNextData);
                    }
                    // Java: bPasswdOK = Arrays.equals(nextDataHash, nextNextData);
                    bPasswdOK = nextDataHash == nextNextData;
                    // Java: return bPasswdOK;
                    return bPasswdOK;
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
                return false;
            }
        }
        // Java: else
        else {
            // Java: return false;
            return false;
        }
        // Java: else
        // Java: return false;
    }

    // Java: boolean readEncryptHeadInfo(int docType, byte[] data, IntegerField offset, String strPassword, ProviderInfo provider)
    pub fn readEncryptHeadInfo_bytes(docType: i32, data: &[u8], offset: &mut IntegerFieldLocal, strPassword: &str, provider: &mut ProviderInfoStruct) -> bool {
        // Java: if (data != null && provider != null && offset != null && strPassword != null)
        // Java: int nEncryptHeadInfoLen = offset.get();
        let mut nEncryptHeadInfoLen = offset.get();
        // Java: byte[] encryptVersion = LittleEndian.getByteArray(data, nEncryptHeadInfoLen, 4);
        let encryptVersion = le_get_byte_array(data, nEncryptHeadInfoLen as usize, 4);
        // Java: if (encryptVersion.length != 4)
        if encryptVersion.len() != 4 {
            // Java: return false;
            return false;
        }
        // Java: else
        // Java: nEncryptHeadInfoLen += 4;
        nEncryptHeadInfoLen += 4;
        // Java: offset.set(nEncryptHeadInfoLen);
        offset.set(nEncryptHeadInfoLen);
        // Java: int nMajor = LittleEndian.getUShort(encryptVersion);
        let nMajor = le_get_ushort(&encryptVersion, 0);
        // Java: int nMinor = LittleEndian.getUShort(encryptVersion, 2);
        let nMinor = le_get_ushort(&encryptVersion, 2);
        // Java: if (nMajor == 1 && nMinor == 1)
        if nMajor == 1 && nMinor == 1 {
            // Java: return verifySTDEncryptInfo(docType, data, offset, strPassword, provider);
            return Self::verifySTDEncryptInfo_bytes(docType, data, offset, strPassword, provider);
        }
        // Java: else if ((nMajor == 2 || nMinor == 2) && (nMajor == 3 || nMinor == 2))
        else if (nMajor == 2 || nMinor == 2) && (nMajor == 3 || nMinor == 2) {
            // Java: byte[] encryptContext = LittleEndian.getByteArray(data, nEncryptHeadInfoLen, 4);
            let encryptContext = le_get_byte_array(data, nEncryptHeadInfoLen as usize, 4);
            // Java: nEncryptHeadInfoLen += 4;
            nEncryptHeadInfoLen += 4;
            // Java: offset.set(nEncryptHeadInfoLen);
            offset.set(nEncryptHeadInfoLen);
            // Java: int nEncryptContext = LittleEndian.getInt(encryptContext);
            let _nEncryptContext = le_get_int(&encryptContext, 0);
            // Java: IOAssert.logTrue(nEncryptContext == ENCRYPTDOCATTR || nEncryptContext == NO_ENCRYPTDOCATTR || nEncryptContext == MSO2007ENCRYPTDOCATTR);
            // Java: byte[] encryptHeadInfo = LittleEndian.getByteArray(data, nEncryptHeadInfoLen, 36);
            let encryptHeadInfo = le_get_byte_array(data, nEncryptHeadInfoLen as usize, 36);
            // Java: if (encryptHeadInfo.length != 36)
            if encryptHeadInfo.len() != 36 {
                // Java: return false;
                return false;
            }
            // Java: else
            // Java: nEncryptHeadInfoLen += 36;
            nEncryptHeadInfoLen += 36;
            // Java: offset.set(nEncryptHeadInfoLen);
            offset.set(nEncryptHeadInfoLen);
            // Java: byte[] nameData = null;
            // Java: int nameLen = LittleEndian.getInt(encryptHeadInfo) - 32;
            let mut nameLen = le_get_int(&encryptHeadInfo, 0) - 32;
            // Java: if (nameLen <= 0)
            if nameLen <= 0 {
                // Java: return false;
                return false;
            }
            // Java: else
            // Java: nameData = LittleEndian.getByteArray(data, nEncryptHeadInfoLen, nameLen);
            let nameData = le_get_byte_array(data, nEncryptHeadInfoLen as usize, nameLen as usize);
            // Java: nEncryptHeadInfoLen += nameLen;
            nEncryptHeadInfoLen += nameLen;
            // Java: offset.set(nEncryptHeadInfoLen);
            offset.set(nEncryptHeadInfoLen);
            // Java: byte[] cbInfo2 = LittleEndian.getByteArray(data, nEncryptHeadInfoLen, 4);
            let cbInfo2 = le_get_byte_array(data, nEncryptHeadInfoLen as usize, 4);
            // Java: nEncryptHeadInfoLen += 4;
            nEncryptHeadInfoLen += 4;
            // Java: offset.set(nEncryptHeadInfoLen);
            offset.set(nEncryptHeadInfoLen);
            // Java: Object initData = null;
            // Java: int nLen = LittleEndian.getInt(cbInfo2);
            let mut nLen = le_get_int(&cbInfo2, 0);
            // Java: if (cbInfo2.length == 4 && nLen > 0)
            if cbInfo2.len() == 4 && nLen > 0 {
                // Java: byte[] var35 = LittleEndian.getByteArray(data, nEncryptHeadInfoLen, nLen);
                let var35 = le_get_byte_array(data, nEncryptHeadInfoLen as usize, nLen as usize);
                // Java: nEncryptHeadInfoLen += nLen;
                nEncryptHeadInfoLen += nLen;
                // Java: offset.set(nEncryptHeadInfoLen);
                offset.set(nEncryptHeadInfoLen);
                // Java: if (nameLen > 2 && nameData[nameLen - 2] == 0)
                if nameLen > 2 && nameData[(nameLen - 2) as usize] == 0 {
                    // Java: nameLen -= 2;
                    nameLen -= 2;
                }
                // Java: String strProvName = null;
                // Java: try
                // Java: strProvName = new String(nameData, 0, nameLen, "UTF-16LE");
                let strProvName = String::from_utf16_lossy(
                    &nameData[..nameLen as usize].chunks(2).map(|c| u16::from_le_bytes([c[0], *c.get(1).unwrap_or(&0)])).collect::<Vec<u16>>()
                );
                // Java: catch (UnsupportedEncodingException var24)
                // Java: provider.init_Other(docType, strProvName, LittleEndian.getInt(encryptHeadInfo, 24), strPassword, LittleEndian.getInt(encryptHeadInfo, 16), LittleEndian.getInt(encryptHeadInfo, 12), LittleEndian.getInt(encryptHeadInfo, 20), var35, LittleEndian.getInt(encryptHeadInfo, 4));
                provider.init_Other(
                    docType, &strProvName,
                    le_get_int(&encryptHeadInfo, 24),
                    strPassword,
                    le_get_int(&encryptHeadInfo, 16),
                    le_get_int(&encryptHeadInfo, 12),
                    le_get_int(&encryptHeadInfo, 20),
                    Some(&var35),
                    le_get_int(&encryptHeadInfo, 4),
                );
                // Java: boolean bPasswdOK = false;
                let mut bPasswdOK = false;
                // Java: byte[] nextData = LittleEndian.getByteArray(data, nEncryptHeadInfoLen, nLen);
                let mut nextData = le_get_byte_array(data, nEncryptHeadInfoLen as usize, nLen as usize);
                // Java: if (nextData.length != nLen)
                if nextData.len() != nLen as usize {
                    // Java: return false;
                    return false;
                }
                // Java: else
                // Java: nEncryptHeadInfoLen += nLen;
                nEncryptHeadInfoLen += nLen;
                // Java: offset.set(nEncryptHeadInfoLen);
                offset.set(nEncryptHeadInfoLen);
                // Java: IntegerField nKeyIndex = new IntegerField(0, 0);
                let mut nKeyIndex = IntegerFieldLocal::new(0, 0);
                // Java: nextData = decryptBytes(provider, nKeyIndex, nextData, 0, nLen);
                if let Some(d) = Self::decryptBytes(provider, &mut nKeyIndex, &nextData, 0, nLen) {
                    nextData = d;
                }
                // Java: nLen = LittleEndian.getInt(data, nEncryptHeadInfoLen);
                nLen = le_get_int(data, nEncryptHeadInfoLen as usize);
                // Java: nEncryptHeadInfoLen += 4;
                nEncryptHeadInfoLen += 4;
                // Java: offset.set(nEncryptHeadInfoLen);
                offset.set(nEncryptHeadInfoLen);
                // Java: if (provider.nEncryptDocumentType == ProviderInfo.CRYPTDOCUMENT_MSO2007)
                if provider.nEncryptDocumentType == ProviderInfo::CRYPTDOCUMENT_MSO2007 {
                    // Java: nLen += 12;
                    nLen += 12;
                }
                // Java: byte[] nextNextData = null;
                let mut nextNextData: Vec<u8>;
                // Java: if (nLen > 0)
                if nLen > 0 {
                    // Java: nextNextData = LittleEndian.getByteArray(data, nEncryptHeadInfoLen, nLen);
                    nextNextData = le_get_byte_array(data, nEncryptHeadInfoLen as usize, nLen as usize);
                    // Java: if (nextNextData == null || nextNextData.length != nLen)
                    if nextNextData.len() != nLen as usize {
                        // Java: return false;
                        return false;
                    }
                    // Java: nEncryptHeadInfoLen += nLen;
                    nEncryptHeadInfoLen += nLen;
                    // Java: offset.set(nEncryptHeadInfoLen);
                    offset.set(nEncryptHeadInfoLen);
                }
                // Java: else
                else {
                    // Java: nextNextData = new byte[0];
                    nextNextData = Vec::new();
                }
                // Java: DigestSHA1 sha = new DigestSHA1();
                let mut sha = DigestSHA1::new();
                // Java: byte[] nextDataHash = new byte[20];
                let mut nextDataHash = vec![0u8; 20];
                // Java: sha.get(nextData, 0, nextData.length, nextDataHash);
                sha.get_with_params(&nextData, 0, nextData.len() as i32, &mut nextDataHash);
                // Java: nextNextData = provider.cipher.doFinal(nextNextData);
                if let Some(ref mut cipher) = provider.cipher {
                    nextNextData = cipher.doFinal_bytes(&nextNextData);
                }
                // Java: bPasswdOK = Arrays.equals(nextDataHash, nextNextData);
                bPasswdOK = nextDataHash == nextNextData;
                // Java: return bPasswdOK;
                return bPasswdOK;
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
            return false;
        }
        // Java: else
        // Java: return false;
    }

    // Java: void decryptXOR(MSCodec_Xor95 xor, DocumentInputStream stream, int nOffset, HWPFOutputStream out)
    // Java: if (xor != null && out != null)
    // Java: xor.initCipher();
    // Java: xor.skip(nOffset);
    // Java: stream.seek((long)nOffset);
    // Java: int nLen = 4096;
    // Java: byte[] data = new byte[nLen];
    // Java: while((nLen = stream.read(data)) != -1)
    // Java: byte[] buffer = xor.decode(data, 0, nLen);
    // Java: out.write(buffer, 0, nLen);

    // Java: byte[] decryptXOR(MSCodec_Xor95 xor, byte[] data, int nOffset, ByteArrayOutputStream out)
    // Java: if (xor != null && out != null)
    // Java: xor.initCipher();
    // Java: xor.skip(nOffset);
    // Java: for(int nPos = nOffset; nPos < data.length; nPos += 16534)
    // Java: int nLen = data.length - nPos > 16534 ? 16534 : data.length - nPos;
    // Java: byte[] buffer = xor.decode(data, nPos, nLen);
    // Java: out.write(buffer, 0, nLen);
    // Java: return out.toByteArray();
    // Java: else
    // Java: return null;

    // Java: byte[] cryptDecrypt(ProviderInfo cryptInf, byte[] data, int nOffset, int nLen)
    pub fn cryptDecrypt(cryptInf: &mut ProviderInfoStruct, data: &[u8], nOffset: i32, nLen: i32) -> Option<Vec<u8>> {
        // Java: byte[] ret = null;
        // Java: if (cryptInf.nEncryptType == ProviderInfo.ENCRYPT_STD)
        if cryptInf.nEncryptType == ProviderInfo::ENCRYPT_STD {
            // Java: ret = new byte[nLen];
            let mut ret = vec![0u8; nLen as usize];
            // Java: cryptInf.m_pStd.decode(data, nOffset, nLen, ret);
            if let Some(ref mut std97) = cryptInf.m_pStd {
                std97.decode(data, nOffset, nLen, &mut ret);
            }
            Some(ret)
        }
        // Java: else if (cryptInf.nEncryptType == ProviderInfo.ENCRYPT_XOR)
        else if cryptInf.nEncryptType == ProviderInfo::ENCRYPT_XOR {
            // Java: ret = cryptInf.m_pXOR.decode(data, nOffset, nLen);
            if let Some(ref mut xor) = cryptInf.m_pXOR {
                let mut data_copy = data.to_vec();
                Some(xor.decode(&mut data_copy, nOffset, nLen))
            } else {
                None
            }
        }
        // Java: else if (cryptInf.nEncryptType == ProviderInfo.ENCRYPT_OTHER)
        else if cryptInf.nEncryptType == ProviderInfo::ENCRYPT_OTHER {
            // Java: ret = new byte[nLen];
            let mut ret = vec![0u8; nLen as usize];
            // Java: cryptInf.cipher.doFinal(data, nOffset, nLen, ret);
            if let Some(ref mut cipher) = cryptInf.cipher {
                cipher.doFinal_range(data, nOffset, nLen, &mut ret);
            }
            Some(ret)
        } else {
            // Java: return ret;
            None
        }
    }

    // Java: boolean createCryptKey(ProviderInfo cryptInf, int keyIndex)
    pub fn createCryptKey(cryptInf: &mut ProviderInfoStruct, keyIndex: i32) -> bool {
        // Java: boolean bRet = false;
        let mut bRet = false;
        // Java: if (cryptInf.nEncryptType == ProviderInfo.ENCRYPT_STD)
        if cryptInf.nEncryptType == ProviderInfo::ENCRYPT_STD {
            // Java: bRet = cryptInf.m_pStd.initCipher(keyIndex);
            if let Some(ref mut std97) = cryptInf.m_pStd {
                bRet = std97.initCipher(keyIndex);
            }
        }
        // Java: else if (cryptInf.nEncryptType == ProviderInfo.ENCRYPT_XOR)
        else if cryptInf.nEncryptType == ProviderInfo::ENCRYPT_XOR {
            // Java: cryptInf.m_pXOR.initCipher();
            if let Some(ref mut xor) = cryptInf.m_pXOR {
                xor.initCipher();
            }
            // Java: bRet = true;
            bRet = true;
        }
        // Java: else if (cryptInf.nEncryptType == ProviderInfo.ENCRYPT_OTHER)
        else if cryptInf.nEncryptType == ProviderInfo::ENCRYPT_OTHER {
            // Java: try
            // Java: byte[] block = new byte[4];
            let mut block = [0u8; 4];
            // Java: LittleEndian.putInt(block, keyIndex);
            le_put_int(&mut block, 0, keyIndex);
            // Java: cryptInf.initCipher(cryptInf.pInitData, cryptInf.nInitDataLen, cryptInf.strPassword, cryptInf.nKeyLen / 8, block);
            let init_data = cryptInf.pInitData.clone();
            let init_data_len = cryptInf.nInitDataLen;
            let password = cryptInf.strPassword.clone();
            let key_len = cryptInf.nKeyLen;
            cryptInf.initCipher(&init_data, init_data_len, &password, key_len / 8, &block);
            // Java: bRet = true;
            bRet = true;
            // Java: catch (Exception var4)
            // Java: bRet = false;
        }
        // Java: return bRet;
        bRet
    }

    // Java: byte[] decryptBlock(ProviderInfo cryptInf, IntegerField keyIndex, byte[] data, int nOffset, int offset, int nLen)
    fn decryptBlock(cryptInf: &mut ProviderInfoStruct, keyIndex: &mut IntegerFieldLocal, data: &[u8], nOffset: i32, offset: i32, nLen: i32) -> Option<Vec<u8>> {
        // Java: IOAssert.logNotNull("cryptInf should not be null!", cryptInf);
        // Java: IOAssert.logNotNull("data should not be null!", data);
        // Java: IOAssert.logTrue("nLen <= ENCRYPT_BLOCK_SIZE", nLen <= ENCRYPT_BLOCK_SIZE);
        // Java: IOAssert.logTrue("( nLen + nOffset )== ENCRYPT_BLOCK_SIZE", nLen + nOffset == ENCRYPT_BLOCK_SIZE);
        // Java: byte[] ret = null;
        let mut ret = None;
        // Java: if (createCryptKey(cryptInf, keyIndex.get()))
        if Self::createCryptKey(cryptInf, keyIndex.get()) {
            // Java: if (offset > 0)
            if offset > 0 {
                // Java: cryptDecrypt(cryptInf, data, nOffset, offset);
                Self::cryptDecrypt(cryptInf, data, nOffset, offset);
            }
            // Java: if (nLen > 0)
            if nLen > 0 {
                // Java: ret = cryptDecrypt(cryptInf, data, nOffset + offset, nLen);
                ret = Self::cryptDecrypt(cryptInf, data, nOffset + offset, nLen);
                // Java: keyIndex.set(keyIndex.get() + 1);
                keyIndex.set(keyIndex.get() + 1);
            }
        }
        // Java: return ret;
        ret
    }

    // Java: byte[] decryptBytes(ProviderInfo cryptInfo, IntegerField keyIndex, byte[] data, int nOffset, int nLen)
    pub fn decryptBytes(cryptInfo: &mut ProviderInfoStruct, keyIndex: &mut IntegerFieldLocal, data: &[u8], nOffset: i32, nLen: i32) -> Option<Vec<u8>> {
        // Java: byte[] ret = null;
        let mut ret = None;
        // Java: if (createCryptKey(cryptInfo, keyIndex.get()))
        if Self::createCryptKey(cryptInfo, keyIndex.get()) {
            // Java: ret = cryptDecrypt(cryptInfo, data, nOffset, nLen);
            ret = Self::cryptDecrypt(cryptInfo, data, nOffset, nLen);
            // Java: keyIndex.set(keyIndex.get() + 1);
            keyIndex.set(keyIndex.get() + 1);
        }
        // Java: return ret;
        ret
    }

    // Java: void decryptStreamBuffer(DocumentInputStream dataStrm, int nOffset, ProviderInfo cryptInfo, HWPFOutputStream out)
    // Java: IOAssert.logTrue("dataStrm should not  be null", dataStrm != null);
    // Java: IOAssert.logTrue("out should not  be null", out != null);
    // Java: IOAssert.logTrue("cryptInfo should not  be null", cryptInfo != null);
    // Java: if (dataStrm != null)
    // Java: dataStrm.seek((long)nOffset);
    // Java: if (cryptInfo.nEncryptType == ProviderInfo.ENCRYPT_XOR)
    // Java: decryptXOR(cryptInfo.m_pXOR, dataStrm, nOffset, out);
    // Java: int nStartOffset = nOffset / ENCRYPT_BLOCK_SIZE * ENCRYPT_BLOCK_SIZE;
    // Java: nOffset %= ENCRYPT_BLOCK_SIZE;
    // Java: IntegerField keyIndex = new IntegerField(0, 0);
    // Java: byte[] data = new byte[ENCRYPT_BLOCK_SIZE];
    // Java: int nLen = 0;
    // Java: dataStrm.seek((long)nStartOffset);
    // Java: for(; (nLen = dataStrm.read(data)) != -1; nOffset = 0)
    // Java: if (nLen == ENCRYPT_BLOCK_SIZE)
    // Java: byte[] retData = decryptBlock(cryptInfo, keyIndex, data, 0, nOffset, ENCRYPT_BLOCK_SIZE - nOffset);
    // Java: out.write(retData, 0, retData.length);
    // Java: else
    // Java: byte[] retData = decryptBytes(cryptInfo, keyIndex, data, 0, nLen);
    // Java: out.write(retData, 0, retData.length);

    // Java: byte[] decryptStreamBuffer(byte[] dataStrm, int nOffset, ProviderInfo cryptInfo, ByteArrayOutputStream out)
    // Java: IOAssert.logTrue("dataStrm should not  be null", dataStrm != null);
    // Java: IOAssert.logTrue("out should not  be null", out != null);
    // Java: IOAssert.logTrue("cryptInfo should not  be null", cryptInfo != null);
    // Java: if (dataStrm == null)
    // Java: return null;
    // Java: else
    // Java: IOAssert.logTrue("nOffset should be less than length", nOffset < dataStrm.length);
    // Java: if (cryptInfo.nEncryptType == ProviderInfo.ENCRYPT_XOR)
    // Java: return decryptXOR(cryptInfo.m_pXOR, dataStrm, nOffset, out);
    // Java: else
    // Java: int nStartOffset = nOffset / ENCRYPT_BLOCK_SIZE * ENCRYPT_BLOCK_SIZE;
    // Java: nOffset %= ENCRYPT_BLOCK_SIZE;
    // Java: for(IntegerField keyIndex = new IntegerField(0, 0); nStartOffset < dataStrm.length; nOffset = 0)
    // Java: if (dataStrm.length - nStartOffset > ENCRYPT_BLOCK_SIZE)
    // Java: byte[] data = decryptBlock(cryptInfo, keyIndex, dataStrm, nStartOffset, nOffset, ENCRYPT_BLOCK_SIZE - nOffset);
    // Java: nStartOffset += ENCRYPT_BLOCK_SIZE;
    // Java: out.write(data, 0, data.length);
    // Java: else
    // Java: byte[] data = decryptBytes(cryptInfo, keyIndex, dataStrm, nStartOffset, dataStrm.length - nStartOffset);
    // Java: nStartOffset = dataStrm.length;
    // Java: out.write(data, 0, data.length);
    // Java: return out.toByteArray();

    // Java: byte[] cryptEncrypt(ProviderInfo cryptInf, byte[] data, int nOffset, int nLen)
    fn cryptEncrypt(cryptInf: &mut ProviderInfoStruct, data: &[u8], nOffset: i32, nLen: i32) -> Option<Vec<u8>> {
        // Java: byte[] ret = null;
        // Java: if (cryptInf.nEncryptType == ProviderInfo.ENCRYPT_STD)
        if cryptInf.nEncryptType == ProviderInfo::ENCRYPT_STD {
            // Java: ret = new byte[nLen];
            let mut ret = vec![0u8; nLen as usize];
            // Java: cryptInf.m_pStd.encode(data, nOffset, nLen, ret);
            if let Some(ref mut std97) = cryptInf.m_pStd {
                std97.encode(data, nOffset, nLen, &mut ret);
            }
            Some(ret)
        }
        // Java: else if (cryptInf.nEncryptType == ProviderInfo.ENCRYPT_XOR)
        else if cryptInf.nEncryptType == ProviderInfo::ENCRYPT_XOR {
            // Java: ret = cryptInf.m_pXOR.encode(data, nOffset, nLen);
            if let Some(ref mut xor) = cryptInf.m_pXOR {
                let mut data_copy = data.to_vec();
                Some(xor.encode(&mut data_copy, nOffset, nLen))
            } else {
                None
            }
        }
        // Java: else if (cryptInf.nEncryptType == ProviderInfo.ENCRYPT_OTHER)
        else if cryptInf.nEncryptType == ProviderInfo::ENCRYPT_OTHER {
            // Java: ret = new byte[nLen];
            let mut ret = vec![0u8; nLen as usize];
            // Java: cryptInf.cipher.doFinal(data, nOffset, nLen, ret);
            if let Some(ref mut cipher) = cryptInf.cipher {
                cipher.doFinal_range(data, nOffset, nLen, &mut ret);
            }
            Some(ret)
        } else {
            // Java: return ret;
            None
        }
    }

    // Java: byte[] encryptBytes(ProviderInfo cryptInfo, IntegerField keyIndex, byte[] data, int nOffset, int nLen)
    pub fn encryptBytes(cryptInfo: &mut ProviderInfoStruct, keyIndex: &mut IntegerFieldLocal, data: &[u8], nOffset: i32, nLen: i32) -> Option<Vec<u8>> {
        // Java: byte[] ret = null;
        let mut ret = None;
        // Java: if (createCryptKey(cryptInfo, keyIndex.get()))
        if Self::createCryptKey(cryptInfo, keyIndex.get()) {
            // Java: ret = cryptEncrypt(cryptInfo, data, nOffset, nLen);
            ret = Self::cryptEncrypt(cryptInfo, data, nOffset, nLen);
            // Java: keyIndex.set(keyIndex.get() + 1);
            keyIndex.set(keyIndex.get() + 1);
        }
        // Java: return ret;
        ret
    }

    // Java: byte[] encryptBlock(ProviderInfo cryptInf, IntegerField keyIndex, byte[] data, int nOffset, int offset, int nLen)
    fn encryptBlock(cryptInf: &mut ProviderInfoStruct, keyIndex: &mut IntegerFieldLocal, data: &[u8], nOffset: i32, offset: i32, nLen: i32) -> Option<Vec<u8>> {
        // Java: IOAssert.logNotNull("cryptInf should not be null!", cryptInf);
        // Java: IOAssert.logNotNull("data should not be null!", data);
        // Java: IOAssert.logTrue("nLen <= ENCRYPT_BLOCK_SIZE", nLen <= ENCRYPT_BLOCK_SIZE);
        // Java: IOAssert.logTrue("( nLen + nOffset )== ENCRYPT_BLOCK_SIZE", nLen + nOffset == ENCRYPT_BLOCK_SIZE);
        // Java: byte[] ret = null;
        let mut ret = None;
        // Java: if (createCryptKey(cryptInf, keyIndex.get()))
        if Self::createCryptKey(cryptInf, keyIndex.get()) {
            // Java: if (offset > 0)
            if offset > 0 {
                // Java: cryptEncrypt(cryptInf, data, nOffset, offset);
                Self::cryptEncrypt(cryptInf, data, nOffset, offset);
            }
            // Java: if (nLen > 0)
            if nLen > 0 {
                // Java: ret = cryptEncrypt(cryptInf, data, nOffset + offset, nLen);
                ret = Self::cryptEncrypt(cryptInf, data, nOffset + offset, nLen);
                // Java: keyIndex.set(keyIndex.get() + 1);
                keyIndex.set(keyIndex.get() + 1);
            }
        }
        // Java: return ret;
        ret
    }

    // Java: byte[] encryptStreamBuffer(byte[] dataStrm, int nOffset, ProviderInfo cryptInfo, ByteArrayOutputStream out)
    pub fn encryptStreamBuffer_bytes(dataStrm: &[u8], nOffset: i32, cryptInfo: &mut ProviderInfoStruct) -> Option<Vec<u8>> {
        // Java: if (dataStrm != null && dataStrm.length != 0 && cryptInfo != null && out != null)
        if dataStrm.is_empty() { return None; }
        let mut out = Vec::new();
        // Java: int nStartOffset = nOffset / ENCRYPT_BLOCK_SIZE * ENCRYPT_BLOCK_SIZE;
        let mut nStartOffset = nOffset / ENCRYPT_BLOCK_SIZE * ENCRYPT_BLOCK_SIZE;
        // Java: nOffset %= ENCRYPT_BLOCK_SIZE;
        let mut nOffset = nOffset % ENCRYPT_BLOCK_SIZE;
        // Java: for(IntegerField keyIndex = new IntegerField(0, 0); nStartOffset < dataStrm.length; nOffset = 0)
        let mut keyIndex = IntegerFieldLocal::new(0, 0);
        while nStartOffset < dataStrm.len() as i32 {
            // Java: if (dataStrm.length - nStartOffset > ENCRYPT_BLOCK_SIZE)
            if dataStrm.len() as i32 - nStartOffset > ENCRYPT_BLOCK_SIZE {
                // Java: byte[] data = encryptBlock(cryptInfo, keyIndex, dataStrm, nStartOffset, nOffset, ENCRYPT_BLOCK_SIZE - nOffset);
                if let Some(data) = Self::encryptBlock(cryptInfo, &mut keyIndex, dataStrm, nStartOffset, nOffset, ENCRYPT_BLOCK_SIZE - nOffset) {
                    // Java: nStartOffset += ENCRYPT_BLOCK_SIZE;
                    nStartOffset += ENCRYPT_BLOCK_SIZE;
                    // Java: out.write(data, 0, data.length);
                    out.extend_from_slice(&data);
                } else {
                    nStartOffset += ENCRYPT_BLOCK_SIZE;
                }
            }
            // Java: else
            else {
                // Java: byte[] data = encryptBytes(cryptInfo, keyIndex, dataStrm, nStartOffset, dataStrm.length - nStartOffset);
                if let Some(data) = Self::encryptBytes(cryptInfo, &mut keyIndex, dataStrm, nStartOffset, dataStrm.len() as i32 - nStartOffset) {
                    // Java: nStartOffset = dataStrm.length;
                    nStartOffset = dataStrm.len() as i32;
                    // Java: out.write(data, 0, data.length);
                    out.extend_from_slice(&data);
                } else {
                    nStartOffset = dataStrm.len() as i32;
                }
            }
            nOffset = 0;
        }
        // Java: return out.toByteArray();
        Some(out)
        // Java: else
        // Java: return null;
    }

    // Java: IOLEStream encryptStreamBuffer(IOLEStream dataStrm, int nOffset, ProviderInfo cryptInfo, IOLEStream out)
    // Java: if (dataStrm != null && cryptInfo != null && out != null)
    // Java: OLEEntry entry = new OLEEntry();
    // Java: dataStrm.stat(entry);
    // Java: if (entry.getSize() <= 0)
    // Java: return null;
    // Java: else
    // Java: int nStartOffset = nOffset / ENCRYPT_BLOCK_SIZE * ENCRYPT_BLOCK_SIZE;
    // Java: nOffset %= ENCRYPT_BLOCK_SIZE;
    // Java: IntegerField keyIndex = new IntegerField(0, 0);
    // Java: byte[] data = new byte[ENCRYPT_BLOCK_SIZE];
    // Java: dataStrm.seek(nStartOffset);
    // Java: for(; nStartOffset < entry.getSize(); nOffset = 0)
    // Java: if (entry.getSize() - nStartOffset > ENCRYPT_BLOCK_SIZE)
    // Java: dataStrm.read(data, 0, ENCRYPT_BLOCK_SIZE);
    // Java: byte[] ret = encryptBlock(cryptInfo, keyIndex, data, 0, nOffset, ENCRYPT_BLOCK_SIZE - nOffset);
    // Java: nStartOffset += ENCRYPT_BLOCK_SIZE;
    // Java: out.write(ret, 0, ret.length);
    // Java: else
    // Java: dataStrm.read(data, 0, entry.getSize() - nStartOffset);
    // Java: byte[] ret = encryptBytes(cryptInfo, keyIndex, data, 0, entry.getSize() - nStartOffset);
    // Java: nStartOffset = entry.getSize();
    // Java: out.write(ret, 0, ret.length);
    // Java: return out;
    // Java: else
    // Java: return null;
}
