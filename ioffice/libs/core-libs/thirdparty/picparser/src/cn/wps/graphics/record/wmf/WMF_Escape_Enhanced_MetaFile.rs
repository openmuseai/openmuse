#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_Escape_Enhanced_MetaFile extends WMFRecord
pub struct WMF_Escape_Enhanced_MetaFile {
    // Java field: boolean mIsValid
    mIsValid: bool,
    // Java field: FileOutputStream mOutStream
    mOutStream: Option<i32>,
    // Java field: String mPath
    mPath: Option<String>,
}

impl WMF_Escape_Enhanced_MetaFile {
    // Java: WMF_Escape_Enhanced_MetaFile()
    pub fn new() -> Self {
        // Java: super();
        WMF_Escape_Enhanced_MetaFile {
            mIsValid: true,
            mOutStream: None,
            mPath: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int size)
    pub fn read(&mut self, _wmf: i32, _size: i32) -> Option<i32> {
        // Java: if (!this.mIsValid)
        if !self.mIsValid {
            // Java: return null;
            return None;
        // Java: else
        } else {
            // Java: try
            {
                // Java: int commentIdentifier = wmf.readUnsignedInt();
                let commentIdentifier: i32 = 0;
                // Java: int commentType = wmf.readUnsignedInt();
                let commentType: i32 = 0;
                // Java: int verion = wmf.readUnsignedInt();
                let verion: i32 = 0;
                // Java: int checkSum = wmf.readUnsignedShort();
                let checkSum: i32 = 0;
                // Java: int flags = wmf.readUnsignedInt();
                let flags: i32 = 0;
                // Java: if (commentIdentifier != 1128680791 || commentType != 1 || verion != 65536 || flags != 0)
                if commentIdentifier != 1128680791 || commentType != 1 || verion != 65536 || flags != 0 {
                    // Java: this.mIsValid = false;
                    self.mIsValid = false;
                    // Java: return null;
                    return None;
                }
            }
            // Java: catch (ArrayIndexOutOfBoundsException e)
            {
                // Java: e.printStackTrace();
                // Java: this.mIsValid = false;
                self.mIsValid = false;
                // Java: return null;
                return None;
            }

            // Java: int commentRecordCount = wmf.readUnsignedInt();
            let commentRecordCount: i32 = 0;
            // Java: int currentRecordSize = wmf.readUnsignedInt();
            let currentRecordSize: i32 = 0;
            // Java: int remainingBytes = wmf.readUnsignedInt();
            let remainingBytes: i32 = 0;
            // Java: int enhancedMetafileDataSize = wmf.readUnsignedInt();
            let enhancedMetafileDataSize: i32 = 0;
            // Java: byte[] buf = new byte[currentRecordSize];
            let buf: Vec<u8> = vec![0u8; currentRecordSize as usize];
            // Java: wmf.read(buf);
            // Java: OutputStream os = this.getOutStream();
            let os = self.getOutStream();
            // Java: if (os != null)
            if os.is_some() {
                // Java: try
                {
                    // Java: os.write(buf);
                }
                // Java: catch (IOException e)
                {
                    // Java: this.mIsValid = false;
                    self.mIsValid = false;
                    // Java: e.printStackTrace();
                }
            }

            // Java: if (remainingBytes == 0)
            if remainingBytes == 0 {
                // Java: this.closeStream();
                self.closeStream();
                // Java: return this;
                return Some(0);
            // Java: else
            } else {
                // Java: return remainingBytes == 0 ? this : null;
                return if remainingBytes == 0 { Some(0) } else { None };
            }
        }
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
    }

    // Java: protected OutputStream getOutStream()
    pub fn getOutStream(&mut self) -> Option<i32> {
        // Java: if (this.mOutStream == null)
        if self.mOutStream.is_none() {
            // Java: String dir = Platform.getCurrentPidTmpDir();
            let dir = String::new();
            // Java: File file = new File(dir);
            let mut file: i32 = 0;
            // Java: if (!file.exists())
            {
                // Java: file.mkdirs();
            }

            // Java: this.mPath = dir + "/" + System.currentTimeMillis() + ".emf";
            self.mPath = Some(format!("{}/{}.emf", dir, 0));
            // Java: file = new File(this.mPath);
            file = 0;
            // Java: if (!file.exists())
            {
                // Java: try
                {
                    // Java: file.createNewFile();
                }
                // Java: catch (IOException e)
                {
                    // Java: e.printStackTrace();
                }
            }

            // Java: try
            {
                // Java: this.mOutStream = new FileOutputStream(file);
                self.mOutStream = Some(0);
            }
            // Java: catch (FileNotFoundException e)
            {
                // Java: e.printStackTrace();
            }
        }

        // Java: return this.mOutStream;
        self.mOutStream
    }

    // Java: protected void closeStream()
    pub fn closeStream(&mut self) {
        // Java: if (this.mOutStream != null)
        if self.mOutStream.is_some() {
            // Java: try
            {
                // Java: this.mOutStream.close();
            }
            // Java: catch (IOException e)
            {
                // Java: e.printStackTrace();
            }

            // Java: this.mOutStream = null;
            self.mOutStream = None;
        }
    }

    // Java: public String getPath()
    pub fn getPath(&self) -> Option<String> {
        // Java: return this.mPath;
        self.mPath.clone()
    }

    // Java: public boolean isValid()
    pub fn isValid(&self) -> bool {
        // Java: return this.mIsValid;
        self.mIsValid
    }
}
