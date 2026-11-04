#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import cn.wps.util.zip.model.ZipBaseModel
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import java.util.HashMap
use std::path::PathBuf;
use std::collections::HashMap;
use std::io::{Read, Write, Seek, SeekFrom};

use crate::util::zip::model::ZipBaseModel::ZipBaseModel;

// Java: public class FileDataStorage
pub struct FileDataStorage {
    // Java field: File mFile
    pub mFile: Option<PathBuf>,
    // Java field: HashMap<String, Object> mUserData
    pub mUserData: HashMap<String, Box<dyn std::any::Any + Send>>,
    // Java field: boolean mDisposed
    mDisposed: bool,
    // Java field: ZipBaseModel mZipBaseModel
    pub mZipBaseModel: Option<ZipBaseModel>,
}

// Java: public class FileDataStorage
impl FileDataStorage {
    // Java: FileDataStorage(File file)
    pub fn new(file: PathBuf) -> Self {
        // Java: super();
        // Java: this.mFile = file;
        Self {
            mFile: Some(file),
            mUserData: HashMap::new(),
            mDisposed: false,
            mZipBaseModel: None,
        }
    }

    // Java: FileDataStorage(_data: &[u8])
    pub fn new_from_data(data: &[u8]) -> Self {
        // Java: super();
        let mut s = Self {
            mFile: None,
            mUserData: HashMap::new(),
            mDisposed: false,
            mZipBaseModel: None,
        };
        // Java: this.setData(data);
        s.setData(data);
        s
    }

    // Java: FileDataStorage(ZipBaseModel zipBaseModel)
    pub fn new_from_zip(zip_base_model: ZipBaseModel) -> Self {
        // Java: super();
        // Java: this.mZipBaseModel = zipBaseModel;
        // Java: this.mFile = new File("");
        Self {
            mFile: Some(PathBuf::from("")),
            mUserData: HashMap::new(),
            mDisposed: false,
            mZipBaseModel: Some(zip_base_model),
        }
    }

    // Java: ZipBaseModel getZipBaseModel()
    pub fn getZipBaseModel(&self) -> &Option<ZipBaseModel> {
        // Java: return this.mZipBaseModel;
        &self.mZipBaseModel
    }

    // Java: void setZipBaseModel(ZipBaseModel zipBaseModel)
    pub fn setZipBaseModel(&mut self, zip_base_model: ZipBaseModel) {
        // Java: this.mZipBaseModel = zipBaseModel;
        self.mZipBaseModel = Some(zip_base_model);
    }

    // Java: void dispose(boolean isDeleteFile)
    fn dispose(&mut self, is_delete_file: bool) {
        // Java: if (!this.mDisposed)
        if !self.mDisposed
        {
            // Java: this.mDisposed = true;
            self.mDisposed = true;
            // Java: if (this.mFile != null && this.mFile.exists())
            if let Some(ref path) = self.mFile {
                if path.exists()
                {
                    // Java: if (isDeleteFile)
                    if is_delete_file
                    {
                        // Java: this.mFile.delete();
                        let _ = std::fs::remove_file(path);
                    }
                    // Java: this.mFile = null;
                }
            }
            self.mFile = None;
            // Java: this.clearZipModel();
            self.clearZipModel();
            // Java: this.disposeUserData();
            self.disposeUserData();
        }
    }

    // Java: void disposeUserData()
    pub fn disposeUserData(&mut self) {
        // Java: if (null != this.mUserData)
        {
            // Java: for(String key : this.mUserData.keySet())
            // Java: Object o = this.mUserData.get(key);
            // Java: if (o instanceof FileDataStorage)
            // Java: boolean isDeleteFile = true;
            // Java: if (key.equals("cn.wps.poi.drawing.KPictureRegistry.KEY_CORE_PICTURE"))
            // Java: isDeleteFile = false;
            // Java: ((FileDataStorage)o).dispose(isDeleteFile);
            // Java: this.mUserData.clear();
            self.mUserData.clear();
        }
    }

    // Java: boolean isValid()
    fn isValid(&self) -> bool {
        // Java: return this.mFile != null;
        self.mFile.is_some()
    }

    // Java: String getName()
    pub fn getName(&mut self) -> String {
        // Java: return this.mFile != null && this.mFile.exists() && this.mFile.length() > 0L ? this.mFile.getName() : this.getFile(true).getName();
        if let Some(ref path) = self.mFile {
            if path.exists() && path.metadata().map(|m| m.len() > 0).unwrap_or(false) {
                return path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            }
        }
        let file = self.getFile(true);
        file.as_ref().map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default()).unwrap_or_default()
    }

    // Java: String getSuffix()
    pub fn getSuffix(&self) -> Option<String> {
        // Java: if (this.mFile != null && this.mFile.length() > 0L)
        if let Some(ref path) = self.mFile {
            if path.metadata().map(|m| m.len() > 0).unwrap_or(false)
            {
                // Java: String name = this.mFile.getName();
                let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                // Java: int index = name.lastIndexOf(".");
                let index = name.rfind('.');
                // Java: if (index > 0 && index < name.length() - 1)
                if let Some(idx) = index {
                    if idx > 0 && idx < name.len() - 1
                    {
                        // Java: return this.mFile.getName().substring(index + 1, name.length());
                        return Some(name[idx + 1..].to_string());
                    }
                }
            }
        }
        // Java: return this.mZipBaseModel != null ? this.mZipBaseModel.getDecompressSuffix() : null;
        self.mZipBaseModel.as_ref().map(|z| z.getDecompressSuffix().to_string())
    }

    // Java: File getFile(boolean isDecompress)
    pub fn getFile(&mut self, is_decompress: bool) -> &Option<PathBuf> {
        // Java: if (this.mFile != null && this.mZipBaseModel != null && (!this.mFile.exists() || this.mFile.length() <= 0L))
        if self.mFile.is_some() && self.mZipBaseModel.is_some() {
            let need_decompress = self.mFile.as_ref().map(|p| !p.exists() || p.metadata().map(|m| m.len() <= 0).unwrap_or(true)).unwrap_or(false);
            if need_decompress
            {
                // Java: File file = this.mZipBaseModel.getFile(isDecompress);
                let file = self.mZipBaseModel.as_mut().unwrap().getFile(is_decompress).clone();
                // Java: if (isDecompress)
                if is_decompress
                {
                    // Java: this.mFile = file;
                    self.mFile = file;
                }
                // Java: return file;
                return &self.mFile;
            }
        }
        // Java: else
        {
            // Java: return this.mFile;
            &self.mFile
        }
    }

    // Java: File getFile()
    pub fn getFile_default(&mut self) -> &Option<PathBuf> {
        // Java: return this.getFile(true);
        self.getFile(true)
    }

    // Java: void setUserData(String key, Object value)
    pub fn setUserData(&mut self, key: String, value: Box<dyn std::any::Any + Send>) {
        // Java: this.mUserData.put(key, value);
        self.mUserData.insert(key, value);
    }

    // Java: Object getUserData(String key)
    pub fn getUserData_by_key(&self, key: &str) -> Option<&Box<dyn std::any::Any + Send>> {
        // Java: return this.mUserData.get(key);
        self.mUserData.get(key)
    }

    // Java: int getSize()
    pub fn getSize(&mut self) -> i32 {
        // Java: return !this.isValid() ? 0 : (int)this.getFile().length();
        if !self.isValid() {
            return 0;
        }
        let file = self.getFile_default().clone();
        file.as_ref().map(|p| p.metadata().map(|m| m.len() as i32).unwrap_or(0)).unwrap_or(0)
    }

    // Java: _getData: &[u8]()
    pub fn getData(&mut self) -> Option<Vec<u8>> {
        // Java: if (!this.isValid())
        if !self.isValid()
        {
            // Java: return null;
            return None;
        }
        // Java: else
        {
            // Java: try
            {
                // Java: _data: &[u8] = new byte[this.getSize()];
                let size = self.getSize() as usize;
                let mut data = vec![0u8; size];
                // Java: FileInputStream input = new FileInputStream(this.getFile());
                let file = self.getFile_default().clone();
                if let Some(ref path) = file {
                    if let Ok(mut input) = std::fs::File::open(path) {
                        // Java: input.read(data);
                        let _ = input.read(&mut data);
                        // Java: input.close();
                        // Java: return data;
                        return Some(data);
                    }
                }
            }
            // Java: catch (IOException var3)
            {
                // Java: return null;
                None
            }
        }
    }

    // Java: int getData(int start, int length, _buf: &[u8], int index)
    pub fn getData_range(&mut self, start: i32, length: i32, buf: &mut [u8], index: i32) -> i32 {
        // Java: if (!this.isValid())
        if !self.isValid()
        {
            // Java: return 0;
            return 0;
        }
        // Java: else
        {
            // Java: try
            {
                // Java: RandomAccessFile reader = new RandomAccessFile(this.getFile(), "r");
                let file = self.getFile_default().clone();
                if let Some(ref path) = file {
                    if let Ok(mut reader) = std::fs::File::open(path) {
                        // Java: reader.seek((long)start);
                        let _ = reader.seek(SeekFrom::Start(start as u64));
                        // Java: int n = reader.read(buf, index, length);
                        let n = reader.read(&mut buf[index as usize..(index + length) as usize]).unwrap_or(0);
                        // Java: reader.close();
                        // Java: return n;
                        return n as i32;
                    }
                }
            }
            // Java: catch (IOException var7)
            {
                // Java: throw new RuntimeException("Can not read from file: " + this.getFile());
                panic!("Can not read from file");
            }
        }
    }

    // Java: void setData(_data: &[u8])
    pub fn setData(&mut self, data: &[u8]) {
        // Java: if (this.mFile != null && this.mFile.exists())
        if let Some(ref path) = self.mFile {
            if path.exists()
            {
                // Java: this.mFile.delete();
                let _ = std::fs::remove_file(path);
            }
        }
        // Java: this.clearZipModel();
        self.clearZipModel();
        // Java: try
        {
            // Java: this.mFile = PlatformAdapter.createTempFile("fds-", ".tmp");
            let temp_path = std::env::temp_dir().join(format!("fds-{}.tmp", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()));
            // Java: FileOutputStream output = FileUtil.getFileOutput(this.mFile);
            // Java: output.write(data);
            // Java: FileUtil.closeQuietly(output);
            if let Ok(mut output) = std::fs::File::create(&temp_path) {
                let _ = output.write_all(data);
            }
            self.mFile = Some(temp_path);
        }
        // Java: catch (IOException var3)
        // Java: throw new RuntimeException("Can not write file to temporary directory!!!");
    }

    // Java: void setData(int index, _src: &[u8], int srcindex, int length)
    pub fn setData_range(&mut self, index: i32, src: &[u8], srcindex: i32, length: i32) {
        // Java: if (this.isValid())
        if self.isValid()
        {
            // Java: try
            {
                // Java: RandomAccessFile reader = new RandomAccessFile(this.getFile(), "rw");
                let file = self.getFile_default().clone();
                if let Some(ref path) = file {
                    if let Ok(mut reader) = std::fs::OpenOptions::new().read(true).write(true).open(path) {
                        // Java: reader.seek((long)index);
                        let _ = reader.seek(SeekFrom::Start(index as u64));
                        // Java: reader.write(src, srcindex, length);
                        let _ = reader.write_all(&src[srcindex as usize..(srcindex + length) as usize]);
                        // Java: reader.close();
                    }
                }
            }
            // Java: catch (IOException var6)
            {
                // Java: throw new RuntimeException("Can not write to file: " + this.getFile());
            }
        }
    }

    // Java: static FileDataStorage createFileDataStorage(_data: &[u8], File file)
    pub fn createFileDataStorage_with_file(data: &[u8], file: PathBuf) -> FileDataStorage {
        // Java: if (!file.exists())
        if !file.exists()
        {
            // Java: file.createNewFile();
            let _ = std::fs::File::create(&file);
        }
        // Java: FileOutputStream writer = FileUtil.getFileOutput(file);
        // Java: writer.write(data);
        // Java: FileUtil.closeQuietly(writer);
        if let Ok(mut writer) = std::fs::File::create(&file) {
            let _ = writer.write_all(data);
        }
        // Java: return new FileDataStorage(file);
        FileDataStorage::new(file)
    }

    // Java: static FileDataStorage createFileDataStorage(_data: &[u8], String path)
    pub fn createFileDataStorage_with_path(data: &[u8], path: &str) -> FileDataStorage {
        // Java: return createFileDataStorage(data, new File(path));
        Self::createFileDataStorage_with_file(data, PathBuf::from(path))
    }

    // Java: static FileDataStorage createFileDataStorage(_data: &[u8])
    pub fn createFileDataStorage_from_data(data: &[u8]) -> FileDataStorage {
        // Java: File file = PlatformAdapter.createTempFile("fds-", ".tmp");
        let file = std::env::temp_dir().join(format!("fds-{}.tmp", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()));
        // Java: return createFileDataStorage(data, file);
        Self::createFileDataStorage_with_file(data, file)
    }

    // Java: static FileDataStorage createFileDataStorage(InputStream input, int length, File file)
    pub fn createFileDataStorage_from_stream_with_file(input: &mut dyn Read, length: i32, file: PathBuf) -> FileDataStorage {
        // Java: if (!file.exists())
        if !file.exists()
        {
            // Java: file.createNewFile();
            let _ = std::fs::File::create(&file);
        }
        // Java: FileOutputStream writer = FileUtil.getFileOutput(file);
        // Java: StreamUtil.copyStream(input, length, writer);
        // Java: FileUtil.closeQuietly(writer);
        if let Ok(mut writer) = std::fs::File::create(&file) {
            let mut buf = vec![0u8; 8192];
            let mut remaining = length as usize;
            while remaining > 0 {
                let to_read = std::cmp::min(remaining, buf.len());
                match input.read(&mut buf[..to_read]) {
                    Ok(0) => break,
                    Ok(n) => {
                        let _ = writer.write_all(&buf[..n]);
                        remaining -= n;
                    }
                    Err(_) => break,
                }
            }
        }
        // Java: return new FileDataStorage(file);
        FileDataStorage::new(file)
    }

    // Java: static FileDataStorage createFileDataStorage(InputStream input, int length, String path)
    pub fn createFileDataStorage_from_stream_with_path(input: &mut dyn Read, length: i32, path: &str) -> FileDataStorage {
        // Java: return createFileDataStorage(input, length, new File(path));
        Self::createFileDataStorage_from_stream_with_file(input, length, PathBuf::from(path))
    }

    // Java: static FileDataStorage createFileDataStorage(InputStream input, int length)
    pub fn createFileDataStorage_from_stream(input: &mut dyn Read, length: i32) -> FileDataStorage {
        // Java: File file = PlatformAdapter.createTempFile("fds-", ".tmp");
        let file = std::env::temp_dir().join(format!("fds-{}.tmp", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()));
        // Java: return createFileDataStorage(input, length, file);
        Self::createFileDataStorage_from_stream_with_file(input, length, file)
    }

    // Java: InputStream getReader()
    pub fn getReader(&mut self) -> Option<std::fs::File> {
        // Java: return new FileInputStream(this.getFile());
        let file = self.getFile_default().clone();
        file.as_ref().and_then(|p| std::fs::File::open(p).ok())
    }

    // Java: OutputStream getWriter()
    pub fn getWriter(&mut self) -> Option<std::fs::File> {
        // Java: return new FileOutputStream(this.getFile());
        let file = self.getFile_default().clone();
        file.as_ref().and_then(|p| std::fs::File::create(p).ok())
    }

    // Java: void deleteFile()
    pub fn deleteFile(&mut self) {
        // Java: if (null != this.mFile && this.mFile.exists())
        if let Some(ref path) = self.mFile {
            if path.exists()
            {
                // Java: this.mFile.delete();
                let _ = std::fs::remove_file(path);
            }
        }
        // Java: this.clearZipModel();
        self.clearZipModel();
    }

    // Java: void resetFile(File file)
    pub fn resetFile(&mut self, file: PathBuf) {
        // Java: this.mFile = file;
        self.mFile = Some(file);
        // Java: this.clearZipModel();
        self.clearZipModel();
    }

    // Java: void clearZipModel()
    fn clearZipModel(&mut self) {
        // Java: if (this.mZipBaseModel != null)
        if let Some(ref mut z) = self.mZipBaseModel
        {
            // Java: this.mZipBaseModel.dispose(true);
            z.dispose(true);
            // Java: this.mZipBaseModel = null;
        }
        self.mZipBaseModel = None;
    }

    // Java: public HashMap<String, Object> getUserData()
    pub fn getUserData(&self) -> &HashMap<String, Box<dyn std::any::Any + Send>> {
        // Java: return this.mUserData;
        &self.mUserData
    }

    // Java: void setUserData(HashMap<String, Object> userData)
    pub fn setUserData_map(&mut self, user_data: HashMap<String, Box<dyn std::any::Any + Send>>) {
        // Java: if (null != userData)
        {
            // Java: this.mUserData = userData;
            self.mUserData = user_data;
        }
    }
}
