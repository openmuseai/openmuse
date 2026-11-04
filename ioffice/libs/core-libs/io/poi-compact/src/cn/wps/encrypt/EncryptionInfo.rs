// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
use crate::org::apache::poi::poifs::filesystem::DirectoryNode::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use crate::org::apache::poi::poifs::filesystem::POIFSFileSystem::POIFSFileSystem;
// Java: import org.apache.poi.util.LittleEndian

use crate::cn::wps::encrypt::EncryptionHeader::EncryptionHeader;
use crate::cn::wps::encrypt::EncryptionVerifier::EncryptionVerifier;

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
    data[offset + 2] = (value >> 16) as u8;
    data[offset + 3] = (value >> 24) as u8;
}

// Java: public class EncryptionInfo
pub struct EncryptionInfo {
    // Java field: int versionMajor
    pub(crate) versionMajor: i32,
    // Java field: int versionMinor
    pub(crate) versionMinor: i32,
    // Java field: int encryptionFlags
    pub(crate) encryptionFlags: i32,
    // Java field: EncryptionHeader header
    pub(crate) header: Option<EncryptionHeader>,
    // Java field: EncryptionVerifier verifier
    pub(crate) verifier: Option<EncryptionVerifier>,
    // Java field: byte[] allInfo
    pub(crate) allInfo: Option<Vec<u8>>,
    // Java field: byte[] version
    pub(crate) version: Option<Vec<u8>>,
}

// Java: public class EncryptionInfo
impl EncryptionInfo {
    // Java: EncryptionInfo(POIFSFileSystem fs)
    pub fn new_from_fs(fs: &mut POIFSFileSystem) -> Self {
        // Java: this(fs.getRoot());
        Self::new_from_dir(fs.getRoot())
    }

    // Java: EncryptionInfo(DirectoryNode dir)
    pub fn new_from_dir(dir: &mut DirectoryNode) -> Self {
        // Java: super();
        // Java: DocumentInputStream dis = dir.createDocumentInputStream("EncryptionInfo");
        let mut dis = dir.createDocumentInputStream_by_name("EncryptionInfo").expect("createDocumentInputStream failed");
        // Java: this.versionMajor = dis.readShort();
        let versionMajor = dis.readShort() as i32;
        // Java: this.versionMinor = dis.readShort();
        let versionMinor = dis.readShort() as i32;
        // Java: this.encryptionFlags = dis.readInt();
        let encryptionFlags = dis.readInt();
        let header;
        let verifier;
        // Java: if (this.versionMajor == 4 && this.versionMinor == 4 && this.encryptionFlags == 64)
        if versionMajor == 4 && versionMinor == 4 && encryptionFlags == 64 {
            // Java: StringBuilder builder = new StringBuilder();
            let mut builder = String::new();
            // Java: byte[] xmlDescriptor = new byte[dis.available()];
            let avail = dis.available() as usize;
            let mut xmlDescriptor = vec![0u8; avail];
            // Java: dis.read(xmlDescriptor);
            let _ = dis.read(&mut xmlDescriptor);
            // Java: for(byte b : xmlDescriptor)
            for &b in &xmlDescriptor {
                // Java: builder.append((char)b);
                builder.push(b as char);
            }
            // Java: String descriptor = builder.toString();
            let descriptor = builder;
            // Java: this.header = new EncryptionHeader(descriptor);
            header = Some(EncryptionHeader::new_from_descriptor(&descriptor));
            // Java: this.verifier = new EncryptionVerifier(descriptor);
            verifier = Some(EncryptionVerifier::new_from_descriptor(&descriptor));
        }
        // Java: else
        else {
            // Java: dis.readInt();
            let _ = dis.readInt();
            // Java: this.header = new EncryptionHeader(dis);
            let h = EncryptionHeader::new_from_stream(&mut dis);
            // Java: if (this.header.getAlgorithm() == 26625)
            if h.getAlgorithm() == 26625 {
                // Java: this.verifier = new EncryptionVerifier(dis, 20);
                verifier = Some(EncryptionVerifier::new_from_stream(&mut dis, 20));
            }
            // Java: else
            else {
                // Java: this.verifier = new EncryptionVerifier(dis, 32);
                verifier = Some(EncryptionVerifier::new_from_stream(&mut dis, 32));
            }
            header = Some(h);
        }
        EncryptionInfo {
            versionMajor,
            versionMinor,
            encryptionFlags,
            header,
            verifier,
            allInfo: None,
            version: None,
        }
    }

    // Java: EncryptionInfo(String password)
    pub fn new_from_password(password: &str) -> Self {
        // Java: super();
        // Java: this.allInfo = new byte[224];
        let mut allInfo = vec![0u8; 224];
        // Java: this.versionMajor = 3;
        let versionMajor = 3;
        // Java: int offset = 0;
        let mut offset: usize = 0;
        // Java: this.versionMinor = 2;
        let versionMinor = 2;
        // Java: LittleEndian.putInt(this.allInfo, offset, 131075);
        le_put_int(&mut allInfo, offset, 131075);
        // Java: offset += 4;
        offset += 4;
        // Java: this.encryptionFlags = 36;
        let encryptionFlags = 36;
        // Java: LittleEndian.putInt(this.allInfo, offset, 36);
        le_put_int(&mut allInfo, offset, 36);
        // Java: offset += 4;
        offset += 4;
        // Java: LittleEndian.putInt(this.allInfo, offset, 140);
        le_put_int(&mut allInfo, offset, 140);
        // Java: offset += 4;
        offset += 4;
        // Java: Integer iOffset = new Integer(offset);
        let iOffset = offset as i32;
        // Java: this.header = new EncryptionHeader(this.allInfo, iOffset);
        let header = EncryptionHeader::new_from_bytes(&mut allInfo, iOffset);
        // Java: this.verifier = new EncryptionVerifier(this.allInfo, this.header.getIOffset(), password, true);
        let verifier = EncryptionVerifier::new_from_bytes(&mut allInfo, header.getIOffset().unwrap_or(0), password, true);
        EncryptionInfo {
            versionMajor,
            versionMinor,
            encryptionFlags,
            header: Some(header),
            verifier: Some(verifier),
            allInfo: Some(allInfo),
            version: None,
        }
    }

    // Java: int getVersionMajor()
    pub fn getVersionMajor(&self) -> i32 {
        // Java: return this.versionMajor;
        self.versionMajor
    }

    // Java: int getVersionMinor()
    pub fn getVersionMinor(&self) -> i32 {
        // Java: return this.versionMinor;
        self.versionMinor
    }

    // Java: int getEncryptionFlags()
    pub fn getEncryptionFlags(&self) -> i32 {
        // Java: return this.encryptionFlags;
        self.encryptionFlags
    }

    // Java: EncryptionHeader getHeader()
    pub fn getHeader(&self) -> Option<&EncryptionHeader> {
        // Java: return this.header;
        self.header.as_ref()
    }

    // Java: EncryptionVerifier getVerifier()
    pub fn getVerifier(&self) -> Option<&EncryptionVerifier> {
        // Java: return this.verifier;
        self.verifier.as_ref()
    }

    // Java: byte[] getAllInfo()
    pub fn getAllInfo(&self) -> Option<&[u8]> {
        // Java: return this.allInfo;
        self.allInfo.as_deref()
    }
}
