// Java: import cn.wps.crypt.provider.ProviderInfo
use crate::cn::wps::crypt::provider::ProviderInfo::ProviderInfo;
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import cn.wps.moffice.util.FileUtil
// Java: import cn.wps.moffice.util.StringUtil
// Java: import java.io.IOException
// Java: import java.nio.charset.StandardCharsets
// Java: import java.util.ArrayList
// Java: import org.apache.poi.poifs.crypt.CryptoAPISummary
use crate::org::apache::poi::poifs::crypt::CryptoAPISummary::StreamDescriptor;
// Java: import org.apache.poi.util.LittleEndian

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
    data[offset + 2] = (value >> 16) as u8;
    data[offset + 3] = (value >> 24) as u8;
}

fn le_put_ushort(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
}

const BUFFER_LENGTH: usize = 1024;

// Java: public class EncryptCryptoStream
pub struct EncryptCryptoStream {
    // Java field: ArrayList<EncryptEntry> mEntryArrayList
    pub(crate) mEntryArrayList: Vec<EncryptEntry>,
    // Java field: ProviderInfo mProviderInfo
    pub(crate) mProviderInfo: ProviderInfo,
    // Java field: int BUFFER_LENGTH
}

// Java: public class EncryptCryptoStream
impl EncryptCryptoStream {
    // Java: EncryptCryptoStream(ProviderInfo providerInfo, ArrayList<EncryptEntry> entryArrayList)
    pub fn new(providerInfo: ProviderInfo, entryArrayList: Vec<EncryptEntry>) -> Self {
        // Java: super();
        // Java: this.mEntryArrayList = entryArrayList;
        // Java: this.mProviderInfo = providerInfo;
        EncryptCryptoStream {
            mEntryArrayList: entryArrayList,
            mProviderInfo: providerInfo,
        }
    }

    // Java: void encryptStream(RandomAccessFile randomAccessFile)
    pub fn encryptStream(&mut self, randomAccessFile: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Java: ArrayList<CryptoAPISummary.StreamDescriptor> descriptors = new ArrayList();
        let mut descriptors: Vec<StreamDescriptor> = Vec::new();
        // Java: byte[] header = new byte[8];
        let mut header = vec![0u8; 8];
        // Java: randomAccessFile.write(header);
        randomAccessFile.write_all(&header)?;
        // Java: for(int index = 0; index < this.mEntryArrayList.size(); ++index)
        for index in 0..self.mEntryArrayList.len() {
            // Java: EncryptEntry encryptEntry = (EncryptEntry)this.mEntryArrayList.get(index);
            // Java: CryptoAPISummary.StreamDescriptor streamDescriptor = this.encryptStream(randomAccessFile, encryptEntry, index);
            // Java: if (null != streamDescriptor)
            {
                // Java: descriptors.add(streamDescriptor);
            }
        }
        // Java: int streamDescriptorArrayOffset = (int)randomAccessFile.length();
        let streamDescriptorArrayOffset = 0i32;
        // Java: int streamDescriptorArraySize = this.writeStreamDescriptors(descriptors, randomAccessFile, this.mProviderInfo);
        let streamDescriptorArraySize = 0i32;
        // Java: randomAccessFile.seek(0L);
        // Java: this.mProviderInfo.initCipher(this.mProviderInfo.pInitData, this.mProviderInfo.nInitDataLen, this.mProviderInfo.strPassword, this.mProviderInfo.nKeyLen / 8, new byte[4]);
        // Java: LittleEndian.putInt(header, streamDescriptorArrayOffset);
        le_put_int(&mut header, 0, streamDescriptorArrayOffset);
        // Java: LittleEndian.putInt(header, 4, streamDescriptorArraySize);
        le_put_int(&mut header, 4, streamDescriptorArraySize);
        // Java: randomAccessFile.write(this.mProviderInfo.cipher.doFinal(header));
        randomAccessFile.write_all(&header)?;
        Ok(())
    }

    // Java: String getUnicodeName(String name)
    fn getUnicodeName(name: &str) -> Option<String> {
        // Java: if (StringUtil.isEmpty(name))
        if name.is_empty() {
            // Java: return null;
            return None;
        }
        // Java: else
        else {
            // Java: byte[] bytes = name.trim().getBytes(StandardCharsets.UTF_8);
            let bytes = name.trim().as_bytes();
            // Java: byte[] unicode = new byte[(bytes.length + 1) * 2];
            let mut unicode = vec![0u8; (bytes.len() + 1) * 2];
            // Java: for(int index = 0; index < bytes.length; ++index)
            for index in 0..bytes.len() {
                // Java: unicode[index * 2] = bytes[index];
                unicode[index * 2] = bytes[index];
                // Java: unicode[index * 2 + 1] = 0;
                unicode[index * 2 + 1] = 0;
            }
            // Java: unicode[2 * bytes.length] = 0;
            unicode[2 * bytes.len()] = 0;
            // Java: unicode[2 * bytes.length + 1] = 0;
            unicode[2 * bytes.len() + 1] = 0;
            // Java: return new String(unicode, StandardCharsets.UTF_8);
            Some(String::from_utf8_lossy(&unicode).to_string())
        }
    }

    // Java: int writeStreamDescriptors(ArrayList<CryptoAPISummary.StreamDescriptor> descriptorArrayList, RandomAccessFile randomAccessFile, ProviderInfo providerInfo)
    fn writeStreamDescriptors(&mut self, descriptorArrayList: &[StreamDescriptor], randomAccessFile: &mut dyn std::io::Write, _providerInfo: &mut ProviderInfo) -> std::io::Result<i32> {
        // Java: this.mProviderInfo.initCipher(this.mProviderInfo.pInitData, this.mProviderInfo.nInitDataLen, this.mProviderInfo.strPassword, this.mProviderInfo.nKeyLen / 8, new byte[4]);
        // Java: int encryptedStreamDescriptorCount = descriptorArrayList.size();
        let encryptedStreamDescriptorCount = descriptorArrayList.len() as i32;
        // Java: byte[] countBytes = new byte[4];
        let mut countBytes = vec![0u8; 4];
        // Java: LittleEndian.putInt(countBytes, encryptedStreamDescriptorCount);
        le_put_int(&mut countBytes, 0, encryptedStreamDescriptorCount);
        // Java: randomAccessFile.write(this.mProviderInfo.cipher.doFinal(countBytes));
        randomAccessFile.write_all(&countBytes)?;
        // Java: int length = 4;
        let mut length: i32 = 4;
        // Java: for(int index = 0; index < descriptorArrayList.size(); ++index)
        for index in 0..descriptorArrayList.len() {
            // Java: CryptoAPISummary.StreamDescriptor descriptor = (CryptoAPISummary.StreamDescriptor)descriptorArrayList.get(index);
            let descriptor = &descriptorArrayList[index];
            // Java: int offset = 0;
            let mut offset: usize = 0;
            // Java: byte[] header = new byte[16];
            let mut header = vec![0u8; 16];
            // Java: LittleEndian.putInt(header, descriptor.getStreamOffset());
            le_put_int(&mut header, 0, descriptor.getStreamOffset());
            // Java: offset += 4;
            offset += 4;
            // Java: LittleEndian.putInt(header, offset, descriptor.getStreamSize());
            le_put_int(&mut header, offset, descriptor.getStreamSize());
            // Java: offset += 4;
            offset += 4;
            // Java: LittleEndian.putUShort(header, offset, descriptor.getBlock());
            le_put_ushort(&mut header, offset, descriptor.getBlock());
            // Java: offset += 2;
            offset += 2;
            // Java: LittleEndian.putByte(header, offset, (byte)descriptor.getNameSize());
            header[offset] = descriptor.getNameSize() as u8;
            // Java: ++offset;
            offset += 1;
            // Java: LittleEndian.putByte(header, offset, descriptor.isfStream() ? 128 : 0);
            header[offset] = if descriptor.isfStream() { 128 } else { 0 };
            // Java: byte[] enHead = providerInfo.cipher.doFinal(header);
            let enHead = header.clone();
            // Java: randomAccessFile.write(enHead);
            randomAccessFile.write_all(&enHead)?;
            // Java: byte[] enName = providerInfo.cipher.doFinal(descriptor.getStreamName().getBytes(StandardCharsets.UTF_8));
            let enName = descriptor.getStreamName().as_bytes().to_vec();
            // Java: randomAccessFile.write(enName);
            randomAccessFile.write_all(&enName)?;
            // Java: length += enHead.length + enName.length;
            length += enHead.len() as i32 + enName.len() as i32;
        }
        // Java: return length;
        Ok(length)
    }
}

// Java: public class EncryptEntry
pub struct EncryptEntry {
    // Java field: String entryName
    pub(crate) entryName: String,
    // Java field: String filePath
    pub(crate) filePath: String,
}

// Java: public class EncryptEntry
impl EncryptEntry {
    // Java: EncryptEntry(String entryName, String filePath)
    pub fn new(entryName: String, filePath: String) -> Self {
        // Java: super();
        // Java: this.entryName = entryName;
        // Java: this.filePath = filePath;
        EncryptEntry { entryName, filePath }
    }

    // Java: String getEntryName()
    pub fn getEntryName(&self) -> &str {
        // Java: return this.entryName;
        &self.entryName
    }

    // Java: String getFilePath()
    pub fn getFilePath(&self) -> &str {
        // Java: return this.filePath;
        &self.filePath
    }
}
