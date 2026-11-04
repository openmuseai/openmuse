#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import cn.wps.moffice.kfs.tools.Tools
use crate::kfs::tools::Tools::Tools;
use super::CfsCore::*;

// Java: public class File extends java.io.File
pub struct File {
    path: String,
}

impl Default for File {
    fn default() -> Self { Self { path: String::new() } }
}

// Java: public class File extends java.io.File
impl File {
    // Java: public File(java.io.File sdkFile)
    pub fn new__file(sdkFile: &File) -> File {
        File { path: sdkFile.path.clone() }
    }

    // Java: public File(File dir, String name)
    pub fn new(dir: &File, name: &str) -> File {
        let path = format!("{}/{}", dir.path, name);
        File { path }
    }

    // Java: public File(String path)
    pub fn new__string(path: String) -> File {
        File { path }
    }

    // Java: public File(String dirPath, String name)
    pub fn new__string_string(dirPath: String, name: String) -> File {
        File { path: format!("{}/{}", dirPath, name) }
    }

    pub fn getPath(&self) -> String { self.path.clone() }
    pub fn getAbsolutePath(&self) -> String { self.path.clone() }
    pub fn getName(&self) -> String { self.path.rsplit('/').next().unwrap_or("").to_string() }
    pub fn getParent(&self) -> Option<String> {
        std::path::Path::new(&self.path).parent().map(|p| p.to_string_lossy().to_string())
    }

    // Java: public boolean exists()
    pub fn exists(&self) -> bool {
        // Java: int fd = CfsCore.nativeCfsOpen(Tools.string2bytes(this.getPath()), CfsCore.O_RDONLY);
        let fd = CfsCore::nativeCfsOpen(&Tools::string2bytes(self.path.clone()), O_RDONLY);
        if fd >= 0 {
            CfsCore::nativeCfsClose(fd);
            true
        } else {
            false
        }
    }

    // Java: public boolean delete()
    pub fn delete(&self) -> bool { false }

    // Java: public boolean mkdir()
    pub fn mkdir(&self) -> bool {
        let fd = CfsCore::nativeCfsOpen(&Tools::string2bytes(self.path.clone()), O_CREAT);
        if fd >= 0 { CfsCore::nativeCfsClose(fd); true } else { false }
    }

    // Java: public boolean mkdirs()
    pub fn mkdirs(&self) -> bool { self.mkdir() }

    // Java: public long length()
    pub fn length(&self) -> i64 {
        let fd = CfsCore::nativeCfsOpen(&Tools::string2bytes(self.path.clone()), O_RDONLY);
        if fd >= 0 {
            let len = CfsCore::nativeCfsLength(fd);
            CfsCore::nativeCfsClose(fd);
            len
        } else {
            0
        }
    }

    // Java: public boolean renameTo(java.io.File newPath)
    pub fn renameTo(&self, newPath: &File) -> bool {
        CfsCore::nativeCfsRename(
            &Tools::string2bytes(self.path.clone()),
            &Tools::string2bytes(newPath.path.clone())
        ) == 0
    }

    // Java: public boolean createNewFile()
    pub fn createNewFile(&self) -> bool {
        let fd = CfsCore::nativeCfsOpen(
            &Tools::string2bytes(self.path.clone()),
            O_CREAT | O_WRONLY | O_EXCL
        );
        if fd >= 0 { CfsCore::nativeCfsClose(fd); true } else { false }
    }

    // Java: public static File createTempFile(String prefix, String suffix)
    pub fn createTempFile(prefix: &str, suffix: &str) -> File {
        let suf = if suffix.is_empty() { ".tmp" } else { suffix };
        use std::time::SystemTime;
        let n = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_nanos();
        let name = format!("{}{}{}", prefix, n, suf);
        let tmp = std::env::temp_dir().to_string_lossy().to_string();
        File::new__string_string(tmp, name)
    }

    // Java: public File getParentFile()
    pub fn getParentFile(&self) -> Option<File> {
        self.getParent().map(|p| File::new__string(p))
    }

    // Java: public File getAbsoluteFile()
    pub fn getAbsoluteFile(&self) -> File { File::new__string(self.getAbsolutePath()) }

    // Java: public File getCanonicalFile()
    pub fn getCanonicalFile(&self) -> File { File::new__string(self.path.clone()) }

    // Java: public boolean isDirectory()
    pub fn isDirectory(&self) -> bool { false }

    // Java: public boolean isFile()
    pub fn isFile(&self) -> bool { true }

    // Java: public String[] list()
    pub fn list(&self) -> Vec<String> { Vec::new() }

    // Java: public File[] listFiles()
    pub fn listFiles(&self) -> Vec<File> { Vec::new() }

    // Java: public boolean canRead()
    pub fn canRead(&self) -> bool { true }
    // Java: public boolean canWrite()
    pub fn canWrite(&self) -> bool { true }
    // Java: public boolean canExecute()
    pub fn canExecute(&self) -> bool { true }
    // Java: public long lastModified()
    pub fn lastModified(&self) -> i64 { 0 }
    // Java: public boolean setLastModified(long time)
    pub fn setLastModified(&self, _time: i64) -> bool { false }
    // Java: public boolean setReadOnly()
    pub fn setReadOnly(&self) -> bool { false }

    // Java: public boolean export(String desPath)
    pub fn export(&self, desPath: &str) -> bool {
        // Java: return CfsUtil.export(this, desPath);
        super::CfsUtil::CfsUtil::export(self, desPath)
    }
}
