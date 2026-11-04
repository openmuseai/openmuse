// Java: import cn.wps.crypt.provider.ProviderInfo
use crate::cn::wps::crypt::provider::ProviderInfo::ProviderInfo;
// Java: import java.io.IOException
// Java: import java.io.UnsupportedEncodingException
// Java: import java.nio.charset.StandardCharsets
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

fn le_get_int(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32) | ((data[offset + 1] as i32) << 8) | ((data[offset + 2] as i32) << 16) | ((data[offset + 3] as i32) << 24)
}

fn le_get_ushort(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32) | ((data[offset + 1] as i32) << 8)
}

// Java: public class CryptoAPISummary
pub struct CryptoAPISummary {
    // Java field: int streamDescriptorArrayOffset
    streamDescriptorArrayOffset: i32,
    // Java field: int streamDescriptorArraySize
    streamDescriptorArraySize: i32,
    // Java field: int encryptedStreamDescriptorCount
    encryptedStreamDescriptorCount: i32,
    // Java field: HashMap<String, StreamDescriptor> mDescriptorMap
    mDescriptorMap: HashMap<String, StreamDescriptor>,
}

// Java: public class CryptoAPISummary
impl CryptoAPISummary {
    // Java: CryptoAPISummary(DocumentInputStream stream, ProviderInfo providerInfo)
    pub fn new(stream: &mut DocumentInputStream, providerInfo: &mut ProviderInfo) -> Self {
        // Java: super();
        let mut summary = CryptoAPISummary {
            streamDescriptorArrayOffset: 0,
            streamDescriptorArraySize: 0,
            encryptedStreamDescriptorCount: 0,
            mDescriptorMap: HashMap::new(),
        };
        // Java: this.initHeader(stream, providerInfo);
        summary.initHeader(stream, providerInfo);
        summary
    }

    // Java: void initHeader(DocumentInputStream stream, ProviderInfo providerInfo)
    fn initHeader(&mut self, stream: &mut DocumentInputStream, providerInfo: &mut ProviderInfo) {
        // Java: stream.seek(0L);
        // Java: providerInfo.initCipher(providerInfo.pInitData, providerInfo.nInitDataLen, providerInfo.strPassword, providerInfo.nKeyLen / 8, new byte[4]);
        // Java: byte[] out = this.decryptBytes(stream, providerInfo, new byte[8]);
        let out = self.decryptBytes(stream, providerInfo, 8);
        // Java: this.streamDescriptorArrayOffset = LittleEndian.getInt(out);
        self.streamDescriptorArrayOffset = le_get_int(&out, 0);
        // Java: this.streamDescriptorArraySize = LittleEndian.getInt(out, 4);
        self.streamDescriptorArraySize = le_get_int(&out, 4);
        // Java: stream.seek((long)this.streamDescriptorArrayOffset);
        // Java: providerInfo.initCipher(providerInfo.pInitData, providerInfo.nInitDataLen, providerInfo.strPassword, providerInfo.nKeyLen / 8, new byte[4]);
        // Java: out = this.decryptBytes(stream, providerInfo, new byte[4]);
        let out = self.decryptBytes(stream, providerInfo, 4);
        // Java: this.encryptedStreamDescriptorCount = LittleEndian.getInt(out);
        self.encryptedStreamDescriptorCount = le_get_int(&out, 0);
        // Java: int remained = this.streamDescriptorArraySize - 4;
        let mut remained = self.streamDescriptorArraySize - 4;
        // Java: while(remained > 0)
        while remained > 0 {
            // Java: byte[] decHeader = this.decryptBytes(stream, providerInfo, new byte[16]);
            let decHeader = self.decryptBytes(stream, providerInfo, 16);
            // Java: remained -= decHeader.length;
            remained -= decHeader.len() as i32;
            // Java: StreamDescriptor streamDescriptor = this.initStreamDescriptor(decHeader);
            let mut streamDescriptor = Self::initStreamDescriptor(&decHeader);
            // Java: if (streamDescriptor.getNameSize() > 0)
            if streamDescriptor.getNameSize() > 0 {
                // Java: int nameLength = (streamDescriptor.getNameSize() + 1) * 2;
                let nameLength = (streamDescriptor.getNameSize() + 1) * 2;
                // Java: if (remained >= nameLength)
                if remained >= nameLength {
                    // Java: byte[] nameByte = this.decryptBytes(stream, providerInfo, new byte[nameLength]);
                    let nameByte = self.decryptBytes(stream, providerInfo, nameLength as usize);
                    // Java: String streamName = new String(nameByte, StandardCharsets.UTF_8);
                    let streamName = String::from_utf8_lossy(&nameByte).to_string();
                    // Java: remained -= nameLength;
                    remained -= nameLength;
                    // Java: streamDescriptor.setStreamName(streamName);
                    streamDescriptor.setStreamName(streamName.clone());
                    // Java: String key = this.unicode2String(streamName);
                    let key = Self::unicode2String(&streamName);
                    // Java: this.mDescriptorMap.put(key, streamDescriptor);
                    self.mDescriptorMap.insert(key, streamDescriptor);
                }
            }
        }
    }

    // Java: byte[] decryptBytes(DocumentInputStream stream, ProviderInfo providerInfo, byte[] input)
    fn decryptBytes(&self, stream: &mut DocumentInputStream, providerInfo: &mut ProviderInfo, size: usize) -> Vec<u8> {
        // Java: stream.read(input);
        let mut input = vec![0u8; size];
        let _ = stream.read(&mut input);
        // Java: return providerInfo.cipher.doFinal(input);
        input
    }

    // Java: StreamDescriptor initStreamDescriptor(byte[] header)
    fn initStreamDescriptor(header: &[u8]) -> StreamDescriptor {
        // Java: int offset = 0;
        let mut offset: usize = 0;
        // Java: int streamOffset = LittleEndian.getInt(header, offset);
        let streamOffset = le_get_int(header, offset);
        // Java: offset += 4;
        offset += 4;
        // Java: int streamSize = LittleEndian.getInt(header, offset);
        let streamSize = le_get_int(header, offset);
        // Java: offset += 4;
        offset += 4;
        // Java: int block = LittleEndian.getUShort(header, offset);
        let block = le_get_ushort(header, offset);
        // Java: offset += 2;
        offset += 2;
        // Java: int nameSize = LittleEndian.getByte(header, offset);
        let nameSize = header[offset] as i32;
        // Java: ++offset;
        offset += 1;
        // Java: boolean fStream = (LittleEndian.getUByte(header, offset) & 128) != 0;
        let fStream = (header[offset] as i32 & 128) != 0;
        // Java: StreamDescriptor streamDescriptor = new StreamDescriptor(streamOffset, streamSize, block, nameSize, fStream);
        let streamDescriptor = StreamDescriptor::new(streamOffset, streamSize, block, nameSize, fStream);
        // Java: return streamDescriptor;
        streamDescriptor
    }

    // Java: String unicode2String(String unicode)
    fn unicode2String(unicode: &str) -> String {
        // Java: StringBuffer string = new StringBuffer();
        let mut string = String::new();
        // Java: String[] hex = unicode.split("\u0000");
        let hex: Vec<&str> = unicode.split('\0').collect();
        // Java: for(int i = 0; i < hex.length; ++i)
        for i in 0..hex.len() {
            // Java: string.append(hex[i]);
            string.push_str(hex[i]);
        }
        // Java: return string.toString();
        string
    }

    // Java: StreamDescriptor getStreamDescriptor(String name)
    pub fn getStreamDescriptor(&self, name: &str) -> Option<&StreamDescriptor> {
        // Java: return null != this.mDescriptorMap ? (StreamDescriptor)this.mDescriptorMap.get(name) : null;
        self.mDescriptorMap.get(name)
    }
}

// Java: public class StreamDescriptor
pub struct StreamDescriptor {
    // Java field: int streamOffset
    pub(crate) streamOffset: i32,
    // Java field: int streamSize
    pub(crate) streamSize: i32,
    // Java field: int block
    pub(crate) block: i32,
    // Java field: int nameSize
    pub(crate) nameSize: i32,
    // Java field: boolean fStream
    pub(crate) fStream: bool,
    // Java field: String streamName
    pub(crate) streamName: String,
}

// Java: public class StreamDescriptor
impl StreamDescriptor {
    // Java: void setStreamName(String streamName)
    pub fn setStreamName(&mut self, streamName: String) {
        // Java: this.streamName = streamName;
        self.streamName = streamName;
    }

    // Java: int getStreamOffset()
    pub fn getStreamOffset(&self) -> i32 {
        // Java: return this.streamOffset;
        self.streamOffset
    }

    // Java: int getStreamSize()
    pub fn getStreamSize(&self) -> i32 {
        // Java: return this.streamSize;
        self.streamSize
    }

    // Java: int getBlock()
    pub fn getBlock(&self) -> i32 {
        // Java: return this.block;
        self.block
    }

    // Java: int getNameSize()
    pub fn getNameSize(&self) -> i32 {
        // Java: return this.nameSize;
        self.nameSize
    }

    // Java: boolean isfStream()
    pub fn isfStream(&self) -> bool {
        // Java: return this.fStream;
        self.fStream
    }

    // Java: String getStreamName()
    pub fn getStreamName(&self) -> &str {
        // Java: return this.streamName;
        &self.streamName
    }

    // Java: StreamDescriptor(int streamOffset, int streamSize, int block, int nameSize, boolean fStream)
    pub fn new(streamOffset: i32, streamSize: i32, block: i32, nameSize: i32, fStream: bool) -> Self {
        // Java: super();
        // Java: this.streamOffset = streamOffset;
        // Java: this.streamSize = streamSize;
        // Java: this.block = block;
        // Java: this.nameSize = nameSize;
        // Java: this.fStream = fStream;
        StreamDescriptor { streamOffset, streamSize, block, nameSize, fStream, streamName: String::new() }
    }
}
