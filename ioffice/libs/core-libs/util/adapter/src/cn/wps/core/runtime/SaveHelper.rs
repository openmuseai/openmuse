// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import cn.wps.moffice.util.FileUtil
// Java: import java.io.FileFilter
// Java: import java.io.IOException
// Java: import java.io.UnsupportedEncodingException
// Java: import java.security.MessageDigest
// Java: import java.security.NoSuchAlgorithmException
// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.Comparator
// Java: import java.util.List

// Java field: int MAX_BACKUPS
pub(crate) const MAX_BACKUPS: i32 = 1;
// Java field: String sFileNameTooLong
pub(crate) const S_FILE_NAME_TOO_LONG: &str = "File name too long";

// Java: public class SaveHelper
pub struct SaveHelper {
    // Java field: AppType type
    pub(crate) r#type: AppType,
    // Java field: File destFile
    destFile: Box<dyn std::any::Any>,
    // Java field: File tmpFile
    pub(crate) tmpFile: Option<Box<dyn std::any::Any>>,
    // Java field: List<File> tmpFiles
    tmpFiles: Vec<Box<dyn std::any::Any>>,
}

// Java: public enum AppType
pub struct AppType {
    // Java field: String dirName
    pub(crate) dirName: String,
}

// Java: public class BackupFileFilter
pub struct BackupFileFilter {
    // Java field: String srcFileName
    pub(crate) srcFileName: String,
}

// Java: public class BackupFileComparator
pub struct BackupFileComparator {
    // Java field: String srcFileName
    pub(crate) srcFileName: String,
}

// Java: public class SaveHelper
impl SaveHelper {
    // Java: SaveHelper(AppType type, File destFile)
    pub fn new(_type: AppType, _destFile: Box<dyn std::any::Any>) -> Self {
      // Java: super();
      // Java: this.type = type;
      // Java: this.destFile = destFile;
      SaveHelper {
          r#type: _type,
          destFile: _destFile,
          tmpFile: None,
          tmpFiles: Vec::new(),
      }
    }

    // Java: File startSaveFile()
    pub fn startSaveFile(&mut self) -> Box<dyn std::any::Any> {
      // Java: String appTemp = Platform.getTempDirectory(this.destFile.getPath());
      // Java: File tmpDir = new File(appTemp, this.type.dirName);
      // Java: if (!tmpDir.exists() && !tmpDir.mkdirs())
      {
         // Java: throw new IOException("mkdir failed");
      }
      // Java: else
      {
         // Java: try
         {
            // Java: this.tmpFile = File.createTempFile(this.destFile.getName(), ".tmp", tmpDir);
         }
         // Java: catch (IOException e)
         {
            // Java: String message = e.getMessage();
            // Java: if (null == message || message.indexOf("File name too long") == -1)
            {
               // Java: throw e;
            }
            // Java: this.tmpFile = File.createTempFile("tmp", ".tmp", tmpDir);
         }
         // Java: return this.tmpFile;
      }
      todo!()
    }

    // Java: static void backupFile(AppType type, File destFile)
    pub fn backupFile(_type: &AppType, _destFile: &dyn std::any::Any) {
      // Java: if (destFile.exists())
      {
         // Java: File[] backups = try2Backup(type, destFile);
         // Java: if (null != backups)
         {
            // Java: for(int i = 0; i < backups.length - 1 + 1; ++i)
            {
               // Java: backups[i].delete();
               // Java: backups[i] = null;
            }
         }
      }
    }

    // Java: static File[] try2Backup(AppType type, File destFile)
    pub fn try2Backup(_type: &AppType, _destFile: &dyn std::any::Any) -> Option<Vec<Box<dyn std::any::Any>>> {
      // Java: File[] ret = null;
      // Java: for(char ch = 'A'; ch < 'J'; ++ch)
      {
         // Java: try
         {
            // Java: ret = doRealBackup(type, destFile, ch);
            // Java: break;
         }
         // Java: catch (Exception ignored)
         {
         }
      }
      // Java: return ret;
      todo!()
    }

    // Java: static File[] doRealBackup(AppType type, File destFile, char seqCh)
    pub fn doRealBackup(_type: &AppType, _destFile: &dyn std::any::Any, _seqCh: char) -> Option<Vec<Box<dyn std::any::Any>>> {
      // Java: File backupDir = createBackupDir(type, destFile);
      // Java: String src_name = destFile.getName();
      // Java: File[] backups = backupDir.listFiles(new BackupFileFilter(src_name));
      // Java: Arrays.sort(backups, new BackupFileComparator(src_name));
      // Java: int seq = 0;
      // Java: if (backups.length > 0)
      {
         // Java: seq = Integer.valueOf(backups[backups.length - 1].getName().substring(src_name.length() + 1)) + 1;
      }
      // Java: File backupFile = null;
      // Java: do
      {
         // Java: backupFile = new File(backupDir, src_name + seqCh + '.' + Integer.toString(seq));
         // Java: ++seq;
      }
      // Java: moveOrCopyFile(destFile, backupFile);
      // Java: return backups;
      todo!()
    }

    // Java: static File createBackupDir(AppType type, File destFile)
    pub fn createBackupDir(_type: &AppType, _destFile: &dyn std::any::Any) -> Box<dyn std::any::Any> {
      // Java: String appBackup = Platform.getBackupDirectory(destFile.getPath());
      // Java: File backupParentDir = new File(appBackup, type.dirName);
      // Java: String hash = md5digest(destFile.getParent());
      // Java: File backupDir = new File(backupParentDir, hash);
      // Java: if (!backupDir.exists() && !backupDir.mkdirs())
      {
         // Java: for(char ch = 'A'; ch < 'J'; ++ch)
         {
            // Java: backupDir = new File(backupParentDir, hash + ch);
            // Java: if (backupDir.exists() || backupDir.mkdirs())
            {
               // Java: break;
            }
         }
      }
      // Java: if (!backupDir.exists())
      {
         // Java: throw new IOException("mkdir failed; backpuDir: " + backupDir.getAbsolutePath());
      }
      // Java: else
      {
         // Java: return backupDir;
      }
      todo!()
    }

    // Java: void finishSaving()
    pub fn finishSaving(&mut self) {
      // Java: if (this.tmpFile != null && this.tmpFile.exists() && this.tmpFile.length() > 0L)
      {
         // Java: if (this.destFile.exists())
         {
            // Java: backupFile(this.type, this.destFile);
         }
         // Java: moveOrCopyFile(this.tmpFile, this.destFile);
         // Java: if (this.tmpFile.exists())
         {
            // Java: this.tmpFile.delete();
         }
         // Java: int i = 0;
         // Java: for(int size = this.tmpFiles.size(); i < size; ++i)
         {
            // Java: File tmp = (File)this.tmpFiles.get(i);
            // Java: if (tmp != null && tmp.exists())
            {
               // Java: tmp.delete();
            }
         }
      }
      // Java: else
      {
         // Java: throw new IOException("File is empty");
      }
    }

    // Java: void discardSaving()
    pub fn discardSaving(&mut self) {
      // Java: if (this.tmpFile != null && this.tmpFile.exists())
      {
         // Java: this.tmpFile.delete();
      }
      // Java: int i = 0;
      // Java: for(int size = this.tmpFiles.size(); i < size; ++i)
      {
         // Java: File tmp = (File)this.tmpFiles.get(i);
         // Java: if (tmp != null && tmp.exists())
         {
            // Java: tmp.delete();
         }
      }
    }

    // Java: void addTempFile(File tmp)
    pub fn addTempFile(&mut self, _tmp: Box<dyn std::any::Any>) {
      // Java: this.tmpFiles.add(tmp);
      self.tmpFiles.push(_tmp);
    }

    // Java: static void moveOrCopyFile(File src, File dest)
    pub fn moveOrCopyFile(_src: &dyn std::any::Any, _dest: &dyn std::any::Any) {
      // Java: if (!src.exists())
      {
         // Java: throw new IOException("source file not exists");
      }
      // Java: else
      {
         // Java: if (dest.exists())
         {
            // Java: String path = dest.getAbsolutePath();
            // Java: if (!dest.delete())
            {
               // Java: throw new IOException("failed to delete destination file; Path: " + path);
            }
         }
         // Java: try
         {
            // Java: if (src.renameTo(dest))
            {
               // Java: return;
            }
         }
         // Java: catch (RuntimeException var3)
         {
         }
         // Java: copyFile(src, dest);
      }
    }

    // Java: static void copyFile(File src, File dest, boolean overwrite)
    pub fn copyFile_with_overwrite(_src: &dyn std::any::Any, _dest: &dyn std::any::Any, _overwrite: bool) {
      // Java: if (null != src && null != dest)
      {
         // Java: if (!src.getAbsolutePath().equalsIgnoreCase(dest.getAbsolutePath()))
         {
            // Java: if (!src.exists())
            {
               // Java: throw new IOException("source file not exists");
            }
            // Java: else if (!overwrite && dest.exists())
            {
               // Java: throw new IOException("overwrite destination file without permission");
            }
            // Java: else
            {
               // Java: File tmpFile = null;
               // Java: FileInputStream is = null;
               // Java: FileOutputStream os = null;
               // Java: try
               {
                  // Java: File parentDir = dest.getParentFile();
                  // Java: if (parentDir != null)
                  {
                     // Java: parentDir.mkdirs();
                  }
                  // Java: tmpFile = File.createTempFile("moffice", (String)null, parentDir);
                  // Java: is = new FileInputStream(src);
                  // Java: os = new FileOutputStream(tmpFile);
                  // Java: FileUtil.copy(is, os, (byte[])null);
                  // Java: if (tmpFile.renameTo(dest))
                  {
                     // Java: tmpFile = null;
                  }
               }
               // Java: finally
               {
                  // Java: FileUtil.closeQuietly(os);
                  // Java: FileUtil.closeQuietly(is);
                  // Java: if (null != tmpFile)
                  {
                     // Java: tmpFile.delete();
                  }
               }
            }
         }
      }
    }

    // Java: static void copyFile(File src, File dest)
    pub fn copyFile(_src: &dyn std::any::Any, _dest: &dyn std::any::Any) {
      // Java: copyFile(src, dest, true);
    }

    // Java: static String md5digest(String input)
    pub fn md5digest(_input: &str) -> Option<String> {
      // Java: MessageDigest algorithm;
      // Java: try
      {
         // Java: algorithm = MessageDigest.getInstance("MD5");
      }
      // Java: catch (NoSuchAlgorithmException var7)
      {
         // Java: return null;
      }
      // Java: algorithm.reset();
      // Java: try
      {
         // Java: algorithm.update(input.getBytes("UTF-8"));
      }
      // Java: catch (UnsupportedEncodingException var6)
      {
         // Java: return null;
      }
      // Java: byte[] messageDigest = algorithm.digest();
      // Java: if (messageDigest != null && messageDigest.length == 16)
      {
         // Java: StringBuffer hexString = new StringBuffer();
         // Java: for(int i = 0; i < messageDigest.length; ++i)
         {
            // Java: String hex = Integer.toHexString(255 & messageDigest[i]);
            // Java: if (hex.length() == 1)
            {
               // Java: hexString.append('0');
            }
            // Java: hexString.append(hex);
         }
         // Java: return hexString.toString();
      }
      // Java: else
      {
         // Java: return null;
      }
      todo!()
    }

}

// Java: public enum AppType
impl AppType {
    // Java: AppType(String dirName)
    pub fn new(_dirName: &str) -> Self {
         // Java: this.dirName = dirName;
         AppType { dirName: _dirName.to_string() }
    }

    // Java: String getDirName()
    pub fn getDirName(&self) -> &str {
         // Java: return this.dirName;
         &self.dirName
    }

}

// Java: public class BackupFileFilter
impl BackupFileFilter {
    // Java: BackupFileFilter(String srcFileName)
    pub fn new(_srcFileName: &str) -> Self {
         // Java: super();
         // Java: this.srcFileName = srcFileName;
         BackupFileFilter { srcFileName: _srcFileName.to_string() }
    }

    // Java: boolean accept(java.io.File pathname)
    pub fn accept(&self, _pathname: &dyn std::any::Any) -> bool {
         // Java: String name = pathname.getName();
         // Java: if (name.length() < this.srcFileName.length() + 2)
         {
            // Java: return false;
         }
         // Java: else if (name.startsWith(this.srcFileName) && name.charAt(this.srcFileName.length()) == '.')
         {
            // Java: int i = this.srcFileName.length() + 1;
            // Java: for(int len = name.length(); i < len; ++i)
            {
               // Java: if (!Character.isDigit(name.charAt(i)))
               {
                  // Java: return false;
               }
            }
            // Java: return true;
         }
         // Java: else
         {
            // Java: return false;
         }
         todo!()
    }

}

// Java: public class BackupFileComparator
impl BackupFileComparator {
    // Java: BackupFileComparator(String srcFileName)
    pub fn new(_srcFileName: &str) -> Self {
         // Java: super();
         // Java: this.srcFileName = srcFileName;
         BackupFileComparator { srcFileName: _srcFileName.to_string() }
    }

    // Java: int compare(File lhs, File rhs)
    pub fn compare(&self, _lhs: &dyn std::any::Any, _rhs: &dyn std::any::Any) -> i32 {
         // Java: String lseq_str = lhs.getName().substring(this.srcFileName.length() + 1);
         // Java: String rseq_str = rhs.getName().substring(this.srcFileName.length() + 1);
         // Java: int lseq = Integer.valueOf(lseq_str);
         // Java: int rseq = Integer.valueOf(rseq_str);
         // Java: return lseq - rseq;
         todo!()
    }

}
