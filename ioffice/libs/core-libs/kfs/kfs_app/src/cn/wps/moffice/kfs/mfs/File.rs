#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]

// Java: import cn.wps.moffice.kfs.mfs.core.iface.IMfsDisk
use crate::kfs::mfs::core::iface::IMfsDisk::*;
// Java: import cn.wps.moffice.kfs.mfs.core.iface.IMfsDiskManager
use crate::kfs::mfs::core::iface::IMfsDiskManager::*;
// Java: import cn.wps.moffice.kfs.mfs.core.iface.IMfsFile
use crate::kfs::mfs::core::iface::IMfsFile::*;

// Java: public class File extends java.io.File
pub struct File {
    // Java field:    private IMfsDisk mMfsDisk;
    mMfsDisk: Option<Box<dyn IMfsDisk>>,
    path: String,
}

impl Default for File {
    fn default() -> Self {
        Self { mMfsDisk: None, path: String::new() }
    }
}

// Java: public class File extends java.io.File
impl File {
    // Java: public File(File dir, String name)
    pub fn new(dir: &File, name: &str) -> File {
        // Java: this(dir == null ? null : dir.getPath(), name);
        let path = format!("{}/{}", dir.path, name);
        Self { mMfsDisk: None, path }
    }

    // Java: public File(String path)
    pub fn new__string(path: String) -> File {
        Self { mMfsDisk: None, path }
    }

    // Java: public File(java.io.File file)
    pub fn new__file(file: &File) -> File {
        // Java: this(file.getAbsolutePath());
        Self { mMfsDisk: None, path: file.getAbsolutePath() }
    }

    // Java: public File(java.io.File file, String name)
    pub fn new__file_string(file: &File, name: &str) -> File {
        // Java: this(file.getAbsolutePath(), name);
        let path = format!("{}/{}", file.getAbsolutePath(), name);
        Self { mMfsDisk: None, path }
    }

    // Java: public File(String dirPath, String name)
    pub fn new__string_string(dirPath: String, name: String) -> File {
        let path = format!("{}/{}", dirPath, name);
        Self { mMfsDisk: None, path }
    }

    pub fn getPath(&self) -> String { self.path.clone() }
    pub fn getAbsolutePath(&self) -> String { self.path.clone() }
    pub fn getCanonicalPath(&self) -> String { self.path.clone() }
    pub fn getName(&self) -> String {
        self.path.rsplit('/').next().unwrap_or("").to_string()
    }
    pub fn getParent(&self) -> Option<String> {
        let p = std::path::Path::new(&self.path);
        p.parent().map(|pp| pp.to_string_lossy().to_string())
    }

    // Java: public boolean canExecute()
    pub fn canExecute(&self) -> bool { true }

    // Java: public boolean canRead()
    pub fn canRead(&self) -> bool { true }

    // Java: public boolean canWrite()
    pub fn canWrite(&self) -> bool { true }

    // Java: public boolean delete()
    pub fn delete(&mut self) -> bool {
        // Java: if (super.exists())
        if self.exists0() {
            // Java: return super.delete();
            std::fs::remove_file(&self.path).is_ok()
        }
        // Java: else
        else {
            // Java: IMfsFile file = this.getMfsFile();
            // Java: return null != file ? file.delete() : false;
            if let Some(mut file) = self.getMfsFile() {
                file.delete()
            } else {
                false
            }
        }
    }

    // Java: public boolean exists()
    pub fn exists(&mut self) -> bool {
        // Java: return super.exists() || this.getMfsFile() != null;
        self.exists0() || self.getMfsFile().is_some()
    }

    // Java: public boolean exists0()
    pub fn exists0(&self) -> bool {
        // Java: return super.exists();
        std::path::Path::new(&self.path).exists()
    }

    // Java: public File getAbsoluteFile()
    pub fn getAbsoluteFile(&self) -> File {
        // Java: return new File(this.getAbsolutePath());
        File::new__string(self.getAbsolutePath())
    }

    // Java: public File getCanonicalFile()
    pub fn getCanonicalFile(&self) -> File {
        // Java: return new File(this.getCanonicalPath());
        File::new__string(self.getCanonicalPath())
    }

    // Java: public File getParentFile()
    pub fn getParentFile(&self) -> Option<File> {
        // Java: String tempParent = this.getParent();
        // Java: return tempParent == null ? null : new File(tempParent);
        self.getParent().map(|p| File::new__string(p))
    }

    // Java: public boolean isDirectory()
    pub fn isDirectory(&mut self) -> bool {
        // Java: if (super.exists())
        if self.exists0() {
            std::path::Path::new(&self.path).is_dir()
        }
        // Java: else
        else {
            // Java: IMfsFile file = this.getMfsFile();
            if let Some(mut file) = self.getMfsFile() {
                // Java: return !file.isFile();
                !file.isFile()
            } else {
                false
            }
        }
    }

    // Java: public boolean isFile()
    pub fn isFile(&mut self) -> bool {
        // Java: if (super.exists())
        if self.exists0() {
            std::path::Path::new(&self.path).is_file()
        }
        // Java: else
        else {
            // Java: IMfsFile file = this.getMfsFile();
            // Java: return null != file ? file.isFile() : false;
            if let Some(mut file) = self.getMfsFile() {
                file.isFile()
            } else {
                false
            }
        }
    }

    // Java: public long lastModified()
    pub fn lastModified(&mut self) -> i64 {
        // Java: if (super.exists())
        if self.exists0() {
            std::fs::metadata(&self.path)
                .and_then(|m| m.modified())
                .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as i64)
                .unwrap_or(0)
        }
        // Java: else
        else {
            // Java: IMfsFile file = this.getMfsFile();
            // Java: return null != file ? file.lastModified() : 0L;
            if let Some(mut file) = self.getMfsFile() {
                file.lastModified()
            } else {
                0
            }
        }
    }

    // Java: public boolean setLastModified(long time)
    pub fn setLastModified(&mut self, time: i64) -> bool {
        // Java: if (time < 0L)
        if time < 0 {
            // Java: throw new IllegalArgumentException("illegal time");
            panic!("IllegalArgumentException: illegal time");
        }
        // Java: else if (super.exists())
        if self.exists0() {
            false // platform-specific, simplified
        }
        // Java: else
        else {
            // Java: IMfsFile file = this.getMfsFile();
            if let Some(mut file) = self.getMfsFile() {
                file.setLastModified(time);
            }
            false
        }
    }

    // Java: public boolean setReadOnly()
    pub fn setReadOnly(&mut self) -> bool {
        // Java: return this.setWritable(false, false);
        self.setWritable(false, false)
    }

    // Java: public boolean setExecutable(boolean executable, boolean ownerOnly)
    pub fn setExecutable(&self, _executable: bool, _ownerOnly: bool) -> bool {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: public boolean setExecutable(boolean executable)
    pub fn setExecutable__boolean(&self, executable: bool) -> bool {
        self.setExecutable(executable, true)
    }

    // Java: public boolean setReadable(boolean readable, boolean ownerOnly)
    pub fn setReadable(&self, _readable: bool, _ownerOnly: bool) -> bool {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: public boolean setReadable(boolean readable)
    pub fn setReadable__boolean(&self, readable: bool) -> bool {
        self.setReadable(readable, true)
    }

    // Java: public boolean setWritable(boolean writable, boolean ownerOnly)
    pub fn setWritable(&self, _writable: bool, _ownerOnly: bool) -> bool {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: public boolean setWritable(boolean writable)
    pub fn setWritable__boolean(&self, writable: bool) -> bool {
        self.setWritable(writable, true)
    }

    // Java: public long length()
    pub fn length(&mut self) -> i64 {
        // Java: if (super.exists())
        if self.exists0() {
            std::fs::metadata(&self.path).map(|m| m.len() as i64).unwrap_or(0)
        }
        // Java: else
        else {
            // Java: IMfsFile file = this.getMfsFile();
            // Java: return null != file ? file.length() : 0L;
            if let Some(mut file) = self.getMfsFile() {
                file.length()
            } else {
                0
            }
        }
    }

    // Java: public String[] list()
    pub fn list(&mut self) -> Vec<String> {
        // Java: if (super.exists())
        if self.exists0() {
            if let Ok(entries) = std::fs::read_dir(&self.path) {
                entries.filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().to_string())).collect()
            } else {
                Vec::new()
            }
        }
        // Java: else
        else {
            // Java: IMfsFile file = this.getMfsFile();
            // Java: return null != file ? file.list() : null;
            if let Some(mut file) = self.getMfsFile() {
                file.list()
            } else {
                Vec::new()
            }
        }
    }

    // Java: public File[] listFiles(FilenameFilter filter)
    pub fn listFiles(&mut self) -> Vec<File> {
        // Java: return this.filenamesToFiles(this.list());
        let names = self.list();
        self.filenamesToFiles(names)
    }

    // Java: private File[] filenamesToFiles(String[] filenames)
    fn filenamesToFiles(&self, filenames: Vec<String>) -> Vec<File> {
        // Java: int count = filenames.length;
        // Java: File[] result = new File[count];
        let mut result = Vec::with_capacity(filenames.len());
        // Java: for(int i = 0; i < count; ++i)
        for name in &filenames {
            // Java: result[i] = new File(this.getPath(), filenames[i]);
            result.push(File::new__string_string(self.path.clone(), name.clone()));
        }
        // Java: return result;
        result
    }

    // Java: public boolean mkdir()
    pub fn mkdir(&mut self) -> bool {
        // Java: try { this.mkdirErrno(); return true; } catch (...) { return false; }
        self.mkdirErrno().is_ok()
    }

    // Java: private void mkdirErrno() throws MfsException
    fn mkdirErrno(&mut self) -> Result<(), String> {
        // Java: IMfsDisk disk = this.getDisk();
        // Java: if (this.getMfsFile() == null && null != disk)
        if self.getMfsFile().is_none() {
            if let Some(disk) = &mut self.mMfsDisk {
                // Java: disk.newMfsFile(this.getPath(), true, true);
                disk.newMfsFile(self.path.clone(), true, true);
            }
        }
        Ok(())
    }

    // Java: public boolean mkdirs()
    pub fn mkdirs(&mut self) -> bool {
        // Java: return this.mkdirs(false);
        self.mkdir()
    }

    // Java: public boolean createNewFile() throws IOException
    pub fn createNewFile(&mut self) -> bool {
        // Java: if (null == this.getDisk()) return false;
        if self.mMfsDisk.is_none() {
            return false;
        }
        if let Some(disk) = &mut self.mMfsDisk {
            // Java: return this.getDisk().newMfsFile(this.getPath(), false, true) != null;
            let _f = disk.newMfsFile(self.path.clone(), false, true);
            return true; // simplified
        }
        false
    }

    // Java: public static File createTempFile(String prefix, String suffix) throws IOException
    pub fn createTempFile(prefix: &str, suffix: &str) -> File {
        // Java: return createTempFile(prefix, suffix, (File)null);
        Self::createTempFile__with_dir(prefix, suffix, None)
    }

    // Java: public static File createTempFile(String prefix, String suffix, File directory) throws IOException
    pub fn createTempFile__with_dir(prefix: &str, suffix: &str, _directory: Option<&File>) -> File {
        // Java: if (prefix.length() < 3)
        if prefix.len() < 3 {
            panic!("IllegalArgumentException: prefix must be at least 3 characters");
        }
        let suf = if suffix.is_empty() { ".tmp" } else { suffix };
        let random_num: u64 = rand_simple();
        let name = format!("{}{}{}", prefix, random_num, suf);
        let tmp_dir = std::env::temp_dir().to_string_lossy().to_string();
        File::new__string_string(tmp_dir, name)
    }

    // Java: public boolean renameTo(java.io.File newPath)
    pub fn renameTo(&mut self, newPath: &File) -> bool {
        // Java: if (super.exists())
        if self.exists0() {
            std::fs::rename(&self.path, &newPath.path).is_ok()
        }
        // Java: else if (null == this.getDisk())
        else if self.mMfsDisk.is_none() {
            false
        }
        // Java: else
        else {
            if let Some(disk) = &mut self.mMfsDisk {
                // Java: boolean result = this.getDisk().renameFile(this.getPath(), newPath.getAbsolutePath());
                let result = disk.renameFile(self.path.clone(), newPath.path.clone());
                if result {
                    // Java: this.resetDisk();
                    self.resetDisk();
                }
                result
            } else {
                false
            }
        }
    }

    // Java: IMfsDisk getDisk()
    pub fn getDisk(&mut self) -> Option<&mut Box<dyn IMfsDisk>> {
        // Java: if (null != this.mMfsDisk) return this.mMfsDisk;
        if self.mMfsDisk.is_some() {
            return self.mMfsDisk.as_mut();
        }
        // Java: IMfsDiskManager mdm = MfsReflectHelper.getDiskManager(this);
        // Reflection-based disk manager lookup - simplified for Rust
        None
    }

    // Java: private void resetDisk()
    fn resetDisk(&mut self) {
        // Java: this.mMfsDisk = null;
        self.mMfsDisk = None;
    }

    // Java: IMfsFile getMfsFile()
    pub fn getMfsFile(&mut self) -> Option<Box<dyn IMfsFile>> {
        // Java: return null != this.getDisk() ? this.getDisk().getMfsFile(this.getPath()) : null;
        if let Some(disk) = &mut self.mMfsDisk {
            Some(disk.getMfsFile(self.path.clone()))
        } else {
            None
        }
    }

    // Java: public boolean export(String desPath)
    pub fn export(&mut self, desPath: String) -> bool {
        // Java: IMfsFile file = this.getMfsFile();
        // Java: return null == file ? false : file.export(desPath);
        if let Some(mut file) = self.getMfsFile() {
            file.export(desPath)
        } else {
            false
        }
    }
}

fn rand_simple() -> u64 {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
}
