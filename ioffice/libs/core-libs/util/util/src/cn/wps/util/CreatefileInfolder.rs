// Java: import cn.wps.base.assertion.Assert
use crate::base::assertion::Assert::Assert;
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
use crate::moffice::crash::NoSpaceLeftException::NoSpaceLeftException;
// Java: import cn.wps.moffice.kfs.File
pub struct KfsFile;
// Java: import java.io.IOException
use std::io;

use std::path::MAIN_SEPARATOR;
use std::time::{SystemTime, UNIX_EPOCH};

// Java field: String TAG
pub(crate) const TAG: &str = "CreatefileInfolder";
// Java field: int MAX_SUBFOLDER_COUNT
pub(crate) const MAX_SUBFOLDER_COUNT: i32 = 125;
// Java field: int MAX_FILE_COUNT
pub(crate) const MAX_FILE_COUNT: i32 = 128;

// Java: public class CreatefileInfolder
pub struct CreatefileInfolder {
}

// Java: public class CreatefileInfolder
impl CreatefileInfolder {
    // Java: CreatefileInfolder()
    pub fn new() -> Self {
        // Java: super();
        CreatefileInfolder {}
    }

    // Java: static public static Pair<File, Boolean> createFileInFolder(String root, String name)
    pub fn createFileInFolder(root: &str, name: &str) -> Option<(std::path::PathBuf, bool)> {
        // Java: return createFile(createFolder(root, name), name);
        let folder = Self::createFolder(root, name);
        Self::createFile(&folder, name)
    }

    // Java: static File createTempFileInFolder(String root, String prefix, String suffix)
    pub fn createTempFileInFolder(root: &str, prefix: Option<&str>, suffix: Option<&str>) -> std::path::PathBuf {
        // Java: if (suffix == null)
        let suffix = match suffix {
            // Java: suffix = ".tmp";
            None => ".tmp",
            Some(s) => s,
        };
        // Java: if (prefix == null)
        let prefix = match prefix {
            // Java: prefix = "";
            None => "",
            Some(p) => p,
        };
        // Java: int TEMPFILENUM = 100000;
        let _TEMPFILENUM: i32 = 100000;
        // Java: String name = prefix + (int)System.currentTimeMillis() % 100000 + suffix;
        let millis = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as i32;
        let name = format!("{}{}{}", prefix, millis % 100000, suffix);
        // Java: Pair<File, Boolean> pair = createFile(createFolder(root, name), name);
        let folder = Self::createFolder(root, &name);
        let pair = Self::createFile(&folder, &name);
        // Java: Assert.assertTrue("randomly named temp file should always be NOT existing before created.", (Boolean)pair.second);
        if let Some((_, created)) = &pair {
            assert!(created, "randomly named temp file should always be NOT existing before created.");
        }
        // Java: return (File)pair.first;
        pair.map(|(f, _)| f).unwrap()
    }

    // Java: static String getFileExistPath(String root, String name)
    pub fn getFileExistPath(root: &str, name: &str) -> Option<String> {
        // Java: String folderPath = getSubFolderPath(root, name);
        let folderPath = Self::getSubFolderPath(root, name);
        // Java: String filePath;
        let filePath;
        // Java: if (folderPath.endsWith(File.separator))
        if folderPath.ends_with(MAIN_SEPARATOR) {
            // Java: filePath = folderPath + name;
            filePath = format!("{}{}", folderPath, name);
        // Java: else
        } else {
            // Java: filePath = folderPath + File.separator + name;
            filePath = format!("{}{}{}", folderPath, MAIN_SEPARATOR, name);
        }
        // Java: return (new File(filePath)).exists() ? filePath : null;
        if std::path::Path::new(&filePath).exists() { Some(filePath) } else { None }
    }

    // Java: static public static Pair<File, Boolean> createFile(String folderPath, String name)
    pub fn createFile(folderPath: &str, name: &str) -> Option<(std::path::PathBuf, bool)> {
        // Java: File file = new File(folderPath, name);
        let file = std::path::PathBuf::from(folderPath).join(name);
        // Java: try
        match std::fs::OpenOptions::new().write(true).create_new(true).open(&file) {
            Ok(_) => {
                // Java: boolean created = file.createNewFile();
                let created = true;
                // Java: return new Pair<File, Boolean>(file, created);
                Some((file, created))
            }
            // Java: catch (IOException e)
            Err(e) => {
                // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
                if NoSpaceLeftException::isNoSpaceLeftException(Some(&e)) {
                    // Java: throw new RuntimeException(e);
                    panic!("No space left: {}", e);
                // Java: else
                } else {
                    if file.exists() {
                        return Some((file, false));
                    }
                    // Java: return null;
                    None
                }
            }
        }
    }

    // Java: static String createFolder(String root, String name)
    fn createFolder(root: &str, name: &str) -> String {
        // Java: String folderPath = getSubFolderPath(root, name);
        let folderPath = Self::getSubFolderPath(root, name);
        // Java: File folder = new File(folderPath);
        let folder = std::path::Path::new(&folderPath);
        // Java: if (!folder.exists())
        if !folder.exists() {
            // Java: folder.mkdir();
            let _ = std::fs::create_dir(&folder);
        }
        // Java: return folderPath;
        folderPath
    }

    // Java: static int fileCount(String folderPath)
    fn fileCount(folderPath: &str) -> i32 {
        // Java: File file = new File(folderPath);
        let file = std::path::Path::new(folderPath);
        // Java: return file.exists() ? file.listFiles().length : -1;
        if file.exists() {
            std::fs::read_dir(file).map(|entries| entries.count() as i32).unwrap_or(-1)
        } else {
            -1
        }
    }

    // Java: static boolean isFileExist(String folderPath, String name)
    fn isFileExist(folderPath: &str, name: &str) -> bool {
        // Java: File file = new File(folderPath + File.separator + name);
        let file = std::path::PathBuf::from(folderPath).join(name);
        // Java: return file.exists();
        file.exists()
    }

    // Java: static int hash(String fileName)
    fn hash(fileName: &str) -> i32 {
        // Java: long h = 0L;
        let mut h: i64 = 0;
        // Java: long g = 0L;
        let mut g: i64;
        // Java: int i = 0;
        let mut i: usize = 0;
        // Java: for(int len = fileName.length(); i < len; ++i)
        let len = fileName.len();
        for ch in fileName.chars() {
            // Java: h = (h << 4) + (long)fileName.charAt(i);
            h = (h << 4) + (ch as i64);
            // Java: if ((g = h & -268435456L) != 0L)
            g = h & -268435456i64;
            if g != 0 {
                // Java: h ^= g >> 24;
                h ^= g >> 24;
                // Java: h ^= g;
                h ^= g;
            }
            i += 1;
        }
        // Java: h &= 2147483647L;
        h &= 2147483647i64;
        // Java: i = 16000;
        let _i = 16000;
        // Java: long ret = h % 16000L / 128L;
        let ret = h % 16000i64 / 128i64;
        // Java: Assert.assertTrue("ret should be in 32bit.", (ret & -65536L) == 0L);
        assert!((ret & -65536i64) == 0, "ret should be in 32bit.");
        // Java: return (int)ret;
        ret as i32
    }

    // Java: static String getSubFolderPath(String root, String name)
    fn getSubFolderPath(root: &str, name: &str) -> String {
        // Java: int startSubFolder = hash(name);
        let mut startSubFolder = Self::hash(name);
        let mut root = root.to_string();
        // Java: if (root.endsWith(File.separator))
        let sep = MAIN_SEPARATOR.to_string();
        if root.ends_with(&sep) {
            // Java: root = root.substring(0, root.length() - 2);
            root = root[..root.len() - 2].to_string();
        }
        // Java: int fileExistSubFolder = getFileExistSubFolderNum(root, name, startSubFolder);
        let fileExistSubFolder = Self::getFileExistSubFolderNum(&root, name, startSubFolder);
        // Java: if (fileExistSubFolder > -1)
        if fileExistSubFolder > -1 {
            // Java: return root + File.separator + fileExistSubFolder;
            return format!("{}{}{}", root, MAIN_SEPARATOR, fileExistSubFolder);
        // Java: else
        } else {
            // Java: String subfolderPath;
            let mut subfolderPath;
            // Java: for(subfolderPath = root + File.separator + startSubFolder; fileCount(subfolderPath) >= 128; subfolderPath = root + File.separator + startSubFolder)
            subfolderPath = format!("{}{}{}", root, MAIN_SEPARATOR, startSubFolder);
            while Self::fileCount(&subfolderPath) >= 128 {
                // Java: ++startSubFolder;
                startSubFolder += 1;
                subfolderPath = format!("{}{}{}", root, MAIN_SEPARATOR, startSubFolder);
            }
            // Java: return subfolderPath;
            subfolderPath
        }
    }

    // Java: static int getFileExistSubFolderNum(String root, String fileName, int startSubFolder)
    fn getFileExistSubFolderNum(root: &str, fileName: &str, startSubFolder: i32) -> i32 {
        // Java: File file = new File(root);
        let file = std::path::Path::new(root);
        // Java: File[] subFolders = file.listFiles();
        let subFolders: Vec<std::fs::DirEntry> = match std::fs::read_dir(file) {
            Ok(entries) => entries.filter_map(|e| e.ok()).collect(),
            Err(_) => return -1,
        };
        // Java: int i = 0;
        let mut i: usize = 0;
        // Java: for(int len = subFolders.length; i < len; ++i)
        let len = subFolders.len();
        while i < len {
            // Java: File subFolder = subFolders[i];
            let subFolder = &subFolders[i];
            let name = subFolder.file_name().to_string_lossy().to_string();
            let path = subFolder.path();
            // Java: if (subFolder.isDirectory() && isFigure(subFolder.getName()) && startSubFolder <= Integer.parseInt(subFolder.getName()) && isFileExist(subFolder.getAbsolutePath(), fileName))
            if path.is_dir()
                && Self::isFigure(Some(&name))
                && startSubFolder <= name.parse::<i32>().unwrap_or(-1)
                && Self::isFileExist(&path.to_string_lossy(), fileName)
            {
                // Java: return Integer.parseInt(subFolder.getName());
                return name.parse::<i32>().unwrap_or(-1);
            }
            i += 1;
        }
        // Java: return -1;
        -1
    }

    // Java: static boolean isFigure(String checkStr)
    fn isFigure(checkStr: Option<&str>) -> bool {
        // Java: if (checkStr == null)
        if checkStr.is_none() {
            // Java: return false;
            return false;
        // Java: else
        } else {
            let checkStr = checkStr.unwrap();
            // Java: int i = 0;
            // Java: for(int len = checkStr.length(); i < len; ++i)
            for ch in checkStr.chars() {
                // Java: char c = checkStr.charAt(i);
                let c = ch;
                // Java: if (c < '0' || c > '9')
                if c < '0' || c > '9' {
                    // Java: return false;
                    return false;
                }
            }
            // Java: return true;
            true
        }
    }
}
