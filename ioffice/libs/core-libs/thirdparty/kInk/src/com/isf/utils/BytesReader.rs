// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::KSAssert;
// Java: import cn.wps.moffice.kfs.FileInputStream
use kfs_kfs_app::kfs::mfs::FileInputStream::FileInputStream;
// Java: import cn.wps.moffice.util.KSLog
use util::moffice::util::KSLog::KSLog;
// Java: import cn.wps.moffice.util.LittleEndian
use util::moffice::util::LittleEndian::LittleEndian;
// Java: import java.io.FileNotFoundException
// Java: import java.io.IOException
// Java: import java.io.InputStream
use std::io::Read;

// Java field: String TAG
const TAG: &str = "BytesReader";
// Java field: int BUFFER_SIZE
pub(crate) const BUFFER_SIZE: i32 = 4096;

// Java: public class BytesReader
pub struct BytesReader {
    // Java field: byte[] mBuffer
    mBuffer: Vec<u8>,
    // Java field: int mPos
    mPos: i32,
    // Java field: InputStream mIs
    mIs: Option<Box<dyn Read>>,
    // Java field: boolean mEndOfStream
    pub(crate) mEndOfStream: bool,
    // Java field: int mPosOfStream
    pub(crate) mPosOfStream: i32,
}

// Java: public class BytesReader
impl BytesReader {
    // Java: BytesReader(InputStream is)
    pub fn BytesReader_from_stream(is: Box<dyn Read>) -> Self {
        // Java: super();
        // Java: KSAssert.assertNotNull("is should not be null!", is);
        // Java: this.mBuffer = new byte[4096];
        // Java: this.mPos = 4096;
        // Java: this.mEndOfStream = false;
        // Java: this.mIs = is;
        // Java: this.mPosOfStream = 0;
        Self {
            mBuffer: vec![0u8; 4096],
            mPos: 4096,
            mEndOfStream: false,
            mIs: Some(is),
            mPosOfStream: 0,
        }
    }

    // Java: BytesReader(String path)
    pub fn BytesReader_from_path(path: &str) -> Self {
        // Java: super();
        // Java: KSAssert.assertNotNull("path should not be null!", path);
        // Java: this.mBuffer = new byte[4096];
        // Java: this.mPos = 4096;
        // Java: this.mEndOfStream = false;
        let mut result = Self {
            mBuffer: vec![0u8; 4096],
            mPos: 4096,
            mEndOfStream: false,
            mIs: None,
            mPosOfStream: 0,
        };
        // Java: try
        // Java: this.mIs = new FileInputStream(path);
        match FileInputStream::from_name(path) {
            Ok(fis) => {
                result.mIs = Some(Box::new(fis));
            }
            // Java: catch (FileNotFoundException e)
            Err(e) => {
                // Java: KSLog.e(TAG, "FileNotFoundException", e);
                KSLog::e(TAG, &format!("FileNotFoundException: {}", e));
            }
        }
        result
    }

    // Java: int getByte()
    pub fn getByte(&mut self) -> i32 {
        // Java: KSAssert.assertNotNull("mBuffer should not be null!", this.mBuffer);
        // Java: if (4096 - this.mPos < 1)
        if 4096 - self.mPos < 1 {
            // Java: this.read();
            self.read();
        }
        // Java: int ret = this.mBuffer[this.mPos++] & 255;
        let ret = (self.mBuffer[self.mPos as usize] as i32) & 255;
        self.mPos += 1;
        // Java: ++this.mPosOfStream;
        self.mPosOfStream += 1;
        // Java: return ret;
        ret
    }

    // Java: int getInt()
    pub fn getInt(&mut self) -> i32 {
        // Java: if (4096 - this.mPos < 4)
        if 4096 - self.mPos < 4 {
            // Java: this.read();
            self.read();
        }
        // Java: int ret = LittleEndian.getInt(this.mBuffer, this.mPos);
        let ret = LittleEndian::getInt(&self.mBuffer, self.mPos as usize);
        self.mPos += 4;
        // Java: ++this.mPosOfStream;
        self.mPosOfStream += 4;
        // Java: return ret;
        ret
    }

    // Java: int getShort()
    pub fn getShort(&mut self) -> i32 {
        // Java: if (4096 - this.mPos < 2)
        if 4096 - self.mPos < 2 {
            // Java: this.read();
            self.read();
        }
        // Java: int ret = LittleEndian.getShort(this.mBuffer, this.mPos) & '\uffff';
        let ret = (LittleEndian::getShort(&self.mBuffer, self.mPos as usize) as i32) & 0xFFFF;
        self.mPos += 2;
        // Java: ++this.mPosOfStream;
        self.mPosOfStream += 2;
        // Java: return ret;
        ret
    }

    // Java: boolean isEOF()
    pub fn isEOF(&self) -> bool {
        // Java: KSAssert.assertNotNull("mIs should not be null!", this.mIs);
        // Java: return this.mEndOfStream && this.mPos >= 4096;
        self.mEndOfStream && self.mPos >= 4096
    }

    // Java: void read()
    pub fn read(&mut self) {
        // Java: KSAssert.assertNotNull("mIs should not be null!", this.mIs);
        // Java: int end = 4096 - this.mPos;
        let end = (4096 - self.mPos) as usize;
        // Java: for(int i = 0; i < end; ++i)
        for i in 0..end {
            // Java: this.mBuffer[i] = this.mBuffer[i + this.mPos];
            self.mBuffer[i] = self.mBuffer[i + self.mPos as usize];
        }
        // Java: try
        if let Some(ref mut is) = self.mIs {
            // Java: int n = this.mIs.read(this.mBuffer, end, 4096 - end);
            match is.read(&mut self.mBuffer[end..4096]) {
                Ok(n) => {
                    // Java: if (-1 == n)
                    if n == 0 {
                        // Java: this.mEndOfStream = true;
                        self.mEndOfStream = true;
                    }
                }
                // Java: catch (IOException e)
                Err(e) => {
                    // Java: KSLog.e(TAG, "IOException", e);
                    KSLog::e(TAG, &format!("IOException: {}", e));
                }
            }
        }
        // Java: this.mPos = 0;
        self.mPos = 0;
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: KSAssert.assertNotNull("mIs should not be null!", this.mIs);
        // Java: try
        // Java: this.mIs.close();
        // Java: catch (IOException e)
        // Java: KSLog.e(TAG, "IOException", e);
        self.mIs = None;
    }

    // Java: int pos()
    pub fn pos(&self) -> i32 {
        // Java: return this.mPosOfStream;
        self.mPosOfStream
    }

    // Java: byte getSingleByte()
    pub fn getSingleByte(&mut self) -> i8 {
        // Java: KSAssert.assertNotNull("mBuffer should not be null!", this.mBuffer);
        // Java: if (4096 - this.mPos < 1)
        if 4096 - self.mPos < 1 {
            // Java: this.read();
            self.read();
        }
        // Java: byte ret = LittleEndian.getByte(this.mBuffer, this.mPos++);
        let ret = LittleEndian::getByte(&self.mBuffer, self.mPos as usize) as i8;
        self.mPos += 1;
        // Java: ++this.mPosOfStream;
        self.mPosOfStream += 1;
        // Java: return ret;
        ret
    }
}
