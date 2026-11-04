#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import cn.wps.moffice.kfs.raw.EntryBase
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import cn.wps.moffice.kfs.zip.ZipFile
// Java: import cn.wps.moffice.kfs.zip.ZipOutputStream
// Java: import cn.wps.moffice.util.FileUtil
// Java: import cn.wps.util.manager.FileManager
// Java: import cn.wps.util.zip.model.buffer.ThreadBufferUtils
// Java: import java.io.BufferedOutputStream
// Java: import java.io.Closeable
// Java: import java.io.FileNotFoundException
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import java.util.zip.Inflater
use std::path::PathBuf;
use std::io::{Read, Write, BufWriter};

use crate::util::zip::model::buffer::ThreadBufferUtils::ThreadBufferUtils;
use crate::util::manager::FileManager::FileManager;

// Java: public class ZipBaseModel implements ZipOutputStream.DeflateDataCallBack, ZipOutputStream.IListener
pub struct ZipBaseModel {
    // Java field: File mDecompressedFile
    mDecompressedFile: Option<PathBuf>,
    // Java field: String mDecompressSuffix
    mDecompressSuffix: String,
    // Java field: File mCompressedFile
    mCompressedFile: Option<PathBuf>,
    // Java field: BufferedOutputStream mCompressedOutputStream
    mCompressedOutputStream: Option<BufWriter<std::fs::File>>,
    // Java field: ZipEntry mZipEntry
    mZipEntry: Option<()>,
    // Java field: FileManager mFileManager
    mFileManager: FileManager,
}

// Java: public class ZipBaseModel implements ZipOutputStream.DeflateDataCallBack, ZipOutputStream.IListener
impl ZipBaseModel {
    // Java: ZipBaseModel(ZipFile.RAFStream inputStream, String decompressSuffix, ZipEntry zipEntry, int minRatio, FileManager fileManager)
    pub fn new(input_stream: &mut dyn Read, decompress_suffix: &str, min_ratio: i32, file_manager: FileManager) -> Self {
        // Java: super();
        // Java: this.mDecompressSuffix = decompressSuffix;
        // Java: this.mFileManager = fileManager;
        // Java: if (this.initZipEntry(zipEntry))
        // Java: this.initData(inputStream);
        // Java: else if (this.getCompressedRatio() < minRatio)
        // Java: this.initDataRatio(inputStream);
        // Java: this.mZipEntry = null;
        // Java: else if (this.mDecompressedFile == null)
        // Java: this.initDataZip(inputStream);
        Self {
            mDecompressedFile: None,
            mDecompressSuffix: decompress_suffix.to_string(),
            mCompressedFile: None,
            mCompressedOutputStream: None,
            mZipEntry: None,
            mFileManager: file_manager,
        }
    }

    // Java: ZipBaseModel(ZipFile.RAFStream inputStream, String decompressSuffix, ZipEntry zipEntry, FileManager fileManager)
    pub fn new_default(input_stream: &mut dyn Read, decompress_suffix: &str, file_manager: FileManager) -> Self {
        // Java: this(inputStream, decompressSuffix, zipEntry, 10, fileManager);
        Self::new(input_stream, decompress_suffix, 10, file_manager)
    }

    // Java: void initDataRatio(ZipFile.RAFStream inputStream)
    fn initDataRatio(&mut self, input_stream: &mut dyn Read) {
        // Java: boolean decompression = this.decompression(inputStream);
        let decompression = self.decompression(input_stream);
        // Java: if (!decompression)
        if !decompression
        {
            // Java: this.mDecompressedFile = null;
            self.mDecompressedFile = None;
        }
    }

    // Java: boolean initZipEntry(ZipEntry zipEntry)
    fn initZipEntry(&mut self) -> bool {
        // Java: if (zipEntry == null)
        // Java: throw new RuntimeException("zipEntry == null");
        // Java: else
        // Java: this.mZipEntry = (ZipEntry)zipEntry.clone();
        // Java: this.mZipEntry.setMode(1);
        // Java: return zipEntry.getMethod() == 0;
        false
    }

    // Java: int getCompressedRatio()
    fn getCompressedRatio(&self) -> i32 {
        // Java: if (this.mZipEntry == null)
        if self.mZipEntry.is_none()
        {
            // Java: return 100;
            return 100;
        }
        // Java: else
        {
            // Java: long size = this.mZipEntry.getSize();
            // Java: long compressedSize = this.mZipEntry.getCompressedSize();
            // Java: return compressedSize > 0L && size > 0L ? (int)((size - compressedSize) * 100L / size) : 0;
            0
        }
    }

    // Java: ZipBaseModel(String decompressSuffix, FileManager fileManager)
    pub fn new_empty(decompress_suffix: &str, file_manager: FileManager) -> Self {
        // Java: super();
        // Java: this.mDecompressSuffix = decompressSuffix;
        // Java: this.mFileManager = fileManager;
        Self {
            mDecompressedFile: None,
            mDecompressSuffix: decompress_suffix.to_string(),
            mCompressedFile: None,
            mCompressedOutputStream: None,
            mZipEntry: None,
            mFileManager: file_manager,
        }
    }

    // Java: void initData(InputStream inputStream)
    fn initData(&mut self, input_stream: &mut dyn Read) {
        // Java: OutputStream outputStream = null;
        // Java: try
        {
            // Java: this.mDecompressedFile = this.mFileManager.createTempFile(this.mDecompressSuffix);
            self.mDecompressedFile = self.mFileManager.createTempFile(&self.mDecompressSuffix);
            // Java: outputStream = new FileOutputStream(this.mDecompressedFile);
            // Java: FileUtil.copy(inputStream, outputStream, ThreadBufferUtils.getBytes());
            if let Some(ref path) = self.mDecompressedFile {
                if let Ok(mut output) = std::fs::File::create(path) {
                    let mut buf = vec![0u8; 8192];
                    loop {
                        match input_stream.read(&mut buf) {
                            Ok(0) => break,
                            Ok(n) => { let _ = output.write_all(&buf[..n]); }
                            Err(_) => break,
                        }
                    }
                }
            }
        }
        // Java: catch (Exception e)
        // Java: IOLog.e("ZipBaseModel", "initData error", e);
        // Java: this.mDecompressedFile = null;
        // Java: finally
        // Java: FileUtil.closeQuietly((Closeable)outputStream);
    }

    // Java: void initDataZip(ZipFile.RAFStream rafStream)
    fn initDataZip(&mut self, raf_stream: &mut dyn Read) {
        // Java: FileOutputStream fileOutputStream = null;
        // Java: try
        {
            // Java: this.mCompressedFile = this.mFileManager.createTempFile(this.mDecompressSuffix);
            self.mCompressedFile = self.mFileManager.createTempFile(&self.mDecompressSuffix);
            // Java: fileOutputStream = new FileOutputStream(this.mCompressedFile);
            // Java: FileUtil.copy(rafStream, fileOutputStream, ThreadBufferUtils.getBytes());
            if let Some(ref path) = self.mCompressedFile {
                if let Ok(mut output) = std::fs::File::create(path) {
                    let mut buf = vec![0u8; 8192];
                    loop {
                        match raf_stream.read(&mut buf) {
                            Ok(0) => break,
                            Ok(n) => { let _ = output.write_all(&buf[..n]); }
                            Err(_) => break,
                        }
                    }
                }
            }
        }
        // Java: catch (Exception e)
        // Java: IOLog.e("ZipBaseModel", "initDataZip error", e);
        // Java: this.mCompressedFile = null;
        // Java: finally
        // Java: FileUtil.closeQuietly(fileOutputStream);
    }

    // Java: String getDecompressSuffix()
    pub fn getDecompressSuffix(&self) -> &str {
        // Java: return this.mDecompressSuffix;
        &self.mDecompressSuffix
    }

    // Java: void clearUncompress()
    fn clearUncompress(&mut self) {
        // Java: this.mCompressedFile = null;
        self.mCompressedFile = None;
        // Java: this.mZipEntry = null;
        self.mZipEntry = None;
        // Java: this.mDecompressedFile = null;
        self.mDecompressedFile = None;
    }

    // Java: File getUncompressFile(boolean isJudgeExists)
    pub fn getUncompressFile(&mut self, is_judge_exists: bool) -> &Option<PathBuf> {
        // Java: if (this.mCompressedFile != null && isJudgeExists && !this.mCompressedFile.exists())
        if is_judge_exists {
            if let Some(ref f) = self.mCompressedFile {
                if !f.exists()
                {
                    // Java: this.clearUncompress();
                    self.clearUncompress();
                }
            }
        }
        // Java: return this.mCompressedFile;
        &self.mCompressedFile
    }

    // Java: File getUncompressFile()
    pub fn getUncompressFile_default(&mut self) -> &Option<PathBuf> {
        // Java: return this.getUncompressFile(false);
        self.getUncompressFile(false)
    }

    // Java: ZipEntry getZipEntry()
    pub fn getZipEntry(&self) -> &Option<()> {
        // Java: return this.mZipEntry;
        &self.mZipEntry
    }

    // Java: boolean writerUncompressFile(OutputStream outputStream)
    pub fn writerUncompressFile(&mut self, output_stream: &mut dyn Write) -> bool {
        // Java: File uncompressFile = this.getUncompressFile(true);
        // Java: if (uncompressFile == null)
        let uncompress_file = self.getUncompressFile(true).clone();
        if uncompress_file.is_none()
        {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: FileInputStream fileInputStream = null;
            // Java: try
            {
                // Java: fileInputStream = new FileInputStream(uncompressFile);
                // Java: FileUtil.copy(fileInputStream, outputStream, ThreadBufferUtils.getBytes());
                if let Some(ref path) = uncompress_file {
                    if let Ok(mut input) = std::fs::File::open(path) {
                        let mut buf = vec![0u8; 8192];
                        loop {
                            match input.read(&mut buf) {
                                Ok(0) => break,
                                Ok(n) => { let _ = output_stream.write_all(&buf[..n]); }
                                Err(_) => break,
                            }
                        }
                        // Java: boolean e = true;
                        // Java: return e;
                        return true;
                    }
                }
            }
            // Java: catch (Exception e)
            {
                // Java: this.clearUncompress();
                self.clearUncompress();
                // Java: IOLog.e("ZipBaseModel", "writerUnCompressFile fail", e);
            }
            // Java: finally
            // Java: FileUtil.closeQuietly((Closeable)fileInputStream);
            // Java: return false;
            false
        }
    }

    // Java: File getFile(boolean isDecompress)
    pub fn getFile(&mut self, is_decompress: bool) -> &Option<PathBuf> {
        // Java: if (this.mDecompressedFile == null)
        if self.mDecompressedFile.is_none()
        {
            // Java: this.mDecompressedFile = this.createDecompressionFile();
            self.mDecompressedFile = self.createDecompressionFile();
        }
        // Java: if (isDecompress && this.mDecompressedFile != null && this.mDecompressedFile.length() <= 0L && this.mCompressedFile != null && this.mCompressedFile.exists() && this.mCompressedFile.length() > 0L)
        // Java: boolean result = false;
        // Java: try
        // Java: result = this.decompression(new FileInputStream(this.mCompressedFile));
        // Java: catch (FileNotFoundException e)
        // Java: e.printStackTrace();
        // Java: if (!result)
        // Java: this.mCompressedFile = null;
        // Java: return this.mDecompressedFile;
        &self.mDecompressedFile
    }

    // Java: File createDecompressionFile()
    fn createDecompressionFile(&self) -> Option<PathBuf> {
        // Java: return this.mFileManager.createTempFile(this.mDecompressSuffix);
        self.mFileManager.createTempFile(&self.mDecompressSuffix)
    }

    // Java: boolean decompression(InputStream inputStream)
    fn decompression(&mut self, input_stream: &mut dyn Read) -> bool {
        // Java: if (this.mDecompressedFile == null)
        if self.mDecompressedFile.is_none()
        {
            // Java: this.mDecompressedFile = this.createDecompressionFile();
            self.mDecompressedFile = self.createDecompressionFile();
        }
        // Java: if (this.mZipEntry != null && this.mDecompressedFile != null)
        if self.mZipEntry.is_some() && self.mDecompressedFile.is_some()
        {
            // Java: FileOutputStream fileOutputStream = null;
            // Java: ZipFile.ZipInflaterInputStream inflaterInputStream = null;
            // Java: boolean var5;
            // Java: try
            {
                // Java: fileOutputStream = new FileOutputStream(this.mDecompressedFile);
                // Java: inflaterInputStream = this.getInflaterInputStream(inputStream);
                // Java: FileUtil.copy(inflaterInputStream, fileOutputStream, ThreadBufferUtils.getBytes());
                // Java: fileOutputStream.flush();
                // Java: boolean t = true;
                // Java: return t;
                return true;
            }
            // Java: catch (Throwable t)
            {
                // Java: IOLog.e("ZipBaseModel", "解压文件出错 ", t);
                // Java: var5 = false;
            }
            // Java: finally
            // Java: FileUtil.closeQuietly(fileOutputStream);
            // Java: FileUtil.closeQuietly((Closeable)inflaterInputStream);
            // Java: return var5;
        }
        // Java: else
        {
            // Java: return false;
            false
        }
    }

    // Java: ZipFile.ZipInflaterInputStream getInflaterInputStream(InputStream inputStream)
    fn getInflaterInputStream(&self, _input_stream: &mut dyn Read) {
        // Java: int bufSize = Math.max(1024, (int)Math.min(this.mZipEntry.getSize(), 65535L));
        // Java: return new ZipFile.ZipInflaterInputStream(inputStream, new Inflater(true), bufSize, this.mZipEntry);
    }

    // Java: void dispose(boolean isDeleteFile)
    pub fn dispose(&mut self, is_delete_file: bool) {
        // Java: if (!isDeleteFile)
        if !is_delete_file
        {
            // Java: this.mCompressedFile = null;
            self.mCompressedFile = None;
            // Java: this.mDecompressedFile = null;
            self.mDecompressedFile = None;
        }
        // Java: else
        else
        {
            // Java: if (this.mCompressedFile != null)
            if let Some(ref path) = self.mCompressedFile
            {
                // Java: this.mCompressedFile.delete();
                let _ = std::fs::remove_file(path);
                // Java: this.mCompressedFile = null;
            }
            self.mCompressedFile = None;
            // Java: if (this.mDecompressedFile != null)
            if let Some(ref path) = self.mDecompressedFile
            {
                // Java: this.mDecompressedFile.delete();
                let _ = std::fs::remove_file(path);
                // Java: this.mDecompressedFile = null;
            }
            self.mDecompressedFile = None;
        }
    }

    // Java: void beginCompress()
    pub fn beginCompress(&mut self) {
        // Java: if (this.mCompressedFile == null)
        if self.mCompressedFile.is_none()
        {
            // Java: try
            {
                // Java: this.mCompressedFile = this.mFileManager.createTempFile("");
                self.mCompressedFile = self.mFileManager.createTempFile("");
                // Java: this.mCompressedOutputStream = new BufferedOutputStream(new FileOutputStream(this.mCompressedFile));
                if let Some(ref path) = self.mCompressedFile {
                    if let Ok(f) = std::fs::File::create(path) {
                        self.mCompressedOutputStream = Some(BufWriter::new(f));
                    }
                }
            }
            // Java: catch (Exception e)
            {
                // Java: IOLog.e("ZipBaseModel", "beginCompress error", e);
            }
        }
    }

    // Java: void endCompress()
    pub fn endCompress(&mut self) {
        // Java: if (this.mCompressedFile != null && this.mCompressedOutputStream != null)
        if self.mCompressedFile.is_some() && self.mCompressedOutputStream.is_some()
        {
            // Java: FileUtil.closeQuietly((Closeable)this.mCompressedOutputStream);
            self.mCompressedOutputStream = None;
            // Java: if (this.mCompressedFile.length() <= 0L)
            if let Some(ref path) = self.mCompressedFile {
                if path.metadata().map(|m| m.len()).unwrap_or(0) <= 0
                {
                    // Java: this.mCompressedFile.delete();
                    let _ = std::fs::remove_file(path);
                    // Java: this.mCompressedFile = null;
                    self.mCompressedFile = None;
                }
            }
            // Java: this.mCompressedOutputStream = null;
        }
    }

    // Java: void onUpdateEntry(EntryBase entry)
    pub fn onUpdateEntry(&mut self) {
        // Java: if (this.mZipEntry == null)
        if self.mZipEntry.is_none()
        {
            // Java: this.mZipEntry = (ZipEntry)entry.clone();
            // Java: this.mZipEntry.setMode(1);
            // Java: if (this.mZipEntry.getMethod() == 0 && this.mCompressedFile != null && this.mCompressedFile.length() > 0L)
            {
                // Java: this.mZipEntry = null;
                // Java: FileUtil.closeQuietly((Closeable)this.mCompressedOutputStream);
                // Java: this.mCompressedOutputStream = null;
                // Java: this.mCompressedFile.delete();
                // Java: this.mCompressedFile = null;
                // Java: System.out.println("data error onUpdateEntry");
            }
        }
        // Java: this.endCompress();
        self.endCompress();
    }

    // Java: void callBack(_buf: &[u8], int off, int len)
    pub fn callBack(&mut self, buf: &[u8], off: i32, len: i32) {
        // Java: if (this.mCompressedOutputStream == null)
        if self.mCompressedOutputStream.is_none()
        {
            // Java: this.beginCompress();
            self.beginCompress();
        }
        // Java: if (this.mCompressedOutputStream != null)
        if let Some(ref mut out) = self.mCompressedOutputStream
        {
            // Java: try
            {
                // Java: this.mCompressedOutputStream.write(buf, off, len);
                let _ = out.write_all(&buf[off as usize..(off + len) as usize]);
            }
            // Java: catch (IOException e)
            {
                // Java: IOLog.e("ZipBaseModel", "callBack error", e);
            }
        }
    }

    // Java: boolean isValidModel()
    pub fn isValidModel(&self) -> bool {
        // Java: return this.mDecompressedFile != null && this.mDecompressedFile.length() > 0L || this.mCompressedFile != null && this.mCompressedFile.length() > 0L;
        let decomp_valid = self.mDecompressedFile.as_ref().map_or(false, |p| p.metadata().map(|m| m.len() > 0).unwrap_or(false));
        let comp_valid = self.mCompressedFile.as_ref().map_or(false, |p| p.metadata().map(|m| m.len() > 0).unwrap_or(false));
        decomp_valid || comp_valid
    }
}
