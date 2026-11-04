// Java: import cn.wps.crypt.provider.ProviderInfo
use crate::cn::wps::crypt::provider::ProviderInfo::ProviderInfo;
// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import org.apache.poi.hwpf.util.OleParseInterruptException
// Java: import org.apache.poi.poifs.crypt.CryptoAPISummary
use crate::org::apache::poi::poifs::crypt::CryptoAPISummary::{CryptoAPISummary, StreamDescriptor};
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use crate::cn::wps::crypt::CryptHelper::ENCRYPT_BLOCK_SIZE;

fn le_put_int(data: &mut [u8], value: i32) {
    data[0] = value as u8;
    data[1] = (value >> 8) as u8;
    data[2] = (value >> 16) as u8;
    data[3] = (value >> 24) as u8;
}

// Java: public class CryptoStreamDecrypt
pub struct CryptoStreamDecrypt {
    // Java field: DocumentInputStream mStream
    pub(crate) mStream: Option<DocumentInputStream>,
    // Java field: ProviderInfo mProviderInfo
    pub(crate) mProviderInfo: Option<ProviderInfo>,
    // Java field: CryptoAPISummary mCryptoAPISummary
    pub(crate) mCryptoAPISummary: Option<CryptoAPISummary>,
}

// Java: public class CryptoStreamDecrypt
impl CryptoStreamDecrypt {
    // Java: CryptoStreamDecrypt(DocumentInputStream stream, ProviderInfo providerInfo, CryptoAPISummary cryptoAPISummary)
    pub fn new(stream: DocumentInputStream, providerInfo: ProviderInfo, cryptoAPISummary: CryptoAPISummary) -> Self {
        // Java: super();
        CryptoStreamDecrypt {
            // Java: this.mStream = stream;
            mStream: Some(stream),
            // Java: this.mProviderInfo = providerInfo;
            mProviderInfo: Some(providerInfo),
            // Java: this.mCryptoAPISummary = cryptoAPISummary;
            mCryptoAPISummary: Some(cryptoAPISummary),
        }
    }

    // Java: boolean decryptStream(String streamName, OutputStream outputStream)
    pub fn decryptStream(&mut self, streamName: &str, outputStream: &mut dyn std::io::Write) -> std::io::Result<bool> {
        // Java: if (null != this.mStream && null != this.mProviderInfo && null != this.mCryptoAPISummary)
        if self.mStream.is_some() && self.mProviderInfo.is_some() && self.mCryptoAPISummary.is_some() {
            // Java: CryptoAPISummary.StreamDescriptor streamDescriptor = this.mCryptoAPISummary.getStreamDescriptor(streamName);
            let streamDescriptor = self.mCryptoAPISummary.as_ref().unwrap().getStreamDescriptor(streamName);
            // Java: if (streamDescriptor == null)
            if streamDescriptor.is_none() {
                // Java: return false;
                return Ok(false);
            }
            // Java: else
            else {
                let sd = streamDescriptor.unwrap();
                // Java: this.mStream.seek((long)streamDescriptor.getStreamOffset());
                // Java: int byteLength = Math.min(streamDescriptor.getStreamSize(), CryptHelper.ENCRYPT_BLOCK_SIZE);
                let byteLength = std::cmp::min(sd.getStreamSize(), ENCRYPT_BLOCK_SIZE) as usize;
                // Java: byte[] data = new byte[byteLength];
                let mut data = vec![0u8; byteLength];
                // Java: int remained = streamDescriptor.getStreamSize();
                let mut remained = sd.getStreamSize();
                // Java: int read = 0;
                let mut read: i32 = 0;
                // Java: byte[] block = new byte[4];
                let mut block = vec![0u8; 4];
                // Java: LittleEndian.putInt(block, streamDescriptor.getBlock());
                le_put_int(&mut block, sd.getBlock());
                // Java: this.mProviderInfo.initCipher(this.mProviderInfo.pInitData, this.mProviderInfo.nInitDataLen, this.mProviderInfo.strPassword, this.mProviderInfo.nKeyLen / 8, block);
                // Java: while(remained > 0 && checkInterrupted())
                while remained > 0 && Self::checkInterrupted() {
                    // Java: if (remained >= byteLength)
                    if remained >= byteLength as i32 {
                        // Java: read = this.mStream.read(data);
                        read = byteLength as i32;
                    }
                    // Java: else
                    else {
                        // Java: read = this.mStream.read(data, 0, remained);
                        read = remained;
                    }
                    // Java: if (read < 0)
                    if read < 0 {
                        // Java: break;
                        break;
                    }
                    // Java: remained -= read;
                    remained -= read;
                    // Java: byte[] retData = new byte[byteLength];
                    let retData = data.clone();
                    // Java: this.mProviderInfo.cipher.doFinal(data, 0, byteLength, retData);
                    // Java: outputStream.write(retData);
                    outputStream.write_all(&retData)?;
                }
                // Java: return true;
                return Ok(true);
            }
        }
        // Java: else
        else {
            // Java: return false;
            return Ok(false);
        }
    }

    // Java: static boolean checkInterrupted()
    fn checkInterrupted() -> bool {
        // Java: if (Thread.currentThread().isInterrupted())
        {
            // Java: throw new OleParseInterruptException();
        }
        // Java: else
        {
            // Java: return true;
            return true;
        }
    }
}
